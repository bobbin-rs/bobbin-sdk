use std::mem;
use Sexp;
use Token;
use tokenizer::reader::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError(String); 

pub fn parse<'a>(buf: &'a [u8]) -> Result<Sexp<'a>, ParseError> {
    let mut stack: Vec<Vec<Sexp>> = Vec::new();
    let mut top: Vec<Sexp> = Vec::new();

    let mut r = BufReader::new(buf);
    while let Some(tok) = r.read() {
        match tok {
            Token::DocStart(_) => {},
            Token::Error(_, reason) => {
                return Err(ParseError(reason.to_owned()))
            },
            Token::WhiteSpace(_) | Token::Comment(_) => continue,
            Token::ListStart(_) => {
                let mut t = Vec::new();
                t.push(Sexp::Token(tok));
                mem::swap(&mut top, &mut t);
                stack.push(t)                
            },
            Token::ListEnd(_) => {
                if let Some(mut t) = stack.pop() {
                    mem::swap(&mut top, &mut t);
                    if let Sexp::Token(ls) = t.remove(0) {                    
                        top.push(Sexp::List(t, ls, tok));
                    } else {
                        panic!("Expected ListStart as first member of array")
                    }
                } else {
                    return Err(ParseError(format!("Unexpected close parenthesis at line {}", r.line_of(&tok))))
                }
            },
            Token::Symbol(..) | Token::String(..) | Token::Number(..) |
            Token::U8(..) | Token::U16(..) | Token::U32(..) | Token::U64(..) |
            Token::I8(..) | Token::I16(..) | Token::I32(..) | Token::I64(..) |
            Token::F32(..) | Token::F64(..) => {                
                top.push(Sexp::Token(tok))
            },                    
            Token::DocEnd(_) => {},
        }
    }

    if stack.len() > 1 {
        Err(ParseError("Incomplete expression".to_owned()))
    } else {
        if top.len() > 1 {
            Err(ParseError("More than one item at top level".to_owned()))
        } else {
            Ok(top.pop().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atoms() {
        let abc = Sexp::Token(Token::Symbol("abc"));
        assert_eq!(parse(b"abc").unwrap(), abc);
        assert_eq!(parse(b"  abc").unwrap(), abc);
        assert_eq!(parse(b"abc  ").unwrap(), abc);

        let abc_str = Sexp::Token(Token::String("\"abc\""));
        assert_eq!(parse(b"\"abc\"").unwrap(), abc_str);
        assert_eq!(parse(b"  \"abc\"").unwrap(), abc_str);
        assert_eq!(parse(b"\"abc\"  ").unwrap(), abc_str);
        
        assert_eq!(parse(b"0").unwrap(), Sexp::Token(Token::Number("0")));
        assert_eq!(parse(b"0x0").unwrap(), Sexp::Token(Token::Number("0x0")));

        assert_eq!(parse(b"0u8").unwrap(), Sexp::Token(Token::U8("0u8", 0)));
        assert_eq!(parse(b"0u16").unwrap(), Sexp::Token(Token::U16("0u16", 0)));
        assert_eq!(parse(b"0u32").unwrap(), Sexp::Token(Token::U32("0u32", 0)));
        assert_eq!(parse(b"0u64").unwrap(), Sexp::Token(Token::U64("0u64", 0)));

        assert_eq!(parse(b"0x0u8").unwrap(), Sexp::Token(Token::U8("0x0u8", 0)));
        assert_eq!(parse(b"0x0u16").unwrap(), Sexp::Token(Token::U16("0x0u16", 0)));
        assert_eq!(parse(b"0x0u32").unwrap(), Sexp::Token(Token::U32("0x0u32", 0)));
        assert_eq!(parse(b"0x0u64").unwrap(), Sexp::Token(Token::U64("0x0u64", 0)));


        assert_eq!(parse(b"0i8").unwrap(), Sexp::Token(Token::I8("0i8", 0)));
        assert_eq!(parse(b"0i16").unwrap(), Sexp::Token(Token::I16("0i16", 0)));
        assert_eq!(parse(b"0i32").unwrap(), Sexp::Token(Token::I32("0i32", 0)));
        assert_eq!(parse(b"0i64").unwrap(), Sexp::Token(Token::I64("0i64", 0)));

        assert_eq!(parse(b"0x0i8").unwrap(), Sexp::Token(Token::I8("0x0i8", 0)));
        assert_eq!(parse(b"0x0i16").unwrap(), Sexp::Token(Token::I16("0x0i16", 0)));
        assert_eq!(parse(b"0x0i32").unwrap(), Sexp::Token(Token::I32("0x0i32", 0)));
        assert_eq!(parse(b"0x0i64").unwrap(), Sexp::Token(Token::I64("0x0i64", 0)));

        assert_eq!(parse(b"-1i8").unwrap(), Sexp::Token(Token::I8("-1i8", -1)));
        assert_eq!(parse(b"-1i16").unwrap(), Sexp::Token(Token::I16("-1i16", -1)));
        assert_eq!(parse(b"-1i32").unwrap(), Sexp::Token(Token::I32("-1i32", -1)));
        assert_eq!(parse(b"-1i64").unwrap(), Sexp::Token(Token::I64("-1i64", -1)));

        assert_eq!(parse(b"0f32").unwrap(), Sexp::Token(Token::F32("0f32", 0.0)));
        assert_eq!(parse(b"0f64").unwrap(), Sexp::Token(Token::F64("0f64", 0.0)));

        assert_eq!(parse(b"-1.0f32").unwrap(), Sexp::Token(Token::F32("-1.0f32", -1.0)));
        assert_eq!(parse(b"-1.0f64").unwrap(), Sexp::Token(Token::F64("-1.0f64", -1.0)));

        assert_eq!(parse(b"1e8f32").unwrap(), Sexp::Token(Token::F32("1e8f32", 1e8)));
        assert_eq!(parse(b"1e8f64").unwrap(), Sexp::Token(Token::F64("1e8f64", 1e8)));

        assert_eq!(parse(b"1.0e8f32").unwrap(), Sexp::Token(Token::F32("1.0e8f32", 1.0e8)));
        assert_eq!(parse(b"1.0e8f64").unwrap(), Sexp::Token(Token::F64("1.0e8f64", 1.0e8)));

        assert_eq!(parse(b"-1.0e8f32").unwrap(), Sexp::Token(Token::F32("-1.0e8f32", -1.0e8)));
        assert_eq!(parse(b"-1.0e8f64").unwrap(), Sexp::Token(Token::F64("-1.0e8f64", -1.0e8)));
    }

    #[test]
    fn test_parse_lists() {
        let abc = Sexp::Token(Token::Symbol("abc"));
        let def = Sexp::Token(Token::Symbol("def"));
        let ghi = Sexp::Token(Token::Symbol("ghi"));
        //let jkl = Sexp::Token(Token::Symbol("jkl"));


        assert_eq!(parse(b"(abc)").unwrap(), Sexp::from(vec![abc.clone()]));
        assert_eq!(parse(b"(abc def)").unwrap(), Sexp::from(vec![abc.clone(), def.clone()]));
        assert_eq!(parse(b"(abc (def))").unwrap(), Sexp::from(vec![abc.clone(), Sexp::from(vec![def.clone()])]));
        assert_eq!(parse(b"(abc (def) ghi)").unwrap(), Sexp::from(vec![            
            abc.clone(), Sexp::from(vec![def.clone()]), ghi.clone()
        ]));
    }

    #[test]
    fn test_parse_comment() {
        assert_eq!(parse(b"abc;comment").unwrap(), Sexp::Token(Token::Symbol("abc")));
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(parse(b")").unwrap_err(), ParseError("Unexpected close parenthesis at line 1".to_owned()));
        assert_eq!(parse(b"\n)").unwrap_err(), ParseError("Unexpected close parenthesis at line 2".to_owned()));
    }
}