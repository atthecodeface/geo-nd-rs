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

@file    vector_op.rs
@brief   Part of geometry library
 */

//a Imports
use crate::{Num, Float};
use super::matrix_op as matrix;

//a Vector constructors
//fp zero
/// Create a zero vector of the correct size
///
/// # Example
///
/// ```
/// use geometry::vector;
/// let a = vector::zero::<f32, 4>();
/// assert_eq!( a, [0., 0., 0., 0.]);
/// ```
///
pub fn zero<V:Num,const D:usize> () -> [V; D] { [V::zero();D] }

//mp set_zero
/// Set the vector in-place to be zero
///
/// # Example
///
/// ```
/// use geometry::vector;
/// let mut a = [1., 2., 3.];
/// vector::set_zero(&mut a);
/// assert_eq!( a, [0., 0., 0.]);
/// ```
///
pub fn set_zero<V:Num> (v:&mut [V]) {
    for c in v.iter_mut() { c.set_zero(); }
}

//fp is_zero
/// Return true if the vector is all zeros
///
/// # Example
///
/// ```
/// use geometry::vector;
/// let mut a = [1., 2., 3.];
/// vector::set_zero(&mut a);
/// assert!( vector::is_zero(&a) );
/// ```
///
pub fn is_zero<V:Num> (v:&[V]) -> bool {
    for c in v { if !c.is_zero() {return false;}}
    true
}

//fp cross_product3
/// Return the outer product (cross product) of two 3-dimensional vectors
///
/// The outer product of two 3D vectors A and B is perpendicular to both A and B.
///
/// # Examples
///
/// ```
/// use geometry::vector;
/// let a = [3., 4., 5.];
/// let b = [2., 17., 1.];
/// let x = vector::cross_product3(&a, &b);
/// assert!( vector::dot( &a, &x ) < 1E-8 );
/// assert!( vector::dot( &b, &x ) < 1E-8 );
///
/// let x = [1., 0., 0.];
/// let y = [0., 1., 0.];
/// let z = vector::cross_product3(&x, &y);
/// assert_eq!( z, [0., 0., 1.] );
/// ```
pub fn cross_product3<V:Num> (x:&[V;3], y:&[V;3]) -> [V;3] {
    let c0 = x[1] * y[2] - x[2] * y[1];
    let c1 = x[2] * y[0] - x[0] * y[2];
    let c2 = x[0] * y[1] - x[1] * y[0];
    [c0, c1, c2]
}

//fp mix
/// Find the linear interpolation between two vectors by a parameter `t`.
///
/// # Example
///
/// ```
/// use geometry::vector;
/// let a = [3., 1.];
/// let b = [2., 3.];
/// assert_eq!( vector::mix( &a, &b, 0.),  [3., 1.]);
/// assert_eq!( vector::mix( &a, &b, 1.),  [2., 3.]);
/// assert_eq!( vector::mix( &a, &b, 0.5), [2.5, 2.]);
/// ```
///
pub fn mix<V:Float,const D:usize> (a:&[V;D], b:&[V;D], t:V) -> [V;D] {
    let mut v = zero();
    let omt = V::one() - t;
    for i in 0..D {
        v[i] = a[i] * omt + b[i] * t;
    }
    v
}

//fp axis_of_rotation3
/// Find the axis of rotation of a Matrix3
///
/// A rotation matrix R has the property that R.v = k.v for any vector
/// v along the axis of rotation; this is what defines it to be the
/// axis of rotation. This is to say that v is an Eigenvector of
/// R. For pure rotations the value of k is 1; it must be a real
/// number in any case.
///
/// Hence to find the axis of rotation of a Matrix3 R requires finding
/// the Eigenvector of R that has a real Eigenvalue - there will
/// always be at least one (more than one implies a pure scaling
/// matrix).
///
/// The algorithm used is iterative; it requires the angle of rotation
/// to be non-tiny (i.e. > 1/1000 radians, or about 1/16 of a
/// degree)
///
/// Note that R . axis = axis; also we will consider p being any vector perpendicular to the axis.
///
/// Consider the matrix RI =  R - 99999/100000 * I (where I is the Identity)
///
/// Then RI . axis = R.axis - 99999/100000*I.axis = axis * 1/100000
///
/// And RI . p  = R.p - 99999/100000*I.p, and note then that
/// if the rotation is > 1/1000 radians then |RI.p| > 1/1000
///
/// Inverting RI yields RI_inv. Consider (RI * RI_inv) * axis:
///
///   axis = RI * RI_inv * axis = RI * (RI_inv * axis)
///
/// Since RI * axis = axis / 100000, RI_inv * axis must be 100000*axis
///
/// Consider any vector p perpendicular to the axis, and then (RI * RI_inv) * p:
///
///   p = RI * (RI_inv * p); this tells us that |RI_inv * p| < 1000
///
/// Hence the off-axis component is scaled by less than 1000, where
/// the axis component is scaled by 100000; normalizing will provide a
/// new vector that has the off-axis component reduced by at least a
/// factor of 100.
///
/// This provides us with an iterative approach:
///
/// ```text
///  let x = any vector p + k*axis
///  let y1 = RI_inv * x / 100000
///  // y1 is now k*axis + (p'/100) for some p' such that |p'|=|p|
///  let y2 = RI_inv * y1 / 100000
///  // y2 is now k*axis + (p''/10000) for some p'' such that |p''|=|p|
///  let y3 = RI_inv * y2 / 100000
///  // yn will approach the axis of rotation as n increases
/// ```
/// etc.
///
/// The rate of convergence depends on the angle of rotation; for a
/// rotation of > 1/10 radians the off-axis amount will reduce by
/// around 2^12 each iteration, but for 1/1000 radians it will be only
/// about 2^6 per iteration. For a 52-bit mantissa ten iterations
/// suffices in all cases.
///
/// A starting guess for the axis can be (1,0,0); if the actual axis
/// of rotation is the Y-axis or the Z-axis then the algorithm does
/// not converge; and a second guess of (0,1,0) can be used, or even
/// (0,0,1) if that does not converge.
///
/// If the rotation is very close to 0 then any axis will do :-)
///
/// Another option for this is to consider any vector x and R.x; the
/// cross-product of the two should be the axis of rotation. This
/// clearly has problems if the rotation is by 180 degrees, and is
/// unstable for small rotations (as is any determination of the
/// axis).
///
/// Another approach is to just consider it as a rotation matrix
///
/// Unit axis vector = (x,y,z); c=cos(angle), t=1-c, s=sin(angle)
///
/// Matrix:
///
///  t.x^2 + c       t.xy - s.z      t.xz + s.y
///  t.yx + s.z      t.y^2 + c       t.yz - s.x
///  t.zx - s.y      t.zy + s.x      t.z^2 + c
///
/// Adding the digaonal yields 3c + t = 2+2c
/// Then subtracting across the diagonal yields 2sx, 2sy, 2sz
///
/// However, if 2+2c is close to 0 then this fails
pub fn axis_of_rotation3<V:Float>(rotation:&[V;9]) -> [V;3] {
    let mut rot_min_id = rotation.clone();
    let almost_one  = V::from(99999).unwrap() / V::from(100000).unwrap();
    let almost_zero = V::one() / V::from(100000).unwrap();
    rot_min_id[0] = rot_min_id[8] - almost_one;
    rot_min_id[4] = rot_min_id[8] - almost_one;
    rot_min_id[8] = rot_min_id[8] - almost_one;
    let rot_min_id_i = matrix::inverse3(&rot_min_id);
    for j in 0..3 {
        let mut v = [V::zero(); 3];
        v[j] = V::one();
        let mut last_v = [V::zero(); 3];
        for _ in 0..10 {
            last_v = v;
            v = normalize(matrix::transform_vec3( &rot_min_id_i, &v ));
        }
        if distance_sq(&v, &last_v) < almost_zero { return v; }
    }
    [V::zero(); 3]
}

//a Combinations
//cp scale
/// Scale ever element of a vector by a single scaling factor
//
/// # Example
///
/// ```
/// use geometry::vector;
/// let a = vector::scale([1., 2., 3.], 2.);
/// assert_eq!( a, [2., 4., 6.]);
/// ```
///
pub fn scale<V:Num,const D:usize> (mut v:[V;D], s:V) -> [V;D] {
    for c in &mut v { *c = (*c) * s; }
    v
}

//cp comp_mult
/// Consume the vector and return a new vector that is the original
/// scaled in each coordinate a different scale factor
//
/// # Example
///
/// ```
/// use geometry::vector;
/// let a = vector::comp_mult([2., 4., 6.], &[1., 0., 2.]);
/// assert_eq!( a, [2., 0., 12.]);
/// ```
///
pub fn comp_mult<V:Num,const D:usize> (mut v:[V;D], s:&[V;D]) -> [V;D] {
    for i in 0..D { v[i] = v[i] * s[i]; }
    v
}

//cp reduce
/// Reduce ever element of a vector by a single scaling factor
//
/// # Example
///
/// ```
/// use geometry::vector;
/// let a = vector::reduce([2., 4., 6.], 2.);
/// assert_eq!( a, [1., 2., 3.]);
/// ```
///
pub fn reduce<V:Num,const D:usize> (mut v:[V;D], s:V) -> [V;D] {
    for c in &mut v { *c = (*c) / s; }
    v
}

//cp add
/// Add another vector scaled by a value to a vector, returning the sum
///
/// # Example
///
/// ```
/// use geometry::vector;
/// let a = [3., 1.];
/// let b = [2., 3.];
/// assert_eq!( vector::add( a, &b, 3.), [9., 10.]);
/// ```
///
pub fn add<V:Num,const D:usize> (mut v:[V;D], other:&[V;D], scale:V) -> [V;D] {
    for i in 0..D {
        v[i] = v[i] + other[i] * scale;
    }
    v
}

//cp sub
/// Consume the vector, and return a new vector that is the sum of
/// this and a borrowed other vector scaled
pub fn sub<V:Num,const D:usize> (mut v:[V;D], other:&[V;D], scale:V) -> [V;D] {
    for i in 0..D {
        v[i] = v[i] - other[i] * scale;
    }
    v
}

//cp clamp
/// Clamp each element to be between min and max
///
/// # Example
///
/// ```
/// use geometry::vector;
/// let a = [-1., 3.];
/// assert_eq!( vector::clamp( a, 0., 1.), [0., 1.]);
/// assert_eq!( vector::clamp( a, -1., 1.), [-1., 1.]);
/// assert_eq!( vector::clamp( a, -10., 10.), [-1., 3.]);
/// ```
///
pub fn clamp<V:Float,const D:usize> (mut a:[V;D], min:V, max:V) -> [V;D] {
    for i in 0..D {
        a[i] = if a[i] < min {min} else if a[i] > max {max} else {a[i]};
    }
    a
}

//cp normalize
/// Normalize (make unit length) a vector if possible
///
/// If its length is too close to 0, then return the zero vector
///
/// # Example
///
/// ```
/// use geometry::vector;
/// assert_eq!( vector::normalize([1.,-1.]), [1./(2.0f64).sqrt(),-1./(2.0f64).sqrt()] );
/// assert_eq!( vector::normalize([0.,0.]), [0.,0.] );
/// ```
///
pub fn normalize<V:Float,const D:usize> (mut v:[V;D]) -> [V;D] {
    let l = length(&v);
    if l < V::epsilon() {
        set_zero(&mut v);
    } else {
        v = reduce(v, l);
    }
    v
}

//cp rotate_around
/// Rotate a vector within a plane around a
/// *pivot* point by the specified angle
///
/// The plane of rotation is specified by providing two vector indices for the elements to adjust. For a 2D rotation then the values of c0 and c1 should be 0 and 1.
///
/// For a 3D rotation about the Z axis, they should be 0 and 1; for
/// rotation about the Y axis they should be 2 and 0; and for rotation
/// about the X axis they should be 1 and 2.
///
/// # Examples
///
/// ```
/// use geometry::vector;
/// let a = [3., 4., 5.];
/// let pivot = [3., 3., 0.];
/// // Rotate by 90 degress anticlockwise about the Z-axis pivoting on (3,3,0)
/// assert!( vector::distance_sq( &vector::rotate_around(a, &pivot, (90.0_f32).to_radians(), 0, 1 ), &[2., 3., 5.] ) < 1E-8 );
/// // Rotate by 90 degress anticlockwise about the X-axis pivoting on (3,3,0)
/// assert!( vector::distance_sq( &vector::rotate_around(a, &pivot, (90.0_f32).to_radians(), 1, 2 ), &[3., -2., 1.] ) < 1E-8 );
/// ```
///
pub fn rotate_around<V:Float,const D:usize> (mut v:[V;D], pivot:&[V;D], angle:V, c0:usize, c1:usize) -> [V;D] {
    let (s,c) = angle.sin_cos();
    let dx = v[c0] - pivot[c0];
    let dy = v[c1] - pivot[c1];
    let x1 = c*dx - s*dy;
    let y1 = c*dy + s*dx;
    v[c0] = x1 + pivot[c0];
    v[c1] = y1 + pivot[c1];
    v
}

//a Accessors or mutations
//mp length_sq
/// Return the length^2 of the vector
///
/// # Example
///
/// ```
/// use geometry::vector;
/// assert_eq!( vector::length_sq(&[3., 4.]), 25. );
/// ```
///
pub fn length_sq<V:Num> (v:&[V]) -> V {
    let mut r = V::zero();
    for c in v.iter() { r = r + (*c) * (*c) }
    r
}

//mp length
/// Return the length of the vector
///
/// # Example
///
/// ```
/// use geometry::vector;
/// assert_eq!( vector::length(&[3., 4.]), 5. );
/// ```
///
pub fn length<V:Float> (v:&[V]) -> V {
    length_sq(v).sqrt()
}

//mp distance_sq
/// Return the distance square between two vectors
///
/// # Example
///
/// ```
/// use geometry::vector;
/// assert_eq!( vector::distance_sq(&[1.,-1.], &[4., 3.]), 25. );
/// ```
///
pub fn distance_sq<V:Num,const D:usize> (v:&[V;D], other:&[V;D]) -> V {
    let mut r = V::zero();
    for i in 0..D {
        let d = v[i] - other[i];
        r = r + d * d;
    }
    r
}

//mp distance
/// Return the distance between two vectors
///
/// # Example
///
/// ```
/// use geometry::vector;
/// assert_eq!( vector::distance(&[1.,-1.], &[4., 3.]), 5. );
/// ```
///
pub fn distance<V:Float,const D:usize> (v:&[V;D], other:&[V;D]) -> V {
    distance_sq(v,other).sqrt()
}

//mp dot
/// Return the inner product (aka dot product or scalar product) of this and another vector
///
/// # Example
///
/// ```
/// use geometry::vector;
/// assert_eq!( vector::dot(&[1.,-1.], &[4., 1.]), 3. );
/// ```
///
pub fn dot<V:Num,const D:usize> (v:&[V;D], other:&[V;D]) -> V {
    let mut r = V::zero();
    for i in 0..D {
        r = r + v[i]*other[i];
    }
    r
}

//a Formatting
//mp fmt - format a `Vector` for display
/// Format the vector for display
///
/// # Examples
///
/// ```
/// use geometry::vector;
/// struct Pt { c : [f32;2] };
/// impl std::fmt::Display for Pt {
///   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { vector::fmt(f, &self.c) }
/// }
/// assert_eq!( format!("{}", &Pt{c:[0., 1.]} ), "(0,1)" );
/// ```
pub fn fmt<V:Num>(f: &mut std::fmt::Formatter, v : &[V]) -> std::fmt::Result {
    for i in 0..v.len() {
        if i==0 {
            write!(f, "({}", v[i])?;
        } else {
            write!(f, ",{}", v[i])?;
        }
    }
    write!(f, ")")
}

/*
    #f transformMat3
    @staticmethod
    def transformMat4(a:Vec4,x:Vec4,M:Mat4) -> Vec4:
        c0=M[0]*x[0] + M[4]*x[1] + M[8]*x[2]  + M[12]*x[3];
        c1=M[1]*x[0] + M[5]*x[1] + M[9]*x[2]  + M[13]*x[3];
        c2=M[2]*x[0] + M[6]*x[1] + M[10]*x[2] + M[14]*x[3];
        c3=M[3]*x[0] + M[7]*x[1] + M[11]*x[2] + M[15]*x[3];
        a[0]=c0; a[1]=c1; a[2]=c2; a[3]=c3;
        return a
 */


