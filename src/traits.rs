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

@file    vector_op.rs
@brief   Part of geometry library
 */

//a Num and Float traits
//tp Num
/// Trait required for matrix or vector elements
pub trait Num : std::ops::Neg<Output=Self> +
    num_traits::Num + num_traits::NumAssignOps +
    Clone + Copy + PartialEq + std::fmt::Display + std::fmt::Debug {
    }

//tp Float
/// Trait required for matrix or vector elements such that also need operations such as sqrt, sin/cos, etc
/// Annoyingly this includes NumCast
pub trait Float : Num + num_traits::Float {
    #[inline]
    fn int(n:isize) -> Self { Self::from(n).unwrap() }
    #[inline]
    fn frac(n:isize, d:usize) -> Self { Self::from((n as f32)/(d as f32)).unwrap() }
}

//ti Num for f32/f64/i32/i64/isize
impl Num for f32 {}
impl Num for f64 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for isize {}

//ti Float for f32/f64
impl Float for f32 {}
impl Float for f64 {}

//a Vector and SqMatrix
//tt Vector
pub trait Vector<F:Float, const D:usize> : Clone
    + Copy
    + std::fmt::Debug
    + std::default::Default
    + std::convert::AsRef<[F;D]>
    + std::convert::AsMut<[F;D]>
    + std::convert::AsRef<[F]>
    + std::convert::AsMut<[F]>
    + std::ops::Index<usize, Output = F>
    + std::ops::Neg<Output = Self>
    + std::ops::Add<Self, Output = Self>
    + std::ops::Add<F, Output = Self>
    + std::ops::AddAssign<Self>
    + std::ops::AddAssign<F>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Sub<F, Output = Self>
    + std::ops::SubAssign<Self>
    + std::ops::SubAssign<F>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Mul<F, Output = Self>
    + std::ops::MulAssign<Self>
    + std::ops::MulAssign<F>
    + std::ops::Div<Self, Output = Self>
    + std::ops::Div<F, Output = Self>
    + std::ops::DivAssign<Self>
    + std::ops::DivAssign<F> {
        fn from_array(data:[F;D]) -> Self;
        fn zero() -> Self;
        fn is_zero(&self) -> bool;
        fn set_zero(&mut self);
        fn reduce_sum(&self) -> F;
        fn mix(&self, other:&Self, t:F) -> Self;
        fn dot(&self, other:&Self) -> F;
        fn length_sq(&self) -> F { self.dot(self) }
        fn length(&self)    -> F { self.length_sq().sqrt() }
        fn distance_sq(&self, other:&Self) -> F { (*self - *other).length_sq() }
        fn distance(&self, other:&Self) -> F { self.distance_sq(other).sqrt() }
        fn normalize(&mut self) { let l = self.length(); if l < F::epsilon() {self.set_zero()} else {*self /= l} }
        // clamp
        // rotate_around
    }

//tt SqMatrix
pub trait SqMatrix<V:Vector<F,D>, F:Float, const D:usize, const D2:usize> : Clone
    + Copy
    + std::fmt::Debug
    + std::default::Default
    + std::convert::AsRef<[F;D2]>
    + std::convert::AsMut<[F;D2]>
    + std::convert::AsRef<[F]>
    + std::convert::AsMut<[F]>
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Mul<F, Output = Self>
    + std::ops::MulAssign<F>
    + std::ops::Div<F, Output = Self>
    + std::ops::DivAssign<F> {
        fn from_array(data:[F;D2]) -> Self;
        fn identity() -> Self;
        fn zero() -> Self;
        fn is_zero(&self) -> bool;
        fn set_zero(&mut self);
        // absmax
        fn transpose(&self) -> Self;
        fn determinant(&self) -> F;
        fn inverse(&self) -> Self;
        fn transform(&self, v:V) -> V;
    }

//a Vector3D, Geometry3D
//tt Vector3D
pub trait Vector3D<Scalar:Float> {
    type Vec2 : Vector<Scalar, 2>;
    type Vec3 : Vector<Scalar, 3>;
    type Vec4 : Vector<Scalar, 4>;
}

//tt Geometry3D
pub trait Geometry3D<Scalar:Float> {
    type Vec3 : Vector<Scalar, 3>;
    type Vec4 : Vector<Scalar, 4>;
    type Mat3 : SqMatrix<Self::Vec3, Scalar, 3, 9>;
    type Mat4 : SqMatrix<Self::Vec4, Scalar, 4, 16>;
    // fn perspective4
    // fn translate4
    // fn from_quat3
    // fn from_quat4
    // fn of_transform3/4?
    // cross_product3
    // axis_of_rotation3/4
    // clamp
}

//tt Geometry2D
pub trait Geometry2D<Scalar:Float> {
    type Vec2 : Vector<Scalar, 2>;
    type Mat2 : SqMatrix<Self::Vec2, Scalar, 2, 4>;
}

