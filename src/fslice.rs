//a Imports
use super::{Float, Vector, SqMatrix, Vector3D, Geometry3D};
use super::{vector, matrix};

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

//mi binary_op!
macro_rules! binary_op {
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

//a FSlice
//tp FSlice
#[derive(Clone, Copy, Debug)]
pub struct FSlice<F:Float, const D:usize> { data: [F;D] }

//ip FSlice
index_ops! { FSlice }
ref_op! { FSlice, [F;D] }
ref_op! { FSlice, [F] }
binary_op! { FSlice, Add, add, +, AddAssign, add_assign, += }
binary_op! { FSlice, Sub, sub, -, SubAssign, sub_assign, -= }
binary_op! { FSlice, Mul, mul, *, MulAssign, mul_assign, *= }
binary_op! { FSlice, Div, div, /, DivAssign, div_assign, /= }

impl <F:Float, const D:usize> std::ops::Neg for FSlice<F, D> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut data = [F::zero();D];
        for i in 0..D { data[i] = -self.data[i]; }
        Self { data }
    }
}
impl <F:Float, const D:usize> std::default::Default for FSlice<F, D> {
    fn default() -> Self { Self { data:vector::zero() } }
}

//ip Vector<F,D> for FSlice
impl <F:Float, const D:usize> Vector<F, D> for FSlice<F, D> {
    fn from_array(data:[F;D]) -> Self { Self { data  } }
    fn zero() -> Self {
        Self { data:vector::zero() }
    }
    fn is_zero(&self) -> bool {
        vector::is_zero(&self.data)
    }
    fn set_zero(&mut self) {
        vector::set_zero(&mut self.data)
    }
    fn mix(&self, other:&Self, t:F) -> Self {
        Self { data:vector::mix(&self.data, &other.data, t) }
    }
    fn reduce_sum(&self) -> F {
        let mut r = F::zero();
        for d in self.data { r = r + d }
        r
    }
    fn dot(&self, other:&Self) -> F {
        vector::dot(&self.data, &other.data)
    }
}

