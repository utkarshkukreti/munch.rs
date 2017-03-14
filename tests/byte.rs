extern crate munch;

use munch::byte::*;

#[macro_use]
mod t;

fn is_alphabetic(b: u8) -> bool {
    match b {
        b'a'...b'z' | b'A'...b'Z' => true,
        _ => false,
    }
}

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

#[test]
fn satisfy() {
    tb! {
        Satisfy(is_alphabetic) => {
            b"" => Err((0, Error::Satisfy)),
            b"p" => Ok((1, b'p')),
            b"pr" => Ok((1, b'p')),
        },
    }
}

#[test]
fn take_while() {
    tb! {
        TakeWhile(is_alphabetic) => {
            b"" => Ok((0, b"".as_ref())),
            b"p" => Ok((1, b"p".as_ref())),
            b"pr" => Ok((2, b"pr".as_ref())),
            b"pr2" => Ok((2, b"pr".as_ref())),
            b"pr2h" => Ok((2, b"pr".as_ref())),
        },
    }
}

#[test]
fn take_while1() {
    tb! {
        TakeWhile1(is_alphabetic) => {
            b"" => Err((0, Error::TakeWhile1)),
            b"p" => Ok((1, b"p".as_ref())),
            b"pr" => Ok((2, b"pr".as_ref())),
            b"pr2" => Ok((2, b"pr".as_ref())),
            b"pr2h" => Ok((2, b"pr".as_ref())),
        },
    }
}
