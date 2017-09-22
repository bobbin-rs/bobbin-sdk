pub use bobbin_common::digital::*;
pub use bobbin_common::{AltFn, Pin};
pub use ::chip::gpio::*;

use ::chip::cmu::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Disabled = 0x0,
    Input = 0x1,
    InputPull = 0x2,
    InputPullFilter = 0x3,
    PushPull = 0x4,
    PushPullAlt = 0x5,
    WiredOr = 0x6,
    WiredOrPullDown = 0x7,
    WiredAnd = 0x8,
    WiredAndFilter = 0x9,
    WiredAndPullUp = 0xA,
    WiredAndPullUpFilter = 0xB,
    WiredAndAlt = 0xC,
    WiredAndAltFilter = 0xD,
    WiredAndAltPullUp = 0xE,
    WiredAndAltPullUpFilter = 0xF,
}

pub fn init() {
    // Enable GPIO
    CMU.with_hfbusclken0(|r| r.set_gpio(true));
}

impl GpioPin {
    pub fn set_mode(&self, value: Mode) -> &Self {
        let value = value as u8;
        if (self.index >> 3) & 0b1 != 0 {
            self.port.with_modeh(|r| r.set_mode(self.index & 0b111, value))
        } else {
            self.port.with_model(|r| r.set_mode(self.index & 0b111, value))
        };
        self
    }

    pub fn mode_disabled(&self) -> &Self {
        self.set_mode(Mode::Disabled)
    }

    pub fn mode_input(&self) -> &Self {
        self.set_mode(Mode::Input)
    }

    pub fn mode_input_pull(&self) -> &Self {
        self.set_mode(Mode::InputPull)
    }

    pub fn mode_input_pull_filter(&self) -> &Self {
        self.set_mode(Mode::InputPullFilter)
    }

    pub fn mode_push_pull(&self) -> &Self {
        self.set_mode(Mode::PushPull)
    }

    pub fn mode_push_pull_alt(&self) -> &Self {
        self.set_mode(Mode::PushPullAlt)
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