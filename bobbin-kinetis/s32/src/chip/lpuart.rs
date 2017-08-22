#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::lpuart::*;

pub const LPUART0: Lpuart0 = Periph(0x4006a000, Lpuart0Id {});
pub const LPUART1: Lpuart1 = Periph(0x4006b000, Lpuart1Id {});
pub const LPUART2: Lpuart2 = Periph(0x4006c000, Lpuart2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpuart0Id {}
pub type Lpuart0 = Periph<Lpuart0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpuart1Id {}
pub type Lpuart1 = Periph<Lpuart1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpuart2Id {}
pub type Lpuart2 = Periph<Lpuart2Id>;

impl super::sig::Signal<super::sig::Lpuart0Tx> for Lpuart0 {}
impl super::sig::SignalTx<super::sig::Lpuart0Tx> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Rx> for Lpuart0 {}
impl super::sig::SignalRx<super::sig::Lpuart0Rx> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Cts> for Lpuart0 {}
impl super::sig::SignalCts<super::sig::Lpuart0Cts> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Rts> for Lpuart0 {}
impl super::sig::SignalRts<super::sig::Lpuart0Rts> for Lpuart0 {}

impl super::sig::Signal<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::SignalTx<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::SignalRx<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::SignalCts<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rts> for Lpuart1 {}
impl super::sig::SignalRts<super::sig::Lpuart1Rts> for Lpuart1 {}

impl super::sig::Signal<super::sig::Lpuart2Tx> for Lpuart2 {}
impl super::sig::SignalTx<super::sig::Lpuart2Tx> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Rx> for Lpuart2 {}
impl super::sig::SignalRx<super::sig::Lpuart2Rx> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Cts> for Lpuart2 {}
impl super::sig::SignalCts<super::sig::Lpuart2Cts> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Rts> for Lpuart2 {}
impl super::sig::SignalRts<super::sig::Lpuart2Rts> for Lpuart2 {}


pub trait IrqLpuartRxtx<T> {
   fn irq_lpuart_rxtx(&self) -> super::irq::Irq<T>;
}

pub trait RegisterLpuartRxtxHandler {
   fn register_lpuart_rxtx_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpuartRxtx>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleLpuartRxtx {
   fn handle_lpuart_rxtx(&self);
}

impl IrqLpuartRxtx<super::irq::Lpuart0RxtxId> for Lpuart0 {
   fn irq_lpuart_rxtx(&self) -> super::irq::IrqLpuart0Rxtx { super::irq::IRQ_LPUART0_RXTX }
}

impl RegisterLpuartRxtxHandler for Lpuart0 {
   fn register_lpuart_rxtx_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpuartRxtx>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLpuartRxtx>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lpuart_rxtx() }
       }
       super::irq::set_handler(31, Some(wrapper::<F>));
       super::irq::IrqGuard::new(31)
   }
}

impl IrqLpuartRxtx<super::irq::Lpuart1RxtxId> for Lpuart1 {
   fn irq_lpuart_rxtx(&self) -> super::irq::IrqLpuart1Rxtx { super::irq::IRQ_LPUART1_RXTX }
}

impl RegisterLpuartRxtxHandler for Lpuart1 {
   fn register_lpuart_rxtx_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpuartRxtx>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLpuartRxtx>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lpuart_rxtx() }
       }
       super::irq::set_handler(33, Some(wrapper::<F>));
       super::irq::IrqGuard::new(33)
   }
}

impl IrqLpuartRxtx<super::irq::Lpuart2RxtxId> for Lpuart2 {
   fn irq_lpuart_rxtx(&self) -> super::irq::IrqLpuart2Rxtx { super::irq::IRQ_LPUART2_RXTX }
}

impl RegisterLpuartRxtxHandler for Lpuart2 {
   fn register_lpuart_rxtx_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleLpuartRxtx>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleLpuartRxtx>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_lpuart_rxtx() }
       }
       super::irq::set_handler(35, Some(wrapper::<F>));
       super::irq::IrqGuard::new(35)
   }
}

