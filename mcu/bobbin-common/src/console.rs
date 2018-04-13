use core::fmt::{self, Write};

pub trait Putc {
    fn console_putc(&self, c: u8);
}

pub const DIGITS: &[u8; 16] = b"0123456789abcdef";    
pub static mut CONSOLE: Option<Console> = None;

pub fn set_console(c: Console) {
    unsafe { CONSOLE = Some(c) }
}

pub struct Console(&'static Putc);

impl Console {
    pub fn new(other: &'static Putc) -> Self {
        Console(other)
    }

    pub fn write(&self, buf: &[u8]) {
        for byte in buf {
            if *byte == b'\n' {
                self.0.console_putc(b'\r');
            }
            self.0.console_putc(*byte);
        }
    }

    pub fn write_u32(&self, mut v: u32, base: u32) {
        let mut buf = [0u8; 6];
        let mut i = buf.len();
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

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {        
        for byte in s.as_bytes().iter().cloned() {
            if byte == b'\n' {
                self.0.console_putc(b'\r');
            }
            self.0.console_putc(byte);
        }
        Ok(())
    }
}

pub fn console_borrow() -> Option<&'static Console> {
    unsafe { CONSOLE.as_ref() }
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