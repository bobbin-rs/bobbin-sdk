use bobbin_cortexm::chip::scb::SCB;
use chip::pwr::PWR;

pub const POWER: Power = Power {};

pub struct Power {}

impl Power {
    #[inline(always)]
    pub fn sleep_wfi() {
        SCB.with_scr(|r| r.set_sleepdeep(0).set_sleeponexit(0));
        unsafe { asm!("wfi") }
    }

    #[inline(always)]
    pub fn sleep_wfi_on_exit() {
        SCB.with_scr(|r| r.set_sleepdeep(0).set_sleeponexit(1));
        unsafe { asm!("wfi") }
    }

    #[inline(always)]
    pub fn sleep_wfe() {
        SCB.with_scr(|r| r.set_sleepdeep(0).set_sevonpend(1));
        unsafe { asm!("wfe") }
    }

    #[inline(always)]
    pub fn stop_wfi() {
        PWR.with_cr(|r| r.set_pdds(0));
        SCB.with_scr(|r| r.set_sleepdeep(1));
        unsafe { asm!("wfi") }
    }

    #[inline(always)]
    pub fn stop_wfe() {
        PWR.with_cr(|r| r.set_pdds(0));
        SCB.with_scr(|r| r.set_sleepdeep(1).set_sevonpend(1));
        unsafe { asm!("wfe") }
    }

    #[inline(always)]
    pub fn standby_wfi() {
        PWR.with_cr(|r| r.set_pdds(1));
        SCB.with_scr(|r| r.set_sleepdeep(1));
        unsafe { asm!("wfi") }
    }

    #[inline(always)]
    pub fn standby_wfe() {
        PWR.with_cr(|r| r.set_pdds(0));
        SCB.with_scr(|r| r.set_sleepdeep(1).set_sevonpend(1));    
        unsafe { asm!("wfe") }
    }
}