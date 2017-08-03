pub use bobbin_common::timer::*;
pub use chip::tcc::*;
pub use super::pm::PmEnabled;

pub trait TccExt {
}

impl<T> TccExt for Periph<T> {
}

pub trait TccChExt {
    fn cc(&self) -> u32;
    fn set_cc(&self, u32) -> &Self;
}

impl<P, T> TccChExt for Channel<P, T> {
    fn cc(&self) -> u32 {
        self.periph.cc(self.index).cc()
    }
    fn set_cc(&self, value: u32) -> &Self {
        self.periph.with_cc(self.index, |r| r.set_cc(value));
        self
    }
}

impl<T> Timer<u16> for Periph<T> {
    fn start(&self, value: u16) -> &Self {        
        self.set_period(value);        
        self.with_wave(|r| r.set_wavegen(0x0));
        self.with_ctrla(|r| r.set_enable(0x1));
        self.set_ctrlbset(Ctrlbset(0).set_cmd(0x1));
        self
    }
    fn stop(&self) -> &Self {
        self.set_ctrlbset(Ctrlbset(0).set_cmd(0x2));
        self.with_ctrla(|r| r.set_enable(0x0));
        self
    }
    fn running(&self) -> bool {
        self.status().stop() == 0
    }

    fn period(&self) -> u16 {
        self.per().per() as u16
    }

    fn set_period(&self, value: u16) -> &Self {
        self.set_per(Per(0).set_per(value as u32))        
    }

    fn counter(&self) -> u16 {
        self.count().count() as u16
    }

    fn timeout_flag(&self) -> bool {
        self.intflag().ovf() != 0
    }

    fn clr_timeout_flag(&self) -> &Self {
        self.set_intflag(Intflag(0).set_ovf(1));
        self
    }    
}


impl<T> Prescale<u16> for Periph<T> {
    fn prescale(&self) -> u16 {
        1 << self.ctrla().prescaler() 
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
        self.with_ctrla(|r| r.set_prescaler(shift))
    }
}

impl<T> SetCounter<u16> for Periph<T> {
    fn set_counter(&self, value: u16) -> &Self {
        self.set_count(Count(0).set_count(value as u32));
        self
    }
}


impl<P, T> Compare<u16> for Channel<P, T> {
    fn compare(&self) -> u16 {
        self.periph().cc(self.index()).cc() as u16
    }

    fn set_compare(&self, value: u16) -> &Self {
        self.periph().set_cc(self.index(), Cc(0).set_cc(value as u32));        
        self
    }

    fn compare_flag(&self) -> bool {
        self.periph().intflag().mc(self.index()) != 0
    }

    fn clr_compare_flag(&self) -> &Self {
        self.periph().set_intflag(Intflag(0).set_mc(self.index(), 1));
        self
    }

}