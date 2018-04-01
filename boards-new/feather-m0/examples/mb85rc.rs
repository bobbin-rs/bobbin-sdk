#![no_std]
#![no_main]

#[macro_use]
extern crate feather_m0 as board;
extern crate embedded_hal as hal;
extern crate examples;

use board::common::i2c::*;
use board::mcu::pin::*;
use board::mcu::sercom::*;

// SERCOM3
// SCL = PA22
// SDA = PA23

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    let brd = board::board();
    println!("Running I2C");
    
    let i2c = SERCOM3;
    let i2c_scl = PA22;
    let i2c_sda = PA23;

    i2c.gate_enable();
    i2c_scl.port().gate_enable();
    i2c_sda.port().gate_enable();

    i2c_scl.connect_to(i2c);
    i2c_sda.connect_to(i2c);

    i2c.init_i2c(240);
    i2c.enable_i2c();

    
    let i2c_drv = I2cDriver::new(i2c.into());
    let mut app = examples::mb85rc::Example::new(brd.console(), i2c_drv);
    let _ = app.run();
    loop {}

}

pub struct Error;

pub struct I2cDriver { i2c: SercomPeriph }

impl I2cDriver {
    pub fn new(i2c: SercomPeriph) -> Self {
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