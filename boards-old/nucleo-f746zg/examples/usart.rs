#![no_std]
#![feature(asm)]

extern crate nucleo_f746zg as board;

// use board::mcu::rcc::*;
use board::mcu::usart::*;
use board::mcu::pin::*;

const USART: Usart3 = USART3;
const USART_TX: Pd8 = PD8;
const USART_RX: Pd9 = PD9;
const USART_CLOCK: u32 = 16_000_000; // Use HSI Clock
const USART_BAUD: u32 = 115_200;

fn main() {
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
        USART.write(b"Tick...\n");
        board::delay(500);
    }
}
