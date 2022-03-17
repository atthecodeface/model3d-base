//a Basic types
//tp Vec3
/// 3-dimensional vector
pub type Vec3 = [f32; 3];

//tp Vec4
/// 3-dimensional vector with extra coord (1 for position, 0 for direction)
pub type Vec4 = [f32; 4];

//tp Mat3
/// 3-by-3 matrix for transformation of Vec3
pub type Mat3 = [f32; 9];

//tp Mat4
/// 4-by-4 matrix for transformation of Vec4
pub type Mat4 = [f32; 16];

//tp Quat - Quaternion
/// Quaternion
pub type Quat = [f32; 4];

//a Buffer
//tp BufferElementType
/// The type of an element in a buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferElementType {
    /// 32-bit floating point
    Float32,
    /// 16-bit floating point
    Float16,
    /// 8-bit integers
    Int8,
    /// 16-bit integers
    Int16,
    /// 32-bit integers
    Int32,
}

//a Drawing
/// A [VertexAttr] is a possible vertex attribute that can be used by
/// a renderer; a vertex always has a position attribute, but
/// additional attributes may or maynot be provided by a model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexAttr {
    /// Position (3xf32) of the point
    Position,
    /// Normal (3xf32) at the point
    Normal,
    /// Color at the point (4xf32)
    Color,
    /// Tangent at the point (4xf32?)
    Tangent,
    /// A set of joints (n x int)
    Joints,
    /// Weights (n x f16?) to apply to each bone[joint[i]]
    Weights,
    /// Texture coordinates (2 x f32)
    TexCoords0,
    /// Texture coordinates (2 x f32)
    TexCoords1,
    /// Texture coordinates (2 x f32)
    TexCoords2,
}

//tp PrimitiveType
/// Type of a primitive
///
/// This is set to match the GLTF
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PrimitiveType {
    /// Points (of an indeterminate size?)
    Points,
    /// Lines (n-1) (ab, cd, ef, ...)
    Lines,
    /// Close loop of (n) lines (ab, cd, ef, ..., za)
    LineLoop,
    /// Connected (n-1) lines (ab, bc, cd, de, ...)
    LineStrip,
    /// Individual (n/3) triangles (one for every three indices)
    Triangles,
    /// Strip of (n-2) triangles (abc, bcd, cde, def, ...)
    TriangleStrip,
    /// Fan of (n-2) triangles (abc, acd, ade, aef, ...)
    TriangleFan,
}

//tp MaterialAspect
/// The aspect of a material
#[derive(Debug)]
pub enum MaterialAspect {
    /// Color (notionally RGBA as 4xf32)
    Color,
    /// Normal
    Normal,
    /// MetallicRoughness (notionally MR as 2xf32)
    MetallicRoughness,
    /// Occlusion (as f32)
    Occlusion,
    /// Emission (as f32)
    Emission,
}
