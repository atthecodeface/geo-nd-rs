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

@file    matrix_op.rs
@brief   Square matrix operations - part of geometry library
 */

//a Imports
use crate::{Num, Float};
use super::matrixr_op;

//fp identity
/// Create an identity square matrix of a given dimension
pub fn identity<V:Num,const D2:usize,const D:usize> () -> [V;D2] {
    let mut r = [V::zero(); D2];
    for i in 0..D {
        r[i*(D+1)] = V::one();
    }
    r
}

//fp identity2
/// Create a 2-by-2 identity matrix
pub fn identity2<V:Num>() -> [V;4] {
    identity::<V,4,2>()
}

//fp identity3
/// Create a 3-by-3 identity matrix
pub fn identity3<V:Num>() -> [V;9] {
    identity::<V,9,3>()
}

//fp identity4
/// Create a 4-by-4 identity matrix
pub fn identity4<V:Num>() -> [V;16] {
    identity::<V,16,4>()
}

//fp determinant2
/// Find the determinant of a 2-by-2 matrix
pub fn determinant2<V:Num> (m:&[V;4]) -> V {
    m[0] * m[3] - m[1] * m[2]
}

//fp inverse2
/// Find the inverse of a 2-by-2 matrix
///
/// # Example
///
/// ```
/// use geometry::vector::{length, sub};
/// use geometry::matrix::{identity2, inverse2, multiply2, MatrixType};
/// let i = identity2();
/// assert!( length(&sub(inverse2(&i), &i, 1.)) < 1E-8 );
/// for a in &[ [1.,0., 0.,1.],
///             [1.,0., 1.,1.],
///             [1.,2., 7.,2.] ] {
///     let a_inv = inverse2(&a);
///     assert!( length(&sub(multiply2(&a_inv,&a), &i, 1.)) < 1E-6 );
/// }
/// ```
///
pub fn inverse2<V:Float> (m:&[V;4]) -> [V;4] {
    let d = determinant2(m);
    let r_d = {
        if V::abs(d) > V::epsilon() {
            V::one() / d
        } else {
            V::zero()
        }
    };

    [ m[3]*r_d, -m[1]*r_d, -m[2]*r_d, m[0]*r_d ]
}

//fp determinant3
/// Find the determinant of a 3-by-3 matrix
pub fn determinant3<V:Num> (m:&[V;9]) -> V {
    m[0]*(m[4]*m[8] - m[5]*m[7]) +
        m[1]*(m[5]*m[6] - m[3]*m[8]) +
        m[2]*(m[3]*m[7] - m[4]*m[6])
}

//fp inverse3
/// Find the inverse of a 3-by-3 matrix
///
/// # Example
///
/// ```
/// use geometry::vector::{length, sub};
/// use geometry::matrix::{identity3, inverse3, multiply3, MatrixType};
/// let i = identity3();
/// assert!( length(&sub(inverse3(&i), &i, 1.)) < 1E-8 );
/// for a in &[ [1.0,0.,0., 0.,1.,0.,  0.,0.,1.],
///             [1.,0.,0., 0.,1.,1.,  0.,0.,1.],
///             [1.,3.,2., 0.,2.,3., -1.,2.,3.] ] {
///     let a_inv = inverse3(&a);
///     assert!( length(&sub(multiply3(&a_inv,&a), &i, 1.)) < 1E-6 );
/// }
/// ```
///
pub fn inverse3<V:Float> (m:&[V;9]) -> [V;9] {
    let mut r = [V::zero(); 9];
    let d = determinant3(m);
    let r_d = {
        if V::abs(d) > V::epsilon() {
            V::one() / d
        } else {
            V::zero()
        }
    };

    r[0] = (m[3+1]*m[6+2] - m[3+2]*m[6+1])*r_d;
    r[3] = (m[3+2]*m[6+0] - m[3+0]*m[6+2])*r_d;
    r[6] = (m[3+0]*m[6+1] - m[3+1]*m[6+0])*r_d;

    r[1] = (m[6+1]*m[0+2] - m[6+2]*m[0+1])*r_d;
    r[4] = (m[6+2]*m[0+0] - m[6+0]*m[0+2])*r_d;
    r[7] = (m[6+0]*m[0+1] - m[6+1]*m[0+0])*r_d;

    r[2] = (m[0+1]*m[3+2] - m[0+2]*m[3+1])*r_d;
    r[5] = (m[0+2]*m[3+0] - m[0+0]*m[3+2])*r_d;
    r[8] = (m[0+0]*m[3+1] - m[0+1]*m[3+0])*r_d;
    r
}

//fp from_quat3
/// Create a rotation 3-by-3 matrix from a quaternion
pub fn from_quat3<V:Float>(q:[V;4]) -> [V;9] {
    let mut r = [V::zero();9];
    let x = q[0];
    let y = q[1];
    let z = q[2];
    let w = q[3];
    let one = V::one();
    let two = V::from(2).unwrap();

    r[0+0]= one - two*y*y - two*z*z;
    r[0+1]=     two*x*y + two*z*w;
    r[0+2]=     two*x*z - two*y*w;
    r[3+1]= one - two*z*z - two*x*x;
    r[3+2]=     two*y*z + two*x*w;
    r[3+0]=     two*x*y - two*z*w;
    r[6+2]= one - two*x*x - two*y*y;
    r[6+0]=     two*z*x + two*y*w;
    r[6+1]=     two*y*z - two*x*w;
    r
}

//fp determinant4
/// Find the determinant of a 4-by-4 matrix
pub fn determinant4<V:Num> (m:&[V;16]) -> V {
    m[0] * (  m[4+1] * (m[8+2]*m[12+3]-m[8+3]*m[12+2]) +
            ( m[4+2] * (m[8+3]*m[12+1]-m[8+1]*m[12+3])) +
            ( m[4+3] * (m[8+1]*m[12+2]-m[8+2]*m[12+1])) ) -
    m[1] * (  m[4+2] * (m[8+3]*m[12+0]-m[8+0]*m[12+3]) +
            ( m[4+3] * (m[8+0]*m[12+2]-m[8+2]*m[12+0])) +
            ( m[4+0] * (m[8+2]*m[12+3]-m[8+3]*m[12+2])) ) +
    m[2] * (  m[4+3] * (m[8+0]*m[12+1]-m[8+1]*m[12+0]) +
            ( m[4+0] * (m[8+1]*m[12+3]-m[8+3]*m[12+1])) +
            ( m[4+1] * (m[8+3]*m[12+0]-m[8+0]*m[12+3])) ) -
    m[3] * (  m[4+0] * (m[8+1]*m[12+2]-m[8+2]*m[12+1]) +
            ( m[4+1] * (m[8+2]*m[12+0]-m[8+0]*m[12+2])) +
            ( m[4+2] * (m[8+0]*m[12+1]-m[8+1]*m[12+0])) )

}

//fp inverse4
/// Find the inverse of a 4-by-4 matrix
///
/// # Example
///
/// ```
/// use geometry::vector::{length, sub};
/// use geometry::matrix::{identity4, inverse4, multiply4, MatrixType};
/// let i = identity4();
/// assert!( length(&sub(inverse4(&i), &i, 1.)) < 1E-8 );
/// for a in &[ [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,0.,1., 0.,0.,1.,0.],
///             [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,1., 0.,0.,1.,0.],
///             [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,1.,1.],
///             [1.,3.,2.,1., 0.,2.,3.,3., -1.,2.,3.,2., 0.,0.,2.,1.] ] {
///     let a_inv = inverse4(&a);
///     assert!( length(&sub(multiply4(&a_inv,&a), &i, 1.)) < 1E-6 );
/// }
/// ```
///
pub fn inverse4<V:Float> (m:&[V;16]) -> [V;16] {
    let d = determinant4(m);
    let mut r = [V::zero(); 16];
    if V::abs(d) > V::epsilon() {
        let r_d = V::one() / d;

        for j in 0..4 {
            let a = ((j+1) & 3) * 4;
            let b = ((j+2) & 3) * 4;
            let c = ((j+3) & 3) * 4;
            for i in 0..4 {
                let x = (i+1) & 3;
                let y = (i+2) & 3;
                let z = (i+3) & 3;
                let sc = if (i+j)&1 == 0 {V::one()} else {-V::one()};
                r[i*4+j] = ( ( m[a+x]*m[b+y]-m[b+x]*m[a+y]) * m[c+z] +
                             ( m[a+y]*m[b+z]-m[b+y]*m[a+z]) * m[c+x] +
                             ( m[a+z]*m[b+x]-m[b+z]*m[a+x]) * m[c+y] ) *  sc * r_d;
            }
        }
    }
    r
}

//fp multiply2
/// Multiply two square 2x2 matrices and produce a result
pub fn multiply2<V:Float>(a:&[V;2*2], b:&[V;2*2]) -> [V;2*2] {
    matrixr_op::multiply::<V,4,4,4,2,2,2>( a, b )
}

//fp transform_vec2
/// Multiply a vec2 by a 2x2 matrix
pub fn transform_vec2<V:Float>(m:&[V;4], v:&[V;2]) -> [V;2] {
    matrixr_op::transform_vec::<V,4,2,2>( m, v )
}

//fp multiply3
/// Multiply two square 3x3 matrices and produce a result
pub fn multiply3<V:Float>(a:&[V;9], b:&[V;9]) -> [V;9] {
    matrixr_op::multiply::<V,9,9,9,3,3,3>( a, b )
}

//fp transform_vec3
/// Multiply a vec3 by a 3x3 matrix
pub fn transform_vec3<V:Float>(m:&[V;9], v:&[V;3]) -> [V;3] {
    matrixr_op::transform_vec::<V,9,3,3>( m, v )
}

//fp multiply4
/// Multiply two square 4x4 matrices and produce a result
pub fn multiply4<V:Float>(a:&[V;16], b:&[V;16]) -> [V;16] {
    matrixr_op::multiply::<V,16,16,16,4,4,4>( a, b )
}

//fp transform_vec4
/// Multiply a vec4 by a 4x4 matrix
pub fn transform_vec4<V:Float>(m:&[V;16], v:&[V;4]) -> [V;4] {
    matrixr_op::transform_vec::<V,16,4,4>( m, v )
}

//fp translate4
/// Translate a 4-by-4 matrix by a vector - standard graphics approach
///
/// Same as postmultiply by [1 0 0 v0], [0 1 0 v1], [0 0 1 v2], [0 0 0 1]
pub fn translate4<V:Num> (m:&[V;16], v:&[V;4]) -> [V;16] {
    let mut r = m.clone();
    r[12] = m[ 0]*v[0] + m[4+0]*v[1] + m[8+0]*v[2] + m[12+0];
    r[13] = m[ 1]*v[0] + m[4+1]*v[1] + m[8+1]*v[2] + m[12+1];
    r[14] = m[ 2]*v[0] + m[4+2]*v[1] + m[8+2]*v[2] + m[12+2];
    r[15] = m[ 3]*v[0] + m[4+3]*v[1] + m[8+3]*v[2] + m[12+3];
    r
}

//fp perspective4
/// Create a perspective graphics matrix
pub fn perspective4<V:Float> (fov:V, aspect:V, near:V, far:V) -> [V;16] {
    let mut r = [V::zero(); 16];
    let two = V::from(2).unwrap();
    let f = V::one() / V::tan(fov / two );
    r[0] = f / aspect;
    r[5] = f;
    r[11] = -V::one();
    let nf = V::one() / (near - far);
    r[10] = (far + near) * nf;
    r[14] = two * far * near * nf;
    r
}

//fp from_quat4
/// Create a rotation 4-by-4 matrix from a quaternion
pub fn from_quat4<V:Float>(q:[V;4]) -> [V;16] {
    let mut r = [V::zero();16];
    let x = q[0];
    let y = q[1];
    let z = q[2];
    let w = q[3];
    let one = V::one();
    let two = V::from(2).unwrap();

    r[0+0]= one - two*y*y - two*z*z;
    r[0+1]=     two*x*y + two*z*w;
    r[0+2]=     two*x*z - two*y*w;
    // r[0+3]= 0;
    r[4+1]= one - two*z*z - two*x*x;
    r[4+2]=     two*y*z + two*x*w;
    r[4+0]=     two*x*y - two*z*w;
    // r[4+3]= 0;
    r[8+2]= one - two*x*x - two*y*y;
    r[8+0]=     two*z*x + two*y*w;
    r[8+1]=     two*y*z - two*x*w;
    // r[8+3]= 0;
    // r[12]=0; r[13]=0; a[14]=0;
    r[15] = V::one();
    r
}

/*
    #f invert
    @staticmethod
    def invert(a:Mat3,x:Mat3) -> Mat3:
        x00 = x[ 0]; x01 = x[ 1]; x02 = x[ 2]; x03 = x[ 3];
        x10 = x[ 4]; x11 = x[ 5]; x12 = x[ 6]; x13 = x[ 7];
        x20 = x[ 8]; x21 = x[ 9]; x22 = x[10]; x23 = x[11];
        x30 = x[12]; x31 = x[13]; x32 = x[14]; x33 = x[15];
        b00 = x00 * x11 - x01 * x10;
        b01 = x00 * x12 - x02 * x10;
        b02 = x00 * x13 - x03 * x10;
        b03 = x01 * x12 - x02 * x11;
        b04 = x01 * x13 - x03 * x11;
        b05 = x02 * x13 - x03 * x12;
        b06 = x20 * x31 - x21 * x30;
        b07 = x20 * x32 - x22 * x30;
        b08 = x20 * x33 - x23 * x30;
        b09 = x21 * x32 - x22 * x31;
        b10 = x21 * x33 - x23 * x31;
        b11 = x22 * x33 - x23 * x32;
        d = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
        if (abs(d)>1E-8): d=1/d;
        a[0]  = (x11 * b11 - x12 * b10 + x13 * b09) * d;
        a[1]  = (x02 * b10 - x01 * b11 - x03 * b09) * d;
        a[2]  = (x31 * b05 - x32 * b04 + x33 * b03) * d;
        a[3]  = (x22 * b04 - x21 * b05 - x23 * b03) * d;
        a[4]  = (x12 * b08 - x10 * b11 - x13 * b07) * d;
        a[5]  = (x00 * b11 - x02 * b08 + x03 * b07) * d;
        a[6]  = (x32 * b02 - x30 * b05 - x33 * b01) * d;
        a[7]  = (x20 * b05 - x22 * b02 + x23 * b01) * d;
        a[8]  = (x10 * b10 - x11 * b08 + x13 * b06) * d;
        a[9]  = (x01 * b08 - x00 * b10 - x03 * b06) * d;
        a[10] = (x30 * b04 - x31 * b02 + x33 * b00) * d;
        a[11] = (x21 * b02 - x20 * b04 - x23 * b00) * d;
        a[12] = (x11 * b07 - x10 * b09 - x12 * b06) * d;
        a[13] = (x00 * b09 - x01 * b07 + x02 * b06) * d;
        a[14] = (x31 * b01 - x30 * b03 - x32 * b00) * d;
        a[15] = (x20 * b03 - x21 * b01 + x22 * b00) * d;
        return a;
    #f getRotation
    # from www.euclideanspace
    @staticmethod
    def getRotation(q:Quat, m:Mat4) -> quat:
        lr0 = 1.0/math.hypot(m[0+0], m[4+0], m[8+0]);
        lr1 = 1.0/math.hypot(m[0+1], m[4+1], m[8+1]);
        lr2 = 1.0/math.hypot(m[0+2], m[4+2], m[8+2]);
        m00 = m[0]*lr0; m10=m[1]*lr1; m20=m[ 2]*lr2;
        m01 = m[4]*lr0; m11=m[5]*lr1; m21=m[ 6]*lr2;
        m02 = m[8]*lr0; m12=m[9]*lr1; m22=m[10]*lr2;
        tr = m00 + m11 + m22;
        if (tr > 0) :
            S = math.sqrt(tr+1.0) * 2; # S=4*qw
            w = 0.25 * S;
            x = (m21 - m12) / S;
            y = (m02 - m20) / S;
            z = (m10 - m01) / S;
            pass
        elif ((m00 > m11) and (m00 > m22)):
            S = math.sqrt(1.0 + m00 - m11 - m22) * 2; # S=4*qx
            w = (m21 - m12) / S;
            x = 0.25 * S;
            y = (m01 + m10) / S;
            z = (m02 + m20) / S;
            pass
        elif (m11 > m22):
            S = math.sqrt(1.0 + m11 - m00 - m22) * 2; # S=4*qy
            w = (m02 - m20) / S;
            x = (m01 + m10) / S;
            y = 0.25 * S;
            z = (m12 + m21) / S;
            pass
        else:
            S = math.sqrt(1.0 + m22 - m00 - m11) * 2; # S=4*qz
            w = (m10 - m01) / S;
            x = (m02 + m20) / S;
            y = (m12 + m21) / S;
            z = 0.25 * S;
            pass
        q[0]=x; q[1]=y; q[2]=z; q[3]=w;
        return q;
    #f scale
    def scale(a:Mat4, x:Mat4, s:Sequence[float]) -> Mat4:
        for i in range(4):
            cs=1.
            if i<len(s): cs=s[i]
            for j in range(4):
                a[4*i+j] = x[4*i+j]*cs;
                pass
            pass
        return a;
    #f All done
    pass

 */
