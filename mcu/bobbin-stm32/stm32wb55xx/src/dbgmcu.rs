::bobbin_mcu::periph!( DBGMCU, Dbgmcu, DBGMCU_PERIPH, DbgmcuPeriph, DBGMCU_OWNED, DBGMCU_REF_COUNT, 0xe0042000, 0x00, 0x2b);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DBGMCU Peripheral"]
pub struct DbgmcuPeriph(pub usize); 

impl DbgmcuPeriph {
    #[doc="Get the IDCODE Register."]
    #[inline] pub fn idcode_reg(&self) -> ::bobbin_mcu::register::Register<Idcode> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Idcode, 0x0)
    }

    #[doc="Get the *mut pointer for the IDCODE register."]
    #[inline] pub fn idcode_mut(&self) -> *mut Idcode { 
        self.idcode_reg().ptr()
    }

    #[doc="Get the *const pointer for the IDCODE register."]
    #[inline] pub fn idcode_ptr(&self) -> *const Idcode { 
        self.idcode_reg().ptr()
    }

    #[doc="Read the IDCODE register."]
    #[inline] pub fn idcode(&self) -> Idcode { 
        self.idcode_reg().read()
    }

    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x4)
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

    #[doc="Get the APB1FZR1 Register."]
    #[inline] pub fn apb1fzr1_reg(&self) -> ::bobbin_mcu::register::Register<Apb1fzr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb1fzr1, 0x3c)
    }

    #[doc="Get the *mut pointer for the APB1FZR1 register."]
    #[inline] pub fn apb1fzr1_mut(&self) -> *mut Apb1fzr1 { 
        self.apb1fzr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB1FZR1 register."]
    #[inline] pub fn apb1fzr1_ptr(&self) -> *const Apb1fzr1 { 
        self.apb1fzr1_reg().ptr()
    }

    #[doc="Read the APB1FZR1 register."]
    #[inline] pub fn apb1fzr1(&self) -> Apb1fzr1 { 
        self.apb1fzr1_reg().read()
    }

    #[doc="Write the APB1FZR1 register."]
    #[inline] pub fn write_apb1fzr1(&self, value: Apb1fzr1) -> &Self { 
        self.apb1fzr1_reg().write(value);
        self
    }

    #[doc="Set the APB1FZR1 register."]
    #[inline] pub fn set_apb1fzr1<F: FnOnce(Apb1fzr1) -> Apb1fzr1>(&self, f: F) -> &Self {
        self.apb1fzr1_reg().set(f);
        self
    }

    #[doc="Modify the APB1FZR1 register."]
    #[inline] pub fn with_apb1fzr1<F: FnOnce(Apb1fzr1) -> Apb1fzr1>(&self, f: F) -> &Self {
        self.apb1fzr1_reg().with(f);
        self
    }

    #[doc="Get the C2AP_B1FZR1 Register."]
    #[inline] pub fn c2ap_b1fzr1_reg(&self) -> ::bobbin_mcu::register::Register<C2apB1fzr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2apB1fzr1, 0x40)
    }

    #[doc="Get the *mut pointer for the C2AP_B1FZR1 register."]
    #[inline] pub fn c2ap_b1fzr1_mut(&self) -> *mut C2apB1fzr1 { 
        self.c2ap_b1fzr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2AP_B1FZR1 register."]
    #[inline] pub fn c2ap_b1fzr1_ptr(&self) -> *const C2apB1fzr1 { 
        self.c2ap_b1fzr1_reg().ptr()
    }

    #[doc="Read the C2AP_B1FZR1 register."]
    #[inline] pub fn c2ap_b1fzr1(&self) -> C2apB1fzr1 { 
        self.c2ap_b1fzr1_reg().read()
    }

    #[doc="Write the C2AP_B1FZR1 register."]
    #[inline] pub fn write_c2ap_b1fzr1(&self, value: C2apB1fzr1) -> &Self { 
        self.c2ap_b1fzr1_reg().write(value);
        self
    }

    #[doc="Set the C2AP_B1FZR1 register."]
    #[inline] pub fn set_c2ap_b1fzr1<F: FnOnce(C2apB1fzr1) -> C2apB1fzr1>(&self, f: F) -> &Self {
        self.c2ap_b1fzr1_reg().set(f);
        self
    }

    #[doc="Modify the C2AP_B1FZR1 register."]
    #[inline] pub fn with_c2ap_b1fzr1<F: FnOnce(C2apB1fzr1) -> C2apB1fzr1>(&self, f: F) -> &Self {
        self.c2ap_b1fzr1_reg().with(f);
        self
    }

    #[doc="Get the APB1FZR2 Register."]
    #[inline] pub fn apb1fzr2_reg(&self) -> ::bobbin_mcu::register::Register<Apb1fzr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb1fzr2, 0x44)
    }

    #[doc="Get the *mut pointer for the APB1FZR2 register."]
    #[inline] pub fn apb1fzr2_mut(&self) -> *mut Apb1fzr2 { 
        self.apb1fzr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB1FZR2 register."]
    #[inline] pub fn apb1fzr2_ptr(&self) -> *const Apb1fzr2 { 
        self.apb1fzr2_reg().ptr()
    }

    #[doc="Read the APB1FZR2 register."]
    #[inline] pub fn apb1fzr2(&self) -> Apb1fzr2 { 
        self.apb1fzr2_reg().read()
    }

    #[doc="Write the APB1FZR2 register."]
    #[inline] pub fn write_apb1fzr2(&self, value: Apb1fzr2) -> &Self { 
        self.apb1fzr2_reg().write(value);
        self
    }

    #[doc="Set the APB1FZR2 register."]
    #[inline] pub fn set_apb1fzr2<F: FnOnce(Apb1fzr2) -> Apb1fzr2>(&self, f: F) -> &Self {
        self.apb1fzr2_reg().set(f);
        self
    }

    #[doc="Modify the APB1FZR2 register."]
    #[inline] pub fn with_apb1fzr2<F: FnOnce(Apb1fzr2) -> Apb1fzr2>(&self, f: F) -> &Self {
        self.apb1fzr2_reg().with(f);
        self
    }

    #[doc="Get the C2APB1FZR2 Register."]
    #[inline] pub fn c2apb1fzr2_reg(&self) -> ::bobbin_mcu::register::Register<C2apb1fzr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2apb1fzr2, 0x48)
    }

    #[doc="Get the *mut pointer for the C2APB1FZR2 register."]
    #[inline] pub fn c2apb1fzr2_mut(&self) -> *mut C2apb1fzr2 { 
        self.c2apb1fzr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2APB1FZR2 register."]
    #[inline] pub fn c2apb1fzr2_ptr(&self) -> *const C2apb1fzr2 { 
        self.c2apb1fzr2_reg().ptr()
    }

    #[doc="Read the C2APB1FZR2 register."]
    #[inline] pub fn c2apb1fzr2(&self) -> C2apb1fzr2 { 
        self.c2apb1fzr2_reg().read()
    }

    #[doc="Write the C2APB1FZR2 register."]
    #[inline] pub fn write_c2apb1fzr2(&self, value: C2apb1fzr2) -> &Self { 
        self.c2apb1fzr2_reg().write(value);
        self
    }

    #[doc="Set the C2APB1FZR2 register."]
    #[inline] pub fn set_c2apb1fzr2<F: FnOnce(C2apb1fzr2) -> C2apb1fzr2>(&self, f: F) -> &Self {
        self.c2apb1fzr2_reg().set(f);
        self
    }

    #[doc="Modify the C2APB1FZR2 register."]
    #[inline] pub fn with_c2apb1fzr2<F: FnOnce(C2apb1fzr2) -> C2apb1fzr2>(&self, f: F) -> &Self {
        self.c2apb1fzr2_reg().with(f);
        self
    }

    #[doc="Get the APB2FZR Register."]
    #[inline] pub fn apb2fzr_reg(&self) -> ::bobbin_mcu::register::Register<Apb2fzr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apb2fzr, 0x4c)
    }

    #[doc="Get the *mut pointer for the APB2FZR register."]
    #[inline] pub fn apb2fzr_mut(&self) -> *mut Apb2fzr { 
        self.apb2fzr_reg().ptr()
    }

    #[doc="Get the *const pointer for the APB2FZR register."]
    #[inline] pub fn apb2fzr_ptr(&self) -> *const Apb2fzr { 
        self.apb2fzr_reg().ptr()
    }

    #[doc="Read the APB2FZR register."]
    #[inline] pub fn apb2fzr(&self) -> Apb2fzr { 
        self.apb2fzr_reg().read()
    }

    #[doc="Write the APB2FZR register."]
    #[inline] pub fn write_apb2fzr(&self, value: Apb2fzr) -> &Self { 
        self.apb2fzr_reg().write(value);
        self
    }

    #[doc="Set the APB2FZR register."]
    #[inline] pub fn set_apb2fzr<F: FnOnce(Apb2fzr) -> Apb2fzr>(&self, f: F) -> &Self {
        self.apb2fzr_reg().set(f);
        self
    }

    #[doc="Modify the APB2FZR register."]
    #[inline] pub fn with_apb2fzr<F: FnOnce(Apb2fzr) -> Apb2fzr>(&self, f: F) -> &Self {
        self.apb2fzr_reg().with(f);
        self
    }

    #[doc="Get the C2APB2FZR Register."]
    #[inline] pub fn c2apb2fzr_reg(&self) -> ::bobbin_mcu::register::Register<C2apb2fzr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2apb2fzr, 0x48)
    }

    #[doc="Get the *mut pointer for the C2APB2FZR register."]
    #[inline] pub fn c2apb2fzr_mut(&self) -> *mut C2apb2fzr { 
        self.c2apb2fzr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2APB2FZR register."]
    #[inline] pub fn c2apb2fzr_ptr(&self) -> *const C2apb2fzr { 
        self.c2apb2fzr_reg().ptr()
    }

    #[doc="Read the C2APB2FZR register."]
    #[inline] pub fn c2apb2fzr(&self) -> C2apb2fzr { 
        self.c2apb2fzr_reg().read()
    }

    #[doc="Write the C2APB2FZR register."]
    #[inline] pub fn write_c2apb2fzr(&self, value: C2apb2fzr) -> &Self { 
        self.c2apb2fzr_reg().write(value);
        self
    }

    #[doc="Set the C2APB2FZR register."]
    #[inline] pub fn set_c2apb2fzr<F: FnOnce(C2apb2fzr) -> C2apb2fzr>(&self, f: F) -> &Self {
        self.c2apb2fzr_reg().set(f);
        self
    }

    #[doc="Modify the C2APB2FZR register."]
    #[inline] pub fn with_c2apb2fzr<F: FnOnce(C2apb2fzr) -> C2apb2fzr>(&self, f: F) -> &Self {
        self.c2apb2fzr_reg().with(f);
        self
    }

}

#[doc="MCU Device ID Code Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Idcode(pub u32);
impl Idcode {
    #[doc="Device Identifier"]
    #[inline] pub fn dev_id(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DEV_ID != 0"]
    #[inline] pub fn test_dev_id(&self) -> bool {
        self.dev_id() != 0
    }

    #[doc="Sets the DEV_ID field."]
    #[inline] pub fn set_dev_id<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Revision Identifier"]
    #[inline] pub fn rev_id(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if REV_ID != 0"]
    #[inline] pub fn test_rev_id(&self) -> bool {
        self.rev_id() != 0
    }

    #[doc="Sets the REV_ID field."]
    #[inline] pub fn set_rev_id<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Idcode {
    #[inline]
    fn from(other: u32) -> Self {
         Idcode(other)
    }
}

impl ::core::fmt::Display for Idcode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Idcode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dev_id() != 0 { try!(write!(f, " dev_id=0x{:x}", self.dev_id()))}
        if self.rev_id() != 0 { try!(write!(f, " rev_id=0x{:x}", self.rev_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug MCU Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Debug Sleep Mode"]
    #[inline] pub fn dbg_sleep(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBG_SLEEP != 0"]
    #[inline] pub fn test_dbg_sleep(&self) -> bool {
        self.dbg_sleep() != 0
    }

    #[doc="Sets the DBG_SLEEP field."]
    #[inline] pub fn set_dbg_sleep<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Debug Stop Mode"]
    #[inline] pub fn dbg_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DBG_STOP != 0"]
    #[inline] pub fn test_dbg_stop(&self) -> bool {
        self.dbg_stop() != 0
    }

    #[doc="Sets the DBG_STOP field."]
    #[inline] pub fn set_dbg_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Debug Standby Mode"]
    #[inline] pub fn dbg_standby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DBG_STANDBY != 0"]
    #[inline] pub fn test_dbg_standby(&self) -> bool {
        self.dbg_standby() != 0
    }

    #[doc="Sets the DBG_STANDBY field."]
    #[inline] pub fn set_dbg_standby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Trace port and clock enable"]
    #[inline] pub fn trace_ioen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TRACE_IOEN != 0"]
    #[inline] pub fn test_trace_ioen(&self) -> bool {
        self.trace_ioen() != 0
    }

    #[doc="Sets the TRACE_IOEN field."]
    #[inline] pub fn set_trace_ioen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="External trigger output enable"]
    #[inline] pub fn trgoen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if TRGOEN != 0"]
    #[inline] pub fn test_trgoen(&self) -> bool {
        self.trgoen() != 0
    }

    #[doc="Sets the TRGOEN field."]
    #[inline] pub fn set_trgoen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
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
        if self.dbg_sleep() != 0 { try!(write!(f, " dbg_sleep"))}
        if self.dbg_stop() != 0 { try!(write!(f, " dbg_stop"))}
        if self.dbg_standby() != 0 { try!(write!(f, " dbg_standby"))}
        if self.trace_ioen() != 0 { try!(write!(f, " trace_ioen"))}
        if self.trgoen() != 0 { try!(write!(f, " trgoen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 Low Freeze Register CPU1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1fzr1(pub u32);
impl Apb1fzr1 {
    #[doc="Debug Timer 2 stopped when Core is halted"]
    #[inline] pub fn dbg_timer2_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBG_TIMER2_STOP != 0"]
    #[inline] pub fn test_dbg_timer2_stop(&self) -> bool {
        self.dbg_timer2_stop() != 0
    }

    #[doc="Sets the DBG_TIMER2_STOP field."]
    #[inline] pub fn set_dbg_timer2_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RTC counter stopped when core is halted"]
    #[inline] pub fn dbg_rtc_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DBG_RTC_STOP != 0"]
    #[inline] pub fn test_dbg_rtc_stop(&self) -> bool {
        self.dbg_rtc_stop() != 0
    }

    #[doc="Sets the DBG_RTC_STOP field."]
    #[inline] pub fn set_dbg_rtc_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="WWDG counter stopped when core is halted"]
    #[inline] pub fn dbg_wwdg_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DBG_WWDG_STOP != 0"]
    #[inline] pub fn test_dbg_wwdg_stop(&self) -> bool {
        self.dbg_wwdg_stop() != 0
    }

    #[doc="Sets the DBG_WWDG_STOP field."]
    #[inline] pub fn set_dbg_wwdg_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="IWDG counter stopped when core is halted"]
    #[inline] pub fn dbg_iwdg_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DBG_IWDG_STOP != 0"]
    #[inline] pub fn test_dbg_iwdg_stop(&self) -> bool {
        self.dbg_iwdg_stop() != 0
    }

    #[doc="Sets the DBG_IWDG_STOP field."]
    #[inline] pub fn set_dbg_iwdg_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Debug I2C1 SMBUS timeout stopped when Core is halted"]
    #[inline] pub fn dbg_i2c1_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DBG_I2C1_STOP != 0"]
    #[inline] pub fn test_dbg_i2c1_stop(&self) -> bool {
        self.dbg_i2c1_stop() != 0
    }

    #[doc="Sets the DBG_I2C1_STOP field."]
    #[inline] pub fn set_dbg_i2c1_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Debug I2C3 SMBUS timeout stopped when core is halted"]
    #[inline] pub fn dbg_i2c3_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if DBG_I2C3_STOP != 0"]
    #[inline] pub fn test_dbg_i2c3_stop(&self) -> bool {
        self.dbg_i2c3_stop() != 0
    }

    #[doc="Sets the DBG_I2C3_STOP field."]
    #[inline] pub fn set_dbg_i2c3_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Debug LPTIM1 stopped when Core is halted"]
    #[inline] pub fn dbg_lptim1_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if DBG_LPTIM1_STOP != 0"]
    #[inline] pub fn test_dbg_lptim1_stop(&self) -> bool {
        self.dbg_lptim1_stop() != 0
    }

    #[doc="Sets the DBG_LPTIM1_STOP field."]
    #[inline] pub fn set_dbg_lptim1_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Apb1fzr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1fzr1(other)
    }
}

impl ::core::fmt::Display for Apb1fzr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1fzr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbg_timer2_stop() != 0 { try!(write!(f, " dbg_timer2_stop"))}
        if self.dbg_rtc_stop() != 0 { try!(write!(f, " dbg_rtc_stop"))}
        if self.dbg_wwdg_stop() != 0 { try!(write!(f, " dbg_wwdg_stop"))}
        if self.dbg_iwdg_stop() != 0 { try!(write!(f, " dbg_iwdg_stop"))}
        if self.dbg_i2c1_stop() != 0 { try!(write!(f, " dbg_i2c1_stop"))}
        if self.dbg_i2c3_stop() != 0 { try!(write!(f, " dbg_i2c3_stop"))}
        if self.dbg_lptim1_stop() != 0 { try!(write!(f, " dbg_lptim1_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 Low Freeze Register CPU2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2apB1fzr1(pub u32);
impl C2apB1fzr1 {
    #[doc="LPTIM2 counter stopped when core is halted"]
    #[inline] pub fn dbg_lptim2_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBG_LPTIM2_STOP != 0"]
    #[inline] pub fn test_dbg_lptim2_stop(&self) -> bool {
        self.dbg_lptim2_stop() != 0
    }

    #[doc="Sets the DBG_LPTIM2_STOP field."]
    #[inline] pub fn set_dbg_lptim2_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RTC counter stopped when core is halted"]
    #[inline] pub fn dbg_rtc_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DBG_RTC_STOP != 0"]
    #[inline] pub fn test_dbg_rtc_stop(&self) -> bool {
        self.dbg_rtc_stop() != 0
    }

    #[doc="Sets the DBG_RTC_STOP field."]
    #[inline] pub fn set_dbg_rtc_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="IWDG stopped when core is halted"]
    #[inline] pub fn dbg_iwdg_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DBG_IWDG_STOP != 0"]
    #[inline] pub fn test_dbg_iwdg_stop(&self) -> bool {
        self.dbg_iwdg_stop() != 0
    }

    #[doc="Sets the DBG_IWDG_STOP field."]
    #[inline] pub fn set_dbg_iwdg_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="I2C1 SMBUS timeout stopped when core is halted"]
    #[inline] pub fn dbg_i2c1_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DBG_I2C1_STOP != 0"]
    #[inline] pub fn test_dbg_i2c1_stop(&self) -> bool {
        self.dbg_i2c1_stop() != 0
    }

    #[doc="Sets the DBG_I2C1_STOP field."]
    #[inline] pub fn set_dbg_i2c1_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="I2C3 SMBUS timeout stopped when core is halted"]
    #[inline] pub fn dbg_i2c3_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if DBG_I2C3_STOP != 0"]
    #[inline] pub fn test_dbg_i2c3_stop(&self) -> bool {
        self.dbg_i2c3_stop() != 0
    }

    #[doc="Sets the DBG_I2C3_STOP field."]
    #[inline] pub fn set_dbg_i2c3_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="LPTIM1 counter stopped when core is halted"]
    #[inline] pub fn dbg_lptim1_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if DBG_LPTIM1_STOP != 0"]
    #[inline] pub fn test_dbg_lptim1_stop(&self) -> bool {
        self.dbg_lptim1_stop() != 0
    }

    #[doc="Sets the DBG_LPTIM1_STOP field."]
    #[inline] pub fn set_dbg_lptim1_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for C2apB1fzr1 {
    #[inline]
    fn from(other: u32) -> Self {
         C2apB1fzr1(other)
    }
}

impl ::core::fmt::Display for C2apB1fzr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2apB1fzr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbg_lptim2_stop() != 0 { try!(write!(f, " dbg_lptim2_stop"))}
        if self.dbg_rtc_stop() != 0 { try!(write!(f, " dbg_rtc_stop"))}
        if self.dbg_iwdg_stop() != 0 { try!(write!(f, " dbg_iwdg_stop"))}
        if self.dbg_i2c1_stop() != 0 { try!(write!(f, " dbg_i2c1_stop"))}
        if self.dbg_i2c3_stop() != 0 { try!(write!(f, " dbg_i2c3_stop"))}
        if self.dbg_lptim1_stop() != 0 { try!(write!(f, " dbg_lptim1_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 High Freeze Register CPU1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1fzr2(pub u32);
impl Apb1fzr2 {
    #[doc="LPTIM2 counter stopped when core is halted"]
    #[inline] pub fn dbg_lptim2_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DBG_LPTIM2_STOP != 0"]
    #[inline] pub fn test_dbg_lptim2_stop(&self) -> bool {
        self.dbg_lptim2_stop() != 0
    }

    #[doc="Sets the DBG_LPTIM2_STOP field."]
    #[inline] pub fn set_dbg_lptim2_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Apb1fzr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1fzr2(other)
    }
}

impl ::core::fmt::Display for Apb1fzr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1fzr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbg_lptim2_stop() != 0 { try!(write!(f, " dbg_lptim2_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 High Freeze Register CPU2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2apb1fzr2(pub u32);
impl C2apb1fzr2 {
    #[doc="LPTIM2 counter stopped when core is halted"]
    #[inline] pub fn dbg_lptim2_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DBG_LPTIM2_STOP != 0"]
    #[inline] pub fn test_dbg_lptim2_stop(&self) -> bool {
        self.dbg_lptim2_stop() != 0
    }

    #[doc="Sets the DBG_LPTIM2_STOP field."]
    #[inline] pub fn set_dbg_lptim2_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for C2apb1fzr2 {
    #[inline]
    fn from(other: u32) -> Self {
         C2apb1fzr2(other)
    }
}

impl ::core::fmt::Display for C2apb1fzr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2apb1fzr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbg_lptim2_stop() != 0 { try!(write!(f, " dbg_lptim2_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 Freeze Register CPU1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2fzr(pub u32);
impl Apb2fzr {
    #[doc="TIM1 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim1_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DBG_TIM1_STOP != 0"]
    #[inline] pub fn test_dbg_tim1_stop(&self) -> bool {
        self.dbg_tim1_stop() != 0
    }

    #[doc="Sets the DBG_TIM1_STOP field."]
    #[inline] pub fn set_dbg_tim1_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TIM16 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim16_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DBG_TIM16_STOP != 0"]
    #[inline] pub fn test_dbg_tim16_stop(&self) -> bool {
        self.dbg_tim16_stop() != 0
    }

    #[doc="Sets the DBG_TIM16_STOP field."]
    #[inline] pub fn set_dbg_tim16_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM17 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim17_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if DBG_TIM17_STOP != 0"]
    #[inline] pub fn test_dbg_tim17_stop(&self) -> bool {
        self.dbg_tim17_stop() != 0
    }

    #[doc="Sets the DBG_TIM17_STOP field."]
    #[inline] pub fn set_dbg_tim17_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for Apb2fzr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb2fzr(other)
    }
}

impl ::core::fmt::Display for Apb2fzr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb2fzr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbg_tim1_stop() != 0 { try!(write!(f, " dbg_tim1_stop"))}
        if self.dbg_tim16_stop() != 0 { try!(write!(f, " dbg_tim16_stop"))}
        if self.dbg_tim17_stop() != 0 { try!(write!(f, " dbg_tim17_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 Freeze Register CPU2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2apb2fzr(pub u32);
impl C2apb2fzr {
    #[doc="TIM1 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim1_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DBG_TIM1_STOP != 0"]
    #[inline] pub fn test_dbg_tim1_stop(&self) -> bool {
        self.dbg_tim1_stop() != 0
    }

    #[doc="Sets the DBG_TIM1_STOP field."]
    #[inline] pub fn set_dbg_tim1_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TIM16 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim16_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DBG_TIM16_STOP != 0"]
    #[inline] pub fn test_dbg_tim16_stop(&self) -> bool {
        self.dbg_tim16_stop() != 0
    }

    #[doc="Sets the DBG_TIM16_STOP field."]
    #[inline] pub fn set_dbg_tim16_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM17 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim17_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if DBG_TIM17_STOP != 0"]
    #[inline] pub fn test_dbg_tim17_stop(&self) -> bool {
        self.dbg_tim17_stop() != 0
    }

    #[doc="Sets the DBG_TIM17_STOP field."]
    #[inline] pub fn set_dbg_tim17_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for C2apb2fzr {
    #[inline]
    fn from(other: u32) -> Self {
         C2apb2fzr(other)
    }
}

impl ::core::fmt::Display for C2apb2fzr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2apb2fzr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbg_tim1_stop() != 0 { try!(write!(f, " dbg_tim1_stop"))}
        if self.dbg_tim16_stop() != 0 { try!(write!(f, " dbg_tim16_stop"))}
        if self.dbg_tim17_stop() != 0 { try!(write!(f, " dbg_tim17_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

