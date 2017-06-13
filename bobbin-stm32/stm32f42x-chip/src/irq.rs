use ::core::marker::PhantomData;
pub type Handler = extern "C" fn();

pub const IRQ_PVD_IRQ: Irq<PvdIrqId> = Irq(1, PvdIrqId {});
pub const IRQ_TAMP_STAMP: Irq<TampStampId> = Irq(2, TampStampId {});
pub const IRQ_RCC: Irq<RccId> = Irq(5, RccId {});
pub const IRQ_EXTI0: Irq<Exti0Id> = Irq(6, Exti0Id {});
pub const IRQ_EXTI1: Irq<Exti1Id> = Irq(7, Exti1Id {});
pub const IRQ_EXTI2: Irq<Exti2Id> = Irq(8, Exti2Id {});
pub const IRQ_EXTI3: Irq<Exti3Id> = Irq(9, Exti3Id {});
pub const IRQ_EXTI4: Irq<Exti4Id> = Irq(10, Exti4Id {});
pub const IRQ_DMA1_STREAM0: Irq<Dma1Stream0Id> = Irq(11, Dma1Stream0Id {});
pub const IRQ_DMA1_STREAM1: Irq<Dma1Stream1Id> = Irq(12, Dma1Stream1Id {});
pub const IRQ_DMA1_STREAM2: Irq<Dma1Stream2Id> = Irq(13, Dma1Stream2Id {});
pub const IRQ_DMA1_STREAM3: Irq<Dma1Stream3Id> = Irq(14, Dma1Stream3Id {});
pub const IRQ_DMA1_STREAM4: Irq<Dma1Stream4Id> = Irq(15, Dma1Stream4Id {});
pub const IRQ_DMA1_STREAM5: Irq<Dma1Stream5Id> = Irq(16, Dma1Stream5Id {});
pub const IRQ_DMA1_STREAM6: Irq<Dma1Stream6Id> = Irq(17, Dma1Stream6Id {});
pub const IRQ_ADC: Irq<AdcId> = Irq(18, AdcId {});
pub const IRQ_EXTI9_5: Irq<Exti95Id> = Irq(23, Exti95Id {});
pub const IRQ_TIM1_BRK_TIM9: Irq<Tim1BrkTim9Id> = Irq(24, Tim1BrkTim9Id {});
pub const IRQ_TIM1_UP_TIM10: Irq<Tim1UpTim10Id> = Irq(25, Tim1UpTim10Id {});
pub const IRQ_TIM1_TRG_COM_TIM11: Irq<Tim1TrgComTim11Id> = Irq(26, Tim1TrgComTim11Id {});
pub const IRQ_TIM1_CC: Irq<Tim1CcId> = Irq(27, Tim1CcId {});
pub const IRQ_TIM2: Irq<Tim2Id> = Irq(28, Tim2Id {});
pub const IRQ_TIM3: Irq<Tim3Id> = Irq(29, Tim3Id {});
pub const IRQ_TIM4: Irq<Tim4Id> = Irq(30, Tim4Id {});
pub const IRQ_I2C1_EV: Irq<I2c1EvId> = Irq(31, I2c1EvId {});
pub const IRQ_I2C1_ER: Irq<I2c1ErId> = Irq(32, I2c1ErId {});
pub const IRQ_I2C2_EV: Irq<I2c2EvId> = Irq(33, I2c2EvId {});
pub const IRQ_I2C2_ER: Irq<I2c2ErId> = Irq(34, I2c2ErId {});
pub const IRQ_SPI1: Irq<Spi1Id> = Irq(35, Spi1Id {});
pub const IRQ_SPI2: Irq<Spi2Id> = Irq(36, Spi2Id {});
pub const IRQ_USART1: Irq<Usart1Id> = Irq(37, Usart1Id {});
pub const IRQ_USART2: Irq<Usart2Id> = Irq(38, Usart2Id {});
pub const IRQ_USART3: Irq<Usart3Id> = Irq(39, Usart3Id {});
pub const IRQ_EXTI15_10: Irq<Exti1510Id> = Irq(40, Exti1510Id {});
pub const IRQ_TIM8_BRK_TIM12: Irq<Tim8BrkTim12Id> = Irq(43, Tim8BrkTim12Id {});
pub const IRQ_TIM8_UP_TIM13: Irq<Tim8UpTim13Id> = Irq(44, Tim8UpTim13Id {});
pub const IRQ_TIM8_TRG_COM_TIM14: Irq<Tim8TrgComTim14Id> = Irq(45, Tim8TrgComTim14Id {});
pub const IRQ_TIM8_CC: Irq<Tim8CcId> = Irq(46, Tim8CcId {});
pub const IRQ_DMA1_STREAM7: Irq<Dma1Stream7Id> = Irq(47, Dma1Stream7Id {});
pub const IRQ_TIM5: Irq<Tim5Id> = Irq(50, Tim5Id {});
pub const IRQ_SPI3: Irq<Spi3Id> = Irq(51, Spi3Id {});
pub const IRQ_TIM6_DAC: Irq<Tim6DacId> = Irq(54, Tim6DacId {});
pub const IRQ_TIM7: Irq<Tim7Id> = Irq(55, Tim7Id {});
pub const IRQ_DMA2_STREAM0: Irq<Dma2Stream0Id> = Irq(56, Dma2Stream0Id {});
pub const IRQ_DMA2_STREAM1: Irq<Dma2Stream1Id> = Irq(57, Dma2Stream1Id {});
pub const IRQ_DMA2_STREAM2: Irq<Dma2Stream2Id> = Irq(58, Dma2Stream2Id {});
pub const IRQ_DMA2_STREAM3: Irq<Dma2Stream3Id> = Irq(59, Dma2Stream3Id {});
pub const IRQ_DMA2_STREAM4: Irq<Dma2Stream4Id> = Irq(60, Dma2Stream4Id {});
pub const IRQ_ETH_IRQ: Irq<EthIrqId> = Irq(61, EthIrqId {});
pub const IRQ_ETH_WKUP_IRQ: Irq<EthWkupIrqId> = Irq(62, EthWkupIrqId {});
pub const IRQ_DMA2_STREAM5: Irq<Dma2Stream5Id> = Irq(68, Dma2Stream5Id {});
pub const IRQ_DMA2_STREAM6: Irq<Dma2Stream6Id> = Irq(69, Dma2Stream6Id {});
pub const IRQ_DMA2_STREAM7: Irq<Dma2Stream7Id> = Irq(70, Dma2Stream7Id {});
pub const IRQ_USART6: Irq<Usart6Id> = Irq(71, Usart6Id {});
pub const IRQ_I2C3_EV: Irq<I2c3EvId> = Irq(72, I2c3EvId {});
pub const IRQ_I2C3_ER: Irq<I2c3ErId> = Irq(73, I2c3ErId {});
pub const IRQ_UART7: Irq<Uart7Id> = Irq(82, Uart7Id {});
pub const IRQ_UART8: Irq<Uart8Id> = Irq(83, Uart8Id {});
pub const IRQ_SPI4: Irq<Spi4Id> = Irq(84, Spi4Id {});
pub const IRQ_SPI5: Irq<Spi5Id> = Irq(85, Spi5Id {});
pub const IRQ_SPI6: Irq<Spi6Id> = Irq(86, Spi6Id {});

pub struct PvdIrqId {} // IRQ 1
pub struct TampStampId {} // IRQ 2
pub struct RccId {} // IRQ 5
pub struct Exti0Id {} // IRQ 6
pub struct Exti1Id {} // IRQ 7
pub struct Exti2Id {} // IRQ 8
pub struct Exti3Id {} // IRQ 9
pub struct Exti4Id {} // IRQ 10
pub struct Dma1Stream0Id {} // IRQ 11
pub struct Dma1Stream1Id {} // IRQ 12
pub struct Dma1Stream2Id {} // IRQ 13
pub struct Dma1Stream3Id {} // IRQ 14
pub struct Dma1Stream4Id {} // IRQ 15
pub struct Dma1Stream5Id {} // IRQ 16
pub struct Dma1Stream6Id {} // IRQ 17
pub struct AdcId {} // IRQ 18
pub struct Exti95Id {} // IRQ 23
pub struct Tim1BrkTim9Id {} // IRQ 24
pub struct Tim1UpTim10Id {} // IRQ 25
pub struct Tim1TrgComTim11Id {} // IRQ 26
pub struct Tim1CcId {} // IRQ 27
pub struct Tim2Id {} // IRQ 28
pub struct Tim3Id {} // IRQ 29
pub struct Tim4Id {} // IRQ 30
pub struct I2c1EvId {} // IRQ 31
pub struct I2c1ErId {} // IRQ 32
pub struct I2c2EvId {} // IRQ 33
pub struct I2c2ErId {} // IRQ 34
pub struct Spi1Id {} // IRQ 35
pub struct Spi2Id {} // IRQ 36
pub struct Usart1Id {} // IRQ 37
pub struct Usart2Id {} // IRQ 38
pub struct Usart3Id {} // IRQ 39
pub struct Exti1510Id {} // IRQ 40
pub struct Tim8BrkTim12Id {} // IRQ 43
pub struct Tim8UpTim13Id {} // IRQ 44
pub struct Tim8TrgComTim14Id {} // IRQ 45
pub struct Tim8CcId {} // IRQ 46
pub struct Dma1Stream7Id {} // IRQ 47
pub struct Tim5Id {} // IRQ 50
pub struct Spi3Id {} // IRQ 51
pub struct Tim6DacId {} // IRQ 54
pub struct Tim7Id {} // IRQ 55
pub struct Dma2Stream0Id {} // IRQ 56
pub struct Dma2Stream1Id {} // IRQ 57
pub struct Dma2Stream2Id {} // IRQ 58
pub struct Dma2Stream3Id {} // IRQ 59
pub struct Dma2Stream4Id {} // IRQ 60
pub struct EthIrqId {} // IRQ 61
pub struct EthWkupIrqId {} // IRQ 62
pub struct Dma2Stream5Id {} // IRQ 68
pub struct Dma2Stream6Id {} // IRQ 69
pub struct Dma2Stream7Id {} // IRQ 70
pub struct Usart6Id {} // IRQ 71
pub struct I2c3EvId {} // IRQ 72
pub struct I2c3ErId {} // IRQ 73
pub struct Uart7Id {} // IRQ 82
pub struct Uart8Id {} // IRQ 83
pub struct Spi4Id {} // IRQ 84
pub struct Spi5Id {} // IRQ 85
pub struct Spi6Id {} // IRQ 86

pub fn set_handler(index: usize, handler: Option<Handler>) {
  unsafe { R_INTERRUPT_HANDLERS[index] = handler };
}

use super::nvic::{NVIC, Iser, Icer, Ispr, Icpr, Stir};
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Irq<T>(usize, T);
impl<T> Irq<T> {
   pub fn index(&self) -> usize { self.0 }

   pub fn is_enabled(&self) -> bool { NVIC.iser((self.0 >> 5)).setena((self.0 & 0b11111)) != 0 }

   pub fn set_enabled(&self, value: bool) {
      if value {
         NVIC.set_iser((self.0 >> 5), Iser(0).set_setena((self.0 & 0b11111), 1));
      } else {
         NVIC.set_icer((self.0 >> 5), Icer(0).set_clrena((self.0 & 0b11111), 1));
      }
   }

   pub fn is_pending(&self) -> bool {
       NVIC.ispr((self.0 >> 5)).setpend((self.0 & 0b11111)) != 0
   }

   pub fn set_pending(&self, value: bool) {
       if value {
           NVIC.set_ispr((self.0 >> 5), Ispr(0).set_setpend((self.0 & 0b11111), 1));
       } else {
           NVIC.set_icpr((self.0 >> 5), Icpr(0).set_clrpend((self.0 & 0b11111), 1));
       }
   }

   pub fn is_active(&self) -> bool {
       NVIC.iabr((self.0 >> 5)).active((self.0 & 0b11111)) != 0
   }

   pub fn priority(&self) -> u8 {
       NVIC.ipr((self.0 >> 4)).pri((self.0 & 0b1111)) as u8
   }

   pub fn set_priority(&self, value: u8) {
       NVIC.with_ipr((self.0 >> 4), |r| r.set_pri((self.0 & 0b1111), value as u32));
   }

   pub fn trigger_interrupt(&self) {
       NVIC.set_stir(Stir(0).set_intid(self.0 as u32));
   }

   pub fn set_handler(&self, handler: Option<Handler>) {
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

impl RegisterHandler for Irq<PvdIrqId> {
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

impl RegisterHandler for Irq<TampStampId> {
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

impl RegisterHandler for Irq<RccId> {
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

impl RegisterHandler for Irq<Exti0Id> {
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

impl RegisterHandler for Irq<Exti1Id> {
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

impl RegisterHandler for Irq<Exti2Id> {
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

impl RegisterHandler for Irq<Exti3Id> {
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

impl RegisterHandler for Irq<Exti4Id> {
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

impl RegisterHandler for Irq<Dma1Stream0Id> {
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

impl RegisterHandler for Irq<Dma1Stream1Id> {
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

impl RegisterHandler for Irq<Dma1Stream2Id> {
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

impl RegisterHandler for Irq<Dma1Stream3Id> {
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

impl RegisterHandler for Irq<Dma1Stream4Id> {
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

impl RegisterHandler for Irq<Dma1Stream5Id> {
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

impl RegisterHandler for Irq<Dma1Stream6Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(17, Some(wrapper::<F>));
       IrqGuard::new(17)
   }
}

impl RegisterHandler for Irq<AdcId> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(18, Some(wrapper::<F>));
       IrqGuard::new(18)
   }
}

impl RegisterHandler for Irq<Exti95Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(23, Some(wrapper::<F>));
       IrqGuard::new(23)
   }
}

impl RegisterHandler for Irq<Tim1BrkTim9Id> {
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

impl RegisterHandler for Irq<Tim1UpTim10Id> {
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

impl RegisterHandler for Irq<Tim1TrgComTim11Id> {
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

impl RegisterHandler for Irq<Tim1CcId> {
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

impl RegisterHandler for Irq<Tim2Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(28, Some(wrapper::<F>));
       IrqGuard::new(28)
   }
}

impl RegisterHandler for Irq<Tim3Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(29, Some(wrapper::<F>));
       IrqGuard::new(29)
   }
}

impl RegisterHandler for Irq<Tim4Id> {
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

impl RegisterHandler for Irq<I2c1EvId> {
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

impl RegisterHandler for Irq<I2c1ErId> {
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

impl RegisterHandler for Irq<I2c2EvId> {
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

impl RegisterHandler for Irq<I2c2ErId> {
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

impl RegisterHandler for Irq<Spi1Id> {
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

impl RegisterHandler for Irq<Spi2Id> {
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

impl RegisterHandler for Irq<Usart1Id> {
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

impl RegisterHandler for Irq<Usart2Id> {
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

impl RegisterHandler for Irq<Usart3Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(39, Some(wrapper::<F>));
       IrqGuard::new(39)
   }
}

impl RegisterHandler for Irq<Exti1510Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(40, Some(wrapper::<F>));
       IrqGuard::new(40)
   }
}

impl RegisterHandler for Irq<Tim8BrkTim12Id> {
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

impl RegisterHandler for Irq<Tim8UpTim13Id> {
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

impl RegisterHandler for Irq<Tim8TrgComTim14Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(45, Some(wrapper::<F>));
       IrqGuard::new(45)
   }
}

impl RegisterHandler for Irq<Tim8CcId> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(46, Some(wrapper::<F>));
       IrqGuard::new(46)
   }
}

impl RegisterHandler for Irq<Dma1Stream7Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(47, Some(wrapper::<F>));
       IrqGuard::new(47)
   }
}

impl RegisterHandler for Irq<Tim5Id> {
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

impl RegisterHandler for Irq<Spi3Id> {
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

impl RegisterHandler for Irq<Tim6DacId> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(54, Some(wrapper::<F>));
       IrqGuard::new(54)
   }
}

impl RegisterHandler for Irq<Tim7Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(55, Some(wrapper::<F>));
       IrqGuard::new(55)
   }
}

impl RegisterHandler for Irq<Dma2Stream0Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(56, Some(wrapper::<F>));
       IrqGuard::new(56)
   }
}

impl RegisterHandler for Irq<Dma2Stream1Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(57, Some(wrapper::<F>));
       IrqGuard::new(57)
   }
}

impl RegisterHandler for Irq<Dma2Stream2Id> {
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

impl RegisterHandler for Irq<Dma2Stream3Id> {
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

impl RegisterHandler for Irq<Dma2Stream4Id> {
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

impl RegisterHandler for Irq<EthIrqId> {
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

impl RegisterHandler for Irq<EthWkupIrqId> {
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

impl RegisterHandler for Irq<Dma2Stream5Id> {
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

impl RegisterHandler for Irq<Dma2Stream6Id> {
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

impl RegisterHandler for Irq<Dma2Stream7Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(70, Some(wrapper::<F>));
       IrqGuard::new(70)
   }
}

impl RegisterHandler for Irq<Usart6Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(71, Some(wrapper::<F>));
       IrqGuard::new(71)
   }
}

impl RegisterHandler for Irq<I2c3EvId> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(72, Some(wrapper::<F>));
       IrqGuard::new(72)
   }
}

impl RegisterHandler for Irq<I2c3ErId> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(73, Some(wrapper::<F>));
       IrqGuard::new(73)
   }
}

impl RegisterHandler for Irq<Uart7Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(82, Some(wrapper::<F>));
       IrqGuard::new(82)
   }
}

impl RegisterHandler for Irq<Uart8Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(83, Some(wrapper::<F>));
       IrqGuard::new(83)
   }
}

impl RegisterHandler for Irq<Spi4Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(84, Some(wrapper::<F>));
       IrqGuard::new(84)
   }
}

impl RegisterHandler for Irq<Spi5Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(85, Some(wrapper::<F>));
       IrqGuard::new(85)
   }
}

impl RegisterHandler for Irq<Spi6Id> {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(86, Some(wrapper::<F>));
       IrqGuard::new(86)
   }
}

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 91] = [
   None,
   None,                          // IRQ 1: PVD through EXTI line detection interrupt
   None,                          // IRQ 2: Tamper and TimeStamp interrupts through the EXTI line
   None,
   None,
   None,                          // IRQ 5: RCC global interrupt
   None,                          // IRQ 6: EXTI Line0 interrupt
   None,                          // IRQ 7: EXTI Line1 interrupt
   None,                          // IRQ 8: EXTI Line2 interrupt
   None,                          // IRQ 9: EXTI Line3 interrupt
   None,                          // IRQ 10: EXTI Line4 interrupt
   None,                          // IRQ 11: DMA1 Stream0 global interrupt
   None,                          // IRQ 12: DMA1 Stream1 global interrupt
   None,                          // IRQ 13: DMA1 Stream2 global interrupt
   None,                          // IRQ 14: DMA1 Stream3 global interrupt
   None,                          // IRQ 15: DMA1 Stream4 global interrupt
   None,                          // IRQ 16: DMA1 Stream5 global interrupt
   None,                          // IRQ 17: DMA1 Stream6 global interrupt
   None,                          // IRQ 18: ADC3 global interrupts
   None,
   None,
   None,
   None,
   None,                          // IRQ 23: EXTI Line[9:5] interrupts
   None,                          // IRQ 24: TIM1 Break interrupt and TIM9 global interrupt
   None,                          // IRQ 25: TIM1 Update interrupt and TIM10 global interrupt
   None,                          // IRQ 26: TIM1 Trigger and Commutation interrupts and TIM11 global interrupt
   None,                          // IRQ 27: TIM1 Capture Compare interrupt
   None,                          // IRQ 28: TIM2 global interrupt
   None,                          // IRQ 29: TIM3 global interrupt
   None,                          // IRQ 30: TIM4 global interrupt
   None,                          // IRQ 31: I2C1 event interrupt
   None,                          // IRQ 32: I2C1 error interrupt
   None,                          // IRQ 33: I2C2 event interrupt
   None,                          // IRQ 34: I2C2 error interrupt
   None,                          // IRQ 35: SPI1 global interrupt
   None,                          // IRQ 36: SPI2 global interrupt
   None,                          // IRQ 37: USART1 global interrupt
   None,                          // IRQ 38: USART2 global interrupt
   None,                          // IRQ 39: USART3 global interrupt
   None,                          // IRQ 40: EXTI Line[15:10] interrupts
   None,
   None,
   None,                          // IRQ 43: TIM8 Break interrupt and TIM12 global interrupt
   None,                          // IRQ 44: TIM8 Update interrupt and TIM13 global interrupt
   None,                          // IRQ 45: TIM8 Trigger and Commutation interrupts and TIM14 global interrupt
   None,                          // IRQ 46: TIM8 Capture Compare interrupt
   None,                          // IRQ 47: DMA1 Stream7 global interrupt
   None,
   None,
   None,                          // IRQ 50: TIM5 global interrupt
   None,                          // IRQ 51: SPI3 global interrupt
   None,
   None,
   None,                          // IRQ 54: TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt
   None,                          // IRQ 55: TIM7 global interrupt
   None,                          // IRQ 56: DMA2 Stream0 global interrupt
   None,                          // IRQ 57: DMA2 Stream1 global interrupt
   None,                          // IRQ 58: DMA2 Stream2 global interrupt
   None,                          // IRQ 59: DMA2 Stream3 global interrupt
   None,                          // IRQ 60: DMA2 Stream4 global interrupt
   None,                          // IRQ 61: Ethernet global interrupt
   None,                          // IRQ 62: Ethernet Wakeup through EXTI line interrupt
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 68: DMA2 Stream5 global interrupt
   None,                          // IRQ 69: DMA2 Stream6 global interrupt
   None,                          // IRQ 70: DMA2 Stream7 global interrupt
   None,                          // IRQ 71: USART6 global interrupt
   None,                          // IRQ 72: I2C3 event interrupt
   None,                          // IRQ 73: I2C3 error interrupt
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 82: UART 7 global interrupt
   None,                          // IRQ 83: UART 8 global interrupt
   None,                          // IRQ 84: SPI 4 global interrupt
   None,                          // IRQ 85: SPI 5 global interrupt
   None,                          // IRQ 86: SPI 6 global interrupt
   None,
   None,
   None,
   None,
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 91] = [None; 91];

