//! This example demonstrates how to create a custom mesh,
//! assign a custom UV mapping for a custom texture,
//! and how to change the UV mapping at run-time.
mod debug;
mod world;

use std::f32::consts::FRAC_PI_2;

#[warn(unused_variables)]
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::{
    color::palettes::css::LIME, input::mouse::AccumulatedMouseMotion, pbr::wireframe::*,
    window::PresentMode,
};
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
        .add_plugins(WireframePlugin)
        //systems
        .add_systems(Update, input_handler)
        .add_systems(Startup, (setup, cursor_grab))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform = Transform::from_xyz(16., 32., 16.);

    // Camera in 3D space.
    commands.spawn((Camera3d::default(), camera_and_light_transform));

    // Light up the scene.
    commands.spawn((PointLight::default(), camera_and_light_transform));

    let chunk_x_amount = 4;
    let chunk_y_amount = 4;
    let chunk_z_amount = 4;

    for x in 0..chunk_x_amount {
        for y in 0..chunk_y_amount {
            for z in 0..chunk_z_amount {
                let xyz = Vec3::new(x as f32, y as f32, z as f32);
                let chunk = Chunk::new(xyz);
                //let texture_handle = asset_server.load("array_texture.png");

                let chunk_mesh_handle: Handle<Mesh> = meshes.add(chunk.build_mesh());
                commands.spawn((
                    Mesh3d(chunk_mesh_handle),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        //base_color_texture: Some(texture_handle.clone()),
                        unlit: false,
                        ..Default::default()
                    })),
                    Transform {
                        translation: Vec3::new((x * 32) as f32, (y * 32) as f32, (z * 32) as f32),
                        ..default()
                    },
                    Wireframe,
                    WireframeColor { color: LIME.into() },
                ));
            }
        }
    }
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
fn cursor_ungrab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor_options.grab_mode = CursorGrabMode::None;
    primary_window.cursor_options.visible = true;
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
