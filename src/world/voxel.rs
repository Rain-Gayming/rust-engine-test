use bevy::math::bool;

#[derive(Clone)]
pub struct Voxel {
    pub is_solid: bool,
}

impl Voxel {
    pub fn new(is_solid: bool) -> Self {
        Voxel { is_solid: is_solid }
    }

    pub fn get_solid(&mut self) -> bool {
        self.is_solid
    }
}
