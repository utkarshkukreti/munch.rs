use {Parser, Result};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error<'a> {
    Byte(u8),
    Bytes(&'a [u8]),
    Satisfy,
    TakeWhile1,
    Any,
}

impl<'a> Parser<&'a [u8]> for u8 {
    type Output = u8;
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a [u8], from: usize) -> Result<Self::Output, Self::Error> {
        if input.get(from) == Some(self) {
            Ok((from + 1, *self))
        } else {
            Err((from, Error::Byte(*self)))
        }
    }
}

impl<'a, 'tmp> Parser<&'a [u8]> for &'tmp [u8] {
    type Output = &'a [u8];
    type Error = Error<'tmp>;

    #[inline(always)]
    fn parse(&mut self, input: &'a [u8], from: usize) -> Result<Self::Output, Self::Error> {
        if input[from..].starts_with(*self) {
            let to = from + self.len();
            Ok((to, &input[from..to]))
        } else {
            Err((from, Error::Bytes(*self)))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Satisfy<F>(pub F) where F: FnMut(u8) -> bool;

impl<'a, F> Parser<&'a [u8]> for Satisfy<F>
    where F: FnMut(u8) -> bool
{
    type Output = u8;
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a [u8], from: usize) -> Result<Self::Output, Self::Error> {
        if let Some(&byte) = input.get(from) {
            if self.0(byte) {
                return Ok((from + 1, byte));
            }
        }
        Err((from, Error::Satisfy))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeWhile<F>(pub F) where F: FnMut(u8) -> bool;

impl<'a, F> Parser<&'a [u8]> for TakeWhile<F>
    where F: FnMut(u8) -> bool
{
    type Output = &'a [u8];
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a [u8], from: usize) -> Result<Self::Output, Self::Error> {
        let to = match input[from..].iter().position(|&byte| !self.0(byte)) {
            Some(position) => from + position,
            None => input.len(),
        };
        Ok((to, &input[from..to]))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeWhile1<F>(pub F) where F: FnMut(u8) -> bool;

impl<'a, F> Parser<&'a [u8]> for TakeWhile1<F>
    where F: FnMut(u8) -> bool
{
    type Output = &'a [u8];
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a [u8], from: usize) -> Result<Self::Output, Self::Error> {
        match TakeWhile(&mut self.0).parse(input, from) {
            Ok((_, b"")) => Err((from, Error::TakeWhile1)),
            otherwise => otherwise,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Capture<P>(pub P);

impl<'a, P> Parser<&'a [u8]> for Capture<P>
    where P: Parser<&'a [u8]>
{
    type Output = &'a [u8];
    type Error = P::Error;

    #[inline(always)]
    fn parse(&mut self, input: &'a [u8], from: usize) -> Result<Self::Output, Self::Error> {
        let (to, _) = self.0.parse(input, from)?;
        Ok((to, &input[from..to]))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Any;

impl<'a> Parser<&'a [u8]> for Any {
    type Output = u8;
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a [u8], from: usize) -> Result<Self::Output, Self::Error> {
        if let Some(&byte) = input.get(from) {
            Ok((from + 1, byte))
        } else {
            Err((from, Error::Any))
        }
    }
}
