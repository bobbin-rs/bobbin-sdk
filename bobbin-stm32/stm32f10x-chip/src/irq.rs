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
pub const IRQ_TIM6_DAC: IrqTim6Dac = Irq(54, Tim6DacId {});
pub const IRQ_TIM7: IrqTim7 = Irq(55, Tim7Id {});
pub const IRQ_TIM2: IrqTim2 = Irq(28, Tim2Id {});
pub const IRQ_TIM3: IrqTim3 = Irq(29, Tim3Id {});
pub const IRQ_TIM4: IrqTim4 = Irq(30, Tim4Id {});
pub const IRQ_TIM1_BRK: IrqTim1Brk = Irq(24, Tim1BrkId {});
pub const IRQ_TIM1_UP: IrqTim1Up = Irq(25, Tim1UpId {});
pub const IRQ_TIM1_TRG_COM: IrqTim1TrgCom = Irq(26, Tim1TrgComId {});
pub const IRQ_TIM1_CC: IrqTim1Cc = Irq(27, Tim1CcId {});
pub const IRQ_ADC: IrqAdc = Irq(18, AdcId {});
pub const IRQ_ADC2: IrqAdc2 = Irq(18, Adc2Id {});
pub const IRQ_ADC3: IrqAdc3 = Irq(18, Adc3Id {});
pub const IRQ_SPI1_IRQ: IrqSpi1Irq = Irq(35, Spi1IrqId {});
pub const IRQ_SPI2_IRQ: IrqSpi2Irq = Irq(36, Spi2IrqId {});
pub const IRQ_SPI3_IRQ: IrqSpi3Irq = Irq(51, Spi3IrqId {});
pub const IRQ_I2C2_EV: IrqI2c2Ev = Irq(33, I2c2EvId {});
pub const IRQ_I2C2_ER: IrqI2c2Er = Irq(34, I2c2ErId {});
pub const IRQ_I2C1_EV: IrqI2c1Ev = Irq(31, I2c1EvId {});
pub const IRQ_I2C1_ER: IrqI2c1Er = Irq(32, I2c1ErId {});
pub const IRQ_USART1_IRQ: IrqUsart1Irq = Irq(37, Usart1IrqId {});
pub const IRQ_USART2_IRQ: IrqUsart2Irq = Irq(38, Usart2IrqId {});
pub const IRQ_USART3_IRQ: IrqUsart3Irq = Irq(39, Usart3IrqId {});
pub const IRQ_DMA1_CH1: IrqDma1Ch1 = Irq(11, Dma1Ch1Id {});
pub const IRQ_DMA1_CH2: IrqDma1Ch2 = Irq(12, Dma1Ch2Id {});
pub const IRQ_DMA1_CH3: IrqDma1Ch3 = Irq(13, Dma1Ch3Id {});
pub const IRQ_DMA1_CH4: IrqDma1Ch4 = Irq(14, Dma1Ch4Id {});
pub const IRQ_DMA1_CH5: IrqDma1Ch5 = Irq(15, Dma1Ch5Id {});
pub const IRQ_DMA1_CH6: IrqDma1Ch6 = Irq(16, Dma1Ch6Id {});
pub const IRQ_DMA1_CH7: IrqDma1Ch7 = Irq(17, Dma1Ch7Id {});
pub const IRQ_DMA2_CH1: IrqDma2Ch1 = Irq(56, Dma2Ch1Id {});
pub const IRQ_DMA2_CH2: IrqDma2Ch2 = Irq(57, Dma2Ch2Id {});
pub const IRQ_DMA2_CH3: IrqDma2Ch3 = Irq(58, Dma2Ch3Id {});
pub const IRQ_DMA2_CH4: IrqDma2Ch4 = Irq(59, Dma2Ch4Id {});
pub const IRQ_DMA2_CH5: IrqDma2Ch5 = Irq(59, Dma2Ch5Id {});

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
pub type IrqTim6Dac = Irq<Tim6DacId>;
pub type IrqTim7 = Irq<Tim7Id>;
pub type IrqTim2 = Irq<Tim2Id>;
pub type IrqTim3 = Irq<Tim3Id>;
pub type IrqTim4 = Irq<Tim4Id>;
pub type IrqTim1Brk = Irq<Tim1BrkId>;
pub type IrqTim1Up = Irq<Tim1UpId>;
pub type IrqTim1TrgCom = Irq<Tim1TrgComId>;
pub type IrqTim1Cc = Irq<Tim1CcId>;
pub type IrqAdc = Irq<AdcId>;
pub type IrqAdc2 = Irq<Adc2Id>;
pub type IrqAdc3 = Irq<Adc3Id>;
pub type IrqSpi1Irq = Irq<Spi1IrqId>;
pub type IrqSpi2Irq = Irq<Spi2IrqId>;
pub type IrqSpi3Irq = Irq<Spi3IrqId>;
pub type IrqI2c2Ev = Irq<I2c2EvId>;
pub type IrqI2c2Er = Irq<I2c2ErId>;
pub type IrqI2c1Ev = Irq<I2c1EvId>;
pub type IrqI2c1Er = Irq<I2c1ErId>;
pub type IrqUsart1Irq = Irq<Usart1IrqId>;
pub type IrqUsart2Irq = Irq<Usart2IrqId>;
pub type IrqUsart3Irq = Irq<Usart3IrqId>;
pub type IrqDma1Ch1 = Irq<Dma1Ch1Id>;
pub type IrqDma1Ch2 = Irq<Dma1Ch2Id>;
pub type IrqDma1Ch3 = Irq<Dma1Ch3Id>;
pub type IrqDma1Ch4 = Irq<Dma1Ch4Id>;
pub type IrqDma1Ch5 = Irq<Dma1Ch5Id>;
pub type IrqDma1Ch6 = Irq<Dma1Ch6Id>;
pub type IrqDma1Ch7 = Irq<Dma1Ch7Id>;
pub type IrqDma2Ch1 = Irq<Dma2Ch1Id>;
pub type IrqDma2Ch2 = Irq<Dma2Ch2Id>;
pub type IrqDma2Ch3 = Irq<Dma2Ch3Id>;
pub type IrqDma2Ch4 = Irq<Dma2Ch4Id>;
pub type IrqDma2Ch5 = Irq<Dma2Ch5Id>;

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
pub struct Tim6DacId {} // IRQ 54
#[doc(hidden)]
pub struct Tim7Id {} // IRQ 55
#[doc(hidden)]
pub struct Tim2Id {} // IRQ 28
#[doc(hidden)]
pub struct Tim3Id {} // IRQ 29
#[doc(hidden)]
pub struct Tim4Id {} // IRQ 30
#[doc(hidden)]
pub struct Tim1BrkId {} // IRQ 24
#[doc(hidden)]
pub struct Tim1UpId {} // IRQ 25
#[doc(hidden)]
pub struct Tim1TrgComId {} // IRQ 26
#[doc(hidden)]
pub struct Tim1CcId {} // IRQ 27
#[doc(hidden)]
pub struct AdcId {} // IRQ 18
#[doc(hidden)]
pub struct Adc2Id {} // IRQ 18
#[doc(hidden)]
pub struct Adc3Id {} // IRQ 18
#[doc(hidden)]
pub struct Spi1IrqId {} // IRQ 35
#[doc(hidden)]
pub struct Spi2IrqId {} // IRQ 36
#[doc(hidden)]
pub struct Spi3IrqId {} // IRQ 51
#[doc(hidden)]
pub struct I2c2EvId {} // IRQ 33
#[doc(hidden)]
pub struct I2c2ErId {} // IRQ 34
#[doc(hidden)]
pub struct I2c1EvId {} // IRQ 31
#[doc(hidden)]
pub struct I2c1ErId {} // IRQ 32
#[doc(hidden)]
pub struct Usart1IrqId {} // IRQ 37
#[doc(hidden)]
pub struct Usart2IrqId {} // IRQ 38
#[doc(hidden)]
pub struct Usart3IrqId {} // IRQ 39
#[doc(hidden)]
pub struct Dma1Ch1Id {} // IRQ 11
#[doc(hidden)]
pub struct Dma1Ch2Id {} // IRQ 12
#[doc(hidden)]
pub struct Dma1Ch3Id {} // IRQ 13
#[doc(hidden)]
pub struct Dma1Ch4Id {} // IRQ 14
#[doc(hidden)]
pub struct Dma1Ch5Id {} // IRQ 15
#[doc(hidden)]
pub struct Dma1Ch6Id {} // IRQ 16
#[doc(hidden)]
pub struct Dma1Ch7Id {} // IRQ 17
#[doc(hidden)]
pub struct Dma2Ch1Id {} // IRQ 56
#[doc(hidden)]
pub struct Dma2Ch2Id {} // IRQ 57
#[doc(hidden)]
pub struct Dma2Ch3Id {} // IRQ 58
#[doc(hidden)]
pub struct Dma2Ch4Id {} // IRQ 59
#[doc(hidden)]
pub struct Dma2Ch5Id {} // IRQ 59

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

impl RegisterHandler for IrqTim6Dac {
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

impl RegisterHandler for IrqAdc {
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

impl RegisterHandler for IrqSpi1Irq {
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

impl RegisterHandler for IrqSpi2Irq {
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

impl RegisterHandler for IrqSpi3Irq {
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

impl RegisterHandler for IrqUsart1Irq {
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

impl RegisterHandler for IrqUsart2Irq {
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

impl RegisterHandler for IrqUsart3Irq {
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

impl RegisterHandler for IrqDma1Ch1 {
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

impl RegisterHandler for IrqDma1Ch2 {
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

impl RegisterHandler for IrqDma1Ch3 {
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

impl RegisterHandler for IrqDma1Ch4 {
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

impl RegisterHandler for IrqDma1Ch5 {
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

impl RegisterHandler for IrqDma1Ch6 {
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

impl RegisterHandler for IrqDma1Ch7 {
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

impl RegisterHandler for IrqDma2Ch1 {
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

impl RegisterHandler for IrqDma2Ch2 {
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

impl RegisterHandler for IrqDma2Ch3 {
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

impl RegisterHandler for IrqDma2Ch4 {
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

impl RegisterHandler for IrqDma2Ch5 {
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

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 68] = [
   None,                          // IRQ 0: Window Watchdog interrupt
   None,                          // IRQ 1: PVD through EXTI line detection interrupt
   None,
   None,
   None,                          // IRQ 4: Flash global interrupt
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
   None,                          // IRQ 24: TIM1 Break interrupt
   None,                          // IRQ 25: TIM1 Update interrupt
   None,                          // IRQ 26: TIM1 Trigger and Commutation interrupt
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
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 48: FSMC global interrupt
   None,
   None,
   None,                          // IRQ 51: SPI3 global interrupt
   None,
   None,
   None,                          // IRQ 54: TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt
   None,                          // IRQ 55: TIM7 global interrupt
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
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 68] = [None; 68];

