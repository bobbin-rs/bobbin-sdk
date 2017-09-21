use cortex_m::peripheral::{syst, syst_mut};

static mut TICK_COUNT_LO: u32 = 0;
static mut TICK_COUNT_HI: u32 = 0;

pub const SYSTICK: SysTick = SysTick {};

pub struct SysTick {}


pub fn init(sysclk_hz: u32) {
    set_reload(sysclk_hz / 1000);
    set_clk_src(true);
    set_enable(true);
    set_inten(true);        
}

pub fn now() -> u32 {
    unsafe { ::core::ptr::read_volatile(&TICK_COUNT_LO  as *const u32) }
}

pub fn wait(t: u32) -> u32 {
    wait_until(now() + t)
}

pub fn wait_until(t: u32) -> u32 {
    loop {
        let now = now();
        if now >= t { return now; }
        unsafe { asm!("wfi") }
    }
}

pub fn set_reload(value: u32) {
    assert!(value >> 24 == 0);
    unsafe { syst_mut().rvr.write(value) }
}


pub fn set_clk_src(value: bool) {
    unsafe {
        if value {
            syst_mut().csr.write(syst().csr.read() | 1 << 2);
        } else {
            syst_mut().csr.write(syst().csr.read() & !(1 << 2));
        }
    }
}    

pub fn set_inten(value: bool) {
    unsafe {
        if value {
            syst_mut().csr.write(syst().csr.read() | 1 << 1);
        } else {
            syst_mut().csr.write(syst().csr.read() & !(1 << 1));
        }
    }
}

pub fn set_enable(value: bool) {
    unsafe {
        if value {
            syst_mut().csr.write(syst().csr.read() | 1 << 0);
        } else {
            syst_mut().csr.write(syst().csr.read() & !(1 << 0));
        }
    }
}

pub fn current_value() -> u32 {
    syst().cvr.read()
}

#[export_name = "_systick"]
pub extern "C" fn systick_handler() {
    unsafe {        
        TICK_COUNT_LO += 1;
        if TICK_COUNT_LO == 0 {
            TICK_COUNT_HI += 1;
        }
    }
}