pub use ::chip::port::*;
use ::chip::gpio::*;
use ::chip::irq::*;
use nvic;

use core::marker::PhantomData;
//use embed_common::io::{DigitalIn, DigitalOut};

pub enum Pull {
    None,
    PullUp,
    PullDown,
}

pub enum InterruptConfig {
    Disabled = 0b0000,
    DmaRisingEdge = 0b0001,
    DmaFallingEdge = 0b0010,
    DmaEitherEdge = 0b0011,
    IrqZero = 0b1000,
    IrqRisingEdge = 0b1001,
    IrqFallingEdge = 0b1010,
    IrqEitherEdge = 0b1011,
    IrqOne = 0b1100,
}

pub struct ModeUnknown {}
pub struct ModeAnalog {}
pub struct ModeInput {}
pub struct ModeOutput {}
pub struct ModeAltFn {}

pub type PinUnknown = Pin<ModeUnknown>;
pub type PinAnalog = Pin<ModeAnalog>;
pub type PinInput = Pin<ModeInput>;
pub type PinOutput = Pin<ModeOutput>;
pub type PinAltFn = Pin<ModeAltFn>;

pub struct PortDevice {
    port: Port,
}

pub fn device(port: Port) -> PortDevice {
    PortDevice { port: port }
}

impl PortDevice {
    pub fn irq(&self) -> Irq {
        match self.port {
            PORTA => IRQ_PORTA,
            PORTB => IRQ_PORTB,
            PORTC => IRQ_PORTC,
            PORTD => IRQ_PORTD,
            PORTE => IRQ_PORTE,
            _ => panic!("Invalid Port"),
        }
    }

    pub fn set_handler(&self, value: Option<unsafe extern "C" fn()>) {
        let irq = self.irq();
        irq.set_handler(value);
        nvic::set_enabled(irq.0, value.is_some());
    }
}

pub struct Pin<T> {
    port: Port,
    index: usize,
    _phantom: PhantomData<T>,
}

pub fn pin(port: Port, index: usize) -> Pin<ModeUnknown> {
    Pin { port: port, index: index, _phantom: PhantomData }
}

impl<T> Pin<T> {
    pub fn port(&self) -> PortDevice {
        device(self.port)
    }
    pub fn gpio(&self) -> Gpio {
        match self.port {
            PORTA => GPIOA,
            PORTB => GPIOB,
            PORTC => GPIOC,
            PORTD => GPIOD,
            PORTE => GPIOE,
            _ => unimplemented!(),
        }
    }

    fn set_mux(&self, value: u8) {
        let mut port = self.port;
        unsafe {
            port.with_pcr(self.index, |r| r.set_mux(value as u32))
        }
    }

    pub fn into_altfn(self, value: u8) -> Pin<ModeAltFn> {
        assert!(value > 1, "ModeAltFn only for mux values 2-7");
        self.set_mux(value);
        Pin { port: self.port, index: self.index, _phantom: PhantomData }
    }

    pub fn into_altfn_unchecked(self) -> Pin<ModeAltFn> {
        Pin { port: self.port, index: self.index, _phantom: PhantomData }
    }

    pub fn into_analog(self) -> Pin<ModeAnalog> {
        self.set_mux(0);
        Pin { port: self.port, index: self.index, _phantom: PhantomData }
    }

    pub fn into_input(self) -> Pin<ModeInput> {
        self.set_mux(1);
        unsafe {
            self.gpio().with_pddr(|r| r.set_pdd(self.index, 0));
        }
        Pin { port: self.port, index: self.index, _phantom: PhantomData }
    }    

    pub fn into_output(self) -> Pin<ModeOutput> {
        self.set_mux(1);
        unsafe {
            self.gpio().with_pddr(|r| r.set_pdd(self.index, 1));
        }
        Pin { port: self.port, index: self.index, _phantom: PhantomData }
    }

    pub fn set_ode(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut port = self.port;
        unsafe {
            port.with_pcr(self.index, |r| r.set_ode(value))
        }        
    }
    
    pub fn with_pull(self, value: Pull) -> Pin<T> {
        let mut port = self.port;
        unsafe {
            match value {
                Pull::None => {
                    port.with_pcr(self.index, |r| r.set_pe(0))
                },
                Pull::PullDown => {
                    port.with_pcr(self.index, |r| r.set_pe(1).set_ps(0))
                },
                Pull::PullUp => {
                    port.with_pcr(self.index, |r| r.set_pe(1).set_ps(1))
                }
            }
        }    
        Pin { port: self.port, index: self.index, _phantom: PhantomData }
    }
}


impl Pin<ModeOutput> {
    pub fn get(&self) -> bool {
        unsafe {
            self.gpio().pdor().pdo(self.index) != 0
        }
    }

    pub fn set(&self, value: bool) {
        unsafe {
            if value {
                self.gpio().set_psor(Psor(0).set_ptso(self.index, 1))
            } else {
                self.gpio().set_pcor(Pcor(0).set_ptco(self.index, 1))
            }
        }
    }
    pub fn toggle(&self) {
        unsafe {
            self.gpio().set_ptor(Ptor(0).set_ptto(self.index, 1))
        }
    }    
}

impl Pin<ModeInput> {
    pub fn isf(&self) -> bool {
        let port = self.port;
        unsafe {
            port.pcr(self.index).isf() != 0
        }
    }

    pub fn clr_isf(&self) {
        let mut port = self.port;
        unsafe {
            port.with_pcr(self.index, |r| r.set_isf(1));
        }
    }

    pub fn irqc(&self) -> InterruptConfig {
        let port = self.port;
        unsafe {
            match port.pcr(self.index).irqc() {
                0b0000 => InterruptConfig::Disabled,
                0b0001 => InterruptConfig::DmaRisingEdge,
                0b0010 => InterruptConfig::DmaFallingEdge,
                0b0011 => InterruptConfig::DmaEitherEdge,
                0b1000 => InterruptConfig::IrqZero,
                0b1001 => InterruptConfig::IrqRisingEdge,
                0b1010 => InterruptConfig::IrqFallingEdge,
                0b1011 => InterruptConfig::IrqEitherEdge,
                0b1100 => InterruptConfig::IrqZero,
                _ => panic!("Invalid InterruptConfig value"),
            }
        }
    }

    pub fn set_irqc(&self, value: InterruptConfig) {
        let mut port = self.port;
        unsafe {
            port.with_pcr(self.index, |r| r.set_irqc(value as u32))
        }
    }

    pub fn get(&self) -> bool {
        unsafe {
            self.gpio().pdir().pdi(self.index) != 0
        }
    }
}

// impl DigitalOut for Pin<ModeOutput> {
//     fn digital_out(&self) -> bool {
//         unsafe {
//             self.gpio().pdor().pdo(self.index) != 0
//         }
//     }

//     fn set_digital_out(&self, value: bool) {
//         unsafe {
//             if value {
//                 self.gpio().set_psor(Psor(0).set_ptso(self.index, 1))
//             } else {
//                 self.gpio().set_pcor(Pcor(0).set_ptco(self.index, 1))
//             }
//         }
//     }
//     fn toggle_digital_out(&self) {        
//         unsafe {
//             self.gpio().set_ptor(Ptor(0).set_ptto(self.index, 1))
//         }
//     }    
// }

// impl DigitalIn for Pin<ModeInput> {
//     fn digital_in(&self) -> bool {
//         unsafe {
//             self.gpio().pdir().pdi(self.index) != 0
//         }
//     }
// }