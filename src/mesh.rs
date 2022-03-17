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

@file    mesh.rs
@brief   Part of 3D model library
 */

//a Imports
use crate::Primitive;

//a Mesh
//tp Mesh
/// A [Mesh] provides an array of primitives, that is notionally drawn
/// from first to last
///
/// The [Mesh] depends on being in an 3D model object, as it is the
/// object that contains the actual materials and vertices to use
#[derive(Debug, Default)]
pub struct Mesh {
    /// The primitive
    pub primitives: Vec<Primitive>,
}

//ip Mesh
impl Mesh {
    //fp new
    /// Create a new mesh
    pub fn new() -> Self {
        let primitives = Vec::new();
        Self { primitives }
    }

    //mp add_primitive
    /// Add a primitive to the [Mesh]
    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }

    //zz All done
}
