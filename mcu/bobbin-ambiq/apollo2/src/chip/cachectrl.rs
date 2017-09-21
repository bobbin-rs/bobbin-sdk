//! Flash Cache Controller
#[allow(unused_imports)] use bobbin_common::*;

periph!(CACHECTRL, Cachectrl, 0x40018000);

#[doc="Flash Cache Controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cachectrl(pub usize);
impl Cachectrl {
    #[doc="Get the *mut pointer for the CACHECFG register."]
    #[inline] pub fn cachecfg_mut(&self) -> *mut Cachecfg { 
        (self.0 + 0x0) as *mut Cachecfg
    }

    #[doc="Get the *const pointer for the CACHECFG register."]
    #[inline] pub fn cachecfg_ptr(&self) -> *const Cachecfg { 
           self.cachecfg_mut()
    }

    #[doc="Read the CACHECFG register."]
    #[inline] pub fn cachecfg(&self) -> Cachecfg { 
        unsafe {
            read_volatile(self.cachecfg_ptr())
        }
    }

    #[doc="Write the CACHECFG register."]
    #[inline] pub fn set_cachecfg<F: FnOnce(Cachecfg) -> Cachecfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cachecfg_mut(), f(Cachecfg(0)));
        }
        self
    }

    #[doc="Modify the CACHECFG register."]
    #[inline] pub fn with_cachecfg<F: FnOnce(Cachecfg) -> Cachecfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cachecfg_mut(), f(self.cachecfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLASHCFG register."]
    #[inline] pub fn flashcfg_mut(&self) -> *mut Flashcfg { 
        (self.0 + 0x4) as *mut Flashcfg
    }

    #[doc="Get the *const pointer for the FLASHCFG register."]
    #[inline] pub fn flashcfg_ptr(&self) -> *const Flashcfg { 
           self.flashcfg_mut()
    }

    #[doc="Read the FLASHCFG register."]
    #[inline] pub fn flashcfg(&self) -> Flashcfg { 
        unsafe {
            read_volatile(self.flashcfg_ptr())
        }
    }

    #[doc="Write the FLASHCFG register."]
    #[inline] pub fn set_flashcfg<F: FnOnce(Flashcfg) -> Flashcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flashcfg_mut(), f(Flashcfg(0)));
        }
        self
    }

    #[doc="Modify the FLASHCFG register."]
    #[inline] pub fn with_flashcfg<F: FnOnce(Flashcfg) -> Flashcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flashcfg_mut(), f(self.flashcfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CACHECTRL register."]
    #[inline] pub fn cachectrl_mut(&self) -> *mut Cachectrl { 
        (self.0 + 0x8) as *mut Cachectrl
    }

    #[doc="Get the *const pointer for the CACHECTRL register."]
    #[inline] pub fn cachectrl_ptr(&self) -> *const Cachectrl { 
           self.cachectrl_mut()
    }

    #[doc="Read the CACHECTRL register."]
    #[inline] pub fn cachectrl(&self) -> Cachectrl { 
        unsafe {
            read_volatile(self.cachectrl_ptr())
        }
    }

    #[doc="Write the CACHECTRL register."]
    #[inline] pub fn set_cachectrl<F: FnOnce(Cachectrl) -> Cachectrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cachectrl_mut(), f(Cachectrl(0)));
        }
        self
    }

    #[doc="Modify the CACHECTRL register."]
    #[inline] pub fn with_cachectrl<F: FnOnce(Cachectrl) -> Cachectrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cachectrl_mut(), f(self.cachectrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the NCR0START register."]
    #[inline] pub fn ncr0start_mut(&self) -> *mut Ncr0start { 
        (self.0 + 0x10) as *mut Ncr0start
    }

    #[doc="Get the *const pointer for the NCR0START register."]
    #[inline] pub fn ncr0start_ptr(&self) -> *const Ncr0start { 
           self.ncr0start_mut()
    }

    #[doc="Read the NCR0START register."]
    #[inline] pub fn ncr0start(&self) -> Ncr0start { 
        unsafe {
            read_volatile(self.ncr0start_ptr())
        }
    }

    #[doc="Write the NCR0START register."]
    #[inline] pub fn set_ncr0start<F: FnOnce(Ncr0start) -> Ncr0start>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ncr0start_mut(), f(Ncr0start(0)));
        }
        self
    }

    #[doc="Modify the NCR0START register."]
    #[inline] pub fn with_ncr0start<F: FnOnce(Ncr0start) -> Ncr0start>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ncr0start_mut(), f(self.ncr0start()));
        }
        self
    }

    #[doc="Get the *mut pointer for the NCR0END register."]
    #[inline] pub fn ncr0end_mut(&self) -> *mut Ncr0end { 
        (self.0 + 0x14) as *mut Ncr0end
    }

    #[doc="Get the *const pointer for the NCR0END register."]
    #[inline] pub fn ncr0end_ptr(&self) -> *const Ncr0end { 
           self.ncr0end_mut()
    }

    #[doc="Read the NCR0END register."]
    #[inline] pub fn ncr0end(&self) -> Ncr0end { 
        unsafe {
            read_volatile(self.ncr0end_ptr())
        }
    }

    #[doc="Write the NCR0END register."]
    #[inline] pub fn set_ncr0end<F: FnOnce(Ncr0end) -> Ncr0end>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ncr0end_mut(), f(Ncr0end(0)));
        }
        self
    }

    #[doc="Modify the NCR0END register."]
    #[inline] pub fn with_ncr0end<F: FnOnce(Ncr0end) -> Ncr0end>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ncr0end_mut(), f(self.ncr0end()));
        }
        self
    }

    #[doc="Get the *mut pointer for the NCR1START register."]
    #[inline] pub fn ncr1start_mut(&self) -> *mut Ncr1start { 
        (self.0 + 0x18) as *mut Ncr1start
    }

    #[doc="Get the *const pointer for the NCR1START register."]
    #[inline] pub fn ncr1start_ptr(&self) -> *const Ncr1start { 
           self.ncr1start_mut()
    }

    #[doc="Read the NCR1START register."]
    #[inline] pub fn ncr1start(&self) -> Ncr1start { 
        unsafe {
            read_volatile(self.ncr1start_ptr())
        }
    }

    #[doc="Write the NCR1START register."]
    #[inline] pub fn set_ncr1start<F: FnOnce(Ncr1start) -> Ncr1start>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ncr1start_mut(), f(Ncr1start(0)));
        }
        self
    }

    #[doc="Modify the NCR1START register."]
    #[inline] pub fn with_ncr1start<F: FnOnce(Ncr1start) -> Ncr1start>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ncr1start_mut(), f(self.ncr1start()));
        }
        self
    }

    #[doc="Get the *mut pointer for the NCR1END register."]
    #[inline] pub fn ncr1end_mut(&self) -> *mut Ncr1end { 
        (self.0 + 0x1c) as *mut Ncr1end
    }

    #[doc="Get the *const pointer for the NCR1END register."]
    #[inline] pub fn ncr1end_ptr(&self) -> *const Ncr1end { 
           self.ncr1end_mut()
    }

    #[doc="Read the NCR1END register."]
    #[inline] pub fn ncr1end(&self) -> Ncr1end { 
        unsafe {
            read_volatile(self.ncr1end_ptr())
        }
    }

    #[doc="Write the NCR1END register."]
    #[inline] pub fn set_ncr1end<F: FnOnce(Ncr1end) -> Ncr1end>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ncr1end_mut(), f(Ncr1end(0)));
        }
        self
    }

    #[doc="Modify the NCR1END register."]
    #[inline] pub fn with_ncr1end<F: FnOnce(Ncr1end) -> Ncr1end>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ncr1end_mut(), f(self.ncr1end()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CACHEMODE register."]
    #[inline] pub fn cachemode_mut(&self) -> *mut Cachemode { 
        (self.0 + 0x30) as *mut Cachemode
    }

    #[doc="Get the *const pointer for the CACHEMODE register."]
    #[inline] pub fn cachemode_ptr(&self) -> *const Cachemode { 
           self.cachemode_mut()
    }

    #[doc="Read the CACHEMODE register."]
    #[inline] pub fn cachemode(&self) -> Cachemode { 
        unsafe {
            read_volatile(self.cachemode_ptr())
        }
    }

    #[doc="Write the CACHEMODE register."]
    #[inline] pub fn set_cachemode<F: FnOnce(Cachemode) -> Cachemode>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cachemode_mut(), f(Cachemode(0)));
        }
        self
    }

    #[doc="Modify the CACHEMODE register."]
    #[inline] pub fn with_cachemode<F: FnOnce(Cachemode) -> Cachemode>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cachemode_mut(), f(self.cachemode()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DMON0 register."]
    #[inline] pub fn dmon0_mut(&self) -> *mut Dmon0 { 
        (self.0 + 0x40) as *mut Dmon0
    }

    #[doc="Get the *const pointer for the DMON0 register."]
    #[inline] pub fn dmon0_ptr(&self) -> *const Dmon0 { 
           self.dmon0_mut()
    }

    #[doc="Read the DMON0 register."]
    #[inline] pub fn dmon0(&self) -> Dmon0 { 
        unsafe {
            read_volatile(self.dmon0_ptr())
        }
    }

    #[doc="Write the DMON0 register."]
    #[inline] pub fn set_dmon0<F: FnOnce(Dmon0) -> Dmon0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmon0_mut(), f(Dmon0(0)));
        }
        self
    }

    #[doc="Modify the DMON0 register."]
    #[inline] pub fn with_dmon0<F: FnOnce(Dmon0) -> Dmon0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmon0_mut(), f(self.dmon0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DMON1 register."]
    #[inline] pub fn dmon1_mut(&self) -> *mut Dmon1 { 
        (self.0 + 0x44) as *mut Dmon1
    }

    #[doc="Get the *const pointer for the DMON1 register."]
    #[inline] pub fn dmon1_ptr(&self) -> *const Dmon1 { 
           self.dmon1_mut()
    }

    #[doc="Read the DMON1 register."]
    #[inline] pub fn dmon1(&self) -> Dmon1 { 
        unsafe {
            read_volatile(self.dmon1_ptr())
        }
    }

    #[doc="Write the DMON1 register."]
    #[inline] pub fn set_dmon1<F: FnOnce(Dmon1) -> Dmon1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmon1_mut(), f(Dmon1(0)));
        }
        self
    }

    #[doc="Modify the DMON1 register."]
    #[inline] pub fn with_dmon1<F: FnOnce(Dmon1) -> Dmon1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmon1_mut(), f(self.dmon1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DMON2 register."]
    #[inline] pub fn dmon2_mut(&self) -> *mut Dmon2 { 
        (self.0 + 0x48) as *mut Dmon2
    }

    #[doc="Get the *const pointer for the DMON2 register."]
    #[inline] pub fn dmon2_ptr(&self) -> *const Dmon2 { 
           self.dmon2_mut()
    }

    #[doc="Read the DMON2 register."]
    #[inline] pub fn dmon2(&self) -> Dmon2 { 
        unsafe {
            read_volatile(self.dmon2_ptr())
        }
    }

    #[doc="Write the DMON2 register."]
    #[inline] pub fn set_dmon2<F: FnOnce(Dmon2) -> Dmon2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmon2_mut(), f(Dmon2(0)));
        }
        self
    }

    #[doc="Modify the DMON2 register."]
    #[inline] pub fn with_dmon2<F: FnOnce(Dmon2) -> Dmon2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmon2_mut(), f(self.dmon2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DMON3 register."]
    #[inline] pub fn dmon3_mut(&self) -> *mut Dmon3 { 
        (self.0 + 0x4c) as *mut Dmon3
    }

    #[doc="Get the *const pointer for the DMON3 register."]
    #[inline] pub fn dmon3_ptr(&self) -> *const Dmon3 { 
           self.dmon3_mut()
    }

    #[doc="Read the DMON3 register."]
    #[inline] pub fn dmon3(&self) -> Dmon3 { 
        unsafe {
            read_volatile(self.dmon3_ptr())
        }
    }

    #[doc="Write the DMON3 register."]
    #[inline] pub fn set_dmon3<F: FnOnce(Dmon3) -> Dmon3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmon3_mut(), f(Dmon3(0)));
        }
        self
    }

    #[doc="Modify the DMON3 register."]
    #[inline] pub fn with_dmon3<F: FnOnce(Dmon3) -> Dmon3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmon3_mut(), f(self.dmon3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IMON0 register."]
    #[inline] pub fn imon0_mut(&self) -> *mut Imon0 { 
        (self.0 + 0x50) as *mut Imon0
    }

    #[doc="Get the *const pointer for the IMON0 register."]
    #[inline] pub fn imon0_ptr(&self) -> *const Imon0 { 
           self.imon0_mut()
    }

    #[doc="Read the IMON0 register."]
    #[inline] pub fn imon0(&self) -> Imon0 { 
        unsafe {
            read_volatile(self.imon0_ptr())
        }
    }

    #[doc="Write the IMON0 register."]
    #[inline] pub fn set_imon0<F: FnOnce(Imon0) -> Imon0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imon0_mut(), f(Imon0(0)));
        }
        self
    }

    #[doc="Modify the IMON0 register."]
    #[inline] pub fn with_imon0<F: FnOnce(Imon0) -> Imon0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imon0_mut(), f(self.imon0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IMON1 register."]
    #[inline] pub fn imon1_mut(&self) -> *mut Imon1 { 
        (self.0 + 0x54) as *mut Imon1
    }

    #[doc="Get the *const pointer for the IMON1 register."]
    #[inline] pub fn imon1_ptr(&self) -> *const Imon1 { 
           self.imon1_mut()
    }

    #[doc="Read the IMON1 register."]
    #[inline] pub fn imon1(&self) -> Imon1 { 
        unsafe {
            read_volatile(self.imon1_ptr())
        }
    }

    #[doc="Write the IMON1 register."]
    #[inline] pub fn set_imon1<F: FnOnce(Imon1) -> Imon1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imon1_mut(), f(Imon1(0)));
        }
        self
    }

    #[doc="Modify the IMON1 register."]
    #[inline] pub fn with_imon1<F: FnOnce(Imon1) -> Imon1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imon1_mut(), f(self.imon1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IMON2 register."]
    #[inline] pub fn imon2_mut(&self) -> *mut Imon2 { 
        (self.0 + 0x58) as *mut Imon2
    }

    #[doc="Get the *const pointer for the IMON2 register."]
    #[inline] pub fn imon2_ptr(&self) -> *const Imon2 { 
           self.imon2_mut()
    }

    #[doc="Read the IMON2 register."]
    #[inline] pub fn imon2(&self) -> Imon2 { 
        unsafe {
            read_volatile(self.imon2_ptr())
        }
    }

    #[doc="Write the IMON2 register."]
    #[inline] pub fn set_imon2<F: FnOnce(Imon2) -> Imon2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imon2_mut(), f(Imon2(0)));
        }
        self
    }

    #[doc="Modify the IMON2 register."]
    #[inline] pub fn with_imon2<F: FnOnce(Imon2) -> Imon2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imon2_mut(), f(self.imon2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IMON3 register."]
    #[inline] pub fn imon3_mut(&self) -> *mut Imon3 { 
        (self.0 + 0x5c) as *mut Imon3
    }

    #[doc="Get the *const pointer for the IMON3 register."]
    #[inline] pub fn imon3_ptr(&self) -> *const Imon3 { 
           self.imon3_mut()
    }

    #[doc="Read the IMON3 register."]
    #[inline] pub fn imon3(&self) -> Imon3 { 
        unsafe {
            read_volatile(self.imon3_ptr())
        }
    }

    #[doc="Write the IMON3 register."]
    #[inline] pub fn set_imon3<F: FnOnce(Imon3) -> Imon3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imon3_mut(), f(Imon3(0)));
        }
        self
    }

    #[doc="Modify the IMON3 register."]
    #[inline] pub fn with_imon3<F: FnOnce(Imon3) -> Imon3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imon3_mut(), f(self.imon3()));
        }
        self
    }

}

#[doc="Flash Cache Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cachecfg(pub u32);
impl Cachecfg {
    #[doc="Enable Cache Monitoring Stats"]
    #[inline] pub fn enable_monitor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if ENABLE_MONITOR != 0"]
    #[inline] pub fn test_enable_monitor(&self) -> bool {
        self.enable_monitor() != 0
    }

    #[doc="Sets the ENABLE_MONITOR field."]
    #[inline] pub fn set_enable_monitor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Enable clock gating of entire data array"]
    #[inline] pub fn data_clkgate(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if DATA_CLKGATE != 0"]
    #[inline] pub fn test_data_clkgate(&self) -> bool {
        self.data_clkgate() != 0
    }

    #[doc="Sets the DATA_CLKGATE field."]
    #[inline] pub fn set_data_clkgate<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Data RAM delay"]
    #[inline] pub fn smdly(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if SMDLY != 0"]
    #[inline] pub fn test_smdly(&self) -> bool {
        self.smdly() != 0
    }

    #[doc="Sets the SMDLY field."]
    #[inline] pub fn set_smdly<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Data RAM delay"]
    #[inline] pub fn dly(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if DLY != 0"]
    #[inline] pub fn test_dly(&self) -> bool {
        self.dly() != 0
    }

    #[doc="Sets the DLY field."]
    #[inline] pub fn set_dly<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Enable LS (light sleep) of cache RAMs. When this bit is set, the cache\'s RAMS will be put into light sleep mode while inactive. NOTE: if the cache is actively used, this may have an adverse affect on power since entering/exiting LS mode may consume more power than would be saved."]
    #[inline] pub fn cache_ls(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CACHE_LS != 0"]
    #[inline] pub fn test_cache_ls(&self) -> bool {
        self.cache_ls() != 0
    }

    #[doc="Sets the CACHE_LS field."]
    #[inline] pub fn set_cache_ls<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Enable clock gating of cache RAMs"]
    #[inline] pub fn cache_clkgate(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CACHE_CLKGATE != 0"]
    #[inline] pub fn test_cache_clkgate(&self) -> bool {
        self.cache_clkgate() != 0
    }

    #[doc="Sets the CACHE_CLKGATE field."]
    #[inline] pub fn set_cache_clkgate<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Enable Flash Instruction Caching"]
    #[inline] pub fn dcache_enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DCACHE_ENABLE != 0"]
    #[inline] pub fn test_dcache_enable(&self) -> bool {
        self.dcache_enable() != 0
    }

    #[doc="Sets the DCACHE_ENABLE field."]
    #[inline] pub fn set_dcache_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enable Flash Instruction Caching"]
    #[inline] pub fn icache_enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ICACHE_ENABLE != 0"]
    #[inline] pub fn test_icache_enable(&self) -> bool {
        self.icache_enable() != 0
    }

    #[doc="Sets the ICACHE_ENABLE field."]
    #[inline] pub fn set_icache_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Bitfield should always be programmed to 0."]
    #[inline] pub fn serial(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SERIAL != 0"]
    #[inline] pub fn test_serial(&self) -> bool {
        self.serial() != 0
    }

    #[doc="Sets the SERIAL field."]
    #[inline] pub fn set_serial<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Sets the cache configuration"]
    #[inline] pub fn config(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if CONFIG != 0"]
    #[inline] pub fn test_config(&self) -> bool {
        self.config() != 0
    }

    #[doc="Sets the CONFIG field."]
    #[inline] pub fn set_config<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Enable Non-cacheable region 1"]
    #[inline] pub fn enable_nc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ENABLE_NC1 != 0"]
    #[inline] pub fn test_enable_nc1(&self) -> bool {
        self.enable_nc1() != 0
    }

    #[doc="Sets the ENABLE_NC1 field."]
    #[inline] pub fn set_enable_nc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enable Non-cacheable region 0"]
    #[inline] pub fn enable_nc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ENABLE_NC0 != 0"]
    #[inline] pub fn test_enable_nc0(&self) -> bool {
        self.enable_nc0() != 0
    }

    #[doc="Sets the ENABLE_NC0 field."]
    #[inline] pub fn set_enable_nc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Sets the cache replacement policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
    #[inline] pub fn lru(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LRU != 0"]
    #[inline] pub fn test_lru(&self) -> bool {
        self.lru() != 0
    }

    #[doc="Sets the LRU field."]
    #[inline] pub fn set_lru<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enables the flash cache controller. I/D caching enabled independently."]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cachecfg {
    #[inline]
    fn from(other: u32) -> Self {
         Cachecfg(other)
    }
}

impl ::core::fmt::Display for Cachecfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cachecfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable_monitor() != 0 { try!(write!(f, " enable_monitor"))}
        if self.data_clkgate() != 0 { try!(write!(f, " data_clkgate"))}
        if self.smdly() != 0 { try!(write!(f, " smdly=0x{:x}", self.smdly()))}
        if self.dly() != 0 { try!(write!(f, " dly=0x{:x}", self.dly()))}
        if self.cache_ls() != 0 { try!(write!(f, " cache_ls"))}
        if self.cache_clkgate() != 0 { try!(write!(f, " cache_clkgate"))}
        if self.dcache_enable() != 0 { try!(write!(f, " dcache_enable"))}
        if self.icache_enable() != 0 { try!(write!(f, " icache_enable"))}
        if self.serial() != 0 { try!(write!(f, " serial"))}
        if self.config() != 0 { try!(write!(f, " config=0x{:x}", self.config()))}
        if self.enable_nc1() != 0 { try!(write!(f, " enable_nc1"))}
        if self.enable_nc0() != 0 { try!(write!(f, " enable_nc0"))}
        if self.lru() != 0 { try!(write!(f, " lru"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flashcfg(pub u32);
impl Flashcfg {
    #[doc="Sets read waitstates (HCLK cycles)"]
    #[inline] pub fn rd_wait(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if RD_WAIT != 0"]
    #[inline] pub fn test_rd_wait(&self) -> bool {
        self.rd_wait() != 0
    }

    #[doc="Sets the RD_WAIT field."]
    #[inline] pub fn set_rd_wait<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Flashcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Flashcfg(other)
    }
}

impl ::core::fmt::Display for Flashcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Flashcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rd_wait() != 0 { try!(write!(f, " rd_wait=0x{:x}", self.rd_wait()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cachectrl(pub u32);
impl Cachectrl {
    #[doc="Enable Flash Sleep Mode"]
    #[inline] pub fn flash1_slm_enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if FLASH1_SLM_ENABLE != 0"]
    #[inline] pub fn test_flash1_slm_enable(&self) -> bool {
        self.flash1_slm_enable() != 0
    }

    #[doc="Sets the FLASH1_SLM_ENABLE field."]
    #[inline] pub fn set_flash1_slm_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Disable Flash Sleep Mode"]
    #[inline] pub fn flash1_slm_disable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FLASH1_SLM_DISABLE != 0"]
    #[inline] pub fn test_flash1_slm_disable(&self) -> bool {
        self.flash1_slm_disable() != 0
    }

    #[doc="Sets the FLASH1_SLM_DISABLE field."]
    #[inline] pub fn set_flash1_slm_disable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Flash Sleep Mode Status"]
    #[inline] pub fn flash1_slm_status(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FLASH1_SLM_STATUS != 0"]
    #[inline] pub fn test_flash1_slm_status(&self) -> bool {
        self.flash1_slm_status() != 0
    }

    #[doc="Sets the FLASH1_SLM_STATUS field."]
    #[inline] pub fn set_flash1_slm_status<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Enable Flash Sleep Mode"]
    #[inline] pub fn flash0_slm_enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FLASH0_SLM_ENABLE != 0"]
    #[inline] pub fn test_flash0_slm_enable(&self) -> bool {
        self.flash0_slm_enable() != 0
    }

    #[doc="Sets the FLASH0_SLM_ENABLE field."]
    #[inline] pub fn set_flash0_slm_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Disable Flash Sleep Mode"]
    #[inline] pub fn flash0_slm_disable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FLASH0_SLM_DISABLE != 0"]
    #[inline] pub fn test_flash0_slm_disable(&self) -> bool {
        self.flash0_slm_disable() != 0
    }

    #[doc="Sets the FLASH0_SLM_DISABLE field."]
    #[inline] pub fn set_flash0_slm_disable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Flash Sleep Mode Status"]
    #[inline] pub fn flash0_slm_status(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FLASH0_SLM_STATUS != 0"]
    #[inline] pub fn test_flash0_slm_status(&self) -> bool {
        self.flash0_slm_status() != 0
    }

    #[doc="Sets the FLASH0_SLM_STATUS field."]
    #[inline] pub fn set_flash0_slm_status<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Cache Ready Status (enabled and not processing an invalidate operation)"]
    #[inline] pub fn cache_ready(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CACHE_READY != 0"]
    #[inline] pub fn test_cache_ready(&self) -> bool {
        self.cache_ready() != 0
    }

    #[doc="Sets the CACHE_READY field."]
    #[inline] pub fn set_cache_ready<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Writing a 1 to this bitfield will reset the cache monitor statistics (DMON0-3, IMON0-3). Statistic gathering can be paused/stopped by disabling the MONITOR_ENABLE bit in CACHECFG, which will maintain the count values until the stats are reset by writing this bitfield."]
    #[inline] pub fn reset_stat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RESET_STAT != 0"]
    #[inline] pub fn test_reset_stat(&self) -> bool {
        self.reset_stat() != 0
    }

    #[doc="Sets the RESET_STAT field."]
    #[inline] pub fn set_reset_stat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Writing a 1 to this bitfield invalidates the flash cache contents."]
    #[inline] pub fn invalidate(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INVALIDATE != 0"]
    #[inline] pub fn test_invalidate(&self) -> bool {
        self.invalidate() != 0
    }

    #[doc="Sets the INVALIDATE field."]
    #[inline] pub fn set_invalidate<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cachectrl {
    #[inline]
    fn from(other: u32) -> Self {
         Cachectrl(other)
    }
}

impl ::core::fmt::Display for Cachectrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cachectrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flash1_slm_enable() != 0 { try!(write!(f, " flash1_slm_enable"))}
        if self.flash1_slm_disable() != 0 { try!(write!(f, " flash1_slm_disable"))}
        if self.flash1_slm_status() != 0 { try!(write!(f, " flash1_slm_status"))}
        if self.flash0_slm_enable() != 0 { try!(write!(f, " flash0_slm_enable"))}
        if self.flash0_slm_disable() != 0 { try!(write!(f, " flash0_slm_disable"))}
        if self.flash0_slm_status() != 0 { try!(write!(f, " flash0_slm_status"))}
        if self.cache_ready() != 0 { try!(write!(f, " cache_ready"))}
        if self.reset_stat() != 0 { try!(write!(f, " reset_stat"))}
        if self.invalidate() != 0 { try!(write!(f, " invalidate"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Cache Noncachable Region 0 Start Address."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ncr0start(pub u32);
impl Ncr0start {
    #[doc="Start address for non-cacheable region 0. The physical address of the start of this region should be programmed to this register and must be aligned to a 16-byte boundary (thus the lower 4 address bits are unused)."]
    #[inline] pub fn addr(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xffff) as u16) } // [19:4]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Ncr0start {
    #[inline]
    fn from(other: u32) -> Self {
         Ncr0start(other)
    }
}

impl ::core::fmt::Display for Ncr0start {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ncr0start {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Cache Noncachable Region 0 End"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ncr0end(pub u32);
impl Ncr0end {
    #[doc="End address for non-cacheable region 0. The physical address of the end of this region should be programmed to this register and must be aligned to a 16-byte boundary (thus the lower 4 address bits are unused)."]
    #[inline] pub fn addr(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xffff) as u16) } // [19:4]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Ncr0end {
    #[inline]
    fn from(other: u32) -> Self {
         Ncr0end(other)
    }
}

impl ::core::fmt::Display for Ncr0end {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ncr0end {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Cache Noncachable Region 1 Start"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ncr1start(pub u32);
impl Ncr1start {
    #[doc="Start address for non-cacheable region 1. The physical address of the start of this region should be programmed to this register and must be aligned to a 16-byte boundary (thus the lower 4 address bits are unused)."]
    #[inline] pub fn addr(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xffff) as u16) } // [19:4]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Ncr1start {
    #[inline]
    fn from(other: u32) -> Self {
         Ncr1start(other)
    }
}

impl ::core::fmt::Display for Ncr1start {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ncr1start {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Cache Noncachable Region 1 End"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ncr1end(pub u32);
impl Ncr1end {
    #[doc="End address for non-cacheable region 1. The physical address of the end of this region should be programmed to this register and must be aligned to a 16-byte boundary (thus the lower 4 address bits are unused)."]
    #[inline] pub fn addr(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xffff) as u16) } // [19:4]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Ncr1end {
    #[inline]
    fn from(other: u32) -> Self {
         Ncr1end(other)
    }
}

impl ::core::fmt::Display for Ncr1end {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ncr1end {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Cache Mode Register. Used to trim performance/power."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cachemode(pub u32);
impl Cachemode {
    #[doc="Disallow Simultaneous Data RAM reads (from 2 line hits on each bus)"]
    #[inline] pub fn throttle6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if THROTTLE6 != 0"]
    #[inline] pub fn test_throttle6(&self) -> bool {
        self.throttle6() != 0
    }

    #[doc="Sets the THROTTLE6 field."]
    #[inline] pub fn set_throttle6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Disallow Data RAM reads (from line hits) during lookup read ops"]
    #[inline] pub fn throttle5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if THROTTLE5 != 0"]
    #[inline] pub fn test_throttle5(&self) -> bool {
        self.throttle5() != 0
    }

    #[doc="Sets the THROTTLE5 field."]
    #[inline] pub fn set_throttle5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Disallow Data RAM reads (from line hits) on tag RAM fill cycles"]
    #[inline] pub fn throttle4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if THROTTLE4 != 0"]
    #[inline] pub fn test_throttle4(&self) -> bool {
        self.throttle4() != 0
    }

    #[doc="Sets the THROTTLE4 field."]
    #[inline] pub fn set_throttle4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Disallow cache data RAM writes on data RAM read cycles"]
    #[inline] pub fn throttle3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if THROTTLE3 != 0"]
    #[inline] pub fn test_throttle3(&self) -> bool {
        self.throttle3() != 0
    }

    #[doc="Sets the THROTTLE3 field."]
    #[inline] pub fn set_throttle3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Disallow cache data RAM writes on tag RAM read cycles"]
    #[inline] pub fn throttle2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if THROTTLE2 != 0"]
    #[inline] pub fn test_throttle2(&self) -> bool {
        self.throttle2() != 0
    }

    #[doc="Sets the THROTTLE2 field."]
    #[inline] pub fn set_throttle2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Disallow cache data RAM writes on tag RAM fill cycles"]
    #[inline] pub fn throttle1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if THROTTLE1 != 0"]
    #[inline] pub fn test_throttle1(&self) -> bool {
        self.throttle1() != 0
    }

    #[doc="Sets the THROTTLE1 field."]
    #[inline] pub fn set_throttle1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cachemode {
    #[inline]
    fn from(other: u32) -> Self {
         Cachemode(other)
    }
}

impl ::core::fmt::Display for Cachemode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cachemode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.throttle6() != 0 { try!(write!(f, " throttle6"))}
        if self.throttle5() != 0 { try!(write!(f, " throttle5"))}
        if self.throttle4() != 0 { try!(write!(f, " throttle4"))}
        if self.throttle3() != 0 { try!(write!(f, " throttle3"))}
        if self.throttle2() != 0 { try!(write!(f, " throttle2"))}
        if self.throttle1() != 0 { try!(write!(f, " throttle1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Cache Total Accesses"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmon0(pub u32);
impl Dmon0 {
    #[doc="Total accesses to data cache"]
    #[inline] pub fn daccess_count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DACCESS_COUNT != 0"]
    #[inline] pub fn test_daccess_count(&self) -> bool {
        self.daccess_count() != 0
    }

    #[doc="Sets the DACCESS_COUNT field."]
    #[inline] pub fn set_daccess_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmon0 {
    #[inline]
    fn from(other: u32) -> Self {
         Dmon0(other)
    }
}

impl ::core::fmt::Display for Dmon0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmon0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Cache Tag Lookups"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmon1(pub u32);
impl Dmon1 {
    #[doc="Total tag lookups from data cache"]
    #[inline] pub fn dlookup_count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DLOOKUP_COUNT != 0"]
    #[inline] pub fn test_dlookup_count(&self) -> bool {
        self.dlookup_count() != 0
    }

    #[doc="Sets the DLOOKUP_COUNT field."]
    #[inline] pub fn set_dlookup_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmon1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dmon1(other)
    }
}

impl ::core::fmt::Display for Dmon1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmon1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Cache Hits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmon2(pub u32);
impl Dmon2 {
    #[doc="Cache hits from lookup operations"]
    #[inline] pub fn dhit_count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DHIT_COUNT != 0"]
    #[inline] pub fn test_dhit_count(&self) -> bool {
        self.dhit_count() != 0
    }

    #[doc="Sets the DHIT_COUNT field."]
    #[inline] pub fn set_dhit_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmon2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dmon2(other)
    }
}

impl ::core::fmt::Display for Dmon2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmon2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Cache Line Hits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmon3(pub u32);
impl Dmon3 {
    #[doc="Cache hits from line cache"]
    #[inline] pub fn dline_count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DLINE_COUNT != 0"]
    #[inline] pub fn test_dline_count(&self) -> bool {
        self.dline_count() != 0
    }

    #[doc="Sets the DLINE_COUNT field."]
    #[inline] pub fn set_dline_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmon3 {
    #[inline]
    fn from(other: u32) -> Self {
         Dmon3(other)
    }
}

impl ::core::fmt::Display for Dmon3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmon3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Instruction Cache Total Accesses"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imon0(pub u32);
impl Imon0 {
    #[doc="Total accesses to Instruction cache"]
    #[inline] pub fn iaccess_count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if IACCESS_COUNT != 0"]
    #[inline] pub fn test_iaccess_count(&self) -> bool {
        self.iaccess_count() != 0
    }

    #[doc="Sets the IACCESS_COUNT field."]
    #[inline] pub fn set_iaccess_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Imon0 {
    #[inline]
    fn from(other: u32) -> Self {
         Imon0(other)
    }
}

impl ::core::fmt::Display for Imon0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imon0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Instruction Cache Tag Lookups"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imon1(pub u32);
impl Imon1 {
    #[doc="Total tag lookups from Instruction cache"]
    #[inline] pub fn ilookup_count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ILOOKUP_COUNT != 0"]
    #[inline] pub fn test_ilookup_count(&self) -> bool {
        self.ilookup_count() != 0
    }

    #[doc="Sets the ILOOKUP_COUNT field."]
    #[inline] pub fn set_ilookup_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Imon1 {
    #[inline]
    fn from(other: u32) -> Self {
         Imon1(other)
    }
}

impl ::core::fmt::Display for Imon1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imon1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Instruction Cache Hits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imon2(pub u32);
impl Imon2 {
    #[doc="Cache hits from lookup operations"]
    #[inline] pub fn ihit_count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if IHIT_COUNT != 0"]
    #[inline] pub fn test_ihit_count(&self) -> bool {
        self.ihit_count() != 0
    }

    #[doc="Sets the IHIT_COUNT field."]
    #[inline] pub fn set_ihit_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Imon2 {
    #[inline]
    fn from(other: u32) -> Self {
         Imon2(other)
    }
}

impl ::core::fmt::Display for Imon2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imon2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Instruction Cache Line Hits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imon3(pub u32);
impl Imon3 {
    #[doc="Cache hits from line cache"]
    #[inline] pub fn iline_count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ILINE_COUNT != 0"]
    #[inline] pub fn test_iline_count(&self) -> bool {
        self.iline_count() != 0
    }

    #[doc="Sets the ILINE_COUNT field."]
    #[inline] pub fn set_iline_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Imon3 {
    #[inline]
    fn from(other: u32) -> Self {
         Imon3(other)
    }
}

impl ::core::fmt::Display for Imon3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imon3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}


