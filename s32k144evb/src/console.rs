use core::fmt::{self, Write, Arguments};
use serial;

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

pub fn init() {
    serial::serial1();
}

pub const CONSOLE: Console = Console {};

pub struct Console {}

impl Console {
    pub fn uart(&self) -> ::hal::lpuart::LpuartDevice {
        serial::serial1_unchecked()
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {        
        let uart = self.uart();
        for byte in s.as_bytes().iter().cloned() {
            if byte == b'\n' {
                uart.putc(b'\r')
            }
            uart.putc(byte)
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn write_fmt(args: Arguments) {    
    CONSOLE.write_fmt(args).ok();
}

#[doc(hidden)]
pub fn write_str(s: &str) {
    CONSOLE.write_str(s).ok();
}