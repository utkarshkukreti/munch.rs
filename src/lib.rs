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

    fn many(self) -> Many<Self>
        where Self: Sized
    {
        Many(self)
    }

    fn many1(self) -> Many1<Self>
        where Self: Sized
    {
        Many1(self)
    }

    fn sep_by<B>(self, b: B) -> SepBy<Self, B>
        where Self: Sized,
              B: Parser<Input, Error = Self::Error>
    {
        SepBy(self, b)
    }

    fn sep_by1<B>(self, b: B) -> SepBy1<Self, B>
        where Self: Sized,
              B: Parser<Input, Error = Self::Error>
    {
        SepBy1(self, b)
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Many<A>(pub A);

impl<A, Input> Parser<Input> for Many<A>
    where A: Parser<Input>,
          Input: Copy
{
    type Output = Vec<A::Output>;
    type Error = A::Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        Many1(|input, from| self.0.parse(input, from))
            .parse(input, from)
            .or_else(|(from2, error)| if from == from2 {
                Ok((from, Vec::new()))
            } else {
                Err((from2, error))
            })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Many1<A>(pub A);

impl<A, Input> Parser<Input> for Many1<A>
    where A: Parser<Input>,
          Input: Copy
{
    type Output = Vec<A::Output>;
    type Error = A::Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        let (mut from, output) = self.0.parse(input, from)?;
        let mut vec = vec![output];
        loop {
            match self.0.parse(input, from) {
                Ok((from2, output)) => {
                    from = from2;
                    vec.push(output);
                }
                Err((from2, error)) => {
                    return if from == from2 {
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
pub struct SepBy<A, B>(pub A, pub B);

impl<A, B, Input> Parser<Input> for SepBy<A, B>
    where A: Parser<Input>,
          B: Parser<Input, Error = A::Error>,
          Input: Copy
{
    type Output = Vec<A::Output>;
    type Error = A::Error;

    fn parse(&mut self, input: Input, mut from: usize) -> Result<Self::Output, Self::Error> {
        let mut vec = Vec::new();

        match self.0.parse(input, from) {
            Ok((from2, output)) => {
                from = from2;
                vec.push(output);
            }
            Err((from2, error)) => {
                return if from == from2 {
                    Ok((from2, vec))
                } else {
                    Err((from2, error))
                }
            }
        }

        loop {
            match self.1.parse(input, from) {
                Ok((from2, _)) => from = from2,
                Err((from2, error)) => {
                    return if from == from2 {
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SepBy1<A, B>(pub A, pub B);

impl<A, B, Input> Parser<Input> for SepBy1<A, B>
    where A: Parser<Input>,
          B: Parser<Input, Error = A::Error>,
          Input: Copy
{
    type Output = Vec<A::Output>;
    type Error = A::Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        let (mut from, output) = self.0.parse(input, from)?;
        let mut vec = vec![output];

        loop {
            match self.1.parse(input, from) {
                Ok((from2, _)) => from = from2,
                Err((from2, error)) => {
                    return if from == from2 {
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
