use bevy::math::Vec2;

#[derive(Clone)]
pub struct Block {
    pub block_name: String,
    pub texture_pos: Vec2,
}

impl Block {
    pub fn stone() -> Block {
        let block = Block {
            block_name: "stone".to_string(),
            texture_pos: Vec2::new(-1., -1.),
        };

        block
    }
    pub fn dirt() -> Block {
        let block = Block {
            block_name: "dirt".to_string(),
            texture_pos: Vec2::new(-1., 1.),
        };

        block
    }
    pub fn sand() -> Block {
        let block = Block {
            block_name: "sand".to_string(),
            texture_pos: Vec2::new(1., 1.),
        };

        block
    }
    pub fn grass() -> Block {
        let block = Block {
            block_name: "grass".to_string(),
            texture_pos: Vec2::new(1., -1.),
        };

        block
    }
}
