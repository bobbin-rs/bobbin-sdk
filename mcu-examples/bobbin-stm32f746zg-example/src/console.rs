#![no_std]
#![feature(asm)]

extern crate cortex_m_rt; // use the cortex-m-rt crate
extern crate panic_abort; // abort on panic
extern crate stm32f74x as mcu; // use the MCU crate

use mcu::bobbin_mcu::prelude::*;
use mcu::bobbin_hal::prelude::*;
use mcu::pin::*;
use mcu::ext::rcc::DedicatedClock;
use mcu::usart::*;

pub const USART: Usart3 = USART3;
const USART_TX: Pd8 = PD8;
const USART_RX: Pd9 = PD9;
const USART_CLOCK: u32 = 16_000_000; // Use HSI Clock
const USART_BAUD: u32 = 115_200;

fn main() {   
    USART_TX
        .port_gate_enable() // Enable the clock for the port associated with this pin.
        .connect_to(USART); // Connect the pin to the USART that we are using.

    USART_RX
        .port_gate_enable() // Enable the clock for the port associated with this pin.
        .connect_to(USART); // Connect the pin to the USART that we are using.

    USART
        .set_clock_source(DedicatedClock::Hsi) // use the 16MHz HSI clock source
        .gate_enable() // Enable the USART clock
        .set_config(|c| c.set_baud_clock(USART_BAUD, USART_CLOCK)) // Configure the USART
        .enable(); // Enable the USART

    loop {
        USART.write(b"Hello, World\r\n"); // Write a byte slice. Use `\r\n` for a newline.
        for _ in 0..10_000_000 { unsafe { asm!("nop") }}
    }    
}
