pub mod str;

pub type Result<Output, Error> = std::result::Result<(usize, Output), (usize, Error)>;

pub trait Parser<Input> {
    type Output;
    type Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error>;
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
