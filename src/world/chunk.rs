use crate::world::chunk_mesh_builder::ChunkMeshBuilder;

use bevy::prelude::*;

use super::{rendering_constants::*, voxel::Block, voxel::Voxel};

#[derive(Clone)]
pub struct Chunk {
    pub voxels_in_chunk: Vec<Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        let mut new_voxels = Vec::new();
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    new_voxels.push(Voxel::new(false, Block::Air));
                }
            }
        }

        Chunk {
            voxels_in_chunk: new_voxels,
        }
    }

    pub fn get_visible(&mut self, position: IVec3) -> bool {
        let index = (position.x
            + position.x * CHUNK_SIZE as i32
            + position.z * CHUNK_SIZE as i32 * CHUNK_SIZE as i32) as usize;
        self.voxels_in_chunk[index].is_visible
    }

    pub fn set_voxel(&mut self, voxel_index: usize, is_visible: bool, block: Block) {
        self.voxels_in_chunk[voxel_index].is_visible = is_visible;
        self.voxels_in_chunk[voxel_index].block = block;
    }
}
