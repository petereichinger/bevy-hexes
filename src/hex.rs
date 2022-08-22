use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        render_resource::{Extent3d, PrimitiveTopology, TextureDimension, TextureFormat},
        texture::ImageSettings,
    },
};
pub struct Hex {
    size: f32,
}

impl Default for Hex {
    fn default() -> Self {
        Hex { size: 1.0 }
    }
}

impl Hex {
    pub fn get_side(n: u8) -> Vec3 {
        
        let n = n % 6u8;
        let degree = 30.0 + 60.0 * n as f32;
        let radians = degree.to_radians();
        Vec3::new(degree.cos(), degree.sin(), 0.0)
    }
}

impl From<Hex> for Mesh {
    fn from(sp: Hex) -> Self {
        let vertices = [
            Vec3::ZERO,
            Hex::get_side(0),
            Hex::get_side(1),
            Hex::get_side(2),
            Hex::get_side(3),
            Hex::get_side(4),
            Hex::get_side(5),
        ];

        info!("{:?}",&vertices);

        let vertices: Vec<[f32; 3]> = vertices.into_iter().map(|v| [v.x, v.y, v.z]).collect();

        let normals : Vec<[f32; 3]> = (0..vertices.len()).into_iter().map(|_n| [0.0, 0.0, 1.0]).collect();

        let uvs : Vec<[f32; 2]> = (0..vertices.len()).into_iter().map(|_n| [0.0, 0.0]).collect();

        let indices = Indices::U32(vec![
            0, 1,2,
            0,2, 1  ]);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        mesh
    }
}
