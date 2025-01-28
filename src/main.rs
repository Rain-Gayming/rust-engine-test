mod game;
mod player;
mod ui;
mod world;

use bevy::ecs::world::CommandQueue;
use bevy::prelude::*;
use bevy::tasks::futures_lite::future;
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};
use bevy::utils::hashbrown::HashMap;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::{pbr::wireframe::*, window::PresentMode};
use bevy_egui::EguiPlugin;
use bevy_fps_ui::*;
use bevy_framepace::Limiter;
use game::game_state::GameState;
use game::settings::GameSettings;
use player::free_cam::*;
use player::player::Player;
use rand::Rng;
use ui::performance::ui_example_system;
use world::biome::BiomeGenerator;
use world::block::Block;
use world::chunk::{self, Chunk};
use world::chunk_mesh_builder::ChunkMeshBuilder;
use world::noise::NoiseGenerator;
use world::rendering_constants::CHUNK_SIZE;
use world::voxel::Voxel;
use world::world::{BiomeMap, ChunkGenerationTasks, ChunkMap, EntityChunkMap, NoiseMap};

#[derive(Component)]
struct ComputeTransform(Task<CommandQueue>);

fn main() {
    let seed: i32 = rand::thread_rng().gen_range(0..10000);
    App::new()
        //plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FpsCounterPlugin)
        .add_plugins(WireframePlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(bevy_framepace::FramepacePlugin)
        //resources
        /* .insert_resource(WireframeConfig {
            global: true,
            default_color: WHITE.into(),
        })*/
        .insert_resource(ChunkMap(HashMap::new()))
        .insert_resource(EntityChunkMap(HashMap::new()))
        .insert_resource(BiomeMap {
            biome_generator: BiomeGenerator::new(seed as u32),
        })
        .insert_resource(NoiseMap {
            noise_generator: NoiseGenerator::new(seed as u32),
        })
        .insert_resource(ChunkGenerationTasks {
            generating_chunks: HashMap::new(),
        })
        .insert_resource(GameSettings::default())
        .insert_resource(GameState { is_paused: false })
        //systems
        .add_systems(Update, (input_handler, cursor_grab, update))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                begin_chunk_generation.run_if(run_if_new_chunk),
                recieve_chunk_generation.run_if(run_if_new_chunk),
                unload_chunks.run_if(run_if_new_chunk),
                update_player_chunk_coord,
            ),
        )
        .add_systems(Update, ui_example_system)
        .run();
}

fn setup(mut commands: Commands) {
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_transform = Transform::from_xyz(16., 32., 16.);
    let light_transform =
        Transform::from_xyz(100., 100., 100.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y);

    //let seed = rand::thread_rng().gen_range(0..10000);

    // Camera in 3D space.
    commands.spawn((
        Camera3d::default(),
        camera_transform,
        Player {
            chunk_position: IVec3::new(0, 0, 0),
            in_new_chunk: true,
        },
    ));

    // Light up the scene.
    commands.spawn((DirectionalLight::default(), light_transform));
}

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
) {
    let mut primary_window = q_windows.single_mut();

    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.is_paused = !state.is_paused;

        if !state.is_paused {
            primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;
            primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
            primary_window.cursor_options.visible = false;
        } else {
            let mut primary_window = q_windows.single_mut();

            primary_window.cursor_options.grab_mode = CursorGrabMode::None;
            primary_window.cursor_options.visible = true;
        }
    }
}
fn update(settings: Res<GameSettings>, mut limiter: ResMut<bevy_framepace::FramepaceSettings>) {
    limiter.limiter = match limiter.limiter {
        Limiter::Auto => Limiter::from_framerate(settings.fps_limit as f64),
        Limiter::Off => Limiter::from_framerate(settings.fps_limit as f64),

        Limiter::Manual(_) => Limiter::from_framerate(settings.fps_limit as f64),
    }
}

fn update_player_chunk_coord(mut player_query: Query<(&mut Player, &Transform)>) {
    for mut player in &mut player_query {
        let position = player.1.translation;
        let old_chunk_coords = IVec3::new(
            player.0.chunk_position.x,
            player.0.chunk_position.y,
            player.0.chunk_position.z,
        );
        let new_chunk_coords = IVec3::new(
            position.x as i32 >> 5,
            position.y as i32 >> 5,
            position.z as i32 >> 5,
        );
        if old_chunk_coords != new_chunk_coords {
            player.0.in_new_chunk = true;
            player.0.chunk_position = new_chunk_coords;
        } else {
            player.0.in_new_chunk = false;
        }
    }
}

fn run_if_new_chunk(player_transform: Single<&Player>) -> bool {
    player_transform.in_new_chunk
}

fn begin_chunk_generation(
    game_settings: Res<GameSettings>,
    mut tasks: ResMut<ChunkGenerationTasks>,
    mut cl_query: Query<(&mut Player, &Transform)>,
    mut chunks: ResMut<ChunkMap>,
) {
    for player in &mut cl_query {
        //x - view dist + x + view dist gets all the chunks around the player
        let task_pool = AsyncComputeTaskPool::get();

        for x in player.0.chunk_position.x - game_settings.render_distance
            ..=player.0.chunk_position.x + game_settings.render_distance
        {
            for y in player.0.chunk_position.y - game_settings.vertical_render_distance
                ..=player.0.chunk_position.y + game_settings.vertical_render_distance
            {
                for z in player.0.chunk_position.z - game_settings.render_distance
                    ..=player.0.chunk_position.z + game_settings.render_distance
                {
                    let chunk_coords: IVec3;
                    chunk_coords = IVec3::new(x as i32, y as i32, z as i32);

                    if y == 0 && !chunks.0.contains_key(&chunk_coords) {
                        let new_chunk = Chunk::new();
                        let gen_chunk = new_chunk.clone();
                        let task = task_pool.spawn(async move {
                            //what goes here exactly?
                            new_chunk
                        });
                        tasks.generating_chunks.insert(chunk_coords, task);
                        chunks.insert(chunk_coords, gen_chunk);
                    }
                }
            }
        }
    }
}

fn recieve_chunk_generation(
    mut my_tasks: ResMut<ChunkGenerationTasks>,
    mut chunks: ResMut<ChunkMap>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_entities: ResMut<EntityChunkMap>,
    asset_server: Res<AssetServer>,
    noise_generator: Res<NoiseMap>,
    biome_generator: Res<BiomeMap>,
) {
    let mut chunks_loaded: i32 = 0;
    my_tasks.generating_chunks.retain(|chunk_coord, task| {
        // check on our task to see how it's doing :)
        let status = block_on(future::poll_once(task));

        // keep the entry in our HashMap only if the task is not done yet
        let retain = status.is_none();

        let chunk_coords = chunk_coord.clone();

        // if this task is done, handle the data it returned!
        if let Some(mut chunk_data) = status {
            chunks_loaded += 1;
            chunks.insert(chunk_coords, chunk_data.clone());

            let mut my_chunk_builder = ChunkMeshBuilder::new();
            let biome = biome_generator
                .biome_generator
                .get_biome(chunk_coords.x as f64, chunk_coords.z as f64);

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let world_pos = local_pos_to_world(
                            *chunk_coord,
                            Vec3::new(x as f32, y as f32, z as f32),
                        );
                        let new_voxel_pos = [x, y, z];
                        let height_variation = noise_generator.noise_generator.get_height(
                            world_pos.x as f32,
                            world_pos.z as f32,
                            biome.frequency,
                            biome.amplitude,
                        );
                        let block: Block;

                        let is_visible = (world_pos.y as f32)
                            < biome.base_height as f32 + (height_variation as f32).round();
                        /*if new_voxel_pos[1] < biome.base_height - 3 {
                            block = Block::stone();
                        } else {
                            block = biome.clone().surface_block;
                        }*/

                        block = biome.clone().surface_block;
                        let voxel = Voxel::new(is_visible, block);

                        chunk_data.voxels_in_chunk.insert(new_voxel_pos, voxel);
                    }
                }
            }

            //actually makes their mesh
            for voxel in chunk_data.voxels_in_chunk.iter() {
                let voxel_position = voxel.0;
                if voxel.1.is_visible {
                    //left face
                    if voxel_position[0] == 0
                        || !chunk_data
                            .voxels_in_chunk
                            .get(&[voxel_position[0] - 1, voxel_position[1], voxel_position[2]])
                            .unwrap()
                            .is_visible
                    {
                        my_chunk_builder.add_face(
                            *voxel_position,
                            2,
                            voxel.1.block.texture_pos_left,
                        );
                    }

                    //right face
                    if voxel_position[0] == CHUNK_SIZE - 1
                        || !chunk_data
                            .voxels_in_chunk
                            .get(&[voxel_position[0] + 1, voxel_position[1], voxel_position[2]])
                            .unwrap()
                            .is_visible
                    {
                        my_chunk_builder.add_face(
                            *voxel_position,
                            3,
                            voxel.1.block.texture_pos_right,
                        );
                    }

                    //bottom face
                    if voxel_position[1] == 0
                        || !chunk_data
                            .voxels_in_chunk
                            .get(&[voxel_position[0], voxel_position[1] - 1, voxel_position[2]])
                            .unwrap()
                            .is_visible
                    {
                        my_chunk_builder.add_face(
                            *voxel_position,
                            5,
                            voxel.1.block.texture_pos_bottom,
                        );
                    }

                    //top faces
                    if voxel_position[1] == CHUNK_SIZE - 1
                        || !chunk_data
                            .voxels_in_chunk
                            .get(&[voxel_position[0], voxel_position[1] + 1, voxel_position[2]])
                            .unwrap()
                            .is_visible
                    {
                        my_chunk_builder.add_face(
                            *voxel_position,
                            0,
                            voxel.1.block.texture_pos_top,
                        );
                    }

                    //front chunk
                    if voxel_position[2] == 0
                        || !chunk_data
                            .voxels_in_chunk
                            .get(&[voxel_position[0], voxel_position[1], voxel_position[2] - 1])
                            .unwrap()
                            .is_visible
                    {
                        my_chunk_builder.add_face(
                            *voxel_position,
                            1,
                            voxel.1.block.texture_pos_front,
                        );
                    }

                    //back chunk
                    if voxel_position[2] == CHUNK_SIZE - 1
                        || !chunk_data
                            .voxels_in_chunk
                            .get(&[voxel_position[0], voxel_position[1], voxel_position[2] + 1])
                            .unwrap()
                            .is_visible
                    {
                        my_chunk_builder.add_face(
                            *voxel_position,
                            4,
                            voxel.1.block.texture_pos_back,
                        );
                    }
                }
            }

            let chunk_mesh_handle: Handle<Mesh> = meshes.add(my_chunk_builder.build());
            let custom_texture_handle: Handle<Image> = asset_server.load("array_texture.png");
            let chunk_id = commands
                .spawn((
                    Transform::from_xyz(
                        chunk_coord.x as f32 * CHUNK_SIZE as f32,
                        chunk_coord.y as f32 * CHUNK_SIZE as f32,
                        chunk_coord.z as f32 * CHUNK_SIZE as f32,
                    ),
                    Mesh3d(chunk_mesh_handle),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(custom_texture_handle),
                        alpha_mode: AlphaMode::Mask(0.2),
                        unlit: false,
                        ..Default::default()
                    })),
                ))
                .id();
            chunk_entities.insert(*chunk_coord, chunk_id);
        }
        retain
    });
    println!(
        "chunk rendering finished with {} chunks rendered",
        chunks_loaded
    );
}

pub fn local_pos_to_world(offset: IVec3, local_pos: Vec3) -> Vec3 {
    Vec3::new(
        local_pos.x as f32 + (offset[0] as f32 * CHUNK_SIZE as f32),
        local_pos.y as f32 + (offset[1] as f32 * CHUNK_SIZE as f32),
        local_pos.z as f32 + (offset[2] as f32 * CHUNK_SIZE as f32),
    )
}
fn unload_chunks(
    settings: Res<GameSettings>,
    player_transform: Single<&Transform, With<Player>>,
    mut chunks: ResMut<ChunkMap>,
    mut chunk_entities: ResMut<EntityChunkMap>,
    mut commands: Commands,
) {
    let mut keys: Vec<IVec3> = Vec::new();
    for key in chunks.keys() {
        keys.push(key.clone());
    }

    let player_pos = player_transform.translation;
    for chunk_pos in keys {
        let chunk_location = Vec3::new(chunk_pos.x as f32, chunk_pos.y as f32, chunk_pos.z as f32);
        let distance = Vec3::distance(player_pos, chunk_location);
        if distance > settings.render_distance as f32 * CHUNK_SIZE as f32 {
            if let Some(entity) = chunk_entities.get(&chunk_pos) {
                commands.entity(*entity).despawn();
                chunk_entities.remove(&chunk_pos);
                chunks.remove(&chunk_pos);
            }
        }
    }
}
