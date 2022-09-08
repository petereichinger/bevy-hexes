use bevy::prelude::{Quat, Vec2, Vec3};

use super::submesh::{SubMesh, Triangle, Vertex};

fn get_hex_point(n: u8, size: f32) -> Vec3 {
    let degree = (60.0 * (n as f32)) - 30.0;
    let radians = degree.to_radians();
    size * Vec3::new(radians.cos(), 0.0, radians.sin())
}

fn get_hex_side_normal(n: u8) -> Vec3 {
    let degree = 60.0 * (n as f32);
    let radians = degree.to_radians();
    Vec3::new(radians.cos(), 0.0, radians.sin())
}

pub fn create_hex(size: f32) -> SubMesh {
    let positions = [
        Vec3::new(0.0, 0.0, 0.0),
        get_hex_point(0, size),
        get_hex_point(1, size),
        get_hex_point(2, size),
        get_hex_point(3, size),
        get_hex_point(4, size),
        get_hex_point(5, size),
    ];

    let vertices = positions
        .into_iter()
        .map(|pos| Vertex {
            position: pos,
            normal: Vec3::Y,
            uv: Vec2::ZERO,
        })
        .collect();

    let triangles = vec![
        Triangle { indices: [0, 2, 1] },
        Triangle { indices: [0, 3, 2] },
        Triangle { indices: [0, 4, 3] },
        Triangle { indices: [0, 5, 4] },
        Triangle { indices: [0, 6, 5] },
        Triangle { indices: [0, 1, 6] },
    ];

    SubMesh::new(vertices, triangles).unwrap()
}

fn create_hex_prism_side(n1: u8, n2: u8, size: f32, height: f32) -> [Vertex; 4] {
    let side_n1 = get_hex_point(n1, size);
    let side_n2 = get_hex_point(n2, size);
    let normal = get_hex_side_normal(n1);

    let v1 = Vertex {
        position: side_n1,
        normal,
        uv: Vec2::ZERO,
    };
    let v2 = Vertex {
        position: side_n1 + (height * Vec3::Y),
        normal,
        uv: Vec2::ZERO,
    };
    let v3 = Vertex {
        position: side_n2,
        normal,
        uv: Vec2::ZERO,
    };
    let v4 = Vertex {
        position: side_n2 + (height * Vec3::Y),
        normal,
        uv: Vec2::ZERO,
    };

    return [v1, v2, v3, v4];
}

pub fn create_hex_prism_sides(size: f32, height: f32) -> SubMesh {
    let mut vertices = vec![];
    let mut triangles = vec![];

    let tri_indices = [[0u32, 3, 2], [0, 1, 3]];

    for i in 0..6 {
        vertices.extend_from_slice(&create_hex_prism_side(i, i + 1u8, size, height));
        let tri_offset = 4 * i as u32;
        tri_indices
            .iter()
            .map(|[a, b, c]| Triangle::new(*a + tri_offset, *b + tri_offset, *c + tri_offset))
            .for_each(|t| triangles.push(t))
    }

    return SubMesh::new(vertices, triangles).unwrap();
}

pub fn create_hex_prism(size: f32, height: f32) -> SubMesh {
    let front_hex = create_hex(size).translate(Vec3::Y * height).unwrap();
    let back_hex = create_hex(size)
        .rotate(Quat::from_rotation_x(180.0_f32.to_radians()))
        .unwrap();
    let double_hex = front_hex.merge(back_hex);

    let prism_sides = create_hex_prism_sides(size, height);

    double_hex.merge(prism_sides)
}
