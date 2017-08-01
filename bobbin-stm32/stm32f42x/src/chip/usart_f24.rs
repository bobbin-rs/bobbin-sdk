pub use stm32_common::chip::usart_f24::*;

pub const USART1: Usart1 = Periph(0x40011000, Usart1Id {});
pub const USART2: Usart2 = Periph(0x40004400, Usart2Id {});
pub const USART3: Usart3 = Periph(0x40004800, Usart3Id {});
pub const UART4: Uart4 = Periph(0x40004c00, Uart4Id {});
pub const UART5: Uart5 = Periph(0x40005000, Uart5Id {});
pub const USART6: Usart6 = Periph(0x40011400, Usart6Id {});
pub const UART7: Uart7 = Periph(0x40007800, Uart7Id {});
pub const UART8: Uart8 = Periph(0x40007c00, Uart8Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Usart1Id {}
pub type Usart1 = Periph<Usart1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Usart2Id {}
pub type Usart2 = Periph<Usart2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Usart3Id {}
pub type Usart3 = Periph<Usart3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart4Id {}
pub type Uart4 = Periph<Uart4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart5Id {}
pub type Uart5 = Periph<Uart5Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Usart6Id {}
pub type Usart6 = Periph<Usart6Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart7Id {}
pub type Uart7 = Periph<Uart7Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart8Id {}
pub type Uart8 = Periph<Uart8Id>;

impl super::sig::Signal<super::sig::Usart1Tx> for Usart1 {}
impl super::sig::SignalTx<super::sig::Usart1Tx> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Rx> for Usart1 {}
impl super::sig::SignalRx<super::sig::Usart1Rx> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Cts> for Usart1 {}
impl super::sig::SignalCts<super::sig::Usart1Cts> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Rts> for Usart1 {}
impl super::sig::SignalRts<super::sig::Usart1Rts> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Ck> for Usart1 {}
impl super::sig::SignalCk<super::sig::Usart1Ck> for Usart1 {}

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

impl super::sig::Signal<super::sig::Usart3Tx> for Usart3 {}
impl super::sig::SignalTx<super::sig::Usart3Tx> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Rx> for Usart3 {}
impl super::sig::SignalRx<super::sig::Usart3Rx> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Cts> for Usart3 {}
impl super::sig::SignalCts<super::sig::Usart3Cts> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Rts> for Usart3 {}
impl super::sig::SignalRts<super::sig::Usart3Rts> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Ck> for Usart3 {}
impl super::sig::SignalCk<super::sig::Usart3Ck> for Usart3 {}

impl super::sig::Signal<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::Uart4Rx> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Cts> for Uart4 {}
impl super::sig::SignalCts<super::sig::Uart4Cts> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Rts> for Uart4 {}
impl super::sig::SignalRts<super::sig::Uart4Rts> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Ck> for Uart4 {}
impl super::sig::SignalCk<super::sig::Uart4Ck> for Uart4 {}

impl super::sig::Signal<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::Uart5Rx> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Cts> for Uart5 {}
impl super::sig::SignalCts<super::sig::Uart5Cts> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Rts> for Uart5 {}
impl super::sig::SignalRts<super::sig::Uart5Rts> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Ck> for Uart5 {}
impl super::sig::SignalCk<super::sig::Uart5Ck> for Uart5 {}

impl super::sig::Signal<super::sig::Usart6Tx> for Usart6 {}
impl super::sig::SignalTx<super::sig::Usart6Tx> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Rx> for Usart6 {}
impl super::sig::SignalRx<super::sig::Usart6Rx> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Cts> for Usart6 {}
impl super::sig::SignalCts<super::sig::Usart6Cts> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Rts> for Usart6 {}
impl super::sig::SignalRts<super::sig::Usart6Rts> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Ck> for Usart6 {}
impl super::sig::SignalCk<super::sig::Usart6Ck> for Usart6 {}

impl super::sig::Signal<super::sig::Uart7Tx> for Uart7 {}
impl super::sig::SignalTx<super::sig::Uart7Tx> for Uart7 {}
impl super::sig::Signal<super::sig::Uart7Rx> for Uart7 {}
impl super::sig::SignalRx<super::sig::Uart7Rx> for Uart7 {}
impl super::sig::Signal<super::sig::Uart7Cts> for Uart7 {}
impl super::sig::SignalCts<super::sig::Uart7Cts> for Uart7 {}
impl super::sig::Signal<super::sig::Uart7Rts> for Uart7 {}
impl super::sig::SignalRts<super::sig::Uart7Rts> for Uart7 {}
impl super::sig::Signal<super::sig::Uart7Ck> for Uart7 {}
impl super::sig::SignalCk<super::sig::Uart7Ck> for Uart7 {}

impl super::sig::Signal<super::sig::Uart8Tx> for Uart8 {}
impl super::sig::SignalTx<super::sig::Uart8Tx> for Uart8 {}
impl super::sig::Signal<super::sig::Uart8Rx> for Uart8 {}
impl super::sig::SignalRx<super::sig::Uart8Rx> for Uart8 {}
impl super::sig::Signal<super::sig::Uart8Cts> for Uart8 {}
impl super::sig::SignalCts<super::sig::Uart8Cts> for Uart8 {}
impl super::sig::Signal<super::sig::Uart8Rts> for Uart8 {}
impl super::sig::SignalRts<super::sig::Uart8Rts> for Uart8 {}
impl super::sig::Signal<super::sig::Uart8Ck> for Uart8 {}
impl super::sig::SignalCk<super::sig::Uart8Ck> for Uart8 {}


pub trait IrqUsart<T> {
   fn irq_usart(&self) -> super::irq::Irq<T>;
}

pub trait RegisterUsartHandler {
   fn register_usart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUsart>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleUsart {
   fn handle_usart(&self);
}

impl IrqUsart<super::irq::Usart1Id> for Usart1 {
   fn irq_usart(&self) -> super::irq::IrqUsart1 { super::irq::IRQ_USART1 }
}

impl RegisterUsartHandler for Usart1 {
   fn register_usart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUsart>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUsart>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_usart() }
       }
       super::irq::set_handler(37, Some(wrapper::<F>));
       super::irq::IrqGuard::new(37)
   }
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
       super::irq::set_handler(38, Some(wrapper::<F>));
       super::irq::IrqGuard::new(38)
   }
}

impl IrqUsart<super::irq::Usart3Id> for Usart3 {
   fn irq_usart(&self) -> super::irq::IrqUsart3 { super::irq::IRQ_USART3 }
}

impl RegisterUsartHandler for Usart3 {
   fn register_usart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUsart>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUsart>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_usart() }
       }
       super::irq::set_handler(39, Some(wrapper::<F>));
       super::irq::IrqGuard::new(39)
   }
}

impl IrqUsart<super::irq::Uart4Id> for Uart4 {
   fn irq_usart(&self) -> super::irq::IrqUart4 { super::irq::IRQ_UART4 }
}

impl RegisterUsartHandler for Uart4 {
   fn register_usart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUsart>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUsart>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_usart() }
       }
       super::irq::set_handler(52, Some(wrapper::<F>));
       super::irq::IrqGuard::new(52)
   }
}

impl IrqUsart<super::irq::Uart5Id> for Uart5 {
   fn irq_usart(&self) -> super::irq::IrqUart5 { super::irq::IRQ_UART5 }
}

impl RegisterUsartHandler for Uart5 {
   fn register_usart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUsart>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUsart>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_usart() }
       }
       super::irq::set_handler(53, Some(wrapper::<F>));
       super::irq::IrqGuard::new(53)
   }
}

impl IrqUsart<super::irq::Usart6Id> for Usart6 {
   fn irq_usart(&self) -> super::irq::IrqUsart6 { super::irq::IRQ_USART6 }
}

impl RegisterUsartHandler for Usart6 {
   fn register_usart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUsart>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUsart>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_usart() }
       }
       super::irq::set_handler(71, Some(wrapper::<F>));
       super::irq::IrqGuard::new(71)
   }
}

impl IrqUsart<super::irq::Uart7Id> for Uart7 {
   fn irq_usart(&self) -> super::irq::IrqUart7 { super::irq::IRQ_UART7 }
}

impl RegisterUsartHandler for Uart7 {
   fn register_usart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUsart>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUsart>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_usart() }
       }
       super::irq::set_handler(82, Some(wrapper::<F>));
       super::irq::IrqGuard::new(82)
   }
}

impl IrqUsart<super::irq::Uart8Id> for Uart8 {
   fn irq_usart(&self) -> super::irq::IrqUart8 { super::irq::IRQ_UART8 }
}

impl RegisterUsartHandler for Uart8 {
   fn register_usart_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUsart>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUsart>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_usart() }
       }
       super::irq::set_handler(83, Some(wrapper::<F>));
       super::irq::IrqGuard::new(83)
   }
}

