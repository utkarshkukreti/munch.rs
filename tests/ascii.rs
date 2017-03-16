extern crate munch;

use munch::ascii::*;
use munch::str;

#[macro_use]
mod t;

#[test]
fn satisfy() {
    t! {
        Satisfy(|u8| u8 == b'p') => {
            "" => Err((0, str::Error::Ascii(Error::Satisfy))),
            "p" => Ok((1, b'p')),
            "pr" => Ok((1, b'p')),
            "pr²" => Ok((1, b'p')),
        },
        Satisfy(|u8| u8 == 0x7F) => {
            "\x7F" => Ok((1, 0x7F)),
        },
        Satisfy(|u8| u8 == "π".as_bytes()[0]) => {
            "π" => Err((0, str::Error::Ascii(Error::Satisfy))),
        },
    }
}
