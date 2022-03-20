//a Documentation
/*!

This provides a function to create [ExampleVertices] object that is a triangle of a specified size at z=0

 */

//a Imports
use super::ExampleVertices;
use crate::{BufferElementType, Renderable, Mesh, Primitive, VertexAttr, PrimitiveType};

/// Add positions, normals and indices to an [ExampleVertices] for a
/// flat upward-facing triangle on z=0 of a given size
pub fn new<'a, R: Renderable>(eg: &mut ExampleVertices<'a, R>, size: f32) {
    let vertex_data = [
        -size, -size, 0.0,
        size, -size, 0.0,
        0.0, size, 0.0,

        0.,0.,1.,
        0.,0.,1.,
        0.,0.,1.,
    ];
    let index_data = [0u8, 1, 2];

    let data_indices = eg.push_data(Box::pin(index_data));
    let data_vertices = eg.push_data(Box::pin(vertex_data));

    let indices = eg.push_view(data_indices, 3, BufferElementType::Int8, 0, 0);
    let vertices = eg.push_view(data_vertices, 3, BufferElementType::Float32, 0, 0);
    let normals = eg.push_view(data_vertices, 3, BufferElementType::Float32, 9*4, 0);

    // Create set of data (indices, vertex data) to by subset into by the meshes and their primitives
    eg.push_vertices(indices, vertices, &[(VertexAttr::Normal, normals)]);
}

/// Create a mesh for the triangle given the vertices index and
/// material index within a parent model3d::Object
///
/// The object should have had the vertices for the triangle (created
/// with new() above) added to it (using a parent [ExampleVertices])
pub fn mesh(v_id:usize, m_id:usize) -> Mesh {
    let mut mesh = Mesh::new();
    mesh.add_primitive(Primitive::new(PrimitiveType::Triangles, v_id, 0, 3, m_id));
    mesh
}
