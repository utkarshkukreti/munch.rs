use std;

use {Parser, Result, str};
use error::{Ascii, Error};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Satisfy<F>(pub F) where F: FnMut(u8) -> bool;

impl<'a, F, Input> Parser<&'a Input> for Satisfy<F>
    where F: FnMut(u8) -> bool,
          Input: AsRef<[u8]> + ?Sized
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
pub struct TakeWhile<F>(pub F) where F: FnMut(u8) -> bool;

impl<'a, F, Input> Parser<&'a Input> for TakeWhile<F>
    where F: FnMut(u8) -> bool,
          Input: AsRef<[u8]> + ?Sized
{
    type Output = &'a str;
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a Input, from: usize) -> Result<Self::Output, Self::Error> {
        let input = input.as_ref();
        let to = match input[from..].iter().position(|&u8| u8 > 0x7F || !self.0(u8)) {
            Some(position) => from + position,
            None => input.len(),
        };

        // All the bytes in `from..to` must be `<= 0x7F`, and therefore valid UTF-8, making the
        // following unchecked conversion safe.
        debug_assert!(std::str::from_utf8(&input[from..to]).is_ok());
        Ok((to, unsafe { std::str::from_utf8_unchecked(&input[from..to]) }))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeWhile1<F>(pub F) where F: FnMut(u8) -> bool;

impl<'a, F, Input> Parser<&'a Input> for TakeWhile1<F>
    where F: FnMut(u8) -> bool,
          Input: AsRef<[u8]> + ?Sized
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
