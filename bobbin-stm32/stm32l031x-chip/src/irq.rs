use ::core::marker::PhantomData;
pub type Handler = extern "C" fn();

pub const IRQ_WWDG: IrqWwdg = Irq(0, WwdgId {});
pub const IRQ_RTC: IrqRtc = Irq(2, RtcId {});
pub const IRQ_LPTIM1: IrqLptim1 = Irq(13, Lptim1Id {});
pub const IRQ_EXTI0: IrqExti0 = Irq(5, Exti0Id {});
pub const IRQ_EXTI1: IrqExti1 = Irq(5, Exti1Id {});
pub const IRQ_EXTI2: IrqExti2 = Irq(6, Exti2Id {});
pub const IRQ_EXTI3: IrqExti3 = Irq(6, Exti3Id {});
pub const IRQ_EXTI4: IrqExti4 = Irq(7, Exti4Id {});
pub const IRQ_EXTI5: IrqExti5 = Irq(7, Exti5Id {});
pub const IRQ_EXTI6: IrqExti6 = Irq(7, Exti6Id {});
pub const IRQ_EXTI7: IrqExti7 = Irq(7, Exti7Id {});
pub const IRQ_EXTI8: IrqExti8 = Irq(7, Exti8Id {});
pub const IRQ_EXTI9: IrqExti9 = Irq(7, Exti9Id {});
pub const IRQ_EXTI10: IrqExti10 = Irq(7, Exti10Id {});
pub const IRQ_EXTI11: IrqExti11 = Irq(7, Exti11Id {});
pub const IRQ_EXTI12: IrqExti12 = Irq(7, Exti12Id {});
pub const IRQ_EXTI13: IrqExti13 = Irq(7, Exti13Id {});
pub const IRQ_EXTI14: IrqExti14 = Irq(7, Exti14Id {});
pub const IRQ_EXTI15: IrqExti15 = Irq(7, Exti15Id {});
pub const IRQ_EXTI16: IrqExti16 = Irq(1, Exti16Id {});
pub const IRQ_EXTI17: IrqExti17 = Irq(2, Exti17Id {});
pub const IRQ_EXTI19: IrqExti19 = Irq(2, Exti19Id {});
pub const IRQ_EXTI20: IrqExti20 = Irq(2, Exti20Id {});
pub const IRQ_EXTI21: IrqExti21 = Irq(12, Exti21Id {});
pub const IRQ_EXTI22: IrqExti22 = Irq(12, Exti22Id {});
pub const IRQ_EXTI23: IrqExti23 = Irq(23, Exti23Id {});
pub const IRQ_EXTI24: IrqExti24 = Irq(24, Exti24Id {});
pub const IRQ_EXTI25: IrqExti25 = Irq(27, Exti25Id {});
pub const IRQ_EXTI26: IrqExti26 = Irq(28, Exti26Id {});
pub const IRQ_EXTI28: IrqExti28 = Irq(29, Exti28Id {});
pub const IRQ_EXTI29: IrqExti29 = Irq(13, Exti29Id {});
pub const IRQ_DMA1_CH1: IrqDma1Ch1 = Irq(9, Dma1Ch1Id {});
pub const IRQ_DMA1_CH2: IrqDma1Ch2 = Irq(10, Dma1Ch2Id {});
pub const IRQ_DMA1_CH3: IrqDma1Ch3 = Irq(10, Dma1Ch3Id {});
pub const IRQ_DMA1_CH4: IrqDma1Ch4 = Irq(11, Dma1Ch4Id {});
pub const IRQ_DMA1_CH5: IrqDma1Ch5 = Irq(11, Dma1Ch5Id {});
pub const IRQ_DMA1_CH6: IrqDma1Ch6 = Irq(11, Dma1Ch6Id {});
pub const IRQ_DMA1_CH7: IrqDma1Ch7 = Irq(11, Dma1Ch7Id {});
pub const IRQ_I2C1: IrqI2c1 = Irq(23, I2c1Id {});
pub const IRQ_TIM2: IrqTim2 = Irq(15, Tim2Id {});
pub const IRQ_TIM21: IrqTim21 = Irq(20, Tim21Id {});
pub const IRQ_TIM22: IrqTim22 = Irq(22, Tim22Id {});
pub const IRQ_USART2: IrqUsart2 = Irq(28, Usart2Id {});
pub const IRQ_LPUART1: IrqLpuart1 = Irq(29, Lpuart1Id {});
pub const IRQ_SPI1: IrqSpi1 = Irq(25, Spi1Id {});

pub type IrqWwdg = Irq<WwdgId>;
pub type IrqRtc = Irq<RtcId>;
pub type IrqLptim1 = Irq<Lptim1Id>;
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
pub type IrqExti19 = Irq<Exti19Id>;
pub type IrqExti20 = Irq<Exti20Id>;
pub type IrqExti21 = Irq<Exti21Id>;
pub type IrqExti22 = Irq<Exti22Id>;
pub type IrqExti23 = Irq<Exti23Id>;
pub type IrqExti24 = Irq<Exti24Id>;
pub type IrqExti25 = Irq<Exti25Id>;
pub type IrqExti26 = Irq<Exti26Id>;
pub type IrqExti28 = Irq<Exti28Id>;
pub type IrqExti29 = Irq<Exti29Id>;
pub type IrqDma1Ch1 = Irq<Dma1Ch1Id>;
pub type IrqDma1Ch2 = Irq<Dma1Ch2Id>;
pub type IrqDma1Ch3 = Irq<Dma1Ch3Id>;
pub type IrqDma1Ch4 = Irq<Dma1Ch4Id>;
pub type IrqDma1Ch5 = Irq<Dma1Ch5Id>;
pub type IrqDma1Ch6 = Irq<Dma1Ch6Id>;
pub type IrqDma1Ch7 = Irq<Dma1Ch7Id>;
pub type IrqI2c1 = Irq<I2c1Id>;
pub type IrqTim2 = Irq<Tim2Id>;
pub type IrqTim21 = Irq<Tim21Id>;
pub type IrqTim22 = Irq<Tim22Id>;
pub type IrqUsart2 = Irq<Usart2Id>;
pub type IrqLpuart1 = Irq<Lpuart1Id>;
pub type IrqSpi1 = Irq<Spi1Id>;

pub struct WwdgId {} // IRQ 0
pub struct RtcId {} // IRQ 2
pub struct Lptim1Id {} // IRQ 13
pub struct Exti0Id {} // IRQ 5
pub struct Exti1Id {} // IRQ 5
pub struct Exti2Id {} // IRQ 6
pub struct Exti3Id {} // IRQ 6
pub struct Exti4Id {} // IRQ 7
pub struct Exti5Id {} // IRQ 7
pub struct Exti6Id {} // IRQ 7
pub struct Exti7Id {} // IRQ 7
pub struct Exti8Id {} // IRQ 7
pub struct Exti9Id {} // IRQ 7
pub struct Exti10Id {} // IRQ 7
pub struct Exti11Id {} // IRQ 7
pub struct Exti12Id {} // IRQ 7
pub struct Exti13Id {} // IRQ 7
pub struct Exti14Id {} // IRQ 7
pub struct Exti15Id {} // IRQ 7
pub struct Exti16Id {} // IRQ 1
pub struct Exti17Id {} // IRQ 2
pub struct Exti19Id {} // IRQ 2
pub struct Exti20Id {} // IRQ 2
pub struct Exti21Id {} // IRQ 12
pub struct Exti22Id {} // IRQ 12
pub struct Exti23Id {} // IRQ 23
pub struct Exti24Id {} // IRQ 24
pub struct Exti25Id {} // IRQ 27
pub struct Exti26Id {} // IRQ 28
pub struct Exti28Id {} // IRQ 29
pub struct Exti29Id {} // IRQ 13
pub struct Dma1Ch1Id {} // IRQ 9
pub struct Dma1Ch2Id {} // IRQ 10
pub struct Dma1Ch3Id {} // IRQ 10
pub struct Dma1Ch4Id {} // IRQ 11
pub struct Dma1Ch5Id {} // IRQ 11
pub struct Dma1Ch6Id {} // IRQ 11
pub struct Dma1Ch7Id {} // IRQ 11
pub struct I2c1Id {} // IRQ 23
pub struct Tim2Id {} // IRQ 15
pub struct Tim21Id {} // IRQ 20
pub struct Tim22Id {} // IRQ 22
pub struct Usart2Id {} // IRQ 28
pub struct Lpuart1Id {} // IRQ 29
pub struct Spi1Id {} // IRQ 25

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

impl RegisterHandler for IrqRtc {
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

impl RegisterHandler for IrqLptim1 {
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

impl RegisterHandler for IrqExti0 {
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

impl RegisterHandler for IrqExti1 {
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

impl RegisterHandler for IrqExti2 {
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

impl RegisterHandler for IrqExti3 {
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

impl RegisterHandler for IrqExti4 {
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

impl RegisterHandler for IrqExti5 {
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

impl RegisterHandler for IrqExti6 {
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

impl RegisterHandler for IrqExti7 {
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

impl RegisterHandler for IrqExti8 {
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

impl RegisterHandler for IrqExti9 {
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

impl RegisterHandler for IrqExti10 {
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

impl RegisterHandler for IrqExti11 {
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

impl RegisterHandler for IrqExti12 {
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

impl RegisterHandler for IrqExti13 {
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

impl RegisterHandler for IrqExti14 {
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

impl RegisterHandler for IrqExti15 {
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
       set_handler(2, Some(wrapper::<F>));
       IrqGuard::new(2)
   }
}

impl RegisterHandler for IrqExti19 {
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

impl RegisterHandler for IrqExti20 {
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

impl RegisterHandler for IrqExti21 {
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

impl RegisterHandler for IrqExti22 {
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

impl RegisterHandler for IrqExti23 {
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

impl RegisterHandler for IrqExti24 {
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

impl RegisterHandler for IrqExti25 {
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

impl RegisterHandler for IrqExti26 {
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

impl RegisterHandler for IrqExti28 {
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

impl RegisterHandler for IrqExti29 {
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

impl RegisterHandler for IrqDma1Ch1 {
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

impl RegisterHandler for IrqDma1Ch2 {
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

impl RegisterHandler for IrqDma1Ch3 {
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

impl RegisterHandler for IrqDma1Ch4 {
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

impl RegisterHandler for IrqDma1Ch5 {
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

impl RegisterHandler for IrqDma1Ch6 {
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

impl RegisterHandler for IrqDma1Ch7 {
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

impl RegisterHandler for IrqI2c1 {
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

impl RegisterHandler for IrqTim2 {
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

impl RegisterHandler for IrqTim21 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(20, Some(wrapper::<F>));
       IrqGuard::new(20)
   }
}

impl RegisterHandler for IrqTim22 {
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

impl RegisterHandler for IrqUsart2 {
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

impl RegisterHandler for IrqLpuart1 {
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

impl RegisterHandler for IrqSpi1 {
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

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 30] = [
   None,                          // IRQ 0: Window Watchdog interrupt
   None,                          // IRQ 1: PVD through EXTI line detection
   None,                          // IRQ 2: RTC global interrupt
   None,                          // IRQ 3: Flash global interrupt
   None,                          // IRQ 4: RCC global interrupt
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 13: LPTIMER1 interrupt through EXTI29
   None,
   None,                          // IRQ 15: TIM2 global interrupt
   None,
   None,
   None,
   None,
   None,                          // IRQ 20: TIMER21 global interrupt
   None,
   None,                          // IRQ 22: TIMER22 global interrupt
   None,                          // IRQ 23: I2C1 global interrupt
   None,
   None,                          // IRQ 25: SPI1_global_interrupt
   None,
   None,
   None,                          // IRQ 28: USART2 global interrupt through EXTI26
   None,                          // IRQ 29: LPUART1 global interrupt through EXTI28
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 30] = [None; 30];

