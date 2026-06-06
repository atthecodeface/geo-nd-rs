use crate::{FArray, Float, Quaternion, SqMatrix3, SqMatrix4, Transform, Vector3, Vector4};

/// A transformation that is a translation . rotation . scaling
/// (i.e. it applies a scaling to an object, then the rotation then
/// translates it)
///
/// This supports only a uniform scaling
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransformationUniform<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
{
    /// Quaternion of the rotation
    rotation: Q,
    /// Translation
    translation: FArray<F, 3>,
    /// Scaling, for all three axes
    scale: F,
}

impl<F, Q> std::default::Default for TransformationUniform<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
    FArray<F, 4>: Vector4<F>,
{
    fn default() -> Self {
        Self {
            rotation: Q::default(),
            translation: FArray::default(),
            scale: F::ONE,
        }
    }
}

impl<F, Q> std::fmt::Display for TransformationUniform<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
    FArray<F, 4>: Vector4<F>,
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

impl<F, Q> std::ops::Mul<FArray<F, 4>> for TransformationUniform<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
    FArray<F, 4>: Vector4<F>,
{
    type Output = FArray<F, 4>;
    fn mul(self, v: FArray<F, 4>) -> Self::Output {
        let v_sc: FArray<F, 3> = [self.scale * v[0], self.scale * v[1], self.scale * v[2]].into();
        let r = self.rotation.apply3(&v_sc) + (self.translation * v[3]);
        [r[0], r[1], r[2], v[3]].into()
    }
}

impl<F, Q> std::ops::Mul<F> for TransformationUniform<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
{
    type Output = Self;
    fn mul(mut self, f: F) -> Self::Output {
        self.scale = self.scale * f;
        self.translation *= f;
        self
    }
}

impl<F, Q> std::ops::MulAssign<F> for TransformationUniform<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
{
    fn mul_assign(&mut self, f: F) {
        *self = *self * f;
    }
}

impl<F, Q> std::ops::Div<F> for TransformationUniform<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
{
    type Output = Self;
    fn div(mut self, f: F) -> Self::Output {
        self.scale = self.scale / f;
        self.translation /= f;
        self
    }
}

impl<F, Q> std::ops::DivAssign<F> for TransformationUniform<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
{
    fn div_assign(&mut self, f: F) {
        *self = *self / f;
    }
}

impl<F, Q> Transform<F> for TransformationUniform<F, Q>
where
    F: Float,
    Q: Quaternion<F>,
    FArray<F, 3>: Vector3<F>,
    FArray<F, 4>: Vector4<F>,
{
    const UNIFORM_SCALING: bool = true;
    type Vec3 = FArray<F, 3>;
    type Vec4 = FArray<F, 4>;
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

    fn scale(&self) -> Option<Self::Vec3> {
        Some([self.scale; 3].into())
    }

    fn uniform_scale(&self) -> Option<F> {
        Some(self.scale)
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
        self.scale = self.scale * scale;
        self.translation *= scale;
        self.rotation = rotation * self.rotation;
        let translation: [F; 3] = transformer.translation().into();
        self.translation = rotation.apply3(&self.translation) + &translation;
        true
    }

    fn inverse_transform(&self) -> Option<Self> {
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

    fn apply3_arr(&self, other: &[F; 3]) -> [F; 3] {
        let v: Self::Vec3 = other.into();
        *(self.rotation.apply3(&(v * self.scale)) + self.translation)
    }

    fn apply4_arr(&self, other: &[F; 4]) -> [F; 4] {
        let v: Self::Vec3 = [other[0], other[1], other[2]].into();
        let r = self.rotation.apply3(&v) * self.scale + (self.translation * other[3]);
        [r[0], r[1], r[2], other[3]]
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

#[cfg(feature = "serde")]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename = "TransformationUniform")]
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
    scale: F,
}

#[cfg(feature = "serde")]
impl<F, Q> serde::Serialize for TransformationUniform<F, Q>
where
    F: Float + for<'de> serde::de::Deserialize<'de> + serde::Serialize,
    Q: Quaternion<F> + for<'de> serde::de::Deserialize<'de> + serde::Serialize,
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
impl<'de, F, Q> serde::Deserialize<'de> for TransformationUniform<F, Q>
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
