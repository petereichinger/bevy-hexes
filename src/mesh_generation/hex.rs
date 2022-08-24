use bevy::prelude::{Vec2, Vec3};

use super::submesh::{SubMesh, Triangle, Vertex};

fn get_side_const<const N: u8>(size: f32) -> Vec3 {
    let degree = (60.0 * (N as f32)) - 30.0;
    let radians = degree.to_radians();
    size * Vec3::new(radians.cos(), radians.sin(), 0.0)
}

pub fn create_hex() -> SubMesh {
    create_hex_with_size(1.0)
}
pub fn create_hex_with_size(size: f32) -> SubMesh {
    let positions = [
        Vec3::ZERO,
        get_side_const::<0>(size),
        get_side_const::<1>(size),
        get_side_const::<2>(size),
        get_side_const::<3>(size),
        get_side_const::<4>(size),
        get_side_const::<5>(size),
    ];

    let vertices = positions
        .into_iter()
        .map(|pos| Vertex {
            position: pos,
            normal: Vec3::Z,
            uv: Vec2::ZERO,
        })
        .collect();

    let triangles = vec![
        Triangle { indices: [0, 1, 2] },
        Triangle { indices: [0, 2, 3] },
        Triangle { indices: [0, 3, 4] },
        Triangle { indices: [0, 4, 5] },
        Triangle { indices: [0, 5, 6] },
        Triangle { indices: [0, 6, 1] },
    ];

    SubMesh::new(vertices, triangles).unwrap()
}
