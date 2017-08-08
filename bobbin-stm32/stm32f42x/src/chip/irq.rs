//! Interrupts

use ::core::marker::PhantomData;
pub type Handler = extern "C" fn();

pub const IRQ_WWDG: IrqWwdg = Irq(0, WwdgId {});
pub const IRQ_EXTI0: IrqExti0 = Irq(6, Exti0Id {});
pub const IRQ_EXTI1: IrqExti1 = Irq(7, Exti1Id {});
pub const IRQ_EXTI2: IrqExti2 = Irq(8, Exti2Id {});
pub const IRQ_EXTI3: IrqExti3 = Irq(9, Exti3Id {});
pub const IRQ_EXTI4: IrqExti4 = Irq(10, Exti4Id {});
pub const IRQ_EXTI5: IrqExti5 = Irq(23, Exti5Id {});
pub const IRQ_EXTI6: IrqExti6 = Irq(23, Exti6Id {});
pub const IRQ_EXTI7: IrqExti7 = Irq(23, Exti7Id {});
pub const IRQ_EXTI8: IrqExti8 = Irq(23, Exti8Id {});
pub const IRQ_EXTI9: IrqExti9 = Irq(23, Exti9Id {});
pub const IRQ_EXTI10: IrqExti10 = Irq(40, Exti10Id {});
pub const IRQ_EXTI11: IrqExti11 = Irq(40, Exti11Id {});
pub const IRQ_EXTI12: IrqExti12 = Irq(40, Exti12Id {});
pub const IRQ_EXTI13: IrqExti13 = Irq(40, Exti13Id {});
pub const IRQ_EXTI14: IrqExti14 = Irq(40, Exti14Id {});
pub const IRQ_EXTI15: IrqExti15 = Irq(40, Exti15Id {});
pub const IRQ_EXTI16: IrqExti16 = Irq(1, Exti16Id {});
pub const IRQ_EXTI17: IrqExti17 = Irq(41, Exti17Id {});
pub const IRQ_EXTI18: IrqExti18 = Irq(42, Exti18Id {});
pub const IRQ_EXTI19: IrqExti19 = Irq(62, Exti19Id {});
pub const IRQ_EXTI20: IrqExti20 = Irq(76, Exti20Id {});
pub const IRQ_EXTI21: IrqExti21 = Irq(2, Exti21Id {});
pub const IRQ_EXTI22: IrqExti22 = Irq(3, Exti22Id {});
pub const IRQ_TIM6: IrqTim6 = Irq(54, Tim6Id {});
pub const IRQ_TIM7: IrqTim7 = Irq(55, Tim7Id {});
pub const IRQ_TIM2: IrqTim2 = Irq(28, Tim2Id {});
pub const IRQ_TIM3: IrqTim3 = Irq(29, Tim3Id {});
pub const IRQ_TIM4: IrqTim4 = Irq(30, Tim4Id {});
pub const IRQ_TIM5: IrqTim5 = Irq(50, Tim5Id {});
pub const IRQ_TIM9: IrqTim9 = Irq(24, Tim9Id {});
pub const IRQ_TIM10: IrqTim10 = Irq(25, Tim10Id {});
pub const IRQ_TIM11: IrqTim11 = Irq(26, Tim11Id {});
pub const IRQ_TIM12: IrqTim12 = Irq(43, Tim12Id {});
pub const IRQ_TIM13: IrqTim13 = Irq(44, Tim13Id {});
pub const IRQ_TIM14: IrqTim14 = Irq(45, Tim14Id {});
pub const IRQ_TIM1_BRK: IrqTim1Brk = Irq(24, Tim1BrkId {});
pub const IRQ_TIM1_UP: IrqTim1Up = Irq(25, Tim1UpId {});
pub const IRQ_TIM1_TRG_COM: IrqTim1TrgCom = Irq(26, Tim1TrgComId {});
pub const IRQ_TIM1_CC: IrqTim1Cc = Irq(27, Tim1CcId {});
pub const IRQ_TIM8_BRK: IrqTim8Brk = Irq(43, Tim8BrkId {});
pub const IRQ_TIM8_UP: IrqTim8Up = Irq(44, Tim8UpId {});
pub const IRQ_TIM8_TRG_COM: IrqTim8TrgCom = Irq(45, Tim8TrgComId {});
pub const IRQ_TIM8_CC: IrqTim8Cc = Irq(46, Tim8CcId {});
pub const IRQ_ADC1: IrqAdc1 = Irq(18, Adc1Id {});
pub const IRQ_ADC2: IrqAdc2 = Irq(18, Adc2Id {});
pub const IRQ_ADC3: IrqAdc3 = Irq(18, Adc3Id {});
pub const IRQ_SPI1: IrqSpi1 = Irq(35, Spi1Id {});
pub const IRQ_SPI2: IrqSpi2 = Irq(36, Spi2Id {});
pub const IRQ_SPI3: IrqSpi3 = Irq(51, Spi3Id {});
pub const IRQ_SPI4: IrqSpi4 = Irq(84, Spi4Id {});
pub const IRQ_SPI5: IrqSpi5 = Irq(85, Spi5Id {});
pub const IRQ_SPI6: IrqSpi6 = Irq(86, Spi6Id {});
pub const IRQ_I2C3_EV: IrqI2c3Ev = Irq(72, I2c3EvId {});
pub const IRQ_I2C3_ER: IrqI2c3Er = Irq(73, I2c3ErId {});
pub const IRQ_I2C2_EV: IrqI2c2Ev = Irq(33, I2c2EvId {});
pub const IRQ_I2C2_ER: IrqI2c2Er = Irq(34, I2c2ErId {});
pub const IRQ_I2C1_EV: IrqI2c1Ev = Irq(31, I2c1EvId {});
pub const IRQ_I2C1_ER: IrqI2c1Er = Irq(32, I2c1ErId {});
pub const IRQ_USART1: IrqUsart1 = Irq(37, Usart1Id {});
pub const IRQ_USART2: IrqUsart2 = Irq(38, Usart2Id {});
pub const IRQ_USART3: IrqUsart3 = Irq(39, Usart3Id {});
pub const IRQ_UART4: IrqUart4 = Irq(52, Uart4Id {});
pub const IRQ_UART5: IrqUart5 = Irq(53, Uart5Id {});
pub const IRQ_USART6: IrqUsart6 = Irq(71, Usart6Id {});
pub const IRQ_UART7: IrqUart7 = Irq(82, Uart7Id {});
pub const IRQ_UART8: IrqUart8 = Irq(83, Uart8Id {});
pub const IRQ_DMA1_STREAM0: IrqDma1Stream0 = Irq(11, Dma1Stream0Id {});
pub const IRQ_DMA1_STREAM1: IrqDma1Stream1 = Irq(12, Dma1Stream1Id {});
pub const IRQ_DMA1_STREAM2: IrqDma1Stream2 = Irq(13, Dma1Stream2Id {});
pub const IRQ_DMA1_STREAM3: IrqDma1Stream3 = Irq(14, Dma1Stream3Id {});
pub const IRQ_DMA1_STREAM4: IrqDma1Stream4 = Irq(15, Dma1Stream4Id {});
pub const IRQ_DMA1_STREAM5: IrqDma1Stream5 = Irq(16, Dma1Stream5Id {});
pub const IRQ_DMA1_STREAM6: IrqDma1Stream6 = Irq(17, Dma1Stream6Id {});
pub const IRQ_DMA1_STREAM7: IrqDma1Stream7 = Irq(47, Dma1Stream7Id {});
pub const IRQ_DMA2_STREAM0: IrqDma2Stream0 = Irq(56, Dma2Stream0Id {});
pub const IRQ_DMA2_STREAM1: IrqDma2Stream1 = Irq(57, Dma2Stream1Id {});
pub const IRQ_DMA2_STREAM2: IrqDma2Stream2 = Irq(58, Dma2Stream2Id {});
pub const IRQ_DMA2_STREAM3: IrqDma2Stream3 = Irq(59, Dma2Stream3Id {});
pub const IRQ_DMA2_STREAM4: IrqDma2Stream4 = Irq(60, Dma2Stream4Id {});
pub const IRQ_DMA2_STREAM5: IrqDma2Stream5 = Irq(68, Dma2Stream5Id {});
pub const IRQ_DMA2_STREAM6: IrqDma2Stream6 = Irq(69, Dma2Stream6Id {});
pub const IRQ_DMA2_STREAM7: IrqDma2Stream7 = Irq(70, Dma2Stream7Id {});

pub type IrqWwdg = Irq<WwdgId>;
pub type IrqExti0 = Irq<Exti0Id>;
pub type IrqExti1 = Irq<Exti1Id>;
pub type IrqExti2 = Irq<Exti2Id>;
pub type IrqExti3 = Irq<Exti3Id>;
pub type IrqExti4 = Irq<Exti4Id>;
pub type IrqExti5 = Irq<Exti5Id>;
pub type IrqExti6 = Irq<Exti6Id>;
pub type IrqExti7 = Irq<Exti7Id>;
pub type IrqExti8 = Irq<Exti8Id>;
pub type IrqExti9 = Irq<Exti9Id>;
pub type IrqExti10 = Irq<Exti10Id>;
pub type IrqExti11 = Irq<Exti11Id>;
pub type IrqExti12 = Irq<Exti12Id>;
pub type IrqExti13 = Irq<Exti13Id>;
pub type IrqExti14 = Irq<Exti14Id>;
pub type IrqExti15 = Irq<Exti15Id>;
pub type IrqExti16 = Irq<Exti16Id>;
pub type IrqExti17 = Irq<Exti17Id>;
pub type IrqExti18 = Irq<Exti18Id>;
pub type IrqExti19 = Irq<Exti19Id>;
pub type IrqExti20 = Irq<Exti20Id>;
pub type IrqExti21 = Irq<Exti21Id>;
pub type IrqExti22 = Irq<Exti22Id>;
pub type IrqTim6 = Irq<Tim6Id>;
pub type IrqTim7 = Irq<Tim7Id>;
pub type IrqTim2 = Irq<Tim2Id>;
pub type IrqTim3 = Irq<Tim3Id>;
pub type IrqTim4 = Irq<Tim4Id>;
pub type IrqTim5 = Irq<Tim5Id>;
pub type IrqTim9 = Irq<Tim9Id>;
pub type IrqTim10 = Irq<Tim10Id>;
pub type IrqTim11 = Irq<Tim11Id>;
pub type IrqTim12 = Irq<Tim12Id>;
pub type IrqTim13 = Irq<Tim13Id>;
pub type IrqTim14 = Irq<Tim14Id>;
pub type IrqTim1Brk = Irq<Tim1BrkId>;
pub type IrqTim1Up = Irq<Tim1UpId>;
pub type IrqTim1TrgCom = Irq<Tim1TrgComId>;
pub type IrqTim1Cc = Irq<Tim1CcId>;
pub type IrqTim8Brk = Irq<Tim8BrkId>;
pub type IrqTim8Up = Irq<Tim8UpId>;
pub type IrqTim8TrgCom = Irq<Tim8TrgComId>;
pub type IrqTim8Cc = Irq<Tim8CcId>;
pub type IrqAdc1 = Irq<Adc1Id>;
pub type IrqAdc2 = Irq<Adc2Id>;
pub type IrqAdc3 = Irq<Adc3Id>;
pub type IrqSpi1 = Irq<Spi1Id>;
pub type IrqSpi2 = Irq<Spi2Id>;
pub type IrqSpi3 = Irq<Spi3Id>;
pub type IrqSpi4 = Irq<Spi4Id>;
pub type IrqSpi5 = Irq<Spi5Id>;
pub type IrqSpi6 = Irq<Spi6Id>;
pub type IrqI2c3Ev = Irq<I2c3EvId>;
pub type IrqI2c3Er = Irq<I2c3ErId>;
pub type IrqI2c2Ev = Irq<I2c2EvId>;
pub type IrqI2c2Er = Irq<I2c2ErId>;
pub type IrqI2c1Ev = Irq<I2c1EvId>;
pub type IrqI2c1Er = Irq<I2c1ErId>;
pub type IrqUsart1 = Irq<Usart1Id>;
pub type IrqUsart2 = Irq<Usart2Id>;
pub type IrqUsart3 = Irq<Usart3Id>;
pub type IrqUart4 = Irq<Uart4Id>;
pub type IrqUart5 = Irq<Uart5Id>;
pub type IrqUsart6 = Irq<Usart6Id>;
pub type IrqUart7 = Irq<Uart7Id>;
pub type IrqUart8 = Irq<Uart8Id>;
pub type IrqDma1Stream0 = Irq<Dma1Stream0Id>;
pub type IrqDma1Stream1 = Irq<Dma1Stream1Id>;
pub type IrqDma1Stream2 = Irq<Dma1Stream2Id>;
pub type IrqDma1Stream3 = Irq<Dma1Stream3Id>;
pub type IrqDma1Stream4 = Irq<Dma1Stream4Id>;
pub type IrqDma1Stream5 = Irq<Dma1Stream5Id>;
pub type IrqDma1Stream6 = Irq<Dma1Stream6Id>;
pub type IrqDma1Stream7 = Irq<Dma1Stream7Id>;
pub type IrqDma2Stream0 = Irq<Dma2Stream0Id>;
pub type IrqDma2Stream1 = Irq<Dma2Stream1Id>;
pub type IrqDma2Stream2 = Irq<Dma2Stream2Id>;
pub type IrqDma2Stream3 = Irq<Dma2Stream3Id>;
pub type IrqDma2Stream4 = Irq<Dma2Stream4Id>;
pub type IrqDma2Stream5 = Irq<Dma2Stream5Id>;
pub type IrqDma2Stream6 = Irq<Dma2Stream6Id>;
pub type IrqDma2Stream7 = Irq<Dma2Stream7Id>;

#[doc(hidden)]
pub struct WwdgId {} // IRQ 0
#[doc(hidden)]
pub struct Exti0Id {} // IRQ 6
#[doc(hidden)]
pub struct Exti1Id {} // IRQ 7
#[doc(hidden)]
pub struct Exti2Id {} // IRQ 8
#[doc(hidden)]
pub struct Exti3Id {} // IRQ 9
#[doc(hidden)]
pub struct Exti4Id {} // IRQ 10
#[doc(hidden)]
pub struct Exti5Id {} // IRQ 23
#[doc(hidden)]
pub struct Exti6Id {} // IRQ 23
#[doc(hidden)]
pub struct Exti7Id {} // IRQ 23
#[doc(hidden)]
pub struct Exti8Id {} // IRQ 23
#[doc(hidden)]
pub struct Exti9Id {} // IRQ 23
#[doc(hidden)]
pub struct Exti10Id {} // IRQ 40
#[doc(hidden)]
pub struct Exti11Id {} // IRQ 40
#[doc(hidden)]
pub struct Exti12Id {} // IRQ 40
#[doc(hidden)]
pub struct Exti13Id {} // IRQ 40
#[doc(hidden)]
pub struct Exti14Id {} // IRQ 40
#[doc(hidden)]
pub struct Exti15Id {} // IRQ 40
#[doc(hidden)]
pub struct Exti16Id {} // IRQ 1
#[doc(hidden)]
pub struct Exti17Id {} // IRQ 41
#[doc(hidden)]
pub struct Exti18Id {} // IRQ 42
#[doc(hidden)]
pub struct Exti19Id {} // IRQ 62
#[doc(hidden)]
pub struct Exti20Id {} // IRQ 76
#[doc(hidden)]
pub struct Exti21Id {} // IRQ 2
#[doc(hidden)]
pub struct Exti22Id {} // IRQ 3
#[doc(hidden)]
pub struct Tim6Id {} // IRQ 54
#[doc(hidden)]
pub struct Tim7Id {} // IRQ 55
#[doc(hidden)]
pub struct Tim2Id {} // IRQ 28
#[doc(hidden)]
pub struct Tim3Id {} // IRQ 29
#[doc(hidden)]
pub struct Tim4Id {} // IRQ 30
#[doc(hidden)]
pub struct Tim5Id {} // IRQ 50
#[doc(hidden)]
pub struct Tim9Id {} // IRQ 24
#[doc(hidden)]
pub struct Tim10Id {} // IRQ 25
#[doc(hidden)]
pub struct Tim11Id {} // IRQ 26
#[doc(hidden)]
pub struct Tim12Id {} // IRQ 43
#[doc(hidden)]
pub struct Tim13Id {} // IRQ 44
#[doc(hidden)]
pub struct Tim14Id {} // IRQ 45
#[doc(hidden)]
pub struct Tim1BrkId {} // IRQ 24
#[doc(hidden)]
pub struct Tim1UpId {} // IRQ 25
#[doc(hidden)]
pub struct Tim1TrgComId {} // IRQ 26
#[doc(hidden)]
pub struct Tim1CcId {} // IRQ 27
#[doc(hidden)]
pub struct Tim8BrkId {} // IRQ 43
#[doc(hidden)]
pub struct Tim8UpId {} // IRQ 44
#[doc(hidden)]
pub struct Tim8TrgComId {} // IRQ 45
#[doc(hidden)]
pub struct Tim8CcId {} // IRQ 46
#[doc(hidden)]
pub struct Adc1Id {} // IRQ 18
#[doc(hidden)]
pub struct Adc2Id {} // IRQ 18
#[doc(hidden)]
pub struct Adc3Id {} // IRQ 18
#[doc(hidden)]
pub struct Spi1Id {} // IRQ 35
#[doc(hidden)]
pub struct Spi2Id {} // IRQ 36
#[doc(hidden)]
pub struct Spi3Id {} // IRQ 51
#[doc(hidden)]
pub struct Spi4Id {} // IRQ 84
#[doc(hidden)]
pub struct Spi5Id {} // IRQ 85
#[doc(hidden)]
pub struct Spi6Id {} // IRQ 86
#[doc(hidden)]
pub struct I2c3EvId {} // IRQ 72
#[doc(hidden)]
pub struct I2c3ErId {} // IRQ 73
#[doc(hidden)]
pub struct I2c2EvId {} // IRQ 33
#[doc(hidden)]
pub struct I2c2ErId {} // IRQ 34
#[doc(hidden)]
pub struct I2c1EvId {} // IRQ 31
#[doc(hidden)]
pub struct I2c1ErId {} // IRQ 32
#[doc(hidden)]
pub struct Usart1Id {} // IRQ 37
#[doc(hidden)]
pub struct Usart2Id {} // IRQ 38
#[doc(hidden)]
pub struct Usart3Id {} // IRQ 39
#[doc(hidden)]
pub struct Uart4Id {} // IRQ 52
#[doc(hidden)]
pub struct Uart5Id {} // IRQ 53
#[doc(hidden)]
pub struct Usart6Id {} // IRQ 71
#[doc(hidden)]
pub struct Uart7Id {} // IRQ 82
#[doc(hidden)]
pub struct Uart8Id {} // IRQ 83
#[doc(hidden)]
pub struct Dma1Stream0Id {} // IRQ 11
#[doc(hidden)]
pub struct Dma1Stream1Id {} // IRQ 12
#[doc(hidden)]
pub struct Dma1Stream2Id {} // IRQ 13
#[doc(hidden)]
pub struct Dma1Stream3Id {} // IRQ 14
#[doc(hidden)]
pub struct Dma1Stream4Id {} // IRQ 15
#[doc(hidden)]
pub struct Dma1Stream5Id {} // IRQ 16
#[doc(hidden)]
pub struct Dma1Stream6Id {} // IRQ 17
#[doc(hidden)]
pub struct Dma1Stream7Id {} // IRQ 47
#[doc(hidden)]
pub struct Dma2Stream0Id {} // IRQ 56
#[doc(hidden)]
pub struct Dma2Stream1Id {} // IRQ 57
#[doc(hidden)]
pub struct Dma2Stream2Id {} // IRQ 58
#[doc(hidden)]
pub struct Dma2Stream3Id {} // IRQ 59
#[doc(hidden)]
pub struct Dma2Stream4Id {} // IRQ 60
#[doc(hidden)]
pub struct Dma2Stream5Id {} // IRQ 68
#[doc(hidden)]
pub struct Dma2Stream6Id {} // IRQ 69
#[doc(hidden)]
pub struct Dma2Stream7Id {} // IRQ 70

pub fn set_handler(index: usize, handler: Option<Handler>) {
  unsafe { 
     assert!(R_INTERRUPT_HANDLERS[index].is_some() != handler.is_some());
     R_INTERRUPT_HANDLERS[index] = handler
  };
}

use super::nvic::{NVIC, Iser, Icer, Ispr, Icpr, Stir};
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Irq<T>(usize, T);
impl<T> Irq<T> {
   pub fn index(&self) -> usize { self.0 }

   pub fn is_enabled(&self) -> bool { NVIC.iser((self.0 >> 5)).setena((self.0 & 0b11111)) != 0 }

   pub fn set_enabled(&self, value: bool) {
      if value {
         assert!(self.handler().is_some());
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
       NVIC.iabr((self.0 >> 5)).active(self.0 & 0b11111) != 0
   }

   pub fn priority(&self) -> u8 {
       NVIC.ipr((self.0 >> 4)).pri(self.0 & 0b1111).into()
   }

   pub fn set_priority(&self, value: u8) {
       NVIC.with_ipr((self.0 >> 4), |r| r.set_pri(self.0 & 0b1111, value));
   }

   pub fn trigger_interrupt(&self) {
       NVIC.set_stir(Stir(0).set_intid(self.0));
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

impl RegisterHandler for IrqWwdg {
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

impl RegisterHandler for IrqExti0 {
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

impl RegisterHandler for IrqExti1 {
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

impl RegisterHandler for IrqExti2 {
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

impl RegisterHandler for IrqExti3 {
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

impl RegisterHandler for IrqExti4 {
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

impl RegisterHandler for IrqExti5 {
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

impl RegisterHandler for IrqExti6 {
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

impl RegisterHandler for IrqExti7 {
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

impl RegisterHandler for IrqExti8 {
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

impl RegisterHandler for IrqExti9 {
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

impl RegisterHandler for IrqExti10 {
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

impl RegisterHandler for IrqExti11 {
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

impl RegisterHandler for IrqExti12 {
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

impl RegisterHandler for IrqExti13 {
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

impl RegisterHandler for IrqExti14 {
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

impl RegisterHandler for IrqExti15 {
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

impl RegisterHandler for IrqExti16 {
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

impl RegisterHandler for IrqExti17 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(41, Some(wrapper::<F>));
       IrqGuard::new(41)
   }
}

impl RegisterHandler for IrqExti18 {
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

impl RegisterHandler for IrqExti19 {
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

impl RegisterHandler for IrqExti20 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(76, Some(wrapper::<F>));
       IrqGuard::new(76)
   }
}

impl RegisterHandler for IrqExti21 {
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

impl RegisterHandler for IrqExti22 {
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

impl RegisterHandler for IrqTim6 {
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

impl RegisterHandler for IrqTim7 {
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

impl RegisterHandler for IrqTim2 {
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

impl RegisterHandler for IrqTim3 {
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

impl RegisterHandler for IrqTim4 {
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

impl RegisterHandler for IrqTim5 {
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

impl RegisterHandler for IrqTim9 {
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

impl RegisterHandler for IrqTim10 {
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

impl RegisterHandler for IrqTim11 {
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

impl RegisterHandler for IrqTim12 {
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

impl RegisterHandler for IrqTim13 {
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

impl RegisterHandler for IrqTim14 {
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

impl RegisterHandler for IrqTim1Brk {
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

impl RegisterHandler for IrqTim1Up {
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

impl RegisterHandler for IrqTim1TrgCom {
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

impl RegisterHandler for IrqTim1Cc {
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

impl RegisterHandler for IrqTim8Brk {
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

impl RegisterHandler for IrqTim8Up {
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

impl RegisterHandler for IrqTim8TrgCom {
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

impl RegisterHandler for IrqTim8Cc {
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

impl RegisterHandler for IrqAdc1 {
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

impl RegisterHandler for IrqAdc2 {
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

impl RegisterHandler for IrqAdc3 {
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

impl RegisterHandler for IrqSpi1 {
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

impl RegisterHandler for IrqSpi2 {
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

impl RegisterHandler for IrqSpi3 {
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

impl RegisterHandler for IrqSpi4 {
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

impl RegisterHandler for IrqSpi5 {
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

impl RegisterHandler for IrqSpi6 {
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

impl RegisterHandler for IrqI2c3Ev {
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

impl RegisterHandler for IrqI2c3Er {
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

impl RegisterHandler for IrqI2c2Ev {
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

impl RegisterHandler for IrqI2c2Er {
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

impl RegisterHandler for IrqI2c1Ev {
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

impl RegisterHandler for IrqI2c1Er {
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

impl RegisterHandler for IrqUsart1 {
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

impl RegisterHandler for IrqUsart2 {
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

impl RegisterHandler for IrqUsart3 {
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

impl RegisterHandler for IrqUart4 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(52, Some(wrapper::<F>));
       IrqGuard::new(52)
   }
}

impl RegisterHandler for IrqUart5 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(53, Some(wrapper::<F>));
       IrqGuard::new(53)
   }
}

impl RegisterHandler for IrqUsart6 {
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

impl RegisterHandler for IrqUart7 {
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

impl RegisterHandler for IrqUart8 {
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

impl RegisterHandler for IrqDma1Stream0 {
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

impl RegisterHandler for IrqDma1Stream1 {
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

impl RegisterHandler for IrqDma1Stream2 {
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

impl RegisterHandler for IrqDma1Stream3 {
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

impl RegisterHandler for IrqDma1Stream4 {
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

impl RegisterHandler for IrqDma1Stream5 {
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

impl RegisterHandler for IrqDma1Stream6 {
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

impl RegisterHandler for IrqDma1Stream7 {
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

impl RegisterHandler for IrqDma2Stream0 {
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

impl RegisterHandler for IrqDma2Stream1 {
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

impl RegisterHandler for IrqDma2Stream2 {
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

impl RegisterHandler for IrqDma2Stream3 {
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

impl RegisterHandler for IrqDma2Stream4 {
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

impl RegisterHandler for IrqDma2Stream5 {
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

impl RegisterHandler for IrqDma2Stream6 {
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

impl RegisterHandler for IrqDma2Stream7 {
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

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 91] = [
   None,                          // IRQ 0: Window Watchdog interrupt
   None,                          // IRQ 1: PVD through EXTI line detection interrupt
   None,
   None,
   None,
   None,                          // IRQ 5: RCC global interrupt
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
   None,                          // IRQ 18: ADC3 global interrupts
   None,
   None,
   None,
   None,
   None,
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
   None,
   None,
   None,
   None,                          // IRQ 43: TIM8 Break interrupt and TIM12 global interrupt
   None,                          // IRQ 44: TIM8 Update interrupt and TIM13 global interrupt
   None,                          // IRQ 45: TIM8 Trigger and Commutation interrupts and TIM14 global interrupt
   None,                          // IRQ 46: TIM8 Capture Compare interrupt
   None,
   None,
   None,
   None,                          // IRQ 50: TIM5 global interrupt
   None,                          // IRQ 51: SPI3 global interrupt
   None,                          // IRQ 52: UART4 global interrupt
   None,                          // IRQ 53: UART5 global interrupt
   None,                          // IRQ 54: TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt
   None,                          // IRQ 55: TIM7 global interrupt
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 61: Ethernet global interrupt
   None,                          // IRQ 62: Ethernet Wakeup through EXTI line interrupt
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
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

