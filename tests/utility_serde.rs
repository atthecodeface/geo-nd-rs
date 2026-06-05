#[cfg(feature = "serde")]
use serde_test::{Token, assert_tokens};

mod utility;
#[cfg(feature = "serde")]
use utility::{Quat, TFT_3, Vector3};

#[test]
#[cfg(feature = "serde")]
fn utility_serde() -> Result<(), String> {
    let v: Vector3 = TFT_3.into();
    assert_tokens(
        &v,
        &[
            Token::Tuple { len: 3 },
            Token::F32(3.0),
            Token::F32(4.0),
            Token::F32(12.0),
            Token::TupleEnd,
        ],
    );
    let q: Quat = [0.1, 0.3, 0.5, 0.7].into();
    assert_tokens(
        &q,
        &[
            Token::Tuple { len: 4 },
            Token::F32(0.1),
            Token::F32(0.3),
            Token::F32(0.5),
            Token::F32(0.7),
            Token::TupleEnd,
        ],
    );
    Ok(())
}

#[test]
#[cfg(feature = "serde")]
fn transformation() -> Result<(), String> {
    use geo_nd::{QArray, Transformation};
    let t: Transformation<f32, QArray<f32>> = Transformation::default();
    assert_tokens(
        &t,
        &[
            Token::Struct {
                name: "Transformation",
                len: 3,
            },
            Token::Str("rotation"),
            Token::Tuple { len: 4 },
            Token::F32(0.0),
            Token::F32(0.0),
            Token::F32(0.0),
            Token::F32(1.0),
            Token::TupleEnd,
            Token::Str("translation"),
            Token::Tuple { len: 3 },
            Token::F32(0.0),
            Token::F32(0.0),
            Token::F32(0.0),
            Token::TupleEnd,
            Token::Str("scale"),
            Token::Tuple { len: 3 },
            Token::F32(1.0),
            Token::F32(1.0),
            Token::F32(1.0),
            Token::TupleEnd,
            Token::StructEnd,
        ],
    );
    Ok(())
}

#[test]
#[cfg(feature = "serde")]
fn transformation_uniform() -> Result<(), String> {
    use geo_nd::{QArray, TransformationUniform};
    let t: TransformationUniform<f32, QArray<f32>> = TransformationUniform::default();
    assert_tokens(
        &t,
        &[
            Token::Struct {
                name: "TransformationUniform",
                len: 3,
            },
            Token::Str("rotation"),
            Token::Tuple { len: 4 },
            Token::F32(0.0),
            Token::F32(0.0),
            Token::F32(0.0),
            Token::F32(1.0),
            Token::TupleEnd,
            Token::Str("translation"),
            Token::Tuple { len: 3 },
            Token::F32(0.0),
            Token::F32(0.0),
            Token::F32(0.0),
            Token::TupleEnd,
            Token::Str("scale"),
            Token::F32(1.0),
            Token::StructEnd,
        ],
    );
    Ok(())
}
