#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::common::*;
use board::hal::port::*;
use board::hal::uart::*;

use board::hal::clock::Clock;
use board::clock::CLK;

// D0 / PTC16 => UART3RX

pub const UART: Uart3 = UART3;
pub const UART_RX: Ptc16 = PTC16;
pub const UART_TX: Ptc17 = PTC17;
pub const UART_BAUD: u32 = 100000;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    UART.sim_enable();
    UART_RX.port().sim_enable();
    UART_RX.set_pull_up();
    UART_RX.mode_rx(&UART);

    UART_TX.port().sim_enable();
    UART_TX.set_pull_up();
    UART_TX.mode_tx(&UART);
    
    let baud_divider = (UART.clock(&CLK).expect("No bus clock") / (16 * UART_BAUD)) as u16;

    UART
        .with_s2(|r| r.set_rxinv(0))
        .with_c1(|r| r.set_pe(0))
        .with_c3(|r| r.set_txinv(0))
        .with_bdh(|r| r.set_sbns(1).set_sbr((baud_divider >> 8) as u8))
        .with_bdl(|r| r.set_sbr(baud_divider as u8))                
        .with_c2(|r| r.set_te(1).set_re(1));       

    println!("running");    
    let mut payload = [0u8; 25];    
    let mut channels = [0u16; 18];    
    let mut frame_lost: bool;
    let mut failsafe: bool;

    // loop {
    //     UART.putc(b'A');
    //     board::delay(10);
    // }

    let mut n = 0;

    loop {
        while UART.getc() != 0x0f {}
        for i in 0..24 {
            payload[i] = UART.getc();
        }

        
        if n == 10 {
            channels[0]  = (payload[0] as u16) | (payload[1] as u16) << 8 & 0x07FF;
            channels[1]  = (payload[1] as u16 >> 3 | (payload[2] as u16) << 5 ) & 0x07FF;
            channels[2]  = (payload[2] as u16 >> 6 | (payload[3] as u16) << 2 | (payload[4] as u16) << 10)	& 0x07FF;
            channels[3]  = (payload[4] as u16 >> 1 | (payload[5] as u16) << 7) & 0x07FF;
            channels[4]  = (payload[5] as u16 >> 4 | (payload[6] as u16) << 4) & 0x07FF;
            channels[5]  = (payload[6] as u16 >> 7 | (payload[7] as u16) << 1 | (payload[8] as u16) << 9) & 0x07FF;
            channels[6]  = (payload[8] as u16 >> 2 | (payload[9] as u16) << 6) & 0x07FF;
            channels[7]  = (payload[9] as u16 >> 5 | (payload[10] as u16) << 3) & 0x07FF;
            channels[8]  = (payload[11] as u16 | (payload[12] as u16) << 8) & 0x07FF;
            channels[9]  = (payload[12] as u16 >> 3 | (payload[13] as u16) << 5) & 0x07FF;
            channels[10] = (payload[13] as u16 >> 6 | (payload[14] as u16) << 2| (payload[15] as u16) << 10) & 0x07FF;
            channels[11] = (payload[15] as u16 >> 1 | (payload[16] as u16) << 7) & 0x07FF;
            channels[12] = (payload[16] as u16 >> 4 | (payload[17] as u16) << 4) & 0x07FF;
            channels[13] = (payload[17] as u16 >> 7 | (payload[18] as u16) << 1| (payload[19] as u16) << 9) & 0x07FF;
            channels[14] = (payload[19] as u16 >> 2 | (payload[20] as u16) << 6) & 0x07FF;
            channels[15] = (payload[20] as u16 >> 5 | (payload[21] as u16) << 3) & 0x07FF;

            let p = payload[22];
            channels[16] = if ((p & 0x01) != 0) { 2047 } else { 0 };
            channels[17] = if ((p & 0x02) != 0) { 2047 } else { 0 };
            frame_lost = (p & 0x04) != 0;
            failsafe = (p & 0x08) != 0;

            // for i in 22..24 {
            //     print!("{:02x} ", payload[i]);
            // }
            // println!("{} {}", frame_lost, failsafe);
        
            for i in 0..4 {
                print!("{:04} ", channels[i]);
            }
            println!("");
            // println!("{:04} {:04} {:04} {:04}", channels[0], channels[1], channels[2], channels[3]);
            // println!("{:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}", payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6]);
            n = 0;
        } else {
            n += 1;
        }


        
    }
}
