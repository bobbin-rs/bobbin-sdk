use core::fmt::{self, Write, Arguments};
use hal::gpio::*;
use hal::uart::*;
use chip::clkgen::*;
use chip::pwrctrl::*;
// use hal::clock::Clock;
// use chip::afio::*;
// use clock::CLK;

pub const UART: Uart0 = UART0;
pub const UART_TX: Gp22 = GP22;
pub const UART_RX: Gp23 = GP23;
pub const UART_CLOCK: u32 = 48_000_000;
pub const UART_BAUD: u32 = 115_200;

pub fn init() {
    GPIO.unlock();
    // UART_TX
    //     .set_pad_gpio()
    //     .set_gpio_output_pushpull();
    UART_TX.mode_uart_tx(&UART);
    UART_RX.mode_uart_rx(&UART);

    // Enable Power
    PWRCTRL.with_deviceen(|r| r.set_uart0(1));
    // Automatic HCLK speed
    CLKGEN.with_uarten(|r| r.set_uart0en(0x1));


    UART.init();
    enable();
}

pub fn enable() {
    UART.enable();

    // Set Baud and Enable UART    
    // UART
    //     .set_config(|c| c.set_baud(UART_BAUD, UART.clock(&CLK).unwrap()))
    //     .enable();
}

pub fn disable() {
    UART.disable();
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
        for byte in s.as_bytes().iter().cloned() {
            if byte == b'\n' {
                UART.putc(b'\r');
            }
            UART.putc(byte);
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

