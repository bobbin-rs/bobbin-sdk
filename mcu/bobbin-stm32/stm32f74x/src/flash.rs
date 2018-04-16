
::bobbin_mcu::periph!( FLASH, Flash, FLASH_PERIPH, FlashPeriph, FLASH_OWNED, FLASH_REF_COUNT, 0x40023c00, 0x00, 0x01);


#[doc="FLASH"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FlashPeriph(pub usize);
impl FlashPeriph {
    #[doc="Get the ACR Register."]
    #[inline] pub fn acr_reg(&self) -> ::bobbin_mcu::register::Register<Acr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Acr, 0x0)
    }

    #[doc="Get the *mut pointer for the ACR register."]
    #[inline] pub fn acr_mut(&self) -> *mut Acr { 
        self.acr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ACR register."]
    #[inline] pub fn acr_ptr(&self) -> *const Acr { 
        self.acr_reg().ptr()
    }

    #[doc="Read the ACR register."]
    #[inline] pub fn acr(&self) -> Acr { 
        self.acr_reg().read()
    }

    #[doc="Write the ACR register."]
    #[inline] pub fn write_acr(&self, value: Acr) -> &Self { 
        self.acr_reg().write(value);
        self
    }

    #[doc="Set the ACR register."]
    #[inline] pub fn set_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        self.acr_reg().set(f);
        self
    }

    #[doc="Modify the ACR register."]
    #[inline] pub fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        self.acr_reg().with(f);
        self
    }

    #[doc="Get the KEYR Register."]
    #[inline] pub fn keyr_reg(&self) -> ::bobbin_mcu::register::Register<Keyr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr, 0x4)
    }

    #[doc="Get the *mut pointer for the KEYR register."]
    #[inline] pub fn keyr_mut(&self) -> *mut Keyr { 
        self.keyr_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR register."]
    #[inline] pub fn keyr_ptr(&self) -> *const Keyr { 
        self.keyr_reg().ptr()
    }

    #[doc="Write the KEYR register."]
    #[inline] pub fn write_keyr(&self, value: Keyr) -> &Self { 
        self.keyr_reg().write(value);
        self
    }

    #[doc="Set the KEYR register."]
    #[inline] pub fn set_keyr<F: FnOnce(Keyr) -> Keyr>(&self, f: F) -> &Self {
        self.keyr_reg().set(f);
        self
    }

    #[doc="Get the OPTKEYR Register."]
    #[inline] pub fn optkeyr_reg(&self) -> ::bobbin_mcu::register::Register<Optkeyr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Optkeyr, 0x8)
    }

    #[doc="Get the *mut pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_mut(&self) -> *mut Optkeyr { 
        self.optkeyr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_ptr(&self) -> *const Optkeyr { 
        self.optkeyr_reg().ptr()
    }

    #[doc="Write the OPTKEYR register."]
    #[inline] pub fn write_optkeyr(&self, value: Optkeyr) -> &Self { 
        self.optkeyr_reg().write(value);
        self
    }

    #[doc="Set the OPTKEYR register."]
    #[inline] pub fn set_optkeyr<F: FnOnce(Optkeyr) -> Optkeyr>(&self, f: F) -> &Self {
        self.optkeyr_reg().set(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0xc)
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

    #[doc="Get the OPTCR Register."]
    #[inline] pub fn optcr_reg(&self) -> ::bobbin_mcu::register::Register<Optcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Optcr, 0x14)
    }

    #[doc="Get the *mut pointer for the OPTCR register."]
    #[inline] pub fn optcr_mut(&self) -> *mut Optcr { 
        self.optcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OPTCR register."]
    #[inline] pub fn optcr_ptr(&self) -> *const Optcr { 
        self.optcr_reg().ptr()
    }

    #[doc="Read the OPTCR register."]
    #[inline] pub fn optcr(&self) -> Optcr { 
        self.optcr_reg().read()
    }

    #[doc="Write the OPTCR register."]
    #[inline] pub fn write_optcr(&self, value: Optcr) -> &Self { 
        self.optcr_reg().write(value);
        self
    }

    #[doc="Set the OPTCR register."]
    #[inline] pub fn set_optcr<F: FnOnce(Optcr) -> Optcr>(&self, f: F) -> &Self {
        self.optcr_reg().set(f);
        self
    }

    #[doc="Modify the OPTCR register."]
    #[inline] pub fn with_optcr<F: FnOnce(Optcr) -> Optcr>(&self, f: F) -> &Self {
        self.optcr_reg().with(f);
        self
    }

    #[doc="Get the OPTCR1 Register."]
    #[inline] pub fn optcr1_reg(&self) -> ::bobbin_mcu::register::Register<Optcr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Optcr1, 0x18)
    }

    #[doc="Get the *mut pointer for the OPTCR1 register."]
    #[inline] pub fn optcr1_mut(&self) -> *mut Optcr1 { 
        self.optcr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the OPTCR1 register."]
    #[inline] pub fn optcr1_ptr(&self) -> *const Optcr1 { 
        self.optcr1_reg().ptr()
    }

    #[doc="Read the OPTCR1 register."]
    #[inline] pub fn optcr1(&self) -> Optcr1 { 
        self.optcr1_reg().read()
    }

    #[doc="Write the OPTCR1 register."]
    #[inline] pub fn write_optcr1(&self, value: Optcr1) -> &Self { 
        self.optcr1_reg().write(value);
        self
    }

    #[doc="Set the OPTCR1 register."]
    #[inline] pub fn set_optcr1<F: FnOnce(Optcr1) -> Optcr1>(&self, f: F) -> &Self {
        self.optcr1_reg().set(f);
        self
    }

    #[doc="Modify the OPTCR1 register."]
    #[inline] pub fn with_optcr1<F: FnOnce(Optcr1) -> Optcr1>(&self, f: F) -> &Self {
        self.optcr1_reg().with(f);
        self
    }

}

#[doc="Flash access control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc="Latency"]
    #[inline] pub fn latency(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LATENCY != 0"]
    #[inline] pub fn test_latency(&self) -> bool {
        self.latency() != 0
    }

    #[doc="Sets the LATENCY field."]
    #[inline] pub fn set_latency<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Prefetch enable"]
    #[inline] pub fn prften(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PRFTEN != 0"]
    #[inline] pub fn test_prften(&self) -> bool {
        self.prften() != 0
    }

    #[doc="Sets the PRFTEN field."]
    #[inline] pub fn set_prften<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Instruction cache enable"]
    #[inline] pub fn icen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ICEN != 0"]
    #[inline] pub fn test_icen(&self) -> bool {
        self.icen() != 0
    }

    #[doc="Sets the ICEN field."]
    #[inline] pub fn set_icen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data cache enable"]
    #[inline] pub fn dcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DCEN != 0"]
    #[inline] pub fn test_dcen(&self) -> bool {
        self.dcen() != 0
    }

    #[doc="Sets the DCEN field."]
    #[inline] pub fn set_dcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Instruction cache reset"]
    #[inline] pub fn icrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ICRST != 0"]
    #[inline] pub fn test_icrst(&self) -> bool {
        self.icrst() != 0
    }

    #[doc="Sets the ICRST field."]
    #[inline] pub fn set_icrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Data cache reset"]
    #[inline] pub fn dcrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DCRST != 0"]
    #[inline] pub fn test_dcrst(&self) -> bool {
        self.dcrst() != 0
    }

    #[doc="Sets the DCRST field."]
    #[inline] pub fn set_dcrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Acr {
    #[inline]
    fn from(other: u32) -> Self {
         Acr(other)
    }
}

impl ::core::fmt::Display for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.latency() != 0 { try!(write!(f, " latency=0x{:x}", self.latency()))}
        if self.prften() != 0 { try!(write!(f, " prften"))}
        if self.icen() != 0 { try!(write!(f, " icen"))}
        if self.dcen() != 0 { try!(write!(f, " dcen"))}
        if self.icrst() != 0 { try!(write!(f, " icrst"))}
        if self.dcrst() != 0 { try!(write!(f, " dcrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr(pub u32);
impl Keyr {
    #[doc="FPEC key"]
    #[inline] pub fn key(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if KEY != 0"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Sets the KEY field."]
    #[inline] pub fn set_key<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr(other)
    }
}

impl ::core::fmt::Display for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash option key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Optkeyr(pub u32);
impl Optkeyr {
    #[doc="Option byte key"]
    #[inline] pub fn optkey(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if OPTKEY != 0"]
    #[inline] pub fn test_optkey(&self) -> bool {
        self.optkey() != 0
    }

    #[doc="Sets the OPTKEY field."]
    #[inline] pub fn set_optkey<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Optkeyr {
    #[inline]
    fn from(other: u32) -> Self {
         Optkeyr(other)
    }
}

impl ::core::fmt::Display for Optkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Optkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="End of operation"]
    #[inline] pub fn eop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EOP != 0"]
    #[inline] pub fn test_eop(&self) -> bool {
        self.eop() != 0
    }

    #[doc="Sets the EOP field."]
    #[inline] pub fn set_eop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Operation error"]
    #[inline] pub fn operr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OPERR != 0"]
    #[inline] pub fn test_operr(&self) -> bool {
        self.operr() != 0
    }

    #[doc="Sets the OPERR field."]
    #[inline] pub fn set_operr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Write protection error"]
    #[inline] pub fn wrperr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WRPERR != 0"]
    #[inline] pub fn test_wrperr(&self) -> bool {
        self.wrperr() != 0
    }

    #[doc="Sets the WRPERR field."]
    #[inline] pub fn set_wrperr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Programming alignment error"]
    #[inline] pub fn pgaerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PGAERR != 0"]
    #[inline] pub fn test_pgaerr(&self) -> bool {
        self.pgaerr() != 0
    }

    #[doc="Sets the PGAERR field."]
    #[inline] pub fn set_pgaerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Programming parallelism error"]
    #[inline] pub fn pgperr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PGPERR != 0"]
    #[inline] pub fn test_pgperr(&self) -> bool {
        self.pgperr() != 0
    }

    #[doc="Sets the PGPERR field."]
    #[inline] pub fn set_pgperr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Programming sequence error"]
    #[inline] pub fn pgserr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PGSERR != 0"]
    #[inline] pub fn test_pgserr(&self) -> bool {
        self.pgserr() != 0
    }

    #[doc="Sets the PGSERR field."]
    #[inline] pub fn set_pgserr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn bsy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BSY != 0"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy() != 0
    }

    #[doc="Sets the BSY field."]
    #[inline] pub fn set_bsy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.eop() != 0 { try!(write!(f, " eop"))}
        if self.operr() != 0 { try!(write!(f, " operr"))}
        if self.wrperr() != 0 { try!(write!(f, " wrperr"))}
        if self.pgaerr() != 0 { try!(write!(f, " pgaerr"))}
        if self.pgperr() != 0 { try!(write!(f, " pgperr"))}
        if self.pgserr() != 0 { try!(write!(f, " pgserr"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Programming"]
    #[inline] pub fn pg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PG != 0"]
    #[inline] pub fn test_pg(&self) -> bool {
        self.pg() != 0
    }

    #[doc="Sets the PG field."]
    #[inline] pub fn set_pg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Sector Erase"]
    #[inline] pub fn ser(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SER != 0"]
    #[inline] pub fn test_ser(&self) -> bool {
        self.ser() != 0
    }

    #[doc="Sets the SER field."]
    #[inline] pub fn set_ser<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Mass Erase of sectors 0 to 11"]
    #[inline] pub fn mer(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MER != 0"]
    #[inline] pub fn test_mer(&self) -> bool {
        self.mer() != 0
    }

    #[doc="Sets the MER field."]
    #[inline] pub fn set_mer<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Sector number"]
    #[inline] pub fn snb(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if SNB != 0"]
    #[inline] pub fn test_snb(&self) -> bool {
        self.snb() != 0
    }

    #[doc="Sets the SNB field."]
    #[inline] pub fn set_snb<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Program size"]
    #[inline] pub fn psize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if PSIZE != 0"]
    #[inline] pub fn test_psize(&self) -> bool {
        self.psize() != 0
    }

    #[doc="Sets the PSIZE field."]
    #[inline] pub fn set_psize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Mass Erase of sectors 12 to 23"]
    #[inline] pub fn mer1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if MER1 != 0"]
    #[inline] pub fn test_mer1(&self) -> bool {
        self.mer1() != 0
    }

    #[doc="Sets the MER1 field."]
    #[inline] pub fn set_mer1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Start"]
    #[inline] pub fn strt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if STRT != 0"]
    #[inline] pub fn test_strt(&self) -> bool {
        self.strt() != 0
    }

    #[doc="Sets the STRT field."]
    #[inline] pub fn set_strt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn eopie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if EOPIE != 0"]
    #[inline] pub fn test_eopie(&self) -> bool {
        self.eopie() != 0
    }

    #[doc="Sets the EOPIE field."]
    #[inline] pub fn set_eopie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Lock"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.pg() != 0 { try!(write!(f, " pg"))}
        if self.ser() != 0 { try!(write!(f, " ser"))}
        if self.mer() != 0 { try!(write!(f, " mer"))}
        if self.snb() != 0 { try!(write!(f, " snb=0x{:x}", self.snb()))}
        if self.psize() != 0 { try!(write!(f, " psize=0x{:x}", self.psize()))}
        if self.mer1() != 0 { try!(write!(f, " mer1"))}
        if self.strt() != 0 { try!(write!(f, " strt"))}
        if self.eopie() != 0 { try!(write!(f, " eopie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.lock() != 0 { try!(write!(f, " lock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash option control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Optcr(pub u32);
impl Optcr {
    #[doc="Option lock"]
    #[inline] pub fn optlock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OPTLOCK != 0"]
    #[inline] pub fn test_optlock(&self) -> bool {
        self.optlock() != 0
    }

    #[doc="Sets the OPTLOCK field."]
    #[inline] pub fn set_optlock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Option start"]
    #[inline] pub fn optstrt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OPTSTRT != 0"]
    #[inline] pub fn test_optstrt(&self) -> bool {
        self.optstrt() != 0
    }

    #[doc="Sets the OPTSTRT field."]
    #[inline] pub fn set_optstrt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="BOR reset Level"]
    #[inline] pub fn bor_lev(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if BOR_LEV != 0"]
    #[inline] pub fn test_bor_lev(&self) -> bool {
        self.bor_lev() != 0
    }

    #[doc="Sets the BOR_LEV field."]
    #[inline] pub fn set_bor_lev<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="WDG_SW User option bytes"]
    #[inline] pub fn wdg_sw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WDG_SW != 0"]
    #[inline] pub fn test_wdg_sw(&self) -> bool {
        self.wdg_sw() != 0
    }

    #[doc="Sets the WDG_SW field."]
    #[inline] pub fn set_wdg_sw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="nRST_STOP User option bytes"]
    #[inline] pub fn nrst_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if nRST_STOP != 0"]
    #[inline] pub fn test_nrst_stop(&self) -> bool {
        self.nrst_stop() != 0
    }

    #[doc="Sets the nRST_STOP field."]
    #[inline] pub fn set_nrst_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="nRST_STDBY User option bytes"]
    #[inline] pub fn nrst_stdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if nRST_STDBY != 0"]
    #[inline] pub fn test_nrst_stdby(&self) -> bool {
        self.nrst_stdby() != 0
    }

    #[doc="Sets the nRST_STDBY field."]
    #[inline] pub fn set_nrst_stdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Read protect"]
    #[inline] pub fn rdp(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if RDP != 0"]
    #[inline] pub fn test_rdp(&self) -> bool {
        self.rdp() != 0
    }

    #[doc="Sets the RDP field."]
    #[inline] pub fn set_rdp<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Not write protect"]
    #[inline] pub fn nwrp(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
    }

    #[doc="Returns true if nWRP != 0"]
    #[inline] pub fn test_nwrp(&self) -> bool {
        self.nwrp() != 0
    }

    #[doc="Sets the nWRP field."]
    #[inline] pub fn set_nwrp<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Optcr {
    #[inline]
    fn from(other: u32) -> Self {
         Optcr(other)
    }
}

impl ::core::fmt::Display for Optcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Optcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.optlock() != 0 { try!(write!(f, " optlock"))}
        if self.optstrt() != 0 { try!(write!(f, " optstrt"))}
        if self.bor_lev() != 0 { try!(write!(f, " bor_lev=0x{:x}", self.bor_lev()))}
        if self.wdg_sw() != 0 { try!(write!(f, " wdg_sw"))}
        if self.nrst_stop() != 0 { try!(write!(f, " nrst_stop"))}
        if self.nrst_stdby() != 0 { try!(write!(f, " nrst_stdby"))}
        if self.rdp() != 0 { try!(write!(f, " rdp=0x{:x}", self.rdp()))}
        if self.nwrp() != 0 { try!(write!(f, " nwrp=0x{:x}", self.nwrp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash option control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Optcr1(pub u32);
impl Optcr1 {
    #[doc="Not write protect"]
    #[inline] pub fn nwrp(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
    }

    #[doc="Returns true if nWRP != 0"]
    #[inline] pub fn test_nwrp(&self) -> bool {
        self.nwrp() != 0
    }

    #[doc="Sets the nWRP field."]
    #[inline] pub fn set_nwrp<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Optcr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Optcr1(other)
    }
}

impl ::core::fmt::Display for Optcr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Optcr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nwrp() != 0 { try!(write!(f, " nwrp=0x{:x}", self.nwrp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

