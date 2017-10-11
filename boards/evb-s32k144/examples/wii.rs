#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate evb_s32k144 as board;

use board::hal::pcc;
use board::hal::lpi2c::*;
use board::hal::port::*;
use board::common::bits::*;

// pub const i2c_addr: u8 = 0x52;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    let i2c_addr = U7::from(0x52);    

    PORTA.pcc_enable();
    let i2c = LPI2C0;

    let port_scl = PTA3;
    let port_sda = PTA2;

    println!("# I2C WII Start");
    port_scl.mode_i2c_scl(&i2c).set_pull_none();
    port_sda.mode_i2c_sda(&i2c).set_pull_none();

    i2c
        .pcc_set_clock_source(pcc::ClockSource::SPLLDIV2)
        .pcc_enable();

    {
        // let input_clock = i2c.clock(&CLK).unwrap();

        i2c.with_mcfgr0(|r| r.set_cirfifo(0));
        i2c.with_mcfgr1(|r| r.set_prescale(0x1));
        i2c.with_mcfgr2(|r| r.set_filtscl(1).set_filtsda(1).set_busidle(0xBA));        
        i2c.with_mccr0(|r| r.set_clklo(0x3e).set_clkhi(0x35).set_sethold(0x1d).set_datavd(0x0f));
        i2c.with_mfcr(|r| r.set_txwater(0x3).set_rxwater(0x3));
    }

    // Enable Master
    i2c.with_mcr(|r| r.set_men(true));

    println!("I2C Configured");

    let mut tx_buf = [I2cAction::Idle; 64];
    let mut rx_buf = [0u8; 64];
    let s = I2cDriver::new(i2c, &mut tx_buf, &mut rx_buf);
    s.enable_irq(&i2c.irq_lpi2c_master());

    println!("Initialize");
    
    s.write_reg(i2c_addr, 0xf0, 0x55);
    board::delay(1);    
    s.write_reg(i2c_addr, 0xfb, 0x00);
    board::delay(1);
    
    print!("Id: ");
    s.write(i2c_addr, &[0xfa]);
    board::delay(1);
    let mut buf = [0u8; 6];
    s.read(i2c_addr, &mut buf);
    board::delay(1);
    for b in buf.iter() {
        print!("{:02x} ", b);
    }
    println!("");

    let mut n = 0;
    loop {
        s.write(i2c_addr, &[0x00]);
        board::delay(1);
        s.read(i2c_addr, &mut buf);
        board::delay(1);
        if n == 100 {
            for b in buf.iter() {
                print!("{:02x} ", b);
            }
            println!("");
            n = 0;
        } else {
            n += 1;
        }
    }
}
