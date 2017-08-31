use core::fmt::{self, Write, Arguments};
use hal::gpio::*;
use hal::uart::*;
use clock::{self, Clock};

pub const UART: Uart0 = UART0;
pub const UART_RX: Pa0 = PA0;
pub const UART_TX: Pa1 = PA1;
pub const UART_BAUD: u32 = 115_200;

pub fn init() {
    // Enable Clocks
    UART.sysctl_enable();
    UART_TX.port().sysctl_enable();
    UART_RX.port().sysctl_enable();

    // Set Pin Configuration
    UART_TX.mode_tx(&UART);
    UART_RX.mode_rx(&UART);

    // Set Baud and Enable uart

    // UART.configure(UART_BAUD, UART.clock(clock::clk()).unwrap());
    // UART.configure(UART_BAUD, clock::sysclk_hz());
    // UART.configure(UART_BAUD, 120_000_000);
    enable();
}

pub fn enable() {
    UART
        .set_config(|cfg| cfg.set_baud(UART_BAUD, UART.clock(clock::clk()).unwrap()))
        .enable();
}

pub fn disable() {
    UART.disable();    
}

pub fn reinit() {
    UART.enable();    
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

