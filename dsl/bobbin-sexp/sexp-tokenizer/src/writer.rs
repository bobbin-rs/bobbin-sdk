use Token;
use core::fmt::{Write, Result};
use core::ops::Deref;

#[derive(Debug)]
pub struct BufWriter<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl<'a> BufWriter<'a> {
    pub fn write(&mut self, tok: &Token) -> Result {
        self.write_str(tok.as_ref())
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl<'a> BufWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        BufWriter { buf: buf, len: 0 }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn remaining(&self) -> usize {
        self.buf.len() - self.len
    }

    pub fn write_list_start(&mut self) -> Result {
        self.write_str("(")
    }

    pub fn write_list_end(&mut self) -> Result {
        self.write_str(")")
    }

    pub fn write_whitespace(&mut self, s: &str) -> Result {
        self.write_str(s)
    }

    pub fn write_string(&mut self, s: &str) -> Result {
        write!(self, "{:?}", s)
    }

    pub fn write_symbol(&mut self, s: &str) -> Result {
        self.write_str(s)
    }

    pub fn write_number(&mut self, s: &str) -> Result {
        self.write_str(s)
    }

    pub fn write_u64(&mut self, value: u64) -> Result {
        write!(self, "{}u64", value)
    }

    pub fn write_u32(&mut self, value: u32) -> Result {
        write!(self, "{}u32", value)
    }

    pub fn write_u16(&mut self, value: u16) -> Result {
        write!(self, "{}u16", value)
    }

    pub fn write_u8(&mut self, value: u8) -> Result {
        write!(self, "{}u8", value)
    }

    pub fn write_i64(&mut self, value: i64) -> Result {
        write!(self, "{}i64", value)
    }

    pub fn write_i32(&mut self, value: i32) -> Result {
        write!(self, "{}i32", value)
    }

    pub fn write_i16(&mut self, value: i16) -> Result {
        write!(self, "{}i16", value)
    }

    pub fn write_i8(&mut self, value: i8) -> Result {
        write!(self, "{}i8", value)
    }

    pub fn write_f32(&mut self, value: f32) -> Result {
        write!(self, "{}f32", value)
    }

    pub fn write_f64(&mut self, value: f64) -> Result {
        write!(self, "{}f64", value)
    }
}

impl<'a> Deref for BufWriter<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.buf[..self.len]
    }
}

impl<'a> Write for BufWriter<'a> {
    fn write_str(&mut self, s: &str) -> Result {
        for b in s.bytes() {
            self.buf[self.len] = b;
            self.len += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        use self::Token::*;

        let mut buf = [0u8; 64];
        let mut w = BufWriter::new(&mut buf);
        w.write_str(ListStart("(").as_ref()).unwrap();
        w.write_str(Symbol("abc").as_ref()).unwrap();
        w.write_str(ListEnd(")").as_ref()).unwrap();
        assert_eq!(w.as_slice(), b"(abc)");
    }

    #[test]
    fn test_buf_writer() {
        let mut buf = [0u8; 1024];
        let mut w = BufWriter::new(&mut buf);
        w.write_list_start().unwrap();
        w.write_symbol("abc").unwrap();
        w.write_whitespace(" ").unwrap();
        w.write_string("hello").unwrap();
        w.write_whitespace(" ").unwrap();
        w.write_number("123").unwrap();
        w.write_whitespace(" ").unwrap();
        w.write_number("123.456").unwrap();
        w.write_whitespace(" ").unwrap();
        w.write_number("1.23e-10").unwrap();
        w.write_whitespace(" ").unwrap();
        w.write_i32(-1).unwrap();
        w.write_whitespace(" ").unwrap();
        w.write_u64(123).unwrap();
        w.write_whitespace(" ").unwrap();
        w.write_i64(123).unwrap();
        w.write_whitespace(" ").unwrap();
        w.write_f64(3.14).unwrap();
        w.write_whitespace(" ").unwrap();
        w.write_f32(3.14).unwrap();
        w.write_list_end().unwrap();
        assert_eq!(w.as_slice(), &b"(abc \"hello\" 123 123.456 1.23e-10 -1i32 123u64 123i64 3.14f64 3.14f32)"[..]);
    }

}