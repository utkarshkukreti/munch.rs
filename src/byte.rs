use {Parser, Result};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error<'a> {
    Byte(u8),
    Bytes(&'a [u8]),
    Satisfy,
    TakeWhile1,
    Any,
    End,
    Binary(Endianness, Type),
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct End;

impl<'a> Parser<&'a [u8]> for End {
    type Output = ();
    type Error = Error<'static>;

    #[inline(always)]
    fn parse(&mut self, input: &'a [u8], from: usize) -> Result<Self::Output, Self::Error> {
        if input.len() == from {
            Ok((from, ()))
        } else {
            Err((from, Error::End))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Type {
    u8,
    u16,
    u32,
    u64,
    i8,
    i16,
    i32,
    i64,
    f32,
    f64,
}

macro_rules! read {
    ($input:expr, $from:expr, $ty:ident, $endianness:ident, $method:ident) => {{
        // Unsafe code adapted from:
        // https://github.com/BurntSushi/byteorder/blob/f8e7685b3a81/src/lib.rs#L790-L803
        let size = ::std::mem::size_of::<$ty>();
        let mut data: $ty = 0;
        if $input.len() >= $from + size {
            unsafe {
                ::std::ptr::copy_nonoverlapping(
                    $input.as_ptr().offset($from as isize),
                    &mut data as *mut $ty as *mut u8,
                    size);
            }
            Ok(($from + size, data.$method()))
        } else {
            Err(($from, Error::Binary(Endianness::$endianness, Type::$ty)))
        }
    }}
}

pub enum LittleEndian {}

impl LittleEndian {
    #[inline(always)]
    pub fn u8(input: &[u8], from: usize) -> Result<u8, Error<'static>> {
        read!(input, from, u8, Little, to_le)
    }

    #[inline(always)]
    pub fn u16(input: &[u8], from: usize) -> Result<u16, Error<'static>> {
        read!(input, from, u16, Little, to_le)
    }

    #[inline(always)]
    pub fn u32(input: &[u8], from: usize) -> Result<u32, Error<'static>> {
        read!(input, from, u32, Little, to_le)
    }

    #[inline(always)]
    pub fn u64(input: &[u8], from: usize) -> Result<u64, Error<'static>> {
        read!(input, from, u64, Little, to_le)
    }

    #[inline(always)]
    pub fn i8(input: &[u8], from: usize) -> Result<i8, Error<'static>> {
        read!(input, from, i8, Little, to_le)
    }

    #[inline(always)]
    pub fn i16(input: &[u8], from: usize) -> Result<i16, Error<'static>> {
        read!(input, from, i16, Little, to_le)
    }

    #[inline(always)]
    pub fn i32(input: &[u8], from: usize) -> Result<i32, Error<'static>> {
        read!(input, from, i32, Little, to_le)
    }

    #[inline(always)]
    pub fn i64(input: &[u8], from: usize) -> Result<i64, Error<'static>> {
        read!(input, from, i64, Little, to_le)
    }

    #[inline(always)]
    pub fn f32(input: &[u8], from: usize) -> Result<f32, Error<'static>> {
        // Unsafe code adapted from:
        // https://github.com/BurntSushi/byteorder/blob/f8e7685b3a81/src/lib.rs#L517
        Self::u32.map(|u32| unsafe { ::std::mem::transmute::<u32, f32>(u32) })
            .map_err(|_| Error::Binary(Endianness::Little, Type::f32))
            .parse(input, from)
    }

    #[inline(always)]
    pub fn f64(input: &[u8], from: usize) -> Result<f64, Error<'static>> {
        // Unsafe code adapted from:
        // https://github.com/BurntSushi/byteorder/blob/f8e7685b3a81/src/lib.rs#L540
        Self::u64.map(|u64| unsafe { ::std::mem::transmute::<u64, f64>(u64) })
            .map_err(|_| Error::Binary(Endianness::Little, Type::f64))
            .parse(input, from)
    }
}

pub enum BigEndian {}

impl BigEndian {
    #[inline(always)]
    pub fn u8(input: &[u8], from: usize) -> Result<u8, Error<'static>> {
        read!(input, from, u8, Big, to_be)
    }

    #[inline(always)]
    pub fn u16(input: &[u8], from: usize) -> Result<u16, Error<'static>> {
        read!(input, from, u16, Big, to_be)
    }

    #[inline(always)]
    pub fn u32(input: &[u8], from: usize) -> Result<u32, Error<'static>> {
        read!(input, from, u32, Big, to_be)
    }

    #[inline(always)]
    pub fn u64(input: &[u8], from: usize) -> Result<u64, Error<'static>> {
        read!(input, from, u64, Big, to_be)
    }

    #[inline(always)]
    pub fn i8(input: &[u8], from: usize) -> Result<i8, Error<'static>> {
        read!(input, from, i8, Big, to_be)
    }

    #[inline(always)]
    pub fn i16(input: &[u8], from: usize) -> Result<i16, Error<'static>> {
        read!(input, from, i16, Big, to_be)
    }

    #[inline(always)]
    pub fn i32(input: &[u8], from: usize) -> Result<i32, Error<'static>> {
        read!(input, from, i32, Big, to_be)
    }

    #[inline(always)]
    pub fn i64(input: &[u8], from: usize) -> Result<i64, Error<'static>> {
        read!(input, from, i64, Big, to_be)
    }

    #[inline(always)]
    pub fn f32(input: &[u8], from: usize) -> Result<f32, Error<'static>> {
        // Unsafe code adapted from:
        // https://github.com/BurntSushi/byteorder/blob/f8e7685b3a81/src/lib.rs#L517
        Self::u32.map(|u32| unsafe { ::std::mem::transmute::<u32, f32>(u32) })
            .map_err(|_| Error::Binary(Endianness::Big, Type::f32))
            .parse(input, from)
    }

    #[inline(always)]
    pub fn f64(input: &[u8], from: usize) -> Result<f64, Error<'static>> {
        // Unsafe code adapted from:
        // https://github.com/BurntSushi/byteorder/blob/f8e7685b3a81/src/lib.rs#L540
        Self::u64.map(|u64| unsafe { ::std::mem::transmute::<u64, f64>(u64) })
            .map_err(|_| Error::Binary(Endianness::Big, Type::f64))
            .parse(input, from)
    }
}
