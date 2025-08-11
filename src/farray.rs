//a Imports
use serde::{Deserialize, Serialize};

use super::vector;
use super::{Float, Vector};

//a Macros
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

//a Macros to implement the traits
//mi farray_basic_traits!
macro_rules! farray_basic_traits {
    { $f:ty,  $e:expr, $ty:ty } => {

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

//mi farray_ref_traits!
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

//mi farray_convert_traits
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

        impl <'a> std::convert::TryFrom<Vec<$f>> for $ty {
            type Error = Vec<$f>;
            fn try_from(data:Vec<$f>) -> Result<$ty, Self::Error> {
                let data: [$f; $e] = <[$f; $e]>::try_from(data)?;
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

//mi farray_unary_traits!
macro_rules! farray_unary_traits {
    { $f:ty,  $e:expr, $ty:ty } => {

        //ip Neg
        impl std::ops::Neg for $ty {
            type Output = Self;
            fn neg(mut self) -> Self::Output {
                for d in self.data.iter_mut() {
                    *d = -*d;
                }
                self
            }
        }

        //ip Neg
        impl<'a> std::ops::Neg for &'a $ty {
            type Output = $ty;
            fn neg(self) -> Self::Output {
                let mut r = <$ty>::default();
                for i in 0..$e {
                    r.data[i] = -self.data[i];
                }
                r
            }
        }
    }
}

//mi farray_add_sub_traits
macro_rules! farray_add_sub_traits {
    { $f:ty, $e:expr, $ty:ty,
      $trait_op:ident, $fn_op:ident, $op:tt,
      $trait_assign_op:ident, $fn_assign_op:ident, $op_assign:tt  } => {

        // Add/Sub of  &Self  for Self
        impl <'a> std::ops::$trait_op<&'a $ty> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: &'a $ty) -> $ty {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
                self
            }
        }

        // Add/Sub of  Deref to [F; D]  for Self (Covers Self + Self)
        impl<D: std::ops::Deref<Target = [$f; $e]>> std::ops::$trait_op<D> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: D) -> $ty {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
                self
            }
        }

        // Add/Sub of &Self for &Self
        impl <'a> std::ops::$trait_op<&'a $ty> for &'a $ty {
            type Output = $ty;
            fn $fn_op(self, other: &'a $ty) -> $ty {
                *self $op other
            }
        }

        // Add/Sub of  Deref to [F; D]  for &Self (Covers &Self + Self)
        impl<'a, D: std::ops::Deref<Target = [$f; $e]>> std::ops::$trait_op<D> for &'a $ty {
            type Output = $ty;
            fn $fn_op(self, other: D) -> $ty {
                *self $op other
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

        impl <'a> std::ops::$trait_assign_op<&'a $ty> for &mut $ty {
            fn $fn_assign_op(&mut self, other: &'a $ty)  {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
            }
        }

        impl<D: std::ops::Deref<Target = [$f; $e]>> std::ops::$trait_assign_op<D> for &mut $ty {
            fn $fn_assign_op(&mut self, other: D) {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
            }
        }
    }
}

//mi farray_mul_div_traits
macro_rules! farray_mul_div_traits {
    { $f:ty, $e:expr, $ty:ty,
      $trait_op:ident, $fn_op:ident, $op:tt,
      $trait_assign_op:ident, $fn_assign_op:ident, $op_assign:tt  } => {

        // Mul/Div by  $f  for Self
        impl std::ops::$trait_op<$f> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: $f) -> $ty {
                for i in 0..$e {
                    self.data[i] $op_assign other;
                }
                self
            }
        }

        impl std::ops::$trait_assign_op<$f> for $ty {
            fn $fn_assign_op(&mut self, other: $f) {
                for i in 0..$e {self.data[i] $op_assign other;}
            }
        }

        // Mul/Div by  & $f  for Self
        impl <'a> std::ops::$trait_op<&'a $f> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: &'a $f) -> $ty {
                for i in 0..$e {
                    self.data[i] $op_assign other;
                }
                self
            }
        }

        impl <'a> std::ops::$trait_assign_op<&'a $f> for $ty {
            fn $fn_assign_op(&mut self, other: &'a $f) {
                for i in 0..$e {self.data[i] $op_assign other;}
            }
        }
    }
}

//mi farray_serialize_traits
macro_rules! farray_serialize_traits {
    { $f:ty,  $e:expr, $ty:ty } => {

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

    }
}

//mi farray_vector_trait
macro_rules! farray_vector_trait {
    { $f:ty,  $e:expr, $ty:ty } => {

        impl Vector<$f, $e> for $ty {
        }

    }
}

//mi farray_traits
macro_rules! farray_traits {
    { $f:ty,  $e:expr, $ty:ty } => {

        farray_basic_traits!{$f, $e, $ty}
        farray_ref_traits!{$f, $e, $ty}
        farray_convert_traits!{$f, $e, $ty}
        farray_serialize_traits!{$f, $e, $ty}
        farray_unary_traits!{$f, $e, $ty}
        farray_add_sub_traits!{$f, $e, $ty, Add, add, +, AddAssign, add_assign, +=}
        farray_add_sub_traits!{$f, $e, $ty, Sub, sub, -, SubAssign, sub_assign, -=}
        farray_mul_div_traits!{$f, $e, $ty, Mul, mul, *, MulAssign, mul_assign, *=}
        farray_mul_div_traits!{$f, $e, $ty, Div, div, /, DivAssign, div_assign, /=}

        farray_vector_trait!{$f, $e, $ty}
    }
}

//mi Invoke farray_traits!
farray_traits! { f32, 2, FArray<f32,2> }
farray_traits! { f32, 3, FArray<f32,3> }
farray_traits! { f32, 4, FArray<f32,4> }
farray_traits! { f32, 5, FArray<f32,5> }
farray_traits! { f32, 6, FArray<f32,6> }
farray_traits! { f32, 7, FArray<f32,7> }
farray_traits! { f32, 8, FArray<f32,8> }
farray_traits! { f32, 9, FArray<f32,9> }
farray_traits! { f32, 10, FArray<f32,10> }
farray_traits! { f32, 11, FArray<f32,11> }
farray_traits! { f32, 12, FArray<f32,12> }
farray_traits! { f32, 13, FArray<f32,13> }
farray_traits! { f32, 14, FArray<f32,14> }
farray_traits! { f32, 15, FArray<f32,15> }
farray_traits! { f32, 16, FArray<f32,16> }

farray_traits! { f64, 2, FArray<f64,2> }
farray_traits! { f64, 3, FArray<f64,3> }
farray_traits! { f64, 4, FArray<f64,4> }
farray_traits! { f64, 5, FArray<f64,5> }
farray_traits! { f64, 6, FArray<f64,6> }
farray_traits! { f64, 7, FArray<f64,7> }
farray_traits! { f64, 8, FArray<f64,8> }
farray_traits! { f64, 9, FArray<f64,9> }
farray_traits! { f64, 10, FArray<f64,10> }
farray_traits! { f64, 11, FArray<f64,11> }
farray_traits! { f64, 12, FArray<f64,12> }
farray_traits! { f64, 13, FArray<f64,13> }
farray_traits! { f64, 14, FArray<f64,14> }
farray_traits! { f64, 15, FArray<f64,15> }
farray_traits! { f64, 16, FArray<f64,16> }
