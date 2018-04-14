use bobbin_common::bits::*;
pub use bobbin_common::hal::analog::AnalogRead;
pub use ::chip::adc::*;
pub use super::sysctl::SysctlEnabled;
pub use super::gpio::ModeAin;

impl AdcPeriph {
    pub fn init(&self) -> &Self {
        self
    }
}

impl AnalogRead<U12> for AdcCh {
    fn start(&self) -> &Self {
        self.periph
            .with_actss(|r| r.set_asen(0, 0))
            .with_ssctl(0, |r| r.set_ts(0, 0).set_ie(0, 1).set_end(0, 1))
            .with_ssmux(0, |r| r.set_mux(0, self.index))
            .with_actss(|r| r.set_asen(0, 1))
            .set_isc(|r| r.set_in(0, 1))
            .set_pssi(|r| r.set_ss(0, 1));
        self
    }

    fn is_complete(&self) -> bool {
        self.periph.ris().inr(0) != 0
    }

    fn read(&self) -> U12 {
        self.periph.ssfifo(0).data()
    }
}