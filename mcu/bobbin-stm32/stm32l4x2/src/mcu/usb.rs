#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::usb::*;

periph!( USB_SRAM, UsbSram, USB_SRAM_PERIPH, UsbPeriph, 0x40006c00, 0x37);
periph!( USB_FS, UsbFs, USB_FS_PERIPH, UsbPeriph, 0x40006800, 0x38);

