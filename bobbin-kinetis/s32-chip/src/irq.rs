use ::core::marker::PhantomData;
pub type Handler = extern "C" fn();

pub const IRQ_FTM0_FAULT: IrqFtm0Fault = Irq(103, Ftm0FaultId {});
pub const IRQ_FTM0_OVERFLOW: IrqFtm0Overflow = Irq(104, Ftm0OverflowId {});
pub const IRQ_FTM0_CH0: IrqFtm0Ch0 = Irq(99, Ftm0Ch0Id {});
pub const IRQ_FTM0_CH1: IrqFtm0Ch1 = Irq(99, Ftm0Ch1Id {});
pub const IRQ_FTM0_CH2: IrqFtm0Ch2 = Irq(100, Ftm0Ch2Id {});
pub const IRQ_FTM0_CH3: IrqFtm0Ch3 = Irq(100, Ftm0Ch3Id {});
pub const IRQ_FTM0_CH4: IrqFtm0Ch4 = Irq(101, Ftm0Ch4Id {});
pub const IRQ_FTM0_CH5: IrqFtm0Ch5 = Irq(101, Ftm0Ch5Id {});
pub const IRQ_FTM0_CH6: IrqFtm0Ch6 = Irq(102, Ftm0Ch6Id {});
pub const IRQ_FTM0_CH7: IrqFtm0Ch7 = Irq(102, Ftm0Ch7Id {});
pub const IRQ_FTM1_FAULT: IrqFtm1Fault = Irq(109, Ftm1FaultId {});
pub const IRQ_FTM1_OVERFLOW: IrqFtm1Overflow = Irq(110, Ftm1OverflowId {});
pub const IRQ_FTM1_CH0: IrqFtm1Ch0 = Irq(105, Ftm1Ch0Id {});
pub const IRQ_FTM1_CH1: IrqFtm1Ch1 = Irq(105, Ftm1Ch1Id {});
pub const IRQ_FTM1_CH2: IrqFtm1Ch2 = Irq(106, Ftm1Ch2Id {});
pub const IRQ_FTM1_CH3: IrqFtm1Ch3 = Irq(106, Ftm1Ch3Id {});
pub const IRQ_FTM1_CH4: IrqFtm1Ch4 = Irq(107, Ftm1Ch4Id {});
pub const IRQ_FTM1_CH5: IrqFtm1Ch5 = Irq(107, Ftm1Ch5Id {});
pub const IRQ_FTM1_CH6: IrqFtm1Ch6 = Irq(108, Ftm1Ch6Id {});
pub const IRQ_FTM1_CH7: IrqFtm1Ch7 = Irq(108, Ftm1Ch7Id {});
pub const IRQ_FTM2_FAULT: IrqFtm2Fault = Irq(115, Ftm2FaultId {});
pub const IRQ_FTM2_OVERFLOW: IrqFtm2Overflow = Irq(116, Ftm2OverflowId {});
pub const IRQ_FTM2_CH0: IrqFtm2Ch0 = Irq(111, Ftm2Ch0Id {});
pub const IRQ_FTM2_CH1: IrqFtm2Ch1 = Irq(111, Ftm2Ch1Id {});
pub const IRQ_FTM2_CH2: IrqFtm2Ch2 = Irq(112, Ftm2Ch2Id {});
pub const IRQ_FTM2_CH3: IrqFtm2Ch3 = Irq(112, Ftm2Ch3Id {});
pub const IRQ_FTM2_CH4: IrqFtm2Ch4 = Irq(113, Ftm2Ch4Id {});
pub const IRQ_FTM2_CH5: IrqFtm2Ch5 = Irq(113, Ftm2Ch5Id {});
pub const IRQ_FTM2_CH6: IrqFtm2Ch6 = Irq(114, Ftm2Ch6Id {});
pub const IRQ_FTM2_CH7: IrqFtm2Ch7 = Irq(114, Ftm2Ch7Id {});
pub const IRQ_FTM3_FAULT: IrqFtm3Fault = Irq(121, Ftm3FaultId {});
pub const IRQ_FTM3_OVERFLOW: IrqFtm3Overflow = Irq(122, Ftm3OverflowId {});
pub const IRQ_FTM3_CH0: IrqFtm3Ch0 = Irq(117, Ftm3Ch0Id {});
pub const IRQ_FTM3_CH1: IrqFtm3Ch1 = Irq(117, Ftm3Ch1Id {});
pub const IRQ_FTM3_CH2: IrqFtm3Ch2 = Irq(118, Ftm3Ch2Id {});
pub const IRQ_FTM3_CH3: IrqFtm3Ch3 = Irq(118, Ftm3Ch3Id {});
pub const IRQ_FTM3_CH4: IrqFtm3Ch4 = Irq(119, Ftm3Ch4Id {});
pub const IRQ_FTM3_CH5: IrqFtm3Ch5 = Irq(119, Ftm3Ch5Id {});
pub const IRQ_FTM3_CH6: IrqFtm3Ch6 = Irq(120, Ftm3Ch6Id {});
pub const IRQ_FTM3_CH7: IrqFtm3Ch7 = Irq(120, Ftm3Ch7Id {});
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

pub type IrqFtm0Fault = Irq<Ftm0FaultId>;
pub type IrqFtm0Overflow = Irq<Ftm0OverflowId>;
pub type IrqFtm0Ch0 = Irq<Ftm0Ch0Id>;
pub type IrqFtm0Ch1 = Irq<Ftm0Ch1Id>;
pub type IrqFtm0Ch2 = Irq<Ftm0Ch2Id>;
pub type IrqFtm0Ch3 = Irq<Ftm0Ch3Id>;
pub type IrqFtm0Ch4 = Irq<Ftm0Ch4Id>;
pub type IrqFtm0Ch5 = Irq<Ftm0Ch5Id>;
pub type IrqFtm0Ch6 = Irq<Ftm0Ch6Id>;
pub type IrqFtm0Ch7 = Irq<Ftm0Ch7Id>;
pub type IrqFtm1Fault = Irq<Ftm1FaultId>;
pub type IrqFtm1Overflow = Irq<Ftm1OverflowId>;
pub type IrqFtm1Ch0 = Irq<Ftm1Ch0Id>;
pub type IrqFtm1Ch1 = Irq<Ftm1Ch1Id>;
pub type IrqFtm1Ch2 = Irq<Ftm1Ch2Id>;
pub type IrqFtm1Ch3 = Irq<Ftm1Ch3Id>;
pub type IrqFtm1Ch4 = Irq<Ftm1Ch4Id>;
pub type IrqFtm1Ch5 = Irq<Ftm1Ch5Id>;
pub type IrqFtm1Ch6 = Irq<Ftm1Ch6Id>;
pub type IrqFtm1Ch7 = Irq<Ftm1Ch7Id>;
pub type IrqFtm2Fault = Irq<Ftm2FaultId>;
pub type IrqFtm2Overflow = Irq<Ftm2OverflowId>;
pub type IrqFtm2Ch0 = Irq<Ftm2Ch0Id>;
pub type IrqFtm2Ch1 = Irq<Ftm2Ch1Id>;
pub type IrqFtm2Ch2 = Irq<Ftm2Ch2Id>;
pub type IrqFtm2Ch3 = Irq<Ftm2Ch3Id>;
pub type IrqFtm2Ch4 = Irq<Ftm2Ch4Id>;
pub type IrqFtm2Ch5 = Irq<Ftm2Ch5Id>;
pub type IrqFtm2Ch6 = Irq<Ftm2Ch6Id>;
pub type IrqFtm2Ch7 = Irq<Ftm2Ch7Id>;
pub type IrqFtm3Fault = Irq<Ftm3FaultId>;
pub type IrqFtm3Overflow = Irq<Ftm3OverflowId>;
pub type IrqFtm3Ch0 = Irq<Ftm3Ch0Id>;
pub type IrqFtm3Ch1 = Irq<Ftm3Ch1Id>;
pub type IrqFtm3Ch2 = Irq<Ftm3Ch2Id>;
pub type IrqFtm3Ch3 = Irq<Ftm3Ch3Id>;
pub type IrqFtm3Ch4 = Irq<Ftm3Ch4Id>;
pub type IrqFtm3Ch5 = Irq<Ftm3Ch5Id>;
pub type IrqFtm3Ch6 = Irq<Ftm3Ch6Id>;
pub type IrqFtm3Ch7 = Irq<Ftm3Ch7Id>;
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

pub struct Ftm0FaultId {} // IRQ 103
pub struct Ftm0OverflowId {} // IRQ 104
pub struct Ftm0Ch0Id {} // IRQ 99
pub struct Ftm0Ch1Id {} // IRQ 99
pub struct Ftm0Ch2Id {} // IRQ 100
pub struct Ftm0Ch3Id {} // IRQ 100
pub struct Ftm0Ch4Id {} // IRQ 101
pub struct Ftm0Ch5Id {} // IRQ 101
pub struct Ftm0Ch6Id {} // IRQ 102
pub struct Ftm0Ch7Id {} // IRQ 102
pub struct Ftm1FaultId {} // IRQ 109
pub struct Ftm1OverflowId {} // IRQ 110
pub struct Ftm1Ch0Id {} // IRQ 105
pub struct Ftm1Ch1Id {} // IRQ 105
pub struct Ftm1Ch2Id {} // IRQ 106
pub struct Ftm1Ch3Id {} // IRQ 106
pub struct Ftm1Ch4Id {} // IRQ 107
pub struct Ftm1Ch5Id {} // IRQ 107
pub struct Ftm1Ch6Id {} // IRQ 108
pub struct Ftm1Ch7Id {} // IRQ 108
pub struct Ftm2FaultId {} // IRQ 115
pub struct Ftm2OverflowId {} // IRQ 116
pub struct Ftm2Ch0Id {} // IRQ 111
pub struct Ftm2Ch1Id {} // IRQ 111
pub struct Ftm2Ch2Id {} // IRQ 112
pub struct Ftm2Ch3Id {} // IRQ 112
pub struct Ftm2Ch4Id {} // IRQ 113
pub struct Ftm2Ch5Id {} // IRQ 113
pub struct Ftm2Ch6Id {} // IRQ 114
pub struct Ftm2Ch7Id {} // IRQ 114
pub struct Ftm3FaultId {} // IRQ 121
pub struct Ftm3OverflowId {} // IRQ 122
pub struct Ftm3Ch0Id {} // IRQ 117
pub struct Ftm3Ch1Id {} // IRQ 117
pub struct Ftm3Ch2Id {} // IRQ 118
pub struct Ftm3Ch3Id {} // IRQ 118
pub struct Ftm3Ch4Id {} // IRQ 119
pub struct Ftm3Ch5Id {} // IRQ 119
pub struct Ftm3Ch6Id {} // IRQ 120
pub struct Ftm3Ch7Id {} // IRQ 120
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

impl RegisterHandler for IrqFtm0Fault {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(103, Some(wrapper::<F>));
       IrqGuard::new(103)
   }
}

impl RegisterHandler for IrqFtm0Overflow {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(104, Some(wrapper::<F>));
       IrqGuard::new(104)
   }
}

impl RegisterHandler for IrqFtm0Ch0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(99, Some(wrapper::<F>));
       IrqGuard::new(99)
   }
}

impl RegisterHandler for IrqFtm0Ch1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(99, Some(wrapper::<F>));
       IrqGuard::new(99)
   }
}

impl RegisterHandler for IrqFtm0Ch2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(100, Some(wrapper::<F>));
       IrqGuard::new(100)
   }
}

impl RegisterHandler for IrqFtm0Ch3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(100, Some(wrapper::<F>));
       IrqGuard::new(100)
   }
}

impl RegisterHandler for IrqFtm0Ch4 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(101, Some(wrapper::<F>));
       IrqGuard::new(101)
   }
}

impl RegisterHandler for IrqFtm0Ch5 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(101, Some(wrapper::<F>));
       IrqGuard::new(101)
   }
}

impl RegisterHandler for IrqFtm0Ch6 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(102, Some(wrapper::<F>));
       IrqGuard::new(102)
   }
}

impl RegisterHandler for IrqFtm0Ch7 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(102, Some(wrapper::<F>));
       IrqGuard::new(102)
   }
}

impl RegisterHandler for IrqFtm1Fault {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(109, Some(wrapper::<F>));
       IrqGuard::new(109)
   }
}

impl RegisterHandler for IrqFtm1Overflow {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(110, Some(wrapper::<F>));
       IrqGuard::new(110)
   }
}

impl RegisterHandler for IrqFtm1Ch0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(105, Some(wrapper::<F>));
       IrqGuard::new(105)
   }
}

impl RegisterHandler for IrqFtm1Ch1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(105, Some(wrapper::<F>));
       IrqGuard::new(105)
   }
}

impl RegisterHandler for IrqFtm1Ch2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(106, Some(wrapper::<F>));
       IrqGuard::new(106)
   }
}

impl RegisterHandler for IrqFtm1Ch3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(106, Some(wrapper::<F>));
       IrqGuard::new(106)
   }
}

impl RegisterHandler for IrqFtm1Ch4 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(107, Some(wrapper::<F>));
       IrqGuard::new(107)
   }
}

impl RegisterHandler for IrqFtm1Ch5 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(107, Some(wrapper::<F>));
       IrqGuard::new(107)
   }
}

impl RegisterHandler for IrqFtm1Ch6 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(108, Some(wrapper::<F>));
       IrqGuard::new(108)
   }
}

impl RegisterHandler for IrqFtm1Ch7 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(108, Some(wrapper::<F>));
       IrqGuard::new(108)
   }
}

impl RegisterHandler for IrqFtm2Fault {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(115, Some(wrapper::<F>));
       IrqGuard::new(115)
   }
}

impl RegisterHandler for IrqFtm2Overflow {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(116, Some(wrapper::<F>));
       IrqGuard::new(116)
   }
}

impl RegisterHandler for IrqFtm2Ch0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(111, Some(wrapper::<F>));
       IrqGuard::new(111)
   }
}

impl RegisterHandler for IrqFtm2Ch1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(111, Some(wrapper::<F>));
       IrqGuard::new(111)
   }
}

impl RegisterHandler for IrqFtm2Ch2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(112, Some(wrapper::<F>));
       IrqGuard::new(112)
   }
}

impl RegisterHandler for IrqFtm2Ch3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(112, Some(wrapper::<F>));
       IrqGuard::new(112)
   }
}

impl RegisterHandler for IrqFtm2Ch4 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(113, Some(wrapper::<F>));
       IrqGuard::new(113)
   }
}

impl RegisterHandler for IrqFtm2Ch5 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(113, Some(wrapper::<F>));
       IrqGuard::new(113)
   }
}

impl RegisterHandler for IrqFtm2Ch6 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(114, Some(wrapper::<F>));
       IrqGuard::new(114)
   }
}

impl RegisterHandler for IrqFtm2Ch7 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(114, Some(wrapper::<F>));
       IrqGuard::new(114)
   }
}

impl RegisterHandler for IrqFtm3Fault {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(121, Some(wrapper::<F>));
       IrqGuard::new(121)
   }
}

impl RegisterHandler for IrqFtm3Overflow {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(122, Some(wrapper::<F>));
       IrqGuard::new(122)
   }
}

impl RegisterHandler for IrqFtm3Ch0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(117, Some(wrapper::<F>));
       IrqGuard::new(117)
   }
}

impl RegisterHandler for IrqFtm3Ch1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(117, Some(wrapper::<F>));
       IrqGuard::new(117)
   }
}

impl RegisterHandler for IrqFtm3Ch2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(118, Some(wrapper::<F>));
       IrqGuard::new(118)
   }
}

impl RegisterHandler for IrqFtm3Ch3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(118, Some(wrapper::<F>));
       IrqGuard::new(118)
   }
}

impl RegisterHandler for IrqFtm3Ch4 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(119, Some(wrapper::<F>));
       IrqGuard::new(119)
   }
}

impl RegisterHandler for IrqFtm3Ch5 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(119, Some(wrapper::<F>));
       IrqGuard::new(119)
   }
}

impl RegisterHandler for IrqFtm3Ch6 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(120, Some(wrapper::<F>));
       IrqGuard::new(120)
   }
}

impl RegisterHandler for IrqFtm3Ch7 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(120, Some(wrapper::<F>));
       IrqGuard::new(120)
   }
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
   None,                          // IRQ 103: No Description
   None,                          // IRQ 104: No Description
   None,
   None,
   None,
   None,
   None,                          // IRQ 109: No Description
   None,                          // IRQ 110: No Description
   None,
   None,
   None,
   None,
   None,                          // IRQ 115: No Description
   None,                          // IRQ 116: No Description
   None,
   None,
   None,
   None,
   None,                          // IRQ 121: No Description
   None,                          // IRQ 122: No Description
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

