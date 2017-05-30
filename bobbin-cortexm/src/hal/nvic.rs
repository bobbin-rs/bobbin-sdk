pub use ::chip::nvic::*;

pub fn enabled(irq: usize) -> bool {
    NVIC.iser((irq >> 5)).setena((irq & 0b11111)) != 0
}

pub fn set_enabled(irq: usize, value: bool) {
    if value {
        NVIC.set_iser((irq >> 5), Iser(0).set_setena((irq & 0b11111), 1));
    } else {
        NVIC.set_icer((irq >> 5), Icer(0).set_clrena((irq & 0b11111), 1));
    }        
}

pub fn pending(irq: usize) -> bool {
    NVIC.ispr((irq >> 5)).setpend((irq & 0b11111)) != 0
}

pub fn set_pending(irq: usize, value: bool) {
    if value {
        NVIC.set_ispr((irq >> 5), Ispr(0).set_setpend((irq & 0b11111), 1));
    } else {
        NVIC.set_icpr((irq >> 5), Icpr(0).set_clrpend((irq & 0b11111), 1));
    }        
}

pub fn active(irq: usize) -> bool {
    NVIC.iabr((irq >> 5)).active((irq & 0b11111)) != 0
}

pub fn priority(irq: usize) -> u8 {
    NVIC.ipr((irq >> 4)).pri((irq & 0b1111)) as u8
}

pub fn set_priority(irq: usize, value: u8) {
    NVIC.with_ipr((irq >> 4), |r| r.set_pri((irq & 0b1111), value as u32));
}

pub fn trigger_interrupt(irq: usize) {
    NVIC.set_stir(Stir(0).set_intid(irq as u32));
}