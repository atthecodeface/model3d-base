//a Imports
use std::pin::Pin;
// use std::ops::Deref;

use crate::{
    BufferData, BufferElementType, BufferView, ByteBuffer, Renderable, VertexAttr, Vertices,
};

//a ExampleVertices
//tp ExampleVertices
/// This structure provides for creating example objects, particularly with regard to their vertices
///
/// It uses arrays of [Pin]ned data structures so that the data can be safely self-referential
pub struct ExampleVertices<'a, R: Renderable> {
    buffers: Vec<Pin<Box<dyn ByteBuffer>>>,
    data: Vec<Pin<Box<BufferData<'a, R>>>>,
    views: Vec<Pin<Box<BufferView<'a, R>>>>,
    vertices: Vec<Vertices<'a, R>>,
}

//ip ExampleVertices
impl<'a, R: Renderable> ExampleVertices<'a, R> {
    //fp new
    /// Create a new [ExampleVertices]
    ///
    /// This should probably not be Pin<Box<>>
    pub fn new() -> Self {
        let buffers = Vec::new();
        let data = Vec::new();
        let views = Vec::new();
        let vertices = Vec::new();
        Self {
            buffers,
            data,
            views,
            vertices,
        }
    }

    //fp push_data
    /// Push a new [ByteBuffer] implementation and return its index
    pub fn push_data(&mut self, buffer: Pin<Box<dyn ByteBuffer>>) -> usize {
        let n = self.data.len();
        self.buffers.push(buffer);
        let b = &*(self.buffers[n]);
        // let b = self.buffers[n-1].deref();
        let b = unsafe { std::mem::transmute::<&dyn ByteBuffer, &'a dyn ByteBuffer>(b) };
        let data = Box::pin(BufferData::new(b, 0, 0));
        self.data.push(data);
        n
    }

    //fp push_view
    /// Create a new [BufferView] on a particular [ByteBuffer] instance that has already been pushed
    pub fn push_view(
        &mut self,
        data: usize,
        num: u32,
        et: BufferElementType,
        ofs: u32,
        stride: u32,
    ) -> usize {
        let n = self.views.len();
        let d = unsafe {
            std::mem::transmute::<&BufferData<'_, R>, &'a BufferData<'a, R>>(&self.data[data])
        };
        let view = Box::pin(BufferView::new(d, num, et, ofs, stride));
        self.views.push(view);
        n
    }

    //fp push_vertices
    /// Create a new [Vertices] using a set of indices and positions
    ///
    /// This extends the life of the BufferView to that of the ExampleVertices
    ///
    /// This is safe as the BufferView's are in the Vec for ExampleVertices
    pub fn push_vertices(
        &mut self,
        indices: usize,
        positions: usize,
        attrs: &[(VertexAttr, usize)],
    ) -> usize {
        let n = self.vertices.len();
        let i = unsafe {
            std::mem::transmute::<&BufferView<'_, R>, &'a BufferView<'a, R>>(&self.views[indices])
        };
        let v = unsafe {
            std::mem::transmute::<&BufferView<'_, R>, &'a BufferView<'a, R>>(&self.views[positions])
        };
        let mut vertices = Vertices::new(i, v);
        for (attr, view_id) in attrs {
            let v = unsafe {
                std::mem::transmute::<&BufferView<'_, R>, &'a BufferView<'a, R>>(
                    &self.views[*view_id],
                )
            };
            vertices.add_attr(*attr, v);
        }
        self.vertices.push(vertices);
        n
    }

    //fp borrow_vertices
    /// Borrow a set of vertices; this would allow (if mut!) the vertices to have attributes added
    pub fn borrow_vertices(&self, vertices: usize) -> &Vertices<R> {
        &self.vertices[vertices]
    }
}
