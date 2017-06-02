use core::fmt::{Write, Arguments};
//use pin;
//use usart;

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
    //let _ = usart::usart2(115_200);
}

fn console() -> ::hal::usart::UsartDevice {
    unsafe { usart::usart5() }
}

#[doc(hidden)]
pub fn write_fmt(args: Arguments) {    
    console().write_fmt(args).ok();
}

#[doc(hidden)]
pub fn write_str(s: &str) {
    console().write_str(s).ok();
}