//! This create defines Hardware Abstraction Layer traits used within the Bobbin
//! project. Typically, MCU crates implement these traits for peripherals,
//! pins, and channels, and higher-level drivers and applications consume these
//! traits.

#![no_std]
#![feature(const_fn)]

pub mod prelude;

pub mod configure;
pub mod enabled;

pub mod analog;
pub mod can;
pub mod crc;
pub mod digital;
pub mod i2c;
pub mod serial;
pub mod spi;
pub mod timer;
pub mod watchdog;
pub mod delay;
pub mod flash;

pub mod led;
pub mod btn;