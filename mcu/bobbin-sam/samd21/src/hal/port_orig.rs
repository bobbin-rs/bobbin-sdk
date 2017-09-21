use core::marker::PhantomData;

use ::chip::port::{self, Port};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMux {
    PMuxA = 0,
    PMuxB = 1,
    PMuxC = 2,
    PMuxD = 3,
    PMuxE = 4,
    PMuxF = 5,
    PMuxG = 6,
    PMuxH = 7,
    PMuxI = 8,    
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Input = 0,
    Output = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DriveStrength {
    Normal = 0,
    Stronger = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PullEnable {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputEnable {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMuxEnable {
    Disabled,
    Enabled(u8),
}

pub struct ModeUnknown;
pub struct ModeOutput;
pub struct ModeInput;
pub struct ModePMux;

pub type PinUnknown = Pin<ModeUnknown>;
pub type PinOutput = Pin<ModeOutput>;
pub type PinInput = Pin<ModeInput>;
pub type PinPMux = Pin<ModePMux>;


pub struct Pin<M> {
    port: Port,
    index: usize,
    phantom: PhantomData<M>,
}

pub fn pin(port: Port, index: usize) -> Pin<ModeUnknown> {
    Pin { port: port, index: index, phantom: PhantomData }
}

impl<M> Pin<M> {
    pub fn set_pull(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut port = self.port;
        unsafe {
            port.with_pincfg(self.index, |r| r.set_pullen(value));
        }        
    }

    pub fn into_digital_output(self) -> Pin<ModeOutput> {
        let mut port = self.port;
        unsafe {
            port.set_dirset(port::Dirset(0).set_dirset(self.index, 1));
            port.with_pincfg(self.index, |r| r.set_pmuxen(0));
        }
        Pin { port: self.port, index: self.index, phantom: PhantomData }
    }

    pub fn into_digital_input(self) -> Pin<ModeInput> {
        let mut port = self.port;
        unsafe {
            port.set_dirclr(port::Dirclr(0).set_dirclr(self.index, 1));
            port.with_pincfg(self.index, |r| r.set_pmuxen(0));
        }
        Pin { port: self.port, index: self.index, phantom: PhantomData }
    }

    pub fn into_pmux(self, pmux: PMux) -> Pin<PMux> {
        let mut port = self.port;
        let pin = self.index;
        let pin_row = pin >> 1;
        let pin_col = pin & 1;
        unsafe {
            port.with_pincfg(self.index, |r| r.set_pmuxen(1));
            port.with_pmux(pin_row, |r| r.set_pmux(pin_col, pmux as u8));
        }
        Pin { port: self.port, index: self.index, phantom: PhantomData }
    }
}


impl Pin<ModeOutput> {
    pub fn get(&self) -> bool {
        let port = self.port;
        unsafe {
            port.out().out(self.index) != 0
        }        
    }

    pub fn set(&self, value: bool) {
        let mut port = self.port;
        unsafe {
            if value {
                port.set_outset(port::Outset(0).set_outset(self.index, 1))
            } else {
                port.set_outclr(port::Outclr(0).set_outclr(self.index, 1))
            }
        }
    }    
}

impl Pin<ModeInput> {
    pub fn get(&self) -> bool {
        let port =self.port;
        unsafe {
            port._in()._in(self.index) != 0
        }
    }    
}

// impl DigitalOut for Pin<ModeOutput> {
//     fn digital_out(&self) -> bool {
//         port::out(self.port, self.index)
//     }

//     fn set_digital_out(&self, value: bool) {
//         port::set_out(self.port, self.index, value);        
//     }    
// }

// impl DigitalIn for Pin<ModeInput> {
//     fn digital_in(&self) -> bool {
//        port::data_in(self.port, self.index)
//     }    
// }