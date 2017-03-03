mod mac;

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

    fn and_skip<B>(self, b: B) -> AndSkip<Self, B>
        where Self: Sized,
              B: Parser<Input, Error = Self::Error>
    {
        AndSkip(self, b)
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

    fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>
        where Self: Sized,
              F: FnMut(Self::Error) -> Error
    {
        MapErr(self, f)
    }

    fn and_then<F, Output>(self, f: F) -> AndThen<Self, F>
        where Self: Sized,
              F: FnMut(Self::Output) -> std::result::Result<Output, Self::Error>
    {
        AndThen(self, f)
    }

    fn optional(self) -> Optional<Self>
        where Self: Sized
    {
        Optional(self)
    }

    fn repeat<R>(self, range: R) -> Repeat<Self, R>
        where Self: Sized,
              R: Range
    {
        Repeat(self, range)
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

impl<A, B> std::ops::BitOr<B> for P<A> {
    type Output = P<Or<A, B>>;

    fn bitor(self, b: B) -> Self::Output {
        P(Or(self.0, b))
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
pub struct AndSkip<A, B>(pub A, pub B);

impl<A, B, Input> Parser<Input> for AndSkip<A, B>
    where A: Parser<Input>,
          B: Parser<Input, Error = A::Error>,
          Input: Copy
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        let (from, a) = self.0.parse(input, from)?;
        let (from, _) = self.1.parse(input, from)?;
        Ok((from, a))
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MapErr<A, F>(pub A, pub F);

impl<A, F, Input, Error> Parser<Input> for MapErr<A, F>
    where A: Parser<Input>,
          F: FnMut(A::Error) -> Error
{
    type Output = A::Output;
    type Error = Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        self.0
            .parse(input, from)
            .map_err(|(from, error)| (from, self.1(error)))
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AndThen<A, F>(pub A, pub F);

impl<A, F, Input, Output> Parser<Input> for AndThen<A, F>
    where A: Parser<Input>,
          F: FnMut(A::Output) -> std::result::Result<Output, A::Error>
{
    type Output = Output;
    type Error = A::Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        self.0.parse(input, from).and_then(|(from, output)| match self.1(output) {
            Ok(output) => Ok((from, output)),
            Err(error) => Err((from, error)),
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Optional<A>(pub A);

impl<A, Input> Parser<Input> for Optional<A>
    where A: Parser<Input>
{
    type Output = Option<A::Output>;
    type Error = A::Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        match self.0.parse(input, from) {
            Ok((from, output)) => Ok((from, Some(output))),
            Err((from2, _)) if from == from2 => Ok((from, None)),
            Err((from, error)) => Err((from, error)),
        }
    }
}

pub trait Range {
    fn min(&self) -> usize;
    fn max(&self) -> Option<usize>;
}

impl Range for usize {
    fn min(&self) -> usize {
        *self
    }

    fn max(&self) -> Option<usize> {
        Some(*self)
    }
}

impl Range for std::ops::RangeFull {
    fn min(&self) -> usize {
        0
    }

    fn max(&self) -> Option<usize> {
        None
    }
}

impl Range for std::ops::RangeTo<usize> {
    fn min(&self) -> usize {
        0
    }

    fn max(&self) -> Option<usize> {
        Some(self.end)
    }
}

impl Range for std::ops::RangeFrom<usize> {
    fn min(&self) -> usize {
        self.start
    }

    fn max(&self) -> Option<usize> {
        None
    }
}

impl Range for std::ops::Range<usize> {
    fn min(&self) -> usize {
        self.start
    }

    fn max(&self) -> Option<usize> {
        Some(self.end)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Repeat<A, R: Range>(pub A, pub R);

impl<A, R> Repeat<A, R>
    where R: Range
{
    pub fn join<B, Input>(self, b: B) -> Join<A, B, R>
        where A: Parser<Input>,
              B: Parser<Input, Error = A::Error>
    {
        Join(self.0, b, self.1)
    }
}

impl<A, R, Input> Parser<Input> for Repeat<A, R>
    where A: Parser<Input>,
          R: Range,
          Input: Copy
{
    type Output = Vec<A::Output>;
    type Error = A::Error;

    fn parse(&mut self, input: Input, mut from: usize) -> Result<Self::Output, Self::Error> {
        let (min, max) = (self.1.min(), self.1.max());
        let mut vec = vec![];
        loop {
            if Some(vec.len()) == max {
                return Ok((from, vec));
            }

            match self.0.parse(input, from) {
                Ok((from2, output)) => {
                    from = from2;
                    vec.push(output);
                }
                Err((from2, error)) => {
                    return if from == from2 && vec.len() >= min {
                        Ok((from2, vec))
                    } else {
                        Err((from2, error))
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Join<A, B, R: Range>(pub A, pub B, pub R);

impl<A, B, R, Input> Parser<Input> for Join<A, B, R>
    where A: Parser<Input>,
          B: Parser<Input, Error = A::Error>,
          R: Range,
          Input: Copy
{
    type Output = Vec<A::Output>;
    type Error = A::Error;

    fn parse(&mut self, input: Input, mut from: usize) -> Result<Self::Output, Self::Error> {
        let (min, max) = (self.2.min(), self.2.max());
        let mut vec = vec![];

        if max == Some(0) {
            return Ok((from, vec));
        }

        match self.0.parse(input, from) {
            Ok((from2, output)) => {
                from = from2;
                vec.push(output);
            }
            Err((from2, error)) => {
                return if min == 0 {
                    Ok((from2, vec))
                } else {
                    Err((from2, error))
                }
            }
        }

        loop {
            if Some(vec.len()) == max {
                return Ok((from, vec));
            }

            match self.1.parse(input, from) {
                Ok((from2, _)) => from = from2,
                Err((from2, error)) => {
                    return if from == from2 && vec.len() >= min {
                        Ok((from2, vec))
                    } else {
                        Err((from2, error))
                    }
                }
            }

            let (from2, output) = self.0.parse(input, from)?;
            from = from2;
            vec.push(output);
        }
    }
}

#[allow(non_snake_case)]
pub fn Position<Input, Error>(_input: Input, from: usize) -> Result<usize, Error> {
    Ok((from, from))
}
