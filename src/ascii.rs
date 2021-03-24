use crate::error::{Ascii, Error};
use crate::{str, Parser, Result};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Satisfy<F>(pub F)
where
    F: FnMut(u8) -> bool;

impl<'a, F, Input> Parser<&'a Input> for Satisfy<F>
where
    F: FnMut(u8) -> bool,
    Input: AsRef<[u8]> + ?Sized,
{
    type Output = u8;
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a Input, from: usize) -> Result<Self::Output, Self::Error> {
        let input = input.as_ref();
        if let Some(&u8) = input.get(from) {
            if u8 <= 0x7F && self.0(u8) {
                return Ok((from + 1, u8));
            }
        }
        Err((from, Error::Ascii(Ascii::Satisfy)))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeWhile<F>(pub F)
where
    F: FnMut(u8) -> bool;

impl<'a, F, Input> Parser<&'a Input> for TakeWhile<F>
where
    F: FnMut(u8) -> bool,
    Input: AsRef<[u8]> + ?Sized,
{
    type Output = &'a str;
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a Input, from: usize) -> Result<Self::Output, Self::Error> {
        let input = input.as_ref();
        let to = match input[from..]
            .iter()
            .position(|&u8| u8 > 0x7F || !self.0(u8))
        {
            Some(position) => from + position,
            None => input.len(),
        };

        Ok((to, std::str::from_utf8(&input[from..to]).unwrap()))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeWhile1<F>(pub F)
where
    F: FnMut(u8) -> bool;

impl<'a, F, Input> Parser<&'a Input> for TakeWhile1<F>
where
    F: FnMut(u8) -> bool,
    Input: AsRef<[u8]> + ?Sized,
{
    type Output = &'a str;
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a Input, from: usize) -> Result<Self::Output, Self::Error> {
        match TakeWhile(&mut self.0).parse(input, from) {
            Ok((_, "")) => Err((from, Error::Ascii(Ascii::TakeWhile1))),
            otherwise => otherwise,
        }
    }
}
