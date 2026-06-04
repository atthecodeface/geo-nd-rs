use geo_nd::{
    FArray, FArray2, Float, QArray, Quaternion, SqMatrix, SqMatrix3, SqMatrix4, Transform,
    Transformation, Vector, Vector3,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;
type Trans<F> = Transformation<F, FArray<F, 3>, QArray<F>>;
type Vec3<F> = FArray<F, 3>;

fn test_transform<F>(t: &Trans<F>, v: &[F; 3]) -> TestResult
where
    F: Float,
    FArray<F, 3>: Vector3<F>,
    QArray<F>: Quaternion<F>,
{
    let vt: Vec3<F> = t.apply3_arr(v).into();
    let Some(scale) = t.scale() else {
        return Ok(());
    };
    let Some(rotation) = t.rotation() else {
        return Ok(());
    };
    let vt_by_hand: Vec3<F> = [scale[0] * v[0], scale[1] * v[1], scale[2] * v[2]].into();
    let translation: Vec3<F> = t.translation();
    let vt_by_hand = vt_by_hand.apply_q3(&rotation) + translation;
    let dsq = vt.distance_sq_arr(&vt_by_hand);
    if dsq >= F::from_f32(1.0E-6_f32).unwrap() {
        eprintln!(
            "Mismatch in vectors  - {} does not match by hand of {}, for {:?} transformed by {}",
            vt, vt_by_hand, v, t
        );
        assert!(false);
    }
    Ok(())
}

fn test_transforms_pair<F>(t0: &Trans<F>, t1: &Trans<F>, v: &[F; 3]) -> TestResult
where
    F: Float,
    FArray<F, 3>: Vector3<F>,
    QArray<F>: Quaternion<F>,
    FArray2<F, 3, 9>: SqMatrix3<F>,
    FArray2<F, 4, 16>: SqMatrix4<F>,
{
    test_transform(t0, v)?;
    if t1.uniform_scale().is_some() {
        test_transform(t1, v)?;
        let mut t1_of_t0 = t0.clone();
        assert!(
            t1_of_t0.transform_by(t1),
            "Must be able to transform by a uniform scale t1"
        );
        let vt0: Vec3<F> = t0.apply3_arr(v).into();
        let vt1_of_0_a = t1.apply3(vt0);
        let vt1_of_0_b: Vec3<F> = t1_of_t0.apply3_arr(v).into();
        let dsq = vt1_of_0_a.distance_sq_arr(&vt1_of_0_b);
        if dsq >= F::from_f32(1.0E-6_f32).unwrap() {
            eprintln!(
                "Mismatch in vectors  - {} does not match by hand of {}, for {:?} transformed by {} then {}",
                vt1_of_0_a, vt1_of_0_b, v, t0, t1
            );
            assert!(false);
        }
        test_transform(&t1_of_t0, v)?;

        let mut arr3: FArray2<F, 3, 9> = FArray2::identity();
        t1_of_t0.set_mat3(&mut arr3);
        let vt1_of_0_c: Vec3<F> = arr3.transform_arr(v).into();
        let vt1_of_0_c = vt1_of_0_c + t1_of_t0.translation();
        let dsq = vt1_of_0_a.distance_sq_arr(&vt1_of_0_c);
        if dsq >= F::from_f32(1.0E-6_f32).unwrap() {
            eprintln!(
                "Mismatch in vectors  - {} does not match by array of {:?}, for {:?} transformed by {} / array {:?}",
                vt1_of_0_a, vt1_of_0_c, v, t1_of_t0, arr3
            );
            assert!(false);
        }

        let mut arr4: FArray2<F, 4, 16> = FArray2::identity();
        t1_of_t0.set_mat4(&mut arr4);
        let v = arr4.transform_arr(&[v[0], v[1], v[2], F::ONE]);
        assert_eq!(v[3], F::ONE, "Translated vector must have [3] of 1.0");
        let vt1_of_0_d: Vec3<F> = [v[0], v[1], v[2]].into();
        let dsq = vt1_of_0_a.distance_sq_arr(&vt1_of_0_d);
        if dsq >= F::from_f32(1.0E-6_f32).unwrap() {
            eprintln!(
                "Mismatch in vectors  - {} does not match by array of {:?}, for {:?} transformed by {} / array {:?}",
                vt1_of_0_a, vt1_of_0_d, v, t1_of_t0, arr4
            );
            assert!(false);
        }
    }
    Ok(())
}

fn transform_vectors<F>() -> TestResult
where
    F: Float,
    FArray<F, 3>: Vector3<F>,
    QArray<F>: Quaternion<F>,
    FArray2<F, 3, 9>: SqMatrix3<F>,
    FArray2<F, 4, 16>: SqMatrix4<F>,
{
    let mut t0: Trans<F> = Trans::default();
    let mut t1: Trans<F> = Trans::default();
    let half = F::from_f32(0.5).unwrap();
    let three = F::from_f32(3.0).unwrap();
    let mut v: Vec3<F> = [F::ONE, F::ONE + F::ONE, F::ONE + F::ONE + F::ONE].into();
    v *= half;
    test_transforms_pair(&t0, &t1, &[F::ZERO; 3])?;
    test_transforms_pair(&t0, &t1, &v)?;

    t0.translate_by(&v, F::ONE);
    test_transforms_pair(&t0, &t1, &[F::ZERO; 3])?;
    test_transforms_pair(&t0, &t1, &v)?;

    t1.translate_by(&v, half);
    test_transforms_pair(&t0, &t1, &[F::ZERO; 3])?;
    test_transforms_pair(&t0, &t1, &v)?;

    t0.scale_uniform_by(three);
    test_transforms_pair(&t0, &t1, &v)?;

    t0.set_scale(t0.translation());
    test_transforms_pair(&t0, &t1, &v)?;

    t1.scale_uniform_by(half / three);
    test_transforms_pair(&t0, &t1, &v)?;

    let q = QArray::of_axis_angle(&t0.translation(), half);
    t0.rotate_by(&q);
    test_transforms_pair(&t0, &t1, &v)?;

    t1.transform_by(&t0);
    test_transforms_pair(&t0, &t1, &v)?;

    let q = QArray::of_axis_angle(&t1.translation(), three);
    t1.rotate_by(&q);
    test_transforms_pair(&t0, &t1, &v)?;

    eprintln!("Passed all tests, ended with t0:{t0}, t1:{t1}, v:{v}");

    Ok(())
}

#[test]
fn test_transform_vectors() -> TestResult {
    transform_vectors::<f32>()?;
    transform_vectors::<f64>()?;
    Ok(())
}
