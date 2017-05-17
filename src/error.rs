#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error<'a> {
    Any,
    Ascii(Ascii),
    Binary(Endianness, BinaryType),
    Byte(u8),
    Bytes(&'a [u8]),
    Char(char),
    End,
    Peek,
    Satisfy,
    Str(&'a str),
    Take(usize),
    TakeWhile1,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum BinaryType {
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Ascii {
    Satisfy,
    TakeWhile1,
}
