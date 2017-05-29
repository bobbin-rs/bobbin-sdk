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
pub mod flash;
pub mod pwr;
pub mod rcc;
pub mod exti;
pub mod syscfg;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod rng;
pub mod dma;
pub mod rtc;
pub mod lptim;
pub mod i2c;
pub mod tim_gen;
pub mod gpio;
pub mod usart;
pub mod lpusart;
pub mod spi;
