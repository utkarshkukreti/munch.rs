use munch::error::Error;
use munch::str::*;
use munch::*;

#[macro_use]
mod t;

#[test]
fn p() {
    t! {
        'π'.p() => {
            "" => Err((0, Error::Char('π'))),
            "π" => Ok((2, 'π')),
            "πr" => Ok((2, 'π')),
        },
        'π'.p() | 'r' | "²".map(|_| '2') => {
            "" => Err((0, Error::Str("²"))),
            "π" => Ok((2, 'π')),
            "πr" => Ok((2, 'π')),
            "r" => Ok((1, 'r')),
            "r²" => Ok((1, 'r')),
            "²" => Ok((2, '2')),
            "²³" => Ok((2, '2')),
        },
        'π'.p() << 'r' << '²' => {
            "" => Err((0, Error::Char('π'))),
            "π" => Err((2, Error::Char('r'))),
            "πr" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, 'π')),
            "πr²h" => Ok((5, 'π')),
        },
        'π'.p() << 'r' >> '²' => {
            "" => Err((0, Error::Char('π'))),
            "π" => Err((2, Error::Char('r'))),
            "πr" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, '²')),
            "πr²h" => Ok((5, '²')),
        },
        'π'.p() >> 'r' << '²' => {
            "" => Err((0, Error::Char('π'))),
            "π" => Err((2, Error::Char('r'))),
            "πr" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, 'r')),
            "πr²h" => Ok((5, 'r')),
        },
        'π'.p() >> 'r' >> '²' => {
            "" => Err((0, Error::Char('π'))),
            "π" => Err((2, Error::Char('r'))),
            "πr" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, '²')),
            "πr²h" => Ok((5, '²')),
        },
    }
}

#[test]
fn by_ref() {
    t! {
        'π'.by_ref() => {
            "" => Err((0, Error::Char('π'))),
            "π" => Ok((2, 'π')),
            "πr" => Ok((2, 'π')),
        },
    }
}

#[test]
fn pack() {
    #[derive(Debug, PartialEq)]
    enum Error {
        Low,
        High,
        NotANumber,
    }

    let number = |input, from, &(min, max): &(u32, u32)| {
        TakeWhile1(|ch| ch.is_digit(10))
            .map_err(|_| Error::NotANumber)
            .and_then(|str| match str.parse() {
                Ok(number) => {
                    if number < min {
                        Err(Error::Low)
                    } else if number > max {
                        Err(Error::High)
                    } else {
                        Ok(number)
                    }
                }
                Err(_) => Err(Error::NotANumber),
            })
            .parse(input, from)
    };

    t! {
        Pack(&number, (10, 100)) => {
            "" => Err((0, Error::NotANumber)),
            "π" => Err((0, Error::NotANumber)),
            "9" => Err((1, Error::Low)),
            "10" => Ok((2, 10)),
            "55" => Ok((2, 55)),
            "100" => Ok((3, 100)),
            "101" => Err((3, Error::High)),
            "4294967296" => Err((10, Error::NotANumber)),
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
fn and_skip() {
    t! {
        'π'.and_skip("r").and('²') => {
            "" => Err((0, Error::Char('π'))),
            "a" => Err((0, Error::Char('π'))),
            "π" => Err((2, Error::Str("r"))),
            "πh" => Err((2, Error::Str("r"))),
            "πr" => Err((3, Error::Char('²'))),
            "πrh" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, ('π', '²'))),
            "πr²h" => Ok((5, ('π', '²'))),
        },
    }
}

#[test]
fn skip_and() {
    t! {
        "π".skip_and('r').and('²') => {
            "" => Err((0, Error::Str("π"))),
            "a" => Err((0, Error::Str("π"))),
            "π" => Err((2, Error::Char('r'))),
            "πh" => Err((2, Error::Char('r'))),
            "πr" => Err((3, Error::Char('²'))),
            "πrh" => Err((3, Error::Char('²'))),
            "πr²" => Ok((5, ('r', '²'))),
            "πr²h" => Ok((5, ('r', '²'))),
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
fn r#try() {
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
        Munch(munch::error::Error<'static>),
    }

    t! {
        "foo".map_err(Error::Munch) => {
            "" => Err((0, Error::Munch(munch::error::Error::Str("foo")))),
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
    let mut pi = |input: &str, from: usize| {
        if input[from..].starts_with("pi") {
            Ok((from + 2, 'π'))
        } else {
            Err((from, ()))
        }
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
        Munch(munch::error::Error<'static>),
        ParseIntError(std::num::ParseIntError),
    }

    t! {
        TakeWhile1(|ch| '0' <= ch && ch <= '9')
            .map_err(Error::Munch)
            .and_then(|str: &str| str.parse::<u8>().map_err(Error::ParseIntError)) => {
            "" => Err((0, Error::Munch(munch::error::Error::TakeWhile1))),
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
fn bind() {
    t! {
        Any.bind(|ch| ch) => {
            "" => Err((0, Error::Any)),
            "π" => Err((2, Error::Char('π'))),
            "πr" => Err((2, Error::Char('π'))),
            "ππ" => Ok((4, 'π')),
            "ππr" => Ok((4, 'π')),
        },
        Any.bind(|a| std::char::from_u32(a as u32 + 1).unwrap().map(move |b| (a, b))) => {
            "" => Err((0, Error::Any)),
            "π" => Err((2, Error::Char('ρ'))),
            "πr" => Err((2, Error::Char('ρ'))),
            "πρ" => Ok((4, ('π', 'ρ'))),
            "πρς" => Ok((4, ('π', 'ρ'))),
        },
    }
}

#[test]
fn guard() {
    #[derive(Debug, PartialEq)]
    enum Error {
        NotANumber,
        Huge,
        Negative,
        Odd,
    }

    let mut parser = muncher! {
        str <- Capture((Optional('-'), TakeWhile1(|ch| ch.is_digit(10))))
            .map_err(|_| Error::NotANumber),
        let n = str.parse().unwrap(),
        Guard(|| n <= 255, || Error::Huge),
        Guard(|| n >= 0, || Error::Negative),
        Guard(|| n % 2 == 0, || Error::Odd),
        (Ok(n))
    };

    t! {
        parser => {
            "0" => Ok((1, 0)),
            "12" => Ok((2, 12)),
            "124" => Ok((3, 124)),
            "abc" => Err((0, Error::NotANumber)),
            "1234" => Err((4, Error::Huge)),
            "-12" => Err((3, Error::Negative)),
            "123" => Err((3, Error::Odd)),
        },
    }
}

#[test]
fn succeed() {
    t! {
        Succeed::<_, ()>(|| '✓') => {
            "" => Ok((0, '✓')),
            "π" => Ok((0, '✓')),
        },
    }
}

#[test]
fn fail() {
    t! {
        Fail(|| '✗') => {
            "" => Err((0, '✗')),
            "π" => Err((0, '✗')),
        },
    }
}

#[test]
fn repeat() {
    fn t<R: Range>(range: R) {
        use std::cmp::Ordering::*;
        let (min, max) = (range.min(), range.max());
        let mut p1 = ('π', 'r', '²').repeat(range.clone());
        let mut p2 = (Try(('π', 'r')), '²')
            .map(|((a, b), c)| (a, b, c))
            .repeat(range);
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
                    assert_eq!(r1, Err((complete * 5, Error::Char('π'))));
                    assert_eq!(r2, Err((complete * 5, Error::Char('π'))));
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
fn collect() {
    use std::collections::HashMap;

    let mut p = (Any.p() << ':', Any).repeat(..).collect();

    t! {
        p => {
            "" => Ok((0, HashMap::new())),
            "π:r²:h" => Ok((8, [('π', 'r'), ('²', 'h')].iter().cloned().collect())),
        },
    }
}

#[test]
fn fold() {
    fn t<R: Range>(range: R) {
        let mut p1 = ('π', 'r', '²').repeat(range.clone());
        let mut p1f = p1.clone().fold(Vec::new, |mut acc, x| {
            acc.push(x);
            acc
        });

        let flatten = |((a, b), c)| (a, b, c);
        let mut p2 = (Try(('π', 'r')), '²').map(&flatten).repeat(range);
        let mut p2f = p2.clone().fold(Vec::new, |mut acc, x| {
            acc.push(x);
            acc
        });

        for i in 0..36 {
            let string = "πr²".chars().cycle().take(i).collect::<String>();
            assert_eq!(p1.parse(&string, 0), p1f.parse(&string, 0));
            assert_eq!(p2.parse(&string, 0), p2f.parse(&string, 0));
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
fn join() {
    fn t<R: Range>(range: R) {
        use std::cmp::Ordering::*;
        let (min, max) = (range.min(), range.max());
        let mut p1 = 'π'.repeat(range.clone()).join(('r', '²'));
        let mut p2 = 'π'.repeat(range).join(Try(('r', '²')));
        for i in 0..36 {
            let string = "πr²".chars().cycle().take(i).collect::<String>();
            let r1 = p1.parse(&string, 0);
            let r2 = p2.parse(&string, 0);
            let complete = (i + 2) / 3;
            let cmp = match max {
                _ if complete < min => Less,
                Some(max) if complete >= max => Greater,
                _ => Equal,
            };

            if i == 0 || (min, max) == (0, Some(0)) {
                match cmp {
                    Less => {
                        assert_eq!(r1, Err((0, Error::Char('π'))));
                        assert_eq!(r2, Err((0, Error::Char('π'))));
                    }
                    Equal | Greater => {
                        assert_eq!(r1, Ok((0, vec![])));
                        assert_eq!(r2, Ok((0, vec![])));
                    }
                }
            } else {
                match ((i - 1) % 3, cmp) {
                    (0, Less) => {
                        assert_eq!(r1, Err((complete * 5 - 3, Error::Char('r'))));
                        assert_eq!(r2, Err((complete * 5 - 3, Error::Char('r'))));
                    }
                    (1, Less) => {
                        assert_eq!(r1, Err((complete * 5 - 2, Error::Char('²'))));
                        assert_eq!(r2, Err((complete * 5 - 3, Error::Char('²'))));
                    }
                    (1, Equal) => {
                        assert_eq!(r1, Err((complete * 5 - 2, Error::Char('²'))));
                        assert_eq!(r2, Ok((complete * 5 - 3, vec!['π'; complete])));
                    }
                    (2, Less) | (2, Equal) => {
                        assert_eq!(r1, Err((complete * 5, Error::Char('π'))));
                        assert_eq!(r2, Err((complete * 5, Error::Char('π'))));
                    }
                    (0, Equal) | (_, Greater) => {
                        let done = match max {
                            Some(max) => std::cmp::min(complete, max),
                            None => complete,
                        };
                        assert_eq!(r1, Ok((done * 5 - 3, vec!['π'; done])));
                        assert_eq!(r2, Ok((done * 5 - 3, vec!['π'; done])));
                    }
                    _ => unreachable!(),
                }
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
fn join_fold() {
    fn t<R: Range>(range: R) {
        let first = |mut vec: Vec<_>, output| {
            vec.push(output);
            vec
        };
        let rest = |mut vec: Vec<_>, separator, output| {
            assert_eq!(separator, ('r', '²'));
            vec.push(output);
            vec
        };
        let mut p1 = 'π'.repeat(range.clone()).join(('r', '²'));
        let mut p1f = p1.clone().fold(Vec::new, &first, &rest);
        let mut p2 = 'π'.repeat(range).join(Try(('r', '²')));
        let mut p2f = p2.clone().fold(Vec::new, &first, &rest);
        for i in 0..36 {
            let string = "πr²".chars().cycle().take(i).collect::<String>();
            assert_eq!(p1.parse(&string, 0), p1f.parse(&string, 0));
            assert_eq!(p2.parse(&string, 0), p2f.parse(&string, 0));
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
