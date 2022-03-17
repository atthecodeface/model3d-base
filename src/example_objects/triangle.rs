//a Documentation
/*!

This provides a function to create [ExampleVertices] object that is a triangle of a specified size at z=0

 */

//a Imports
use super::ExampleVertices;
use crate::{BufferElementType, Renderable};

/// Create a new [Vertices] object with a triangle at z=0.
pub fn new<'a, R: Renderable>(eg: &mut ExampleVertices<'a, R>, size: f32) {
    let vertex_data = [-size, -size, 0.0, size, -size, 0.0, 0.0, size, 0.0];
    let index_data = [0u8, 1, 2];

    let indices = eg.push_data(Box::pin(index_data));
    let vertices = eg.push_data(Box::pin(vertex_data));

    let indices = eg.push_view(indices, 3, BufferElementType::Int8, 0, 0);
    let vertices = eg.push_view(vertices, 3, BufferElementType::Float32, 0, 0);

    // Create set of data (indices, vertex data) to by subset into by the meshes and their primitives
    eg.push_vertices(indices, vertices);
}
