pub use kinetis_common::ext::*;

pub mod clock;
pub mod flash;

use bobbin_mcu::mcu::{GetActiveIrq, IrqEnable, Pend, Sleep};
use nvic::NVIC;
use scb::SCB;

impl GetActiveIrq for ::Mcu {
    fn get_active_irq() -> u8 {
        get_active_irq()
    }
}

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

