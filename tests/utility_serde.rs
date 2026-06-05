#[cfg(feature = "serde")]
use serde_test::{Token, assert_tokens};

mod utility;
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
