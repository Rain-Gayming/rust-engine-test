use crate::world::chunk_mesh_builder::ChunkMeshBuilder;

use bevy::{ecs::component, prelude::*};
//contains chunk informatiom ( position, voxels, ect )

const CHUNK_X_SIZE: usize = 32;
const CHUNK_Y_SIZE: usize = 32;
const CHUNK_Z_SIZE: usize = 32;

pub struct Chunk {
    voxels: [[[u32; CHUNK_X_SIZE]; CHUNK_Y_SIZE]; CHUNK_Z_SIZE],
    mesh_builder: ChunkMeshBuilder,
    position: Vec3,
}

impl Chunk {
    pub fn new(chunk_pos: Vec3) -> Self {
        let mut voxels = [[[0u32; CHUNK_X_SIZE]; CHUNK_Y_SIZE]; CHUNK_Z_SIZE];
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
            position: chunk_pos,
        }
    }
    pub fn build_mesh(mut self) -> Mesh {
        for x in 0..CHUNK_X_SIZE {
            for y in 0..CHUNK_Y_SIZE {
                for z in 0..CHUNK_Z_SIZE {
                    let val = &mut self.voxels[x][y][z];
                    if *val == 0 {
                        continue;
                    }

                    let coord = [x as u8, y as u8, z as u8];
                    if x == 0 || self.voxels[x - 1][y][z] == 0 {
                        self.mesh_builder.add_face(coord, 2);
                    }

                    if x == CHUNK_X_SIZE - 1 || self.voxels[x + 1][y][z] == 0 {
                        self.mesh_builder.add_face(coord, 3);
                    }

                    if y == 0 || self.voxels[x][y - 1][z] == 0 {
                        self.mesh_builder.add_face(coord, 5);
                    }

                    if y == CHUNK_Y_SIZE - 1 || self.voxels[x][y + 1][z] == 0 {
                        self.mesh_builder.add_face(coord, 0);
                    }

                    if z == 0 || self.voxels[x][y][z - 1] == 0 {
                        self.mesh_builder.add_face(coord, 1);
                    }

                    if z == CHUNK_Z_SIZE - 1 || self.voxels[x][y][z + 1] == 0 {
                        self.mesh_builder.add_face(coord, 4);
                    }
                }
            }
        }

        self.mesh_builder.build()
    }
}
