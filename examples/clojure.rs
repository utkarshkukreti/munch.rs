extern crate munch;

use munch::{Optional, P, Parser, Try};

pub static EXAMPLE: &'static str = "
(defn sum [xs]
  (reduce + 0 xs))
(println (sum [3 -14159 +26535 -89793 -23846]))
(->> (range)
     (map (fn [x] (* x x)))
     (filter even?)
     (take 10)
     (reduce +))
";

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    Integer(i64),
    Symbol(&'a str),
    List(Vec<Value<'a>>),
    Vector(Vec<Value<'a>>),
}

pub fn value(str: &str, from: usize) -> munch::Result<Value, munch::str::Error<'static>> {
    use munch::str::*;

    let ws = || P(TakeWhile(char::is_whitespace));

    let integer = P(Capture(Try((Optional('-'.or('+')), TakeWhile1(|ch| ch.is_digit(10)))))
        .map(|str| Value::Integer(str.parse().unwrap())));

    let is_symbol_head = |ch| match ch {
        'a'...'z' | 'A'...'Z' | '.' | '*' | '+' | '!' | '-' | '_' | '?' | '$' | '%' | '&' |
        '=' | '<' | '>' => true,
        _ => false,
    };
    let is_symbol_tail = |ch| {
        is_symbol_head(ch) ||
        match ch {
            '0'...'9' | ':' | '#' => true,
            _ => false,
        }
    };
    let symbol = Capture((Satisfy(&is_symbol_head), TakeWhile(is_symbol_tail))).map(Value::Symbol);

    let list = P('(') >> ws() >> P(value).repeat(..).map(Value::List) << P(')');
    let vector = P('[') >> ws() >> P(value).repeat(..).map(Value::Vector) << P(']');

    ((integer | symbol | list | vector) << ws()).parse(str, from)
}

pub fn parse(str: &str) -> Result<Vec<Value>, (usize, munch::str::Error<'static>)> {
    use munch::str::*;
    match (P(TakeWhile(char::is_whitespace)) >> value.repeat(..) << End).parse(str, 0) {
        Ok((_, output)) => Ok(output),
        Err((from, error)) => Err((from, error)),
    }
}

#[cfg(not(test))]
pub fn main() {
    use std::io::prelude::*;
    use Value::*;

    assert_eq!(parse(EXAMPLE),
               Ok(vec![List(vec![Symbol("defn"),
                                 Symbol("sum"),
                                 Vector(vec![Symbol("xs")]),
                                 List(vec![Symbol("reduce"),
                                           Symbol("+"),
                                           Integer(0),
                                           Symbol("xs")])]),
                       List(vec![Symbol("println"),
                                 List(vec![Symbol("sum"),
                                           Vector(vec![Integer(3),
                                                       Integer(-14159),
                                                       Integer(26535),
                                                       Integer(-89793),
                                                       Integer(-23846)])])]),
                       List(vec![Symbol("->>"),
                                 List(vec![Symbol("range")]),
                                 List(vec![Symbol("map"),
                                           List(vec![Symbol("fn"),
                                                     Vector(vec![Symbol("x")]),
                                                     List(vec![Symbol("*"),
                                                               Symbol("x"),
                                                               Symbol("x")])])]),
                                 List(vec![Symbol("filter"), Symbol("even?")]),
                                 List(vec![Symbol("take"), Integer(10)]),
                                 List(vec![Symbol("reduce"), Symbol("+")])])]));

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
