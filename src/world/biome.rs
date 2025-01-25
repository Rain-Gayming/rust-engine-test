use super::block::Block;

#[derive(Clone)]
pub struct Biome {
    pub surface_block: Block,
    pub base_height: u8,
    pub frequency: f32,
    pub amplitude: f32,
}

impl Biome {
    pub fn planes() -> Biome {
        let biome = Biome {
            surface_block: Block::grass(),
            base_height: 10,
            frequency: 0.05,
            amplitude: 7.0,
        };
        biome
    }
    pub fn desert() -> Biome {
        let biome = Biome {
            surface_block: Block::sand(),
            base_height: 10,
            frequency: 0.025,
            amplitude: 3.0,
        };
        biome
    }
}
