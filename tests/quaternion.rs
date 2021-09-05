//a Imports
use geo_nd::quat;
use geo_nd::{FArray, Float, QArray, Quaternion, Vector, Geometry3D};
use std::marker::PhantomData;

//a Test type
//ti Banana
// This type is required so that implementations of the associated types of V (e.g. Vec2/3/4) can be used within
type Vec3 = <f32 as Geometry3D<f32>>::Vec3;
type Vec4 = <f32 as Geometry3D<f32>>::Vec4;
type Mat3 = <f32 as Geometry3D<f32>>::Mat3;
type Mat4 = <f32 as Geometry3D<f32>>::Mat4;
type Quat = <f32 as Geometry3D<f32>>::Quat;

//ii Banana
/// Implementation of the test type Banana, exposing the associated
/// types for use within the implementation
///
/// Note that V2, V3, V4 will be implicitly taken from the Vector3D<f32> trait implementation of V
///
/// The impl here must specify that the meet the Vector<f32,N> trait
/// as the compiler will otherwise assume they can be specified and
/// therefore may not have such a trait, even though as associated
/// types of V they must.
fn quat_eq(q: &Quat, q2:&Quat) -> bool {
    let d_sub = *q2 - *q;
    let d_add = *q2 + *q;
    if d_sub.length_sq() < f32::frac(1, 1000) {
        true
    } else if d_add.length_sq() < f32::frac(1, 1000) {
        true
    } else {
        dbg!(q, q2, d_sub, d_sub.length_sq(), d_add, d_add.length_sq());
        false
    }
}
fn quat_eq_rijk(q: &Quat, rijk: (f32, f32, f32, f32)) -> bool {
    let eq = Quat::of_rijk(rijk.0, rijk.1, rijk.2, rijk.3);
    quat_eq(q, &eq)
}
#[test]
fn test() {
    let x = FArray::<f32, 3>::from_array([1., 0., 0.]);
    let y = FArray::<f32, 3>::from_array([0., 1., 0.]);
    let z = FArray::<f32, 3>::from_array([0., 0., 1.]);

    let mut xy = x + y;
    let mut yz = z + y;
    let mut xz = x + z;
    let mut xyz = x + y + z;
    xy.normalize();
    yz.normalize();
    xz.normalize();
    xyz.normalize();

    let ra = std::f32::consts::PI / 2.;
    let rsqrt2 = (0.5_f32).sqrt();

    let q = Quat::unit();
    assert_eq!(q.length(), 1.);
    assert_eq!(q.length_sq(), 1.);
    assert_eq!(quat::as_rijk(q.as_ref()), (1., 0., 0., 0.));
    assert_eq!(q.as_rijk(), (1., 0., 0., 0.));
    assert_eq!(q.conjugate().as_rijk(), (1., 0., 0., 0.));

    let q = Quat::of_rijk(0., 1., 0., 0.);
    assert_eq!(q.length(), 1.);
    assert_eq!(q.length_sq(), 1.);
    assert_eq!(quat::as_rijk(q.as_ref()), (0., 1., 0., 0.));
    assert_eq!(q.as_rijk(), (0., 1., 0., 0.));
    assert_eq!(q.conjugate().as_rijk(), (0., -1., 0., 0.));

    let q = Quat::of_rijk(0., 0., 1., 0.);
    assert_eq!(q.length(), 1.);
    assert_eq!(q.length_sq(), 1.);
    assert_eq!(q.as_rijk(), (0., 0., 1., 0.));
    assert_eq!(q.conjugate().as_rijk(), (0., 0., -1., 0.));

    let q = Quat::of_rijk(0., 0., 0., 1.);
    assert_eq!(q.length(), 1.);
    assert_eq!(q.length_sq(), 1.);
    assert_eq!(q.as_rijk(), (0., 0., 0., 1.));
    assert_eq!(q.conjugate().as_rijk(), (0., 0., 0., -1.));

    let q = Quat::of_rijk(1., 1., 1., 1.);
    assert_eq!(q.length(), 2.);
    assert_eq!(q.length_sq(), 4.);
    assert_eq!(q.as_rijk(), (1., 1., 1., 1.));
    assert_eq!(q.conjugate().as_rijk(), (1., -1., -1., -1.));

    assert_eq!(Quat::of_axis_angle(&x, 0.).as_rijk(), (1., 0., 0., 0.));
    assert_eq!(Quat::of_axis_angle(&y, 0.).as_rijk(), (1., 0., 0., 0.));
    assert_eq!(Quat::of_axis_angle(&z, 0.).as_rijk(), (1., 0., 0., 0.));
    assert_eq!(Quat::of_axis_angle(&xy, 0.).as_rijk(), (1., 0., 0., 0.));
    assert_eq!(Quat::of_axis_angle(&yz, 0.).as_rijk(), (1., 0., 0., 0.));
    assert_eq!(Quat::of_axis_angle(&xz, 0.).as_rijk(), (1., 0., 0., 0.));

    assert!(quat_eq_rijk(
        &Quat::of_axis_angle(&x, ra),
        (rsqrt2, rsqrt2, 0., 0.)
    ));
    assert!(quat_eq_rijk(
        &Quat::of_axis_angle(&x, 2. * ra),
        (0., 1., 0., 0.)
    ));

    assert!(quat_eq_rijk(
        &Quat::of_axis_angle(&y, ra),
        (rsqrt2, 0., rsqrt2, 0.)
    ));
    assert!(quat_eq_rijk(
        &Quat::of_axis_angle(&y, 2. * ra),
        (0., 0., 1., 0.)
    ));

    assert!(quat_eq_rijk(
        &Quat::of_axis_angle(&z, ra),
        (rsqrt2, 0., 0., rsqrt2)
    ));
    assert!(quat_eq_rijk(
        &Quat::of_axis_angle(&z, 2. * ra),
        (0., 0., 0., 1.)
    ));

    assert!(quat_eq_rijk(
        &Quat::of_axis_angle(&xy, ra),
        (rsqrt2, 0.5, 0.5, 0.)
    ));
    assert!(quat_eq_rijk(
        &Quat::of_axis_angle(&yz, ra),
        (rsqrt2, 0., 0.5, 0.5)
    ));
    assert!(quat_eq_rijk(
        &Quat::of_axis_angle(&xz, ra),
        (rsqrt2, 0.5, 0., 0.5)
    ));

    let x90 = Quat::of_axis_angle(&x, ra);
    let y90 = Quat::of_axis_angle(&y, ra);
    let z90 = Quat::of_axis_angle(&z, ra);

    let t1 = x90 * y90;
    let t2 = z90 * x90;
    let t3 = y90 * z90;
    assert!(quat_eq_rijk(&t1, (0.5, 0.5, 0.5, 0.5)));
    assert!(quat_eq_rijk(&t2, (0.5, 0.5, 0.5, 0.5)));
    assert!(quat_eq_rijk(&t3, (0.5, 0.5, 0.5, 0.5)));

    let t1 = x90 / y90;
    let t2 = z90 / x90;
    let t3 = y90 / z90;
    assert!(quat_eq_rijk(&t1, (0.5, 0.5, -0.5, -0.5)));
    assert!(quat_eq_rijk(&t2, (0.5, -0.5, -0.5, 0.5)));
    assert!(quat_eq_rijk(&t3, (0.5, -0.5, 0.5, -0.5)));

    let t1 = y90 * x90;
    let t2 = x90 * z90;
    let t3 = z90 * y90;
    assert!(quat_eq_rijk(&t1, (0.5, 0.5, 0.5, -0.5)));
    assert!(quat_eq_rijk(&t2, (0.5, 0.5, -0.5, 0.5)));
    assert!(quat_eq_rijk(&t3, (0.5, -0.5, 0.5, 0.5)));

    let t1 = y90 / x90;
    let t2 = x90 / z90;
    let t3 = z90 / y90;
    assert!(quat_eq_rijk(&t1, (0.5, -0.5, 0.5, 0.5)));
    assert!(quat_eq_rijk(&t2, (0.5, 0.5, 0.5, -0.5)));
    assert!(quat_eq_rijk(&t3, (0.5, 0.5, -0.5, 0.5)));

    for axis in [&x, &y, &z, &xy, &yz, &xz, &xyz] {
        let t = QArray::of_axis_angle(axis, ra / 3.); // 12 of these makes 360
        let t2 = t * t; // 6 of these
        let t4 = t2 * t2; // 3 of these
        let t12 = t4 * t4 * t4;
        assert!(quat_eq_rijk(&t12, (1., 0., 0., 0.)));
    }
}
#[test]
fn test_matrix() {
    let ra = std::f32::consts::PI / 2.;
    let rsqrt2 = (0.5_f32).sqrt();

    let mut q = Quat::of_rijk(1., 0., 0., 0.);
    let mut m = Mat3::default();
    q.set_rotation3(&mut m);
    assert_eq!(<Mat3 as AsRef<[f32; 9]>>::as_ref(&m), &[1.,0.,0., 0.,1.,0., 0.,0.,1.]);
    let q2 = Quat::of_rotation3(&m);
    assert!(quat_eq(&q, &q2));

    let mut q = Quat::of_rijk(0., 1., 0., 0.);
    let mut m = Mat3::default();
    q.set_rotation3(&mut m);
    assert_eq!(<Mat3 as AsRef<[f32; 9]>>::as_ref(&m), &[1.,0.,0., 0.,-1.,0., 0.,0.,-1.]);
    let q2 = Quat::of_rotation3(&m);
    assert!(quat_eq(&q, &q2));

    let mut q = Quat::of_rijk(0., 0., 1., 0.);
    let mut m = Mat3::default();
    q.set_rotation3(&mut m);
    assert_eq!(<Mat3 as AsRef<[f32; 9]>>::as_ref(&m), &[-1.,0.,0., 0.,1.,0., 0.,0.,-1.]);
    let q2 = Quat::of_rotation3(&m);
    assert!(quat_eq(&q, &q2));

    let mut q = Quat::of_rijk(0., 0., 0., 1.);
    let mut m = Mat3::default();
    q.set_rotation3(&mut m);
    assert_eq!(<Mat3 as AsRef<[f32; 9]>>::as_ref(&m), &[-1.,0.,0., 0.,-1.,0., 0.,0.,1.]);
    let q2 = Quat::of_rotation3(&m);
    assert!(quat_eq(&q, &q2));

    let mut q = Quat::of_rijk(rsqrt2, rsqrt2, 0., 0.);
    let mut m = Mat3::default();
    q.set_rotation3(&mut m);
    assert_eq!(<Mat3 as AsRef<[f32; 9]>>::as_ref(&m), &[1.,0.,0., 0.,0.,-1., 0.,1.,0.]);
    let q2 = Quat::of_rotation3(&m);
    assert!(quat_eq(&q, &q2));

    q = Quat::of_rijk(1., 2., 3., 4.);
    q.normalize();
    q.set_rotation3(&mut m);
    let q2 = Quat::of_rotation3(&m);
    assert!(quat_eq(&q, &q2));

    q = Quat::of_rijk(4., 3., 2., 1.);
    q.normalize();
    q.set_rotation3(&mut m);
    let q2 = Quat::of_rotation3(&m);
    assert!(quat_eq(&q, &q2));
}
