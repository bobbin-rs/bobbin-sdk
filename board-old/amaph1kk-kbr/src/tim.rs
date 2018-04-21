use hal::stimer::*;

pub fn init() {
    STIMER
        .with_stcfg(|r| r.set_clksel(0x5).set_freeze(0));
}

pub fn delay(ms: u32) {
    STIMER
        .with_stcfg(|r| r.set_compare_en(0, 1))
        .set_stmintclr(|r| r.set_compare(0, 1))
        .set_scmpr(0, |r| r.set_value(ms));
    while STIMER.stmintstat().compare(0) == 0 {}
    STIMER.with_stcfg(|r| r.set_compare_en(0, 0));
}