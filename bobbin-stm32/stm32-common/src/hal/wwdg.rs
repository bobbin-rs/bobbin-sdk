pub use chip::wwdg::*;


pub const T_MAX: u32 = 0x7f;
pub const T_MIN: u32 = 0x40;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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


pub trait WwdgExt {
    fn configure(&self, cfg: Config) -> &Self;
    fn activate(&self, u32) -> &Self;
    fn refresh(&self, u32) -> &Self;
}

impl<T> WwdgExt for Periph<T> {
    fn configure(&self, cfg: Config) -> &Self {
        self.set_cfr(|r| r
            .set_ewi(if cfg.early_wake_interrupt { 1 } else { 0 })
            .set_wdgtb(cfg.time_base as u32)
            .set_w(cfg.window)
        )
    }

    fn activate(&self, t: u32) -> &Self {
        self.set_cr(|r| r.set_wdga(1).set_t(t))
    }

    fn refresh(&self, t: u32) -> &Self {
        self.set_cr(|r| r.set_t(t))
    }    
}