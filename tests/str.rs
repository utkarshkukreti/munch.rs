extern crate munch;

use munch::str::*;

#[macro_use]
mod t;

#[test]
fn char() {
    t! {
        'π' => {
            "" => Err((0, Error::Char('π'))),
            "π" => Ok((2, 'π')),
            "πr" => Ok((2, 'π')),
            "πr²" => Ok((2, 'π')),
        },
    }
}

#[test]
fn str() {
    t! {
        "πr" => {
            "" => Err((0, Error::Str("πr"))),
            "π" => Err((0, Error::Str("πr"))),
            "πr" => Ok((3, "πr")),
            "πr²" => Ok((3, "πr")),
        },
    }
}

#[test]
fn satisfy() {
    t! {
        Satisfy(char::is_alphabetic) => {
            "" => Err((0, Error::Satisfy)),
            "π" => Ok((2, 'π')),
            "πr" => Ok((2, 'π')),
        },
    }
}

#[test]
fn take_while() {
    t! {
        TakeWhile(char::is_alphabetic) => {
            "" => Ok((0, "")),
            "π" => Ok((2, "π")),
            "πr" => Ok((3, "πr")),
            "πr²" => Ok((3, "πr")),
            "πr²h" => Ok((3, "πr")),
        },
    }
}

#[test]
fn take_while1() {
    t! {
        TakeWhile1(char::is_alphabetic) => {
            "" => Err((0, Error::TakeWhile1)),
            "π" => Ok((2, "π")),
            "πr" => Ok((3, "πr")),
            "πr²" => Ok((3, "πr")),
            "πr²h" => Ok((3, "πr")),
        },
    }
}

#[test]
fn capture() {
    t! {
        Capture((Satisfy(char::is_alphabetic), TakeWhile(char::is_alphanumeric))) => {
            "1" => Err((0, Error::Satisfy)),
            "π" => Ok((2, "π")),
            "πr" => Ok((3, "πr")),
            "πr²" => Ok((3, "πr")),
            "πrr" => Ok((4, "πrr")),
            "πrrh" => Ok((5, "πrrh")),
        },
    }
}

#[test]
fn any() {
    t! {
        Any => {
            "" => Err((0, Error::Any)),
            "π" => Ok((2, 'π')),
            "πr" => Ok((2, 'π')),
        },
    }
}
