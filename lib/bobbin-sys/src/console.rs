use bobbin_hal::serial::SerialTx;

use core::fmt::{self, Write};

pub const DIGITS: &[u8; 16] = b"0123456789abcdef";    

static mut CONSOLE: Option<Console<'static>> = None;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConsoleMode {
    Raw,
    Cooked,
}

pub trait ConsoleWrite {
    fn write(&self, buf: &[u8]);
    fn putc(&self, b: u8);
}

impl<T: SerialTx<u8>> ConsoleWrite for T {
    fn write(&self, buf: &[u8]) {
        <Self as SerialTx<u8>>::write(self, buf);
    }
    fn putc(&self, b: u8) {
        <Self as SerialTx<u8>>::putc(self, b);
    }
}

#[derive(Clone)]
pub struct Console<'a>(&'a ConsoleWrite, ConsoleMode);

impl<'a> Console<'a> {

    pub fn borrow() -> Option<&'static Console<'static>> {
        unsafe { CONSOLE.as_ref() }
    }    

    pub fn set(console: Console<'static>) {
        unsafe { CONSOLE = Some(console) }
    }

    pub fn new(other: &'a ConsoleWrite, mode: ConsoleMode) -> Self {
        Console(other, mode)
    }

    pub fn write(&self, buf: &[u8]) {
        match self.1 {
            ConsoleMode::Raw => self.0.write(buf),
            ConsoleMode::Cooked => self.write_cooked(buf),
        }
    }

    pub fn writeln(&self, buf: &[u8]) {
        self.write(buf);
        self.write(b"\n");
    }


    pub fn write_cooked(&self, buf: &[u8]) {
        for byte in buf {
            if *byte == b'\n' {
                self.0.putc(b'\r');
            }
            self.0.putc(*byte);
        }        
    }

    pub fn write_u32(&self, mut v: u32, base: u32) {
        let mut buf = [0u8; 6];
        let mut i = buf.len();
        if v == 0 {
            self.write(b"0");            
        } else {
            if base == 16 {
                while v > 0 && i > 0 {
                    i -= 1;
                    buf[i] = DIGITS[(v % 16) as usize];
                    v = v / 16;
                }
            } else {
                while v > 0 && i > 0 {
                    i -= 1;
                    buf[i] = DIGITS[(v % 10) as usize];
                    v = v / 10;
                }        
            }
            self.write(&buf[i..])
        }
    }

    pub fn write_u8_hex(&self, v: u8) {
        self.write(&u8_to_hex(v));
    }

    pub fn write_u16_hex(&self, v: u16) {
        self.write(&u16_to_hex(v));
    }

    pub fn write_u32_hex(&self, v: u32) {
        self.write(&u32_to_hex(v));
    }
}

#[inline]
fn u8_to_hex(c: u8) -> [u8; 2] {
    [DIGITS[((c >> 4) & 0xf) as usize], DIGITS[(c & 0xf) as usize]]
}

#[inline]
fn u16_to_hex(c: u16) -> [u8; 4] {
    let (a, b) = (u8_to_hex((c >> 8) as u8), u8_to_hex(c as u8));
    [a[0], a[1], b[0], b[1]]
}

#[inline]
fn u32_to_hex(c: u32) -> [u8; 8] {
    let (a, b) = (u16_to_hex((c >> 16) as u16), u16_to_hex(c as u16));
    [a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3]]
}
    

impl<'a> fmt::Write for Console<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {        
        self.0.write(s.as_bytes());
        Ok(())
    }
}

#[doc(hidden)]
pub fn with_console<F: FnOnce(&mut Console)>(f: F) {
    unsafe {
        if let Some(ref mut console) = CONSOLE {
            f(console)
        }
    }
}

#[doc(hidden)]
pub fn write_fmt(args: fmt::Arguments) {    
    with_console(|c| {
        c.write_fmt(args).ok();
    });
}

#[doc(hidden)]
pub fn write_str(s: &str) {
    with_console(|c| {
        c.write_str(s).ok();
    });
}

#[doc(hidden)]
pub fn write(buf: &[u8]) {
    with_console(|c| {
        c.write(buf);
    });
}

#[doc(hidden)]
pub fn write_u32(v: u32, base: u32) {
    with_console(|c| {
        c.write_u32(v, base);
    });
}

#[doc(hidden)]
pub fn write_u8_hex(v: u8) {
    with_console(|c| {
        c.write_u8_hex(v);
    });
}

#[doc(hidden)]
pub fn write_u16_hex(v: u16) {
    with_console(|c| {
        c.write_u16_hex(v);
    });
}

#[doc(hidden)]
pub fn write_u32_hex(v: u32) {
    with_console(|c| {
        c.write_u32_hex(v);
    });
}