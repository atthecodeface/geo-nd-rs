//a Imports
use std::ops::Deref;

use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

use super::{quat, vector};
use super::{Float, Quaternion, SqMatrix, SqMatrix3, SqMatrix4, Vector3, Vector4};

//a QArray
//tp QArray
/// The [QArray] is a wrapper around a `D` sized array of [Float]s.
///
/// It provides implementations of the traits required for a [Vector]
/// trait, hence it can be used for a [Vector] of any size `D`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QArray<F>
where
    F: Float,
{
    /// Data is stored i,j,k,r - so that the vector can be used [F;3] without alignment issues
    data: [F; 4],
}

//a Macros to implement the traits
//mi qarray_basic_traits!
macro_rules! qarray_basic_traits {
    { $f:ty, $ty:ty } => {

        //ip Default for Qarray
        impl std::default::Default for $ty {
            fn default() -> Self {
                [<$f>::zero(),
                 <$f>::zero(),
                 <$f>::zero(),
                 <$f>::one(),
                 ].into()
            }
        }

        //ip Display for Qarray
        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                vector::fmt(f, &self.data)
            }
        }
    }
}

//mi qarray_ref_traits!
macro_rules! qarray_ref_traits {
    { $f:ty, $ty:ty } => {

        impl std::ops::Deref for $ty {
            type Target = [$f; 4];
            fn deref(&self) -> &[$f; 4] {
                &self.data
            }
        }
        impl std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut [$f; 4] {
                &mut self.data
            }
        }

        impl std::convert::AsRef<[$f; 4]> for $ty
        {
            fn as_ref(&self) -> &[$f; 4] {
                &self.data
            }
        }
        impl std::convert::AsMut<[$f; 4]> for $ty
        {
            fn as_mut(&mut self) -> &mut [$f; 4] {
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

//mi qarray_convert_traits
macro_rules! qarray_convert_traits {
    { $f:ty, $ty:ty } => {

        impl std::convert::From<[$f; 4]> for $ty {
            fn from(data:[$f; 4]) -> $ty {
                Self { data }
            }
        }
        impl <'a> std::convert::From<&'a [$f; 4]> for $ty {
            fn from(data:&'a [$f; 4]) -> $ty {
                Self { data:*data }
            }
        }

        impl <'a> std::convert::TryFrom<&'a [$f]> for $ty {
            type Error = std::array::TryFromSliceError;
            fn try_from(data:&'a [$f]) -> Result<$ty, Self::Error> {
                let data :[$f; 4] = <[$f; 4]>::try_from(data)?;
                Ok(data.into())
            }
        }

        impl <'a> std::convert::TryFrom<Vec<$f>> for $ty {
            type Error = Vec<$f>;
            fn try_from(data:Vec<$f>) -> Result<$ty, Self::Error> {
                let data: [$f; 4] = <[$f; 4]>::try_from(data)?;
                Ok(data.into())
            }
        }

        impl std::convert::From<$ty> for [$f; 4]  {
            fn from(s: $ty) -> [$f; 4]  {
                s.data
            }
        }
        impl <'a> std::convert::From<&'a $ty> for &'a [$f; 4]  {
            fn from(s: &'a $ty) -> &'a [$f; 4]  {
                &s.data
            }
        }
        impl <'a> std::convert::From<&'a $ty> for [$f; 4]  {
            fn from(s: &'a $ty) -> [$f; 4]  {
                s.data
            }
        }

    }
}

//mi qarray_unary_traits!
macro_rules! qarray_unary_traits {
    { $f:ty, $ty:ty } => {

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
                for i in 0..4 {
                    r.data[i] = -self.data[i];
                }
                r
            }
        }
    }
}

//mi qarray_add_sub_traits
macro_rules! qarray_add_sub_traits {
    { $f:ty, $ty:ty,
      $trait_op:ident, $fn_op:ident, $op:tt,
      $trait_assign_op:ident, $fn_assign_op:ident, $op_assign:tt  } => {

        // Add/Sub of  &Self  for Self
        impl <'a> std::ops::$trait_op<&'a $ty> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: &'a $ty) -> $ty {
                for i in 0..4 {
                    self.data[i] $op_assign other[i];
                }
                self
            }
        }

        // Add/Sub of  Deref to [F; D]  for Self (Covers Self + Self)
        impl<D: std::ops::Deref<Target = [$f; 4]>> std::ops::$trait_op<D> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: D) -> $ty {
                for i in 0..4 {
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
        impl<'a, D: std::ops::Deref<Target = [$f; 4]>> std::ops::$trait_op<D> for &'a $ty {
            type Output = $ty;
            fn $fn_op(self, other: D) -> $ty {
                *self $op other
            }
        }

        impl <'a> std::ops::$trait_assign_op<&'a $ty> for $ty {
            fn $fn_assign_op(&mut self, other: &'a $ty)  {
                for i in 0..4 {
                    self.data[i] $op_assign other[i];
                }
            }
        }

        impl<D: std::ops::Deref<Target = [$f; 4]>> std::ops::$trait_assign_op<D> for $ty {
            fn $fn_assign_op(&mut self, other: D) {
                for i in 0..4 {
                    self.data[i] $op_assign other[i];
                }
            }
        }

        impl <'a> std::ops::$trait_assign_op<&'a $ty> for &mut $ty {
            fn $fn_assign_op(&mut self, other: &'a $ty)  {
                for i in 0..4 {
                    self.data[i] $op_assign other[i];
                }
            }
        }

        impl<D: std::ops::Deref<Target = [$f; 4]>> std::ops::$trait_assign_op<D> for &mut $ty {
            fn $fn_assign_op(&mut self, other: D) {
                for i in 0..4 {
                    self.data[i] $op_assign other[i];
                }
            }
        }
    }
}

//mi qarray_mul_div_traits
macro_rules! qarray_mul_div_traits {
    { $f:ty, $ty:ty } => {
        // Mul of  Self for Self
        impl std::ops::Mul for $ty {
            type Output = $ty;
            fn mul(self, other: $ty) -> $ty {
                quat::multiply(self.deref(), other.deref()).into()
            }
        }

        // Mul of &[F; 4] for Self
        impl <'a> std::ops::Mul<&'a [$f; 4]> for $ty {
            type Output = $ty;
            fn mul(self, other: &'a [$f; 4]) -> $ty {
                quat::multiply(self.deref(), other).into()
            }
        }

        // Mul of &Self for Self
        impl <'a> std::ops::Mul<&'a $ty> for $ty {
            type Output = $ty;
            fn mul(self, other: &'a $ty) -> $ty {
                (&self) * other
            }
        }

        // Mul of &Self for &Self
        impl <'a> std::ops::Mul<&'a $ty> for &'a $ty {
            type Output = $ty;
            fn mul(self, other: &'a $ty) -> $ty {
                *self * other
            }
        }

        // Mul of &[F; D] for &Self (Covers &Self + Self)
        impl<'a> std::ops::Mul<&'a [$f; 4]> for &'a $ty {
            type Output = $ty;
            fn mul(self, other: &'a[$f; 4]) -> $ty {
                *self * other
            }
        }


        impl <'a> std::ops::MulAssign<&'a $ty> for $ty {
            fn mul_assign(&mut self, other: &'a $ty)  {
                *self = self.deref() * other;
            }
        }

        // MulAssign of &[F; 4] for Self
        impl <'a> std::ops::MulAssign<&'a [$f; 4]> for $ty {
            fn mul_assign(&mut self, other: &'a [$f; 4]) {
                *self = self.deref() * other;
            }
        }

        impl std::ops::MulAssign for $ty {
            fn mul_assign(&mut self, other: Self) {
                *self = self.deref() * other.deref();
            }
        }



        // Div of  Self for Self
        impl std::ops::Div for $ty {
            type Output = $ty;
            fn div(self, other: $ty) -> $ty {
                quat::divide(self.deref(), other.deref()).into()
            }
        }

        // Div of &[F; 4] for Self
        impl <'a> std::ops::Div<&'a [$f; 4]> for $ty {
            type Output = $ty;
            fn div(self, other: &'a [$f; 4]) -> $ty {
                quat::divide(self.deref(), other).into()
            }
        }

        // Div of  &Self  for Self
        impl <'a> std::ops::Div<&'a $ty> for $ty {
            type Output = $ty;
            fn div(self, other: &'a $ty) -> $ty {
                (&self) / other
            }
        }

        // Div of &Self for &Self
        impl <'a> std::ops::Div<&'a $ty> for &'a $ty {
            type Output = $ty;
            fn div(self, other: &'a $ty) -> $ty {
                *self / other
            }
        }

        // Div of Self for &Self (Covers &Self + Self)
        impl<'a> std::ops::Div<$ty> for &'a $ty {
            type Output = $ty;
            fn div(self, other: $ty) -> $ty {
                *self / other.deref()
            }
        }

        impl <'a> std::ops::DivAssign<&'a $ty> for $ty {
            fn div_assign(&mut self, other: &'a $ty)  {
                *self = self.deref() / other;
            }
        }

        // DivAssign of &[F; 4] for Self
        impl <'a> std::ops::DivAssign<&'a [$f; 4]> for $ty {
            fn div_assign(&mut self, other: &'a [$f; 4]) {
                *self = *self / other;
            }
        }

        impl std::ops::DivAssign<Self> for $ty {
            fn div_assign(&mut self, other: Self) {
                *self = *self / other.deref();
            }
        }



    }
}

//mi qarray_mul_div_f_traits
macro_rules! qarray_mul_div_f_traits {
    { $f:ty, $ty:ty,
      $trait_op:ident, $fn_op:ident, $op:tt,
      $trait_assign_op:ident, $fn_assign_op:ident, $op_assign:tt  } => {

        // Mul/Div by  $f  for Self
        impl std::ops::$trait_op<$f> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: $f) -> $ty {
                for i in 0..4 {
                    self.data[i] $op_assign other;
                }
                self
            }
        }

        impl std::ops::$trait_assign_op<$f> for $ty {
            fn $fn_assign_op(&mut self, other: $f) {
                for i in 0..4 {self.data[i] $op_assign other;}
            }
        }

        // Mul/Div by  & $f  for Self
        impl <'a> std::ops::$trait_op<&'a $f> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: &'a $f) -> $ty {
                for i in 0..4 {
                    self.data[i] $op_assign other;
                }
                self
            }
        }

        impl <'a> std::ops::$trait_assign_op<&'a $f> for $ty {
            fn $fn_assign_op(&mut self, other: &'a $f) {
                for i in 0..4 {self.data[i] $op_assign other;}
            }
        }
    }
}

//mi qarray_serialize_traits
macro_rules! qarray_serialize_traits {
    { $f:ty, $ty:ty } => {

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
                let array = <[$f; 4]>::deserialize(deserializer)?;
                Ok(array.into())
            }
        }

    }
}

//mi qarray_quaternion_trait
macro_rules! qarray_quaternion_trait {
    { $f:ty,  $ty:ty } => {

        impl Quaternion<$f> for $ty
        {
            #[inline]
            fn as_rijk(&self) -> ($f, $f, $f, $f) {
                quat::as_rijk(self.deref())
            }

            #[inline]
            fn of_rijk(r: $f, i: $f, j: $f, k: $f) -> Self {
                quat::of_rijk(r, i, j, k).into()
            }

            //fp of_rotation3
            /// Find the quaternion of a Matrix3 assuming it is purely a rotation
            #[inline]
            fn of_rotation3<M>(rotation: &M) -> Self
            where
                M: SqMatrix3<$f>
            {
                quat::of_rotation(rotation.as_ref()).into()
            }

            #[inline]
            fn set_rotation3<M> (&self, matrix3:&mut M)
            where
                M: SqMatrix3<$f>
            {
                quat::to_rotation3(&self.data, matrix3.as_mut())
            }

            #[inline]
            fn set_rotation4<M> (&self, matrix:&mut M)
            where
                M: SqMatrix4<$f>
            {
                quat::to_rotation4(&self.data, matrix.as_mut())
            }
        }
    }
}

//mi qarray_traits
macro_rules! qarray_traits {
    { $f:ty, $ty:ty } => {

        qarray_basic_traits!{$f, $ty}
        crate::ref_traits!{$f, 4, $ty}
        crate::convert_traits!{$f, 4, $ty}
        crate::serialize_traits!{$f, 4, $ty}
        crate::unary_traits!{$f, 4, $ty}
        crate::elementwise_traits!{$f, 4, $ty, Add, add, +, AddAssign, add_assign, +=}
        crate::elementwise_traits!{$f, 4, $ty, Sub, sub, -, SubAssign, sub_assign, -=}
        crate::scale_by_f_traits!{$f, $ty, Mul, mul, *, MulAssign, mul_assign, *=}
        crate::scale_by_f_traits!{$f, $ty, Div, div, /, DivAssign, div_assign, /=}
        qarray_mul_div_traits!{$f, $ty}

        qarray_quaternion_trait!{$f, $ty}
    }
}

//mi Invoke qarray_traits!
qarray_traits! { f32, QArray<f32>}
qarray_traits! { f64, QArray<f64>}
