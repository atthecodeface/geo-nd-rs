use crate::{Vector, Vector3D};

//a Macros
macro_rules! binary_op {
    { $t:ty, $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
        impl std::ops::$trait_op<$t> for $t {
            type Output = Self; fn $op(self, other: Self) -> Self { Self(self.0 $binop other.0) }
        }
        impl std::ops::$trait_assign_op<$t> for $t {
            fn $assign_op(&mut self, other: Self) { self.0 $assign_binop other.0; }
        }
        impl std::ops::$trait_op<f32> for $t {
            type Output = Self; fn $op(self, other:f32) -> Self { Self(self.0 $binop other) }
        }
        impl std::ops::$trait_assign_op<f32> for $t {
            fn $assign_op(&mut self, other:f32) { self.0 $assign_binop other; }
        }
    }
}

macro_rules! binary_op_n {
    { $t:ty, $trait_op:ident, $op:ident, $binop:tt, $trait_assign_op:ident, $assign_op:ident, $assign_binop:tt } => {
        impl std::ops::$trait_op<$t> for $t {
            type Output = Self; fn $op(self, other: Self) -> Self { Self(self.0 $binop other.0) }
        }
        impl std::ops::$trait_assign_op<$t> for $t {
            fn $assign_op(&mut self, other: Self) { self.0 $assign_binop other.0; }
        }
        impl std::ops::$trait_op<f32> for $t {
            type Output = Self; fn $op(self, other:f32) -> Self { Self(self.0 $binop Self::of_float(other).0) }
        }
        impl std::ops::$trait_assign_op<f32> for $t {
            fn $assign_op(&mut self, other:f32) { self.0 $assign_binop Self::of_float(other).0; }
        }
    }
}

//a F32x4Vec4
#[derive(Clone, Copy, Debug)]
pub struct F32x4Vec4 (core_simd::f32x4);

//ip Add for F32x4Vec4
binary_op! { F32x4Vec4, Add, add, +, AddAssign, add_assign, += }
binary_op! { F32x4Vec4, Sub, sub, -, SubAssign, sub_assign, -= }
binary_op! { F32x4Vec4, Mul, mul, *, MulAssign, mul_assign, *= }
binary_op! { F32x4Vec4, Div, div, /, DivAssign, div_assign, /= }
impl std::convert::AsRef<[f32;4]> for F32x4Vec4 { fn as_ref(&self) -> &[f32;4] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;4]>(&self.0) } } }
impl std::convert::AsRef<[f32]> for F32x4Vec4 { fn as_ref(&self) -> &[f32] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;4]>(&self.0) } } }
impl std::convert::AsMut<[f32;4]> for F32x4Vec4 { fn as_mut(&mut self) -> &mut [f32;4] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;4]>(&mut self.0) } } }
impl std::convert::AsMut<[f32]> for F32x4Vec4 { fn as_mut(&mut self) -> &mut [f32] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;4]>(&mut self.0) } } }
impl std::default::Default for F32x4Vec4 { fn default() -> Self { Self ( core_simd::f32x4::default() ) } }
impl std::ops::Index<usize> for F32x4Vec4 { type Output = f32; fn index(&self, index: usize) -> &f32 { let x:&[f32]=self.as_ref(); &x[index] } }
impl std::ops::IndexMut<usize> for F32x4Vec4 { fn index_mut(&mut self, index: usize) -> &mut f32 { let x:&mut [f32]=self.as_mut(); &mut x[index] } }
impl std::ops::Neg for F32x4Vec4 { type Output = Self; fn neg(self) -> Self { Self (-self.0) } }

//ip Vector<f32, 4> for F32x4Vec4
impl Vector<f32, 4> for F32x4Vec4 {
    fn from_array(data:[f32;4]) -> Self { Self(core_simd::f32x4::from_array(data)) }
    fn zero() -> Self { Self(core_simd::f32x4::splat(0.)) }
    fn is_zero(&self) -> bool { self.0.lanes_eq(core_simd::f32x4::splat(0.)).all() }
    fn set_zero(&mut self)  { self.0 =  core_simd::f32x4::splat(0.) }
    fn mix(&self, other:&Self, t:f32) -> Self { Self(self.0*(1.0-t) + other.0*t) }
    fn reduce_sum(&self) -> f32 {self.0.horizontal_sum()}
    fn dot(&self, other:&Self) -> f32 {(self.0 * other.0).horizontal_sum()}
}


//a F32x4Vec3
#[derive(Clone, Copy, Debug)]
pub struct F32x4Vec3 (core_simd::f32x4);

//ip Add for F32x4Vec3
binary_op_n! { F32x4Vec3, Add, add, +, AddAssign, add_assign, += }
binary_op_n! { F32x4Vec3, Sub, sub, -, SubAssign, sub_assign, -= }
binary_op_n! { F32x4Vec3, Mul, mul, *, MulAssign, mul_assign, *= }
binary_op!   { F32x4Vec3, Div, div, /, DivAssign, div_assign, /= }
impl std::convert::AsRef<[f32;3]> for F32x4Vec3 { fn as_ref(&self) -> &[f32;3] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;3]>(&self.0) } } }
impl std::convert::AsRef<[f32]> for F32x4Vec3 { fn as_ref(&self) -> &[f32] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;3]>(&self.0) } } }
impl std::convert::AsMut<[f32;3]> for F32x4Vec3 { fn as_mut(&mut self) -> &mut [f32;3] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;3]>(&mut self.0) } } }
impl std::convert::AsMut<[f32]> for F32x4Vec3 { fn as_mut(&mut self) -> &mut [f32] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;3]>(&mut self.0) } } }
impl std::default::Default for F32x4Vec3 { fn default() -> Self { Self ( core_simd::f32x4::default() ) } }
impl std::ops::Index<usize> for F32x4Vec3 { type Output = f32; fn index(&self, index: usize) -> &f32 { let x:&[f32]=self.as_ref(); &x[index] } }
impl std::ops::IndexMut<usize> for F32x4Vec3 { fn index_mut(&mut self, index: usize) -> &mut f32 { let x:&mut [f32]=self.as_mut(); &mut x[index] } }
impl std::ops::Neg for F32x4Vec3 { type Output = Self; fn neg(self) -> Self { Self (-self.0) } }

//ip Vector<f32, 4> for F32x4Vec3
impl F32x4Vec3 {
    fn of_float(f:f32) -> Self { Self::from_array([f,f,f]) }
}
impl Vector<f32, 3> for F32x4Vec3 {
    fn from_array(data:[f32;3]) -> Self { Self(core_simd::f32x4::from_array([data[0],data[1],data[2],0.])) }
    fn zero() -> Self { Self(core_simd::f32x4::splat(0.)) }
    fn is_zero(&self) -> bool { self.0.lanes_eq(core_simd::f32x4::splat(0.)).all() }
    fn set_zero(&mut self)  { self.0 =  core_simd::f32x4::splat(0.) }
    fn mix(&self, other:&Self, t:f32) -> Self { Self(self.0*(1.0-t) + other.0*t) }
    fn reduce_sum(&self) -> f32 {self.0.horizontal_sum()}
    fn dot(&self, other:&Self) -> f32 {(self.0 * other.0).horizontal_sum()}
}

//a F32x4Vec2
#[derive(Clone, Copy, Debug)]
pub struct F32x4Vec2 (core_simd::f32x4);

//ip Add for F32x4Vec2
binary_op_n! { F32x4Vec2, Add, add, +, AddAssign, add_assign, += }
binary_op_n! { F32x4Vec2, Sub, sub, -, SubAssign, sub_assign, -= }
binary_op_n! { F32x4Vec2, Mul, mul, *, MulAssign, mul_assign, *= }
binary_op!   { F32x4Vec2, Div, div, /, DivAssign, div_assign, /= }
impl std::convert::AsRef<[f32;2]> for F32x4Vec2 { fn as_ref(&self) -> &[f32;2] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;2]>(&self.0) } } }
impl std::convert::AsRef<[f32]> for F32x4Vec2 { fn as_ref(&self) -> &[f32] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;2]>(&self.0) } } }
impl std::convert::AsMut<[f32;2]> for F32x4Vec2 { fn as_mut(&mut self) -> &mut [f32;2] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;2]>(&mut self.0) } } }
impl std::convert::AsMut<[f32]> for F32x4Vec2 { fn as_mut(&mut self) -> &mut [f32] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;2]>(&mut self.0) } } }
impl std::default::Default for F32x4Vec2 { fn default() -> Self { Self ( core_simd::f32x4::default() ) } }
impl std::ops::Index<usize> for F32x4Vec2 { type Output = f32; fn index(&self, index: usize) -> &f32 { let x:&[f32]=self.as_ref(); &x[index] } }
impl std::ops::IndexMut<usize> for F32x4Vec2 { fn index_mut(&mut self, index: usize) -> &mut f32 { let x:&mut [f32]=self.as_mut(); &mut x[index] } }
impl std::ops::Neg for F32x4Vec2 { type Output = Self; fn neg(self) -> Self { Self (-self.0) } }

//ip Vector<f32, 4> for F32x4Vec2
impl F32x4Vec2 {
    fn of_float(f:f32) -> Self { Self::from_array([f,f]) }
}
impl Vector<f32, 2> for F32x4Vec2 {
    fn from_array(data:[f32;2]) -> Self { Self(core_simd::f32x4::from_array([data[0],data[1],0.,0.])) }
    fn zero() -> Self { Self(core_simd::f32x4::splat(0.)) }
    fn is_zero(&self) -> bool { self.0.lanes_eq(core_simd::f32x4::splat(0.)).all() }
    fn set_zero(&mut self)  { self.0 =  core_simd::f32x4::splat(0.) }
    fn mix(&self, other:&Self, t:f32) -> Self { Self(self.0*(1.0-t) + other.0*t) }
    fn reduce_sum(&self) -> f32 {self.0.horizontal_sum()}
    fn dot(&self, other:&Self) -> f32 {(self.0 * other.0).horizontal_sum()}
}

//a F32x2Vec2
#[derive(Clone, Copy, Debug)]
pub struct F32x2Vec2 (core_simd::f32x2);

//ip Add for F32x2Vec2
binary_op! { F32x2Vec2, Add, add, +, AddAssign, add_assign, += }
binary_op! { F32x2Vec2, Sub, sub, -, SubAssign, sub_assign, -= }
binary_op! { F32x2Vec2, Mul, mul, *, MulAssign, mul_assign, *= }
binary_op! { F32x2Vec2, Div, div, /, DivAssign, div_assign, /= }
impl std::convert::AsRef<[f32;2]> for F32x2Vec2 { fn as_ref(&self) -> &[f32;2] {unsafe {std::mem::transmute::<&core_simd::f32x2, &[f32;2]>(&self.0) } } }
impl std::convert::AsRef<[f32]> for F32x2Vec2 { fn as_ref(&self) -> &[f32] {unsafe {std::mem::transmute::<&core_simd::f32x2, &[f32;2]>(&self.0) } } }
impl std::convert::AsMut<[f32;2]> for F32x2Vec2 { fn as_mut(&mut self) -> &mut [f32;2] {unsafe {std::mem::transmute::<&mut core_simd::f32x2, &mut [f32;2]>(&mut self.0) } } }
impl std::convert::AsMut<[f32]> for F32x2Vec2 { fn as_mut(&mut self) -> &mut [f32] {unsafe {std::mem::transmute::<&mut core_simd::f32x2, &mut [f32;2]>(&mut self.0) } } }
impl std::default::Default for F32x2Vec2 { fn default() -> Self { Self ( core_simd::f32x2::default() ) } }
impl std::ops::Index<usize> for F32x2Vec2 { type Output = f32; fn index(&self, index: usize) -> &f32 { let x:&[f32]=self.as_ref(); &x[index] } }
impl std::ops::IndexMut<usize> for F32x2Vec2 { fn index_mut(&mut self, index: usize) -> &mut f32 { let x:&mut [f32]=self.as_mut(); &mut x[index] } }
impl std::ops::Neg for F32x2Vec2 { type Output = Self; fn neg(self) -> Self { Self (-self.0) } }

//ip Vector<f32, 2> for F32x2Vec2
impl Vector<f32, 2> for F32x2Vec2 {
    fn from_array(data:[f32;2]) -> Self { Self(core_simd::f32x2::from_array(data)) }
    fn zero() -> Self { Self(core_simd::f32x2::splat(0.)) }
    fn is_zero(&self) -> bool { self.0.lanes_eq(core_simd::f32x2::splat(0.)).all() }
    fn set_zero(&mut self)  { self.0 =  core_simd::f32x2::splat(0.) }
    fn mix(&self, other:&Self, t:f32) -> Self { Self(self.0*(1.0-t) + other.0*t) }
    fn reduce_sum(&self) -> f32 {self.0.horizontal_sum()}
    fn dot(&self, other:&Self) -> f32 {(self.0 * other.0).horizontal_sum()}
}

