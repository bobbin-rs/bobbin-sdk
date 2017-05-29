#![no_std]
extern crate bobbin_cortexm;
pub use bobbin_cortexm::chip::exc;
pub use bobbin_cortexm::chip::nvic;
pub use bobbin_cortexm::chip::scb;
pub use bobbin_cortexm::chip::systick;
pub use bobbin_cortexm::chip::mpu;
pub use bobbin_cortexm::chip::fpu;

extern crate stm32_common;

pub mod irq;
pub mod rcc;
pub mod syscfg;
pub mod flash;
pub mod pwr;
pub mod exti;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod rtc;
pub mod gpio;
pub mod usart;
pub mod i2c;
pub mod spi;
pub mod adc;
pub mod c_adc;
pub mod dac;
pub mod tim_bas;
pub mod tim_gen;
pub mod tim_adv;
pub mod dma;
