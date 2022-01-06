use crate::{BufferClient, BufferData, BufferView, VerticesClient, Vertices, BufferElementType};
use super::ExampleVertices;

/// Create a new [Vertices] object with a triangle at z=0.
pub fn new<'a, V:VerticesClient, B:BufferClient> (eg:&mut ExampleVertices<'a, V, B>, size:f32)  {
    let vertex_data = [-size,-size,0.0, size,-size,0.0, 0.0,size, 0.0];
    let index_data = [0u8,1,2];

    let indices = eg.push_data(Box::pin(index_data));
    let vertices = eg.push_data(Box::pin(vertex_data));

    let indices = eg.push_view(indices, 3, BufferElementType::Int8, 0, 0);
    let vertices = eg.push_view(vertices, 3, BufferElementType::Float32, 0, 0);

    // Create set of data (indices, vertex data) to by subset into by the meshes and their primitives
    eg.push_vertices(indices, vertices);
}
   

