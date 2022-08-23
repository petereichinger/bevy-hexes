use bevy::prelude::{Mesh, Vec2, Vec3};
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
}
