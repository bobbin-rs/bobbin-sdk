#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::usb_fs_pwrclk::*;

periph!( USB_FS_PWRCLK, UsbFsPwrclk, USB_FS_PWRCLK_PERIPH, UsbFsPwrclkPeriph, USB_FS_PWRCLK_OWNED, 0x50000e00, 0x00, 0x17);

