use bevy::{
    prelude::*,
    render::{
        render_resource::PrimitiveTopology,
        mesh::Indices
    }
};
use crate::board::Position;

pub struct QuadMesh {
    verts: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    tris: Vec<u32>,
    quad_idx: u32,
    z: f32
}

impl QuadMesh {
    pub fn new(quad_z: f32) -> QuadMesh {
        QuadMesh {
            verts: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            tris: Vec::new(),
            quad_idx: 0,
            z: quad_z
        }
    }
    pub fn add_quad(&mut self, position: &Position, uv: (u8, u8)) {
        self.verts.push([position.v.x as f32, position.v.y as f32, self.z]);
        self.verts.push([position.v.x as f32, position.v.y as f32 + 1.0, self.z]);
        self.verts.push([position.v.x as f32 + 1.0, position.v.y as f32 + 1.0, self.z]);
        self.verts.push([position.v.x as f32 + 1.0, position.v.y as f32, self.z]);
    
        for _ in 0..4 {
            self.normals.push([0.0, 1.0, 0.0]);
        }
    
        self.uvs.extend(super::sprites::atlas_uvs(uv.0, uv.1, 8));
        let idx = 4 * self.quad_idx;
        self.tris.extend([idx, idx + 2, idx + 1, idx, idx + 3, idx + 2]);
        self.quad_idx += 1;
    }
    pub fn to_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.verts.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());

        mesh.set_indices(Some(Indices::U32(self.tris.clone())));
        mesh
    }
}