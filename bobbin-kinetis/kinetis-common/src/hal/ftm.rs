pub use bobbin_common::timer::*;
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
    fn value(&self) -> u16;
    fn set_value(&self, value: u16) -> &Self;
    fn chf(&self) -> bool { self.csc().chf() != 0}
    fn set_pwmen(&self, value: bool) -> &Self;
}

impl<P, T> FtmChExt for Channel<P, T> {
    fn csc(&self) -> Csc {
        self.periph.csc(self.index)
    }
    fn with_csc<F: FnOnce(Csc) -> Csc>(&self, f: F) -> &Self {
        self.periph.with_csc(self.index, f);
        self
    }
    fn value(&self) -> u16 {
        self.periph.cv(self.index).val() as u16
    }
    fn set_value(&self, value: u16) -> &Self {
        self.periph.set_cv(self.index, Cv(0).set_val(value as u32));
        self
    }
    fn set_pwmen(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_sc(|r| r.set_pwmen(self.index, value));
        self
    }
}

impl<T> Timer<u16> for Periph<T> {
    fn enabled(&self) -> bool {
        self.sc().clks() != 0
    }
    fn set_enabled(&self, value: bool) -> &Self {
        if value {
            self.set_clock(ClockSource::FixedClk)
        } else {
            self.set_clock(ClockSource::Disabled)            
        }
    }

    fn prescaler(&self) -> u16 {
        1u16 << self.sc().ps()
    }

    fn set_prescaler(&self, prescale: u16) -> &Self {
        let value = match prescale {
            1 => 0b000,
            2 => 0b001,
            4 => 0b010,
            8 => 0b011,
            16 => 0b100,
            32 => 0b101,
            64 => 0b110,
            128 => 0b111,
            _ => panic!("Unsupported prescaler value: {}", prescale),
        };
        self.with_sc(|r| r.set_ps(value))
    }

    fn period(&self) -> u16 {
        self.modulo()
    }

    fn set_period(&self, value: u16) -> &Self {
        self.set_modulo(value);
        self
    }

    fn timeout(&self) -> bool {
        self.sc().tof() != 0
    }
    fn clr_timeout(&self) -> &Self {
        self.with_sc(|r| r.set_tof(0))
    }
}

impl<T> Delay<u16> for Periph<T> {
    fn delay(&self, period: u16, prescale: u16) -> &Self {
        self
            .set_prescaler(prescale)
            .set_period(period)
            .clr_timeout()
            .set_enabled(true)
            .wait_timeout()
            .set_enabled(false)
    }
}


impl<P, T> Compare<u16> for Channel<P, T> {
    fn compare(&self) -> u16 {
        self.periph().cv(self.index).val() as u16
    }
    fn set_compare(&self, value: u16) -> &Self {
        self.periph().set_cv(self.index, Cv(0).set_val(value as u32));
        self
    }

    fn compare_flag(&self) -> bool {
        self.periph().csc(self.index()).chf() != 0
    }
    fn clr_compare_flag(&self) -> &Self {
        self.periph().with_csc(self.index(), |r| r.set_chf(0));
        self    
    }
}