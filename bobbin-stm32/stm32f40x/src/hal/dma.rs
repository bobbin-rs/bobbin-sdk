use ::chip::dma::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Ch0 = 0b000,
    Ch1 = 0b001,
    Ch2 = 0b010,
    Ch3 = 0b011,
    Ch4 = 0b100,
    Ch5 = 0b101,
    Ch6 = 0b110,
    Ch7 = 0b111,
}


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

pub fn stream(dma: Dma, index: usize) -> Stream {
    Stream { dma: dma, index: index }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stream {
    dma: Dma,
    index: usize,
}

impl Stream {
    pub fn enabled(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).en() != 0
        }        
    }
    pub fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_en(value));
        }
        self
    }

    pub fn dmeie(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).dmeie() != 0
        }        
    }
    pub fn set_dmeie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_dmeie(value));
        }
        self
    }   

    pub fn teie(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).teie() != 0
        }        
    }

    pub fn set_teie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_teie(value));
        }
        self
    }

    pub fn htie(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).htie() != 0
        }        
    }
    
    pub fn set_htie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_htie(value));
        }
        self
    }   

    pub fn tcie(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).tcie() != 0
        }        
    }
    
    pub fn set_tcie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_tcie(value));
        }
        self
    }   

    pub fn pfctrl(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).pfctrl() != 0
        }        
    }
    
    pub fn set_pfctrl(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_pfctrl(value));
        }
        self
    }

    pub fn dir(&self) -> Dir {
        let dma = self.dma;
        unsafe {
            match dma.scr(self.index).dir() {
                0b00 => Dir::PtoM,
                0b01 => Dir::MtoP,
                0b10 => Dir::MtoM,
                _ => panic!("Invalid DIR"),
            }
        }           
    }

    pub fn set_dir(&self, value: Dir) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_dir(value as u32));
        }
        self
    }    

    pub fn circ(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).circ() != 0
        }        
    }
    
    pub fn set_circ(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_circ(value));
        }
        self
    }  

    pub fn pinc(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).pinc() != 0
        }        
    }
    
    pub fn set_pinc(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_pinc(value));
        }
        self
    }          

    pub fn minc(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).minc() != 0
        }        
    }
    
    pub fn set_minc(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_minc(value));
        }
        self
    }       

    pub fn psize(&self) -> Size {
        let dma = self.dma;
        unsafe {
            match dma.scr(self.index).psize() {
                0b00 => Size::Bit8,
                0b01 => Size::Bit16,
                0b10 => Size::Bit32,
                _ => panic!("Invalid PSIZE"),
            }
        }           
    }

    pub fn set_psize(&self, value: Size) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_psize(value as u32));
        }
        self
    }    

    pub fn msize(&self) -> Size {
        let dma = self.dma;
        unsafe {
            match dma.scr(self.index).msize() {
                0b00 => Size::Bit8,
                0b01 => Size::Bit16,
                0b10 => Size::Bit32,
                _ => panic!("Invalid MSIZE"),
            }
        }           
    }

    pub fn set_msize(&self, value: Size) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_msize(value as u32));
        }
        self
    }

    pub fn pincos(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).pincos() != 0
        }        
    }
    
    pub fn set_pincos(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_pincos(value));
        }
        self
    }     
    
    pub fn pl(&self) -> Priority {
        let dma = self.dma;
        unsafe {
            match dma.scr(self.index).msize() {
                0b00 => Priority::Low,
                0b01 => Priority::Medium,
                0b10 => Priority::High,
                0b11 => Priority::VeryHigh,
                _ => panic!("Invalid PL"),
            }
        }           
    }

    pub fn set_pl(&self, value: Priority) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_pl(value as u32));
        }
        self
    }

    pub fn dbm(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).dbm() != 0
        }        
    }
    
    pub fn set_dbm(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_dbm(value));
        }
        self
    }   

    pub fn ct(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.scr(self.index).ct() != 0
        }        
    }
    
    pub fn set_ct(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_ct(value));
        }
        self
    }       

    pub fn pburst(&self) -> Burst {
        let dma = self.dma;
        unsafe {
            match dma.scr(self.index).msize() {
                0b00 => Burst::Single,
                0b01 => Burst::Incr4,
                0b10 => Burst::Incr8,
                0b11 => Burst::Incr16,
                _ => panic!("Invalid PBURST"),
            }
        }           
    }

    pub fn set_pburst(&self, value: Burst) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_pburst(value as u32));
        }
        self
    }
    
    pub fn mburst(&self) -> Burst {
        let dma = self.dma;
        unsafe {
            match dma.scr(self.index).msize() {
                0b00 => Burst::Single,
                0b01 => Burst::Incr4,
                0b10 => Burst::Incr8,
                0b11 => Burst::Incr16,
                _ => panic!("Invalid MBURST"),
            }
        }           
    }

    pub fn set_mburst(&self, value: Burst) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_mburst(value as u32));
        }
        self
    }    

    pub fn chsel(&self) -> Channel {
        let dma = self.dma;
        unsafe {
            match dma.scr(self.index).msize() {
                0b000 => Channel::Ch0,
                0b001 => Channel::Ch1,
                0b010 => Channel::Ch2,
                0b011 => Channel::Ch3,
                0b100 => Channel::Ch4,
                0b101 => Channel::Ch5,
                0b110 => Channel::Ch6,
                0b111 => Channel::Ch7,
                _ => panic!("Invalid CHSEL"),
            }
        }           
    }

    pub fn set_chsel(&self, value: Channel) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_scr(self.index, |r| r.set_chsel(value as u32));
        }
        self
    }      

    pub fn ndt(&self) -> u16 {
        let dma = self.dma;
        unsafe {
            dma.sndtr(self.index).ndt() as u16
        }
    }

    pub fn set_ndt(&self, value: u16) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_sndtr(self.index, |r| r.set_ndt(value as u32));
        }
        self
    }      

    pub fn pa(&self) -> u32 {
        let dma = self.dma;
        unsafe {
            dma.spar(self.index).pa()
        }
    }

    pub fn set_pa(&self, value: u32) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_spar(self.index, |r| r.set_pa(value as u32));
        }
        self
    }    

    pub fn m0a(&self) -> u32 {
        let dma = self.dma;
        unsafe {
            dma.sm0ar(self.index).m0a()
        }
    }

    pub fn set_m0a(&self, value: u32) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_sm0ar(self.index, |r| r.set_m0a(value as u32));
        }
        self
    }         

    pub fn m1a(&self) -> u32 {
        let dma = self.dma;
        unsafe {
            dma.sm1ar(self.index).m1a()
        }
    }

    pub fn set_m1a(&self, value: u32) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_sm1ar(self.index, |r| r.set_m1a(value as u32));
        }
        self
    }     

    pub fn feie(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.sfcr(self.index).feie() != 0
        }        
    }
    
    pub fn set_feie(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_sfcr(self.index, |r| r.set_feie(value));
        }
        self
    }      

    pub fn fs(&self) -> FifoStatus {
        let dma = self.dma;
        unsafe {
            match dma.sfcr(self.index).fs() {
                0b000 => FifoStatus::Q1,
                0b001 => FifoStatus::Q2,
                0b010 => FifoStatus::Q3,
                0b011 => FifoStatus::Q4,
                0b100 => FifoStatus::Empty,
                0b101 => FifoStatus::Full,                
                _ => panic!("Invalid FS"),
            }
        }           
    }

    pub fn set_fs(&self, value: FifoThreshold) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_sfcr(self.index, |r| r.set_fs(value as u32));
        }
        self
    }    

    pub fn dmdis(&self) -> bool {
        let dma = self.dma;
        unsafe {
            dma.sfcr(self.index).dmdis() != 0
        }        
    }
    
    pub fn set_dmdis(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let mut dma = self.dma;
        unsafe {
            dma.with_sfcr(self.index, |r| r.set_dmdis(value));
        }
        self
    }        

    pub fn fth(&self) -> FifoThreshold {
        let dma = self.dma;
        unsafe {
            match dma.sfcr(self.index).fth() {
                0b00 => FifoThreshold::Q1,
                0b01 => FifoThreshold::Q2,
                0b10 => FifoThreshold::Q3,
                0b11 => FifoThreshold::Q4,
                _ => panic!("Invalid FTH"),
            }
        }           
    }

    pub fn set_fth(&self, value: FifoThreshold) -> &Self {
        let mut dma = self.dma;
        unsafe {
            dma.with_sfcr(self.index, |r| r.set_fth(value as u32));
        }
        self
    }

    pub fn feif(&self) -> bool {
        let dma = self.dma;
        unsafe {
            match self.index {
                0 => dma.lisr().feif0() != 0,
                1 => dma.lisr().feif1() != 0,
                2 => dma.lisr().feif2() != 0,
                3 => dma.lisr().feif3() != 0,
                4 => dma.hisr().feif4() != 0,
                5 => dma.hisr().feif5() != 0,
                6 => dma.hisr().feif6() != 0,
                7 => dma.hisr().feif7() != 0,
                _ => panic!("Invalid Stream Index"),
            }
        }
    }

    pub fn clr_feif(&self) -> &Self {
        let mut dma = self.dma;
        unsafe {
            match self.index {
                0 => dma.with_lifcr(|r| r.set_cfeif0(1)),
                1 => dma.with_lifcr(|r| r.set_cfeif1(1)),
                2 => dma.with_lifcr(|r| r.set_cfeif2(1)),
                3 => dma.with_lifcr(|r| r.set_cfeif3(1)),
                4 => dma.with_hifcr(|r| r.set_cfeif4(1)),
                5 => dma.with_hifcr(|r| r.set_cfeif5(1)),
                6 => dma.with_hifcr(|r| r.set_cfeif6(1)),
                7 => dma.with_hifcr(|r| r.set_cfeif7(1)),
                _ => panic!("Invalid Stream Index"),
            }
        }
        self
    }

    pub fn dmeif(&self) -> bool {
        let dma = self.dma;
        unsafe {
            match self.index {
                0 => dma.lisr().dmeif0() != 0,
                1 => dma.lisr().dmeif1() != 0,
                2 => dma.lisr().dmeif2() != 0,
                3 => dma.lisr().dmeif3() != 0,
                4 => dma.hisr().dmeif4() != 0,
                5 => dma.hisr().dmeif5() != 0,
                6 => dma.hisr().dmeif6() != 0,
                7 => dma.hisr().dmeif7() != 0,
                _ => panic!("Invalid Stream Index"),
            }
        }
    }

    pub fn clr_dmeif(&self) -> &Self {
        let mut dma = self.dma;
        unsafe {
            match self.index {
                0 => dma.with_lifcr(|r| r.set_cdmeif0(1)),
                1 => dma.with_lifcr(|r| r.set_cdmeif1(1)),
                2 => dma.with_lifcr(|r| r.set_cdmeif2(1)),
                3 => dma.with_lifcr(|r| r.set_cdmeif3(1)),
                4 => dma.with_hifcr(|r| r.set_cdmeif4(1)),
                5 => dma.with_hifcr(|r| r.set_cdmeif5(1)),
                6 => dma.with_hifcr(|r| r.set_cdmeif6(1)),
                7 => dma.with_hifcr(|r| r.set_cdmeif7(1)),
                _ => panic!("Invalid Stream Index"),
            }
        }
        self
    }

    pub fn teif(&self) -> bool {
        let dma = self.dma;
        unsafe {
            match self.index {
                0 => dma.lisr().teif0() != 0,
                1 => dma.lisr().teif1() != 0,
                2 => dma.lisr().teif2() != 0,
                3 => dma.lisr().teif3() != 0,
                4 => dma.hisr().teif4() != 0,
                5 => dma.hisr().teif5() != 0,
                6 => dma.hisr().teif6() != 0,
                7 => dma.hisr().teif7() != 0,
                _ => panic!("Invalid Stream Index"),
            }
        }
    }

    pub fn clr_teif(&self) -> &Self {
        let mut dma = self.dma;
        unsafe {
            match self.index {
                0 => dma.with_lifcr(|r| r.set_cteif0(1)),
                1 => dma.with_lifcr(|r| r.set_cteif1(1)),
                2 => dma.with_lifcr(|r| r.set_cteif2(1)),
                3 => dma.with_lifcr(|r| r.set_cteif3(1)),
                4 => dma.with_hifcr(|r| r.set_cteif4(1)),
                5 => dma.with_hifcr(|r| r.set_cteif5(1)),
                6 => dma.with_hifcr(|r| r.set_cteif6(1)),
                7 => dma.with_hifcr(|r| r.set_cteif7(1)),
                _ => panic!("Invalid Stream Index"),
            }
        }
        self
    }

    pub fn htif(&self) -> bool {
        let dma = self.dma;
        unsafe {
            match self.index {
                0 => dma.lisr().htif0() != 0,
                1 => dma.lisr().htif1() != 0,
                2 => dma.lisr().htif2() != 0,
                3 => dma.lisr().htif3() != 0,
                4 => dma.hisr().htif4() != 0,
                5 => dma.hisr().htif5() != 0,
                6 => dma.hisr().htif6() != 0,
                7 => dma.hisr().htif7() != 0,
                _ => panic!("Invalid Stream Index"),
            }
        }
    }

    pub fn clr_htif(&self) -> &Self {
        let mut dma = self.dma;
        unsafe {
            match self.index {
                0 => dma.with_lifcr(|r| r.set_chtif0(1)),
                1 => dma.with_lifcr(|r| r.set_chtif1(1)),
                2 => dma.with_lifcr(|r| r.set_chtif2(1)),
                3 => dma.with_lifcr(|r| r.set_chtif3(1)),
                4 => dma.with_hifcr(|r| r.set_chtif4(1)),
                5 => dma.with_hifcr(|r| r.set_chtif5(1)),
                6 => dma.with_hifcr(|r| r.set_chtif6(1)),
                7 => dma.with_hifcr(|r| r.set_chtif7(1)),
                _ => panic!("Invalid Stream Index"),
            }
        }
        self
    }        

    pub fn tcif(&self) -> bool {
        let dma = self.dma;
        unsafe {
            match self.index {
                0 => dma.lisr().tcif0() != 0,
                1 => dma.lisr().tcif1() != 0,
                2 => dma.lisr().tcif2() != 0,
                3 => dma.lisr().tcif3() != 0,
                4 => dma.hisr().tcif4() != 0,
                5 => dma.hisr().tcif5() != 0,
                6 => dma.hisr().tcif6() != 0,
                7 => dma.hisr().tcif7() != 0,
                _ => panic!("Invalid Stream Index"),
            }
        }
    }

    pub fn clr_tcif(&self) -> &Self {
        let mut dma = self.dma;
        unsafe {
            match self.index {
                0 => dma.with_lifcr(|r| r.set_ctcif0(1)),
                1 => dma.with_lifcr(|r| r.set_ctcif1(1)),
                2 => dma.with_lifcr(|r| r.set_ctcif2(1)),
                3 => dma.with_lifcr(|r| r.set_ctcif3(1)),
                4 => dma.with_hifcr(|r| r.set_ctcif4(1)),
                5 => dma.with_hifcr(|r| r.set_ctcif5(1)),
                6 => dma.with_hifcr(|r| r.set_ctcif6(1)),
                7 => dma.with_hifcr(|r| r.set_ctcif7(1)),
                _ => panic!("Invalid Stream Index"),
            }
        }
        self
    }
}
