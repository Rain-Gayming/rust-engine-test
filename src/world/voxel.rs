use bevy::math::bool;

#[derive(Clone)]
pub struct Voxel {
    pub is_solid: bool,
}

impl Voxel {
    pub fn new(is_solid: bool) -> Self {
        Voxel { is_solid: is_solid }
    }
}
