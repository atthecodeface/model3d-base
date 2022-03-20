use std::rc::Rc;

use crate::{BufferData, BufferClient, ViewClient, MaterialClient, TextureClient, VerticesClient, BufferView, Material, MaterialAspect, Renderable, VertexAttr, Vertices};

#[derive(Debug)]
pub struct Buffer(Rc<u32>);
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

impl BufferClient<Id> for Buffer {
    fn create( &mut self, _data: &BufferData<Id>, _render_context: &mut usize) {
    }
}
impl ViewClient<Id> for Buffer {
    fn create( &mut self, _view: &BufferView<Id>, _attr:VertexAttr, _render_context: &mut usize) {
    }
}

#[derive(Debug, Clone)]
pub struct Id(u32);
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
impl MaterialClient<Id> for Id {
    fn create(&mut self, _material: &dyn Material<Id>, _render_context: &mut usize) {}
    fn drop(&mut self, _material: &dyn Material<Id>, _render_context: &mut usize) {}
}
impl TextureClient for Id {}
impl VerticesClient<Id> for Id {
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

