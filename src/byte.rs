use {Parser, Result};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    Byte(u8),
}

impl<'a> Parser<&'a [u8]> for u8 {
    type Output = u8;
    type Error = Error;

    #[inline(always)]
    fn parse(&mut self, input: &'a [u8], from: usize) -> Result<Self::Output, Self::Error> {
        if input.get(from) == Some(self) {
            Ok((from + 1, *self))
        } else {
            Err((from, Error::Byte(*self)))
        }
    }
}
