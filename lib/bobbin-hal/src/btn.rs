//! Abstractions for reading with digital buttons at
//! a logical level. In this context, a button is "off" when it is in its
//! default state, and "on" after an action has been taken. If a button
//! has no single default state, then logical "on" should correspond with
//! any marking indicators that are present or the expected conventions
//! for the use of the switch.

use digital::DigitalInput;

/// Reads the logical state of a button.
pub trait Btn {
    /// Returns true if the button is active.
    fn on(&self) -> bool;
    /// Returns true if the button is not active.
    fn off(&self) -> bool { !self.on() }
}

/// A wrapper for a [DigitalInput](../digital/index.html) that is
/// logically active when the digital input signal is high (true, 1).
pub struct BtnHigh<T: DigitalInput> {
    pub pin: T,
}

impl<T: DigitalInput> BtnHigh<T> {
    /// Returns a new `BtnHigh` object.
    pub const fn new(pin: T) -> Self {
        BtnHigh { pin }
    }
}

impl<T: DigitalInput> Btn for BtnHigh<T> {
    fn on(&self) -> bool {
        self.pin.input()
    }
}

/// A wrapper for a [DigitalInput](../digital/index.html) that is
/// logically active when the digital input signal is low (zero, 0).
pub struct BtnLow<T: DigitalInput> {
    pub pin: T,
}

impl<T: DigitalInput> BtnLow<T> {
    /// Returns a new `BtnLow` object.
    pub const fn new(pin: T) -> Self {
        BtnLow { pin }
    }
}

impl<T: DigitalInput> Btn for BtnLow<T> {
    fn on(&self) -> bool {
        !self.pin.input()
    }
}