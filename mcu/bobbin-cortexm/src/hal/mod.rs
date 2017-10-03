pub mod nvic;
pub mod scb;
pub mod systick;
pub mod mpu;
pub mod itm;
pub mod dwt;

use bobbin_common::{Irq, Poll};
use core::ops::Drop;

pub struct IrqGuard {
    irq_num: u8
}

impl Drop for IrqGuard {
    fn drop(&mut self) {
        nvic::set_enabled(self.irq_num as usize, false);
        scb::SCB.set_irq_handler(self.irq_num as usize, None);        
    }
}


pub trait RegisterPoll {
    fn register_poll<'a, F: ::core::marker::Sync + ::core::marker::Send + Poll>(&self, f: &F) -> IrqGuard;
}

impl<I: Irq> RegisterPoll for I {
    fn register_poll<'a, F: ::core::marker::Sync + ::core::marker::Send + Poll>(&self, f: &F) -> IrqGuard {
        scb::SCB.set_irq_handler(self.irq_num() as usize, Some(self.wrap(f)));
        nvic::set_enabled(self.irq_num() as usize, true);
        IrqGuard { irq_num: self.irq_num() }
    }    
}