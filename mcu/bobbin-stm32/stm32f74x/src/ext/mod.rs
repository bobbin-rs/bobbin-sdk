pub use stm32_common::ext::*;

pub mod rcc;
pub mod clock;
pub mod flash;

use bobbin_mcu::mcu::{IrqEnable, Pend, Sleep};
use nvic::NVIC;
use scb::SCB;

impl IrqEnable for ::Mcu {
    fn irq_enabled(irq: u8) -> bool { NVIC.enabled(irq) }
    fn irq_enable(irq: u8) { NVIC.set_enabled(irq, true); }
    fn irq_disable(irq: u8) { NVIC.set_enabled(irq, false); }
}

impl Pend for ::Mcu {
    fn pend() { 
        SCB.set_icsr(|r| r.set_pendsvset(1));
    }
}

impl Sleep for ::Mcu {    
    fn sleep() { sleep() }
}

// pub fn init() {    
//     // Enable Instruction Cache
//     SCB.set_iciallu(|r| r);
//     unsafe { asm!("dsb") }
//     unsafe { asm!("isb") }
//     SCB.with_ccr(|r| r.set_ic(1));
// }