use core::fmt::{Write, Arguments};
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

pub fn init() {
    usart::usart2(pin::pa2(), pin::pa15());
}

fn console() -> ::driver::usart::UsartDevice {
    unsafe { usart::usart2_unchecked(pin::pa2(), pin::pa15()) }
}

#[doc(hidden)]
pub fn write_fmt(args: Arguments) {    
    console().write_fmt(args).ok();
}

#[doc(hidden)]
pub fn write_str(s: &str) {
    console().write_str(s).ok();
}