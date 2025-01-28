use crate::world::chunk_mesh_builder::ChunkMeshBuilder;

use bevy::{pbr::wireframe::NoWireframe, prelude::*, utils::HashMap};
//contains chunk informatiom ( position, voxels, ect )

use super::{
    biome::Biome, block::Block, noise::NoiseGenerator, rendering_constants::*, voxel::Voxel,
    world::ChunkMap,
};

#[derive(Clone)]
pub struct Chunk {
    pub voxels_in_chunk: Vec<Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        let mut new_voxels = Vec::new();
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    new_voxels.push(Voxel::new(false, Block::air()));
                }
            }
        }

        Chunk {
            voxels_in_chunk: new_voxels,
        }
    }

    pub fn get_visible(&mut self, position: IVec3) -> bool {
        let index = (position.x
            + position.x * CHUNK_SIZE as i32
            + position.z * CHUNK_SIZE as i32 * CHUNK_SIZE as i32) as usize;
        self.voxels_in_chunk[index].is_visible
    }

    pub fn set_voxel(&mut self, voxel_index: usize, is_visible: bool, block: Block) {
        self.voxels_in_chunk[voxel_index].is_visible = is_visible;
        self.voxels_in_chunk[voxel_index].block = block;
    }

    /*pub fn build_mesh(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        position: IVec3,
        chunks: HashMap<IVec3, Entity>,
        asset_server: &mut Res<AssetServer>,
    ) {
        let mut my_chunk_builder = ChunkMeshBuilder::new();

        println!("{}", self.voxels_in_chunk.len());

        //actually makes their mesh
        for voxel in self.voxels_in_chunk.iter() {
            let voxel_position = voxel.0;
            if voxel.1.is_visible {
                //left face
                if voxel_position[0] == 0
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0] - 1, voxel_position[1], voxel_position[2]])
                        .unwrap()
                        .is_visible
                {
                    my_chunk_builder.add_face(*voxel_position, 2, voxel.1.block.texture_pos_left);
                }

                //right face
                if voxel_position[0] == CHUNK_SIZE - 1
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0] + 1, voxel_position[1], voxel_position[2]])
                        .unwrap()
                        .is_visible
                {
                    my_chunk_builder.add_face(*voxel_position, 3, voxel.1.block.texture_pos_right);
                }

                //bottom face
                if voxel_position[1] == 0
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0], voxel_position[1] - 1, voxel_position[2]])
                        .unwrap()
                        .is_visible
                {
                    my_chunk_builder.add_face(*voxel_position, 5, voxel.1.block.texture_pos_bottom);
                }

                //top faces
                if voxel_position[1] == CHUNK_SIZE - 1
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0], voxel_position[1] + 1, voxel_position[2]])
                        .unwrap()
                        .is_visible
                {
                    my_chunk_builder.add_face(*voxel_position, 0, voxel.1.block.texture_pos_top);
                }

                //front chunk
                if voxel_position[2] == 0
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0], voxel_position[1], voxel_position[2] - 1])
                        .unwrap()
                        .is_visible
                {
                    my_chunk_builder.add_face(*voxel_position, 1, voxel.1.block.texture_pos_front);
                }

                //back chunk
                if voxel_position[2] == CHUNK_SIZE - 1
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0], voxel_position[1], voxel_position[2] + 1])
                        .unwrap()
                        .is_visible
                {
                    my_chunk_builder.add_face(*voxel_position, 4, voxel.1.block.texture_pos_back);
                }
            }
        }

        let chunk_mesh_handle: Handle<Mesh> = meshes.add(my_chunk_builder.build());
        let custom_texture_handle: Handle<Image> = asset_server.load("array_texture.png");
        if let Some(entity) = chunks.get(&position) {
            commands
                .entity(*entity)
                .insert((
                    Mesh3d(chunk_mesh_handle),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(custom_texture_handle),
                        alpha_mode: AlphaMode::Mask(0.2),
                        unlit: false,
                        ..Default::default()
                    })),
                    NoWireframe,
                ))
                .id();
        }
    }

    pub fn local_pos_to_world(offset: IVec3, local_pos: Vec3) -> Vec3 {
        Vec3::new(
            local_pos.x as f32 + (offset[0] as f32 * CHUNK_SIZE as f32),
            local_pos.y as f32 + (offset[1] as f32 * CHUNK_SIZE as f32),
            local_pos.z as f32 + (offset[2] as f32 * CHUNK_SIZE as f32),
        )
    }*/
}
