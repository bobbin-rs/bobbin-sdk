pub use bobbin_common::digital::*;
use bobbin_common::bits::*;
use chip::gpio::Pin;

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

pub trait PinExt {
    fn mode(&self) -> Mode;
    fn set_mode(&self, value: Mode) -> &Self;
    fn output_type(&self) -> OutputType;
    fn set_output_type(&self, value: OutputType) -> &Self;
    fn output_speed(&self) -> OutputSpeed;
    fn set_output_speed(&self, value: OutputSpeed) -> &Self;
    fn pull(&self) -> Pull;
    fn set_pull(&self, value: Pull) -> &Self;
    fn altfn(&self) -> usize;
    fn set_altfn(&self, value: usize) -> &Self;

    fn mode_input(&self) -> &Self;
    fn mode_output(&self) -> &Self;
    fn mode_altfn(&self, usize) -> &Self;
    fn mode_analog(&self) -> &Self;

    fn pull_up(&self) -> &Self;
    fn pull_down(&self) -> &Self;

    fn push_pull(&self) -> &Self;
    fn open_drain(&self) -> &Self;

    // fn input(&self) -> bool;
    // fn output(&self) -> bool;
    // fn set_output(&self, value: bool) -> &Self;
    // fn toggle_output(&self) -> &Self;
}

impl<P, T> PinExt for Pin<P,T> {
    #[inline]
    fn mode(&self) -> Mode {
        match self.port.moder().moder(self.index) {
            U2::B00 => Mode::Input,
            U2::B01 => Mode::Output,
            U2::B10 => Mode::AltFn,
            U2::B11 => Mode::Analog,
        }
    }

    #[inline]
    fn set_mode(&self, value: Mode) -> &Self {
        self.port.with_moder(|r| r.set_moder(self.index, value as u32));
        self
    }

    #[inline]
    fn output_type(&self) -> OutputType {
        match self.port.otyper().ot(self.index) {
            U1::B0 => OutputType::PushPull,
            U1::B1 => OutputType::OpenDrain,
        }
    }

    #[inline]
    fn set_output_type(&self, value: OutputType) -> &Self {
        self.port.with_otyper(|r| r.set_ot(self.index, value as u32));
        self
    }

    #[inline]
    fn output_speed(&self) -> OutputSpeed {
        match self.port.ospeedr().ospeedr(self.index) {
            U2::B00 => OutputSpeed::LowSpeed,
            U2::B01 => OutputSpeed::MediumSpeed,
            U2::B10 => OutputSpeed::FastSpeed,
            U2::B11 => OutputSpeed::HighSpeed,
        }
    }

    #[inline]
    fn set_output_speed(&self, value: OutputSpeed) -> &Self {
        self.port.with_ospeedr(|r| r.set_ospeedr(self.index, value as u32));
        self
    }

    #[inline]
    fn pull(&self) -> Pull {
        match self.port.pupdr().pupdr(self.index) {
            U2::B00 => Pull::None,
            U2::B01 => Pull::PullUp,
            U2::B10 => Pull::PullDown,
            U2::B11 => Pull::Reserved,
        }
    }

    #[inline]
    fn set_pull(&self, value: Pull) -> &Self {
        self.port.with_pupdr(|r| r.set_pupdr(self.index, value as u32));
        self
    }

    #[inline]
    fn altfn(&self) -> usize {
        if self.index < 8 {
            self.port.afrl().afrl(self.index) as usize
        } else {
            self.port.afrh().afrh(self.index) as usize
        }
    }
    
    #[inline]
    fn set_altfn(&self, value: usize) -> &Self {
        if self.index < 8 {
            self.port.with_afrl(|r| r.set_afrl(self.index, value as u32))
        } else {
            self.port.with_afrh(|r| r.set_afrh(self.index - 8, value as u32))
        };
        self
    }

    #[inline]
    fn mode_output(&self) -> &Self {
        self.set_mode(Mode::Output)
    }

    #[inline]
    fn mode_input(&self) -> &Self {
        self.set_mode(Mode::Input)
    }

    #[inline]
    fn mode_altfn(&self, af: usize) -> &Self {
        self.set_mode(Mode::AltFn).set_altfn(af)
    }

    #[inline]
    fn mode_analog(&self) -> &Self {
        self.set_mode(Mode::Analog)
    }

    #[inline]
    fn pull_up(&self) -> &Self {
        self.set_pull(Pull::PullUp)
    }

    #[inline]
    fn pull_down(&self) -> &Self {
        self.set_pull(Pull::PullDown)
    }

    #[inline]
    fn push_pull(&self) -> &Self {
        self.set_output_type(OutputType::PushPull)
    }

    #[inline]
    fn open_drain(&self) -> &Self {
        self.set_output_type(OutputType::OpenDrain)
    }
}

impl<P, T> DigitalInput for Pin<P,T> {
    #[inline]
    fn input(&self) -> bool {
        self.port.idr().idr(self.index) != 0
    }
}

impl<P, T> DigitalOutput for Pin<P,T> {
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