
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPTMR Peripheral"]
pub struct LptmrPeriph(pub usize); 

impl LptmrPeriph {
    #[doc="Get the CSR Register."]
    #[inline] pub fn csr_reg(&self) -> ::bobbin_mcu::register::Register<Csr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr, 0x0)
    }

    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut Csr { 
        self.csr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR register."]
    #[inline] pub fn csr_ptr(&self) -> *const Csr { 
        self.csr_reg().ptr()
    }

    #[doc="Read the CSR register."]
    #[inline] pub fn csr(&self) -> Csr { 
        self.csr_reg().read()
    }

    #[doc="Write the CSR register."]
    #[inline] pub fn write_csr(&self, value: Csr) -> &Self { 
        self.csr_reg().write(value);
        self
    }

    #[doc="Set the CSR register."]
    #[inline] pub fn set_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        self.csr_reg().set(f);
        self
    }

    #[doc="Modify the CSR register."]
    #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        self.csr_reg().with(f);
        self
    }

    #[doc="Get the PSR Register."]
    #[inline] pub fn psr_reg(&self) -> ::bobbin_mcu::register::Register<Psr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Psr, 0x4)
    }

    #[doc="Get the *mut pointer for the PSR register."]
    #[inline] pub fn psr_mut(&self) -> *mut Psr { 
        self.psr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PSR register."]
    #[inline] pub fn psr_ptr(&self) -> *const Psr { 
        self.psr_reg().ptr()
    }

    #[doc="Read the PSR register."]
    #[inline] pub fn psr(&self) -> Psr { 
        self.psr_reg().read()
    }

    #[doc="Write the PSR register."]
    #[inline] pub fn write_psr(&self, value: Psr) -> &Self { 
        self.psr_reg().write(value);
        self
    }

    #[doc="Set the PSR register."]
    #[inline] pub fn set_psr<F: FnOnce(Psr) -> Psr>(&self, f: F) -> &Self {
        self.psr_reg().set(f);
        self
    }

    #[doc="Modify the PSR register."]
    #[inline] pub fn with_psr<F: FnOnce(Psr) -> Psr>(&self, f: F) -> &Self {
        self.psr_reg().with(f);
        self
    }

    #[doc="Get the CMR Register."]
    #[inline] pub fn cmr_reg(&self) -> ::bobbin_mcu::register::Register<Cmr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cmr, 0x8)
    }

    #[doc="Get the *mut pointer for the CMR register."]
    #[inline] pub fn cmr_mut(&self) -> *mut Cmr { 
        self.cmr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CMR register."]
    #[inline] pub fn cmr_ptr(&self) -> *const Cmr { 
        self.cmr_reg().ptr()
    }

    #[doc="Read the CMR register."]
    #[inline] pub fn cmr(&self) -> Cmr { 
        self.cmr_reg().read()
    }

    #[doc="Write the CMR register."]
    #[inline] pub fn write_cmr(&self, value: Cmr) -> &Self { 
        self.cmr_reg().write(value);
        self
    }

    #[doc="Set the CMR register."]
    #[inline] pub fn set_cmr<F: FnOnce(Cmr) -> Cmr>(&self, f: F) -> &Self {
        self.cmr_reg().set(f);
        self
    }

    #[doc="Modify the CMR register."]
    #[inline] pub fn with_cmr<F: FnOnce(Cmr) -> Cmr>(&self, f: F) -> &Self {
        self.cmr_reg().with(f);
        self
    }

    #[doc="Get the CNR Register."]
    #[inline] pub fn cnr_reg(&self) -> ::bobbin_mcu::register::Register<Cnr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cnr, 0xc)
    }

    #[doc="Get the *mut pointer for the CNR register."]
    #[inline] pub fn cnr_mut(&self) -> *mut Cnr { 
        self.cnr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNR register."]
    #[inline] pub fn cnr_ptr(&self) -> *const Cnr { 
        self.cnr_reg().ptr()
    }

    #[doc="Read the CNR register."]
    #[inline] pub fn cnr(&self) -> Cnr { 
        self.cnr_reg().read()
    }

    #[doc="Write the CNR register."]
    #[inline] pub fn write_cnr(&self, value: Cnr) -> &Self { 
        self.cnr_reg().write(value);
        self
    }

    #[doc="Set the CNR register."]
    #[inline] pub fn set_cnr<F: FnOnce(Cnr) -> Cnr>(&self, f: F) -> &Self {
        self.cnr_reg().set(f);
        self
    }

    #[doc="Modify the CNR register."]
    #[inline] pub fn with_cnr<F: FnOnce(Cnr) -> Cnr>(&self, f: F) -> &Self {
        self.cnr_reg().with(f);
        self
    }

}

#[doc="Low Power Timer Control Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc="Timer Enable"]
    #[inline] pub fn ten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TEN != 0"]
    #[inline] pub fn test_ten(&self) -> bool {
        self.ten() != 0
    }

    #[doc="Sets the TEN field."]
    #[inline] pub fn set_ten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer Mode Select"]
    #[inline] pub fn tms(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TMS != 0"]
    #[inline] pub fn test_tms(&self) -> bool {
        self.tms() != 0
    }

    #[doc="Sets the TMS field."]
    #[inline] pub fn set_tms<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Timer Free-Running Counter"]
    #[inline] pub fn tfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TFC != 0"]
    #[inline] pub fn test_tfc(&self) -> bool {
        self.tfc() != 0
    }

    #[doc="Sets the TFC field."]
    #[inline] pub fn set_tfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Timer Pin Polarity"]
    #[inline] pub fn tpp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TPP != 0"]
    #[inline] pub fn test_tpp(&self) -> bool {
        self.tpp() != 0
    }

    #[doc="Sets the TPP field."]
    #[inline] pub fn set_tpp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Timer Pin Select"]
    #[inline] pub fn tps(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if TPS != 0"]
    #[inline] pub fn test_tps(&self) -> bool {
        self.tps() != 0
    }

    #[doc="Sets the TPS field."]
    #[inline] pub fn set_tps<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Timer Interrupt Enable"]
    #[inline] pub fn tie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIE != 0"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie() != 0
    }

    #[doc="Sets the TIE field."]
    #[inline] pub fn set_tie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Timer Compare Flag"]
    #[inline] pub fn tcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TCF != 0"]
    #[inline] pub fn test_tcf(&self) -> bool {
        self.tcf() != 0
    }

    #[doc="Sets the TCF field."]
    #[inline] pub fn set_tcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Csr(other)
    }
}

impl ::core::fmt::Display for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ten() != 0 { try!(write!(f, " ten"))}
        if self.tms() != 0 { try!(write!(f, " tms"))}
        if self.tfc() != 0 { try!(write!(f, " tfc"))}
        if self.tpp() != 0 { try!(write!(f, " tpp"))}
        if self.tps() != 0 { try!(write!(f, " tps=0x{:x}", self.tps()))}
        if self.tie() != 0 { try!(write!(f, " tie"))}
        if self.tcf() != 0 { try!(write!(f, " tcf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Power Timer Prescale Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Psr(pub u32);
impl Psr {
    #[doc="Prescaler Clock Select"]
    #[inline] pub fn pcs(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Prescaler Bypass"]
    #[inline] pub fn pbyp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PBYP != 0"]
    #[inline] pub fn test_pbyp(&self) -> bool {
        self.pbyp() != 0
    }

    #[doc="Sets the PBYP field."]
    #[inline] pub fn set_pbyp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Prescale Value"]
    #[inline] pub fn prescale(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0xf) as u8) } // [6:3]
    }

    #[doc="Returns true if PRESCALE != 0"]
    #[inline] pub fn test_prescale(&self) -> bool {
        self.prescale() != 0
    }

    #[doc="Sets the PRESCALE field."]
    #[inline] pub fn set_prescale<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Psr {
    #[inline]
    fn from(other: u32) -> Self {
         Psr(other)
    }
}

impl ::core::fmt::Display for Psr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Psr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.pbyp() != 0 { try!(write!(f, " pbyp"))}
        if self.prescale() != 0 { try!(write!(f, " prescale=0x{:x}", self.prescale()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Power Timer Compare Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmr(pub u32);
impl Cmr {
    #[doc="Compare Value"]
    #[inline] pub fn compare(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if COMPARE != 0"]
    #[inline] pub fn test_compare(&self) -> bool {
        self.compare() != 0
    }

    #[doc="Sets the COMPARE field."]
    #[inline] pub fn set_compare<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmr {
    #[inline]
    fn from(other: u32) -> Self {
         Cmr(other)
    }
}

impl ::core::fmt::Display for Cmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.compare() != 0 { try!(write!(f, " compare=0x{:x}", self.compare()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Low Power Timer Counter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnr(pub u32);
impl Cnr {
    #[doc="Counter Value"]
    #[inline] pub fn counter(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if COUNTER != 0"]
    #[inline] pub fn test_counter(&self) -> bool {
        self.counter() != 0
    }

    #[doc="Sets the COUNTER field."]
    #[inline] pub fn set_counter<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cnr {
    #[inline]
    fn from(other: u32) -> Self {
         Cnr(other)
    }
}

impl ::core::fmt::Display for Cnr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cnr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.counter() != 0 { try!(write!(f, " counter=0x{:x}", self.counter()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

