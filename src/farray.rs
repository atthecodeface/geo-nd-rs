//a Imports
use serde::{Deserialize, Serialize};

use super::vector;
use super::{Float, Vector, Vector3};

//a Macros
//mi index_ops!
macro_rules! index_ops {
    { $t:ident } => {
        impl <F:Float, const D:usize> std::ops::Index<usize> for $t <F, D> {
            type Output = F;
            fn index(&self, index: usize) -> &F {
                let slice: &[_] = self.as_ref();
                &slice[index]
            }
        }
        impl <F:Float, const D:usize> std::ops::IndexMut<usize> for $t <F, D> {
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
        impl <F:Float, const D:usize> std::convert::AsRef<$t_as> for $t <F, D> {
            fn as_ref(&self) -> &$t_as {&self.data}
        }
        impl <F:Float, const D:usize> std::convert::AsMut<$t_as> for $t <F, D> {
            fn as_mut(&mut self) -> &mut $t_as {&mut self.data}
        }
    }
}

//mi binary_not_f_op!
macro_rules! binary_not_f_op {
    { $t:ident, $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
/*        impl <F:Float, const D:usize> std::ops::$trait_op<Self> for $t<F, D> {
            type Output = Self;
            fn $op(self, other: Self) -> Self {
                let mut data = [F::zero();D];
                for i in 0..D {
                    data[i] = self.data[i] $binop other.data[i];
                }
                Self { data }
            }
    }
        */
    }
}

//mi binary_with_f_op!
macro_rules! binary_with_f_op {
    { $t:ident, $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
        impl <F:Float, const D:usize> std::ops::$trait_op<Self> for $t<F, D> {
            type Output = Self;
            fn $op(self, other: Self) -> Self {
                let mut data = [F::zero();D];
                for i in 0..D {
                    data[i] = self.data[i] $binop other.data[i];
                }
                Self { data }
            }
        }
        impl <F:Float, const D:usize> std::ops::$trait_assign_op<Self> for $t<F, D> {
            fn $assign_op(&mut self, other: Self) {
                for i in 0..D {self.data[i] $assign_binop other.data[i];}
            }
        }
        impl <F:Float, const D:usize> std::ops::$trait_op<F> for $t<F, D> {
            type Output = Self;
            fn $op(self, other: F) -> Self {
                let mut data = [F::zero();D];
                for i in 0..D {
                    data[i] = self.data[i] $binop other;
                }
                Self { data }
            }
    }
        impl <F:Float, const D:usize> std::ops::$trait_assign_op<F> for $t<F, D> {
            fn $assign_op(&mut self, other: F) {
                for i in 0..D {self.data[i] $assign_binop other;}
            }
        }
    }
}

//a FArray
//tp FArray
/// The [FArray] is a wrapper around a `D` sized array of [Float]s.
///
/// It provides implementations of the traits required for a [Vector]
/// trait, hence it can be used for a [Vector] of any size `D`.
///
/// Default must be provided separately as it is only implemented for [F;D] up to D=32
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FArray<F: Float, const D: usize> {
    data: [F; D],
}

binary_with_f_op! { FArray, Mul, mul, *, MulAssign, mul_assign, *= }
binary_with_f_op! { FArray, Div, div, /, DivAssign, div_assign, /= }

//ip Neg for FArray
impl<F: Float, const D: usize> std::ops::Neg for FArray<F, D> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for d in self.data.iter_mut() {
            *d = -*d;
        }
        self
    }
}

//ip Default for FArray
impl<F: Float, const D: usize> std::default::Default for FArray<F, D> {
    fn default() -> Self {
        Self {
            data: vector::zero(),
        }
    }
}

//ip Display for FArray
impl<F: Float, const D: usize> std::fmt::Display for FArray<F, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        vector::fmt(f, &self.data)
    }
}

//ip Vector3<F> for FArray
impl Vector3<f32> for FArray<f32, 3> {}
impl Vector3<f64> for FArray<f64, 3> {}

//a All stuff 2
//mi farray_traits!
macro_rules! farray_ref_traits {
    { $f:ty,  $e:expr, $ty:ty } => {

        impl std::ops::Deref for $ty {
            type Target = [$f; $e];
            fn deref(&self) -> &[$f; $e] {
                &self.data
            }
        }
        impl std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut [$f; $e] {
                &mut self.data
            }
        }

        impl std::convert::AsRef<[$f; $e]> for $ty
        {
            fn as_ref(&self) -> &[$f; $e] {
                &self.data
            }
        }
        impl std::convert::AsMut<[$f; $e]> for $ty
        {
            fn as_mut(&mut self) -> &mut [$f; $e] {
                &mut self.data
            }
        }

        impl std::ops::Index<usize> for $ty {
            type Output = $f;
            fn index(&self, index: usize) -> &$f {
                &self.data[index]
            }
        }
        impl std::ops::IndexMut<usize> for $ty {
            fn index_mut(&mut self, index: usize) -> &mut $f {
                &mut self.data[index]
            }
        }
    }
}

macro_rules! farray_convert_traits {
    { $f:ty,  $e:expr, $ty:ty } => {

        impl std::convert::From<[$f; $e]> for $ty {
            fn from(data:[$f; $e]) -> $ty {
                Self { data }
            }
        }
        impl <'a> std::convert::From<&'a [$f; $e]> for $ty {
            fn from(data:&'a [$f; $e]) -> $ty {
                Self { data:*data }
            }
        }
        impl <'a> std::convert::TryFrom<&'a [$f]> for $ty {
            type Error = std::array::TryFromSliceError;
            fn try_from(data:&'a [$f]) -> Result<$ty, Self::Error> {
                let data :[$f; $e] = <[$f; $e]>::try_from(data)?;
                Ok(data.into())
            }
        }

        impl std::convert::From<$ty> for [$f; $e]  {
            fn from(s: $ty) -> [$f; $e]  {
                s.data
            }
        }
        impl <'a> std::convert::From<&'a $ty> for &'a [$f; $e]  {
            fn from(s: &'a $ty) -> &'a [$f; $e]  {
                &s.data
            }
        }
        impl <'a> std::convert::From<&'a $ty> for [$f; $e]  {
            fn from(s: &'a $ty) -> [$f; $e]  {
                s.data
            }
        }

    }
}

macro_rules! farray_add_sub_traits {
    { $f:ty, $e:expr, $ty:ty,
      $trait_op:ident, $fn_op:ident, $op:tt,
      $trait_assign_op:ident, $fn_assign_op:ident, $op_assign:tt  } => {

        impl <'a> std::ops::$trait_op<&'a $ty> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: &'a $ty) -> $ty {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
                self
            }
        }

        impl<D: std::ops::Deref<Target = [$f; $e]>> std::ops::$trait_op<D> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: D) -> $ty {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
                self
            }
        }

        impl <'a> std::ops::$trait_op<&'a $ty> for &'a $ty {
            type Output = $ty;
            fn $fn_op(self, other: &'a $ty) -> $ty {
                *self + other
            }
        }

        impl<'a, D: std::ops::Deref<Target = [$f; $e]>> std::ops::$trait_op<D> for &'a $ty {
            type Output = $ty;
            fn $fn_op(self, other: D) -> $ty {
                *self + other
            }
        }

        impl <'a> std::ops::$trait_assign_op<&'a $ty> for $ty {
            fn $fn_assign_op(&mut self, other: &'a $ty)  {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
            }
        }

        impl<D: std::ops::Deref<Target = [$f; $e]>> std::ops::$trait_assign_op<D> for $ty {
            fn $fn_assign_op(&mut self, other: D) {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
            }
        }
    }
}

macro_rules! farray_traits {
    { $f:ty,  $e:expr, $ty:ty } => {

        farray_ref_traits!{$f, $e, $ty}
        farray_convert_traits!{$f, $e, $ty}
        farray_add_sub_traits!{$f, $e, $ty, Add, add, +, AddAssign, add_assign, +=}
        farray_add_sub_traits!{$f, $e, $ty, Sub, sub, -, SubAssign, sub_assign, -=}

        impl Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.data.serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $ty {
            fn deserialize<DE>(deserializer: DE) -> Result<Self, DE::Error>
            where
                DE: serde::Deserializer<'de>,
            {
                let array = <[$f; $e]>::deserialize(deserializer)?;
                Ok(array.into())
            }
        }

        impl Vector<$f, $e> for $ty {
            fn is_zero(&self) -> bool {
                !self.data.iter().any(|f| *f!=0.0)
            }
            fn mix(self, other: &Self, t: $f) -> Self {
                Self {
                    data: vector::mix(&self.data, &other.data, t),
                }
            }
            fn reduce_sum(&self) -> $f {
                let mut r = <$f>::default();
                for d in self.data {
                    r += d
                }
                r
            }
            fn dot(&self, other: &Self) -> $f {
                vector::dot(&self.data, &other.data)
            }
        }

    }
}

//mi farray_traits!
farray_traits! { f32, 2, FArray<f32,2> }
farray_traits! { f32, 3, FArray<f32,3> }
farray_traits! { f32, 4, FArray<f32,4> }
farray_traits! { f64, 2, FArray<f64,2> }
farray_traits! { f64, 3, FArray<f64,3> }
farray_traits! { f64, 4, FArray<f64,4> }
