use bobbin_common::bits::*;
pub use ::chip::sim::*;

pub trait SimEnabled {
    fn sim_enabled(&self) -> bool;
    fn sim_set_enabled(&self, value: bool) -> &Self;
    fn sim_enable(&self) -> &Self { self.sim_set_enabled(true); self }
    fn sim_disable(&self) -> &Self { self.sim_set_enabled(false); self }
}

impl<P> SimEnabled for P where P: En {
    fn sim_enabled(&self) -> bool {
        self.en() != 0
    }
    fn sim_set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.set_en(value);
        self
    }
}

pub trait SimSrc {
    fn sim_src(&self) -> u32;
    fn sim_set_src(&self, value: u32) -> &Self;
}

impl<P> SimSrc for P where P: Src {
    fn sim_src(&self) -> u32 {
        self.src()
    }
    fn sim_set_src(&self, value: u32) -> &Self {
        self.set_src(value);
        self
    }
}

pub enum CopTimeout {
    Disabled = 0b00,
    Short = 0b01, 
    Medium = 0b10,
    Long = 0b11,
}

pub enum CopSource {
    Khz = 0,
    Bus = 1,
}

pub trait SimExt {
    fn cop_reset(&self) -> &Self;
    fn cop_source(&self) -> CopSource;
    fn cop_set_source(&self, CopSource) -> &Self;
    fn cop_windowed(&self) -> bool;
    fn cop_set_windowed(&self, bool) -> &Self;
    fn cop_timeout(&self) -> CopTimeout;
    fn cop_set_timeout(&self, CopTimeout) -> &Self;
}

impl SimExt for Sim {
    fn cop_reset(&self) -> &Self {
        self.set_srvcop(|r| r.set_srvcop(0x55));
        self.set_srvcop(|r| r.set_srvcop(0xAA));
        self
    }
    fn cop_source(&self) -> CopSource {
        match self.copc().copclks() {
            U1::B0 => CopSource::Khz,
            U1::B1 => CopSource::Bus,
        }
    }
    fn cop_set_source(&self, value: CopSource) -> &Self {
        self.with_copc(|r| r.set_copclks(value as u32))
    }
    fn cop_windowed(&self) -> bool {
        self.copc().copw() != 0
    }
    fn cop_set_windowed(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_copc(|r| r.set_copw(value))
    }
    fn cop_timeout(&self) -> CopTimeout {
        match self.copc().copt() {
            U2::B00 => CopTimeout::Disabled,
            U2::B01 => CopTimeout::Short,
            U2::B10 => CopTimeout::Medium,
            U2::B11 => CopTimeout::Long,
        }
    }
    fn cop_set_timeout(&self, value: CopTimeout) -> &Self {
        self.with_copc(|r| r.set_copt(value as u32))
    }
}