//a Macros that operate on 'Self.data'
//mi ref_traits - Deref/Mut, AsRef/AsMut, Index<usize>
macro_rules! ref_traits {
    { $f:ty,  $e:expr, $ty:ty } => {

        impl std::ops::Deref for $ty {
            type Target = [$f; $e];
            fn deref(&self) -> &[$f; $e] {
                &self.data
            }
        }
        impl std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut [$f; $e] {
                &mut self.data
            }
        }

        impl std::convert::AsRef<[$f; $e]> for $ty
        {
            fn as_ref(&self) -> &[$f; $e] {
                &self.data
            }
        }
        impl std::convert::AsMut<[$f; $e]> for $ty
        {
            fn as_mut(&mut self) -> &mut [$f; $e] {
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

//mi convert_traits - From<> traits
macro_rules! convert_traits {
    { $f:ty,  $e:expr, $ty:ty } => {

        impl std::convert::From<[$f; $e]> for $ty {
            fn from(data:[$f; $e]) -> $ty {
                Self { data }
            }
        }
        impl <'a> std::convert::From<&'a [$f; $e]> for $ty {
            fn from(data:&'a [$f; $e]) -> $ty {
                Self { data:*data }
            }
        }

        impl <'a> std::convert::TryFrom<&'a [$f]> for $ty {
            type Error = std::array::TryFromSliceError;
            fn try_from(data:&'a [$f]) -> Result<$ty, Self::Error> {
                let data :[$f; $e] = <[$f; $e]>::try_from(data)?;
                Ok(data.into())
            }
        }

        impl <'a> std::convert::TryFrom<Vec<$f>> for $ty {
            type Error = Vec<$f>;
            fn try_from(data:Vec<$f>) -> Result<$ty, Self::Error> {
                let data: [$f; $e] = <[$f; $e]>::try_from(data)?;
                Ok(data.into())
            }
        }

        impl std::convert::From<$ty> for [$f; $e]  {
            fn from(s: $ty) -> [$f; $e]  {
                s.data
            }
        }
        impl <'a> std::convert::From<&'a $ty> for &'a [$f; $e]  {
            fn from(s: &'a $ty) -> &'a [$f; $e]  {
                &s.data
            }
        }
        impl <'a> std::convert::From<&'a $ty> for [$f; $e]  {
            fn from(s: &'a $ty) -> [$f; $e]  {
                s.data
            }
        }

    }
}

//mi unary_traits - Neg
macro_rules! unary_traits {
    { $f:ty,  $e:expr, $ty:ty } => {

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
                for i in 0..$e {
                    r.data[i] = -self.data[i];
                }
                r
            }
        }
    }
}

//mi elementwise_traits - element-wise op and assignop
macro_rules! elementwise_traits {
    { $f:ty, $e:expr, $ty:ty,
      $trait_op:ident, $fn_op:ident, $op:tt,
      $trait_assign_op:ident, $fn_assign_op:ident, $op_assign:tt  } => {

        // Add/Sub of  &Self  for Self
        impl <'a> std::ops::$trait_op<&'a $ty> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: &'a $ty) -> $ty {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
                self
            }
        }

        // Add/Sub of  Deref to [F; D]  for Self (Covers Self + Self)
        impl<D: std::ops::Deref<Target = [$f; $e]>> std::ops::$trait_op<D> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: D) -> $ty {
                for i in 0..$e {
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
        impl<'a, D: std::ops::Deref<Target = [$f; $e]>> std::ops::$trait_op<D> for &'a $ty {
            type Output = $ty;
            fn $fn_op(self, other: D) -> $ty {
                *self $op other
            }
        }

        impl <'a> std::ops::$trait_assign_op<&'a $ty> for $ty {
            fn $fn_assign_op(&mut self, other: &'a $ty)  {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
            }
        }

        impl<D: std::ops::Deref<Target = [$f; $e]>> std::ops::$trait_assign_op<D> for $ty {
            fn $fn_assign_op(&mut self, other: D) {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
            }
        }

        impl <'a> std::ops::$trait_assign_op<&'a $ty> for &mut $ty {
            fn $fn_assign_op(&mut self, other: &'a $ty)  {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
            }
        }

        impl<D: std::ops::Deref<Target = [$f; $e]>> std::ops::$trait_assign_op<D> for &mut $ty {
            fn $fn_assign_op(&mut self, other: D) {
                for i in 0..$e {
                    self.data[i] $op_assign other[i];
                }
            }
        }
    }
}

//mi scale_by_f_traits
macro_rules! scale_by_f_traits {
    { $f:ty, $ty:ty,
      $trait_op:ident, $fn_op:ident, $op:tt,
      $trait_assign_op:ident, $fn_assign_op:ident, $op_assign:tt  } => {

        // Mul/Div by $f for Self
        impl std::ops::$trait_op<$f> for $ty {
            type Output = $ty;
            fn $fn_op(mut self, other: $f) -> $ty {
                for i in 0..4 {
                    self.data[i] $op_assign other;
                }
                self
            }
        }

        // Mul of $f for &Self
        impl <'a> std::ops::$trait_op<$f> for &'a $ty {
            type Output = $ty;
            fn $fn_op(self, other: $f) -> $ty {
                *self $op other
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

//mi serialize_traits
macro_rules! serialize_traits {
    { $f:ty,  $e:expr, $ty:ty } => {

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
                let array = <[$f; $e]>::deserialize(deserializer)?;
                Ok(array.into())
            }
        }

    }
}

//a Export them
pub(crate) use convert_traits;
pub(crate) use elementwise_traits;
pub(crate) use ref_traits;
pub(crate) use scale_by_f_traits;
pub(crate) use serialize_traits;
pub(crate) use unary_traits;
