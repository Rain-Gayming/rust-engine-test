use bevy::math::Vec2;

#[derive(Clone)]
pub struct Block {
    pub texture_pos_top: [u8; 2],
    pub texture_pos_bottom: [u8; 2],
    pub texture_pos_front: [u8; 2],
    pub texture_pos_back: [u8; 2],
    pub texture_pos_left: [u8; 2],
    pub texture_pos_right: [u8; 2],
}

impl Block {
    pub fn air() -> Block {
        Block {
            texture_pos_top: [255, 255],
            texture_pos_bottom: [255, 255],
            texture_pos_front: [255, 255],
            texture_pos_back: [255, 255],
            texture_pos_left: [255, 255],
            texture_pos_right: [255, 255],
        }
    }
    pub fn stone() -> Block {
        Block {
            texture_pos_top: [0, 0],
            texture_pos_bottom: [0, 0],
            texture_pos_front: [0, 0],
            texture_pos_back: [0, 0],
            texture_pos_left: [0, 0],
            texture_pos_right: [0, 0],
        }
    }
    pub fn dirt() -> Block {
        Block {
            texture_pos_top: [0, 1],
            texture_pos_bottom: [0, 1],
            texture_pos_front: [0, 1],
            texture_pos_back: [0, 1],
            texture_pos_right: [0, 1],
            texture_pos_left: [0, 1],
        }
    }
    pub fn sand() -> Block {
        Block {
            texture_pos_top: [1, 1],
            texture_pos_bottom: [1, 1],
            texture_pos_front: [1, 1],
            texture_pos_back: [1, 1],
            texture_pos_left: [1, 1],
            texture_pos_right: [1, 1],
        }
    }
    pub fn grass() -> Block {
        Block {
            texture_pos_top: [1, 0],
            texture_pos_bottom: [1, 0],
            texture_pos_front: [1, 0],
            texture_pos_back: [1, 0],
            texture_pos_left: [1, 0],
            texture_pos_right: [1, 0],
        }
    }
}
