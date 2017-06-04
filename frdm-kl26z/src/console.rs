// PA1 / PA2
// sim::set_uart0_enabled(true);  
// sim::set_uart0_src(0x1);  

// let tx = tx.into_altfn(2);
// let rx = rx.into_altfn(2);
// let u = uart0::device(UART0, tx, rx, 104);    

use core::fmt::{self, Write, Arguments};
use hal::port::*;
use hal::uart0::*;

pub const UART: Uart0 = UART0;
pub const UART_RX: Pta1 = PTA1;
pub const UART_TX: Pta2 = PTA2;
pub const UART_BD: u16 = 104;

pub fn init() {
    use hal::uart0::Uart0Ext;

    // Enable Clocks
    UART.sim_enable();
    UART.sim_set_src(0x1);
    // sim_src(0x1);
    
    UART_TX.port().sim_enable();
    UART_RX.port().sim_enable();

    // Set Pin Configuration
    UART_TX.mode_tx(&UART);
    UART_RX.mode_rx(&UART);

    // Set Baud and Enable USART
    UART.enable(UART_BD);
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
                uart.putc(b'\r')
            }
            uart.putc(byte)
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