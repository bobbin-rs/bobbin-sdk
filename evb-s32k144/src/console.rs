use core::fmt::{self, Write, Arguments};
use hal::port::*;
use hal::lpuart::*;
use hal::pcc::ClockSource;
use hal::clock::Clock;
use clock::CLK;

pub const UART: Lpuart1 = LPUART1;
pub const UART_CLK: ClockSource = ClockSource::SPLLDIV2;
pub const UART_RX: Ptc6 = PTC6;
pub const UART_TX: Ptc7 = PTC7;
pub const UART_BAUD: u32 = 115_000;
//pub const UART_BD: u16 = 22;

pub fn init() {
    // Clock source must be set before enabling clock
    UART.pcc_set_enabled(false);
    UART.pcc_set_clock_source(UART_CLK);
    UART.pcc_set_enabled(true);

    UART.with_global(|r| r.set_rst(1));
    UART.with_global(|r| r.set_rst(0));

    UART_TX.port().pcc_enable();
    UART_RX.port().pcc_enable();

    // Set Pin Configuration
    UART_TX.mode_tx(&UART);
    UART_RX.mode_rx(&UART);

    let sbr = UART.clock(&CLK).unwrap() / (UART_BAUD << 4);

    // Set Baud and Enable USART
    UART
        .set_osr(0b1111)
        .set_sbr(sbr as u16)
        .set_te(true)
        .set_re(true)
        .set_txfe(true)
        .set_rxfe(true);
}

pub fn disable() {
    UART
        .set_te(false)
        .set_re(false)
        .set_txfe(false)
        .set_rxfe(false);
}

pub fn reinit() {
    let sbr = UART.clock(&CLK).unwrap() / (UART_BAUD << 4);

    // Set Baud and Enable USART
    UART
        .set_osr(0b1111)
        .set_sbr(sbr as u16)
        .set_te(true)
        .set_re(true)
        .set_txfe(true)
        .set_rxfe(true);    
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
        let uart = UART;
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