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

@file    primitive.rs
@brief   Part of 3D model library
 */

//a Imports
use crate::PrimitiveType;

//a Primitive
//tp Primitive
/// A primitive consisting of a material and a subset of
/// vertices using a particular range of indices
///
/// This might be, for example, the arm of a robot.
///
/// The [Primitive] depends on being in an 3D model object, as it is the
/// object that contains the actual materials and vertices to use
#[derive(Debug)]
pub struct Primitive {
    /// First index to use
    index_offset: u32,
    /// Number of indices to use
    index_count: u32,
    /// Material to be used in drawing - index within the [Object]
    material_index : u16,
    /// Vertices index within the [Object]
    ///
    /// This provides (effectively) the set of attribute `BufferView`s that the mesh utilizes
    vertices_index : u16,
    /// Type of the primitive
    primitive_type : PrimitiveType,
}

//ip Primitive
impl Primitive {
    //fp new
    /// Create a new Primitive from a Vertices
    pub fn new(primitive_type : PrimitiveType,
               vertices_index : usize,
               index_offset   : u32,
               index_count    : u32,
               material_index : usize
               ) -> Self {
        let index_offset = index_offset as u32;
        let index_count = index_count as u32;
        let material_index = material_index as u16;
        let vertices_index = vertices_index as u16;
        Self {
            index_offset,
            index_count,
            material_index,
            vertices_index,
            primitive_type,
        }
    }

    //mp vertices
    /// Retrieve the data for the vertices in the primitive
    ///
    /// This is the vertices index, the offset index, and the count
    #[inline]
    pub fn vertices(&self) -> (usize, u32, u32) {
        (self.vertices_index as usize, self.index_offset, self.index_count)
    }

    //mp material
    /// Retrieve the material for the primitive - this is the material index
    #[inline]
    pub fn material(&self) -> usize {
        self.material_index as usize
    }

    //mp primitive_type
    /// Retrieve the [PrimitiveType] of the primitive
    #[inline]
    pub fn primitive_type(&self) -> PrimitiveType {
        self.primitive_type
    }

    //zz All done
}
