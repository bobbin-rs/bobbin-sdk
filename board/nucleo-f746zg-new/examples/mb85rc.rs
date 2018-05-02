#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate embedded_hal as hal;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::i2c::*;

// A5 = PB6 = I2C1_SCL
// A4 = PB7 = I2C1_SDA

// Address = 0x35

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    let brd = board::board();
    println!("Running I2C");
    
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

    let i2c_drv = I2cDriver::new(i2c.into());
    let mut app = examples::mb85rc::Example::new(brd.console(), i2c_drv);
    let _ = app.run();
    loop {}

}

pub struct Error;

pub struct I2cDriver { i2c: I2cPeriph }

impl I2cDriver {
    pub fn new(i2c: I2cPeriph) -> Self {
        Self { i2c }
    }    
}

impl hal::blocking::i2c::WriteRead for I2cDriver {
    type Error = Error;

    fn write_read(
        &mut self, 
        address: u8, 
        bytes: &[u8], 
        buffer: &mut [u8]
    ) -> Result<(), Self::Error> {
        self.i2c.transfer(address, bytes, buffer);
        Ok(())
    }

}