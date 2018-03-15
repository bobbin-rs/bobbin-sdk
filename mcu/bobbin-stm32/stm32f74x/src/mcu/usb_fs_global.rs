#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::usb_fs_global::*;

periph!( USB_FS_GLOBAL, UsbFsGlobal, USB_FS_GLOBAL_PERIPH, UsbFsGlobalPeriph, 0x50000000, 0x14);

