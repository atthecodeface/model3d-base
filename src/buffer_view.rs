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

@file    bezier.rs
@brief   Part of geometry library
 */

//a Notes
//
//

//a Imports
use crate::{BufferElementType, BufferData, BufferClient};

//a BufferView
/// A subset of a `BufferData`, used for vertex attributes;
/// hence for use in a vertex attribute pointer.
///
/// A `BufferView` is used for a single attribute of a set of data, such as
/// Position or Normal.
#[derive(Debug)]
pub struct BufferView<'a, T:BufferClient> {
    /// The `BufferData` that contains the actual vertex attribute data
    pub data: &'a BufferData<'a, T>,
    /// Number of elements per vertex - 1 to 4
    pub count: u32,
    /// The type of each element
    pub ele_type : BufferElementType,
    /// Offset from start of buffer to first byte of data
    pub offset : u32,
    /// Stride of data in the buffer - 0 for count*sizeof(ele_type)
    pub stride : u32,
}

//ip BufferView
impl<'a, T:BufferClient> BufferView<'a, T> {
    //fp new
    /// Create a new view of a `BufferData`
    pub fn new(data:&'a BufferData<'a, T>, count:u32, ele_type:BufferElementType, offset:u32, stride:u32) -> Self {
        Self { data, count, ele_type, offset, stride }
    }

    //mp create_client
    /// Create the OpenGL buffer required by the BufferView
    pub fn create_client(&self) {
        self.data.create_client(0)
    }

    //zz All done
}

//ip Display for BufferView
impl <'a, T:BufferClient> std::fmt::Display for BufferView<'a, T> {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f,"BufferView[{:?}#{}]\n  {}+{}+n*{}\n", self.ele_type, self.count, self.data, self.offset,self.stride)
    }
}

//ip DefaultIndentedDisplay for BufferView
impl <'a, T:BufferClient> indent_display::DefaultIndentedDisplay for BufferView<'a, T> {}
