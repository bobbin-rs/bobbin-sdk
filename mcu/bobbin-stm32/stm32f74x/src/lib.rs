#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm, used, use_extern_macros)]

pub extern crate stm32_common;
pub use stm32_common::bobbin_bits;
pub use stm32_common::bobbin_mcu;
pub use stm32_common::bobbin_hal;
pub use stm32_common::nvic;
pub use stm32_common::scb;
pub use stm32_common::systick;
pub use stm32_common::mpu;
pub use stm32_common::fpu;
pub use stm32_common::dcb;
pub use stm32_common::itm;

pub mod ext;
pub mod rcc;
pub mod flash;
pub mod pwr;
pub mod syscfg;
pub mod dbg;
pub mod ethernet_mac;
pub mod ethernet_mmc;
pub mod ethernet_ptp;
pub mod ethernet_dma;
pub mod sdmmc;
pub mod quadspi;
pub mod cec;
pub mod spdif_rx;
pub mod ltdc;
pub mod dma2d;
pub mod hash;
pub mod cryp;
pub mod c_adc;
pub mod dac;
pub mod dcmi;
pub mod usb_fs_global;
pub mod usb_fs_host;
pub mod usb_fs_device;
pub mod usb_fs_pwrclk;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod exti;
pub mod tim_bas;
pub mod tim_gen;
pub mod tim_adv;
pub mod lptim;
pub mod adc;
pub mod spi;
pub mod i2c;
pub mod can;
pub mod gpio;
pub mod usart;
pub mod dma;
pub mod sig;
pub mod pin;
pub mod irq;
pub mod clock;


#[derive(Debug, Default)]
pub struct Stm32f74x {}

impl ::bobbin_mcu::mcu::Mcu for Stm32f74x {
    fn id(&self) -> &'static str { "STM32F74x" }
}


