extern crate munch;

use munch::byte::*;

#[macro_use]
mod t;

#[test]
fn byte() {
    tb! {
        b'p' => {
            b"" => Err((0, Error::Byte(b'p'))),
            b"p" => Ok((1, b'p')),
            b"pr" => Ok((1, b'p')),
            b"pr2" => Ok((1, b'p')),
        },
        207 => {
            "π".as_bytes() => Ok((1, 207)),
        },
    }
}

#[test]
fn bytes() {
    tb! {
        b"pr".as_ref() => {
            b"" => Err((0, Error::Bytes(b"pr"))),
            b"p" => Err((0, Error::Bytes(b"pr"))),
            b"pr" => Ok((2, b"pr".as_ref())),
            b"pr2" => Ok((2, b"pr".as_ref())),
        },
    }
}
