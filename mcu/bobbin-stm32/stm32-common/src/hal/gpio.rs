pub use chip::gpio::*;
pub use bobbin_common::digital::*;
use bobbin_common::bits::*;
use bobbin_common::pin::SetSource;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Input = 0b00,
    Output = 0b01,
    AltFn = 0b10,
    Analog = 0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputType {
    PushPull = 0,
    OpenDrain = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputSpeed {
    LowSpeed =    0b00,
    MediumSpeed = 0b01,
    FastSpeed =   0b10,
    HighSpeed =   0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pull {
    None =     0b00,
    PullUp =   0b01,
    PullDown = 0b10,
    Reserved = 0b11,
}

impl GpioPin {
    #[inline]
    pub fn mode(&self) -> Mode {
        match self.port.moder().moder(self.index) {
            U2::B00 => Mode::Input,
            U2::B01 => Mode::Output,
            U2::B10 => Mode::AltFn,
            U2::B11 => Mode::Analog,
        }
    }

    #[inline]
    pub fn set_mode(&self, value: Mode) -> &Self {
        self.port.with_moder(|r| r.set_moder(self.index, value as u32));
        self
    }

    #[inline]
    pub fn output_type(&self) -> OutputType {
        match self.port.otyper().ot(self.index) {
            U1::B0 => OutputType::PushPull,
            U1::B1 => OutputType::OpenDrain,
        }
    }

    #[inline]
    pub fn set_output_type(&self, value: OutputType) -> &Self {
        self.port.with_otyper(|r| r.set_ot(self.index, value as u32));
        self
    }

    #[inline]
    pub fn output_speed(&self) -> OutputSpeed {
        match self.port.ospeedr().ospeedr(self.index) {
            U2::B00 => OutputSpeed::LowSpeed,
            U2::B01 => OutputSpeed::MediumSpeed,
            U2::B10 => OutputSpeed::FastSpeed,
            U2::B11 => OutputSpeed::HighSpeed,
        }
    }

    #[inline]
    pub fn set_output_speed(&self, value: OutputSpeed) -> &Self {
        self.port.with_ospeedr(|r| r.set_ospeedr(self.index, value as u32));
        self
    }

    #[inline]
    pub fn pull(&self) -> Pull {
        match self.port.pupdr().pupdr(self.index) {
            U2::B00 => Pull::None,
            U2::B01 => Pull::PullUp,
            U2::B10 => Pull::PullDown,
            U2::B11 => Pull::Reserved,
        }
    }

    #[inline]
    pub fn set_pull(&self, value: Pull) -> &Self {
        self.port.with_pupdr(|r| r.set_pupdr(self.index, value as u32));
        self
    }

    #[inline]
    pub fn alt_fn(&self) -> usize {
        if self.index < 8 {
            self.port.afrl().afrl(self.index) as usize
        } else {
            self.port.afrh().afrh(self.index) as usize
        }
    }
    
    #[inline]
    pub fn set_alt_fn(&self, value: usize) -> &Self {
        if self.index < 8 {
            self.port.with_afrl(|r| r.set_afrl(self.index, value as u32));
        } else {
            self.port.with_afrh(|r| r.set_afrh(self.index - 8, value as u32));
        };
        self
    }

    #[inline]
    pub fn mode_output(&self) -> &Self {
        self.set_mode(Mode::Output)
    }

    #[inline]
    pub fn mode_input(&self) -> &Self {
        self.set_mode(Mode::Input)
    }

    #[inline]
    pub fn mode_alt_fn(&self, af: usize) -> &Self {
        self.set_mode(Mode::AltFn).set_alt_fn(af)
    }

    #[inline]
    pub fn mode_analog(&self) -> &Self {
        self.set_mode(Mode::Analog)
    }

    #[inline]
    pub fn pull_none(&self) -> &Self {
        self.set_pull(Pull::None)
    }

    #[inline]
    pub fn pull_up(&self) -> &Self {
        self.set_pull(Pull::PullUp)
    }

    #[inline]
    pub fn pull_down(&self) -> &Self {
        self.set_pull(Pull::PullDown)
    }

    #[inline]
    pub fn push_pull(&self) -> &Self {
        self.set_output_type(OutputType::PushPull)
    }

    #[inline]
    pub fn open_drain(&self) -> &Self {
        self.set_output_type(OutputType::OpenDrain)
    }

    #[inline]
    pub fn speed_low(&self) -> &Self {
        self.set_output_speed(OutputSpeed::LowSpeed)
    }

    #[inline]
    pub fn speed_medium(&self) -> &Self {
        self.set_output_speed(OutputSpeed::MediumSpeed)
    }

    #[inline]
    pub fn speed_fast(&self) -> &Self {
        self.set_output_speed(OutputSpeed::FastSpeed)
    }

    #[inline]
    pub fn speed_high(&self) -> &Self {
        self.set_output_speed(OutputSpeed::HighSpeed)
    }

}

impl DigitalInput for GpioPin {
    #[inline]
    fn input(&self) -> bool {
        self.port.idr().idr(self.index) != 0
    }
}

impl DigitalOutput for GpioPin {
    #[inline]
    fn output(&self) -> bool {
        self.port.odr().odr(self.index) != 0
    }

    #[inline]
    fn set_output(&self, value: bool) -> &Self {
        self.port.set_bsrr(|r|
            if value {
                r.set_bs(self.index, 1)
            } else {
                r.set_br(self.index, 1)
            }
        );
        self
    }

    #[inline]
    fn toggle_output(&self) -> &Self {
        self.port.set_bsrr(|r|
            if self.port.idr().idr(self.index) == 0 {
                r.set_bs(self.index, 1)
            } else {
                r.set_br(self.index, 1)
            }
        );
        self
    }
}

impl SetSource for GpioPin {
    fn set_source(&self, src: u8) {
        self.mode_alt_fn(src as usize);
    }
}