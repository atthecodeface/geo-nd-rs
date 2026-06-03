//a Imports
use geo_nd::matrix;

fn assert_identity(n: usize, m: &[f32]) {
    for i in 0..n * n {
        if i % (n + 1) == 0 {
            assert_eq!(m[i], 1.0);
        } else {
            assert_eq!(m[i], 0.0);
        }
    }
}

fn assert_near_identity(n: usize, m: &[f32]) {
    for i in 0..n * n {
        if i % (n + 1) == 0 {
            assert!((m[i] - 1.0).abs() < 1E-4);
        } else {
            assert!(m[i].abs() < 1E-4);
        }
    }
}

#[test]
fn identity() {
    assert_identity(2, &matrix::identity::<f32, 4, 2>());
    assert_identity(3, &matrix::identity::<f32, 9, 3>());
    assert_identity(4, &matrix::identity::<f32, 16, 4>());
    assert_identity(5, &matrix::identity::<f32, 25, 5>());
    assert_identity(6, &matrix::identity::<f32, 36, 6>());

    assert_identity(2, &matrix::identity2::<f32>());
    assert_identity(3, &matrix::identity3::<f32>());
    assert_identity(4, &matrix::identity4::<f32>());
}

#[test]
fn multiply() {
    let id = &matrix::identity::<f32, 4, 2>();
    let m0: &[f32; 4] = &[1.0, 2.0, 3.0, 4.0];
    let m1: &[f32; 4] = &[2.0, 1.0, -1.0, -2.0];
    assert_eq!(&matrix::multiply::<f32, 4, 4, 4, 2, 2, 2>(id, m0), m0);
    assert_eq!(&matrix::multiply::<f32, 4, 4, 4, 2, 2, 2>(id, m1), m1);
    assert_eq!(
        &matrix::multiply::<f32, 4, 4, 4, 2, 2, 2>(m0, m1),
        &[0., -3., 2., -5.]
    );
}

#[test]
fn invert() {
    let id = matrix::identity::<f32, 4, 2>();
    let m1 = [-1., 0., 0., -1.];
    let m2 = [2., 3., 4., 1.];

    assert_eq!(&matrix::inverse2(&id), &id);
    assert_near_identity(2, &id);
    assert_near_identity(2, &matrix::multiply2(&id, &matrix::inverse2(&id)));

    assert_near_identity(2, &matrix::multiply2(&m1, &matrix::inverse2(&m1)));
    assert_near_identity(2, &matrix::multiply2(&m2, &matrix::inverse2(&m2)));
}

#[test]
fn lup_decompose() {
    let id = matrix::identity::<f32, 4, 2>();
    let m1 = [-1., 0., 0., -1.];
    // m2 = 2 3 | 4 1 whose LU is 4 1 | 1/2 5/2 with pivot [1,0]
    let m2 = [2., 3., 4., 1.];

    let mut lu = [0.; 4];
    let mut l = lu.clone();
    let mut result = lu.clone();
    let mut pivot = [0; 2];
    assert_eq!(
        matrix::lup_decompose(2, &id, &mut lu, &mut pivot),
        1.0,
        "Identity is invertible with determinant 1.0"
    );
    assert_eq!(&pivot, &[0, 1]);
    assert_eq!(&lu, &id);

    assert_eq!(
        matrix::lup_decompose(2, &m1, &mut lu, &mut pivot),
        1.0,
        "M1 is invertible with determinant 1.0"
    );
    assert_eq!(&pivot, &[0, 1]);
    assert_eq!(&lu, &m1, "M1 is an upper matrix so LU(M1) == M1");

    assert_eq!(
        matrix::lup_decompose(2, &m2, &mut lu, &mut pivot),
        -10.0,
        "M2 is invertible with determinant -10.0"
    );
    assert_eq!(
        &pivot,
        &[1, 0],
        "M2[0,1] is larger than M2[0,0] so rows 0 and 1 are swapped"
    );
    assert_eq!(&lu, &[4., 1., 0.5, 2.5], "LU(M2) == [4, 1, 1/2, -5/2]");
    matrix::lu_split(2, &mut lu, &mut l);
    assert_eq!(
        &matrix::multiply2(&l, &lu),
        &[4., 1., 2., 3.],
        "L*U should equal M2 pivoted"
    );
    matrix::unpivot(2, &matrix::multiply2(&l, &lu), &pivot, &mut result);
    assert_eq!(&result, &m2);
}

fn check_lut_invert<const N: usize, const N2: usize>(matrix: &[f32; N2]) {
    let mut lu = [0.; N2];
    let mut result = lu.clone();
    let mut pivot = [0; N];
    let mut tr0 = [0.; N];
    let mut tr1 = [0.; N];

    assert_ne!(
        matrix::lup_decompose(N, matrix, &mut lu, &mut pivot),
        0.0,
        "Matrix is invertible"
    );
    assert!(
        matrix::lup_invert(N, &lu, &pivot, &mut result, &mut tr0, &mut tr1),
        "Matrix is invertible"
    );
    assert_near_identity(
        N,
        &matrix::multiply::<_, N2, N2, N2, N, N, N>(&result, matrix),
    );
    assert_near_identity(
        N,
        &matrix::multiply::<_, N2, N2, N2, N, N, N>(matrix, &result),
    );

    let mut result = matrix.clone();
    matrix::invert::<_, N2, N>(&mut result);
    assert_near_identity(
        N,
        &matrix::multiply::<_, N2, N2, N2, N, N, N>(&result, matrix),
    );
    assert_near_identity(
        N,
        &matrix::multiply::<_, N2, N2, N2, N, N, N>(matrix, &result),
    );
}

#[test]
fn lut_invert() {
    check_lut_invert::<2, 4>(&matrix::identity::<f32, 4, 2>());
    check_lut_invert::<2, 4>(&[2., 3., 4., 1.]);

    check_lut_invert::<3, 9>(&[2., 3., 4., 1., 5., 2., 3., 6., 2.]);
}
