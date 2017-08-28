use core::fmt::{self, Write, Arguments};
use hal::gpio::*;
use hal::usart::*;
use hal::clock::Clock;
use chip::afio::*;
use clock::CLK;

pub const USART: Usart2 = USART2;
pub const USART_TX: Pa2 = PA2;
pub const USART_RX: Pa3 = PA3;
pub const USART_CLOCK: u32 = 36_000_000;
pub const USART_BAUD: u32 = 115_200;

pub fn init() {
    // Enable Clocks
    USART.rcc_enable();
    USART_TX.port().rcc_enable();
    USART_RX.port().rcc_enable();

    AFIO.rcc_enable();
    AFIO.with_mapr(|r| r.set_usart1_remap(1));

    USART_TX.mode_alt_fn();
    USART_RX.mode_input();

    // Set Baud and Enable USART
    USART.enable(USART.clock(&CLK).expect("No clock available") / USART_BAUD);
}

pub fn disable() {
    USART.disable();
}

pub fn reinit() {
    USART.disable().enable(USART.clock(&CLK).expect("No clock available") / USART_BAUD);
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
        let usart = USART;
        for byte in s.as_bytes().iter().cloned() {
            if byte == b'\n' {
                usart.putc(b'\r');
            }
            usart.putc(byte);
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

