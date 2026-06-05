use crate::{
    FArray, FArray2, Float, QArray, Quaternion, SqMatrix, SqMatrix3, SqMatrix4, Transform, Vector3,
    Vector4,
};

impl<F> Transform<F> for FArray2<F, 4, 16>
where
    F: Float,
    QArray<F>: Quaternion<F>,
    FArray2<F, 4, 16>: SqMatrix4<F> + std::ops::Mul<FArray<F, 4>, Output = FArray<F, 4>>,
    FArray<F, 3>: Vector3<F>,
    FArray<F, 4>: Vector4<F>,
{
    const UNIFORM_SCALING: bool = false;
    type Vec3 = FArray<F, 3>;
    type Vec4 = FArray<F, 4>;
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

    fn scale(&self) -> Option<Self::Vec3> {
        None
    }

    fn uniform_scale(&self) -> Option<F> {
        None
    }

    fn translation(&self) -> Self::Vec3 {
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

    fn apply4_arr(&self, other: &[F; 4]) -> [F; 4] {
        self.transform_arr(other)
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
