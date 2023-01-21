//a Imports
use super::{quat, vector};
use super::{Float, Quaternion, SqMatrix, Vector};
use std::marker::PhantomData;

//a Macros
//mi binary_op!
macro_rules! binary_op {
    { op_self, $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
        impl <F, V3, V4> std::ops::$trait_op<Self> for QArray<F, V3, V4>
        where
            F:Float,
            V3:Vector<F,3>, V4:Vector<F,4>,    {
            type Output = Self;
            fn $op(self, other: Self) -> Self {
                let mut data = [F::zero();4];
                for i in 0..4 {
                    data[i] = self.data[i] $binop other.data[i];
                }
                Self::from_array(data)
            }
        }
        impl <F, V3, V4> std::ops::$trait_assign_op<Self> for QArray<F, V3, V4>
        where
            F:Float,
            V3:Vector<F,3>, V4:Vector<F,4>,    {
            fn $assign_op(&mut self, other: Self) {
                for i in 0..4 {self.data[i] $assign_binop other.data[i];}
            }
        }
    };
    { op_f, $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
        impl <F, V3, V4> std::ops::$trait_op<F> for QArray<F, V3, V4>
        where
            F:Float,
            V3:Vector<F,3>, V4:Vector<F,4>,    {
            type Output = Self;
            fn $op(self, other: F) -> Self {
                let mut data = [F::zero();4];
                for i in 0..4 {
                    data[i] = self.data[i] $binop other;
                }
                Self::from_array(data)
            }
        }
        impl <F, V3, V4> std::ops::$trait_assign_op<F> for QArray<F, V3, V4>
        where
            F:Float,
            V3:Vector<F,3>, V4:Vector<F,4>,    {
            fn $assign_op(&mut self, other: F) {
                for i in 0..4 {self.data[i] $assign_binop other;}
            }
        }
    };
    { $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
        binary_op! { op_self, $trait_op, $op, $binop, $trait_assign_op, $assign_op, $assign_binop }
        binary_op! { op_f,    $trait_op, $op, $binop, $trait_assign_op, $assign_op, $assign_binop }
    };
}

//a QArray
//tp QArray
/// The [QArray] is a wrapper around a `D` sized array of [Float]s.
///
/// It provides implementations of the traits required for a [Vector]
/// trait, hence it can be used for a [Vector] of any size `D`.
#[derive(Clone, Copy, Debug)]
pub struct QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    data: V4,
    f: PhantomData<F>,
    v3: PhantomData<V3>,
}

//ip QArray
impl<F, V3, V4> QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    pub fn of_vector(data: V4) -> Self {
        Self {
            data,
            f: PhantomData,
            v3: PhantomData,
        }
    }
}
impl<F, V3, V4> std::convert::AsRef<[F; 4]> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    fn as_ref(&self) -> &[F; 4] {
        self.data.as_ref()
    }
}
impl<F, V3, V4> std::convert::AsMut<[F; 4]> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    fn as_mut(&mut self) -> &mut [F; 4] {
        self.data.as_mut()
    }
}

impl<F, V3, V4> std::convert::AsRef<[F]> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    fn as_ref(&self) -> &[F] {
        self.data.as_ref()
    }
}
impl<F, V3, V4> std::convert::AsMut<[F]> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    fn as_mut(&mut self) -> &mut [F] {
        self.data.as_mut()
    }
}

impl<F, V3, V4> std::ops::Index<usize> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    type Output = F;
    fn index(&self, index: usize) -> &F {
        let slice: &[_] = self.as_ref();
        &slice[index]
    }
}

impl<F, V3, V4> std::ops::IndexMut<usize> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    fn index_mut(&mut self, index: usize) -> &mut F {
        let slice: &mut [_] = self.as_mut();
        &mut slice[index]
    }
}

binary_op! { Add, add, +, AddAssign, add_assign, += }
binary_op! { Sub, sub, -, SubAssign, sub_assign, -= }
binary_op! { op_f, Mul, mul, *, MulAssign, mul_assign, *= }
binary_op! { op_f, Div, div, /, DivAssign, div_assign, /= }

//ip Neg for QArray<F>
impl<F, V3, V4> std::ops::Neg for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut data = [F::zero(); 4];
        for i in 0..4 {
            data[i] = -self.data[i];
        }
        Self::from_array(data)
    }
}

//ip Default for QArray<F>
impl<F, V3, V4> std::default::Default for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    fn default() -> Self {
        Self::of_vector(V4::zero())
    }
}

//ip Display for QArray<F>
impl<F, V3, V4> std::fmt::Display for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.data, f)
    }
}

//ip std::ops::Mul<Self> for QArray<F>
impl<F, V3, V4> std::ops::Mul<Self> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let data = quat::multiply(self.data.as_ref(), other.data.as_ref());
        Self::of_vector(V4::from_array(data))
    }
}

//ip std::ops::MulAssign<Self> for QArray<F>
impl<F, V3, V4> std::ops::MulAssign<Self> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    fn mul_assign(&mut self, other: Self) {
        let data = quat::multiply(self.data.as_ref(), other.data.as_ref());
        *(self.data.as_mut()) = data;
    }
}

//ip std::ops::Div<Self> for QArray<F>
impl<F, V3, V4> std::ops::Div<Self> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let data = quat::divide(self.data.as_ref(), other.data.as_ref());
        Self::of_vector(V4::from_array(data))
    }
}

//ip std::ops::DivAssign<Self> for QArray<F>
impl<F, V3, V4> std::ops::DivAssign<Self> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    fn div_assign(&mut self, other: Self) {
        let data = quat::divide(self.data.as_ref(), other.data.as_ref());
        *(self.data.as_mut()) = data;
    }
}

//ip Quaternion<F> for QArray
impl<F, V3, V4> Quaternion<F, V3, V4> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    fn from_array(data: [F; 4]) -> Self {
        Self::of_vector(V4::from_array(data))
    }
    fn as_rijk(&self) -> (F, F, F, F) {
        quat::as_rijk(self.data.as_ref())
    }
    fn of_rijk(r: F, i: F, j: F, k: F) -> Self {
        Self::from_array(quat::of_rijk(r, i, j, k))
    }
    fn unit() -> Self {
        Self::of_rijk(F::one(), F::zero(), F::zero(), F::zero())
    }
    fn set_zero(&mut self) {
        self.data.set_zero();
    }
    fn mix(&self, other: &Self, t: F) -> Self {
        Self::of_vector(self.data.mix(&other.data, t))
    }
    fn dot(&self, other: &Self) -> F {
        vector::dot(self.as_ref(), other.as_ref())
    }
    //fp of_rotation3
    /// Find the quaternion of a Matrix3 assuming it is purely a rotation
    fn of_rotation3<M>(rotation: &M) -> Self
    where
        M: SqMatrix<V3, F, 3, 9>,
    {
        Self::from_array(quat::of_rotation(rotation.as_ref()))
    }
    fn set_rotation3<M>(&self, matrix: &mut M)
    where
        M: SqMatrix<V3, F, 3, 9>,
    {
        quat::to_rotation3(self.as_ref(), matrix.as_mut())
    }
    fn set_rotation4<M>(&self, matrix: &mut M)
    where
        M: SqMatrix<V4, F, 4, 16>,
    {
        quat::to_rotation4(self.as_ref(), matrix.as_mut())
    }
}

//ip From<[F;D]> for QArray
impl<F, V3, V4> From<[F; 4]> for QArray<F, V3, V4>
where
    F: Float,
    V3: Vector<F, 3>,
    V4: Vector<F, 4>,
{
    fn from(data: [F; 4]) -> Self {
        Self::from_array(data)
    }
}
