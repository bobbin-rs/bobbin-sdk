#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::uart::*;

pub const UART0: Uart0 = Periph(0x4006a000, Uart0Id {});
pub const UART1: Uart1 = Periph(0x4006b000, Uart1Id {});
pub const UART2: Uart2 = Periph(0x4006c000, Uart2Id {});
pub const UART3: Uart3 = Periph(0x4006d000, Uart3Id {});
pub const UART4: Uart4 = Periph(0x400ea000, Uart4Id {});
pub const UART5: Uart5 = Periph(0x400eb000, Uart5Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart0Id {}
pub type Uart0 = Periph<Uart0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart1Id {}
pub type Uart1 = Periph<Uart1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart2Id {}
pub type Uart2 = Periph<Uart2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart3Id {}
pub type Uart3 = Periph<Uart3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart4Id {}
pub type Uart4 = Periph<Uart4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart5Id {}
pub type Uart5 = Periph<Uart5Id>;

impl super::sig::Signal<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::SignalTx<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::Signal<super::sig::Uart0Rx> for Uart0 {}
impl super::sig::SignalRx<super::sig::Uart0Rx> for Uart0 {}

impl super::sig::Signal<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::SignalTx<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::Signal<super::sig::Uart1Rx> for Uart1 {}
impl super::sig::SignalRx<super::sig::Uart1Rx> for Uart1 {}

impl super::sig::Signal<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::SignalTx<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::Signal<super::sig::Uart2Rx> for Uart2 {}
impl super::sig::SignalRx<super::sig::Uart2Rx> for Uart2 {}

impl super::sig::Signal<super::sig::Uart3Tx> for Uart3 {}
impl super::sig::SignalTx<super::sig::Uart3Tx> for Uart3 {}
impl super::sig::Signal<super::sig::Uart3Rx> for Uart3 {}
impl super::sig::SignalRx<super::sig::Uart3Rx> for Uart3 {}

impl super::sig::Signal<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::Uart4Rx> for Uart4 {}

impl super::sig::Signal<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::Uart5Rx> for Uart5 {}


pub trait IrqUartLon<T> {
   fn irq_uart_lon(&self) -> super::irq::Irq<T>;
}

pub trait RegisterUartLonHandler {
   fn register_uart_lon_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartLon>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleUartLon {
   fn handle_uart_lon(&self);
}

pub trait IrqUartRxTx<T> {
   fn irq_uart_rx_tx(&self) -> super::irq::Irq<T>;
}

pub trait RegisterUartRxTxHandler {
   fn register_uart_rx_tx_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartRxTx>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleUartRxTx {
   fn handle_uart_rx_tx(&self);
}

pub trait IrqUartErr<T> {
   fn irq_uart_err(&self) -> super::irq::Irq<T>;
}

pub trait RegisterUartErrHandler {
   fn register_uart_err_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartErr>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleUartErr {
   fn handle_uart_err(&self);
}

impl IrqUartLon<super::irq::Uart0LonId> for Uart0 {
   fn irq_uart_lon(&self) -> super::irq::IrqUart0Lon { super::irq::IRQ_UART0_LON }
}

impl RegisterUartLonHandler for Uart0 {
   fn register_uart_lon_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartLon>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartLon>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_lon() }
       }
       super::irq::set_handler(30, Some(wrapper::<F>));
       super::irq::IrqGuard::new(30)
   }
}

impl IrqUartRxTx<super::irq::Uart0RxTxId> for Uart0 {
   fn irq_uart_rx_tx(&self) -> super::irq::IrqUart0RxTx { super::irq::IRQ_UART0_RX_TX }
}

impl RegisterUartRxTxHandler for Uart0 {
   fn register_uart_rx_tx_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartRxTx>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartRxTx>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_rx_tx() }
       }
       super::irq::set_handler(31, Some(wrapper::<F>));
       super::irq::IrqGuard::new(31)
   }
}

impl IrqUartErr<super::irq::Uart0ErrId> for Uart0 {
   fn irq_uart_err(&self) -> super::irq::IrqUart0Err { super::irq::IRQ_UART0_ERR }
}

impl RegisterUartErrHandler for Uart0 {
   fn register_uart_err_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartErr>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartErr>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_err() }
       }
       super::irq::set_handler(32, Some(wrapper::<F>));
       super::irq::IrqGuard::new(32)
   }
}

impl IrqUartRxTx<super::irq::Uart1RxTxId> for Uart1 {
   fn irq_uart_rx_tx(&self) -> super::irq::IrqUart1RxTx { super::irq::IRQ_UART1_RX_TX }
}

impl RegisterUartRxTxHandler for Uart1 {
   fn register_uart_rx_tx_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartRxTx>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartRxTx>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_rx_tx() }
       }
       super::irq::set_handler(33, Some(wrapper::<F>));
       super::irq::IrqGuard::new(33)
   }
}

impl IrqUartErr<super::irq::Uart1ErrId> for Uart1 {
   fn irq_uart_err(&self) -> super::irq::IrqUart1Err { super::irq::IRQ_UART1_ERR }
}

impl RegisterUartErrHandler for Uart1 {
   fn register_uart_err_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartErr>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartErr>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_err() }
       }
       super::irq::set_handler(34, Some(wrapper::<F>));
       super::irq::IrqGuard::new(34)
   }
}

impl IrqUartRxTx<super::irq::Uart2RxTxId> for Uart2 {
   fn irq_uart_rx_tx(&self) -> super::irq::IrqUart2RxTx { super::irq::IRQ_UART2_RX_TX }
}

impl RegisterUartRxTxHandler for Uart2 {
   fn register_uart_rx_tx_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartRxTx>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartRxTx>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_rx_tx() }
       }
       super::irq::set_handler(35, Some(wrapper::<F>));
       super::irq::IrqGuard::new(35)
   }
}

impl IrqUartErr<super::irq::Uart2ErrId> for Uart2 {
   fn irq_uart_err(&self) -> super::irq::IrqUart2Err { super::irq::IRQ_UART2_ERR }
}

impl RegisterUartErrHandler for Uart2 {
   fn register_uart_err_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartErr>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartErr>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_err() }
       }
       super::irq::set_handler(36, Some(wrapper::<F>));
       super::irq::IrqGuard::new(36)
   }
}

impl IrqUartRxTx<super::irq::Uart3RxTxId> for Uart3 {
   fn irq_uart_rx_tx(&self) -> super::irq::IrqUart3RxTx { super::irq::IRQ_UART3_RX_TX }
}

impl RegisterUartRxTxHandler for Uart3 {
   fn register_uart_rx_tx_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartRxTx>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartRxTx>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_rx_tx() }
       }
       super::irq::set_handler(37, Some(wrapper::<F>));
       super::irq::IrqGuard::new(37)
   }
}

impl IrqUartErr<super::irq::Uart3ErrId> for Uart3 {
   fn irq_uart_err(&self) -> super::irq::IrqUart3Err { super::irq::IRQ_UART3_ERR }
}

impl RegisterUartErrHandler for Uart3 {
   fn register_uart_err_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartErr>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartErr>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_err() }
       }
       super::irq::set_handler(38, Some(wrapper::<F>));
       super::irq::IrqGuard::new(38)
   }
}

impl IrqUartRxTx<super::irq::Uart4RxTxId> for Uart4 {
   fn irq_uart_rx_tx(&self) -> super::irq::IrqUart4RxTx { super::irq::IRQ_UART4_RX_TX }
}

impl RegisterUartRxTxHandler for Uart4 {
   fn register_uart_rx_tx_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartRxTx>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartRxTx>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_rx_tx() }
       }
       super::irq::set_handler(66, Some(wrapper::<F>));
       super::irq::IrqGuard::new(66)
   }
}

impl IrqUartErr<super::irq::Uart4ErrId> for Uart4 {
   fn irq_uart_err(&self) -> super::irq::IrqUart4Err { super::irq::IRQ_UART4_ERR }
}

impl RegisterUartErrHandler for Uart4 {
   fn register_uart_err_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartErr>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartErr>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_err() }
       }
       super::irq::set_handler(67, Some(wrapper::<F>));
       super::irq::IrqGuard::new(67)
   }
}

impl IrqUartRxTx<super::irq::Uart5RxTxId> for Uart5 {
   fn irq_uart_rx_tx(&self) -> super::irq::IrqUart5RxTx { super::irq::IRQ_UART5_RX_TX }
}

impl RegisterUartRxTxHandler for Uart5 {
   fn register_uart_rx_tx_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartRxTx>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartRxTx>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_rx_tx() }
       }
       super::irq::set_handler(68, Some(wrapper::<F>));
       super::irq::IrqGuard::new(68)
   }
}

impl IrqUartErr<super::irq::Uart5ErrId> for Uart5 {
   fn irq_uart_err(&self) -> super::irq::IrqUart5Err { super::irq::IRQ_UART5_ERR }
}

impl RegisterUartErrHandler for Uart5 {
   fn register_uart_err_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleUartErr>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleUartErr>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_uart_err() }
       }
       super::irq::set_handler(69, Some(wrapper::<F>));
       super::irq::IrqGuard::new(69)
   }
}

