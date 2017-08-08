pub use chip::tpm::*;

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

pub enum ClockMode {
    Disabled = 0b00,
    TpmClk = 0b01,
    ExtClk = 0b10,
}

pub trait TpmExt {
    fn set_clock(&self, value: ClockMode) -> &Self;
    fn set_prescale(&self, value: Prescale) -> &Self;
    fn timer_overflow(&self) -> bool;
    fn clr_timer_overflow(&self) -> &Self;
}

impl<T> TpmExt for Periph<T> {
    fn set_clock(&self, value: ClockMode) -> &Self {
        self.with_sc(|r| r.set_cmod(value as u32))
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
}

pub trait TpmChExt {
    fn csc(&self) -> Csc;
    fn with_csc<F: FnOnce(Csc) -> Csc>(&self, f: F) -> &Self;
    fn set_value(&self, value: u16) -> &Self;
}

impl<P, T> TpmChExt for Channel<P, T> {
    fn csc(&self) -> Csc {
        self.periph.csc(self.index)
    }
    fn with_csc<F: FnOnce(Csc) -> Csc>(&self, f: F) -> &Self {
        self.periph.with_csc(self.index, f);
        self
    }
    fn set_value(&self, value: u16) -> &Self {
        self.periph.set_cv(self.index, |r| r.set_val(value as u32));
        self
    }
}