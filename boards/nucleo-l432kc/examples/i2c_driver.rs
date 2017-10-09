#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::i2c::*;
use board::hal::gpio::*;
use board::common::bits::*;

// A5 = PB6 = I2C1_SCL
// A4 = PB7 = I2C1_SDA

// Address = 0x35

#[no_mangle]
pub extern "C" fn main() -> ! {
    // use board::hal::i2c::*;
    board::init();
    println!("Running I2C");
    
    let addr_gyro: U7 = U7::from(0x6B);
    let i2c = I2C1;
    let i2c_port = GPIOB;
    let i2c_scl = PB6; // A5
    let i2c_sda = PB7; // A4

    GPIOA.rcc_enable();
    PA6.mode_input().open_drain();
    PA5.mode_input().open_drain();

    i2c.rcc_enable();
    i2c_port.rcc_enable();

    i2c_scl.mode_i2c_scl(&i2c).open_drain();
    i2c_sda.mode_i2c_sda(&i2c).open_drain();

    println!("# Configuring I2C");

    // i2c.set_config(|c| c.set_timing(0x8.into(), 0x3.into(), 0x0.into(), 0xd.into(), 0x12.into()));
    i2c.set_enabled(false);
    // i2c.set_timingr(|_| Timingr(0x00300619));
    i2c.set_timingr(|r| r
        .set_presc(0x0)
        .set_scldel(0x3)
        .set_sdadel(0x0)
        .set_sclh(0xF)
        .set_scll(0x12)
    );

    let mut tx_buf = [I2cAction::Idle; 64];
    // let mut tx_buf = [0u8; 64];
    let mut rx_buf = [0u8; 64];

    let d = I2cDriver::new(i2c, &mut tx_buf, &mut rx_buf);    
    d.enable_irq(&i2c.irq_i2c_ev());

    println!("I2C Configuration Complete");

    println!("Configuring Gyro");

    println!("ID:   {:02x}", d.read_reg(addr_gyro, 0x0f));
    println!("TEMP: {:02x}", d.read_reg(addr_gyro, 0x26));
    /* Reset then switch to normal mode and enable all three channels */
    d.write_reg(addr_gyro, 0x20, 0x00);
    d.write_reg(addr_gyro, 0x20, 0x0f);
    // for i in 0x20..0x25 {
    //     println!("{:02x}: {:02x}", i, i2c.read_reg(addr_gyro, i));
    // }

    loop {
        // println!("STATUS: {:02x}", i2c.read_reg(addr_gyro, 0x27));
        { 
            let mut buf = [0u8; 6];
            d.transfer(addr_gyro, &[0x28 | 0x80], &mut buf);
            let (xl, xh, yl, yh, zl, zh) = (
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5],
            );

            let x = (((xh as u16) << 8) | (xl as u16)) as i16;
            let y = (((yh as u16) << 8) | (yl as u16)) as i16;
            let z = (((zh as u16) << 8) | (zl as u16)) as i16;
            print!("{:6} {:6} {:6}", x, y, z);
        }
        println!("");

        board::delay(500);
    }
}
