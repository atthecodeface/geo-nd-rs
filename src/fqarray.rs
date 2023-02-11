//a Imports
use serde::{Deserialize, Serialize};

use crate::{FArray, FArray2};
use crate::{Float, QArray, Quaternion, SqMatrix, Transform, Vector};

//tp FQArrayTrans
/// A transformation that is a rotation, translation and scaling
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct FQArrayTrans<F: Float + Serialize> {
    /// Quaternion of the rotation
    quat: QArray<F, FArray<F, 3>, FArray<F, 4>>,
    /// Translation and scaling
    trans_scale: FArray<F, 4>,
}

//ip FQArrayTrans
impl<F: Float + Serialize> std::default::Default for FQArrayTrans<F> {
    fn default() -> Self {
        Self {
            quat: QArray::default(),
            trans_scale: FArray::from_array([F::zero(), F::zero(), F::zero(), F::one()]),
        }
    }
}

//ip Display for FQArrayTrans<F>
impl<F> std::fmt::Display for FQArrayTrans<F>
where
    F: Float + Serialize,
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

//ip FQArrayTrans
impl<F: Float + Serialize>
    Transform<
        F,
        FArray<F, 3>,
        FArray<F, 4>,
        FArray2<F, 4, 16>,
        QArray<F, FArray<F, 3>, FArray<F, 4>>,
    > for FQArrayTrans<F>
{
    fn of_trs(t: FArray<F, 3>, r: QArray<F, FArray<F, 3>, FArray<F, 4>>, s: F) -> Self {
        Self {
            quat: r,
            trans_scale: FArray::from_array([t[0], t[1], t[2], s]),
        }
    }
    fn get_scale(&self) -> F {
        self.trans_scale[3]
    }
    fn get_translation(&self) -> FArray<F, 3> {
        FArray::from_array([
            self.trans_scale[0],
            self.trans_scale[1],
            self.trans_scale[2],
        ])
    }
    fn get_rotation(&self) -> QArray<F, FArray<F, 3>, FArray<F, 4>> {
        self.quat
    }
    fn inverse(&self) -> Self {
        let scale = self.trans_scale[3];
        if scale.abs() < F::epsilon() {
            Self::default()
        } else {
            let scale = F::one() / scale;
            let trans = FArray::from_array([
                self.trans_scale[0],
                self.trans_scale[1],
                self.trans_scale[2],
            ]);
            let iquat = self.quat.conjugate();
            let trans = -(iquat.apply3(&trans)) * scale;
            Self::of_trs(trans, iquat, scale)
        }
    }
    fn invert(&mut self) {
        *self = self.inverse();
    }
    fn as_mat(&self) -> FArray2<F, 4, 16> {
        let mut m = FArray2::<F, 4, 16>::zero();
        self.quat.set_rotation4(&mut m);
        m *= self.trans_scale[3];
        m[3] = self.trans_scale[0];
        m[7] = self.trans_scale[1];
        m[11] = self.trans_scale[2];
        m
    }
}
