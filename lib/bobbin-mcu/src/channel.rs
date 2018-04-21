//! Abstracts peripheral channels such as DMA channels, Timer channels, etc.
//! 
//! Many MCUs have peripherals that can best be modeled as a set of sub-peripherals, which
//! will be referred to here as `channels`. Typical peripherals that might use channels are:
//! 
//! - Timers
//! - DMA Engines
//! - ADCs
//! - DACs
//! - I/O devices with multiple logical endpoints, such as USB or CANBUS interfaces
//! - GPIOs (though these are usually modeled using [pins](../pin/index.html), which are very similar).
//! 
//! Channels are always associated with a specific peripheral instance and have a unique name and
//! index within that peripheral. The index generally corresponds to the value used within the peripheral
//! to access registers and fields associated with the channel, and the name of the channel may or may
//! not be the same as the channel index. 

use periph::Periph;
use signal::SignalType;

/// Represents a channel associated with a peripheral.
pub trait Channel<P: Periph> {
    /// Returns the peripheral instance associated with the peripheral.
    fn periph(&self) -> P { P::default() }
    /// Returns the index of the peripheral.
    fn index(&self) -> u8;
}

/// Indicates that a signal of `SignalType` is associated with a channel.
pub trait ChannelSource<ST: SignalType, SRC> {
    /// Returns the selector used to connect a channel source.
    fn selector(&self) -> u8;
}

/// Connect a signal to a peripheral channel.
pub trait ConnectChannel<SRC, STY, PERIPH, CH>
where 
    STY: SignalType,
    PERIPH: Periph,
    CH: Channel<PERIPH> + ChannelSource<STY, SRC>,
{
    /// Connect a channel associated with `signal_type`.
    fn connect_channel(&self, signal_type: STY, channel: CH);
}
