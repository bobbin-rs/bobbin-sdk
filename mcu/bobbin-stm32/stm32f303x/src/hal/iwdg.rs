pub use ::chip::iwdg::*;

pub enum Prescaler {
    Div4 = 0b000,
    Div8 = 0b001,
    Div16 = 0b010,
    Div32 = 0b011,
    Div64 = 0b100,
    Div128 = 0b101,
    Div256 = 0b110,    
}

pub struct Config {
    pub prescaler: Prescaler,
    pub reload: u32,
    pub window: u32,
}

pub fn configure(mut iwdg: Iwdg, cfg: Config) {
    start(iwdg);
    unlock(iwdg);
    unsafe {
        iwdg.set_pr(Pr(0).set_pr(cfg.prescaler as u32));
        iwdg.set_rlr(Rlr(0).set_rl(cfg.reload));
        while iwdg.sr().0 != 0 {}    
        iwdg.set_winr(Winr(0).set_win(cfg.window));    
    }
    refresh(iwdg)
}

pub fn unlock(mut iwdg: Iwdg) {
    unsafe {
        iwdg.set_kr(Kr(0).set_key(0x5555))
    }
}

pub fn lock(mut iwdg: Iwdg) {
    unsafe {
        iwdg.set_kr(Kr(0).set_key(0xABCD))
    }    
}

pub fn refresh(mut iwdg: Iwdg) {
    unsafe {
        iwdg.set_kr(Kr(0).set_key(0xAAAA))
    }
}

pub fn start(mut iwdg: Iwdg) {
    unsafe {
        iwdg.set_kr(Kr(0).set_key(0xCCCC))
    }
}