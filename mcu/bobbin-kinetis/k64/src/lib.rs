#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate bobbin_bits;
extern crate bobbin_mcu;
extern crate bobbin_hal;
extern crate kinetis_common;


pub mod ext;
pub mod sim;
pub mod mcg;
pub mod mpu;
pub mod osc;
pub mod rcm;
pub mod enet;
pub mod crc;
pub mod wdog;
pub mod ftfe;
pub mod dmamux;
pub mod edma;
pub mod ftm;
pub mod pit;
pub mod lptmr;
pub mod spi;
pub mod i2c;
pub mod uart;
pub mod usb;
pub mod flexcan;
pub mod dac;
pub mod gpio;
pub mod port;
pub mod adc;
pub mod sig;
pub mod pin;
pub mod irq;

pub mod clock;

