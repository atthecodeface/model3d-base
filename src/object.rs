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
use crate::hierarchy;
use crate::Renderable;
use crate::{Component, RenderRecipe, Material, Mesh, Skeleton, Transformation, Vertices, Instantiable};
use hierarchy::Hierarchy;

//a Object
//tp Object
/// A hierarchy of ObjectNode's
///
/// This can be flattened in to an Instantiable
pub struct Object<'a, R>
where
    R: Renderable,
{
    /// Skeleton
    pub skeleton: Option<Skeleton>,
    /// All the vertices used
    pub vertices: Vec<&'a Vertices<'a, R>>,
    /// All the materials used
    pub materials: Vec<&'a dyn Material<R>>,
    /// The meshes etc that make up the object
    pub components: Hierarchy<Component>,
    // The roots of the bones and hierarchical recipes for traversal
    // pub roots   : Vec<(usize, Recipe)>,
    // Meshes - indices in to nodes.nodes array of the meshes in the order of instance
    // pub meshes : Vec<usize>
}

//ip Object
impl<'a, R> Object<'a, R>
where
    R: Renderable,
{
    //fp new
    /// Create a new [Object] with no components
    pub fn new() -> Self {
        let skeleton = None;
        let vertices = Vec::new();
        let materials = Vec::new();
        let components = Hierarchy::new();
        Self {
            skeleton,
            vertices,
            materials,
            components,
        }
    }

    //mp add_vertices
    /// Add vertices to the object
    pub fn add_vertices(&mut self, vertices: &'a Vertices<'a, R>) -> usize {
        let n = self.vertices.len();
        self.vertices.push(vertices);
        n
    }

    //mp borrow_vertices
    /// Borrow one of the vertices
    pub fn borrow_vertices(&self, n: usize) -> &Vertices<'a, R> {
        self.vertices[n]
    }

    //fp add_material
    /// Add a material to the object
    pub fn add_material(&mut self, material: &'a dyn Material<R>) -> usize {
        let n = self.materials.len();
        self.materials.push(material);
        n
    }

    //mp borrow_material
    /// Borrow a materiaal from the object
    pub fn borrow_material(&self, n: usize) -> &dyn Material<R> {
        self.materials[n]
    }

    //fp add_component
    /// Add a component to the hierarchy
    pub fn add_component(
        &mut self,
        parent: Option<usize>,
        transformation: Option<Transformation>,
        mesh: Mesh,
    ) -> usize {
        let node = Component::new(transformation, mesh);
        let child = self.components.add_node(node);
        if let Some(parent) = parent {
            self.components.relate(parent, child);
        }
        child
    }

    //fp relate
    /// Add a relation between two components
    pub fn relate(&mut self, parent: usize, child: usize) {
        self.components.relate(parent, child);
    }

    //mp analyze
    /// Analyze 
    pub fn analyze(
        &mut self,
    )  {
        self.components.find_roots();
    }

    //mp create_render_recipe
    pub fn create_render_recipe(&self) -> RenderRecipe {
        RenderRecipe::from_component_hierarchy(&self.components)
    }

    //mp create_client
    /// Create the clients associated with the object - for vertices and materials
    pub fn create_client(&self, render_context: &mut R::Context) {
        for v in &self.vertices {
            v.create_client(render_context);
        }
    }

    //dp into_instantiable
    pub fn into_instantiable(self) -> Instantiable<R> {
        Instantiable::new(self.skeleton,
                          self.vertices,
                          self.materials,
                          self.components,
                          )
    }
    
    //zz All done
}
