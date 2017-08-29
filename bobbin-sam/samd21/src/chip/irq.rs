//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

irq!(IRQ_DMAC, IrqDmac, 6);
irq!(IRQ_ADC, IrqAdc, 23);
irq!(IRQ_TCC0, IrqTcc0, 15);
irq!(IRQ_TCC1, IrqTcc1, 16);
irq!(IRQ_TCC2, IrqTcc2, 17);
irq!(IRQ_TC3, IrqTc3, 18);
irq!(IRQ_TC4, IrqTc4, 19);
irq!(IRQ_TC5, IrqTc5, 20);
irq!(IRQ_SERCOM0, IrqSercom0, 9);
irq!(IRQ_SERCOM1, IrqSercom1, 10);
irq!(IRQ_SERCOM2, IrqSercom2, 11);
irq!(IRQ_SERCOM3, IrqSercom3, 12);
irq!(IRQ_SERCOM4, IrqSercom4, 13);
irq!(IRQ_SERCOM5, IrqSercom5, 14);


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

