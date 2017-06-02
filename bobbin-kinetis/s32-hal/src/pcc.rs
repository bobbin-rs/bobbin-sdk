use ::chip::pcc::*;

pub enum ClockSource {
    Disabled = 0b000,
    SOSCDIV2 = 0b001,
    SIRCDIV2 = 0b010,
    FIRCDIV2 = 0b011,
    SPLLDIV2 = 0b110,
}

pub trait PccEnabled {
    fn pcc_enabled(&self) -> bool;
    fn pcc_set_enabled(&self, value: bool) -> &Self;
    fn pcc_enable(&self) -> &Self { self.pcc_set_enabled(true); self }
    fn pcc_disable(&self) -> &Self { self.pcc_set_enabled(false); self }
}

impl<P> PccEnabled for P where P: Cgc {
    fn pcc_enabled(&self) -> bool {
        self.cgc() != 0
    }
    fn pcc_set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.set_cgc(value);
        self
    }
}

pub trait PccClockSource {
    fn pcc_clock_source(&self) -> ClockSource;
    fn pcc_set_clock_source(&self, value: ClockSource) -> &Self; 
}

impl<P> PccClockSource for P where P: Pcs {
    fn pcc_clock_source(&self) -> ClockSource {
        match self.pcs() {
            0b000 => ClockSource::Disabled,
            0b001 => ClockSource::SOSCDIV2,
            0b010 => ClockSource::SIRCDIV2,
            0b011 => ClockSource::FIRCDIV2,
            0b110 => ClockSource::SPLLDIV2,
            v @ _ => panic!("Unknown clocksource {:03b}", v),
        }
    }
    fn pcc_set_clock_source(&self, value: ClockSource) -> &Self {
        self.set_pcs(value as u32);
        self
    }
}