pub use chip::gpio::*;
pub use super::rcc::RccEnabled;

pub trait PinExt {
    // fn mode(&self) -> Mode;
    // fn set_mode(&self, value: Mode) -> &Self;       
    fn mode_input(&self) -> &Self;
    fn mode_output(&self) -> &Self;
    fn mode_altfn(&self) -> &Self;
    fn output(&self) -> bool;
    fn set_output(&self, value: bool) -> &Self;
    fn toggle_output(&self) -> &Self;
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
    fn output(&self) -> bool {
        self.port.odr().odr(self.index) != 0
    }

    #[inline]
    fn set_output(&self, value: bool) -> &Self {
        self.port.set_bsrr(
            if value {
                Bsrr(0).set_bs(self.index, 1)
            } else {
                Bsrr(0).set_br(self.index, 1)
            }
        );
        self
    }

    #[inline]
    fn toggle_output(&self) -> &Self {
        self.port.set_bsrr(
            if self.port.idr().idr(self.index) == 0 {
                Bsrr(0).set_bs(self.index, 1)
            } else {
                Bsrr(0).set_br(self.index, 1)
            }
        );
        self
    }    
}