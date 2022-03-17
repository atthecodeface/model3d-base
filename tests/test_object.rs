use std::rc::Rc;

use model3d::{BufferData, BufferView, Material, MaterialAspect, Renderable, Vertices};

#[derive(Debug)]
struct Buffer(Rc<u32>);
impl std::fmt::Display for Buffer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self(Rc::new(0))
    }
}

impl model3d::BufferClient<Id> for Buffer {
    fn create( &mut self, data: &BufferData<Id>, _render_context: &mut usize) {
    }
}
impl model3d::ViewClient<Id> for Buffer {
    fn create( &mut self, view: &BufferView<Id>, _is_indices:bool, _render_context: &mut usize) {
    }
}

#[derive(Debug, Clone)]
struct Id(u32);
impl std::fmt::Display for Id {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}
impl Default for Id {
    fn default() -> Self {
        Self(0)
    }
}
impl model3d::MaterialClient<Id> for Id {
    fn create(&mut self, material: &dyn Material<Id>, _render_context: &mut usize) {}
    fn drop(&mut self, material: &dyn Material<Id>, _render_context: &mut usize) {}
}
impl model3d::TextureClient for Id {}
impl model3d::VerticesClient<Id> for Id {
    fn create(_vertices: &Vertices<Self>, _render_context: &mut usize) -> Self {
        Self::default()
    }
}
impl Renderable for Id {
    type Context = usize;
    type Buffer = Buffer;
    type View = Buffer;
    type Texture = Id;
    type Material = Id;
    type Vertices = Id;
}
use model3d::ExampleVertices;
use std::pin::Pin;

// Create an object and interrogate it
// Create two distinct renderables
#[test]
fn test0() {
    // Create a triangle object with an empty skeleton
    let mut triangle = model3d::ExampleVertices::new();
    model3d::example_objects::triangle::new::<Id>(&mut triangle, 0.5);

    // Using the set of indices/vertex data defined create primitives (a triangle)
    let material = model3d::BaseMaterial::rgba((1., 0., 0., 1.));
    let mut obj: model3d::Object<Id> = model3d::Object::new();
    let v_id = obj.add_vertices(triangle.borrow_vertices(0));
    let m_id = obj.add_material(&material);
    let mut mesh = model3d::Mesh::new();
    mesh.add_primitive(model3d::Primitive::new(
        model3d::PrimitiveType::Triangles,
        v_id,
        0,
        3,
        m_id,
    ));
    obj.add_component(None, None, mesh);
    obj.analyze();
    let x = obj.borrow_vertices(v_id).borrow_client();
    let p: &BufferView<Id> = obj.borrow_vertices(v_id).borrow_indices();
    let p: Option<&Id> = obj
        .borrow_material(m_id)
        .borrow_texture(MaterialAspect::Normal);

    let r = obj.create_render_recipe();
    assert_eq!(r.matrices.len(), 1, "Expected only an identity matrix");
    assert_eq!(r.matrices[0], [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.], "Expected only an identity matrix");
    assert_eq!(r.primitives.len(), 1, "Expected only one primitive");
    assert_eq!(r.matrix_for_primitives.len(), 1, "Expected only one primitive");
    assert_eq!(r.matrix_for_primitives[0], 0, "Expected primitive to use identity");
    // Want to interrogate obj?
    // Create a model3::renderable (given a 'shader')
    // Creating a renderable
}
