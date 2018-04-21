use Token;
use core::str::{FromStr, Utf8Error};

#[derive(Debug)]
enum ReadState {
    Init,
    Start,
    ReadWhiteSpace(usize),
    ReadString(usize),
    ReadStringEscape(usize),
    ReadSymbol(usize),
    ReadNumber(usize),
    ReadComment(usize),
    ErrorState,
    End,
    Done,
}

pub struct BufReader<'a> {
    buf: &'a [u8],
    pos: usize,
    state: ReadState,
}

impl<'a> BufReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        BufReader { buf: buf, pos: 0, state: ReadState::Init }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn line(&self) -> usize {
        let mut line = 0;
        for i in 0..self.pos {
            if self.buf[i] == b'\n' {
                line += 1;
            }
        }
        line
    }

    pub fn pos_of(&self, t: &Token<'a>) -> usize {
        (t.as_ref().as_ptr() as usize) - (self.buf.as_ptr() as usize)
    }

    pub fn line_of(&self, t: &Token<'a>) -> usize {
        let p = self.pos_of(t);
        let mut line = 1;
        for i in 0..p {
            if self.buf[i] == b'\n' {
                line += 1;
            }
        }
        line        
    }

    pub fn len(&self) -> usize {
        self.buf.len() - self.pos
    }

    fn checked_from(&self, p: usize) -> Result<&'a str, Utf8Error> {
        ::core::str::from_utf8(&self.buf[p..self.pos]) 
    }    

    fn unchecked_from(&self, p: usize) -> &'a str {
        unsafe { 
            ::core::str::from_utf8_unchecked(&self.buf[p..self.pos]) 
        }
    }

    pub fn read(&mut self) -> Option<Token<'a>> {
        use self::ReadState::*;
        use self::Token::*;

        match self.state {
            Init => {
                self.state = Start;
                return Some(DocStart(self.unchecked_from(self.pos)))
            },
            End => {
                self.state = Done;
                return Some(DocEnd(self.unchecked_from(self.pos)))
            },
            Done => return None,
            _ => {}
        }

        loop {
            if self.len() == 0 {
                return match self.state {
                    Start => {
                        self.state = Done;
                        Some(DocEnd(self.unchecked_from(self.pos)))
                    },
                    ReadWhiteSpace(p) => {
                        self.state = End;
                        Some(WhiteSpace(self.unchecked_from(p)))
                    },
                    ReadString(_) | ReadStringEscape(_) => {
                        self.state = ErrorState;
                        Some(Error(self.unchecked_from(self.pos), "Unexpected EOF in String"))
                    },
                    ReadSymbol(p) => {
                        self.state = End;                        
                        Some(Symbol(self.unchecked_from(p)))
                    },
                    ReadNumber(p) => match parse_number(&self.buf[p..self.pos]) {
                        e @ Token::Error(..) => {
                            self.state = ErrorState;
                            return Some(e)
                        },
                        n @ _ => {
                            self.state = End;
                            return Some(n)
                        }
                    },
                    ReadComment(p) => {
                        self.state = End;
                        Some(Comment(self.unchecked_from(p)))
                    }
                    _ => panic!("Unexpected State: {:?}", self.state),
                }
            }
            let c = self.buf[self.pos];
            match self.state {
                Start => {
                    match c {
                        b'(' => {
                            self.state = Start;
                            self.pos += 1;
                            return Some(ListStart(self.unchecked_from(self.pos - 1)));
                        },
                        b')' => {
                            self.state = Start;
                            self.pos += 1;
                            return Some(ListEnd(self.unchecked_from(self.pos - 1)));
                        },
                        b' ' | b'\n' | b'\t' => {
                            self.state = ReadWhiteSpace(self.pos);
                            self.pos += 1;
                        },
                        b'\"' => {
                            self.state = ReadString(self.pos);
                            self.pos += 1;
                        },
                        b'a' ... b'z' | b'A' ... b'Z' | b'_' | b':' => {
                            self.state = ReadSymbol(self.pos);
                            self.pos += 1;
                        },
                        b'0' ... b'9' | b'+' | b'-' | b'=' => {
                            self.state = ReadNumber(self.pos);
                            self.pos += 1;
                        },
                        b';' => {
                            self.state = ReadComment(self.pos);
                            self.pos += 1;
                        },
                        _ => {
                            self.state = ErrorState;
                        }
                        //_ => panic!("Unexpected Character {:?}, line {}", c as char, self.line()),

                    }
                },
                ReadWhiteSpace(p) => match c {
                    b' ' | b'\n' | b'\t' => self.pos += 1,
                    _ => {
                        self.state = Start;                        
                        return Some(WhiteSpace(self.unchecked_from(p)));
                    }
                },
                ReadString(p) => match c {
                    b'\\' => {
                        self.state = ReadStringEscape(p);
                        self.pos += 1;
                    },
                    b'\"' => {
                        self.pos += 1;
                        if let Ok(s) = self.checked_from(p) {
                            self.state = Start;
                            return Some(String(s))
                        } else {
                            self.pos = p;
                            self.state = ErrorState;                            
                            return Some(Error(self.unchecked_from(self.pos), "Invalid UTF8 in String"))
                        }
                        
                    }
                    _ => self.pos += 1,
                },
                ReadStringEscape(p) => {
                    self.state = ReadString(p);
                    self.pos += 1;
                },
                ReadSymbol(p) => match c {
                    b'a' ... b'z' | b'A' ... b'Z' | b'0' ... b'9' | b'_' | b'-' | b':' | b'*' => self.pos += 1,
                    _ => {
                        self.state = Start;
                        return Some(Symbol(self.unchecked_from(p)));
                    },
                },
                ReadNumber(p) => match c {
                    b'0' ... b'9' | b'a' ... b'f' | b'A' ... b'F' | b'.' | b'-' | b'u' | b'i' | b'x'  => self.pos += 1,                    
                    _ => {
                        match parse_number(&self.buf[p..self.pos]) {
                            e @ Token::Error(..) => {
                                self.state = ErrorState;
                                return Some(e)
                            },
                            n @ _ => {
                                self.state = Start;
                                return Some(n)
                            }
                        }
                    }
                },
                ReadComment(p) => match c {
                    b'\n' => {
                        if let Ok(s) = self.checked_from(p) {
                            self.state = Start;
                            return Some(Comment(s));
                        } else {
                            self.pos = p;
                            self.state = ErrorState;                            
                            return Some(Error(self.unchecked_from(self.pos), "Invalid UTF8 in Comment"))
                        }
                        
                    },
                    _ => self.pos += 1,
                },
                _ => panic!("Unexpected State: {:?}", self.state),
            }
        }
    }
}

fn parse_number(buf: &[u8]) -> Token {
    use self::Token::*;
    let mut buf = buf;
    let s = unsafe { ::core::str::from_utf8_unchecked(buf)};
    let mut radix = 10;    

    if buf.starts_with(b"0b") {
        radix = 2;
        buf = &buf[2..];
    } else if buf.starts_with(b"0x") {
        radix = 16;
        buf = &buf[2..];
    } 

    let mut n = buf.len();    
    if radix == 16 {
        for i in 0..buf.len() {
            match buf[i] {
                b'u' | b'i' => {
                    n = i;
                    break;
                },
                _ => {}
            }    
        }            
    } else {
        for i in 0..buf.len() {
            match buf[i] {
                b'u' | b'i' | b'f' => {
                    n = i;
                    break;
                },
                _ => {}
            }    
        }            
    }
    
    let value = unsafe { ::core::str::from_utf8_unchecked(&buf[..n]) };    
    
    let suffix = &buf[n..];
    match suffix {
        b"" => return Number(s),
        b"u8" => return u8::from_str_radix(value, radix).map(|v| U8(s, v)).unwrap_or(Error(s, "Invalid Digit")),
        b"u16" => return u16::from_str_radix(value, radix).map(|v| U16(s, v)).unwrap_or(Error(s, "Invalid Digit")),
        b"u32" => return u32::from_str_radix(value, radix).map(|v| U32(s, v)).unwrap_or(Error(s, "Invalid Digit")),
        b"u64" => return u64::from_str_radix(value, radix).map(|v| U64(s, v)).unwrap_or(Error(s, "Invalid Digit")),
        b"i8" => return i8::from_str_radix(value, radix).map(|v| I8(s, v)).unwrap_or(Error(s, "Invalid Digit")),
        b"i16" => return i16::from_str_radix(value, radix).map(|v| I16(s, v)).unwrap_or(Error(s, "Invalid Digit")),
        b"i32" => return i32::from_str_radix(value, radix).map(|v| I32(s, v)).unwrap_or(Error(s, "Invalid Digit")),
        b"i64" => return i64::from_str_radix(value, radix).map(|v| I64(s, v)).unwrap_or(Error(s, "Invalid Digit")),
        b"f32" => return f32::from_str(value).map(|v| F32(s, v)).unwrap_or(Error(s, "Invalid Digit")),
        b"f64" => return f64::from_str(value).map(|v| F64(s, v)).unwrap_or(Error(s, "Invalid Digit")),
        _ => return Error(unsafe { ::core::str::from_utf8_unchecked(suffix) }, "Invalid Number Suffix"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        use self::Token::*;

        assert_eq!(parse_number(b"1234"), Number("1234"));
        assert_eq!(parse_number(b"123u8"), U8("123u8", 123u8));
        assert_eq!(parse_number(b"123u16"), U16("123u16", 123u16));
        assert_eq!(parse_number(b"123u32"), U32("123u32", 123u32));
        assert_eq!(parse_number(b"123u64"), U64("123u64", 123u64));
        assert_eq!(parse_number(b"123i8"), I8("123i8", 123i8));
        assert_eq!(parse_number(b"-123i8"), I8("-123i8", -123i8));
        assert_eq!(parse_number(b"123i16"), I16("123i16", 123i16));
        assert_eq!(parse_number(b"-123i16"), I16("-123i16", -123i16));
        assert_eq!(parse_number(b"123i32"), I32("123i32", 123i32));
        assert_eq!(parse_number(b"-123i32"), I32("-123i32", -123i32));
        assert_eq!(parse_number(b"123i64"), I64("123i64", 123i64));
        assert_eq!(parse_number(b"-123i64"), I64("-123i64", -123i64));
        assert_eq!(parse_number(b"123.456f64"), F64("123.456f64", 123.456f64));
        assert_eq!(parse_number(b"-123.456f64"), F64("-123.456f64", -123.456f64));
        assert_eq!(parse_number(b"123e10f64"), F64("123e10f64", 123e10f64));
        assert_eq!(parse_number(b"123E-10f64"), F64("123E-10f64", 123E-10f64));
        assert_eq!(parse_number(b"123.456f32"), F32("123.456f32", 123.456f32));

        assert_eq!(parse_number(b"123x456u8"), Error("123x456u8", "Invalid Digit"));
        assert_eq!(parse_number(b"123x456u16"), Error("123x456u16", "Invalid Digit"));
        assert_eq!(parse_number(b"123x456u32"), Error("123x456u32", "Invalid Digit"));
        assert_eq!(parse_number(b"123x456u64"), Error("123x456u64", "Invalid Digit"));
        assert_eq!(parse_number(b"123x456i8"), Error("123x456i8", "Invalid Digit"));
        assert_eq!(parse_number(b"123x456i16"), Error("123x456i16", "Invalid Digit"));
        assert_eq!(parse_number(b"123x456i32"), Error("123x456i32", "Invalid Digit"));
        assert_eq!(parse_number(b"123x456i64"), Error("123x456i64", "Invalid Digit"));
        assert_eq!(parse_number(b"123.a45f64"), Error("123.a45f64", "Invalid Digit"));

        assert_eq!(parse_number(b"123u33"), Error("u33","Invalid Number Suffix"));
        
    }

    #[test]
    fn test_symbol() {
        let mut r = BufReader::new(b"core::cmp::min");
        assert_eq!(r.read(),Some(Token::DocStart("")));
        assert_eq!(r.read(), Some(Token::Symbol("core::cmp::min")));
        assert_eq!(r.read(),Some(Token::DocEnd("")));
        assert_eq!(r.read(), None);        
    }

    #[test]
    fn test_tok() {        
        let mut r = BufReader::new(b"abc");
        assert_eq!(r.read(),Some(Token::DocStart("")));
        assert_eq!(r.read(), Some(Token::Symbol("abc")));
        assert_eq!(r.read(),Some(Token::DocEnd("")));
        assert_eq!(r.read(), None);

        let mut r = BufReader::new(b"abc   ");
        assert_eq!(r.read(),Some(Token::DocStart("")));
        assert_eq!(r.read(), Some(Token::Symbol("abc")));
        assert_eq!(r.read(), Some(Token::WhiteSpace("   ")));
        assert_eq!(r.read(),Some(Token::DocEnd("")));
        assert_eq!(r.read(), None);
    }

    #[test]
    fn test_buf_reader() {
        use self::Token::*;
        let mut r = BufReader::new(b"(\"abc\" \"ab\\\"c\" abc abc_def abc-def abc123 123 123.456 1.23e-10 -1i32 123u32 0b1111u8 0x123u32 123i32 3.14f32 3.14f64 0x0 0xf 0b100 0b101)");
        assert_eq!(r.read(),Some(DocStart("")));
        assert_eq!(r.read(),Some(ListStart("(")));
        assert_eq!(r.read(),Some(String("\"abc\"")));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(String("\"ab\\\"c\"")));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(Symbol("abc")));        
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(Symbol("abc_def")));        
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(Symbol("abc-def")));        
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(Symbol("abc123")));        
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(Number("123")));        
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(Number("123.456")));        
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(Number("1.23e-10")));        
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(I32("-1i32", -1)));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(U32("123u32", 123)));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(U8("0b1111u8", 0b1111)));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(U32("0x123u32", 0x123)));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(I32("123i32", 123)));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(F32("3.14f32", 3.14f32)));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(F64("3.14f64", 3.14f64)));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(Number("0x0")));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(Number("0xf")));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(Number("0b100")));
        assert_eq!(r.read(),Some(WhiteSpace(" ")));
        assert_eq!(r.read(),Some(Number("0b101")));
        assert_eq!(r.read(),Some(ListEnd(")")));
        assert_eq!(r.read(),Some(DocEnd("")));
    }

    #[test]
    fn test_read() {
        let mut r = BufReader::new(include_bytes!("../test-data/register.rx"));
        assert_eq!(r.read().unwrap(), Token::DocStart(""));
        assert_eq!(r.read().unwrap(), Token::ListStart("("));
        assert_eq!(r.read().unwrap(), Token::Symbol("register"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace("\n    "));
        assert_eq!(r.read().unwrap(), Token::Comment("; This is a comment"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace("\n    "));
        assert_eq!(r.read().unwrap(), Token::ListStart("("));
        assert_eq!(r.read().unwrap(), Token::Symbol("name"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace(" "));
        assert_eq!(r.read().unwrap(), Token::Symbol("ctrla"));
        assert_eq!(r.read().unwrap(), Token::ListEnd(")"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace(" "));
        assert_eq!(r.read().unwrap(), Token::Comment("; This is another comment"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace("\n    "));
        assert_eq!(r.read().unwrap(), Token::ListStart("("));
        assert_eq!(r.read().unwrap(), Token::Symbol("reset"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace(" "));
        assert_eq!(r.read().unwrap(), Token::U32("0u32",0));
        assert_eq!(r.read().unwrap(), Token::ListEnd(")"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace("\n    "));
        assert_eq!(r.read().unwrap(), Token::ListStart("("));
        assert_eq!(r.read().unwrap(), Token::Symbol("fields"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace("\n        "));
        assert_eq!(r.read().unwrap(), Token::ListStart("("));
        assert_eq!(r.read().unwrap(), Token::Symbol("mode"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace(" "));
        assert_eq!(r.read().unwrap(), Token::Number("3"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace(" "));
        assert_eq!(r.read().unwrap(), Token::Number("5"));
        assert_eq!(r.read().unwrap(), Token::ListEnd(")"));       

        assert_eq!(r.read().unwrap(), Token::WhiteSpace("\n        "));
        assert_eq!(r.read().unwrap(), Token::ListStart("("));
        assert_eq!(r.read().unwrap(), Token::Symbol("enable"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace(" "));
        assert_eq!(r.read().unwrap(), Token::Number("1"));
        assert_eq!(r.read().unwrap(), Token::ListEnd(")"));

        assert_eq!(r.read().unwrap(), Token::WhiteSpace("\n        "));
        assert_eq!(r.read().unwrap(), Token::ListStart("("));
        assert_eq!(r.read().unwrap(), Token::Symbol("swrst"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace(" "));
        assert_eq!(r.read().unwrap(), Token::Number("0"));
        assert_eq!(r.read().unwrap(), Token::ListEnd(")"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace("\n    "));
        assert_eq!(r.read().unwrap(), Token::ListEnd(")"));
        assert_eq!(r.read().unwrap(), Token::WhiteSpace("\n"));
        assert_eq!(r.read().unwrap(), Token::ListEnd(")"));
        assert_eq!(r.read().unwrap(), Token::DocEnd(""));        
    }

    #[test]
    fn test_pos() {
        let mut r = BufReader::new(b"abc def\nghi");
        assert_eq!(r.read().unwrap(), Token::DocStart(""));
        let t = r.read().unwrap();
        assert_eq!(t, Token::Symbol("abc"));
        assert_eq!(r.pos_of(&t), 0);
        assert_eq!(r.line_of(&t), 1);
        let t = r.read().unwrap();
        assert_eq!(r.pos_of(&t), 3);
        assert_eq!(r.line_of(&t), 1);
        let t = r.read().unwrap();
        assert_eq!(t, Token::Symbol("def"));
        assert_eq!(r.pos_of(&t), 4);
        assert_eq!(r.line_of(&t), 1);
        let t = r.read().unwrap();
        assert_eq!(r.pos_of(&t), 7);
        assert_eq!(r.line_of(&t), 1);
        let t = r.read().unwrap();
        assert_eq!(t, Token::Symbol("ghi"));
        assert_eq!(r.pos_of(&t), 8);
        assert_eq!(r.line_of(&t), 2);
        assert_eq!(r.read().unwrap(), Token::DocEnd(""));
    }    
}