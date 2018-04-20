pub mod nvic;
pub mod systick;
pub mod dispatch;
pub mod ms_tick;

#[inline]
pub fn get_active_irq() -> u8 {
    ::scb::SCB.icsr().vectactive().value()
}

#[inline]
pub fn sleep() {
    unsafe { asm!("
        cpsid i
        wfi
        cpsie i
    ")}
}

// use bobbin_sys::{heap, tick};

// impl heap::GetHeap for ::Cortexm {}

// impl tick::GetTick for ::Cortexm {
//     fn enable_tick(&self, ms_hz: u32) {    
//         let st = ::systick::SYSTICK;
//         st.set_reload_value(ms_hz);
//         st.set_current_value(ms_hz);
//         st.set_enabled(true);
//         st.set_tick_interrupt(true);        
//     }

//     fn disable_tick(&self) {
//         let st = ::systick::SYSTICK;
//         st.set_tick_interrupt(false);
//         st.set_enabled(false);        
//     }
// }
