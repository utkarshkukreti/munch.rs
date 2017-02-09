extern crate munch;

use munch::str::*;

#[macro_use]
mod t;

#[test]
fn char() {
    t! {
        'π' => {
            "" => Err((0, Error::Char('π'))),
            "π" => Ok((2, 'π')),
            "πr" => Ok((2, 'π')),
            "πr²" => Ok((2, 'π')),
        },
    }
}
