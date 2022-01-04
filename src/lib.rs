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

@file    lib.rs
@brief   3D model library
 */

//a Documentation
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

/*!
# 3D Model library

This library provides structures and functions to support simple and
complex 3D objects in a
reasonably performant system. It use cases include 3D modeling tools,
games, and 3D user interfaces.

The object model is derived from the Khronos glTF 3D
model/scene description (<https://github.com/KhronosGroup/glTF>),
without explicit support for animation or cameras.

Underlying the data model is the ByteBuffer trait - any data that is
used for the models must support this trait, and implementations are
provided for [_] and for Vec<>.

The base concept for model [BufferData] is a buffer that is borrowed
and that has the [ByteBuffer] trait; the data internally may be
floats, ints, etc, or combinations thereof - from which one creates
[BufferView]s, or which it is itself used as model indices. A
BufferView is a subset of the buffer, and a single buffer may have
many views. A BufferView may, for example, be the vertex positions for
a set of models; it may be texture coordinates; and so on. The
BufferData corresponds on the OpenGL side to an ARRAY_BUFFER or an
ELEMENT_ARRAY_BUFFER; hence it expects to have a VBO associated with
it.

The BufferView here is closer to the glTF Accessor - it combines in
essence the gltF Accessor and the glTF BufferView.

A set of [BufferView]s are borrowed to describe [Vertices], each
BufferView providing one piece of vertex information (such as position
or notmal). A single BufferView may be used by more than one Vertices
object. The Vertices object includes also a Data that provides the
indices. The Vertices object should be considered to be a complete
descriptor of a model or set of models within a single ByteBuffer. In
OpenGL a Vertices object is bound with a shader description to create
a VAO.

A Vertices object is then used by a number of [Primitive]s; each of
these borrows the Vertices object, and it owns an array of
Drawables. Each Drawable is an OpenGL element type (such as
TriangleStrip), a number of indices, and an indication as to which
index within the Vertices object to use as the first index. Each Primitive has a single Material associated with it.

An array of Primitive objects is owned by a Mesh object, which corresponds to
the glTF Mesh - hence the Primitive object here corresponds to a glTF
Primitive within a glTF Mesh. A Mesh might correspond to a table leg or the table top in a model.

A number of Mesh objects are borrowed to form Object Nodes; each Node has its own Transformation that is applied to its Mesh. Hence a table can consist of four Object Nodes that borrow the same table leg Mesh, and a further Object Node that is the table top. A Node may also have a BoneSet associated with it, to provide for *skinned* objects.

An Object Node forms part of an Object's Hierarchy - hence the Object
Nodes are owned by the Object. An Object, then, can be a complete
table, for example. It might also be a posable robot; in this case one
would expect the top level node to have a BoneSet, and there to
perhaps be Object Nodes for the body, head, arms and legs, perhaps
sharing the same Mesh for the two arms and anoter Mesh for the two
legs.

It is worth noting at this point that the lifetime of an Object must
be no more than the lifetime of the data buffer containing its data;
even though the Object may be passed in to a GPU memory, the data used
for building the object then not being required by the CPU (using
STATIC_DRAW). It is, then, clearly useful to consider the Object as a
model *construction* type, not necessarily the model *draw* type.

When it comes to rendering an Object, this requires a Shader. The data
required by a Shader to render an Object depends not just on the
Object but also on the Shader's capabilities (such as does it utilize
vertex tangents). However, there is some data that is common for all
the Shaders that might render a single Object instance - such as the
bone poses for the object, and the mesh matrices.

Hence an Object must have two type created for it prior to rendering. The first is a drawable::Instantatiable. This is a drawable object that in itself may have instantiations.
The drawable::Instantiable contains an owned copy of the BoneSet for
the object, and any transformation data required by the meshes for
drawing (given each object node has its own transformation). The drawable::Instantiable is created from the object using its create_instantiable method.

The second type required for rendering is a shader::Instantiable. This
is used by binding the data required for a shader for an object to a
VAO (Vertex Attribute Object) for the shader; this VAO is used in the
rendering of any instances of the shader::Instantiable. The
shader::Instantiable borrows the drawable::Instantiable created for
the Object; it is created using the Object's bind_shader method.

With STATIC_DRAW the lifetime of the shader::Instantiable can be shorter than that of the Object data - it can have a lifetime of the rendering.

Once an Object has had all of its drawable::Instantiable and shader::Instantiable types created, the Object may be dropped.

A renderable instance of an object then requires a drawable::Instance
to be created from the drawable::Instantiable; this instance has its
own transformation and bone poses, and it borrows the
drawable::Instantiable. The Instance can be rendered with a particular
shader using that shader::Instantiable's `gl_draw` method, which takes
a reference to the Instance. This method then has access to all the
matrices required for the mesh, for the posed bones, and so on.


NOTE THAT THE LIFETIME STUFF ONLY WORKS IF WE ADD A 'PASS ON GL_BUFFER' TO THE BUFFER TYPES

A Shader is created using standard OpenGL calls. It must have the ShaderClass trait.

An instantiable model consists of abstract mesh data and poses of the
elements within its skeleton, if required.

Multiple instances of an instantiable model can be created, each with its own set of poses.

The Hierarchy module provides for a hierarchy of owned elements which
are stored in an array inside the Hierachy structure; the
rerlationship between nodes in the hierarchy are handled by indices
into this array. The Hierarchy is designed to be created, and then
immutably interrogated - although the immutability refers to the
*hierarchy* and the *node array*, not the contents of the nodes - the
node content may be updated at will.

!*/

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

//a Imports and exports
pub mod hierarchy;

mod transformation;
pub use transformation::Transformation;

mod bone;
mod bone_pose;
pub use bone::Bone;
pub use bone_pose::BonePose;

mod skeleton;
mod skeleton_pose;
pub use skeleton::Skeleton;
pub use skeleton_pose::SkeletonPose;

mod byte_buffer;
pub use byte_buffer::ByteBuffer;

mod buffer_data;
pub use buffer_data::{BufferClientID, BufferData};
mod buffer_view;
pub use buffer_view::BufferView;

mod traits;
pub use traits::{Texture};
mod material;
pub use material::{BaseData, Material, BaseMaterial, TexturedMaterial, PbrMaterial};

mod instantiable;
mod instance;
pub use instantiable::Instantiable;
pub use instance::Instance;
