#![allow(non_camel_case_types)]

pub use ::chip::tc::*;

//use ::hal::pm;
use ::chip::irq;
use ::nvic;

use core::marker::PhantomData;

pub type Handler = unsafe extern "C" fn();

pub enum TC8Bit {}
pub enum TC16Bit {}
pub enum TC32Bit {}

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

pub struct TcDevice<M> {
    tc: Tc,
    _phantom: PhantomData<M>,
}

pub fn configure_8bit(tc: Tc, cfg: Config) -> TcDevice<TC8Bit> {
    unsafe {
        let mut tc8 = tc.count8();
        tc8.set_ctrla(count8::Ctrla(0).set_enable(0));
        tc8.set_ctrla(count8::Ctrla(0).set_swrst(1));
        while tc8.ctrla().swrst() == 1 {}

        tc8.set_ctrla(count8::Ctrla(0)
            .set_mode(0x1)
            .set_wavegen(cfg.wavegen as u16)
            .set_prescaler(cfg.prescaler as u16)
            .set_runstdby(if cfg.runstdby { 1 } else { 0 })
            .set_prescsync(cfg.prescsync as u16)            
        );
        while tc8.status().syncbusy() != 0 {}
        tc8.set_intflag(count8::Intflag(0xff));
        TcDevice { tc: tc, _phantom: PhantomData }
    }
}

pub unsafe fn tc_8bit(tc: Tc) -> TcDevice<TC8Bit> {
    TcDevice { tc: tc, _phantom: PhantomData }
}


pub fn configure_16bit(tc: Tc, cfg: Config) -> TcDevice<TC16Bit> {
    unsafe {
        let mut tc16 = tc.count16();
        tc16.set_ctrla(count16::Ctrla(0).set_enable(0));
        tc16.set_ctrla(count16::Ctrla(0).set_swrst(1));
        while tc16.ctrla().swrst() == 1 {}

        tc16.set_ctrla(count16::Ctrla(0)
            .set_mode(0x0)
            .set_wavegen(cfg.wavegen as u16)
            .set_prescaler(cfg.prescaler as u16)
            .set_runstdby(if cfg.runstdby { 1 } else { 0 })
            .set_prescsync(cfg.prescsync as u16)            
        );
        while tc16.status().syncbusy() != 0 {}
        tc16.set_intflag(count16::Intflag(0xff));
        TcDevice { tc: tc, _phantom: PhantomData }
    }
}

pub unsafe fn tc_16bit(tc: Tc) -> TcDevice<TC16Bit> {
    TcDevice { tc: tc, _phantom: PhantomData }
}


pub fn configure_32bit(tc: Tc, cfg: Config) -> TcDevice<TC8Bit> {
    unsafe {
        let mut tc32 = tc.count32();
        tc32.set_ctrla(count32::Ctrla(0).set_enable(0));
        tc32.set_ctrla(count32::Ctrla(0).set_swrst(1));
        while tc32.ctrla().swrst() == 1 {}

        tc32.set_ctrla(count32::Ctrla(0)
            .set_mode(0x2)
            .set_wavegen(cfg.wavegen as u16)
            .set_prescaler(cfg.prescaler as u16)
            .set_runstdby(if cfg.runstdby { 1 } else { 0 })
            .set_prescsync(cfg.prescsync as u16)            
        );
        while tc32.status().syncbusy() != 0 {}
        tc32.set_intflag(count32::Intflag(0xff));
        TcDevice { tc: tc, _phantom: PhantomData }
    }
}

pub unsafe fn tc_32bit(tc: Tc) -> TcDevice<TC32Bit> {
    TcDevice { tc: tc, _phantom: PhantomData }
}

impl<M> TcDevice<M> {
    pub fn irq(&self) -> Option<irq::Irq> {
        match self.tc {
            TC3 => Some(irq::IRQ_TC3),
            TC4 => Some(irq::IRQ_TC4),
            TC5 => Some(irq::IRQ_TC5),
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
            nvic::set_enabled(irq.0 as usize, value);
        } else {
            unimplemented!()
        }        
    }
}


impl TcDevice<TC8Bit> {
    pub fn set_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.with_ctrla(|r| r.set_enable(value))
        }
    }

    pub fn trigger(&self) {
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.set_ctrlbset(count8::Ctrlbset(0).set_cmd(0x1))
        }
    }

    pub fn stop(&self) {
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.set_ctrlbset(count8::Ctrlbset(0).set_cmd(0x2))
        }
    }    

    pub fn set_oneshot(&self, value: bool) {
        unsafe {
            let mut tc8 = self.tc.count8();
            match value {
                true => tc8.set_ctrlbset(count8::Ctrlbset(0).set_oneshot(1)),            
                false => tc8.set_ctrlbclr(count8::Ctrlbclr(0).set_oneshot(1)),
            }
        }
    }

    pub fn set_direction(&self, dir: Direction) {
        unsafe {
            let mut tc8 = self.tc.count8();
            match dir {
                Direction::Up => tc8.set_ctrlbset(count8::Ctrlbset(0).set_dir(1)),
                Direction::Down => tc8.set_ctrlbclr(count8::Ctrlbclr(0).set_dir(1)),
            }
        }
    }

    pub fn count(&self) -> u8 {
        unsafe {
            let tc8 = self.tc.count8();
            tc8.count().count() as u8
        }
    }

    pub fn set_count(&self, value: u8) {
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.set_count(count8::Count(0).set_count(value))
        }
    }

    pub fn period(&self) -> u8 {
        unsafe {
            let tc8 = self.tc.count8();
            tc8.per().per() as u8
        }
    }

    pub fn set_period(&self, value: u8) {
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.set_per(count8::Per(0).set_per(value))
        }
    }

    pub fn cc(&self, index: usize) -> u8 {
        unsafe {
            let tc8 = self.tc.count8();
            tc8.cc(index).cc() as u8
        }
    }
    pub fn set_cc(&self, index: usize, value: u8) {
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.set_cc(index, count8::Cc(0).set_cc(value))
        }
    }    

    // Interrupt Management

    pub fn intenset(&self) -> count8::Intenset {
        unsafe {
            let tc8 = self.tc.count8();
            tc8.intenset()
        }
    }

    pub fn intflag(&self) -> count8::Intflag {
        unsafe {
            let tc8 = self.tc.count8();
            tc8.intflag()
        }
    }


    pub fn set_mc1_enabled(&self, value: bool) {
        unsafe {
            let mut tc8 = self.tc.count8();
            match value {
                true => tc8.set_intenset(count8::Intenset(0).set_mc1(1)),
                false => tc8.set_intenclr(count8::Intenclr(0).set_mc1(1)),
            }            
        }
    }


    pub fn set_mc0_enabled(&self, value: bool) {
        unsafe {
            let mut tc8 = self.tc.count8();
            match value {
                true => tc8.set_intenset(count8::Intenset(0).set_mc0(1)),
                false => tc8.set_intenclr(count8::Intenclr(0).set_mc0(1)),
            }            
        }
    }

    pub fn set_syncrdy_enabled(&self, value: bool) {
        unsafe {
            let mut tc8 = self.tc.count8();
            match value {
                true => tc8.set_intenset(count8::Intenset(0).set_err(1)),
                false => tc8.set_intenclr(count8::Intenclr(0).set_err(1)),
            }            
        }
    }

    pub fn set_ovfenabled(&self, value: bool) {
        unsafe {
            let mut tc8 = self.tc.count8();
            match value {
                true => tc8.set_intenset(count8::Intenset(0).set_ovf(1)),
                false => tc8.set_intenclr(count8::Intenclr(0).set_ovf(1)),
            }            
        }
    }
    

    pub fn set_err_enabled(&self, value: bool) {
        unsafe {
            let mut tc8 = self.tc.count8();
            match value {
                true => tc8.set_intenset(count8::Intenset(0).set_err(1)),
                false => tc8.set_intenclr(count8::Intenclr(0).set_err(1)),
            }            
        }
    }    


    pub fn clr_mc1(&self) {
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.set_intflag(count8::Intflag(0).set_mc1(1));
        }
    }

    pub fn clr_mc0(&self) {
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.set_intflag(count8::Intflag(0).set_mc0(1));
        }
    }

    pub fn clr_syncrdy(&self) {
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.set_intflag(count8::Intflag(0).set_syncrdy(1));
        }
    }   

    pub fn clr_err(&self) {
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.set_intflag(count8::Intflag(0).set_err(1));
        }
    }        

    pub fn clr_ovf(&self) {
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.set_intflag(count8::Intflag(0).set_ovf(1));
        }
    }    

    pub fn clr_all(&self) {
        unsafe {
            let mut tc8 = self.tc.count8();
            tc8.set_intflag(count8::Intflag(0xff));
        }
    }

}

impl TcDevice<TC16Bit> {
    pub fn set_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe {
            let mut tc16 = self.tc.count16();
            tc16.with_ctrla(|r| r.set_enable(value))
        }
    }

    pub fn trigger(&self) {
        unsafe {
            let mut tc16 = self.tc.count16();
            tc16.set_ctrlbset(count16::Ctrlbset(0).set_cmd(0x1))
        }
    }

    pub fn stop(&self) {
        unsafe {
            let mut tc16 = self.tc.count16();
            tc16.set_ctrlbset(count16::Ctrlbset(0).set_cmd(0x2))
        }
    }    

    pub fn set_oneshot(&self, value: bool) {
        unsafe {
            let mut tc16 = self.tc.count16();
            match value {
                true => tc16.set_ctrlbset(count16::Ctrlbset(0).set_oneshot(1)),            
                false => tc16.set_ctrlbclr(count16::Ctrlbclr(0).set_oneshot(1)),
            }
        }
    }

    pub fn set_direction(&self, dir: Direction) {
        unsafe {
            let mut tc16 = self.tc.count16();
            match dir {
                Direction::Up => tc16.set_ctrlbset(count16::Ctrlbset(0).set_dir(1)),
                Direction::Down => tc16.set_ctrlbclr(count16::Ctrlbclr(0).set_dir(1)),
            }
        }
    }

    pub fn count(&self) -> u16 {
        unsafe {
            let tc16 = self.tc.count16();
            tc16.count().count() as u16
        }
    }

    pub fn set_count(&self, value: u16) {
        unsafe {
            let mut tc16 = self.tc.count16();
            tc16.set_count(count16::Count(0).set_count(value))
        }
    }

    pub fn cc(&self, index: usize) -> u16 {
        unsafe {
            let tc16 = self.tc.count16();
            tc16.cc(index).cc() as u16
        }
    }
    pub fn set_cc(&self, index: usize, value: u16) {
        unsafe {
            let mut tc16 = self.tc.count16();
            tc16.set_cc(index, count16::Cc(0).set_cc(value))
        }
    }    

    // Interrupt Management

    pub fn intflag(&self) -> count16::Intflag {
        unsafe {
            let tc16 = self.tc.count16();
            tc16.intflag()
        }
    }


    pub fn set_mc1_enabled(&self, value: bool) {
        unsafe {
            let mut tc16 = self.tc.count16();
            match value {
                true => tc16.set_intenset(count16::Intenset(0).set_mc1(1)),
                false => tc16.set_intenclr(count16::Intenclr(0).set_mc1(1)),
            }            
        }
    }


    pub fn set_mc0_enabled(&self, value: bool) {
        unsafe {
            let mut tc16 = self.tc.count16();
            match value {
                true => tc16.set_intenset(count16::Intenset(0).set_mc0(1)),
                false => tc16.set_intenclr(count16::Intenclr(0).set_mc0(1)),
            }            
        }
    }

    pub fn set_syncrdy_enabled(&self, value: bool) {
        unsafe {
            let mut tc16 = self.tc.count16();
            match value {
                true => tc16.set_intenset(count16::Intenset(0).set_err(1)),
                false => tc16.set_intenclr(count16::Intenclr(0).set_err(1)),
            }            
        }
    }

    pub fn set_ovfenabled(&self, value: bool) {
        unsafe {
            let mut tc16 = self.tc.count16();
            match value {
                true => tc16.set_intenset(count16::Intenset(0).set_ovf(1)),
                false => tc16.set_intenclr(count16::Intenclr(0).set_ovf(1)),
            }            
        }
    }
    

    pub fn set_err_enabled(&self, value: bool) {
        unsafe {
            let mut tc16 = self.tc.count16();
            match value {
                true => tc16.set_intenset(count16::Intenset(0).set_syncrdy(1)),
                false => tc16.set_intenclr(count16::Intenclr(0).set_syncrdy(1)),
            }            
        }
    }    


    pub fn clr_mc1(&self) {
        unsafe {
            let mut tc16 = self.tc.count16();
            tc16.set_intflag(count16::Intflag(0).set_mc1(1));
        }
    }

    pub fn clr_mc0(&self) {
        unsafe {
            let mut tc16 = self.tc.count16();
            tc16.set_intflag(count16::Intflag(0).set_mc0(1));
        }
    }

    pub fn clr_syncrdy(&self) {
        unsafe {
            let mut tc16 = self.tc.count16();
            tc16.set_intflag(count16::Intflag(0).set_syncrdy(1));
        }
    }   

    pub fn clr_err(&self) {
        unsafe {
            let mut tc16 = self.tc.count16();
            tc16.set_intflag(count16::Intflag(0).set_err(1));
        }
    }        

    pub fn clr_ovf(&self) {
        unsafe {
            let mut tc16 = self.tc.count16();
            tc16.set_intflag(count16::Intflag(0).set_ovf(1));
        }
    }    

}




impl TcDevice<TC32Bit> {
    pub fn set_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe {
            let mut tc32 = self.tc.count32();
            tc32.with_ctrla(|r| r.set_enable(value))
        }
    }

    pub fn trigger(&self) {
        unsafe {
            let mut tc32 = self.tc.count32();
            tc32.set_ctrlbset(count32::Ctrlbset(0).set_cmd(0x1))
        }
    }

    pub fn stop(&self) {
        unsafe {
            let mut tc32 = self.tc.count32();
            tc32.set_ctrlbset(count32::Ctrlbset(0).set_cmd(0x2))
        }
    }    

    pub fn set_oneshot(&self, value: bool) {
        unsafe {
            let mut tc32 = self.tc.count32();
            match value {
                true => tc32.set_ctrlbset(count32::Ctrlbset(0).set_oneshot(1)),            
                false => tc32.set_ctrlbclr(count32::Ctrlbclr(0).set_oneshot(1)),
            }
        }
    }

    pub fn set_direction(&self, dir: Direction) {
        unsafe {
            let mut tc32 = self.tc.count32();
            match dir {
                Direction::Up => tc32.set_ctrlbset(count32::Ctrlbset(0).set_dir(1)),
                Direction::Down => tc32.set_ctrlbclr(count32::Ctrlbclr(0).set_dir(1)),
            }
        }
    }

    pub fn count(&self) -> u32 {
        unsafe {
            let tc32 = self.tc.count32();
            tc32.count().count() as u32
        }
    }

    pub fn set_count(&self, value: u32) {
        unsafe {
            let mut tc32 = self.tc.count32();
            tc32.set_count(count32::Count(0).set_count(value))
        }
    }

    pub fn cc(&self, index: usize) -> u32 {
        unsafe {
            let tc32 = self.tc.count32();
            tc32.cc(index).cc() as u32
        }
    }
    pub fn set_cc(&self, index: usize, value: u32) {
        unsafe {
            let mut tc32 = self.tc.count32();
            tc32.set_cc(index, count32::Cc(0).set_cc(value))
        }
    }    

    // Interrupt Management

    pub fn intflag(&self) -> count32::Intflag {
        unsafe {
            let tc32 = self.tc.count32();
            tc32.intflag()
        }
    }


    pub fn set_mc1_enabled(&self, value: bool) {
        unsafe {
            let mut tc32 = self.tc.count32();
            match value {
                true => tc32.set_intenset(count32::Intenset(0).set_mc1(1)),
                false => tc32.set_intenclr(count32::Intenclr(0).set_mc1(1)),
            }            
        }
    }


    pub fn set_mc0_enabled(&self, value: bool) {
        unsafe {
            let mut tc32 = self.tc.count32();
            match value {
                true => tc32.set_intenset(count32::Intenset(0).set_mc0(1)),
                false => tc32.set_intenclr(count32::Intenclr(0).set_mc0(1)),
            }            
        }
    }

    pub fn set_syncrdy_enabled(&self, value: bool) {
        unsafe {
            let mut tc32 = self.tc.count32();
            match value {
                true => tc32.set_intenset(count32::Intenset(0).set_err(1)),
                false => tc32.set_intenclr(count32::Intenclr(0).set_err(1)),
            }            
        }
    }

    pub fn set_ovfenabled(&self, value: bool) {
        unsafe {
            let mut tc32 = self.tc.count32();
            match value {
                true => tc32.set_intenset(count32::Intenset(0).set_ovf(1)),
                false => tc32.set_intenclr(count32::Intenclr(0).set_ovf(1)),
            }            
        }
    }
    

    pub fn set_err_enabled(&self, value: bool) {
        unsafe {
            let mut tc32 = self.tc.count32();
            match value {
                true => tc32.set_intenset(count32::Intenset(0).set_syncrdy(1)),
                false => tc32.set_intenclr(count32::Intenclr(0).set_syncrdy(1)),
            }            
        }
    }    


    pub fn clr_mc1(&self) {
        unsafe {
            let mut tc32 = self.tc.count32();
            tc32.set_intflag(count32::Intflag(0).set_mc1(1));
        }
    }

    pub fn clr_mc0(&self) {
        unsafe {
            let mut tc32 = self.tc.count32();
            tc32.set_intflag(count32::Intflag(0).set_mc0(1));
        }
    }

    pub fn clr_syncrdy(&self) {
        unsafe {
            let mut tc32 = self.tc.count32();
            tc32.set_intflag(count32::Intflag(0).set_syncrdy(1));
        }
    }   

    pub fn clr_err(&self) {
        unsafe {
            let mut tc32 = self.tc.count32();
            tc32.set_intflag(count32::Intflag(0).set_err(1));
        }
    }        

    pub fn clr_ovf(&self) {
        unsafe {
            let mut tc32 = self.tc.count32();
            tc32.set_intflag(count32::Intflag(0).set_ovf(1));
        }
    }    

}

