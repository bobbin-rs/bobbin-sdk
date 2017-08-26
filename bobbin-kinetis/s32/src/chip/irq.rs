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
   None,                          // IRQ 16: No Description
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 22: No Description
   None,                          // IRQ 23: No Description
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
   None,                          // IRQ 39: No Description
   None,                          // IRQ 40: No Description
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 46: No Description
   None,                          // IRQ 47: No Description
   None,
   None,
   None,
   None,
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

