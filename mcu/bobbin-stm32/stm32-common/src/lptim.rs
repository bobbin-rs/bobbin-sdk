
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPTIM Peripheral"]
pub struct LptimPeriph(pub usize); 

impl LptimPeriph {
    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> ::bobbin_mcu::register::Register<Isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Isr, 0x0)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Icr, 0x4)
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        self.icr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
        self.icr_reg().ptr()
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

    #[doc="Get the IER Register."]
    #[inline] pub fn ier_reg(&self) -> ::bobbin_mcu::register::Register<Ier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ier, 0x8)
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

    #[doc="Get the CFGR Register."]
    #[inline] pub fn cfgr_reg(&self) -> ::bobbin_mcu::register::Register<Cfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfgr, 0xc)
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

    #[doc="Get the CMP Register."]
    #[inline] pub fn cmp_reg(&self) -> ::bobbin_mcu::register::Register<Cmp> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cmp, 0x14)
    }

    #[doc="Get the *mut pointer for the CMP register."]
    #[inline] pub fn cmp_mut(&self) -> *mut Cmp { 
        self.cmp_reg().ptr()
    }

    #[doc="Get the *const pointer for the CMP register."]
    #[inline] pub fn cmp_ptr(&self) -> *const Cmp { 
        self.cmp_reg().ptr()
    }

    #[doc="Read the CMP register."]
    #[inline] pub fn cmp(&self) -> Cmp { 
        self.cmp_reg().read()
    }

    #[doc="Write the CMP register."]
    #[inline] pub fn write_cmp(&self, value: Cmp) -> &Self { 
        self.cmp_reg().write(value);
        self
    }

    #[doc="Set the CMP register."]
    #[inline] pub fn set_cmp<F: FnOnce(Cmp) -> Cmp>(&self, f: F) -> &Self {
        self.cmp_reg().set(f);
        self
    }

    #[doc="Modify the CMP register."]
    #[inline] pub fn with_cmp<F: FnOnce(Cmp) -> Cmp>(&self, f: F) -> &Self {
        self.cmp_reg().with(f);
        self
    }

    #[doc="Get the ARR Register."]
    #[inline] pub fn arr_reg(&self) -> ::bobbin_mcu::register::Register<Arr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Arr, 0x18)
    }

    #[doc="Get the *mut pointer for the ARR register."]
    #[inline] pub fn arr_mut(&self) -> *mut Arr { 
        self.arr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ARR register."]
    #[inline] pub fn arr_ptr(&self) -> *const Arr { 
        self.arr_reg().ptr()
    }

    #[doc="Read the ARR register."]
    #[inline] pub fn arr(&self) -> Arr { 
        self.arr_reg().read()
    }

    #[doc="Write the ARR register."]
    #[inline] pub fn write_arr(&self, value: Arr) -> &Self { 
        self.arr_reg().write(value);
        self
    }

    #[doc="Set the ARR register."]
    #[inline] pub fn set_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
        self.arr_reg().set(f);
        self
    }

    #[doc="Modify the ARR register."]
    #[inline] pub fn with_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
        self.arr_reg().with(f);
        self
    }

    #[doc="Get the CNT Register."]
    #[inline] pub fn cnt_reg(&self) -> ::bobbin_mcu::register::Register<Cnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cnt, 0x1c)
    }

    #[doc="Get the *mut pointer for the CNT register."]
    #[inline] pub fn cnt_mut(&self) -> *mut Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNT register."]
    #[inline] pub fn cnt_ptr(&self) -> *const Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Read the CNT register."]
    #[inline] pub fn cnt(&self) -> Cnt { 
        self.cnt_reg().read()
    }

}

#[doc="Interrupt and Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Counter direction change up to down"]
    #[inline] pub fn down(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DOWN != 0"]
    #[inline] pub fn test_down(&self) -> bool {
        self.down() != 0
    }

    #[doc="Sets the DOWN field."]
    #[inline] pub fn set_down<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter direction change down to up"]
    #[inline] pub fn up(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if UP != 0"]
    #[inline] pub fn test_up(&self) -> bool {
        self.up() != 0
    }

    #[doc="Sets the UP field."]
    #[inline] pub fn set_up<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Autoreload register update OK"]
    #[inline] pub fn arrok(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ARROK != 0"]
    #[inline] pub fn test_arrok(&self) -> bool {
        self.arrok() != 0
    }

    #[doc="Sets the ARROK field."]
    #[inline] pub fn set_arrok<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Compare register update OK"]
    #[inline] pub fn cmpok(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMPOK != 0"]
    #[inline] pub fn test_cmpok(&self) -> bool {
        self.cmpok() != 0
    }

    #[doc="Sets the CMPOK field."]
    #[inline] pub fn set_cmpok<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="External trigger edge event"]
    #[inline] pub fn exttrig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EXTTRIG != 0"]
    #[inline] pub fn test_exttrig(&self) -> bool {
        self.exttrig() != 0
    }

    #[doc="Sets the EXTTRIG field."]
    #[inline] pub fn set_exttrig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Autoreload match"]
    #[inline] pub fn arrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ARRM != 0"]
    #[inline] pub fn test_arrm(&self) -> bool {
        self.arrm() != 0
    }

    #[doc="Sets the ARRM field."]
    #[inline] pub fn set_arrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Compare match"]
    #[inline] pub fn cmpm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMPM != 0"]
    #[inline] pub fn test_cmpm(&self) -> bool {
        self.cmpm() != 0
    }

    #[doc="Sets the CMPM field."]
    #[inline] pub fn set_cmpm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.down() != 0 { try!(write!(f, " down"))}
        if self.up() != 0 { try!(write!(f, " up"))}
        if self.arrok() != 0 { try!(write!(f, " arrok"))}
        if self.cmpok() != 0 { try!(write!(f, " cmpok"))}
        if self.exttrig() != 0 { try!(write!(f, " exttrig"))}
        if self.arrm() != 0 { try!(write!(f, " arrm"))}
        if self.cmpm() != 0 { try!(write!(f, " cmpm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="Direction change to down Clear Flag"]
    #[inline] pub fn downcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DOWNCF != 0"]
    #[inline] pub fn test_downcf(&self) -> bool {
        self.downcf() != 0
    }

    #[doc="Sets the DOWNCF field."]
    #[inline] pub fn set_downcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Direction change to UP Clear Flag"]
    #[inline] pub fn upcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if UPCF != 0"]
    #[inline] pub fn test_upcf(&self) -> bool {
        self.upcf() != 0
    }

    #[doc="Sets the UPCF field."]
    #[inline] pub fn set_upcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Autoreload register update OK Clear Flag"]
    #[inline] pub fn arrokcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ARROKCF != 0"]
    #[inline] pub fn test_arrokcf(&self) -> bool {
        self.arrokcf() != 0
    }

    #[doc="Sets the ARROKCF field."]
    #[inline] pub fn set_arrokcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Compare register update OK Clear Flag"]
    #[inline] pub fn cmpokcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMPOKCF != 0"]
    #[inline] pub fn test_cmpokcf(&self) -> bool {
        self.cmpokcf() != 0
    }

    #[doc="Sets the CMPOKCF field."]
    #[inline] pub fn set_cmpokcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="External trigger valid edge Clear Flag"]
    #[inline] pub fn exttrigcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EXTTRIGCF != 0"]
    #[inline] pub fn test_exttrigcf(&self) -> bool {
        self.exttrigcf() != 0
    }

    #[doc="Sets the EXTTRIGCF field."]
    #[inline] pub fn set_exttrigcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Autoreload match Clear Flag"]
    #[inline] pub fn arrmcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ARRMCF != 0"]
    #[inline] pub fn test_arrmcf(&self) -> bool {
        self.arrmcf() != 0
    }

    #[doc="Sets the ARRMCF field."]
    #[inline] pub fn set_arrmcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="compare match Clear Flag"]
    #[inline] pub fn cmpmcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMPMCF != 0"]
    #[inline] pub fn test_cmpmcf(&self) -> bool {
        self.cmpmcf() != 0
    }

    #[doc="Sets the CMPMCF field."]
    #[inline] pub fn set_cmpmcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.downcf() != 0 { try!(write!(f, " downcf"))}
        if self.upcf() != 0 { try!(write!(f, " upcf"))}
        if self.arrokcf() != 0 { try!(write!(f, " arrokcf"))}
        if self.cmpokcf() != 0 { try!(write!(f, " cmpokcf"))}
        if self.exttrigcf() != 0 { try!(write!(f, " exttrigcf"))}
        if self.arrmcf() != 0 { try!(write!(f, " arrmcf"))}
        if self.cmpmcf() != 0 { try!(write!(f, " cmpmcf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="Direction change to down Interrupt Enable"]
    #[inline] pub fn downie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DOWNIE != 0"]
    #[inline] pub fn test_downie(&self) -> bool {
        self.downie() != 0
    }

    #[doc="Sets the DOWNIE field."]
    #[inline] pub fn set_downie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Direction change to UP Interrupt Enable"]
    #[inline] pub fn upie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if UPIE != 0"]
    #[inline] pub fn test_upie(&self) -> bool {
        self.upie() != 0
    }

    #[doc="Sets the UPIE field."]
    #[inline] pub fn set_upie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Autoreload register update OK Interrupt Enable"]
    #[inline] pub fn arrokie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ARROKIE != 0"]
    #[inline] pub fn test_arrokie(&self) -> bool {
        self.arrokie() != 0
    }

    #[doc="Sets the ARROKIE field."]
    #[inline] pub fn set_arrokie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Compare register update OK Interrupt Enable"]
    #[inline] pub fn cmpokie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CMPOKIE != 0"]
    #[inline] pub fn test_cmpokie(&self) -> bool {
        self.cmpokie() != 0
    }

    #[doc="Sets the CMPOKIE field."]
    #[inline] pub fn set_cmpokie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="External trigger valid edge Interrupt Enable"]
    #[inline] pub fn exttrigie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EXTTRIGIE != 0"]
    #[inline] pub fn test_exttrigie(&self) -> bool {
        self.exttrigie() != 0
    }

    #[doc="Sets the EXTTRIGIE field."]
    #[inline] pub fn set_exttrigie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Autoreload match Interrupt Enable"]
    #[inline] pub fn arrmie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ARRMIE != 0"]
    #[inline] pub fn test_arrmie(&self) -> bool {
        self.arrmie() != 0
    }

    #[doc="Sets the ARRMIE field."]
    #[inline] pub fn set_arrmie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Compare match Interrupt Enable"]
    #[inline] pub fn cmpmie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMPMIE != 0"]
    #[inline] pub fn test_cmpmie(&self) -> bool {
        self.cmpmie() != 0
    }

    #[doc="Sets the CMPMIE field."]
    #[inline] pub fn set_cmpmie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.downie() != 0 { try!(write!(f, " downie"))}
        if self.upie() != 0 { try!(write!(f, " upie"))}
        if self.arrokie() != 0 { try!(write!(f, " arrokie"))}
        if self.cmpokie() != 0 { try!(write!(f, " cmpokie"))}
        if self.exttrigie() != 0 { try!(write!(f, " exttrigie"))}
        if self.arrmie() != 0 { try!(write!(f, " arrmie"))}
        if self.cmpmie() != 0 { try!(write!(f, " cmpmie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc="Encoder mode enable"]
    #[inline] pub fn enc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if ENC != 0"]
    #[inline] pub fn test_enc(&self) -> bool {
        self.enc() != 0
    }

    #[doc="Sets the ENC field."]
    #[inline] pub fn set_enc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="counter mode enabled"]
    #[inline] pub fn countmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if COUNTMODE != 0"]
    #[inline] pub fn test_countmode(&self) -> bool {
        self.countmode() != 0
    }

    #[doc="Sets the COUNTMODE field."]
    #[inline] pub fn set_countmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Registers update mode"]
    #[inline] pub fn preload(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PRELOAD != 0"]
    #[inline] pub fn test_preload(&self) -> bool {
        self.preload() != 0
    }

    #[doc="Sets the PRELOAD field."]
    #[inline] pub fn set_preload<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Waveform shape polarity"]
    #[inline] pub fn wavpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if WAVPOL != 0"]
    #[inline] pub fn test_wavpol(&self) -> bool {
        self.wavpol() != 0
    }

    #[doc="Sets the WAVPOL field."]
    #[inline] pub fn set_wavpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Waveform shape"]
    #[inline] pub fn wave(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if WAVE != 0"]
    #[inline] pub fn test_wave(&self) -> bool {
        self.wave() != 0
    }

    #[doc="Sets the WAVE field."]
    #[inline] pub fn set_wave<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Timeout enable"]
    #[inline] pub fn timout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if TIMOUT != 0"]
    #[inline] pub fn test_timout(&self) -> bool {
        self.timout() != 0
    }

    #[doc="Sets the TIMOUT field."]
    #[inline] pub fn set_timout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Trigger enable and polarity"]
    #[inline] pub fn trigen(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if TRIGEN != 0"]
    #[inline] pub fn test_trigen(&self) -> bool {
        self.trigen() != 0
    }

    #[doc="Sets the TRIGEN field."]
    #[inline] pub fn set_trigen<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Trigger selector"]
    #[inline] pub fn trigsel(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if TRIGSEL != 0"]
    #[inline] pub fn test_trigsel(&self) -> bool {
        self.trigsel() != 0
    }

    #[doc="Sets the TRIGSEL field."]
    #[inline] pub fn set_trigsel<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Clock prescaler"]
    #[inline] pub fn presc(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x7) as u8) } // [11:9]
    }

    #[doc="Returns true if PRESC != 0"]
    #[inline] pub fn test_presc(&self) -> bool {
        self.presc() != 0
    }

    #[doc="Sets the PRESC field."]
    #[inline] pub fn set_presc<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Configurable digital filter for trigger"]
    #[inline] pub fn trgflt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if TRGFLT != 0"]
    #[inline] pub fn test_trgflt(&self) -> bool {
        self.trgflt() != 0
    }

    #[doc="Sets the TRGFLT field."]
    #[inline] pub fn set_trgflt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Configurable digital filter for external clock"]
    #[inline] pub fn ckflt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if CKFLT != 0"]
    #[inline] pub fn test_ckflt(&self) -> bool {
        self.ckflt() != 0
    }

    #[doc="Sets the CKFLT field."]
    #[inline] pub fn set_ckflt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn ckpol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if CKPOL != 0"]
    #[inline] pub fn test_ckpol(&self) -> bool {
        self.ckpol() != 0
    }

    #[doc="Sets the CKPOL field."]
    #[inline] pub fn set_ckpol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clock selector"]
    #[inline] pub fn cksel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CKSEL != 0"]
    #[inline] pub fn test_cksel(&self) -> bool {
        self.cksel() != 0
    }

    #[doc="Sets the CKSEL field."]
    #[inline] pub fn set_cksel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.enc() != 0 { try!(write!(f, " enc"))}
        if self.countmode() != 0 { try!(write!(f, " countmode"))}
        if self.preload() != 0 { try!(write!(f, " preload"))}
        if self.wavpol() != 0 { try!(write!(f, " wavpol"))}
        if self.wave() != 0 { try!(write!(f, " wave"))}
        if self.timout() != 0 { try!(write!(f, " timout"))}
        if self.trigen() != 0 { try!(write!(f, " trigen=0x{:x}", self.trigen()))}
        if self.trigsel() != 0 { try!(write!(f, " trigsel=0x{:x}", self.trigsel()))}
        if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
        if self.trgflt() != 0 { try!(write!(f, " trgflt=0x{:x}", self.trgflt()))}
        if self.ckflt() != 0 { try!(write!(f, " ckflt=0x{:x}", self.ckflt()))}
        if self.ckpol() != 0 { try!(write!(f, " ckpol=0x{:x}", self.ckpol()))}
        if self.cksel() != 0 { try!(write!(f, " cksel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Timer start in continuous mode"]
    #[inline] pub fn cntstrt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CNTSTRT != 0"]
    #[inline] pub fn test_cntstrt(&self) -> bool {
        self.cntstrt() != 0
    }

    #[doc="Sets the CNTSTRT field."]
    #[inline] pub fn set_cntstrt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LPTIM start in single mode"]
    #[inline] pub fn sngstrt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SNGSTRT != 0"]
    #[inline] pub fn test_sngstrt(&self) -> bool {
        self.sngstrt() != 0
    }

    #[doc="Sets the SNGSTRT field."]
    #[inline] pub fn set_sngstrt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LPTIM Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.cntstrt() != 0 { try!(write!(f, " cntstrt"))}
        if self.sngstrt() != 0 { try!(write!(f, " sngstrt"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmp(pub u32);
impl Cmp {
    #[doc="Compare value."]
    #[inline] pub fn cmp(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CMP != 0"]
    #[inline] pub fn test_cmp(&self) -> bool {
        self.cmp() != 0
    }

    #[doc="Sets the CMP field."]
    #[inline] pub fn set_cmp<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmp {
    #[inline]
    fn from(other: u32) -> Self {
         Cmp(other)
    }
}

impl ::core::fmt::Display for Cmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmp() != 0 { try!(write!(f, " cmp=0x{:x}", self.cmp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Autoreload Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Arr(pub u32);
impl Arr {
    #[doc="Auto reload value."]
    #[inline] pub fn arr(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if ARR != 0"]
    #[inline] pub fn test_arr(&self) -> bool {
        self.arr() != 0
    }

    #[doc="Sets the ARR field."]
    #[inline] pub fn set_arr<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Arr {
    #[inline]
    fn from(other: u32) -> Self {
         Arr(other)
    }
}

impl ::core::fmt::Display for Arr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Arr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.arr() != 0 { try!(write!(f, " arr=0x{:x}", self.arr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="Counter value."]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cnt {
    #[inline]
    fn from(other: u32) -> Self {
         Cnt(other)
    }
}

impl ::core::fmt::Display for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

