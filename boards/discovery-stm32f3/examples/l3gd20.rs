#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f3 as board;

use board::hal::spi::*;
use board::hal::gpio::*;
use board::common::bits::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("L3GD20/SPI");
    

    let spi = SPI1;
    let spi_miso = PA6;
    let spi_mosi = PA7;
    let spi_sck = PA5;
    let spi_nss = PE3; 

    spi.rcc_enable();
    spi_miso.port().rcc_enable();
    spi_mosi.port().rcc_enable();
    spi_sck.port().rcc_enable();
    spi_nss.port().rcc_enable();

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
    // spi.with_cr1(|r| r.set_cpha(0).set_cpol(0));
    // spi.with_cr2(|r| r.set_frxth(0));
    spi.set_output_enabled(true).set_enabled(true);

    println!("ID:   {:02x}", reg_read(&spi, &spi_nss, 0x0f));
    println!("TEMP: {:02x}", reg_read(&spi, &spi_nss, 0x26));

    /* Reset then switch to normal mode and enable all three channels */
    reg_write(&spi, &spi_nss, 0x20, 0x00);
    reg_write(&spi, &spi_nss, 0x20, 0x0f);
    // for i in 0x20..0x25 {
    //     println!("{:02x}: {:02x}", i, reg_read(&spi, &spi_nss, i));
    // }
    let mut n = 0;
    loop {
        if n == 0 {
            println!("{:>6} {:>6} {:>6}","X","Y","Z");
        }
        // println!("STATUS: {:02x}", reg_read(&spi, &spi_nss, 0x27));
        { 
            let (xl, xh, yl, yh, zl, zh) = (
                reg_read(&spi, &spi_nss, 0x28),
                reg_read(&spi, &spi_nss, 0x29),
                reg_read(&spi, &spi_nss, 0x2a),
                reg_read(&spi, &spi_nss, 0x2b),
                reg_read(&spi, &spi_nss, 0x2c),
                reg_read(&spi, &spi_nss, 0x2d),
            );
            let x = (((xh as u16) << 8) | (xl as u16)) as i16;
            let y = (((yh as u16) << 8) | (yl as u16)) as i16;
            let z = (((zh as u16) << 8) | (zl as u16)) as i16;
            println!("{:6} {:6} {:6}", x, y, z);
        }
        board::delay(500);
        n = if n == 10 { 0 } else { n + 1};
    }
    //     print!(" | ");
    //     {
    //         let (xl, xh, yl, yh, zl, zh) = (
    //             reg_read(&spi, &spi_nss, 0x28),
    //             reg_read(&spi, &spi_nss, 0x29),
    //             reg_read(&spi, &spi_nss, 0x2a),
    //             reg_read(&spi, &spi_nss, 0x2b),
    //             reg_read(&spi, &spi_nss, 0x2c),
    //             reg_read(&spi, &spi_nss, 0x2d),
    //         );
    //         let x = (((xh as u16) << 8) | (xl as u16)) as i16;
    //         let y = (((yh as u16) << 8) | (yl as u16)) as i16;
    //         let z = (((zh as u16) << 8) | (zl as u16)) as i16;
    //         print!("{:6} {:6} {:6}", x, y, z);
    //     }
    //     print!(" | ");
    //     {
    //         let (xl, xh, yl, yh, zl, zh) = (
    //             reg_read(&spi, &spi_nss, 0x03),
    //             reg_read(&spi, &spi_nss, 0x04),
    //             reg_read(&spi, &spi_nss, 0x05),
    //             reg_read(&spi, &spi_nss, 0x06),
    //             reg_read(&spi, &spi_nss, 0x07),
    //             reg_read(&spi, &spi_nss, 0x08),
    //         );
    //         let x = (((xh as u16) << 8) | (xl as u16)) as i16;
    //         let y = (((yh as u16) << 8) | (yl as u16)) as i16;
    //         let z = (((zh as u16) << 8) | (zl as u16)) as i16;
    //         print!("{:6} {:6} {:6}", x, y, z);
    //     }        
    //     // print!(" | ");
    //     // {
    //     //     let (tl, th) = (
    //     //         reg_read(&spi, &spi_nss, 0x31),
    //     //         reg_read(&spi, &spi_nss, 0x32),
    //     //     );
    //     //     let t = (((th as u16) << 8) | (tl as u16)) as i16;
    //     //     print!("{}", t);
    //     // }
    //     print!(" | ");
    //     {
    //         let (pm, pc, _pl, tm, _tl) = (
    //             reg_read(&spi, &spi_nss, 0x01),
    //             reg_read(&spi, &spi_nss, 0x02),
    //             reg_read(&spi, &spi_nss, 0x03),
    //             reg_read(&spi, &spi_nss, 0x04),
    //             reg_read(&spi, &spi_nss, 0x05),
    //         );
    //         let p = (((pm as u16) << 8) | (pc as u16)) as i16;
    //         // let t = (((tm as u16) << 8) | (tl as u16)) as i16;
    //         let t = tm as i8;
    //         print!("{} {}", p, t);
    //     }

    //     println!("");

    //     // let mut buf = [0u8; 6];
    //     // spi.transfer(&[0x20], &mut buf);
    //     // println!("# {:?}", buf);
    //     board::delay(500);
    // }
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
    let cmd = [reg | 1 << 7, 0x00];
    let mut buf = [0u8, 0u8];
    transfer(spi, nss, &cmd, &mut buf);
    buf[1]
}
    