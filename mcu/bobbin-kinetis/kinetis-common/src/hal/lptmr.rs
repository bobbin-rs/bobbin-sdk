pub use bobbin_common::enabled::*;
pub use bobbin_common::timer::*;
pub use chip::lptmr::*;
pub use core::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PrescalerClock {
    McgIrClk = 0b00,
    Lpo = 0b01,
    ErClk32k = 0b10,
    OscErClk = 0b11,
}

impl LptmrPeriph {
    pub fn pcs(&self) -> PrescalerClock {
        unsafe {
            ::core::mem::transmute(self.psr().pcs())
        }
    }

    pub fn set_pcs(&self, value: PrescalerClock) -> &Self {
        self.with_psr(|r| r.set_pcs(value as u8))
    }
}

impl Enabled for LptmrPeriph {
    fn enabled(&self) -> bool {
        self.csr().test_ten()
    }
    fn set_enabled(&self, value: bool) -> &Self {
        self.with_csr(|r| r.set_ten(value))
    }
}

impl Start<u16> for LptmrPeriph {
    fn start(&self, value: u16) -> &Self {
        self.start_up(value)
    }
}

impl StartUp<u16> for LptmrPeriph {
    fn start_up(&self, value: u16) -> &Self {
        self
            .with_csr(|r| r.set_ten(0))
            .set_period(value)
            .clr_timeout()
            .with_csr(|r| r.set_tfc(1).set_ten(1))
    }
}

impl Delay<u16> for LptmrPeriph {
    fn delay(&self, value: u16) -> &Self {
        self
            .set_period(value)
            .with_csr(|r| r.set_tfc(0).set_ten(1))
            .clr_timeout()
            .wait_timeout()
    }
}


impl Stop for LptmrPeriph {
    fn stop(&self) -> &Self {
        self.with_csr(|r| r.set_ten(0))
    }
}

impl Running for LptmrPeriph {
    fn running(&self) -> bool {
        self.csr().ten() != 0
    }
}

impl Period<u16> for LptmrPeriph {
    fn period(&self) -> u16 {
        self.cmr().compare().value()
    }
}

impl SetPeriod<u16> for LptmrPeriph {    
    fn set_period(&self, value: u16) -> &Self {    
        self.set_cmr(|r| r.set_compare(value as u32))
    }
}

impl Counter<u16> for LptmrPeriph {
    fn counter(&self) -> u16 {
        self.set_cnr(|_| Cnr(0)).cnr().counter().value()
    }
}

impl Timeout for LptmrPeriph {
    fn test_timeout(&self) -> bool {
        self.csr().tcf() != 0
    }

    fn clr_timeout(&self) -> &Self {
        self.with_csr(|r| r.set_tcf(1))
    }    
}

impl Compare<u16> for LptmrPeriph {
    fn compare(&self) -> u16 {
        self.cmr().compare().value()
    }
    fn set_compare(&self, value: u16) -> &Self {
        self.set_cmr(|r| r.set_compare(value))
    }
    fn test_compare(&self) -> bool {
        self.csr().test_tcf()
    }
    fn clr_compare(&self) -> &Self {
        self.with_csr(|r| r.set_tcf(1))
    }
}

pub fn test_lptmr(tim: &LptmrPeriph) {
    test_timer(tim, 1024);
    test_timer_up(tim, 1024);    
}