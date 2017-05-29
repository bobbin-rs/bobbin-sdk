pub type Handler = unsafe extern "C" fn();

pub const IRQ_PM: Irq = Irq(0);          // IRQ 0: No Description
pub const IRQ_SYSCTRL: Irq = Irq(1);     // IRQ 1: No Description
pub const IRQ_RTC: Irq = Irq(3);         // IRQ 3: No Description
pub const IRQ_NVMCTRL: Irq = Irq(5);     // IRQ 5: No Description
pub const IRQ_SERCOM0: Irq = Irq(9);     // IRQ 9: No Description
pub const IRQ_SERCOM1: Irq = Irq(10);    // IRQ 10: No Description
pub const IRQ_SERCOM2: Irq = Irq(11);    // IRQ 11: No Description
pub const IRQ_SERCOM3: Irq = Irq(12);    // IRQ 12: No Description
pub const IRQ_SERCOM4: Irq = Irq(13);    // IRQ 13: No Description
pub const IRQ_SERCOM5: Irq = Irq(14);    // IRQ 14: No Description
pub const IRQ_TCC0: Irq = Irq(15);       // IRQ 15: No Description
pub const IRQ_TCC1: Irq = Irq(16);       // IRQ 16: No Description
pub const IRQ_TCC2: Irq = Irq(17);       // IRQ 17: No Description
pub const IRQ_TC3: Irq = Irq(18);        // IRQ 18: No Description
pub const IRQ_TC4: Irq = Irq(19);        // IRQ 19: No Description
pub const IRQ_TC5: Irq = Irq(20);        // IRQ 20: No Description
pub const IRQ_ADC: Irq = Irq(23);        // IRQ 23: No Description

pub fn set_handler(irq: Irq, handler: Option<Handler>) {
  unsafe { R_INTERRUPT_HANDLERS[irq.0] = handler };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Irq(pub usize);

impl Irq {
   pub fn set_handler(&self, handler: Option<Handler>) {
      unsafe { R_INTERRUPT_HANDLERS[self.0] = handler };
   }
}

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 28] = [
   None,                          // IRQ 0: No Description
   None,                          // IRQ 1: No Description
   None,
   None,                          // IRQ 3: No Description
   None,
   None,                          // IRQ 5: No Description
   None,
   None,
   None,
   None,                          // IRQ 9: No Description
   None,                          // IRQ 10: No Description
   None,                          // IRQ 11: No Description
   None,                          // IRQ 12: No Description
   None,                          // IRQ 13: No Description
   None,                          // IRQ 14: No Description
   None,                          // IRQ 15: No Description
   None,                          // IRQ 16: No Description
   None,                          // IRQ 17: No Description
   None,                          // IRQ 18: No Description
   None,                          // IRQ 19: No Description
   None,                          // IRQ 20: No Description
   None,
   None,
   None,                          // IRQ 23: No Description
   None,
   None,
   None,
   None,
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 28] = [None; 28];

