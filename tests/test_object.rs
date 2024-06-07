use model3d_base::example_client::Renderable;
use model3d_base::{BufferAccessor, MaterialAspect};

// Create an object and interrogate it
// Create two distinct renderables
#[test]
fn test0() {
    // Create a triangle object with an empty skeleton
    let mut triangle = model3d_base::ExampleVertices::new();
    model3d_base::example_objects::triangle::new::<Renderable>(&mut triangle, 0.5);

    // Using the set of indices/vertex data defined create primitives (a triangle)
    let material = model3d_base::BaseMaterial::rgba((1., 0., 0., 1.));
    let mut obj: model3d_base::Object<Renderable> = model3d_base::Object::new();
    let v_id = obj.add_vertices(triangle.borrow_vertices(0));
    let m_id = obj.add_material(&material);
    let mesh = model3d_base::example_objects::triangle::mesh(v_id, m_id);
    obj.add_component(None, None, mesh);
    obj.analyze();
    let x = obj.borrow_vertices(v_id).borrow_client();
    let _p: &BufferAccessor<Renderable> = obj.borrow_vertices(v_id).borrow_indices();
    let _p: Option<&Renderable> = obj
        .borrow_material(m_id)
        .borrow_texture(MaterialAspect::Normal);

    drop(x); // so we can desconstruct obj
    let inst = obj.into_instantiable();
    let r = &inst.render_recipe;
    assert_eq!(r.matrices.len(), 1, "Expected only an identity matrix");
    assert_eq!(
        r.matrices[0],
        [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.],
        "Expected only an identity matrix"
    );
    assert_eq!(r.primitives.len(), 1, "Expected only one primitive");
    assert_eq!(
        r.matrix_for_primitives.len(),
        1,
        "Expected only one primitive"
    );
    assert_eq!(
        r.matrix_for_primitives[0], 0,
        "Expected primitive to use identity"
    );
    // Want to interrogate obj?
    // Create a model3::renderable (given a 'shader')
    // Creating a renderable
}
