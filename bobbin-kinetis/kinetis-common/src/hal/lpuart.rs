use core::fmt::{self, Write};

use chip::lpuart::*;

pub trait LpuartExt {
    fn set_osr(&self, value: u16) -> &Self;
    fn set_sbr(&self, value: u16) -> &Self;

    fn set_te(&self, value: bool) -> &Self;
    fn set_re(&self, value: bool) -> &Self;
    fn set_txfe(&self, value: bool) -> &Self;
    fn set_rxfe(&self, value: bool) -> &Self;
    fn rdrf(&self) -> bool;
    fn tdre(&self) -> bool;
    fn rt(&self) -> u8;
    fn set_rt(&self, value: u8) -> &Self;
    fn try_getc(&self) -> Option<u8>;
    fn putc(&self, c: u8) -> &Self;
    fn write(&self, buf: &[u8]) -> &Self;
}

impl<T> LpuartExt for Periph<T> {
    fn set_osr(&self, value: u16) -> &Self {
        self.with_baud(|r| r.set_osr(value as u32))
    }
    fn set_sbr(&self, value: u16) -> &Self {
        self.with_baud(|r| r.set_sbr(value as u32))
    }

    fn set_te(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_ctrl(|r| r.set_te(value))
    }         

    fn set_re(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_ctrl(|r| r.set_re(value))
    }   

    fn set_txfe(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_fifo(|r| r.set_txfe(value))
    }    

    fn set_rxfe(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_fifo(|r| r.set_rxfe(value))
    }    


    fn rdrf(&self) -> bool {
        self.stat().rdrf() != 0
    }

    fn tdre(&self) -> bool {
        self.stat().tdre() != 0
    }

    fn rt(&self) -> u8 {
        self.data().rt() as u8
    }

    fn set_rt(&self, value: u8) -> &Self {
        self.set_data(Data(0).set_rt(value as u32))
    }    

    fn try_getc(&self) -> Option<u8> {
        if self.rdrf() {
            Some(self.rt())
        } else {
            None
        }
    }

    fn putc(&self, c: u8) -> &Self {
        while !self.tdre() {}
        self.set_rt(c)
    }

    fn write(&self, buf: &[u8]) -> &Self {
        for b in buf.iter() {
            self.putc(*b);
        }
        self
    }
}

impl<T> Write for Periph<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}