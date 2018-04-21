//! Traits used for modeling peripheral clock gates.

use ::bits;

/// Query and Set the Peripheral Clock Enable Gate
pub trait GateEn {
    /// Returns true if the gate is enabled.
    fn gate_en(&self) -> bits::U1;
    /// Set the gate state to `value`.
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    /// Enable the gate for the peripheral.
    fn gate_enable(&self) -> &Self { self.set_gate_en(true) }
    /// Disable the gate for the peripheral.
    fn gate_disable(&self) -> &Self { self.set_gate_en(false) }
}    

/// Query and Set the Peripheral Reset Gate.
/// 
/// Enabling this gate will reset the peripheral. Check the MCU documentation
/// to see if the gate will automatically be cleared after a successful reset
/// or if it needs to be manually cleared.
/// 
/// Note: implement a higher-level Reset trait that abstracts the full peripheral
/// reset process.
pub trait GateRst {
    /// Returns true if the Reset gate is enabled.
    fn gate_rst(&self) -> bits::U1;
    /// Set the state of the Reset gate to `value`.
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self;
    /// Toggle the state of the Reset gate.
    fn toggle_gate_reset(&self) -> &Self { self.set_gate_rst(true).set_gate_rst(false) }
}    

/// Query and Set the Peripheral Clock Enable on Sleep Gate
/// 
/// Enabling this gate will allow the peripheral to continue operating while the
/// MCU is in a Sleep state.
pub trait GateSleepEn {
    /// Returns true if the Sleep gate is enabled.
    fn gate_sleep_en(&self) -> bits::U1;
    /// Set the state of the Sleep gate to `value`.
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    /// Enables the Sleep gate.
    fn gate_sleep_enable(&self) -> &Self { self.set_gate_sleep_en(true) }
    /// Disables the Sleep gate.
    fn gate_sleep_disable(&self) -> &Self { self.set_gate_sleep_en(false) }
}

/// Query and Set the Peripheral Clock Enable on Deep Sleep Gate
/// 
/// Enabling this gate will allow the peripheral to continue operating while
/// the MCU is in a Deep Sleep state.
pub trait GateDeepSleepEn {
    /// Returns true if the Deep Sleep gate is enabled.
    fn gate_deep_sleep_en(&self) -> bits::U1;
    /// Sets the state of the Deep Sleep gate to `value`.
    fn set_gate_deep_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    /// Enables the Deep Sleep gate.
    fn gate_deep_sleep_enable(&self) -> &Self { self.set_gate_deep_sleep_en(true) }
    /// Disables the Deep Sleep gate.
    fn gate_deep_sleep_disable(&self) -> &Self { self.set_gate_deep_sleep_en(false) }
}

/// Query and Sets the Peripheral Clock Enable on Stop Gate
/// 
/// Enabling this gate wil allow the peripheral to continue operating while
/// the MCU is in a Stop state.
pub trait GateStopEn {
    /// Returns true if the Stop gate is enabled.
    fn gate_stop_en(&self) -> bits::U1;
    /// Sets the state of the Stop gate to `value`.
    fn set_gate_stop_en<V: Into<bits::U1>>(&self, value: V) -> &Self; 
    /// Enables the Stop gate.
    fn gate_stop_enable(&self) -> &Self { self.set_gate_stop_en(true) }
    /// Disables the Stop gate.
    fn gate_stop_disable(&self) -> &Self { self.set_gate_stop_en(false) }
}
