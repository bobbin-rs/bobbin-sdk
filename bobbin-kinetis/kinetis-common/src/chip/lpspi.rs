#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPSPI Peripheral"]
pub struct LpspiPeriph(pub usize); 


impl LpspiPeriph {
    #[doc="Get the *const pointer for the VERID register."]
    #[inline] pub fn verid_ptr(&self) -> *const Verid { 
        (self.0 + 0x0) as *const Verid
    }

    #[doc="Get the *mut pointer for the VERID register."]
    #[inline] pub fn verid_mut(&self) -> *mut Verid { 
        (self.0 + 0x0) as *mut Verid
    }

    #[doc="Read the VERID register."]
    #[inline] pub fn verid(&self) -> Verid { 
        unsafe {
            read_volatile((self.0 + 0x0) as *const Verid)
        }
    }

    #[doc="Get the *const pointer for the PARAM register."]
    #[inline] pub fn param_ptr(&self) -> *const Param { 
        (self.0 + 0x4) as *const Param
    }

    #[doc="Get the *mut pointer for the PARAM register."]
    #[inline] pub fn param_mut(&self) -> *mut Param { 
        (self.0 + 0x4) as *mut Param
    }

    #[doc="Read the PARAM register."]
    #[inline] pub fn param(&self) -> Param { 
        unsafe {
            read_volatile((self.0 + 0x4) as *const Param)
        }
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
        (self.0 + 0x10) as *const Cr
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        (self.0 + 0x10) as *mut Cr
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            read_volatile((self.0 + 0x10) as *const Cr)
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x10) as *mut Cr, f(Cr(0)));
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x10) as *mut Cr, f(self.cr()));
        }
        self
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        (self.0 + 0x14) as *const Sr
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        (self.0 + 0x14) as *mut Sr
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        unsafe {
            read_volatile((self.0 + 0x14) as *const Sr)
        }
    }

    #[doc="Write the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x14) as *mut Sr, f(Sr(0)));
        }
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x14) as *mut Sr, f(self.sr()));
        }
        self
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const Ier { 
        (self.0 + 0x18) as *const Ier
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        (self.0 + 0x18) as *mut Ier
    }

    #[doc="Read the IER register."]
    #[inline] pub fn ier(&self) -> Ier { 
        unsafe {
            read_volatile((self.0 + 0x18) as *const Ier)
        }
    }

    #[doc="Write the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x18) as *mut Ier, f(Ier(0)));
        }
        self
    }

    #[doc="Modify the IER register."]
    #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x18) as *mut Ier, f(self.ier()));
        }
        self
    }

    #[doc="Get the *const pointer for the DER register."]
    #[inline] pub fn der_ptr(&self) -> *const Der { 
        (self.0 + 0x1c) as *const Der
    }

    #[doc="Get the *mut pointer for the DER register."]
    #[inline] pub fn der_mut(&self) -> *mut Der { 
        (self.0 + 0x1c) as *mut Der
    }

    #[doc="Read the DER register."]
    #[inline] pub fn der(&self) -> Der { 
        unsafe {
            read_volatile((self.0 + 0x1c) as *const Der)
        }
    }

    #[doc="Write the DER register."]
    #[inline] pub fn set_der<F: FnOnce(Der) -> Der>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut Der, f(Der(0)));
        }
        self
    }

    #[doc="Modify the DER register."]
    #[inline] pub fn with_der<F: FnOnce(Der) -> Der>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut Der, f(self.der()));
        }
        self
    }

    #[doc="Get the *const pointer for the CFGR0 register."]
    #[inline] pub fn cfgr0_ptr(&self) -> *const Cfgr0 { 
        (self.0 + 0x20) as *const Cfgr0
    }

    #[doc="Get the *mut pointer for the CFGR0 register."]
    #[inline] pub fn cfgr0_mut(&self) -> *mut Cfgr0 { 
        (self.0 + 0x20) as *mut Cfgr0
    }

    #[doc="Read the CFGR0 register."]
    #[inline] pub fn cfgr0(&self) -> Cfgr0 { 
        unsafe {
            read_volatile((self.0 + 0x20) as *const Cfgr0)
        }
    }

    #[doc="Write the CFGR0 register."]
    #[inline] pub fn set_cfgr0<F: FnOnce(Cfgr0) -> Cfgr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x20) as *mut Cfgr0, f(Cfgr0(0)));
        }
        self
    }

    #[doc="Modify the CFGR0 register."]
    #[inline] pub fn with_cfgr0<F: FnOnce(Cfgr0) -> Cfgr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x20) as *mut Cfgr0, f(self.cfgr0()));
        }
        self
    }

    #[doc="Get the *const pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_ptr(&self) -> *const Cfgr1 { 
        (self.0 + 0x24) as *const Cfgr1
    }

    #[doc="Get the *mut pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_mut(&self) -> *mut Cfgr1 { 
        (self.0 + 0x24) as *mut Cfgr1
    }

    #[doc="Read the CFGR1 register."]
    #[inline] pub fn cfgr1(&self) -> Cfgr1 { 
        unsafe {
            read_volatile((self.0 + 0x24) as *const Cfgr1)
        }
    }

    #[doc="Write the CFGR1 register."]
    #[inline] pub fn set_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x24) as *mut Cfgr1, f(Cfgr1(0)));
        }
        self
    }

    #[doc="Modify the CFGR1 register."]
    #[inline] pub fn with_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x24) as *mut Cfgr1, f(self.cfgr1()));
        }
        self
    }

    #[doc="Get the *const pointer for the DMR0 register."]
    #[inline] pub fn dmr0_ptr(&self) -> *const Dmr0 { 
        (self.0 + 0x30) as *const Dmr0
    }

    #[doc="Get the *mut pointer for the DMR0 register."]
    #[inline] pub fn dmr0_mut(&self) -> *mut Dmr0 { 
        (self.0 + 0x30) as *mut Dmr0
    }

    #[doc="Read the DMR0 register."]
    #[inline] pub fn dmr0(&self) -> Dmr0 { 
        unsafe {
            read_volatile((self.0 + 0x30) as *const Dmr0)
        }
    }

    #[doc="Write the DMR0 register."]
    #[inline] pub fn set_dmr0<F: FnOnce(Dmr0) -> Dmr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x30) as *mut Dmr0, f(Dmr0(0)));
        }
        self
    }

    #[doc="Modify the DMR0 register."]
    #[inline] pub fn with_dmr0<F: FnOnce(Dmr0) -> Dmr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x30) as *mut Dmr0, f(self.dmr0()));
        }
        self
    }

    #[doc="Get the *const pointer for the DMR1 register."]
    #[inline] pub fn dmr1_ptr(&self) -> *const Dmr1 { 
        (self.0 + 0x34) as *const Dmr1
    }

    #[doc="Get the *mut pointer for the DMR1 register."]
    #[inline] pub fn dmr1_mut(&self) -> *mut Dmr1 { 
        (self.0 + 0x34) as *mut Dmr1
    }

    #[doc="Read the DMR1 register."]
    #[inline] pub fn dmr1(&self) -> Dmr1 { 
        unsafe {
            read_volatile((self.0 + 0x34) as *const Dmr1)
        }
    }

    #[doc="Write the DMR1 register."]
    #[inline] pub fn set_dmr1<F: FnOnce(Dmr1) -> Dmr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x34) as *mut Dmr1, f(Dmr1(0)));
        }
        self
    }

    #[doc="Modify the DMR1 register."]
    #[inline] pub fn with_dmr1<F: FnOnce(Dmr1) -> Dmr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x34) as *mut Dmr1, f(self.dmr1()));
        }
        self
    }

    #[doc="Get the *const pointer for the CCR register."]
    #[inline] pub fn ccr_ptr(&self) -> *const Ccr { 
        (self.0 + 0x40) as *const Ccr
    }

    #[doc="Get the *mut pointer for the CCR register."]
    #[inline] pub fn ccr_mut(&self) -> *mut Ccr { 
        (self.0 + 0x40) as *mut Ccr
    }

    #[doc="Read the CCR register."]
    #[inline] pub fn ccr(&self) -> Ccr { 
        unsafe {
            read_volatile((self.0 + 0x40) as *const Ccr)
        }
    }

    #[doc="Write the CCR register."]
    #[inline] pub fn set_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x40) as *mut Ccr, f(Ccr(0)));
        }
        self
    }

    #[doc="Modify the CCR register."]
    #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x40) as *mut Ccr, f(self.ccr()));
        }
        self
    }

    #[doc="Get the *const pointer for the FCR register."]
    #[inline] pub fn fcr_ptr(&self) -> *const Fcr { 
        (self.0 + 0x58) as *const Fcr
    }

    #[doc="Get the *mut pointer for the FCR register."]
    #[inline] pub fn fcr_mut(&self) -> *mut Fcr { 
        (self.0 + 0x58) as *mut Fcr
    }

    #[doc="Read the FCR register."]
    #[inline] pub fn fcr(&self) -> Fcr { 
        unsafe {
            read_volatile((self.0 + 0x58) as *const Fcr)
        }
    }

    #[doc="Write the FCR register."]
    #[inline] pub fn set_fcr<F: FnOnce(Fcr) -> Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x58) as *mut Fcr, f(Fcr(0)));
        }
        self
    }

    #[doc="Modify the FCR register."]
    #[inline] pub fn with_fcr<F: FnOnce(Fcr) -> Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x58) as *mut Fcr, f(self.fcr()));
        }
        self
    }

    #[doc="Get the *const pointer for the FSR register."]
    #[inline] pub fn fsr_ptr(&self) -> *const Fsr { 
        (self.0 + 0x5c) as *const Fsr
    }

    #[doc="Get the *mut pointer for the FSR register."]
    #[inline] pub fn fsr_mut(&self) -> *mut Fsr { 
        (self.0 + 0x5c) as *mut Fsr
    }

    #[doc="Read the FSR register."]
    #[inline] pub fn fsr(&self) -> Fsr { 
        unsafe {
            read_volatile((self.0 + 0x5c) as *const Fsr)
        }
    }

    #[doc="Get the *const pointer for the TCR register."]
    #[inline] pub fn tcr_ptr(&self) -> *const Tcr { 
        (self.0 + 0x60) as *const Tcr
    }

    #[doc="Get the *mut pointer for the TCR register."]
    #[inline] pub fn tcr_mut(&self) -> *mut Tcr { 
        (self.0 + 0x60) as *mut Tcr
    }

    #[doc="Read the TCR register."]
    #[inline] pub fn tcr(&self) -> Tcr { 
        unsafe {
            read_volatile((self.0 + 0x60) as *const Tcr)
        }
    }

    #[doc="Write the TCR register."]
    #[inline] pub fn set_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x60) as *mut Tcr, f(Tcr(0)));
        }
        self
    }

    #[doc="Modify the TCR register."]
    #[inline] pub fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x60) as *mut Tcr, f(self.tcr()));
        }
        self
    }

    #[doc="Get the *const pointer for the TDR register."]
    #[inline] pub fn tdr_ptr(&self) -> *const Tdr { 
        (self.0 + 0x64) as *const Tdr
    }

    #[doc="Get the *mut pointer for the TDR register."]
    #[inline] pub fn tdr_mut(&self) -> *mut Tdr { 
        (self.0 + 0x64) as *mut Tdr
    }

    #[doc="Write the TDR register."]
    #[inline] pub fn set_tdr<F: FnOnce(Tdr) -> Tdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x64) as *mut Tdr, f(Tdr(0)));
        }
        self
    }

    #[doc="Get the *const pointer for the RSR register."]
    #[inline] pub fn rsr_ptr(&self) -> *const Rsr { 
        (self.0 + 0x70) as *const Rsr
    }

    #[doc="Get the *mut pointer for the RSR register."]
    #[inline] pub fn rsr_mut(&self) -> *mut Rsr { 
        (self.0 + 0x70) as *mut Rsr
    }

    #[doc="Read the RSR register."]
    #[inline] pub fn rsr(&self) -> Rsr { 
        unsafe {
            read_volatile((self.0 + 0x70) as *const Rsr)
        }
    }

    #[doc="Get the *const pointer for the RDR register."]
    #[inline] pub fn rdr_ptr(&self) -> *const Rdr { 
        (self.0 + 0x74) as *const Rdr
    }

    #[doc="Get the *mut pointer for the RDR register."]
    #[inline] pub fn rdr_mut(&self) -> *mut Rdr { 
        (self.0 + 0x74) as *mut Rdr
    }

    #[doc="Read the RDR register."]
    #[inline] pub fn rdr(&self) -> Rdr { 
        unsafe {
            read_volatile((self.0 + 0x74) as *const Rdr)
        }
    }

}

#[doc="Version ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc="Module Identification Number"]
    #[inline] pub fn feature(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Module Identification Number"]
    #[inline] pub fn test_feature(&self) -> bool {
        self.feature() != 0
    }

    #[doc="Module Identification Number"]
    #[inline] pub fn set_feature<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Minor Version Number"]
    #[inline] pub fn minor(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Minor Version Number"]
    #[inline] pub fn test_minor(&self) -> bool {
        self.minor() != 0
    }

    #[doc="Minor Version Number"]
    #[inline] pub fn set_minor<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Major Version Number"]
    #[inline] pub fn major(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Major Version Number"]
    #[inline] pub fn test_major(&self) -> bool {
        self.major() != 0
    }

    #[doc="Major Version Number"]
    #[inline] pub fn set_major<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Verid {
    #[inline]
    fn from(other: u32) -> Self {
         Verid(other)
    }
}

impl ::core::fmt::Display for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.feature() != 0 { try!(write!(f, " feature=0x{:x}", self.feature()))}
        if self.minor() != 0 { try!(write!(f, " minor=0x{:x}", self.minor()))}
        if self.major() != 0 { try!(write!(f, " major=0x{:x}", self.major()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Parameter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
    #[doc="Transmit FIFO Size"]
    #[inline] pub fn txfifo(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Transmit FIFO Size"]
    #[inline] pub fn test_txfifo(&self) -> bool {
        self.txfifo() != 0
    }

    #[doc="Transmit FIFO Size"]
    #[inline] pub fn set_txfifo<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Size"]
    #[inline] pub fn rxfifo(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Receive FIFO Size"]
    #[inline] pub fn test_rxfifo(&self) -> bool {
        self.rxfifo() != 0
    }

    #[doc="Receive FIFO Size"]
    #[inline] pub fn set_rxfifo<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Param {
    #[inline]
    fn from(other: u32) -> Self {
         Param(other)
    }
}

impl ::core::fmt::Display for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfifo() != 0 { try!(write!(f, " txfifo=0x{:x}", self.txfifo()))}
        if self.rxfifo() != 0 { try!(write!(f, " rxfifo=0x{:x}", self.rxfifo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Module Enable"]
    #[inline] pub fn men(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Module Enable"]
    #[inline] pub fn test_men(&self) -> bool {
        self.men() != 0
    }

    #[doc="Module Enable"]
    #[inline] pub fn set_men<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Software Reset"]
    #[inline] pub fn rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Software Reset"]
    #[inline] pub fn test_rst(&self) -> bool {
        self.rst() != 0
    }

    #[doc="Software Reset"]
    #[inline] pub fn set_rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Doze mode enable"]
    #[inline] pub fn dozen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Doze mode enable"]
    #[inline] pub fn test_dozen(&self) -> bool {
        self.dozen() != 0
    }

    #[doc="Doze mode enable"]
    #[inline] pub fn set_dozen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Debug Enable"]
    #[inline] pub fn dbgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Debug Enable"]
    #[inline] pub fn test_dbgen(&self) -> bool {
        self.dbgen() != 0
    }

    #[doc="Debug Enable"]
    #[inline] pub fn set_dbgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Reset Transmit FIFO"]
    #[inline] pub fn rtf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Reset Transmit FIFO"]
    #[inline] pub fn test_rtf(&self) -> bool {
        self.rtf() != 0
    }

    #[doc="Reset Transmit FIFO"]
    #[inline] pub fn set_rtf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Reset Receive FIFO"]
    #[inline] pub fn rrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Reset Receive FIFO"]
    #[inline] pub fn test_rrf(&self) -> bool {
        self.rrf() != 0
    }

    #[doc="Reset Receive FIFO"]
    #[inline] pub fn set_rrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
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
        if self.men() != 0 { try!(write!(f, " men"))}
        if self.rst() != 0 { try!(write!(f, " rst"))}
        if self.dozen() != 0 { try!(write!(f, " dozen"))}
        if self.dbgen() != 0 { try!(write!(f, " dbgen"))}
        if self.rtf() != 0 { try!(write!(f, " rtf"))}
        if self.rrf() != 0 { try!(write!(f, " rrf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Transmit Data Flag"]
    #[inline] pub fn tdf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Transmit Data Flag"]
    #[inline] pub fn test_tdf(&self) -> bool {
        self.tdf() != 0
    }

    #[doc="Transmit Data Flag"]
    #[inline] pub fn set_tdf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data Flag"]
    #[inline] pub fn rdf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Receive Data Flag"]
    #[inline] pub fn test_rdf(&self) -> bool {
        self.rdf() != 0
    }

    #[doc="Receive Data Flag"]
    #[inline] pub fn set_rdf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Word Complete Flag"]
    #[inline] pub fn wcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Word Complete Flag"]
    #[inline] pub fn test_wcf(&self) -> bool {
        self.wcf() != 0
    }

    #[doc="Word Complete Flag"]
    #[inline] pub fn set_wcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame Complete Flag"]
    #[inline] pub fn fcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Frame Complete Flag"]
    #[inline] pub fn test_fcf(&self) -> bool {
        self.fcf() != 0
    }

    #[doc="Frame Complete Flag"]
    #[inline] pub fn set_fcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transfer Complete Flag"]
    #[inline] pub fn tcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Transfer Complete Flag"]
    #[inline] pub fn test_tcf(&self) -> bool {
        self.tcf() != 0
    }

    #[doc="Transfer Complete Flag"]
    #[inline] pub fn set_tcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Transmit Error Flag"]
    #[inline] pub fn tef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Transmit Error Flag"]
    #[inline] pub fn test_tef(&self) -> bool {
        self.tef() != 0
    }

    #[doc="Transmit Error Flag"]
    #[inline] pub fn set_tef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Receive Error Flag"]
    #[inline] pub fn _ref(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Receive Error Flag"]
    #[inline] pub fn test_ref(&self) -> bool {
        self._ref() != 0
    }

    #[doc="Receive Error Flag"]
    #[inline] pub fn set_ref<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Match Flag"]
    #[inline] pub fn dmf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Data Match Flag"]
    #[inline] pub fn test_dmf(&self) -> bool {
        self.dmf() != 0
    }

    #[doc="Data Match Flag"]
    #[inline] pub fn set_dmf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Module Busy Flag"]
    #[inline] pub fn mbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Module Busy Flag"]
    #[inline] pub fn test_mbf(&self) -> bool {
        self.mbf() != 0
    }

    #[doc="Module Busy Flag"]
    #[inline] pub fn set_mbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
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
        if self.tdf() != 0 { try!(write!(f, " tdf"))}
        if self.rdf() != 0 { try!(write!(f, " rdf"))}
        if self.wcf() != 0 { try!(write!(f, " wcf"))}
        if self.fcf() != 0 { try!(write!(f, " fcf"))}
        if self.tcf() != 0 { try!(write!(f, " tcf"))}
        if self.tef() != 0 { try!(write!(f, " tef"))}
        if self._ref() != 0 { try!(write!(f, " _ref"))}
        if self.dmf() != 0 { try!(write!(f, " dmf"))}
        if self.mbf() != 0 { try!(write!(f, " mbf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="Transmit Data Interrupt Enable"]
    #[inline] pub fn tdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Transmit Data Interrupt Enable"]
    #[inline] pub fn test_tdie(&self) -> bool {
        self.tdie() != 0
    }

    #[doc="Transmit Data Interrupt Enable"]
    #[inline] pub fn set_tdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data Interrupt Enable"]
    #[inline] pub fn rdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Receive Data Interrupt Enable"]
    #[inline] pub fn test_rdie(&self) -> bool {
        self.rdie() != 0
    }

    #[doc="Receive Data Interrupt Enable"]
    #[inline] pub fn set_rdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Word Complete Interrupt Enable"]
    #[inline] pub fn wcie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Word Complete Interrupt Enable"]
    #[inline] pub fn test_wcie(&self) -> bool {
        self.wcie() != 0
    }

    #[doc="Word Complete Interrupt Enable"]
    #[inline] pub fn set_wcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame Complete Interrupt Enable"]
    #[inline] pub fn fcie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Frame Complete Interrupt Enable"]
    #[inline] pub fn test_fcie(&self) -> bool {
        self.fcie() != 0
    }

    #[doc="Frame Complete Interrupt Enable"]
    #[inline] pub fn set_fcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transfer Complete Interrupt Enable"]
    #[inline] pub fn tcie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Transfer Complete Interrupt Enable"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Transfer Complete Interrupt Enable"]
    #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Transmit Error Interrupt Enable"]
    #[inline] pub fn teie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Transmit Error Interrupt Enable"]
    #[inline] pub fn test_teie(&self) -> bool {
        self.teie() != 0
    }

    #[doc="Transmit Error Interrupt Enable"]
    #[inline] pub fn set_teie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Receive Error Interrupt Enable"]
    #[inline] pub fn reie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Receive Error Interrupt Enable"]
    #[inline] pub fn test_reie(&self) -> bool {
        self.reie() != 0
    }

    #[doc="Receive Error Interrupt Enable"]
    #[inline] pub fn set_reie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Match Interrupt Enable"]
    #[inline] pub fn dmie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Data Match Interrupt Enable"]
    #[inline] pub fn test_dmie(&self) -> bool {
        self.dmie() != 0
    }

    #[doc="Data Match Interrupt Enable"]
    #[inline] pub fn set_dmie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Ier {
    #[inline]
    fn from(other: u32) -> Self {
         Ier(other)
    }
}

impl ::core::fmt::Display for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tdie() != 0 { try!(write!(f, " tdie"))}
        if self.rdie() != 0 { try!(write!(f, " rdie"))}
        if self.wcie() != 0 { try!(write!(f, " wcie"))}
        if self.fcie() != 0 { try!(write!(f, " fcie"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.teie() != 0 { try!(write!(f, " teie"))}
        if self.reie() != 0 { try!(write!(f, " reie"))}
        if self.dmie() != 0 { try!(write!(f, " dmie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Der(pub u32);
impl Der {
    #[doc="Transmit Data DMA Enable"]
    #[inline] pub fn tdde(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Transmit Data DMA Enable"]
    #[inline] pub fn test_tdde(&self) -> bool {
        self.tdde() != 0
    }

    #[doc="Transmit Data DMA Enable"]
    #[inline] pub fn set_tdde<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data DMA Enable"]
    #[inline] pub fn rdde(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Receive Data DMA Enable"]
    #[inline] pub fn test_rdde(&self) -> bool {
        self.rdde() != 0
    }

    #[doc="Receive Data DMA Enable"]
    #[inline] pub fn set_rdde<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Der {
    #[inline]
    fn from(other: u32) -> Self {
         Der(other)
    }
}

impl ::core::fmt::Display for Der {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Der {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tdde() != 0 { try!(write!(f, " tdde"))}
        if self.rdde() != 0 { try!(write!(f, " rdde"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr0(pub u32);
impl Cfgr0 {
    #[doc="Host Request Enable"]
    #[inline] pub fn hren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Host Request Enable"]
    #[inline] pub fn test_hren(&self) -> bool {
        self.hren() != 0
    }

    #[doc="Host Request Enable"]
    #[inline] pub fn set_hren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Host Request Polarity"]
    #[inline] pub fn hrpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Host Request Polarity"]
    #[inline] pub fn test_hrpol(&self) -> bool {
        self.hrpol() != 0
    }

    #[doc="Host Request Polarity"]
    #[inline] pub fn set_hrpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Host Request Select"]
    #[inline] pub fn hrsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Host Request Select"]
    #[inline] pub fn test_hrsel(&self) -> bool {
        self.hrsel() != 0
    }

    #[doc="Host Request Select"]
    #[inline] pub fn set_hrsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Circular FIFO Enable"]
    #[inline] pub fn cirfifo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Circular FIFO Enable"]
    #[inline] pub fn test_cirfifo(&self) -> bool {
        self.cirfifo() != 0
    }

    #[doc="Circular FIFO Enable"]
    #[inline] pub fn set_cirfifo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive Data Match Only"]
    #[inline] pub fn rdmo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Receive Data Match Only"]
    #[inline] pub fn test_rdmo(&self) -> bool {
        self.rdmo() != 0
    }

    #[doc="Receive Data Match Only"]
    #[inline] pub fn set_rdmo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Cfgr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr0(other)
    }
}

impl ::core::fmt::Display for Cfgr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hren() != 0 { try!(write!(f, " hren"))}
        if self.hrpol() != 0 { try!(write!(f, " hrpol"))}
        if self.hrsel() != 0 { try!(write!(f, " hrsel"))}
        if self.cirfifo() != 0 { try!(write!(f, " cirfifo"))}
        if self.rdmo() != 0 { try!(write!(f, " rdmo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
    #[doc="Master Mode"]
    #[inline] pub fn master(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Master Mode"]
    #[inline] pub fn test_master(&self) -> bool {
        self.master() != 0
    }

    #[doc="Master Mode"]
    #[inline] pub fn set_master<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Sample Point"]
    #[inline] pub fn sample(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Sample Point"]
    #[inline] pub fn test_sample(&self) -> bool {
        self.sample() != 0
    }

    #[doc="Sample Point"]
    #[inline] pub fn set_sample<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Automatic PCS"]
    #[inline] pub fn autopcs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Automatic PCS"]
    #[inline] pub fn test_autopcs(&self) -> bool {
        self.autopcs() != 0
    }

    #[doc="Automatic PCS"]
    #[inline] pub fn set_autopcs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="No Stall"]
    #[inline] pub fn nostall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="No Stall"]
    #[inline] pub fn test_nostall(&self) -> bool {
        self.nostall() != 0
    }

    #[doc="No Stall"]
    #[inline] pub fn set_nostall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Peripheral Chip Select Polarity"]
    #[inline] pub fn pcspol(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Peripheral Chip Select Polarity"]
    #[inline] pub fn test_pcspol(&self) -> bool {
        self.pcspol() != 0
    }

    #[doc="Peripheral Chip Select Polarity"]
    #[inline] pub fn set_pcspol<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Match Configuration"]
    #[inline] pub fn matcfg(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Match Configuration"]
    #[inline] pub fn test_matcfg(&self) -> bool {
        self.matcfg() != 0
    }

    #[doc="Match Configuration"]
    #[inline] pub fn set_matcfg<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pin Configuration"]
    #[inline] pub fn pincfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Pin Configuration"]
    #[inline] pub fn test_pincfg(&self) -> bool {
        self.pincfg() != 0
    }

    #[doc="Pin Configuration"]
    #[inline] pub fn set_pincfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Output Config"]
    #[inline] pub fn outcfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Output Config"]
    #[inline] pub fn test_outcfg(&self) -> bool {
        self.outcfg() != 0
    }

    #[doc="Output Config"]
    #[inline] pub fn set_outcfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Peripheral Chip Select Configuration"]
    #[inline] pub fn pcscfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Peripheral Chip Select Configuration"]
    #[inline] pub fn test_pcscfg(&self) -> bool {
        self.pcscfg() != 0
    }

    #[doc="Peripheral Chip Select Configuration"]
    #[inline] pub fn set_pcscfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Cfgr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr1(other)
    }
}

impl ::core::fmt::Display for Cfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.master() != 0 { try!(write!(f, " master"))}
        if self.sample() != 0 { try!(write!(f, " sample"))}
        if self.autopcs() != 0 { try!(write!(f, " autopcs"))}
        if self.nostall() != 0 { try!(write!(f, " nostall"))}
        if self.pcspol() != 0 { try!(write!(f, " pcspol=0x{:x}", self.pcspol()))}
        if self.matcfg() != 0 { try!(write!(f, " matcfg=0x{:x}", self.matcfg()))}
        if self.pincfg() != 0 { try!(write!(f, " pincfg=0x{:x}", self.pincfg()))}
        if self.outcfg() != 0 { try!(write!(f, " outcfg"))}
        if self.pcscfg() != 0 { try!(write!(f, " pcscfg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Match Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmr0(pub u32);
impl Dmr0 {
    #[doc="Match 0 Value"]
    #[inline] pub fn match0(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Match 0 Value"]
    #[inline] pub fn test_match0(&self) -> bool {
        self.match0() != 0
    }

    #[doc="Match 0 Value"]
    #[inline] pub fn set_match0<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Dmr0(other)
    }
}

impl ::core::fmt::Display for Dmr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Match Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmr1(pub u32);
impl Dmr1 {
    #[doc="Match 1 Value"]
    #[inline] pub fn match1(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Match 1 Value"]
    #[inline] pub fn test_match1(&self) -> bool {
        self.match1() != 0
    }

    #[doc="Match 1 Value"]
    #[inline] pub fn set_match1<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dmr1(other)
    }
}

impl ::core::fmt::Display for Dmr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc="SCK Divider"]
    #[inline] pub fn sckdiv(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="SCK Divider"]
    #[inline] pub fn test_sckdiv(&self) -> bool {
        self.sckdiv() != 0
    }

    #[doc="SCK Divider"]
    #[inline] pub fn set_sckdiv<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Delay Between Transfers"]
    #[inline] pub fn dbt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Delay Between Transfers"]
    #[inline] pub fn test_dbt(&self) -> bool {
        self.dbt() != 0
    }

    #[doc="Delay Between Transfers"]
    #[inline] pub fn set_dbt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PCS to SCK Delay"]
    #[inline] pub fn pcssck(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="PCS to SCK Delay"]
    #[inline] pub fn test_pcssck(&self) -> bool {
        self.pcssck() != 0
    }

    #[doc="PCS to SCK Delay"]
    #[inline] pub fn set_pcssck<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="SCK to PCS Delay"]
    #[inline] pub fn sckpcs(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="SCK to PCS Delay"]
    #[inline] pub fn test_sckpcs(&self) -> bool {
        self.sckpcs() != 0
    }

    #[doc="SCK to PCS Delay"]
    #[inline] pub fn set_sckpcs<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ccr {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr(other)
    }
}

impl ::core::fmt::Display for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sckdiv() != 0 { try!(write!(f, " sckdiv=0x{:x}", self.sckdiv()))}
        if self.dbt() != 0 { try!(write!(f, " dbt=0x{:x}", self.dbt()))}
        if self.pcssck() != 0 { try!(write!(f, " pcssck=0x{:x}", self.pcssck()))}
        if self.sckpcs() != 0 { try!(write!(f, " sckpcs=0x{:x}", self.sckpcs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fcr(pub u32);
impl Fcr {
    #[doc="Transmit FIFO Watermark"]
    #[inline] pub fn txwater(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Transmit FIFO Watermark"]
    #[inline] pub fn test_txwater(&self) -> bool {
        self.txwater() != 0
    }

    #[doc="Transmit FIFO Watermark"]
    #[inline] pub fn set_txwater<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Watermark"]
    #[inline] pub fn rxwater(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Receive FIFO Watermark"]
    #[inline] pub fn test_rxwater(&self) -> bool {
        self.rxwater() != 0
    }

    #[doc="Receive FIFO Watermark"]
    #[inline] pub fn set_rxwater<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Fcr {
    #[inline]
    fn from(other: u32) -> Self {
         Fcr(other)
    }
}

impl ::core::fmt::Display for Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txwater() != 0 { try!(write!(f, " txwater=0x{:x}", self.txwater()))}
        if self.rxwater() != 0 { try!(write!(f, " rxwater=0x{:x}", self.rxwater()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fsr(pub u32);
impl Fsr {
    #[doc="Transmit FIFO Count"]
    #[inline] pub fn txcount(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Transmit FIFO Count"]
    #[inline] pub fn test_txcount(&self) -> bool {
        self.txcount() != 0
    }

    #[doc="Transmit FIFO Count"]
    #[inline] pub fn set_txcount<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Count"]
    #[inline] pub fn rxcount(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Receive FIFO Count"]
    #[inline] pub fn test_rxcount(&self) -> bool {
        self.rxcount() != 0
    }

    #[doc="Receive FIFO Count"]
    #[inline] pub fn set_rxcount<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Fsr {
    #[inline]
    fn from(other: u32) -> Self {
         Fsr(other)
    }
}

impl ::core::fmt::Display for Fsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txcount() != 0 { try!(write!(f, " txcount=0x{:x}", self.txcount()))}
        if self.rxcount() != 0 { try!(write!(f, " rxcount=0x{:x}", self.rxcount()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit Command Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc="Frame Size"]
    #[inline] pub fn framesz(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Frame Size"]
    #[inline] pub fn test_framesz(&self) -> bool {
        self.framesz() != 0
    }

    #[doc="Frame Size"]
    #[inline] pub fn set_framesz<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Width"]
    #[inline] pub fn width(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Transfer Width"]
    #[inline] pub fn test_width(&self) -> bool {
        self.width() != 0
    }

    #[doc="Transfer Width"]
    #[inline] pub fn set_width<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Transmit Data Mask"]
    #[inline] pub fn txmsk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Transmit Data Mask"]
    #[inline] pub fn test_txmsk(&self) -> bool {
        self.txmsk() != 0
    }

    #[doc="Transmit Data Mask"]
    #[inline] pub fn set_txmsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Receive Data Mask"]
    #[inline] pub fn rxmsk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Receive Data Mask"]
    #[inline] pub fn test_rxmsk(&self) -> bool {
        self.rxmsk() != 0
    }

    #[doc="Receive Data Mask"]
    #[inline] pub fn set_rxmsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Continuing Command"]
    #[inline] pub fn contc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Continuing Command"]
    #[inline] pub fn test_contc(&self) -> bool {
        self.contc() != 0
    }

    #[doc="Continuing Command"]
    #[inline] pub fn set_contc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Continuous Transfer"]
    #[inline] pub fn cont(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Continuous Transfer"]
    #[inline] pub fn test_cont(&self) -> bool {
        self.cont() != 0
    }

    #[doc="Continuous Transfer"]
    #[inline] pub fn set_cont<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Byte Swap"]
    #[inline] pub fn bysw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Byte Swap"]
    #[inline] pub fn test_bysw(&self) -> bool {
        self.bysw() != 0
    }

    #[doc="Byte Swap"]
    #[inline] pub fn set_bysw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="LSB First"]
    #[inline] pub fn lsbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="LSB First"]
    #[inline] pub fn test_lsbf(&self) -> bool {
        self.lsbf() != 0
    }

    #[doc="LSB First"]
    #[inline] pub fn set_lsbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Peripheral Chip Select"]
    #[inline] pub fn pcs(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Peripheral Chip Select"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Peripheral Chip Select"]
    #[inline] pub fn set_pcs<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Prescaler Value"]
    #[inline] pub fn prescale(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Prescaler Value"]
    #[inline] pub fn test_prescale(&self) -> bool {
        self.prescale() != 0
    }

    #[doc="Prescaler Value"]
    #[inline] pub fn set_prescale<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Clock Phase"]
    #[inline] pub fn cpha(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Clock Phase"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha() != 0
    }

    #[doc="Clock Phase"]
    #[inline] pub fn set_cpha<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn cpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn set_cpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Tcr {
    #[inline]
    fn from(other: u32) -> Self {
         Tcr(other)
    }
}

impl ::core::fmt::Display for Tcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.framesz() != 0 { try!(write!(f, " framesz=0x{:x}", self.framesz()))}
        if self.width() != 0 { try!(write!(f, " width=0x{:x}", self.width()))}
        if self.txmsk() != 0 { try!(write!(f, " txmsk"))}
        if self.rxmsk() != 0 { try!(write!(f, " rxmsk"))}
        if self.contc() != 0 { try!(write!(f, " contc"))}
        if self.cont() != 0 { try!(write!(f, " cont"))}
        if self.bysw() != 0 { try!(write!(f, " bysw"))}
        if self.lsbf() != 0 { try!(write!(f, " lsbf"))}
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.prescale() != 0 { try!(write!(f, " prescale=0x{:x}", self.prescale()))}
        if self.cpha() != 0 { try!(write!(f, " cpha"))}
        if self.cpol() != 0 { try!(write!(f, " cpol"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdr(pub u32);
impl Tdr {
    #[doc="Transmit Data"]
    #[inline] pub fn data(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Transmit Data"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Transmit Data"]
    #[inline] pub fn set_data<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdr {
    #[inline]
    fn from(other: u32) -> Self {
         Tdr(other)
    }
}

impl ::core::fmt::Display for Tdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rsr(pub u32);
impl Rsr {
    #[doc="Start Of Frame"]
    #[inline] pub fn sof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Start Of Frame"]
    #[inline] pub fn test_sof(&self) -> bool {
        self.sof() != 0
    }

    #[doc="Start Of Frame"]
    #[inline] pub fn set_sof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RX FIFO Empty"]
    #[inline] pub fn rxempty(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="RX FIFO Empty"]
    #[inline] pub fn test_rxempty(&self) -> bool {
        self.rxempty() != 0
    }

    #[doc="RX FIFO Empty"]
    #[inline] pub fn set_rxempty<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Rsr {
    #[inline]
    fn from(other: u32) -> Self {
         Rsr(other)
    }
}

impl ::core::fmt::Display for Rsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sof() != 0 { try!(write!(f, " sof"))}
        if self.rxempty() != 0 { try!(write!(f, " rxempty"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdr(pub u32);
impl Rdr {
    #[doc="Receive Data"]
    #[inline] pub fn data(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Receive Data"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Receive Data"]
    #[inline] pub fn set_data<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdr {
    #[inline]
    fn from(other: u32) -> Self {
         Rdr(other)
    }
}

impl ::core::fmt::Display for Rdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}


