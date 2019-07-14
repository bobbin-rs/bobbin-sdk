#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm)]

pub extern crate stm32_common;
pub use stm32_common::bobbin_bits;
pub use stm32_common::bobbin_mcu;
pub use stm32_common::bobbin_hal;
pub use stm32_common::bobbin_sys;
pub use stm32_common::nvic;
pub use stm32_common::scb;
pub use stm32_common::systick;
pub use stm32_common::mpu;
pub use stm32_common::fpu;
pub use stm32_common::dcb;
pub use stm32_common::itm;

pub mod ext;
pub mod flash;
pub mod pwr;
pub mod rcc;
pub mod syscfg;
pub mod dma;
pub mod crc;
pub mod iwdg;
pub mod wwdg;
pub mod i2c;
pub mod rng;
pub mod adc;
pub mod gpio;
pub mod lptim;
pub mod usart;
pub mod lpuart;
pub mod spi;
pub mod rtc;
pub mod exti;
pub mod dmamux;
pub mod lcd;
pub mod tsc;
pub mod comp;
pub mod quadspi;
pub mod aes1;
pub mod hardware_semaphore;
pub mod sai;
pub mod tim2;
pub mod tim16;
pub mod tim17;
pub mod tim1;
pub mod vref;
pub mod usb;
pub mod crs;
pub mod dbgmcu;
pub mod pka;
pub mod ipcc;
pub mod sig;
pub mod pin;
pub mod clock;


#[derive(Debug, Default)]
pub struct Stm32wb55xx {}

impl ::bobbin_mcu::mcu::Mcu for Stm32wb55xx {
    fn id(&self) -> &'static str { "STM32WB55xx" }
}

pub type Mcu = Stm32wb55xx;

