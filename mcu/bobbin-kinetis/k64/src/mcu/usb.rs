#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::usb::*;

periph!( USB0, Usb0, USB0_PERIPH, UsbPeriph, 0x40072000, 0x1a);

