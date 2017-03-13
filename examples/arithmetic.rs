extern crate munch;

use munch::{Optional, P, Parser};

pub fn expr(str: &str, from: usize) -> munch::Result<i64, munch::str::Error<'static>> {
    use munch::str::*;

    let ws = P(TakeWhile(char::is_whitespace));

    let integer = P(Capture((Optional('-'), TakeWhile1(|ch| ch.is_digit(10))))
        .map(|str| str.parse().unwrap()));

    let factor = (P('(') >> ws >> expr << ws << ')' | integer) << ws;

    let term = factor.repeat(1..)
        .join('*'.or('/') << ws)
        .fold(|| 0, |_, x| x, |acc, op, x| match op {
            '*' => acc * x,
            '/' => acc / x,
            _ => unreachable!(),
        });

    term.repeat(1..)
        .join('+'.or('-') << ws)
        .fold(|| 0, |_, x| x, |acc, op, x| match op {
            '+' => acc + x,
            '-' => acc - x,
            _ => unreachable!(),
        })
        .parse(str, from)
}

pub fn parse(str: &str) -> Result<i64, (usize, munch::str::Error<'static>)> {
    use munch::str::*;

    let ws = P(TakeWhile(char::is_whitespace));
    (ws >> expr << ws << End).parse(str, 0).map(|(_, output)| output)
}

#[cfg(not(test))]
pub fn main() {
    use std::io::prelude::*;

    assert_eq!(parse(" 1 + 2 "), Ok(3));
    assert_eq!(parse("1 + 2 * 3/4"), Ok(2));
    assert_eq!(parse("2 * 3+4 * 5"), Ok(26));
    assert_eq!(parse("1 + -2 * -3 * -1"), Ok(-5));
    assert_eq!(parse("(1 + -2) * -3 * -1"), Ok(-3));

    let mut string = String::new();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut stdout = std::io::stdout();
    loop {
        print!("> ");
        stdout.flush().unwrap();
        stdin.read_line(&mut string).unwrap();
        println!("{:?}", parse(&string));
        string.clear();
    }
}
