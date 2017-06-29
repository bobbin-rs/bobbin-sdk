pub use stm32_common::chip::usart::*;

pub const USART2: Usart2 = Periph(0x40004400, Usart2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Usart2Id {}
pub type Usart2 = Periph<Usart2Id>;

impl super::sig::Signal<super::sig::Usart2Tx> for Usart2 {}
impl super::sig::SignalTx<super::sig::Usart2Tx> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Rx> for Usart2 {}
impl super::sig::SignalRx<super::sig::Usart2Rx> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Cts> for Usart2 {}
impl super::sig::SignalCts<super::sig::Usart2Cts> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Rts> for Usart2 {}
impl super::sig::SignalRts<super::sig::Usart2Rts> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Ck> for Usart2 {}
impl super::sig::SignalCk<super::sig::Usart2Ck> for Usart2 {}


pub trait IrqUsart<T> {
   fn irq_usart(&self) -> super::irq::Irq<T>;
}

pub trait RegisterUsartHandler {
   fn register_usart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUsart>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleUsart {
   fn handle_usart(&self);
}

impl IrqUsart<super::irq::Usart2Id> for Usart2 {
   fn irq_usart(&self) -> super::irq::IrqUsart2 { super::irq::IRQ_USART2 }
}

impl RegisterUsartHandler for Usart2 {
   fn register_usart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUsart>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUsart>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_usart() }
       }
       super::irq::set_handler(28, Some(wrapper::<F>));
       super::irq::IrqGuard::new(28)
   }
}

