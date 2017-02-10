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
