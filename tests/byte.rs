extern crate munch;

use munch::byte::*;
use munch::error::{BinaryType, Endianness, Error};

#[macro_use]
mod t;

fn is_alphabetic(b: u8) -> bool {
    match b {
        b'a'..=b'z' | b'A'..=b'Z' => true,
        _ => false,
    }
}

fn is_alphanumeric(b: u8) -> bool {
    is_alphabetic(b)
        || match b {
            b'0'..=b'9' => true,
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

#[test]
fn capture() {
    tb! {
        Capture((Satisfy(is_alphabetic), TakeWhile(is_alphanumeric))) => {
            b"1" => Err((0, Error::Satisfy)),
            b"p" => Ok((1, b"p".as_ref())),
            b"pr" => Ok((2, b"pr".as_ref())),
            b"pr2" => Ok((3, b"pr2".as_ref())),
            b"prr" => Ok((3, b"prr".as_ref())),
            b"prrh" => Ok((4, b"prrh".as_ref())),
        },
    }
}

#[test]
fn any() {
    tb! {
        Any => {
            b"" => Err((0, Error::Any)),
            b"p" => Ok((1, b'p')),
            b"pr" => Ok((1, b'p')),
        },
    }
}

#[test]
fn peek() {
    tb! {
        Peek => {
            b"" => Err((0, Error::Peek)),
            b"p" => Ok((0, b'p')),
            b"pr" => Ok((0, b'p')),
        },
    }
}

#[test]
fn take() {
    tb! {
        Take(0) => {
            b"" => Ok((0, b"".as_ref())),
            b"p" => Ok((0, b"".as_ref())),
            b"pr" => Ok((0, b"".as_ref())),
        },
        Take(2) => {
            b"" => Err((0, Error::Take(2))),
            b"p" => Err((0, Error::Take(2))),
            b"pr" => Ok((2, b"pr".as_ref())),
            b"pr2" => Ok((2, b"pr".as_ref())),
        },
    }
}

#[test]
fn end() {
    tb! {
        End => {
            b"" => Ok((0, ())),
            b"p" => Err((0, Error::End)),
            b"pr" => Err((0, Error::End)),
        },
        (b'p', End) => {
            b"" => Err((0, Error::Byte(b'p'))),
            b"p" => Ok((1, (b'p', ()))),
            b"pr" => Err((1, Error::End)),
        },
    }
}

#[test]
fn little_endian() {
    let buffer = [148, 213, 176, 241, 166, 150, 135, 255, 183];
    let s = |n| &buffer[..n];
    let e = |ty| Error::Binary(Endianness::Little, ty);

    tb! {
        LittleEndian::u8 => {
            s(0) => Err((0, e(BinaryType::u8))),
            s(1) => Ok((1, 148)),
            s(2) => Ok((1, 148)),
        },
        LittleEndian::u16 => {
            s(0) => Err((0, e(BinaryType::u16))),
            s(1) => Err((0, e(BinaryType::u16))),
            s(2) => Ok((2, 54676)),
            s(3) => Ok((2, 54676)),
        },
        LittleEndian::u32 => {
            s(0) => Err((0, e(BinaryType::u32))),
            s(1) => Err((0, e(BinaryType::u32))),
            s(2) => Err((0, e(BinaryType::u32))),
            s(3) => Err((0, e(BinaryType::u32))),
            s(4) => Ok((4, 4054898068)),
            s(5) => Ok((4, 4054898068)),
        },
        LittleEndian::u64 => {
            s(0) => Err((0, e(BinaryType::u64))),
            s(1) => Err((0, e(BinaryType::u64))),
            s(2) => Err((0, e(BinaryType::u64))),
            s(3) => Err((0, e(BinaryType::u64))),
            s(4) => Err((0, e(BinaryType::u64))),
            s(5) => Err((0, e(BinaryType::u64))),
            s(6) => Err((0, e(BinaryType::u64))),
            s(7) => Err((0, e(BinaryType::u64))),
            s(8) => Ok((8, 18412851245291197844)),
            s(9) => Ok((8, 18412851245291197844)),
        },
        LittleEndian::i8 => {
            s(0) => Err((0, e(BinaryType::i8))),
            s(1) => Ok((1, -108)),
            s(2) => Ok((1, -108)),
        },
        LittleEndian::i16 => {
            s(0) => Err((0, e(BinaryType::i16))),
            s(1) => Err((0, e(BinaryType::i16))),
            s(2) => Ok((2, -10860)),
            s(3) => Ok((2, -10860)),
        },
        LittleEndian::i32 => {
            s(0) => Err((0, e(BinaryType::i32))),
            s(1) => Err((0, e(BinaryType::i32))),
            s(2) => Err((0, e(BinaryType::i32))),
            s(3) => Err((0, e(BinaryType::i32))),
            s(4) => Ok((4, -240069228)),
            s(5) => Ok((4, -240069228)),
        },
        LittleEndian::i64 => {
            s(0) => Err((0, e(BinaryType::i64))),
            s(1) => Err((0, e(BinaryType::i64))),
            s(2) => Err((0, e(BinaryType::i64))),
            s(3) => Err((0, e(BinaryType::i64))),
            s(4) => Err((0, e(BinaryType::i64))),
            s(5) => Err((0, e(BinaryType::i64))),
            s(6) => Err((0, e(BinaryType::i64))),
            s(7) => Err((0, e(BinaryType::i64))),
            s(8) => Ok((8, -33892828418353772)),
            s(9) => Ok((8, -33892828418353772)),
        },
        LittleEndian::f32 => {
            s(0) => Err((0, e(BinaryType::f32))),
            s(1) => Err((0, e(BinaryType::f32))),
            s(2) => Err((0, e(BinaryType::f32))),
            s(3) => Err((0, e(BinaryType::f32))),
            s(4) => Ok((4, -1.7512819788279716e30)),
            s(5) => Ok((4, -1.7512819788279716e30)),
        },
        LittleEndian::f64 => {
            s(0) => Err((0, e(BinaryType::f64))),
            s(1) => Err((0, e(BinaryType::f64))),
            s(2) => Err((0, e(BinaryType::f64))),
            s(3) => Err((0, e(BinaryType::f64))),
            s(4) => Err((0, e(BinaryType::f64))),
            s(5) => Err((0, e(BinaryType::f64))),
            s(6) => Err((0, e(BinaryType::f64))),
            s(7) => Err((0, e(BinaryType::f64))),
            s(8) => Ok((8, -2.070549673017168e306)),
            s(9) => Ok((8, -2.070549673017168e306)),
        },
    }
}

#[test]
fn big_endian() {
    let buffer = [148, 213, 176, 241, 166, 150, 135, 255, 183];
    let s = |n| &buffer[..n];
    let e = |ty| Error::Binary(Endianness::Big, ty);

    tb! {
        BigEndian::u8 => {
            s(0) => Err((0, e(BinaryType::u8))),
            s(1) => Ok((1, 148)),
            s(2) => Ok((1, 148)),
        },
        BigEndian::u16 => {
            s(0) => Err((0, e(BinaryType::u16))),
            s(1) => Err((0, e(BinaryType::u16))),
            s(2) => Ok((2, 38101)),
            s(3) => Ok((2, 38101)),
        },
        BigEndian::u32 => {
            s(0) => Err((0, e(BinaryType::u32))),
            s(1) => Err((0, e(BinaryType::u32))),
            s(2) => Err((0, e(BinaryType::u32))),
            s(3) => Err((0, e(BinaryType::u32))),
            s(4) => Ok((4, 2497032433)),
            s(5) => Ok((4, 2497032433)),
        },
        BigEndian::u64 => {
            s(0) => Err((0, e(BinaryType::u64))),
            s(1) => Err((0, e(BinaryType::u64))),
            s(2) => Err((0, e(BinaryType::u64))),
            s(3) => Err((0, e(BinaryType::u64))),
            s(4) => Err((0, e(BinaryType::u64))),
            s(5) => Err((0, e(BinaryType::u64))),
            s(6) => Err((0, e(BinaryType::u64))),
            s(7) => Err((0, e(BinaryType::u64))),
            s(8) => Ok((8, 10724672639581194239)),
            s(9) => Ok((8, 10724672639581194239)),
        },
        BigEndian::i8 => {
            s(0) => Err((0, e(BinaryType::i8))),
            s(1) => Ok((1, -108)),
            s(2) => Ok((1, -108)),
        },
        BigEndian::i16 => {
            s(0) => Err((0, e(BinaryType::i16))),
            s(1) => Err((0, e(BinaryType::i16))),
            s(2) => Ok((2, -27435)),
            s(3) => Ok((2, -27435)),
        },
        BigEndian::i32 => {
            s(0) => Err((0, e(BinaryType::i32))),
            s(1) => Err((0, e(BinaryType::i32))),
            s(2) => Err((0, e(BinaryType::i32))),
            s(3) => Err((0, e(BinaryType::i32))),
            s(4) => Ok((4, -1797934863)),
            s(5) => Ok((4, -1797934863)),
        },
        BigEndian::i64 => {
            s(0) => Err((0, e(BinaryType::i64))),
            s(1) => Err((0, e(BinaryType::i64))),
            s(2) => Err((0, e(BinaryType::i64))),
            s(3) => Err((0, e(BinaryType::i64))),
            s(4) => Err((0, e(BinaryType::i64))),
            s(5) => Err((0, e(BinaryType::i64))),
            s(6) => Err((0, e(BinaryType::i64))),
            s(7) => Err((0, e(BinaryType::i64))),
            s(8) => Ok((8, -7722071434128357377)),
            s(9) => Ok((8, -7722071434128357377)),
        },
        BigEndian::f32 => {
            s(0) => Err((0, e(BinaryType::f32))),
            s(1) => Err((0, e(BinaryType::f32))),
            s(2) => Err((0, e(BinaryType::f32))),
            s(3) => Err((0, e(BinaryType::f32))),
            s(4) => Ok((4, -2.1577294798898393e-26)),
            s(5) => Ok((4, -2.1577294798898393e-26)),
        },
        BigEndian::f64 => {
            s(0) => Err((0, e(BinaryType::f64))),
            s(1) => Err((0, e(BinaryType::f64))),
            s(2) => Err((0, e(BinaryType::f64))),
            s(3) => Err((0, e(BinaryType::f64))),
            s(4) => Err((0, e(BinaryType::f64))),
            s(5) => Err((0, e(BinaryType::f64))),
            s(6) => Err((0, e(BinaryType::f64))),
            s(7) => Err((0, e(BinaryType::f64))),
            s(8) => Ok((8, -2.6391637269993256e-208)),
            s(9) => Ok((8, -2.6391637269993256e-208)),
        },
    }
}
