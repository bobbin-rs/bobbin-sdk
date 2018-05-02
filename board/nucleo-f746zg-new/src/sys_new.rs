#![allow(unused_variables)]

use Board;
use bobbin_sys_new as sys;
use ::mcu::usart::*;

impl sys::SystemProvider for Board {
    type McuProvider = Mcu;
    type ClockProvider = Clk;
    type ConsoleProvider = con::Con<Usart3>;
    type TickProvider = Tck;
    type HeapProvider = Heap;
    type IrqDispatchProvider = IrqDispatch;
}

pub struct Mcu {}    
impl sys::McuProvider for Mcu {
    fn init() -> Self { Self {} }
}

pub struct Clk {
}
impl sys::ClockProvider for Clk {
    fn init() -> Self { Self {} }
}

pub struct Heap {}

impl sys::HeapProvider for Heap {
    fn init() -> Self { Self {} }    
}

pub mod con {
    use super::*;
    use ::prelude::*;
    use ::mcu::ext::rcc::{DedicatedClock};
    use ::mcu::usart::*;
    use ::mcu::pin::*;       
    use core::marker::PhantomData;
    use core::ops::Deref;

    const USART: Usart3 = USART3;
    const USART_TX: Pd8 = PD8;
    const USART_RX: Pd9 = PD9;
    const USART_CLOCK: u32 = 16_000_000; // Use HSI Clock
    const USART_BAUD: u32 = 115_200;
            
    pub struct Con<T> 
    where
        T: Deref<Target=UsartPeriph> + ClockSource<DedicatedClock> + GateEn + Default
    {
        usart: T
    }

    impl<T> sys::ConsoleProvider for Con<T> 
    where
        T: Deref<Target=UsartPeriph> + ClockSource<DedicatedClock> + GateEn + Default
    {
        fn init<M: sys::McuProvider, C: sys::ClockProvider, H: sys::HeapProvider>(mcu: &mut sys::Mcu<M>, clk: &mut sys::Clocks<C>, mem: &mut sys::Heap<H>) -> Self {            
            USART_TX
                .port_gate_enable()
                .connect_to(USART);

            USART_RX
                .port_gate_enable()
                .connect_to(USART);

            let usart = T::default();
            usart
                .set_clock_source(DedicatedClock::Hsi)
                .gate_enable()
                .set_config(|c| c.set_baud_clock(USART_BAUD, USART_CLOCK))
                .enable();
            
            Self { usart  }
        }

        fn write(&self, buf: &[u8]) {
            <UsartPeriph as SerialTx<u8>>::write(USART.as_periph(), buf);
        }
    }
}


pub struct Tck {}
impl sys::TickProvider for Tck {
    fn init<M: sys::McuProvider, C: sys::ClockProvider, H: sys::HeapProvider>(mcu: &mut sys::Mcu<M>, clk: &mut sys::Clocks<C>, mem: &mut sys::Heap<H>) -> Self {
        Self {}
    }
}

pub struct IrqDispatch {    
}
impl sys::IrqDispatchProvider for IrqDispatch {
    fn init<M: sys::McuProvider, C: sys::ClockProvider, H: sys::HeapProvider>(mcu: &mut sys::Mcu<M>, clk: &mut sys::Clocks<C>, mem: &mut sys::Heap<H>) -> Self {
        Self {}
    }
}


