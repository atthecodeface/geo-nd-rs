//a Imports
use serde::Serialize;

use crate::FArray;
use crate::{Float, QArray, Quaternion, SqMatrix4, Transform, Vector};

//tp FQArrayTrans
/// A transformation that is a translation . scaling . rotation
/// (i.e. it applies the rotation to an object, then scales it, then
/// translates it)
///
/// This should probably mirror the QArray in using an F, V3 and V4.
#[derive(Clone, Copy, Debug)] //, Serialize, Deserialize)]
pub struct FQArrayTrans<F>
where
    F: Float + Serialize,
    FArray<F, 4>: Vector<F, 4>,
    QArray<F>: Quaternion<F>,
{
    /// Quaternion of the rotation
    quat: QArray<F>,
    /// Translation and scaling
    trans_scale: [F; 4],
}

//mi fqarray_basic_traits!
macro_rules! fqarray_basic_traits {
    { $f:ty, $ty:ty } => {

        //ip Default
        impl std::default::Default for $ty
        {
            fn default() -> Self {
                use num_traits::{One, Zero};
                Self {
                    quat: QArray::default(),
                    trans_scale: [<$f>::zero(), <$f>
                                  ::zero(), <$f>::zero(), <$f>::one()]
                }
            }
        }

        //ip Display
        impl std::fmt::Display for $ty
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "trans[+({},{},{}) rot{} *{}]",
                    self.trans_scale[0],
                    self.trans_scale[1],
                    self.trans_scale[2],
                    self.quat,
                    self.trans_scale[3],
                )
            }
        }
    }
}

//mi fqarray_transform_trait!
macro_rules! fqarray_transform_trait {
    { $f:ty,  $v:ty, $q:ty, $ty:ty } => {

        //ip Transform for FQArrayTrans
        impl Transform<$f, $v, $q> for $ty
        {
            //fp of_trs
            /// Create an [FQArrayTrans] from a Vector3 translation, Quat
            /// rotation and Float scaling
            fn of_trs(t: $v, r: $q, s: $f) -> Self {
                Self {
                    quat: r,
                    trans_scale: [t[0], t[1], t[2], s],
                }
            }

            //mp scale
            /// Get the scaling of the transformation
            fn scale(&self) -> $f {
                self.trans_scale[3]
            }

            //mp translation
            /// Get the translation of the transformation
            fn translation(&self) -> $v {
                [
                    self.trans_scale[0],
                    self.trans_scale[1],
                    self.trans_scale[2],
                ]
                    .into()
            }

            //mp rotation
            /// Get the rotation of the transformation
            fn rotation(&self) -> QArray<$f> {
                self.quat
            }

            //cp inverse
            /// Get a transformation that is the inverse of this
            fn inverse(&self) -> Self {
                use num_traits::{Float, One};
                let scale = self.trans_scale[3];
                if scale.abs() < <$f>::epsilon() {
                    Self::default()
                } else {
                    let scale = <$f>::one() / scale;
                    let trans: $v  = [
                        self.trans_scale[0],
                        self.trans_scale[1],
                        self.trans_scale[2],
                    ]
                        .into();
                    let iquat = self.quat.conjugate();
                    let trans = iquat.apply3(&trans);
                    let trans = trans * -scale;
                    Self::of_trs(trans, iquat, scale)
                }
            }

            //mp invert
            /// Invert this transformation
            fn invert(&mut self) {
                *self = self.inverse();
            }

            //mp as_mat
            /// Return the matrix
            fn as_mat<M: SqMatrix4<$f>>(&self) -> M {
                let mut m = M::default();
                self.quat.set_rotation4(&mut m);
                m *= self.trans_scale[3];
                m[3] = self.trans_scale[0];
                m[7] = self.trans_scale[1];
                m[11] = self.trans_scale[2];
                m
            }
        }
    }
}

//a New
//mi fqarray_traits
macro_rules! fqarray_traits {
    { $f:ty,  $v:ty, $q:ty, $ty:ty } => {
        fqarray_basic_traits! {$f, $ty}
        fqarray_transform_trait! {$f, $v, $q, $ty}
    }
}

fqarray_traits! {f32, FArray<f32,3>,  QArray<f32>, FQArrayTrans<f32>}
fqarray_traits! {f64, FArray<f64,3>,  QArray<f64>, FQArrayTrans<f64>}
