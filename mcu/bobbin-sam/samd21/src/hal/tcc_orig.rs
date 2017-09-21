#![allow(non_camel_case_types)]

pub use ::chip::tcc::*;

use ::chip::irq;
use ::nvic;

pub type Handler = unsafe extern "C" fn();

pub enum Prescsync {
    GCLK = 0x0,
    PRESC = 0x1,
    RESYNC = 0x2,
}

pub enum Prescaler {
    Div1 = 0x0,
    Div2 = 0x1,
    Div4 = 0x2,
    Div8 = 0x3,
    Div16 = 0x4,
    Div64 = 0x5,
    Div256 = 0x6,
    Div1024 = 0x7,
}

pub enum Wavegen {
    NFRQ = 0x0,
    MFRQ = 0x1,
    NPWM = 0x2,
    MPWM = 0x3,
    DSCRITICAL = 0x4,
    DSBOTTOM = 0x5,
    DSBOTH = 0x6,
    DSTOP = 0x7,
}

pub enum Direction {
    Up = 0x0,
    Down = 0x1,
}

pub struct Config {
    pub wavegen: Wavegen,
    pub prescaler: Prescaler,
    pub runstdby: bool,
    pub prescsync: Prescsync,
}

pub struct TccDevice {
    tcc: Tcc,
}

pub fn configure(mut tcc: Tcc, cfg: Config) -> TccDevice {
    unsafe {
        tcc.set_ctrla(Ctrla(0).set_enable(0));
        while tcc.syncbusy().enable() == 1 {}
        tcc.set_ctrla(Ctrla(0).set_swrst(1));
        while tcc.syncbusy().swrst() == 1 {}

        tcc.set_ctrla(Ctrla(0)
            .set_prescaler(cfg.prescaler as u32)
            .set_runstdby(if cfg.runstdby { 1 } else { 0 })
            .set_prescsync(cfg.prescsync as u32)            
        );        
        tcc.set_wave(Wave(0).set_wavegen(cfg.wavegen as u32));
        tcc.set_intflag(Intflag(0xff));
        TccDevice { tcc: tcc }
    }
}

pub unsafe fn tcc(tcc: Tcc) -> TccDevice {
    TccDevice { tcc: tcc }
}

impl TccDevice {
    pub fn irq(&self) -> Option<irq::Irq> {
        match self.tcc {
            TCC0 => Some(irq::IRQ_TCC0),
            TCC1 => Some(irq::IRQ_TCC1),
            TCC2 => Some(irq::IRQ_TCC2),
            _ => None,
        }
    }
    pub fn set_handler(&self, handler: Option<Handler>) {
        if let Some(irq) = self.irq() {
            irq.set_handler(handler)
        } else {
            unimplemented!()
        }        
    }

    pub fn set_handler_enabled(&self, value: bool) {
        if let Some(irq) = self.irq() {
            nvic::set_enabled(irq.0, value);
        } else {
            unimplemented!()
        }        
    }
}


impl TccDevice {
    pub fn set_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe {
            let mut tcc = self.tcc;
            tcc.with_ctrla(|r| r.set_enable(value))
        }
    }

    pub fn trigger(&self) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_ctrlbset(Ctrlbset(0).set_cmd(0x1))
        }
    }

    pub fn stop(&self) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_ctrlbset(Ctrlbset(0).set_cmd(0x2))
        }
    }    

    pub fn set_oneshot(&self, value: bool) {
        unsafe {
            let mut tcc = self.tcc;
            match value {
                true => tcc.set_ctrlbset(Ctrlbset(0).set_oneshot(1)),            
                false => tcc.set_ctrlbclr(Ctrlbclr(0).set_oneshot(1)),
            }
        }
    }

    pub fn set_direction(&self, dir: Direction) {
        unsafe {
            let mut tcc = self.tcc;
            match dir {
                Direction::Up => tcc.set_ctrlbset(Ctrlbset(0).set_dir(1)),
                Direction::Down => tcc.set_ctrlbclr(Ctrlbclr(0).set_dir(1)),
            }
        }
    }

    pub fn count(&self) -> u32 {
        unsafe {
            let mut tcc = self.tcc;
            // Force a read synchronization of COUNT
            tcc.set_ctrlbset(Ctrlbset(0).set_cmd(0x04));
            tcc.count().count() as u32
        }
    }

    pub fn set_count(&self, value: u32) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_count(Count(0).set_count(value))
        }
    }

    pub fn period(&self) -> u32 {
        unsafe {
            let tcc = self.tcc;
            tcc.per().per() as u32
        }
    }

    pub fn set_period(&self, value: u32) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_per(Per(0).set_per(value))
        }
    }

    pub fn cc(&self, index: usize) -> u32 {
        unsafe {
            let tcc = self.tcc;
            tcc.cc(index).cc() as u32
        }
    }
    pub fn set_cc(&self, index: usize, value: u32) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_cc(index, Cc(0).set_cc(value))
        }
    }    

    // Interrupt Management

    pub fn intflag(&self) -> Intflag {
        unsafe {
            let tcc = self.tcc;
            tcc.intflag()
        }
    }


    pub fn set_mc3_enabled(&self, value: bool) {
        unsafe {
            let mut tcc = self.tcc;
            match value {
                true => tcc.set_intenset(Intenset(0).set_mc3(1)),
                false => tcc.set_intenclr(Intenclr(0).set_mc3(1)),
            }            
        }
    }

    pub fn set_mc2_enabled(&self, value: bool) {
        unsafe {
            let mut tcc = self.tcc;
            match value {
                true => tcc.set_intenset(Intenset(0).set_mc2(1)),
                false => tcc.set_intenclr(Intenclr(0).set_mc2(1)),
            }            
        }
    }


    pub fn set_mc1_enabled(&self, value: bool) {
        unsafe {
            let mut tcc = self.tcc;
            match value {
                true => tcc.set_intenset(Intenset(0).set_mc1(1)),
                false => tcc.set_intenclr(Intenclr(0).set_mc1(1)),
            }            
        }
    }


    pub fn set_mc0_enabled(&self, value: bool) {
        unsafe {
            let mut tcc = self.tcc;
            match value {
                true => tcc.set_intenset(Intenset(0).set_mc0(1)),
                false => tcc.set_intenclr(Intenclr(0).set_mc0(1)),
            }            
        }
    }

    pub fn set_err_enabled(&self, value: bool) {
        unsafe {
            let mut tcc = self.tcc;
            match value {
                true => tcc.set_intenset(Intenset(0).set_err(1)),
                false => tcc.set_intenclr(Intenclr(0).set_err(1)),
            }            
        }
    }    

    pub fn set_cnt_enabled(&self, value: bool) {
        unsafe {
            let mut tcc = self.tcc;
            match value {
                true => tcc.set_intenset(Intenset(0).set_cnt(1)),
                false => tcc.set_intenclr(Intenclr(0).set_cnt(1)),
            }            
        }
    }    

    pub fn set_trg_enabled(&self, value: bool) {
        unsafe {
            let mut tcc = self.tcc;
            match value {
                true => tcc.set_intenset(Intenset(0).set_trg(1)),
                false => tcc.set_intenclr(Intenclr(0).set_trg(1)),
            }            
        }
    }    

    pub fn set_ovfenabled(&self, value: bool) {
        unsafe {
            let mut tcc = self.tcc;
            match value {
                true => tcc.set_intenset(Intenset(0).set_ovf(1)),
                false => tcc.set_intenclr(Intenclr(0).set_ovf(1)),
            }            
        }
    }
    
    pub fn clr_mc3(&self) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_intflag(Intflag(0).set_mc3(1));
        }
    }    
    
    pub fn clr_mc2(&self) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_intflag(Intflag(0).set_mc2(1));
        }
    }        

    pub fn clr_mc1(&self) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_intflag(Intflag(0).set_mc1(1));
        }
    }

    pub fn clr_mc0(&self) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_intflag(Intflag(0).set_mc0(1));
        }
    }

    pub fn clr_err(&self) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_intflag(Intflag(0).set_err(1));
        }
    }        

    pub fn clr_cnt(&self) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_intflag(Intflag(0).set_cnt(1));
        }
    }        

    pub fn clr_trg(&self) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_intflag(Intflag(0).set_trg(1));
        }
    }        

    pub fn clr_ovf(&self) {
        unsafe {
            let mut tcc = self.tcc;
            tcc.set_intflag(Intflag(0).set_ovf(1));
        }
    }    

}
