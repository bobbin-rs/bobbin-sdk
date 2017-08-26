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
   None,                          // IRQ 39: No Description
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

