#[allow(unused_imports)] use bobbin_common::*;

periph!( OPAMP, Opamp, _OPAMP, OpampPeriph, 0x40007800);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="OPAMP Peripheral"]
pub struct OpampPeriph(pub usize); 



impl OpampPeriph {
    #[doc="Get the *mut pointer for the OPAMP1_CSR register."]
    #[inline] pub fn opamp1_csr_mut(&self) -> *mut Opamp1Csr { 
        (self.0 + 0x0) as *mut Opamp1Csr
    }

    #[doc="Get the *const pointer for the OPAMP1_CSR register."]
    #[inline] pub fn opamp1_csr_ptr(&self) -> *const Opamp1Csr { 
           self.opamp1_csr_mut()
    }

    #[doc="Read the OPAMP1_CSR register."]
    #[inline] pub fn opamp1_csr(&self) -> Opamp1Csr { 
        unsafe {
            read_volatile(self.opamp1_csr_ptr())
        }
    }

    #[doc="Write the OPAMP1_CSR register."]
    #[inline] pub fn set_opamp1_csr<F: FnOnce(Opamp1Csr) -> Opamp1Csr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp1_csr_mut(), f(Opamp1Csr(0)));
        }
        self
    }

    #[doc="Modify the OPAMP1_CSR register."]
    #[inline] pub fn with_opamp1_csr<F: FnOnce(Opamp1Csr) -> Opamp1Csr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp1_csr_mut(), f(self.opamp1_csr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OPAMP1_OTR register."]
    #[inline] pub fn opamp1_otr_mut(&self) -> *mut Opamp1Otr { 
        (self.0 + 0x4) as *mut Opamp1Otr
    }

    #[doc="Get the *const pointer for the OPAMP1_OTR register."]
    #[inline] pub fn opamp1_otr_ptr(&self) -> *const Opamp1Otr { 
           self.opamp1_otr_mut()
    }

    #[doc="Read the OPAMP1_OTR register."]
    #[inline] pub fn opamp1_otr(&self) -> Opamp1Otr { 
        unsafe {
            read_volatile(self.opamp1_otr_ptr())
        }
    }

    #[doc="Write the OPAMP1_OTR register."]
    #[inline] pub fn set_opamp1_otr<F: FnOnce(Opamp1Otr) -> Opamp1Otr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp1_otr_mut(), f(Opamp1Otr(0)));
        }
        self
    }

    #[doc="Modify the OPAMP1_OTR register."]
    #[inline] pub fn with_opamp1_otr<F: FnOnce(Opamp1Otr) -> Opamp1Otr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp1_otr_mut(), f(self.opamp1_otr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OPAMP1_LPOTR register."]
    #[inline] pub fn opamp1_lpotr_mut(&self) -> *mut Opamp1Lpotr { 
        (self.0 + 0x8) as *mut Opamp1Lpotr
    }

    #[doc="Get the *const pointer for the OPAMP1_LPOTR register."]
    #[inline] pub fn opamp1_lpotr_ptr(&self) -> *const Opamp1Lpotr { 
           self.opamp1_lpotr_mut()
    }

    #[doc="Read the OPAMP1_LPOTR register."]
    #[inline] pub fn opamp1_lpotr(&self) -> Opamp1Lpotr { 
        unsafe {
            read_volatile(self.opamp1_lpotr_ptr())
        }
    }

    #[doc="Write the OPAMP1_LPOTR register."]
    #[inline] pub fn set_opamp1_lpotr<F: FnOnce(Opamp1Lpotr) -> Opamp1Lpotr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp1_lpotr_mut(), f(Opamp1Lpotr(0)));
        }
        self
    }

    #[doc="Modify the OPAMP1_LPOTR register."]
    #[inline] pub fn with_opamp1_lpotr<F: FnOnce(Opamp1Lpotr) -> Opamp1Lpotr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp1_lpotr_mut(), f(self.opamp1_lpotr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OPAMP2_CSR register."]
    #[inline] pub fn opamp2_csr_mut(&self) -> *mut Opamp2Csr { 
        (self.0 + 0x10) as *mut Opamp2Csr
    }

    #[doc="Get the *const pointer for the OPAMP2_CSR register."]
    #[inline] pub fn opamp2_csr_ptr(&self) -> *const Opamp2Csr { 
           self.opamp2_csr_mut()
    }

    #[doc="Read the OPAMP2_CSR register."]
    #[inline] pub fn opamp2_csr(&self) -> Opamp2Csr { 
        unsafe {
            read_volatile(self.opamp2_csr_ptr())
        }
    }

    #[doc="Write the OPAMP2_CSR register."]
    #[inline] pub fn set_opamp2_csr<F: FnOnce(Opamp2Csr) -> Opamp2Csr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp2_csr_mut(), f(Opamp2Csr(0)));
        }
        self
    }

    #[doc="Modify the OPAMP2_CSR register."]
    #[inline] pub fn with_opamp2_csr<F: FnOnce(Opamp2Csr) -> Opamp2Csr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp2_csr_mut(), f(self.opamp2_csr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OPAMP2_OTR register."]
    #[inline] pub fn opamp2_otr_mut(&self) -> *mut Opamp2Otr { 
        (self.0 + 0x14) as *mut Opamp2Otr
    }

    #[doc="Get the *const pointer for the OPAMP2_OTR register."]
    #[inline] pub fn opamp2_otr_ptr(&self) -> *const Opamp2Otr { 
           self.opamp2_otr_mut()
    }

    #[doc="Read the OPAMP2_OTR register."]
    #[inline] pub fn opamp2_otr(&self) -> Opamp2Otr { 
        unsafe {
            read_volatile(self.opamp2_otr_ptr())
        }
    }

    #[doc="Write the OPAMP2_OTR register."]
    #[inline] pub fn set_opamp2_otr<F: FnOnce(Opamp2Otr) -> Opamp2Otr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp2_otr_mut(), f(Opamp2Otr(0)));
        }
        self
    }

    #[doc="Modify the OPAMP2_OTR register."]
    #[inline] pub fn with_opamp2_otr<F: FnOnce(Opamp2Otr) -> Opamp2Otr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp2_otr_mut(), f(self.opamp2_otr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OPAMP2_LPOTR register."]
    #[inline] pub fn opamp2_lpotr_mut(&self) -> *mut Opamp2Lpotr { 
        (self.0 + 0x18) as *mut Opamp2Lpotr
    }

    #[doc="Get the *const pointer for the OPAMP2_LPOTR register."]
    #[inline] pub fn opamp2_lpotr_ptr(&self) -> *const Opamp2Lpotr { 
           self.opamp2_lpotr_mut()
    }

    #[doc="Read the OPAMP2_LPOTR register."]
    #[inline] pub fn opamp2_lpotr(&self) -> Opamp2Lpotr { 
        unsafe {
            read_volatile(self.opamp2_lpotr_ptr())
        }
    }

    #[doc="Write the OPAMP2_LPOTR register."]
    #[inline] pub fn set_opamp2_lpotr<F: FnOnce(Opamp2Lpotr) -> Opamp2Lpotr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp2_lpotr_mut(), f(Opamp2Lpotr(0)));
        }
        self
    }

    #[doc="Modify the OPAMP2_LPOTR register."]
    #[inline] pub fn with_opamp2_lpotr<F: FnOnce(Opamp2Lpotr) -> Opamp2Lpotr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.opamp2_lpotr_mut(), f(self.opamp2_lpotr()));
        }
        self
    }

}

#[doc="OPAMP1 control/status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Opamp1Csr(pub u32);
impl Opamp1Csr {
    #[doc="Operational amplifier Enable"]
    #[inline] pub fn opaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OPAEN != 0"]
    #[inline] pub fn test_opaen(&self) -> bool {
        self.opaen() != 0
    }

    #[doc="Sets the OPAEN field."]
    #[inline] pub fn set_opaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Operational amplifier Low Power Mode"]
    #[inline] pub fn opalpm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OPALPM != 0"]
    #[inline] pub fn test_opalpm(&self) -> bool {
        self.opalpm() != 0
    }

    #[doc="Sets the OPALPM field."]
    #[inline] pub fn set_opalpm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Operational amplifier PGA mode"]
    #[inline] pub fn opamode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if OPAMODE != 0"]
    #[inline] pub fn test_opamode(&self) -> bool {
        self.opamode() != 0
    }

    #[doc="Sets the OPAMODE field."]
    #[inline] pub fn set_opamode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Operational amplifier Programmable amplifier gain value"]
    #[inline] pub fn pga_gain(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if PGA_GAIN != 0"]
    #[inline] pub fn test_pga_gain(&self) -> bool {
        self.pga_gain() != 0
    }

    #[doc="Sets the PGA_GAIN field."]
    #[inline] pub fn set_pga_gain<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Inverting input selection"]
    #[inline] pub fn vm_sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if VM_SEL != 0"]
    #[inline] pub fn test_vm_sel(&self) -> bool {
        self.vm_sel() != 0
    }

    #[doc="Sets the VM_SEL field."]
    #[inline] pub fn set_vm_sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Non inverted input selection"]
    #[inline] pub fn vp_sel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if VP_SEL != 0"]
    #[inline] pub fn test_vp_sel(&self) -> bool {
        self.vp_sel() != 0
    }

    #[doc="Sets the VP_SEL field."]
    #[inline] pub fn set_vp_sel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Calibration mode enabled"]
    #[inline] pub fn calon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CALON != 0"]
    #[inline] pub fn test_calon(&self) -> bool {
        self.calon() != 0
    }

    #[doc="Sets the CALON field."]
    #[inline] pub fn set_calon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Calibration selection"]
    #[inline] pub fn calsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CALSEL != 0"]
    #[inline] pub fn test_calsel(&self) -> bool {
        self.calsel() != 0
    }

    #[doc="Sets the CALSEL field."]
    #[inline] pub fn set_calsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="allows to switch from AOP offset trimmed values to AOP offset"]
    #[inline] pub fn usertrim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USERTRIM != 0"]
    #[inline] pub fn test_usertrim(&self) -> bool {
        self.usertrim() != 0
    }

    #[doc="Sets the USERTRIM field."]
    #[inline] pub fn set_usertrim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Operational amplifier calibration output"]
    #[inline] pub fn calout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CALOUT != 0"]
    #[inline] pub fn test_calout(&self) -> bool {
        self.calout() != 0
    }

    #[doc="Sets the CALOUT field."]
    #[inline] pub fn set_calout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Operational amplifier power supply range for stability"]
    #[inline] pub fn opa_range(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if OPA_RANGE != 0"]
    #[inline] pub fn test_opa_range(&self) -> bool {
        self.opa_range() != 0
    }

    #[doc="Sets the OPA_RANGE field."]
    #[inline] pub fn set_opa_range<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Opamp1Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Opamp1Csr(other)
    }
}

impl ::core::fmt::Display for Opamp1Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Opamp1Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.opaen() != 0 { try!(write!(f, " opaen"))}
        if self.opalpm() != 0 { try!(write!(f, " opalpm"))}
        if self.opamode() != 0 { try!(write!(f, " opamode=0x{:x}", self.opamode()))}
        if self.pga_gain() != 0 { try!(write!(f, " pga_gain=0x{:x}", self.pga_gain()))}
        if self.vm_sel() != 0 { try!(write!(f, " vm_sel=0x{:x}", self.vm_sel()))}
        if self.vp_sel() != 0 { try!(write!(f, " vp_sel"))}
        if self.calon() != 0 { try!(write!(f, " calon"))}
        if self.calsel() != 0 { try!(write!(f, " calsel"))}
        if self.usertrim() != 0 { try!(write!(f, " usertrim"))}
        if self.calout() != 0 { try!(write!(f, " calout"))}
        if self.opa_range() != 0 { try!(write!(f, " opa_range"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OPAMP1 offset trimming register in normal mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Opamp1Otr(pub u32);
impl Opamp1Otr {
    #[doc="Trim for NMOS differential pairs"]
    #[inline] pub fn trimoffsetn(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if TRIMOFFSETN != 0"]
    #[inline] pub fn test_trimoffsetn(&self) -> bool {
        self.trimoffsetn() != 0
    }

    #[doc="Sets the TRIMOFFSETN field."]
    #[inline] pub fn set_trimoffsetn<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Trim for PMOS differential pairs"]
    #[inline] pub fn trimoffsetp(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if TRIMOFFSETP != 0"]
    #[inline] pub fn test_trimoffsetp(&self) -> bool {
        self.trimoffsetp() != 0
    }

    #[doc="Sets the TRIMOFFSETP field."]
    #[inline] pub fn set_trimoffsetp<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Opamp1Otr {
    #[inline]
    fn from(other: u32) -> Self {
         Opamp1Otr(other)
    }
}

impl ::core::fmt::Display for Opamp1Otr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Opamp1Otr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trimoffsetn() != 0 { try!(write!(f, " trimoffsetn=0x{:x}", self.trimoffsetn()))}
        if self.trimoffsetp() != 0 { try!(write!(f, " trimoffsetp=0x{:x}", self.trimoffsetp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OPAMP1 offset trimming register in low-power mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Opamp1Lpotr(pub u32);
impl Opamp1Lpotr {
    #[doc="Trim for NMOS differential pairs"]
    #[inline] pub fn trimlpoffsetn(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if TRIMLPOFFSETN != 0"]
    #[inline] pub fn test_trimlpoffsetn(&self) -> bool {
        self.trimlpoffsetn() != 0
    }

    #[doc="Sets the TRIMLPOFFSETN field."]
    #[inline] pub fn set_trimlpoffsetn<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Trim for PMOS differential pairs"]
    #[inline] pub fn trimlpoffsetp(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if TRIMLPOFFSETP != 0"]
    #[inline] pub fn test_trimlpoffsetp(&self) -> bool {
        self.trimlpoffsetp() != 0
    }

    #[doc="Sets the TRIMLPOFFSETP field."]
    #[inline] pub fn set_trimlpoffsetp<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Opamp1Lpotr {
    #[inline]
    fn from(other: u32) -> Self {
         Opamp1Lpotr(other)
    }
}

impl ::core::fmt::Display for Opamp1Lpotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Opamp1Lpotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trimlpoffsetn() != 0 { try!(write!(f, " trimlpoffsetn=0x{:x}", self.trimlpoffsetn()))}
        if self.trimlpoffsetp() != 0 { try!(write!(f, " trimlpoffsetp=0x{:x}", self.trimlpoffsetp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OPAMP2 control/status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Opamp2Csr(pub u32);
impl Opamp2Csr {
    #[doc="Operational amplifier Enable"]
    #[inline] pub fn opaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OPAEN != 0"]
    #[inline] pub fn test_opaen(&self) -> bool {
        self.opaen() != 0
    }

    #[doc="Sets the OPAEN field."]
    #[inline] pub fn set_opaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Operational amplifier Low Power Mode"]
    #[inline] pub fn opalpm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OPALPM != 0"]
    #[inline] pub fn test_opalpm(&self) -> bool {
        self.opalpm() != 0
    }

    #[doc="Sets the OPALPM field."]
    #[inline] pub fn set_opalpm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Operational amplifier PGA mode"]
    #[inline] pub fn opamode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if OPAMODE != 0"]
    #[inline] pub fn test_opamode(&self) -> bool {
        self.opamode() != 0
    }

    #[doc="Sets the OPAMODE field."]
    #[inline] pub fn set_opamode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Operational amplifier Programmable amplifier gain value"]
    #[inline] pub fn pga_gain(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if PGA_GAIN != 0"]
    #[inline] pub fn test_pga_gain(&self) -> bool {
        self.pga_gain() != 0
    }

    #[doc="Sets the PGA_GAIN field."]
    #[inline] pub fn set_pga_gain<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Inverting input selection"]
    #[inline] pub fn vm_sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if VM_SEL != 0"]
    #[inline] pub fn test_vm_sel(&self) -> bool {
        self.vm_sel() != 0
    }

    #[doc="Sets the VM_SEL field."]
    #[inline] pub fn set_vm_sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Non inverted input selection"]
    #[inline] pub fn vp_sel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if VP_SEL != 0"]
    #[inline] pub fn test_vp_sel(&self) -> bool {
        self.vp_sel() != 0
    }

    #[doc="Sets the VP_SEL field."]
    #[inline] pub fn set_vp_sel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Calibration mode enabled"]
    #[inline] pub fn calon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CALON != 0"]
    #[inline] pub fn test_calon(&self) -> bool {
        self.calon() != 0
    }

    #[doc="Sets the CALON field."]
    #[inline] pub fn set_calon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Calibration selection"]
    #[inline] pub fn calsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CALSEL != 0"]
    #[inline] pub fn test_calsel(&self) -> bool {
        self.calsel() != 0
    }

    #[doc="Sets the CALSEL field."]
    #[inline] pub fn set_calsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="allows to switch from AOP offset trimmed values to AOP offset"]
    #[inline] pub fn usertrim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USERTRIM != 0"]
    #[inline] pub fn test_usertrim(&self) -> bool {
        self.usertrim() != 0
    }

    #[doc="Sets the USERTRIM field."]
    #[inline] pub fn set_usertrim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Operational amplifier calibration output"]
    #[inline] pub fn calout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CALOUT != 0"]
    #[inline] pub fn test_calout(&self) -> bool {
        self.calout() != 0
    }

    #[doc="Sets the CALOUT field."]
    #[inline] pub fn set_calout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Opamp2Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Opamp2Csr(other)
    }
}

impl ::core::fmt::Display for Opamp2Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Opamp2Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.opaen() != 0 { try!(write!(f, " opaen"))}
        if self.opalpm() != 0 { try!(write!(f, " opalpm"))}
        if self.opamode() != 0 { try!(write!(f, " opamode=0x{:x}", self.opamode()))}
        if self.pga_gain() != 0 { try!(write!(f, " pga_gain=0x{:x}", self.pga_gain()))}
        if self.vm_sel() != 0 { try!(write!(f, " vm_sel=0x{:x}", self.vm_sel()))}
        if self.vp_sel() != 0 { try!(write!(f, " vp_sel"))}
        if self.calon() != 0 { try!(write!(f, " calon"))}
        if self.calsel() != 0 { try!(write!(f, " calsel"))}
        if self.usertrim() != 0 { try!(write!(f, " usertrim"))}
        if self.calout() != 0 { try!(write!(f, " calout"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OPAMP2 offset trimming register in normal mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Opamp2Otr(pub u32);
impl Opamp2Otr {
    #[doc="Trim for NMOS differential pairs"]
    #[inline] pub fn trimoffsetn(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if TRIMOFFSETN != 0"]
    #[inline] pub fn test_trimoffsetn(&self) -> bool {
        self.trimoffsetn() != 0
    }

    #[doc="Sets the TRIMOFFSETN field."]
    #[inline] pub fn set_trimoffsetn<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Trim for PMOS differential pairs"]
    #[inline] pub fn trimoffsetp(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if TRIMOFFSETP != 0"]
    #[inline] pub fn test_trimoffsetp(&self) -> bool {
        self.trimoffsetp() != 0
    }

    #[doc="Sets the TRIMOFFSETP field."]
    #[inline] pub fn set_trimoffsetp<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Opamp2Otr {
    #[inline]
    fn from(other: u32) -> Self {
         Opamp2Otr(other)
    }
}

impl ::core::fmt::Display for Opamp2Otr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Opamp2Otr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trimoffsetn() != 0 { try!(write!(f, " trimoffsetn=0x{:x}", self.trimoffsetn()))}
        if self.trimoffsetp() != 0 { try!(write!(f, " trimoffsetp=0x{:x}", self.trimoffsetp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OPAMP2 offset trimming register in low-power mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Opamp2Lpotr(pub u32);
impl Opamp2Lpotr {
    #[doc="Trim for NMOS differential pairs"]
    #[inline] pub fn trimlpoffsetn(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if TRIMLPOFFSETN != 0"]
    #[inline] pub fn test_trimlpoffsetn(&self) -> bool {
        self.trimlpoffsetn() != 0
    }

    #[doc="Sets the TRIMLPOFFSETN field."]
    #[inline] pub fn set_trimlpoffsetn<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Trim for PMOS differential pairs"]
    #[inline] pub fn trimlpoffsetp(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if TRIMLPOFFSETP != 0"]
    #[inline] pub fn test_trimlpoffsetp(&self) -> bool {
        self.trimlpoffsetp() != 0
    }

    #[doc="Sets the TRIMLPOFFSETP field."]
    #[inline] pub fn set_trimlpoffsetp<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Opamp2Lpotr {
    #[inline]
    fn from(other: u32) -> Self {
         Opamp2Lpotr(other)
    }
}

impl ::core::fmt::Display for Opamp2Lpotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Opamp2Lpotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trimlpoffsetn() != 0 { try!(write!(f, " trimlpoffsetn=0x{:x}", self.trimlpoffsetn()))}
        if self.trimlpoffsetp() != 0 { try!(write!(f, " trimlpoffsetp=0x{:x}", self.trimlpoffsetp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


