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
/// Default must be provided separately as it is only implemented
/// for \[F; D\] up to D=32
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
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
        crate::ref_traits!{$f, $e, $ty}
        crate::convert_traits!{$f, $e, $ty}
        crate::unary_traits!{$f, $e, $ty}
        crate::elementwise_traits!{$f, $e, $ty, Add, add, +, AddAssign, add_assign, +=}
        crate::elementwise_traits!{$f, $e, $ty, Sub, sub, -, SubAssign, sub_assign, -=}
        crate::serialize_traits!{$f, $e, $ty}
        farray_mul_div_traits!{$f, $e, $ty, Mul, mul, *, MulAssign, mul_assign, *=}
        farray_mul_div_traits!{$f, $e, $ty, Div, div, /, DivAssign, div_assign, /=}

        farray_vector_trait!{$f, $e, $ty}
    }
}

//mi Invoke farray_traits!
farray_traits! { f32, 1, FArray<f32,1> }
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

farray_traits! { f64, 1, FArray<f64,1> }
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
