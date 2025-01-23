use crate::world::chunk_mesh_builder::ChunkMeshBuilder;

use bevy::{pbr::wireframe::NoWireframe, prelude::*, utils::HashMap};
//contains chunk informatiom ( position, voxels, ect )

use super::{noise::NoiseGenerator, rendering_constants::*, voxel::Voxel, world::ChunkMap};

#[derive(Clone)]
pub struct Chunk {
    voxels_in_chunk: HashMap<[u8; 3], Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            voxels_in_chunk: HashMap::new(),
        }
    }
    pub fn build_mesh(
        mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        position: IVec3,
        _chunks: &mut ChunkMap,
        noise_generator: NoiseGenerator,
    ) -> Entity {
        let mut my_chunk_builder = ChunkMeshBuilder::new();

        //adds the voxels to the hashmap
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let world_pos =
                        Self::local_pos_to_world(position, Vec3::new(x as f32, y as f32, z as f32));
                    let mut is_solid = false;
                    let new_voxel_pos: [u8; 3];
                    if y < 30 {
                        new_voxel_pos = [x as u8, y as u8, z as u8];
                    } else {
                        let height_variation = noise_generator.get_height(
                            world_pos.x as f32,
                            world_pos.z as f32,
                            0.05,
                            7.,
                        );

                        let new_y: u8 = (10. + (height_variation as f32)).round() as u8;
                        new_voxel_pos = [x as u8, new_y, z as u8];
                        is_solid = true;
                    }
                    let voxel = Voxel::new(is_solid);

                    self.voxels_in_chunk.insert(new_voxel_pos, voxel);
                }
            }
        }

        //actually makes their mesh
        for voxel in self.voxels_in_chunk.iter() {
            let voxel_position = voxel.0;

            if voxel.1.is_solid {
                //left face
                if voxel_position[0] == 0
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0] - 1, voxel_position[1], voxel_position[2]])
                        .unwrap()
                        .is_solid
                {
                    my_chunk_builder.add_face(*voxel_position, 2);
                }

                //right face
                if voxel_position[0] == CHUNK_SIZE - 1
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0] + 1, voxel_position[1], voxel_position[2]])
                        .unwrap()
                        .is_solid
                {
                    my_chunk_builder.add_face(*voxel_position, 3);
                }

                //bottom face
                if voxel_position[1] == 0
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0], voxel_position[1] - 1, voxel_position[2]])
                        .unwrap()
                        .is_solid
                {
                    my_chunk_builder.add_face(*voxel_position, 5);
                }

                //top faces
                if voxel_position[1] == CHUNK_SIZE - 1
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0], voxel_position[1] + 1, voxel_position[2]])
                        .unwrap()
                        .is_solid
                {
                    my_chunk_builder.add_face(*voxel_position, 0);
                }

                //front chunk
                if voxel_position[2] == 0
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0], voxel_position[1], voxel_position[2] - 1])
                        .unwrap()
                        .is_solid
                {
                    my_chunk_builder.add_face(*voxel_position, 1);
                }

                //back chunk
                if voxel_position[2] == CHUNK_SIZE - 1
                    || !self
                        .voxels_in_chunk
                        .get(&[voxel_position[0], voxel_position[1], voxel_position[2] + 1])
                        .unwrap()
                        .is_solid
                {
                    my_chunk_builder.add_face(*voxel_position, 4);
                }
            }
        }

        let chunk_mesh_handle: Handle<Mesh> = meshes.add(my_chunk_builder.build());

        let id = commands
            .spawn((
                Mesh3d(chunk_mesh_handle),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgba(0.2, 0.7, 0.1, 1.0),
                    alpha_mode: AlphaMode::Mask(0.2),
                    unlit: false,
                    ..Default::default()
                })),
                Transform {
                    translation: Vec3::new(
                        (position.x * 32) as f32,
                        (position.y * 32) as f32,
                        (position.z * 32) as f32,
                    ),
                    ..default()
                },
                NoWireframe,
            ))
            .id();

        id
    }
    pub fn local_pos_to_world(offset: IVec3, local_pos: Vec3) -> Vec3 {
        Vec3::new(
            local_pos.x as f32 + (offset[0] as f32 * CHUNK_SIZE as f32),
            local_pos.y as f32 + (offset[1] as f32 * CHUNK_SIZE as f32),
            local_pos.z as f32 + (offset[2] as f32 * CHUNK_SIZE as f32),
        )
    }
}
