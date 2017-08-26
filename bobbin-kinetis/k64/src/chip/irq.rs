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
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 86] = [
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
   None,
   None,                          // IRQ 24: No Description
   None,                          // IRQ 25: No Description
   None,                          // IRQ 26: No Description
   None,                          // IRQ 27: No Description
   None,
   None,
   None,                          // IRQ 30: No Description
   None,                          // IRQ 31: No Description
   None,                          // IRQ 32: No Description
   None,                          // IRQ 33: No Description
   None,                          // IRQ 34: No Description
   None,                          // IRQ 35: No Description
   None,                          // IRQ 36: No Description
   None,                          // IRQ 37: No Description
   None,                          // IRQ 38: No Description
   None,                          // IRQ 39: No Description
   None,
   None,
   None,                          // IRQ 42: No Description
   None,                          // IRQ 43: No Description
   None,                          // IRQ 44: No Description
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
   None,                          // IRQ 58: No Description
   None,                          // IRQ 59: No Description
   None,                          // IRQ 60: No Description
   None,                          // IRQ 61: No Description
   None,                          // IRQ 62: No Description
   None,                          // IRQ 63: No Description
   None,
   None,                          // IRQ 65: No Description
   None,                          // IRQ 66: No Description
   None,                          // IRQ 67: No Description
   None,                          // IRQ 68: No Description
   None,                          // IRQ 69: No Description
   None,
   None,
   None,
   None,                          // IRQ 73: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 82: No Description
   None,                          // IRQ 83: No Description
   None,                          // IRQ 84: No Description
   None,                          // IRQ 85: No Description
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 86] = [None; 86];

