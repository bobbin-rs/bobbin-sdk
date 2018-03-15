use core::fmt::{self, Write};

pub use ::chip::lpuart::*;

pub struct LpuartDevice {
    lpuart: Lpuart
}

pub fn device(lpuart: Lpuart) -> LpuartDevice {
    LpuartDevice { lpuart: lpuart }
}

impl LpuartDevice {
    pub fn lpuart(&self) -> Lpuart {
        self.lpuart
    }

    // BAUD
    pub fn osr(&self) -> u8 {
        let u = self.lpuart;
        unsafe {
            u.baud().osr() as u8
        }        
    }

    pub fn set_osr(&self, value: u16) -> &Self {
        let mut u = self.lpuart;
        unsafe {
            u.with_baud(|r| r.set_osr(value as u32))
        }
        self
    }    

    pub fn sbr(&self) -> u16 {
        let u = self.lpuart;
        unsafe {
            u.baud().sbr() as u16
        }
    }
    
    pub fn set_sbr(&self, value: u16) -> &Self {
        let mut u = self.lpuart;
        unsafe {
            u.with_baud(|r| r.set_sbr(value as u32))
        }
        self
    }

    // STAT

    pub fn tdre(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.stat().tdre() != 0
        }            
    }

    pub fn tc(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.stat().tc() != 0
        }            
    }

    pub fn rdrf(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.stat().rdrf() != 0
        }            
    }

    pub fn idle(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.stat().idle() != 0
        }            
    }
    
    pub fn clr_idle(&self) {
        let mut u = self.lpuart;
        unsafe {
            u.with_stat(|r| r.set_idle(1))
        }            
    }    

    pub fn or(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.stat().or() != 0
        }            
    }
    
    pub fn clr_or(&self) {
        let mut u = self.lpuart;
        unsafe {
            u.with_stat(|r| r.set_or(1))
        }            
    }    

    pub fn nf(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.stat().nf() != 0
        }            
    }
    
    pub fn clr_nf(&self) {
        let mut u = self.lpuart;
        unsafe {
            u.with_stat(|r| r.set_nf(1))
        }            
    }    

    pub fn fe(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.stat().fe() != 0
        }            
    }
    
    pub fn clr_fe(&self) {
        let mut u = self.lpuart;
        unsafe {
            u.with_stat(|r| r.set_fe(1))
        }            
    }        

    pub fn pf(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.stat().pf() != 0
        }            
    }
    
    pub fn clr_pf(&self) {
        let mut u = self.lpuart;
        unsafe {
            u.with_stat(|r| r.set_pf(1))
        }            
    }            

    // CTRL
    
    pub fn orie(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().orie() != 0
        }            
    }    

    pub fn set_orie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_orie(value));
        }            
        self
    }   

    pub fn neie(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().neie() != 0
        }            
    }    

    pub fn set_neie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_neie(value));
        }            
        self
    }   

    pub fn feie(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().feie() != 0
        }            
    }    

    pub fn set_feie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_feie(value));
        }            
        self
    }   

    pub fn peie(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().peie() != 0
        }            
    }    

    pub fn set_peie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_peie(value));
        }            
        self
    }   

    pub fn tie(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().tie() != 0
        }            
    }    

    pub fn set_tie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_tie(value));
        }            
        self
    }   

    pub fn tcie(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().tcie() != 0
        }            
    }    

    pub fn set_tcie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_tcie(value));
        }            
        self
    }     

    pub fn rie(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().rie() != 0
        }            
    }    

    pub fn set_rie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_rie(value));
        }            
        self
    }

    pub fn ilie(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().ilie() != 0
        }            
    }    

    pub fn set_ilie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_ilie(value));
        }            
        self
    }   

    pub fn te(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().te() != 0
        }            
    }    

    pub fn set_te(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_te(value));
        }            
        self
    }     

    pub fn re(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().re() != 0
        }            
    }    

    pub fn set_re(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_re(value));
        }            
        self
    }   

    pub fn loops(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().loops() != 0
        }            
    }    

    pub fn set_loops(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_loops(value));
        }            
        self
    }   

    pub fn rsrc(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.ctrl().rsrc() != 0
        }            
    }    

    pub fn set_rsrc(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_ctrl(|r| r.set_rsrc(value));
        }            
        self
    }   

    pub fn noisy(&self) -> u8 {
        let u = self.lpuart;
        unsafe {
            u.data().noisy() as u8
        }
    }

    pub fn paritye(&self) -> u8 {
        let u = self.lpuart;
        unsafe {
            u.data().paritye() as u8
        }
    }

    // pub fn rxempt(&self) -> u8 {
    //     let u = self.lpuart;
    //     unsafe {
    //         u.data().rxempt() as u8
    //     }
    // }

    pub fn rt(&self) -> u8 {
        let u = self.lpuart;
        unsafe {
            u.data().rt() as u8
        }
    }

    pub fn set_rt(&self, value: u8) -> &Self {
        let mut u = self.lpuart;
        unsafe {
            u.set_data(Data(0).set_rt(value as u32))
        }
        self
    }

    // FIFO

    pub fn txempt(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.fifo().txempt() != 0
        }
    }    

    pub fn rxempt(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.fifo().rxempt() != 0
        }
    }    

    pub fn txof(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.fifo().txof() != 0
        }
    }    

    pub fn clr_txof(&self) -> &Self {
        let mut u = self.lpuart;
        unsafe {
            u.with_fifo(|r| r.set_txof(1))
        }
        self
    }

    pub fn rxuf(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.fifo().rxuf() != 0
        }
    }    

    pub fn clr_rxuf(&self) -> &Self {
        let mut u = self.lpuart;
        unsafe {
            u.with_fifo(|r| r.set_rxuf(1))
        }
        self
    }

    pub fn set_txflush(&self) -> &Self {
        let mut u = self.lpuart;
        unsafe {
            u.with_fifo(|r| r.set_txflush(1))
        }
        self
    }

    pub fn set_rxflush(&self) -> &Self {
        let mut u = self.lpuart;
        unsafe {
            u.with_fifo(|r| r.set_rxflush(1))
        }
        self
    }

    pub fn txofe(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.fifo().txofe() != 0
        }
    }    

    pub fn set_txofe(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_fifo(|r| r.set_txofe(value))
        }
        self
    }

    pub fn rxufe(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.fifo().rxufe() != 0
        }
    }    

    pub fn set_rxufe(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_fifo(|r| r.set_rxufe(value))
        }
        self
    }
    
    pub fn rxfe(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.fifo().rxfe() != 0
        }
    }    

    pub fn set_rxfe(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_fifo(|r| r.set_rxfe(value))
        }
        self
    }    

    pub fn txfe(&self) -> bool {
        let u = self.lpuart;
        unsafe {
            u.fifo().txfe() != 0
        }
    }    

    pub fn set_txfe(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut u = self.lpuart;
        unsafe {
            u.with_fifo(|r| r.set_txfe(value))
        }
        self
    }    

    pub fn try_getc(&self) -> Option<u8> {
        if self.rdrf() {
            Some(self.rt())
        } else {
            None
        }
    }

    pub fn putc(&self, c: u8) {
        while !self.tdre() {}
        self.set_rt(c);
    }

    pub fn write(&self, buf: &[u8]) {
        for b in buf.iter() {
            self.putc(*b);
        }
    }
}

impl Write for LpuartDevice {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}