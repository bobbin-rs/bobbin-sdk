use ::core::marker::PhantomData;
pub type Handler = extern "C" fn();

pub const IRQ_USART1_EXTI25: IrqUsart1Exti25 = Irq(37, Usart1Exti25Id {});
pub const IRQ_USART2_EXTI26: IrqUsart2Exti26 = Irq(38, Usart2Exti26Id {});
pub const IRQ_USART3_EXTI28: IrqUsart3Exti28 = Irq(39, Usart3Exti28Id {});
pub const IRQ_UART4_EXTI34: IrqUart4Exti34 = Irq(52, Uart4Exti34Id {});
pub const IRQ_UART5_EXTI35: IrqUart5Exti35 = Irq(53, Uart5Exti35Id {});
pub const IRQ_I2C1_EV: IrqI2c1Ev = Irq(31, I2c1EvId {});
pub const IRQ_I2C1_ER: IrqI2c1Er = Irq(32, I2c1ErId {});
pub const IRQ_I2C2_EV: IrqI2c2Ev = Irq(33, I2c2EvId {});
pub const IRQ_I2C2_ER: IrqI2c2Er = Irq(34, I2c2ErId {});
pub const IRQ_I2C3_EV: IrqI2c3Ev = Irq(72, I2c3EvId {});
pub const IRQ_I2C3_ER: IrqI2c3Er = Irq(73, I2c3ErId {});
pub const IRQ_SPI1: IrqSpi1 = Irq(35, Spi1Id {});
pub const IRQ_SPI2: IrqSpi2 = Irq(36, Spi2Id {});
pub const IRQ_SPI3: IrqSpi3 = Irq(51, Spi3Id {});
pub const IRQ_ADC1: IrqAdc1 = Irq(18, Adc1Id {});
pub const IRQ_ADC2: IrqAdc2 = Irq(18, Adc2Id {});
pub const IRQ_ADC3: IrqAdc3 = Irq(47, Adc3Id {});
pub const IRQ_ADC4: IrqAdc4 = Irq(61, Adc4Id {});
pub const IRQ_TIM6_DACUNDER: IrqTim6Dacunder = Irq(54, Tim6DacunderId {});
pub const IRQ_TIM6: IrqTim6 = Irq(54, Tim6Id {});
pub const IRQ_TIM7: IrqTim7 = Irq(55, Tim7Id {});
pub const IRQ_TIM2: IrqTim2 = Irq(28, Tim2Id {});
pub const IRQ_TIM3: IrqTim3 = Irq(29, Tim3Id {});
pub const IRQ_TIM4: IrqTim4 = Irq(30, Tim4Id {});
pub const IRQ_TIM15: IrqTim15 = Irq(24, Tim15Id {});
pub const IRQ_TIM16: IrqTim16 = Irq(25, Tim16Id {});
pub const IRQ_TIM17: IrqTim17 = Irq(26, Tim17Id {});
pub const IRQ_TIM1_BRK: IrqTim1Brk = Irq(24, Tim1BrkId {});
pub const IRQ_TIM1_UP: IrqTim1Up = Irq(25, Tim1UpId {});
pub const IRQ_TIM1_TRG_COM: IrqTim1TrgCom = Irq(26, Tim1TrgComId {});
pub const IRQ_TIM1_CC: IrqTim1Cc = Irq(27, Tim1CcId {});
pub const IRQ_TIM8_BRK: IrqTim8Brk = Irq(43, Tim8BrkId {});
pub const IRQ_TIM8_UP: IrqTim8Up = Irq(44, Tim8UpId {});
pub const IRQ_TIM8_TRG_COM: IrqTim8TrgCom = Irq(45, Tim8TrgComId {});
pub const IRQ_TIM8_CC: IrqTim8Cc = Irq(46, Tim8CcId {});
pub const IRQ_TIM20_BRK: IrqTim20Brk = Irq(77, Tim20BrkId {});
pub const IRQ_TIM20_UP: IrqTim20Up = Irq(78, Tim20UpId {});
pub const IRQ_TIM20_TRG_COM: IrqTim20TrgCom = Irq(79, Tim20TrgComId {});
pub const IRQ_TIM20_CC: IrqTim20Cc = Irq(80, Tim20CcId {});
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
pub const IRQ_DMA2_CH5: IrqDma2Ch5 = Irq(60, Dma2Ch5Id {});

pub type IrqUsart1Exti25 = Irq<Usart1Exti25Id>;
pub type IrqUsart2Exti26 = Irq<Usart2Exti26Id>;
pub type IrqUsart3Exti28 = Irq<Usart3Exti28Id>;
pub type IrqUart4Exti34 = Irq<Uart4Exti34Id>;
pub type IrqUart5Exti35 = Irq<Uart5Exti35Id>;
pub type IrqI2c1Ev = Irq<I2c1EvId>;
pub type IrqI2c1Er = Irq<I2c1ErId>;
pub type IrqI2c2Ev = Irq<I2c2EvId>;
pub type IrqI2c2Er = Irq<I2c2ErId>;
pub type IrqI2c3Ev = Irq<I2c3EvId>;
pub type IrqI2c3Er = Irq<I2c3ErId>;
pub type IrqSpi1 = Irq<Spi1Id>;
pub type IrqSpi2 = Irq<Spi2Id>;
pub type IrqSpi3 = Irq<Spi3Id>;
pub type IrqAdc1 = Irq<Adc1Id>;
pub type IrqAdc2 = Irq<Adc2Id>;
pub type IrqAdc3 = Irq<Adc3Id>;
pub type IrqAdc4 = Irq<Adc4Id>;
pub type IrqTim6Dacunder = Irq<Tim6DacunderId>;
pub type IrqTim6 = Irq<Tim6Id>;
pub type IrqTim7 = Irq<Tim7Id>;
pub type IrqTim2 = Irq<Tim2Id>;
pub type IrqTim3 = Irq<Tim3Id>;
pub type IrqTim4 = Irq<Tim4Id>;
pub type IrqTim15 = Irq<Tim15Id>;
pub type IrqTim16 = Irq<Tim16Id>;
pub type IrqTim17 = Irq<Tim17Id>;
pub type IrqTim1Brk = Irq<Tim1BrkId>;
pub type IrqTim1Up = Irq<Tim1UpId>;
pub type IrqTim1TrgCom = Irq<Tim1TrgComId>;
pub type IrqTim1Cc = Irq<Tim1CcId>;
pub type IrqTim8Brk = Irq<Tim8BrkId>;
pub type IrqTim8Up = Irq<Tim8UpId>;
pub type IrqTim8TrgCom = Irq<Tim8TrgComId>;
pub type IrqTim8Cc = Irq<Tim8CcId>;
pub type IrqTim20Brk = Irq<Tim20BrkId>;
pub type IrqTim20Up = Irq<Tim20UpId>;
pub type IrqTim20TrgCom = Irq<Tim20TrgComId>;
pub type IrqTim20Cc = Irq<Tim20CcId>;
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

pub struct Usart1Exti25Id {} // IRQ 37
pub struct Usart2Exti26Id {} // IRQ 38
pub struct Usart3Exti28Id {} // IRQ 39
pub struct Uart4Exti34Id {} // IRQ 52
pub struct Uart5Exti35Id {} // IRQ 53
pub struct I2c1EvId {} // IRQ 31
pub struct I2c1ErId {} // IRQ 32
pub struct I2c2EvId {} // IRQ 33
pub struct I2c2ErId {} // IRQ 34
pub struct I2c3EvId {} // IRQ 72
pub struct I2c3ErId {} // IRQ 73
pub struct Spi1Id {} // IRQ 35
pub struct Spi2Id {} // IRQ 36
pub struct Spi3Id {} // IRQ 51
pub struct Adc1Id {} // IRQ 18
pub struct Adc2Id {} // IRQ 18
pub struct Adc3Id {} // IRQ 47
pub struct Adc4Id {} // IRQ 61
pub struct Tim6DacunderId {} // IRQ 54
pub struct Tim6Id {} // IRQ 54
pub struct Tim7Id {} // IRQ 55
pub struct Tim2Id {} // IRQ 28
pub struct Tim3Id {} // IRQ 29
pub struct Tim4Id {} // IRQ 30
pub struct Tim15Id {} // IRQ 24
pub struct Tim16Id {} // IRQ 25
pub struct Tim17Id {} // IRQ 26
pub struct Tim1BrkId {} // IRQ 24
pub struct Tim1UpId {} // IRQ 25
pub struct Tim1TrgComId {} // IRQ 26
pub struct Tim1CcId {} // IRQ 27
pub struct Tim8BrkId {} // IRQ 43
pub struct Tim8UpId {} // IRQ 44
pub struct Tim8TrgComId {} // IRQ 45
pub struct Tim8CcId {} // IRQ 46
pub struct Tim20BrkId {} // IRQ 77
pub struct Tim20UpId {} // IRQ 78
pub struct Tim20TrgComId {} // IRQ 79
pub struct Tim20CcId {} // IRQ 80
pub struct Dma1Ch1Id {} // IRQ 11
pub struct Dma1Ch2Id {} // IRQ 12
pub struct Dma1Ch3Id {} // IRQ 13
pub struct Dma1Ch4Id {} // IRQ 14
pub struct Dma1Ch5Id {} // IRQ 15
pub struct Dma1Ch6Id {} // IRQ 16
pub struct Dma1Ch7Id {} // IRQ 17
pub struct Dma2Ch1Id {} // IRQ 56
pub struct Dma2Ch2Id {} // IRQ 57
pub struct Dma2Ch3Id {} // IRQ 58
pub struct Dma2Ch4Id {} // IRQ 59
pub struct Dma2Ch5Id {} // IRQ 60

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

impl RegisterHandler for IrqUsart1Exti25 {
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

impl RegisterHandler for IrqUsart2Exti26 {
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

impl RegisterHandler for IrqUsart3Exti28 {
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

impl RegisterHandler for IrqUart4Exti34 {
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

impl RegisterHandler for IrqUart5Exti35 {
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
       set_handler(47, Some(wrapper::<F>));
       IrqGuard::new(47)
   }
}

impl RegisterHandler for IrqAdc4 {
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

impl RegisterHandler for IrqTim6Dacunder {
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

impl RegisterHandler for IrqTim15 {
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

impl RegisterHandler for IrqTim16 {
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

impl RegisterHandler for IrqTim17 {
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

impl RegisterHandler for IrqTim20Brk {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(77, Some(wrapper::<F>));
       IrqGuard::new(77)
   }
}

impl RegisterHandler for IrqTim20Up {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(78, Some(wrapper::<F>));
       IrqGuard::new(78)
   }
}

impl RegisterHandler for IrqTim20TrgCom {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(79, Some(wrapper::<F>));
       IrqGuard::new(79)
   }
}

impl RegisterHandler for IrqTim20Cc {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(80, Some(wrapper::<F>));
       IrqGuard::new(80)
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
       set_handler(60, Some(wrapper::<F>));
       IrqGuard::new(60)
   }
}

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 82] = [
   None,                          // IRQ 0: Window Watchdog interrupt
   None,                          // IRQ 1: PVD through EXTI line detection interrupt
   None,                          // IRQ 2: Tamper and TimeStamp interrupts
   None,                          // IRQ 3: RTC Wakeup interrupt through the EXTI line
   None,                          // IRQ 4: Flash global interrupt
   None,                          // IRQ 5: RCC global interrupt
   None,                          // IRQ 6: EXTI Line0 interrupt
   None,                          // IRQ 7: EXTI Line3 interrupt
   None,                          // IRQ 8: EXTI Line2 and Touch sensing interrupts
   None,                          // IRQ 9: EXTI Line3 interrupt
   None,                          // IRQ 10: EXTI Line4 interrupt
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 18: ADC2 global interrupt
   None,
   None,
   None,
   None,
   None,                          // IRQ 23: EXTI Line5 to Line9 interrupts
   None,                          // IRQ 24: TIM1 break interrupt
   None,                          // IRQ 25: TIM1 update interrupt
   None,                          // IRQ 26: TIM1 trigger and commutation interrupt
   None,                          // IRQ 27: TIM1 capture compare interrupt
   None,                          // IRQ 28: TIM2 global interrupt
   None,                          // IRQ 29: TIM3 global interrupt
   None,                          // IRQ 30: TIM4 global interrupt
   None,                          // IRQ 31: I2C1 event interrupt and EXTI Line23 interrupt
   None,                          // IRQ 32: I2C1 error interrupt
   None,                          // IRQ 33: I2C2 event interrupt & EXTI Line24 interrupt
   None,                          // IRQ 34: I2C2 error interrupt
   None,                          // IRQ 35: SPI1 global interrupt
   None,                          // IRQ 36: SPI2 global interrupt
   None,                          // IRQ 37: USART1 global interrupt and EXTI Line 25 interrupt
   None,                          // IRQ 38: USART2 global interrupt and EXTI Line 26 interrupt
   None,                          // IRQ 39: USART3 global interrupt and EXTI Line 28 interrupt
   None,                          // IRQ 40: EXTI Line15 to Line10 interrupts
   None,                          // IRQ 41: RTC alarm interrupt
   None,
   None,                          // IRQ 43: TIM8 break interrupt
   None,                          // IRQ 44: TIM8 update interrupt
   None,                          // IRQ 45: TIM8 Trigger and commutation interrupts
   None,                          // IRQ 46: TIM8 capture compare interrupt
   None,                          // IRQ 47: ADC3 global interrupt
   None,
   None,
   None,
   None,                          // IRQ 51: SPI3 global interrupt
   None,                          // IRQ 52: UART4 global and EXTI Line 34 interrupts
   None,                          // IRQ 53: UART5 global and EXTI Line 35 interrupts
   None,                          // IRQ 54: TIM6 global interrupt
   None,                          // IRQ 55: TIM7 global interrupt
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 61: ADC4 global interrupt
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
   None,                          // IRQ 72: I2C3 event interrupt
   None,                          // IRQ 73: I2C3 error interrupt
   None,
   None,
   None,                          // IRQ 76: USB wakeup from Suspend and EXTI Line 18
   None,                          // IRQ 77: TIM20 break interrupt
   None,                          // IRQ 78: TIM20 update interrupt
   None,                          // IRQ 79: TIM20 Trigger and commutation interrupts
   None,                          // IRQ 80: TIM20 capture compare interrupt
   None,
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 82] = [None; 82];

