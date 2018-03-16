pub use mcu::bobbin_common::console::*;
use common::periph::IntoPeriph;
use common::configure::Configure;
use mcu::enabled::Enabled;
use mcu::uart::*;
use mcu::pin::*;
use clock::*;

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

    let baud_div = tree().u32_for(UART) / (16 * UART_BAUD);
    // let baud_div = UART.clock(&CLK).expect("No bus clock") / (16 * UART_BAUD);
    UART
        .set_config(|c| c.set_baud_divisor(baud_div as u16))
        .enable();
    set_console(Console::new(UART.into_periph()));
    
}


// pub fn write_str(_s: &str) {
//     unimplemented!()
// }

// pub fn write_u32(_v: u32, _radix: u32) {
//     unimplemented!()
// }