use hal::cryotimer::*;

pub const CLOCK: u32 = 40_000_000;
pub const RATE: u32 = 1000;
pub const DIV: u32 = CLOCK / RATE;

pub fn init() {
}

pub fn delay(ms: u32) {
    CRYOTIMER.cmu_enable();

    // Use LFXO, Divide by 32 for 1024Hz
    CRYOTIMER.with_ctrl(|r| r.set_oscsel(2).set_presc(5));
    CRYOTIMER.set_periodsel(|r| r.set_periodsel(0));

    CRYOTIMER.set_ifc(|r| r.set_period(1));
    CRYOTIMER.with_ctrl(|r| r.set_en(1));
    for _ in 0..ms {       
        while CRYOTIMER._if().period() == 0 {}
        CRYOTIMER.set_ifc(|r| r.set_period(1));
    }
    CRYOTIMER.with_ctrl(|r| r.set_en(0));

    CRYOTIMER.cmu_disable();
}