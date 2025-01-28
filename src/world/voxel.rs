use bevy::math::bool;

use super::block::Block;

#[derive(Clone)]
pub struct Voxel {
    pub is_visible: bool,
    pub block: Block,
}

impl Voxel {
    pub fn new(is_visible: bool, block: Block) -> Self {
        Voxel { is_visible, block }
    }

    pub fn get_visible(self) -> bool {
        self.is_visible
    }
}
