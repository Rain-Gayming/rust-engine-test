use std::clone;

use bevy::{prelude::*, tasks::Task};

use super::{biome::BiomeGenerator, chunk::Chunk, noise::NoiseGenerator};
use bevy::utils::HashMap;

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct ChunkMap(#[deref] pub HashMap<IVec3, Chunk>);
#[derive(Resource, Deref, DerefMut, Clone)]
pub struct EntityChunkMap(#[deref] pub HashMap<IVec3, Entity>);

#[derive(Resource)]
pub struct BiomeMap {
    pub biome_generator: BiomeGenerator,
}
#[derive(Resource)]
pub struct NoiseMap {
    pub noise_generator: NoiseGenerator,
}

#[derive(Resource)]
pub struct ChunkGenerationTasks {
    pub generating_chunks: HashMap<IVec3, Task<Chunk>>,
}
