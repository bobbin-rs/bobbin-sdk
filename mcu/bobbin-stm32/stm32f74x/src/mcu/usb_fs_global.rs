#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::usb_fs_global::*;

periph!( USB_FS_GLOBAL, UsbFsGlobal, USB_FS_GLOBAL_PERIPH, UsbFsGlobalPeriph, 0x50000000, 0x00, 0x14);

