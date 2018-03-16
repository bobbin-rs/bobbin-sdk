pub use mcu::bobbin_common::console::*;
use common::periph::IntoPeriph;
use mcu::uart::*;
use mcu::pin::*;

pub const UART: Uart0 = UART0;
pub const UART_RX: Ptb16 = PTB16;
pub const UART_TX: Ptb17 = PTB17;
// pub const UART_BD: u16 = 65;
pub const UART_BAUD: u32 = 115_200;

pub fn init() {
    // Enable Clocks
    UART.gate_enable();
    UART_TX.port().gate_enable();
    UART_TX.connect_to(UART);
    UART_RX.port().gate_enable();
    UART_RX.connect_to(UART);

    // let baud_div = UART.clock(&CLK).expect("No bus clock") / (16 * UART_BAUD);
    UART
        .set_config(|c| c.set_baud_divisor(65))
        .enable();
    set_console(Console::new(UART.into_periph()));
    
}


// pub fn write_str(_s: &str) {
//     unimplemented!()
// }

// pub fn write_u32(_v: u32, _radix: u32) {
//     unimplemented!()
// }