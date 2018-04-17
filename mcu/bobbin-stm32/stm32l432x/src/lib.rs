#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

extern crate bobbin_bits;
extern crate bobbin_mcu;
extern crate bobbin_hal;
extern crate stm32_common;

pub extern crate bobbin_cortexm;
pub use bobbin_cortexm::nvic;
pub use bobbin_cortexm::scb;
pub use bobbin_cortexm::systick;
pub use bobbin_cortexm::mpu;
pub use bobbin_cortexm::fpu;
pub use bobbin_cortexm::dcb;
pub use bobbin_cortexm::itm;
pub use bobbin_cortexm::dwt;

pub mod ext;
pub mod flash;
pub mod pwr;
pub mod rcc;
pub mod syscfg;
pub mod c_adc;
pub mod dac;
pub mod rng;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod rtc;
pub mod lptim;
pub mod exti;
pub mod dma;
pub mod i2c;
pub mod tim_adv;
pub mod tim_bas;
pub mod tim_gen;
pub mod gpio;
pub mod usart;
pub mod lpuart;
pub mod spi;
pub mod adc;
pub mod sig;
pub mod pin;
pub mod irq;
pub mod clock;


#[derive(Debug, Default)]
pub struct Stm32l432x {}

impl ::bobbin_mcu::mcu::Mcu for Stm32l432x {
    fn id(&self) -> &'static str { "STM32L432x" }
}


