use bevy::asset::RenderAssetUsages;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use super::rendering_constants::*;

#[derive(Default)]
pub struct ChunkMeshBuilder {
    pub vertices: Vec<[f32; 3]>,
    pub triangles: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub face_count: u32,
}

impl ChunkMeshBuilder {
    pub fn new() -> Self {
        return Self {
            vertices: Vec::with_capacity(131072),
            triangles: Vec::with_capacity(393216),
            normals: Vec::with_capacity(131072),
            uvs: Vec::with_capacity(131072),
            face_count: 0,
        };
    }

    fn add_vec3(mut base: [f32; 3], addition: [u8; 3]) -> [f32; 3] {
        for i in 0..3 {
            base[i] += addition[i] as f32;
        }
        base
    }
    pub fn add_face(&mut self, coord: [u8; 3], face_index: u8) {
        for i in &VERTICES[face_index as usize] {
            self.vertices.push(Self::add_vec3(*i, coord));
        }

        let mut arr = TRIANGLES.clone();
        self.triangles.extend_from_slice({
            for i in &mut arr {
                *i += 4 * self.face_count;
            }
            &arr
        });

        for _ in 0..4 {
            self.normals.push(NORMALS[face_index as usize]);
        }

        self.uvs.extend_from_slice(&UVS);
        self.face_count += 1;
    }

    pub fn build(self) -> Mesh {
        let mut msh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );

        msh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        msh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        msh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);

        msh.insert_indices(Indices::U32(self.triangles));

        msh
    }
}
