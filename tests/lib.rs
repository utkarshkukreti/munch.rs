extern crate munch;

use munch::*;
use munch::str::*;

#[macro_use]
mod t;

#[test]
fn p() {
    t! {
        P('π') => {
            "" => Err((0, Error::Char('π'))),
            "π" => Ok((2, 'π')),
            "πr" => Ok((2, 'π')),
        },
    }
}

#[test]
fn and() {
    t! {
        'π'.and('r').and('²') => {
            "" => Err((0, Error::Char('π'))),
            "a" => Err((0, Error::Char('π'))),
            "π" => Err((2, Error::Char('r'))),
            "πh" => Err((2, Error::Char('r'))),
            "πr" => Err((3, Error::Char('²'))),
            "πrh" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, (('π', 'r'), '²'))),
            "πr²h" => Ok((5, (('π', 'r'), '²'))),
        },
    }
}

#[test]
fn or() {
    t! {
        'π'.or('r').or('²') => {
            "" => Err((0, Error::Char('²'))),
            "a" => Err((0, Error::Char('²'))),
            "π" => Ok((2, 'π')),
            "r" => Ok((1, 'r')),
            "²" => Ok((2, '²')),
        },
        'π'.and('r').and('²').or('2'.and('π').and('r')) => {
            "" => Err((0, Error::Char('2'))),
            "π" => Err((2, Error::Char('r'))),
            "πr" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, (('π', 'r'), '²'))),
            "πr²h" => Ok((5, (('π', 'r'), '²'))),
            "2" => Err((1, Error::Char('π'))),
            "2π" => Err((3, Error::Char('r'))),
            "2πr" => Ok((4, (('2', 'π'), 'r'))),
            "2πrh" => Ok((4, (('2', 'π'), 'r'))),
        },
        'π'.and('r').and('²').or('π'.and('r').and('2')) => {
            "πr²" => Ok((5, (('π', 'r'), '²'))),
            "πr2" => Err((3, Error::Char('²'))),
        },
        Try('π'.and('r').and('²')).or('π'.and('r').and('2')) => {
            "πr²" => Ok((5, (('π', 'r'), '²'))),
            "πr2" => Ok((4, (('π', 'r'), '2'))),
        },
    }
}

#[test]
fn try() {
    t! {
        Try('π'.and('r').and('²')) => {
            "" => Err((0, Error::Char('π'))),
            "a" => Err((0, Error::Char('π'))),
            "π" => Err((0, Error::Char('r'))),
            "πh" => Err((0, Error::Char('r'))),
            "πr" => Err((0, Error::Char('²'))),
            "πrh" => Err((0, Error::Char('²'))),
            "πr²" => Ok((5, (('π', 'r'), '²'))),
            "πr²h" => Ok((5, (('π', 'r'), '²'))),
        },
    }
}

#[test]
fn map() {
    t! {
        "one".or("two").or("three").map(|str| str.len())
          .or(Satisfy(char::is_alphabetic).map(|ch| ch.len_utf8())) => {
            "one" => Ok((3, 3)),
            "two" => Ok((3, 3)),
            "three" => Ok((5, 5)),
            "a" => Ok((1, 1)),
            "π" => Ok((2, 2)),
        },
    }
}

#[test]
fn tuple() {
    t! {
        ('a',) => {
            "a" => Ok((1, ('a',))),
        },
        ('a', 'b') => {
            "ab" => Ok((2, ('a', 'b'))),
        },
        ('a', 'b', 'c') => {
            "abc" => Ok((3, ('a', 'b', 'c'))),
        },
        ('a', 'b', 'c', 'd') => {
            "abcd" => Ok((4, ('a', 'b', 'c', 'd'))),
        },
        ('a', 'b', 'c', 'd', 'e') => {
            "abcde" => Ok((5, ('a', 'b', 'c', 'd', 'e'))),
        },
        ('a', 'b', 'c', 'd', 'e', 'f') => {
            "abcdef" => Ok((6, ('a', 'b', 'c', 'd', 'e', 'f'))),
        },
        ('a', 'b', 'c', 'd', 'e', 'f', 'g') => {
            "abcdefg" => Ok((7, ('a', 'b', 'c', 'd', 'e', 'f', 'g'))),
        },
        ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h') => {
            "abcdefgh" => Ok((8, ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'))),
        },
        ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i') => {
            "abcdefghi" => Ok((9, ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'))),
        },
        ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j') => {
            "abcdefghij" => Ok((10, ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'))),
        },
        ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k') => {
            "abcdefghijk" => Ok((11, ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'))),
        },
        ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l') => {
            "abcdefghijkl" =>
                Ok((12, ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'))),
        },
    }
}

#[test]
fn fn_() {
    let mut pi = |input: &str, from: usize| if input[from..].starts_with("pi") {
        Ok((from + 2, 'π'))
    } else {
        Err((from, ()))
    };

    t! {
        pi => {
            "" => Err((0, ())),
            "pi" => Ok((2, 'π')),
            "pie" => Ok((2, 'π')),
        },
    }
}
