#[allow(unused_imports)] use bobbin_common::*;

periph!( DBGMCU, Dbgmcu, _DBGMCU, DbgmcuPeriph, 0xe0042000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DBGMCU Peripheral"]
pub struct DbgmcuPeriph(pub usize); 



impl DbgmcuPeriph {
    #[doc="Get the *mut pointer for the IDCODE register."]
    #[inline] pub fn idcode_mut(&self) -> *mut Idcode { 
        (self.0 + 0x0) as *mut Idcode
    }

    #[doc="Get the *const pointer for the IDCODE register."]
    #[inline] pub fn idcode_ptr(&self) -> *const Idcode { 
           self.idcode_mut()
    }

    #[doc="Read the IDCODE register."]
    #[inline] pub fn idcode(&self) -> Idcode { 
        unsafe {
            read_volatile(self.idcode_ptr())
        }
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        (self.0 + 0x4) as *mut Cr
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
           self.cr_mut()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            read_volatile(self.cr_ptr())
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(Cr(0)));
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(self.cr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB1FZR1 register."]
    #[inline] pub fn apb1fzr1_mut(&self) -> *mut Apb1fzr1 { 
        (self.0 + 0x8) as *mut Apb1fzr1
    }

    #[doc="Get the *const pointer for the APB1FZR1 register."]
    #[inline] pub fn apb1fzr1_ptr(&self) -> *const Apb1fzr1 { 
           self.apb1fzr1_mut()
    }

    #[doc="Read the APB1FZR1 register."]
    #[inline] pub fn apb1fzr1(&self) -> Apb1fzr1 { 
        unsafe {
            read_volatile(self.apb1fzr1_ptr())
        }
    }

    #[doc="Write the APB1FZR1 register."]
    #[inline] pub fn set_apb1fzr1<F: FnOnce(Apb1fzr1) -> Apb1fzr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1fzr1_mut(), f(Apb1fzr1(0)));
        }
        self
    }

    #[doc="Modify the APB1FZR1 register."]
    #[inline] pub fn with_apb1fzr1<F: FnOnce(Apb1fzr1) -> Apb1fzr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1fzr1_mut(), f(self.apb1fzr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB1FZR2 register."]
    #[inline] pub fn apb1fzr2_mut(&self) -> *mut Apb1fzr2 { 
        (self.0 + 0xc) as *mut Apb1fzr2
    }

    #[doc="Get the *const pointer for the APB1FZR2 register."]
    #[inline] pub fn apb1fzr2_ptr(&self) -> *const Apb1fzr2 { 
           self.apb1fzr2_mut()
    }

    #[doc="Read the APB1FZR2 register."]
    #[inline] pub fn apb1fzr2(&self) -> Apb1fzr2 { 
        unsafe {
            read_volatile(self.apb1fzr2_ptr())
        }
    }

    #[doc="Write the APB1FZR2 register."]
    #[inline] pub fn set_apb1fzr2<F: FnOnce(Apb1fzr2) -> Apb1fzr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1fzr2_mut(), f(Apb1fzr2(0)));
        }
        self
    }

    #[doc="Modify the APB1FZR2 register."]
    #[inline] pub fn with_apb1fzr2<F: FnOnce(Apb1fzr2) -> Apb1fzr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1fzr2_mut(), f(self.apb1fzr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB2FZR register."]
    #[inline] pub fn apb2fzr_mut(&self) -> *mut Apb2fzr { 
        (self.0 + 0x10) as *mut Apb2fzr
    }

    #[doc="Get the *const pointer for the APB2FZR register."]
    #[inline] pub fn apb2fzr_ptr(&self) -> *const Apb2fzr { 
           self.apb2fzr_mut()
    }

    #[doc="Read the APB2FZR register."]
    #[inline] pub fn apb2fzr(&self) -> Apb2fzr { 
        unsafe {
            read_volatile(self.apb2fzr_ptr())
        }
    }

    #[doc="Write the APB2FZR register."]
    #[inline] pub fn set_apb2fzr<F: FnOnce(Apb2fzr) -> Apb2fzr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2fzr_mut(), f(Apb2fzr(0)));
        }
        self
    }

    #[doc="Modify the APB2FZR register."]
    #[inline] pub fn with_apb2fzr<F: FnOnce(Apb2fzr) -> Apb2fzr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2fzr_mut(), f(self.apb2fzr()));
        }
        self
    }

}

#[doc="DBGMCU_IDCODE"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Idcode(pub u32);
impl Idcode {
    #[doc="Device identifier"]
    #[inline] pub fn dev_id(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DEV_ID != 0"]
    #[inline] pub fn test_dev_id(&self) -> bool {
        self.dev_id() != 0
    }

    #[doc="Sets the DEV_ID field."]
    #[inline] pub fn set_dev_id<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Revision identifie"]
    #[inline] pub fn rev_id(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if REV_ID != 0"]
    #[inline] pub fn test_rev_id(&self) -> bool {
        self.rev_id() != 0
    }

    #[doc="Sets the REV_ID field."]
    #[inline] pub fn set_rev_id<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
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

#[doc="Debug MCU configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Debug Sleep mode"]
    #[inline] pub fn dbg_sleep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBG_SLEEP != 0"]
    #[inline] pub fn test_dbg_sleep(&self) -> bool {
        self.dbg_sleep() != 0
    }

    #[doc="Sets the DBG_SLEEP field."]
    #[inline] pub fn set_dbg_sleep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Debug Stop mode"]
    #[inline] pub fn dbg_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DBG_STOP != 0"]
    #[inline] pub fn test_dbg_stop(&self) -> bool {
        self.dbg_stop() != 0
    }

    #[doc="Sets the DBG_STOP field."]
    #[inline] pub fn set_dbg_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Debug Standby mode"]
    #[inline] pub fn dbg_standby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DBG_STANDBY != 0"]
    #[inline] pub fn test_dbg_standby(&self) -> bool {
        self.dbg_standby() != 0
    }

    #[doc="Sets the DBG_STANDBY field."]
    #[inline] pub fn set_dbg_standby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Trace pin assignment control"]
    #[inline] pub fn trace_ioen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TRACE_IOEN != 0"]
    #[inline] pub fn test_trace_ioen(&self) -> bool {
        self.trace_ioen() != 0
    }

    #[doc="Sets the TRACE_IOEN field."]
    #[inline] pub fn set_trace_ioen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Trace pin assignment control"]
    #[inline] pub fn trace_mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if TRACE_MODE != 0"]
    #[inline] pub fn test_trace_mode(&self) -> bool {
        self.trace_mode() != 0
    }

    #[doc="Sets the TRACE_MODE field."]
    #[inline] pub fn set_trace_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
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
        if self.trace_mode() != 0 { try!(write!(f, " trace_mode=0x{:x}", self.trace_mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug MCU APB1 freeze register1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1fzr1(pub u32);
impl Apb1fzr1 {
    #[doc="TIM2 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim2_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBG_TIM2_STOP != 0"]
    #[inline] pub fn test_dbg_tim2_stop(&self) -> bool {
        self.dbg_tim2_stop() != 0
    }

    #[doc="Sets the DBG_TIM2_STOP field."]
    #[inline] pub fn set_dbg_tim2_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TIM6 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim6_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DBG_TIM6_STOP != 0"]
    #[inline] pub fn test_dbg_tim6_stop(&self) -> bool {
        self.dbg_tim6_stop() != 0
    }

    #[doc="Sets the DBG_TIM6_STOP field."]
    #[inline] pub fn set_dbg_tim6_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM7 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim7_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DBG_TIM7_STOP != 0"]
    #[inline] pub fn test_dbg_tim7_stop(&self) -> bool {
        self.dbg_tim7_stop() != 0
    }

    #[doc="Sets the DBG_TIM7_STOP field."]
    #[inline] pub fn set_dbg_tim7_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RTC counter stopped when core is halted"]
    #[inline] pub fn dbg_rtc_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DBG_RTC_STOP != 0"]
    #[inline] pub fn test_dbg_rtc_stop(&self) -> bool {
        self.dbg_rtc_stop() != 0
    }

    #[doc="Sets the DBG_RTC_STOP field."]
    #[inline] pub fn set_dbg_rtc_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Window watchdog counter stopped when core is halted"]
    #[inline] pub fn dbg_wwdg_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DBG_WWDG_STOP != 0"]
    #[inline] pub fn test_dbg_wwdg_stop(&self) -> bool {
        self.dbg_wwdg_stop() != 0
    }

    #[doc="Sets the DBG_WWDG_STOP field."]
    #[inline] pub fn set_dbg_wwdg_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Independent watchdog counter stopped when core is halted"]
    #[inline] pub fn dbg_iwdg_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DBG_IWDG_STOP != 0"]
    #[inline] pub fn test_dbg_iwdg_stop(&self) -> bool {
        self.dbg_iwdg_stop() != 0
    }

    #[doc="Sets the DBG_IWDG_STOP field."]
    #[inline] pub fn set_dbg_iwdg_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="I2C1 SMBUS timeout counter stopped when core is halted"]
    #[inline] pub fn dbg_i2c1_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DBG_I2C1_STOP != 0"]
    #[inline] pub fn test_dbg_i2c1_stop(&self) -> bool {
        self.dbg_i2c1_stop() != 0
    }

    #[doc="Sets the DBG_I2C1_STOP field."]
    #[inline] pub fn set_dbg_i2c1_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="I2C2 SMBUS timeout counter stopped when core is halted"]
    #[inline] pub fn dbg_i2c2_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DBG_I2C2_STOP != 0"]
    #[inline] pub fn test_dbg_i2c2_stop(&self) -> bool {
        self.dbg_i2c2_stop() != 0
    }

    #[doc="Sets the DBG_I2C2_STOP field."]
    #[inline] pub fn set_dbg_i2c2_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="I2C3 SMBUS timeout counter stopped when core is halted"]
    #[inline] pub fn dbg_i2c3_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if DBG_I2C3_STOP != 0"]
    #[inline] pub fn test_dbg_i2c3_stop(&self) -> bool {
        self.dbg_i2c3_stop() != 0
    }

    #[doc="Sets the DBG_I2C3_STOP field."]
    #[inline] pub fn set_dbg_i2c3_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="bxCAN stopped when core is halted"]
    #[inline] pub fn dbg_can_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DBG_CAN_STOP != 0"]
    #[inline] pub fn test_dbg_can_stop(&self) -> bool {
        self.dbg_can_stop() != 0
    }

    #[doc="Sets the DBG_CAN_STOP field."]
    #[inline] pub fn set_dbg_can_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="LPTIM1 counter stopped when core is halted"]
    #[inline] pub fn dbg_lptim1_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if DBG_LPTIM1_STOP != 0"]
    #[inline] pub fn test_dbg_lptim1_stop(&self) -> bool {
        self.dbg_lptim1_stop() != 0
    }

    #[doc="Sets the DBG_LPTIM1_STOP field."]
    #[inline] pub fn set_dbg_lptim1_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.dbg_tim2_stop() != 0 { try!(write!(f, " dbg_tim2_stop"))}
        if self.dbg_tim6_stop() != 0 { try!(write!(f, " dbg_tim6_stop"))}
        if self.dbg_tim7_stop() != 0 { try!(write!(f, " dbg_tim7_stop"))}
        if self.dbg_rtc_stop() != 0 { try!(write!(f, " dbg_rtc_stop"))}
        if self.dbg_wwdg_stop() != 0 { try!(write!(f, " dbg_wwdg_stop"))}
        if self.dbg_iwdg_stop() != 0 { try!(write!(f, " dbg_iwdg_stop"))}
        if self.dbg_i2c1_stop() != 0 { try!(write!(f, " dbg_i2c1_stop"))}
        if self.dbg_i2c2_stop() != 0 { try!(write!(f, " dbg_i2c2_stop"))}
        if self.dbg_i2c3_stop() != 0 { try!(write!(f, " dbg_i2c3_stop"))}
        if self.dbg_can_stop() != 0 { try!(write!(f, " dbg_can_stop"))}
        if self.dbg_lptim1_stop() != 0 { try!(write!(f, " dbg_lptim1_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug MCU APB1 freeze register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1fzr2(pub u32);
impl Apb1fzr2 {
    #[doc="LPTIM2 counter stopped when core is halted"]
    #[inline] pub fn dbg_lptim2_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DBG_LPTIM2_STOP != 0"]
    #[inline] pub fn test_dbg_lptim2_stop(&self) -> bool {
        self.dbg_lptim2_stop() != 0
    }

    #[doc="Sets the DBG_LPTIM2_STOP field."]
    #[inline] pub fn set_dbg_lptim2_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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

#[doc="Debug MCU APB2 freeze register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2fzr(pub u32);
impl Apb2fzr {
    #[doc="TIM1 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim1_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DBG_TIM1_STOP != 0"]
    #[inline] pub fn test_dbg_tim1_stop(&self) -> bool {
        self.dbg_tim1_stop() != 0
    }

    #[doc="Sets the DBG_TIM1_STOP field."]
    #[inline] pub fn set_dbg_tim1_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TIM15 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim15_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DBG_TIM15_STOP != 0"]
    #[inline] pub fn test_dbg_tim15_stop(&self) -> bool {
        self.dbg_tim15_stop() != 0
    }

    #[doc="Sets the DBG_TIM15_STOP field."]
    #[inline] pub fn set_dbg_tim15_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="TIM16 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim16_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DBG_TIM16_STOP != 0"]
    #[inline] pub fn test_dbg_tim16_stop(&self) -> bool {
        self.dbg_tim16_stop() != 0
    }

    #[doc="Sets the DBG_TIM16_STOP field."]
    #[inline] pub fn set_dbg_tim16_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
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
        if self.dbg_tim15_stop() != 0 { try!(write!(f, " dbg_tim15_stop"))}
        if self.dbg_tim16_stop() != 0 { try!(write!(f, " dbg_tim16_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


