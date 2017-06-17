use ::core::marker::PhantomData;
pub type Handler = extern "C" fn();

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
pub const IRQ_USART6: IrqUsart6 = Irq(71, Usart6Id {});
pub const IRQ_UART4_IRQ: IrqUart4Irq = Irq(52, Uart4IrqId {});
pub const IRQ_UART5_IRQ: IrqUart5Irq = Irq(53, Uart5IrqId {});
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
pub type IrqUsart6 = Irq<Usart6Id>;
pub type IrqUart4Irq = Irq<Uart4IrqId>;
pub type IrqUart5Irq = Irq<Uart5IrqId>;
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

pub struct Tim6Id {} // IRQ 54
pub struct Tim7Id {} // IRQ 55
pub struct Tim2Id {} // IRQ 28
pub struct Tim3Id {} // IRQ 29
pub struct Tim4Id {} // IRQ 30
pub struct Tim5Id {} // IRQ 50
pub struct Tim9Id {} // IRQ 24
pub struct Tim10Id {} // IRQ 25
pub struct Tim11Id {} // IRQ 26
pub struct Tim12Id {} // IRQ 43
pub struct Tim13Id {} // IRQ 44
pub struct Tim14Id {} // IRQ 45
pub struct Tim1BrkId {} // IRQ 24
pub struct Tim1UpId {} // IRQ 25
pub struct Tim1TrgComId {} // IRQ 26
pub struct Tim1CcId {} // IRQ 27
pub struct Tim8BrkId {} // IRQ 43
pub struct Tim8UpId {} // IRQ 44
pub struct Tim8TrgComId {} // IRQ 45
pub struct Tim8CcId {} // IRQ 46
pub struct Adc1Id {} // IRQ 18
pub struct Adc2Id {} // IRQ 18
pub struct Adc3Id {} // IRQ 18
pub struct Spi1Id {} // IRQ 35
pub struct Spi2Id {} // IRQ 36
pub struct Spi3Id {} // IRQ 51
pub struct Spi4Id {} // IRQ 84
pub struct Spi5Id {} // IRQ 85
pub struct Spi6Id {} // IRQ 86
pub struct I2c3EvId {} // IRQ 72
pub struct I2c3ErId {} // IRQ 73
pub struct I2c2EvId {} // IRQ 33
pub struct I2c2ErId {} // IRQ 34
pub struct I2c1EvId {} // IRQ 31
pub struct I2c1ErId {} // IRQ 32
pub struct Usart1Id {} // IRQ 37
pub struct Usart2Id {} // IRQ 38
pub struct Usart3Id {} // IRQ 39
pub struct Usart6Id {} // IRQ 71
pub struct Uart4IrqId {} // IRQ 52
pub struct Uart5IrqId {} // IRQ 53
pub struct Dma1Stream0Id {} // IRQ 11
pub struct Dma1Stream1Id {} // IRQ 12
pub struct Dma1Stream2Id {} // IRQ 13
pub struct Dma1Stream3Id {} // IRQ 14
pub struct Dma1Stream4Id {} // IRQ 15
pub struct Dma1Stream5Id {} // IRQ 16
pub struct Dma1Stream6Id {} // IRQ 17
pub struct Dma1Stream7Id {} // IRQ 47
pub struct Dma2Stream0Id {} // IRQ 56
pub struct Dma2Stream1Id {} // IRQ 57
pub struct Dma2Stream2Id {} // IRQ 58
pub struct Dma2Stream3Id {} // IRQ 59
pub struct Dma2Stream4Id {} // IRQ 60
pub struct Dma2Stream5Id {} // IRQ 68
pub struct Dma2Stream6Id {} // IRQ 69
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

impl RegisterHandler for IrqUart4Irq {
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

impl RegisterHandler for IrqUart5Irq {
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
   None,
   None,
   None,
   None,                          // IRQ 50: TIM5 global interrupt
   None,                          // IRQ 51: SPI3 global interrupt
   None,                          // IRQ 52: UART4 global interrupt
   None,                          // IRQ 53: UART5 global interrupt
   None,                          // IRQ 54: TIM6 global interrupt
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
   None,
   None,
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

