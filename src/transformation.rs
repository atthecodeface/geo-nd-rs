//a Imports
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{
    FArray, FArray2, Float, QArray, Quaternion, SqMatrix, SqMatrix3, SqMatrix4, Transform, Vector3,
    vector,
};

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

impl<F, V3, Q> Transform<F> for Transformation<F, V3, Q>
where
    F: Float,
    V3: Vector3<F>,
    Q: Quaternion<F>,
{
    const UNIFORM_SCALING: bool = false;
    type Vector = V3;
    type Quat = Q;
    fn of_trs<A: AsRef<[F; 3]>>(t: A, r: Q, s: A) -> Option<Self> {
        Some(Self {
            rotation: r,
            translation: t.as_ref().into(),
            scale: s.as_ref().into(),
            phantom: PhantomData,
        })
    }

    fn of_trsu<A: AsRef<[F; 3]>>(t: A, r: Q, s: F) -> Self {
        Self {
            rotation: r,
            translation: t.as_ref().into(),
            scale: [s; 3].into(),
            phantom: PhantomData,
        }
    }

    fn is_uniform_scale(&self) -> bool {
        self.scale[0] == self.scale[1] && self.scale[0] == self.scale[2]
    }

    fn scale(&self) -> Option<Self::Vector> {
        Some(self.scale)
    }

    fn uniform_scale(&self) -> Option<F> {
        if self.is_uniform_scale() {
            Some(self.scale[0])
        } else {
            None
        }
    }

    fn translation(&self) -> Self::Vector {
        self.translation
    }

    fn rotation(&self) -> Option<Q> {
        Some(self.rotation)
    }

    fn set_identity(&mut self) {
        self.rotation.set_identity();
        self.translation = V3::default();
        self.scale = [F::ONE; 3].into();
    }

    fn set_scale<A: AsRef<[F; 3]>>(&mut self, scale: A) -> bool {
        self.scale = scale.as_ref().into();
        true
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

    fn scale_uniform_by(&mut self, scale: F) {
        self.translation *= scale;
        self.scale *= scale;
    }

    fn scale_by<A: AsRef<[F; 3]>>(&mut self, scale: A) -> bool {
        if self.rotation.as_rijk().0 != F::ONE {
            return false;
        }
        let scale = scale.as_ref();
        self.translation[0] = self.translation[0] * scale[0];
        self.translation[1] = self.translation[1] * scale[1];
        self.translation[2] = self.translation[2] * scale[2];
        self.scale[0] = self.scale[0] * scale[0];
        self.scale[1] = self.scale[1] * scale[1];
        self.scale[2] = self.scale[2] * scale[2];
        true
    }

    fn translate_by<A: AsRef<[F; 3]>>(&mut self, translation: A, scale: F) {
        let translation: V3 = translation.as_ref().into();
        self.translation += translation * scale;
    }

    fn rotate_by(&mut self, quaternion: &Q) {
        self.translation = quaternion.apply3(&self.translation);
        self.rotation = *quaternion * self.rotation;
    }

    fn transform_by<T: Transform<F, Quat = Self::Quat>>(&mut self, transformer: &T) -> bool {
        let Some(scale) = transformer.uniform_scale() else {
            return false;
        };
        let Some(rotation) = transformer.rotation() else {
            return false;
        };
        self.scale *= scale;
        self.translation *= scale;
        self.rotation = rotation * self.rotation;
        let translation: [F; 3] = transformer.translation().into();
        self.translation = rotation.apply3(&self.translation) + &translation;
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
            Self::of_trs(trans, iquat, [scale; 3].into())
        }
    }

    fn invert(&mut self) -> bool {
        if !self.is_uniform_scale() {
            return false;
        }
        *self = self.inverse().unwrap();
        true
    }
    fn apply3_arr(&self, other: &[F; 3]) -> [F; 3] {
        let v: V3 = vector::comp_mult(*self.scale, other).into();
        *(self.rotation.apply3(&v) + self.translation)
    }

    fn as_mat3<M: SqMatrix3<F>>(&self) -> M {
        let mut m = M::default();
        self.rotation.set_rotation3(&mut m);
        for (i, c) in m.iter_mut().enumerate() {
            *c = *c * self.scale[i % 3];
        }
        m
    }

    fn as_mat4<M: SqMatrix4<F>>(&self) -> M {
        let mut m = M::default();
        self.rotation.set_rotation4(&mut m);
        for (i, c) in m.iter_mut().take(12).enumerate() {
            if (i % 4) == 3 {
                *c = self.translation[i / 4];
            } else {
                *c = *c * self.scale[i % 4];
            }
        }
        m
    }
}
/// A transformation that is a translation . rotation . scaling
/// (i.e. it applies a scaling to an object, then the rotation then
/// translates it)
///
/// This supports only a uniform scaling
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TransformationUniform<F, V3, Q>
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
    scale: F,
}

impl<F, V3, Q> TransformationUniform<F, V3, Q>
where
    F: Float,
    V3: Vector3<F>,
    Q: Quaternion<F>,
{
}

impl<F, V3, Q> std::default::Default for TransformationUniform<F, V3, Q>
where
    F: Float,
    V3: Vector3<F>,
    Q: Quaternion<F>,
{
    fn default() -> Self {
        Self {
            rotation: Q::default(),
            translation: V3::default(),
            scale: F::ONE,
        }
    }
}

impl<F, V3, Q> std::fmt::Display for TransformationUniform<F, V3, Q>
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

impl<F, V3, Q> Transform<F> for TransformationUniform<F, V3, Q>
where
    F: Float,
    V3: Vector3<F>,
    Q: Quaternion<F>,
{
    const UNIFORM_SCALING: bool = true;
    type Vector = V3;
    type Quat = Q;
    fn of_trs<A: AsRef<[F; 3]>>(t: A, r: Q, s: A) -> Option<Self> {
        let s = s.as_ref();
        if s[0] != s[1] || s[0] != s[2] {
            return None;
        }
        Some(Self {
            rotation: r,
            translation: t.as_ref().into(),
            scale: s[0],
        })
    }

    fn of_trsu<A: AsRef<[F; 3]>>(t: A, r: Q, s: F) -> Self {
        Self {
            rotation: r,
            translation: t.as_ref().into(),
            scale: s,
        }
    }

    fn is_uniform_scale(&self) -> bool {
        true
    }

    fn scale(&self) -> Option<Self::Vector> {
        Some([self.scale; 3].into())
    }

    fn uniform_scale(&self) -> Option<F> {
        Some(self.scale)
    }

    fn translation(&self) -> Self::Vector {
        self.translation
    }

    fn rotation(&self) -> Option<Q> {
        Some(self.rotation)
    }

    fn set_identity(&mut self) {
        self.rotation.set_identity();
        self.translation = V3::default();
        self.scale = F::ONE;
    }

    fn set_scale<A: AsRef<[F; 3]>>(&mut self, scale: A) -> bool {
        let s = scale.as_ref();
        if s[0] != s[1] || s[0] != s[2] {
            false
        } else {
            self.scale = s[0];
            true
        }
    }

    fn set_uniform_scale(&mut self, scale: F) {
        self.scale = scale;
    }

    fn set_translation<A: AsRef<[F; 3]>>(&mut self, translation: A) {
        self.translation = translation.as_ref().into();
    }

    fn set_rotation(&mut self, rotation: Q) {
        self.rotation = rotation;
    }

    fn scale_uniform_by(&mut self, scale: F) {
        self.translation *= scale;
        self.scale = self.scale * scale;
    }

    fn scale_by<A: AsRef<[F; 3]>>(&mut self, scale: A) -> bool {
        let scale = scale.as_ref();
        if scale[0] != scale[1] || scale[0] != scale[2] {
            false
        } else {
            self.translation[0] = self.translation[0] * scale[0];
            self.translation[1] = self.translation[1] * scale[0];
            self.translation[2] = self.translation[2] * scale[0];
            self.scale = self.scale * scale[0];
            true
        }
    }

    fn translate_by<A: AsRef<[F; 3]>>(&mut self, translation: A, scale: F) {
        let translation: V3 = translation.as_ref().into();
        self.translation += translation * scale;
    }

    fn rotate_by(&mut self, quaternion: &Q) {
        self.translation = quaternion.apply3(&self.translation);
        self.rotation = *quaternion * self.rotation;
    }

    fn transform_by<T: Transform<F, Quat = Self::Quat>>(&mut self, transformer: &T) -> bool {
        let Some(scale) = transformer.uniform_scale() else {
            return false;
        };
        let Some(rotation) = transformer.rotation() else {
            return false;
        };
        self.scale = self.scale * scale;
        self.translation *= scale;
        self.rotation = rotation * self.rotation;
        let translation: [F; 3] = transformer.translation().into();
        self.translation = rotation.apply3(&self.translation) + &translation;
        true
    }

    fn inverse(&self) -> Option<Self> {
        let scale = self.scale;
        if scale.abs() < F::epsilon() {
            Some(Self::default())
        } else {
            let scale = scale.recip();
            let iquat = self.rotation.conjugate();
            let trans = iquat.apply3(&self.translation);
            let trans = trans * -scale;
            Some(Self::of_trsu(trans, iquat, scale))
        }
    }

    fn invert(&mut self) -> bool {
        *self = self.inverse().unwrap();
        true
    }
    fn apply3_arr(&self, other: &[F; 3]) -> [F; 3] {
        let v: V3 = other.into();
        *(self.rotation.apply3(&(v * self.scale)) + self.translation)
    }

    fn as_mat3<M: SqMatrix3<F>>(&self) -> M {
        let mut m = M::default();
        self.rotation.set_rotation3(&mut m);
        m *= self.scale;
        m
    }

    fn as_mat4<M: SqMatrix4<F>>(&self) -> M {
        let mut m = M::default();
        self.rotation.set_rotation4(&mut m);
        for (i, c) in m.iter_mut().take(12).enumerate() {
            if (i % 4) == 3 {
                *c = self.translation[i / 4];
            } else {
                *c = *c * self.scale;
            }
        }
        m
    }
}

impl<F> Transform<F> for FArray2<F, 4, 16>
where
    F: Float,
    QArray<F>: Quaternion<F>,
    FArray2<F, 4, 16>: SqMatrix4<F>,
    FArray<F, 3>: Vector3<F>,
{
    const UNIFORM_SCALING: bool = false;
    type Vector = FArray<F, 3>;
    type Quat = QArray<F>;
    fn of_trs<A: AsRef<[F; 3]>>(t: A, r: Self::Quat, s: A) -> Option<Self> {
        let t = t.as_ref();
        let mut m = Self::default();
        r.set_rotation4(&mut m);
        m.scale_by(s);
        m[3] = t[0];
        m[7] = t[1];
        m[11] = t[2];
        Some(m)
    }

    fn of_trsu<A: AsRef<[F; 3]>>(t: A, r: Self::Quat, s: F) -> Self {
        let t = t.as_ref();
        let mut m = Self::default();
        r.set_rotation4(&mut m);
        m.scale_uniform_by(s);
        m[3] = t[0];
        m[7] = t[1];
        m[11] = t[2];
        m
    }

    fn is_uniform_scale(&self) -> bool {
        false
    }

    fn scale(&self) -> Option<Self::Vector> {
        None
    }

    fn uniform_scale(&self) -> Option<F> {
        None
    }

    fn translation(&self) -> Self::Vector {
        [self[3], self[7], self[11]].into()
    }

    fn rotation(&self) -> Option<Self::Quat> {
        None
    }

    fn set_identity(&mut self) {
        *self = Self::identity();
    }

    fn set_scale<A: AsRef<[F; 3]>>(&mut self, _scale: A) -> bool {
        false
    }

    fn set_uniform_scale(&mut self, scale: F) {
        let s = self.determinant();
        if s.abs() > F::epsilon() {
            self.scale_uniform_by(scale / s);
        }
    }

    fn set_translation<A: AsRef<[F; 3]>>(&mut self, translation: A) {
        let t = translation.as_ref();
        self[3] = t[0];
        self[7] = t[1];
        self[11] = t[2];
    }

    fn set_rotation(&mut self, rotation: Self::Quat) {
        let s = self.determinant();
        if s.abs() > F::epsilon() {
            let mut m: FArray2<F, 4, 16> = [F::ZERO; 16].into();
            rotation.set_rotation4(&mut m);
            m.scale_uniform_by(s);
            m[3] = self[3];
            m[7] = self[7];
            m[11] = self[11];
        }
    }

    fn scale_uniform_by(&mut self, scale: F) {
        for c in self.iter_mut().take(12) {
            *c = *c * scale;
        }
    }

    fn scale_by<A: AsRef<[F; 3]>>(&mut self, scale: A) -> bool {
        for (i, c) in self.iter_mut().take(12).enumerate() {
            *c = *c * scale.as_ref()[i / 4];
        }
        true
    }

    fn translate_by<A: AsRef<[F; 3]>>(&mut self, translation: A, scale: F) {
        let translation = translation.as_ref();
        for i in 0..3 {
            self[i * 4 + 3] = self[i * 4 + 3] + translation[i] * scale;
        }
    }

    fn rotate_by(&mut self, quaternion: &Self::Quat) {
        let mut m: Self = [F::ZERO; 16].into();
        quaternion.set_rotation4(&mut m);
        *self = *self * m;
    }

    fn transform_by<T: Transform<F, Quat = Self::Quat>>(&mut self, transformer: &T) -> bool {
        let m = transformer.as_mat4::<Self>();
        *self = *self * m;
        true
    }

    fn inverse(&self) -> Option<Self> {
        Some(<Self as SqMatrix<_, _, _>>::inverse(self))
    }

    fn invert(&mut self) -> bool {
        *self = <Self as SqMatrix<_, _, _>>::inverse(self);
        true
    }

    fn apply3_arr(&self, other: &[F; 3]) -> [F; 3] {
        let v = [other[0], other[1], other[2], F::ONE];
        let v = self.transform_arr(&v);
        [v[0], v[1], v[2]]
    }

    fn as_mat3<M: SqMatrix3<F>>(&self) -> M {
        let mut m = M::default();
        for (i, c) in m.iter_mut().enumerate() {
            *c = self[i + i / 3];
        }
        m
    }

    fn as_mat4<M: SqMatrix4<F>>(&self) -> M {
        self.as_ref().into()
    }
}
