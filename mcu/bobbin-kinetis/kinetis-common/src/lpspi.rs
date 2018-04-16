
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPSPI Peripheral"]
pub struct LpspiPeriph(pub usize); 

impl LpspiPeriph {
    #[doc="Get the VERID Register."]
    #[inline] pub fn verid_reg(&self) -> ::bobbin_mcu::register::Register<Verid> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Verid, 0x0)
    }

    #[doc="Get the *mut pointer for the VERID register."]
    #[inline] pub fn verid_mut(&self) -> *mut Verid { 
        self.verid_reg().ptr()
    }

    #[doc="Get the *const pointer for the VERID register."]
    #[inline] pub fn verid_ptr(&self) -> *const Verid { 
        self.verid_reg().ptr()
    }

    #[doc="Read the VERID register."]
    #[inline] pub fn verid(&self) -> Verid { 
        self.verid_reg().read()
    }

    #[doc="Get the PARAM Register."]
    #[inline] pub fn param_reg(&self) -> ::bobbin_mcu::register::Register<Param> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Param, 0x4)
    }

    #[doc="Get the *mut pointer for the PARAM register."]
    #[inline] pub fn param_mut(&self) -> *mut Param { 
        self.param_reg().ptr()
    }

    #[doc="Get the *const pointer for the PARAM register."]
    #[inline] pub fn param_ptr(&self) -> *const Param { 
        self.param_reg().ptr()
    }

    #[doc="Read the PARAM register."]
    #[inline] pub fn param(&self) -> Param { 
        self.param_reg().read()
    }

    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x10)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x14)
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

    #[doc="Write the SR register."]
    #[inline] pub fn write_sr(&self, value: Sr) -> &Self { 
        self.sr_reg().write(value);
        self
    }

    #[doc="Set the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().set(f);
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().with(f);
        self
    }

    #[doc="Get the IER Register."]
    #[inline] pub fn ier_reg(&self) -> ::bobbin_mcu::register::Register<Ier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ier, 0x18)
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Read the IER register."]
    #[inline] pub fn ier(&self) -> Ier { 
        self.ier_reg().read()
    }

    #[doc="Write the IER register."]
    #[inline] pub fn write_ier(&self, value: Ier) -> &Self { 
        self.ier_reg().write(value);
        self
    }

    #[doc="Set the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        self.ier_reg().set(f);
        self
    }

    #[doc="Modify the IER register."]
    #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        self.ier_reg().with(f);
        self
    }

    #[doc="Get the DER Register."]
    #[inline] pub fn der_reg(&self) -> ::bobbin_mcu::register::Register<Der> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Der, 0x1c)
    }

    #[doc="Get the *mut pointer for the DER register."]
    #[inline] pub fn der_mut(&self) -> *mut Der { 
        self.der_reg().ptr()
    }

    #[doc="Get the *const pointer for the DER register."]
    #[inline] pub fn der_ptr(&self) -> *const Der { 
        self.der_reg().ptr()
    }

    #[doc="Read the DER register."]
    #[inline] pub fn der(&self) -> Der { 
        self.der_reg().read()
    }

    #[doc="Write the DER register."]
    #[inline] pub fn write_der(&self, value: Der) -> &Self { 
        self.der_reg().write(value);
        self
    }

    #[doc="Set the DER register."]
    #[inline] pub fn set_der<F: FnOnce(Der) -> Der>(&self, f: F) -> &Self {
        self.der_reg().set(f);
        self
    }

    #[doc="Modify the DER register."]
    #[inline] pub fn with_der<F: FnOnce(Der) -> Der>(&self, f: F) -> &Self {
        self.der_reg().with(f);
        self
    }

    #[doc="Get the CFGR0 Register."]
    #[inline] pub fn cfgr0_reg(&self) -> ::bobbin_mcu::register::Register<Cfgr0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfgr0, 0x20)
    }

    #[doc="Get the *mut pointer for the CFGR0 register."]
    #[inline] pub fn cfgr0_mut(&self) -> *mut Cfgr0 { 
        self.cfgr0_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFGR0 register."]
    #[inline] pub fn cfgr0_ptr(&self) -> *const Cfgr0 { 
        self.cfgr0_reg().ptr()
    }

    #[doc="Read the CFGR0 register."]
    #[inline] pub fn cfgr0(&self) -> Cfgr0 { 
        self.cfgr0_reg().read()
    }

    #[doc="Write the CFGR0 register."]
    #[inline] pub fn write_cfgr0(&self, value: Cfgr0) -> &Self { 
        self.cfgr0_reg().write(value);
        self
    }

    #[doc="Set the CFGR0 register."]
    #[inline] pub fn set_cfgr0<F: FnOnce(Cfgr0) -> Cfgr0>(&self, f: F) -> &Self {
        self.cfgr0_reg().set(f);
        self
    }

    #[doc="Modify the CFGR0 register."]
    #[inline] pub fn with_cfgr0<F: FnOnce(Cfgr0) -> Cfgr0>(&self, f: F) -> &Self {
        self.cfgr0_reg().with(f);
        self
    }

    #[doc="Get the CFGR1 Register."]
    #[inline] pub fn cfgr1_reg(&self) -> ::bobbin_mcu::register::Register<Cfgr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfgr1, 0x24)
    }

    #[doc="Get the *mut pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_mut(&self) -> *mut Cfgr1 { 
        self.cfgr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_ptr(&self) -> *const Cfgr1 { 
        self.cfgr1_reg().ptr()
    }

    #[doc="Read the CFGR1 register."]
    #[inline] pub fn cfgr1(&self) -> Cfgr1 { 
        self.cfgr1_reg().read()
    }

    #[doc="Write the CFGR1 register."]
    #[inline] pub fn write_cfgr1(&self, value: Cfgr1) -> &Self { 
        self.cfgr1_reg().write(value);
        self
    }

    #[doc="Set the CFGR1 register."]
    #[inline] pub fn set_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        self.cfgr1_reg().set(f);
        self
    }

    #[doc="Modify the CFGR1 register."]
    #[inline] pub fn with_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        self.cfgr1_reg().with(f);
        self
    }

    #[doc="Get the DMR0 Register."]
    #[inline] pub fn dmr0_reg(&self) -> ::bobbin_mcu::register::Register<Dmr0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dmr0, 0x30)
    }

    #[doc="Get the *mut pointer for the DMR0 register."]
    #[inline] pub fn dmr0_mut(&self) -> *mut Dmr0 { 
        self.dmr0_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMR0 register."]
    #[inline] pub fn dmr0_ptr(&self) -> *const Dmr0 { 
        self.dmr0_reg().ptr()
    }

    #[doc="Read the DMR0 register."]
    #[inline] pub fn dmr0(&self) -> Dmr0 { 
        self.dmr0_reg().read()
    }

    #[doc="Write the DMR0 register."]
    #[inline] pub fn write_dmr0(&self, value: Dmr0) -> &Self { 
        self.dmr0_reg().write(value);
        self
    }

    #[doc="Set the DMR0 register."]
    #[inline] pub fn set_dmr0<F: FnOnce(Dmr0) -> Dmr0>(&self, f: F) -> &Self {
        self.dmr0_reg().set(f);
        self
    }

    #[doc="Modify the DMR0 register."]
    #[inline] pub fn with_dmr0<F: FnOnce(Dmr0) -> Dmr0>(&self, f: F) -> &Self {
        self.dmr0_reg().with(f);
        self
    }

    #[doc="Get the DMR1 Register."]
    #[inline] pub fn dmr1_reg(&self) -> ::bobbin_mcu::register::Register<Dmr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dmr1, 0x34)
    }

    #[doc="Get the *mut pointer for the DMR1 register."]
    #[inline] pub fn dmr1_mut(&self) -> *mut Dmr1 { 
        self.dmr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMR1 register."]
    #[inline] pub fn dmr1_ptr(&self) -> *const Dmr1 { 
        self.dmr1_reg().ptr()
    }

    #[doc="Read the DMR1 register."]
    #[inline] pub fn dmr1(&self) -> Dmr1 { 
        self.dmr1_reg().read()
    }

    #[doc="Write the DMR1 register."]
    #[inline] pub fn write_dmr1(&self, value: Dmr1) -> &Self { 
        self.dmr1_reg().write(value);
        self
    }

    #[doc="Set the DMR1 register."]
    #[inline] pub fn set_dmr1<F: FnOnce(Dmr1) -> Dmr1>(&self, f: F) -> &Self {
        self.dmr1_reg().set(f);
        self
    }

    #[doc="Modify the DMR1 register."]
    #[inline] pub fn with_dmr1<F: FnOnce(Dmr1) -> Dmr1>(&self, f: F) -> &Self {
        self.dmr1_reg().with(f);
        self
    }

    #[doc="Get the CCR Register."]
    #[inline] pub fn ccr_reg(&self) -> ::bobbin_mcu::register::Register<Ccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccr, 0x40)
    }

    #[doc="Get the *mut pointer for the CCR register."]
    #[inline] pub fn ccr_mut(&self) -> *mut Ccr { 
        self.ccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCR register."]
    #[inline] pub fn ccr_ptr(&self) -> *const Ccr { 
        self.ccr_reg().ptr()
    }

    #[doc="Read the CCR register."]
    #[inline] pub fn ccr(&self) -> Ccr { 
        self.ccr_reg().read()
    }

    #[doc="Write the CCR register."]
    #[inline] pub fn write_ccr(&self, value: Ccr) -> &Self { 
        self.ccr_reg().write(value);
        self
    }

    #[doc="Set the CCR register."]
    #[inline] pub fn set_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        self.ccr_reg().set(f);
        self
    }

    #[doc="Modify the CCR register."]
    #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        self.ccr_reg().with(f);
        self
    }

    #[doc="Get the FCR Register."]
    #[inline] pub fn fcr_reg(&self) -> ::bobbin_mcu::register::Register<Fcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fcr, 0x58)
    }

    #[doc="Get the *mut pointer for the FCR register."]
    #[inline] pub fn fcr_mut(&self) -> *mut Fcr { 
        self.fcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the FCR register."]
    #[inline] pub fn fcr_ptr(&self) -> *const Fcr { 
        self.fcr_reg().ptr()
    }

    #[doc="Read the FCR register."]
    #[inline] pub fn fcr(&self) -> Fcr { 
        self.fcr_reg().read()
    }

    #[doc="Write the FCR register."]
    #[inline] pub fn write_fcr(&self, value: Fcr) -> &Self { 
        self.fcr_reg().write(value);
        self
    }

    #[doc="Set the FCR register."]
    #[inline] pub fn set_fcr<F: FnOnce(Fcr) -> Fcr>(&self, f: F) -> &Self {
        self.fcr_reg().set(f);
        self
    }

    #[doc="Modify the FCR register."]
    #[inline] pub fn with_fcr<F: FnOnce(Fcr) -> Fcr>(&self, f: F) -> &Self {
        self.fcr_reg().with(f);
        self
    }

    #[doc="Get the FSR Register."]
    #[inline] pub fn fsr_reg(&self) -> ::bobbin_mcu::register::Register<Fsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fsr, 0x5c)
    }

    #[doc="Get the *mut pointer for the FSR register."]
    #[inline] pub fn fsr_mut(&self) -> *mut Fsr { 
        self.fsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the FSR register."]
    #[inline] pub fn fsr_ptr(&self) -> *const Fsr { 
        self.fsr_reg().ptr()
    }

    #[doc="Read the FSR register."]
    #[inline] pub fn fsr(&self) -> Fsr { 
        self.fsr_reg().read()
    }

    #[doc="Get the TCR Register."]
    #[inline] pub fn tcr_reg(&self) -> ::bobbin_mcu::register::Register<Tcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tcr, 0x60)
    }

    #[doc="Get the *mut pointer for the TCR register."]
    #[inline] pub fn tcr_mut(&self) -> *mut Tcr { 
        self.tcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TCR register."]
    #[inline] pub fn tcr_ptr(&self) -> *const Tcr { 
        self.tcr_reg().ptr()
    }

    #[doc="Read the TCR register."]
    #[inline] pub fn tcr(&self) -> Tcr { 
        self.tcr_reg().read()
    }

    #[doc="Write the TCR register."]
    #[inline] pub fn write_tcr(&self, value: Tcr) -> &Self { 
        self.tcr_reg().write(value);
        self
    }

    #[doc="Set the TCR register."]
    #[inline] pub fn set_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        self.tcr_reg().set(f);
        self
    }

    #[doc="Modify the TCR register."]
    #[inline] pub fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        self.tcr_reg().with(f);
        self
    }

    #[doc="Get the TDR Register."]
    #[inline] pub fn tdr_reg(&self) -> ::bobbin_mcu::register::Register<Tdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tdr, 0x64)
    }

    #[doc="Get the *mut pointer for the TDR register."]
    #[inline] pub fn tdr_mut(&self) -> *mut Tdr { 
        self.tdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TDR register."]
    #[inline] pub fn tdr_ptr(&self) -> *const Tdr { 
        self.tdr_reg().ptr()
    }

    #[doc="Write the TDR register."]
    #[inline] pub fn write_tdr(&self, value: Tdr) -> &Self { 
        self.tdr_reg().write(value);
        self
    }

    #[doc="Set the TDR register."]
    #[inline] pub fn set_tdr<F: FnOnce(Tdr) -> Tdr>(&self, f: F) -> &Self {
        self.tdr_reg().set(f);
        self
    }

    #[doc="Get the RSR Register."]
    #[inline] pub fn rsr_reg(&self) -> ::bobbin_mcu::register::Register<Rsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rsr, 0x70)
    }

    #[doc="Get the *mut pointer for the RSR register."]
    #[inline] pub fn rsr_mut(&self) -> *mut Rsr { 
        self.rsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RSR register."]
    #[inline] pub fn rsr_ptr(&self) -> *const Rsr { 
        self.rsr_reg().ptr()
    }

    #[doc="Read the RSR register."]
    #[inline] pub fn rsr(&self) -> Rsr { 
        self.rsr_reg().read()
    }

    #[doc="Get the RDR Register."]
    #[inline] pub fn rdr_reg(&self) -> ::bobbin_mcu::register::Register<Rdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rdr, 0x74)
    }

    #[doc="Get the *mut pointer for the RDR register."]
    #[inline] pub fn rdr_mut(&self) -> *mut Rdr { 
        self.rdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RDR register."]
    #[inline] pub fn rdr_ptr(&self) -> *const Rdr { 
        self.rdr_reg().ptr()
    }

    #[doc="Read the RDR register."]
    #[inline] pub fn rdr(&self) -> Rdr { 
        self.rdr_reg().read()
    }

}

#[doc="Version ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc="Module Identification Number"]
    #[inline] pub fn feature(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if FEATURE != 0"]
    #[inline] pub fn test_feature(&self) -> bool {
        self.feature() != 0
    }

    #[doc="Sets the FEATURE field."]
    #[inline] pub fn set_feature<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Minor Version Number"]
    #[inline] pub fn minor(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if MINOR != 0"]
    #[inline] pub fn test_minor(&self) -> bool {
        self.minor() != 0
    }

    #[doc="Sets the MINOR field."]
    #[inline] pub fn set_minor<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Major Version Number"]
    #[inline] pub fn major(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if MAJOR != 0"]
    #[inline] pub fn test_major(&self) -> bool {
        self.major() != 0
    }

    #[doc="Sets the MAJOR field."]
    #[inline] pub fn set_major<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
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
    #[inline] pub fn txfifo(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TXFIFO != 0"]
    #[inline] pub fn test_txfifo(&self) -> bool {
        self.txfifo() != 0
    }

    #[doc="Sets the TXFIFO field."]
    #[inline] pub fn set_txfifo<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Size"]
    #[inline] pub fn rxfifo(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if RXFIFO != 0"]
    #[inline] pub fn test_rxfifo(&self) -> bool {
        self.rxfifo() != 0
    }

    #[doc="Sets the RXFIFO field."]
    #[inline] pub fn set_rxfifo<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
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
    #[inline] pub fn men(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MEN != 0"]
    #[inline] pub fn test_men(&self) -> bool {
        self.men() != 0
    }

    #[doc="Sets the MEN field."]
    #[inline] pub fn set_men<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Software Reset"]
    #[inline] pub fn rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RST != 0"]
    #[inline] pub fn test_rst(&self) -> bool {
        self.rst() != 0
    }

    #[doc="Sets the RST field."]
    #[inline] pub fn set_rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Doze mode enable"]
    #[inline] pub fn dozen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DOZEN != 0"]
    #[inline] pub fn test_dozen(&self) -> bool {
        self.dozen() != 0
    }

    #[doc="Sets the DOZEN field."]
    #[inline] pub fn set_dozen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Debug Enable"]
    #[inline] pub fn dbgen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DBGEN != 0"]
    #[inline] pub fn test_dbgen(&self) -> bool {
        self.dbgen() != 0
    }

    #[doc="Sets the DBGEN field."]
    #[inline] pub fn set_dbgen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Reset Transmit FIFO"]
    #[inline] pub fn rtf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RTF != 0"]
    #[inline] pub fn test_rtf(&self) -> bool {
        self.rtf() != 0
    }

    #[doc="Sets the RTF field."]
    #[inline] pub fn set_rtf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Reset Receive FIFO"]
    #[inline] pub fn rrf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RRF != 0"]
    #[inline] pub fn test_rrf(&self) -> bool {
        self.rrf() != 0
    }

    #[doc="Sets the RRF field."]
    #[inline] pub fn set_rrf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn tdf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TDF != 0"]
    #[inline] pub fn test_tdf(&self) -> bool {
        self.tdf() != 0
    }

    #[doc="Sets the TDF field."]
    #[inline] pub fn set_tdf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data Flag"]
    #[inline] pub fn rdf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RDF != 0"]
    #[inline] pub fn test_rdf(&self) -> bool {
        self.rdf() != 0
    }

    #[doc="Sets the RDF field."]
    #[inline] pub fn set_rdf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Word Complete Flag"]
    #[inline] pub fn wcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WCF != 0"]
    #[inline] pub fn test_wcf(&self) -> bool {
        self.wcf() != 0
    }

    #[doc="Sets the WCF field."]
    #[inline] pub fn set_wcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame Complete Flag"]
    #[inline] pub fn fcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FCF != 0"]
    #[inline] pub fn test_fcf(&self) -> bool {
        self.fcf() != 0
    }

    #[doc="Sets the FCF field."]
    #[inline] pub fn set_fcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transfer Complete Flag"]
    #[inline] pub fn tcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TCF != 0"]
    #[inline] pub fn test_tcf(&self) -> bool {
        self.tcf() != 0
    }

    #[doc="Sets the TCF field."]
    #[inline] pub fn set_tcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Transmit Error Flag"]
    #[inline] pub fn tef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TEF != 0"]
    #[inline] pub fn test_tef(&self) -> bool {
        self.tef() != 0
    }

    #[doc="Sets the TEF field."]
    #[inline] pub fn set_tef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Receive Error Flag"]
    #[inline] pub fn _ref(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if REF != 0"]
    #[inline] pub fn test_ref(&self) -> bool {
        self._ref() != 0
    }

    #[doc="Sets the REF field."]
    #[inline] pub fn set_ref<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Match Flag"]
    #[inline] pub fn dmf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DMF != 0"]
    #[inline] pub fn test_dmf(&self) -> bool {
        self.dmf() != 0
    }

    #[doc="Sets the DMF field."]
    #[inline] pub fn set_dmf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Module Busy Flag"]
    #[inline] pub fn mbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if MBF != 0"]
    #[inline] pub fn test_mbf(&self) -> bool {
        self.mbf() != 0
    }

    #[doc="Sets the MBF field."]
    #[inline] pub fn set_mbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn tdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TDIE != 0"]
    #[inline] pub fn test_tdie(&self) -> bool {
        self.tdie() != 0
    }

    #[doc="Sets the TDIE field."]
    #[inline] pub fn set_tdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data Interrupt Enable"]
    #[inline] pub fn rdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RDIE != 0"]
    #[inline] pub fn test_rdie(&self) -> bool {
        self.rdie() != 0
    }

    #[doc="Sets the RDIE field."]
    #[inline] pub fn set_rdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Word Complete Interrupt Enable"]
    #[inline] pub fn wcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WCIE != 0"]
    #[inline] pub fn test_wcie(&self) -> bool {
        self.wcie() != 0
    }

    #[doc="Sets the WCIE field."]
    #[inline] pub fn set_wcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame Complete Interrupt Enable"]
    #[inline] pub fn fcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FCIE != 0"]
    #[inline] pub fn test_fcie(&self) -> bool {
        self.fcie() != 0
    }

    #[doc="Sets the FCIE field."]
    #[inline] pub fn set_fcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transfer Complete Interrupt Enable"]
    #[inline] pub fn tcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
    #[inline] pub fn set_tcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Transmit Error Interrupt Enable"]
    #[inline] pub fn teie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TEIE != 0"]
    #[inline] pub fn test_teie(&self) -> bool {
        self.teie() != 0
    }

    #[doc="Sets the TEIE field."]
    #[inline] pub fn set_teie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Receive Error Interrupt Enable"]
    #[inline] pub fn reie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if REIE != 0"]
    #[inline] pub fn test_reie(&self) -> bool {
        self.reie() != 0
    }

    #[doc="Sets the REIE field."]
    #[inline] pub fn set_reie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Match Interrupt Enable"]
    #[inline] pub fn dmie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DMIE != 0"]
    #[inline] pub fn test_dmie(&self) -> bool {
        self.dmie() != 0
    }

    #[doc="Sets the DMIE field."]
    #[inline] pub fn set_dmie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn tdde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TDDE != 0"]
    #[inline] pub fn test_tdde(&self) -> bool {
        self.tdde() != 0
    }

    #[doc="Sets the TDDE field."]
    #[inline] pub fn set_tdde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data DMA Enable"]
    #[inline] pub fn rdde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RDDE != 0"]
    #[inline] pub fn test_rdde(&self) -> bool {
        self.rdde() != 0
    }

    #[doc="Sets the RDDE field."]
    #[inline] pub fn set_rdde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn hren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HREN != 0"]
    #[inline] pub fn test_hren(&self) -> bool {
        self.hren() != 0
    }

    #[doc="Sets the HREN field."]
    #[inline] pub fn set_hren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Host Request Polarity"]
    #[inline] pub fn hrpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HRPOL != 0"]
    #[inline] pub fn test_hrpol(&self) -> bool {
        self.hrpol() != 0
    }

    #[doc="Sets the HRPOL field."]
    #[inline] pub fn set_hrpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Host Request Select"]
    #[inline] pub fn hrsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HRSEL != 0"]
    #[inline] pub fn test_hrsel(&self) -> bool {
        self.hrsel() != 0
    }

    #[doc="Sets the HRSEL field."]
    #[inline] pub fn set_hrsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Circular FIFO Enable"]
    #[inline] pub fn cirfifo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CIRFIFO != 0"]
    #[inline] pub fn test_cirfifo(&self) -> bool {
        self.cirfifo() != 0
    }

    #[doc="Sets the CIRFIFO field."]
    #[inline] pub fn set_cirfifo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive Data Match Only"]
    #[inline] pub fn rdmo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RDMO != 0"]
    #[inline] pub fn test_rdmo(&self) -> bool {
        self.rdmo() != 0
    }

    #[doc="Sets the RDMO field."]
    #[inline] pub fn set_rdmo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn master(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MASTER != 0"]
    #[inline] pub fn test_master(&self) -> bool {
        self.master() != 0
    }

    #[doc="Sets the MASTER field."]
    #[inline] pub fn set_master<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Sample Point"]
    #[inline] pub fn sample(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SAMPLE != 0"]
    #[inline] pub fn test_sample(&self) -> bool {
        self.sample() != 0
    }

    #[doc="Sets the SAMPLE field."]
    #[inline] pub fn set_sample<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Automatic PCS"]
    #[inline] pub fn autopcs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if AUTOPCS != 0"]
    #[inline] pub fn test_autopcs(&self) -> bool {
        self.autopcs() != 0
    }

    #[doc="Sets the AUTOPCS field."]
    #[inline] pub fn set_autopcs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="No Stall"]
    #[inline] pub fn nostall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if NOSTALL != 0"]
    #[inline] pub fn test_nostall(&self) -> bool {
        self.nostall() != 0
    }

    #[doc="Sets the NOSTALL field."]
    #[inline] pub fn set_nostall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Peripheral Chip Select Polarity"]
    #[inline] pub fn pcspol(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if PCSPOL != 0"]
    #[inline] pub fn test_pcspol(&self) -> bool {
        self.pcspol() != 0
    }

    #[doc="Sets the PCSPOL field."]
    #[inline] pub fn set_pcspol<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Match Configuration"]
    #[inline] pub fn matcfg(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if MATCFG != 0"]
    #[inline] pub fn test_matcfg(&self) -> bool {
        self.matcfg() != 0
    }

    #[doc="Sets the MATCFG field."]
    #[inline] pub fn set_matcfg<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pin Configuration"]
    #[inline] pub fn pincfg(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if PINCFG != 0"]
    #[inline] pub fn test_pincfg(&self) -> bool {
        self.pincfg() != 0
    }

    #[doc="Sets the PINCFG field."]
    #[inline] pub fn set_pincfg<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Output Config"]
    #[inline] pub fn outcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if OUTCFG != 0"]
    #[inline] pub fn test_outcfg(&self) -> bool {
        self.outcfg() != 0
    }

    #[doc="Sets the OUTCFG field."]
    #[inline] pub fn set_outcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Peripheral Chip Select Configuration"]
    #[inline] pub fn pcscfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PCSCFG != 0"]
    #[inline] pub fn test_pcscfg(&self) -> bool {
        self.pcscfg() != 0
    }

    #[doc="Sets the PCSCFG field."]
    #[inline] pub fn set_pcscfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn match0(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if MATCH0 != 0"]
    #[inline] pub fn test_match0(&self) -> bool {
        self.match0() != 0
    }

    #[doc="Sets the MATCH0 field."]
    #[inline] pub fn set_match0<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
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
    #[inline] pub fn match1(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if MATCH1 != 0"]
    #[inline] pub fn test_match1(&self) -> bool {
        self.match1() != 0
    }

    #[doc="Sets the MATCH1 field."]
    #[inline] pub fn set_match1<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
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
    #[inline] pub fn sckdiv(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCKDIV != 0"]
    #[inline] pub fn test_sckdiv(&self) -> bool {
        self.sckdiv() != 0
    }

    #[doc="Sets the SCKDIV field."]
    #[inline] pub fn set_sckdiv<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Delay Between Transfers"]
    #[inline] pub fn dbt(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DBT != 0"]
    #[inline] pub fn test_dbt(&self) -> bool {
        self.dbt() != 0
    }

    #[doc="Sets the DBT field."]
    #[inline] pub fn set_dbt<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PCS to SCK Delay"]
    #[inline] pub fn pcssck(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if PCSSCK != 0"]
    #[inline] pub fn test_pcssck(&self) -> bool {
        self.pcssck() != 0
    }

    #[doc="Sets the PCSSCK field."]
    #[inline] pub fn set_pcssck<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="SCK to PCS Delay"]
    #[inline] pub fn sckpcs(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if SCKPCS != 0"]
    #[inline] pub fn test_sckpcs(&self) -> bool {
        self.sckpcs() != 0
    }

    #[doc="Sets the SCKPCS field."]
    #[inline] pub fn set_sckpcs<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
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
    #[inline] pub fn txwater(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if TXWATER != 0"]
    #[inline] pub fn test_txwater(&self) -> bool {
        self.txwater() != 0
    }

    #[doc="Sets the TXWATER field."]
    #[inline] pub fn set_txwater<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Watermark"]
    #[inline] pub fn rxwater(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if RXWATER != 0"]
    #[inline] pub fn test_rxwater(&self) -> bool {
        self.rxwater() != 0
    }

    #[doc="Sets the RXWATER field."]
    #[inline] pub fn set_rxwater<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
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
    #[inline] pub fn txcount(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if TXCOUNT != 0"]
    #[inline] pub fn test_txcount(&self) -> bool {
        self.txcount() != 0
    }

    #[doc="Sets the TXCOUNT field."]
    #[inline] pub fn set_txcount<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Count"]
    #[inline] pub fn rxcount(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if RXCOUNT != 0"]
    #[inline] pub fn test_rxcount(&self) -> bool {
        self.rxcount() != 0
    }

    #[doc="Sets the RXCOUNT field."]
    #[inline] pub fn set_rxcount<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
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
    #[inline] pub fn framesz(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if FRAMESZ != 0"]
    #[inline] pub fn test_framesz(&self) -> bool {
        self.framesz() != 0
    }

    #[doc="Sets the FRAMESZ field."]
    #[inline] pub fn set_framesz<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Width"]
    #[inline] pub fn width(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if WIDTH != 0"]
    #[inline] pub fn test_width(&self) -> bool {
        self.width() != 0
    }

    #[doc="Sets the WIDTH field."]
    #[inline] pub fn set_width<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Transmit Data Mask"]
    #[inline] pub fn txmsk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TXMSK != 0"]
    #[inline] pub fn test_txmsk(&self) -> bool {
        self.txmsk() != 0
    }

    #[doc="Sets the TXMSK field."]
    #[inline] pub fn set_txmsk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Receive Data Mask"]
    #[inline] pub fn rxmsk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RXMSK != 0"]
    #[inline] pub fn test_rxmsk(&self) -> bool {
        self.rxmsk() != 0
    }

    #[doc="Sets the RXMSK field."]
    #[inline] pub fn set_rxmsk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Continuing Command"]
    #[inline] pub fn contc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CONTC != 0"]
    #[inline] pub fn test_contc(&self) -> bool {
        self.contc() != 0
    }

    #[doc="Sets the CONTC field."]
    #[inline] pub fn set_contc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Continuous Transfer"]
    #[inline] pub fn cont(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if CONT != 0"]
    #[inline] pub fn test_cont(&self) -> bool {
        self.cont() != 0
    }

    #[doc="Sets the CONT field."]
    #[inline] pub fn set_cont<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Byte Swap"]
    #[inline] pub fn bysw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if BYSW != 0"]
    #[inline] pub fn test_bysw(&self) -> bool {
        self.bysw() != 0
    }

    #[doc="Sets the BYSW field."]
    #[inline] pub fn set_bysw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="LSB First"]
    #[inline] pub fn lsbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if LSBF != 0"]
    #[inline] pub fn test_lsbf(&self) -> bool {
        self.lsbf() != 0
    }

    #[doc="Sets the LSBF field."]
    #[inline] pub fn set_lsbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Peripheral Chip Select"]
    #[inline] pub fn pcs(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Prescaler Value"]
    #[inline] pub fn prescale(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PRESCALE != 0"]
    #[inline] pub fn test_prescale(&self) -> bool {
        self.prescale() != 0
    }

    #[doc="Sets the PRESCALE field."]
    #[inline] pub fn set_prescale<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Clock Phase"]
    #[inline] pub fn cpha(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CPHA != 0"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha() != 0
    }

    #[doc="Sets the CPHA field."]
    #[inline] pub fn set_cpha<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn cpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CPOL != 0"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Sets the CPOL field."]
    #[inline] pub fn set_cpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
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
    #[inline] pub fn sof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SOF != 0"]
    #[inline] pub fn test_sof(&self) -> bool {
        self.sof() != 0
    }

    #[doc="Sets the SOF field."]
    #[inline] pub fn set_sof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RX FIFO Empty"]
    #[inline] pub fn rxempty(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RXEMPTY != 0"]
    #[inline] pub fn test_rxempty(&self) -> bool {
        self.rxempty() != 0
    }

    #[doc="Sets the RXEMPTY field."]
    #[inline] pub fn set_rxempty<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
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

