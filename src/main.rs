//! This example demonstrates how to create a custom mesh,
//! assign a custom UV mapping for a custom texture,
//! and how to change the UV mapping at run-time.
mod debug;
mod world;

#[warn(unused_variables)]
use bevy::prelude::*;
use bevy::{scene::ron::de, transform, window::PresentMode};
use bevy_fps_ui::*;
use world::chunk::Chunk;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FpsCounterPlugin)
        //systems
        .add_systems(Update, input_handler)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform = Transform::from_xyz(0., 40., 30.);

    // Camera in 3D space.
    commands.spawn((Camera3d::default(), camera_and_light_transform));

    // Light up the scene.
    commands.spawn((PointLight::default(), camera_and_light_transform));

    let chunk_x_amount = 16;
    let chunk_y_amount = 1;
    let chunk_z_amount = 16;

    for x in 0..chunk_x_amount {
        for y in 0..chunk_y_amount {
            for z in 0..chunk_z_amount {
                let xyz = Vec3::new(x as f32, y as f32, z as f32);
                let chunk = Chunk::new(xyz);
                let texture_handle = asset_server.load("array_texture.png");

                let chunk_mesh_handle: Handle<Mesh> = meshes.add(chunk.build_mesh());
                commands.spawn((
                    Mesh3d(chunk_mesh_handle),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture_handle.clone()),
                        unlit: false,
                        ..Default::default()
                    })),
                    Transform {
                        translation: Vec3::new((x * 32) as f32, (y * 32) as f32, (z * 32) as f32),
                        ..default()
                    },
                ));
            }
        }
    }
}

fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for mut transform in &mut query {
            transform.translation.y += 0.1
        }
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        for mut transform in &mut query {
            transform.translation.y -= 0.1
        }
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        for mut transform in &mut query {
            transform.translation.x += 0.1
        }
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        for mut transform in &mut query {
            transform.translation.x -= 0.1
        }
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        for mut transform in &mut query {
            transform.translation.z -= 0.1
        }
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        for mut transform in &mut query {
            transform.translation.z += 0.1
        }
    }
}
