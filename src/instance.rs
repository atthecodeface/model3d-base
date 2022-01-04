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

@file    drawable.rs
@brief   Part of OpenGL library
 */

//a Notes
//
//

//a Imports
use crate::{Mat4, Transformation, BonePoseSet, Instantiable};

//a Instance
//tp Instance
/// A drawable::Instance contains the instance data for an instance of a drawable::Instantiable
///
/// It requires a base transformation, an array of BonePose (which matches the Instantiable's BoneSet array), and an array of Mat4 for each bone in the BonePose array.
pub struct Instance<'a> {
    /// Reference to the Instantiable
    instantiable : &'a Instantiable,
    /// The transformation to apply to this model instance
    pub transformation : Transformation,
    /// Matrix for the transformation (must be updated after updating Transformation),
    pub trans_mat : Mat4,
    /// The sets of BonePose corresponding to the BoneSet array in the Instantiable
    pub bone_poses   : Vec<BonePoseSet<'a>>,
    /// Transformation matrices for the bones
    pub bone_matrices   : Vec<Mat4>,
}

impl <'a> Instance<'a> {
    //fp new
    /// Create a new [Instance] from an [Instantiable]
    ///
    /// This contains an array of [BonePoseSet]s to allow elements of
    /// the [Instantiable] to be posed, and respective matrices for
    /// drawing the meshes within the [Instantiable]
    ///
    /// It should contain appropriate Materials too
    pub fn new(instantiable:&'a Instantiable, num_bone_matrices:usize) -> Self {
        let transformation = Transformation::new();
        let trans_mat = [0.;16];
        let bone_poses = Vec::new();
        let mut bone_matrices = Vec::with_capacity(num_bone_matrices);
        for _ in 0..num_bone_matrices {
            bone_matrices.push([0.;16]);
        }
        Self { instantiable, transformation, trans_mat, bone_poses, bone_matrices }
    }
}
