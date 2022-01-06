use model3d::{BufferData, BufferView, MaterialAspect};


#[derive(Debug)]
struct Id (u32);
impl std::fmt::Display for Id {
    fn fmt(&self, fmt:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}
impl model3d::BufferClient for Id {
    fn none() -> Self { Self (0) }
    fn is_none(&self, _reason:usize) -> bool { self.0 == 0 }
    fn create(&mut self, data:&BufferData<Self>, _reason:usize) {}
    fn destroy(&mut self, data:&BufferData<Self>, _reason:usize) {}
}
impl model3d::TextureClient for Id {
}
impl model3d::VerticesClient for Id {
}

use std::pin::Pin;
use model3d::ExampleVertices;
#[test]
fn test0() {
    // Create triangle
    let mut triangle = model3d::ExampleVertices::new();
    model3d::example_objects::triangle::new(&mut triangle, 0.5);
    // Using the set of indices/vertex data defined create primitives (a triangle)
    let material = model3d::BaseMaterial::rgba((1.,0.,0.,1.));
    let mut obj = model3d::Object::new();
    let v_id = obj.add_vertices(triangle.borrow_vertices(0));
    let m_id = obj.add_material(&material);
    let mut mesh = model3d::Mesh::new();
    mesh.add_primitive( model3d::Primitive::new(model3d::PrimitiveType::Triangles, v_id, 0, 3, m_id) );
    obj.add_component( None, None, mesh);
    let x:&Option<Id>     = obj.borrow_vertices(v_id).borrow_client();
    let p:&BufferView<Id> = obj.borrow_vertices(v_id).borrow_indices();
    let p:Option<&Id> = obj.borrow_material(m_id).borrow_texture(MaterialAspect::Normal);
}
