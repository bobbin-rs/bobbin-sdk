#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate frdm_k64f as board;

use board::common::*;
use board::common::hal::digital::*;
use board::hal::port::*;
use board::hal::uart::*;
use board::btn::*;
use board::hal::clock::Clock;
use board::clock::CLK;

// D0 / PTC16 => UART3RX

pub const UART: Uart3 = UART3;
pub const UART_RX: Ptc16 = PTC16;
pub const UART_TX: Ptc17 = PTC17;
pub const UART_BAUD: u32 = 125000;

pub const PWR: Ptb9 = PTB9;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    UART.sim_enable();
    UART_RX.port().sim_enable();
    UART_RX.set_pull_up();
    UART_RX.gpio_pin().set_output(true);
    UART_RX.set_mux_gpio();


    PWR.port().sim_enable();
    PWR.set_mux_gpio();
    PWR.set_pull_down();
    PWR.gpio_pin()
        .set_dir_output()
        .set_output(false);
    board::delay(500);
    PWR.gpio_pin().set_output(true);
    board::delay(10);
    PWR.gpio_pin().set_output(false);
    board::delay(10);
    PWR.gpio_pin().set_output(true);

    // loop {
    //     PWR.gpio_pin().toggle_output();
    //     board::delay(150);
    // }



    board::delay(1);
    // if !BTN0.input() {
    if true {
        // board::delay(50);

        let rx = UART_RX.gpio_pin();

        rx.set_dir_output();
        for _ in 0..9 {
            rx.set_output(true);    
            for i in 0..50000 { unsafe { asm!("nop") }}
            rx.set_output(false);    
            for i in 0..50000 { unsafe { asm!("nop") }}
            // board::delay(1);
        }
        rx.set_output(true);
        println!("BIND Mode");

        loop {}
    }

    UART_RX.mode_rx(&UART);

    UART_TX.port().sim_enable();
    UART_TX.set_pull_up();
    UART_TX.mode_tx(&UART);
    
    let baud_divider = (UART.clock(&CLK).expect("No bus clock") / (16 * UART_BAUD)) as u16;

    UART
        .with_s2(|r| r.set_rxinv(0))
        .with_c1(|r| r.set_pe(0))
        .with_c3(|r| r.set_txinv(0))
        .with_bdh(|r| r.set_sbns(0).set_sbr((baud_divider >> 8) as u8))
        .with_bdl(|r| r.set_sbr(baud_divider as u8))                
        .with_c2(|r| r.set_te(1).set_re(1));       

    println!("Running");
    loop {
        let c = UART.getc();
        println!("{:02x}", c);
    } 
}
