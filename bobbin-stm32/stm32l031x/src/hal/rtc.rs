use ::chip::rtc::*;

#[derive(Debug)]
pub struct Date(pub Dr);

#[derive(Debug)]
pub struct Time(pub Tr);

impl Date {
    pub fn year(&self) -> u32 {
        2000 + self.0.yt() * 10 + self.0.yu()
    }
    pub fn month(&self) -> u32 {
        self.0.mt() * 10 + self.0.mu()
    }
    pub fn day(&self) -> u32 {
        self.0.dt() * 10 + self.0.du()
    }
}

impl Time {
    pub fn hour(&self) -> u32 {
        self.0.ht() * 10 + self.0.hu()
    }
    pub fn minute(&self) -> u32 {
        self.0.mnt() * 10 + self.0.mnu()
    }
    pub fn second(&self) -> u32 {
        self.0.st() * 10 + self.0.su()
    }
}

pub enum WakeupClock {
    Div16 = 0b000,
    Div8 = 0b001,
    Div4 = 0b010,
    Div2 = 0b011,
    Ckspre = 0b100,
    CkspreWut = 0b110,
}


pub fn initialized() -> bool {
    unsafe {
        RTC.isr().inits() != 0
    }
}

pub fn unlock() {
    unsafe {
        RTC.set_wpr(Wpr(0).set_key(0xca));
        RTC.set_wpr(Wpr(0).set_key(0x53));        
    }
}

pub fn lock() {
    unsafe {
        RTC.set_wpr(Wpr(0).set_key(0xff));
    }    
}

pub fn init() -> bool {
    unsafe { RTC.isr().init() != 0 }
}

pub fn set_init(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        RTC.with_isr(|r| r.set_init(value));

        if value == 1 {
            while RTC.isr().initf() == 0 {}
        }
    }
}

pub fn set_prescaler(prediv_a: u8, prediv_s: u16) {
    unsafe {
        RTC.with_prer(|r| r.set_prediv_a(prediv_a as u32).set_prediv_s(prediv_s as u32));
    }
}

pub fn set_wakeup_clock(clocksel: WakeupClock) {
    unsafe {
        while RTC.isr().wutwf() == 0 {}
        RTC.with_cr(|r| r.set_wucksel(clocksel as u32))
    }
}

pub fn set_wakeup_timer(value: u16) {
    unsafe {
        while RTC.isr().wutwf() == 0 {}
        RTC.set_wutr(Wutr(0).set_wut(value as u32))
    }
}

pub fn set_wakeup_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        RTC.with_cr(|r| r.set_wute(value))
    }
}

pub fn set_wakeup_interrupt_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        RTC.with_cr(|r| r.set_wutie(value))
    }
}

pub fn wakeup_timer_flag() -> bool {
    unsafe {
        RTC.isr().wutf() != 0
    }    
}

pub fn clear_wakeup_timer_flag() {
    unsafe {
        RTC.with_isr(|r| r.set_wutf(0))
    }
}

pub fn date() -> Date {
    unsafe {
        Date(RTC.dr())
    }
}

pub fn time() -> Time {
    unsafe {
        Time(RTC.tr())
    }
}
