#![no_std]
//extern crate core;

pub mod reader;
pub mod writer;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    WhiteSpace(&'a str),
    Symbol(&'a str),
    String(&'a str),
    Number(&'a str),
    U8(&'a str, u8),
    U16(&'a str, u16),
    U32(&'a str, u32),
    U64(&'a str, u64),
    I8(&'a str, i8),
    I16(&'a str, i16),
    I32(&'a str, i32),
    I64(&'a str, i64),
    F32(&'a str, f32),
    F64(&'a str, f64),
    ListStart(&'a str),
    ListEnd(&'a str),
    Comment(&'a str),
    Error(&'a str, &'static str),
    DocStart(&'a str),
    DocEnd(&'a str),
}

impl<'a> AsRef<str> for Token<'a> {
    fn as_ref(&self) -> &str {        
        use self::Token::*;
        match *self {
            WhiteSpace(ref s) => s,
            Symbol(ref s) => s,
            String(ref s) => s,
            Number(ref s) => s,
            U8(ref s, _) => s,
            U16(ref s, _) => s,
            U32(ref s, _) => s,
            U64(ref s, _) => s,
            I8(ref s, _) => s,
            I16(ref s, _) => s,
            I32(ref s, _) => s,
            I64(ref s, _) => s,
            F32(ref s, _) => s,
            F64(ref s, _) => s,
            ListStart(ref s) => s,
            ListEnd(ref s) => s,
            Comment(ref s) => s,
            Error(ref s, _) => s,
            DocStart(ref s) => s,
            DocEnd(ref s) => s,
            // _ => unimplemented!()
        }
    }    
}

#[cfg(test)]
mod tests {        
    use core::fmt::Write;
    use super::reader::*;
    use super::writer::*;

    #[test]
    fn test_roundtrip() {
        let input = b"(\"abc\" abc abc_def abc-def 123 123.456 -1i32 123u32 0b1111u8 0x123u32 123i32 3.14f32 3.14f64)";
        let mut buf = [0u8; 128];
        let mut w = BufWriter::new(&mut buf);
        let mut r = BufReader::new(input);
        while let Some(t) = r.read() {
            w.write_str(t.as_ref()).unwrap();
        }
        assert_eq!(w.as_slice(), &input[..]);
    }
}
