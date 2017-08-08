//! Interrupts

use ::core::marker::PhantomData;
pub type Handler = extern "C" fn();

pub const IRQ_WDOG: IrqWdog = Irq(22, WdogId {});
pub const IRQ_DMA_ERROR: IrqDmaError = Irq(16, DmaErrorId {});
pub const IRQ_DMA0: IrqDma0 = Irq(0, Dma0Id {});
pub const IRQ_DMA1: IrqDma1 = Irq(1, Dma1Id {});
pub const IRQ_DMA2: IrqDma2 = Irq(2, Dma2Id {});
pub const IRQ_DMA3: IrqDma3 = Irq(3, Dma3Id {});
pub const IRQ_DMA4: IrqDma4 = Irq(4, Dma4Id {});
pub const IRQ_DMA5: IrqDma5 = Irq(5, Dma5Id {});
pub const IRQ_DMA6: IrqDma6 = Irq(6, Dma6Id {});
pub const IRQ_DMA7: IrqDma7 = Irq(7, Dma7Id {});
pub const IRQ_DMA8: IrqDma8 = Irq(8, Dma8Id {});
pub const IRQ_DMA9: IrqDma9 = Irq(9, Dma9Id {});
pub const IRQ_DMA10: IrqDma10 = Irq(10, Dma10Id {});
pub const IRQ_DMA11: IrqDma11 = Irq(11, Dma11Id {});
pub const IRQ_DMA12: IrqDma12 = Irq(12, Dma12Id {});
pub const IRQ_DMA13: IrqDma13 = Irq(13, Dma13Id {});
pub const IRQ_DMA14: IrqDma14 = Irq(14, Dma14Id {});
pub const IRQ_DMA15: IrqDma15 = Irq(15, Dma15Id {});
pub const IRQ_FTM0: IrqFtm0 = Irq(42, Ftm0Id {});
pub const IRQ_FTM1: IrqFtm1 = Irq(43, Ftm1Id {});
pub const IRQ_FTM2: IrqFtm2 = Irq(44, Ftm2Id {});
pub const IRQ_PIT0: IrqPit0 = Irq(48, Pit0Id {});
pub const IRQ_PIT1: IrqPit1 = Irq(49, Pit1Id {});
pub const IRQ_PIT2: IrqPit2 = Irq(50, Pit2Id {});
pub const IRQ_PIT3: IrqPit3 = Irq(51, Pit3Id {});
pub const IRQ_LPTMR0: IrqLptmr0 = Irq(58, Lptmr0Id {});
pub const IRQ_SPI0: IrqSpi0 = Irq(26, Spi0Id {});
pub const IRQ_SPI1: IrqSpi1 = Irq(27, Spi1Id {});
pub const IRQ_SPI2: IrqSpi2 = Irq(65, Spi2Id {});
pub const IRQ_I2C0: IrqI2c0 = Irq(24, I2c0Id {});
pub const IRQ_I2C1: IrqI2c1 = Irq(25, I2c1Id {});
pub const IRQ_UART0_LON: IrqUart0Lon = Irq(30, Uart0LonId {});
pub const IRQ_UART0_RX_TX: IrqUart0RxTx = Irq(31, Uart0RxTxId {});
pub const IRQ_UART0_ERR: IrqUart0Err = Irq(32, Uart0ErrId {});
pub const IRQ_UART1_RX_TX: IrqUart1RxTx = Irq(33, Uart1RxTxId {});
pub const IRQ_UART1_ERR: IrqUart1Err = Irq(34, Uart1ErrId {});
pub const IRQ_UART2_RX_TX: IrqUart2RxTx = Irq(35, Uart2RxTxId {});
pub const IRQ_UART2_ERR: IrqUart2Err = Irq(36, Uart2ErrId {});
pub const IRQ_UART3_RX_TX: IrqUart3RxTx = Irq(37, Uart3RxTxId {});
pub const IRQ_UART3_ERR: IrqUart3Err = Irq(38, Uart3ErrId {});
pub const IRQ_UART4_RX_TX: IrqUart4RxTx = Irq(66, Uart4RxTxId {});
pub const IRQ_UART4_ERR: IrqUart4Err = Irq(67, Uart4ErrId {});
pub const IRQ_UART5_RX_TX: IrqUart5RxTx = Irq(68, Uart5RxTxId {});
pub const IRQ_UART5_ERR: IrqUart5Err = Irq(69, Uart5ErrId {});
pub const IRQ_PORTA: IrqPorta = Irq(59, PortaId {});
pub const IRQ_PORTB: IrqPortb = Irq(60, PortbId {});
pub const IRQ_PORTC: IrqPortc = Irq(61, PortcId {});
pub const IRQ_PORTD: IrqPortd = Irq(62, PortdId {});
pub const IRQ_PORTE: IrqPorte = Irq(63, PorteId {});

pub type IrqWdog = Irq<WdogId>;
pub type IrqDmaError = Irq<DmaErrorId>;
pub type IrqDma0 = Irq<Dma0Id>;
pub type IrqDma1 = Irq<Dma1Id>;
pub type IrqDma2 = Irq<Dma2Id>;
pub type IrqDma3 = Irq<Dma3Id>;
pub type IrqDma4 = Irq<Dma4Id>;
pub type IrqDma5 = Irq<Dma5Id>;
pub type IrqDma6 = Irq<Dma6Id>;
pub type IrqDma7 = Irq<Dma7Id>;
pub type IrqDma8 = Irq<Dma8Id>;
pub type IrqDma9 = Irq<Dma9Id>;
pub type IrqDma10 = Irq<Dma10Id>;
pub type IrqDma11 = Irq<Dma11Id>;
pub type IrqDma12 = Irq<Dma12Id>;
pub type IrqDma13 = Irq<Dma13Id>;
pub type IrqDma14 = Irq<Dma14Id>;
pub type IrqDma15 = Irq<Dma15Id>;
pub type IrqFtm0 = Irq<Ftm0Id>;
pub type IrqFtm1 = Irq<Ftm1Id>;
pub type IrqFtm2 = Irq<Ftm2Id>;
pub type IrqPit0 = Irq<Pit0Id>;
pub type IrqPit1 = Irq<Pit1Id>;
pub type IrqPit2 = Irq<Pit2Id>;
pub type IrqPit3 = Irq<Pit3Id>;
pub type IrqLptmr0 = Irq<Lptmr0Id>;
pub type IrqSpi0 = Irq<Spi0Id>;
pub type IrqSpi1 = Irq<Spi1Id>;
pub type IrqSpi2 = Irq<Spi2Id>;
pub type IrqI2c0 = Irq<I2c0Id>;
pub type IrqI2c1 = Irq<I2c1Id>;
pub type IrqUart0Lon = Irq<Uart0LonId>;
pub type IrqUart0RxTx = Irq<Uart0RxTxId>;
pub type IrqUart0Err = Irq<Uart0ErrId>;
pub type IrqUart1RxTx = Irq<Uart1RxTxId>;
pub type IrqUart1Err = Irq<Uart1ErrId>;
pub type IrqUart2RxTx = Irq<Uart2RxTxId>;
pub type IrqUart2Err = Irq<Uart2ErrId>;
pub type IrqUart3RxTx = Irq<Uart3RxTxId>;
pub type IrqUart3Err = Irq<Uart3ErrId>;
pub type IrqUart4RxTx = Irq<Uart4RxTxId>;
pub type IrqUart4Err = Irq<Uart4ErrId>;
pub type IrqUart5RxTx = Irq<Uart5RxTxId>;
pub type IrqUart5Err = Irq<Uart5ErrId>;
pub type IrqPorta = Irq<PortaId>;
pub type IrqPortb = Irq<PortbId>;
pub type IrqPortc = Irq<PortcId>;
pub type IrqPortd = Irq<PortdId>;
pub type IrqPorte = Irq<PorteId>;

#[doc(hidden)]
pub struct WdogId {} // IRQ 22
#[doc(hidden)]
pub struct DmaErrorId {} // IRQ 16
#[doc(hidden)]
pub struct Dma0Id {} // IRQ 0
#[doc(hidden)]
pub struct Dma1Id {} // IRQ 1
#[doc(hidden)]
pub struct Dma2Id {} // IRQ 2
#[doc(hidden)]
pub struct Dma3Id {} // IRQ 3
#[doc(hidden)]
pub struct Dma4Id {} // IRQ 4
#[doc(hidden)]
pub struct Dma5Id {} // IRQ 5
#[doc(hidden)]
pub struct Dma6Id {} // IRQ 6
#[doc(hidden)]
pub struct Dma7Id {} // IRQ 7
#[doc(hidden)]
pub struct Dma8Id {} // IRQ 8
#[doc(hidden)]
pub struct Dma9Id {} // IRQ 9
#[doc(hidden)]
pub struct Dma10Id {} // IRQ 10
#[doc(hidden)]
pub struct Dma11Id {} // IRQ 11
#[doc(hidden)]
pub struct Dma12Id {} // IRQ 12
#[doc(hidden)]
pub struct Dma13Id {} // IRQ 13
#[doc(hidden)]
pub struct Dma14Id {} // IRQ 14
#[doc(hidden)]
pub struct Dma15Id {} // IRQ 15
#[doc(hidden)]
pub struct Ftm0Id {} // IRQ 42
#[doc(hidden)]
pub struct Ftm1Id {} // IRQ 43
#[doc(hidden)]
pub struct Ftm2Id {} // IRQ 44
#[doc(hidden)]
pub struct Pit0Id {} // IRQ 48
#[doc(hidden)]
pub struct Pit1Id {} // IRQ 49
#[doc(hidden)]
pub struct Pit2Id {} // IRQ 50
#[doc(hidden)]
pub struct Pit3Id {} // IRQ 51
#[doc(hidden)]
pub struct Lptmr0Id {} // IRQ 58
#[doc(hidden)]
pub struct Spi0Id {} // IRQ 26
#[doc(hidden)]
pub struct Spi1Id {} // IRQ 27
#[doc(hidden)]
pub struct Spi2Id {} // IRQ 65
#[doc(hidden)]
pub struct I2c0Id {} // IRQ 24
#[doc(hidden)]
pub struct I2c1Id {} // IRQ 25
#[doc(hidden)]
pub struct Uart0LonId {} // IRQ 30
#[doc(hidden)]
pub struct Uart0RxTxId {} // IRQ 31
#[doc(hidden)]
pub struct Uart0ErrId {} // IRQ 32
#[doc(hidden)]
pub struct Uart1RxTxId {} // IRQ 33
#[doc(hidden)]
pub struct Uart1ErrId {} // IRQ 34
#[doc(hidden)]
pub struct Uart2RxTxId {} // IRQ 35
#[doc(hidden)]
pub struct Uart2ErrId {} // IRQ 36
#[doc(hidden)]
pub struct Uart3RxTxId {} // IRQ 37
#[doc(hidden)]
pub struct Uart3ErrId {} // IRQ 38
#[doc(hidden)]
pub struct Uart4RxTxId {} // IRQ 66
#[doc(hidden)]
pub struct Uart4ErrId {} // IRQ 67
#[doc(hidden)]
pub struct Uart5RxTxId {} // IRQ 68
#[doc(hidden)]
pub struct Uart5ErrId {} // IRQ 69
#[doc(hidden)]
pub struct PortaId {} // IRQ 59
#[doc(hidden)]
pub struct PortbId {} // IRQ 60
#[doc(hidden)]
pub struct PortcId {} // IRQ 61
#[doc(hidden)]
pub struct PortdId {} // IRQ 62
#[doc(hidden)]
pub struct PorteId {} // IRQ 63

pub fn set_handler(index: usize, handler: Option<Handler>) {
  unsafe { 
     assert!(R_INTERRUPT_HANDLERS[index].is_some() != handler.is_some());
     R_INTERRUPT_HANDLERS[index] = handler
  };
}

use super::nvic::NVIC;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Irq<T>(usize, T);
impl<T> Irq<T> {
   pub fn index(&self) -> usize { self.0 }

   pub fn is_enabled(&self) -> bool { NVIC.iser((self.0 >> 5)).setena((self.0 & 0b11111)) != 0 }

   pub fn set_enabled(&self, value: bool) {
      if value {
         assert!(self.handler().is_some());
         NVIC.set_iser((self.0 >> 5), |r| r.set_setena((self.0 & 0b11111), 1));
      } else {
         NVIC.set_icer((self.0 >> 5), |r| r.set_clrena((self.0 & 0b11111), 1));
      }
   }

   pub fn is_pending(&self) -> bool {
       NVIC.ispr((self.0 >> 5)).setpend((self.0 & 0b11111)) != 0
   }

   pub fn set_pending(&self, value: bool) {
       if value {
           NVIC.set_ispr((self.0 >> 5), |r| r.set_setpend((self.0 & 0b11111), 1));
       } else {
           NVIC.set_icpr((self.0 >> 5), |r| r.set_clrpend((self.0 & 0b11111), 1));
       }
   }

   pub fn is_active(&self) -> bool {
       NVIC.iabr((self.0 >> 5)).active(self.0 & 0b11111) != 0
   }

   pub fn priority(&self) -> u8 {
       NVIC.ipr((self.0 >> 4)).pri(self.0 & 0b1111).into()
   }

   pub fn set_priority(&self, value: u8) {
       NVIC.with_ipr((self.0 >> 4), |r| r.set_pri(self.0 & 0b1111, value));
   }

   pub fn trigger_interrupt(&self) {
       NVIC.set_stir(|r| r.set_intid(self.0));
   }

   pub fn handler(&self) -> Option<Handler> { unsafe { R_INTERRUPT_HANDLERS[self.0] } }

   pub fn set_handler(&self, handler: Option<Handler>) {
      unsafe { assert!(R_INTERRUPT_HANDLERS[self.0].is_some() != handler.is_some()); };
      unsafe { R_INTERRUPT_HANDLERS[self.0] = handler };
   }
}

pub struct IrqHandle {}
pub struct IrqGuard<'a>(usize, PhantomData<&'a IrqHandle>);
impl<'a> IrqGuard<'a> {
   pub fn new(index: usize) -> Self {
      IrqGuard(index, PhantomData)
   }
}
impl<'a> Drop for IrqGuard<'a> {
   fn drop(&mut self) {
      set_handler(self.0, None)
   }
}


pub trait RegisterHandler {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a>;
}

pub trait HandleInterrupt {
   fn handle_interrupt(&self);
}

impl RegisterHandler for IrqWdog {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(22, Some(wrapper::<F>));
       IrqGuard::new(22)
   }
}

impl RegisterHandler for IrqDmaError {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(16, Some(wrapper::<F>));
       IrqGuard::new(16)
   }
}

impl RegisterHandler for IrqDma0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(0, Some(wrapper::<F>));
       IrqGuard::new(0)
   }
}

impl RegisterHandler for IrqDma1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(1, Some(wrapper::<F>));
       IrqGuard::new(1)
   }
}

impl RegisterHandler for IrqDma2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(2, Some(wrapper::<F>));
       IrqGuard::new(2)
   }
}

impl RegisterHandler for IrqDma3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(3, Some(wrapper::<F>));
       IrqGuard::new(3)
   }
}

impl RegisterHandler for IrqDma4 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(4, Some(wrapper::<F>));
       IrqGuard::new(4)
   }
}

impl RegisterHandler for IrqDma5 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(5, Some(wrapper::<F>));
       IrqGuard::new(5)
   }
}

impl RegisterHandler for IrqDma6 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(6, Some(wrapper::<F>));
       IrqGuard::new(6)
   }
}

impl RegisterHandler for IrqDma7 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(7, Some(wrapper::<F>));
       IrqGuard::new(7)
   }
}

impl RegisterHandler for IrqDma8 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(8, Some(wrapper::<F>));
       IrqGuard::new(8)
   }
}

impl RegisterHandler for IrqDma9 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(9, Some(wrapper::<F>));
       IrqGuard::new(9)
   }
}

impl RegisterHandler for IrqDma10 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(10, Some(wrapper::<F>));
       IrqGuard::new(10)
   }
}

impl RegisterHandler for IrqDma11 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(11, Some(wrapper::<F>));
       IrqGuard::new(11)
   }
}

impl RegisterHandler for IrqDma12 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(12, Some(wrapper::<F>));
       IrqGuard::new(12)
   }
}

impl RegisterHandler for IrqDma13 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(13, Some(wrapper::<F>));
       IrqGuard::new(13)
   }
}

impl RegisterHandler for IrqDma14 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(14, Some(wrapper::<F>));
       IrqGuard::new(14)
   }
}

impl RegisterHandler for IrqDma15 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(15, Some(wrapper::<F>));
       IrqGuard::new(15)
   }
}

impl RegisterHandler for IrqFtm0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(42, Some(wrapper::<F>));
       IrqGuard::new(42)
   }
}

impl RegisterHandler for IrqFtm1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(43, Some(wrapper::<F>));
       IrqGuard::new(43)
   }
}

impl RegisterHandler for IrqFtm2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(44, Some(wrapper::<F>));
       IrqGuard::new(44)
   }
}

impl RegisterHandler for IrqPit0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(48, Some(wrapper::<F>));
       IrqGuard::new(48)
   }
}

impl RegisterHandler for IrqPit1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(49, Some(wrapper::<F>));
       IrqGuard::new(49)
   }
}

impl RegisterHandler for IrqPit2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(50, Some(wrapper::<F>));
       IrqGuard::new(50)
   }
}

impl RegisterHandler for IrqPit3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(51, Some(wrapper::<F>));
       IrqGuard::new(51)
   }
}

impl RegisterHandler for IrqLptmr0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(58, Some(wrapper::<F>));
       IrqGuard::new(58)
   }
}

impl RegisterHandler for IrqSpi0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(26, Some(wrapper::<F>));
       IrqGuard::new(26)
   }
}

impl RegisterHandler for IrqSpi1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(27, Some(wrapper::<F>));
       IrqGuard::new(27)
   }
}

impl RegisterHandler for IrqSpi2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(65, Some(wrapper::<F>));
       IrqGuard::new(65)
   }
}

impl RegisterHandler for IrqI2c0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(24, Some(wrapper::<F>));
       IrqGuard::new(24)
   }
}

impl RegisterHandler for IrqI2c1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(25, Some(wrapper::<F>));
       IrqGuard::new(25)
   }
}

impl RegisterHandler for IrqUart0Lon {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(30, Some(wrapper::<F>));
       IrqGuard::new(30)
   }
}

impl RegisterHandler for IrqUart0RxTx {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(31, Some(wrapper::<F>));
       IrqGuard::new(31)
   }
}

impl RegisterHandler for IrqUart0Err {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(32, Some(wrapper::<F>));
       IrqGuard::new(32)
   }
}

impl RegisterHandler for IrqUart1RxTx {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(33, Some(wrapper::<F>));
       IrqGuard::new(33)
   }
}

impl RegisterHandler for IrqUart1Err {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(34, Some(wrapper::<F>));
       IrqGuard::new(34)
   }
}

impl RegisterHandler for IrqUart2RxTx {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(35, Some(wrapper::<F>));
       IrqGuard::new(35)
   }
}

impl RegisterHandler for IrqUart2Err {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(36, Some(wrapper::<F>));
       IrqGuard::new(36)
   }
}

impl RegisterHandler for IrqUart3RxTx {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(37, Some(wrapper::<F>));
       IrqGuard::new(37)
   }
}

impl RegisterHandler for IrqUart3Err {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(38, Some(wrapper::<F>));
       IrqGuard::new(38)
   }
}

impl RegisterHandler for IrqUart4RxTx {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(66, Some(wrapper::<F>));
       IrqGuard::new(66)
   }
}

impl RegisterHandler for IrqUart4Err {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(67, Some(wrapper::<F>));
       IrqGuard::new(67)
   }
}

impl RegisterHandler for IrqUart5RxTx {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(68, Some(wrapper::<F>));
       IrqGuard::new(68)
   }
}

impl RegisterHandler for IrqUart5Err {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(69, Some(wrapper::<F>));
       IrqGuard::new(69)
   }
}

impl RegisterHandler for IrqPorta {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(59, Some(wrapper::<F>));
       IrqGuard::new(59)
   }
}

impl RegisterHandler for IrqPortb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(60, Some(wrapper::<F>));
       IrqGuard::new(60)
   }
}

impl RegisterHandler for IrqPortc {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(61, Some(wrapper::<F>));
       IrqGuard::new(61)
   }
}

impl RegisterHandler for IrqPortd {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(62, Some(wrapper::<F>));
       IrqGuard::new(62)
   }
}

impl RegisterHandler for IrqPorte {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(63, Some(wrapper::<F>));
       IrqGuard::new(63)
   }
}

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 86] = [
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 16: No Description
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 22: No Description
   None,
   None,                          // IRQ 24: No Description
   None,                          // IRQ 25: No Description
   None,                          // IRQ 26: No Description
   None,                          // IRQ 27: No Description
   None,
   None,
   None,                          // IRQ 30: No Description
   None,                          // IRQ 31: No Description
   None,                          // IRQ 32: No Description
   None,                          // IRQ 33: No Description
   None,                          // IRQ 34: No Description
   None,                          // IRQ 35: No Description
   None,                          // IRQ 36: No Description
   None,                          // IRQ 37: No Description
   None,                          // IRQ 38: No Description
   None,
   None,
   None,
   None,                          // IRQ 42: No Description
   None,                          // IRQ 43: No Description
   None,                          // IRQ 44: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 58: No Description
   None,                          // IRQ 59: No Description
   None,                          // IRQ 60: No Description
   None,                          // IRQ 61: No Description
   None,                          // IRQ 62: No Description
   None,                          // IRQ 63: No Description
   None,
   None,                          // IRQ 65: No Description
   None,                          // IRQ 66: No Description
   None,                          // IRQ 67: No Description
   None,                          // IRQ 68: No Description
   None,                          // IRQ 69: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 82: No Description
   None,                          // IRQ 83: No Description
   None,                          // IRQ 84: No Description
   None,                          // IRQ 85: No Description
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 86] = [None; 86];

