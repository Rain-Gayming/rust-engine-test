use bevy::math::bool;

#[derive(Eq, PartialEq, Default, Copy, Clone, Debug)]
pub enum Block {
    #[default]
    Air,
    Grass,
    Dirt,
    Stone,
    Sand,
}

#[derive(Clone)]
pub struct Voxel {
    pub block: Block,
}

impl Voxel {
    pub fn new(block: Block) -> Self {
        Voxel { block }
    }
}
impl Block {
    pub fn is_solid(&self) -> bool {
        match self {
            Block::Air => false,
            Block::Grass => true,
            Block::Dirt => true,
            Block::Stone => true,
            Block::Sand => true,
        }
    }
    pub fn is_air(&self) -> bool {
        !self.is_solid()
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct BlockData {
    pub block_type: Block,
}
