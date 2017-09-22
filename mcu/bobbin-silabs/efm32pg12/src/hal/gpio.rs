pub use bobbin_common::digital::*;
pub use bobbin_common::{AltFn, Pin};
pub use ::chip::gpio::*;

use bobbin_common::bits::*;

impl GpioPin {
    pub fn set_mode<V: Into<U4>>(&self, value: V) -> &Self {
        if (self.index >> 3) & 0b1 != 0 {
            self.port.with_modeh(|r| r.set_mode(self.index & 0b111, value))
        } else {
            self.port.with_model(|r| r.set_mode(self.index & 0b111, value))
        };
        self
    }

    pub fn mode_output(&self) -> &Self {
        self.set_mode(0b0100)
    }

    pub fn mode_input(&self) -> &Self {
        self.set_mode(0b001)
    }
}

impl DigitalInput for GpioPin {
    fn input(&self) -> bool {
        self.port.din().test_din(self.index)
    }
}

impl DigitalOutput for GpioPin {
    fn output(&self) -> bool {
        self.port.dout().test_dout(self.index)
    }

    fn set_output(&self, value: bool) -> &Self {
        self.port.with_dout(|r| r.set_dout(self.index, value));
        self
    }

    fn toggle_output(&self) -> &Self {
        self.port.set_douttgl(|r| r.set_douttgl(self.index, 1));
        self
    }
}