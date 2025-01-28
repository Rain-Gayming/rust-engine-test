use noise::{NoiseFn, Perlin};

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
        Biome {
            surface_block: Block::grass(),
            base_height: 10,
            frequency: 0.05,
            amplitude: 7.0,
        }
    }
    pub fn desert() -> Biome {
        Biome {
            surface_block: Block::sand(),
            base_height: 10,
            frequency: 0.025,
            amplitude: 3.0,
        }
    }
}

#[derive(Clone)]
pub struct BiomeGenerator {
    temperature_noise: Perlin,
    rainfall_noise: Perlin,
}

impl BiomeGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            temperature_noise: Perlin::new(seed),
            rainfall_noise: Perlin::new(seed.wrapping_add(1)),
        }
    }

    pub fn get_biome(&self, wx: f64, wz: f64) -> Biome {
        let scale = 0.03;
        let temperature = self.temperature_noise.get([wx * scale, wz * scale]);
        let rainfall = self.rainfall_noise.get([wx * scale, wz * scale]);
        //println!("rainfall: {}", rainfall);
        //println!("temp: {}", temperature);
        match (temperature, rainfall) {
            (t, r) if t > 0.5 && r < -0.3 => Biome::desert(),
            _ => Biome::planes(),
        }
    }
}
