use {Parser, Result, str};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    Satisfy,
    TakeWhile1,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Satisfy<F>(pub F) where F: FnMut(u8) -> bool;

impl<'a, F> Parser<&'a str> for Satisfy<F>
    where F: FnMut(u8) -> bool
{
    type Output = u8;
    type Error = str::Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        if let Some(&u8) = input.as_bytes().get(from) {
            if u8 <= 0x7F && self.0(u8) {
                return Ok((from + 1, u8));
            }
        }
        Err((from, str::Error::Ascii(Error::Satisfy)))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeWhile<F>(pub F) where F: FnMut(u8) -> bool;

impl<'a, F> Parser<&'a str> for TakeWhile<F>
    where F: FnMut(u8) -> bool
{
    type Output = &'a str;
    type Error = str::Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        let to = match input[from..].as_bytes().iter().position(|&u8| u8 > 0x7F || !self.0(u8)) {
            Some(position) => from + position,
            None => input.len(),
        };
        Ok((to, &input[from..to]))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeWhile1<F>(pub F) where F: FnMut(u8) -> bool;

impl<'a, F> Parser<&'a str> for TakeWhile1<F>
    where F: FnMut(u8) -> bool
{
    type Output = &'a str;
    type Error = str::Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a str, from: usize) -> Result<Self::Output, Self::Error> {
        match TakeWhile(&mut self.0).parse(input, from) {
            Ok((_, "")) => Err((from, str::Error::Ascii(Error::TakeWhile1))),
            otherwise => otherwise,
        }
    }
}
