pub use periph::dma_f3::*;
// use bobbin_common::Channel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Burst {
    Single = 0b00,
    Incr4 = 0b01,
    Incr8 = 0b10,
    Incr16 = 0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low = 0b00,
    Medium = 0b01,
    High = 0b10,
    VeryHigh = 0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Bit8 = 0b00,
    Bit16 = 0b01,
    Bit32 = 0b10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    PtoM = 0b0,
    MtoP = 0b1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FifoStatus {
    Q1 = 0b000,
    Q2 = 0b001,
    Q3 = 0b010,
    Q4 = 0b011,
    Empty = 0b100,
    Full = 0b101,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FifoThreshold {
    Q1 = 0b00,
    Q2 = 0b01,
    Q3 = 0b10,
    Q4 = 0b11,
}

impl DmaCh {
    pub fn teif(&self) -> bool {
        self.periph.isr().teif(self.index) != 0
    }

    pub fn htif(&self) -> bool {
        self.periph.isr().htif(self.index) != 0
    }

    pub fn tcif(&self) -> bool {
        self.periph.isr().tcif(self.index) != 0
    }

    pub fn gif(&self) -> bool {
        self.periph.isr().gif(self.index) != 0
    }

    pub fn clr_teif(&self) -> &Self {
        self.periph.set_ifcr(|r| r.set_cteif(self.index, 1));
        self
    }      

    pub fn clr_tcif(&self) -> &Self {
        self.periph.set_ifcr(|r| r.set_ctcif(self.index, 1));
        self
    }      

    pub fn clr_htif(&self) -> &Self {
        self.periph.set_ifcr(|r| r.set_chtif(self.index, 1));
        self
    }      

    pub fn clr_gif(&self) -> &Self {
        self.periph.set_ifcr(|r| r.set_cgif(self.index, 1));
        self
    }      

    pub fn ccr(&self) -> Ccr {
        self.periph.ccr(self.index)
    }
    pub fn set_ccr(&self, value: Ccr) -> &Self {
        self.periph.set_ccr(self.index, |_| value);
        self
    }
    pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        self.periph.with_ccr(self.index, f);
        self
    }

    pub fn ndt(&self) -> u16 {
        self.periph.cndtr(self.index).ndt().into()
    }

    pub fn set_ndt(&self, value: u16) -> &Self {
        self.periph.set_cndtr(self.index, |r| r.set_ndt(value));
        self
    }

    pub fn pa(&self) -> u32 {
        self.periph.cpar(self.index).pa().into()
    }

    pub fn set_pa(&self, value: u32) -> &Self {
        self.periph.set_cpar(self.index, |r| r.set_pa(value));
        self
    }    
    
    pub fn ma(&self) -> u32 {
        self.periph.cmar(self.index).ma().into()
    }

    pub fn set_ma(&self, value: u32) -> &Self {
        self.periph.set_cmar(self.index, |r| r.set_ma(value));
        self
    }    

    pub fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_ccr(|r| r.set_en(value))        
    }
    pub fn set_dir(&self, value: Dir) -> &Self { self.with_ccr(|r| r.set_dir(value as u32)) }    
    pub fn set_psize(&self, value: Size) -> &Self { self.with_ccr(|r| r.set_psize(value as u32)) }
    pub fn set_msize(&self, value: Size) -> &Self { self.with_ccr(|r| r.set_msize(value as u32)) }

    pub fn set_mem2mem(&self, value: bool) -> &Self {
        self.with_ccr(|r| r.set_mem2mem(value))
    }

    pub fn set_pinc(&self, value: bool) -> &Self {
        self.with_ccr(|r| r.set_pinc(value))
    }

    pub fn set_minc(&self, value: bool) -> &Self {
        self.with_ccr(|r| r.set_minc(value))
    }

    pub fn set_circ(&self, value: bool) -> &Self {
        self.with_ccr(|r| r.set_circ(value))
    }

    pub fn set_tcie(&self, value: bool) -> &Self {
        self.with_ccr(|r| r.set_tcie(value))
    }    
}
