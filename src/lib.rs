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

/*a To do

remove indexing from quaternions

Add matrix trait which takes out most of the sqmatrix traits

Document and get transform to work

Fix rotate_x, rotate_y, rotate_z

Make quaternion *not* be of V3 V4 but have them in the where of the functions as it is with of_rotation3

quaternion distance functions?

*/
//a Documentation
#![warn(missing_docs)]
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

# Function operation

The functions for geometry manipulation are provided in the [vector],
[mat] and [quat] modules.

## Basic operation

```
use geo_nd::vector;
let y = [0., 1.];
let x = [1., 0.];
assert_eq!( vector::dot(&x, &y), 0., "Dot product of X and Y axis vectors is zero");
let xy = vector::add(x,&y,2.);
assert_eq!( xy, [1., 2.], "x + 2*y");
assert_eq!( vector::length_sq(&xy), (5.), "|x + 2*y|^2 = 5");
assert_eq!( vector::length(&xy), (5.0f64).sqrt(), "|x + 2*y| = sqrt(5)");
```

# Provided traits

The library provides traits for types that can be vectors, matrices, and quaternions.

## Vector

Types that provide [Vector] can be manipulated with negation, addition,
subtraction, and can be scaled with multiplication and division by
their 'float'; their components can be accessed through indexing
(e.g. a[0]) immutably and mutably.

 As non-traditional vector operations they can be piece-wise
multiplied and divided also, which can be useful in graphcis
applications; they can also be piece-wise added and subtracted from
using their 'float'.

They also support [Copy], [std::default::Default], [std::fmt::Debug], and [std::fmt::Display],
[serde::Serialize], [serde::Deserialize].

They provide AsRef for float arrays of length 4 and slices for fast import and export from memory structures.

### Vector3

Vector3 types are 3-element vectors which additionally provide a
[Vector3::cross_product] method, which does not exist (in a simply
well defined manner) for other vector sizes.

## SqMatrix

Types that provide [SqMatrix] are square matrices that can be
manipulated with negation, addition, subtraction, and multiplicaton, and can be
scaled with multiplication and division by their 'float'; their
components can be accessed through indexing (e.g. a[0]) immutably and
mutably. The index is a usize, in row-major order (i.e. [0] is row
zero column zero, [1] is row 0 column 1, and [nr] is row 1 column 0
for a matrixt that is 'nr' by 'nc' rows by columns.)

 They can also be piece-wise added and subtracted from using their
'float'.

They also support [Copy], [std::default::Default], [std::fmt::Debug], and [std::fmt::Display],
[serde::Serialize], [serde::Deserialize].

They provide AsRef for float arrays of length 4 and slices for fast import and export from memory structures.


### SqMatrix4

Types that provide [SqMatrix4] are 4-by-4 matrices. Additional methods
are provided for graphics operations, and so the matrices are treated
as 3-by-3 transformation matrices with a translation vector and the
last element the distance scaling.

They provide [SqMatrix] and additionally support graphically-useful
constructors 'perspective' and 'look_at', and support translation by
vectors.

## Quaternion

Quaternions are a mathematical means for describing a 3 dimensional rotation around
the origin.

Types that provide [Quaternion] have associated 3-element and
4-element vectors types that must provide the [Vector] trait.

Types that provide [Quaternion] can be
manipulated with negation, addition, subtraction, and multiplicaton, and can be
scaled with multiplication and division by their 'float'.

They also support [Copy], [std::default::Default], [std::fmt::Debug], and [std::fmt::Display],
[serde::Serialize], [serde::Deserialize].

They provide AsRef for float arrays of length 4 and slices for fast
import and export from memory structures. Currently the mapping of the
arrays is (i, j, k, r).

### Constructors

Types providing the [Quaternion] trait can be constructed from:

* a unit quaternion (1.0 + 0*i + 0*j + 0*k)

* (r,i,j,k) tuples

* the conjugate of another quaternion, i.e. (r,-i,-j,-k)

* a rotation around a [Vector<3>] axis by an angle (in radians)

* a rotation around one of the axes applied to another quaternion

* another quaternion applied to a rotation around one of the axes

* from a square matrix that describes a pure rotation (no scaling)

* that describes a rotation of a camera looking down the negative Z
  axis with the Y axis as up, to one looking along a specified
  direction with a (perpendicular) up direction

* the rotation that provides the shortest great circle path for one
  unit vector to another (the axis of the rotation is the
  perpendicular to both)

* the weighted average of a number of quaternions

The trait provides many application methods for quaternions, perhaps
the most important being [Quaternion::apply3] and
[Quaternion::apply4], which allow the quaternion to be applied to a
3-elemet or 4-element vector (the latter being common in graphics,
where the fourth element is usually 1 for a point, and 0 for a vector
translation).

# Provided types

The library provides types that simply wrap `f32` and `f64` arrays,
providing imlpementations of the traits and hence supporting vectors, matrices and quaternions. This is perhaps the
simplest way to use the library.

## Vector types

The [FArray] type is a wrapper around an N-element array of floats,
and it supports the [Vector] trait.

## SqMatrix types

The [FArray2] type is a wrapper around an N-by-N-element array of floats,
and it supports the [SqMatrix] trait.

## Quaternion types

The [QArray] type is a wrapper around an 4-element array of floats,
and it supports the [Quaternion] trait.

# Examples

## Two dimensions

```
// Import the traits
use geo_nd::{Vector, SqMatrix};

// Aliases for the types
pub type Point2D = geo_nd::FArray<f64, 2>;
pub type Mat2x2 = geo_nd::FArray2<f64, 2, 4>;

let x : Point2D = [1.0, 0.0].into();
let y : Point2D = [0.0, 1.0].into();

let c = 30.0_f64.to_radians().cos();
let s = 30.0_f64.to_radians().sin();
let rot30 : Mat2x2 = [c, -s, s, c].into();

let rot60 = rot30 * rot30;

// Rotating x anticlockwise by 30 and 60 should turn it into y
let is_it_y = rot60.transform(&rot30.transform(&x));

// Check that the distance between them is tiny
assert!((y-is_it_y).length_sq() < 1.0E-8);

assert!(y.distance(&is_it_y) < 1.0E-8);

let rot90 = rot60 * rot30;
let rot180 = rot90 * rot90;

let xy = x + y;
let is_it_zero = xy + rot180.transform(&xy);
assert!(is_it_zero.length() < 1.0E-8);
```

## Three dimensions

```
// Import the traits
use geo_nd::{Quaternion, SqMatrix, Vector};

// Aliases for the types
pub type Point3D = geo_nd::FArray<f64, 3>;
pub type Mat3x3 = geo_nd::FArray2<f64, 3, 9>;
pub type Point4D = geo_nd::FArray<f64, 4>;
pub type Quat = geo_nd::QArray<f64, Point3D, Point4D>;

let x : Point3D = [1., 0., 0.].into();
let y : Point3D = [0., 1., 0.].into();
let z : Point3D = [0., 0., 1.].into();

// qx rotates around the X axis by 90 degrees
// [X,0,0] is unchanged
// [0,1,0] maps to [0,0,1]
// [0,0,1] maps to [0,-1,0]
let qx = Quat::unit().rotate_x(90.0_f64.to_radians());
assert!(z.distance(&qx.apply3(&y)) < 1.0E-8);
assert!(y.distance(&qx.apply3(&-z)) < 1.0E-8);
assert!(x.distance(&qx.apply3(&x)) < 1.0E-8);

// qy rotates around the Y axis by 90 degrees
// [1,0,0] maps to [0,0,-1]
// [0,Y,0] is unchanged
// [0,0,1] maps to [1,0,0]
let qy = Quat::unit().rotate_y(90.0_f64.to_radians());
assert!(x.distance(&qy.apply3(&z)) < 1.0E-8);
assert!(z.distance(&qy.apply3(&-x)) < 1.0E-8);
assert!(y.distance(&qy.apply3(&y)) < 1.0E-8);

// qx * qy applies qx to (qy applied to a vector)
// Hence this chains the qx mapping onto the qy mapping
// [1,0,0] -> [0,0,-1] -> [0,1,0]
// [0,1,0] -> [0,1,0] -> [0,0,1]
// [0,0,1] -> [1,0,0] -> [1,0,0]
//
// This is actually a 120 degree rotation around (1,1,1)
// (qy * qx is a 120 degree rotation around (1,-1,1))
let qxy = qx * qy;
assert!(y.distance(&qxy.apply3(&x)) < 1.0E-8);
assert!(z.distance(&qxy.apply3(&y)) < 1.0E-8);
assert!(x.distance(&qxy.apply3(&z)) < 1.0E-8);

let mut m = Mat3x3::default();
qxy.set_rotation3(&mut m);
// qxy will be [0,0,1,  1,0,0, 0,1,0]
// give or take floating point errors
assert!((m.transform(&x) - y).length() < 1.0E-8);
assert!((m.transform(&y) - z).length() < 1.0E-8);
assert!((m.transform(&z) - x).length() < 1.0E-8);
```
!*/

//a Imports
mod matrix_op;
mod matrixr_op;
mod quaternion_op;
mod traits;
mod vector_op;

mod farray;
mod farray2;
mod fqarray;
mod qarray;

//a Exports
pub use farray::FArray;
pub use farray2::FArray2;
pub use fqarray::FQArrayTrans;
pub use qarray::QArray;
pub use traits::{
    Float, Geometry2D, Geometry3D, Num, Quaternion, SqMatrix, SqMatrix3, SqMatrix4, Transform,
    Vector, Vector3, Vector3D,
};

/// Vector functions module
///
/// This module provides numerous N-dimensional vector operations operating on [Num; N] (or [Float; N]).
pub mod vector {
    pub use super::vector_op::*;
}

/// Quaternion module
pub mod quat {
    pub use super::quaternion_op::*;
}

/// Matrix library
pub mod matrix {
    pub use super::matrix_op::*;
    pub use super::matrixr_op::*;
}

//a SIMD configuration
#[cfg(feature = "simd")]
extern crate core_simd;
#[cfg(feature = "simd")]
pub mod simd {
    mod simd_vec;
    pub use self::simd_vec::{F32x2Vec2, F32x4Vec2, F32x4Vec3, F32x4Vec4};

    //tp SimdVecF32A16 - empty struct that provides a wrapper for the associated types
    pub struct VecF32A16 {}
    impl crate::Vector3D<f32> for VecF32A16 {
        type Vec2 = F32x4Vec2;
        type Vec3 = F32x4Vec3;
        type Vec4 = F32x4Vec4;
    }

    //tp SimdVecF32A8 - empty struct that provides a wrapper for the associated types
    pub struct VecF32A8 {}
    impl crate::Vector3D<f32> for VecF32A8 {
        type Vec2 = F32x2Vec2;
        type Vec3 = F32x4Vec3;
        type Vec4 = F32x4Vec4;
    }
}

//a Vector3D and Geometry3D for f32/f64 using FArray/FArray2
//ip Vector3D for f32
impl Vector3D<f32> for f32 {
    type Vec2 = FArray<f32, 2>;
    type Vec3 = FArray<f32, 3>;
    type Vec4 = FArray<f32, 4>;
}

//ip Geometry3D for f32
impl Geometry3D<f32> for f32 {
    type Vec3 = FArray<f32, 3>;
    type Vec4 = FArray<f32, 4>;
    type Mat3 = FArray2<f32, 3, 9>;
    type Mat4 = FArray2<f32, 4, 16>;
    type Quat = QArray<f32, FArray<f32, 3>, FArray<f32, 4>>;
    type Trans = FQArrayTrans<f32>;
}

//ip Geometry2D for f32
impl Geometry2D<f32> for f32 {
    type Vec2 = FArray<f32, 2>;
    type Mat2 = FArray2<f32, 2, 4>;
}

//ip Vector3D for f64
impl Vector3D<f64> for f64 {
    type Vec2 = FArray<f64, 2>;
    type Vec3 = FArray<f64, 3>;
    type Vec4 = FArray<f64, 4>;
}

//ip Geometry3D for f64
impl Geometry3D<f64> for f64 {
    type Vec3 = FArray<f64, 3>;
    type Vec4 = FArray<f64, 4>;
    type Mat3 = FArray2<f64, 3, 9>;
    type Mat4 = FArray2<f64, 4, 16>;
    type Quat = QArray<f64, FArray<f64, 3>, FArray<f64, 4>>;
    type Trans = FQArrayTrans<f64>;
}

//ip Geometry2D for f64
impl Geometry2D<f64> for f64 {
    type Vec2 = FArray<f64, 2>;
    type Mat2 = FArray2<f64, 2, 4>;
}

//a GLSL-compatible things - bit of a place holder currently
/// The [glsl] module is a place-holder for types that are compatible with GLSL
pub mod glsl {
    /// GLSL 2-component vector of float
    pub type Vec2 = [f32; 2];
    /// GLSL 3-component vector of float
    pub type Vec3 = [f32; 3];
    /// GLSL 4-component vector of float
    pub type Vec4 = [f32; 4];
    /// GLSL 2-component vector of double
    pub type DVec2 = [f64; 2];
    /// GLSL 3-component vector of double
    pub type DVec3 = [f64; 3];
    /// GLSL 4-component vector of double
    pub type DVec4 = [f64; 4];
    /// GLSL 2-component vector of signed integer
    pub type IVec2 = [i32; 2];
    /// GLSL 3-component vector of signed integer
    pub type IVec3 = [i32; 3];
    /// GLSL 4-component vector of signed integer
    pub type IVec4 = [i32; 4];
    /// GLSL 2x2 floating-point matrix
    pub type Mat2 = [f32; 4];
    /// GLSL 3x3 floating-point matrix
    pub type Mat3 = [f32; 9];
    /// GLSL 4x4 floating-point matrix
    pub type Mat4 = [f32; 16];
    /// GLSL 2x2 double-precision floating-point matrix
    pub type DMat2 = [f64; 4];
    /// GLSL 3x3double-precision floating-point matrix
    pub type DMat3 = [f64; 9];
    /// GLSL 4x4 double-precision floating-point matrix
    pub type DMat4 = [f64; 16];
}
