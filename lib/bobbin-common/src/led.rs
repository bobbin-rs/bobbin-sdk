use hal::digital::DigitalOutput;

pub trait GetLed {
    fn get_led(&self, index: usize) -> &Led;
    fn get_led_count(&self) -> usize;
}

pub trait Led {
    fn on(&self);
    fn off(&self);
    fn toggle(&self);
    fn read(&self) -> bool;
}

pub struct LedHigh<T: DigitalOutput> {
    pub pin: T,
}

impl<T: DigitalOutput> LedHigh<T> {
    pub const fn new(pin: T) -> Self {
        LedHigh { pin }
    }
}


impl<T: DigitalOutput> Led for LedHigh<T> {
    fn on(&self) {
        self.pin.set_output(true);
    }
    fn off(&self) {
        self.pin.set_output(false);
    }
    fn toggle(&self) {
        self.pin.toggle_output();
    }
    fn read(&self) -> bool {
        if self.pin.output() { true } else { false }
    }
}

pub struct LedLow<T: DigitalOutput> {
    pub pin: T,
}

impl<T: DigitalOutput> LedLow<T> {
    pub const fn new(pin: T) -> Self {
        LedLow { pin }
    }
}

impl<T: DigitalOutput> Led for LedLow<T> {
    fn on(&self) {
        self.pin.set_output(false);
    }
    fn off(&self) {
        self.pin.set_output(true);
    }
    fn toggle(&self) {
        self.pin.toggle_output();
    }
    fn read(&self) -> bool {
        if self.pin.output() { false } else { true }
    }
}