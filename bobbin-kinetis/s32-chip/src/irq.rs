use ::core::marker::PhantomData;
pub type Handler = extern "C" fn();

pub const IRQ_LPIT0_CH0: IrqLpit0Ch0 = Irq(48, Lpit0Ch0Id {});
pub const IRQ_LPIT0_CH1: IrqLpit0Ch1 = Irq(49, Lpit0Ch1Id {});
pub const IRQ_LPIT0_CH2: IrqLpit0Ch2 = Irq(50, Lpit0Ch2Id {});
pub const IRQ_LPIT0_CH3: IrqLpit0Ch3 = Irq(51, Lpit0Ch3Id {});
pub const IRQ_CAN0_ORED: IrqCan0Ored = Irq(78, Can0OredId {});
pub const IRQ_CAN0_ERROR: IrqCan0Error = Irq(79, Can0ErrorId {});
pub const IRQ_CAN0_WAKE_UP: IrqCan0WakeUp = Irq(80, Can0WakeUpId {});
pub const IRQ_CAN0_ORED_0_15_MB: IrqCan0Ored015Mb = Irq(81, Can0Ored015MbId {});
pub const IRQ_CAN0_ORED_16_31_MB: IrqCan0Ored1631Mb = Irq(82, Can0Ored1631MbId {});
pub const IRQ_CAN1_ORED: IrqCan1Ored = Irq(85, Can1OredId {});
pub const IRQ_CAN1_ERROR: IrqCan1Error = Irq(86, Can1ErrorId {});
pub const IRQ_CAN1_ORED_0_15_MB: IrqCan1Ored015Mb = Irq(88, Can1Ored015MbId {});
pub const IRQ_CAN2_ORED: IrqCan2Ored = Irq(92, Can2OredId {});
pub const IRQ_CAN2_ERROR: IrqCan2Error = Irq(93, Can2ErrorId {});
pub const IRQ_CAN2_ORED_0_15_MB: IrqCan2Ored015Mb = Irq(95, Can2Ored015MbId {});
pub const IRQ_PORTA: IrqPorta = Irq(59, PortaId {});
pub const IRQ_PORTB: IrqPortb = Irq(60, PortbId {});
pub const IRQ_PORTC: IrqPortc = Irq(61, PortcId {});
pub const IRQ_PORTD: IrqPortd = Irq(62, PortdId {});
pub const IRQ_PORTE: IrqPorte = Irq(63, PorteId {});
pub const IRQ_LPUART0_RXTX: IrqLpuart0Rxtx = Irq(31, Lpuart0RxtxId {});
pub const IRQ_LPUART1_RXTX: IrqLpuart1Rxtx = Irq(33, Lpuart1RxtxId {});
pub const IRQ_LPUART2_RXTX: IrqLpuart2Rxtx = Irq(35, Lpuart2RxtxId {});
pub const IRQ_LPSPI0: IrqLpspi0 = Irq(26, Lpspi0Id {});
pub const IRQ_LPSPI1: IrqLpspi1 = Irq(27, Lpspi1Id {});
pub const IRQ_LPSPI2: IrqLpspi2 = Irq(28, Lpspi2Id {});

pub type IrqLpit0Ch0 = Irq<Lpit0Ch0Id>;
pub type IrqLpit0Ch1 = Irq<Lpit0Ch1Id>;
pub type IrqLpit0Ch2 = Irq<Lpit0Ch2Id>;
pub type IrqLpit0Ch3 = Irq<Lpit0Ch3Id>;
pub type IrqCan0Ored = Irq<Can0OredId>;
pub type IrqCan0Error = Irq<Can0ErrorId>;
pub type IrqCan0WakeUp = Irq<Can0WakeUpId>;
pub type IrqCan0Ored015Mb = Irq<Can0Ored015MbId>;
pub type IrqCan0Ored1631Mb = Irq<Can0Ored1631MbId>;
pub type IrqCan1Ored = Irq<Can1OredId>;
pub type IrqCan1Error = Irq<Can1ErrorId>;
pub type IrqCan1Ored015Mb = Irq<Can1Ored015MbId>;
pub type IrqCan2Ored = Irq<Can2OredId>;
pub type IrqCan2Error = Irq<Can2ErrorId>;
pub type IrqCan2Ored015Mb = Irq<Can2Ored015MbId>;
pub type IrqPorta = Irq<PortaId>;
pub type IrqPortb = Irq<PortbId>;
pub type IrqPortc = Irq<PortcId>;
pub type IrqPortd = Irq<PortdId>;
pub type IrqPorte = Irq<PorteId>;
pub type IrqLpuart0Rxtx = Irq<Lpuart0RxtxId>;
pub type IrqLpuart1Rxtx = Irq<Lpuart1RxtxId>;
pub type IrqLpuart2Rxtx = Irq<Lpuart2RxtxId>;
pub type IrqLpspi0 = Irq<Lpspi0Id>;
pub type IrqLpspi1 = Irq<Lpspi1Id>;
pub type IrqLpspi2 = Irq<Lpspi2Id>;

pub struct Lpit0Ch0Id {} // IRQ 48
pub struct Lpit0Ch1Id {} // IRQ 49
pub struct Lpit0Ch2Id {} // IRQ 50
pub struct Lpit0Ch3Id {} // IRQ 51
pub struct Can0OredId {} // IRQ 78
pub struct Can0ErrorId {} // IRQ 79
pub struct Can0WakeUpId {} // IRQ 80
pub struct Can0Ored015MbId {} // IRQ 81
pub struct Can0Ored1631MbId {} // IRQ 82
pub struct Can1OredId {} // IRQ 85
pub struct Can1ErrorId {} // IRQ 86
pub struct Can1Ored015MbId {} // IRQ 88
pub struct Can2OredId {} // IRQ 92
pub struct Can2ErrorId {} // IRQ 93
pub struct Can2Ored015MbId {} // IRQ 95
pub struct PortaId {} // IRQ 59
pub struct PortbId {} // IRQ 60
pub struct PortcId {} // IRQ 61
pub struct PortdId {} // IRQ 62
pub struct PorteId {} // IRQ 63
pub struct Lpuart0RxtxId {} // IRQ 31
pub struct Lpuart1RxtxId {} // IRQ 33
pub struct Lpuart2RxtxId {} // IRQ 35
pub struct Lpspi0Id {} // IRQ 26
pub struct Lpspi1Id {} // IRQ 27
pub struct Lpspi2Id {} // IRQ 28

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

impl RegisterHandler for IrqLpit0Ch0 {
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

impl RegisterHandler for IrqLpit0Ch1 {
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

impl RegisterHandler for IrqLpit0Ch2 {
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

impl RegisterHandler for IrqLpit0Ch3 {
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

impl RegisterHandler for IrqCan0Ored {
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

impl RegisterHandler for IrqCan0Error {
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

impl RegisterHandler for IrqCan0WakeUp {
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

impl RegisterHandler for IrqCan0Ored015Mb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(81, Some(wrapper::<F>));
       IrqGuard::new(81)
   }
}

impl RegisterHandler for IrqCan0Ored1631Mb {
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

impl RegisterHandler for IrqCan1Ored {
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

impl RegisterHandler for IrqCan1Error {
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

impl RegisterHandler for IrqCan1Ored015Mb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(88, Some(wrapper::<F>));
       IrqGuard::new(88)
   }
}

impl RegisterHandler for IrqCan2Ored {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(92, Some(wrapper::<F>));
       IrqGuard::new(92)
   }
}

impl RegisterHandler for IrqCan2Error {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(93, Some(wrapper::<F>));
       IrqGuard::new(93)
   }
}

impl RegisterHandler for IrqCan2Ored015Mb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(95, Some(wrapper::<F>));
       IrqGuard::new(95)
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

impl RegisterHandler for IrqLpuart0Rxtx {
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

impl RegisterHandler for IrqLpuart1Rxtx {
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

impl RegisterHandler for IrqLpuart2Rxtx {
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

impl RegisterHandler for IrqLpspi0 {
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

impl RegisterHandler for IrqLpspi1 {
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

impl RegisterHandler for IrqLpspi2 {
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

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 147] = [
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
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 22: No Description
   None,
   None,
   None,
   None,                          // IRQ 26: No Description
   None,                          // IRQ 27: No Description
   None,                          // IRQ 28: No Description
   None,
   None,
   None,                          // IRQ 31: No Description
   None,
   None,                          // IRQ 33: No Description
   None,
   None,                          // IRQ 35: No Description
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
   None,                          // IRQ 48: No Description
   None,                          // IRQ 49: No Description
   None,                          // IRQ 50: No Description
   None,                          // IRQ 51: No Description
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 57: No Description
   None,
   None,                          // IRQ 59: No Description
   None,                          // IRQ 60: No Description
   None,                          // IRQ 61: No Description
   None,                          // IRQ 62: No Description
   None,                          // IRQ 63: No Description
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
   None,                          // IRQ 78: No Description
   None,                          // IRQ 79: No Description
   None,                          // IRQ 80: No Description
   None,                          // IRQ 81: No Description
   None,                          // IRQ 82: No Description
   None,
   None,
   None,                          // IRQ 85: No Description
   None,                          // IRQ 86: No Description
   None,
   None,                          // IRQ 88: No Description
   None,
   None,
   None,
   None,                          // IRQ 92: No Description
   None,                          // IRQ 93: No Description
   None,
   None,                          // IRQ 95: No Description
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
   None,
   None,
   None,
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 147] = [None; 147];

