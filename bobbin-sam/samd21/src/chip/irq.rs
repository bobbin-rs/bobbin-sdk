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
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 28] = [
   None,                          // IRQ 0: No Description
   None,                          // IRQ 1: No Description
   None,                          // IRQ 2: No Description
   None,                          // IRQ 3: No Description
   None,
   None,                          // IRQ 5: No Description
   None,                          // IRQ 6: No Description
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

