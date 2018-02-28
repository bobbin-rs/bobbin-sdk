use core::fmt;

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

pub struct Console<T> {}

impl<T> Write for Console<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut c = T::default();
        for byte in s.as_bytes().iter().cloned() {
            if byte == b'\n' {
                c.putc(b'\r');
            }
            c.putc(byte);
        }
        Ok(())
    }
}
