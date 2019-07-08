::bobbin_mcu::periph!( PKA, Pka, PKA_PERIPH, PkaPeriph, PKA_OWNED, PKA_REF_COUNT, 0x58002000, 0x00, 0x2c);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PKA Peripheral"]
pub struct PkaPeriph(pub usize); 

impl PkaPeriph {
    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x0)
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        self.cr_reg().read()
    }

    #[doc="Write the CR register."]
    #[inline] pub fn write_cr(&self, value: Cr) -> &Self { 
        self.cr_reg().write(value);
        self
    }

    #[doc="Set the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().set(f);
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().with(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x4)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Get the CLRFR Register."]
    #[inline] pub fn clrfr_reg(&self) -> ::bobbin_mcu::register::Register<Clrfr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clrfr, 0x8)
    }

    #[doc="Get the *mut pointer for the CLRFR register."]
    #[inline] pub fn clrfr_mut(&self) -> *mut Clrfr { 
        self.clrfr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLRFR register."]
    #[inline] pub fn clrfr_ptr(&self) -> *const Clrfr { 
        self.clrfr_reg().ptr()
    }

    #[doc="Read the CLRFR register."]
    #[inline] pub fn clrfr(&self) -> Clrfr { 
        self.clrfr_reg().read()
    }

    #[doc="Write the CLRFR register."]
    #[inline] pub fn write_clrfr(&self, value: Clrfr) -> &Self { 
        self.clrfr_reg().write(value);
        self
    }

    #[doc="Set the CLRFR register."]
    #[inline] pub fn set_clrfr<F: FnOnce(Clrfr) -> Clrfr>(&self, f: F) -> &Self {
        self.clrfr_reg().set(f);
        self
    }

    #[doc="Modify the CLRFR register."]
    #[inline] pub fn with_clrfr<F: FnOnce(Clrfr) -> Clrfr>(&self, f: F) -> &Self {
        self.clrfr_reg().with(f);
        self
    }

    #[doc="Get the VERR Register."]
    #[inline] pub fn verr_reg(&self) -> ::bobbin_mcu::register::Register<Verr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Verr, 0x1ff4)
    }

    #[doc="Get the *mut pointer for the VERR register."]
    #[inline] pub fn verr_mut(&self) -> *mut Verr { 
        self.verr_reg().ptr()
    }

    #[doc="Get the *const pointer for the VERR register."]
    #[inline] pub fn verr_ptr(&self) -> *const Verr { 
        self.verr_reg().ptr()
    }

    #[doc="Read the VERR register."]
    #[inline] pub fn verr(&self) -> Verr { 
        self.verr_reg().read()
    }

    #[doc="Get the IPIDR Register."]
    #[inline] pub fn ipidr_reg(&self) -> ::bobbin_mcu::register::Register<Ipidr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ipidr, 0x1ff8)
    }

    #[doc="Get the *mut pointer for the IPIDR register."]
    #[inline] pub fn ipidr_mut(&self) -> *mut Ipidr { 
        self.ipidr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IPIDR register."]
    #[inline] pub fn ipidr_ptr(&self) -> *const Ipidr { 
        self.ipidr_reg().ptr()
    }

    #[doc="Read the IPIDR register."]
    #[inline] pub fn ipidr(&self) -> Ipidr { 
        self.ipidr_reg().read()
    }

    #[doc="Get the SIDR Register."]
    #[inline] pub fn sidr_reg(&self) -> ::bobbin_mcu::register::Register<Sidr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sidr, 0x1ffc)
    }

    #[doc="Get the *mut pointer for the SIDR register."]
    #[inline] pub fn sidr_mut(&self) -> *mut Sidr { 
        self.sidr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SIDR register."]
    #[inline] pub fn sidr_ptr(&self) -> *const Sidr { 
        self.sidr_reg().ptr()
    }

    #[doc="Read the SIDR register."]
    #[inline] pub fn sidr(&self) -> Sidr { 
        self.sidr_reg().read()
    }

}

#[doc="Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Address error interrupt enable"]
    #[inline] pub fn addrerrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if ADDRERRIE != 0"]
    #[inline] pub fn test_addrerrie(&self) -> bool {
        self.addrerrie() != 0
    }

    #[doc="Sets the ADDRERRIE field."]
    #[inline] pub fn set_addrerrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="RAM error interrupt enable"]
    #[inline] pub fn ramerrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RAMERRIE != 0"]
    #[inline] pub fn test_ramerrie(&self) -> bool {
        self.ramerrie() != 0
    }

    #[doc="Sets the RAMERRIE field."]
    #[inline] pub fn set_ramerrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn procendie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PROCENDIE != 0"]
    #[inline] pub fn test_procendie(&self) -> bool {
        self.procendie() != 0
    }

    #[doc="Sets the PROCENDIE field."]
    #[inline] pub fn set_procendie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="PKA Operation Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Security Enable"]
    #[inline] pub fn seclvl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SECLVL != 0"]
    #[inline] pub fn test_seclvl(&self) -> bool {
        self.seclvl() != 0
    }

    #[doc="Sets the SECLVL field."]
    #[inline] pub fn set_seclvl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Start the operation"]
    #[inline] pub fn start(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Peripheral Enable"]
    #[inline] pub fn en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="Sets the EN field."]
    #[inline] pub fn set_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addrerrie() != 0 { try!(write!(f, " addrerrie"))}
        if self.ramerrie() != 0 { try!(write!(f, " ramerrie"))}
        if self.procendie() != 0 { try!(write!(f, " procendie"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.seclvl() != 0 { try!(write!(f, " seclvl"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.en() != 0 { try!(write!(f, " en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PKA status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Address error flag"]
    #[inline] pub fn addrerrf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if ADDRERRF != 0"]
    #[inline] pub fn test_addrerrf(&self) -> bool {
        self.addrerrf() != 0
    }

    #[doc="Sets the ADDRERRF field."]
    #[inline] pub fn set_addrerrf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="RAM error flag"]
    #[inline] pub fn ramerrf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RAMERRF != 0"]
    #[inline] pub fn test_ramerrf(&self) -> bool {
        self.ramerrf() != 0
    }

    #[doc="Sets the RAMERRF field."]
    #[inline] pub fn set_ramerrf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="PKA End of Operation flag"]
    #[inline] pub fn procendf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PROCENDF != 0"]
    #[inline] pub fn test_procendf(&self) -> bool {
        self.procendf() != 0
    }

    #[doc="Sets the PROCENDF field."]
    #[inline] pub fn set_procendf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="PKA Operation in progress"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
    }
}

impl ::core::fmt::Display for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addrerrf() != 0 { try!(write!(f, " addrerrf"))}
        if self.ramerrf() != 0 { try!(write!(f, " ramerrf"))}
        if self.procendf() != 0 { try!(write!(f, " procendf"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PKA clear flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clrfr(pub u32);
impl Clrfr {
    #[doc="Clear Address error flag"]
    #[inline] pub fn addrerrfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if ADDRERRFC != 0"]
    #[inline] pub fn test_addrerrfc(&self) -> bool {
        self.addrerrfc() != 0
    }

    #[doc="Sets the ADDRERRFC field."]
    #[inline] pub fn set_addrerrfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Clear RAM error flag"]
    #[inline] pub fn ramerrfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RAMERRFC != 0"]
    #[inline] pub fn test_ramerrfc(&self) -> bool {
        self.ramerrfc() != 0
    }

    #[doc="Sets the RAMERRFC field."]
    #[inline] pub fn set_ramerrfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Clear PKA End of Operation flag"]
    #[inline] pub fn procendfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PROCENDFC != 0"]
    #[inline] pub fn test_procendfc(&self) -> bool {
        self.procendfc() != 0
    }

    #[doc="Sets the PROCENDFC field."]
    #[inline] pub fn set_procendfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Clrfr {
    #[inline]
    fn from(other: u32) -> Self {
         Clrfr(other)
    }
}

impl ::core::fmt::Display for Clrfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clrfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addrerrfc() != 0 { try!(write!(f, " addrerrfc"))}
        if self.ramerrfc() != 0 { try!(write!(f, " ramerrfc"))}
        if self.procendfc() != 0 { try!(write!(f, " procendfc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PKA version register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verr(pub u32);
impl Verr {
    #[doc="Minor revision"]
    #[inline] pub fn minrev(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MINREV != 0"]
    #[inline] pub fn test_minrev(&self) -> bool {
        self.minrev() != 0
    }

    #[doc="Sets the MINREV field."]
    #[inline] pub fn set_minrev<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Major revision"]
    #[inline] pub fn majrev(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if MAJREV != 0"]
    #[inline] pub fn test_majrev(&self) -> bool {
        self.majrev() != 0
    }

    #[doc="Sets the MAJREV field."]
    #[inline] pub fn set_majrev<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Verr {
    #[inline]
    fn from(other: u32) -> Self {
         Verr(other)
    }
}

impl ::core::fmt::Display for Verr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Verr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.minrev() != 0 { try!(write!(f, " minrev=0x{:x}", self.minrev()))}
        if self.majrev() != 0 { try!(write!(f, " majrev=0x{:x}", self.majrev()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PKA identification register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipidr(pub u32);
impl Ipidr {
    #[doc="Identification Code"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ipidr {
    #[inline]
    fn from(other: u32) -> Self {
         Ipidr(other)
    }
}

impl ::core::fmt::Display for Ipidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PKA size ID register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sidr(pub u32);
impl Sidr {
    #[doc="Side Identification Code"]
    #[inline] pub fn sid(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SID != 0"]
    #[inline] pub fn test_sid(&self) -> bool {
        self.sid() != 0
    }

    #[doc="Sets the SID field."]
    #[inline] pub fn set_sid<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sidr {
    #[inline]
    fn from(other: u32) -> Self {
         Sidr(other)
    }
}

impl ::core::fmt::Display for Sidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

