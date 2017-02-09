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
