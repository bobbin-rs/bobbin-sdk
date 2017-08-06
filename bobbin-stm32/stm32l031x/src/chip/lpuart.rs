#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::lpuart::*;

pub const LPUART1: Lpuart1 = Periph(0x40004800, Lpuart1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpuart1Id {}
pub type Lpuart1 = Periph<Lpuart1Id>;

impl super::sig::Signal<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::SignalTx<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::SignalRx<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::SignalCts<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rts> for Lpuart1 {}
impl super::sig::SignalRts<super::sig::Lpuart1Rts> for Lpuart1 {}


pub trait IrqLpuart<T> {
   fn irq_lpuart(&self) -> super::irq::Irq<T>;
}

pub trait RegisterLpuartHandler {
   fn register_lpuart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpuart>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleLpuart {
   fn handle_lpuart(&self);
}

impl IrqLpuart<super::irq::Lpuart1Id> for Lpuart1 {
   fn irq_lpuart(&self) -> super::irq::IrqLpuart1 { super::irq::IRQ_LPUART1 }
}

impl RegisterLpuartHandler for Lpuart1 {
   fn register_lpuart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpuart>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLpuart>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lpuart() }
       }
       super::irq::set_handler(29, Some(wrapper::<F>));
       super::irq::IrqGuard::new(29)
   }
}

