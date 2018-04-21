use prelude::*;

use mcu::uart::*;
use mcu::pin::*;
use clock::*;

pub const UART: Uart0 = UART0;
pub const UART_RX: Ptb16 = PTB16;
pub const UART_TX: Ptb17 = PTB17;
pub const UART_BD: u16 = 65;
pub const UART_BAUD: u32 = 115_200;

pub fn init() {
    // Enable Clocks
    UART.gate_enable();
    UART_TX.port().gate_enable();
    UART_RX.port().gate_enable();

    UART_TX.connect_to(UART);
    UART_RX.connect_to(UART);

    let baud_div = SystemClock::default().clock_for(UART).as_u32() / (16 * UART_BAUD);
    UART
        .set_config(|c| c.set_baud_divisor(baud_div as u16))
        .set_enabled(true);
    Console::set(Console::new(UART.as_periph(), ConsoleMode::Cooked));
}
