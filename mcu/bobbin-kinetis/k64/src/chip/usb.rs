#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::usb::*;

periph!( USB0, Usb0, _USB0, UsbPeriph, 0x40072000);




pub trait IrqUsb<T> {
    fn irq_usb(&self) -> T;
}

impl IrqUsb<super::irq::IrqUsb0> for Usb0 {
    fn irq_usb(&self) -> super::irq::IrqUsb0 { super::irq::IRQ_USB0 }
}

