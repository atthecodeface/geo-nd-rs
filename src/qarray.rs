//a Imports
use super::{Float, Vector, SqMatrix, Quaternion};
use super::{vector, quat};

//a Macros
//mi index_ops!
macro_rules! index_ops {
    { $t:ident } => {
        impl <F:Float> std::ops::Index<usize> for $t <F> {
            type Output = F;
            fn index(&self, index: usize) -> &F {
                let slice: &[_] = self.as_ref();
                &slice[index]
            }
        }
        impl <F:Float> std::ops::IndexMut<usize> for $t <F> {
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
        impl <F:Float> std::convert::AsRef<$t_as> for $t <F> {
            fn as_ref(&self) -> &$t_as {&self.data}
        }
        impl <F:Float> std::convert::AsMut<$t_as> for $t <F> {
            fn as_mut(&mut self) -> &mut $t_as {&mut self.data}
        }
    }
}

//mi binary_op!
macro_rules! binary_op {
    { op_self, $t:ident, $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
        impl <F:Float> std::ops::$trait_op<Self> for $t<F> {
            type Output = Self;
            fn $op(self, other: Self) -> Self {
                let mut data = [F::zero();4];
                for i in 0..4 {
                    data[i] = self.data[i] $binop other.data[i];
                }
                Self { data }
            }
        }
        impl <F:Float> std::ops::$trait_assign_op<Self> for $t<F> {
            fn $assign_op(&mut self, other: Self) {
                for i in 0..4 {self.data[i] $assign_binop other.data[i];}
            }
        }
    };
    { op_f, $t:ident, $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
        impl <F:Float> std::ops::$trait_op<F> for $t<F> {
            type Output = Self;
            fn $op(self, other: F) -> Self {
                let mut data = [F::zero();4];
                for i in 0..4 {
                    data[i] = self.data[i] $binop other;
                }
                Self { data }
            }
        }
        impl <F:Float> std::ops::$trait_assign_op<F> for $t<F> {
            fn $assign_op(&mut self, other: F) {
                for i in 0..4 {self.data[i] $assign_binop other;}
            }
        }
    };
    { $t:ident, $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
        binary_op! { op_self, $t, $trait_op, $op, $binop, $trait_assign_op, $assign_op, $assign_binop }
        binary_op! { op_f,    $t, $trait_op, $op, $binop, $trait_assign_op, $assign_op, $assign_binop }
    };
}

//a QArray
//tp QArray
/// The [QArray] is a wrapper around a `D` sized array of [Float]s.
///
/// It provides implementations of the traits required for a [Vector]
/// trait, hence it can be used for a [Vector] of any size `D`.
#[derive(Clone, Copy, Debug)]
pub struct QArray<F:Float> { data: [F;4] }

//ip QArray
index_ops! { QArray }
ref_op! { QArray, [F;4] }
ref_op! { QArray, [F] }
binary_op! { QArray, Add, add, +, AddAssign, add_assign, += }
binary_op! { QArray, Sub, sub, -, SubAssign, sub_assign, -= }
binary_op! { op_f, QArray, Mul, mul, *, MulAssign, mul_assign, *= }
binary_op! { op_f, QArray, Div, div, /, DivAssign, div_assign, /= }

//ip Neg for QArray<F>
impl <F:Float> std::ops::Neg for QArray<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut data = [F::zero();4];
        for i in 0..4 { data[i] = -self.data[i]; }
        Self { data }
    }
}

//ip Default for QArray<F>
impl <F:Float> std::default::Default for QArray<F> {
    fn default() -> Self { Self { data:vector::zero() } }
}

//ip Display for QArray<F>
impl <F:Float> std::fmt::Display for QArray<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        vector::fmt(f, &self.data)
    }
}

//ip std::ops::Mul<Self> for QArray<F>
impl <F:Float> std::ops::Mul<Self> for QArray<F> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let data = quat::multiply(&self.data, &other.data);
        Self { data }
    }
}

//ip std::ops::MulAssign<Self> for QArray<F>
impl <F:Float> std::ops::MulAssign<Self> for QArray<F> {
    fn mul_assign(&mut self, other: Self) {
        let data = quat::multiply(&self.data, &other.data);
        self.data = data;
    }
}

//ip std::ops::Div<Self> for QArray<F>
impl <F:Float> std::ops::Div<Self> for QArray<F> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let data = quat::divide(&self.data, &other.data);
        Self { data }
    }
}

//ip std::ops::DivAssign<Self> for QArray<F>
impl <F:Float> std::ops::DivAssign<Self> for QArray<F> {
    fn div_assign(&mut self, other: Self) {
        let data = quat::divide(&self.data, &other.data);
        self.data = data;
    }
}

//ip Quaternion<F> for QArray
impl <F:Float> Quaternion<F> for QArray<F> {
    fn from_array(data:[F;4]) -> Self {
        Self { data  }
    }
    fn as_rijk(&self) -> (F, F, F, F) {
        quat::as_rijk(&self.data)
    }
    fn of_rijk(r:F, i:F, j:F, k:F) -> Self {
        Self { data:quat::of_rijk(r,i,j,k) }
    }
    fn unit() -> Self {
        Self { data:quat::of_rijk(F::one(),F::zero(),F::zero(),F::zero()) }
    }
    fn set_zero(&mut self) {
        vector::set_zero(&mut self.data)
    }
    fn mix(&self, other:&Self, t:F) -> Self {
        Self { data:vector::mix(&self.data, &other.data, t) }
    }
    fn dot(&self, other:&Self) -> F {
        vector::dot(&self.data, &other.data)
    }
    //fp of_rotation3
    /// Find the quaternion of a Matrix3 assuming it is purely a rotation
    fn of_rotation3<M,V> (rotation:&M) -> Self
    where V:Vector<F,3>,M:SqMatrix<V, F, 3, 9> {
        Self { data:quat::of_rotation(rotation.as_ref()) }
    }
    fn set_rotation3<M,V> (&self, matrix:&mut M)
    where V:Vector<F,3>,M:SqMatrix<V, F, 3, 9> {
        quat::to_rotation3(&self.data, matrix.as_mut())
    }
    fn set_rotation4<M,V> (&self, matrix:&mut M)
    where V:Vector<F,4>,M:SqMatrix<V, F, 4, 16> {
        quat::to_rotation4(&self.data, matrix.as_mut())
    }
        
}

