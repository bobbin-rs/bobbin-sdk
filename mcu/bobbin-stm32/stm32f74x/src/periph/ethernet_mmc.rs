//! Ethernet: MAC management counters

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Ethernet: MAC management counters"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EthernetMmcPeriph(pub usize);
impl EthernetMmcPeriph {
    #[doc="Get the *mut pointer for the MMCCR register."]
    #[inline] pub fn mmccr_mut(&self) -> *mut Mmccr { 
        (self.0 + 0x0) as *mut Mmccr
    }

    #[doc="Get the *const pointer for the MMCCR register."]
    #[inline] pub fn mmccr_ptr(&self) -> *const Mmccr { 
           self.mmccr_mut()
    }

    #[doc="Read the MMCCR register."]
    #[inline] pub fn mmccr(&self) -> Mmccr { 
        unsafe {
            read_volatile(self.mmccr_ptr())
        }
    }

    #[doc="Write the MMCCR register."]
    #[inline] pub fn set_mmccr<F: FnOnce(Mmccr) -> Mmccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmccr_mut(), f(Mmccr(0)));
        }
        self
    }

    #[doc="Modify the MMCCR register."]
    #[inline] pub fn with_mmccr<F: FnOnce(Mmccr) -> Mmccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmccr_mut(), f(self.mmccr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MMCRIR register."]
    #[inline] pub fn mmcrir_mut(&self) -> *mut Mmcrir { 
        (self.0 + 0x4) as *mut Mmcrir
    }

    #[doc="Get the *const pointer for the MMCRIR register."]
    #[inline] pub fn mmcrir_ptr(&self) -> *const Mmcrir { 
           self.mmcrir_mut()
    }

    #[doc="Read the MMCRIR register."]
    #[inline] pub fn mmcrir(&self) -> Mmcrir { 
        unsafe {
            read_volatile(self.mmcrir_ptr())
        }
    }

    #[doc="Write the MMCRIR register."]
    #[inline] pub fn set_mmcrir<F: FnOnce(Mmcrir) -> Mmcrir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmcrir_mut(), f(Mmcrir(0)));
        }
        self
    }

    #[doc="Modify the MMCRIR register."]
    #[inline] pub fn with_mmcrir<F: FnOnce(Mmcrir) -> Mmcrir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmcrir_mut(), f(self.mmcrir()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MMCTIR register."]
    #[inline] pub fn mmctir_mut(&self) -> *mut Mmctir { 
        (self.0 + 0x8) as *mut Mmctir
    }

    #[doc="Get the *const pointer for the MMCTIR register."]
    #[inline] pub fn mmctir_ptr(&self) -> *const Mmctir { 
           self.mmctir_mut()
    }

    #[doc="Read the MMCTIR register."]
    #[inline] pub fn mmctir(&self) -> Mmctir { 
        unsafe {
            read_volatile(self.mmctir_ptr())
        }
    }

    #[doc="Get the *mut pointer for the MMCRIMR register."]
    #[inline] pub fn mmcrimr_mut(&self) -> *mut Mmcrimr { 
        (self.0 + 0xc) as *mut Mmcrimr
    }

    #[doc="Get the *const pointer for the MMCRIMR register."]
    #[inline] pub fn mmcrimr_ptr(&self) -> *const Mmcrimr { 
           self.mmcrimr_mut()
    }

    #[doc="Read the MMCRIMR register."]
    #[inline] pub fn mmcrimr(&self) -> Mmcrimr { 
        unsafe {
            read_volatile(self.mmcrimr_ptr())
        }
    }

    #[doc="Write the MMCRIMR register."]
    #[inline] pub fn set_mmcrimr<F: FnOnce(Mmcrimr) -> Mmcrimr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmcrimr_mut(), f(Mmcrimr(0)));
        }
        self
    }

    #[doc="Modify the MMCRIMR register."]
    #[inline] pub fn with_mmcrimr<F: FnOnce(Mmcrimr) -> Mmcrimr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmcrimr_mut(), f(self.mmcrimr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MMCTIMR register."]
    #[inline] pub fn mmctimr_mut(&self) -> *mut Mmctimr { 
        (self.0 + 0x10) as *mut Mmctimr
    }

    #[doc="Get the *const pointer for the MMCTIMR register."]
    #[inline] pub fn mmctimr_ptr(&self) -> *const Mmctimr { 
           self.mmctimr_mut()
    }

    #[doc="Read the MMCTIMR register."]
    #[inline] pub fn mmctimr(&self) -> Mmctimr { 
        unsafe {
            read_volatile(self.mmctimr_ptr())
        }
    }

    #[doc="Write the MMCTIMR register."]
    #[inline] pub fn set_mmctimr<F: FnOnce(Mmctimr) -> Mmctimr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmctimr_mut(), f(Mmctimr(0)));
        }
        self
    }

    #[doc="Modify the MMCTIMR register."]
    #[inline] pub fn with_mmctimr<F: FnOnce(Mmctimr) -> Mmctimr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmctimr_mut(), f(self.mmctimr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MMCTGFSCCR register."]
    #[inline] pub fn mmctgfsccr_mut(&self) -> *mut Mmctgfsccr { 
        (self.0 + 0x4c) as *mut Mmctgfsccr
    }

    #[doc="Get the *const pointer for the MMCTGFSCCR register."]
    #[inline] pub fn mmctgfsccr_ptr(&self) -> *const Mmctgfsccr { 
           self.mmctgfsccr_mut()
    }

    #[doc="Read the MMCTGFSCCR register."]
    #[inline] pub fn mmctgfsccr(&self) -> Mmctgfsccr { 
        unsafe {
            read_volatile(self.mmctgfsccr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the MMCTGFMSCCR register."]
    #[inline] pub fn mmctgfmsccr_mut(&self) -> *mut Mmctgfmsccr { 
        (self.0 + 0x50) as *mut Mmctgfmsccr
    }

    #[doc="Get the *const pointer for the MMCTGFMSCCR register."]
    #[inline] pub fn mmctgfmsccr_ptr(&self) -> *const Mmctgfmsccr { 
           self.mmctgfmsccr_mut()
    }

    #[doc="Read the MMCTGFMSCCR register."]
    #[inline] pub fn mmctgfmsccr(&self) -> Mmctgfmsccr { 
        unsafe {
            read_volatile(self.mmctgfmsccr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the MMCTGFCR register."]
    #[inline] pub fn mmctgfcr_mut(&self) -> *mut Mmctgfcr { 
        (self.0 + 0x68) as *mut Mmctgfcr
    }

    #[doc="Get the *const pointer for the MMCTGFCR register."]
    #[inline] pub fn mmctgfcr_ptr(&self) -> *const Mmctgfcr { 
           self.mmctgfcr_mut()
    }

    #[doc="Read the MMCTGFCR register."]
    #[inline] pub fn mmctgfcr(&self) -> Mmctgfcr { 
        unsafe {
            read_volatile(self.mmctgfcr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the MMCRFCECR register."]
    #[inline] pub fn mmcrfcecr_mut(&self) -> *mut Mmcrfcecr { 
        (self.0 + 0x94) as *mut Mmcrfcecr
    }

    #[doc="Get the *const pointer for the MMCRFCECR register."]
    #[inline] pub fn mmcrfcecr_ptr(&self) -> *const Mmcrfcecr { 
           self.mmcrfcecr_mut()
    }

    #[doc="Read the MMCRFCECR register."]
    #[inline] pub fn mmcrfcecr(&self) -> Mmcrfcecr { 
        unsafe {
            read_volatile(self.mmcrfcecr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the MMCRFAECR register."]
    #[inline] pub fn mmcrfaecr_mut(&self) -> *mut Mmcrfaecr { 
        (self.0 + 0x98) as *mut Mmcrfaecr
    }

    #[doc="Get the *const pointer for the MMCRFAECR register."]
    #[inline] pub fn mmcrfaecr_ptr(&self) -> *const Mmcrfaecr { 
           self.mmcrfaecr_mut()
    }

    #[doc="Read the MMCRFAECR register."]
    #[inline] pub fn mmcrfaecr(&self) -> Mmcrfaecr { 
        unsafe {
            read_volatile(self.mmcrfaecr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the MMCRGUFCR register."]
    #[inline] pub fn mmcrgufcr_mut(&self) -> *mut Mmcrgufcr { 
        (self.0 + 0xc4) as *mut Mmcrgufcr
    }

    #[doc="Get the *const pointer for the MMCRGUFCR register."]
    #[inline] pub fn mmcrgufcr_ptr(&self) -> *const Mmcrgufcr { 
           self.mmcrgufcr_mut()
    }

    #[doc="Read the MMCRGUFCR register."]
    #[inline] pub fn mmcrgufcr(&self) -> Mmcrgufcr { 
        unsafe {
            read_volatile(self.mmcrgufcr_ptr())
        }
    }

}

#[doc="Ethernet MMC control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmccr(pub u32);
impl Mmccr {
    #[doc="no description available"]
    #[inline] pub fn cr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CR != 0"]
    #[inline] pub fn test_cr(&self) -> bool {
        self.cr() != 0
    }

    #[doc="Sets the CR field."]
    #[inline] pub fn set_cr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn csr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CSR != 0"]
    #[inline] pub fn test_csr(&self) -> bool {
        self.csr() != 0
    }

    #[doc="Sets the CSR field."]
    #[inline] pub fn set_csr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ror(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ROR != 0"]
    #[inline] pub fn test_ror(&self) -> bool {
        self.ror() != 0
    }

    #[doc="Sets the ROR field."]
    #[inline] pub fn set_ror<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if MCF != 0"]
    #[inline] pub fn test_mcf(&self) -> bool {
        self.mcf() != 0
    }

    #[doc="Sets the MCF field."]
    #[inline] pub fn set_mcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mcp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MCP != 0"]
    #[inline] pub fn test_mcp(&self) -> bool {
        self.mcp() != 0
    }

    #[doc="Sets the MCP field."]
    #[inline] pub fn set_mcp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mcfhp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MCFHP != 0"]
    #[inline] pub fn test_mcfhp(&self) -> bool {
        self.mcfhp() != 0
    }

    #[doc="Sets the MCFHP field."]
    #[inline] pub fn set_mcfhp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Mmccr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmccr(other)
    }
}

impl ::core::fmt::Display for Mmccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cr() != 0 { try!(write!(f, " cr"))}
        if self.csr() != 0 { try!(write!(f, " csr"))}
        if self.ror() != 0 { try!(write!(f, " ror"))}
        if self.mcf() != 0 { try!(write!(f, " mcf"))}
        if self.mcp() != 0 { try!(write!(f, " mcp"))}
        if self.mcfhp() != 0 { try!(write!(f, " mcfhp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC receive interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrir(pub u32);
impl Mmcrir {
    #[doc="no description available"]
    #[inline] pub fn rfces(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RFCES != 0"]
    #[inline] pub fn test_rfces(&self) -> bool {
        self.rfces() != 0
    }

    #[doc="Sets the RFCES field."]
    #[inline] pub fn set_rfces<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rfaes(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RFAES != 0"]
    #[inline] pub fn test_rfaes(&self) -> bool {
        self.rfaes() != 0
    }

    #[doc="Sets the RFAES field."]
    #[inline] pub fn set_rfaes<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rgufs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RGUFS != 0"]
    #[inline] pub fn test_rgufs(&self) -> bool {
        self.rgufs() != 0
    }

    #[doc="Sets the RGUFS field."]
    #[inline] pub fn set_rgufs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Mmcrir {
    #[inline]
    fn from(other: u32) -> Self {
         Mmcrir(other)
    }
}

impl ::core::fmt::Display for Mmcrir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rfces() != 0 { try!(write!(f, " rfces"))}
        if self.rfaes() != 0 { try!(write!(f, " rfaes"))}
        if self.rgufs() != 0 { try!(write!(f, " rgufs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC transmit interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctir(pub u32);
impl Mmctir {
    #[doc="no description available"]
    #[inline] pub fn tgfscs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TGFSCS != 0"]
    #[inline] pub fn test_tgfscs(&self) -> bool {
        self.tgfscs() != 0
    }

    #[doc="Sets the TGFSCS field."]
    #[inline] pub fn set_tgfscs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tgfmscs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TGFMSCS != 0"]
    #[inline] pub fn test_tgfmscs(&self) -> bool {
        self.tgfmscs() != 0
    }

    #[doc="Sets the TGFMSCS field."]
    #[inline] pub fn set_tgfmscs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tgfs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if TGFS != 0"]
    #[inline] pub fn test_tgfs(&self) -> bool {
        self.tgfs() != 0
    }

    #[doc="Sets the TGFS field."]
    #[inline] pub fn set_tgfs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

}

impl From<u32> for Mmctir {
    #[inline]
    fn from(other: u32) -> Self {
         Mmctir(other)
    }
}

impl ::core::fmt::Display for Mmctir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tgfscs() != 0 { try!(write!(f, " tgfscs"))}
        if self.tgfmscs() != 0 { try!(write!(f, " tgfmscs"))}
        if self.tgfs() != 0 { try!(write!(f, " tgfs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC receive interrupt mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrimr(pub u32);
impl Mmcrimr {
    #[doc="no description available"]
    #[inline] pub fn rfcem(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RFCEM != 0"]
    #[inline] pub fn test_rfcem(&self) -> bool {
        self.rfcem() != 0
    }

    #[doc="Sets the RFCEM field."]
    #[inline] pub fn set_rfcem<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rfaem(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RFAEM != 0"]
    #[inline] pub fn test_rfaem(&self) -> bool {
        self.rfaem() != 0
    }

    #[doc="Sets the RFAEM field."]
    #[inline] pub fn set_rfaem<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rgufm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RGUFM != 0"]
    #[inline] pub fn test_rgufm(&self) -> bool {
        self.rgufm() != 0
    }

    #[doc="Sets the RGUFM field."]
    #[inline] pub fn set_rgufm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Mmcrimr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmcrimr(other)
    }
}

impl ::core::fmt::Display for Mmcrimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rfcem() != 0 { try!(write!(f, " rfcem"))}
        if self.rfaem() != 0 { try!(write!(f, " rfaem"))}
        if self.rgufm() != 0 { try!(write!(f, " rgufm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC transmit interrupt mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctimr(pub u32);
impl Mmctimr {
    #[doc="no description available"]
    #[inline] pub fn tgfscm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TGFSCM != 0"]
    #[inline] pub fn test_tgfscm(&self) -> bool {
        self.tgfscm() != 0
    }

    #[doc="Sets the TGFSCM field."]
    #[inline] pub fn set_tgfscm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tgfmscm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TGFMSCM != 0"]
    #[inline] pub fn test_tgfmscm(&self) -> bool {
        self.tgfmscm() != 0
    }

    #[doc="Sets the TGFMSCM field."]
    #[inline] pub fn set_tgfmscm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tgfm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TGFM != 0"]
    #[inline] pub fn test_tgfm(&self) -> bool {
        self.tgfm() != 0
    }

    #[doc="Sets the TGFM field."]
    #[inline] pub fn set_tgfm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Mmctimr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmctimr(other)
    }
}

impl ::core::fmt::Display for Mmctimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tgfscm() != 0 { try!(write!(f, " tgfscm"))}
        if self.tgfmscm() != 0 { try!(write!(f, " tgfmscm"))}
        if self.tgfm() != 0 { try!(write!(f, " tgfm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC transmitted good frames after a single collision counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctgfsccr(pub u32);
impl Mmctgfsccr {
    #[doc="no description available"]
    #[inline] pub fn tgfscc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TGFSCC != 0"]
    #[inline] pub fn test_tgfscc(&self) -> bool {
        self.tgfscc() != 0
    }

    #[doc="Sets the TGFSCC field."]
    #[inline] pub fn set_tgfscc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmctgfsccr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmctgfsccr(other)
    }
}

impl ::core::fmt::Display for Mmctgfsccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctgfsccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC transmitted good frames after more than a single collision"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctgfmsccr(pub u32);
impl Mmctgfmsccr {
    #[doc="no description available"]
    #[inline] pub fn tgfmscc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TGFMSCC != 0"]
    #[inline] pub fn test_tgfmscc(&self) -> bool {
        self.tgfmscc() != 0
    }

    #[doc="Sets the TGFMSCC field."]
    #[inline] pub fn set_tgfmscc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmctgfmsccr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmctgfmsccr(other)
    }
}

impl ::core::fmt::Display for Mmctgfmsccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctgfmsccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC transmitted good frames counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmctgfcr(pub u32);
impl Mmctgfcr {
    #[doc="HTL"]
    #[inline] pub fn tgfc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TGFC != 0"]
    #[inline] pub fn test_tgfc(&self) -> bool {
        self.tgfc() != 0
    }

    #[doc="Sets the TGFC field."]
    #[inline] pub fn set_tgfc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmctgfcr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmctgfcr(other)
    }
}

impl ::core::fmt::Display for Mmctgfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmctgfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC received frames with CRC error counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrfcecr(pub u32);
impl Mmcrfcecr {
    #[doc="no description available"]
    #[inline] pub fn rfcfc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RFCFC != 0"]
    #[inline] pub fn test_rfcfc(&self) -> bool {
        self.rfcfc() != 0
    }

    #[doc="Sets the RFCFC field."]
    #[inline] pub fn set_rfcfc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmcrfcecr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmcrfcecr(other)
    }
}

impl ::core::fmt::Display for Mmcrfcecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrfcecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MMC received frames with alignment error counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrfaecr(pub u32);
impl Mmcrfaecr {
    #[doc="no description available"]
    #[inline] pub fn rfaec(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RFAEC != 0"]
    #[inline] pub fn test_rfaec(&self) -> bool {
        self.rfaec() != 0
    }

    #[doc="Sets the RFAEC field."]
    #[inline] pub fn set_rfaec<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmcrfaecr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmcrfaecr(other)
    }
}

impl ::core::fmt::Display for Mmcrfaecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrfaecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MMC received good unicast frames counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmcrgufcr(pub u32);
impl Mmcrgufcr {
    #[doc="no description available"]
    #[inline] pub fn rgufc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RGUFC != 0"]
    #[inline] pub fn test_rgufc(&self) -> bool {
        self.rgufc() != 0
    }

    #[doc="Sets the RGUFC field."]
    #[inline] pub fn set_rgufc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmcrgufcr {
    #[inline]
    fn from(other: u32) -> Self {
         Mmcrgufcr(other)
    }
}

impl ::core::fmt::Display for Mmcrgufcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmcrgufcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

