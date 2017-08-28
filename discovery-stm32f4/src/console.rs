use core::fmt::{self, Write, Arguments};
use hal::gpio::*;
use hal::usart::*;
use hal::clock::Clock;
use clock::CLK;

pub const USART: Usart1 = USART1;
pub const USART_TX: Pb6 = PB6;
pub const USART_RX: Pb7 = PB7;
pub const USART_CLOCK: u32 = 84_000_000;
pub const USART_BAUD: u32 = 115_200;

// USART 1 / PB6 / PB7
// Clock @ 168_000_000
// APB2 @ Clock / 2 = 84_000_000

pub fn init() {
    // Enable Clocks
    USART.rcc_enable();
    USART_TX.port().rcc_enable();
    USART_RX.port().rcc_enable();

    // Set Pin Configuration
    USART_TX.mode_tx(&USART);
    USART_RX.mode_rx(&USART);

    // Set Baud and Enable USART
    USART.enable(USART.clock(&CLK).unwrap() / USART_BAUD);
}

pub fn disable() {
    USART.disable();
}

pub fn reinit() {
    USART.enable(USART.clock(&CLK).unwrap() / USART_BAUD);
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

