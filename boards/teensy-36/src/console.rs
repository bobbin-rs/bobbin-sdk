use core::fmt::{self, Write, Arguments};
use hal::port::*;
use hal::uart::*;

pub const UART: Uart0 = UART0;
pub const UART_RX: Ptb16 = PTB16;
pub const UART_TX: Ptb17 = PTB17;
pub const UART_BD: u16 = 65;

pub fn init() {
    // Enable Clocks
    UART.sim_enable();
    UART_TX.port().sim_enable();
    UART_RX.port().sim_enable();

    // Set Pin Configuration
    UART_TX.mode_tx(&UART);
    UART_RX.mode_rx(&UART);

    // Set Baud and Enable USART
    UART
        .set_config(|c| c.set_baud_divisor(UART_BD))
        .enable();  
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