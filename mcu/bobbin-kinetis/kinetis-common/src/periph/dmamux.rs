
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMAMUX Peripheral"]
pub struct DmamuxPeriph(pub usize); 

impl DmamuxPeriph {
    #[doc="Get the *mut pointer for the CHCFG register."]
    #[inline] pub fn chcfg_mut<I: Into<bits::R16>>(&self, index: I) -> *mut Chcfg { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x0 + (index)) as *mut Chcfg
    }

    #[doc="Get the *const pointer for the CHCFG register."]
    #[inline] pub fn chcfg_ptr<I: Into<bits::R16>>(&self, index: I) -> *const Chcfg { 
           self.chcfg_mut(index)
    }

    #[doc="Read the CHCFG register."]
    #[inline] pub fn chcfg<I: Into<bits::R16>>(&self, index: I) -> Chcfg { 
        unsafe {
            read_volatile(self.chcfg_ptr(index))
        }
    }

    #[doc="Write the CHCFG register."]
    #[inline] pub fn set_chcfg<I: Into<bits::R16>, F: FnOnce(Chcfg) -> Chcfg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg_mut(index), f(Chcfg(0)));
        }
        self
    }

    #[doc="Modify the CHCFG register."]
    #[inline] pub fn with_chcfg<I: Into<bits::R16> + Copy, F: FnOnce(Chcfg) -> Chcfg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg_mut(index), f(self.chcfg(index)));
        }
        self
    }

}

#[doc="Channel Configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg(pub u8);
impl Chcfg {
    #[doc="DMA Channel Source (Slot)"]
    #[inline] pub fn source(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if SOURCE != 0"]
    #[inline] pub fn test_source(&self) -> bool {
        self.source() != 0
    }

    #[doc="Sets the SOURCE field."]
    #[inline] pub fn set_source<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DMA Channel Trigger Enable"]
    #[inline] pub fn trig(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TRIG != 0"]
    #[inline] pub fn test_trig(&self) -> bool {
        self.trig() != 0
    }

    #[doc="Sets the TRIG field."]
    #[inline] pub fn set_trig<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DMA Channel Enable"]
    #[inline] pub fn enbl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ENBL != 0"]
    #[inline] pub fn test_enbl(&self) -> bool {
        self.enbl() != 0
    }

    #[doc="Sets the ENBL field."]
    #[inline] pub fn set_enbl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Chcfg {
    #[inline]
    fn from(other: u8) -> Self {
         Chcfg(other)
    }
}

impl ::core::fmt::Display for Chcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.source() != 0 { try!(write!(f, " source=0x{:x}", self.source()))}
        if self.trig() != 0 { try!(write!(f, " trig"))}
        if self.enbl() != 0 { try!(write!(f, " enbl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

