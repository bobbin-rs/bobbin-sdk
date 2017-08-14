use bobbin_common::bits::*;
pub use bobbin_common::analog::AnalogRead;
pub use ::chip::adc::*;
pub use super::sysctl::SysctlEnabled;
pub use super::gpio::ModeAin;

pub trait AdcExt {
    fn init(&self) -> &Self;
}

impl<T> AdcExt for Periph<T> {
    fn init(&self) -> &Self {
        self
    }
}

impl<P, T> AnalogRead<U12> for Channel<P, T> {
    fn analog_read(&self) -> U12 {
        self.periph
            .with_actss(|r| r.set_asen(0, 0))
            .with_ssctl(0, |r| r.set_ts(0, 0).set_ie(0, 1).set_end(0, 1))
            .with_ssmux(0, |r| r.set_mux(0, self.index()))
            .with_actss(|r| r.set_asen(0, 1))
            .set_pssi(|r| r.set_ss(0, 1));
        while self.periph.ris().inr(0) == 0 {}
        self.periph
            .set_isc(|r| r.set_in(0, 1))
            .ssfifo(0).data()
    }
}