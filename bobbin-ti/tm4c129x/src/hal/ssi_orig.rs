pub use ::chip::ssi::*;
use ::driver::gpio::{Pin, AltFn};
use ::hal::sysctl;

use embed_common::io::{self, Read, Write, Transfer, Error};
use core::marker::PhantomData;

pub struct Disabled {}
pub struct Master {}

pub struct SsiDevice<M> {
    ssi: Ssi,
    sck: Pin<AltFn>,
    mosi: Option<Pin<AltFn>>,
    miso: Option<Pin<AltFn>>,
    phantom: PhantomData<M>,
}

pub fn device(ssi: Ssi, sck: Pin<AltFn>, mosi: Option<Pin<AltFn>>, miso: Option<Pin<AltFn>>) -> SsiDevice<Disabled> {
    SsiDevice { ssi: ssi, sck: sck, mosi: mosi, miso: miso, phantom: PhantomData }
}

impl<M> SsiDevice<M> {
    pub fn into_master(self) -> SsiDevice<Master> {
        sysctl::set_ssi_enabled(self.ssi, true);
        let mut ssi = self.ssi;
        unsafe {
            // BR = SysClk / (CPSDVSR * (1 + SCR))
            // with SysClk = 120Mhz and desired BR of 1Mhz
            // 1Mhz = 120Mhz / (CPSDVSR * (1 + SCR))
            // 120 = (CPSDVSR * (1 + SCR))
            // CPSDVSR = 30
            // SCR = 39

            // // Wait for SSE=0
            // while ssi.cr0().sse() != 0 {}

            ssi.with_cr1(|r| r.set_sse(0));
            
            // Master Mode
            ssi.set_cr1(Cr1(0));

            ssi.set_cpsr(Cpsr(0).set_cpsdvsr(30));

            ssi.set_cr0(Cr0(0)
                .set_scr(39)
                .set_sph(0)
                .set_spo(0)
                .set_frf(0)
                .set_dss(0x7)
            );
            ssi.with_cr1(|r| r.set_sse(1));

        }
        SsiDevice { ssi: self.ssi, sck: self.sck, mosi: self.mosi, miso: self.miso, phantom: PhantomData }
    }
}

impl Write for SsiDevice<Master> {
    fn write(&self, bytes: &[u8]) -> Result<usize, Error> {
        let mut ssi = self.ssi;
        unsafe {
            for i in 0..bytes.len() {
                while ssi.sr().tnf() == 0 {}
                ssi.set_dr(Dr(0).set_data(bytes[i] as u32));
            }
            while ssi.sr().bsy() != 0 {}
        }
        Ok(bytes.len())
    }

    fn write_all(&self, bytes: &[u8]) -> Result<usize, Error> {
        self.write(bytes)
    }

    fn flush(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Read for SsiDevice<Master> {
    fn read(&self, bytes: &mut [u8]) -> Result<usize, Error> {
        let mut ssi = self.ssi;
        unsafe {
            // Flush FIFO
            while ssi.sr().rne() != 0 {
                let _ = ssi.dr().data();
            }
            
            for i in 0..bytes.len() {
                while ssi.sr().tnf() == 0 {}
                ssi.set_dr(Dr(0).set_data(0xff));
                while ssi.sr().rne() == 0 {}
                bytes[i] = ssi.dr().data() as u8;
            }
        }        
        Ok(bytes.len())
    }
    fn read_all(&self, bytes: &mut [u8]) -> Result<usize, Error> {
        self.read(bytes)
    }
}

impl Transfer for SsiDevice<Master> {
    fn transfer(&self, bytes_out: &[u8], bytes_in: &mut [u8]) -> Result<usize, Error> {
        let mut ssi = self.ssi;
        unsafe {
            // Flush FIFO
            while ssi.sr().rne() != 0 {
                let _ = ssi.dr().data();
            }

            for i in 0..bytes_out.len() {
                while ssi.sr().tnf() == 0 {}
                ssi.set_dr(Dr(0).set_data(bytes_out[i] as u32));                
                while ssi.sr().rne() == 0 {}
                bytes_in[i] = ssi.dr().data() as u8;
            }
        }        
        Ok(bytes_out.len())
    }
}

impl io::SpiDevice for SsiDevice<Master> {}