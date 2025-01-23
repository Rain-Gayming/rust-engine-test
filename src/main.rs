mod world;

use std::f32::consts::FRAC_PI_2;

use bevy::color::palettes::css::WHITE;
#[warn(unused_variables)]
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::{input::mouse::AccumulatedMouseMotion, pbr::wireframe::*, window::PresentMode};
use bevy_fps_ui::*;
use world::world::{ChunkLoader, ChunkMap};
fn main() {
    App::new()
        //plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Voxel Engine".into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FpsCounterPlugin)
        .add_plugins(WireframePlugin)
        //resources
        .insert_resource(WireframeConfig {
            global: true,
            default_color: WHITE.into(),
        })
        .insert_resource(ChunkMap(HashMap::new()))
        //systems
        .add_systems(Update, (input_handler, cursor_ungrab))
        .add_systems(Startup, (setup, cursor_grab))
        .add_systems(Update, chunk_loader_system)
        .run();
}

fn setup(mut commands: Commands) {
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_transform = Transform::from_xyz(16., 32., 16.);
    let light_transform =
        Transform::from_xyz(100., 100., 100.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y);

    // Camera in 3D space.
    commands.spawn((
        Camera3d::default(),
        camera_transform,
        ChunkLoader {
            player_position: IVec3::new(0, 0, 0),
            loaded_chunks: vec![],
            chunk_entities: HashMap::new(),
        },
    ));

    // Light up the scene.
    commands.spawn((DirectionalLight::default(), light_transform));
}

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor_options.visible = false;
}
fn cursor_ungrab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();

        primary_window.cursor_options.grab_mode = CursorGrabMode::None;
        primary_window.cursor_options.visible = true;
    }
}
fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera3d>>,

    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    for mut transform in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity += forward;
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            velocity -= forward;
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            velocity -= right;
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            velocity += right;
        } else if keyboard_input.pressed(KeyCode::Space) {
            velocity += Vec3::Y;
        } else if keyboard_input.pressed(KeyCode::ControlLeft) {
            velocity -= Vec3::Y;
        }

        velocity = velocity.normalize_or_zero();

        transform.translation += velocity * time.delta_secs() * 15.
    }
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * 0.003;
        let delta_pitch = -delta.y * 0.003;

        for mut transform in &mut query {
            let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
            let yaw = yaw + delta_yaw;

            const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
            let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }
}

fn chunk_loader_system(
    mut cl_query: Query<(&mut ChunkLoader, &Transform)>,
    mut chunk_map: ResMut<ChunkMap>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut chunk_loader, transform) in cl_query.iter_mut() {
        let loader_position = transform.translation;
        let loader_chunk = IVec3::new(
            loader_position.x as i32 >> 5,
            loader_position.y as i32 >> 5,
            loader_position.z as i32 >> 5,
        );

        let render_distance = 4;
        let vertical_render_distance = 1;
        chunk_loader.update_player_position(
            loader_chunk,
            render_distance,
            vertical_render_distance,
            &mut chunk_map,
            &mut commands,
            &mut materials,
            &mut meshes,
        );
    }
}
