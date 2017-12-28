#![no_std]
#![no_main]

#[macro_use]
extern crate audio_node as board;

use board::hal::gpio::*;
use board::hal::dfsdm::*;
use board::console;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    if true {
        play_square(18, 50);
    }

    // CLKOUT = PC2
    // DATIN0 = PB1


    let pdm = DFSDM;
    let pdm_clk = PC2;
    let pdm_dat = PB1;


    // Enable Peripheral Clocks
    // SYSCLK = 80Mhz
    // PCLK2 = 80Mhz

    pdm.rcc_enable();
    pdm_clk.port().rcc_enable();
    pdm_dat.port().rcc_enable();

    // Setup GPIO Configuration

    pdm_clk.mode_dfsdm_ckout(&pdm).pull_none().speed_high();
    pdm_dat.mode_dfsdm_datin0(&pdm).pull_none().speed_high();

    // Configure DFSDM

    {
        // println!("Configuring DFSDM");
        // Channel 0 => Filter 0

        // PCLK2 = 80Mhz, OUT=8Khz
        // SCK = PCLK2 / 40 = 2.0MHz
        // OUT = SCK / 250 = 8Khz 
        // Range: +/- 16777216 == 24 bit

        let clkdiv = 40;
        let ford = 3; // Sinc3 Filter Type
        let fosr = 250; 
        let iosr = 1;

        // Configure Clock Out

        pdm.with_chcfgr1(0, |r| r
            .set_ckoutsrc(0) // System Clock
            .set_ckoutdiv(clkdiv - 1) // CLKDIV = 250
        );

        // Enable DFSDM
        pdm.with_chcfgr1(0, |r| r.set_dfsdmen(1));

        // Configure Input Channel 0

        pdm.with_chcfgr1(0, |r| r
            .set_chinsel(0) // Input from Channel Pin 0
            .set_chen(0) // Channel Disabled
            .set_ckaben(1) // Clock Absence Detector Enabled
            .set_scden(0) // Short Circuit Detector Disabled
            .set_spicksel(0b01) // Clock from CKOUT
            .set_sitp(0b00) // Sample Rising Edge
        );

        pdm.with_chcfgr2(0, |r| r
            .set_offset(0) // Offset 0
            .set_dtrbs(0) // Data Right Bit Shift 16 bits
        );

         // Enable Channel 0

        pdm.with_chcfgr1(0, |r| r.set_chen(1));

        // println!("CH0CFGR1: {:?}", pdm.chcfgr1(0));
        // println!("CH0CFGR2: {:?}", pdm.chcfgr2(0));

        // Configure Digital Filter 0

        pdm.with_fltcr1(0, |r| r
            .set_rch(0b00) // Regular Channel = 0
            .set_rcont(1) // Continuous Mode
        );

        pdm.with_fltfcr(0, |r| r
            .set_ford(ford) 
            .set_fosr(fosr - 1)
            .set_iosr(iosr - 1)
        );

        // Enable Filter 0
        pdm.with_fltcr1(0, |r| r.set_dfen(1)); 

        // println!("FLT0CR1:  {:?}", pdm.fltcr1(0));
        // println!("FLT0FCR:  {:?}", pdm.fltfcr(0));

        // println!("Configuration Complete");
    }

    {
        // println!("Checking for Clock");
        loop {
            if pdm.fltisr(0).test_ckabf(0) {
                pdm.set_flticr(0, |r| r.set_clrckabf(0, 1));
            } else {
                break;
            }
        }
        // println!("Clock Present");
    }

    let mut buf = [0u8; 4096];
    {
        // println!("Starting Regular Conversion");
        
        // Clear Flags
        pdm.set_flticr(0, |_| Flticr(0xffffffff));

        // Start Regular Conversion

        pdm.with_fltcr1(0, |r| r.set_rswstart(1));

        while pdm.fltisr(0).rcip() == 0 {}

        let timeout = 10_000_000;

        let mut i = 0;
        loop {
            // if i == buf.len() {
            //     break;
            // }
            let mut n = timeout;
            loop {
                let isr = pdm.fltisr(0);
                if isr.reocf() != 0 { break; }
                if isr.rcip() != 1 { 
                    panic!("RCIP Not Set");
                }
                if n == 0 {
                    panic!("Timeout");
                }
                n -= 1;
            }
            let v = pdm.fltrdatar(0).rdata().value() as u32;
            let b = (v >> 16) as u8;
            console::putc(b);
            // buf[i] = b;
            i += 1;
        }
    }
    // println!("Conversion Complete");    
    // dump(&buf[..]);
    send_u8(&buf[..]);
    loop {}
}


fn play_square(period: u32, a: i8) {
    loop {
        for _ in 0..period {
            console::putc(a as u8);
        }
        for _ in 0..period {
            console::putc(-a as u8);
        }
    }
}

fn send_u8(buf: &[u8]) {
    use board::console;
    for b in buf.iter() {
        console::putc(*b);
    }
}

fn dump(buf: &[u32]) {
    for (i, b) in buf.iter().enumerate() {
        if i % 16 == 0 { println!("") }
        let u = b << 8;
        let s = u as i32;
        let v = s >> 8;
        print!("{},", v);
        // if true {
        //     print!("0x{:06x},", b);
        // }
    }
}

