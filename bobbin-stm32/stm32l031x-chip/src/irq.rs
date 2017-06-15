use ::core::marker::PhantomData;
pub type Handler = extern "C" fn();

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
pub const IRQ_SPI1: IrqSpi1 = Irq(25, Spi1Id {});

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
pub type IrqSpi1 = Irq<Spi1Id>;

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
   None,                          // IRQ 5: EXTI Line[1:0] interrupts
   None,                          // IRQ 6: EXTI Line[3:2] interrupts
   None,                          // IRQ 7: EXTI Line15 and EXTI4 interrupts
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
   None,                          // IRQ 28: USART2 global interrupt
   None,                          // IRQ 29: RNG global interrupt and LPUART1 global interrupt through
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 30] = [None; 30];

