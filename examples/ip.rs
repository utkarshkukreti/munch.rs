extern crate munch;

use munch::{P, Parser};

#[derive(Clone, Debug, PartialEq)]
pub struct Ip(pub u8, pub u8, pub u8, pub u8);

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    ExpectedDot,
    ExpectedEnd,
    ExpectedInteger,
    IntegerOverflow,
}

pub fn parse(str: &str) -> Result<Ip, (usize, Error)> {
    use self::Error;
    use munch::ascii;
    use munch::str::*;

    let octet = || {
        P(ascii::Satisfy(|b| b'0' <= b && b <= b'9')
                .repeat(1..3)
                .fold(|| 0, |acc, x| acc * 10 + x as u16 - 48))
            .map_err(|_| Error::ExpectedInteger)
            .and_then(|n| if n < 256 {
                Ok(n as u8)
            } else {
                Err(Error::IntegerOverflow)
            })
    };

    let dot = || '.'.map_err(|_| Error::ExpectedDot);

    (octet() << dot(),
     octet() << dot(),
     octet() << dot(),
     octet() << End.map_err(|_| Error::ExpectedEnd))
        .map(|(a, b, c, d)| Ip(a, b, c, d))
        .parse(str, 0)
        .map(|(_, ip)| ip)
}

#[cfg(not(test))]
pub fn main() {
    use std::io::prelude::*;

    assert_eq!(parse("0.0.0.0"), Ok(Ip(0, 0, 0, 0)));
    assert_eq!(parse("255.255.255.255"), Ok(Ip(255, 255, 255, 255)));
    assert_eq!(parse("0.0"), Err((3, Error::ExpectedDot)));
    assert_eq!(parse("0.0.0.0."), Err((7, Error::ExpectedEnd)));
    assert_eq!(parse("0.0."), Err((4, Error::ExpectedInteger)));
    assert_eq!(parse("0.0.0.256"), Err((9, Error::IntegerOverflow)));

    let mut string = String::new();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut stdout = std::io::stdout();
    loop {
        print!("> ");
        stdout.flush().unwrap();
        stdin.read_line(&mut string).unwrap();
        println!("{:?}", parse(string.trim_right()));
        string.clear();
    }
}
