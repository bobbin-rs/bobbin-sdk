//! Abstractions for accessing and controlling LEDs.

use digital::DigitalOutput;

/// Returns a LED by index.
pub trait GetLed {
    /// Return the LED for index `index`. Panics if `index` is not less than the
    /// LED count.
    fn get_led(&self, index: usize) -> &Led;
    /// Return the number of LEDs available.
    fn get_led_count(&self) -> usize;
}

/// Abstracts a simple digital LED.
pub trait Led {
    /// Turns the LED on.
    fn on(&self);
    /// Turns the LED off.
    fn off(&self);
    /// Toggles the LED output.
    fn toggle(&self);
    /// Returns true if the LED is on.
    fn read(&self) -> bool;
}

/// A wrapper for a [DigitalOutput](../digital/index.html) that is active
/// when the output is high (true, 1).
pub struct LedHigh<T: DigitalOutput> {
    pub pin: T,
}

impl<T: DigitalOutput> LedHigh<T> {
    /// Returns a new `LedHigh` wrapper.
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

/// A wrapper for a [DigitalOutput](../digital/index.html) that is active
/// when the output is low (false, 0).
pub struct LedLow<T: DigitalOutput> {
    pub pin: T,
}

impl<T: DigitalOutput> LedLow<T> {
    /// Returns a new `LedLow` wrapper.
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