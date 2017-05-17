extern crate munch;

use munch::ascii::*;
use munch::error::*;
use munch::str;

#[macro_use]
mod t;

fn is_alphabetic(u8: u8) -> bool {
    match u8 {
        b'a'...b'z' | b'A'...b'Z' => true,
        _ => false,
    }
}

#[test]
fn satisfy() {
    t! {
        Satisfy(|u8| u8 == b'p') => {
            "" => Err((0, Error::Ascii(Ascii::Satisfy))),
            "p" => Ok((1, b'p')),
            "pr" => Ok((1, b'p')),
            "pr²" => Ok((1, b'p')),
        },
        Satisfy(|u8| u8 == 0x7F) => {
            "\x7F" => Ok((1, 0x7F)),
        },
        Satisfy(|u8| u8 == "π".as_bytes()[0]) => {
            "π" => Err((0, Error::Ascii(Ascii::Satisfy))),
        },
    }
}

#[test]
fn take_while() {
    t! {
        TakeWhile(is_alphabetic) => {
            "" => Ok((0, "")),
            "p" => Ok((1, "p")),
            "pr" => Ok((2, "pr")),
            "pr2" => Ok((2, "pr")),
            "pr2h" => Ok((2, "pr")),
        },
        TakeWhile(|u8| u8 == 0x7F) => {
            "\x7F" => Ok((1, "\x7F")),
            "\x7F\x7F" => Ok((2, "\x7F\x7F")),
        },
        TakeWhile(|u8| u8 == "π".as_bytes()[0]) => {
            "π" => Ok((0, "")),
        },
    }
}

#[test]
fn take_while1() {
    t! {
        TakeWhile1(is_alphabetic) => {
            "" => Err((0, Error::Ascii(Ascii::TakeWhile1))),
            "p" => Ok((1, "p")),
            "pr" => Ok((2, "pr")),
            "pr2" => Ok((2, "pr")),
            "pr2h" => Ok((2, "pr")),
        },
        TakeWhile1(|u8| u8 == 0x7F) => {
            "\x7F" => Ok((1, "\x7F")),
            "\x7F\x7F" => Ok((2, "\x7F\x7F")),
        },
        TakeWhile1(|u8| u8 == "π".as_bytes()[0]) => {
            "π" => Err((0, Error::Ascii(Ascii::TakeWhile1))),
        },
    }
}
