#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Flash"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FlashPeriph(pub usize);
impl FlashPeriph {
    #[doc="Get the ACR Register."]
    #[inline] pub fn acr_reg(&self) -> Register<Acr> { 
        Register::new(self.0 as *mut Acr, 0x0)
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
    #[inline] pub fn keyr_reg(&self) -> Register<Keyr> { 
        Register::new(self.0 as *mut Keyr, 0x4)
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
    #[inline] pub fn optkeyr_reg(&self) -> Register<Optkeyr> { 
        Register::new(self.0 as *mut Optkeyr, 0x8)
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
    #[inline] pub fn sr_reg(&self) -> Register<Sr> { 
        Register::new(self.0 as *mut Sr, 0xc)
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
    #[inline] pub fn cr_reg(&self) -> Register<Cr> { 
        Register::new(self.0 as *mut Cr, 0x10)
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

    #[doc="Get the AR Register."]
    #[inline] pub fn ar_reg(&self) -> Register<Ar> { 
        Register::new(self.0 as *mut Ar, 0x14)
    }

    #[doc="Get the *mut pointer for the AR register."]
    #[inline] pub fn ar_mut(&self) -> *mut Ar { 
        self.ar_reg().ptr()
    }

    #[doc="Get the *const pointer for the AR register."]
    #[inline] pub fn ar_ptr(&self) -> *const Ar { 
        self.ar_reg().ptr()
    }

    #[doc="Write the AR register."]
    #[inline] pub fn write_ar(&self, value: Ar) -> &Self { 
        self.ar_reg().write(value);
        self
    }

    #[doc="Set the AR register."]
    #[inline] pub fn set_ar<F: FnOnce(Ar) -> Ar>(&self, f: F) -> &Self {
        self.ar_reg().set(f);
        self
    }

    #[doc="Get the OBR Register."]
    #[inline] pub fn obr_reg(&self) -> Register<Obr> { 
        Register::new(self.0 as *mut Obr, 0x1c)
    }

    #[doc="Get the *mut pointer for the OBR register."]
    #[inline] pub fn obr_mut(&self) -> *mut Obr { 
        self.obr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OBR register."]
    #[inline] pub fn obr_ptr(&self) -> *const Obr { 
        self.obr_reg().ptr()
    }

    #[doc="Read the OBR register."]
    #[inline] pub fn obr(&self) -> Obr { 
        self.obr_reg().read()
    }

    #[doc="Get the WRPR Register."]
    #[inline] pub fn wrpr_reg(&self) -> Register<Wrpr> { 
        Register::new(self.0 as *mut Wrpr, 0x20)
    }

    #[doc="Get the *mut pointer for the WRPR register."]
    #[inline] pub fn wrpr_mut(&self) -> *mut Wrpr { 
        self.wrpr_reg().ptr()
    }

    #[doc="Get the *const pointer for the WRPR register."]
    #[inline] pub fn wrpr_ptr(&self) -> *const Wrpr { 
        self.wrpr_reg().ptr()
    }

    #[doc="Read the WRPR register."]
    #[inline] pub fn wrpr(&self) -> Wrpr { 
        self.wrpr_reg().read()
    }

}

#[doc="Flash access control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc="LATENCY"]
    #[inline] pub fn latency(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LATENCY != 0"]
    #[inline] pub fn test_latency(&self) -> bool {
        self.latency() != 0
    }

    #[doc="Sets the LATENCY field."]
    #[inline] pub fn set_latency<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PRFTBE"]
    #[inline] pub fn prftbe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PRFTBE != 0"]
    #[inline] pub fn test_prftbe(&self) -> bool {
        self.prftbe() != 0
    }

    #[doc="Sets the PRFTBE field."]
    #[inline] pub fn set_prftbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="PRFTBS"]
    #[inline] pub fn prftbs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PRFTBS != 0"]
    #[inline] pub fn test_prftbs(&self) -> bool {
        self.prftbs() != 0
    }

    #[doc="Sets the PRFTBS field."]
    #[inline] pub fn set_prftbs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
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
        if self.prftbe() != 0 { try!(write!(f, " prftbe"))}
        if self.prftbs() != 0 { try!(write!(f, " prftbs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr(pub u32);
impl Keyr {
    #[doc="Flash Key"]
    #[inline] pub fn fkeyr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if FKEYR != 0"]
    #[inline] pub fn test_fkeyr(&self) -> bool {
        self.fkeyr() != 0
    }

    #[doc="Sets the FKEYR field."]
    #[inline] pub fn set_fkeyr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
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
    #[inline] pub fn optkeyr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if OPTKEYR != 0"]
    #[inline] pub fn test_optkeyr(&self) -> bool {
        self.optkeyr() != 0
    }

    #[doc="Sets the OPTKEYR field."]
    #[inline] pub fn set_optkeyr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
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

#[doc="Flash status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="End of operation"]
    #[inline] pub fn eop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if EOP != 0"]
    #[inline] pub fn test_eop(&self) -> bool {
        self.eop() != 0
    }

    #[doc="Sets the EOP field."]
    #[inline] pub fn set_eop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Write protection error"]
    #[inline] pub fn wrprt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WRPRT != 0"]
    #[inline] pub fn test_wrprt(&self) -> bool {
        self.wrprt() != 0
    }

    #[doc="Sets the WRPRT field."]
    #[inline] pub fn set_wrprt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Programming error"]
    #[inline] pub fn pgerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PGERR != 0"]
    #[inline] pub fn test_pgerr(&self) -> bool {
        self.pgerr() != 0
    }

    #[doc="Sets the PGERR field."]
    #[inline] pub fn set_pgerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn bsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BSY != 0"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy() != 0
    }

    #[doc="Sets the BSY field."]
    #[inline] pub fn set_bsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.wrprt() != 0 { try!(write!(f, " wrprt"))}
        if self.pgerr() != 0 { try!(write!(f, " pgerr"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Force option byte loading"]
    #[inline] pub fn force_optload(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FORCE_OPTLOAD != 0"]
    #[inline] pub fn test_force_optload(&self) -> bool {
        self.force_optload() != 0
    }

    #[doc="Sets the FORCE_OPTLOAD field."]
    #[inline] pub fn set_force_optload<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn eopie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if EOPIE != 0"]
    #[inline] pub fn test_eopie(&self) -> bool {
        self.eopie() != 0
    }

    #[doc="Sets the EOPIE field."]
    #[inline] pub fn set_eopie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Option bytes write enable"]
    #[inline] pub fn optwre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if OPTWRE != 0"]
    #[inline] pub fn test_optwre(&self) -> bool {
        self.optwre() != 0
    }

    #[doc="Sets the OPTWRE field."]
    #[inline] pub fn set_optwre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Lock"]
    #[inline] pub fn lock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Start"]
    #[inline] pub fn strt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if STRT != 0"]
    #[inline] pub fn test_strt(&self) -> bool {
        self.strt() != 0
    }

    #[doc="Sets the STRT field."]
    #[inline] pub fn set_strt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Option byte erase"]
    #[inline] pub fn opter(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if OPTER != 0"]
    #[inline] pub fn test_opter(&self) -> bool {
        self.opter() != 0
    }

    #[doc="Sets the OPTER field."]
    #[inline] pub fn set_opter<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Option byte programming"]
    #[inline] pub fn optpg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OPTPG != 0"]
    #[inline] pub fn test_optpg(&self) -> bool {
        self.optpg() != 0
    }

    #[doc="Sets the OPTPG field."]
    #[inline] pub fn set_optpg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Mass erase"]
    #[inline] pub fn mer(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MER != 0"]
    #[inline] pub fn test_mer(&self) -> bool {
        self.mer() != 0
    }

    #[doc="Sets the MER field."]
    #[inline] pub fn set_mer<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Page erase"]
    #[inline] pub fn per(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Programming"]
    #[inline] pub fn pg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PG != 0"]
    #[inline] pub fn test_pg(&self) -> bool {
        self.pg() != 0
    }

    #[doc="Sets the PG field."]
    #[inline] pub fn set_pg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.force_optload() != 0 { try!(write!(f, " force_optload"))}
        if self.eopie() != 0 { try!(write!(f, " eopie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.optwre() != 0 { try!(write!(f, " optwre"))}
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.strt() != 0 { try!(write!(f, " strt"))}
        if self.opter() != 0 { try!(write!(f, " opter"))}
        if self.optpg() != 0 { try!(write!(f, " optpg"))}
        if self.mer() != 0 { try!(write!(f, " mer"))}
        if self.per() != 0 { try!(write!(f, " per"))}
        if self.pg() != 0 { try!(write!(f, " pg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ar(pub u32);
impl Ar {
    #[doc="Flash address"]
    #[inline] pub fn far(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if FAR != 0"]
    #[inline] pub fn test_far(&self) -> bool {
        self.far() != 0
    }

    #[doc="Sets the FAR field."]
    #[inline] pub fn set_far<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ar {
    #[inline]
    fn from(other: u32) -> Self {
         Ar(other)
    }
}

impl ::core::fmt::Display for Ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Option byte register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Obr(pub u32);
impl Obr {
    #[doc="Option byte error"]
    #[inline] pub fn opterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OPTERR != 0"]
    #[inline] pub fn test_opterr(&self) -> bool {
        self.opterr() != 0
    }

    #[doc="Sets the OPTERR field."]
    #[inline] pub fn set_opterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Level 1 protection status"]
    #[inline] pub fn level1_prot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LEVEL1_PROT != 0"]
    #[inline] pub fn test_level1_prot(&self) -> bool {
        self.level1_prot() != 0
    }

    #[doc="Sets the LEVEL1_PROT field."]
    #[inline] pub fn set_level1_prot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Level 2 protection status"]
    #[inline] pub fn level2_prot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LEVEL2_PROT != 0"]
    #[inline] pub fn test_level2_prot(&self) -> bool {
        self.level2_prot() != 0
    }

    #[doc="Sets the LEVEL2_PROT field."]
    #[inline] pub fn set_level2_prot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="WDG_SW"]
    #[inline] pub fn wdg_sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WDG_SW != 0"]
    #[inline] pub fn test_wdg_sw(&self) -> bool {
        self.wdg_sw() != 0
    }

    #[doc="Sets the WDG_SW field."]
    #[inline] pub fn set_wdg_sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="nRST_STOP"]
    #[inline] pub fn nrst_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if nRST_STOP != 0"]
    #[inline] pub fn test_nrst_stop(&self) -> bool {
        self.nrst_stop() != 0
    }

    #[doc="Sets the nRST_STOP field."]
    #[inline] pub fn set_nrst_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="nRST_STDBY"]
    #[inline] pub fn nrst_stdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if nRST_STDBY != 0"]
    #[inline] pub fn test_nrst_stdby(&self) -> bool {
        self.nrst_stdby() != 0
    }

    #[doc="Sets the nRST_STDBY field."]
    #[inline] pub fn set_nrst_stdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="BOOT1"]
    #[inline] pub fn boot1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if BOOT1 != 0"]
    #[inline] pub fn test_boot1(&self) -> bool {
        self.boot1() != 0
    }

    #[doc="Sets the BOOT1 field."]
    #[inline] pub fn set_boot1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="VDDA_MONITOR"]
    #[inline] pub fn vdda_monitor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if VDDA_MONITOR != 0"]
    #[inline] pub fn test_vdda_monitor(&self) -> bool {
        self.vdda_monitor() != 0
    }

    #[doc="Sets the VDDA_MONITOR field."]
    #[inline] pub fn set_vdda_monitor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="SRAM_PARITY_CHECK"]
    #[inline] pub fn sram_parity_check(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SRAM_PARITY_CHECK != 0"]
    #[inline] pub fn test_sram_parity_check(&self) -> bool {
        self.sram_parity_check() != 0
    }

    #[doc="Sets the SRAM_PARITY_CHECK field."]
    #[inline] pub fn set_sram_parity_check<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Data0"]
    #[inline] pub fn data0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if Data0 != 0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0() != 0
    }

    #[doc="Sets the Data0 field."]
    #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Data1"]
    #[inline] pub fn data1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if Data1 != 0"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1() != 0
    }

    #[doc="Sets the Data1 field."]
    #[inline] pub fn set_data1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Obr {
    #[inline]
    fn from(other: u32) -> Self {
         Obr(other)
    }
}

impl ::core::fmt::Display for Obr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Obr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.opterr() != 0 { try!(write!(f, " opterr"))}
        if self.level1_prot() != 0 { try!(write!(f, " level1_prot"))}
        if self.level2_prot() != 0 { try!(write!(f, " level2_prot"))}
        if self.wdg_sw() != 0 { try!(write!(f, " wdg_sw"))}
        if self.nrst_stop() != 0 { try!(write!(f, " nrst_stop"))}
        if self.nrst_stdby() != 0 { try!(write!(f, " nrst_stdby"))}
        if self.boot1() != 0 { try!(write!(f, " boot1"))}
        if self.vdda_monitor() != 0 { try!(write!(f, " vdda_monitor"))}
        if self.sram_parity_check() != 0 { try!(write!(f, " sram_parity_check"))}
        if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
        if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Write protection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrpr(pub u32);
impl Wrpr {
    #[doc="Write protect"]
    #[inline] pub fn wrp(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if WRP != 0"]
    #[inline] pub fn test_wrp(&self) -> bool {
        self.wrp() != 0
    }

    #[doc="Sets the WRP field."]
    #[inline] pub fn set_wrp<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wrpr {
    #[inline]
    fn from(other: u32) -> Self {
         Wrpr(other)
    }
}

impl ::core::fmt::Display for Wrpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

