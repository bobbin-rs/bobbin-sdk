//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

irq!(IRQ_GPIO_EVEN, IrqGpioEven, 10);
irq!(IRQ_GPIO_ODD, IrqGpioOdd, 18);

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
    None,
    None,
    None,
    None,
    None,
    None,
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

#[cfg_attr(target_os="none", link_section=".bss.r_interrupts")]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 50] = [None; 50];

