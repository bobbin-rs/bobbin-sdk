//! Interrupts

use ::core::marker::PhantomData;
pub type Handler = extern "C" fn();

pub const IRQ_DMA0: IrqDma0 = Irq(0, Dma0Id {});
pub const IRQ_DMA1: IrqDma1 = Irq(1, Dma1Id {});
pub const IRQ_DMA2: IrqDma2 = Irq(2, Dma2Id {});
pub const IRQ_DMA3: IrqDma3 = Irq(3, Dma3Id {});
pub const IRQ_TPM0: IrqTpm0 = Irq(17, Tpm0Id {});
pub const IRQ_TPM1: IrqTpm1 = Irq(18, Tpm1Id {});
pub const IRQ_TPM2: IrqTpm2 = Irq(19, Tpm2Id {});
pub const IRQ_PIT: IrqPit = Irq(22, PitId {});
pub const IRQ_PORTA: IrqPorta = Irq(30, PortaId {});
pub const IRQ_PORTC: IrqPortc = Irq(31, PortcId {});
pub const IRQ_PORTD: IrqPortd = Irq(31, PortdId {});
pub const IRQ_UART0: IrqUart0 = Irq(12, Uart0Id {});
pub const IRQ_UART1: IrqUart1 = Irq(13, Uart1Id {});
pub const IRQ_UART2: IrqUart2 = Irq(14, Uart2Id {});

pub type IrqDma0 = Irq<Dma0Id>;
pub type IrqDma1 = Irq<Dma1Id>;
pub type IrqDma2 = Irq<Dma2Id>;
pub type IrqDma3 = Irq<Dma3Id>;
pub type IrqTpm0 = Irq<Tpm0Id>;
pub type IrqTpm1 = Irq<Tpm1Id>;
pub type IrqTpm2 = Irq<Tpm2Id>;
pub type IrqPit = Irq<PitId>;
pub type IrqPorta = Irq<PortaId>;
pub type IrqPortc = Irq<PortcId>;
pub type IrqPortd = Irq<PortdId>;
pub type IrqUart0 = Irq<Uart0Id>;
pub type IrqUart1 = Irq<Uart1Id>;
pub type IrqUart2 = Irq<Uart2Id>;

#[doc(hidden)]
pub struct Dma0Id {} // IRQ 0
#[doc(hidden)]
pub struct Dma1Id {} // IRQ 1
#[doc(hidden)]
pub struct Dma2Id {} // IRQ 2
#[doc(hidden)]
pub struct Dma3Id {} // IRQ 3
#[doc(hidden)]
pub struct Tpm0Id {} // IRQ 17
#[doc(hidden)]
pub struct Tpm1Id {} // IRQ 18
#[doc(hidden)]
pub struct Tpm2Id {} // IRQ 19
#[doc(hidden)]
pub struct PitId {} // IRQ 22
#[doc(hidden)]
pub struct PortaId {} // IRQ 30
#[doc(hidden)]
pub struct PortcId {} // IRQ 31
#[doc(hidden)]
pub struct PortdId {} // IRQ 31
#[doc(hidden)]
pub struct Uart0Id {} // IRQ 12
#[doc(hidden)]
pub struct Uart1Id {} // IRQ 13
#[doc(hidden)]
pub struct Uart2Id {} // IRQ 14

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

impl RegisterHandler for IrqTpm0 {
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

impl RegisterHandler for IrqTpm1 {
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

impl RegisterHandler for IrqTpm2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(19, Some(wrapper::<F>));
       IrqGuard::new(19)
   }
}

impl RegisterHandler for IrqPit {
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

impl RegisterHandler for IrqPorta {
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

impl RegisterHandler for IrqPortc {
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

impl RegisterHandler for IrqPortd {
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

impl RegisterHandler for IrqUart0 {
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

impl RegisterHandler for IrqUart1 {
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

impl RegisterHandler for IrqUart2 {
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

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 48] = [
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
   None,                          // IRQ 12: No Description
   None,                          // IRQ 13: No Description
   None,                          // IRQ 14: No Description
   None,
   None,
   None,                          // IRQ 17: No Description
   None,                          // IRQ 18: No Description
   None,                          // IRQ 19: No Description
   None,
   None,
   None,                          // IRQ 22: No Description
   None,
   None,
   None,
   None,
   None,                          // IRQ 27: No Description
   None,
   None,
   None,                          // IRQ 30: No Description
   None,                          // IRQ 31: No Description
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
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 48] = [None; 48];

