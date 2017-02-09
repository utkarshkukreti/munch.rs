use {Parser, Result};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    Char(char),
}

impl<'a> Parser<&'a str> for char {
    type Output = char;
    type Error = Error;

    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        if input[from..].starts_with(*self) {
            Ok((from + self.len_utf8(), *self))
        } else {
            Err((from, Error::Char(*self)))
        }
    }
}
