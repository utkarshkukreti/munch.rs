use std::collections::HashMap;

use munch::{Optional, Parser};

pub type Value<'a> = HashMap<&'a str, HashMap<&'a str, &'a str>>;

pub fn parse(str: &str) -> Result<Value, (usize, munch::error::Error<'static>)> {
    use munch::str::*;

    let s = || TakeWhile(|ch| ch == ' ' || ch == '\t');
    let ws = TakeWhile(char::is_whitespace);

    let header = '['.p() >> TakeWhile1(|ch| ch != ']') << ']' << ws;

    let comment = ';'.p() >> TakeWhile(|ch| ch != '\n') << ws;

    let key = TakeWhile1(char::is_alphanumeric);
    let value = TakeWhile(|ch| ch != '\n' && ch != ';');

    let kv = (
        key.p() << s() << '=' << s(),
        value.p() << s() << Optional(comment) << ws,
    );
    let kvs = kv.repeat(..).collect();

    let section = (header, kvs);
    let sections = section.repeat(..).collect();

    (ws.p() >> sections << End)
        .parse(str, 0)
        .map(|(_, output)| output)
}

pub static EXAMPLE: &str = "
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
