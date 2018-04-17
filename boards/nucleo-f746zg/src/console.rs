use bobbin_sys::console::*;
use bobbin_mcu::AsPeriph;
use bobbin_mcu::clock::ClockSource;
use bobbin_mcu::gate::GateEn;
use bobbin_mcu::pin::{PortGateEn, ConnectTo, Pin};
use bobbin_hal::configure::Configure;
use bobbin_hal::enabled::Enabled;

use bobbin_mcu::pin::SetSource;

use mcu::ext::rcc::DedicatedClock;
// use mcu::ext::gpio::SetSource;
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

    set_console(Console::new(USART.as_periph(), ConsoleMode::Cooked));
}
