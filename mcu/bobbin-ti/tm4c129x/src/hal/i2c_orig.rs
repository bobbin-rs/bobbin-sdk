pub use ::chip::i2c::*;
use ::driver::gpio::{Pin, AltFn};
use ::hal::sysctl;
//use ::hal::clock;

use embed_common::io::{self, Error};
use core::marker::PhantomData;

pub struct Disabled {}
pub struct Master {}

pub struct I2cDevice<M> {
    i2c: I2c,
    _scl: Pin<AltFn>,
    _sda: Pin<AltFn>,
    _phantom: PhantomData<M>,
}

pub fn device(i2c: I2c, scl: Pin<AltFn>, sda: Pin<AltFn>) -> I2cDevice<Disabled> {    
    I2cDevice { i2c: i2c, _scl: scl, _sda: sda, _phantom: PhantomData }
}

pub const BUSY_WAIT: usize = 50;

impl<M> I2cDevice<M> {
    pub fn into_master(self) -> I2cDevice<Master> {
        sysctl::set_i2c_enabled(self.i2c, true);
        let i2c = self.i2c;
        unsafe {
            i2c.master().set_mcr(master::Mcr(0).set_mfe(1));
            // Set the desired SCL clock speed of 100 Kbps by writing the I2CMTPR register with the correct value.
            // The value written to the I2CMTPR register represents the number of system clock periods in one SCL 
            // clock period. The TPR value is determined by the following equation:
            // TPR = (System Clock/(2*(SCL_LP + SCL_HP)*SCL_CLK))-1;
            // TPR = (20MHz/(2*(6+4)*100000))-1;
            // TPR = 9
            // Write the I2CMTPR register with the value of 0x0000.0009.

            // Set to 400Khz for now
            // let clk = 400_000;
            // let tpr = (clock::sysclk_hz() / (2 * (6 + 4) * clk)) - 1;
            // TPR = 15
            let tpr = 15;
            //let tpr = 0x3b;
            i2c.master().set_mtpr(master::Mtpr(0).set_tpr(tpr));
        }
        I2cDevice { i2c: self.i2c, _scl: self._scl, _sda: self._sda, _phantom: PhantomData }
    }
}

impl io::I2cDevice<u8> for I2cDevice<Master> {
    fn write_addr(&self, addr: u8, data: &[u8]) -> Result<usize, Error>{
        let i2c = self.i2c;        
        unsafe {
            i2c.master().set_msa(master::Msa(0).set_sa(addr as u32));    
            while i2c.master().mcs_read().busbsy() != 0 {}
            let len = data.len();
            for i in 0..data.len() {
                let start = if i == 0 { 1 } else { 0 };
                let stop = if i == len - 1 { 1 } else { 0 };
                i2c.master().set_mdr(master::Mdr(0).set_data(data[i] as u32));
                i2c.master().set_mcs_write(master::McsWrite(0)
                    .set_qcmd(0)
                    .set_start(start)
                    .set_stop(stop)
                    .set_run(1)
                );
                // NOTE - BUSY will not change immediately after write, must wait.
                //while i2c.master().mcs_read().busy() == 0 {}
                for _ in 0..BUSY_WAIT { asm!("nop")}
                while i2c.master().mcs_read().busy() != 0 {}
            }
            //i2c.master().set_mcs_write(master::McsWrite(0).set_run(0));
            //while i2c.master().mcs_read().busbsy() != 0 {}
        }
        Ok(data.len())
    }

    fn read_addr(&self, addr: u8, data: &mut [u8]) -> Result<usize, Error> {
        //trace!("read {}", data.len());
        let i2c = self.i2c;
        unsafe {
            i2c.master().set_msa(master::Msa(0).set_sa(addr as u32).set_rs(1));
            let len = data.len();
            for i in 0..data.len() {
                let start = if i == 0 { 1 } else { 0 };
                let stop = if i == len - 1 { 1 } else { 0 };    
                let ack =  if i == len - 1 { 0 } else { 1 };
                //trace!("{}: start: {} stop: {} ack: {}", i, start, stop, ack);
                i2c.master().set_mcs_write(master::McsWrite(0)
                    .set_start(start)
                    .set_stop(stop)
                    .set_ack(ack)
                    .set_run(1)
                );                
                // NOTE - BUSY will not change immediately after write, must wait.
                //while i2c.master().mcs_read().busy() == 0 {}
                for _ in 0..BUSY_WAIT { asm!("nop")}
                while i2c.master().mcs_read().busy() != 0 {}
                if i2c.master().mcs_read().error() != 0 {
                    trace!("Error: {:?}", i2c.master().mcs_read());
                    i2c.master().set_mcs_write(master::McsWrite(0)
                        .set_stop(1)
                        .set_run(0)
                    );
                    return Err(Error::Numeric(0x1));
                }
                
                data[i] = i2c.master().mdr().data() as u8;
            }            
        }
        Ok(data.len())
    }

    fn transfer_addr(&self, addr: u8, data_out: &[u8], data_in: &mut [u8]) -> Result<(usize, usize), Error> {
        //trace!("transfer {} {}", data_out.len(), data_in.len());
        let i2c = self.i2c;   
        let (n_out, n_in) = (data_out.len(), data_in.len());
        // try!(self.write_addr(addr, data_out));
        // try!(self.read_addr(addr, data_in));
        unsafe {
            while i2c.master().mcs_read().busbsy() != 0 {}
            i2c.master().set_msa(master::Msa(0).set_sa(addr as u32));

            let len = data_out.len();
            for i in 0..data_out.len() {
                let start = if i == 0 || i == len - 1 { 1 } else { 0 };
                let stop = 0;
                i2c.master().set_mdr(master::Mdr(0).set_data(data_out[i] as u32));
                i2c.master().set_mcs_write(master::McsWrite(0)
                    .set_start(start)
                    .set_stop(stop)
                    .set_run(1)
                );
                // NOTE - BUSY will not change immediately after write, must wait.
                //while i2c.master().mcs_read().busy() == 0 {}
                for _ in 0..BUSY_WAIT { asm!("nop")}
                while i2c.master().mcs_read().busy() != 0 {}
            }
            i2c.master().set_msa(master::Msa(0).set_sa(addr as u32).set_rs(1));
            let len = data_in.len();
            for i in 0..data_in.len() {
                let start = if i == 0 { 1 } else { 0 };
                let stop = if i == len - 1 { 1 } else { 0 };    
                let ack =  if i == len - 1 { 0 } else { 1 };
                i2c.master().set_mcs_write(master::McsWrite(0)
                    .set_start(start)
                    .set_stop(stop)
                    .set_ack(ack)
                    .set_run(1)
                );
                // NOTE - BUSY will not change immediately after write, must wait.
                //while i2c.master().mcs_read().busy() == 0 {}
                for _ in 0..BUSY_WAIT { asm!("nop")}
                while i2c.master().mcs_read().busy() != 0 {}
                data_in[i] = i2c.master().mdr().data() as u8;
            }            
        }
        Ok((n_out, n_in))
    }
}