//a Imports
use serde::{Deserialize, Serialize};

use crate::{matrix, vector, FArray};
use crate::{Float, SqMatrix, SqMatrix3, SqMatrix4, Vector};

//a Macros
//mi index_ops!
macro_rules! index_ops {
    { $t:ident } => {
        impl <F:Float, const D:usize, const D2:usize> std::ops::Index<usize> for $t <F, D, D2> {
            type Output = F;
            fn index(&self, index: usize) -> &F {
                let slice: &[_] = self.as_ref();
                &slice[index]
            }
        }
        impl <F:Float, const D:usize, const D2:usize> std::ops::IndexMut<usize> for $t <F, D, D2> {
            fn index_mut(&mut self, index: usize) -> &mut F {
                let slice: &mut [_] = self.as_mut();
                &mut slice[index]
            }
        }
    }
}

//mi ref_op!
macro_rules! ref_op {
    { $t:ident, $t_as:ty } => {
        impl <F:Float, const D:usize, const D2:usize> std::convert::AsRef<$t_as> for $t <F, D, D2> {
            fn as_ref(&self) -> &$t_as {&self.data}
        }
        impl <F:Float, const D:usize, const D2:usize> std::convert::AsMut<$t_as> for $t <F, D, D2> {
            fn as_mut(&mut self) -> &mut $t_as {&mut self.data}
        }
    }
}

//mi binary_op!
macro_rules! binary_op {
    { $t:ident, $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
        impl <F:Float, const D:usize, const D2:usize> std::ops::$trait_op<Self> for $t<F, D, D2> {
            type Output = Self;
            fn $op(self, other: Self) -> Self {
                let mut data = [F::zero();D2];
                for i in 0..D2 {
                    data[i] = self.data[i] $binop other.data[i];
                }
                Self { data }
            }
        }
        impl <F:Float, const D:usize, const D2:usize> std::ops::$trait_assign_op<Self> for $t<F, D, D2> {
            fn $assign_op(&mut self, other: Self) {
                for i in 0..D2 {self.data[i] $assign_binop other.data[i];}
            }
        }
        impl <F:Float, const D:usize, const D2:usize> std::ops::$trait_op<F> for $t<F, D, D2> {
            type Output = Self;
            fn $op(self, other: F) -> Self {
                let mut data = [F::zero();D2];
                for i in 0..D2 {
                    data[i] = self.data[i] $binop other;
                }
                Self { data }
            }
        }
        impl <F:Float, const D:usize, const D2:usize> std::ops::$trait_assign_op<F> for $t<F, D, D2> {
            fn $assign_op(&mut self, other: F) {
                for i in 0..D2 {self.data[i] $assign_binop other;}
            }
        }
    }
}

//mi binary_op_by_f!
macro_rules! binary_op_by_f {
    { $t:ident, $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
        impl <F:Float, const D:usize, const D2:usize> std::ops::$trait_op<F> for $t<F, D, D2> {
            type Output = Self;
            fn $op(self, other: F) -> Self {
                let mut data = [F::zero();D2];
                for i in 0..D2 {
                    data[i] = self.data[i] $binop other;
                }
                Self { data }
            }
        }
        impl <F:Float, const D:usize, const D2:usize> std::ops::$trait_assign_op<F> for $t<F, D, D2> {
            fn $assign_op(&mut self, other: F) {
                for i in 0..D2 {self.data[i] $assign_binop other;}
            }
        }
    }
}

//a FArray2
//tp FArray2
/// The [FArray2] is a wrapper around a `D2 = D`^2` sized array of [Float]s.
///
/// It provides implementations of the traits required for a [SqMatrix]
/// trait operating on an [FArray] of dimesion D.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FArray2<F: Float, const D: usize, const D2: usize> {
    data: [F; D2],
}

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
            fn determinant(&self) -> $f {
                $det_fn (&self.data)
            }
            fn inverse(&self) -> Self {
                $inv_fn (&self.data).into()
            }
        }
    }
}

//a New
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

farray2_traits! {f32,2,4,FArray2<f32,2,4>, matrix::determinant2, matrix::inverse2}
farray2_traits! {f32,3,9,FArray2<f32,3,9>, matrix::determinant3, matrix::inverse3}
farray2_traits! {f32,4,16,FArray2<f32,4,16>, matrix::determinant4, matrix::inverse4}

farray2_traits! {f64,2,4,FArray2<f64,2,4>, matrix::determinant2, matrix::inverse2}
farray2_traits! {f64,3,9,FArray2<f64,3,9>, matrix::determinant3, matrix::inverse3}
farray2_traits! {f64,4,16,FArray2<f64,4,16>, matrix::determinant4, matrix::inverse4}
