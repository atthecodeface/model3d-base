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

@file    vertices.rs
@brief   Part of 3d Model library
 */

//a Notes
//
//

//a Imports
use crate::{BufferView, BufferClient};
use crate::{VerticesClient, VertexAttr};

//a Vertices
//tp Vertices
/// A set of vertices using one or more [BufferData] through [BufferView]s.
///
/// A number of [Vertices] is used by an `Object`, its components and their meshes; one is used for each primitive within a mesh for its elements.
/// The actual elements will be sets of triangles (as stripes or
/// whatever) which use these vertices.
///
/// A [Vertices] object includes a lot of options for vertices, and
/// different renderers (or different render stages) may require
/// different subsets of these indices. As such, in OpenGL for
/// example, a [Vertices] object may end up with more than one
/// `VAO`. This data is part of the [VerticesClient] struct
/// associated with the [Vertices]
///
/// When it comes to creating an instance of a mesh, that instance
/// will have specific transformations and materials for each of its
/// primitives; rendering the instance with a shader will require
/// enabling the [Vertices] client for that shader, setting
/// appropriate render options (uniforms in OpenGL)
#[derive(Debug)]
pub struct Vertices<'vertices, V:VerticesClient, B:BufferClient> {
    indices    : &'vertices BufferView<'vertices, B>,
    position   : &'vertices BufferView<'vertices, B>,
    client     : Option<V>,
    attrs      : Vec<(VertexAttr, &'vertices BufferView<'vertices, B>)>,
}

//ip Vertices
impl <'vertices, V:VerticesClient, B:BufferClient> Vertices<'vertices, V, B> {
    //fp new
    /// Create a new [Vertices] object with no additional attributes
    pub fn new(indices: &'vertices BufferView<'vertices, B>, position:&'vertices BufferView<'vertices, B>) -> Self {
        let attrs = Vec::new();
        let client = None;
        Self { indices,
               position,
               client,
               attrs
        }
    }

    //mp borrow_indices
    /// Borrow the indices [BufferView]
    pub fn borrow_indices<'a>(&'a self) -> &'a BufferView<'vertices, B> {
        self.indices
    }

    //mp borrow_position
    /// Borrow the position [BufferView]
    pub fn borrow_position<'a>(&'a self) -> &'a BufferView<'vertices, B> {
        self.position
    }

    //mp borrow_attr
    /// Borrow an attribute [BufferView] if the [Vertices] has one
    pub fn borrow_attr<'a>(&'a self, attr:VertexAttr) ->  Option<&'a BufferView<'vertices, B>> {
        for i in 0..self.attrs.len() {
            if self.attrs[i].0 == attr {
                return Some(self.attrs[i].1);
            }
        }
        None
    }

    //zz All done
}
