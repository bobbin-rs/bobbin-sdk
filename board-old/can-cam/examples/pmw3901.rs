#![no_std]
#![no_main]

#[macro_use]
extern crate can_cam as board;

use board::hal::gpio::*;
use board::hal::spi::*;

// https://github.com/WSCKY/STM32F746G-Discovery/blob/5e8f619a67d7aeb9385b228e98e6f91a7204f90a/Projects/STemWin_FreeRTOS_PMW3901MB/Src/PMW3901MB.c

// const float FlowOdometry::_degrees_per_pixel(4.2);
// https://github.com/jeskesen/kobayashi_maru/blob/master/kobayashi_maru_firmware/FlowOdometry.cpp

// Speed - 7.4 rad/s

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("PMW3901 Test");


    let spi = SPI4;
    let port = GPIOE;
    let spi_miso = PE13;
    let spi_mosi = PE14;
    let spi_sck = PE12;
    let spi_nss = PE11;

    spi.rcc_enable();
    port.rcc_enable();
    GPIOD.rcc_enable();

    // NOTE: Pins must be set with output speed HIGH or leading edge
    // of transmission will occasionally be missed.

    println!("Testing SPI");

    spi_miso.mode_spi_miso(&spi).speed_high().pull_up();
    spi_mosi.mode_spi_mosi(&spi).speed_high().push_pull();
    spi_sck.mode_spi_sck(&spi).speed_high().push_pull();
    spi_nss.mode_output();

    spi.set_config(|cfg| cfg
        .set_data_size(DataSize::Bits8)
        .set_master(true)
        .set_baud_divider(0b011.into())
    );
    spi.with_cr1(|r| r.set_cpha(1).set_cpol(1));
    spi.with_cr2(|r| r.set_frxth(1));
    spi.set_output_enabled(true).set_enabled(true);

    spi_nss.set_output(true);
    board::delay(1);
    spi_nss.set_output(false);
    board::delay(1);
    spi_nss.set_output(true);
    board::delay(1);

    // Power-Up Reset
    reg_write(&spi, &spi_nss, 0x3a, 0x5a);

    board::delay(50);

    read_motion_count(&spi, &spi_nss);
    board::delay(1);

    println!("Initializing");
    for &(r, v) in INIT1.iter() {
        reg_write(&spi, &spi_nss, r, v);
        board::delay(1);
    }
    board::delay(10);
    for &(r, v) in INIT2.iter() {
        reg_write(&spi, &spi_nss, r, v);
        board::delay(1);
    }
    println!("Init Complete");

    println!("0x00: 0x{:02x} - should be 0x49", reg_read(&spi, &spi_nss, 0x00));    
    println!("0x5f: 0x{:02x} - should be 0xb6", reg_read(&spi, &spi_nss, 0x5f));
    // println!("0x01: 0x{:02x} - should be 0x00", reg_read(&spi, &spi_nss, 0x01));


    

    // {
    //     println!("Raw Data Grab");
    //     for i in 0..0xff {
    //         println!("{:02x}", i);
    //         reg_write(&spi, &spi_nss, 0x58, i);
    //         // for j in 0..2 {
    //             let status = reg_read(&spi, &spi_nss, 0x59);
    //             if status != 0x00 {
    //                 println!("  status: {:02x}", status);
    //             }
    //             board::delay(50);
    //         // }
    //     }
    // }
    // println!("raw data grab");
    // reg_write(&spi, &spi_nss, 0x58, 0xff);
    // for _ in 0..100 {
    //     let status = reg_read(&spi, &spi_nss, 0x58);
    //     if status != 0x00 {
    //         println!("  status: {:02x}", status);
    //     }
    //     board::delay(50);
    // }
    // loop {}


    // Burst Read

    loop {
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;
        let mut sq: u32 = 0;
        let mut shut: u32 = 0;
        for i in 0..1 {
            let b = read_burst(&spi, &spi_nss);
            // println!("X: {:4} Y: {:4} SQ: {:4}", b.dx, b.dy, b.sq);
            dx += b.dx as i32;
            dy += b.dy as i32;
            sq += b.sq as u32;
            shut += b.shut as u32;
        }
        println!("x: {:4} y: {:4} sq: {:4} {:4}", dx, dy, sq, shut);
        board::delay(100);
    }


    // loop {
    //     let mut x: i32 = 0;
    //     let mut y: i32 = 0;
    //     for _ in 0..1 {
    //         let (dx, dy) = read_motion_count(&spi, &spi_nss);
    //         x += dx as i32;
    //         y += dy as i32;
    //     }
    //     println!("{} {}", x, y);
    //     board::delay(200);
    // }

    // let test_data = [(0x42, 0x12), (0x01, 0x09), (0x02, 0x1a), (0x03, 0x0b), (0x04, 0x00), (0x05, 0x52), (0x06, 0x6c)];

    // for &(tx, rx) in test_data.iter() {
    //     println!("0x{:02x}: 0x{:02x}", tx, rx);
    //     assert_eq!(reg_read(&spi, &spi_nss, tx), rx);
    // }
    loop {}
}


pub struct Burst {
    dx: i16,
    dy: i16,
    sq: u8,
    sum: u8,
    max: u8,
    min: u8,
    shut: u16,
}

fn read_burst(spi: &SpiPeriph, nss: &GpioPin) -> Burst {
    let mut buf = [0u8; 12];
    nss.set_output(false);
    while !spi.can_tx() {}
    spi.tx(0x16u8);
    while !spi.can_rx() {}
    let _: u8 = spi.rx();
    board::delay(1);
    for i in 0..buf.len() {
        while !spi.can_tx() {}  
        spi.tx(0xffu8);
        while !spi.can_rx() {}
        buf[i] = spi.rx();
    }
    nss.set_output(true);
    for b in buf.iter() {
        print!("{:02x} ", b);
    }
    print!("    ");
    Burst {
        dx: (buf[2] as u16 | (buf[3] as u16) << 8) as i16,
        dy: (buf[4] as u16 | (buf[5] as u16) << 8) as i16,
        sq: buf[6],
        sum: buf[7],
        max: buf[8],
        min: buf[9],
        shut: buf[10] as u16 | (buf[11] as u16) << 8,
    }
    
}

fn read_motion_count(spi: &SpiPeriph, nss: &GpioPin) -> (i16, i16) {
    reg_read(&spi, &nss, 0x02);
    // println!("{:02x}{:02x} {:02x}{:02x}", 
    //     reg_read(&spi, &nss, 0x04), reg_read(&spi, &nss, 0x03),
    //     reg_read(&spi, &nss, 0x06), reg_read(&spi, &nss, 0x05)
    // );

    let dx = ((reg_read(&spi, &nss, 0x03) as u16) | (reg_read(&spi, &nss, 0x04) as u16) << 8) as i16;
    let dy = ((reg_read(&spi, &nss, 0x05) as u16) | (reg_read(&spi, &nss, 0x06) as u16) << 8) as i16;
    (dx, dy)
}

fn transfer(spi: &SpiPeriph, nss: &GpioPin, src: &[u8], dst: &mut[u8]) {
    let mut i = 0;
    let mut j = 0;
    nss.set_output(false);
    loop {
        if i < src.len() && spi.can_tx() {
            spi.tx(src[i]);
            i += 1;
        }
        if j < dst.len() && spi.can_rx() {
            dst[j] = spi.rx();
            j += 1;
        }
        if j == dst.len() {
            break;
        }        
    }
    nss.set_output(true);
}


fn reg_write(spi: &SpiPeriph, nss: &GpioPin, reg: u8, val: u8) {
    let cmd = [reg, val];
    let mut buf = [0u8, 0u8];
    transfer(spi, nss, &cmd, &mut buf)    
}
    

fn reg_read(spi: &SpiPeriph, nss: &GpioPin, reg: u8) -> u8 {
    let cmd = [reg, 0x00];
    let mut buf = [0u8, 0u8];
    transfer(spi, nss, &cmd, &mut buf);
    buf[1]
}
    
const INIT1: [(u8, u8); 59] = [
    // From PMW3901MB datasheet
    (0x7F, 0x00),
    (0x61, 0xAD),
    (0x7F, 0x03),
    (0x40, 0x00),
    (0x7F, 0x05),
    (0x41, 0xB3),
    (0x43, 0xF1),
    (0x45, 0x14),
    (0x5B, 0x32),
    (0x5F, 0x34),
    (0x7B, 0x08),
    (0x7F, 0x06),
    (0x44, 0x1B),
    (0x40, 0xBF),
    (0x4E, 0x3F),
    //
    (0x7F, 0x08),
    (0x65, 0x20),
    (0x6A, 0x18),
    (0x7F, 0x09),
    (0x4F, 0xAF),
    (0x5F, 0x40),
    (0x48, 0x80),
    (0x49, 0x80),
    (0x57, 0x77),
    (0x60, 0x78),
    (0x61, 0x78),
    (0x62, 0x08),
    (0x63, 0x50),
    (0x7F, 0x0A),
    (0x45, 0x60),
    (0x7F, 0x00),
    (0x4D, 0x11),
    (0x55, 0x80),
    (0x74, 0x1F),
    (0x75, 0x1F),
    (0x4A, 0x78),
    (0x4B, 0x78),
    (0x44, 0x08),
    (0x45, 0x50),
    (0x64, 0xFF),
    (0x65, 0x1F),
    (0x7F, 0x14),
    (0x65, 0x60),
    (0x66, 0x08),
    (0x63, 0x78),
    (0x7F, 0x15),
    (0x48, 0x58),
    (0x7F, 0x07),
    (0x41, 0x0D),
    (0x43, 0x14),
    (0x4B, 0x0E),
    (0x45, 0x0F),
    (0x44, 0x42),
    (0x4C, 0x80),
    (0x7F, 0x10),
    (0x5B, 0x02),
    (0x7F, 0x07),
    (0x40, 0x41),
    (0x70, 0x00),
];
const INIT2: [(u8, u8); 14] = [
    (0x32, 0x44),
    (0x7F, 0x07),
    (0x40, 0x40),
    (0x7F, 0x06),
    (0x62, 0xf0),
    (0x63, 0x00),
    (0x7F, 0x0D),
    (0x48, 0xC0),
    (0x6F, 0xd5),
    (0x7F, 0x00),
    (0x5B, 0xa0),
    (0x4E, 0xA8),
    (0x5A, 0x50),
    (0x40, 0x80),    
];