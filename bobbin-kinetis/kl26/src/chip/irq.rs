//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

irq!(IRQ_DMA0, IrqDma0, 0);
irq!(IRQ_DMA1, IrqDma1, 1);
irq!(IRQ_DMA2, IrqDma2, 2);
irq!(IRQ_DMA3, IrqDma3, 3);
irq!(IRQ_TPM0, IrqTpm0, 17);
irq!(IRQ_TPM1, IrqTpm1, 18);
irq!(IRQ_TPM2, IrqTpm2, 19);
irq!(IRQ_PIT, IrqPit, 22);
irq!(IRQ_PORTA, IrqPorta, 30);
irq!(IRQ_PORTC, IrqPortc, 31);
irq!(IRQ_PORTD, IrqPortd, 31);
irq!(IRQ_UART0, IrqUart0, 12);
irq!(IRQ_UART1, IrqUart1, 13);
irq!(IRQ_UART2, IrqUart2, 14);
irq!(IRQ_ADC0, IrqAdc0, 39);


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

