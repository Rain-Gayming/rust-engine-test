use bevy::{prelude::*, render::render_resource::ShaderType};

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
    fn new(player_position: IVec3) -> ChunkLoader {
        ChunkLoader {
            player_position,
            loaded_chunks: vec![],
            chunk_entities: HashMap::new(),
        }
    }

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

            //load the chunks around the new chunk
            let chunks_to_load = self.get_chunks_to_load(new_position, view_distance);
            for chunk_coords in chunks_to_load {
                self.load_chunk(chunk_coords, chunks, commands, materials, meshes);
            }

            //update player position reference
            self.player_position = new_position;
        }
    }

    fn get_chunks_to_load(&self, position: IVec3, view_distance: i32) -> Vec<IVec3> {
        let mut chunks_to_load = vec![];
        //x - view dist + x + view dist gets all the chunks around the player

        for x in position.x - view_distance..=position.x + view_distance {
            for y in position.y - view_distance..=position.y + view_distance {
                for z in position.z - view_distance..=position.z + view_distance {
                    let chunk_coords = IVec3::new(x, y, z);
                    if !self.loaded_chunks.contains(&chunk_coords) {
                        chunks_to_load.push(chunk_coords);
                        //println!("loading {}", chunk_coords);
                    }
                }
            }
        }
        chunks_to_load
    }

    fn get_chunks_to_unload(&self, position: IVec3, view_distance: i32) -> Vec<IVec3> {
        let mut chunks_to_unload = vec![];
        println!("{}", self.loaded_chunks.size());
        for chunk_coords in &self.loaded_chunks {
            println!("a chunk is loaded at: {}", chunk_coords);
            let distance = (chunk_coords.x - position.x).abs()
                + (chunk_coords.y - position.y).abs()
                + (chunk_coords.z - position.z).abs();
            if distance > view_distance + 1 {
                chunks_to_unload.push(*chunk_coords);
                println!("unloading {}", chunk_coords);
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
        let chunk = Chunk::new();
        chunk.build_mesh(commands, meshes, materials, chunks, chunk_coords);
        /*println!(
            "Loaded chunk at ({}, {}, {})",
            chunk_coords.x, chunk_coords.y, chunk_coords.z
        );*/
        let chunks_to_unload = self.get_chunks_to_unload(old_chunk_coords, view_distance);
        for chunk_coords in chunks_to_unload {
            println!("chunk found unload");
            self.unload_chunk(chunk_coords, commands);
        }
    }

    fn unload_chunk(&mut self, chunk_coords: IVec3, commands: &mut Commands) {
        let index = self.loaded_chunks.iter().position(|&c| c == chunk_coords);
        if let Some(entity) = self.chunk_entities.remove(&chunk_coords) {
            commands.entity(entity).despawn_recursive();
        }
        if let Some(i) = index {
            self.loaded_chunks.remove(i);
            println!(
                "Unloaded chunk at ({}, {}, {})",
                chunk_coords.x, chunk_coords.y, chunk_coords.z
            );
        }
    }
}
