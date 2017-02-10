#[macro_use]
extern crate munch;

use munch::*;
use munch::str::*;

#[macro_use]
mod t;

#[test]
fn mac() {
    #[derive(Debug, PartialEq)]
    enum Error {
        Munch(munch::str::Error<'static>),
        ParseIntError(std::num::ParseIntError),
        AllEqual(u8),
    }

    let mut u8 = TakeWhile1(|ch| '0' <= ch && ch <= '9')
        .map_err(Error::Munch)
        .and_then(|str: &str| str.parse::<u8>().map_err(Error::ParseIntError));

    #[derive(Debug, PartialEq)]
    struct Ip(u8, u8, u8, u8);

    let mut ip = muncher! {
        a <- u8,
        P('.').map_err(Error::Munch),
        b <- |input, from| u8.parse(input, from),
        Satisfy(|ch| ch == '.').map_err(Error::Munch),
        c <- u8,
        let c = c + 1,
        P('.').map_err(Error::Munch),
        d <- u8,
        (if a == b && b == c && c == d {
            Err(Error::AllEqual(a))
        } else {
            Ok(Ip(a, b, c, d))
        })
    };

    t! {
        ip => {
            "" => Err((0, Error::Munch(munch::str::Error::TakeWhile1))),
            "a" => Err((0, Error::Munch(munch::str::Error::TakeWhile1))),
            "1" => Err((1, Error::Munch(munch::str::Error::Char('.')))),
            "1." => Err((2, Error::Munch(munch::str::Error::TakeWhile1))),
            "1.2" => Err((3, Error::Munch(munch::str::Error::Satisfy))),
            "1.2." => Err((4, Error::Munch(munch::str::Error::TakeWhile1))),
            "1.2.3.4" => Ok((7, Ip(1, 2, 4, 4))),
            "256" => Err((3, Error::ParseIntError("256".parse::<u8>().err().unwrap()))),
            "1.2.3.256" => Err((9, Error::ParseIntError("256".parse::<u8>().err().unwrap()))),
            "1.1.0.1" => Err((7, Error::AllEqual(1))),
        },
    }
}