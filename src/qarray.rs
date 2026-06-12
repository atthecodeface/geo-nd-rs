use std::ops::Deref;

use num_traits::{ConstOne, ConstZero};

use super::{Float, Quaternion, SqMatrix, SqMatrix3, SqMatrix4, Vector};
use super::{quat, vector};

//a QArray
//tp QArray
/// The [QArray] is a wrapper around a `D` sized array of [Float]s.
///
/// It provides implementations of the traits required for a [Vector]
/// trait, hence it can be used for a [Vector] of any size `D`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
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
                [<$f>::ZERO,
                 <$f>::ZERO,
                 <$f>::ZERO,
                 <$f>::ONE,
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

        // Mul of &Self for Self
        impl <'a> std::ops::Mul<&'a $ty> for $ty {
            type Output = $ty;
            fn mul(self, other: &'a $ty) -> $ty {
                quat::multiply(self.deref(), other.deref()).into()
            }
        }

        // Mul of &Self for &Self
        impl <'a> std::ops::Mul<&'a $ty> for &'a $ty {
            type Output = $ty;
            fn mul(self, other: &'a $ty) -> $ty {
                quat::multiply(self.deref(), other.deref()).into()
            }
        }

        // Mul of &Self for &Self
        impl<'a> std::ops::Mul<$ty> for &'a $ty {
            type Output = $ty;
            fn mul(self, other: $ty) -> $ty {
                quat::multiply(self.deref(), other.deref()).into()
            }
        }

        impl <'a> std::ops::MulAssign<&'a $ty> for $ty {
            fn mul_assign(&mut self, other: &'a $ty)  {
                *self = self.deref() * other;
            }
        }

        impl std::ops::MulAssign for $ty {
            fn mul_assign(&mut self, other: Self) {
                *self = self.deref() * other;
            }
        }

        // Mul of &V for &Self
        impl<'a, V:Vector<$f, 3>> std::ops::Mul<&'a V> for &'a $ty
         {
            type Output = V;
            fn mul(self, other: &'a V) -> V {
                quat::apply3(&self.data, other.as_ref()).into()
            }
        }

        // Mul of &V for Self
        impl<'a, V:Vector<$f, 3>> std::ops::Mul<&'a V> for $ty
         {
            type Output = V;
            fn mul(self, other: &'a V) -> V {
                quat::apply3(&self.data, other.as_ref()).into()
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
                quat::divide(self.deref(), other).into()
            }
        }

        // Div of &Self for &Self
        impl <'a> std::ops::Div<&'a $ty> for &'a $ty {
            type Output = $ty;
            fn div(self, other: &'a $ty) -> $ty {
                quat::divide(self.deref(), other).into()
            }
        }

        // Div of Self for &Self (Covers &Self + Self)
        impl<'a> std::ops::Div<$ty> for &'a $ty {
            type Output = $ty;
            fn div(self, other: $ty) -> $ty {
                quat::divide(self.deref(), &other).into()
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
                M: SqMatrix<$f, 3, 9>
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
