//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

irq!(IRQ_USART0_RX, IrqUsart0Rx, 12);
irq!(IRQ_USART0_TX, IrqUsart0Tx, 13);
irq!(IRQ_USART1_RX, IrqUsart1Rx, 20);
irq!(IRQ_USART1_TX, IrqUsart1Tx, 21);
irq!(IRQ_USART2_RX, IrqUsart2Rx, 40);
irq!(IRQ_USART2_TX, IrqUsart2Tx, 41);
irq!(IRQ_USART3_RX, IrqUsart3Rx, 43);
irq!(IRQ_USART3_TX, IrqUsart3Tx, 44);
irq!(IRQ_LEUART0, IrqLeuart0, 22);

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
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 50] = [
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
    None,                          // IRQ 10: No Description
    None,
    None,                          // IRQ 12: No Description
    None,                          // IRQ 13: No Description
    None,
    None,
    None,
    None,
    None,                          // IRQ 18: No Description
    None,
    None,                          // IRQ 20: No Description
    None,                          // IRQ 21: No Description
    None,                          // IRQ 22: No Description
    None,
    None,                          // IRQ 24: No Description
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
    None,                          // IRQ 40: No Description
    None,                          // IRQ 41: No Description
    None,
    None,                          // IRQ 43: No Description
    None,                          // IRQ 44: No Description
    None,
    None,
    None,
    None,
    None,
];

#[cfg_attr(target_os="none", link_section=".bss.r_interrupts")]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 50] = [None; 50];

