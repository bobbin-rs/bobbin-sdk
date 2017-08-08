pub use chip::iwdg::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Prescaler {
    Div4 = 0b000,
    Div8 = 0b001,
    Div16 = 0b010,
    Div32 = 0b011,
    Div64 = 0b100,
    Div128 = 0b101,
    Div256 = 0b110,    
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Config {
    pub prescaler: Prescaler,
    pub reload: u32,
    pub window: u32,
}

pub trait IwdgExt {
    fn configure(&self, cfg: Config) -> &Self;
    fn unlock(&self) -> &Self;
    fn lock(&self) -> &Self;
    fn refresh(&self) -> &Self;
    fn start(&self) -> &Self;
}

impl<T> IwdgExt for Periph<T> {
    fn configure(&self, cfg: Config) -> &Self {
        self.start().unlock();
        self.set_pr(|r| r.set_pr(cfg.prescaler as u32));
        self.set_rlr(|r| r.set_rl(cfg.reload));
        while self.sr().0 != 0 {}    
        self.set_winr(|r| r.set_win(cfg.window));
        self.refresh()
    }
    fn unlock(&self) -> &Self {
        self.set_kr(|r| r.set_key(0x5555))
    }
    fn lock(&self) -> &Self {
        self.set_kr(|r| r.set_key(0xABCD))
    }
    fn refresh(&self) -> &Self {
        self.set_kr(|r| r.set_key(0xAAAA))
    }
    fn start(&self) -> &Self {
        self.set_kr(|r| r.set_key(0xCCCC))
    }
}
