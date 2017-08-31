pub use bobbin_common::configure::*;
pub use bobbin_common::serial::*;

use bobbin_common::bits::*;
use core::fmt::{self, Write};

use chip::lpuart::*;

#[derive(Default)]
pub struct Config {
    baud: Baud,
    ctrl: Ctrl,
}

impl Config {
    pub fn set_baud_divisor(&mut self, baud_divisor: U13) -> &Self {
        self.baud.set_sbr(baud_divisor);
        self
    }
}

impl Configure<Config> for LpuartPeriph {
    fn config(&self) -> Config {
        Config {
            baud: self.baud(),
            ctrl: self.ctrl(),
        }
    }

    fn configure(&self, cfg: Config) -> &Self {
        self
            .set_baud(|_| cfg.baud)
            .set_ctrl(|_| cfg.ctrl)
    }
}

impl LpuartPeriph {
    pub fn set_osr(&self, value: u16) -> &Self {
        self.with_baud(|r| r.set_osr(value as u32))
    }
    pub fn set_sbr(&self, value: u16) -> &Self {
        self.with_baud(|r| r.set_sbr(value as u32))
    }

    pub fn set_te(&self, value: bool) -> &Self {
        self.with_ctrl(|r| r.set_te(value))
    }         

    pub fn set_re(&self, value: bool) -> &Self {
        self.with_ctrl(|r| r.set_re(value))
    }   

    pub fn set_txfe(&self, value: bool) -> &Self {
        self.with_fifo(|r| r.set_txfe(value))
    }    

    pub fn set_rxfe(&self, value: bool) -> &Self {
        self.with_fifo(|r| r.set_rxfe(value))
    }    


    pub fn rdrf(&self) -> bool {
        self.stat().rdrf() != 0
    }

    pub fn tdre(&self) -> bool {
        self.stat().tdre() != 0
    }

    pub fn rt(&self) -> u8 {
        self.data().rt().into()
    }

    pub fn set_rt(&self, value: u8) -> &Self {
        self.set_data(|r| r.set_rt(value as u32))
    }    

    // pub fn try_getc(&self) -> Option<u8> {
    //     if self.rdrf() {
    //         Some(self.rt())
    //     } else {
    //         None
    //     }
    // }

    // pub fn putc(&self, c: u8) -> &Self {
    //     while !self.tdre() {}
    //     self.set_rt(c)
    // }

    // pub fn write(&self, buf: &[u8]) -> &Self {
    //     for b in buf.iter() {
    //         self.putc(*b);
    //     }
    //     self
    // }
}

impl Write for LpuartPeriph {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}

impl SerialTx<u8> for LpuartPeriph {    
    fn can_tx(&self) -> bool {
        self.tdre()
    }

    fn tx(&self, c: u8) -> &Self {
        self.set_rt(c)
    }
}

impl SerialRx<u8> for LpuartPeriph {
    fn can_rx(&self) -> bool {
        self.rdrf()
    }

    fn rx(&self) -> u8 {
        self.rt()
    }
}