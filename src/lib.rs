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
@brief   Geometry library
 */

//a Documentation
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
/*!

# Geometry library

This library provides for N-dimensional geometrical objects,
particularly *Vector*s, *Matrix*, *Quaternion* operations.

The underlying type is \[Num; N\], so the data may be shared simply
with other libraries, including OpenGL.

The library mirrors the operation of 'glm' in some sense.

The desire for the library is that it does not undergo much
development; it provides a stable and simple basis for operations that
are common mathematical operations, with no aim for it to grow into a
larger linear algebra library.

The library operates on arrays of elements that support the
[`Num`](Num) trait, which requires basic arithmetic operations, copy,
clone, debug and display; some functions further require the
[`Float`](Float) trait, which also requires operations such as sqrt,
sin/cos, etc.

The library does not expose any types: all of the operations it
supports are provided through functions.

## Caveats

The functions in the library use const generics, but as const generic
evaulations are currently unstable it requires more consts than should
be required. For example, to create an identity square matrix the
`matrix::identity` function has the generic signature `<V:Num, const
D2:usize, const D:usize>`. The value of `D2` *must* equal `D*D`. The
function returns `[V; D2]`.

Ideally the function should just take D as a const generic argument
and the type would be `[V;D*D]`, but that is unstable (and there are
some other issues).

Additionally, the inference of a type for `V` is sometimes required to
be forced, so there may be a small amount of turbofish notation such
as `identity2::<f32>()`.

## Basic operation

```
extern crate geometry;
use geometry::vector;
let y = [0., 1.];
let x = [1., 0.];
assert_eq!( vector::dot(&x, &y), 0., "Dot product of X and Y axis vectors is zero");
let xy = vector::add(x,&y,2.);
assert_eq!( xy, [1., 2.], "x + 2*y");
assert_eq!( vector::length_sq(&xy), (5.), "|x + 2*y|^2 = 5");
assert_eq!( vector::length(&xy), (5.0f64).sqrt(), "|x + 2*y| = sqrt(5)");
```

!*/

//a Crates
extern crate num_traits;

//a Imports
mod traits;
mod vector_op;
mod quaternion_op;
mod matrixr_op;
mod matrix_op;

mod fslice;
mod fslice2;

//a Exports
pub use traits::*;
pub use fslice::FSlice;
pub use fslice2::FSlice2;

/// Vector functions module
///
/// This module provides numerous N-dimensional vector operations operating on [Num; N] (or [Float; N]).
pub mod vector   {
    pub use super::vector_op::* ;
}

/// Quaternion module
pub mod quat     {
    pub use super::quaternion_op::* ;
}

/// Matrix library
pub mod matrix   {
    pub use super::matrixr_op::* ;
    pub use super::matrix_op::* ;
}

//a SIMD configuration
#[cfg(feature="simd")]
extern crate core_simd;
#[cfg(feature="simd")]
pub mod simd {
    mod simd_vec;
    pub use self::simd_vec::{F32x4Vec2, F32x4Vec3,F32x4Vec4, F32x2Vec2};

    //tp SimdVecF32A16 - empty struct that provides a wrapper for the associated types
    pub struct VecF32A16 {}
    impl crate::Vector3D<f32> for VecF32A16 {
        type Vec2   = F32x4Vec2;
        type Vec3   = F32x4Vec3;
        type Vec4   = F32x4Vec4;
    }

    //tp SimdVecF32A8 - empty struct that provides a wrapper for the associated types
    pub struct VecF32A8 {}
    impl crate::Vector3D<f32> for VecF32A8 {
        type Vec2   = F32x2Vec2;
        type Vec3   = F32x4Vec3;
        type Vec4   = F32x4Vec4;
    }
}

//a Implementations
//a Vector3D and Geometry3D for f32/f64 using FSlice/FSlice2
//ip Vector3D for f32
impl Vector3D<f32> for f32 {
    type Vec2 = FSlice<f32,2>;
    type Vec3 = FSlice<f32,3>;
    type Vec4 = FSlice<f32,4>;
}

//ip Geometry3D for f32
impl Geometry3D<f32> for f32 {
    type Vec3 = FSlice<f32,3>;
    type Vec4 = FSlice<f32,4>;
    type Mat3 = FSlice2<f32,3,9>;
    type Mat4 = FSlice2<f32,4,16>;
}

//ip Geometry2D for f32
impl Geometry2D<f32> for f32 {
    type Vec2 = FSlice<f32,2>;
    type Mat2 = FSlice2<f32,2,4>;
}

//ip Vector3D for f64
impl Vector3D<f64> for f64 {
    type Vec2 = FSlice<f64,2>;
    type Vec3 = FSlice<f64,3>;
    type Vec4 = FSlice<f64,4>;
}

//ip Geometry3D for f64
impl Geometry3D<f64> for f64 {
    type Vec3 = FSlice<f64,3>;
    type Vec4 = FSlice<f64,4>;
    type Mat3 = FSlice2<f64,3,9>;
    type Mat4 = FSlice2<f64,4,16>;
}

//ip Geometry2D for f64
impl Geometry2D<f64> for f64 {
    type Vec2 = FSlice<f64,2>;
    type Mat2 = FSlice2<f64,2,4>;
}

/*a Stuff
// a Generic types as per GLSL
/// GLSL 2-component vector of float
pub type Vec2 = [f32;2];
/// GLSL 3-component vector of float
pub type Vec3 = [f32;3];
/// GLSL 4-component vector of float
pub type Vec4 = [f32;4];
/// GLSL 2-component vector of double
pub type DVec2 = [f64;2];
/// GLSL 3-component vector of double
pub type DVec3 = [f64;3];
/// GLSL 4-component vector of double
pub type DVec4 = [f64;4];
/// GLSL 2-component vector of signed integer
pub type IVec2 = [i32;2];
/// GLSL 3-component vector of signed integer
pub type IVec3 = [i32;3];
/// GLSL 4-component vector of signed integer
pub type IVec4 = [i32;4];
/// GLSL 2x2 floating-point matrix
pub type Mat2 = [f32;4];
/// GLSL 3x3 floating-point matrix
pub type Mat3 = [f32;9];
/// GLSL 4x4 floating-point matrix
pub type Mat4 = [f32;16];
/// GLSL 2x2 double-precision floating-point matrix
pub type DMat2 = [f64;4];
/// GLSL 3x3double-precision floating-point matrix
pub type DMat3 = [f64;9];
/// GLSL 4x4 double-precision floating-point matrix
pub type DMat4 = [f64;16];
 */
