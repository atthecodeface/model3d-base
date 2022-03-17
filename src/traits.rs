use crate::{BufferData, BufferView, Vertices};
use crate::{MaterialAspect, MaterialBaseData};

//a BufferClient
//tt BufferClient
/// Trait supported by a BufferData client
///
/// A buffer client is created first by a buffer as 'none'
///
/// The data may be created more than once with the same buffer; the client
/// is responsible for deduplication within the render context if required
pub trait BufferClient<R: Renderable<Buffer = Self> + ?Sized>:
    Sized + std::fmt::Display + std::fmt::Debug + std::default::Default
{
    /// Create a client
    fn create(&mut self, data: &BufferData<R>, render_context: &mut R::Context);
}

//tt ViewClient
/// Trait supported by a BufferView client
///
/// A buffer client is created first by a buffer as 'none'
///
/// Before a view is creataed the data will be created at least once
///
/// The data may be created more than once with the same buffer; the client
/// is responsible for dedupliclation within the render context if required
pub trait ViewClient<R: Renderable<View = Self> + ?Sized>:
    Sized + std::fmt::Display + std::fmt::Debug + std::default::Default
{
    /// Create a client
    fn create(&mut self, view: &BufferView<R>, is_indices:bool, render_context: &mut R::Context);
}

//tt TextureClient
/// The trait that must be supported by a client texture
pub trait TextureClient: Sized + std::fmt::Debug {}

//tt MaterialClient
/// Trait supported by a material client
pub trait MaterialClient<R: Renderable + ?Sized>:
    Sized + std::fmt::Display + std::fmt::Debug + std::default::Default
{
    /// Create a client for a reason - reason 0 is reserved
    fn create(&mut self, material: &dyn Material<R>, render_context: &mut R::Context);
    /// Destroy a client given a reason - reason 0 implies all
    fn drop(&mut self, material: &dyn Material<R>, render_context: &mut R::Context);
}

//tt VerticesClient
/// The trait that must be supported by a client vertices
///
/// Clone is required as Vertices can be borrowed by more than one object, and an
/// instantiable object contains the [VerticesClient] for the Vertices
/// 
pub trait VerticesClient<R: Renderable<Vertices = Self> + ?Sized>:
Sized + std::fmt::Debug + std::default::Default + Clone
{
    /// Create a client
    fn create(vertices: &Vertices<R>, render_context: &mut R::Context) -> Self;
}

//tt Renderable
/// The [Renderable] trait must be implemented by a type that is a
/// client of the 3D model system. It provides associated types for a
/// renderable context (this might be a particular shader program
/// within a OpenGL context, for example), and then its own structures
/// that are used to hold [BufferData], textures, materials, and sets
/// of renderable [Vertices].
pub trait Renderable {
    /// A context that is used in any call to a 'create' client method
    ///
    /// For renderers that may need to render the same object instance
    /// with more than one render pipeline - and for each to have a
    /// distinct [VerticesClient] entry - the context should be able
    /// to contain an indicator (used during object.make_renderable)
    /// of which render pipeline is being created
    type Context;
    /// The renderer's type that reflects a [BufferData]
    type Buffer: BufferClient<Self>;
    /// The renderer's type that reflects a [BufferView]
    type View: ViewClient<Self>;
    /// The renderer's type that represents a texture; this is
    /// supplied to material creation, and hence is less a product of
    /// the renderer and more an input to the 3D model library
    type Texture: TextureClient;
    /// The renderer's type that reflects a [Material]; this is expected
    /// to be an extraction of the aspects of a material that the
    /// renderer pipelines can apply.
    type Material: MaterialClient<Self>;
    /// The renderer's type that reflects a [BufferView] of indices
    /// and the associated [BufferView]s of attributes supported by a
    /// particular pipeline within the renderer
    type Vertices: VerticesClient<Self>;
    // type Instantiable : ;
}

//tt Material
/// A [Material] provides means to access the data for a material, be
/// it simple of full PBR. A fragment shader may require some aspects
/// of a material to be provided to it for rendering, and this API
/// allows that information to be gathered from any kind of material
pub trait Material<R: Renderable> {
    /// Invoked when an 3D model object is made renderable
    fn create_renderable(&self, _render_context: &mut R::Context) {}

    /// Borrow the basic data of a material - color and base
    /// metallic/roughness, for example
    fn borrow_base_data(&self) -> &MaterialBaseData;
    /// Borrow the texture ID associated with an aspect
    fn borrow_texture(&self, _aspect: MaterialAspect) -> Option<&R::Texture> {
        None
    }
}
