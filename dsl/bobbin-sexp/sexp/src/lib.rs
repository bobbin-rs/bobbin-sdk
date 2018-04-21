extern crate bobbin_sexp_tokenizer as tokenizer;

pub mod parser;
use tokenizer::Token;


#[derive(Debug, Clone, PartialEq)]
pub enum Sexp<'a> {
    List(Vec<Sexp<'a>>, Token<'a>, Token<'a>),
    Token(Token<'a>),
}

impl<'a> From<Token<'a>> for Sexp<'a> {
    fn from(other: Token<'a>) -> Self {
        Sexp::Token(other)
    }
}

impl<'a> From<Vec<Sexp<'a>>> for Sexp<'a> {
    fn from(other: Vec<Sexp<'a>>) -> Self {
        Sexp::List(other, Token::ListStart("("), Token::ListEnd(")"))
    }
}

impl<'a> Sexp<'a> {
    pub fn token(&self) -> Option<&Token> {
        if let &Sexp::Token(ref tok) = self {
            Some(tok)
        } else {
            None
        }
    }

    pub fn symbol(&self) -> Option<&str> {
        if let Some(&Token::Symbol(ref s)) = self.token() {
            Some(s)
        } else {
            None
        }
    }

    pub fn string(&self) -> Option<&str> {
        if let Some(&Token::String(ref s)) = self.token() {
            Some(s)
        } else {
            None
        }
    }

    pub fn number(&self) -> Option<&str> {
        if let Some(&Token::Number(ref s)) = self.token() {
            Some(s)
        } else {
            None
        }
    }

    pub fn list(&self) -> Option<&[Sexp]> {
        if let &Sexp::List(ref list, _, _) = self {
            Some(list)
        } else {
            None
        }
    }

    pub fn split_first(&self) -> Option<(&Sexp, &[Sexp])> {
        self.list().map(|list| (&list[0], &list[1..]))
    }

    pub fn first(&self) -> Option<&Sexp> {
        self.split_first().map(|(first, _)| first)
    }

    pub fn rest(&self) -> Option<&[Sexp]> {
        self.split_first().map(|(_, rest)| rest)
    }
}

pub fn symbol<'a>(s: &'a str) -> Sexp<'a> {
    Sexp::Token(Token::Symbol(s))
}

pub fn string<'a>(s: &'a str) -> Sexp<'a> {
    Sexp::Token(Token::String(s))
}

pub fn number<'a>(s: &'a str) -> Sexp<'a> {
    Sexp::Token(Token::Number(s))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let abc = symbol("abc");
        let def = symbol("def");
        let list = Sexp::from(vec![abc.clone(), def.clone()]);
        assert!(list.list().is_some());
        assert_eq!(list.list(), Some(&[abc.clone(), def.clone()][..]));
        assert!(list.list().unwrap()[0] == abc);
        assert!(list.list().unwrap()[1] == def);

        assert_eq!(list.first(), Some(&abc));
        assert_eq!(list.rest(), Some(&[def.clone()][..]));
    }
}