//! A simple S-expression tokenizer, extended to recognize rust-style numbers.
#![no_std]

pub mod reader;
pub mod writer;

/// A Sexp Token
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    /// A whitespace token
    WhiteSpace(&'a str),
    /// A symbol token
    Symbol(&'a str),
    /// A string token
    String(&'a str),
    /// A number token
    Number(&'a str),
    /// A u8 token
    U8(&'a str, u8),
    /// A u16 token
    U16(&'a str, u16),
    /// A u32 token
    U32(&'a str, u32),
    /// A u64 token
    U64(&'a str, u64),
    /// An i8 token
    I8(&'a str, i8),
    /// An i16 token
    I16(&'a str, i16),
    /// An i32 token
    I32(&'a str, i32),
    /// An i64 token
    I64(&'a str, i64),
    /// A f32 token
    F32(&'a str, f32),
    /// A f64 token
    F64(&'a str, f64),
    /// A list start (open parenthesis) token
    ListStart(&'a str),
    /// A list end (close parenthesis) token
    ListEnd(&'a str),
    /// A comment token
    Comment(&'a str),
    /// An Error token
    Error(&'a str, &'static str),
    /// A document start token
    DocStart(&'a str),
    /// A document end token
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
