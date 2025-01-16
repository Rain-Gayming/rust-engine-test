use crate::world::chunk_mesh_builder::ChunkMeshBuilder;

use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::Indices, utils::HashMap};
use iyes_perf_ui::entries;
//contains chunk informatiom ( position, voxels, ect )

use super::{rendering_constants::*, world::ChunkMap};

#[derive(Default)]
pub struct Chunk {
    voxels: [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    mesh_builder: ChunkMeshBuilder,
}

impl Chunk {
    pub fn new() -> Self {
        let mut voxels = [[[0u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        for x in 0..32usize {
            for y in 0..32usize {
                for z in 0..32usize {
                    voxels[x][y][z] = 1;
                }
            }
        }

        Chunk {
            voxels,
            mesh_builder: ChunkMeshBuilder::new(),
        }
    }
    pub fn build_mesh(
        mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        chunks: &mut ChunkMap,
        position: IVec3,
    ) -> Entity {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let val = &mut self.voxels[x][y][z];
                    if *val == 0 {
                        continue;
                    }

                    let coord = [x as u8, y as u8, z as u8];
                    if x == 0 || self.voxels[x - 1][y][z] == 0 {
                        self.mesh_builder.add_face(coord, 2);
                    }

                    if x == CHUNK_SIZE - 1 || self.voxels[x + 1][y][z] == 0 {
                        self.mesh_builder.add_face(coord, 3);
                    }

                    if y == 0 || self.voxels[x][y - 1][z] == 0 {
                        self.mesh_builder.add_face(coord, 5);
                    }

                    if y == CHUNK_SIZE - 1 || self.voxels[x][y + 1][z] == 0 {
                        self.mesh_builder.add_face(coord, 0);
                    }

                    if z == 0 || self.voxels[x][y][z - 1] == 0 {
                        self.mesh_builder.add_face(coord, 1);
                    }

                    if z == CHUNK_SIZE - 1 || self.voxels[x][y][z + 1] == 0 {
                        self.mesh_builder.add_face(coord, 4);
                    }
                }
            }
        }

        let chunk_mesh_handle: Handle<Mesh> = meshes.add(self.mesh_builder.build());

        let id = commands
            .spawn((
                Mesh3d(chunk_mesh_handle),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgba(0.2, 0.7, 0.1, 0.0),
                    alpha_mode: AlphaMode::Mask(0.2),
                    unlit: false,
                    ..Default::default()
                })),
                Transform {
                    translation: Vec3::new(
                        (position.x * 32) as f32,
                        (position.y * 32) as f32,
                        (position.z * 32) as f32,
                    ),
                    ..default()
                },
            ))
            .id();

        id
    }
}
