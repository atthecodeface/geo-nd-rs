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
        impl <F:Float, const D:usize> std::ops::$trait_assign_op<Self> for $t<F, D> {
            fn $assign_op(&mut self, other: Self) {
                for i in 0..D {self.data[i] $assign_binop other.data[i];}
            }
        }
        impl <F:Float, const D:usize> std::ops::$trait_assign_op<F> for $t<F, D> {
            fn $assign_op(&mut self, other: F) {
                for i in 0..D {self.data[i] $assign_binop other;}
            }
        }
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
#[derive(Clone, Copy, Debug)]
pub struct FArray<F: Float, const D: usize> {
    data: [F; D],
}

//ip Serialize for FArray
impl<F: Float + serde::Serialize, const D: usize> Serialize for FArray<F, D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut seq = serializer.serialize_tuple(D)?;
        for e in self.data.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

//ip Deserialize for FArray
impl<'de, F: Float + serde::Deserialize<'de>, const D: usize> Deserialize<'de> for FArray<F, D> {
    fn deserialize<DE>(deserializer: DE) -> Result<Self, DE::Error>
    where
        DE: serde::Deserializer<'de>,
    {
        let array = Vec::<F>::deserialize(deserializer)?;
        if array.len() != D {
            return Err(serde::de::Error::invalid_length(array.len(), &"<D> floats"));
        }
        let mut data = Self::default();
        for (i, d) in array.into_iter().enumerate() {
            data[i] = d;
        }
        Ok(data)
    }
}

//ip FArray
index_ops! { FArray }
ref_op! { FArray, [F;D] }
ref_op! { FArray, [F] }

binary_not_f_op! { FArray, Add, add, +, AddAssign, add_assign, += }
binary_not_f_op! { FArray, Sub, sub, -, SubAssign, sub_assign, -= }
binary_with_f_op! { FArray, Mul, mul, *, MulAssign, mul_assign, *= }
binary_with_f_op! { FArray, Div, div, /, DivAssign, div_assign, /= }

//ip Neg for FArray
impl<F: Float, const D: usize> std::ops::Neg for FArray<F, D> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        let data: &mut [F; D] = self.as_mut();
        for d in data.iter_mut() {
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
macro_rules! farray_traits {
    { $ty:ty,  $e:expr } => {

impl Vector<$ty, $e> for FArray<$ty, $e> {
    fn is_zero(&self) -> bool {
        vector::is_zero(&self.data)
    }
    fn set_zero(&mut self) {
        vector::set_zero(&mut self.data)
    }
    fn mix(self, other: &Self, t: $ty) -> Self {
        Self {
            data: vector::mix(&self.data, &other.data, t),
        }
    }
    fn reduce_sum(&self) -> $ty {
        let mut r = <$ty>::default();
        for d in self.data {
            r += d
        }
        r
    }
    fn dot(&self, other: &Self) -> $ty {
        vector::dot(&self.data, &other.data)
    }
}

        impl std::convert::From<[$ty; $e]> for FArray<$ty, $e> {
            fn from(data:[$ty; $e]) -> FArray<$ty, $e> {
                Self { data }
            }
        }
        impl std::convert::From<FArray<$ty, $e>> for [$ty; $e]  {
            fn from(s: FArray<$ty, $e>) -> [$ty; $e]  {
                s.data
            }
        }
        impl <'a> std::convert::From<&'a FArray<$ty, $e>> for &'a [$ty; $e]  {
            fn from(s: &'a FArray<$ty, $e>) -> &'a [$ty; $e]  {
                &s.data
            }
        }

        impl std::ops::Deref for FArray<$ty, $e> {
            type Target = [$ty; $e];
            fn deref(&self) -> &[$ty; $e] {
                &self.data
            }
        }


        // If we implement Deref< Target = []>, AsRef<[]>, Add<&[],
        //
        // then AsRef<[]> conflicts with Add<&[]
        //
        // If we implement
        //         impl<A: std::ops::Deref<Target = [$ty; $e]>> std::ops::Add<A> for FArray<$ty, $e> {
        // conflicts with
        //         impl std::ops::Add<&[$ty; $e]> for FArray<$ty, $e> {
        //
        // If we implement
        //         impl<A: std::ops::Deref<Target = [$ty; $e]>> std::ops::Add<A> for FArray<$ty, $e> {
        // conflicts with
        //         impl std::ops::Add<[$ty; $e]> for FArray<$ty, $e> {
        //

        /*
        impl std::ops::Add for FArray<$ty, $e> {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                self + &other
            }
        }

        impl std::ops::Sub for FArray<$ty, $e> {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                self - &other
            }
        }

         */

        impl<A: std::ops::Deref<Target = [$ty; $e]>> std::ops::Add<A> for FArray<$ty, $e> {
            type Output = Self;
            fn add(self, other: A) -> Self {
                let mut data = [0.0; $e];
                for i in 0..$e {
                    data[i] = self.data[i] + other[i];
                }
                Self { data }
            }
        }

        impl<A: std::ops::Deref<Target = [$ty; $e]>> std::ops::Sub<A> for FArray<$ty, $e> {
            type Output = Self;
            fn sub(self, other: A) -> Self {
                let mut data = [0.0; $e];
                for i in 0..$e {
                    data[i] = self.data[i] - other[i];
                }
                Self { data }
            }
        }

        /*
        impl<A: std::convert::AsRef<[$ty; $e]>> std::ops::Add<A> for FArray<$ty, $e> {
            type Output = Self;
            fn add(self, other: A) -> Self {
                let mut data = [0.0; $e];
                let other = other.as_ref();
                for i in 0..$e {
                    data[i] = self.data[i] + other[i];
                }
                Self { data }
            }
        }

        impl<A: std::convert::AsRef<[$ty; $e]>> std::ops::Sub<A> for FArray<$ty, $e> {
            type Output = Self;
            fn sub(self, other: A) -> Self {
                let mut data = [0.0; $e];
                let other = other.as_ref();
                for i in 0..$e {
                    data[i] = self.data[i] - other[i];
                }
                Self { data }
            }
        }
         */

        /*
        impl std::ops::Add<[$ty; $e]> for FArray<$ty, $e> {
            type Output = Self;
            fn add(self, other: [$ty; $e]) -> Self {
                let mut data = [F::zero(); $e];
                let other = other.as_ref();
                for i in 0..$e {
                    data[i] = self.data[i] + other[i];
                }
                Self { data }
            }
        }

        impl std::ops::Sub<[$ty; $e]> for FArray<$ty, $e> {
            type Output = Self;
            fn sub(self, other: [$ty; $e]) -> Self {
                let mut data = [F::zero(); $e];
                let other = other.as_ref();
                for i in 0..$e {
                    data[i] = self.data[i] - other[i];
                }
                Self { data }
            }
    }
        */
    }
}

//mi farray_traits!
farray_traits! { f32, 2 }
farray_traits! { f32, 3 }
farray_traits! { f32, 4 }
farray_traits! { f64, 2 }
farray_traits! { f64, 3 }
farray_traits! { f64, 4 }
