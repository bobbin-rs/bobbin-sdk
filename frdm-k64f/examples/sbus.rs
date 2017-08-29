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
        .with_s2(|r| r.set_rxinv(1).set_msbf(0))
        .with_c1(|r| r.set_pe(0))
        .with_c3(|r| r.set_txinv(1))
        .with_bdh(|r| r.set_sbns(1).set_sbr((baud_divider >> 8) as u8))
        .with_bdl(|r| r.set_sbr(baud_divider as u8))                
        .with_c2(|r| r.set_te(1).set_re(1));       

    println!("running");    
    let mut payload = [0u8; 25];    
    let mut channels = [0u16; 16];

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

    	channels[0]  = ((payload[0] as u16) | (payload[1] as u16) << 8) & 0x07FF;
    	channels[1]  = ((payload[1] as u16 >> 3 | (payload[2] as u16) <<5 ) & 0x07FF);
    	channels[2]  = ((payload[2] as u16 >> 6 | (payload[3] as u16) << 2 | (payload[4] as u16) << 10)	& 0x07FF);
    	channels[3]  = ((payload[4] as u16 >> 1 | (payload[5] as u16) << 7) & 0x07FF);
    	// channels[4]  = (int16_t) ((payload[5]>>4 |payload[6]<<4)                          & 0x07FF);
    	// channels[5]  = (int16_t) ((payload[6]>>7 |payload[7]<<1 |payload[8]<<9)   		& 0x07FF);
    	// channels[6]  = (int16_t) ((payload[8]>>2 |payload[9]<<6)                          & 0x07FF);
    	// channels[7]  = (int16_t) ((payload[9]>>5 |payload[10]<<3)                         & 0x07FF);
    	// channels[8]  = (int16_t) ((payload[11]   |payload[12]<<8)                         & 0x07FF);
    	// channels[9]  = (int16_t) ((payload[12]>>3|payload[13]<<5)                         & 0x07FF);
    	// channels[10] = (int16_t) ((payload[13]>>6|payload[14]<<2|payload[15]<<10) 		& 0x07FF);
    	// channels[11] = (int16_t) ((payload[15]>>1|payload[16]<<7)                         & 0x07FF);
    	// channels[12] = (int16_t) ((payload[16]>>4|payload[17]<<4)                         & 0x07FF);
    	// channels[13] = (int16_t) ((payload[17]>>7|payload[18]<<1|payload[19]<<9)  		& 0x07FF);
    	// channels[14] = (int16_t) ((payload[19]>>2|payload[20]<<6)                         & 0x07FF);
    	// channels[15] = (int16_t) ((payload[20]>>5|payload[21]<<3)                         & 0x07FF);


        
        if n == 16 {
        //     // println!("  {:016b} {:016b} {:016b} {:016b}", channels[0], channels[1], channels[2], channels[3]);
        //     for i in 0..6 {
        //         print!("{:08b}", payload[i]);
        //     }
        //     println!("");
        //     n = 0;
            println!("{:04} {:04} {:04} {:04}", channels[0], channels[1], channels[2], channels[3]);
            // println!("{:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}", payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6]);
        } else {
            n += 1;
        }


        
    }
}
