use crate::{FArray, matrix, vector};
use crate::{Float, SqMatrix, Vector};

//a FArray2
//tp FArray2
/// The [FArray2] is a wrapper around a `D2 = D`^2` sized array of [Float]s.
///
/// It provides implementations of the traits required for a [SqMatrix]
/// trait operating on an [FArray] of dimesion D.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct FArray2<F: Float, const D: usize, const D2: usize> {
    data: [F; D2],
}

//a Macros
//mi farray2_basic_traits!
macro_rules! farray2_basic_traits {
    { $f:ty,  $d:expr,  $d2:expr, $ty:ty } => {

        //ip Default for FArray
        impl std::default::Default for $ty {
            fn default() -> Self {
                Self {
                    data: vector::zero(),
                }
            }
        }

        //ip Display for FArray
        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                vector::fmt(f, &self.data)
            }
        }
    }
}

//mi farray2_mul_traits!
macro_rules! farray2_mul_traits {
    { $f:ty,  $d:expr,  $d2:expr, $ty:ty } => {
        //ip FArray2
        //ip Mul, MulAssign <Self> for FArray2
        impl std::ops::Mul<Self> for $ty {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                matrix::multiply::<$f, $d2, $d2, $d2, $d, $d, $d>(&self.data, &other.data).into()
            }
        }
        impl std::ops::MulAssign<Self> for $ty {
            fn mul_assign(&mut self, other: Self) {
                *self = *self * other;
            }
        }
    }
}

//ip SqMatrix<F,2,4> for FArray2
//mi farray2_sqmatrix_trait!
macro_rules! farray2_sqmatrix_trait {
    { $f:ty,  $d:expr,  $d2:expr, $ty:ty, $det_fn:expr, $inv_fn:expr } => {

        impl SqMatrix<$f, $d, $d2> for $ty
        where
            FArray<$f, $d>: Vector<$f, $d>,
        {
            fn transpose(&self) -> Self {
                matrix::transpose::<$f, $d2, $d, $d>(self.data).into()
            }
            fn transform<T>(&self, v: &T) -> T
            where
                T: std::ops::Deref<Target = [$f; $d]>,
                T: From<[$f; $d]>
            {
                matrix::multiply::<$f, $d2, $d, $d, $d, $d, 1>(&self.data, v).into()
            }
            fn transform_arr(&self, v: &[$f; $d]) -> [$f; $d]
            {
                matrix::multiply::<$f, $d2, $d, $d, $d, $d, 1>(&self.data, v).into()
            }
            fn determinant(&self) -> $f {
                $det_fn (&self.data)
            }
            fn inverse(&self) -> Self {
                $inv_fn (&self.data).into()
            }
        }
    }
}

//mi farray2_traits
macro_rules! farray2_traits {
    { $f:ty,  $d:expr,  $d2:expr, $ty:ty, $det_fn:expr, $inv_fn:expr } => {
        farray2_basic_traits! {$f, $d, $d2, $ty}

        crate::ref_traits!{$f, $d2, $ty}
        crate::convert_traits!{$f, $d2, $ty}
        crate::serialize_traits!{$f, $d2, $ty}
        crate::unary_traits!{$f, $d2, $ty}
        crate::elementwise_traits!{$f, $d2, $ty, Add, add, +, AddAssign, add_assign, +=}
        crate::elementwise_traits!{$f, $d2, $ty, Sub, sub, -, SubAssign, sub_assign, -=}
        crate::scale_by_f_traits!{$f, $ty, Mul, mul, *, MulAssign, mul_assign, *=}
        crate::scale_by_f_traits!{$f, $ty, Div, div, /, DivAssign, div_assign, /=}

        farray2_mul_traits! {$f, $d, $d2, $ty}

        farray2_sqmatrix_trait! {$f, $d, $d2, $ty, $det_fn, $inv_fn}
    }
}

macro_rules! farray2_4_mul_vecs {
    { $f:ty,  $ty:ty } => {
        impl std::ops::Mul<FArray<$f, 4>> for $ty {
            type Output = FArray<$f, 4>;
            fn mul(self, v: FArray<$f, 4>) -> Self::Output {
                self.transform_arr(&v).into()
            }
        }
    }
}

//a Use macros
farray2_traits! {f32,2,4,FArray2<f32,2,4>, matrix::determinant2, matrix::inverse2}
farray2_traits! {f32,3,9,FArray2<f32,3,9>, matrix::determinant3, matrix::inverse3}
farray2_traits! {f32,4,16,FArray2<f32,4,16>, matrix::determinant4, matrix::inverse4}

farray2_traits! {f64,2,4,FArray2<f64,2,4>, matrix::determinant2, matrix::inverse2}
farray2_traits! {f64,3,9,FArray2<f64,3,9>, matrix::determinant3, matrix::inverse3}
farray2_traits! {f64,4,16,FArray2<f64,4,16>, matrix::determinant4, matrix::inverse4}

farray2_4_mul_vecs! {f32, FArray2<f32,4,16> }
farray2_4_mul_vecs! {f64, FArray2<f64,4,16> }
