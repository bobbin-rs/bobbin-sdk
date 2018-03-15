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

impl TpmPeriph {
    pub fn set_clock(&self, value: ClockMode) -> &Self {
        self.with_sc(|r| r.set_cmod(value as u32))
    }
    pub fn set_prescale(&self, value: Prescale) -> &Self {
        self.with_sc(|r| r.set_ps(value as u32))
    }
    pub fn timer_overflow(&self) -> bool {
        self.sc().tof() != 0
    }
    pub fn clr_timer_overflow(&self) -> &Self {
        self.with_sc(|r| r.set_tof(1))
    }
}

impl TpmCh {
    pub fn csc(&self) -> Csc {
        self.periph.csc(self.index)
    }
    pub fn with_csc<F: FnOnce(Csc) -> Csc>(&self, f: F) -> &Self {
        self.periph.with_csc(self.index, f);
        self
    }
    pub fn set_value(&self, value: u16) -> &Self {
        self.periph.set_cv(self.index, |r| r.set_val(value as u32));
        self
    }
}