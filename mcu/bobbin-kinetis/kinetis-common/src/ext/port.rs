use bobbin_bits::*;
pub use bobbin_mcu::pin::SetSource;
pub use port::*;

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

impl PortPin {
    pub fn mux(&self) -> U3 {
        self.port.pcr(self.index).mux().into()
    }
    pub fn set_mux<V: Into<U3>>(&self, value: V) -> &Self {
        self.port.with_pcr(self.index, |r| r.set_mux(value));
        self
    }

    pub fn set_mux_disabled(&self) -> &Self {
        self.set_mux(0)
    }

    pub fn set_mux_gpio(&self) -> &Self {
        self.set_mux(1)
    }
    pub fn set_pull(&self, value: Pull) -> &Self {
        self.port.with_pcr(self.index, |r| match value {
            Pull::None => r.set_pe(0).set_ps(0),
            Pull::PullDown => r.set_pe(1).set_ps(0),
            Pull::PullUp => r.set_pe(1).set_ps(1),
        });
        self
    }
    pub fn set_pull_none(&self) -> &Self {
        self.set_pull(Pull::None)
    }
    pub fn set_pull_down(&self) -> &Self {
        self.set_pull(Pull::PullDown)
    }
    pub fn set_pull_up(&self) -> &Self {
        self.set_pull(Pull::PullUp)
    }
    pub fn set_ode(&self, value: bool) -> &Self {
        self.port.with_pcr(self.index, |r| r.set_ode(value));
        self
    }
    pub fn isf(&self) -> bool {
        self.port.pcr(self.index).isf() != 0
    }
    pub fn clr_isf(&self) -> &Self {
        self.port.with_pcr(self.index, |r| r.set_isf(1));
        self
    }
    pub fn irqc(&self) -> InterruptConfig {
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
    pub fn set_irqc(&self, value: InterruptConfig) -> &Self {
        self.port.with_pcr(self.index, |r| r.set_irqc(value as u8));
        self
    }    
}


impl SetSource for PortPin {
    fn set_source<V: Into<U4>>(&self, src: V) {
        let src: u8 = src.into().value();
        self.set_mux(src);
    }
}