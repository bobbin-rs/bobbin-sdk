use hal::digital::DigitalInput;

pub trait Btn {
    fn on(&self) -> bool;
    fn off(&self) -> bool { !self.on() }
}

pub struct BtnHigh<T: DigitalInput> {
    pub pin: T,
}

impl<T: DigitalInput> BtnHigh<T> {
    pub const fn new(pin: T) -> Self {
        BtnHigh { pin }
    }
}


impl<T: DigitalInput> Btn for BtnHigh<T> {
    fn on(&self) -> bool {
        self.pin.input()
    }
}

pub struct BtnLow<T: DigitalInput> {
    pub pin: T,
}

impl<T: DigitalInput> BtnLow<T> {
    pub const fn new(pin: T) -> Self {
        BtnLow { pin }
    }
}

impl<T: DigitalInput> Btn for BtnLow<T> {
    fn on(&self) -> bool {
        !self.pin.input()
    }
}