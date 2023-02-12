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
use crate::matrix_op as matrix;
use crate::vector_op as vector;
use crate::{Float, Num};

//a Notes on matrices of quaternions
/// 1 - 2*j2 - 2*k2           2*i*j - 2*k*r        2*i*k + 2*j*r
///     2*i*j + 2*k*r     1 - 2*i2 - 2*k2          2*j*k - 2*i*r
///     2*i*k - 2*j*r         2*j*k + 2*i*r    1 - 2*i2 - 2*j2
///
/// m[0] + m[4] + m[9] = 3 - 4(i^2+j^2+k^2) = 3 - 4(1-r^2) = 4r^2 - 1
/// m[7] - m[5] = 4*i*r ( if <0 then i<0 )
/// m[2] - m[6] = 4*j*r ( if <0 then j<0 )
/// m[3] - m[1] = 4*k*r ( if <0 then k<0 )

//a Constructors and destructors
//fp new
/// Create a new quaternion
pub fn new<V: Num>() -> [V; 4] {
    [V::zero(), V::zero(), V::zero(), V::one()]
}

//fp as_rijk
/// Return the breakdown of a quaternion
#[inline]
pub fn as_rijk<V: Num>(v: &[V; 4]) -> (V, V, V, V) {
    (v[3], v[0], v[1], v[2])
}

//fp of_rijk
/// Create a quaternion from its components
#[inline]
pub fn of_rijk<V: Num>(r: V, i: V, j: V, k: V) -> [V; 4] {
    [i, j, k, r]
}

//fp identity
/// Create an identity quaternion
#[inline]
pub fn identity<V: Num>() -> [V; 4] {
    [V::zero(), V::zero(), V::zero(), V::one()]
}

//fp of_axis_angle
/// Find the quaternion for a rotation of an angle around an axis
pub fn of_axis_angle<V: Float>(axis: &[V; 3], angle: V) -> [V; 4] {
    let (s, c) = V::sin_cos(angle / V::from(2).unwrap());
    let l = vector::length(axis);
    if l < V::epsilon() {
        identity()
    } else {
        let s = s / l;
        let i = s * axis[0];
        let j = s * axis[1];
        let k = s * axis[2];
        let r = c;
        [i, j, k, r]
    }
}

//fp as_axis_angle
/// Return the axis of the rotation and the angle from the quaternion
pub fn as_axis_angle<V: Float>(q: &[V; 4]) -> ([V; 3], V) {
    let (r, i, j, k) = as_rijk(q);
    let i2 = i * i;
    let j2 = j * j;
    let k2 = k * k;
    let l = (i2 + j2 + k2).sqrt();
    if l < V::epsilon() {
        ([i, j, k], V::zero())
    } else {
        let rl = V::one() / l;
        ([i * rl, j * rl, k * rl], V::atan2(l, r))
    }
}

//fp to_rotation3
/// Convert a matrix-3 from the quaternion
pub fn to_rotation3<V: Float>(q: &[V; 4], m: &mut [V; 9]) {
    let i2 = q[0] * q[0];
    let j2 = q[1] * q[1];
    let k2 = q[2] * q[2];
    let r2 = q[3] * q[3];

    let l2 = r2 + i2 + j2 + k2;
    let rl2 = V::one() / l2;

    m[0] = (r2 + i2 - j2 - k2) * rl2;
    m[4] = (r2 - i2 + j2 - k2) * rl2;
    m[8] = (r2 - i2 - j2 + k2) * rl2;

    let drl2 = V::frac(2, 1) * rl2;

    m[1] = (q[0] * q[1] - q[2] * q[3]) * drl2;
    m[3] = (q[0] * q[1] + q[2] * q[3]) * drl2;

    m[2] = (q[2] * q[0] + q[1] * q[3]) * drl2;
    m[6] = (q[2] * q[0] - q[1] * q[3]) * drl2;

    m[5] = (q[1] * q[2] - q[0] * q[3]) * drl2;
    m[7] = (q[1] * q[2] + q[0] * q[3]) * drl2;
}

//fp to_rotation4
/// Convert to a matrix-4 from a unit quaternion
pub fn to_rotation4<V: Float>(q: &[V; 4], m: &mut [V; 16]) {
    let i2 = q[0] * q[0];
    let j2 = q[1] * q[1];
    let k2 = q[2] * q[2];
    let r2 = q[3] * q[3];

    let l2 = r2 + i2 + j2 + k2;
    let rl2 = V::one() / l2;

    m[0] = (r2 + i2 - j2 - k2) * rl2;
    m[5] = (r2 - i2 + j2 - k2) * rl2;
    m[10] = (r2 - i2 - j2 + k2) * rl2;

    let drl2 = V::frac(2, 1) * rl2;

    m[1] = (q[0] * q[1] - q[2] * q[3]) * drl2;
    m[4] = (q[0] * q[1] + q[2] * q[3]) * drl2;

    m[2] = (q[2] * q[0] + q[1] * q[3]) * drl2;
    m[8] = (q[2] * q[0] - q[1] * q[3]) * drl2;

    m[6] = (q[1] * q[2] - q[0] * q[3]) * drl2;
    m[9] = (q[1] * q[2] + q[0] * q[3]) * drl2;

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
pub fn of_rotation<V: Float>(m: &[V; 9]) -> [V; 4] {
    fn safe_sqrt<V: Float>(x: V) -> V {
        if x < V::zero() {
            V::zero()
        } else {
            x.sqrt()
        }
    }
    let r = safe_sqrt(V::one() + m[0] + m[4] + m[8]) * V::frac(1, 2);
    let mut i = safe_sqrt(V::one() + m[0] - m[4] - m[8]) * V::frac(1, 2);
    let mut j = safe_sqrt(V::one() - m[0] + m[4] - m[8]) * V::frac(1, 2);
    let mut k = safe_sqrt(V::one() - m[0] - m[4] + m[8]) * V::frac(1, 2);

    let r_i_4 = m[7] - m[5];
    let r_j_4 = m[2] - m[6];
    let r_k_4 = m[3] - m[1];
    if r_i_4 < -V::epsilon() {
        i = -i;
    }
    if r_j_4 < -V::epsilon() {
        j = -j;
    }
    if r_k_4 < -V::epsilon() {
        k = -k;
    }

    [i, j, k, r]
}

//fp look_at
/// Create quaternion for a rotation that maps unit dirn to (0,0,-1) and unit up to (0,1,0)
pub fn look_at<V: Float>(dirn: &[V; 3], up: &[V; 3]) -> [V; 4] {
    let m = matrix::look_at3(dirn, up);
    of_rotation(&m)
}

//a Mapping functions
//cp invert
/// Get the quaternion inverse
pub fn invert<V: Float>(a: &[V; 4]) -> [V; 4] {
    let l = vector::length_sq(a);
    let r_l = {
        if l < V::epsilon() {
            V::zero()
        } else {
            V::one() / l
        }
    };
    [-a[0] * r_l, -a[1] * r_l, -a[2] * r_l, a[3] * r_l]
}

//cp conjugate
/// Find the conjugate of a quaternion
pub fn conjugate<V: Num>(a: &[V; 4]) -> [V; 4] {
    [-a[0], -a[1], -a[2], a[3]]
}

//cp normalize
/// Find the conjugate of a quaternion
pub fn normalize<V: Float>(a: [V; 4]) -> [V; 4] {
    vector::normalize(a)
}

//cp rotate_x
/// Find a rotation about the X-axis
pub fn rotate_x<V: Float>(a: &[V; 4], angle: V) -> [V; 4] {
    let (s, c) = V::sin_cos(angle / V::from(2).unwrap());
    let i = a[0] * c + a[3] * s;
    let j = a[1] * c + a[2] * s;
    let k = a[2] * c - a[1] * s;
    let r = a[3] * c - a[0] * s;
    [i, j, k, r]
}

//cp rotate_y
/// Find a rotation about the Y-axis
pub fn rotate_y<V: Float>(a: &[V; 4], angle: V) -> [V; 4] {
    let (s, c) = V::sin_cos(angle / V::from(2).unwrap());
    let i = a[0] * c - a[2] * s;
    let j = a[1] * c + a[3] * s;
    let k = a[2] * c + a[0] * s;
    let r = a[3] * c - a[1] * s;
    [i, j, k, r]
}

//cp rotate_z
/// Find a rotation about the Z-axis
pub fn rotate_z<V: Float>(a: &[V; 4], angle: V) -> [V; 4] {
    let (s, c) = V::sin_cos(angle / V::from(2).unwrap());
    let i = a[0] * c + a[1] * s;
    let j = a[1] * c - a[0] * s;
    let k = a[2] * c + a[3] * s;
    let r = a[3] * c - a[2] * s;
    [i, j, k, r]
}

//cp multiply
/// Multiply two quaternions together
#[inline]
pub fn multiply<V: Num>(a: &[V; 4], b: &[V; 4]) -> [V; 4] {
    let i = a[0] * b[3] + a[3] * b[0] + a[1] * b[2] - a[2] * b[1];
    let j = a[1] * b[3] + a[3] * b[1] + a[2] * b[0] - a[0] * b[2];
    let k = a[2] * b[3] + a[3] * b[2] + a[0] * b[1] - a[1] * b[0];
    let r = a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2];
    [i, j, k, r]
}

//cp divide
/// Multiply one quaternion by the conjugate of the other / len2 of other
pub fn divide<V: Float>(a: &[V; 4], b: &[V; 4]) -> [V; 4] {
    let l2 = vector::length_sq(b);
    if l2 < V::epsilon() {
        [V::zero(); 4]
    } else {
        let i = a[0] * b[3] - a[3] * b[0] - a[1] * b[2] + a[2] * b[1];
        let j = a[1] * b[3] - a[3] * b[1] - a[2] * b[0] + a[0] * b[2];
        let k = a[2] * b[3] - a[3] * b[2] - a[0] * b[1] + a[1] * b[0];
        let r = a[3] * b[3] + a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        [i / l2, j / l2, k / l2, r / l2]
    }
}

//fp nlerp
/// A simple normalized LERP from one quaterion to another (not spherical)
pub fn nlerp<V: Float>(t: V, in0: &[V; 4], in1: &[V; 4]) -> [V; 4] {
    normalize(vector::mix(in0, in1, t))
}

//a Operational functions
//fp distance_sq
/// Get a measure of the 'distance' between two quaternions
pub fn distance_sq<V: Float>(a: &[V; 4], b: &[V; 4]) -> V {
    let qi = invert(a);
    let mut qn = multiply(&qi, b);
    if qn[3] < V::zero() {
        qn[3] += V::one();
    } else {
        qn[3] -= V::one();
    }
    vector::length_sq(&qn)
}

//fp distance
/// Get a measure of the 'distance' between two quaternions
pub fn distance<V: Float>(a: &[V; 4], b: &[V; 4]) -> V {
    distance_sq(a, b).sqrt()
}

//fp get_axis_angle
/// Get the axis of a quaternion, and the angle of rotation it corresponds to
pub fn get_axis_angle<V: Float>(q: &[V; 4]) -> ([V; 3], V) {
    let angle = V::from(2).unwrap() * V::acos(q[3]);
    let axis = vector::normalize([q[0], q[1], q[2]]);
    (axis, angle)
}

//fp to_euler
/// Convert the quaternion to a bank, heading, altitude tuple - applied in that order
pub fn to_euler<V: Float>(q: &[V; 4]) -> (V, V, V) {
    let i = q[0];
    let j = q[1];
    let k = q[2];
    let r = q[3];
    let test = i * j + r * k;
    let two = V::from(2).unwrap();
    let almost_half = V::from(4_999_999).unwrap() / V::from(10_000_000).unwrap();
    let halfpi = V::zero().acos();
    let (heading, attitude, bank) = {
        if test > almost_half {
            (two * V::atan2(i, r), halfpi, V::zero())
        } else if test < -almost_half {
            (-two * V::atan2(i, r), -halfpi, V::zero())
        } else {
            let i2 = i * i;
            let j2 = j * j;
            let k2 = k * k;
            (
                V::atan2(two * j * r - two * i * k, V::one() - two * j2 - two * k2),
                V::asin(two * test),
                V::atan2(two * i * r - two * j * k, V::one() - two * i2 - two * k2),
            )
        }
    };
    (bank, heading, attitude)
}

//fp apply3
/// Apply the quaternion to a vector3
pub fn apply3<V: Float>(q: &[V; 4], v: &[V; 3]) -> [V; 3] {
    let (r, i, j, k) = as_rijk(q);
    let two = V::frac(2, 1);
    let x = (r * r + i * i - j * j - k * k) * v[0]
        + two * (i * k + r * j) * v[2]
        + two * (i * j - r * k) * v[1];
    let y = (r * r - i * i + j * j - k * k) * v[1]
        + two * (j * i + r * k) * v[0]
        + two * (j * k - r * i) * v[2];
    let z = (r * r - i * i - j * j + k * k) * v[2]
        + two * (k * j + r * i) * v[1]
        + two * (k * i - r * j) * v[0];
    [x, y, z]
}

//fp apply4
/// Apply the quaternion to a vector3
pub fn apply4<V: Float>(q: &[V; 4], v: &[V; 4]) -> [V; 4] {
    let (r, i, j, k) = as_rijk(q);
    let two = V::frac(2, 1);
    let x = (r * r + i * i - j * j - k * k) * v[0]
        + two * (i * k + r * j) * v[2]
        + two * (i * j - r * k) * v[1];
    let y = (r * r - i * i + j * j - k * k) * v[1]
        + two * (j * i + r * k) * v[0]
        + two * (j * k - r * i) * v[2];
    let z = (r * r - i * i - j * j + k * k) * v[2]
        + two * (k * j + r * i) * v[1]
        + two * (k * i - r * j) * v[0];
    [x, y, z, v[3]]
}

//fp weighted_average
/// Calculate the weighted average of two unit quaternions
///
/// w_a + w_b must be 1.
///
/// See http://www.acsu.buffalo.edu/~johnc/ave_quat07.pdf
/// Averaging Quaternions by F. Landis Markley
pub fn weighted_average<V: Float>(qa: &[V; 4], w_a: V, qb: &[V; 4], w_b: V) -> [V; 4] {
    let (ra, ia, ja, ka) = as_rijk(qa);
    let (rb, ib, jb, kb) = as_rijk(qb);
    let four = V::frac(4, 1);
    let w_diff = w_a - w_b;
    let q1_q2 = ra * rb + ia * ib + ja * jb + ka * kb;
    let z_sq = w_diff * w_diff + four * w_a * w_b * q1_q2 * q1_q2;
    let z = z_sq.sqrt();
    let rw_a_sq = w_a * (z + w_diff) / z / (z + w_a + w_b);
    let rw_b_sq = w_b * (z - w_diff) / z / (z + w_a + w_b);
    let rw_a = rw_a_sq.sqrt();
    let rw_b = rw_b_sq.sqrt() * q1_q2.signum();
    of_rijk(
        rw_a * ra + rw_b * rb,
        rw_a * ia + rw_b * ib,
        rw_a * ja + rw_b * jb,
        rw_a * ka + rw_b * kb,
    )
}

//fp weighted_average_many
/// Calculate the weighted average of many unit quaternions
///
/// weights need not add up to 1
///
/// This is an approximation compared to the Landis Markley paper
pub fn weighted_average_many<V: Float>(values: &[(V, [V; 4])]) -> [V; 4] {
    assert!(!values.is_empty());
    let num_values = values.len();
    if num_values == 1 {
        values[0].1
    } else {
        let mut next_values = Vec::new();
        for i in 0..(num_values + 1) / 2 {
            if 2 * i + 1 == num_values {
                let (w, v) = values[2 * i];
                next_values.push((w, v));
            } else {
                let (w1, v1) = values[2 * i];
                let (w2, v2) = values[2 * i + 1];
                let w12 = w1 + w2;
                let av = weighted_average(&v1, w1 / w12, &v2, w2 / w12);
                next_values.push((w12, av));
            }
        }
        weighted_average_many(&next_values)
    }
}

//fp get_rotation_of_vec_to_vec
/// Get a quaternion that is a rotation of one vector to another
///
/// The vectors must be unit vectors
pub fn get_rotation_of_vec_to_vec<V: Float>(a: &[V; 3], b: &[V; 3]) -> [V; 4] {
    let obtuse = vector::dot(a, b) < V::zero();
    let cp = vector::cross_product3(a, b);
    let sa = vector::length(&cp);
    let angle = sa.asin();
    let angle = if obtuse {
        let pi = (-V::one()).acos();
        pi - angle
    } else {
        angle
    };
    of_axis_angle(&cp, angle)
}
