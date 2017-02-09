pub mod str;

pub type Result<Output, Error> = std::result::Result<(usize, Output), (usize, Error)>;

pub trait Parser<Input> {
    type Output;
    type Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error>;
}
