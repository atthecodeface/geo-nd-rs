//a Imports
use super::{Float, Vector, SqMatrix, Vector3D, Geometry3D};
use super::{FSlice, vector, matrix};

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

//a FSlice2
//tp FSlice2
#[derive(Clone, Copy, Debug)]
pub struct FSlice2 <F:Float, const D:usize, const D2:usize> { data: [F;D2] }

//ip FSlice2
index_ops! { FSlice2 }
ref_op! { FSlice2, [F;D2] }
ref_op! { FSlice2, [F] }
binary_op! { FSlice2, Add, add, +, AddAssign, add_assign, += }
binary_op! { FSlice2, Sub, sub, -, SubAssign, sub_assign, -= }
binary_op! { FSlice2, Mul, mul, *, MulAssign, mul_assign, *= }
binary_op! { FSlice2, Div, div, /, DivAssign, div_assign, /= }

impl <F:Float, const D:usize, const D2:usize>   std::default::Default for FSlice2<F, D, D2> {
    fn default() -> Self { Self { data:vector::zero() } }
}

//ip SqMatrix<F,D,D2> for FSlice2
impl <F:Float> SqMatrix<FSlice<F,2>, F, 2, 4> for FSlice2<F, 2, 4> {
    fn from_array(data:[F;4]) -> Self { Self { data  } }
    fn zero() -> Self { Self { data:vector::zero() }    }
    fn identity() -> Self { Self { data:vector::zero() }   }
    fn is_zero(&self) -> bool { vector::is_zero(&self.data)    }
    fn set_zero(&mut self) { vector::set_zero(&mut self.data)    }
    fn transpose(&self) -> Self {
        Self::from_array(matrix::transpose::<F,4,2,2> (self.data) )
    }
    fn determinant(&self) -> F { matrix::determinant2(&self.data) }
    fn inverse(&self) -> Self {
        Self::from_array(matrix::inverse2(&self.data))
    }
    fn transform(&self, v:FSlice<F,2>) -> FSlice<F,2> {
        FSlice::from_array(matrix::multiply::<F,4,2,2,2,2,1> (&self.data, v.as_ref()))
    }
}

impl <F:Float> SqMatrix<FSlice<F,3>, F, 3, 9> for FSlice2<F, 3, 9> {
    fn from_array(data:[F;9]) -> Self { Self { data  } }
    fn zero() -> Self { Self { data:vector::zero() }    }
    fn identity() -> Self { Self { data:vector::zero() }   }
    fn is_zero(&self) -> bool { vector::is_zero(&self.data)    }
    fn set_zero(&mut self) { vector::set_zero(&mut self.data)    }
    fn transpose(&self) -> Self {
        Self::from_array(matrix::transpose::<F,9,3,3> (self.data) )
    }
    fn determinant(&self) -> F { matrix::determinant3(&self.data) }
    fn inverse(&self) -> Self {
        Self::from_array(matrix::inverse3(&self.data))
    }
    fn transform(&self, v:FSlice<F,3>) -> FSlice<F,3> {
        FSlice::from_array(matrix::multiply::<F,9,3,3,3,3,1> (&self.data, v.as_ref()))
    }
}

impl <F:Float> SqMatrix<FSlice<F,4>, F, 4, 16> for FSlice2<F, 4, 16> {
    fn from_array(data:[F;16]) -> Self { Self { data  } }
    fn zero() -> Self { Self { data:vector::zero() }    }
    fn identity() -> Self { Self { data:vector::zero() }   }
    fn is_zero(&self) -> bool { vector::is_zero(&self.data)    }
    fn set_zero(&mut self) { vector::set_zero(&mut self.data)    }
    fn transpose(&self) -> Self {
        Self::from_array(matrix::transpose::<F,16,4,4> (self.data) )
    }
    fn determinant(&self) -> F { matrix::determinant4(&self.data) }
    fn inverse(&self) -> Self {
        Self::from_array(matrix::inverse4(&self.data))
    }
    fn transform(&self, v:FSlice<F,4>) -> FSlice<F,4> {
        FSlice::from_array(matrix::multiply::<F,16,4,4,4,4,1> (&self.data, v.as_ref()))
    }
}

