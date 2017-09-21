pub use ::chip::i2c::*;
use port::*;
use sim;

use core::marker::PhantomData;
use embed_common::io::{self, Error};

// See http://cache.freescale.com/files/analog/doc/app_note/AN4342.pdf

pub struct Master {}
pub struct Slave {}

pub struct I2cDevice<T> {
    i2c: I2c,
    _scl: PinAltFn,
    _sda: PinAltFn,
    _phantom: PhantomData<T>,
}

pub fn master(i2c: I2c, scl: PinAltFn, sda: PinAltFn) -> I2cDevice<Master> {
    I2cDevice { i2c: i2c, _scl: scl, _sda: sda, _phantom: PhantomData }
}

impl I2cDevice<Master> {
    pub fn enable(&self, mult: u8, icr: u8) {
        let mut i2c = self.i2c;
        sim::set_i2c_enabled(self.i2c, true);
        unsafe {
            // 1. Write: Frequency Divider register to set the I2C baud rate (see example in description of ICR)
            i2c.set_f(F(0).set_mult(mult).set_icr(icr));
            // 2. Write: Control Register 1 to enable the I2C module and interrupts
            i2c.with_c1(|r| r.set_iicen(1).set_iicie(1));
            // 3. Initialize RAM variables (IICEN = 1 and IICIE = 1) for transmit data
            // 4. Initialize RAM variables used to achieve the routine shown in the following figure
            // 5. Write: Control Register 1 to enable TX
            // 6. Write: Control Register 1 to enable MST (master mode)
            // 7. Write: Data register with the address of the target slave (the LSB of this byte
            // determines whether the communication is master receive or transmit)
        }
    }    
}

impl io::I2cDevice<u8> for I2cDevice<Master> {
    fn write_addr(&self, addr: u8, bytes: &[u8]) -> Result<usize, Error> {
        let mut i2c = self.i2c;
        unsafe {
            // Wait while Busy
            while i2c.s().busy() != 0 {}
            // Send Start
            i2c.with_c1(|r| r.set_tx(1).set_mst(1));                        
            // Send Slave Address
            i2c.set_d(D(0).set_data(addr << 1 ));            
            // wait for interrupt
            while i2c.s().iicif() == 0 {}
            i2c.with_s(|r| r.set_iicif(1));

            for i in 0..bytes.len() {                
                // Send Byte
                i2c.set_d(D(0).set_data(bytes[i]));
                // wait for interrupt
                while i2c.s().iicif() == 0 {}
                i2c.with_s(|r| r.set_iicif(1));
            }
            
            i2c.with_c1(|r| r.set_mst(0).set_tx(0));
        }
        Ok(bytes.len())
    }
    fn read_addr(&self, addr: u8, bytes: &mut [u8]) -> Result<usize, Error> {
        let mut i2c = self.i2c;
        unsafe {
            // Wait while Busy
            while i2c.s().busy() != 0 {}
            // Send Start
            i2c.with_c1(|r| r.set_tx(1).set_mst(1));                        
            // Send Slave Address
            i2c.set_d(D(0).set_data((addr << 1) | 1));
            // wait for interrupt
            while i2c.s().iicif() == 0 {}
            i2c.with_s(|r| r.set_iicif(1));

            // Enter Receive Mode with ACK
            i2c.with_c1(|r| r.set_tx(0).set_txak(0));

            // Receive dummy byte
            let _ = i2c.d().data();
            // wait for interrupt
            while i2c.s().iicif() == 0 {}
            i2c.with_s(|r| r.set_iicif(1));

            let len = bytes.len();

            for i in 0..len-2 {
                i2c.with_c1(|r| r.set_txak(0));
                bytes[i] = i2c.d().data() as u8;
                // wait for interrupt
                while i2c.s().iicif() == 0 {}
                i2c.with_s(|r| r.set_iicif(1));
            }
            // Set NACK
            i2c.with_c1(|r| r.set_txak(1));

            bytes[len-2] = i2c.d().data() as u8;
            // wait for interrupt
            while i2c.s().iicif() == 0 {}
            i2c.with_s(|r| r.set_iicif(1));
            // send stop
            i2c.with_c1(|r| r.set_mst(0));

            bytes[len-1] = i2c.d().data() as u8;

            Ok(bytes.len())
        }
    }
    fn transfer_addr(&self, addr: u8, bytes_out: &[u8], bytes_in: &mut [u8]) -> Result<(usize, usize), Error> {
        let n_out = try!(self.write_addr(addr, bytes_out));
        let n_in = try!(self.read_addr(addr, bytes_in));
        Ok((n_out, n_in))
    }
}