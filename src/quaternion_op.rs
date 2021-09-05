/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    quaternion_op.rs
@brief   Part of geometry library
 */

//a Imports
use crate::{Num, Float};
use crate::vector_op as vector;
use crate::matrix_op as matrix;

//a Notes on matrices of quaternions
/// 1 - 2*j2 - 2*k2           2*i*j - 2*k*r        2*i*k + 2*j*r
///     2*i*j + 2*k*r     1 - 2*i2 - 2*k2          2*j*k - 2*i*r
///     2*i*k - 2*j*r         2*j*k + 2*i*r    1 - 2*i2 - 2*j2
///
/// m[0] + m[4] + m[9] = 3 - 4(i^2+j^2+k^2) = 3 - 4(1-r^2) = 4r^2 - 1
/// m[7] - m[5] = 4*i*r ( if <0 then i<0 )
/// m[2] - m[6] = 4*j*r ( if <0 then j<0 )
/// m[3] - m[1] = 4*k*r ( if <0 then k<0 )

//a Constructors and desconstructors
//fp new
/// Create a new quaternion
pub fn new<V:Num>() -> [V; 4] {
    [V::zero(), V::zero(), V::zero(), V::one(), ]
}

//fp as_rijk
/// Return the breakdown of a quaternion
pub fn as_rijk<V:Num>(v:&[V;4]) -> (V, V, V, V) {
    (v[3], v[0], v[1], v[2])
}

//fp of_rijk
/// Create a quaternion from its components
pub fn of_rijk<V:Num>(r:V, i:V, j:V, k:V) -> [V;4] {
    [i, j, k, r]
}

//fp identity
/// Create an identity quaternion
pub fn identity<V:Num>() -> [V;4] {
    [V::zero(), V::zero(), V::zero(), V::one()]
}

//fp of_axis_angle
/// Find the quaternion for a rotation of an angle around an axis
pub fn of_axis_angle<V:Float>(axis:&[V;3], angle:V) -> [V;4] {
    let (s,c) = V::sin_cos(angle / V::from(2).unwrap());
        let i = s * axis[0];
        let j = s * axis[1];
        let k = s * axis[2];
        let r = c;
    [ i, j, k, r ]
}

//fp to_rotation3
/// Convert a matrix-3 from the quaternion
pub fn to_rotation3<V:Float>(q:&[V;4], m:&mut [V;9])  {
    let i2 = q[0] * q[0];
    let j2 = q[1] * q[1];
    let k2 = q[2] * q[2];
    let r2 = q[3] * q[3];

    let l2  = r2 + i2 + j2 + k2;
    let rl2 = V::one() / l2;

    m[0] = (r2 + i2 - j2 - k2) * rl2;
    m[4] = (r2 - i2 + j2 - k2) * rl2;
    m[8] = (r2 - i2 - j2 + k2) * rl2;

    let drl2 = V::frac(2,1) * rl2;

    m[1] = (q[0]*q[1] - q[2]*q[3]) * drl2 ;
    m[3] = (q[0]*q[1] + q[2]*q[3]) * drl2 ;

    m[2] = (q[2]*q[0] + q[1]*q[3]) * drl2 ;
    m[6] = (q[2]*q[0] - q[1]*q[3]) * drl2 ;
    
    m[5] = (q[1]*q[2] - q[0]*q[3]) * drl2 ;
    m[7] = (q[1]*q[2] + q[0]*q[3]) * drl2 ;
    
}

//fp to_rotation4
/// Convert to a matrix-4 from a unit quaternion
pub fn to_rotation4<V:Float>(q:&[V;4], m:&mut [V;16])  {
    let i2 = q[0] * q[0];
    let j2 = q[1] * q[1];
    let k2 = q[2] * q[2];
    let r2 = q[3] * q[3];

    let l2  = r2 + i2 + j2 + k2;
    let rl2 = V::one() / l2;

    m[0] = (r2 + i2 - j2 - k2) * rl2;
    m[5] = (r2 - i2 + j2 - k2) * rl2;
    m[10] = (r2 - i2 - j2 + k2) * rl2;

    let drl2 = V::frac(2,1) * rl2;

    m[1] = (q[0]*q[1] - q[2]*q[3]) * drl2 ;
    m[4] = (q[0]*q[1] + q[2]*q[3]) * drl2 ;

    m[2] = (q[2]*q[0] + q[1]*q[3]) * drl2 ;
    m[8] = (q[2]*q[0] - q[1]*q[3]) * drl2 ;
    
    m[6] = (q[1]*q[2] - q[0]*q[3]) * drl2 ;
    m[9] = (q[1]*q[2] + q[0]*q[3]) * drl2 ;

    m[3] = V::zero();
    m[7] = V::zero();
    m[11] = V::zero();
    m[12] = V::zero();
    m[13] = V::zero();
    m[14] = V::zero();
    m[15] = V::one();
}

//fp of_rotation
/// Find the quaternion of a Matrix3 assuming it is purely a rotation
pub fn of_rotation<V:Float>(m:&[V;9]) -> [V;4] {
    fn safe_sqrt<V:Float>(x:V) -> V { if x<V::zero() {V::zero()} else {x.sqrt()} }
    let r = safe_sqrt(V::one() + m[0] + m[4] + m[8]) * V::frac(1,2);
    let mut i = safe_sqrt(V::one() + m[0] - m[4] - m[8]) * V::frac(1,2);
    let mut j = safe_sqrt(V::one() - m[0] + m[4] - m[8]) * V::frac(1,2);
    let mut k = safe_sqrt(V::one() - m[0] - m[4] + m[8]) * V::frac(1,2);

    let r_i_4 = m[7] - m[5];
    let r_j_4 = m[2] - m[6];
    let r_k_4 = m[3] - m[1];
    if r_i_4 < -V::epsilon() { i = -i; }
    if r_j_4 < -V::epsilon() { j = -j; }
    if r_k_4 < -V::epsilon() { k = -k; }

    [i, j, k, r]
}

//fp of_rotation_old
/// Find the quaternion of a Matrix3 assuming it is purely a rotation
pub fn of_rotation_old<V:Float>(rotation:&[V;9]) -> [V;4] {
    let axis = vector::axis_of_rotation3(rotation);

    // Find a decent vector not parallel to the axis
    let mut w = [V::one(), V::zero(), V::zero()];
    if V::abs(axis[0]) > (V::from(9).unwrap() / V::from(10).unwrap()) { w[0] = V::zero(); w[1] = V::one(); }

    // Find three vectors (axis, na0, na1) that are all mutually perpendicular
    let na0 = vector::normalize(vector::cross_product3(&w, &axis));
    let na1 = vector::normalize(vector::cross_product3(&axis, &na0));

    // Rotate na0, na1 around the axis of rotation by angle A - i.e. apply 'rotation'
    let na0_r = matrix::transform_vec3( &rotation, &na0 );
    let na1_r = matrix::transform_vec3( &rotation, &na1 );

    //  Get angle of rotation
    let cos_angle =  vector::dot(&na0, &na0_r);
    let sin_angle = -vector::dot(&na0, &na1_r);
    let angle     = V::atan2(sin_angle, cos_angle);

    of_axis_angle(&axis, angle)
}

//a Mapping functions
//cp invert
/// Get the quaternion inverse
pub fn invert<V:Float>(a:&[V;4]) -> [V;4] {
    let l = vector::length_sq(a);
    let r_l = {
        if l < V::epsilon() { V::zero() } else {V::one()/l}
    };
    [ -a[0]*r_l,
      -a[1]*r_l,
       -a[2]*r_l,
       a[3]*r_l ]
}

//cp conjugate
/// Find the conjugate of a quaternion
pub fn conjugate<V:Num>(a:&[V;4]) -> [V;4] {
    [ -a[0], -a[1], -a[2], a[3] ]
}

//cp normalize
/// Find the conjugate of a quaternion
pub fn normalize<V:Float>(a:[V;4]) -> [V;4] {
    vector::normalize(a)
}

//cp rotate_x
/// Find a rotation about the X-axis
pub fn rotate_x<V:Float>(a:&[V;4], angle:V) -> [V;4] {
    let (s,c) = V::sin_cos(angle / V::from(2).unwrap());
    let i = a[0] * c + a[3] * s;
    let j = a[1] * c + a[2] * s;
    let k = a[2] * c - a[1] * s;
    let r = a[3] * c - a[0] * s;
    [ i, j, k, r ]
}

//cp rotate_y
/// Find a rotation about the Y-axis
pub fn rotate_y<V:Float>(a:&[V;4], angle:V) -> [V;4] {
    let (s,c) = V::sin_cos(angle / V::from(2).unwrap());
    let i = a[0] * c - a[2] * s;
    let j = a[1] * c + a[3] * s;
    let k = a[2] * c + a[0] * s;
    let r = a[3] * c - a[1] * s;
    [ i, j, k, r ]
}

//cp rotate_z
/// Find a rotation about the Z-axis
pub fn rotate_z<V:Float>(a:&[V;4], angle:V) -> [V;4] {
    let (s,c) = V::sin_cos(angle / V::from(2).unwrap());
    let i = a[0] * c + a[1] * s;
    let j = a[1] * c - a[0] * s;
    let k = a[2] * c + a[3] * s;
    let r = a[3] * c - a[2] * s;
    [ i, j, k, r ]
}

//cp multiply
/// Multiply two quaternions together
pub fn multiply<V:Num>(a:&[V;4], b:&[V;4]) -> [V;4] {
    let i = a[0]*b[3] + a[3]*b[0] + a[1]*b[2] - a[2]*b[1];
    let j = a[1]*b[3] + a[3]*b[1] + a[2]*b[0] - a[0]*b[2];
    let k = a[2]*b[3] + a[3]*b[2] + a[0]*b[1] - a[1]*b[0];
    let r = a[3]*b[3] - a[0]*b[0] - a[1]*b[1] - a[2]*b[2];
    dbg!(a,b);
    dbg!(r,i,j,k);
    [ i, j, k, r ]
}

//cp divide
/// Multiply one quaternion by the conjugate of the other / len2 of other
pub fn divide<V:Float>(a:&[V;4], b:&[V;4]) -> [V;4] {
    let l2 = vector::length_sq(b);
    if l2 < V::epsilon() {
        [V::zero(); 4]
    } else {
        let i = a[0]*b[3] - a[3]*b[0] - a[1]*b[2] + a[2]*b[1];
        let j = a[1]*b[3] - a[3]*b[1] - a[2]*b[0] + a[0]*b[2];
        let k = a[2]*b[3] - a[3]*b[2] - a[0]*b[1] + a[1]*b[0];
        let r = a[3]*b[3] + a[0]*b[0] + a[1]*b[1] + a[2]*b[2];
        [ i/l2, j/l2, k/l2, r/l2 ]
    }
}

//fp nlerp
/// A simple normalized LERP from one quaterion to another (not spherical)
pub fn nlerp<V:Float>(t:V, in0:&[V;4], in1:&[V;4]) -> [V;4] {
    normalize(vector::mix(in0, in1, t))
}

//a Operational functions
//fp distance_sq
/// Get a measure of the 'distance' between two quaternions
pub fn distance_sq<V:Float>(a:&[V;4], b:&[V;4]) -> V {
    let qi = invert(a);
    let mut qn = multiply(&qi, b);
    if qn[3] < V::zero() {
        qn[3] = qn[3] + V::one();
    } else {
        qn[3] = qn[3] - V::one();
    }
    vector::length_sq(&qn)
}

//fp distance
/// Get a measure of the 'distance' between two quaternions
pub fn distance<V:Float>(a:&[V;4], b:&[V;4]) -> V {
    distance_sq(a,b).sqrt()
}

//fp get_axis_angle
/// Get the axis of a quaternion, and the angle of rotation it corresponds to
pub fn get_axis_angle<V:Float>(q:&[V;4]) -> ([V;3], V) {
    let angle = V::from(2).unwrap() * V::acos(q[3]);
    let axis  = vector::normalize([q[0], q[1], q[2]]);
    (axis, angle)
}

//fp to_euler
/// Convert the quaternion to a bank, heading, altitude tuple - applied in that order
pub fn to_euler<V:Float>(q:&[V;4]) -> (V,V,V) {
    let i=q[0];
    let j=q[1];
    let k=q[2];
    let r=q[3];
    let test = i*j + r*k;
    let two = V::from(2).unwrap();
    let almost_half = V::from(4_999_999).unwrap() / V::from(10_000_000).unwrap();
    let halfpi = V::zero().acos();
    let (heading, attitude, bank) = {
        if test > almost_half {
            (two*V::atan2(i,r), halfpi, V::zero())
        } else if test < -almost_half {
            (-two*V::atan2(i,r), -halfpi, V::zero())
        } else {
            let i2 = i*i;
            let j2 = j*j;
            let k2 = k*k;
            (V::atan2(two*j*r - two*i*k , V::one() - two*j2 - two*k2),
             V::asin(two*test),
             V::atan2(two*i*r - two*j*k , V::one() - two*i2 - two*k2)
            )
        }
    };
    (bank, heading, attitude)
}

