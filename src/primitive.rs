//a Imports
use crate::PrimitiveType;

//a Primitive
//tp Primitive
/// A primitive consisting of a material and a subset of
/// vertices using a particular range of indices
///
/// This might be, for example, the arm of a robot.
///
/// The [Primitive] depends on being in an 3D model [crate::Object] (or its derived [crate::Instantiable], as it is the
/// object that contains the actual materials and vertices to use
///
/// This *SHOULD* be optimized to fit within half a cache line (32 bytes)
///
/// Missing:
///
/// uses bones?
/// index type (u8, u16, u32) - is this part of indices?
#[derive(Debug, Clone)]
pub struct Primitive {
    /// First index to use
    ///
    /// If all 1s then vertices_index is invalid?
    index_offset: u32,
    /// Number of indices to use
    index_count: u32,
    /// Material to be used in drawing - index within the [crate::Object]
    material_index: u16,
    /// Vertices index within the [crate::Object]
    ///
    /// This provides (effectively) the set of attribute `BufferView`s that the mesh utilizes
    vertices_index: u16,
    /// Type of the primitive (u8)
    primitive_type: PrimitiveType,
}

//ip Primitive
impl Primitive {
    //fp new
    /// Create a new Primitive from a Vertices
    ///
    /// use the indices' BufferView.ele_type: BufferElementType as index size
    pub fn new(
        primitive_type: PrimitiveType,
        vertices_index: usize,
        index_offset: u32,
        index_count: u32,
        material_index: usize,
    ) -> Self {
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
        (
            self.vertices_index as usize,
            self.index_offset,
            self.index_count,
        )
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

    //mp vertices_index
    /// Retrieve the index into the [crate::Object] vertices array that this
    /// primitive uses
    pub fn vertices_index(&self) -> usize {
        self.vertices_index as usize
    }

    //mp material_index
    /// Retrieve the index into the [crate::Object] materials array that this
    /// primitive uses
    pub fn material_index(&self) -> usize {
        self.material_index as usize
    }

    //mp index_count
    /// Get the number of indices required to draw this primitive
    pub fn index_count(&self) -> u32 {
        self.index_count
    }

    //mp byte_offset
    /// Get the byte offset within the indices buffer view to the
    /// first index used by this primitive
    pub fn byte_offset(&self) -> usize {
        self.index_offset as usize
    }

    //zz All done
}
