pub use ::chip::tim_gen::*;

pub enum Direction {
    Up = 0,
    Down = 1,
}

pub enum OcMode {
    Frozen =        0b0000,
    Active =        0b0001,
    Inactive =      0b0010,
    Toggle =        0b0011,
    ForceInactive = 0b0100,
    ForceActive =   0b0101,
    Pwm1 =          0b0110,
    Pwm2 =          0b0111,
    CombinedPwm1 =  0b1100,
    CombinedPwm2 =  0b1101,
    AsymPwm1 =      0b1110,
    AsymPwm2 =      0b1111,
}

pub enum CcSelect {
    Output   = 0b00,
    InputA   = 0b01,
    InputB   = 0b10,
    InputTRC = 0b11,
}

pub const CH1: usize = 0;
pub const CH2: usize = 1;
pub const CH3: usize = 2;
pub const CH4: usize = 3;

pub trait TimGenExt {
    fn set_enabled(&self, value: bool) -> &Self;
    fn set_direction(&self, value: Direction) -> &Self;
    fn set_prescaler(&self, value: u16) -> &Self;
    fn set_update_event(&self) -> &Self;
    fn update_interrupt_flag(&self) -> bool;
    fn clr_update_interrupt_flag(&self) -> &Self;
    fn set_auto_reload(&self, value: u32) -> &Self;
    fn delay(&self, reload: u32, prescaler: u16);
}

impl TimGenExt for TimGenImpl {
    fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_cr1(|r| r.set_cen(value))
    }

    fn set_direction(&self, value: Direction) -> &Self {
        self.with_cr1(|r| r.set_dir(value as u32))
    }

    fn set_prescaler(&self, value: u16) -> &Self {
        self.set_psc(Psc(0).set_psc(value as u32))
    }

    fn set_update_event(&self) -> &Self {
        self.set_egr(Egr(0).set_ug(1))
    }

    fn update_interrupt_flag(&self) -> bool {
        self.sr().uif() != 0
    }

    fn clr_update_interrupt_flag(&self) -> &Self {
        self.with_sr(|r| r.set_uif(0))
    }    

    fn set_auto_reload(&self, value: u32) -> &Self {
        self.set_arr(Arr(value))
    }

    fn delay(&self, reload: u32, prescaler: u16) {
        self
            .set_prescaler(prescaler)
            .set_update_event()
            .clr_update_interrupt_flag()
            .set_auto_reload(reload)
            .set_enabled(true);
        while self.update_interrupt_flag() == false {}
        self
            .clr_update_interrupt_flag()
            .set_enabled(false);
    }    
}