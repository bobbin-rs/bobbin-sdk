//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

irq!(IRQ_CTIMER, IrqCtimer, 13);
irq!(IRQ_ADC, IrqAdc, 16);
irq!(IRQ_IOMSTR0, IrqIomstr0, 6);
irq!(IRQ_IOMSTR1, IrqIomstr1, 7);
irq!(IRQ_IOMSTR2, IrqIomstr2, 8);
irq!(IRQ_IOMSTR3, IrqIomstr3, 9);
irq!(IRQ_IOMSTR4, IrqIomstr4, 10);
irq!(IRQ_IOMSTR5, IrqIomstr5, 11);
irq!(IRQ_IOSLAVE, IrqIoslave, 4);
irq!(IRQ_IOSLAVEACC, IrqIoslaveacc, 5);
irq!(IRQ_PDM, IrqPdm, 17);
irq!(IRQ_UART0, IrqUart0, 14);
irq!(IRQ_UART1, IrqUart1, 15);
irq!(IRQ_VCOMP, IrqVcomp, 3);
irq!(IRQ_GPIO, IrqGpio, 12);

pub fn handler(index: usize) -> Option<Handler> {
    unsafe { 
        R_INTERRUPT_HANDLERS[index]
    } 
}

pub fn set_handler(index: usize, handler: Option<Handler>) {
    unsafe { 
        assert!(R_INTERRUPT_HANDLERS[index].is_some() != handler.is_some());
        R_INTERRUPT_HANDLERS[index] = handler
  };
}

#[cfg_attr(target_os="none", link_section=".vector.interrupts")]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 33] = [
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
    None,                          // IRQ 21: No Description
    None,                          // IRQ 22: No Description
    None,                          // IRQ 23: No Description
    None,                          // IRQ 24: No Description
    None,                          // IRQ 25: No Description
    None,                          // IRQ 26: No Description
    None,
    None,
    None,
    None,
    None,
    None,
];

#[cfg_attr(target_os="none", link_section=".bss.r_interrupts")]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 33] = [None; 33];

