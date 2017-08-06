pub use chip::dma_f3::*;

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

pub trait ChannelExt {
    fn teif(&self) -> bool;    
    fn htif(&self) -> bool;
    fn tcif(&self) -> bool;
    fn gif(&self) -> bool;

    fn clr_teif(&self) -> &Self;
    fn clr_htif(&self) -> &Self;
    fn clr_tcif(&self) -> &Self;
    fn clr_gif(&self) -> &Self;
    
    fn ccr(&self) -> Ccr;
    fn set_ccr(&self, Ccr) -> &Self;
    fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, F) -> &Self;

    fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_ccr(|r| r.set_en(value))        
    }
    fn set_dir(&self, value: Dir) -> &Self { self.with_ccr(|r| r.set_dir(value as u32)) }    
    fn set_psize(&self, value: Size) -> &Self { self.with_ccr(|r| r.set_psize(value as u32)) }
    fn set_msize(&self, value: Size) -> &Self { self.with_ccr(|r| r.set_msize(value as u32)) }

    fn set_mem2mem(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_ccr(|r| r.set_mem2mem(value))
    }



    fn set_pinc(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_ccr(|r| r.set_pinc(value))
    }

    fn set_minc(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_ccr(|r| r.set_minc(value))
    }

    fn set_circ(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_ccr(|r| r.set_circ(value))
    }

    fn set_tcie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_ccr(|r| r.set_tcie(value))
    }    

    fn ndt(&self) -> u16;
    fn set_ndt(&self, value: u16) -> &Self;

    fn pa(&self) -> u32;
    fn set_pa(&self, value: u32) -> &Self;    

    fn ma(&self) -> u32;
    fn set_ma(&self, value: u32) -> &Self;    
}

impl<P, T> ChannelExt for Channel<P, T> {
    fn teif(&self) -> bool {
        self.periph().isr().teif(self.index()) != 0
    }

    fn htif(&self) -> bool {
        self.periph().isr().htif(self.index()) != 0
    }

    fn tcif(&self) -> bool {
        self.periph().isr().tcif(self.index()) != 0
    }

    fn gif(&self) -> bool {
        self.periph().isr().gif(self.index()) != 0
    }

    fn clr_teif(&self) -> &Self {
        self.periph().set_ifcr(Ifcr(0).set_cteif(self.index(), 1));
        self
    }      

    fn clr_tcif(&self) -> &Self {
        self.periph().set_ifcr(Ifcr(0).set_ctcif(self.index(), 1));
        self
    }      

    fn clr_htif(&self) -> &Self {
        self.periph().set_ifcr(Ifcr(0).set_chtif(self.index(), 1));
        self
    }      

    fn clr_gif(&self) -> &Self {
        self.periph().set_ifcr(Ifcr(0).set_cgif(self.index(), 1));
        self
    }      

    fn ccr(&self) -> Ccr {
        self.periph().ccr(self.index)
    }
    fn set_ccr(&self, value: Ccr) -> &Self {
        self.periph().set_ccr(self.index, value);
        self
    }
    fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        self.periph().with_ccr(self.index, f);
        self
    }

    fn ndt(&self) -> u16 {
        self.periph().cndtr(self.index).ndt().into()
    }

    fn set_ndt(&self, value: u16) -> &Self {
        self.periph().set_cndtr(self.index, Cndtr(0).set_ndt(value));
        self
    }

    fn pa(&self) -> u32 {
        self.periph().cpar(self.index).pa().into()
    }

    fn set_pa(&self, value: u32) -> &Self {
        self.periph().set_cpar(self.index, Cpar(0).set_pa(value));
        self
    }    
    
    fn ma(&self) -> u32 {
        self.periph().cmar(self.index).ma().into()
    }

    fn set_ma(&self, value: u32) -> &Self {
        self.periph().set_cmar(self.index, Cmar(0).set_ma(value));
        self
    }
}