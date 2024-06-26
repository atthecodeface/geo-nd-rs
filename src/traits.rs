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

//a Imports
use crate::{quat, vector};

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

    //fp pi
    /// Return the value of PI
    fn pi() -> Self;

    //fp tau
    /// Return the value of 2*PI
    fn tau() -> Self;
}

//ti Num for f32/f64/i32/i64/isize
impl Num for f32 {}
impl Num for f64 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for isize {}

//ti Float for f32/f64
impl Float for f32 {
    #[inline]
    fn pi() -> Self {
        std::f32::consts::PI
    }
    #[inline]
    fn tau() -> Self {
        std::f32::consts::TAU
    }
}
impl Float for f64 {
    #[inline]
    fn pi() -> Self {
        std::f64::consts::PI
    }
    #[inline]
    fn tau() -> Self {
        std::f64::consts::TAU
    }
}

//a Vector, SqMatrix, Quaternion
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
    #[must_use]
    fn from_array(data: [F; D]) -> Self;

    //fp zero
    /// Create a vector whose elements are all zero
    #[must_use]
    fn zero() -> Self;

    //fp into_array
    /// Create a vector from an array of [Float]
    #[must_use]
    fn into_array(self) -> [F; D];

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
    #[must_use]
    fn mix(self, other: &Self, t: F) -> Self;

    //mp dot
    /// Return the dot product of two vectors
    fn dot(&self, other: &Self) -> F;

    //mp length_sq
    /// Return the square of the length of the vector
    #[inline]
    fn length_sq(&self) -> F {
        self.dot(self)
    }

    //mp length
    /// Return the length of the vector
    #[inline]
    fn length(&self) -> F {
        self.length_sq().sqrt()
    }

    //mp distance_sq
    /// Return the square of the distance between this vector and another
    #[inline]
    fn distance_sq(&self, other: &Self) -> F {
        (*self - *other).length_sq()
    }

    //mp distance
    /// Return the distance between this vector and another
    #[inline]
    fn distance(&self, other: &Self) -> F {
        self.distance_sq(other).sqrt()
    }

    //mp normalize
    /// Normalize the vector; if its length is close to zero, then set it to be zero
    #[inline]
    #[must_use]
    fn normalize(mut self) -> Self {
        let l = self.length();
        if l < F::epsilon() {
            self.set_zero()
        } else {
            self /= l
        }
        self
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
    #[must_use]
    fn from_array(data: [F; D2]) -> Self;

    //fp into_array
    /// Create a vector from an array of [Float]
    #[must_use]
    fn into_array(self) -> [F; D2];

    //fp identity
    /// Create an identity [SqMatrix]
    #[must_use]
    fn identity() -> Self;

    //fp zero
    /// Create a zero [SqMatrix]
    #[must_use]
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
    fn transform(&self, v: &V) -> V;
}

//tt Vector3
/// The [Vector3] trait describes a 3-dimensional vector of [Float]
///
pub trait Vector3<F: Float>: Vector<F, 3> {
    /// Cross product of two 3-element vectors
    #[must_use]
    fn cross_product(&self, other: &Self) -> Self {
        Self::from_array(vector::cross_product3(self.as_ref(), other.as_ref()))
    }
    /// Get a point on a sphere uniformly distributed for a point
    /// where x in [0,1) and y in [0,1)
    #[must_use]
    fn uniform_dist_sphere3(x: [F; 2], map: bool) -> Self {
        Self::from_array(vector::uniform_dist_sphere3(x, map))
    }
}

//tt SqMatrix3
/// The [SqMatrix3] trait describes a 3-dimensional square matrix of [Float] type that operates on a [Vector].
///
pub trait SqMatrix3<V3: Vector<F, 3>, F: Float>: SqMatrix<V3, F, 3, 9> {
    // fn invert(&mut self);
    // fn inverse(&self) -> Self;
}

//tt SqMatrix4
/// The [SqMatrix4] trait describes a 4-dimensional square matrix of [Float] type that operates on a [Vector].
///
pub trait SqMatrix4<F: Float, V3: Vector<F, 3>, V4: Vector<F, 4>>: SqMatrix<V4, F, 4, 16> {
    // fn invert(&mut self);
    // fn inverse(&self) -> Self;
    /// Generate a perspective matrix
    fn perspective(fov: F, aspect: F, near: F, far: F) -> Self;

    /// Generate a matrix that represents a 'look at a vector'
    fn look_at(eye: &V3, center: &V3, up: &V3) -> Self;

    /// Translate the matrix by a Vec3
    fn translate3(&mut self, by: &V3);

    /// Translate the matrix by a Vec4
    fn translate4(&mut self, by: &V4);
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
    + std::ops::AddAssign<Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::SubAssign<Self>
    // scale
    + std::ops::Mul<F, Output = Self>
    + std::ops::MulAssign<F>
    + std::ops::Div<F, Output = Self>
    + std::ops::DivAssign<F>
    // apply to self
    + std::ops::Mul<Self, Output = Self>
    + std::ops::MulAssign<Self>
    + std::ops::Div<Self, Output = Self>
    + std::ops::DivAssign<Self>
    // apply to V3 - cannot support this as we already have F as RHS of Mul - can only have one trait there
    // + std::ops::Mul<V3, Output = V3>
where V3:Vector<F,3>, V4:Vector<F,4>, F:Float
{
    //cp from_array
    /// Create a quaternion from an array of [Float]
    ///
    /// The order must be [i, j, k, r]
    #[must_use]
    fn from_array(data:[F;4]) -> Self;

    //cp of_rijk
    /// Create from r, i, j, k
    #[must_use]
    fn of_rijk(r:F, i:F, j:F, k:F) -> Self;

    //cp conjugate
    /// Create the conjugate of a quaternion
    #[must_use]
    #[inline]
    fn conjugate(self) -> Self {
        let (r,i,j,k) = self.as_rijk();
        Self::of_rijk(r,-i,-j,-k)
    }

    //cp unit
    /// Create a quaternion whose elements are all zero
    #[must_use]
    fn unit() -> Self;

    //cp of_axis_angle
    /// Create a unit quaternion for a rotation of an angle about an axis
    #[must_use]
    fn of_axis_angle(axis:&V3, angle:F) -> Self {
        Self::from_array(quat::of_axis_angle(axis.as_ref(), angle))
    }

    //cp rotate_x
    /// Apply a rotation about the X-axis to this quaternion
    #[inline]
    #[must_use]
    fn rotate_x(self, angle: F) -> Self {
        Self::from_array(quat::rotate_x(self.as_ref(), angle))
    }

    //cp rotate_y
    /// Apply a rotation about the Y-axis to this quaternion
    #[inline]
    #[must_use]
    fn rotate_y(self, angle: F) -> Self {
        Self::from_array(quat::rotate_y(self.as_ref(), angle))
    }

    //cp rotate_z
    /// Apply a rotation about the Z-axis to this quaternion
    #[inline]
    #[must_use]
    fn rotate_z(self, angle: F) -> Self {
        Self::from_array(quat::rotate_z(self.as_ref(), angle))
    }

    //cp of_rotation3
    /// Find the unit quaternion of a Matrix3 assuming it is purely a rotation
    #[must_use]
    fn of_rotation3<M> (rotation:&M) -> Self
    where M:SqMatrix<V3, F, 3, 9>;

    //cp look_at
    /// Create a quaternion that maps a unit V3 of dirn to (0,0,-1) and a unit V3 of up (if perpendicular to dirn) to (0,1,0)
    #[must_use]
    fn look_at(dirn:&V3, up:&V3) -> Self {
        Self::from_array(quat::look_at(dirn.as_ref(), up.as_ref()))
    }

    //cp rotation_of_vec_to_vec
    /// Get a quaternion that is a rotation of one vector to another
    ///
    /// The vectors must be unit vectors
    #[must_use]
    fn rotation_of_vec_to_vec(a: &V3, b: &V3) -> Self {
        Self::from_array(quat::rotation_of_vec_to_vec(a.as_ref(), b.as_ref()))
    }

    //cp weighted_average_pair
    /// Calculate the weighted average of two unit quaternions
    ///
    /// w_a + w_b must be 1.
    ///
    /// See http://www.acsu.buffalo.edu/~johnc/ave_quat07.pdf
    /// Averaging Quaternions by F. Landis Markley
    #[must_use]
    fn weighted_average_pair(&self, w_a: F, qb: &Self, w_b: F) -> Self {
        Self::from_array(quat::weighted_average_pair(self.as_ref(), w_a, qb.as_ref(), w_b))
    }

    //cp weighted_average_many
    /// Calculate the weighted average of many unit quaternions
    ///
    /// weights need not add up to 1
    ///
    /// This is an approximation compared to the Landis Markley paper
    #[must_use]
    fn weighted_average_many<I: Iterator<Item = (F, Self)>>(value_iter:I) -> Self {
        let value_iter = value_iter.map(|(w,v)| (w,v.into_array()));
        Self::from_array(quat::weighted_average_many(value_iter))
    }

    //mp into_array
    /// Create an array [Float] for the fquaternion in order i, j, k, r
    #[must_use]
    fn into_array(self) -> [F;4];

    //fp as_rijk
    /// Break out into r, i, j, k
    fn as_rijk(&self) -> (F, F, F, F);

    //fp as_axis_angle
    /// Find the axis and angle of rotation for a (non-unit) quaternion
    fn as_axis_angle(&self) -> (V3, F) {
        let (axis, angle) = quat::as_axis_angle(self.as_ref());
        (V3::from_array(axis), angle)
    }

    //mp set_zero
    /// Set the quaternion to be all zeros
    fn set_zero(&mut self);

    //mp mix
    /// Create a linear combination of this [Quaternion] and another using parameter `t` from zero to one
    #[must_use]
    fn mix(self, other:&Self, t:F) -> Self;

    //mp dot
    /// Return the dot product of two quaternions; basically used for length
    #[must_use]
    fn dot(self, other:&Self) -> F;

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
    #[must_use]
    fn normalize(mut self) -> Self {
        let l = self.length();
        if l < F::epsilon() {self.set_zero()} else {self /= l}
        self
    }

    //fp set_rotation3
    /// Set a Matrix3 to be the rotation matrix corresponding to the unit quaternion
    fn set_rotation3<M> (&self, m:&mut M)
    where M:SqMatrix<V3, F, 3, 9>;

    //fp set_rotation4
    /// Set a Matrix4 to be the rotation matrix corresponding to the unit quaternion
    fn set_rotation4<M> (&self, m:&mut M)
    where M:SqMatrix<V4, F, 4, 16>;

    //fp apply3
    /// Apply the quaternion to a V3
    #[must_use]
    fn apply3(self, other: &V3) -> V3 {
        let data = quat::apply3(self.as_ref(), other.as_ref());
        V3::from_array(data)
    }

    //fp apply4
    /// Apply the quaternion to a V4
    #[must_use]
    fn apply4(self, other: &V4) -> V4 {
        let data = quat::apply4(self.as_ref(), other.as_ref());
        V4::from_array(data)
    }

    //zz All done
}

//tt Transform
/// The [Transform] trait describes a translation, rotation and
/// scaling for 3D, represented eventually as a Mat4
///
/// A transformation that is a translation . scaling . rotation
/// (i.e. it applies the rotation to an object, then scales it, then
/// translates it)
pub trait Transform<F, V3, V4, M4, Q>:
    Clone + Copy + std::fmt::Debug + std::fmt::Display + std::default::Default
// + std::ops::Neg<Output = Self>
// apply to self - this is possible
// + std::ops::Mul<Self, Output = Self>
// + std::ops::MulAssign<Self>
// + std::ops::Div<Self, Output = Self>
// + std::ops::DivAssign<Self>
// translation of self - can only choose one of V3 or V4
// + std::ops::Add<V3, Output = Self>
// + std::ops::AddAssign<V3>
// + std::ops::Sub<V3, Output = Self>
// + std::ops::SubAssign<V3>
// + std::ops::Add<V4, Output = Self>
// + std::ops::AddAssign<V4>
// + std::ops::Sub<V4, Output = Self>
// + std::ops::SubAssign<V4>
// scaling
// + std::ops::Mul<F, Output = Self>
// + std::ops::MulAssign<F>
// + std::ops::Div<F, Output = Self>
// + std::ops::DivAssign<F>
// rotation
// + std::ops::Mul<Q, Output = Self>
// + std::ops::MulAssign<Q>
// + std::ops::Div<Q, Output = Self>
// + std::ops::DivAssign<Q>
// and probably where Q:std::ops::Mul<Self, Output=Self> etc
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
    M4: SqMatrix4<F, V3, V4>,
    Q: Quaternion<F, V3, V4>,
{
    /// Create a transformation that is a translation, rotation and scaling
    fn of_trs(t: V3, r: Q, s: F) -> Self;
    /// Get the scale of the transform
    fn scale(&self) -> F;
    /// Get a translation by a vector
    fn translation(&self) -> V3;
    /// Get the rotation of the transfirnatuib
    fn rotation(&self) -> Q;
    /// Get the inverse transformation
    fn inverse(&self) -> Self;
    /// Invert the transformation
    fn invert(&mut self);
    /// Convert it to a 4-by-4 matrix
    fn as_mat(&self) -> M4;
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
    /// The type of a 3D vector with an additional '1' expected in its extra element if it is a position
    type Vec4: Vector<Scalar, 4>;
    /// The type of a 3D matrix that can transform Vec3
    type Mat3: SqMatrix3<Self::Vec3, Scalar>;
    /// The type of a 3D matrix which allows for translations, that can transform Vec4
    type Mat4: SqMatrix4<Scalar, Self::Vec3, Self::Vec4>;
    /// The quaternion type that provides for rotations in 3D
    type Quat: Quaternion<Scalar, Self::Vec3, Self::Vec4>;
    /// The transform type
    type Trans: Transform<Scalar, Self::Vec3, Self::Vec4, Self::Mat4, Self::Quat>;
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
