::bobbin_mcu::periph!( CRS, Crs, CRS_PERIPH, CrsPeriph, CRS_OWNED, CRS_REF_COUNT, 0x40006000, 0x00, 0x2a);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="CRS Peripheral"]
pub struct CrsPeriph(pub usize); 

impl CrsPeriph {
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

    #[doc="Get the CFGR Register."]
    #[inline] pub fn cfgr_reg(&self) -> ::bobbin_mcu::register::Register<Cfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfgr, 0x4)
    }

    #[doc="Get the *mut pointer for the CFGR register."]
    #[inline] pub fn cfgr_mut(&self) -> *mut Cfgr { 
        self.cfgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFGR register."]
    #[inline] pub fn cfgr_ptr(&self) -> *const Cfgr { 
        self.cfgr_reg().ptr()
    }

    #[doc="Read the CFGR register."]
    #[inline] pub fn cfgr(&self) -> Cfgr { 
        self.cfgr_reg().read()
    }

    #[doc="Write the CFGR register."]
    #[inline] pub fn write_cfgr(&self, value: Cfgr) -> &Self { 
        self.cfgr_reg().write(value);
        self
    }

    #[doc="Set the CFGR register."]
    #[inline] pub fn set_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        self.cfgr_reg().set(f);
        self
    }

    #[doc="Modify the CFGR register."]
    #[inline] pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        self.cfgr_reg().with(f);
        self
    }

    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> ::bobbin_mcu::register::Register<Isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Isr, 0x8)
    }

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ISR register."]
    #[inline] pub fn isr_ptr(&self) -> *const Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Read the ISR register."]
    #[inline] pub fn isr(&self) -> Isr { 
        self.isr_reg().read()
    }

    #[doc="Get the ICR Register."]
    #[inline] pub fn icr_reg(&self) -> ::bobbin_mcu::register::Register<Icr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Icr, 0xc)
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        self.icr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
        self.icr_reg().ptr()
    }

    #[doc="Read the ICR register."]
    #[inline] pub fn icr(&self) -> Icr { 
        self.icr_reg().read()
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn write_icr(&self, value: Icr) -> &Self { 
        self.icr_reg().write(value);
        self
    }

    #[doc="Set the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        self.icr_reg().set(f);
        self
    }

    #[doc="Modify the ICR register."]
    #[inline] pub fn with_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        self.icr_reg().with(f);
        self
    }

}

#[doc="CRS control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="SYNC event OK interrupt enable"]
    #[inline] pub fn syncokie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SYNCOKIE != 0"]
    #[inline] pub fn test_syncokie(&self) -> bool {
        self.syncokie() != 0
    }

    #[doc="Sets the SYNCOKIE field."]
    #[inline] pub fn set_syncokie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SYNC warning interrupt enable"]
    #[inline] pub fn syncwarnie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SYNCWARNIE != 0"]
    #[inline] pub fn test_syncwarnie(&self) -> bool {
        self.syncwarnie() != 0
    }

    #[doc="Sets the SYNCWARNIE field."]
    #[inline] pub fn set_syncwarnie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization or trimming error interrupt enable"]
    #[inline] pub fn errie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Expected SYNC interrupt enable"]
    #[inline] pub fn esyncie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ESYNCIE != 0"]
    #[inline] pub fn test_esyncie(&self) -> bool {
        self.esyncie() != 0
    }

    #[doc="Sets the ESYNCIE field."]
    #[inline] pub fn set_esyncie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Frequency error counter enable"]
    #[inline] pub fn cen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CEN != 0"]
    #[inline] pub fn test_cen(&self) -> bool {
        self.cen() != 0
    }

    #[doc="Sets the CEN field."]
    #[inline] pub fn set_cen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Automatic trimming enable"]
    #[inline] pub fn autotrimen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if AUTOTRIMEN != 0"]
    #[inline] pub fn test_autotrimen(&self) -> bool {
        self.autotrimen() != 0
    }

    #[doc="Sets the AUTOTRIMEN field."]
    #[inline] pub fn set_autotrimen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Automatic trimming enable"]
    #[inline] pub fn swsync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SWSYNC != 0"]
    #[inline] pub fn test_swsync(&self) -> bool {
        self.swsync() != 0
    }

    #[doc="Sets the SWSYNC field."]
    #[inline] pub fn set_swsync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="HSI48 oscillator smooth trimming"]
    #[inline] pub fn trim(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if TRIM != 0"]
    #[inline] pub fn test_trim(&self) -> bool {
        self.trim() != 0
    }

    #[doc="Sets the TRIM field."]
    #[inline] pub fn set_trim<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
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
        if self.syncokie() != 0 { try!(write!(f, " syncokie"))}
        if self.syncwarnie() != 0 { try!(write!(f, " syncwarnie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.esyncie() != 0 { try!(write!(f, " esyncie"))}
        if self.cen() != 0 { try!(write!(f, " cen"))}
        if self.autotrimen() != 0 { try!(write!(f, " autotrimen"))}
        if self.swsync() != 0 { try!(write!(f, " swsync"))}
        if self.trim() != 0 { try!(write!(f, " trim=0x{:x}", self.trim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRS configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc="Counter reload value"]
    #[inline] pub fn reload(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if RELOAD != 0"]
    #[inline] pub fn test_reload(&self) -> bool {
        self.reload() != 0
    }

    #[doc="Sets the RELOAD field."]
    #[inline] pub fn set_reload<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Frequency error limit"]
    #[inline] pub fn felim(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if FELIM != 0"]
    #[inline] pub fn test_felim(&self) -> bool {
        self.felim() != 0
    }

    #[doc="Sets the FELIM field."]
    #[inline] pub fn set_felim<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="SYNCDIV"]
    #[inline] pub fn syncdiv(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if SYNCDIV != 0"]
    #[inline] pub fn test_syncdiv(&self) -> bool {
        self.syncdiv() != 0
    }

    #[doc="Sets the SYNCDIV field."]
    #[inline] pub fn set_syncdiv<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SYNC signal source selection"]
    #[inline] pub fn syncsrc(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if SYNCSRC != 0"]
    #[inline] pub fn test_syncsrc(&self) -> bool {
        self.syncsrc() != 0
    }

    #[doc="Sets the SYNCSRC field."]
    #[inline] pub fn set_syncsrc<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SYNC polarity selection"]
    #[inline] pub fn syncpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if SYNCPOL != 0"]
    #[inline] pub fn test_syncpol(&self) -> bool {
        self.syncpol() != 0
    }

    #[doc="Sets the SYNCPOL field."]
    #[inline] pub fn set_syncpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Cfgr {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr(other)
    }
}

impl ::core::fmt::Display for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.reload() != 0 { try!(write!(f, " reload=0x{:x}", self.reload()))}
        if self.felim() != 0 { try!(write!(f, " felim=0x{:x}", self.felim()))}
        if self.syncdiv() != 0 { try!(write!(f, " syncdiv=0x{:x}", self.syncdiv()))}
        if self.syncsrc() != 0 { try!(write!(f, " syncsrc=0x{:x}", self.syncsrc()))}
        if self.syncpol() != 0 { try!(write!(f, " syncpol"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRS interrupt and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="SYNC event OK flag"]
    #[inline] pub fn syncokf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SYNCOKF != 0"]
    #[inline] pub fn test_syncokf(&self) -> bool {
        self.syncokf() != 0
    }

    #[doc="Sets the SYNCOKF field."]
    #[inline] pub fn set_syncokf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SYNC warning flag"]
    #[inline] pub fn syncwarnf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SYNCWARNF != 0"]
    #[inline] pub fn test_syncwarnf(&self) -> bool {
        self.syncwarnf() != 0
    }

    #[doc="Sets the SYNCWARNF field."]
    #[inline] pub fn set_syncwarnf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error flag"]
    #[inline] pub fn errf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ERRF != 0"]
    #[inline] pub fn test_errf(&self) -> bool {
        self.errf() != 0
    }

    #[doc="Sets the ERRF field."]
    #[inline] pub fn set_errf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Expected SYNC flag"]
    #[inline] pub fn esyncf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ESYNCF != 0"]
    #[inline] pub fn test_esyncf(&self) -> bool {
        self.esyncf() != 0
    }

    #[doc="Sets the ESYNCF field."]
    #[inline] pub fn set_esyncf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="SYNC error"]
    #[inline] pub fn syncerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SYNCERR != 0"]
    #[inline] pub fn test_syncerr(&self) -> bool {
        self.syncerr() != 0
    }

    #[doc="Sets the SYNCERR field."]
    #[inline] pub fn set_syncerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="SYNC missed"]
    #[inline] pub fn syncmiss(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SYNCMISS != 0"]
    #[inline] pub fn test_syncmiss(&self) -> bool {
        self.syncmiss() != 0
    }

    #[doc="Sets the SYNCMISS field."]
    #[inline] pub fn set_syncmiss<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Trimming overflow or underflow"]
    #[inline] pub fn trimovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TRIMOVF != 0"]
    #[inline] pub fn test_trimovf(&self) -> bool {
        self.trimovf() != 0
    }

    #[doc="Sets the TRIMOVF field."]
    #[inline] pub fn set_trimovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Frequency error direction"]
    #[inline] pub fn fedir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FEDIR != 0"]
    #[inline] pub fn test_fedir(&self) -> bool {
        self.fedir() != 0
    }

    #[doc="Sets the FEDIR field."]
    #[inline] pub fn set_fedir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Frequency error capture"]
    #[inline] pub fn fecap(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if FECAP != 0"]
    #[inline] pub fn test_fecap(&self) -> bool {
        self.fecap() != 0
    }

    #[doc="Sets the FECAP field."]
    #[inline] pub fn set_fecap<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Isr(other)
    }
}

impl ::core::fmt::Display for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.syncokf() != 0 { try!(write!(f, " syncokf"))}
        if self.syncwarnf() != 0 { try!(write!(f, " syncwarnf"))}
        if self.errf() != 0 { try!(write!(f, " errf"))}
        if self.esyncf() != 0 { try!(write!(f, " esyncf"))}
        if self.syncerr() != 0 { try!(write!(f, " syncerr"))}
        if self.syncmiss() != 0 { try!(write!(f, " syncmiss"))}
        if self.trimovf() != 0 { try!(write!(f, " trimovf"))}
        if self.fedir() != 0 { try!(write!(f, " fedir"))}
        if self.fecap() != 0 { try!(write!(f, " fecap=0x{:x}", self.fecap()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRS interrupt flag clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="SYNC event OK clear flag"]
    #[inline] pub fn syncokc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SYNCOKC != 0"]
    #[inline] pub fn test_syncokc(&self) -> bool {
        self.syncokc() != 0
    }

    #[doc="Sets the SYNCOKC field."]
    #[inline] pub fn set_syncokc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="warning clear flag"]
    #[inline] pub fn syncwarnc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SYNCWARNC != 0"]
    #[inline] pub fn test_syncwarnc(&self) -> bool {
        self.syncwarnc() != 0
    }

    #[doc="Sets the SYNCWARNC field."]
    #[inline] pub fn set_syncwarnc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error clear flag"]
    #[inline] pub fn errc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ERRC != 0"]
    #[inline] pub fn test_errc(&self) -> bool {
        self.errc() != 0
    }

    #[doc="Sets the ERRC field."]
    #[inline] pub fn set_errc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Expected SYNC clear flag"]
    #[inline] pub fn esyncc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ESYNCC != 0"]
    #[inline] pub fn test_esyncc(&self) -> bool {
        self.esyncc() != 0
    }

    #[doc="Sets the ESYNCC field."]
    #[inline] pub fn set_esyncc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Icr(other)
    }
}

impl ::core::fmt::Display for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.syncokc() != 0 { try!(write!(f, " syncokc"))}
        if self.syncwarnc() != 0 { try!(write!(f, " syncwarnc"))}
        if self.errc() != 0 { try!(write!(f, " errc"))}
        if self.esyncc() != 0 { try!(write!(f, " esyncc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

