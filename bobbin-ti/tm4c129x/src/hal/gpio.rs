pub use bobbin_common::digital::*;
use bobbin_common::*;
pub use chip::gpio::*;
pub use super::sysctl::SysctlEnabled;
use chip::sig::{SignalTx, SignalRx, SignalCcp, SignalPwm, SignalAin};

pub enum Dir {
    In = 0,
    Out = 1,    
}

pub trait ModeTx<SIG, PERIPH> {
    fn mode_tx(&self, _: &PERIPH) -> &Self;
}

pub trait AltFnTx<SIG, PERIPH> {
    fn alt_fn_tx(&self, _: &PERIPH) -> u8;
}

pub trait ModeRx<SIG, PERIPH> {
    fn mode_rx(&self, _: &PERIPH) -> &Self;
}

pub trait ModeCcp<SIG, PERIPH> {
    fn mode_ccp(&self, _: &PERIPH) -> &Self;
}

pub trait ModePwm<SIG, PERIPH> {
    fn mode_pwm(&self, _: &PERIPH) -> &Self;
}

pub trait ModeAin<SIG, PERIPH> {
    fn mode_ain(&self, _: &PERIPH) -> &Self;
}

// impl<PERIPH, PIN, SIG> ModeTx<SIG, PERIPH> for PIN where PERIPH: SignalTx<SIG>, PIN: AltFn<SIG> {
//     fn mode_tx(&self, _: &PERIPH) -> &Self {
//         self.mode_altfn(self.alt_fn());
//         self
//     }
// }

impl<PERIPH, PIN, SIG> AltFnTx<SIG, PERIPH> for PIN where PERIPH: SignalTx<SIG>, PIN: AltFn<SIG> {
    fn alt_fn_tx(&self, _: &PERIPH) -> u8 {
        self.alt_fn()
    }
}

// impl<PERIPH, PIN, SIG> ModeRx<SIG, PERIPH> for PIN where PERIPH: SignalRx<SIG>, PIN: AltFn<SIG>, PIN: Pin<GpioPeriph> {
//     fn mode_rx(&self, _: &PERIPH) -> &Self {
//         self.mode_altfn(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeCcp<SIG, PERIPH> for PIN where PERIPH: SignalCcp<SIG>, PIN: AltFn<SIG>, PIN: Pin<GpioPeriph> {
//     fn mode_ccp(&self, _: &PERIPH) -> &Self {
//         self.mode_altfn(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModePwm<SIG, PERIPH> for PIN where PERIPH: SignalPwm<SIG>, PIN: AltFn<SIG>, PIN: Pin<GpioPeriph> {
//     fn mode_pwm(&self, _: &PERIPH) -> &Self {
//         self.mode_altfn(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeAin<SIG, PERIPH> for PIN where PERIPH: SignalAin<SIG>, PIN: AltFn<SIG>, PIN: Pin<GpioPeriph> {
//     fn mode_ain(&self, _: &PERIPH) -> &Self {
//         self.mode_altfn(self.alt_fn());        
//         self.set_digital_enable(false).set_analog_select(true);
//         self
//     }
// }

// pub trait GpioExt {
//     fn set_dir(&self, value: Dir) -> &Self;
//     fn set_afsel(&self, value: bool) -> &Self;
//     fn set_pullup_select(&self, value: bool) -> &Self;
//     fn set_pulldown_select(&self, value: bool) -> &Self;
//     fn set_open_drain_select(&self, value: bool) -> &Self;
//     fn set_digital_enable(&self, value: bool) -> &Self;
//     fn set_analog_select(&self, value: bool) -> &Self;
//     fn set_port_control(&self, value: usize) -> &Self;

//     fn mode_input(&self) -> &Self {
//         self.set_dir(Dir::In);
//         self.set_afsel(false);
//         self.set_digital_enable(true);
//         self.set_port_control(0);
//         self
//     }

//     fn mode_output(&self) -> &Self {
//         self.set_dir(Dir::Out);
//         self.set_afsel(false);
//         self.set_digital_enable(true);
//         self.set_port_control(0);
//         self
//     }    

//     fn mode_altfn(&self, value: usize) -> &Self {
//         self.set_dir(Dir::In);
//         self.set_afsel(true);
//         self.set_digital_enable(true);
//         self.set_port_control(value);
//         self
//     }

//     fn pull_up(&self) -> &Self {
//         self.set_pullup_select(true)
//     }

//     fn pull_down(&self) -> &Self {
//         self.set_pulldown_select(true)
//     }    
// }

impl<T: Deref<Target=GpioPeriph>> Pin<T> {
    pub fn set_dir(&self, value: Dir) -> &Self {
        self.port().with_dir(|r| r.set_dir(self.index(), value as u32));
        self
    }

    pub fn set_afsel(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port().with_afsel(|r| r.set_afsel(self.index(), value));
        self
    }

    pub fn set_pullup_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port().with_pur(|r| r.set_pue(self.index(), value));
        self
    }

    pub fn set_pulldown_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port().with_pdr(|r| r.set_pde(self.index(), value));
        self
    }

    pub fn set_open_drain_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port().with_odr(|r| r.set_ode(self.index(), value));
        self
    }

    pub fn set_digital_enable(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port().with_den(|r| r.set_den(self.index(), value));
        self
    }

    pub fn set_analog_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port().with_amsel(|r| r.set_gpioamsel(self.index(), value));
        self
    }

    pub fn set_port_control(&self, value: usize) -> &Self {
        self.port().with_pctl(|r| r.set_pmc(self.index(), value as u32));
        self
    }

    pub fn mode_input(&self) -> &Self {
        self.set_dir(Dir::In);
        self.set_afsel(false);
        self.set_digital_enable(true);
        self.set_port_control(0);
        self
    }

    pub fn mode_output(&self) -> &Self {
        self.set_dir(Dir::Out);
        self.set_afsel(false);
        self.set_digital_enable(true);
        self.set_port_control(0);
        self
    }    

    pub fn mode_altfn(&self, value: usize) -> &Self {
        self.set_dir(Dir::In);
        self.set_afsel(true);
        self.set_digital_enable(true);
        self.set_port_control(value);
        self
    }

    pub fn pull_up(&self) -> &Self {
        self.set_pullup_select(true)
    }

    pub fn pull_down(&self) -> &Self {
        self.set_pulldown_select(true)
    }    
}

impl<T: Deref<Target=GpioPeriph>> DigitalInput for Pin<T>  {        
    fn input(&self) -> bool {            
        self.port().data().data(self.index()) != 0
    }           
}

// impl<T: Deref<Target=GpioPeriph>> DigitalOutput for Pin<T> {
//     fn output(&self) -> bool {
//         self.port().data().data(self.index()) != 0
//     }   

//     fn set_output(&self, value: bool) -> &Self {
//         let value = if value { 1 } else { 0 };
//         self.port().with_data(|r| r.set_data(self.index(), value));
//         self
//     }    

//     fn toggle_output(&self) -> &Self {
//         self.set_output(!self.output())
//     }
// }

impl DigitalOutput for Pin<Gpiof> {
    fn output(&self) -> bool {
        self.port().data().data(self.index()) != 0
    }   

    fn set_output(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port().with_data(|r| r.set_data(self.index(), value));
        self
    }    

    fn toggle_output(&self) -> &Self {
        self.set_output(!self.output())
    }    
}