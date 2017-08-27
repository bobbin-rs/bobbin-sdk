pub use chip::dma::*;

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
    PtoM = 0b00,
    MtoP = 0b01,
    MtoM = 0b10,    
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
    pub fn feif(&self) -> bool {
        let p = self.periph;
        match self.index {
            0 => p.lisr().feif0() != 0,
            1 => p.lisr().feif1() != 0,
            2 => p.lisr().feif2() != 0,
            3 => p.lisr().feif3() != 0,
            4 => p.hisr().feif4() != 0,
            5 => p.hisr().feif5() != 0,
            6 => p.hisr().feif6() != 0,
            7 => p.hisr().feif7() != 0,
            _ => panic!("Invalid channel index"),
        }
    }

    pub fn dmeif(&self) -> bool {
        let p = self.periph;
        match self.index {
            0 => p.lisr().dmeif0() != 0,
            1 => p.lisr().dmeif1() != 0,
            2 => p.lisr().dmeif2() != 0,
            3 => p.lisr().dmeif3() != 0,
            4 => p.hisr().dmeif4() != 0,
            5 => p.hisr().dmeif5() != 0,
            6 => p.hisr().dmeif6() != 0,
            7 => p.hisr().dmeif7() != 0,
            _ => panic!("Invalid channel index"),
        }
    }

    pub fn teif(&self) -> bool {
        let p = self.periph;
        match self.index {
            0 => p.lisr().teif0() != 0,
            1 => p.lisr().teif1() != 0,
            2 => p.lisr().teif2() != 0,
            3 => p.lisr().teif3() != 0,
            4 => p.hisr().teif4() != 0,
            5 => p.hisr().teif5() != 0,
            6 => p.hisr().teif6() != 0,
            7 => p.hisr().teif7() != 0,
            _ => panic!("Invalid channel index"),
        }
    }    

    pub fn htif(&self) -> bool {
        let p = self.periph;
        match self.index {
            0 => p.lisr().htif0() != 0,
            1 => p.lisr().htif1() != 0,
            2 => p.lisr().htif2() != 0,
            3 => p.lisr().htif3() != 0,
            4 => p.hisr().htif4() != 0,
            5 => p.hisr().htif5() != 0,
            6 => p.hisr().htif6() != 0,
            7 => p.hisr().htif7() != 0,
            _ => panic!("Invalid channel index"),
        }
    }    

    pub fn tcif(&self) -> bool {
        let p = self.periph;
        match self.index {
            0 => p.lisr().tcif0() != 0,
            1 => p.lisr().tcif1() != 0,
            2 => p.lisr().tcif2() != 0,
            3 => p.lisr().tcif3() != 0,
            4 => p.hisr().tcif4() != 0,
            5 => p.hisr().tcif5() != 0,
            6 => p.hisr().tcif6() != 0,
            7 => p.hisr().tcif7() != 0,
            _ => panic!("Invalid channel index"),
        }
    }            

    pub fn clr_feif(&self) -> &Self {
        let p = self.periph;
        match self.index {
            0 => p.set_lifcr(|r| r.set_cfeif0(1)),
            1 => p.set_lifcr(|r| r.set_cfeif1(1)),
            2 => p.set_lifcr(|r| r.set_cfeif2(1)),
            3 => p.set_lifcr(|r| r.set_cfeif3(1)),
            4 => p.set_hifcr(|r| r.set_cfeif4(1)),
            5 => p.set_hifcr(|r| r.set_cfeif5(1)),
            6 => p.set_hifcr(|r| r.set_cfeif6(1)),
            7 => p.set_hifcr(|r| r.set_cfeif7(1)),           
            _ => panic!("Invalid channel index"),
        };
        self
    }      

    pub fn clr_dmeif(&self) -> &Self {
        let p = self.periph;
        match self.index {
            0 => p.set_lifcr(|r| r.set_cdmeif0(1)),
            1 => p.set_lifcr(|r| r.set_cdmeif1(1)),
            2 => p.set_lifcr(|r| r.set_cdmeif2(1)),
            3 => p.set_lifcr(|r| r.set_cdmeif3(1)),
            4 => p.set_hifcr(|r| r.set_cdmeif4(1)),
            5 => p.set_hifcr(|r| r.set_cdmeif5(1)),
            6 => p.set_hifcr(|r| r.set_cdmeif6(1)),
            7 => p.set_hifcr(|r| r.set_cdmeif7(1)),           
            _ => panic!("Invalid channel index"),
        };
        self
    }      

    pub fn clr_teif(&self) -> &Self {
        let p = self.periph;
        match self.index {
            0 => p.set_lifcr(|r| r.set_cteif0(1)),
            1 => p.set_lifcr(|r| r.set_cteif1(1)),
            2 => p.set_lifcr(|r| r.set_cteif2(1)),
            3 => p.set_lifcr(|r| r.set_cteif3(1)),
            4 => p.set_hifcr(|r| r.set_cteif4(1)),
            5 => p.set_hifcr(|r| r.set_cteif5(1)),
            6 => p.set_hifcr(|r| r.set_cteif6(1)),
            7 => p.set_hifcr(|r| r.set_cteif7(1)),           
            _ => panic!("Invalid channel index"),
        };
        self
    }

    pub fn clr_htif(&self) -> &Self {
        let p = self.periph;
        match self.index {
            0 => p.set_lifcr(|r| r.set_chtif0(1)),
            1 => p.set_lifcr(|r| r.set_chtif1(1)),
            2 => p.set_lifcr(|r| r.set_chtif2(1)),
            3 => p.set_lifcr(|r| r.set_chtif3(1)),
            4 => p.set_hifcr(|r| r.set_chtif4(1)),
            5 => p.set_hifcr(|r| r.set_chtif5(1)),
            6 => p.set_hifcr(|r| r.set_chtif6(1)),
            7 => p.set_hifcr(|r| r.set_chtif7(1)),           
            _ => panic!("Invalid channel index"),
        };
        self
    }                  

    pub fn clr_tcif(&self) -> &Self {
        let p = self.periph;
        match self.index {
            0 => p.set_lifcr(|r| r.set_ctcif0(1)),
            1 => p.set_lifcr(|r| r.set_ctcif1(1)),
            2 => p.set_lifcr(|r| r.set_ctcif2(1)),
            3 => p.set_lifcr(|r| r.set_ctcif3(1)),
            4 => p.set_hifcr(|r| r.set_ctcif4(1)),
            5 => p.set_hifcr(|r| r.set_ctcif5(1)),
            6 => p.set_hifcr(|r| r.set_ctcif6(1)),
            7 => p.set_hifcr(|r| r.set_ctcif7(1)),           
            _ => panic!("Invalid channel index"),
        };
        self
    }

    pub fn scr(&self) -> Scr {
        self.periph.scr(self.index)
    }
    pub fn set_scr(&self, value: Scr) -> &Self {
        self.periph.set_scr(self.index, |_| value);
        self
    }
    pub fn with_scr<F: FnOnce(Scr) -> Scr>(&self, f: F) -> &Self {
        self.periph.with_scr(self.index, f);
        self
    }

    pub fn ndt(&self) -> u16 {
        self.periph.sndtr(self.index).ndt().into()
    }

    pub fn set_ndt(&self, value: u16) -> &Self {
        self.periph.set_sndtr(self.index, |r| r.set_ndt(value));
        self
    }

    pub fn pa(&self) -> u32 {
        self.periph.spar(self.index).pa().into()
    }

    pub fn set_pa(&self, value: u32) -> &Self {
        self.periph.set_spar(self.index, |r| r.set_pa(value));
        self
    }    
    
    pub fn m0a(&self) -> u32 {
        self.periph.sm0ar(self.index).m0a().into()
    }

    pub fn set_m0a(&self, value: u32) -> &Self {
        self.periph.set_sm0ar(self.index, |r| r.set_m0a(value));
        self
    }    

    pub fn m1a(&self) -> u32 {
        self.periph.sm1ar(self.index).m1a().into()
    }

    pub fn set_m1a(&self, value: u32) -> &Self {
        self.periph.set_sm1ar(self.index, |r| r.set_m1a(value));
        self
    }            

    pub fn sfcr(&self) -> Sfcr {
        self.periph.sfcr(self.index)
    }
    pub fn set_sfcr(&self, value: Sfcr) -> &Self {
        self.periph.set_sfcr(self.index, |_| value);
        self
    }
    pub fn with_sfcr<F: FnOnce(Sfcr) -> Sfcr>(&self, f: F) -> &Self {
        self.periph.with_sfcr(self.index, f);
        self
    }    
    pub fn set_chsel(&self, value: u8) -> &Self {
        self.periph.with_scr(self.index, |r| r.set_chsel(value));
        self
    }

    pub fn is_enabled(&self) -> bool {
        self.scr().en() != 0
    }
    pub fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_scr(|r| r.set_en(value))        
    }
    pub fn set_dir(&self, value: Dir) -> &Self { self.with_scr(|r| r.set_dir(value as u32)) }
    pub fn set_psize(&self, value: Size) -> &Self { self.with_scr(|r| r.set_psize(value as u32)) }
    pub fn set_msize(&self, value: Size) -> &Self { self.with_scr(|r| r.set_msize(value as u32)) }

    pub fn set_pinc(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_scr(|r| r.set_pinc(value))
    }

    pub fn set_minc(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_scr(|r| r.set_minc(value))
    }

    pub fn set_circ(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_scr(|r| r.set_circ(value))
    }

    pub fn set_tcie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_scr(|r| r.set_tcie(value))
    }
}
