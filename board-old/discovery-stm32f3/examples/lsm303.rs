#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f3 as board;

use board::hal::i2c::*;
use board::hal::gpio::*;
use board::common::bits::*;

// Address = 0x35

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running I2C");
    
    let addr_accel: U7 = U7::from(0x32 >> 1);
    let addr_mag: U7 = U7::from(0x3d >> 1);

    let i2c = I2C1;
    let i2c_port = GPIOB;
    let i2c_scl = PB6;
    let i2c_sda = PB7;

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

    println!("I2C Configuration Complete");

    println!("Configuring Accel");
    // i2c.write_reg(addr_accel, 0x20, 0x00);
    i2c.write_reg(addr_accel, 0x20, 0x57);
    // println!("v: {:02x}", i2c.read_reg(addr_accel, 0x20));
    
    // for i in 0x20..0x26 {
    //     println!("{:02x}: {:02x}", i, i2c.read_reg(addr_accel, i));
    // }

    println!("Configuring Magnetometer");
    i2c.write_reg(addr_mag, 0x02, 0x00);
    i2c.write_reg(addr_mag, 0x01, 0x20);
    i2c.write_reg(addr_mag, 0x00, 0x90);
    // println!("V: {:02x}", i2c.read_reg(addr_mag, 0x00));

    println!("Accelerometer | Magnetometer");

    loop {
        {
            let (xl, xh, yl, yh, zl, zh) = (
                i2c.read_reg(addr_accel, 0x28),
                i2c.read_reg(addr_accel, 0x29),
                i2c.read_reg(addr_accel, 0x2a),
                i2c.read_reg(addr_accel, 0x2b),
                i2c.read_reg(addr_accel, 0x2c),
                i2c.read_reg(addr_accel, 0x2d),
            );
            let x = (((xh as u16) << 8) | (xl as u16)) as i16;
            let y = (((yh as u16) << 8) | (yl as u16)) as i16;
            let z = (((zh as u16) << 8) | (zl as u16)) as i16;
            print!("{:6} {:6} {:6}", x, y, z);
        }
        print!(" | ");
        {
            let (xl, xh, yl, yh, zl, zh) = (
                i2c.read_reg(addr_mag, 0x03),
                i2c.read_reg(addr_mag, 0x04),
                i2c.read_reg(addr_mag, 0x05),
                i2c.read_reg(addr_mag, 0x06),
                i2c.read_reg(addr_mag, 0x07),
                i2c.read_reg(addr_mag, 0x08),
            );
            let x = (((xh as u16) << 8) | (xl as u16)) as i16;
            let y = (((yh as u16) << 8) | (yl as u16)) as i16;
            let z = (((zh as u16) << 8) | (zl as u16)) as i16;
            print!("{:6} {:6} {:6}", x, y, z);
        }        
        println!("");

        // let mut buf = [0u8; 6];
        // i2c.transfer(addr_gyro, &[0x20], &mut buf);
        // println!("# {:?}", buf);
        board::delay(500);
    }
}
