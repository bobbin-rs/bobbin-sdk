pub type Handler = unsafe extern "C" fn();

pub const IRQ_WDOG_EWM: Irq = Irq(22);   // IRQ 22: No Description
pub const IRQ_LPSPI0: Irq = Irq(26);     // IRQ 26: No Description
pub const IRQ_LPSPI1: Irq = Irq(27);     // IRQ 27: No Description
pub const IRQ_LPSPI2: Irq = Irq(28);     // IRQ 28: No Description
pub const IRQ_LPUART0_RXTX: Irq = Irq(31); // IRQ 31: No Description
pub const IRQ_LPUART1_RXTX: Irq = Irq(33); // IRQ 33: No Description
pub const IRQ_LPUART2_RXTX: Irq = Irq(35); // IRQ 35: No Description
pub const IRQ_LPIT0_CH0: Irq = Irq(48);  // IRQ 48: No Description
pub const IRQ_LPIT0_CH1: Irq = Irq(49);  // IRQ 49: No Description
pub const IRQ_LPIT0_CH2: Irq = Irq(50);  // IRQ 50: No Description
pub const IRQ_LPIT0_CH3: Irq = Irq(51);  // IRQ 51: No Description
pub const IRQ_SCG: Irq = Irq(57);        // IRQ 57: No Description
pub const IRQ_PORTA: Irq = Irq(59);      // IRQ 59: No Description
pub const IRQ_PORTB: Irq = Irq(60);      // IRQ 60: No Description
pub const IRQ_PORTC: Irq = Irq(61);      // IRQ 61: No Description
pub const IRQ_PORTD: Irq = Irq(62);      // IRQ 62: No Description
pub const IRQ_PORTE: Irq = Irq(63);      // IRQ 63: No Description
pub const IRQ_CAN0_ORED: Irq = Irq(78);  // IRQ 78: No Description
pub const IRQ_CAN0_ERROR: Irq = Irq(79); // IRQ 79: No Description
pub const IRQ_CAN0_WAKE_UP: Irq = Irq(80); // IRQ 80: No Description
pub const IRQ_CAN0_ORED_0_15_MB: Irq = Irq(81); // IRQ 81: No Description
pub const IRQ_CAN0_ORED_16_31_MB: Irq = Irq(82); // IRQ 82: No Description
pub const IRQ_CAN1_ORED: Irq = Irq(85);  // IRQ 85: No Description
pub const IRQ_CAN1_ERROR: Irq = Irq(86); // IRQ 86: No Description
pub const IRQ_CAN1_ORED_0_15_MB: Irq = Irq(88); // IRQ 88: No Description
pub const IRQ_CAN2_ORED: Irq = Irq(92);  // IRQ 92: No Description
pub const IRQ_CAN2_ERROR: Irq = Irq(93); // IRQ 93: No Description
pub const IRQ_CAN2_ORED_0_15_MB: Irq = Irq(95); // IRQ 95: No Description

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

