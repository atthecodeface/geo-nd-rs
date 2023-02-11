//a Imports
use serde::{Deserialize, Serialize};

use super::{matrix, vector, FArray};
use super::{Float, SqMatrix, SqMatrix3, SqMatrix4, Vector};

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
                for i in 0..D {
                    data[i] = self.data[i] $binop other.data[i];
                }
                Self { data }
            }
        }
        impl <F:Float, const D:usize, const D2:usize> std::ops::$trait_assign_op<Self> for $t<F, D, D2> {
            fn $assign_op(&mut self, other: Self) {
                for i in 0..D {self.data[i] $assign_binop other.data[i];}
            }
        }
        impl <F:Float, const D:usize, const D2:usize> std::ops::$trait_op<F> for $t<F, D, D2> {
            type Output = Self;
            fn $op(self, other: F) -> Self {
                let mut data = [F::zero();D2];
                for i in 0..D {
                    data[i] = self.data[i] $binop other;
                }
                Self { data }
            }
        }
        impl <F:Float, const D:usize, const D2:usize> std::ops::$trait_assign_op<F> for $t<F, D, D2> {
            fn $assign_op(&mut self, other: F) {
                for i in 0..D {self.data[i] $assign_binop other;}
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
#[derive(Clone, Copy, Debug)]
pub struct FArray2<F: Float, const D: usize, const D2: usize> {
    data: [F; D2],
}

//ip FArray2
index_ops! { FArray2 }
ref_op! { FArray2, [F;D2] }
ref_op! { FArray2, [F] }
binary_op! { FArray2, Add, add, +, AddAssign, add_assign, += }
binary_op! { FArray2, Sub, sub, -, SubAssign, sub_assign, -= }
binary_op! { FArray2, Mul, mul, *, MulAssign, mul_assign, *= }
binary_op! { FArray2, Div, div, /, DivAssign, div_assign, /= }

impl<F: Float, const D: usize, const D2: usize> std::default::Default for FArray2<F, D, D2> {
    fn default() -> Self {
        Self {
            data: vector::zero(),
        }
    }
}

//ip Serialize for FArray2
impl<F: Float + serde::Serialize, const D: usize, const D2: usize> Serialize for FArray2<F, D, D2> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut seq = serializer.serialize_tuple(D2)?;
        for e in self.data.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

//ip Deserialize for FArray2
impl<'de, F: Float + serde::Deserialize<'de>, const D: usize, const D2: usize> Deserialize<'de>
    for FArray2<F, D, D2>
{
    fn deserialize<DE>(deserializer: DE) -> Result<Self, DE::Error>
    where
        DE: serde::Deserializer<'de>,
    {
        let array = Vec::<F>::deserialize(deserializer)?;
        if array.len() != D2 {
            return Err(serde::de::Error::invalid_length(array.len(), &"<D> floats"));
        }
        let mut data = Self::default();
        for (i, d) in array.into_iter().enumerate() {
            data[i] = d;
        }
        Ok(data)
    }
}

//ip From<[F;D]> for FArray
impl<F: Float, const D: usize, const D2: usize> From<[F; D2]> for FArray2<F, D, D2> {
    fn from(data: [F; D2]) -> Self {
        Self { data }
    }
}
//ip SqMatrix<F,2,4> for FArray2
impl<F: Float> SqMatrix<FArray<F, 2>, F, 2, 4> for FArray2<F, 2, 4> {
    fn from_array(data: [F; 4]) -> Self {
        Self { data }
    }
    fn zero() -> Self {
        Self {
            data: vector::zero(),
        }
    }
    fn identity() -> Self {
        Self {
            data: vector::zero(),
        }
    }
    fn is_zero(&self) -> bool {
        vector::is_zero(&self.data)
    }
    fn set_zero(&mut self) {
        vector::set_zero(&mut self.data)
    }
    fn transpose(&self) -> Self {
        Self::from_array(matrix::transpose::<F, 4, 2, 2>(self.data))
    }
    fn determinant(&self) -> F {
        matrix::determinant2(&self.data)
    }
    fn inverse(&self) -> Self {
        Self::from_array(matrix::inverse2(&self.data))
    }
    fn transform(&self, v: &FArray<F, 2>) -> FArray<F, 2> {
        FArray::from_array(matrix::multiply::<F, 4, 2, 2, 2, 2, 1>(
            &self.data,
            v.as_ref(),
        ))
    }
}

//ip SqMatrix<F,3,9> for FArray2
impl<F: Float> SqMatrix<FArray<F, 3>, F, 3, 9> for FArray2<F, 3, 9> {
    fn from_array(data: [F; 9]) -> Self {
        Self { data }
    }
    fn zero() -> Self {
        Self {
            data: vector::zero(),
        }
    }
    fn identity() -> Self {
        Self {
            data: vector::zero(),
        }
    }
    fn is_zero(&self) -> bool {
        vector::is_zero(&self.data)
    }
    fn set_zero(&mut self) {
        vector::set_zero(&mut self.data)
    }
    fn transpose(&self) -> Self {
        Self::from_array(matrix::transpose::<F, 9, 3, 3>(self.data))
    }
    fn determinant(&self) -> F {
        matrix::determinant3(&self.data)
    }
    fn inverse(&self) -> Self {
        Self::from_array(matrix::inverse3(&self.data))
    }
    fn transform(&self, v: &FArray<F, 3>) -> FArray<F, 3> {
        FArray::from_array(matrix::multiply::<F, 9, 3, 3, 3, 3, 1>(
            &self.data,
            v.as_ref(),
        ))
    }
}

//ip SqMatrix3<F> for FArray2
impl<F: Float> SqMatrix3<FArray<F, 3>, F> for FArray2<F, 3, 9> {}

//ip SqMatrix<F,4,16> for FArray2
impl<F: Float> SqMatrix<FArray<F, 4>, F, 4, 16> for FArray2<F, 4, 16> {
    fn from_array(data: [F; 16]) -> Self {
        Self { data }
    }
    fn zero() -> Self {
        Self {
            data: vector::zero(),
        }
    }
    fn identity() -> Self {
        Self {
            data: vector::zero(),
        }
    }
    fn is_zero(&self) -> bool {
        vector::is_zero(&self.data)
    }
    fn set_zero(&mut self) {
        vector::set_zero(&mut self.data)
    }
    fn transpose(&self) -> Self {
        Self::from_array(matrix::transpose::<F, 16, 4, 4>(self.data))
    }
    fn determinant(&self) -> F {
        matrix::determinant4(&self.data)
    }
    fn inverse(&self) -> Self {
        Self::from_array(matrix::inverse4(&self.data))
    }
    fn transform(&self, v: &FArray<F, 4>) -> FArray<F, 4> {
        FArray::from_array(matrix::multiply::<F, 16, 4, 4, 4, 4, 1>(
            &self.data,
            v.as_ref(),
        ))
    }
}

//ip SqMatrix4<F> for FArray2
impl<F: Float> SqMatrix4<F, FArray<F, 3>, FArray<F, 4>> for FArray2<F, 4, 16> {
    fn perspective(fov: F, aspect: F, near: F, far: F) -> Self {
        Self::from_array(matrix::perspective4(fov, aspect, near, far))
    }
    fn look_at(eye: &FArray<F, 3>, center: &FArray<F, 3>, up: &FArray<F, 3>) -> Self {
        Self::from_array(matrix::look_at4(eye.as_ref(), center.as_ref(), up.as_ref()))
    }
    fn translate3(&mut self, by: &FArray<F, 3>) {
        self.data[3] += by[0];
        self.data[7] += by[1];
        self.data[11] += by[2];
    }
    fn translate4(&mut self, by: &FArray<F, 4>) {
        self.data[3] += by[0];
        self.data[7] += by[1];
        self.data[11] += by[2];
    }
}
