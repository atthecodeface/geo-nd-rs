use crate::{FArray, Float, Quaternion, SqMatrix3, SqMatrix4, Transform, Vector3, Vector4, vector};

/// A transformation that is a translation . rotation . scaling
/// (i.e. it applies a scaling to an object, then the rotation then
/// translates it)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transformation<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
{
    /// Quaternion of the rotation
    rotation: Q,
    /// Translation
    translation: FArray<F, 3>,
    /// Scaling, for  all three axes
    scale: FArray<F, 3>,
}

impl<F, Q> std::default::Default for Transformation<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
{
    fn default() -> Self {
        Self {
            rotation: Q::default(),
            translation: FArray::<F, 3>::default(),
            scale: [F::ONE; 3].into(),
        }
    }
}

impl<F, Q> std::fmt::Display for Transformation<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
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

impl<F, Q> std::ops::Mul<FArray<F, 4>> for Transformation<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
    FArray<F, 4>: Vector4<F>,
{
    type Output = FArray<F, 4>;
    fn mul(self, v: FArray<F, 4>) -> Self::Output {
        let v_sc: FArray<F, 3> = [
            self.scale[0] * v[0],
            self.scale[1] * v[1],
            self.scale[2] * v[2],
        ]
        .into();
        let r = self.rotation.apply3(&v_sc) + (self.translation * v[3]);
        [r[0], r[1], r[2], v[3]].into()
    }
}

impl<F, Q> std::ops::Mul<F> for Transformation<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
{
    type Output = Self;
    fn mul(mut self, f: F) -> Self::Output {
        self.scale *= f;
        self.translation *= f;
        self
    }
}

impl<F, Q> std::ops::MulAssign<F> for Transformation<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
{
    fn mul_assign(&mut self, f: F) {
        *self = *self * f;
    }
}

impl<F, Q> std::ops::Div<F> for Transformation<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
{
    type Output = Self;
    fn div(mut self, f: F) -> Self::Output {
        self.scale /= f;
        self.translation /= f;
        self
    }
}

impl<F, Q> std::ops::DivAssign<F> for Transformation<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
{
    fn div_assign(&mut self, f: F) {
        *self = *self / f;
    }
}

impl<F, Q> Transform<F> for Transformation<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
    FArray<F, 4>: Vector4<F>,
{
    const UNIFORM_SCALING: bool = false;
    type Vec3 = FArray<F, 3>;
    type Vec4 = FArray<F, 4>;
    type Quat = Q;
    fn of_trs<A: AsRef<[F; 3]>>(t: A, r: Q, s: A) -> Option<Self> {
        Some(Self {
            rotation: r,
            translation: t.as_ref().into(),
            scale: s.as_ref().into(),
        })
    }

    fn of_trsu<A: AsRef<[F; 3]>>(t: A, r: Q, s: F) -> Self {
        Self {
            rotation: r,
            translation: t.as_ref().into(),
            scale: [s; 3].into(),
        }
    }

    fn is_uniform_scale(&self) -> bool {
        self.scale[0] == self.scale[1] && self.scale[0] == self.scale[2]
    }

    fn scale(&self) -> Option<Self::Vec3> {
        Some(self.scale)
    }

    fn uniform_scale(&self) -> Option<F> {
        if self.is_uniform_scale() {
            Some(self.scale[0])
        } else {
            None
        }
    }

    fn translation(&self) -> Self::Vec3 {
        self.translation
    }

    fn rotation(&self) -> Option<Q> {
        Some(self.rotation)
    }

    fn set_identity(&mut self) {
        self.rotation.set_identity();
        self.translation = FArray::default();
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
        let translation: Self::Vec3 = translation.as_ref().into();
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
        let v: Self::Vec3 = vector::comp_mult(*self.scale, other).into();
        *(self.rotation.apply3(&v) + self.translation)
    }

    fn apply4_arr(&self, other: &[F; 4]) -> [F; 4] {
        let v: Self::Vec3 = vector::comp_mult(*self.scale, &[other[0], other[1], other[2]]).into();
        let r = self.rotation.apply3(&v) + (self.translation * other[3]);
        [r[0], r[1], r[2], other[3]]
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

#[cfg(feature = "serde")]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename = "Transformation")]
struct X<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F> + serde::de::DeserializeOwned + serde::Serialize,
{
    rotation: Q,
    /// Translation
    translation: FArray<F, 3>,
    /// Scaling, for  all three axes
    scale: FArray<F, 3>,
}

#[cfg(feature = "serde")]
impl<F, Q> serde::Serialize for Transformation<F, Q>
where
    F: Float + for <'de> serde::de::Deserialize<'de> + serde::Serialize,
    Q: Quaternion<F> + for <'de> serde::de::Deserialize<'de> + serde::Serialize,
    FArray<F, 3>: Vector3<F> + serde::de::DeserializeOwned + serde::Serialize,
    FArray<F, 4>: Vector4<F> + serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (X {
            rotation: self.rotation,
            translation: self.translation,
            scale: self.scale,
        })
        .serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, F, Q> serde::Deserialize<'de> for Transformation<F, Q>
where
    F: Float + serde::de::Deserialize<'de> + serde::Serialize,
    Q: Quaternion<F> + serde::de::Deserialize<'de> + serde::Serialize,
    FArray<F, 3>: Vector3<F> + serde::de::DeserializeOwned + serde::Serialize,
    FArray<F, 4>: Vector4<F>,
{
    fn deserialize<DE>(deserializer: DE) -> Result<Self, DE::Error>
    where
        DE: serde::Deserializer<'de>,
    {
        let x = X::<F, Q>::deserialize(deserializer)?;
        Ok(Self {
            rotation: x.rotation,
            translation: x.translation,
            scale: x.scale,
        })
    }
}
