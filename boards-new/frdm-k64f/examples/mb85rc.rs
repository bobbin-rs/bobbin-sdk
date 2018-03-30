#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;
extern crate embedded_hal as hal;
extern crate examples;
extern crate log;

use board::mcu::pin::*;
use board::mcu::i2c::*;
use board::logger;

// I2C0_SCL = PTE24
// I2C0_SDA = PTE25

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    logger::set_log_level(log::LogLevelFilter::Info).unwrap();

    let brd = board::board();
    println!("Running I2C");
    
    let i2c = I2C0;
    let i2c_scl = PTE24;
    let i2c_sda = PTE25;

    i2c.gate_enable();
    i2c_scl.port().gate_enable();
    i2c_sda.port().gate_enable();

    i2c_scl.connect_to(i2c);
    i2c_sda.connect_to(i2c);

    i2c_scl.set_ode(true);
    i2c_sda.set_ode(true);

    println!("# Configuring I2C");

    let mult = 0;
    let icr = 0x1c;

    i2c.init(mult, icr);
    
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