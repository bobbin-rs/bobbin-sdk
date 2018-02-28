use core::fmt::{self, Write};

/// Macro for sending `print!`-formatted messages over the Console
#[macro_export]
macro_rules! print {
    ($s:expr) => {
        $crate::console::write_str($s)
    };
    ($($arg:tt)*) => {
        $crate::console::write_fmt(format_args!($($arg)*))
    };
}

/// Macro for sending `print!`-formatted messages over the Console, with a
/// newline
#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\n"), $($arg)*)
    };
}

pub trait Putc {
    fn console_putc(&self, c: u8);
}

pub static mut CONSOLE: Option<Console> = None;

pub fn set_console(c: Console) {
    unsafe { CONSOLE = Some(c) }
}

pub struct Console(&'static Putc);

impl Console {
    pub fn new(other: &'static Putc) -> Self {
        Console(other)
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
