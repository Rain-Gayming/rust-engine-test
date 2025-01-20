#[derive(Clone)]
pub struct Voxel {
    pub is_solid: bool,
    pub position: [u8; 3],
}

impl Voxel {
    pub fn new(is_solid: bool, position: [u8; 3]) -> Self {
        Voxel {
            is_solid: is_solid,
            position: position,
        }
    }

    pub fn get_solid(&mut self) -> bool {
        self.is_solid
    }
}
