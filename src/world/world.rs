use bevy::prelude::*;

use super::chunk::Chunk;
use bevy::{ecs::component, prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct ChunkMap(HashMap<IVec3, Chunk>);

pub(crate) fn generate_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let chunk_x_amount = 4;
    let chunk_y_amount = 4;
    let chunk_z_amount = 4;

    for x in 0..chunk_x_amount {
        for y in 0..chunk_y_amount {
            for z in 0..chunk_z_amount {
                let xyz = Vec3::new(x as f32, y as f32, z as f32);
                let chunk = Chunk::new(xyz);
                let texture_handle = asset_server.load("test.png");

                let chunk_mesh_handle: Handle<Mesh> = meshes.add(chunk.build_mesh());
                commands.spawn((
                    Mesh3d(chunk_mesh_handle),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgba(0.2, 0.7, 0.1, 0.0),
                        alpha_mode: AlphaMode::Mask(0.2),

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
