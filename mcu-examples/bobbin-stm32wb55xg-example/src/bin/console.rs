#![no_std]
#![no_main]

extern crate bobbin_stm32wb55xg_example;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32wb55xx as mcu;

use bobbin_stm32wb55xg_example::init;
use cortex_m::asm;
use cortex_m_rt::entry;
use mcu::bobbin_hal::prelude::*;
use mcu::bobbin_mcu::prelude::*;
use mcu::ext::rcc::UsartClock;
use mcu::pin::*;
use mcu::usart::*;

// USART1 is connected to Nucleo's ST-Link F1 chip.
// Data is available via ST-Link's Virtual COM Port
pub const USART: Usart1 = USART1;

const USART_TX: Pb6 = PB6;
const USART_RX: Pb7 = PB7;

const USART_CLOCK: u32 = 64_000_000; // Use SYSCLK 64MHz (via PLL) Clock
const USART_BAUD: u32 = 115_200;

#[entry]
fn main() -> ! {
    init();

    USART_TX
        .port_gate_enable() // Enable the clock for the port associated with this pin.
        .connect_to(USART); // Connect the pin to the USART that we are using.

    USART_RX
        .port_gate_enable() // Enable the clock for the port associated with this pin.
        .connect_to(USART); // Connect the pin to the USART that we are using.

    USART
        .set_clock_source(UsartClock::Sysclk) // use the 64MHz SYSCLK clock source
        .gate_enable() // Enable the USART clock
        .set_config(|c| c.set_baud_clock(USART_BAUD, USART_CLOCK)) // Configure the USART
        .enable(); // Enable the USART

    loop {
        USART.write(b"Hello, World\r\n"); // Write a byte slice. Use `\r\n` for a newline.
        for _ in 0..1_000_000 {
            asm::nop()
        }
    }
}
