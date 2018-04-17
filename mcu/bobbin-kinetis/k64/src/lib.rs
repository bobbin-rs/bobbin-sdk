#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]
pub extern crate kinetis_common;
pub use kinetis_common::bobbin_bits;
pub use kinetis_common::bobbin_mcu;
pub use kinetis_common::bobbin_hal;
pub use kinetis_common::nvic;
pub use kinetis_common::scb;
pub use kinetis_common::systick;
pub use kinetis_common::mpu;
pub use kinetis_common::fpu;
pub use kinetis_common::dcb;
pub use kinetis_common::itm;

pub mod ext;
pub mod sim;
pub mod mcg;
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


#[derive(Debug, Default)]
pub struct K64 {}

impl ::bobbin_mcu::mcu::Mcu for K64 {
    fn id(&self) -> &'static str { "K64" }
}


