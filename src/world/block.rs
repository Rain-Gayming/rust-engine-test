use bevy::math::Vec2;

#[derive(Clone)]
pub struct Block {
    pub block_name: String,
    pub texture_pos_top: Vec2,
    pub texture_pos_bottom: Vec2,
    pub texture_pos_front: Vec2,
    pub texture_pos_back: Vec2,
    pub texture_pos_left: Vec2,
    pub texture_pos_right: Vec2,
}

impl Block {
    pub fn stone() -> Block {
        let block = Block {
            block_name: "stone".to_string(),
            texture_pos_top: Vec2::new(-1., -1.),
            texture_pos_bottom: Vec2::new(-1., -1.),
            texture_pos_front: Vec2::new(-1., -1.),
            texture_pos_back: Vec2::new(-1., -1.),
            texture_pos_left: Vec2::new(-1., -1.),
            texture_pos_right: Vec2::new(-1., -1.),
        };

        block
    }
    pub fn dirt() -> Block {
        let block = Block {
            block_name: "dirt".to_string(),
            texture_pos_top: Vec2::new(-1., 1.),
            texture_pos_bottom: Vec2::new(-1., 1.),
            texture_pos_front: Vec2::new(-1., 1.),
            texture_pos_back: Vec2::new(-1., 1.),
            texture_pos_left: Vec2::new(-1., 1.),
            texture_pos_right: Vec2::new(-1., 1.),
        };

        block
    }
    pub fn sand() -> Block {
        let block = Block {
            block_name: "sand".to_string(),
            texture_pos_top: Vec2::new(1., 1.),
            texture_pos_bottom: Vec2::new(1., 1.),
            texture_pos_front: Vec2::new(1., 1.),
            texture_pos_back: Vec2::new(1., 1.),
            texture_pos_left: Vec2::new(1., 1.),
            texture_pos_right: Vec2::new(1., 1.),
        };

        block
    }
    pub fn grass() -> Block {
        let block = Block {
            block_name: "grass".to_string(),
            texture_pos_top: Vec2::new(1., -1.),
            texture_pos_bottom: Vec2::new(1., -1.),
            texture_pos_front: Vec2::new(1., -1.),
            texture_pos_back: Vec2::new(1., -1.),
            texture_pos_left: Vec2::new(1., -1.),
            texture_pos_right: Vec2::new(1., -1.),
        };

        block
    }
}
