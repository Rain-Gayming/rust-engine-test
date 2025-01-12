//! This example demonstrates how to create a custom mesh,
//! assign a custom UV mapping for a custom texture,
//! and how to change the UV mapping at run-time.
#[warn(unused_variables)]
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use bevy_fps_ui::*;

#[derive(Component)]
struct CustomUV;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsCounterPlugin)
        //systems
        .add_systems(Update, input_handler)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let chunk_x_size: i32 = 16;
    let chunk_y_size: i32 = 16;
    let chunk_z_size: i32 = 16;

    for x in 0..chunk_x_size {
        for y in 0..chunk_y_size {
            for z in 0..chunk_z_size {
                // Import the custom texture.
                let custom_texture_handle: Handle<Image> = asset_server.load("array_texture.png");
                // Create and save a handle to the mesh.
                let cube_mesh_handle: Handle<Mesh> =
                    meshes.add(create_voxel(x as f32, y as f32, z as f32));

                // Render the mesh with the custom texture, and add the marker.
                commands.spawn((
                    Mesh3d(cube_mesh_handle),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(custom_texture_handle),
                        ..default()
                    })),
                    CustomUV,
                ));
            }
        }
    }
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(0., 20., 20.).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands.spawn((Camera3d::default(), camera_and_light_transform));

    // Light up the scene.
    commands.spawn((PointLight::default(), camera_and_light_transform));
}

#[rustfmt::skip]
fn create_voxel(
    x_offset: f32,
    y_offset: f32,
    z_offset: f32
) -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // top (facing towards +y)
            [-0.5 + x_offset, 0.5 + y_offset, -0.5 + z_offset], // vertex with index 0
            [0.5 + x_offset, 0.5 + y_offset, -0.5 + z_offset], // vertex with index 1
            [0.5 + x_offset, 0.5 + y_offset, 0.5+ z_offset], // etc. until 23
            [-0.5 + x_offset, 0.5 + y_offset, 0.5 + z_offset],
            // bottom   (-y)
            [-0.5 + x_offset, -0.5 + y_offset, -0.5 + z_offset],
            [0.5 + x_offset, -0.5 + y_offset, -0.5 + z_offset],
            [0.5 + x_offset, -0.5 + y_offset, 0.5 + z_offset],
            [-0.5 + x_offset, -0.5 + y_offset, 0.5 + z_offset],
            // right    (+x)
            [0.5 + x_offset, -0.5 + y_offset, -0.5 + z_offset],
            [0.5 + x_offset, -0.5 + y_offset, 0.5+ z_offset],
            [0.5 + x_offset, 0.5 + y_offset, 0.5 + z_offset], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [0.5 + x_offset, 0.5 + y_offset, -0.5 + z_offset],
            // left     (-x)
            [-0.5 + x_offset, -0.5 + y_offset, -0.5 + z_offset],
            [-0.5 + x_offset, -0.5 + y_offset, 0.5+ z_offset],
            [-0.5 + x_offset, 0.5 + y_offset, 0.5 + z_offset],
            [-0.5 + x_offset, 0.5 + y_offset, -0.5 + z_offset],
            // back     (+z)
            [-0.5 + x_offset, -0.5 + y_offset, 0.5 + z_offset],
            [-0.5 + x_offset, 0.5 + y_offset, 0.5 + z_offset],
            [0.5 + x_offset, 0.5 + y_offset, 0.5 + z_offset],
            [0.5 + x_offset, -0.5 + y_offset, 0.5 + z_offset],
            // forward  (-z)
            [-0.5 + x_offset, -0.5 + y_offset, -0.5 + z_offset],
            [-0.5 + x_offset, 0.5 + y_offset, -0.5 + z_offset],
            [0.5 + x_offset, 0.5 + y_offset, -0.5 + z_offset],
            [0.5 + x_offset, -0.5 + y_offset, -0.5 + z_offset],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.2],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
            // Assigning the UV coords for the right side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the left side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the back side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
            // Assigning the UV coords for the forward side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ]))
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
    if keyboard_input.pressed(KeyCode::KeyW) {
        for mut transform in &mut query {
            transform.translation.x += 0.1
        }
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        for mut transform in &mut query {
            transform.translation.x -= 0.1
        }
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        for mut transform in &mut query {
            transform.translation.z -= 0.1
        }
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        for mut transform in &mut query {
            transform.translation.z += 0.1
        }
    }
}
