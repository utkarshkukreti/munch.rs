use {Parser, Result, ascii};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error<'a> {
    Ascii(ascii::Error),
    Char(char),
    Str(&'a str),
    Satisfy,
    TakeWhile1,
    Any,
    End,
}

impl<'a> Parser<&'a str> for char {
    type Output = char;
    type Error = Error<'static>;

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        if let Some(char) = input[from..].chars().next() {
            if self.0(char) {
                return Ok((from + char.len_utf8(), char));
            }
        }
        Err((from, Error::Satisfy))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeWhile<F>(pub F) where F: FnMut(char) -> bool;

impl<'a, F> Parser<&'a str> for TakeWhile<F>
    where F: FnMut(char) -> bool
{
    type Output = &'a str;
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        let mut chars = input[from..].chars();
        let to = match chars.by_ref().skip_while(|&char| self.0(char)).next() {
            Some(char) => input.len() - chars.as_str().len() - char.len_utf8(),
            None => input.len(),
        };
        Ok((to, &input[from..to]))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeWhile1<F>(pub F) where F: FnMut(char) -> bool;

impl<'a, F> Parser<&'a str> for TakeWhile1<F>
    where F: FnMut(char) -> bool
{
    type Output = &'a str;
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        match TakeWhile(&mut self.0).parse(input, from) {
            Ok((_, "")) => Err((from, Error::TakeWhile1)),
            otherwise => otherwise,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Capture<P>(pub P);

impl<'a, P> Parser<&'a str> for Capture<P>
    where P: Parser<&'a str>
{
    type Output = &'a str;
    type Error = P::Error;

    #[inline(always)]
    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        let (to, _) = self.0.parse(input, from)?;
        Ok((to, &input[from..to]))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Any;

impl<'a> Parser<&'a str> for Any {
    type Output = char;
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        if let Some(char) = input[from..].chars().next() {
            Ok((from + char.len_utf8(), char))
        } else {
            Err((from, Error::Any))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct End;

impl<'a> Parser<&'a str> for End {
    type Output = ();
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        if input.len() == from {
            Ok((from, ()))
        } else {
            Err((from, Error::End))
        }
    }
}
