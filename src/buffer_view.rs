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
use std::cell::RefCell;

use crate::{ViewClient, BufferData, BufferElementType, Renderable};

//a BufferView
//tp BufferView
/// A subset of a `BufferData`, used for vertex attributes;
/// hence for use in a vertex attribute pointer.
///
/// A `BufferView` is used for a single attribute of a set of data, such as
/// Position or Normal.
#[derive(Debug)]
pub struct BufferView<'a, R: Renderable + ?Sized> {
    /// The `BufferData` that contains the actual vertex attribute data
    pub data: &'a BufferData<'a, R>,
    /// For attributes: number of elements per vertex (1 to 4)
    /// For indices: number of indices in the buffer
    pub count: u32,
    /// The type of each element
    ///
    /// For indices this must be Int8, Int16 or Int32
    pub ele_type: BufferElementType,
    /// Offset from start of buffer to first byte of data
    pub byte_offset: u32,
    /// Stride of data in the buffer - 0 for count*sizeof(ele_type)
    /// Unused for indices
    pub stride: u32,
    /// The client bound to data[byte_offset] .. + byte_length
    ///
    /// This must be held as a [RefCell] as the [BufferData] is
    /// created early in the process, prior to any `BufferView`s using
    /// it - which then have shared references to the daata - but the
    /// client is created afterwards
    rc_client: RefCell<R::View>,
}

//ip BufferView
impl<'a, R: Renderable> BufferView<'a, R> {
    //fp new
    /// Create a new view of a `BufferData`
    pub fn new(
        data: &'a BufferData<'a, R>,
        count: u32, // count is number of ele_type in an attribute
        ele_type: BufferElementType,
        byte_offset: u32, // offset in bytes?
        stride: u32, // stride between elements (0->count*sizeof(ele_type))
    ) -> Self {
        let rc_client = RefCell::new(R::View::default());
        Self {
            data,
            count,
            ele_type,
            byte_offset,
            stride,
            rc_client,
        }
    }

    //mp create_client
    /// Create the render buffer required by the BufferView
    pub fn create_client(&self, is_indices:bool, render_context: &mut R::Context) {
        self.rc_client.borrow_mut().create(self, is_indices, render_context);
    }

    //ap borrow_client
    /// Borrow the client
    pub fn borrow_client(&self) -> std::cell::Ref<R::View> {
        self.rc_client.borrow()
    }

    //zz All done
}

//ip Display for BufferView
impl<'a, R: Renderable> std::fmt::Display for BufferView<'a, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "BufferView[{:?}#{}]\n  {}+{}+n*{}\n",
            self.ele_type, self.count, self.data, self.byte_offset, self.stride
        )
    }
}

//ip DefaultIndentedDisplay for BufferView
impl<'a, R: Renderable> indent_display::DefaultIndentedDisplay for BufferView<'a, R> {}
