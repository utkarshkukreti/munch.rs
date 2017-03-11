mod mac;

pub mod str;

pub type Result<Output, Error> = std::result::Result<(usize, Output), (usize, Error)>;

pub trait Parser<Input> {
    type Output;
    type Error;

    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error>;

    #[inline(always)]
    fn and<B>(self, b: B) -> P<And<Self, B>>
        where Self: Sized,
              B: Parser<Input, Error = Self::Error>
    {
        P(And(self, b))
    }

    #[inline(always)]
    fn and_skip<B>(self, b: B) -> P<AndSkip<Self, B>>
        where Self: Sized,
              B: Parser<Input, Error = Self::Error>
    {
        P(AndSkip(self, b))
    }

    #[inline(always)]
    fn skip_and<B>(self, b: B) -> P<SkipAnd<Self, B>>
        where Self: Sized,
              B: Parser<Input, Error = Self::Error>
    {
        P(SkipAnd(self, b))
    }

    #[inline(always)]
    fn or<B>(self, b: B) -> P<Or<Self, B>>
        where Self: Sized,
              B: Parser<Input, Output = Self::Output, Error = Self::Error>
    {
        P(Or(self, b))
    }

    #[inline(always)]
    fn map<F, Output>(self, f: F) -> P<Map<Self, F>>
        where Self: Sized,
              F: FnMut(Self::Output) -> Output
    {
        P(Map(self, f))
    }

    #[inline(always)]
    fn map_err<F, Error>(self, f: F) -> P<MapErr<Self, F>>
        where Self: Sized,
              F: FnMut(Self::Error) -> Error
    {
        P(MapErr(self, f))
    }

    #[inline(always)]
    fn and_then<F, Output>(self, f: F) -> P<AndThen<Self, F>>
        where Self: Sized,
              F: FnMut(Self::Output) -> std::result::Result<Output, Self::Error>
    {
        P(AndThen(self, f))
    }

    #[inline(always)]
    fn optional(self) -> P<Optional<Self>>
        where Self: Sized
    {
        P(Optional(self))
    }

    #[inline(always)]
    fn repeat<R>(self, range: R) -> P<Repeat<Self, R>>
        where Self: Sized,
              R: Range
    {
        P(Repeat(self, range))
    }
}

impl<F, Input, Output, Error> Parser<Input> for F
    where F: FnMut(Input, usize) -> Result<Output, Error>
{
    type Output = Output;
    type Error = Error;

    #[inline(always)]
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

    #[inline(always)]
    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        self.0.parse(input, from)
    }
}

impl<A, B> std::ops::BitOr<B> for P<A> {
    type Output = P<Or<A, B>>;

    #[inline(always)]
    fn bitor(self, b: B) -> Self::Output {
        P(Or(self.0, b))
    }
}

impl<A, B> std::ops::Shl<B> for P<A> {
    type Output = P<AndSkip<A, B>>;

    #[inline(always)]
    fn shl(self, b: B) -> Self::Output {
        P(AndSkip(self.0, b))
    }
}

impl<A, B> std::ops::Shr<B> for P<A> {
    type Output = P<SkipAnd<A, B>>;

    #[inline(always)]
    fn shr(self, b: B) -> Self::Output {
        P(SkipAnd(self.0, b))
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

    #[inline(always)]
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

    #[inline(always)]
    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        let (from, a) = self.0.parse(input, from)?;
        let (from, _) = self.1.parse(input, from)?;
        Ok((from, a))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SkipAnd<A, B>(pub A, pub B);

impl<A, B, Input> Parser<Input> for SkipAnd<A, B>
    where A: Parser<Input>,
          B: Parser<Input, Error = A::Error>,
          Input: Copy
{
    type Output = B::Output;
    type Error = A::Error;

    #[inline(always)]
    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        let (from, _) = self.0.parse(input, from)?;
        self.1.parse(input, from)
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

    #[inline(always)]
    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        match self.0.parse(input, from) {
            Ok((from, output)) => Ok((from, output)),
            Err((from2, error)) => {
                if from == from2 {
                    self.1.parse(input, from)
                } else {
                    Err((from2, error))
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Try<A>(pub A);

impl<A, Input> Parser<Input> for Try<A>
    where A: Parser<Input>
{
    type Output = A::Output;
    type Error = A::Error;

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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
            #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
    fn parse(&mut self, input: Input, from: usize) -> Result<Self::Output, Self::Error> {
        match self.0.parse(input, from) {
            Ok((from, output)) => Ok((from, Some(output))),
            Err((from2, _)) if from == from2 => Ok((from, None)),
            Err((from, error)) => Err((from, error)),
        }
    }
}

pub trait Range {
    #[inline(always)]
    fn min(&self) -> usize;
    #[inline(always)]
    fn max(&self) -> Option<usize>;
}

impl Range for usize {
    #[inline(always)]
    fn min(&self) -> usize {
        *self
    }

    #[inline(always)]
    fn max(&self) -> Option<usize> {
        Some(*self)
    }
}

impl Range for std::ops::RangeFull {
    #[inline(always)]
    fn min(&self) -> usize {
        0
    }

    #[inline(always)]
    fn max(&self) -> Option<usize> {
        None
    }
}

impl Range for std::ops::RangeTo<usize> {
    #[inline(always)]
    fn min(&self) -> usize {
        0
    }

    #[inline(always)]
    fn max(&self) -> Option<usize> {
        Some(self.end)
    }
}

impl Range for std::ops::RangeFrom<usize> {
    #[inline(always)]
    fn min(&self) -> usize {
        self.start
    }

    #[inline(always)]
    fn max(&self) -> Option<usize> {
        None
    }
}

impl Range for std::ops::Range<usize> {
    #[inline(always)]
    fn min(&self) -> usize {
        self.start
    }

    #[inline(always)]
    fn max(&self) -> Option<usize> {
        Some(self.end)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Repeat<A, R: Range>(pub A, pub R);

impl<A, R> P<Repeat<A, R>>
    where R: Range
{
    #[inline(always)]
    pub fn join<B, Input>(self, b: B) -> P<Join<A, B, R>>
        where A: Parser<Input>,
              B: Parser<Input, Error = A::Error>
    {
        P(Join((self.0).0, b, (self.0).1))
    }

    #[inline(always)]
    pub fn fold<Acc, F, Input, Output>(self, acc: Acc, f: F) -> P<Fold<A, R, Acc, F>>
        where A: Parser<Input>,
              Acc: FnMut() -> Output,
              F: FnMut(Output, A::Output) -> Output
    {
        P(Fold((self.0).0, (self.0).1, acc, f))
    }
}

impl<A, R, Input> Parser<Input> for Repeat<A, R>
    where A: Parser<Input>,
          R: Range,
          Input: Copy
{
    type Output = Vec<A::Output>;
    type Error = A::Error;

    #[inline(always)]
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
pub struct Fold<A, R: Range, Acc, F>(pub A, pub R, pub Acc, pub F);

impl<A, R, Acc, F, Input, Output> Parser<Input> for Fold<A, R, Acc, F>
    where A: Parser<Input>,
          R: Range,
          Acc: FnMut() -> Output,
          F: FnMut(Output, A::Output) -> Output,
          Input: Copy
{
    type Output = Output;
    type Error = A::Error;

    #[inline(always)]
    fn parse(&mut self, input: Input, mut from: usize) -> Result<Self::Output, Self::Error> {
        let (min, max) = (self.1.min(), self.1.max());
        let mut acc = self.2();
        let mut done = 0;
        loop {
            if Some(done) == max {
                return Ok((from, acc));
            }

            match self.0.parse(input, from) {
                Ok((from2, output)) => {
                    from = from2;
                    acc = self.3(acc, output);
                    done += 1;
                }
                Err((from2, error)) => {
                    return if from == from2 && done >= min {
                        Ok((from2, acc))
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

    #[inline(always)]
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
#[inline(always)]
pub fn Position<Input, Error>(_input: Input, from: usize) -> Result<usize, Error> {
    Ok((from, from))
}
