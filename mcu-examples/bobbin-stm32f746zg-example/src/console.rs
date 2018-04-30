#![no_std]
#![feature(asm)]

extern crate cortex_m_rt;
extern crate panic_abort;
extern crate stm32f74x as mcu;

use mcu::bobbin_mcu::prelude::*;
use mcu::bobbin_hal::prelude::*;
use mcu::ext::rcc::DedicatedClock;
use mcu::usart::*;
use mcu::pin::*;

pub const USART: Usart3 = USART3;
const USART_TX: Pd8 = PD8;
const USART_RX: Pd9 = PD9;
const USART_CLOCK: u32 = 16_000_000; // Use HSI Clock
const USART_BAUD: u32 = 115_200;

fn main() {
    mcu::ext::init();
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

    loop {
        USART.write(b"Hello, World\r\n");
        for _ in 0..10_000_000 { unsafe { asm!("nop") }}
    }    
}
