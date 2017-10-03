pub mod nvic;
pub mod scb;
pub mod systick;
pub mod mpu;
pub mod itm;
pub mod dwt;

use bobbin_common::{Irq, Poll, Handler};
use core::ops::Drop;
use core::marker::PhantomData;

#[must_use]
pub struct IrqGuard<G> {
    irq_num: u8,
    _phantom: PhantomData<G>,
}

impl<G> Drop for IrqGuard<G> {
    fn drop(&mut self) {
        nvic::set_enabled(self.irq_num as usize, false);
        scb::SCB.set_irq_handler(self.irq_num as usize, None);        
    }
}

pub trait RegisterHandler {
    fn register_handler(&self, h: Handler) -> IrqGuard<Handler>;
}

impl<I: Irq> RegisterHandler for I {
    fn register_handler(&self, h: Handler) -> IrqGuard<Handler> {
        scb::SCB.set_irq_handler(self.irq_num() as usize, Some(h));
        nvic::set_enabled(self.irq_num() as usize, true);
        IrqGuard { irq_num: self.irq_num(), _phantom: PhantomData }
    }    
}


pub trait RegisterPoll {
    fn register_poll<'a, F: ::core::marker::Sync + ::core::marker::Send + Poll>(&self, f: &'a F) -> IrqGuard<&'a F>;
}

impl<I: Irq> RegisterPoll for I {
    fn register_poll<'a, F: ::core::marker::Sync + ::core::marker::Send + Poll>(&self, f: &'a F) -> IrqGuard<&'a F> {
        scb::SCB.set_irq_handler(self.irq_num() as usize, Some(self.wrap(f)));
        nvic::set_enabled(self.irq_num() as usize, true);
        IrqGuard { irq_num: self.irq_num(), _phantom: PhantomData }
    }    
}