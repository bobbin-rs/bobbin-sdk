pub type Handler = unsafe extern "C" fn();

pub const IRQ_UART0: Irq = Irq(12);      // IRQ 12: No Description
pub const IRQ_UART1: Irq = Irq(13);      // IRQ 13: No Description
pub const IRQ_UART2: Irq = Irq(14);      // IRQ 14: No Description
pub const IRQ_PIT: Irq = Irq(22);        // IRQ 22: No Description
pub const IRQ_MCG: Irq = Irq(27);        // IRQ 27: No Description
pub const IRQ_PORTA: Irq = Irq(30);      // IRQ 30: No Description
pub const IRQ_PORTC_PORTD: Irq = Irq(31); // IRQ 31: No Description

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
   None,
   None,
   None,
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

