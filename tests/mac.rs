use munch::str::*;
use munch::*;

#[macro_use]
mod t;

#[test]
fn mac() {
    #[derive(Debug, PartialEq)]
    enum Error {
        Munch(munch::error::Error<'static>),
        ParseInt(std::num::ParseIntError),
        AllEqual(u8),
    }

    let mut u8 = TakeWhile1(|ch| '0' <= ch && ch <= '9')
        .map_err(Error::Munch)
        .and_then(|str: &str| str.parse::<u8>().map_err(Error::ParseInt));

    #[derive(Debug, PartialEq)]
    struct Ip(u8, u8, u8, u8);

    let mut ip = muncher! {
        a <- u8,
        '.'.map_err(Error::Munch),
        b <- |input, from| u8.parse(input, from),
        Satisfy(|ch| ch == '.').map_err(Error::Munch),
        c <- u8,
        let c = c + 1,
        '.'.map_err(Error::Munch),
        d <- u8,
        (if a == b && b == c && c == d {
            Err(Error::AllEqual(a))
        } else {
            Ok(Ip(a, b, c, d))
        })
    };

    t! {
        ip => {
            "" => Err((0, Error::Munch(munch::error::Error::TakeWhile1))),
            "a" => Err((0, Error::Munch(munch::error::Error::TakeWhile1))),
            "1" => Err((1, Error::Munch(munch::error::Error::Char('.')))),
            "1." => Err((2, Error::Munch(munch::error::Error::TakeWhile1))),
            "1.2" => Err((3, Error::Munch(munch::error::Error::Satisfy))),
            "1.2." => Err((4, Error::Munch(munch::error::Error::TakeWhile1))),
            "1.2.3.4" => Ok((7, Ip(1, 2, 4, 4))),
            "256" => Err((3, Error::ParseInt("256".parse::<u8>().err().unwrap()))),
            "1.2.3.256" => Err((9, Error::ParseInt("256".parse::<u8>().err().unwrap()))),
            "1.1.0.1" => Err((7, Error::AllEqual(1))),
        },
    }

    let mut count = muncher! {
        let mut n = 0,
        Any.map(|_| n += 1).repeat(1..),
        (Ok(n))
    };

    t! {
        count => {
            "" => Err((0, munch::error::Error::Any)),
            "π" => Ok((2, 1)),
            "πr" => Ok((3, 2)),
            "πr²" => Ok((5, 3)),
        },
    }

    let mut json = muncher! {
        next <- Peek,
        value <- @match (next) {
            'n' => "null",
            'f' => "false",
            't' => "true",
            '0' | '1' | '2' ..= '9' => TakeWhile1(|ch| ch.is_digit(10)),
            _ => |_, from| Err((from, munch::error::Error::Satisfy)),
        },
        End,
        (Ok(value))
    };

    t! {
        json => {
            "n" => Err((0, munch::error::Error::Str("null"))),
            "nu" => Err((0, munch::error::Error::Str("null"))),
            "nul" => Err((0, munch::error::Error::Str("null"))),
            "null" => Ok((4, "null")),
            "null." => Err((4, munch::error::Error::End)),
            "f" => Err((0, munch::error::Error::Str("false"))),
            "false" => Ok((5, "false")),
            "false." => Err((5, munch::error::Error::End)),
            "t" => Err((0, munch::error::Error::Str("true"))),
            "true" => Ok((4, "true")),
            "true." => Err((4, munch::error::Error::End)),
            "0" => Ok((1, "0")),
            "0123" => Ok((4, "0123")),
            "0123." => Err((4, munch::error::Error::End)),
            "π" => Err((0, munch::error::Error::Satisfy)),
        },
    }
}
