use crate::vector_op as vector;
use crate::{Float, Num, f_const, matrix};

//a Notes on matrices of quaternions
// 1 - 2*j2 - 2*k2           2*i*j - 2*k*r        2*i*k + 2*j*r
//     2*i*j + 2*k*r     1 - 2*i2 - 2*k2          2*j*k - 2*i*r
//     2*i*k - 2*j*r         2*j*k + 2*i*r    1 - 2*i2 - 2*j2
//
// m[0] + m[4] + m[9] = 3 - 4(i^2+j^2+k^2) = 3 - 4(1-r^2) = 4r^2 - 1
// m[7] - m[5] = 4*i*r ( if <0 then i<0 )
// m[2] - m[6] = 4*j*r ( if <0 then j<0 )
// m[3] - m[1] = 4*k*r ( if <0 then k<0 )

//a Constructors and destructors
//fp new
/// Create a new quaternion
#[must_use]
#[inline]
pub fn new<V: Num>() -> [V; 4] {
    [V::ZERO, V::ZERO, V::ZERO, V::ONE]
}

//fp as_rijk
/// Return the breakdown of a quaternion
#[must_use]
#[inline]
pub fn as_rijk<V: Num>(v: &[V; 4]) -> (V, V, V, V) {
    (v[3], v[0], v[1], v[2])
}

//fp of_rijk
/// Create a quaternion from its components
#[must_use]
#[inline]
pub fn of_rijk<V: Num>(r: V, i: V, j: V, k: V) -> [V; 4] {
    [i, j, k, r]
}

//fp identity
/// Create an identity quaternion
#[must_use]
#[inline]
pub fn identity<V: Num>() -> [V; 4] {
    [V::ZERO, V::ZERO, V::ZERO, V::ONE]
}

//fp of_axis_angle
/// Find the quaternion for a rotation of an angle around an axis
///
/// The axis need not be a unit vector
#[must_use]
#[inline]
pub fn of_axis_angle<V: Float>(axis: &[V; 3], angle: V) -> [V; 4] {
    let (s, c) = V::sin_cos(angle * f_const!(V, 1, 2));
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
#[must_use]
#[inline]
pub fn as_axis_angle<V: Float>(q: &[V; 4]) -> ([V; 3], V) {
    let (r, i, j, k) = as_rijk(q);
    let i2 = i * i;
    let j2 = j * j;
    let k2 = k * k;
    let l = (i2 + j2 + k2).sqrt();
    if l < V::epsilon() {
        ([i, j, k], V::ZERO)
    } else {
        let rl = V::ONE / l;
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
    let rl2 = V::ONE / l2;

    m[0] = (r2 + i2 - j2 - k2) * rl2;
    m[4] = (r2 - i2 + j2 - k2) * rl2;
    m[8] = (r2 - i2 - j2 + k2) * rl2;

    let drl2 = rl2 + rl2;

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
    let rl2 = V::ONE / l2;

    m[0] = (r2 + i2 - j2 - k2) * rl2;
    m[5] = (r2 - i2 + j2 - k2) * rl2;
    m[10] = (r2 - i2 - j2 + k2) * rl2;

    let drl2 = rl2 + rl2;

    m[1] = (q[0] * q[1] - q[2] * q[3]) * drl2;
    m[4] = (q[0] * q[1] + q[2] * q[3]) * drl2;

    m[2] = (q[2] * q[0] + q[1] * q[3]) * drl2;
    m[8] = (q[2] * q[0] - q[1] * q[3]) * drl2;

    m[6] = (q[1] * q[2] - q[0] * q[3]) * drl2;
    m[9] = (q[1] * q[2] + q[0] * q[3]) * drl2;

    m[3] = V::ZERO;
    m[7] = V::ZERO;
    m[11] = V::ZERO;
    m[12] = V::ZERO;
    m[13] = V::ZERO;
    m[14] = V::ZERO;
    m[15] = V::ONE;
}

//fp of_rotation
/// Find the quaternion of a Matrix3 assuming it is purely a rotation
#[must_use]
pub fn of_rotation<V: Float>(m: &[V; 9]) -> [V; 4] {
    eprintln!("quat::of_rotation - WARNING - THIS IS NOT NUMERICALLT STABLE");
    fn safe_sqrt<V: Float>(x: V) -> V {
        if x < V::ZERO { V::ZERO } else { x.sqrt() }
    }
    let half = f_const!(V, 1, 2);
    let r = safe_sqrt(V::ONE + m[0] + m[4] + m[8]) * half;
    let mut i = safe_sqrt(V::ONE + m[0] - m[4] - m[8]) * half;
    let mut j = safe_sqrt(V::ONE - m[0] + m[4] - m[8]) * half;
    let mut k = safe_sqrt(V::ONE - m[0] - m[4] + m[8]) * half;

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
#[must_use]
pub fn look_at<V: Float>(dirn: &[V; 3], up: &[V; 3]) -> [V; 4] {
    let d = vector::normalize(*dirn);
    let du = vector::dot(&d, up);
    let u = [up[0] - d[0] * du, up[1] - d[1] * du, up[2] - d[2] * du];
    let u = vector::normalize(u);
    mapping_vector_pair_to_vector_pair(
        (&d, &u),
        (&[V::ZERO, V::ZERO, -V::ONE], &[V::ZERO, V::ONE, V::ZERO]),
    )
}

//a Mapping functions
//cp invert
/// Get the quaternion inverse
#[must_use]
#[inline]
pub fn invert<V: Float>(a: &[V; 4]) -> [V; 4] {
    let l = vector::length_sq(a);
    let r_l = {
        if l < V::epsilon() {
            V::ZERO
        } else {
            V::ONE / l
        }
    };
    [-a[0] * r_l, -a[1] * r_l, -a[2] * r_l, a[3] * r_l]
}

//cp conjugate
/// Find the conjugate of a quaternion
#[must_use]
#[inline]
pub fn conjugate<V: Num>(a: &[V; 4]) -> [V; 4] {
    [-a[0], -a[1], -a[2], a[3]]
}

//cp normalize
/// Find the conjugate of a quaternion
#[must_use]
#[inline]
pub fn normalize<V: Float>(a: [V; 4]) -> [V; 4] {
    vector::normalize(a)
}

//cp rotate_x
/// Apply a rotation about the X-axis to this quaternion
///
/// Actually a(as quaternion) * [cos, sin, 0, 0, 0] (rijk)
///
/// Rotation about the X axis is c+i*s
///
/// `(c+i*s) * a[] = c*a - s*a[i] + i.s*a[r] - j.s*a[k] + k.s*a[j]`
#[must_use]
#[inline]
pub fn rotate_x<V: Float>(a: &[V; 4], angle: V) -> [V; 4] {
    let (s, c) = V::sin_cos(angle * f_const!(V, 1, 2));
    let i = a[0] * c + a[3] * s;
    let j = a[1] * c + a[2] * s;
    let k = a[2] * c - a[1] * s;
    let r = a[3] * c - a[0] * s;
    [i, j, k, r]
}

//cp rotate_y
/// Apply a rotation about the Y-axis to this quaternion
#[must_use]
#[inline]
pub fn rotate_y<V: Float>(a: &[V; 4], angle: V) -> [V; 4] {
    let (s, c) = V::sin_cos(angle * f_const!(V, 1, 2));
    let i = a[0] * c - a[2] * s;
    let j = a[1] * c + a[3] * s;
    let k = a[2] * c + a[0] * s;
    let r = a[3] * c - a[1] * s;
    [i, j, k, r]
}

//cp rotate_z
/// Apply a rotation about the Z-axis to this quaternion
#[must_use]
#[inline]
pub fn rotate_z<V: Float>(a: &[V; 4], angle: V) -> [V; 4] {
    let (s, c) = V::sin_cos(angle * f_const!(V, 1, 2));
    let i = a[0] * c + a[1] * s;
    let j = a[1] * c - a[0] * s;
    let k = a[2] * c + a[3] * s;
    let r = a[3] * c - a[2] * s;
    [i, j, k, r]
}

//cp multiply
/// Multiply two quaternions together
#[must_use]
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
#[must_use]
#[inline]
pub fn divide<V: Float>(a: &[V; 4], b: &[V; 4]) -> [V; 4] {
    let l2 = vector::length_sq(b);
    if l2 < V::epsilon() {
        [V::ZERO; 4]
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
#[must_use]
#[inline]
pub fn nlerp<V: Float>(t: V, in0: &[V; 4], in1: &[V; 4]) -> [V; 4] {
    normalize(vector::mix(in0, in1, t))
}

//a Operational functions
//fp distance_sq
/// Get a measure of the 'distance' between two unit quaternions.
///
/// This is a postive value less than or equal to 1.0
///
/// This uses the real component of A'. B; A'.B is the quaternion describing the
/// rotation by B followed by reversing the rotation by A. If they were equal
/// then the net rotation is zero - but as this is a quaternion, a zero rotation
/// can have a real value of +-1.
///
/// i.e. if A==B then the real component is 1, and if A==-B then the real component
/// is -1
#[inline]
pub fn distance_sq<V: Float>(a: &[V; 4], b: &[V; 4]) -> V {
    V::ONE - (a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]).abs()
}

//fp distance
/// Get a measure of the 'distance' between two quaternions
///
/// See [distance_sq] for more details
///
/// This is a postive value less than or equal to 1.0
#[inline]
pub fn distance<V: Float>(a: &[V; 4], b: &[V; 4]) -> V {
    distance_sq(a, b).sqrt()
}

//fp to_euler
/// Convert the quaternion to a bank, heading, altitude tuple - applied in that order
#[must_use]
pub fn to_euler<V: Float>(q: &[V; 4]) -> (V, V, V) {
    let zero = V::ZERO;
    let one = V::ONE;
    let two = one + one;
    let almost_half = f_const!(V, 499_999, 1_000_000);
    let halfpi = V::from_f32(std::f32::consts::FRAC_PI_2).unwrap();
    let i = q[0];
    let j = q[1];
    let k = q[2];
    let r = q[3];
    let test = i * j + r * k;
    let (heading, attitude, bank) = {
        if test > almost_half {
            (two * V::atan2(i, r), halfpi, zero)
        } else if test < -almost_half {
            (-two * V::atan2(i, r), -halfpi, zero)
        } else {
            let i2 = i * i;
            let j2 = j * j;
            let k2 = k * k;
            (
                V::atan2(two * j * r - two * i * k, V::ONE - two * j2 - two * k2),
                V::asin(two * test),
                V::atan2(two * i * r - two * j * k, V::ONE - two * i2 - two * k2),
            )
        }
    };
    (bank, heading, attitude)
}

//fp apply3
/// Apply the quaternion to a vector3
#[must_use]
pub fn apply3<V: Float>(q: &[V; 4], v: &[V; 3]) -> [V; 3] {
    let one = V::ONE;
    let two = one + one;
    let (r, i, j, k) = as_rijk(q);
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
#[must_use]
pub fn apply4<V: Float>(q: &[V; 4], v: &[V; 4]) -> [V; 4] {
    let one = V::ONE;
    let two = one + one;
    let (r, i, j, k) = as_rijk(q);
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

//fp weighted_average_pair
/// Calculate the weighted average of two unit quaternions
///
/// See <http://www.acsu.buffalo.edu/~johnc/ave_quat07.pdf>
/// Averaging Quaternions by F. Landis Markley
#[must_use]
pub fn weighted_average_pair<V: Float>(qa: &[V; 4], w_a: V, qb: &[V; 4], w_b: V) -> [V; 4] {
    let four = V::from_u8(4).unwrap();
    let (ra, ia, ja, ka) = as_rijk(qa);
    let (rb, ib, jb, kb) = as_rijk(qb);
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
/// weights need not add up to 1, but must be nonzero
///
/// This is an approximation compared to the Landis Markley paper
#[must_use]
pub fn weighted_average_many<I: Iterator<Item = (V, [V; 4])>, V: Float>(
    mut values_iter: I,
) -> [V; 4] {
    let (mut weight_ih, mut value_ih) = values_iter
        .next()
        .expect("weighted_average_many MUST be invoked with at least one quaternion");
    let mut next_values = Vec::new();
    loop {
        if let Some((second_weight, second_value)) = values_iter.next() {
            let w12 = weight_ih + second_weight;
            let av = weighted_average_pair(
                &value_ih,
                weight_ih / w12,
                &second_value,
                second_weight / w12,
            );
            next_values.push((w12, av));
        } else {
            // No value to pair with in-hand; if there are none on the
            // list then the iterator only had one entry, so return that
            if next_values.is_empty() {
                return value_ih;
            } else {
                // Iterator must have returned at least 2n+1 (n>=1) entries
                //
                // next_values must be 'n' long already, make it n+1 and break
                next_values.push((weight_ih, value_ih));
                break;
            }
        }
        // In-hand values have been consumed, fill them up or finish
        if let Some((w, v)) = values_iter.next() {
            // Iterator must have returned at least 3 values now
            weight_ih = w;
            value_ih = v;
        } else {
            // Iterator has returned 2n values with n>=1
            break;
        }
    }
    if next_values.len() == 1 {
        next_values[0].1
    } else {
        let values_iter = next_values.into_iter();
        weighted_average_many(values_iter)
    }
}

//fp get_rotation_of_vec_to_vec
/// Get a quaternion that is a rotation of one vector to another
///
/// The vectors must be unit vectors
#[must_use]
pub fn rotation_of_vec_to_vec<V: Float>(a: &[V; 3], b: &[V; 3]) -> [V; 4] {
    let pi = V::from_f32(std::f32::consts::PI).unwrap();
    let obtuse = vector::dot(a, b) < V::ZERO;
    let mut cp = vector::cross_product3(a, b);
    let sa = vector::length(&cp);
    // If this is a rotation by 0 or 180 degrees... we need a different axis...
    if sa < V::epsilon() {
        // If the X coordinate of 'a' is zero then (1,0,0) is perpendicular to it, so use that
        //
        // else a is (x,y,z) (x!=0) and (x,y,z) x (0,1,1) wil be a nonzero vector
        if a[0].abs() < V::epsilon() {
            cp = [V::ONE, V::ZERO, V::ZERO];
        } else {
            cp = vector::cross_product3(a, &[V::ZERO, V::ONE, V::ONE]);
        }
    }
    let angle = sa.asin();
    let angle = if obtuse { pi - angle } else { angle };
    of_axis_angle(&cp, angle)
}

/// Create a quaternion for a rotation that maps di_m to di_c, and
/// dj_m to dj_c. All must be unit vectors.
///
/// This is used to map between two views of the same 3D space, so the angle
/// between di_m and dj_m should be the same as that between di_c and dj_c.
///
/// The resultant quaternion will map dj_m to a unit vector in the same plane as
/// di_c and dj_c, with the angle between di_c and it equal to the angle between
/// di_m and dj_m. This will *equal* dj_c if the angle between di_m and dj_m
/// equals the angle between di_c and dj_c.
///
/// This will generate two quaternions (Qm2z and Qz2c) that map di_m
/// to the Z axis, and the Z axis to di_c. Applying both of these
/// quaternions in succession maps di_m to di_c.
///
/// It maps dj_m to dj_m' and dj_c *backwards* to dj_c', where the
/// difference between v1' and w1' should be *just* a rotation
/// around the Z axis. Creating a rotation that implements the
/// rotation around the Z axis provides a third quaternion (Qrotz).
///
/// The three quaternions can be applied Qz2c * Qrotz * Qm2z to map di_m to di_c
/// (for sure); if the angle eqaulity above is met then dj_m will map to dj_c too.
#[must_use]
pub fn mapping_vector_pair_to_vector_pair<F: Float>(
    (di_m, dj_m): (&[F; 3], &[F; 3]),
    (di_c, dj_c): (&[F; 3], &[F; 3]),
) -> [F; 4] {
    let z_axis: [F; 3] = [F::zero(), F::zero(), F::one()];
    let qi_c = rotation_of_vec_to_vec(di_c, &z_axis);
    let qi_m = rotation_of_vec_to_vec(di_m, &z_axis);

    let dj_c_rotated = apply3(&qi_c, dj_c);
    let dj_m_rotated = apply3(&qi_m, dj_m);

    let theta_dj_m = dj_m_rotated[0].atan2(dj_m_rotated[1]);
    let theta_dj_c = dj_c_rotated[0].atan2(dj_c_rotated[1]);

    let theta = theta_dj_m - theta_dj_c;
    let theta_div_2 = theta / (F::one() + F::one());
    let cos_2theta = theta_div_2.cos();
    let sin_2theta = theta_div_2.sin();

    let q_z = of_rijk(cos_2theta, F::zero(), F::zero(), sin_2theta);
    multiply(&conjugate(&qi_c), &multiply(&q_z, &qi_m))
}
