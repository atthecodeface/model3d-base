use crate::BufferData;
use crate::{MaterialAspect, MaterialBaseData};

//a BufferClient
//tt BufferClient
/// Trait supported by a buffer client
pub trait BufferClient : Sized + std::fmt::Display + std::fmt::Debug {
    /// Create a none
    fn none() -> Self;
    /// Determine if a client (with a reason) is none; if reason is 0, then if any client is not none return true
    fn is_none(&self, reason:usize) -> bool;
    /// Create a client for a reason - reason 0 is reserved
    fn create(&mut self, data:&BufferData<Self>, reason:usize);
    /// Destroy a client given a reason - reason 0 implies all
    fn drop(&mut self, data:&BufferData<Self>, reason:usize);
}

//tt TextureClient
/// The trait that must be supported by a client texture
pub trait TextureClient : Sized + std::fmt::Debug {
}

//tt MaterialClient
/// Trait supported by a material client
pub trait MaterialClient<R:Renderable + ?Sized> : Sized + std::fmt::Display + std::fmt::Debug {
    /// Create a none
    fn none() -> Self;
    /// Determine if a client (with a reason) is none; if reason is 0, then if any client is not none return true
    fn is_none(&self, reason:usize) -> bool;
    /// Create a client for a reason - reason 0 is reserved
    fn create(&mut self, material:&dyn Material<R>, reason:usize);
    /// Destroy a client given a reason - reason 0 implies all
    fn drop(&mut self, material:&dyn Material<R>, reason:usize);
}

//tt VerticesClient
/// The trait that must be supported by a client texture
pub trait VerticesClient : Sized + std::fmt::Debug {
}


//tt Renderable
pub trait Renderable {
    type Buffer : BufferClient;
    type Texture : TextureClient;
    type Material : MaterialClient<Self>;
    type Vertices : VerticesClient;
}

//tt Material
/// A [Material] provides means to access the data for a material, be
/// it simple of full PBR. A fragment shader may require some aspects
/// of a material to be provided to it for rendering, and this API
/// allows that information to be gathered from any kind of material
pub trait Material<R:Renderable> {
    fn create_renderable(&self, reason:usize) {}
    fn destroy_renderable(&self, reason:usize) {}

    /// Borrow the basic data of a material - color and base
    /// metallic/roughness, for example
    fn borrow_base_data(&self) -> &MaterialBaseData;
    /// Borrow the texture ID associated with an aspect
    fn borrow_texture(&self, _aspect:MaterialAspect) -> Option<&R::Texture> {
        None
    }
}

