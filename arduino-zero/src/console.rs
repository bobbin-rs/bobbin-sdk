use core::fmt::{self, Write, Arguments};
use hal::port::*;
use hal::sercom::*;
use hal::gclk;

pub const SERCOM: Sercom5 = SERCOM5;
pub const SERCOM_TX: Pb22 = PB22;
pub const SERCOM_RX: Pb23 = PB23;

pub fn init() {
    SERCOM.pm_set_enabled(true);
    SERCOM_RX.port.pm_set_enabled(true);
    SERCOM_TX.port.pm_set_enabled(true);
    // Set GCLK_GEN0 as source for SERCOM

    gclk::GCLK.set_clkctrl(|r| r
        .set_id(0x14 + 5)
        .set_gen(0x0)
        .set_clken(1)
    );
    // Wait for synchronization
    while gclk::GCLK.status().syncbusy() != 0 {}

    // Set Pin Configuration
    SERCOM_TX.mode_pad_2(&SERCOM);
    SERCOM_RX.mode_pad_3(&SERCOM);

    // let _rx = pin::pb22().into_pmux(PMux::PMuxD);
    // let _rx = pin::pb23().into_pmux(PMux::PMuxD);
    // let u = usart::device(SERCOM5);
    SERCOM.configure(63018, 1, 3);   
    SERCOM.set_enabled(true);
}

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

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {        
        let uart = SERCOM;
        for byte in s.as_bytes().iter().cloned() {
            if byte == b'\n' {
                uart.putc(b'\r');
            }
            uart.putc(byte);
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