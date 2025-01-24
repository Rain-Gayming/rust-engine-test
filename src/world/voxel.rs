use bevy::math::bool;

use super::block::Block;

#[derive(Clone)]
pub struct Voxel {
    pub is_solid: bool,
    pub block: Block,
}

impl Voxel {
    pub fn new(is_solid: bool, block: Block) -> Self {
        Voxel {
            is_solid: is_solid,
            block: block,
        }
    }
}
