//a Imports
extern crate geometry;

use geometry::{Vector, Vector3D};
use std::marker::PhantomData;

//a Test type
//ti Banana
// This type is required so that implementations of the associated types of V (e.g. Vec2/3/4) can be used within
struct Banana<V:Vector3D<f32>> {p:PhantomData<V>}

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
impl <V2, V3, V4, V> Banana<V>
where V2:Vector<f32,2>,
      V3:Vector<f32,3>,
      V4:Vector<f32,4>,
      V:Vector3D<f32, Vec2 = V2, Vec3 = V3, Vec4 = V4> {
    // test length, length_sq, distance, distance_sq, and ALU op both of V and f32
    fn test_vecn_len_dist<T:Vector<f32,D>, const D:usize>(zero:T, ones:T, v:T, v_len2:f32, v_o_dist2:f32) {
        assert_eq!(zero.length(), 0., "Length of zero vector is 0");
        assert_eq!(zero.length_sq(), 0., "Length^2 of zero vector is 0");

        let v_len = v_len2.sqrt();
        assert_eq!(v.length(), v_len, "Length of 3,4,12 vector is 13");
        assert_eq!(v.length_sq(), v_len2, "Length^2 of 3,4,12 vector is 13");
        assert_eq!(v.distance(&zero), v_len, "Separation is 13");
        assert_eq!(v.distance_sq(&zero), v_len2, "Separation^2 is 13");

        assert_eq!(ones.length_sq(), D as f32, "Length^2 of ones vector is #dim");
        assert_eq!((ones * 0.).length(), 0., "Length of ones*0 is 0");
        assert_eq!((ones * 1.).length_sq(), D as f32, "Length of ones*1 is 13");
        assert_eq!((ones / 1.).length_sq(), D as f32, "Length of ones/1 is 13");
        assert_eq!((ones + zero).length_sq(), D as f32, "Length of ones + zero is 13");
        assert_eq!((ones - zero).length_sq(), D as f32, "Length of ones + zero is 13");

        assert_eq!((v * 0.).length(), 0., "Length of v*0 is 0");
        assert_eq!((v * 1.).length(), v_len, "Length of v*1 is 13");
        assert_eq!((v / 1.).length(), v_len, "Length of v/1 is 13");
        assert_eq!((v + zero).length(), v_len, "Length of v + zero is 13");
        assert_eq!((v - zero).length(), v_len, "Length of v + zero is 13");
        assert_eq!((v * 2.).length(), 2.*v_len, "Length of v*2 is 26");
        assert_eq!((v * ones).length(), v_len, "Length of v*ones is 13");
        assert_eq!((v.distance_sq(&ones)), v_o_dist2, "Distance^2 of v-ones is 134");
        assert_eq!((v - 1.).length_sq(), v_o_dist2, "v - 1. Length 134");
        assert_eq!((v + (-1.)).length_sq(), v_o_dist2, "v + -1. Length 134");
        assert_eq!(((-v) + zero).length(), v_len, "Length of (-v) + zero is 13");
        assert_eq!(((-v) + v).length(), 0., "Length of (-v) + v is 0");
    }
    // test all assign_op and index access
    fn test_vecn_index<T:Vector<f32,D>, const D:usize>(zero:T, ones:T, v:T, n:usize, vn:f32) {
        assert_eq!(zero[n], 0., "zero has coordinate N of 0.");
        assert_eq!(ones[n], 1., "ones has coordinate N of 1.");
        assert_eq!(v[n], vn, "v has coordinate N of vn");
        assert_eq!((v-zero)[n], vn, "v-zeros has coordinate N of vn");
        assert_eq!((v-ones)[n], vn-1., "v-ones has coordinate N of vn-1");
        assert_eq!((v+0.)[n], vn, "v+[0.] has coordinate N of vn");
        assert_eq!((v+1.)[n], vn+1., "v+[1.] has coordinate N of vn+1");
        assert_eq!((v-0.)[n], vn, "v-[0.] has coordinate N of vn");
        assert_eq!((v-1.)[n], vn-1., "v-[1.] has coordinate N of vn-1");
        assert_eq!((v*0.)[n], 0., "v*[0.] has coordinate N of 0");
        assert_eq!((v*1.)[n], vn, "v*[1.] has coordinate N of vn");
        assert_eq!((v*0.)[n], 0., "v*[0.] has coordinate N of 0");
        assert_eq!((v*1.)[n], vn, "v*[1.] has coordinate N of vn");
        let mut d = v;
        d *= ones;
        assert_eq!(d[n], vn);
        d += ones;
        assert_eq!(d[n], vn+1.);
        d /= ones;
        assert_eq!(d[n], vn+1.);
        d -= ones;
        assert_eq!(d[n], vn);
        d += d;
        d *= 3.;
        d -= v;
        d /= 5.;
        assert_eq!(d[n], vn);
        d += 2.;
        assert_eq!(d[n], vn+2.);
        d -= 3.;
        assert_eq!(d[n], vn-1.);
    }

    fn test_vec4() {
        let zero  =  V4::zero();
        let ones  =  V4::from_array([1.,1.,1.,1.]);
        let b     =  V4::from_array([3.,4.,12.,84.]);
        Self::test_vecn_len_dist( zero, ones, b, 85.*85., 134.+83.*83.);
        Self::test_vecn_index( zero, ones, b, 0, 3.);
        Self::test_vecn_index( zero, ones, b, 1, 4.);
        Self::test_vecn_index( zero, ones, b, 2, 12.);
        Self::test_vecn_index( zero, ones, b, 3, 84.);
    }

    fn test_vec3() {
        let zero  =  V3::zero();
        let ones  =  V3::from_array([1.,1.,1.]);
        let b     =  V3::from_array([3.,4.,12.]);
        Self::test_vecn_len_dist( zero, ones, b, 169., 134.);
        Self::test_vecn_index( zero, ones, b, 0, 3.);
        Self::test_vecn_index( zero, ones, b, 1, 4.);
        Self::test_vecn_index( zero, ones, b, 2, 12.);
    }

    fn test_vec2() {
        let zero  =  V2::zero();
        let ones  =  V2::from_array([1.,1.]);
        let b     =  V2::from_array([3.,4.]);
        Self::test_vecn_len_dist( zero, ones, b, 25., 13.);
        Self::test_vecn_index( zero, ones, b, 0, 3.);
        Self::test_vecn_index( zero, ones, b, 1, 4.);
    }
}

#[test]
fn test_fslice() {
    Banana::<f32>::test_vec2();
    Banana::<f32>::test_vec3();
    Banana::<f32>::test_vec4();
}

#[cfg(feature="simd")]
extern crate core_simd;
#[cfg(feature="simd")]
mod test_simd {
    use geometry::simd::{VecF32A16};
    #[test]
    fn test_simd() {
        super::Banana::<VecF32A16>::test_vec2();
        super::Banana::<VecF32A16>::test_vec3();
        super::Banana::<VecF32A16>::test_vec4();
    }
}
