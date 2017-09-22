pub use bobbin_common::digital::*;
pub use bobbin_common::{AltFn, Pin};
pub use ::chip::gpio::*;

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