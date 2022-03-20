//a Imports
use std::rc::Rc;

use crate::{BufferData, BufferClient, ViewClient, MaterialClient, TextureClient, VerticesClient, BufferView, Material, Renderable, VertexAttr, Vertices};

//a Buffer
//tp Buffer
/// A Buffer, which is used for both a [BufferData] and a BufferView client
///
/// This is a reference counted object - each [BufferData] has a
/// seperate one of these, and each [BufferView] clones it so that if
/// there are N views then (after deconstruction of the object) a
/// Buffer will have a strong count of the number of views upon it
#[derive(Debug, Clone)]
pub struct Buffer(Rc<u32>);

//ip Display for Buffer
impl std::fmt::Display for Buffer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

//ip Default for Buffer
impl Default for Buffer {
    fn default() -> Self {
        Self(Rc::new(0))
    }
}

//ip BufferClient for Buffer
impl BufferClient<Id> for Buffer {
    fn create( &mut self, _data: &BufferData<Id>, _render_context: &mut usize) {
        // No need to do anything; the 
    }
}

//ip ViewClient for Buffer
impl ViewClient<Id> for Buffer {
    fn create( &mut self, view: &BufferView<Id>, _attr:VertexAttr, render_context: &mut usize) {
        view.data.create_client(render_context);
        *self = view.data.borrow_client().clone();
    }
}

//a Id
//tp Id
/// The thing that is Renderable - pretty much a place-holder
///
/// This is also used as a MaterialClient, TextureClient and VerticesClient
#[derive(Debug, Clone)]
pub struct Id(u32);

//ip Display for Id
impl std::fmt::Display for Id {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

//ip Default for Id
impl Default for Id {
    fn default() -> Self {
        Self(0)
    }
}

//ip MaterialClient for Id
impl MaterialClient<Id> for Id {
    fn create(&mut self, _material: &dyn Material<Id>, _render_context: &mut usize) {}
    fn drop(&mut self, _material: &dyn Material<Id>, _render_context: &mut usize) {}
}

//ip TextureClient for Id
impl TextureClient for Id {}

//ip VerticesClient for Id
impl VerticesClient<Id> for Id {
    fn create(_vertices: &Vertices<Self>, _render_context: &mut usize) -> Self {
        Self::default()
    }
}

//ip Renderable for Id
impl Renderable for Id {
    type Context = usize;
    type Buffer = Buffer;
    type View = Buffer;
    type Texture = Id;
    type Material = Id;
    type Vertices = Id;
}

