use {Parser, Result};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error<'a> {
    Char(char),
    Str(&'a str),
}

impl<'a> Parser<&'a str> for char {
    type Output = char;
    type Error = Error<'static>;

    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        if input[from..].starts_with(*self) {
            Ok((from + self.len_utf8(), *self))
        } else {
            Err((from, Error::Char(*self)))
        }
    }
}

impl<'a, 'tmp> Parser<&'a str> for &'tmp str {
    type Output = &'a str;
    type Error = Error<'tmp>;

    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        if input[from..].starts_with(*self) {
            let to = from + self.len();
            Ok((to, &input[from..to]))
        } else {
            Err((from, Error::Str(*self)))
        }
    }
}
