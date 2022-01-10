/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    object.rs
@brief   Part of 3D model library
 */

//a Imports
use crate::{Renderable, TextureClient, VerticesClient, BufferClient};
use crate::{Transformation, Skeleton, Material, Component, Mesh, Vertices};
use crate::hierarchy;
use hierarchy::{Hierarchy};

//a Object
/// A hierarchy of ObjectNode's
///
/// This can be flattened in to an Instantiable
pub struct Object<'a, R>
where R:Renderable
{
    /// Skeleton
    pub skeleton : Option<Skeleton>,
    /// All the vertices used
    pub vertices : Vec<&'a Vertices<'a, R::Vertices, R::Buffer>>,
    /// All the materials used
    pub materials : Vec<&'a dyn Material<R>>,
    /// The meshes etc that make up the object
    pub components   : Hierarchy<Component>,
    // The roots of the bones and hierarchical recipes for traversal
    // pub roots   : Vec<(usize, Recipe)>,
    // Meshes - indices in to nodes.nodes array of the meshes in the order of instance
    // pub meshes : Vec<usize>
}

//ip Object
impl <'a, R> Object<'a, R>
    where R:Renderable {
    //fp new
    /// Create a new [Object] with no components
    pub fn new() -> Self {
        let skeleton = None;
        let vertices = Vec::new();
        let materials = Vec::new();
        let components = Hierarchy::new();
        // let roots = Vec::new();
        // let meshes = Vec::new();
        Self { skeleton, vertices, materials, components }
    }

    //mp add_vertices
    pub fn add_vertices(&mut self, vertices:&'a Vertices<'a, R::Vertices, R::Buffer>) -> usize {
        let n = self.vertices.len();
        self.vertices.push(vertices);
        n
    }

    //mp borrow_vertices
    pub fn borrow_vertices(&self, n:usize) -> &Vertices<'a, R::Vertices, R::Buffer> {
        self.vertices[n]
    }

    //fp add_material
    pub fn add_material(&mut self, material:&'a dyn Material<R>) -> usize {
        let n = self.materials.len();
        self.materials.push(material);
        n
    }

    //mp borrow_material
    pub fn borrow_material(&self, n:usize) -> &dyn Material<R> {
        self.materials[n]
    }

    //fp add_component
    /// Add a component to the hierarchy
    pub fn add_component(&mut self,
                         parent : Option<usize>,
                         transformation : Option<Transformation>,
                         mesh : Mesh ) -> usize {
        let node = Component::new(transformation, mesh);
        let child = self.components.add_node(node);
        if let Some(parent) = parent {
            self.components.relate( parent, child);
        }
        child
    }

    //fp relate
    /// Add a relation between two components
    pub fn relate(&mut self, parent:usize, child:usize) {
        self.components.relate( parent, child);
    }

    /*
    pub fn add_meshes_of_node_iter(&self, meshes:&mut Vec<usize>, drawable:&mut drawable::Instantiable, iter:NodeIter<ObjectNode>) {
        let mut parent = None;
        let mut transformation = None;
        let mut bone_matrices = (0,0);
        let mut mesh_stack = Vec::new();
        for op in iter {
            match op {
                NodeIterOp::Push((n,obj_node), _has_children) => {
                    mesh_stack.push((parent, transformation, bone_matrices));
                    if let Some(bone_set) = obj_node.bones {
                        bone_matrices = drawable.add_bone_set(bone_set);
                    }
                    if let Some(obj_transformation) = &obj_node.transformation {
                        if transformation.is_none() {
                            transformation = Some(obj_transformation.mat4());
                        } else {
                            transformation = Some(matrix::multiply4(&transformation.unwrap(), &obj_transformation.mat4()));
                        }
                    }
                    if obj_node.mesh.is_some() {
                        let index = drawable.add_mesh(&parent, &transformation, &bone_matrices);
                        assert_eq!(index, meshes.len());
                        meshes.push(n);
                        parent = Some(index);
                        transformation = None;
                    }
                },
                NodeIterOp::Pop(_,_) => {
                    let ptb = mesh_stack.pop().unwrap();
                    parent = ptb.0;
                    transformation = ptb.1;
                    bone_matrices = ptb.2;
                },
            }
        }
    }

    pub fn create_instantiable(&mut self) -> drawable::Instantiable {
        self.nodes.find_roots();
        let mut drawable = drawable::Instantiable::new();
        let mut meshes = Vec::new();
        for r in self.nodes.borrow_roots() {
            self.add_meshes_of_node_iter(&mut meshes, &mut drawable, self.nodes.iter_from_root(*r));
        }
        self.meshes = meshes;
        drawable
    }
    pub fn bind_shader<'b, S:ShaderClass>(&self, drawable:&'b drawable::Instantiable, shader:&S) -> shader::Instantiable<'b> {
        let mut s = shader::Instantiable::new(drawable);
        for i in 0..self.meshes.len() {
            let obj_node = self.nodes.borrow_node(self.meshes[i]);
            assert!(obj_node.mesh.is_some(), "Mesh at node must be Some() for it to have been added to the self.meshes array by add_meshes_of_node_iter");
            let mesh = obj_node.mesh.unwrap();
            mesh.add_shader_drawables(shader, &mut s);
        }
        s
    }
*/
}

