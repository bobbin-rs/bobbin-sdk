extern crate bobbin_sexp as sexp;
extern crate bobbin_sexp_tokenizer as sexp_tokenizer;

use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{self, Read, Write, BufRead, BufReader};
//use std::str::FromStr;

use sexp::Sexp;
use sexp::parser::{parse, ParseError};
use sexp_tokenizer::Token;

#[derive(Debug)]
pub enum BuildError {
    ParseError(ParseError),
    IoError(io::Error),
    Error(String),
}

impl From<ParseError> for BuildError {
    fn from(other: ParseError) -> Self {
        BuildError::ParseError(other)
    }
}

impl From<io::Error> for BuildError {
    fn from(other: io::Error) -> Self {
        BuildError::IoError(other)
    }
}

fn main() {
    gen_linker();
    gen_config();
    pins().unwrap();
}

fn gen_linker() {
    // Pass our linker script to the top crate
    let script = "stm32l0.ld";
    let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fs::copy(src.join(script), out.join(script)).unwrap();
    println!("cargo:rustc-link-search={}", out.display());    
}

fn gen_config() {
    let src_path = Path::new("src/config.rx");
    let dst_path = Path::new("src/config.rs");

    let src = File::open(src_path).unwrap();
    let mut dst = File::create(dst_path).unwrap();

    for line in BufReader::new(src).lines() {
        writeln!(dst, "// {}", line.unwrap()).unwrap();
    }
}

fn pins() -> Result<(), BuildError> {
    let src_path = Path::new("src/pins.rx");
    let dst_path = Path::new("src/pins.rs");

    let mut src = File::open(src_path)?;
    let dst = File::create(dst_path)?;

    let mut buf: Vec<u8> = Vec::new();
    src.read_to_end(&mut buf)?;
    let s = parse(&buf)?;

    if let Some((head, rest)) = s.split_first() {
        match head {
            &Sexp::Token(Token::Symbol("pins")) => {
                gen_pins(dst, rest)
            },
            _ => Err(BuildError::Error(format!("Expected pins, got {:?}", s)))
        }
    } else {
        Err(BuildError::Error(format!("Expected pins, got {:?}", s)))
    }
}

fn gen_pins<W: Write>(mut w: W, s: &[Sexp]) -> Result<(), BuildError> {
    writeln!(w, "use hal::gpio;")?;
    writeln!(w, "")?;

    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => {
                let a = expect_symbol(&arr[0])?;
                let b = expect_symbol(&arr[1])?;
                //writeln!(w, "// {} {}", a, b)?;
                writeln!(w, "pub fn {}() -> gpio::PinUnknown {{ {}() }}", a, b)?;
            },
            _ => {},
        }
    }
    Ok(())
}

fn expect_symbol<'a>(s: &'a Sexp) -> Result<&'a str, BuildError> {
    if let &Sexp::Token(Token::Symbol(s)) = s {
        Ok(s)
    } else {
        Err(BuildError::Error(format!("Expected Symbol, got {:?}",  s)))
    }
}

// fn expect_number(s: &Sexp) -> Result<u64, BuildError> {
//     if let &Sexp::Token(Token::Number(n)) = s {
//         if let Ok(v) = u64::from_str(n) {
//             Ok(v)
//         } else {
//             Err(BuildError::Error(format!("Expected unsigned integer, got  {:?}", n)))
//         }
//     } else {
//         Err(BuildError::Error(format!("Expected unsigned integer, got {:?}", s)))
//     }
// }
