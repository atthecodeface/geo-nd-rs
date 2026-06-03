//a Imports
use crate::{matrix, quat, vector};

//a IsSquared
//tp IsSquared
/// This trait, with `D2 = D^2`, should be implemented for [();D] to
/// indicate that `D2` is indeed `D*D`
///
/// This permits the `SqMatrix` trait to only be implemented for actually square matrices
pub trait IsSquared<const D: usize, const D2: usize> {}
impl IsSquared<2, 4> for [(); 2] {}
impl IsSquared<3, 9> for [(); 3] {}
impl IsSquared<4, 16> for [(); 4] {}

//a Num and Float traits
//tp Num
/// The [Num] trait is required for matrix or vector elements; it is
/// not a float, and so some of the matrix and vector operations can
/// operate on integer types such as i32, i64 and isize; it can also
/// operate on rational numbers (such as num::Rational64)
///
/// The fundamental difference between this and Float is the lack of support
/// for functions such as sqrt(), cos(), abs() (!), powi(_), etc
///
/// The trait requires basic numeric operations, plus specifically [std::fmt::Display].
///
/// v0.7 added PartialOrd, FromPrimitve
pub trait Num:
    Copy
    + PartialEq
    + PartialOrd
    + std::fmt::Display
    + std::fmt::Debug
    + std::ops::Neg<Output = Self>
    + num_traits::Num
    + num_traits::ConstOne
    + num_traits::ConstZero
    + num_traits::FromPrimitive
{
}

//ip Num
/// Num is implemented for all types that support the traits
impl<T> Num for T where
    T: Copy
        + PartialEq
        + PartialOrd
        + std::fmt::Display
        + std::fmt::Debug
        + std::ops::Neg<Output = Self>
        + num_traits::Num
        + num_traits::ConstOne
        + num_traits::ConstZero
        + num_traits::FromPrimitive
{
}

//tp Float
/// The [Float] trait is required for matrix or vector elements which
/// have a float aspect, such as `sqrt`.
///
/// The trait is essentially `num_traits::Float`, but it supplies
/// implicit methods for construction of a [Float] from an `isize`
/// value, or as a rational from a pair of `isize` values.
pub trait Float: Num + num_traits::Float {
    /// Generate a value that is the fraction of a signed integer numerator and an unsigned integer denominator
    fn frac(n: i32, d: u32) -> Self {
        Self::from_i32(n).unwrap() / Self::from_u32(d).unwrap()
    }
}

impl<T> Float for T where T: Num + num_traits::Float {}

//a Array and Quat traits
//tp ArrayBasic
pub trait ArrayBasic:
    Clone + Copy + std::fmt::Debug + std::fmt::Display + std::default::Default + PartialEq<Self>
{
}

//ip ArrayBasic
impl<T> ArrayBasic for T where
    T: Clone + Copy + std::fmt::Debug + std::fmt::Display + std::default::Default + PartialEq<Self>
{
}

//tp ArrayRef
pub trait ArrayRef<F, const D:usize>:
    std::convert::AsRef<[F; D]> // Note, [F;D] implements AsRef only for [F] which is an issue
    + std::convert::AsMut<[F; D]> // Note, [F;D] implements AsRef only for [F] which is an issue
    + std::ops::Deref<Target = [F;D]>
    + std::ops::DerefMut
{
}

//ip ArrayRef
impl<T, F, const D: usize> ArrayRef<F, D> for T where
    T: std::convert::AsRef<[F; D]> // Note, [F;D] implements AsRef only for [F] which is an issue
        + std::convert::AsMut<[F; D]>
        // Note, [F;D] implements AsRef only for [F] which is an issue
        + std::ops::Deref<Target = [F; D]>
        + std::ops::DerefMut
{
}

//tp ArrayIndex
pub trait ArrayIndex<F>: std::ops::Index<usize, Output = F> + std::ops::IndexMut<usize> {}

//ip ArrayIndex
impl<T, F> ArrayIndex<F> for T where
    T: std::ops::Index<usize, Output = F> + std::ops::IndexMut<usize>
{
}

//ip ArrayConvert
pub trait ArrayConvert<F, const D: usize>:
    std::convert::From<[F; D]>
    + for<'a> std::convert::From<&'a [F; D]>
    + for<'a> std::convert::TryFrom<&'a [F]>
    + for<'a> std::convert::TryFrom<Vec<F>>
    + std::convert::Into<[F; D]>
{
}

//ip ArrayConvert
impl<T, F, const D: usize> ArrayConvert<F, D> for T where
    T: std::convert::From<[F; D]>
        + for<'a> std::convert::From<&'a [F; D]>
        + for<'a> std::convert::TryFrom<&'a [F]>
        + for<'a> std::convert::TryFrom<Vec<F>>
        + std::convert::Into<[F; D]>
{
}

//tt ArrayAddSubNeg
pub trait ArrayAddSubNeg<F, const D: usize>:
    Sized
    + std::ops::Neg<Output = Self>
    + std::ops::Add<Self, Output = Self>
    + for<'a> std::ops::Add<&'a [F; D], Output = Self>
    + for<'a> std::ops::Add<&'a Self, Output = Self>
    + std::ops::AddAssign<Self>
    + for<'a> std::ops::AddAssign<&'a [F; D]>
    + std::ops::Sub<Self, Output = Self>
    + for<'a> std::ops::Sub<&'a [F; D], Output = Self>
    + for<'a> std::ops::Sub<&'a Self, Output = Self>
    + std::ops::SubAssign<Self>
    + for<'a> std::ops::SubAssign<&'a [F; D]>
{
}
//tt ArrayAddSubNeg
impl<T, F, const D: usize> ArrayAddSubNeg<F, D> for T where
    T: Sized
        + std::ops::Neg<Output = Self>
        + std::ops::Add<Self, Output = Self>
        + for<'a> std::ops::Add<&'a [F; D], Output = Self>
        + for<'a> std::ops::Add<&'a Self, Output = Self>
        + std::ops::AddAssign<Self>
        + for<'a> std::ops::AddAssign<&'a [F; D]>
        + std::ops::Sub<Self, Output = Self>
        + for<'a> std::ops::Sub<&'a [F; D], Output = Self>
        + for<'a> std::ops::Sub<&'a Self, Output = Self>
        + std::ops::SubAssign<Self>
        + for<'a> std::ops::SubAssign<&'a [F; D]>
{
}

//tp ArrayScale
pub trait ArrayScale<F>:
    std::ops::Mul<F, Output = Self>
    + std::ops::MulAssign<F>
    + std::ops::Div<F, Output = Self>
    + std::ops::DivAssign<F>
{
}

//ip ArrayScale
impl<T, F> ArrayScale<F> for T where
    T: std::ops::Mul<F, Output = Self>
        + std::ops::MulAssign<F>
        + std::ops::Div<F, Output = Self>
        + std::ops::DivAssign<F>
{
}

//tp ArrayMulDiv - not used yet
#[allow(dead_code)]
pub trait ArrayMulDiv<F, const D: usize>:
    Sized
    + std::ops::Mul<Self, Output = Self>
    + for<'a> std::ops::Mul<&'a [F; 4], Output = Self>
    + for<'a> std::ops::Mul<&'a Self, Output = Self>
    + std::ops::MulAssign<Self>
    + for<'a> std::ops::MulAssign<&'a [F; 4]>
    + std::ops::Div<Self, Output = Self>
    + for<'a> std::ops::Div<&'a [F; 4], Output = Self>
    + for<'a> std::ops::Div<&'a Self, Output = Self>
    + std::ops::DivAssign<Self>
    + for<'a> std::ops::DivAssign<&'a [F; 4]>
{
}

//ip ArrayMulDiv
impl<T, F, const D: usize> ArrayMulDiv<F, D> for T where
    T: Sized
        + std::ops::Mul<Self, Output = Self>
        + for<'a> std::ops::Mul<&'a [F; 4], Output = Self>
        + for<'a> std::ops::Mul<&'a Self, Output = Self>
        + std::ops::MulAssign<Self>
        + for<'a> std::ops::MulAssign<&'a [F; 4]>
        + std::ops::Div<Self, Output = Self>
        + for<'a> std::ops::Div<&'a [F; 4], Output = Self>
        + for<'a> std::ops::Div<&'a Self, Output = Self>
        + std::ops::DivAssign<Self>
        + for<'a> std::ops::DivAssign<&'a [F; 4]>
{
}

//tp QuatMulDiv
// Neat trick
//
// pub trait RefCanBeMultipliedBy<T> {}
// impl<X, T> RefCanBeMultipliedBy<T> for X where for<'a> &'a X: std::ops::Mul<T, Output = X> {}
pub trait QuatMulDiv<F, const D: usize>:
    Sized
    + std::ops::Mul<Self, Output = Self>
    + for<'a> std::ops::Mul<&'a Self, Output = Self>
    + std::ops::MulAssign<Self>
    + std::ops::Div<Self, Output = Self>
    + for<'a> std::ops::Div<&'a Self, Output = Self>
    + std::ops::DivAssign<Self>
{
}

//ip QuatMulDiv
impl<T, F, const D: usize> QuatMulDiv<F, D> for T where
    T: Sized
        + std::ops::Mul<Self, Output = Self>
        + for<'a> std::ops::Mul<&'a Self, Output = Self>
        + std::ops::MulAssign<Self>
        + std::ops::Div<Self, Output = Self>
        + for<'a> std::ops::Div<&'a Self, Output = Self>
        + std::ops::DivAssign<Self>
{
}

//a Vector, Vector2, Vector3, Vector4
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
///
pub trait Vector<F: Float, const D: usize>:
    ArrayBasic
    + ArrayRef<F, D> // Can we move this to specifically Vector3 and Vector4? Maybe just the asref?
    + ArrayIndex<F>
    + ArrayConvert<F, D>
    + ArrayAddSubNeg<F, D>
    + ArrayScale<F>
{
    /// Return true if the vector is all zeros
    fn set_zero(&mut self) {
        *self = [F::ZERO;D].into();
    }

    /// Return true if the vector is all zeros
    fn is_zero(&self) -> bool {
        !self.deref().iter().any(|f| !f.is_zero())
    }

    /// Return the dot product of two vectors
    #[inline]
    fn dot(&self, other: &[F; D]) -> F {
        vector::dot(self.deref(), other)
    }

    /// Return the sum of the components
    fn reduce_sum(&self) -> F {
        let mut r = F::zero();
        for d in self.deref() {
            r = r + *d
        }
        r
    }

    /// Return the square of the length of the vector
    #[inline]
    fn length_sq(&self) -> F {
        self.dot(self)
    }

    /// Return the length of the vector
    #[inline]
    fn length(&self) -> F {
        self.length_sq().sqrt()
    }

    /// Normalize the vector; if its length is close to zero, then set it to be zero
    #[inline]
    #[must_use]
    fn normalize(mut self) -> Self {
        let l = self.length();
        if l < F::epsilon() {
            self = Self::default()
        } else {
            self /= l
        }
        self
    }

    /// Return the square of the distance between this vector and another
    #[inline]
    fn distance_sq(&self, other: &[F; D]) -> F {
        (*self - other).length_sq()
    }

    /// Return the distance between this vector and another
    #[inline]
    fn distance(&self, other: &[F; D]) -> F {
        self.distance_sq(other).sqrt()
    }

    /// Create a linear combination of this [Vector] and another using parameter `t` from zero to one
    #[must_use]
    fn mix<A>(self, other: A, t: F) -> Self
    where
        A: std::ops::Deref<Target = [F; D]>,
    {
        vector::mix(self.deref(), other.deref(), t).into()
    }

    /// Create a linear combination of a number of vectors given a scale for each
    ///
    /// If the lengths of the slices provided differ, then the entries in the
    /// longer slice (without pairs to go with) are ignored
    #[must_use]
    fn sum_scaled<A>(v:&[A], scales:&[F]) -> Self
    where
        A: std::ops::Deref<Target = [F; D]>,
    {
        scales
            .iter()
            .zip(v.iter())
            .fold([F::ZERO; D], |mut acc, (s, v)| {
                for (a, v) in acc.iter_mut().zip(v.iter()) {
                    *a = *a + *v * *s;
                }
                acc
            }).into()
    }

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

    //cp cross_product - where D = 3
    /// Cross product of two 3-element vectors
    #[must_use]
    fn cross_product(&self, other: &[F; 3]) -> Self
    where
        Self: From<[F; 3]>,
        Self: AsRef<[F; 3]>, // so that it knows as_ref() returns &[F;3], i.e. D is 3
    {
        vector::cross_product3(self.as_ref(), other).into()
    }

    //fp apply_q3
    /// Apply a quaternion to a V3
    ///
    /// This can either take other as &\[F;3\] and produce \[F; 3\], or
    ///  &D where D:Deref<Target = \[F; 3\]> and D:From<\[F; 3\]
    ///
    /// If it takes the former then it can operate on \[F;3\] and
    /// anything that is Deref<Target = \[F;3\]>, but it needs its result
    /// cast into the correct vector
    ///
    /// If it tkes the latter then it cannot operate on \[F;3\], but its
    /// result need not be cast
    #[must_use]
    fn apply_q3<Q>(&self, q: &Q) -> Self
    where
        Q: Quaternion<F>,
        Self: From<[F; 3]>,  // Enforce D = 3
        Self: AsRef<[F; 3]>, // so that it knows as_ref() returns &[F;3], i.e. D is 3
    {
        quat::apply3(q.deref(), self.as_ref()).into()
    }

    //fp apply_q4
    /// Apply a quaternion to a V4
    ///
    /// This can either take other as &\[F;3\] and produce \[F; 3\], or
    ///  &D where D:Deref<Target =\[F; 3\]> and D:From<\[F; 3\]
    ///
    /// If it takes the former then it can operate on \[F;3\] and
    /// anything that is Deref<Target = \[F;3\]>, but it needs its result
    /// cast into the correct vector
    ///
    /// If it tkes the latter then it cannot operate on \[F;3\], but its
    /// result need not be cast
    #[must_use]
    fn apply_q4<Q>(&self, q: &Q) -> Self
    where
        Q: Quaternion<F>,
        Self: From<[F; 4]>,
        Self: AsRef<[F; 4]>, // so that it knows as_ref() returns &[F;3], i.e. D is 3
    {
        quat::apply4(q.deref(), self.as_ref()).into()
    }

    //mp transformed_by_m
    /// Multiply the vector by the matrix to transform it
    fn transformed_by_m<const D2: usize>(&mut self, m: &[F; D2]) -> &mut Self
    where
        [(); D]: IsSquared<D, D2>,
    {
        *self = matrix::multiply::<F, D2, D, D, D, D, 1>(m, self.deref()).into();
        self
    }

    //cp uniform_dist_sphere3
    /// Get a point on a sphere uniformly distributed for a point
    /// where x in [0,1) and y in [0,1)
    #[must_use]
    fn uniform_dist_sphere3(x: [F; 2], map: bool) -> Self
    where
        Self: From<[F; 3]>,
    {
        (vector::uniform_dist_sphere3(x, map)).into()
    }
}

//tt Vector2
/// The [Vector2] trait describes a 3-dimensional vector of [Float]
///
pub trait Vector2<F: Float>: Vector<F, 2> {}
impl<F, V> Vector2<F> for V
where
    F: Float,
    V: Vector<F, 2>,
{
}

//tt Vector3
/// The [Vector3] trait describes a 3-dimensional vector of [Float]
///
pub trait Vector3<F: Float>: Vector<F, 3> {}
impl<F, V> Vector3<F> for V
where
    F: Float,
    V: Vector<F, 3>,
{
}

//tt Vector4
/// The [Vector4] trait describes a 3-dimensional vector of [Float]
///
pub trait Vector4<F: Float>: Vector<F, 4> {}
impl<F, V> Vector4<F> for V
where
    F: Float,
    V: Vector<F, 4>,
{
}

//a SqMatrix
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
pub trait SqMatrix<F: Float, const D: usize, const D2: usize>:
    ArrayBasic
    + ArrayRef<F, D2>
    + ArrayIndex<F>
    + ArrayConvert<F, D2>
    + ArrayAddSubNeg<F, D2>
    + ArrayScale<F>
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
{
    /// Create an identity matrix
    fn identity() -> Self {
        matrix::identity::<F, D2, D>().into()
    }

    /// Return true if the matrix is zero
    fn is_zero(&self) -> bool {
        vector::is_zero(self.deref())
    }

    /// Set the matrix to zero
    fn set_zero(&mut self) -> &mut Self {
        vector::set_zero(self.deref_mut());
        self
    }

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
    fn transform<T>(&self, v: &T) -> T
    where
        T: std::ops::Deref<Target = [F; D]>,
        T: From<[F; D]>;
}

//tt SqMatrix2
/// The [SqMatrix2] trait describes a 2-dimensional vector of [Float]
///
pub trait SqMatrix2<F: Float>: SqMatrix<F, 2, 4> {}
impl<F, M> SqMatrix2<F> for M
where
    F: Float,
    M: SqMatrix<F, 2, 4>,
{
}

//tt SqMatrix3
/// The [SqMatrix3] trait describes a 2-dimensional vector of [Float]
///
pub trait SqMatrix3<F: Float>: SqMatrix<F, 3, 9> {}
impl<F, M> SqMatrix3<F> for M
where
    F: Float,
    M: SqMatrix<F, 3, 9>,
{
}

//tt SqMatrix4
/// The [SqMatrix4] trait describes a 2-dimensional vector of [Float]
///
pub trait SqMatrix4<F: Float>: SqMatrix<F, 4, 16> {
    /// Generate a perspective matrix
    fn perspective(fov: F, aspect: F, near: F, far: F) -> Self {
        matrix::perspective4(fov, aspect, near, far).into()
    }

    /// Generate a matrix that represents a 'look at a vector'
    fn look_at(eye: &[F; 3], center: &[F; 3], up: &[F; 3]) -> Self {
        matrix::look_at4(eye, center, up).into()
    }

    /// Scale the matrix for a transformation - only update top *3* rows
    fn scale3(&mut self, by: F) {
        for d in self.iter_mut().take(12) {
            *d = *d * by;
        }
    }
    /// Translate the matrix by a Vec3
    fn translate3(&mut self, by: &[F; 3]) {
        self[3] = self[3] + by[0];
        self[7] = self[7] + by[1];
        self[11] = self[11] + by[2];
    }

    /// Translate the matrix by a Vec4
    fn translate4(&mut self, by: &[F; 4]) {
        self[3] = self[3] + by[0];
        self[7] = self[7] + by[1];
        self[11] = self[11] + by[2];
    }
}
impl<F, M> SqMatrix4<F> for M
where
    F: Float,
    M: SqMatrix<F, 4, 16>,
{
}

//a Quaternion
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
pub trait Quaternion<F: Float>:
    ArrayBasic
    + ArrayRef<F, 4>
    + ArrayIndex<F>
    + ArrayConvert<F, 4>
    + ArrayAddSubNeg<F, 4>
    + QuatMulDiv<F, 4>
    + ArrayScale<F>
{
    /// Create from r, i, j, k
    #[must_use]
    fn of_rijk(r: F, i: F, j: F, k: F) -> Self;

    /// Create the conjugate of a quaternion
    #[must_use]
    #[inline]
    fn conjugate(self) -> Self {
        let (r, i, j, k) = self.as_rijk();
        Self::of_rijk(r, -i, -j, -k)
    }

    //fp as_rijk
    /// Break out into r, i, j, k
    fn as_rijk(&self) -> (F, F, F, F);

    //fp as_axis_angle
    /// Find the axis and angle of rotation for a (non-unit) quaternion
    fn as_axis_angle<V: From<[F; 3]>>(&self) -> (V, F) {
        let (axis, angle) = quat::as_axis_angle(self.as_ref());
        (axis.into(), angle)
    }

    /// Set to an identity transformation
    fn set_identity(&mut self) {
        *self = Self::default();
    }

    /// Set the quaternion to be all zeros
    fn set_zero(&mut self) {
        *self = [F::ZERO; 4].into();
    }

    /// Create a linear combination of this [Quaternion] and another using parameter `t` from zero to one
    #[must_use]
    fn mix(self, other: &[F; 4], t: F) -> Self {
        vector::mix(self.deref(), other, t).into()
    }

    /// Return the dot product of two quaternions; basically used for length
    fn dot(self, other: &Self) -> F {
        vector::dot(self.deref(), other.deref())
    }

    /// Return the square of the length of the quaternion
    fn length_sq(&self) -> F {
        self.dot(self)
    }

    /// Return the length of the quaternion
    fn length(&self) -> F {
        self.length_sq().sqrt()
    }

    /// Return the square of the distance between this quaternion and another
    fn distance_sq(&self, other: &Self) -> F {
        quat::distance_sq(self, other)
    }

    /// Return the distance between this quaternion and another
    fn distance(&self, other: &Self) -> F {
        self.distance_sq(other).sqrt()
    }

    /// Normalize the quaternion; if its length is close to zero, then set it to be zero
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

    /// Create a unit quaternion for a rotation of an angle about an axis
    #[must_use]
    fn of_axis_angle(axis: &[F; 3], angle: F) -> Self {
        quat::of_axis_angle(axis, angle).into()
    }

    /// Apply a rotation about the X-axis to this quaternion
    #[inline]
    #[must_use]
    fn rotate_x(self, angle: F) -> Self {
        quat::rotate_x(self.as_ref(), angle).into()
    }

    /// Apply a rotation about the Y-axis to this quaternion
    #[inline]
    #[must_use]
    fn rotate_y(self, angle: F) -> Self {
        quat::rotate_y(self.as_ref(), angle).into()
    }

    /// Apply a rotation about the Z-axis to this quaternion
    #[inline]
    #[must_use]
    fn rotate_z(self, angle: F) -> Self {
        quat::rotate_z(self.as_ref(), angle).into()
    }

    /// Create a quaternion that maps a unit V3 of dirn to (0,0,-1) and a unit
    /// V3 of up (if perpendicular to dirn) to (0,1,0)
    #[must_use]
    fn look_at(dirn: &[F; 3], up: &[F; 3]) -> Self {
        quat::look_at(dirn, up).into()
    }

    //cp rotation_of_vec_to_vec
    /// Get a quaternion that is a rotation of one vector to another
    ///
    /// The vectors must be unit vectors
    #[must_use]
    fn rotation_of_vec_to_vec(a: &[F; 3], b: &[F; 3]) -> Self {
        quat::rotation_of_vec_to_vec(a, b).into()
    }

    //cp mapping_vector_pair_to_vector_pair
    /// Create a quaternion that maps (v0, v1) to (w0, w1)
    ///
    /// This will map v0 to the Z axis, and the Z axis to w0. Applying
    /// both of these quaternions in succession maps v0 to w0.
    ///
    /// It maps v1 to v1' and w1 *backwards* to w1', where the
    /// difference between v1' and w1' should be *just* a rotation
    /// around the Z axis. Creating a rotation to do this is a third quaternion
    ///
    /// Then the three quaternions can be applied appropriately to map
    /// v0 to w0 (for sure), and v1 to w1 (if the intermediate
    /// rotation were just a Z-rotation)
    fn mapping_vector_pair_to_vector_pair(
        (di_m, dj_m): (&[F; 3], &[F; 3]),
        (di_c, dj_c): (&[F; 3], &[F; 3]),
    ) -> Self {
        let z_axis: [F; 3] = [F::zero(), F::zero(), F::one()];
        let qi_c = Self::rotation_of_vec_to_vec(di_c, &z_axis);
        let qi_m = Self::rotation_of_vec_to_vec(di_m, &z_axis);

        let dj_c_rotated = quat::apply3(&qi_c, dj_c);
        let dj_m_rotated = quat::apply3(&qi_m, dj_m);

        let theta_dj_m = dj_m_rotated[0].atan2(dj_m_rotated[1]);
        let theta_dj_c = dj_c_rotated[0].atan2(dj_c_rotated[1]);
        let theta = theta_dj_m - theta_dj_c;
        let theta_div_2 = theta / (F::one() + F::one());
        let cos_2theta = theta_div_2.cos();
        let sin_2theta = theta_div_2.sin();
        let q_z = Self::of_rijk(cos_2theta, F::zero(), F::zero(), sin_2theta);
        qi_c.conjugate() * q_z * qi_m
    }

    //cp weighted_average_pair
    /// Calculate the weighted average of two unit quaternions
    ///
    /// w_a + w_b must be 1.
    ///
    /// See <http://www.acsu.buffalo.edu/~johnc/ave_quat07.pdf>
    /// Averaging Quaternions by F. Landis Markley
    #[must_use]
    fn weighted_average_pair(&self, w_a: F, qb: &Self, w_b: F) -> Self {
        quat::weighted_average_pair(self.as_ref(), w_a, qb.as_ref(), w_b).into()
    }

    //cp weighted_average_many
    /// Calculate the weighted average of many unit quaternions
    ///
    /// weights need not add up to 1
    ///
    /// This is an approximation compared to the Landis Markley paper
    #[must_use]
    fn weighted_average_many<A: Into<[F; 4]>, I: Iterator<Item = (F, A)>>(value_iter: I) -> Self {
        let value_iter = value_iter.map(|(w, v)| (w, v.into()));
        quat::weighted_average_many(value_iter).into()
    }

    //cp of_rotation3
    /// Find the unit quaternion of a Matrix3 assuming it is purely a rotation
    #[must_use]
    fn of_rotation3<M>(rotation: &M) -> Self
    where
        M: SqMatrix3<F>;

    //fp set_rotation3
    /// Set a Matrix3 to be the rotation matrix corresponding to the unit quaternion
    fn set_rotation3<M>(&self, m: &mut M)
    where
        M: SqMatrix3<F>;

    //fp set_rotation4
    /// Set a Matrix4 to be the rotation matrix corresponding to the unit quaternion
    fn set_rotation4<M>(&self, m: &mut M)
    where
        M: SqMatrix4<F>;

    //fp apply3
    /// Apply the quaternion to a V3
    ///
    /// This can either take other as &\[F;3\] and produce \[F; 3\], or
    ///  &D where D:Deref<Target =\[F; 3\]> and D:From<\[F; 3\]>
    ///
    /// If it takes the former then it can operate on \[F;3\] and
    /// anything that is Deref<Target=\[F;3\]>, but it needs its result
    /// cast into the correct vector
    ///
    /// If it tkes the latter then it cannot operate on \[F;3\], but its
    /// result need not be cast
    #[must_use]
    fn apply3<T>(&self, other: &T) -> T
    where
        T: std::ops::Deref<Target = [F; 3]>,
        T: From<[F; 3]>,
    {
        quat::apply3(self.deref(), other.deref()).into()
    }

    /// Apply the quaternion to a [F; 3]
    ///
    /// See apply3 for why this is provided
    ///
    #[must_use]
    fn apply3_arr(&self, other: &[F; 3]) -> [F; 3] {
        quat::apply3(self.deref(), other)
    }

    //fp apply4
    /// Apply the quaternion to a V4
    #[must_use]
    fn apply4<T>(&self, other: &T) -> T
    where
        T: std::ops::Deref<Target = [F; 4]>,
        T: From<[F; 4]>,
    {
        quat::apply4(self.deref(), other.deref()).into()
    }

    /// Apply the quaternion to a [F; 3]
    ///
    /// See apply3 for why this is provided
    ///
    #[must_use]
    fn apply4_arr(&self, other: &[F; 4]) -> [F; 4] {
        quat::apply4(self.deref(), other)
    }

    //zz All done
}

//a Transform
//tt Transform
/// The [Transform] trait describes a translation, rotation and
/// scaling for 3D, represented eventually as a Mat4
///
/// A transformation that is a translation . scaling . rotation
/// (i.e. it applies the rotation to an object, then scales it, then
/// translates it)
pub trait Transform<F, V3, Q>:
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
    Q: Quaternion<F>,
{
    /// Create a transformation that is a translation, rotation and scaling
    fn of_trs<A: AsRef<[F; 3]>>(t: A, r: Q, s: A) -> Self;
    /// Create a transformation that is a translation, rotation and uniform scaling
    fn of_trsu<A: AsRef<[F; 3]>>(t: A, r: Q, s: F) -> Self;
    /// Get the scale of the transform in each of the axes
    fn scale(&self) -> V3;
    /// Get the scale of the transform in each of the axes
    fn uniform_scale(&self) -> Option<F>;
    /// Get a translation by a vector
    fn translation(&self) -> V3;
    /// Set to an identity transformation
    fn set_identity(&mut self) {
        self.set_uniform_scale(F::ONE);
        self.set_translation(V3::default());
        self.set_rotation(Q::default());
    }
    /// Get the rotation of the transformation
    fn rotation(&self) -> Q;
    /// Set the scale of the transformation
    fn set_scale<A: AsRef<[F; 3]>>(&mut self, scale: A);
    /// Set the scale of the transformation
    fn set_uniform_scale(&mut self, scale: F);
    /// Set the translation of the transformation
    fn set_translation<A: AsRef<[F; 3]>>(&mut self, translation: A);
    /// Set the rotation of a transformation
    fn set_rotation(&mut self, rotation: Q);
    /// Pre-apply a uniform scaling
    fn scale_by(&mut self, scale: F);
    /// Pre-apply a scaled translation to the transformation
    fn translate_by<A: AsRef<[F; 3]>>(&mut self, translation: A, scale: F);
    /// Rotate the transformation by a quaternion
    fn rotate_by(&mut self, quaternion: Q);
    /// Transform this transformation by another *UNIFORM* scale transformation;
    /// if is not uniform scaling then return false
    fn transform_by(&mut self, transformer: &Self) -> bool;
    /// Get the inverse transformation
    ///
    /// This is only possible if the scaling is uniform
    #[must_use]
    fn inverse(&self) -> Option<Self>;
    /// Invert the transformation
    ///
    /// This will return false if the scaling is not uniform
    fn invert(&mut self) -> bool;
    /// Convert it to a 4-by-4 matrix and set it
    fn set_mat<M: SqMatrix4<F>>(&self, mat4: &mut M) {
        *mat4 = self.as_mat();
    }
    /// Convert it to a 4-by-4 matrix
    #[must_use]
    fn as_mat<M: SqMatrix4<F>>(&self) -> M;
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
    type Mat3: SqMatrix3<Scalar>;
    /// The type of a 3D matrix which allows for translations, that can transform Vec4
    type Mat4: SqMatrix4<Scalar>;
    /// The quaternion type that provides for rotations in 3D
    type Quat: Quaternion<Scalar>;
    /// The transform type
    type Trans: Transform<Scalar, Self::Vec3, Self::Quat>;
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
    type Mat2: SqMatrix<Scalar, 2, 4>;
}
