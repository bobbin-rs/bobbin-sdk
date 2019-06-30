#![no_std]
#![feature(asm, naked_functions, linkage, core_intrinsics, global_asm)]

pub extern crate sam_common;
pub use sam_common::bobbin_bits;
pub use sam_common::bobbin_mcu;
pub use sam_common::bobbin_hal;
pub use sam_common::bobbin_sys;
pub use sam_common::nvic;
pub use sam_common::scb;
pub use sam_common::systick;

pub mod ext;
pub mod mclk;
pub mod ac;
pub mod adc;
pub mod aes;
pub mod ccl;
pub mod cmcc;
pub mod dac;
pub mod dmac;
pub mod dsu;
pub mod eic;
pub mod evsys;
pub mod freqm;
pub mod gclk;
pub mod adc1;
pub mod icm;
pub mod i2s;
pub mod nvmctrl;
pub mod oscctrl;
pub mod osc32kctrl;
pub mod pac;
pub mod pcc;
pub mod pdec;
pub mod pm;
pub mod port;
pub mod qspi;
pub mod ramecc;
pub mod rstc;
pub mod rtc;
pub mod sdhc;
pub mod sercom;
pub mod supc;
pub mod tc;
pub mod tcc;
pub mod trng;
pub mod usb;
pub mod wdt;
pub mod sig;
pub mod pin;

