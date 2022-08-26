use bevy::prelude::{Mesh, Quat, Vec2, Vec3};
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub(super) position: Vec3,
    pub(super) normal: Vec3,
    pub(super) uv: Vec2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Triangle {
    pub(super) indices: [u32; 3],
}

impl Triangle {
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        Triangle { indices: [a, b, c] }
    }
}

impl From<&[u32; 3]> for Triangle {
    fn from(indices: &[u32; 3]) -> Self {
        return Triangle {
            indices: indices.clone(),
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SubMesh {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
}

impl SubMesh {
    pub fn new(vertices: Vec<Vertex>, triangles: Vec<Triangle>) -> Result<SubMesh, ()> {
        let indices_in_range = triangles
            .iter()
            .flat_map(|t| t.indices.iter())
            .all(|idx| (*idx as usize) < vertices.len());

        if !indices_in_range {
            return Err(());
        }

        Ok(SubMesh {
            vertices,
            triangles,
        })
    }

    #[inline(always)]
    fn modify_vertices<F>(vertices: &mut Vec<Vertex>, fun: F)
    where
        F: Fn(&Vertex) -> Vertex,
    {
        for vertex in vertices {
            *vertex = fun(vertex);
        }
    }

    pub fn rotate(mut self, rotation: Quat) -> Result<SubMesh, ()> {
        if rotation.is_nan() {
            return Err(());
        }

        if rotation.length_squared() < f32::EPSILON {
            return Err(());
        }

        let rotation = rotation.normalize();

        SubMesh::modify_vertices(&mut self.vertices, |v| Vertex {
            position: rotation * v.position,
            normal: rotation * v.normal,
            uv: v.uv,
        });

        Ok(self)
    }

    pub fn translate(mut self, translation: Vec3) -> Result<SubMesh, ()> {
        if translation.is_nan() {
            return Err(());
        }

        SubMesh::modify_vertices(&mut self.vertices, |v| Vertex {
            position: translation + v.position,
            normal: v.normal,
            uv: v.uv,
        });

        Ok(self)
    }

    pub fn merge(self, mut other: SubMesh) -> SubMesh {
        let mut vertices = self.vertices;
        let mut triangles = self.triangles;
        let vert_offset = vertices.len() as u32;

        vertices.reserve(other.vertices.len());
        triangles.reserve(other.triangles.len());

        vertices.append(&mut other.vertices);

        other
            .triangles
            .into_iter()
            .map(|t| Triangle {
                indices: t.indices.map(|i| i + vert_offset),
            })
            .for_each(|t| triangles.push(t));

        SubMesh {
            vertices,
            triangles,
        }
    }
}

impl From<SubMesh> for Mesh {
    fn from(x: SubMesh) -> Self {
        let (vertices, (normals, uvs)): (Vec<_>, (Vec<_>, Vec<_>)) = x
            .vertices
            .iter()
            .copied()
            .map(|v| (v.position, (v.normal, v.uv)))
            .unzip();

        let vertices: Vec<_> = vertices.into_iter().map(|v| [v.x, v.y, v.z]).collect();
        let normals: Vec<_> = normals.into_iter().map(|n| [n.x, n.y, n.z]).collect();
        let uvs: Vec<_> = uvs.into_iter().map(|v| [v.x, v.y]).collect();

        let indices = Indices::U32(
            x.triangles
                .iter()
                .flat_map(|t| t.indices.iter())
                .copied()
                .collect(),
        );

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        mesh
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn emtpy_is_ok() {
        let empty = SubMesh::new(vec![], vec![]);

        assert_ne!(empty, Err(()));
    }
    #[test]
    fn invalid_triangle_returns_err() {
        let invalid = SubMesh::new(vec![], vec![Triangle { indices: [0, 1, 2] }]);

        assert_eq!(invalid, Err(()));
    }

    #[test]
    fn simple_triangle_returns_ok() {
        let v1 = Vertex {
            position: Vec3::NEG_X,
            normal: Vec3::Z,
            uv: Vec2::ZERO,
        };
        let v2 = Vertex {
            position: Vec3::X,
            normal: Vec3::Z,
            uv: Vec2::X,
        };
        let v3 = Vertex {
            position: Vec3::Y,
            normal: Vec3::Z,
            uv: Vec2::ONE,
        };
        let triangle = SubMesh::new(vec![v1, v2, v3], vec![Triangle { indices: [0, 1, 2] }]);

        assert_ne!(triangle, Err(()));
    }
}
