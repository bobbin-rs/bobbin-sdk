pub use bobbin_common::timer::*;
pub use chip::lptmr::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PrescalerClock {
    McgIrClk = 0b00,
    Lpo = 0b01,
    ErClk32k = 0b10,
    OscErClk = 0b11,
}

pub trait LptmrExt {
    fn pcs(&self) -> PrescalerClock;
    fn set_pcs(&self, value: PrescalerClock) -> &Self;
}

impl<T> LptmrExt for Periph<T> {
    fn pcs(&self) -> PrescalerClock {
        unsafe {
            ::core::mem::transmute(self.psr().pcs())
        }
    }

    fn set_pcs(&self, value: PrescalerClock) -> &Self {
        self.with_psr(|r| r.set_pcs(value as u8))
    }
}

impl<T> Start<u16> for Periph<T> {
    fn start(&self, value: u16) -> &Self {
        self.start_up(value)
    }
}

impl<T> StartUp<u16> for Periph<T> {
    fn start_up(&self, value: u16) -> &Self {
        self
            .set_period(value)
            .with_csr(|r| r.set_tfc(1).set_ten(1))
    }
}

impl<T> StartUpOnce<u16> for Periph<T> {
    fn start_up_once(&self, value: u16) -> &Self {
        self
            .set_period(value)
            .with_csr(|r| r.set_tfc(0).set_ten(1))
    }
}

impl<T> Delay<u16> for Periph<T> {
    fn delay(&self, value: u16) -> &Self {
        self
            .set_period(value)
            .with_csr(|r| r.set_tfc(0).set_ten(1))
            .clr_timeout_flag()
            .wait_timeout_flag()
    }
}


impl<T> Timer<u16> for Periph<T> {
    fn stop(&self) -> &Self {
        self.with_csr(|r| r.set_ten(0))
    }

    fn running(&self) -> bool {
        self.csr().ten() != 0
    }

    fn period(&self) -> u16 {
        self.cmr().compare().value()
    }
    
    fn set_period(&self, value: u16) -> &Self {    
        self.set_cmr(|r| r.set_compare(value as u32))
    }

    fn counter(&self) -> u16 {
        self.cnr().counter().value()
    }

    fn timeout_flag(&self) -> bool {
        self.csr().tcf() != 0
    }

    fn clr_timeout_flag(&self) -> &Self {
        self.with_csr(|r| r.set_tcf(1))
    }    
}