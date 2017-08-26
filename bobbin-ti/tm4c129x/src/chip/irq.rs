//! Interrupts

pub type Handler = extern "C" fn();


pub fn set_handler(index: usize, handler: Option<Handler>) {
  unsafe { 
     assert!(R_INTERRUPT_HANDLERS[index].is_some() != handler.is_some());
     R_INTERRUPT_HANDLERS[index] = handler
  };
}

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 114] = [
   None,                          // IRQ 0: No Description
   None,                          // IRQ 1: No Description
   None,                          // IRQ 2: No Description
   None,                          // IRQ 3: No Description
   None,                          // IRQ 4: No Description
   None,                          // IRQ 5: No Description
   None,                          // IRQ 6: No Description
   None,                          // IRQ 7: No Description
   None,                          // IRQ 8: No Description
   None,                          // IRQ 9: No Description
   None,
   None,
   None,
   None,
   None,                          // IRQ 14: No Description
   None,                          // IRQ 15: No Description
   None,                          // IRQ 16: No Description
   None,                          // IRQ 17: No Description
   None,                          // IRQ 18: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 28: No Description
   None,
   None,                          // IRQ 30: No Description
   None,                          // IRQ 31: No Description
   None,                          // IRQ 32: No Description
   None,                          // IRQ 33: No Description
   None,                          // IRQ 34: No Description
   None,
   None,
   None,                          // IRQ 37: No Description
   None,
   None,
   None,                          // IRQ 40: No Description
   None,
   None,
   None,
   None,                          // IRQ 44: No Description
   None,                          // IRQ 45: No Description
   None,                          // IRQ 46: No Description
   None,                          // IRQ 47: No Description
   None,                          // IRQ 48: No Description
   None,                          // IRQ 49: No Description
   None,
   None,
   None,
   None,                          // IRQ 53: No Description
   None,                          // IRQ 54: No Description
   None,                          // IRQ 55: No Description
   None,                          // IRQ 56: No Description
   None,                          // IRQ 57: No Description
   None,                          // IRQ 58: No Description
   None,                          // IRQ 59: No Description
   None,                          // IRQ 60: No Description
   None,                          // IRQ 61: No Description
   None,                          // IRQ 62: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 72: No Description
   None,                          // IRQ 73: No Description
   None,
   None,
   None,                          // IRQ 76: No Description
   None,                          // IRQ 77: No Description
   None,                          // IRQ 78: No Description
   None,                          // IRQ 79: No Description
   None,                          // IRQ 80: No Description
   None,                          // IRQ 81: No Description
   None,                          // IRQ 82: No Description
   None,                          // IRQ 83: No Description
   None,                          // IRQ 84: No Description
   None,                          // IRQ 85: No Description
   None,                          // IRQ 86: No Description
   None,                          // IRQ 87: No Description
   None,                          // IRQ 88: No Description
   None,                          // IRQ 89: No Description
   None,                          // IRQ 90: No Description
   None,                          // IRQ 91: No Description
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
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 114] = [None; 114];

