use crate::{ByteBuffer, BufferClient, BufferElementType, VerticesClient, BufferData, BufferView, Vertices};
pub struct ExampleVertices<'a, V:VerticesClient, B:BufferClient> {
    buffers : Vec<Box<dyn ByteBuffer>>,
    data : Vec<BufferData<'a, B>>,
    views : Vec<BufferView<'a, B>>,
    vertices : Vec<Vertices<'a, V, B>>,
}
impl <'a, V:VerticesClient, B:BufferClient> ExampleVertices<'a, V, B> {
    pub fn new() -> Self {
        let buffers = Vec::new();
        let data = Vec::new();
        let views = Vec::new();
        let vertices = Vec::new();
        Self { buffers, data, views, vertices }
    }
    pub fn push_data(&mut self, buffer:Box<dyn ByteBuffer>) -> usize {
        let n = self.data.len();
        self.buffers.push(buffer);
        let b = &*(self.buffers[n-1]);
        let b = unsafe  {std::mem::transmute::<&dyn ByteBuffer, &'a dyn ByteBuffer>(b) };
        let data = BufferData::new(b, 0, 0);
        self.data.push(data);
        n
    }
    pub fn push_view(&mut self, data:usize, num:u32, et:BufferElementType, ofs:u32, length:u32) -> usize{
        let n = self.views.len();
        let d = unsafe  {std::mem::transmute::<&BufferData<'_, B>, &'a BufferData<'a, B>>(&self.data[data]) };
        let view = BufferView::new(d, num, et, ofs, length);
        self.views.push(view);
        n
    }
    pub fn push_vertices(&mut self, indices:usize, vertices:usize) -> usize {
        let n = self.vertices.len();
        let i = unsafe {std::mem::transmute::<&BufferView<'_, B>, &'a BufferView<'a, B>>(&self.views[indices]) };
        let v = unsafe {std::mem::transmute::<&BufferView<'_, B>, &'a BufferView<'a, B>>(&self.views[vertices]) };
        let vertices = Vertices::new(i, v);
        self.vertices.push(vertices);
        n
    }
}
pub mod triangle;
