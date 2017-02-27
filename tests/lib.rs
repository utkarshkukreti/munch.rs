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
        P('π') | 'r' | "²".map(|_| '2') => {
            "" => Err((0, Error::Str("²"))),
            "π" => Ok((2, 'π')),
            "πr" => Ok((2, 'π')),
            "r" => Ok((1, 'r')),
            "r²" => Ok((1, 'r')),
            "²" => Ok((2, '2')),
            "²³" => Ok((2, '2')),
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
fn map_err() {
    #[derive(Debug, PartialEq)]
    enum Error {
        Munch(munch::str::Error<'static>),
    }

    t! {
        "foo".map_err(Error::Munch) => {
            "" => Err((0, Error::Munch(munch::str::Error::Str("foo")))),
            "foo" => Ok((3, "foo")),
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

#[test]
fn and_then() {
    #[derive(Debug, PartialEq)]
    enum Error {
        Munch(munch::str::Error<'static>),
        ParseIntError(std::num::ParseIntError),
    }

    t! {
        TakeWhile1(|ch| '0' <= ch && ch <= '9')
            .map_err(Error::Munch)
            .and_then(|str: &str| str.parse::<u8>().map_err(Error::ParseIntError)) => {
            "" => Err((0, Error::Munch(munch::str::Error::TakeWhile1))),
            "0" => Ok((1, 0)),
            "255" => Ok((3, 255)),
            "256" => Err((3, Error::ParseIntError("256".parse::<u8>().err().unwrap()))),
            "1024" => Err((4, Error::ParseIntError("1024".parse::<u8>().err().unwrap()))),
        },
    }
}

#[test]
fn optional() {
    t! {
        'π'.optional() => {
            "" => Ok((0, None)),
            "π" => Ok((2, Some('π'))),
            "πr" => Ok((2, Some('π'))),
        },
        ('π', 'r', "²").optional() => {
            "" => Ok((0, None)),
            "π" => Err((2, Error::Char('r'))),
            "πr" => Err((3, Error::Str("²"))),
            "πr²" => Ok((5, Some(('π', 'r', "²")))),
            "πr²h" => Ok((5, Some(('π', 'r', "²")))),
        },
        (Try(('π', 'r')), "²").optional() => {
            "" => Ok((0, None)),
            "π" => Ok((0, None)),
            "πr" => Err((3, Error::Str("²"))),
            "πr²" => Ok((5, Some((('π', 'r'), "²")))),
            "πr²h" => Ok((5, Some((('π', 'r'), "²")))),
        },
    }
}

#[test]
fn many() {
    let tuple = ('π', 'r', '²');
    t! {
        tuple.many() => {
            "" => Ok((0, vec![tuple; 0])),
            "π" => Err((2, Error::Char('r'))),
            "πr" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, vec![tuple; 1])),
            "πr²π" => Err((7, Error::Char('r'))),
            "πr²πr" => Err((8, Error::Char('²'))),
            "πr²πr²" => Ok((10, vec![tuple; 2])),
            "πr²πr²π" => Err((12, Error::Char('r'))),
            "πr²πr²πr" => Err((13, Error::Char('²'))),
            "πr²πr²πr²" => Ok((15, vec![tuple; 3])),
        },
        (Try(('π', 'r')), '²').map(|((a, b), c)| (a, b, c)).many() => {
            "" => Ok((0, vec![tuple; 0])),
            "π" => Ok((0, vec![tuple; 0])),
            "πr" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, vec![tuple; 1])),
            "πr²π" => Ok((5, vec![tuple; 1])),
            "πr²πr" => Err((8, Error::Char('²'))),
            "πr²πr²" => Ok((10, vec![tuple; 2])),
            "πr²πr²π" => Ok((10, vec![tuple; 2])),
            "πr²πr²πr" => Err((13, Error::Char('²'))),
            "πr²πr²πr²" => Ok((15, vec![tuple; 3])),
        },
    }
}

#[test]
fn many1() {
    let tuple = ('π', 'r', '²');
    t! {
        tuple.many1() => {
            "" => Err((0, Error::Char('π'))),
            "π" => Err((2, Error::Char('r'))),
            "πr" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, vec![tuple; 1])),
            "πr²π" => Err((7, Error::Char('r'))),
            "πr²πr" => Err((8, Error::Char('²'))),
            "πr²πr²" => Ok((10, vec![tuple; 2])),
            "πr²πr²π" => Err((12, Error::Char('r'))),
            "πr²πr²πr" => Err((13, Error::Char('²'))),
            "πr²πr²πr²" => Ok((15, vec![tuple; 3])),
        },
        (Try(('π', 'r')), '²').map(|((a, b), c)| (a, b, c)).many1() => {
            "" => Err((0, Error::Char('π'))),
            "π" => Err((0, Error::Char('r'))),
            "πr" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, vec![tuple; 1])),
            "πr²π" => Ok((5, vec![tuple; 1])),
            "πr²πr" => Err((8, Error::Char('²'))),
            "πr²πr²" => Ok((10, vec![tuple; 2])),
            "πr²πr²π" => Ok((10, vec![tuple; 2])),
            "πr²πr²πr" => Err((13, Error::Char('²'))),
            "πr²πr²πr²" => Ok((15, vec![tuple; 3])),
        },
    }
}

#[test]
fn repeat() {
    fn t<R: Range + Clone>(range: R) {
        use std::cmp::Ordering::*;
        let (min, max) = (range.min(), range.max());
        let mut p1 = ('π', 'r', '²').repeat(range.clone());
        let mut p2 = (Try(('π', 'r')), '²').map(|((a, b), c)| (a, b, c)).repeat(range.clone());
        for i in 0..36 {
            let string = "πr²".chars().cycle().take(i).collect::<String>();
            let r1 = p1.parse(&string, 0);
            let r2 = p2.parse(&string, 0);
            let complete = i / 3;
            let cmp = match max {
                _ if complete < min => Less,
                Some(max) if complete >= max => Greater,
                _ => Equal,
            };
            match (i % 3, cmp) {
                (0, Less) => {
                    assert_eq!(r1, Err((complete * 5 + 0, Error::Char('π'))));
                    assert_eq!(r2, Err((complete * 5 + 0, Error::Char('π'))));
                }
                (1, Less) => {
                    assert_eq!(r1, Err((complete * 5 + 2, Error::Char('r'))));
                    assert_eq!(r2, Err((complete * 5, Error::Char('r'))));
                }
                (1, Equal) => {
                    assert_eq!(r1, Err((complete * 5 + 2, Error::Char('r'))));
                    assert_eq!(r2, Ok((complete * 5, vec![('π', 'r', '²'); complete])));
                }
                (2, Less) | (2, Equal) => {
                    assert_eq!(r1, Err((complete * 5 + 3, Error::Char('²'))));
                    assert_eq!(r2, Err((complete * 5 + 3, Error::Char('²'))));
                }
                (0, Equal) | (_, Greater) => {
                    let done = match max {
                        Some(max) => std::cmp::min(complete, max),
                        None => complete,
                    };
                    assert_eq!(r1, Ok((done * 5, vec![('π', 'r', '²'); done])));
                    assert_eq!(r2, Ok((done * 5, vec![('π', 'r', '²'); done])));
                }
                _ => unreachable!(),
            }
        }
    }

    t(..);
    for i in 0..10 {
        t(i);
        t(..i);
        t(i..);
        for j in i..10 {
            t(i..j);
        }
    }
}

#[test]
fn sep_by() {
    t! {
        'π'.sep_by(('r', '²')) => {
            "" => Ok((0, vec!['π'; 0])),
            "π" => Ok((2, vec!['π'; 1])),
            "πr" => Err((3, Error::Char('²'))),
            "πr²" => Err((5, Error::Char('π'))),
            "πr²π" => Ok((7, vec!['π'; 2])),
            "πr²πr" => Err((8, Error::Char('²'))),
            "πr²πr²" => Err((10, Error::Char('π'))),
            "πr²πr²π" => Ok((12, vec!['π'; 3])),
            "πr²πr²πr" => Err((13, Error::Char('²'))),
            "πr²πr²πr²" => Err((15, Error::Char('π'))),
        },
        'π'.sep_by(Try(('r', '²'))) => {
            "" => Ok((0, vec!['π'; 0])),
            "π" => Ok((2, vec!['π'; 1])),
            "πr" => Ok((2, vec!['π'; 1])),
            "πr²" => Err((5, Error::Char('π'))),
            "πr²π" => Ok((7, vec!['π'; 2])),
            "πr²πr" => Ok((7, vec!['π'; 2])),
            "πr²πr²" => Err((10, Error::Char('π'))),
            "πr²πr²π" => Ok((12, vec!['π'; 3])),
            "πr²πr²πr" => Ok((12, vec!['π'; 3])),
            "πr²πr²πr²" => Err((15, Error::Char('π'))),
        },
    }
}

#[test]
fn sep_by1() {
    t! {
        'π'.sep_by1(('r', '²')) => {
            "" => Err((0, Error::Char('π'))),
            "π" => Ok((2, vec!['π'; 1])),
            "πr" => Err((3, Error::Char('²'))),
            "πr²" => Err((5, Error::Char('π'))),
            "πr²π" => Ok((7, vec!['π'; 2])),
            "πr²πr" => Err((8, Error::Char('²'))),
            "πr²πr²" => Err((10, Error::Char('π'))),
            "πr²πr²π" => Ok((12, vec!['π'; 3])),
            "πr²πr²πr" => Err((13, Error::Char('²'))),
            "πr²πr²πr²" => Err((15, Error::Char('π'))),
        },
        'π'.sep_by1(Try(('r', '²'))) => {
            "" => Err((0, Error::Char('π'))),
            "π" => Ok((2, vec!['π'; 1])),
            "πr" => Ok((2, vec!['π'; 1])),
            "πr²" => Err((5, Error::Char('π'))),
            "πr²π" => Ok((7, vec!['π'; 2])),
            "πr²πr" => Ok((7, vec!['π'; 2])),
            "πr²πr²" => Err((10, Error::Char('π'))),
            "πr²πr²π" => Ok((12, vec!['π'; 3])),
            "πr²πr²πr" => Ok((12, vec!['π'; 3])),
            "πr²πr²πr²" => Err((15, Error::Char('π'))),
        },
    }
}

#[test]
fn position() {
    let mut p = muncher! {
        TakeWhile(char::is_whitespace),
        lo <- Position,
        word <- TakeWhile1(|ch| !ch.is_whitespace()),
        hi <- Position,
        TakeWhile(char::is_whitespace),
        (Ok((lo, word, hi)))
    };

    assert_eq!(p.parse("", 0), Err((0, Error::TakeWhile1)));
    assert_eq!(p.parse("  ", 0), Err((2, Error::TakeWhile1)));
    assert_eq!(p.parse("π", 0), Ok((2, (0, "π", 2))));
    assert_eq!(p.parse("πr²", 0), Ok((5, (0, "πr²", 5))));
    assert_eq!(p.parse("  π", 0), Ok((4, (2, "π", 4))));
    assert_eq!(p.parse("  πr²", 0), Ok((7, (2, "πr²", 7))));
    assert_eq!(p.parse("  π  ", 0), Ok((6, (2, "π", 4))));
    assert_eq!(p.parse("  πr²  ", 0), Ok((9, (2, "πr²", 7))));
    assert_eq!(p.parse("  π  h", 0), Ok((6, (2, "π", 4))));
    assert_eq!(p.parse("  πr²  h", 0), Ok((9, (2, "πr²", 7))));
}
