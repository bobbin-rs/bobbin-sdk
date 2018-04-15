pub use bobbin_sys::console::*;

use mcu::rcc::*;
use mcu::usart::*;
use mcu::pin::*;

pub const USART: Usart3 = USART3;
const USART_TX: Pd8 = PD8;
const USART_RX: Pd9 = PD9;
const USART_CLOCK: u32 = 16_000_000; // Use HSI Clock
const USART_BAUD: u32 = 115_200;

// pub const CONSOLE: UsartConsole = UsartConsole { periph: USART3_PERIPH };

pub fn init() {
    USART_TX
        .port_gate_enable()
        .connect_to(USART);

    USART_RX
        .port_gate_enable()
        .connect_to(USART);

    USART
        .set_clock_source(DedicatedClock::Hsi)
        .gate_enable()
        .set_config(|c| c.set_baud_clock(USART_BAUD, USART_CLOCK))
        .enable();

    set_console(&USART3_PERIPH, ConsoleMode::Cooked);
}
