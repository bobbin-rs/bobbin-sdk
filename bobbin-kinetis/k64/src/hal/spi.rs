pub use ::chip::spi::*;
use ::driver::port::*;
use ::hal::sim;

//use core::marker::PhantomData;
use embed_common::io::{self, Read, Write, Transfer, Error};

// See https://community.nxp.com/thread/372146

pub struct SpiDevice {
    spi: Spi,
    _sck: Pin<AltFn>,
    _miso: Option<Pin<AltFn>>,
    _mosi: Option<Pin<AltFn>>,
}

pub fn device(spi: Spi, sck: Pin<AltFn>, miso: Option<Pin<AltFn>>, mosi: Option<Pin<AltFn>>) -> SpiDevice {
    SpiDevice {
        spi: spi,
        _sck: sck,
        _miso: miso,
        _mosi: mosi,
    }
}

impl SpiDevice {
    pub fn enable(&self, br: u8, pbr: u8) {
        sim::set_spi_enabled(self.spi, true);
        let mut spi = self.spi;
        unsafe {
            // enable clock
            // set master mode
            // set mcr
            // set baud rate
            // set ctar
            // remove halt

            // Halt and clear FIFOs
            spi.with_mcr(|r| r.set_halt(1).set_clr_txf(1).set_clr_rxf(1));
            spi.with_mcr(|r| r.set_mdis(0).set_mstr(1));

            // Set configuration 0
            spi.set_ctar(0, Ctar(0)
                .set_br(br as u32)
                .set_pbr(pbr as u32)
                .set_fmsz(7)
            );            
        }
    }
}

impl Write for SpiDevice {
    fn write(&self, bytes: &[u8]) -> Result<usize, Error> {
        let mut spi = self.spi;
        unsafe {            
            // Flush FIFOs
            spi.with_mcr(|r| r.set_halt(1).set_clr_txf(1).set_clr_rxf(1));
            
            // Clear status bits
            spi.set_sr(Sr(0)
                .set_tcf(1)
                .set_eoqf(1)
                .set_tfuf(1)
                .set_tfff(1)
                .set_rfof(1)
                .set_rfdf(1));
            spi.with_mcr(|r| r.set_halt(0)); 

            let len = bytes.len();
            for i in 0..len {
                let last = if i == len - 1 { 1 } else { 0 };

                // NOTE
                // TFFF flag clears automatically when DMA is used to fill TX FIFO.
                // To clear TFFF when not using DMA, follow these steps for every PUSH performed using CPU to fill TX FIFO:
                // 1. Wait until TFFF = 1.
                // 2. Write data to PUSHR using CPU.
                // 3. Clear TFFF by writing a 1 to its location. If TX FIFO is not
                // full, this flag will not clear.

                while spi.sr().tfff() == 0 {}                
                spi.set_pushr(Pushr(0).set_ctas(0).set_eoq(last).set_txdata(bytes[i]).set_pcs(0, 1));
                spi.set_sr(Sr(0).set_tfff(1));
            }            
            while spi.sr().eoqf() == 0 {}
            spi.set_sr(Sr(0).set_eoqf(1));
            spi.with_mcr(|r| r.set_halt(1));
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

impl Read for SpiDevice {
    fn read(&self, bytes: &mut [u8]) -> Result<usize, Error> {
        let mut spi = self.spi;
        unsafe {            
            // Flush FIFOs
            spi.with_mcr(|r| r.set_halt(1).set_clr_txf(1).set_clr_rxf(1));
            // Clear status bits
            spi.set_sr(Sr(0).set_tcf(1).set_eoqf(1).set_tfuf(1).set_tfff(1).set_rfof(1).set_rfdf(1).set_pcs(0, 1));
            spi.with_mcr(|r| r.set_halt(0)); 
            let len = bytes.len();
            for i in 0..len {
                spi.set_pushr(Pushr(0).set_ctas(0).set_txdata(0xff));
                while spi.sr().rfdf() == 0 {}
                bytes[i] = spi.popr().rxdata() as u8;
                spi.set_sr(Sr(0).set_rfdf(1));
            }                        
            spi.with_mcr(|r| r.set_halt(1)); 
        }        
        Ok(bytes.len())
    }

    fn read_all(&self, bytes: &mut [u8]) -> Result<usize, Error> {
        self.read(bytes)
    }
}

impl Transfer for SpiDevice {
    fn transfer(&self, bytes_out: &[u8], bytes_in: &mut [u8]) -> Result<usize, Error> {
        let mut spi = self.spi;
        unsafe {
            // Flush FIFOs
            spi.with_mcr(|r| r.set_halt(1).set_clr_txf(1).set_clr_rxf(1));
            
            // Clear status bits
            spi.set_sr(Sr(0)
                .set_tcf(1)
                .set_eoqf(1)
                .set_tfuf(1)
                .set_tfff(1)
                .set_rfof(1)
                .set_rfdf(1));
            spi.with_mcr(|r| r.set_halt(0)); 

            let len = bytes_out.len();
            for i in 0..len {
                let last = if i == len - 1 { 1 } else { 0 };

                // NOTE
                // TFFF flag clears automatically when DMA is used to fill TX FIFO.
                // To clear TFFF when not using DMA, follow these steps for every PUSH performed using CPU to fill TX FIFO:
                // 1. Wait until TFFF = 1.
                // 2. Write data to PUSHR using CPU.
                // 3. Clear TFFF by writing a 1 to its location. If TX FIFO is not
                // full, this flag will not clear.

                while spi.sr().tfff() == 0 {}                
                spi.set_pushr(Pushr(0).set_ctas(0).set_eoq(last).set_txdata(bytes_out[i] as u32).set_pcs(0, 1));
                spi.set_sr(Sr(0).set_tfff(1));

                while spi.sr().rfdf() == 0 {}
                bytes_in[i] = spi.popr().rxdata() as u8;
                spi.set_sr(Sr(0).set_rfdf(1));                
            }            
            while spi.sr().eoqf() == 0 {}
            spi.set_sr(Sr(0).set_eoqf(1));
            spi.with_mcr(|r| r.set_halt(1));
        }
        Ok(bytes_out.len())
    }
}

impl io::SpiDevice for SpiDevice {}