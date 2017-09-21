pub use ::chip::wwdg::*;

pub const T_MAX: u32 = 0x7f;
pub const T_MIN: u32 = 0x40;

pub enum TimeBase {
    Div1 = 0b00,
    Div2 = 0b01,
    Div4 = 0b10,
    Div8 = 0b11,
}

pub struct Config {
    pub early_wake_interrupt: bool,
    pub time_base: TimeBase,
    pub window: u32,
}

pub fn configure(mut wwdg: Wwdg, cfg: Config) {
    unsafe {
        wwdg.set_cfr(Cfr(0)
            .set_ewi(if cfg.early_wake_interrupt { 1 } else { 0 })
            .set_wdgtb(cfg.time_base as u32)
            .set_w(cfg.window)
        );
    }
}

pub fn activate(mut wwdg: Wwdg, t: u32) {
    unsafe {
        wwdg.set_cr(Cr(0).set_wdga(1).set_t(t))
    }
}

pub fn refresh(mut wwdg: Wwdg, t: u32) {
    unsafe {
        wwdg.set_cr(Cr(0).set_t(t))
    }
}

pub fn ewif(wwdg: Wwdg) -> bool {
    unsafe {
        wwdg.sr().ewif() != 0
    }
}

pub fn clr_ewif(mut wwdg: Wwdg) {
    unsafe {
        wwdg.set_sr(Sr(0).set_ewif(1))
    }
}