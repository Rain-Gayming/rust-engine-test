use crate::world::chunk_mesh_builder::ChunkMeshBuilder;
use rand::prelude::*;

use bevy::{pbr::wireframe::NoWireframe, prelude::*, utils::HashMap};
//contains chunk informatiom ( position, voxels, ect )

use super::{noise::NoiseGenerator, rendering_constants::*, voxel::Voxel, world::ChunkMap};

#[derive(Clone)]
pub struct Chunk {
    mesh_builder: ChunkMeshBuilder,
    voxels_in_chunk: HashMap<[u8; 3], Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            mesh_builder: ChunkMeshBuilder::new(),
            voxels_in_chunk: HashMap::new(),
        }
    }
    pub fn build_mesh(
        mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        position: IVec3,
        chunks: &mut ChunkMap,
    ) -> Entity {
        let seed = rand::thread_rng().gen_range(0..100);
        let noise_generator = NoiseGenerator::new(seed);

        //adds the voxels to the hashmap
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let mut is_solid = false;
                    let new_voxel_pos: [u8; 3];
                    new_voxel_pos = [x, y, z];

                    let voxel = Voxel::new(is_solid, new_voxel_pos);

                    self.voxels_in_chunk.insert([x, y, z], voxel);
                }
            }
        }

        let front_chunk = IVec3::new(position.x, position.y, position.z - 1);
        let back_chunk = IVec3::new(position.x, position.y, position.z + 1);
        let top_chunk = IVec3::new(position.x, position.y + 1, position.z);
        let bottom_chunk = IVec3::new(position.x, position.y - 1, position.z);
        let left_chunk = IVec3::new(position.x - 1, position.y, position.z);
        let right_chunk = IVec3::new(position.x + 1, position.y, position.z);

        //actually makes their mesh
        for voxel in self.voxels_in_chunk.iter() {
            let voxel_position = voxel.0;

            /*let front_voxel = [voxel_position[0] + 1, voxel_position[1], voxel_position[2]];
            let back_voxel = [voxel_position[0] - 1, voxel_position[1], voxel_position[2]];
            let top_voxel = [voxel_position[0], voxel_position[1] + 1, voxel_position[2]];
            let bottom_voxel = [voxel_position[0], voxel_position[1] - 1, voxel_position[2]];
            let left_voxel = [voxel_position[0], voxel_position[1], voxel_position[2] - 1];
            let right_voxel = [voxel_position[0], voxel_position[1], voxel_position[2] + 1];*/
            //left face
            if voxel_position[0] == 0
                || !self
                    .voxels_in_chunk
                    .get(&[voxel_position[0] - 1, voxel_position[1], voxel_position[2]])
                    .is_some()
            {
                self.mesh_builder.add_face(*voxel_position, 2);
            }

            //right face
            if voxel_position[0] == CHUNK_SIZE - 1
                || !self
                    .voxels_in_chunk
                    .get(&[voxel_position[0] + 1, voxel_position[1], voxel_position[2]])
                    .is_some()
            {
                self.mesh_builder.add_face(*voxel_position, 3);
            }

            //bottom face
            if voxel_position[1] == 0
                || !self
                    .voxels_in_chunk
                    .get(&[voxel_position[0], voxel_position[1] - 1, voxel_position[2]])
                    .is_some()
            {
                self.mesh_builder.add_face(*voxel_position, 5);
            }

            //top faces
            if voxel_position[1] == CHUNK_SIZE - 1
                || !self
                    .voxels_in_chunk
                    .get(&[voxel_position[0], voxel_position[1] + 1, voxel_position[2]])
                    .is_some()
            {
                self.mesh_builder.add_face(*voxel_position, 0);
            }

            //front chunk
            if voxel_position[2] == 0
                || !self
                    .voxels_in_chunk
                    .get(&[voxel_position[0], voxel_position[1], voxel_position[2] - 1])
                    .is_some()
            {
                self.mesh_builder.add_face(*voxel_position, 1);
            }

            //back chunk
            if voxel_position[2] == CHUNK_SIZE - 1
                || !self
                    .voxels_in_chunk
                    .get(&[voxel_position[0], voxel_position[1], voxel_position[2] + 1])
                    .is_some()
            {
                self.mesh_builder.add_face(*voxel_position, 4);
            }
        }

        let chunk_mesh_handle: Handle<Mesh> = meshes.add(self.mesh_builder.build());

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
}
