use std::io::{self, Read};
use std::str::FromStr;
use std::fs::File;
use std::path::Path;
use sexp::Sexp;
use sexp::parser::{parse, ParseError};
use sexp_tokenizer::Token;
use {TopLevel, Access, Board, Connection, Device, Region, Crate, Module, Peripheral, PeripheralGroup, Interrupt, Signal};
use {Exception, Cluster, Register, Field, EnumeratedValue};
use {PathElement};
use {Pin, AltFn, Clock, Variant};

#[derive(Debug)]
pub enum ReadError {
    ParseError(ParseError),
    IoError(io::Error),
    Error(String),
}

impl From<ParseError> for ReadError {
    fn from(other: ParseError) -> Self {
        ReadError::ParseError(other)
    }
}

impl From<io::Error> for ReadError {
    fn from(other: io::Error) -> Self {
        ReadError::IoError(other)
    }
}

struct Context<'a> {
    path: &'a Path,
    buf: &'a [u8],
}

impl<'a> Context<'a> {
    fn new(path: &'a Path, buf: &'a [u8]) -> Context<'a> {
        Context { path: path, buf: buf }
    }

    fn buf(&self) -> &'a [u8] {
        self.buf
    }

    fn path(&self) -> &'a Path {
        self.path
    }

    fn pos_of(&self, buf: &str) -> usize {
        (buf.as_ptr() as usize) - (self.buf.as_ptr() as usize)
    }

    fn line_of(&self, buf: &str) -> usize {
        let p = self.pos_of(buf);
        let mut line = 1;
        for i in 0..p {
            if self.buf[i] == b'\n' {
                line += 1;
            }
        }
        line        
    }
    
    fn location_of(&self, s: &Sexp) -> String {
        match s {
            &Sexp::List(_, ref ls, _) => {
                format!("[{}, line {}]", self.path.display(), self.line_of(ls.as_ref()))
            },
            &Sexp::Token(ref s) => {
                format!("[{}, line {}]", self.path.display(), self.line_of(s.as_ref()))
            }
        }
        
    }
}


fn expect_symbol<'a>(ctx: &Context, s: &'a Sexp) -> Result<&'a str, ReadError> {
    if let &Sexp::Token(Token::Symbol(s)) = s {
        Ok(s)
    } else {
        Err(ReadError::Error(format!("{}: Expected Symbol, got {:?}", ctx.location_of(s), s)))
    }
}

fn expect_string<'a>(ctx: &Context, s: &'a Sexp) -> Result<&'a str, ReadError> {
    if let &Sexp::Token(Token::String(s)) = s {
        Ok(&s[1..(s.len() - 1)])
    } else {
        Err(ReadError::Error(format!("{}: Expected String, got {:?}", ctx.location_of(s), s)))
    }
}

fn expect_u64(ctx: &Context, s: &Sexp) -> Result<u64, ReadError> {
    if let &Sexp::Token(Token::Number(n)) = s {
        if n.starts_with("0b") {
            if let Ok(v) = u64::from_str_radix(&n[2..], 16) {
                Ok(v)
            } else {
                Err(ReadError::Error(format!("{}: Expected unsigned integer, got {:?}", ctx.location_of(s), n)))
            }
        } else if n.starts_with("0x") {
            if let Ok(v) = u64::from_str_radix(&n[2..], 16) {
                Ok(v)
            } else {
                Err(ReadError::Error(format!("{}: Expected unsigned integer, got {:?}", ctx.location_of(s), n)))                
            }
        } else if let Ok(v) = u64::from_str(n) {
            Ok(v)
        } else {
            Err(ReadError::Error(format!("{}: Expected unsigned integer, got  {:?}", ctx.location_of(s), n)))
        }
    } else {
        Err(ReadError::Error(format!("{}: Expected unsigned integer, got {:?}", ctx.location_of(s), s)))
    }
}

#[allow(dead_code)]
fn expect_bits(ctx: &Context, s: &Sexp) -> Result<(u64, u64), ReadError> {
    match s {
        &Sexp::Token(Token::Number(n)) => {
            if let Ok(v) = u64::from_str(n) {
                Ok((v,1))
            } else {
                Err(ReadError::Error(format!("{}: Expected bits, got: {:?}", ctx.location_of(s), n)))
            }
        },
        &Sexp::List(ref arr, _, _) => {
            if arr.len() != 2 {
                Err(ReadError::Error(format!("{}: Expected (#,#), got {:?}", ctx.location_of(s), s)))
            } else {
                let offset = try!(expect_u64(ctx, &arr[0]));
                let size = try!(expect_u64(ctx, &arr[1]));
                Ok((offset, size))
            }
        },
        _ => Err(ReadError::Error(format!("{}: Expected (#,#), got {:?}", ctx.location_of(s), s))),
    }
}

fn expect_access(ctx: &Context, s: &Sexp) -> Result<Access, ReadError> {
    match try!(expect_symbol(ctx, s)) {
        "read-only" => Ok(Access::ReadOnly),
        "write-only" => Ok(Access::WriteOnly),
        "read-write" => Ok(Access::ReadWrite),
        "write-once" => Ok(Access::ReadWrite),
        "read-write-once" => Ok(Access::ReadWriteOnce),
        _ => Err(ReadError::Error(format!("{}: Expected access type, got {:?}", ctx.location_of(s), s))),
    }
}

pub fn read<R: Read>(r: &mut R, p: &Path) -> Result<TopLevel, ReadError> {    
    let mut buf: Vec<u8> = Vec::new();
    try!(r.read_to_end(&mut buf));
    let ctx = Context::new(p, &buf);
    read_buf(&ctx)
}

fn read_buf(ctx: &Context) -> Result<TopLevel, ReadError> {
    let buf = ctx.buf();
    let p = ctx.path();
    let s = try!(parse(buf));
    if let Some((head, rest)) = s.split_first() {
        match head {
            &Sexp::Token(Token::Symbol("board")) => read_board(ctx, rest).map(TopLevel::Board),
            &Sexp::Token(Token::Symbol("device")) => read_device(ctx, rest).map(TopLevel::Device),
            &Sexp::Token(Token::Symbol("peripheral")) => read_peripheral(ctx, rest).map(TopLevel::Peripheral),
            h @ _ => Err(ReadError::Error(format!("{}: Expected device, got {:?}", ctx.location_of(head), h))),
        }
    } else {
        Err(ReadError::Error(format!("{}: Expected list, got {:?}", ctx.location_of(&s), s)))
    }
}
fn read_board(ctx: &Context, s: &[Sexp]) -> Result<Board, ReadError> {
    let path = ctx.path();
    let mut b = Board::default();
    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => {                
                match arr[0].symbol() {
                    Some("name") => b.name = String::from(try!(read_name(ctx, &arr[1]))),
                    Some("description") => b.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                    Some("device") => b.devices.push(try!(read_device(ctx, &arr[1..]))),
                    Some("connections") => b.connections.extend(try!(read_connections(ctx, &arr[1..]))),
                    Some("clock") => b.clocks.push(try!(read_clock(ctx, &arr[1..]))),
                    Some("path") => b.paths.push(try!(read_path(ctx, &arr[1..]))),
                    _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
                }
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }
    }
    Ok(b)
}


fn read_connections(ctx: &Context, s: &[Sexp]) -> Result<Vec<Connection>, ReadError> {
    let path = ctx.path();
    let mut connections: Vec<Connection> = Vec::new();

    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("connection") => connections.push(try!(read_connection(ctx, &arr[1..]))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }        
    }

    Ok(connections)
}

fn read_connection(ctx: &Context, s: &[Sexp]) -> Result<Connection, ReadError> {
    let mut connection: Connection = Connection::default();

    if s.len() != 2 {
        return Err(ReadError::Error(format!("Expected (src, dst) for Connection, got {:?}", s)));
    }
    if let Sexp::List(ref arr, _, _) = s[0] {
        if arr.len() != 2 {
            return Err(ReadError::Error(format!("Expected (obj, signal) for Src, got {:?}", s)));            
        }
        connection.device_a = String::from(try!(expect_symbol(ctx, &arr[0])));
        connection.signal_a = String::from(try!(expect_symbol(ctx, &arr[1])));
    }
    if let Sexp::List(ref arr, _, _) = s[1] {
        if arr.len() != 2 {
            return Err(ReadError::Error(format!("Expected (obj, signal) for Dst, got {:?}", s)));            
        }
        connection.device_b = String::from(try!(expect_symbol(ctx, &arr[0])));
        connection.signal_b = String::from(try!(expect_symbol(ctx, &arr[1])));
    }

    Ok(connection)
}


fn read_path(ctx: &Context, s: &[Sexp]) -> Result<::Path, ReadError> {
    let mut path: ::Path = ::Path::default();

    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => {
                if arr.len() != 2 {
                    return Err(ReadError::Error(format!("Expected (src, dst) for Path Element, got {:?}", s)));
                }
                let mut pe = PathElement::default();
                pe.device = String::from(try!(expect_symbol(ctx, &arr[0])));
                pe.signal = String::from(try!(expect_symbol(ctx, &arr[1])));
                path.path_elements.push(pe)
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }        
    }

    Ok(path)
}

fn read_device(ctx: &Context, s: &[Sexp]) -> Result<Device, ReadError> {
    let path = ctx.path();
    let mut d = Device::default();
    for s in s.iter() {
        match s {
            &Sexp::Token(Token::Symbol(name)) => d.name = String::from(name),
            &Sexp::List(ref arr, _, _) => {                
                match arr[0].symbol() {
                    Some("vendor") => d.vendor = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                    Some("vendor_id") => d.vendor_id = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                    Some("name") => d.name = String::from(try!(read_name(ctx, &arr[1]))),
                    Some("size") => d.size = Some(try!(expect_u64(ctx, &arr[1]))),
                    Some("access") => d.access = Some(try!(expect_access(ctx, &arr[1]))),
                    Some("description") => d.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                    Some("interrupt-count") => d.interrupt_count = Some(try!(expect_u64(ctx, &arr[1]))),
                    Some("peripheral") => d.peripherals.push(try!(read_peripheral(ctx, &arr[1..]))),
                    Some("peripheral-group") => d.peripheral_groups.push(try!(read_peripheral_group(ctx, &arr[1..]))),
                    Some("exceptions") => d.exceptions.extend(try!(read_exceptions(ctx, &arr[1..]))),
                    Some("crate") => d.crates.push(try!(read_crate(ctx, &arr[1..]))),
                    Some("regions") => d.regions.extend(try!(read_regions(ctx, &arr[1..]))),
                    Some("variants") => d.variants.extend(try!(read_variants(ctx, &arr[1..]))),
                    Some("signal") => d.signals.push(try!(read_signal(ctx, &arr[1..]))),
                    Some("clock") => d.clocks.push(try!(read_clock(ctx, &arr[1..]))),
                    _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
                }
            }
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }
    }
    Ok(d)
}

fn read_exceptions(ctx: &Context, s: &[Sexp]) -> Result<Vec<Exception>, ReadError> {
    let path = ctx.path();
    let mut exceptions: Vec<Exception> = Vec::new();

    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("exception") => exceptions.push(try!(read_exception(ctx, &arr[1..]))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }        
    }

    Ok(exceptions)
}


fn read_exception(ctx: &Context, s: &[Sexp]) -> Result<Exception, ReadError> {
    let mut e = Exception::default();
    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => e.name = String::from(try!(expect_symbol(ctx, &arr[1]))),
                Some("description") => e.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s))),
        }
    }
    Ok(e)
}


fn read_crate(ctx: &Context, s: &[Sexp]) -> Result<Crate, ReadError> {
    let path = ctx.path();
    let mut c = Crate::default();

    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => c.name = String::from(try!(expect_symbol(ctx, &arr[1]))),
                Some("module") => c.modules.push(try!(read_module(ctx, &arr[1..]))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }        
    }

    Ok(c)
}

fn read_variants(ctx: &Context, s: &[Sexp]) -> Result<Vec<Variant>, ReadError> {
    let path = ctx.path();
    let mut variants: Vec<Variant> = Vec::new();

    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("variant") => variants.push(try!(read_variant(ctx, &arr[1..]))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }        
    }

    Ok(variants)
}

fn read_variant(ctx: &Context, s: &[Sexp]) -> Result<Variant, ReadError> {
    let mut v = Variant::default();
    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => v.name = String::from(try!(expect_symbol(ctx, &arr[1]))),
                Some("link") => v.link = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                Some("description") => v.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s))),
        }
    }
    Ok(v)
}


fn read_regions(ctx: &Context, s: &[Sexp]) -> Result<Vec<Region>, ReadError> {
    let path = ctx.path();
    let mut regions: Vec<Region> = Vec::new();

    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("region") => regions.push(try!(read_region(ctx, &arr[1..]))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }        
    }

    Ok(regions)
}

fn read_region(ctx: &Context, s: &[Sexp]) -> Result<Region, ReadError> {
    let path = ctx.path();
    let mut r = Region::default();

    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => r.name = String::from(try!(expect_symbol(ctx, &arr[1]))),
                Some("type") => r.rtype = String::from(try!(expect_symbol(ctx, &arr[1]))),
                Some("offset") => r.offset = try!(expect_u64(ctx, &arr[1])),
                Some("size") => r.size = try!(expect_u64(ctx, &arr[1])),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }        
    }

    Ok(r)
}

fn read_module(ctx: &Context, s: &[Sexp]) -> Result<Module, ReadError> {
    let mut m = Module::default();
    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => m.name = String::from(try!(expect_symbol(ctx, &arr[1]))),
                Some("as") => m._as = Some(String::from(try!(expect_symbol(ctx, &arr[1])))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s))),
        }
    }
    Ok(m)
}

fn read_feature(ctx: &Context, s: &[Sexp]) -> Result<Vec<String>, ReadError> {
    let mut features: Vec<String> = Vec::new();
    for s in s.iter() {
        // println!("{:?}", s);
        features.push(String::from(try!(expect_symbol(ctx, s))));
    }
    Ok(features)
}

fn read_peripheral_group(ctx: &Context, s: &[Sexp]) -> Result<PeripheralGroup, ReadError> {
    let path = ctx.path();
    let mut pg = PeripheralGroup::default();

    for s in s.iter() {
        // println!("{:?}", s);
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => pg.name = String::from(try!(read_name(ctx, &arr[1]))),
                Some("peripheral") => pg.peripherals.push(try!(read_peripheral(ctx, &arr[1..]))),
                Some("module") => pg.modules.push(try!(read_module(ctx, &arr[1..]))),
                Some("prototype") => {
                    let mut path_buf = path.parent().unwrap().to_path_buf();
                    path_buf.push(try!(expect_string(ctx, &arr[1])));                    
                    let mut buf: Vec<u8> = Vec::new();                    
                    let mut input = try!(File::open(&path_buf));
                    try!(input.read_to_end(&mut buf));

                    let ctx_new = Context::new(&path_buf, &buf);
                    let s_proto = try!(parse(&buf));

                    let p = if let Some((head, rest)) = s_proto.split_first() {
                        match head {
                            &Sexp::Token(Token::Symbol("peripheral")) => try!(read_peripheral(&ctx_new, rest)),
                            h @ _ => return Err(ReadError::Error(format!("{}: Expected peripheral, got {:?}", ctx_new.location_of(&s_proto), h))),
                        }
                    } else {
                        return Err(ReadError::Error(format!("{}: Expected list, got {:?}", ctx_new.location_of(&s_proto), &s_proto)))
                    };
                    pg.prototype = Some(p);
                },     
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }
    }

    Ok(pg)
}

fn read_peripheral(ctx: &Context, s: &[Sexp]) -> Result<Peripheral, ReadError> {
    let path = ctx.path();
    let mut p = Peripheral::default();
    for s in s.iter() {
        // println!("{:?}", s);
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {                
                Some("include") => {
                    let mut path_buf = path.parent().unwrap().to_path_buf();
                    path_buf.push(try!(expect_string(ctx, &arr[1])));                    
                    let mut buf: Vec<u8> = Vec::new();                    
                    let mut input = try!(File::open(&path_buf));
                    try!(input.read_to_end(&mut buf));

                    let ctx_new = Context::new(&path_buf, &buf);
                    let s_include = try!(parse(&buf));

                    p = if let Some((head, rest)) = s_include.split_first() {
                        match head {
                            &Sexp::Token(Token::Symbol("peripheral")) => try!(read_peripheral(&ctx_new, rest)),
                            h @ _ => return Err(ReadError::Error(format!("{}: Expected peripheral, got {:?}", ctx_new.location_of(&s_include), h))),
                        }
                    } else {
                        return Err(ReadError::Error(format!("{}: Expected list, got {:?}", ctx_new.location_of(&s_include), &s_include)))
                    };
                },        
                Some("module") => p.modules.push(try!(read_module(ctx, &arr[1..]))),
                Some("feature") => p.features.extend(try!(read_feature(ctx, &arr[1..]))),
                Some("derived-from") => p.derived_from = Some(String::from(try!(expect_symbol(ctx, &arr[1])))),
                Some("group-name") => p.group_name = Some(String::from(try!(expect_symbol(ctx, &arr[1])))),
                Some("name") => p.name = String::from(try!(read_name(ctx, &arr[1]))),
                Some("description") => p.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                Some("address") => p.address = try!(expect_u64(ctx, &arr[1])),
                Some("size") => p.size = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("access") => p.access = Some(try!(expect_access(ctx, &arr[1]))),
                Some("reset-value") => p.reset_value = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("reset-mask") => p.reset_mask = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("interrupt") => p.interrupts.push(try!(read_interrupt(ctx, &arr[1..]))),
                Some("signal") => p.signals.push(try!(read_signal(ctx, &arr[1..]))),
                Some("pin") => p.pins.push(try!(read_pin(ctx, &arr[1..]))),
                Some("cluster") => p.clusters.push(try!(read_cluster(ctx, &arr[1..]))),
                Some("register") => p.registers.push(try!(read_register(ctx, &arr[1..]))),
                Some("dim") => p.dim = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("dim-increment") => p.dim_increment = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("dim-index") => p.dim_index = Some(String::from(try!(expect_string(ctx, &arr[1])))),                
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }        
    }
    Ok(p)
}

fn read_pin(ctx: &Context, s: &[Sexp]) -> Result<Pin, ReadError> {
    let path = ctx.path();
    let mut p = Pin::default();

    for s in s.iter() {
        // println!("{:?}", s);
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => p.name = String::from(try!(read_name(ctx, &arr[1]))),
                Some("index") => p.index = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("altfn") => p.altfns.push(try!(read_altfn(ctx, &arr[1..]))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }
    }    

    Ok(p)
}

fn read_altfn(ctx: &Context, s: &[Sexp]) -> Result<AltFn, ReadError> {
    let path = ctx.path();
    let mut af = AltFn::default();

    if s.len() != 2 {
        return Err(ReadError::Error(format!("Expected index and signal for AltFn, got {:?}", s)));
    }

    af.index = try!(expect_u64(ctx, &s[0]));
    af.signal = String::from(try!(expect_symbol(ctx, &s[1])));

    Ok(af)
}

fn read_interrupt(ctx: &Context, s: &[Sexp]) -> Result<Interrupt, ReadError> {
    let mut i = Interrupt::default();
    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => i.name = String::from(try!(expect_symbol(ctx, &arr[1]))),
                Some("description") => i.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                Some("value") => i.value = try!(expect_u64(ctx, &arr[1])),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s))),
        }
    }
    Ok(i)
}

fn read_signal(ctx: &Context, s: &[Sexp]) -> Result<Signal, ReadError> {
    let mut sig = Signal::default();
    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => sig.name = String::from(try!(expect_symbol(ctx, &arr[1]))),
                Some("description") => sig.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s))),
        }
    }
    Ok(sig)
}

fn read_clock(ctx: &Context, s: &[Sexp]) -> Result<Clock, ReadError> {
    let mut clk = Clock::default();
    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => clk.name = String::from(try!(expect_symbol(ctx, &arr[1]))),
                Some("speed") => clk.speed = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("description") => clk.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s))),
        }
    }
    Ok(clk)
}


fn read_cluster(ctx: &Context, s: &[Sexp]) -> Result<Cluster, ReadError> {
    let mut c = Cluster::default();
    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => c.name = String::from(try!(read_name(ctx, &arr[1]))),
                Some("description") => c.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                Some("offset") => c.offset = try!(expect_u64(ctx, &arr[1])),
                Some("size") => c.size = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("access") => c.access = Some(try!(expect_access(ctx, &arr[1]))),
                Some("reset-value") => c.reset_value = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("reset-mask") => c.reset_mask = Some(try!(expect_u64(ctx, &arr[1]))),                
                Some("register") => c.registers.push(try!(read_register(ctx, &arr[1..]))),
                Some("dim") => c.dim = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("dim-increment") => c.dim_increment = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("dim-index") => c.dim_index = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }        
    }
    Ok(c)
}


fn read_register(ctx: &Context, s: &[Sexp]) -> Result<Register, ReadError> {
    let mut r = Register::default();
    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => r.name = String::from(try!(read_name(ctx, &arr[1]))),
                Some("description") => r.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),                
                Some("offset") => r.offset = try!(expect_u64(ctx, &arr[1])),
                Some("size") => r.size = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("access") => r.access = Some(try!(expect_access(ctx, &arr[1]))),
                Some("reset-value") => r.reset_value = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("reset-mask") => r.reset_mask = Some(try!(expect_u64(ctx, &arr[1]))),                
                Some("field") => r.fields.push(try!(read_field(ctx, &arr[1..]))),
                Some("dim") => r.dim = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("dim-increment") => r.dim_increment = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("dim-index") => r.dim_index = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s),arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}",  ctx.location_of(s), s)))
        }        
    }
    Ok(r)
}

fn read_field(ctx: &Context, s: &[Sexp]) -> Result<Field, ReadError> {
    let mut f = Field::default();
    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("name") => f.name = String::from(try!(read_name(ctx, &arr[1]))),
                Some("description") => f.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                Some("bit-offset") => f.bit_offset = try!(expect_u64(ctx, &arr[1])),
                Some("bit-width") => f.bit_width = try!(expect_u64(ctx, &arr[1])),
                Some("access") => f.access = Some(try!(expect_access(ctx, &arr[1]))),
                Some("value") => f.enumerated_values.push(try!(read_enumerated_value(ctx, &arr[1..]))),
                Some("dim") => f.dim = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("dim-increment") => f.dim_increment = Some(try!(expect_u64(ctx, &arr[1]))),
                Some("dim-index") => f.dim_index = Some(String::from(try!(expect_string(ctx, &arr[1])))),                     
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }        
    }
    Ok(f)
}

fn read_enumerated_value(ctx: &Context, s: &[Sexp]) -> Result<EnumeratedValue, ReadError> {
    let mut v = EnumeratedValue::default();
    for s in s.iter() {
        match s {
            &Sexp::List(ref arr, _, _) => match arr[0].symbol() {
                Some("value") => v.value = String::from(try!(expect_string(ctx, &arr[1]))),
                Some("name") => v.name = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                Some("description") => v.description = Some(String::from(try!(expect_string(ctx, &arr[1])))),
                _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), arr)))
            },
            _ => return Err(ReadError::Error(format!("{}: Unexpected item: {:?}", ctx.location_of(s), s)))
        }        
    }
    Ok(v)
}

fn read_name<'a>(ctx: &Context, s: &'a Sexp) -> Result<&'a str, ReadError> {
    match s {
        &Sexp::Token(Token::Symbol(s)) => {
            Ok(s)
        },
        &Sexp::Token(Token::String(s)) => {
            Ok(&s[1..(s.len()-1)])
        },
        _ => Err(ReadError::Error(format!("{}: Expected String or Symbol, got {:?}", ctx.location_of(s), s)))
    }
}


#[cfg(xtest)]
mod tests {
    use super::*;


    #[test]
    fn test_stm32f0xx() {
        let buf = include_bytes!("../testdata/STM32F0xx.rx");
        let _ = read_buf(&buf[..],Path::new("../testdata/STM32F0xx.rx")).unwrap();
    }

    #[test]
    fn test_mk64f12() {
        let buf = include_bytes!("../testdata/MK64F12.rx");
        let _ = read_buf(&buf[..]).unwrap();
    }

    #[test]
    fn test_tm4c123gh6pz() {
        let buf = include_bytes!("../testdata/TM4C123GH6PZ.rx");
        let _ = read_buf(&buf[..]).unwrap();
    }    

    #[test]
    fn test_atsamd21g18a() {
        let buf = include_bytes!("../testdata/ATSAMD21G18A.rx");
        let _ = read_buf(&buf[..]).unwrap();
    }

    #[test]
    fn test_lpc408x() {
        let buf = include_bytes!("../testdata/LPC408x_7x_v0.7.rx");
        let _ = read_buf(&buf[..]).unwrap();
    }    
    
    #[test]
    fn test_nrf51() {
        let buf = include_bytes!("../testdata/nrf51.rx");
        let _ = read_buf(&buf[..]).unwrap();
    }        
}