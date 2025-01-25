use bevy::prelude::*;

use super::{biome::BiomeGenerator, chunk::Chunk, noise::NoiseGenerator};
use bevy::utils::HashMap;

#[derive(Resource, Deref, DerefMut)]
pub struct ChunkMap(#[deref] pub HashMap<IVec3, Chunk>);

#[derive(Component)]
pub struct ChunkLoader {
    pub player_position: IVec3,
    pub loaded_chunks: Vec<IVec3>,
    pub chunk_entities: HashMap<IVec3, Entity>,
    pub noise_generator: NoiseGenerator,
    pub biome_generator: BiomeGenerator,
}

impl ChunkLoader {
    pub fn update_player_position(
        &mut self,
        new_position: IVec3,
        view_distance: i32,
        vertical_view_distance: i32,
        chunks: &mut ChunkMap,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        asset_server: &mut Res<AssetServer>,
    ) {
        let old_chunk_coords = self.player_position;
        let new_chunk_coords = new_position;

        //has the player enetered a new chunk
        if old_chunk_coords != new_chunk_coords {
            println!("loading in a chunk");
            println!("old chunk: {}", old_chunk_coords);
            println!("new chunk: {}", new_chunk_coords);

            //load the chunks around the new chunk
            self.load_chunks(
                new_position,
                view_distance,
                vertical_view_distance,
                chunks,
                commands,
                materials,
                meshes,
                asset_server,
            );

            //unload the old chunks
            self.unload_chunks(
                old_chunk_coords,
                view_distance,
                vertical_view_distance,
                commands,
            );

            //update player position reference
            self.player_position = new_position;
        }
    }

    fn load_chunks(
        &mut self,
        position: IVec3,
        view_distance: i32,
        vertical_view_distance: i32,
        chunks: &mut ChunkMap,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        asset_server: &mut Res<AssetServer>,
    ) {
        //x - view dist + x + view dist gets all the chunks around the player

        let mut chunks_loaded: i32 = 0;
        for x in position.x - view_distance..=position.x + view_distance {
            for y in position.y - vertical_view_distance..=position.y + vertical_view_distance {
                for z in position.z - view_distance..=position.z + view_distance {
                    let chunk_coords: IVec3;
                    chunk_coords = IVec3::new(x, y, z);

                    if y == 0 {
                        if !self.loaded_chunks.contains(&chunk_coords) {
                            chunks_loaded += 1;
                            //create a chunk
                            let chunk = Chunk::new();

                            //add it to loaded chunks
                            self.loaded_chunks.push(chunk_coords);

                            //make its mesh
                            chunks.0.insert(chunk_coords, chunk.clone());
                            let new_chunk = chunk.build_mesh(
                                commands,
                                meshes,
                                materials,
                                chunk_coords,
                                chunks,
                                self.noise_generator.clone(),
                                asset_server,
                                self.biome_generator
                                    .get_biome(chunk_coords.x as f64, chunk_coords.z as f64),
                            );
                            self.chunk_entities.insert(chunk_coords, new_chunk);
                            /*println!(
                                "Loaded chunk at ({}, {}, {})",
                                chunk_coords.x, chunk_coords.y, chunk_coords.z
                            );*/
                        }
                    }
                }
            }
        }
        println!("chunks loaded: {}", chunks_loaded);
    }

    fn unload_chunks(
        &mut self,
        position: IVec3,
        view_distance: i32,
        vertical_view_distance: i32,
        commands: &mut Commands,
    ) {
        println!("getting chunks to unload");

        let mut chunks_unloaded: i32 = 0;
        for chunk_coords in self.loaded_chunks.clone() {
            let f_pos = Vec3::new(position.x as f32, position.y as f32, position.z as f32);
            let f_c_pos = Vec3::new(
                chunk_coords.x as f32,
                chunk_coords.y as f32 - vertical_view_distance as f32,
                chunk_coords.z as f32,
            );
            let distance = Vec3::distance(f_pos, f_c_pos);

            if distance > view_distance as f32 {
                if let Some(entity) = self.chunk_entities.get(&chunk_coords) {
                    commands.entity(*entity).despawn_recursive();
                } else {
                    println!("ERROR: chunk not unloaded for some reason");
                    return;
                }

                chunks_unloaded += 1;
                //remove the chunk
                let loaded_index = self
                    .loaded_chunks
                    .iter()
                    .position(|&r| r == chunk_coords)
                    .unwrap();
                self.loaded_chunks.remove(loaded_index);
            }
        }
        println!("chunks unloaded: {}", chunks_unloaded);
    }
}
