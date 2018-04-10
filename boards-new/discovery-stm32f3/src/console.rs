pub use mcu::bobbin_common::console::*;
use mcu::bobbin_common::periph::IntoPeriph;

use mcu::rcc::*;
use mcu::usart::*;
use mcu::pin::*;
// use ::clock::*;

pub const USART: Usart1 = USART1;
pub const USART_TX: Pc4 = PC4;
pub const USART_RX: Pc5 = PC5;
const USART_CLOCK: u32 = 8_000_000; // Use HSI Clock
const USART_BAUD: u32 = 115_200;


pub fn init() {
    USART_TX
        .port_gate_enable()
        .connect_to(USART);

    USART_RX
        .port_gate_enable()
        .connect_to(USART);

    USART
        .set_clock_source(UsartClock::Hsi)
        .gate_enable()
        .set_config(|c| c.set_baud_clock(USART_BAUD, USART_CLOCK))
        .enable();

    set_console(Console::new(USART.into_periph()));
}

impl super::DiscoveryStm32f3 {
    pub fn console(&self) -> Console {
        Console::new(USART.into_periph())
    }
}