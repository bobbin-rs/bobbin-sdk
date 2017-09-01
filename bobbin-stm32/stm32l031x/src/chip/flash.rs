//! Flash
#[allow(unused_imports)] use bobbin_common::*;

periph!(FLASH, Flash, 0x40022000);

#[doc="Flash"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Flash(pub usize);
impl Flash {
    #[doc="Get the *const pointer for the ACR register."]
    #[inline] pub fn acr_ptr(&self) -> *const Acr { 
        (self.0 + 0x0) as *const Acr
    }

    #[doc="Get the *mut pointer for the ACR register."]
    #[inline] pub fn acr_mut(&self) -> *mut Acr { 
        (self.0 + 0x0) as *mut Acr
    }

    #[doc="Read the ACR register."]
    #[inline] pub fn acr(&self) -> Acr { 
        unsafe {
            read_volatile((self.0 + 0x0) as *const Acr)
        }
    }

    #[doc="Write the ACR register."]
    #[inline] pub fn set_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x0) as *mut Acr, f(Acr(0)));
        }
        self
    }

    #[doc="Modify the ACR register."]
    #[inline] pub fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x0) as *mut Acr, f(self.acr()));
        }
        self
    }

    #[doc="Get the *const pointer for the PECR register."]
    #[inline] pub fn pecr_ptr(&self) -> *const Pecr { 
        (self.0 + 0x4) as *const Pecr
    }

    #[doc="Get the *mut pointer for the PECR register."]
    #[inline] pub fn pecr_mut(&self) -> *mut Pecr { 
        (self.0 + 0x4) as *mut Pecr
    }

    #[doc="Read the PECR register."]
    #[inline] pub fn pecr(&self) -> Pecr { 
        unsafe {
            read_volatile((self.0 + 0x4) as *const Pecr)
        }
    }

    #[doc="Write the PECR register."]
    #[inline] pub fn set_pecr<F: FnOnce(Pecr) -> Pecr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x4) as *mut Pecr, f(Pecr(0)));
        }
        self
    }

    #[doc="Modify the PECR register."]
    #[inline] pub fn with_pecr<F: FnOnce(Pecr) -> Pecr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x4) as *mut Pecr, f(self.pecr()));
        }
        self
    }

    #[doc="Get the *const pointer for the PDKEYR register."]
    #[inline] pub fn pdkeyr_ptr(&self) -> *const Pdkeyr { 
        (self.0 + 0x8) as *const Pdkeyr
    }

    #[doc="Get the *mut pointer for the PDKEYR register."]
    #[inline] pub fn pdkeyr_mut(&self) -> *mut Pdkeyr { 
        (self.0 + 0x8) as *mut Pdkeyr
    }

    #[doc="Write the PDKEYR register."]
    #[inline] pub fn set_pdkeyr<F: FnOnce(Pdkeyr) -> Pdkeyr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x8) as *mut Pdkeyr, f(Pdkeyr(0)));
        }
        self
    }

    #[doc="Get the *const pointer for the PEKEYR register."]
    #[inline] pub fn pekeyr_ptr(&self) -> *const Pekeyr { 
        (self.0 + 0xc) as *const Pekeyr
    }

    #[doc="Get the *mut pointer for the PEKEYR register."]
    #[inline] pub fn pekeyr_mut(&self) -> *mut Pekeyr { 
        (self.0 + 0xc) as *mut Pekeyr
    }

    #[doc="Write the PEKEYR register."]
    #[inline] pub fn set_pekeyr<F: FnOnce(Pekeyr) -> Pekeyr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xc) as *mut Pekeyr, f(Pekeyr(0)));
        }
        self
    }

    #[doc="Get the *const pointer for the PRGKEYR register."]
    #[inline] pub fn prgkeyr_ptr(&self) -> *const Prgkeyr { 
        (self.0 + 0x10) as *const Prgkeyr
    }

    #[doc="Get the *mut pointer for the PRGKEYR register."]
    #[inline] pub fn prgkeyr_mut(&self) -> *mut Prgkeyr { 
        (self.0 + 0x10) as *mut Prgkeyr
    }

    #[doc="Write the PRGKEYR register."]
    #[inline] pub fn set_prgkeyr<F: FnOnce(Prgkeyr) -> Prgkeyr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x10) as *mut Prgkeyr, f(Prgkeyr(0)));
        }
        self
    }

    #[doc="Get the *const pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_ptr(&self) -> *const Optkeyr { 
        (self.0 + 0x14) as *const Optkeyr
    }

    #[doc="Get the *mut pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_mut(&self) -> *mut Optkeyr { 
        (self.0 + 0x14) as *mut Optkeyr
    }

    #[doc="Write the OPTKEYR register."]
    #[inline] pub fn set_optkeyr<F: FnOnce(Optkeyr) -> Optkeyr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x14) as *mut Optkeyr, f(Optkeyr(0)));
        }
        self
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        (self.0 + 0x18) as *const Sr
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        (self.0 + 0x18) as *mut Sr
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        unsafe {
            read_volatile((self.0 + 0x18) as *const Sr)
        }
    }

    #[doc="Write the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x18) as *mut Sr, f(Sr(0)));
        }
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x18) as *mut Sr, f(self.sr()));
        }
        self
    }

    #[doc="Get the *const pointer for the OBR register."]
    #[inline] pub fn obr_ptr(&self) -> *const Obr { 
        (self.0 + 0x1c) as *const Obr
    }

    #[doc="Get the *mut pointer for the OBR register."]
    #[inline] pub fn obr_mut(&self) -> *mut Obr { 
        (self.0 + 0x1c) as *mut Obr
    }

    #[doc="Read the OBR register."]
    #[inline] pub fn obr(&self) -> Obr { 
        unsafe {
            read_volatile((self.0 + 0x1c) as *const Obr)
        }
    }

    #[doc="Get the *const pointer for the WRPR register."]
    #[inline] pub fn wrpr_ptr(&self) -> *const Wrpr { 
        (self.0 + 0x20) as *const Wrpr
    }

    #[doc="Get the *mut pointer for the WRPR register."]
    #[inline] pub fn wrpr_mut(&self) -> *mut Wrpr { 
        (self.0 + 0x20) as *mut Wrpr
    }

    #[doc="Read the WRPR register."]
    #[inline] pub fn wrpr(&self) -> Wrpr { 
        unsafe {
            read_volatile((self.0 + 0x20) as *const Wrpr)
        }
    }

    #[doc="Write the WRPR register."]
    #[inline] pub fn set_wrpr<F: FnOnce(Wrpr) -> Wrpr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x20) as *mut Wrpr, f(Wrpr(0)));
        }
        self
    }

    #[doc="Modify the WRPR register."]
    #[inline] pub fn with_wrpr<F: FnOnce(Wrpr) -> Wrpr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x20) as *mut Wrpr, f(self.wrpr()));
        }
        self
    }

}

#[doc="Access control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc="Latency"]
    #[inline] pub fn latency(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Latency"]
    #[inline] pub fn test_latency(&self) -> bool {
        self.latency() != 0
    }

    #[doc="Latency"]
    #[inline] pub fn set_latency<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Prefetch enable"]
    #[inline] pub fn prften(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Prefetch enable"]
    #[inline] pub fn test_prften(&self) -> bool {
        self.prften() != 0
    }

    #[doc="Prefetch enable"]
    #[inline] pub fn set_prften<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Flash mode during Sleep"]
    #[inline] pub fn sleep_pd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Flash mode during Sleep"]
    #[inline] pub fn test_sleep_pd(&self) -> bool {
        self.sleep_pd() != 0
    }

    #[doc="Flash mode during Sleep"]
    #[inline] pub fn set_sleep_pd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Flash mode during Run"]
    #[inline] pub fn run_pd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Flash mode during Run"]
    #[inline] pub fn test_run_pd(&self) -> bool {
        self.run_pd() != 0
    }

    #[doc="Flash mode during Run"]
    #[inline] pub fn set_run_pd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Disable Buffer"]
    #[inline] pub fn desab_buf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Disable Buffer"]
    #[inline] pub fn test_desab_buf(&self) -> bool {
        self.desab_buf() != 0
    }

    #[doc="Disable Buffer"]
    #[inline] pub fn set_desab_buf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Pre-read data address"]
    #[inline] pub fn pre_read(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Pre-read data address"]
    #[inline] pub fn test_pre_read(&self) -> bool {
        self.pre_read() != 0
    }

    #[doc="Pre-read data address"]
    #[inline] pub fn set_pre_read<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
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
        if self.latency() != 0 { try!(write!(f, " latency"))}
        if self.prften() != 0 { try!(write!(f, " prften"))}
        if self.sleep_pd() != 0 { try!(write!(f, " sleep_pd"))}
        if self.run_pd() != 0 { try!(write!(f, " run_pd"))}
        if self.desab_buf() != 0 { try!(write!(f, " desab_buf"))}
        if self.pre_read() != 0 { try!(write!(f, " pre_read"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Program/erase control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pecr(pub u32);
impl Pecr {
    #[doc="FLASH_PECR and data EEPROM lock"]
    #[inline] pub fn pelock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="FLASH_PECR and data EEPROM lock"]
    #[inline] pub fn test_pelock(&self) -> bool {
        self.pelock() != 0
    }

    #[doc="FLASH_PECR and data EEPROM lock"]
    #[inline] pub fn set_pelock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Program memory lock"]
    #[inline] pub fn prglock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Program memory lock"]
    #[inline] pub fn test_prglock(&self) -> bool {
        self.prglock() != 0
    }

    #[doc="Program memory lock"]
    #[inline] pub fn set_prglock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Option bytes block lock"]
    #[inline] pub fn optlock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Option bytes block lock"]
    #[inline] pub fn test_optlock(&self) -> bool {
        self.optlock() != 0
    }

    #[doc="Option bytes block lock"]
    #[inline] pub fn set_optlock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Program memory selection"]
    #[inline] pub fn prog(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Program memory selection"]
    #[inline] pub fn test_prog(&self) -> bool {
        self.prog() != 0
    }

    #[doc="Program memory selection"]
    #[inline] pub fn set_prog<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data EEPROM selection"]
    #[inline] pub fn data(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Data EEPROM selection"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Data EEPROM selection"]
    #[inline] pub fn set_data<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Fixed time data write for Byte, Half Word and Word programming"]
    #[inline] pub fn ftdw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Fixed time data write for Byte, Half Word and Word programming"]
    #[inline] pub fn test_ftdw(&self) -> bool {
        self.ftdw() != 0
    }

    #[doc="Fixed time data write for Byte, Half Word and Word programming"]
    #[inline] pub fn set_ftdw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Page or Double Word erase mode"]
    #[inline] pub fn erase(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Page or Double Word erase mode"]
    #[inline] pub fn test_erase(&self) -> bool {
        self.erase() != 0
    }

    #[doc="Page or Double Word erase mode"]
    #[inline] pub fn set_erase<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Half Page/Double Word programming mode"]
    #[inline] pub fn fprg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Half Page/Double Word programming mode"]
    #[inline] pub fn test_fprg(&self) -> bool {
        self.fprg() != 0
    }

    #[doc="Half Page/Double Word programming mode"]
    #[inline] pub fn set_fprg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Parallel bank mode"]
    #[inline] pub fn parallelbank(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Parallel bank mode"]
    #[inline] pub fn test_parallelbank(&self) -> bool {
        self.parallelbank() != 0
    }

    #[doc="Parallel bank mode"]
    #[inline] pub fn set_parallelbank<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="End of programming interrupt enable"]
    #[inline] pub fn eopie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="End of programming interrupt enable"]
    #[inline] pub fn test_eopie(&self) -> bool {
        self.eopie() != 0
    }

    #[doc="End of programming interrupt enable"]
    #[inline] pub fn set_eopie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Launch the option byte loading"]
    #[inline] pub fn obl_launch(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Launch the option byte loading"]
    #[inline] pub fn test_obl_launch(&self) -> bool {
        self.obl_launch() != 0
    }

    #[doc="Launch the option byte loading"]
    #[inline] pub fn set_obl_launch<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for Pecr {
    #[inline]
    fn from(other: u32) -> Self {
         Pecr(other)
    }
}

impl ::core::fmt::Display for Pecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pelock() != 0 { try!(write!(f, " pelock"))}
        if self.prglock() != 0 { try!(write!(f, " prglock"))}
        if self.optlock() != 0 { try!(write!(f, " optlock"))}
        if self.prog() != 0 { try!(write!(f, " prog"))}
        if self.data() != 0 { try!(write!(f, " data"))}
        if self.ftdw() != 0 { try!(write!(f, " ftdw"))}
        if self.erase() != 0 { try!(write!(f, " erase"))}
        if self.fprg() != 0 { try!(write!(f, " fprg"))}
        if self.parallelbank() != 0 { try!(write!(f, " parallelbank"))}
        if self.eopie() != 0 { try!(write!(f, " eopie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.obl_launch() != 0 { try!(write!(f, " obl_launch"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power down key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdkeyr(pub u32);
impl Pdkeyr {
    #[doc="RUN_PD in FLASH_ACR key"]
    #[inline] pub fn pdkeyr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="RUN_PD in FLASH_ACR key"]
    #[inline] pub fn test_pdkeyr(&self) -> bool {
        self.pdkeyr() != 0
    }

    #[doc="RUN_PD in FLASH_ACR key"]
    #[inline] pub fn set_pdkeyr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdkeyr {
    #[inline]
    fn from(other: u32) -> Self {
         Pdkeyr(other)
    }
}

impl ::core::fmt::Display for Pdkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Program/erase key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pekeyr(pub u32);
impl Pekeyr {
    #[doc="FLASH_PEC and data EEPROM key"]
    #[inline] pub fn pekeyr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="FLASH_PEC and data EEPROM key"]
    #[inline] pub fn test_pekeyr(&self) -> bool {
        self.pekeyr() != 0
    }

    #[doc="FLASH_PEC and data EEPROM key"]
    #[inline] pub fn set_pekeyr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pekeyr {
    #[inline]
    fn from(other: u32) -> Self {
         Pekeyr(other)
    }
}

impl ::core::fmt::Display for Pekeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pekeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Program memory key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prgkeyr(pub u32);
impl Prgkeyr {
    #[doc="Program memory key"]
    #[inline] pub fn prgkeyr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Program memory key"]
    #[inline] pub fn test_prgkeyr(&self) -> bool {
        self.prgkeyr() != 0
    }

    #[doc="Program memory key"]
    #[inline] pub fn set_prgkeyr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Prgkeyr {
    #[inline]
    fn from(other: u32) -> Self {
         Prgkeyr(other)
    }
}

impl ::core::fmt::Display for Prgkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prgkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Option byte key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Optkeyr(pub u32);
impl Optkeyr {
    #[doc="Option byte key"]
    #[inline] pub fn optkeyr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Option byte key"]
    #[inline] pub fn test_optkeyr(&self) -> bool {
        self.optkeyr() != 0
    }

    #[doc="Option byte key"]
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

#[doc="Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Write/erase operations in progress"]
    #[inline] pub fn bsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Write/erase operations in progress"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy() != 0
    }

    #[doc="Write/erase operations in progress"]
    #[inline] pub fn set_bsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="End of operation"]
    #[inline] pub fn eop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="End of operation"]
    #[inline] pub fn test_eop(&self) -> bool {
        self.eop() != 0
    }

    #[doc="End of operation"]
    #[inline] pub fn set_eop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End of high voltage"]
    #[inline] pub fn endhv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="End of high voltage"]
    #[inline] pub fn test_endhv(&self) -> bool {
        self.endhv() != 0
    }

    #[doc="End of high voltage"]
    #[inline] pub fn set_endhv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Flash memory module ready after low power mode"]
    #[inline] pub fn ready(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Flash memory module ready after low power mode"]
    #[inline] pub fn test_ready(&self) -> bool {
        self.ready() != 0
    }

    #[doc="Flash memory module ready after low power mode"]
    #[inline] pub fn set_ready<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Write protected error"]
    #[inline] pub fn wrperr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Write protected error"]
    #[inline] pub fn test_wrperr(&self) -> bool {
        self.wrperr() != 0
    }

    #[doc="Write protected error"]
    #[inline] pub fn set_wrperr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Programming alignment error"]
    #[inline] pub fn pgaerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Programming alignment error"]
    #[inline] pub fn test_pgaerr(&self) -> bool {
        self.pgaerr() != 0
    }

    #[doc="Programming alignment error"]
    #[inline] pub fn set_pgaerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Size error"]
    #[inline] pub fn sizerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Size error"]
    #[inline] pub fn test_sizerr(&self) -> bool {
        self.sizerr() != 0
    }

    #[doc="Size error"]
    #[inline] pub fn set_sizerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Option validity error"]
    #[inline] pub fn optverr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Option validity error"]
    #[inline] pub fn test_optverr(&self) -> bool {
        self.optverr() != 0
    }

    #[doc="Option validity error"]
    #[inline] pub fn set_optverr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="RDERR"]
    #[inline] pub fn rderr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="RDERR"]
    #[inline] pub fn test_rderr(&self) -> bool {
        self.rderr() != 0
    }

    #[doc="RDERR"]
    #[inline] pub fn set_rderr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="NOTZEROERR"]
    #[inline] pub fn notzeroerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="NOTZEROERR"]
    #[inline] pub fn test_notzeroerr(&self) -> bool {
        self.notzeroerr() != 0
    }

    #[doc="NOTZEROERR"]
    #[inline] pub fn set_notzeroerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="FWWERR"]
    #[inline] pub fn fwwerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="FWWERR"]
    #[inline] pub fn test_fwwerr(&self) -> bool {
        self.fwwerr() != 0
    }

    #[doc="FWWERR"]
    #[inline] pub fn set_fwwerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
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
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        if self.eop() != 0 { try!(write!(f, " eop"))}
        if self.endhv() != 0 { try!(write!(f, " endhv"))}
        if self.ready() != 0 { try!(write!(f, " ready"))}
        if self.wrperr() != 0 { try!(write!(f, " wrperr"))}
        if self.pgaerr() != 0 { try!(write!(f, " pgaerr"))}
        if self.sizerr() != 0 { try!(write!(f, " sizerr"))}
        if self.optverr() != 0 { try!(write!(f, " optverr"))}
        if self.rderr() != 0 { try!(write!(f, " rderr"))}
        if self.notzeroerr() != 0 { try!(write!(f, " notzeroerr"))}
        if self.fwwerr() != 0 { try!(write!(f, " fwwerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Option byte register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Obr(pub u32);
impl Obr {
    #[doc="Read protection"]
    #[inline] pub fn rdprt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Read protection"]
    #[inline] pub fn test_rdprt(&self) -> bool {
        self.rdprt() != 0
    }

    #[doc="Read protection"]
    #[inline] pub fn set_rdprt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="BOR_LEV"]
    #[inline] pub fn bor_lev(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="BOR_LEV"]
    #[inline] pub fn test_bor_lev(&self) -> bool {
        self.bor_lev() != 0
    }

    #[doc="BOR_LEV"]
    #[inline] pub fn set_bor_lev<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Selection of protection mode of WPR bits"]
    #[inline] pub fn sprmod(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Selection of protection mode of WPR bits"]
    #[inline] pub fn test_sprmod(&self) -> bool {
        self.sprmod() != 0
    }

    #[doc="Selection of protection mode of WPR bits"]
    #[inline] pub fn set_sprmod<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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
        if self.rdprt() != 0 { try!(write!(f, " rdprt=0x{:x}", self.rdprt()))}
        if self.bor_lev() != 0 { try!(write!(f, " bor_lev=0x{:x}", self.bor_lev()))}
        if self.sprmod() != 0 { try!(write!(f, " sprmod"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Write protection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrpr(pub u32);
impl Wrpr {
    #[doc="Write protection"]
    #[inline] pub fn wrp(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Write protection"]
    #[inline] pub fn test_wrp(&self) -> bool {
        self.wrp() != 0
    }

    #[doc="Write protection"]
    #[inline] pub fn set_wrp<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
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
        if self.wrp() != 0 { try!(write!(f, " wrp=0x{:x}", self.wrp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


