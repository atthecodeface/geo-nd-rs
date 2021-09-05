//a Imports
use geo_nd::quat;
use geo_nd::{FArray, QArray, Float, Vector, Quaternion};
use std::marker::PhantomData;

//a Test type
//ti Banana
// This type is required so that implementations of the associated types of V (e.g. Vec2/3/4) can be used within

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
fn quat_eq_rijk<F:Float, Q:Quaternion<F>>(q:&Q, rijk:(F, F, F, F)) -> bool {
    let eq = Q::of_rijk(rijk.0, rijk.1, rijk.2, rijk.3);
    let d_sub = eq - *q;
    let d_add = eq + *q;
    if d_sub.length_sq() < F::frac(1,1000) {
        true
    } else if d_add.length_sq() < F::frac(1,1000) {
        true
    } else {
        dbg!(q, rijk, d_sub, d_sub.length_sq(), d_add, d_add.length_sq());
        false
    }
}
#[test]
fn test() {
    let x = FArray::<f32, 3>::from_array([1.,0.,0.]);
    let y = FArray::<f32, 3>::from_array([0.,1.,0.]);
    let z = FArray::<f32, 3>::from_array([0.,0.,1.]);

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

    let q = QArray::<f32>::unit();
    assert_eq!(q.length(), 1.);
    assert_eq!(q.length_sq(), 1.);
    assert_eq!(quat::as_rijk(q.as_ref()), (1., 0., 0., 0.));
    assert_eq!(q.as_rijk(), (1., 0., 0., 0.));
    assert_eq!(q.conjugate().as_rijk(), (1., 0., 0., 0.));

    let q = QArray::<f32>::of_rijk(0.,1.,0.,0.);
    assert_eq!(q.length(), 1.);
    assert_eq!(q.length_sq(), 1.);
    assert_eq!(quat::as_rijk(q.as_ref()), (0., 1., 0., 0.));
    assert_eq!(q.as_rijk(), (0., 1., 0., 0.));
    assert_eq!(q.conjugate().as_rijk(), (0., -1., 0., 0.));
    
    let q = QArray::<f32>::of_rijk(0.,0.,1.,0.);
    assert_eq!(q.length(), 1.);
    assert_eq!(q.length_sq(), 1.);
    assert_eq!(q.as_rijk(), (0., 0., 1., 0.));
    assert_eq!(q.conjugate().as_rijk(), (0., 0., -1., 0.));
    
    let q = QArray::<f32>::of_rijk(0.,0.,0.,1.);
    assert_eq!(q.length(), 1.);
    assert_eq!(q.length_sq(), 1.);
    assert_eq!(q.as_rijk(), (0., 0., 0., 1.));
    assert_eq!(q.conjugate().as_rijk(), (0., 0., 0., -1.));
    
    let q = QArray::<f32>::of_rijk(1.,1.,1.,1.);
    assert_eq!(q.length(), 2.);
    assert_eq!(q.length_sq(), 4.);
    assert_eq!(q.as_rijk(), (1., 1., 1., 1.));
    assert_eq!(q.conjugate().as_rijk(), (1., -1., -1., -1.));
    
    assert_eq!(QArray::<f32>::of_axis_angle(&x, 0.).as_rijk(), (1., 0., 0., 0.));
    assert_eq!(QArray::<f32>::of_axis_angle(&y, 0.).as_rijk(), (1., 0., 0., 0.));
    assert_eq!(QArray::<f32>::of_axis_angle(&z, 0.).as_rijk(), (1., 0., 0., 0.));
    assert_eq!(QArray::<f32>::of_axis_angle(&xy, 0.).as_rijk(), (1., 0., 0., 0.));
    assert_eq!(QArray::<f32>::of_axis_angle(&yz, 0.).as_rijk(), (1., 0., 0., 0.));
    assert_eq!(QArray::<f32>::of_axis_angle(&xz, 0.).as_rijk(), (1., 0., 0., 0.));

    assert!(quat_eq_rijk(&QArray::<f32>::of_axis_angle(&x, ra), (rsqrt2, rsqrt2, 0., 0.)));
    assert!(quat_eq_rijk(&QArray::<f32>::of_axis_angle(&x, 2.*ra), (0., 1., 0., 0.)));

    assert!(quat_eq_rijk(&QArray::<f32>::of_axis_angle(&y, ra), (rsqrt2, 0., rsqrt2, 0.)));
    assert!(quat_eq_rijk(&QArray::<f32>::of_axis_angle(&y, 2.*ra), (0., 0., 1., 0.)));

    assert!(quat_eq_rijk(&QArray::<f32>::of_axis_angle(&z, ra), (rsqrt2, 0., 0., rsqrt2)));
    assert!(quat_eq_rijk(&QArray::<f32>::of_axis_angle(&z, 2.*ra), (0., 0., 0., 1.)));

    assert!(quat_eq_rijk(&QArray::<f32>::of_axis_angle(&xy, ra), (rsqrt2, 0.5, 0.5, 0.)));
    assert!(quat_eq_rijk(&QArray::<f32>::of_axis_angle(&yz, ra), (rsqrt2, 0.,  0.5, 0.5)));
    assert!(quat_eq_rijk(&QArray::<f32>::of_axis_angle(&xz, ra), (rsqrt2, 0.5,  0., 0.5)));

    let x90 = QArray::<f32>::of_axis_angle(&x, ra);
    let y90 = QArray::<f32>::of_axis_angle(&y, ra);
    let z90 = QArray::<f32>::of_axis_angle(&z, ra);

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
        let t = QArray::of_axis_angle(axis, ra/3.); // 12 of these makes 360
        let t2 = t * t; // 6 of these
        let t4 = t2 * t2; // 3 of these
        let t12 = t4 * t4 * t4;
        assert!(quat_eq_rijk(&t12, (1., 0., 0., 0.)));
    }
}
