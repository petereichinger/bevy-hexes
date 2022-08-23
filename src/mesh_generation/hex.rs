use bevy::prelude::{Vec2, Vec3};

use super::submesh::{SubMesh, Triangle, Vertex};

fn get_side_const<const N: u8>() -> Vec3 {
    let degree = (60.0 * (N as f32)) - 30.0;
    let radians = degree.to_radians();
    Vec3::new(radians.cos(), radians.sin(), 0.0)
}

pub fn create_hex() -> SubMesh {
    let poss = [
        Vec3::ZERO,
        get_side_const::<0>(),
        get_side_const::<1>(),
        get_side_const::<2>(),
        get_side_const::<3>(),
        get_side_const::<4>(),
        get_side_const::<5>(),
    ];

    let vertices = poss
        .into_iter()
        .map(|pos| Vertex {
            position: pos,
            normal: Vec3::Z,
            uv: Vec2::ZERO,
        })
        .collect();

    let triangles = [0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1]
        .chunks(3)
        .map(|c| Triangle {
            indices: [c[0], c[1], c[2]],
        })
        .collect();

    dbg!(&vertices);
    dbg!(&triangles);
    SubMesh::new(vertices, triangles).unwrap()
}
