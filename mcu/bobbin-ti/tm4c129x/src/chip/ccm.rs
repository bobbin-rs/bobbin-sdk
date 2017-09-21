//! Register map for CCM0 peripheral
#[allow(unused_imports)] use bobbin_common::*;

periph!(CCM0, Ccm, 0x44030000);

#[doc="Register map for CCM0 peripheral"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ccm(pub usize);
impl Ccm {
    #[doc="Get the *mut pointer for the CRCCTRL register."]
    #[inline] pub fn crcctrl_mut(&self) -> *mut Crcctrl { 
        (self.0 + 0x400) as *mut Crcctrl
    }

    #[doc="Get the *const pointer for the CRCCTRL register."]
    #[inline] pub fn crcctrl_ptr(&self) -> *const Crcctrl { 
           self.crcctrl_mut()
    }

    #[doc="Read the CRCCTRL register."]
    #[inline] pub fn crcctrl(&self) -> Crcctrl { 
        unsafe {
            read_volatile(self.crcctrl_ptr())
        }
    }

    #[doc="Write the CRCCTRL register."]
    #[inline] pub fn set_crcctrl<F: FnOnce(Crcctrl) -> Crcctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crcctrl_mut(), f(Crcctrl(0)));
        }
        self
    }

    #[doc="Modify the CRCCTRL register."]
    #[inline] pub fn with_crcctrl<F: FnOnce(Crcctrl) -> Crcctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crcctrl_mut(), f(self.crcctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CRCSEED register."]
    #[inline] pub fn crcseed_mut(&self) -> *mut Crcseed { 
        (self.0 + 0x410) as *mut Crcseed
    }

    #[doc="Get the *const pointer for the CRCSEED register."]
    #[inline] pub fn crcseed_ptr(&self) -> *const Crcseed { 
           self.crcseed_mut()
    }

    #[doc="Read the CRCSEED register."]
    #[inline] pub fn crcseed(&self) -> Crcseed { 
        unsafe {
            read_volatile(self.crcseed_ptr())
        }
    }

    #[doc="Write the CRCSEED register."]
    #[inline] pub fn set_crcseed<F: FnOnce(Crcseed) -> Crcseed>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crcseed_mut(), f(Crcseed(0)));
        }
        self
    }

    #[doc="Modify the CRCSEED register."]
    #[inline] pub fn with_crcseed<F: FnOnce(Crcseed) -> Crcseed>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crcseed_mut(), f(self.crcseed()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CRCDIN register."]
    #[inline] pub fn crcdin_mut(&self) -> *mut Crcdin { 
        (self.0 + 0x414) as *mut Crcdin
    }

    #[doc="Get the *const pointer for the CRCDIN register."]
    #[inline] pub fn crcdin_ptr(&self) -> *const Crcdin { 
           self.crcdin_mut()
    }

    #[doc="Read the CRCDIN register."]
    #[inline] pub fn crcdin(&self) -> Crcdin { 
        unsafe {
            read_volatile(self.crcdin_ptr())
        }
    }

    #[doc="Write the CRCDIN register."]
    #[inline] pub fn set_crcdin<F: FnOnce(Crcdin) -> Crcdin>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crcdin_mut(), f(Crcdin(0)));
        }
        self
    }

    #[doc="Modify the CRCDIN register."]
    #[inline] pub fn with_crcdin<F: FnOnce(Crcdin) -> Crcdin>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crcdin_mut(), f(self.crcdin()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CRCRSLTPP register."]
    #[inline] pub fn crcrsltpp_mut(&self) -> *mut Crcrsltpp { 
        (self.0 + 0x418) as *mut Crcrsltpp
    }

    #[doc="Get the *const pointer for the CRCRSLTPP register."]
    #[inline] pub fn crcrsltpp_ptr(&self) -> *const Crcrsltpp { 
           self.crcrsltpp_mut()
    }

    #[doc="Read the CRCRSLTPP register."]
    #[inline] pub fn crcrsltpp(&self) -> Crcrsltpp { 
        unsafe {
            read_volatile(self.crcrsltpp_ptr())
        }
    }

    #[doc="Write the CRCRSLTPP register."]
    #[inline] pub fn set_crcrsltpp<F: FnOnce(Crcrsltpp) -> Crcrsltpp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crcrsltpp_mut(), f(Crcrsltpp(0)));
        }
        self
    }

    #[doc="Modify the CRCRSLTPP register."]
    #[inline] pub fn with_crcrsltpp<F: FnOnce(Crcrsltpp) -> Crcrsltpp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crcrsltpp_mut(), f(self.crcrsltpp()));
        }
        self
    }

}

#[doc="CRC Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crcctrl(pub u32);
impl Crcctrl {
    #[doc="Operation Type"]
    #[inline] pub fn ype(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if YPE != 0"]
    #[inline] pub fn test_ype(&self) -> bool {
        self.ype() != 0
    }

    #[doc="Sets the YPE field."]
    #[inline] pub fn set_ype<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endian Control"]
    #[inline] pub fn endian(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if ENDIAN != 0"]
    #[inline] pub fn test_endian(&self) -> bool {
        self.endian() != 0
    }

    #[doc="Sets the ENDIAN field."]
    #[inline] pub fn set_endian<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Bit reverse enable"]
    #[inline] pub fn br(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BR != 0"]
    #[inline] pub fn test_br(&self) -> bool {
        self.br() != 0
    }

    #[doc="Sets the BR field."]
    #[inline] pub fn set_br<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Output Reverse Enable"]
    #[inline] pub fn obr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OBR != 0"]
    #[inline] pub fn test_obr(&self) -> bool {
        self.obr() != 0
    }

    #[doc="Sets the OBR field."]
    #[inline] pub fn set_obr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Result Inverse Enable"]
    #[inline] pub fn resinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RESINV != 0"]
    #[inline] pub fn test_resinv(&self) -> bool {
        self.resinv() != 0
    }

    #[doc="Sets the RESINV field."]
    #[inline] pub fn set_resinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Input Data Size"]
    #[inline] pub fn size(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SIZE != 0"]
    #[inline] pub fn test_size(&self) -> bool {
        self.size() != 0
    }

    #[doc="Sets the SIZE field."]
    #[inline] pub fn set_size<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CRC Initialization"]
    #[inline] pub fn init(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if INIT != 0"]
    #[inline] pub fn test_init(&self) -> bool {
        self.init() != 0
    }

    #[doc="Sets the INIT field."]
    #[inline] pub fn set_init<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Crcctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Crcctrl(other)
    }
}

impl ::core::fmt::Display for Crcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ype() != 0 { try!(write!(f, " ype=0x{:x}", self.ype()))}
        if self.endian() != 0 { try!(write!(f, " endian=0x{:x}", self.endian()))}
        if self.br() != 0 { try!(write!(f, " br"))}
        if self.obr() != 0 { try!(write!(f, " obr"))}
        if self.resinv() != 0 { try!(write!(f, " resinv"))}
        if self.size() != 0 { try!(write!(f, " size"))}
        if self.init() != 0 { try!(write!(f, " init=0x{:x}", self.init()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC SEED/Context"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crcseed(pub u32);
impl Crcseed {
    #[doc="SEED/Context Value"]
    #[inline] pub fn seed(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SEED != 0"]
    #[inline] pub fn test_seed(&self) -> bool {
        self.seed() != 0
    }

    #[doc="Sets the SEED field."]
    #[inline] pub fn set_seed<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Crcseed {
    #[inline]
    fn from(other: u32) -> Self {
         Crcseed(other)
    }
}

impl ::core::fmt::Display for Crcseed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crcseed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC Data Input"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crcdin(pub u32);
impl Crcdin {
    #[doc="Data Input"]
    #[inline] pub fn datain(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATAIN != 0"]
    #[inline] pub fn test_datain(&self) -> bool {
        self.datain() != 0
    }

    #[doc="Sets the DATAIN field."]
    #[inline] pub fn set_datain<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Crcdin {
    #[inline]
    fn from(other: u32) -> Self {
         Crcdin(other)
    }
}

impl ::core::fmt::Display for Crcdin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crcdin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC Post Processing Result"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crcrsltpp(pub u32);
impl Crcrsltpp {
    #[doc="Post Processing Result"]
    #[inline] pub fn rsltpp(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RSLTPP != 0"]
    #[inline] pub fn test_rsltpp(&self) -> bool {
        self.rsltpp() != 0
    }

    #[doc="Sets the RSLTPP field."]
    #[inline] pub fn set_rsltpp<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Crcrsltpp {
    #[inline]
    fn from(other: u32) -> Self {
         Crcrsltpp(other)
    }
}

impl ::core::fmt::Display for Crcrsltpp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crcrsltpp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}


