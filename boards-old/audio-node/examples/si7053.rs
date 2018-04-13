#![no_std]
#![no_main]

#[macro_use]
extern crate audio_node as board;

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
    println!("SI7053 Temperature Sensor");
    
    let addr: U7 = U7::from(0b1000000);
    let i2c = I2C1;
    let i2c_port = GPIOB;
    let i2c_scl = PB8;
    let i2c_sda = PB9;

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

    let mut buf = [0u8; 2];
    d.transfer(addr, &[0x84, 0xb8], &mut buf);
    println!("FW: {:02x}{:02x}", buf[0], buf[1]);

    let mut buf = [0u8; 14];
    d.transfer(addr, &[0xfa, 0x0f], &mut buf[0..8]);
    d.transfer(addr, &[0xfc, 0xc9], &mut buf[8..14]);
    println!("BUF: {:?}", buf);
    let id = [buf[0], buf[2], buf[4], buf[6], buf[8], buf[9], buf[11], buf[12]];
    print!("ID:");
    for b in id.iter() {
        print!(" {:02x}", b);
    }
    println!("");

    loop {
        let mut buf = [0u8; 2];
        d.transfer(addr, &[0xE3], &mut buf);
        let t = (buf[0] as u16) << 8 | buf[1] as u16;
        let tc = ((175.72 * t as f32) / 65536.0) - 46.85;
        let tf = tc * 9.0 / 5.0 + 32.0; 
        println!("{:4.4} C / {:4.4} F", tc, tf);
        board::delay(500);
    }
}
