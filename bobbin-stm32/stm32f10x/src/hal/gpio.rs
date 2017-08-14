pub use bobbin_common::digital::*;
pub use chip::gpio::*;
pub use super::rcc::RccEnabled;

pub trait PinExt {
    // fn mode(&self) -> Mode;
    // fn set_mode(&self, value: Mode) -> &Self;       
    fn mode_input(&self) -> &Self;
    fn mode_output(&self) -> &Self;
    fn mode_altfn(&self) -> &Self;
    fn mode_analog(&self) -> &Self;
}

impl<P, T> PinExt for Pin<P,T> {
    #[inline]
    fn mode_output(&self) -> &Self {
        match self.index {
            0 ... 8 => {
                self.port.with_crl(|r| r.set_cnf(self.index, 0b00).set_mode(self.index, 0b01));
            },
            _ => {
                self.port.with_crh(|r| r.set_cnf(self.index - 8, 0b00).set_mode(self.index - 8, 0b01));
            }
        }
        self
    }
    #[inline]
    fn mode_input(&self) -> &Self {
        match self.index {
            0 ... 8 => {
                self.port.with_crl(|r| r.set_cnf(self.index, 0b10).set_mode(self.index, 0b00));
            },
            _ => {
                self.port.with_crh(|r| r.set_cnf(self.index - 8, 0b10).set_mode(self.index - 8, 0b00));
            }
        }
        self
    }
    
    #[inline]
    fn mode_altfn(&self) -> &Self {
        match self.index {
            0 ... 8 => {
                self.port.with_crl(|r| r.set_cnf(self.index, 0b10).set_mode(self.index, 0b01));
            },
            _ => {
                self.port.with_crh(|r| r.set_cnf(self.index - 8, 0b10).set_mode(self.index - 8, 0b01));
            }
        }
        self
    }
    #[inline]
    fn mode_analog(&self) -> &Self {
        match self.index {
            0 ... 8 => {
                self.port.with_crl(|r| r.set_cnf(self.index, 0).set_mode(self.index, 0));
            },
            _ => {
                self.port.with_crh(|r| r.set_cnf(self.index - 8, 0).set_mode(self.index - 8, 0));
            }
        }
        self
    }    
}

impl<P, T> DigitalOutput for Pin<P, T> {
    #[inline]
    fn output(&self) -> bool {
        self.port.odr().odr(self.index) != 0
    }

    #[inline]
    fn set_output(&self, value: bool) -> &Self {
        if value {
            self.port.set_bsrr(|r| r.set_bs(self.index, 1))
        } else {
            self.port.set_bsrr(|r| r.set_br(self.index, 1))
        };
        self
    }

    #[inline]
    fn toggle_output(&self) -> &Self {
        if self.port.odr().odr(self.index) == 0 {
            self.port.set_bsrr(|r| r.set_bs(self.index, 1))
        } else {
            self.port.set_bsrr(|r| r.set_br(self.index, 1))
        };
        self
    }    
}

impl<P, T> DigitalInput for Pin<P, T> {
    fn input(&self) -> bool {
        self.port.idr().idr(self.index) != 0
    }
}