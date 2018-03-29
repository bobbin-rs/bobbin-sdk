#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::i2c::*;

// A5 = PB6 = I2C1_SCL
// A4 = PB7 = I2C1_SDA

// Address = 0x35

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running I2C");
    
    let addr: U7 = U7::from(0b1010_000);

    let i2c = I2C1;
    let i2c_port = GPIOB;
    let i2c_scl = PB8; // D15
    let i2c_sda = PB9; // D14

    GPIOA.gate_enable();
    PA6.mode_input().open_drain();
    PA5.mode_input().open_drain();

    i2c.gate_enable();
    i2c_port.gate_enable();

    i2c_scl.connect_to(i2c);
    i2c_sda.connect_to(i2c);

    i2c_scl.open_drain();
    i2c_sda.open_drain();

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

    let fram = Mb85rs { i2c: i2c.into(), addr };

    println!("I2C Configuration Complete");

    let buf = fram.device_id(addr);
    // i2c.transfer(addr_id, &[0b1010_0000], &mut buf);    
    
    print!("ID: ");
    for i in 0..buf.len() {
        print!("{:02x} ", buf[i]);
    }
    println!("");
    

    for i in 0..0x40 {
        fram.write(i, i as u8);
    }    

    for i in 0..0x40 {
        let v = fram.read(i);
        if i % 16 == 0 {
            if i > 0 {
                println!("");
            }
            print!("{:04x}:", i)
        }
        if i % 8 == 0 {
            print!(" ");
        }
        print!(" {:02x}", v);
    }
    println!("");

    for i in 0..0x40 {
        fram.write(i, 0);
    }    

    for i in 0..0x40 {
        let v = fram.read(i);
        if i % 16 == 0 {
            if i > 0 {
                println!("");
            }
            print!("{:04x}:", i)
        }
        if i % 8 == 0 {
            print!(" ");
        }
        print!(" {:02x}", v);
    }
    println!("");    
    loop {}

}

pub struct Mb85rs {
    i2c: I2cPeriph,
    addr: U7,
}

impl Mb85rs {
    pub fn device_id(&self, i2c_addr: U7) -> [u8; 3] {
        let mut buf = [0u8; 3];
        self.i2c.transfer(U7::from(0x7c), &[i2c_addr.value() << 1], &mut buf);
        buf
    }
    pub fn write(&self, addr: u16, val: u8) {
        let out = [(addr >> 8) as u8, addr as u8, val];
        self.i2c.transfer(self.addr, &out, &mut []);        
    }

    pub fn read(&self, addr: u16) -> u8 {
        let out = [(addr >> 8) as u8, addr as u8];
        let mut buf = [0x00];
        self.i2c.transfer(self.addr, &out, &mut buf);
        buf[0]    
    }
}