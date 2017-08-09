pub use bobbin_common::timer::*;
pub use chip::tc::*;
pub use super::pm::PmEnabled;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Prescsync {
    GCLK = 0x0,
    PRESC = 0x1,
    RESYNC = 0x2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Wavegen {
    NFRQ = 0x0,
    MFRQ = 0x1,
    NPWM = 0x2,
    MPWM = 0x3,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

pub trait TcExt {
    fn configure_16bit(&self, cfg: Config) -> &Self;
    fn intflag(&self) -> count16::Intflag;
    fn set_enabled(&self, value: bool) -> &Self;
    fn set_cc(&self, index: usize, value: u16) -> &Self;
    fn clr_syncrdy(&self) -> &Self;
    fn set_mc0_enabled(&self, value: bool) -> &Self;
}

impl<T> TcExt for Periph<T> {
    fn configure_16bit(&self, cfg: Config) -> &Self {
        let tc16 = self.count16();
        tc16.set_ctrla(|r| r.set_enable(0));
        tc16.set_ctrla(|r| r.set_swrst(1));
        while tc16.ctrla().swrst() == 1 {}

        tc16.set_ctrla(|r| r
            .set_mode(0x0)
            .set_wavegen(cfg.wavegen as u16)
            .set_prescaler(cfg.prescaler as u16)
            .set_runstdby(cfg.runstdby)
            .set_prescsync(cfg.prescsync as u16)            
        );
        while tc16.status().syncbusy() != 0 {}
        tc16.set_intflag(|_| count16::Intflag(0xff));
        self
    }


    fn intflag(&self) -> count16::Intflag {
        let tc16 = self.count16();
        tc16.intflag()
    }    

    fn set_enabled(&self, value: bool) -> &Self {
        let tc16 = self.count16();
        tc16.with_ctrla(|r| r.set_enable(value));
        self
    }    

    fn set_cc(&self, index: usize, value: u16) -> &Self {
        let tc16 = self.count16();
        tc16.set_cc(index, |r| r.set_cc(value));
        self
    }    


    fn clr_syncrdy(&self) -> &Self {
        let tc16 = self.count16();
        tc16.set_intflag(|r| r.set_syncrdy(1));
        self
    }    
        
    fn set_mc0_enabled(&self, value: bool) -> &Self {
        let tc16 = self.count16();
        match value {
            true => tc16.set_intenset(|r| r.set_mc0(1)),
            false => tc16.set_intenclr(|r| r.set_mc0(1)),
        };
        self
    }
}

impl<T> Timer<u16> for Periph<T> {
    fn stop(&self) -> &Self {
        self.count16().set_ctrlbset(|r| r.set_cmd(0x2));
        self.count16().with_ctrla(|r| r.set_enable(0x0));
        while self.count16().status().syncbusy() != 0 {}
        self
    }
    fn running(&self) -> bool {
        self.count16().status().stop() == 0
    }

    fn period(&self) -> u16 {
        self.count16().cc(0).cc().value()
    }

    fn set_period(&self, value: u16) -> &Self {
        self.count16().set_cc(0, |r| r.set_cc(value));
        self
    }

    fn counter(&self) -> u16 {
        self.count16().count().count().value()
    }

    fn timeout_flag(&self) -> bool {
        self.count16().intflag().mc0() != 0
    }

    fn clr_timeout_flag(&self) -> &Self {
        self.count16().set_intflag(|r| r.set_mc0(1));
        self
    }
}

impl<T> Start<u16> for Periph<T> {
    fn start(&self, value: u16) -> &Self {        
        self.set_period(value);        
        self.count16().with_ctrla(|r| r.set_enable(0x1).set_wavegen(0x1));
        while self.count16().status().syncbusy() != 0 {}
        self.count16().set_ctrlbset(|r| r.set_cmd(0x1));
        self
    }    
}

impl<T> Prescale<u16> for Periph<T> {
    fn prescale(&self) -> u16 {
        1 << self.count16().ctrla().prescaler().value() 
    }
    fn set_prescale(&self, value: u16) -> &Self {
        let shift = match value {
            1 => 0,
            2 => 1,
            4 => 2,
            8 => 3,
            16 => 4,
            32 => 5,
            64 => 6,
            128 => 7,
            256 => 8,
            512 => 9,
            1024 => 10,
            _ => panic!("Invalid prescaler value"),
        };
        self.count16().with_ctrla(|r| r.set_prescaler(shift));
        self
    }
}

impl<T> SetCounter<u16> for Periph<T> {
    fn set_counter(&self, value: u16) -> &Self {
        self.count16().set_count(|r| r.set_count(value));
        self
    }
}

impl<T> Compare<u16> for Periph<T> {
    fn compare(&self) -> u16 {
        self.count16().cc(1).cc().value()
    }

    fn set_compare(&self, value: u16) -> &Self {
        self.count16().set_cc(1, |r| r.set_cc(value));        
        self
    }

    fn compare_flag(&self) -> bool {
        self.count16().intflag().mc1() != 0
    }

    fn clr_compare_flag(&self) -> &Self {
        self.count16().set_intflag(|r| r.set_mc1(1));
        self
    }

}

impl<T> Delay<u16> for Periph<T> {
    fn delay(&self, value: u16) -> &Self {
        self
            .start(value)
            .clr_timeout_flag()
            .wait_timeout_flag()
            .stop()
    }
}