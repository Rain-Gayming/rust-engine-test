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
                    new_voxels.push(Voxel::new(Block::Air));
                }
            }
        }

        Chunk {
            voxels_in_chunk: new_voxels,
        }
    }

    pub fn set_voxel(&mut self, voxel_index: usize, block: Block) {
        self.voxels_in_chunk[voxel_index].block = block;
    }
}
