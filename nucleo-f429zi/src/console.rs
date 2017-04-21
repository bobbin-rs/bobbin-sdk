use core::fmt::{self, Write, Arguments};
use pin;
use usart;

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

pub const CONSOLE: Console = Console {};

pub struct Console {}

impl Console {
    pub fn init(&self, _baud: u32) {
        usart::usart3(pin::pd8(), pin::pd9());
    }

    pub fn usart(&self) -> ::hal::usart::UsartDevice {
        unsafe { usart::usart3_unchecked(pin::pd8(), pin::pd9()) }
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {        
        let usart = self.usart();
        for byte in s.as_bytes().iter().cloned() {
            if byte == b'\n' {
                usart.putc(b'\r')
            }
            usart.putc(byte)
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