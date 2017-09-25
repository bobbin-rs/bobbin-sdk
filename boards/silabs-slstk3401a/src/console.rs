use core::fmt::{self, Write, Arguments};
use hal::cmu::*;
use hal::gpio::*;
use hal::usart::*;
// use hal::clock::Clock;
// use clock::CLK;

pub const USART: Usart0 = USART0;
pub const USART_TX: Pa0 = PA0;
pub const USART_RX: Pa1 = PA1;
pub const USART_CS: Pa5 = PA5;
pub const USART_CLOCK: u32 = 40_000_000;
pub const USART_BAUD: u32 = 115_200;

// Assume HFRCO @ 19MHz
// HFSRCLK = 19MHZ
// HFCLK = 19MHz
// HFPERCLK = 19MHz

// Enable USB VCOM by setting PA5 High
// UART0 TX - PA0
// UART0 RX - PA1

// Pin Routing
// US0_CS: PA5 @ Location 2
// US0_TX: PA0 @ Location 0
// US0_RX: PA1 @ Location 0


pub fn init() {
    // Enable Clocks
    ::hal::gpio::init();
    CMU.with_ctrl(|r| r.set_hfperclken(true));
    USART0.cmu_enable();

    USART_TX.mode_push_pull_alt();
    USART_RX.mode_push_pull_alt();
    USART_CS.mode_push_pull().set_output(true);

    USART0.with_routepen(|r| r.set_txpen(1).set_rxpen(1));
    USART0.with_routeloc0(|r| r.set_rxloc(0).set_txloc(0));

    enable();
}

pub fn enable() {
    // Set USART0 Baud
    // Set USART0 Configuration
    // br = fHFPERCLK/(oversample x (1 + USARTn_CLKDIV/256))

    // USART_BAUD = USART_CLOCK / (oversample * (1 + CLKDIV / 256))
    // (oversample * (1 + CLKDIV / 256)) = USART_CLOCK / USART_BAUD
    // (1 + CLKDIV / 256)) = USART_CLOCK / USART_BAUD
    // CLKDIV = ((USART_CLOCK / USART_BAUD) - 1) * 256
    // USARTn_CLKDIV = 256 x (fHFPERCLK/(oversample x brdesired) - 1)

    let div = 256 * (USART_CLOCK / (16 * USART_BAUD) - 1);
    // let div = 5299 >> 3;
    let div = div >> 3;
    // let div = div / 8;

    USART.set_clkdiv(|r| r.set_div(div));
    USART.set_cmd(|r| r.set_rxen(1).set_txen(1));

}

pub fn disable() {
    // USART.disable();
    USART.set_cmd(|r| r.set_rxen(0).set_txen(0));
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

