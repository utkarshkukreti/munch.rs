extern crate munch;

use std::collections::HashMap;

use munch::{Optional, P, Parser};

pub type Value<'a> = HashMap<&'a str, HashMap<&'a str, &'a str>>;

pub fn parse(str: &str) -> Result<Value, (usize, munch::str::Error<'static>)> {
    use munch::str::*;

    let s = || P(TakeWhile(|ch| ch == ' ' || ch == '\t'));
    let ws = || P(TakeWhile(char::is_whitespace));

    let header = P('[') >> TakeWhile1(|ch| ch != ']') << ']' << ws();

    let comment = P(';') >> TakeWhile(|ch| ch != '\n') << ws();

    let key = P(TakeWhile1(char::is_alphanumeric));
    let value = P(TakeWhile(|ch| ch != '\n' && ch != ';'));

    let kv = (key << s() << '=' << s(), value << s() << Optional(comment) << ws());
    let kvs = kv.repeat(..).collect();

    let section = (header, kvs);
    let sections = section.repeat(..).collect();

    match (ws() >> sections << End).parse(str, 0) {
        Ok((_, output)) => Ok(output),
        Err((from, error)) => Err((from, error)),
    }
}

pub static EXAMPLE: &'static str = "
[owner]
name=John Doe
organization=Acme Widgets Inc.

[database]
server=192.0.2.62
port=143
file=payroll.dat
";

#[cfg(not(test))]
pub fn main() {
    println!("{:?}", parse(EXAMPLE));
}
