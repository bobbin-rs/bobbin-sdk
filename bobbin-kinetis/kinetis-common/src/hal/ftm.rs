pub use chip::ftm::*;

pub enum Prescale {
    Div1 = 0b000,
    Div2 = 0b001,
    Div4 = 0b010,
    Div8 = 0b011,
    Div16 = 0b100,
    Div32 = 0b101,
    Div64 = 0b110,
    Div128 = 0b111,
}

pub enum ClockSource {
    Disabled = 0b00,
    SystemClk = 0b01,
    FixedClk = 0b10,
}

pub trait FtmExt {
    fn set_clock(&self, value: ClockSource) -> &Self;
    fn set_prescale(&self, value: Prescale) -> &Self;
    fn timer_overflow(&self) -> bool;
    fn clr_timer_overflow(&self) -> &Self;
    fn count(&self) -> u16;
    fn set_count(&self, value: u16) -> &Self;
    fn modulo(&self) -> u16;
    fn set_modulo(&self, value: u16) -> &Self;
}

impl<T> FtmExt for Periph<T> {
    fn set_clock(&self, value: ClockSource) -> &Self {
        self.with_sc(|r| r.set_clks(value as u32))
    }
    fn set_prescale(&self, value: Prescale) -> &Self {
        self.with_sc(|r| r.set_ps(value as u32))
    }
    fn timer_overflow(&self) -> bool {
        self.sc().tof() != 0
    }
    fn clr_timer_overflow(&self) -> &Self {
        self.with_sc(|r| r.set_tof(1))    
    }
    fn count(&self) -> u16 {
        self.cnt().count() as u16
    }
    fn set_count(&self, value: u16) -> &Self {
        self.set_cnt(Cnt(0).set_count(value as u32))
    }
    fn modulo(&self) -> u16 {
        self._mod()._mod() as u16
    }
    fn set_modulo(&self, value: u16) -> &Self {
        self.set_mod(Mod(0).set_mod(value as u32))
    }    
}

pub trait FtmChExt {
    fn csc(&self) -> Csc;
    fn with_csc<F: FnOnce(Csc) -> Csc>(&self, f: F) -> &Self;
    fn set_value(&self, value: u16) -> &Self;
    fn chf(&self) -> bool { self.csc().chf() != 0}
}

impl<P, T> FtmChExt for Channel<P, T> {
    fn csc(&self) -> Csc {
        self.periph.csc(self.index)
    }
    fn with_csc<F: FnOnce(Csc) -> Csc>(&self, f: F) -> &Self {
        self.periph.with_csc(self.index, f);
        self
    }
    fn set_value(&self, value: u16) -> &Self {
        self.periph.set_cv(self.index, Cv(0).set_val(value as u32));
        self
    }
}