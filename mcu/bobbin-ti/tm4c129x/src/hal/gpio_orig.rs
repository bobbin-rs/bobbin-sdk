pub use ::chip::gpio::*;
pub use sysctl;

use core::marker::PhantomData;
// use embed_common::io::{DigitalIn, DigitalOut};

pub struct ModeUnknown;
pub struct ModeDisabled;
pub struct ModeInput;
pub struct ModeOutput;
pub struct ModeAltFn;

pub type PinUnknown = Pin<ModeUnknown>;
pub type PinDisabled = Pin<ModeDisabled>;
pub type PinInput = Pin<ModeInput>;
pub type PinOutput = Pin<ModeOutput>;
pub type PinAltFn = Pin<ModeAltFn>;

pub struct Pin<M> {
    gpio: Gpio,
    pin: usize,
    _phantom: PhantomData<M>,
}

pub fn pin(gpio: Gpio, index: usize) -> Pin<ModeUnknown> {
    Pin {
        gpio: gpio,
        pin: index,
        _phantom: PhantomData,
    }
}

impl<M> Pin<M> {
    pub fn into_disabled(self) -> Pin<ModeDisabled> {
        let mut gpio = self.gpio;
        unsafe {            
            gpio.with_dir(|r| r.set_dir(self.pin, 0));
            gpio.with_afsel(|r| r.set_afsel(self.pin, 0));
            gpio.with_den(|r| r.set_den(self.pin, 0));  
            gpio.with_pctl(|r| r.set_pmc(self.pin, 0));          
        }        
        Pin { gpio: self.gpio, pin: self.pin, _phantom: PhantomData }
    }

    pub fn into_disabled_unchecked(self) -> Pin<ModeDisabled> {
        Pin { gpio: self.gpio, pin: self.pin, _phantom: PhantomData }
    }

    pub fn into_output(self) -> Pin<ModeOutput> {
        sysctl::set_gpio_enabled(self.gpio, true);
        let mut gpio = self.gpio;
        unsafe {            
            gpio.with_dir(|r| r.set_dir(self.pin, 1));
            gpio.with_afsel(|r| r.set_afsel(self.pin, 0));
            gpio.with_den(|r| r.set_den(self.pin, 1));            
            gpio.with_pctl(|r| r.set_pmc(self.pin, 0));
        }
        Pin { gpio: self.gpio, pin: self.pin, _phantom: PhantomData }
    }

    pub fn into_output_unchecked(self) -> Pin<ModeOutput> {
        Pin { gpio: self.gpio, pin: self.pin, _phantom: PhantomData }
    }

    pub fn into_input(self) -> Pin<ModeInput> {
        sysctl::set_gpio_enabled(self.gpio, true);
        let mut gpio = self.gpio;
        unsafe {            
            gpio.with_dir(|r| r.set_dir(self.pin, 0));
            gpio.with_afsel(|r| r.set_afsel(self.pin, 0));
            gpio.with_den(|r| r.set_den(self.pin, 1));            
            gpio.with_pctl(|r| r.set_pmc(self.pin, 0));
        }
        Pin { gpio: self.gpio, pin: self.pin, _phantom: PhantomData }
    }    

    pub fn into_input_unchecked(self) -> Pin<ModeInput> {
        Pin { gpio: self.gpio, pin: self.pin, _phantom: PhantomData }
    }
    

    pub fn into_alt_fn(self, value: u8) -> Pin<ModeAltFn> {
        sysctl::set_gpio_enabled(self.gpio, true);
        let mut gpio = self.gpio;
        unsafe {            
            gpio.with_dir(|r| r.set_dir(self.pin, 0));            
            gpio.with_afsel(|r| r.set_afsel(self.pin, 1));
            gpio.with_den(|r| r.set_den(self.pin, 1));
            gpio.with_pctl(|r| r.set_pmc(self.pin, value as u32));
        }
        Pin { gpio: self.gpio, pin: self.pin, _phantom: PhantomData }
    }     

    pub fn into_alt_fn_unchecked(self) -> Pin<ModeAltFn> {
        Pin { gpio: self.gpio, pin: self.pin, _phantom: PhantomData }
    }

    pub fn with_open_drain(self, value: bool) -> Pin<M> {
        let value = if value { 1 } else { 0 };
        let mut gpio = self.gpio;
        unsafe {            
            gpio.with_odr(|r| r.set_ode(self.pin, value));
        }        
        Pin { gpio: self.gpio, pin: self.pin, _phantom: PhantomData }        
    }

    pub fn with_pullup(self, value: bool) -> Pin<M> {
        let value = if value { 1 } else { 0 };
        let mut gpio = self.gpio;
        unsafe {            
            gpio.with_pur(|r| r.set_pue(self.pin, value));
        }        
        Pin { gpio: self.gpio, pin: self.pin, _phantom: PhantomData }
    }
}

impl Pin<ModeOutput> {
    pub fn get(&self) -> bool {
        let gpio = self.gpio;
        unsafe {
            gpio.data().data(self.pin) != 0
        }
    }   

    pub fn set(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut gpio = self.gpio;
        unsafe {
            gpio.with_data(|r| r.set_data(self.pin, value));
        }
    }    
    pub fn toggle(&self) {
        self.set(!self.get())
    }
}

impl Pin<ModeInput> {
    pub fn get(&self) -> bool {
        let gpio = self.gpio;
        unsafe {
            gpio.data().data(self.pin) != 0
        }
    }    
}

// impl DigitalOut for Pin<DigitalOutput> {

//     fn digital_out(&self) -> bool {
//         let gpio = self.gpio;
//         unsafe {
//             gpio.data().data(self.pin) != 0
//         }
//     }    
//     fn set_digital_out(&self, value: bool) {
//         let value = if value { 1 } else { 0 };
//         let mut gpio = self.gpio;
//         unsafe {
//             gpio.with_data(|r| r.set_data(self.pin, value));
//         }
//     }    
// }

// impl DigitalIn for Pin<DigitalInput> {
//     fn digital_in(&self) -> bool {
//         let gpio = self.gpio;
//         unsafe {
//             gpio.data().data(self.pin) != 0
//         }
//     }    
// }

impl Pin<ModeAltFn> {
    pub fn af(&self) -> u8 {
        let gpio = self.gpio;
        unsafe {
            gpio.pctl().pmc(self.pin) as u8
        }
    }
}