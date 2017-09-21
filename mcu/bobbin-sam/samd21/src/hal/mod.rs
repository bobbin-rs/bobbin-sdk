pub use bobbin_cortexm::hal::{nvic, scb, systick};

pub mod clock;
pub mod port;
pub mod gclk;
pub mod pm;
pub mod tc;
pub mod tcc;
pub mod sercom;
pub mod dmac;
pub mod wdt;
pub mod adc;
pub mod i2c;
pub mod spi;

// // pub mod port;
// // pub mod usart;
// // // pub mod spi;

// // pub mod tc;
// // pub mod tcc;
// // pub mod adc;