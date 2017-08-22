use bobbin_common::bits::*;
use chip::port::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
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

pub enum Pull {
    None,
    PullDown,
    PullUp,
}

pub trait PinExt {
    fn mux(&self) -> usize;
    fn set_mux(&self, value: usize) -> &Self;
    fn set_mux_gpio(&self) -> &Self;
    fn set_pull(&self, value: Pull) -> &Self;
    fn set_pull_down(&self) -> &Self;
    fn set_pull_up(&self) -> &Self;
    fn isf(&self) -> bool;
    fn clr_isf(&self) -> &Self;
    fn irqc(&self) -> InterruptConfig;
    fn set_irqc(&self, value: InterruptConfig) -> &Self;    
}

impl<P, T> PinExt for Pin<P, T> {
    fn mux(&self) -> usize {
        self.port.pcr(self.index).mux() as usize
    }
    fn set_mux(&self, value: usize) -> &Self {
        self.port.with_pcr(self.index, |r| r.set_mux(value as u32));
        self
    }
    fn set_mux_gpio(&self) -> &Self {
        self.set_mux(1)
    }
    fn set_pull(&self, value: Pull) -> &Self {
        self.port.with_pcr(self.index, |r| match value {
            Pull::None => r.set_pe(0).set_ps(0),
            Pull::PullDown => r.set_pe(1).set_ps(0),
            Pull::PullUp => r.set_pe(1).set_ps(1),
        });
        self
    }
    fn set_pull_down(&self) -> &Self {
        self.set_pull(Pull::PullDown)
    }
    fn set_pull_up(&self) -> &Self {
        self.set_pull(Pull::PullUp)
    }
    
    fn isf(&self) -> bool {
        self.port.pcr(self.index).isf() != 0
    }
    fn clr_isf(&self) -> &Self {
        self.port.with_pcr(self.index, |r| r.set_isf(1));
        self
    }
    fn irqc(&self) -> InterruptConfig {
        match self.port.pcr(self.index).irqc() {
            U4::B0000 => InterruptConfig::Disabled,
            U4::B0001 => InterruptConfig::DmaRisingEdge,
            U4::B0010 => InterruptConfig::DmaFallingEdge,
            U4::B0011 => InterruptConfig::DmaEitherEdge,
            U4::B0100 => unreachable!(),
            U4::B0101 => unreachable!(),
            U4::B0110 => unreachable!(),
            U4::B0111 => unreachable!(),
            U4::B1000 => InterruptConfig::IrqZero,
            U4::B1001 => InterruptConfig::IrqRisingEdge,
            U4::B1010 => InterruptConfig::IrqFallingEdge,
            U4::B1011 => InterruptConfig::IrqEitherEdge,
            U4::B1100 => InterruptConfig::IrqOne,
            U4::B1101 => unreachable!(),
            U4::B1110 => unreachable!(),
            U4::B1111 => unreachable!(),            
        }
    }
    fn set_irqc(&self, value: InterruptConfig) -> &Self {
        self.port.with_pcr(self.index, |r| r.set_irqc(value as u8));
        self
    }    
}
