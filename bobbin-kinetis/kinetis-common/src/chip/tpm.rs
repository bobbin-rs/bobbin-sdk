#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TPM Peripheral"]
pub struct TpmPeriph(pub usize); 


impl TpmPeriph {
    #[doc="Get the *mut pointer for the SC register."]
    #[inline] pub fn sc_mut(&self) -> *mut Sc { 
        (self.0 + 0x0) as *mut Sc
    }

    #[doc="Get the *const pointer for the SC register."]
    #[inline] pub fn sc_ptr(&self) -> *const Sc { 
           self.sc_mut()
    }

    #[doc="Read the SC register."]
    #[inline] pub fn sc(&self) -> Sc { 
        unsafe {
            read_volatile(self.sc_ptr())
        }
    }

    #[doc="Write the SC register."]
    #[inline] pub fn set_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc_mut(), f(Sc(0)));
        }
        self
    }

    #[doc="Modify the SC register."]
    #[inline] pub fn with_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc_mut(), f(self.sc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CNT register."]
    #[inline] pub fn cnt_mut(&self) -> *mut Cnt { 
        (self.0 + 0x4) as *mut Cnt
    }

    #[doc="Get the *const pointer for the CNT register."]
    #[inline] pub fn cnt_ptr(&self) -> *const Cnt { 
           self.cnt_mut()
    }

    #[doc="Read the CNT register."]
    #[inline] pub fn cnt(&self) -> Cnt { 
        unsafe {
            read_volatile(self.cnt_ptr())
        }
    }

    #[doc="Write the CNT register."]
    #[inline] pub fn set_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cnt_mut(), f(Cnt(0)));
        }
        self
    }

    #[doc="Modify the CNT register."]
    #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cnt_mut(), f(self.cnt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MOD register."]
    #[inline] pub fn mod_mut(&self) -> *mut Mod { 
        (self.0 + 0x8) as *mut Mod
    }

    #[doc="Get the *const pointer for the MOD register."]
    #[inline] pub fn mod_ptr(&self) -> *const Mod { 
           self.mod_mut()
    }

    #[doc="Read the MOD register."]
    #[inline] pub fn _mod(&self) -> Mod { 
        unsafe {
            read_volatile(self.mod_ptr())
        }
    }

    #[doc="Write the MOD register."]
    #[inline] pub fn set_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mod_mut(), f(Mod(0)));
        }
        self
    }

    #[doc="Modify the MOD register."]
    #[inline] pub fn with_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mod_mut(), f(self._mod()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSC register."]
    #[inline] pub fn csc_mut<I: Into<bits::R6>>(&self, index: I) -> *mut Csc { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0xc + (index << 3)) as *mut Csc
    }

    #[doc="Get the *const pointer for the CSC register."]
    #[inline] pub fn csc_ptr<I: Into<bits::R6>>(&self, index: I) -> *const Csc { 
           self.csc_mut(index)
    }

    #[doc="Read the CSC register."]
    #[inline] pub fn csc<I: Into<bits::R6>>(&self, index: I) -> Csc { 
        unsafe {
            read_volatile(self.csc_ptr(index))
        }
    }

    #[doc="Write the CSC register."]
    #[inline] pub fn set_csc<I: Into<bits::R6>, F: FnOnce(Csc) -> Csc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.csc_mut(index), f(Csc(0)));
        }
        self
    }

    #[doc="Modify the CSC register."]
    #[inline] pub fn with_csc<I: Into<bits::R6> + Copy, F: FnOnce(Csc) -> Csc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.csc_mut(index), f(self.csc(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CV register."]
    #[inline] pub fn cv_mut<I: Into<bits::R6>>(&self, index: I) -> *mut Cv { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x10 + (index << 3)) as *mut Cv
    }

    #[doc="Get the *const pointer for the CV register."]
    #[inline] pub fn cv_ptr<I: Into<bits::R6>>(&self, index: I) -> *const Cv { 
           self.cv_mut(index)
    }

    #[doc="Read the CV register."]
    #[inline] pub fn cv<I: Into<bits::R6>>(&self, index: I) -> Cv { 
        unsafe {
            read_volatile(self.cv_ptr(index))
        }
    }

    #[doc="Write the CV register."]
    #[inline] pub fn set_cv<I: Into<bits::R6>, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cv_mut(index), f(Cv(0)));
        }
        self
    }

    #[doc="Modify the CV register."]
    #[inline] pub fn with_cv<I: Into<bits::R6> + Copy, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cv_mut(index), f(self.cv(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        (self.0 + 0x50) as *mut Status
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
           self.status_mut()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        unsafe {
            read_volatile(self.status_ptr())
        }
    }

    #[doc="Write the STATUS register."]
    #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.status_mut(), f(Status(0)));
        }
        self
    }

    #[doc="Modify the STATUS register."]
    #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.status_mut(), f(self.status()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CONF register."]
    #[inline] pub fn conf_mut(&self) -> *mut Conf { 
        (self.0 + 0x84) as *mut Conf
    }

    #[doc="Get the *const pointer for the CONF register."]
    #[inline] pub fn conf_ptr(&self) -> *const Conf { 
           self.conf_mut()
    }

    #[doc="Read the CONF register."]
    #[inline] pub fn conf(&self) -> Conf { 
        unsafe {
            read_volatile(self.conf_ptr())
        }
    }

    #[doc="Write the CONF register."]
    #[inline] pub fn set_conf<F: FnOnce(Conf) -> Conf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.conf_mut(), f(Conf(0)));
        }
        self
    }

    #[doc="Modify the CONF register."]
    #[inline] pub fn with_conf<F: FnOnce(Conf) -> Conf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.conf_mut(), f(self.conf()));
        }
        self
    }

}

#[doc="Status and Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc(pub u32);
impl Sc {
    #[doc="Prescale Factor Selection"]
    #[inline] pub fn ps(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if PS != 0"]
    #[inline] pub fn test_ps(&self) -> bool {
        self.ps() != 0
    }

    #[doc="Sets the PS field."]
    #[inline] pub fn set_ps<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Mode Selection"]
    #[inline] pub fn cmod(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if CMOD != 0"]
    #[inline] pub fn test_cmod(&self) -> bool {
        self.cmod() != 0
    }

    #[doc="Sets the CMOD field."]
    #[inline] pub fn set_cmod<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Center-Aligned PWM Select"]
    #[inline] pub fn cpwms(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CPWMS != 0"]
    #[inline] pub fn test_cpwms(&self) -> bool {
        self.cpwms() != 0
    }

    #[doc="Sets the CPWMS field."]
    #[inline] pub fn set_cpwms<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Timer Overflow Interrupt Enable"]
    #[inline] pub fn toie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TOIE != 0"]
    #[inline] pub fn test_toie(&self) -> bool {
        self.toie() != 0
    }

    #[doc="Sets the TOIE field."]
    #[inline] pub fn set_toie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Timer Overflow Flag"]
    #[inline] pub fn tof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TOF != 0"]
    #[inline] pub fn test_tof(&self) -> bool {
        self.tof() != 0
    }

    #[doc="Sets the TOF field."]
    #[inline] pub fn set_tof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DMA Enable"]
    #[inline] pub fn dma(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DMA != 0"]
    #[inline] pub fn test_dma(&self) -> bool {
        self.dma() != 0
    }

    #[doc="Sets the DMA field."]
    #[inline] pub fn set_dma<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Sc {
    #[inline]
    fn from(other: u32) -> Self {
         Sc(other)
    }
}

impl ::core::fmt::Display for Sc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ps() != 0 { try!(write!(f, " ps=0x{:x}", self.ps()))}
        if self.cmod() != 0 { try!(write!(f, " cmod=0x{:x}", self.cmod()))}
        if self.cpwms() != 0 { try!(write!(f, " cpwms"))}
        if self.toie() != 0 { try!(write!(f, " toie"))}
        if self.tof() != 0 { try!(write!(f, " tof"))}
        if self.dma() != 0 { try!(write!(f, " dma"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="Counter value"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cnt {
    #[inline]
    fn from(other: u32) -> Self {
         Cnt(other)
    }
}

impl ::core::fmt::Display for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Modulo"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mod(pub u32);
impl Mod {
    #[doc="Modulo value"]
    #[inline] pub fn _mod(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MOD != 0"]
    #[inline] pub fn test_mod(&self) -> bool {
        self._mod() != 0
    }

    #[doc="Sets the MOD field."]
    #[inline] pub fn set_mod<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mod {
    #[inline]
    fn from(other: u32) -> Self {
         Mod(other)
    }
}

impl ::core::fmt::Display for Mod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self._mod() != 0 { try!(write!(f, " mod=0x{:x}", self._mod()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel (n) Status and Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csc(pub u32);
impl Csc {
    #[doc="DMA Enable"]
    #[inline] pub fn dma(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA != 0"]
    #[inline] pub fn test_dma(&self) -> bool {
        self.dma() != 0
    }

    #[doc="Sets the DMA field."]
    #[inline] pub fn set_dma<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Edge or Level Select"]
    #[inline] pub fn elsa(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ELSA != 0"]
    #[inline] pub fn test_elsa(&self) -> bool {
        self.elsa() != 0
    }

    #[doc="Sets the ELSA field."]
    #[inline] pub fn set_elsa<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Edge or Level Select"]
    #[inline] pub fn elsb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ELSB != 0"]
    #[inline] pub fn test_elsb(&self) -> bool {
        self.elsb() != 0
    }

    #[doc="Sets the ELSB field."]
    #[inline] pub fn set_elsb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel Mode Select"]
    #[inline] pub fn msa(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MSA != 0"]
    #[inline] pub fn test_msa(&self) -> bool {
        self.msa() != 0
    }

    #[doc="Sets the MSA field."]
    #[inline] pub fn set_msa<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel Mode Select"]
    #[inline] pub fn msb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MSB != 0"]
    #[inline] pub fn test_msb(&self) -> bool {
        self.msb() != 0
    }

    #[doc="Sets the MSB field."]
    #[inline] pub fn set_msb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel Interrupt Enable"]
    #[inline] pub fn chie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CHIE != 0"]
    #[inline] pub fn test_chie(&self) -> bool {
        self.chie() != 0
    }

    #[doc="Sets the CHIE field."]
    #[inline] pub fn set_chie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel Flag"]
    #[inline] pub fn chf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHF != 0"]
    #[inline] pub fn test_chf(&self) -> bool {
        self.chf() != 0
    }

    #[doc="Sets the CHF field."]
    #[inline] pub fn set_chf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Csc {
    #[inline]
    fn from(other: u32) -> Self {
         Csc(other)
    }
}

impl ::core::fmt::Display for Csc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dma() != 0 { try!(write!(f, " dma"))}
        if self.elsa() != 0 { try!(write!(f, " elsa"))}
        if self.elsb() != 0 { try!(write!(f, " elsb"))}
        if self.msa() != 0 { try!(write!(f, " msa"))}
        if self.msb() != 0 { try!(write!(f, " msb"))}
        if self.chie() != 0 { try!(write!(f, " chie"))}
        if self.chf() != 0 { try!(write!(f, " chf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel (n) Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cv(pub u32);
impl Cv {
    #[doc="Channel Value"]
    #[inline] pub fn val(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if VAL != 0"]
    #[inline] pub fn test_val(&self) -> bool {
        self.val() != 0
    }

    #[doc="Sets the VAL field."]
    #[inline] pub fn set_val<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cv {
    #[inline]
    fn from(other: u32) -> Self {
         Cv(other)
    }
}

impl ::core::fmt::Display for Cv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.val() != 0 { try!(write!(f, " val=0x{:x}", self.val()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capture and Compare Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="Channel n Flag"]
    #[inline] pub fn chf<I: Into<bits::R6>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHF != 0"]
    #[inline] pub fn test_chf<I: Into<bits::R6>>(&self, index: I) -> bool{
        self.chf(index) != 0
    }

    #[doc="Sets the CHF field."]
    #[inline] pub fn set_chf<I: Into<bits::R6>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Status {
    #[inline]
    fn from(other: u32) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.chf(0) != 0 { try!(write!(f, " chf[0]"))}
        if self.chf(1) != 0 { try!(write!(f, " chf[1]"))}
        if self.chf(2) != 0 { try!(write!(f, " chf[2]"))}
        if self.chf(3) != 0 { try!(write!(f, " chf[3]"))}
        if self.chf(4) != 0 { try!(write!(f, " chf[4]"))}
        if self.chf(5) != 0 { try!(write!(f, " chf[5]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Conf(pub u32);
impl Conf {
    #[doc="Doze Enable"]
    #[inline] pub fn dozeen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DOZEEN != 0"]
    #[inline] pub fn test_dozeen(&self) -> bool {
        self.dozeen() != 0
    }

    #[doc="Sets the DOZEEN field."]
    #[inline] pub fn set_dozeen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Debug Mode"]
    #[inline] pub fn dbgmode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if DBGMODE != 0"]
    #[inline] pub fn test_dbgmode(&self) -> bool {
        self.dbgmode() != 0
    }

    #[doc="Sets the DBGMODE field."]
    #[inline] pub fn set_dbgmode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Global time base enable"]
    #[inline] pub fn gtbeen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GTBEEN != 0"]
    #[inline] pub fn test_gtbeen(&self) -> bool {
        self.gtbeen() != 0
    }

    #[doc="Sets the GTBEEN field."]
    #[inline] pub fn set_gtbeen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter Start on Trigger"]
    #[inline] pub fn csot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CSOT != 0"]
    #[inline] pub fn test_csot(&self) -> bool {
        self.csot() != 0
    }

    #[doc="Sets the CSOT field."]
    #[inline] pub fn set_csot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter Stop On Overflow"]
    #[inline] pub fn csoo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CSOO != 0"]
    #[inline] pub fn test_csoo(&self) -> bool {
        self.csoo() != 0
    }

    #[doc="Sets the CSOO field."]
    #[inline] pub fn set_csoo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Counter Reload On Trigger"]
    #[inline] pub fn crot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CROT != 0"]
    #[inline] pub fn test_crot(&self) -> bool {
        self.crot() != 0
    }

    #[doc="Sets the CROT field."]
    #[inline] pub fn set_crot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Trigger Select"]
    #[inline] pub fn trgsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if TRGSEL != 0"]
    #[inline] pub fn test_trgsel(&self) -> bool {
        self.trgsel() != 0
    }

    #[doc="Sets the TRGSEL field."]
    #[inline] pub fn set_trgsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Conf {
    #[inline]
    fn from(other: u32) -> Self {
         Conf(other)
    }
}

impl ::core::fmt::Display for Conf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Conf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dozeen() != 0 { try!(write!(f, " dozeen"))}
        if self.dbgmode() != 0 { try!(write!(f, " dbgmode=0x{:x}", self.dbgmode()))}
        if self.gtbeen() != 0 { try!(write!(f, " gtbeen"))}
        if self.csot() != 0 { try!(write!(f, " csot"))}
        if self.csoo() != 0 { try!(write!(f, " csoo"))}
        if self.crot() != 0 { try!(write!(f, " crot"))}
        if self.trgsel() != 0 { try!(write!(f, " trgsel=0x{:x}", self.trgsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub struct TpmCh { pub periph: TpmPeriph, pub index: usize }

