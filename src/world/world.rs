use bevy::prelude::*;

use super::chunk::Chunk;
use bevy::utils::HashMap;

#[derive(Resource, Deref, DerefMut)]
pub struct ChunkMap(#[deref] pub HashMap<IVec3, Chunk>);

#[derive(Component)]
pub struct ChunkLoader {
    pub player_position: IVec3,
    pub loaded_chunks: Vec<IVec3>,
    pub chunk_entities: HashMap<IVec3, Entity>,
}

impl ChunkLoader {
    pub fn update_player_position(
        &mut self,
        new_position: IVec3,
        view_distance: i32,
        chunks: &mut ChunkMap,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        let old_chunk_coords = self.player_position;
        let new_chunk_coords = new_position;

        //has the player enetered a new chunk
        if old_chunk_coords != new_chunk_coords {
            println!("loading in a chunk");
            println!("old chunk: {}", old_chunk_coords);
            println!("new chunk: {}", new_chunk_coords);

            //unload the old chunks
            let chunks_to_unload = self.get_chunks_to_unload(old_chunk_coords, view_distance);
            for chunk_coords in chunks_to_unload {
                self.unload_chunk(chunk_coords, commands);
            }

            //load the chunks around the new chunk
            let chunks_to_load = self.get_chunks_to_load(new_position, view_distance);
            for chunk_coords in chunks_to_load {
                self.load_chunk(chunk_coords, chunks, commands, materials, meshes);
            }

            //update player position reference
            self.player_position = new_position;
        }
    }

    fn get_chunks_to_load(&mut self, position: IVec3, view_distance: i32) -> Vec<IVec3> {
        let mut chunks_to_load = vec![];
        //x - view dist + x + view dist gets all the chunks around the player

        for x in position.x - view_distance..=position.x + view_distance {
            for y in position.y - view_distance..=position.y + view_distance {
                for z in position.z - view_distance..=position.z + view_distance {
                    let chunk_coords: IVec3;
                    if y <= 5 {
                        chunk_coords = IVec3::new(x, y, z);
                    } else {
                        chunk_coords = IVec3::new(x, 5, z);
                    }
                    if !self.loaded_chunks.contains(&chunk_coords) {
                        chunks_to_load.push(chunk_coords);
                        //println!("loading {}", chunk_coords);
                    }
                }
            }
        }

        println!(
            "loader: total chunks currently loaded: {}",
            &self.loaded_chunks.len()
        );
        chunks_to_load
    }

    fn get_chunks_to_unload(&mut self, position: IVec3, view_distance: i32) -> Vec<IVec3> {
        let mut chunks_to_unload = vec![];
        println!("getting chunks to unload");
        println!(
            "unloader: total chunks currently loaded: {}",
            &self.loaded_chunks.len()
        );
        for chunk_coords in self.loaded_chunks.clone() {
            let distance = (chunk_coords.x - position.x).abs()
                + (chunk_coords.y - position.y).abs()
                + (chunk_coords.z - position.z).abs();
            if distance > view_distance {
                chunks_to_unload.push(chunk_coords);
                /*println!(
                    "chunk: {} is too far (at distance {}) despawning",
                    chunk_coords, distance
                );*/
            }
        }
        chunks_to_unload
    }

    fn load_chunk(
        &mut self,
        chunk_coords: IVec3,
        chunks: &mut ChunkMap,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        if !self.loaded_chunks.contains(&chunk_coords) {
            //create a chunk
            let chunk = Chunk::new();

            //add it to loaded chunks
            self.loaded_chunks.push(chunk_coords);

            //make its mesh
            chunks.0.insert(chunk_coords, chunk.clone());
            let new_chunk = chunk.build_mesh(commands, meshes, materials, chunk_coords);
            self.chunk_entities.insert(chunk_coords, new_chunk);
            /*println!(
                "Loaded chunk at ({}, {}, {})",
                chunk_coords.x, chunk_coords.y, chunk_coords.z
            );*/
        }
    }

    fn unload_chunk(&mut self, chunk_coords: IVec3, commands: &mut Commands) {
        //does the chunk have an entity component?
        //unload it if so throw an error if not
        if let Some(entity) = self.chunk_entities.get(&chunk_coords) {
            commands.entity(*entity).despawn_recursive();
            println!(
                "Unloaded chunk at ({}, {}, {})",
                chunk_coords.x, chunk_coords.y, chunk_coords.z
            );
        } else {
            println!("ERROR: chunk not unloaded for some reason");
            return;
        }

        //remove the chunk
        let loaded_index = self
            .loaded_chunks
            .iter()
            .position(|&r| r == chunk_coords)
            .unwrap();
        self.loaded_chunks.remove(loaded_index);
    }
}
