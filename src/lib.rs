pub mod str;

pub type Result<Output, Error> = std::result::Result<(usize, Output), (usize, Error)>;

pub trait Parser<Input> {
    type Output;
    type Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error>;

    fn and<B>(self, b: B) -> And<Self, B>
        where Self: Sized,
              B: Parser<Input, Error = Self::Error>
    {
        And(self, b)
    }

    fn or<B>(self, b: B) -> Or<Self, B>
        where Self: Sized,
              B: Parser<Input, Output = Self::Output, Error = Self::Error>
    {
        Or(self, b)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct P<A>(pub A);

impl<A, Input> Parser<Input> for P<A>
    where A: Parser<Input>
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        self.0.parse(input, from)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct And<A, B>(pub A, pub B);

impl<A, B, Input> Parser<Input> for And<A, B>
    where A: Parser<Input>,
          B: Parser<Input, Error = A::Error>,
          Input: Copy
{
    type Output = (A::Output, B::Output);
    type Error = A::Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        let (from, a) = self.0.parse(input, from)?;
        let (from, b) = self.1.parse(input, from)?;
        Ok((from, (a, b)))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Or<A, B>(pub A, pub B);

impl<A, B, Input> Parser<Input> for Or<A, B>
    where A: Parser<Input>,
          B: Parser<Input, Output = A::Output, Error = A::Error>,
          Input: Copy
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        self.0
            .parse(input, from)
            .or_else(|_| self.1.parse(input, from))
    }
}
