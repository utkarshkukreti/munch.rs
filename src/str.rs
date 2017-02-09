use {Parser, Result};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error<'a> {
    Char(char),
    Str(&'a str),
    Satisfy,
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Satisfy<F>(pub F) where F: FnMut(char) -> bool;

impl<'a, F> Parser<&'a str> for Satisfy<F>
    where F: FnMut(char) -> bool
{
    type Output = char;
    type Error = Error<'static>;

    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        if let Some(char) = input[from..].chars().next() {
            if self.0(char) {
                return Ok((from + char.len_utf8(), char));
            }
        }
        Err((from, Error::Satisfy))
    }
}
