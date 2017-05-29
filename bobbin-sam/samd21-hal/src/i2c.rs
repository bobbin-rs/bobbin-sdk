use core::marker::PhantomData;

use embed_common::io::{self, Error};

pub use ::chip::sercom::*;
use ::driver::port::*;
use ::hal::pm;
use ::hal::i2c;
use ::chip::gclk;

fn sercom_index(sercom: Sercom) -> u16 {
    match sercom {
        SERCOM0 => 0,
        SERCOM1 => 1,
        SERCOM2 => 2,
        SERCOM3 => 3,
        SERCOM4 => 4,
        SERCOM5 => 5,
        _ => unimplemented!(),
    }
}

pub struct Unknown;
pub struct Master;

pub struct I2cDevice<M> {
    dev: Sercom,
    scl: Pin<ModePMux>,
    sda: Pin<ModePMux>,
    phantom: PhantomData<M>,
}

pub fn new(dev: Sercom, scl: Pin<ModePMux>, sda: Pin<ModePMux>) -> I2cDevice<Unknown> {
    I2cDevice { dev: dev, scl: scl, sda: sda, phantom: PhantomData }
}
impl<M> I2cDevice<M> {
    pub fn into_master(self, baud: u8) -> I2cDevice<Master> {
        pm::set_sercom_enabled(self.dev, true);

        unsafe {
            let mut s = self.dev.i2cm();

            // Set GCLK_GEN0 as source for SERCOM

            gclk::GCLK.set_clkctrl(gclk::Clkctrl(0)
                .set_id(0x14 + sercom_index(self.dev))
                .set_gen(0x0)
                .set_clken(1)
            );

            // Wait for synchronization
            while gclk::GCLK.status().syncbusy() != 0 {}

            // Disable the SERCOM module
            s.with_ctrla(|r| r.set_enable(0));

            // Wait for synchronization
            while s.syncbusy().swrst() != 0 {}
            
            // Software Reset
            s.with_ctrla(|r| r.set_swrst(1));
        
            // Wait for synchronization
            while s.ctrla().swrst() != 0 {}

            // Wait for synchronization
            while s.syncbusy().swrst() != 0 || s.syncbusy().enable() != 0 {}

            // Set master mode
            s.set_ctrla(i2cm::Ctrla(0).set_mode(0x5));
            // Set master mode and enable SCL Clock Stretch mode (stretch after ACK bit)
            //s.set_ctrla(i2c::Ctrla(0).set_mode(0x5).set_sdahold(0x3).set_sclsm(1));

            // Enable Smart mode
            //s.set_ctrlb(i2c::Ctrlb(0).set_smen(1));

            // set baud rate
            // SystemCoreClock / ( 2 * baudrate) - 1
            // 48Mhz / (2 * baudrate) - 1
            // baudrate = 100_000
            // baud = 240

            s.set_baud(i2cm::Baud(0).set_baud(baud as u32));
        }

        I2cDevice { dev: self.dev, scl: self.scl, sda: self.sda, phantom: PhantomData }
    }
    pub fn enable(&self) {
        i2c::enable(self.dev)
    }
}

impl io::I2cDevice<u8> for I2cDevice<Master> {
    fn write_addr(&self, addr: u8, data: &[u8]) -> Result<usize, Error> {
        i2c::write(self.dev, addr, data);
        Ok(data.len())
    }

    fn read_addr(&self, addr: u8, data: &mut [u8]) -> Result<usize, Error> {
        i2c::read(self.dev, addr, data);
        Ok(data.len())
    }

    fn transfer_addr(&self, addr: u8, data_out: &[u8], data_in: &mut [u8]) -> Result<(usize, usize), Error> {        
        let n_out = try!(self.write_addr(addr, data_out));
        let n_in = try!(self.read_addr(addr, data_in));
        Ok((n_out, n_in))
    }
}