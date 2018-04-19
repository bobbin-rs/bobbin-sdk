#![no_std]
pub extern crate bobbin_cortexm;
pub use bobbin_cortexm::bobbin_bits;
pub use bobbin_cortexm::bobbin_mcu;
pub use bobbin_cortexm::bobbin_hal;
pub use bobbin_cortexm::bobbin_sys;
pub use bobbin_cortexm::nvic;
pub use bobbin_cortexm::scb;
pub use bobbin_cortexm::systick;
pub use bobbin_cortexm::mpu;
pub use bobbin_cortexm::fpu;
pub use bobbin_cortexm::dcb;
pub use bobbin_cortexm::itm;

pub mod ext;
pub mod exti;
pub mod dma;
pub mod dma_f3;
pub mod gpio;
pub mod lpuart;
pub mod usart;
pub mod usart_f24;
pub mod lptim;
pub mod tim_bas;
pub mod tim_gen;
pub mod tim_adv;
pub mod crc;
pub mod crc_24;
pub mod rng;
pub mod rtc;
pub mod iwdg;
pub mod wwdg;
pub mod adc_f1;
pub mod adc_f24;
pub mod adc_f3;
pub mod adc_l0;
pub mod i2c_v1;
pub mod i2c_v2;
pub mod spi_v1;
pub mod spi_v2;
pub mod dcmi;
pub mod dac;
pub mod sig;
pub mod pin;

pub const STM32: Stm32 = Stm32 {};
pub struct Stm32 {}

impl Stm32 {
    pub fn is_stm32() -> bool { true }
}

impl ::core::ops::Deref for Stm32 {
    type Target = bobbin_cortexm::Cortexm;
    fn deref(&self) -> &Self::Target {
        &bobbin_cortexm::CORTEXM
    }
}
