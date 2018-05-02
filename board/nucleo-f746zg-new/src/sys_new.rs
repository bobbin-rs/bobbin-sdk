#![allow(unused_variables)]

use Board;
use bobbin_sys_new as sys;

impl sys::SystemProvider for Board {
    type McuProvider = Mcu;
    type ClockProvider = Clk;
    type ConsoleProvider = Con;
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

pub struct Con {}

impl sys::ConsoleProvider for Con {
    fn init<M: sys::McuProvider, C: sys::ClockProvider, H: sys::HeapProvider>(mcu: &mut sys::Mcu<M>, clk: &mut sys::Clocks<C>, mem: &mut sys::Heap<H>) -> Self {
        Self {}
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


