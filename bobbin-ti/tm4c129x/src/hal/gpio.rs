pub use chip::gpio::*;
pub use super::sysctl::SysctlEnabled;
use chip::sig::{SignalTx, SignalRx, SignalCcp, SignalPwm};

pub enum Dir {
    In = 0,
    Out = 1,    
}

pub trait ModeTx<T, S> {
    fn mode_tx(&self, _: &S) -> &Self;
}

pub trait ModeRx<T, S> {
    fn mode_rx(&self, _: &S) -> &Self;
}

pub trait ModeCcp<T, S> {
    fn mode_ccp(&self, _: &S) -> &Self;
}

pub trait ModePwm<T, S> {
    fn mode_pwm(&self, _: &S) -> &Self;
}


impl<P, O, S, T> ModeTx<T, S> for Pin<P, O> where S: SignalTx<T>, P: AltFn<T> {
    fn mode_tx(&self, _: &S) -> &Self {
        self.mode_altfn(self.id.alt_fn());
        self
    }
}

impl<P, O, S, T> ModeRx<T, S> for Pin<P, O> where S: SignalRx<T>, P: AltFn<T> {
    fn mode_rx(&self, _: &S) -> &Self {
        self.mode_altfn(self.id.alt_fn());
        self
    }
}

impl<P, O, S, T> ModeCcp<T, S> for Pin<P, O> where S: SignalCcp<T>, P: AltFn<T> {
    fn mode_ccp(&self, _: &S) -> &Self {
        self.mode_altfn(self.id.alt_fn());
        self
    }
}

impl<P, O, S, T> ModePwm<T, S> for Pin<P, O> where S: SignalPwm<T>, P: AltFn<T> {
    fn mode_pwm(&self, _: &S) -> &Self {
        self.mode_altfn(self.id.alt_fn());
        self
    }
}

pub trait GpioExt {
    fn set_dir(&self, value: Dir) -> &Self;
    fn set_afsel(&self, value: bool) -> &Self;
    fn set_pullup_select(&self, value: bool) -> &Self;
    fn set_pulldown_select(&self, value: bool) -> &Self;
    fn set_open_drain_select(&self, value: bool) -> &Self;
    fn set_digital_enable(&self, value: bool) -> &Self;
    fn set_analog_select(&self, value: bool) -> &Self;
    fn set_port_control(&self, value: usize) -> &Self;
    fn mode_input(&self) -> &Self;
    fn mode_output(&self) -> &Self;
    fn mode_altfn(&self, value: usize) -> &Self;
    fn pull_up(&self) -> &Self;
    fn pull_down(&self) -> &Self;
    fn input(&self) -> bool;
    fn output(&self) -> bool;
    fn set_output(&self, value: bool) -> &Self;
    fn toggle_output(&self) -> &Self;
}

impl<P, T> GpioExt for Pin<P, T> {
    fn set_dir(&self, value: Dir) -> &Self {
        self.port.with_dir(|r| r.set_dir(self.index, value as u32));
        self
    }

    fn set_afsel(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_afsel(|r| r.set_afsel(self.index, value));
        self
    }

    fn set_pullup_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_pur(|r| r.set_pue(self.index, value));
        self
    }

    fn set_pulldown_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_pdr(|r| r.set_pde(self.index, value));
        self
    }

    fn set_open_drain_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_odr(|r| r.set_ode(self.index, value));
        self
    }

    fn set_digital_enable(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_den(|r| r.set_den(self.index, value));
        self
    }

    fn set_analog_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_amsel(|r| r.set_gpioamsel(self.index, value));
        self
    }

    fn set_port_control(&self, value: usize) -> &Self {
        self.port.with_pctl(|r| r.set_pmc(self.index, value as u32));
        self
    }

    fn mode_input(&self) -> &Self {
        self.set_dir(Dir::In);
        self.set_afsel(false);
        self.set_digital_enable(true);
        self.set_port_control(0);
        self
    }

    fn mode_output(&self) -> &Self {
        self.set_dir(Dir::Out);
        self.set_afsel(false);
        self.set_digital_enable(true);
        self.set_port_control(0);
        self
    }    

    fn mode_altfn(&self, value: usize) -> &Self {
        self.set_dir(Dir::In);
        self.set_afsel(true);
        self.set_digital_enable(true);
        self.set_port_control(value);
        self
    }

    fn pull_up(&self) -> &Self {
        self.set_pullup_select(true)
    }

    fn pull_down(&self) -> &Self {
        self.set_pulldown_select(true)
    }
    
    fn input(&self) -> bool {            
        self.port.data().data(self.index) != 0
    }           

    fn output(&self) -> bool {
        self.port.data().data(self.index) != 0
    }   

    fn set_output(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_data(|r| r.set_data(self.index, value));
        self
    }    

    fn toggle_output(&self) -> &Self {
        self.set_output(!self.output())
    }
}

