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
/// The [Num] trait is required for matrix or vector elements; it is
/// not a float, and so some of the matrix and vector operations can
/// operate on integer types such as i32, i64 and isize
///
/// The trait requires basic numeric operations, plus specifically [std::fmt::Display].
pub trait Num:
    std::ops::Neg<Output = Self>
    + num_traits::Num
    + num_traits::NumAssignOps
    + Clone
    + Copy
    + PartialEq
    + std::fmt::Display
    + std::fmt::Debug
{
}

//tp Float
/// The [Float] trait is required for matrix or vector elements which have a float aspect, such as `sqrt`.
///
/// The trait is essentially `num_traits::Float`, but it supplies
/// implicit methods for construction of a [Float] from an `isize`
/// value, or as a rational from a pair of `isize` values.
///
/// As num_traits::Float includes num_traits::NumCast it is not
/// possible to require, as would perhaps be desired, a From<f32>
/// trait, without conflicts occurring.
///
pub trait Float: Num + num_traits::Float {
    //fp int
    /// Create a [Float] from an `isize` value; this should support
    /// constant implementations with no run-time overhead.
    #[inline]
    fn int(n: isize) -> Self {
        Self::from(n).unwrap()
    }

    //fp frac
    /// Create a [Float] as a fraction described by a numerator and
    /// denomiator pair of `isize` values; this should support
    /// constant implementations with no run-time overhead.
    #[inline]
    fn frac(n: isize, d: usize) -> Self {
        Self::from((n as f32) / (d as f32)).unwrap()
    }
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
/// The [Vector] trait describes an N-dimensional vector of [Float] type.
///
/// Such [Vector]s support basic vector arithmetic using addition and
/// subtraction, and they provide component-wise multiplication and
/// division, using the standard operators on two [Vector]s.
///
/// They also support basic arithmetic to all components of the
/// [Vector] for addition, subtraction, multiplication and division by
/// a scalar [Float] value type that they are comprised of. Hence a
/// `v:Vector<F>` may be scaled by a `s:F` using `v * s`.
///
/// The [Vector] can be indexed only by a `usize`; that is individual
/// components of the vector can be accessed, but ranges may not.
pub trait Vector<F: Float, const D: usize>:
    Clone
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
    + std::default::Default
    + std::convert::AsRef<[F; D]>
    + std::convert::AsMut<[F; D]>
    + std::convert::AsRef<[F]>
    + std::convert::AsMut<[F]>
    + std::ops::Index<usize, Output = F>
    + std::ops::IndexMut<usize>
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
    + std::ops::DivAssign<F>
{
    //fp from_array
    /// Create a vector from an array of [Float]
    fn from_array(data: [F; D]) -> Self;

    //fp zero
    /// Create a vector whose elements are all zero
    fn zero() -> Self;

    //mp is_zero
    /// Return true if the vector is all zeros
    fn is_zero(&self) -> bool;

    //mp set_zero
    /// Set the vector to be all zeros
    fn set_zero(&mut self);

    //mp reduce_sum
    /// Sum all of the components of the vector
    fn reduce_sum(&self) -> F;

    //mp mix
    /// Create a linear combination of this [Vector] and another using parameter `t` from zero to one
    fn mix(&self, other: &Self, t: F) -> Self;

    //mp dot
    /// Return the dot product of two vectors
    fn dot(&self, other: &Self) -> F;

    //mp length_sq
    /// Return the square of the length of the vector
    fn length_sq(&self) -> F {
        self.dot(self)
    }

    //mp length
    /// Return the length of the vector
    fn length(&self) -> F {
        self.length_sq().sqrt()
    }

    //mp distance_sq
    /// Return the square of the distance between this vector and another
    fn distance_sq(&self, other: &Self) -> F {
        (*self - *other).length_sq()
    }

    //mp distance
    /// Return the distance between this vector and another
    fn distance(&self, other: &Self) -> F {
        self.distance_sq(other).sqrt()
    }

    //mp normalize
    /// Normalize the vector; if its length is close to zero, then set it to be zero
    fn normalize(&mut self) {
        let l = self.length();
        if l < F::epsilon() {
            self.set_zero()
        } else {
            *self /= l
        }
    }
    // clamp

    //cp rotate_around
    /// Rotate a vector within a plane around a
    /// *pivot* point by the specified angle
    ///
    /// The plane of rotation is specified by providing two vector indices for the elements to adjust. For a 2D rotation then the values of c0 and c1 should be 0 and 1.
    ///
    /// For a 3D rotation about the Z axis, they should be 0 and 1; for
    /// rotation about the Y axis they should be 2 and 0; and for rotation
    /// about the X axis they should be 1 and 2.
    ///
    fn rotate_around(mut self, pivot: &Self, angle: F, c0: usize, c1: usize) -> Self {
        let (s, c) = angle.sin_cos();
        let dx = self[c0] - pivot[c0];
        let dy = self[c1] - pivot[c1];
        let x1 = c * dx - s * dy;
        let y1 = c * dy + s * dx;
        self[c0] = x1 + pivot[c0];
        self[c1] = y1 + pivot[c1];
        self
    }
}

//tt SqMatrix
/// The [SqMatrix] trait describes an N-dimensional square matrix of [Float] type that operates on a [Vector].
///
/// This trait is not stable.
///
/// Such [SqMatrix] support basic arithmetic using addition and
/// subtraction, and they provide component-wise multiplication and
/// division, using the standard operators on two [SqMatrix]s.
///
/// They also support basic arithmetic to all components of the
/// [SqMatrix] for addition, subtraction, multiplication and division by
/// a scalar [Float] value type that they are comprised of. Hence a
/// `m:SqMatrix<F>` may be scaled by a `s:F` using `m * s`.
pub trait SqMatrix<V: Vector<F, D>, F: Float, const D: usize, const D2: usize>:
    Clone
    + Copy
    + std::fmt::Debug
    + std::default::Default
    + std::convert::AsRef<[F; D2]>
    + std::convert::AsMut<[F; D2]>
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
    + std::ops::DivAssign<F>
{
    //fp from_array
    /// Create a [SqMatrix] from an array of [Float]s
    fn from_array(data: [F; D2]) -> Self;

    //fp identity
    /// Create an identity [SqMatrix]
    fn identity() -> Self;

    //fp zero
    /// Create a zero [SqMatrix]
    fn zero() -> Self;

    //fp is_zero
    /// Return true if the matrix is zer
    fn is_zero(&self) -> bool;

    //fp set_zero
    /// Set the matrix to zero
    fn set_zero(&mut self);

    // absmax

    //mp transpose
    /// Return a transpose matrix
    fn transpose(&self) -> Self;

    //mp determinant
    /// Calculate the determinant of the matrix
    fn determinant(&self) -> F;

    //mp inverse
    /// Create an inverse matrix
    fn inverse(&self) -> Self;

    //mp transform
    /// Apply the matrix to a vector to transform it
    fn transform(&self, v: V) -> V;
}

//tt Quaternion
/// The [Quaternion] trait describes a 4-dimensional vector of [Float] type.
///
/// Such [Quaternion]s support basic arithmetic using addition and
/// subtraction, and they provide quaternion multiplication and division.
///
/// They also support basic arithmetic to all components of the
/// [Quaternion] for addition, subtraction, multiplication and division by
/// a scalar [Float] value type that they are comprised of. Hence a
/// `q:Quaternion<F>` may be scaled by a `s:F` using `q * s`.
///
/// The [Quaternion] can be indexed only by a `usize`; that is individual
/// components of the vector can be accessed, but ranges may not.
pub trait Quaternion<F, V3, V4> : Clone
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
    + std::default::Default
    + std::convert::AsRef<[F;4]>
    + std::convert::AsMut<[F;4]>
    + std::convert::AsRef<[F]>
    + std::convert::AsMut<[F]>
    + std::ops::Index<usize, Output = F>
    + std::ops::IndexMut<usize>
    + std::ops::Neg<Output = Self>
    + std::ops::Add<Self, Output = Self>
    + std::ops::Add<F, Output = Self>
    + std::ops::AddAssign<Self>
    + std::ops::AddAssign<F>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Sub<F, Output = Self>
    + std::ops::SubAssign<Self>
    + std::ops::SubAssign<F>
    // + std::ops::Mul<Self, Output = Self>
    + std::ops::Mul<F, Output = Self>
    // + std::ops::MulAssign<Self>
    + std::ops::MulAssign<F>
    // + std::ops::Div<Self, Output = Self>
    + std::ops::Div<F, Output = Self>
    // + std::ops::DivAssign<Self>
    + std::ops::DivAssign<F>
where V3:Vector<F,3>, V4:Vector<F,4>, F:Float
{
        //fp from_array
        /// Create a quaternion from an array of [Float]
        fn from_array(data:[F;4]) -> Self;

        //fp as_rijk
        /// Break out into r, i, j, k
        fn as_rijk(&self) -> (F, F, F, F);

        //fp of_rijk
        /// Create from r, i, j, k
        fn of_rijk(r:F, i:F, j:F, k:F) -> Self;

        //fp conjugate
        /// Create the conjugate of a quaternion
        fn conjugate(&self) -> Self {
            let (r,i,j,k) = self.as_rijk();
            Self::of_rijk(r,-i,-j,-k)
        }

        //fp unit
        /// Create a quaternion whose elements are all zero
        fn unit() -> Self;

        //fp of_axis_angle
        fn of_axis_angle(axis:&V3, angle:F) -> Self {
            let (s,c) = F::sin_cos(angle / F::from(2).unwrap());
            let i = s * axis[0];
            let j = s * axis[1];
            let k = s * axis[2];
            let r = c;
            Self::of_rijk(r,i,j,k)
        }

        //mp set_zero
        /// Set the quaternion to be all zeros
        fn set_zero(&mut self);

        //mp mix
        /// Create a linear combination of this [Quaternion] and another using parameter `t` from zero to one
        fn mix(&self, other:&Self, t:F) -> Self;

        //mp dot
        /// Return the dot product of two quaternions; basically used for length
        fn dot(&self, other:&Self) -> F;

        //mp length_sq
        /// Return the square of the length of the quaternion
        fn length_sq(&self) -> F { self.dot(self) }

        //mp length
        /// Return the length of the quaternion
        fn length(&self)    -> F { self.length_sq().sqrt() }

        //mp distance_sq
        /// Return the square of the distance between this quaternion and another
        fn distance_sq(&self, other:&Self) -> F { (*self - *other).length_sq() }

        //mp distance
        /// Return the distance between this quaternion and another
        fn distance(&self, other:&Self) -> F { self.distance_sq(other).sqrt() }

        //mp normalize
        /// Normalize the quaternion; if its length is close to zero, then set it to be zero
        fn normalize(&mut self) { let l = self.length(); if l < F::epsilon() {self.set_zero()} else {*self /= l} }

        //fp of_rotation3
        /// Find the unit quaternion of a Matrix3 assuming it is purely a rotation
        fn of_rotation3<M> (rotation:&M) -> Self
        where M:SqMatrix<V3, F, 3, 9>;

        //fp set_rotation3
        /// Set a Matrix3 to be the rotation matrix corresponding to the unit quaternion
        fn set_rotation3<M> (&self, m:&mut M)
        where M:SqMatrix<V3, F, 3, 9>;

        //fp set_rotation4
        /// Set a Matrix4 to be the rotation matrix corresponding to the unit quaternion
        fn set_rotation4<M> (&self, m:&mut M)
        where M:SqMatrix<V4, F, 4, 16>;

        //zz All done
    }

//a Vector3D, Geometry3D
//tt Vector3D
/// This is probably a temporary trait used until SIMD supports Geometry3D and Geometry2D
///
/// The [Vector3D] trait describes vectors that may be used for
/// 3D geometry
pub trait Vector3D<Scalar: Float> {
    /// The type of a 2D vector
    type Vec2: Vector<Scalar, 2>;
    /// The type of a 3D vector
    type Vec3: Vector<Scalar, 3>;
    /// The type of a 3D vector with an additional '1' expected in its extra element
    type Vec4: Vector<Scalar, 4>;
}

//tt Geometry3D
/// The [Geometry3D] trait supplies a framework for implementing 3D
/// vector and matrix operations, and should also include the
/// quaternion type.
///
/// An implementation of [Geometry3D] can be used for OpenGL and Vulkan graphics, for example.
pub trait Geometry3D<Scalar: Float> {
    /// The type of a 3D vector
    type Vec3: Vector<Scalar, 3>;
    /// The type of a 3D vector with an additional '1' expected in its extra element
    type Vec4: Vector<Scalar, 4>;
    /// The type of a 3D matrix that can transform Vec3
    type Mat3: SqMatrix<Self::Vec3, Scalar, 3, 9>;
    /// The type of a 3D matrix which allows for translations, that can transform Vec4
    type Mat4: SqMatrix<Self::Vec4, Scalar, 4, 16>;
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
/// This is an experimental trait - it bundles together a Vec2 and a Mat2.
///
/// The [Geometry2D] trait supplies a framework for implementing 2D
/// vector and matrix operations.
pub trait Geometry2D<Scalar: Float> {
    /// The type of a 2D vector
    type Vec2: Vector<Scalar, 2>;
    /// The type of a 2D matrix that can transform a Vec2
    type Mat2: SqMatrix<Self::Vec2, Scalar, 2, 4>;
}
