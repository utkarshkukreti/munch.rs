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

    fn map<F, Output>(self, f: F) -> Map<Self, F>
        where Self: Sized,
              F: FnMut(Self::Output) -> Output
    {
        Map(self, f)
    }
}

impl<F, Input, Output, Error> Parser<Input> for F
    where F: FnMut(Input, usize) -> Result<Output, Error>
{
    type Output = Output;
    type Error = Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        self(input, from)
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
            .or_else(|(from2, error)| if from == from2 {
                self.1.parse(input, from)
            } else {
                Err((from2, error))
            })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Try<A>(pub A);

impl<A, Input> Parser<Input> for Try<A>
    where A: Parser<Input>
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        self.0.parse(input, from).map_err(|(_, error)| (from, error))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Map<A, F>(pub A, pub F);

impl<A, F, Input, Output> Parser<Input> for Map<A, F>
    where A: Parser<Input>,
          F: FnMut(A::Output) -> Output
{
    type Output = Output;
    type Error = A::Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        self.0
            .parse(input, from)
            .map(|(from, output)| (from, self.1(output)))
    }
}

macro_rules! tuple_impl {
    ($(($head:ident $($tail:ident)*),)+) => {$(
        impl<$head $(,$tail)*, Input> Parser<Input> for ($head, $($tail),*)
            where $head: Parser<Input>
                  $(, $tail: Parser<Input, Error = $head::Error>)*,
                  Input: Copy
        {
            type Output = ($head::Output, $($tail::Output),*);
            type Error = $head::Error;

            #[allow(non_snake_case)]
            fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
                let (ref mut $head, $(ref mut $tail),*) = *self;
                let (from, $head) = $head.parse(input, from)?;
                $(let (from, $tail) = $tail.parse(input, from)?;)*
                Ok((from, ($head, $($tail),*)))
            }
        }
    )+}
}

tuple_impl! {
    (A),
    (A B),
    (A B C),
    (A B C D),
    (A B C D E),
    (A B C D E F),
    (A B C D E F G),
    (A B C D E F G H),
    (A B C D E F G H I),
    (A B C D E F G H I J),
    (A B C D E F G H I J K),
    (A B C D E F G H I J K L),
}
