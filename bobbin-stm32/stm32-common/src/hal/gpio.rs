use core::marker::PhantomData;
use chip::gpio::{self, GpioImpl};

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

pub struct ModeUnknown;
pub struct ModeOutput;
pub struct ModeInput;
pub struct ModeAltFn;
pub struct ModeAnalog;

pub struct Pin<M> {
    port: GpioImpl,
    index: usize,
    phantom: PhantomData<M>,
}

pub type PinUnknown = Pin<ModeUnknown>;
pub type PinOutput = Pin<ModeOutput>;
pub type PinInput = Pin<ModeInput>;
pub type PinAltFn = Pin<ModeAltFn>;
pub type PinAnalog = Pin<ModeAnalog>;

pub fn pinfn(pinfn: (GpioImpl, usize, usize)) -> Pin<ModeAltFn> {
    pin((pinfn.0, pinfn.1)).into_altfn(pinfn.2)
}

pub fn pin(pin: (GpioImpl, usize)) -> Pin<ModeUnknown> {
    Pin { port: pin.0, index: pin.1, phantom: PhantomData }
}

pub fn pin_output(pin: (GpioImpl, usize)) -> Pin<ModeOutput> {
    Pin { port: pin.0, index: pin.1, phantom: PhantomData }
}

pub fn pin_input(pin: (GpioImpl, usize)) -> Pin<ModeInput> {
    Pin { port: pin.0, index: pin.1, phantom: PhantomData }
}

pub fn pin_altfn(pin: (GpioImpl, usize)) -> Pin<ModeAltFn> {
    Pin { port: pin.0, index: pin.1, phantom: PhantomData }
}

pub fn pin_analog(pin: (GpioImpl, usize)) -> Pin<ModeAnalog> {
    Pin { port: pin.0, index: pin.1, phantom: PhantomData }
}

impl<T> Pin<T> {
    pub fn gpio(&self) -> GpioImpl {
        self.port
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_mode(&self, value: Mode) {
        let port = self.port;
        unsafe {
            port.with_moder(|r| r.set_moder(self.index, value as u32))
        }
    }

    pub fn set_output_type(&self, value: OutputType) {
        let port = self.port;
        unsafe {
            port.with_otyper(|r| r.set_ot(self.index, value as u32))
        }
    }

    pub fn set_output_speed(&self, value: OutputSpeed) {
        let port = self.port;
        unsafe {
            port.with_ospeedr(|r| r.set_ospeedr(self.index, value as u32))
        }
    }

    pub fn set_pull(&self, value: Pull) {
        let port = self.port;
        unsafe {
            port.with_pupdr(|r| r.set_pupdr(self.index, value as u32))
        }        
    }

    pub fn set_altfn(&self, value: usize) {
        let port = self.port;
        let pin = self.index;
        unsafe {
            if pin < 8 {
                port.with_afrl(|r| r.set_afrl(pin, value as u32))
            } else {
                port.with_afrh(|r| r.set_afrh(pin - 8, value as u32))
            }
        }        
    }

    pub fn into_output(self) -> Pin<ModeOutput> {
        self.set_mode(Mode::Output);
        unsafe { self.into_output_unchecked() }
    }

    pub unsafe fn into_output_unchecked(self) -> Pin<ModeOutput> {
        Pin { port: self.port, index: self.index, phantom: PhantomData }
    }

    pub fn into_input(self, value: Pull) -> Pin<ModeInput> {
        self.set_mode(Mode::Input);
        self.set_pull(value);
        unsafe { self.into_input_unchecked() }
    }

    pub unsafe fn into_input_unchecked(self) -> Pin<ModeInput> {
        Pin { port: self.port, index: self.index, phantom: PhantomData }
    }

    pub fn into_altfn(self, value: usize) -> Pin<ModeAltFn> {
        self.set_mode(Mode::AltFn);
        self.set_altfn(value);
        unsafe { self.into_altfn_unchecked() }
    }

    pub unsafe fn into_altfn_unchecked(self) -> Pin<ModeAltFn> {
        Pin { port: self.port, index: self.index, phantom: PhantomData }
    }

    pub fn into_analog(self) -> Pin<ModeAnalog> {
        self.set_mode(Mode::Analog);
        Pin { port: self.port, index: self.index, phantom: PhantomData }
    }

    pub unsafe fn into_analog_unchecked(self) -> Pin<ModeAnalog> {
        Pin { port: self.port, index: self.index, phantom: PhantomData }
    }    

}

impl Pin<ModeOutput> {    
    pub fn get(&self) -> bool {
        let port = self.port;
        unsafe {
            port.odr().odr(self.index) != 0
        }        
    }    

    pub fn set(&self, value: bool) {
        let port = self.port;        
        unsafe {
            port.set_bsrr(
                if value {
                    gpio::Bsrr(0).set_bs(self.index, 1)
                } else {
                    gpio::Bsrr(0).set_br(self.index, 1)
                }
            );
        }
    }    
    pub fn toggle(&self) {
        self.set(!self.get())
    }
}

impl Pin<ModeInput> {
    pub fn get(&self) -> bool {
        let port = self.port;
        unsafe {
            port.idr().idr(self.index) != 0
        }        
    }        
}
