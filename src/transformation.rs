//a Imports
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{Float, Quaternion, SqMatrix4, Transform, Vector3};

//tp FQArrayTrans
/// A transformation that is a translation . rotation . scaling
/// (i.e. it applies a scaling to an object, then the rotation then
/// translates it)
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Transformation<F, V3, Q>
where
    F: Float,
    V3: Vector3<F>,
    Q: Quaternion<F>,
{
    /// Quaternion of the rotation
    rotation: Q,
    /// Translation
    translation: V3,
    /// Scaling, for all three axes
    scale: V3,
    phantom: PhantomData<F>,
}

impl<F, V3, Q> Transformation<F, V3, Q>
where
    F: Float,
    V3: Vector3<F>,
    Q: Quaternion<F>,
{
    fn is_uniform_scale(&self) -> bool {
        self.scale[0] == self.scale[1] && self.scale[0] == self.scale[2]
    }
}

impl<F, V3, Q> std::default::Default for Transformation<F, V3, Q>
where
    F: Float,
    V3: Vector3<F>,
    Q: Quaternion<F>,
{
    fn default() -> Self {
        Self {
            rotation: Q::default(),
            translation: V3::default(),
            scale: [F::ONE; 3].into(),
            phantom: PhantomData,
        }
    }
}

impl<F, V3, Q> std::fmt::Display for Transformation<F, V3, Q>
where
    F: Float,
    V3: Vector3<F>,
    Q: Quaternion<F>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "trans[+({},{},{}) rot{} *{}]",
            self.translation[0],
            self.translation[1],
            self.translation[2],
            self.rotation,
            self.scale,
        )
    }
}

impl<F, V3, Q> Transform<F, V3, Q> for Transformation<F, V3, Q>
where
    F: Float,
    V3: Vector3<F>,
    Q: Quaternion<F>,
{
    fn of_trs<A: AsRef<[F; 3]>>(t: A, r: Q, s: A) -> Self {
        Self {
            rotation: r,
            translation: t.as_ref().into(),
            scale: s.as_ref().into(),
            phantom: PhantomData,
        }
    }

    fn of_trsu<A: AsRef<[F; 3]>>(t: A, r: Q, s: F) -> Self {
        Self {
            rotation: r,
            translation: t.as_ref().into(),
            scale: [s; 3].into(),
            phantom: PhantomData,
        }
    }

    fn scale(&self) -> V3 {
        self.scale
    }

    fn uniform_scale(&self) -> Option<F> {
        if self.is_uniform_scale() {
            Some(self.scale[0])
        } else {
            None
        }
    }

    fn translation(&self) -> V3 {
        self.translation
    }

    fn rotation(&self) -> Q {
        self.rotation
    }

    fn set_scale<A: AsRef<[F; 3]>>(&mut self, scale: A) {
        self.scale = scale.as_ref().into();
    }

    fn set_uniform_scale(&mut self, scale: F) {
        self.scale = [scale; 3].into();
    }

    fn set_translation<A: AsRef<[F; 3]>>(&mut self, translation: A) {
        self.translation = translation.as_ref().into();
    }

    fn set_rotation(&mut self, rotation: Q) {
        self.rotation = rotation;
    }

    fn scale_by(&mut self, scale: F) {
        self.translation = self.translation * scale;
        self.scale = self.scale * scale;
    }
    fn translate_by<A: AsRef<[F; 3]>>(&mut self, translation: A, scale: F) {
        let translation: V3 = translation.as_ref().into();
        self.translation = self.translation + translation * scale;
    }

    fn rotate_by(&mut self, quaternion: Q) {
        self.translation = quaternion.apply3(&self.translation);
        self.rotation = quaternion * self.rotation;
    }
    fn transform_by(&mut self, transformer: &Self) -> bool {
        if !transformer.is_uniform_scale() {
            return false;
        }
        self.scale = self.scale * transformer.scale[0];
        self.translation = self.translation * transformer.scale[0];
        self.rotation = transformer.rotation * self.rotation;
        self.translation = transformer.rotation.apply3(&self.translation) + transformer.translation;
        true
    }

    fn inverse(&self) -> Option<Self> {
        if !self.is_uniform_scale() {
            return None;
        }
        let scale = self.scale[0];
        if scale.abs() < F::epsilon() {
            Some(Self::default())
        } else {
            let scale = scale.recip();
            let iquat = self.rotation.conjugate();
            let trans = iquat.apply3(&self.translation);
            let trans = trans * -scale;
            Some(Self::of_trs(trans, iquat, [scale; 3].into()))
        }
    }

    fn invert(&mut self) -> bool {
        if !self.is_uniform_scale() {
            return false;
        }
        *self = self.inverse().unwrap();
        true
    }

    fn as_mat<M: SqMatrix4<F>>(&self) -> M {
        let mut m = M::default();
        self.rotation.set_rotation4(&mut m);
        for c in m.iter_mut().take(3) {
            *c = *c * self.scale[0];
        }
        for c in m.iter_mut().skip(4).take(3) {
            *c = *c * self.scale[1];
        }
        for c in m.iter_mut().skip(8).take(3) {
            *c = *c * self.scale[2];
        }
        m[3] = self.translation[0];
        m[7] = self.translation[1];
        m[11] = self.translation[2];
        m
    }
}
