use chip::gpio::{self, PinImpl};

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

    fn input(&self) -> bool;
    fn output(&self) -> bool;
    fn set_output(&self, value: bool) -> &Self;
    fn toggle_output(&self) -> &Self;
}

impl PinExt for PinImpl {
    fn mode(&self) -> Mode {
        match self.port.moder().moder(self.index) {
            0b00 => Mode::Input,
            0b01 => Mode::Output,
            0b10 => Mode::AltFn,
            0b11 => Mode::Analog,
            _ => panic!("Invalid value for mode")
        }
    }

    fn set_mode(&self, value: Mode) -> &Self {
        self.port.with_moder(|r| r.set_moder(self.index, value as u32));
        self
    }

    fn output_type(&self) -> OutputType {
        match self.port.otyper().ot(self.index) {
            0b0 => OutputType::PushPull,
            0b1 => OutputType::OpenDrain,
            _ => panic!("Invalid value for output_type")
        }
    }

    fn set_output_type(&self, value: OutputType) -> &Self {
        self.port.with_otyper(|r| r.set_ot(self.index, value as u32));
        self
    }

    fn output_speed(&self) -> OutputSpeed {
        match self.port.ospeedr().ospeedr(self.index) {
            0b00 => OutputSpeed::LowSpeed,
            0b01 => OutputSpeed::MediumSpeed,
            0b10 => OutputSpeed::FastSpeed,
            0b11 => OutputSpeed::HighSpeed,
            _ => panic!("Invalid value for output_speed")
        }
    }

    fn set_output_speed(&self, value: OutputSpeed) -> &Self {
        self.port.with_ospeedr(|r| r.set_ospeedr(self.index, value as u32));
        self
    }

    fn pull(&self) -> Pull {
        match self.port.pupdr().pupdr(self.index) {
            0b00 => Pull::None,
            0b01 => Pull::PullUp,
            0b10 => Pull::PullDown,
            0b11 => Pull::Reserved,
            _ => panic!("Invalid value for pull")
        }
    }

    fn set_pull(&self, value: Pull) -> &Self {
        self.port.with_pupdr(|r| r.set_pupdr(self.index, value as u32));
        self
    }

    fn altfn(&self) -> usize {
        if self.index < 8 {
            self.port.afrl().afrl(self.index) as usize
        } else {
            self.port.afrh().afrh(self.index) as usize
        }
    }
    fn set_altfn(&self, value: usize) -> &Self {
        if self.index < 8 {
            self.port.with_afrl(|r| r.set_afrl(self.index, value as u32))
        } else {
            self.port.with_afrh(|r| r.set_afrh(self.index - 8, value as u32))
        };
        self
    }

    fn input(&self) -> bool {
        self.port.idr().idr(self.index) != 0
    }

    fn output(&self) -> bool {
        self.port.odr().odr(self.index) != 0
    }

    fn set_output(&self, value: bool) -> &Self {
        self.port.set_bsrr(
            if value {
                gpio::Bsrr(0).set_bs(self.index, 1)
            } else {
                gpio::Bsrr(0).set_br(self.index, 1)
            }
        );
        self
    }

    fn toggle_output(&self) -> &Self {
        self.port.set_bsrr(
            if self.port.idr().idr(self.index) == 0 {
                gpio::Bsrr(0).set_bs(self.index, 1)
            } else {
                gpio::Bsrr(0).set_br(self.index, 1)
            }
        );
        self
    }

}