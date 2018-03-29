use core::fmt::{self, Arguments, Write};

use chip::itm::*;

/// Macro for sending `print!`-formatted messages to the ITM (Instrumentation
/// Trace Macrocell).
#[macro_export]
macro_rules! iprint {
    ($s:expr) => {
        $crate::itm::write_str($s)
    };
    ($($arg:tt)*) => {
        $crate::itm::write_fmt(format_args!($($arg)*))
    };
}

/// Macro for sending `print!`-formatted messages to the ITM, with a newline
#[macro_export]
macro_rules! iprintln {
    ($fmt:expr) => {
        iprint!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        iprint!(concat!($fmt, "\n"), $($arg)*)
    };
}

pub struct Port {}

impl Port {
    pub fn write_u8(&self, value: u8) {
        while ITM.stim(0).data() == 0 {}
        ITM.set_stim8(0, |_| Stim8(value));
    }

    pub fn write_u16(&self, value: u16) {
        while ITM.stim(0).data() == 0 {}
        ITM.set_stim16(0, |_| Stim16(value));
    }    

    pub fn write_u32(&self, value: u32) {
        while ITM.stim(0).data() == 0 {}
        ITM.set_stim(0, |_| Stim(value));
    }    

    pub fn write(&self, value: &[u8]) {
        for b in value.iter() {
            self.write_u8(*b)
        }
    }
}

impl Write for Port {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        self.write(value.as_bytes());
        Ok(())
    }
}

#[doc(hidden)]
pub fn write_fmt(args: Arguments) {
    Port {}.write_fmt(args).ok();
}

#[doc(hidden)]
pub fn write_str(s: &str) {
    Port {}.write_str(s).ok();
}
