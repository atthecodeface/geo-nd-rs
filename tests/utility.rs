#![allow(dead_code)]

use geo_nd::{Quaternion, Vector};

//a Utility tests for vector3
pub type Vector2 = geo_nd::FArray<f32, 2>;
pub type Vector3 = geo_nd::FArray<f32, 3>;
pub type Vector4 = geo_nd::FArray<f32, 4>;
pub type Quat = geo_nd::QArray<f32>;

//a Constants
pub const ZERO_2: [f32; 2] = [0.0; 2];
pub const TF_2: [f32; 2] = [3.0, 4.0];
pub const ZERO_3: [f32; 3] = [0.0; 3];
pub const ONE_3: [f32; 3] = [1.0; 3];
pub const TFT_3: [f32; 3] = [3.0, 4.0, 12.0];
pub const ZERO_4: [f32; 4] = [0.0; 4];
pub const ONE_4: [f32; 4] = [1.0; 4];
pub const TFTO_4: [f32; 4] = [3.0, 4.0, 12.0, 1.0];

pub const REF_ZERO_2: &[f32; 2] = &[0.0; 2];
pub const REF_ZERO_3: &[f32; 3] = &[0.0; 3];
pub const REF_ZERO_4: &[f32; 4] = &[0.0; 4];
pub const REF_ONE_4: &[f32; 4] = &[1.0; 4];

//a Functions
fn does_as_ref2<A: AsRef<[f32; 2]>>(t: A) -> (f32, f32) {
    (t.as_ref()[0], t.as_ref()[1])
}

//a Vector Tests
//f utility_convert
#[test]
fn utility_convert() -> Result<(), String> {
    // From [f32; D]
    let mut pt2: Vector2;
    let mut pt3: Vector3;
    let mut pt4: Vector4;
    let mut res_pt2: Result<Vector2, _>;

    pt2 = ZERO_2.into();
    assert_eq!(pt2.length(), 0.0);

    pt3 = ZERO_3.into();
    assert_eq!(pt3.length(), 0.0);

    pt3 = TFT_3.into();
    assert_eq!(pt3.length(), 13.0);

    pt4 = ZERO_4.into();
    assert_eq!(pt4.length(), 0.0);

    pt4 = ONE_4.into();
    assert_eq!(pt4.length_sq(), 4.0);

    // From &[f32; D]
    pt2 = REF_ZERO_2.into();
    assert_eq!(pt2.length(), 0.0);

    pt3 = REF_ZERO_3.into();
    assert_eq!(pt3.length(), 0.0);

    pt4 = REF_ZERO_4.into();
    assert_eq!(pt4.length(), 0.0);

    pt4 = REF_ONE_4.into();
    assert_eq!(pt4.length_sq(), 4.0);

    // TryFrom &[f3]
    res_pt2 = REF_ZERO_2.as_slice().try_into();
    assert_eq!(res_pt2.unwrap().length(), 0.0);
    res_pt2 = REF_ZERO_3.as_slice().try_into();
    assert!(res_pt2.is_err());

    pt3 = REF_ZERO_3.as_slice().try_into().unwrap();
    assert_eq!(pt3.length(), 0.0);

    pt4 = REF_ZERO_4.as_slice().try_into().unwrap();
    assert_eq!(pt4.length(), 0.0);

    pt4 = REF_ONE_4.as_slice().try_into().unwrap();
    assert_eq!(pt4.length_sq(), 4.0);

    pt2 = TF_2.into();
    assert_eq!(does_as_ref2(pt2).0, 3.0);
    assert_eq!(does_as_ref2(pt2).1, 4.0);

    let v: Vec<_> = TFT_3.into();
    pt3 = <_>::try_from(v).unwrap();
    let _ = pt3;

    Ok(())
}

//f utility_index
#[test]
fn utility_indext() -> Result<(), String> {
    // From [f32; D]
    let mut pt3: Vector3;

    pt3 = TFT_3.into();
    assert_eq!(pt3[0], 3.0);
    assert_eq!(pt3[1], 4.0);
    assert_eq!(pt3[2], 12.0);

    pt3[0] = 0.0;
    pt3[1] = 0.0;
    pt3[2] = 0.0;

    Ok(())
}

//f utility_add
#[test]
fn utility_add() -> Result<(), String> {
    // From [f32; D]
    let pt_a: Vector3;
    let pt_b: Vector3;

    // From &[f32; 3]
    pt_a = ZERO_3.into();
    pt_b = TFT_3.into();

    let n = pt_a + pt_b;

    assert_eq!(n, pt_b);

    let n = n + &[1.0_f32, 2., 3.];
    assert_eq!(n[0], 4.0);

    let n = n + &pt_a;
    assert_eq!(n[0], 4.0);

    let mut n = &pt_b + &pt_b;
    assert_eq!(n[0], 6.0);

    let mut n_mut = &mut n;

    n_mut += &pt_b;
    assert_eq!(n[0], 9.0);

    Ok(())
}

//f utility_mul
#[test]
fn utility_mul() -> Result<(), String> {
    // From [f32; D]
    let pt_b: Vector3;

    // From &[f32; 3]
    pt_b = TFT_3.into();

    let n = pt_b * 2.0;
    assert_eq!(n[0], 6.0);

    let n = pt_b * (&3.0);
    assert_eq!(n[0], 9.0);

    Ok(())
}

//f utility_neg
#[test]
fn utility_neg() -> Result<(), String> {
    // From [f32; D]
    let pt_a: Vector3;
    let pt_b: Vector3;

    // From &[f32; 3]
    pt_a = ZERO_3.into();
    pt_b = TFT_3.into();

    let n = -pt_b;
    assert_eq!(n[0], -3.0);

    let n = -&pt_a;
    assert_eq!(n[0], 0.0);

    Ok(())
}

//f utility_vec3
#[test]
fn utility_vec3() -> Result<(), String> {
    // From [f32; D]
    let pt_a: Vector3;
    let pt_b: Vector3;

    // From &[f32; 3]
    pt_a = TFT_3.into();
    pt_b = TFT_3.into();

    let n = pt_a.cross_product(&pt_b);
    assert_eq!(n[0], 0.0);
    let n = pt_a.cross_product(&pt_b);
    assert_eq!(n[0], 0.0);

    // let n = pt_a.cross_product([1., 2., 3.]);
    // let n = pt_a.cross_product(&[1., 2., 3.]);

    Ok(())
}

//f utility_quat_vec3
#[test]
fn utility_quat_vec3() -> Result<(), String> {
    // From [f32; D]
    let pt_a: Vector3;

    // From &[f32; 3]
    pt_a = TFT_3.into();

    assert_eq!(pt_a[0], 3.0);
    assert_eq!(pt_a[1], 4.0);
    assert_eq!(pt_a[2], 12.0);

    let rot_x90 = Quat::default().rotate_x(std::f32::consts::PI / 2.0);
    let mut p = rot_x90 * &pt_a;
    eprintln!("{p}");
    assert!((p[0] - 3.0).abs() < 1E-4);
    assert!((p[1] - -12.0).abs() < 1E-4);
    assert!((p[2] - 4.0).abs() < 1E-4);

    p = (&rot_x90) * &p;
    eprintln!("{p}");
    assert!((p[0] - 3.0).abs() < 1E-4);
    assert!((p[1] - -4.0).abs() < 1E-4);
    assert!((p[2] - -12.0).abs() < 1E-4);

    p = rot_x90 * &p;
    eprintln!("{p}");
    assert!((p[0] - 3.0).abs() < 1E-4);
    assert!((p[1] - 12.0).abs() < 1E-4);
    assert!((p[2] - -4.0).abs() < 1E-4);

    p = (&rot_x90) * &p;
    eprintln!("{p}");
    assert!((p[0] - 3.0).abs() < 1E-4);
    assert!((p[1] - 4.0).abs() < 1E-4);
    assert!((p[2] - 12.0).abs() < 1E-4);

    Ok(())
}
