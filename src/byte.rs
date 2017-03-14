use {Parser, Result};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error<'a> {
    Byte(u8),
    Bytes(&'a [u8]),
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
