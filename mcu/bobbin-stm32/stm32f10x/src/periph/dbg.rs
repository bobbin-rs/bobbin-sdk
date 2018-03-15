//! Debug support

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Debug support"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DbgPeriph(pub usize);
impl DbgPeriph {
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

}

#[doc="DBGMCU_IDCODE"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Idcode(pub u32);
impl Idcode {
    #[doc="DEV_ID"]
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

    #[doc="REV_ID"]
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

#[doc="DBGMCU_CR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="DBG_SLEEP"]
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

    #[doc="DBG_STOP"]
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

    #[doc="DBG_STANDBY"]
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

    #[doc="TRACE_IOEN"]
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

    #[doc="TRACE_MODE"]
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

    #[doc="DBG_IWDG_STOP"]
    #[inline] pub fn dbg_iwdg_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DBG_IWDG_STOP != 0"]
    #[inline] pub fn test_dbg_iwdg_stop(&self) -> bool {
        self.dbg_iwdg_stop() != 0
    }

    #[doc="Sets the DBG_IWDG_STOP field."]
    #[inline] pub fn set_dbg_iwdg_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DBG_WWDG_STOP"]
    #[inline] pub fn dbg_wwdg_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DBG_WWDG_STOP != 0"]
    #[inline] pub fn test_dbg_wwdg_stop(&self) -> bool {
        self.dbg_wwdg_stop() != 0
    }

    #[doc="Sets the DBG_WWDG_STOP field."]
    #[inline] pub fn set_dbg_wwdg_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="DBG_TIM1_STOP"]
    #[inline] pub fn dbg_tim1_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DBG_TIM1_STOP != 0"]
    #[inline] pub fn test_dbg_tim1_stop(&self) -> bool {
        self.dbg_tim1_stop() != 0
    }

    #[doc="Sets the DBG_TIM1_STOP field."]
    #[inline] pub fn set_dbg_tim1_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="DBG_TIM2_STOP"]
    #[inline] pub fn dbg_tim2_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DBG_TIM2_STOP != 0"]
    #[inline] pub fn test_dbg_tim2_stop(&self) -> bool {
        self.dbg_tim2_stop() != 0
    }

    #[doc="Sets the DBG_TIM2_STOP field."]
    #[inline] pub fn set_dbg_tim2_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="DBG_TIM3_STOP"]
    #[inline] pub fn dbg_tim3_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DBG_TIM3_STOP != 0"]
    #[inline] pub fn test_dbg_tim3_stop(&self) -> bool {
        self.dbg_tim3_stop() != 0
    }

    #[doc="Sets the DBG_TIM3_STOP field."]
    #[inline] pub fn set_dbg_tim3_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DBG_TIM4_STOP"]
    #[inline] pub fn dbg_tim4_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DBG_TIM4_STOP != 0"]
    #[inline] pub fn test_dbg_tim4_stop(&self) -> bool {
        self.dbg_tim4_stop() != 0
    }

    #[doc="Sets the DBG_TIM4_STOP field."]
    #[inline] pub fn set_dbg_tim4_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="DBG_CAN1_STOP"]
    #[inline] pub fn dbg_can1_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DBG_CAN1_STOP != 0"]
    #[inline] pub fn test_dbg_can1_stop(&self) -> bool {
        self.dbg_can1_stop() != 0
    }

    #[doc="Sets the DBG_CAN1_STOP field."]
    #[inline] pub fn set_dbg_can1_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DBG_I2C1_SMBUS_TIMEOUT"]
    #[inline] pub fn dbg_i2c1_smbus_timeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DBG_I2C1_SMBUS_TIMEOUT != 0"]
    #[inline] pub fn test_dbg_i2c1_smbus_timeout(&self) -> bool {
        self.dbg_i2c1_smbus_timeout() != 0
    }

    #[doc="Sets the DBG_I2C1_SMBUS_TIMEOUT field."]
    #[inline] pub fn set_dbg_i2c1_smbus_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline] pub fn dbg_i2c2_smbus_timeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DBG_I2C2_SMBUS_TIMEOUT != 0"]
    #[inline] pub fn test_dbg_i2c2_smbus_timeout(&self) -> bool {
        self.dbg_i2c2_smbus_timeout() != 0
    }

    #[doc="Sets the DBG_I2C2_SMBUS_TIMEOUT field."]
    #[inline] pub fn set_dbg_i2c2_smbus_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DBG_TIM8_STOP"]
    #[inline] pub fn dbg_tim8_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DBG_TIM8_STOP != 0"]
    #[inline] pub fn test_dbg_tim8_stop(&self) -> bool {
        self.dbg_tim8_stop() != 0
    }

    #[doc="Sets the DBG_TIM8_STOP field."]
    #[inline] pub fn set_dbg_tim8_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="DBG_TIM5_STOP"]
    #[inline] pub fn dbg_tim5_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if DBG_TIM5_STOP != 0"]
    #[inline] pub fn test_dbg_tim5_stop(&self) -> bool {
        self.dbg_tim5_stop() != 0
    }

    #[doc="Sets the DBG_TIM5_STOP field."]
    #[inline] pub fn set_dbg_tim5_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="DBG_TIM6_STOP"]
    #[inline] pub fn dbg_tim6_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if DBG_TIM6_STOP != 0"]
    #[inline] pub fn test_dbg_tim6_stop(&self) -> bool {
        self.dbg_tim6_stop() != 0
    }

    #[doc="Sets the DBG_TIM6_STOP field."]
    #[inline] pub fn set_dbg_tim6_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DBG_TIM7_STOP"]
    #[inline] pub fn dbg_tim7_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if DBG_TIM7_STOP != 0"]
    #[inline] pub fn test_dbg_tim7_stop(&self) -> bool {
        self.dbg_tim7_stop() != 0
    }

    #[doc="Sets the DBG_TIM7_STOP field."]
    #[inline] pub fn set_dbg_tim7_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="DBG_CAN2_STOP"]
    #[inline] pub fn dbg_can2_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DBG_CAN2_STOP != 0"]
    #[inline] pub fn test_dbg_can2_stop(&self) -> bool {
        self.dbg_can2_stop() != 0
    }

    #[doc="Sets the DBG_CAN2_STOP field."]
    #[inline] pub fn set_dbg_can2_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
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
        if self.dbg_iwdg_stop() != 0 { try!(write!(f, " dbg_iwdg_stop"))}
        if self.dbg_wwdg_stop() != 0 { try!(write!(f, " dbg_wwdg_stop"))}
        if self.dbg_tim1_stop() != 0 { try!(write!(f, " dbg_tim1_stop"))}
        if self.dbg_tim2_stop() != 0 { try!(write!(f, " dbg_tim2_stop"))}
        if self.dbg_tim3_stop() != 0 { try!(write!(f, " dbg_tim3_stop"))}
        if self.dbg_tim4_stop() != 0 { try!(write!(f, " dbg_tim4_stop"))}
        if self.dbg_can1_stop() != 0 { try!(write!(f, " dbg_can1_stop"))}
        if self.dbg_i2c1_smbus_timeout() != 0 { try!(write!(f, " dbg_i2c1_smbus_timeout"))}
        if self.dbg_i2c2_smbus_timeout() != 0 { try!(write!(f, " dbg_i2c2_smbus_timeout"))}
        if self.dbg_tim8_stop() != 0 { try!(write!(f, " dbg_tim8_stop"))}
        if self.dbg_tim5_stop() != 0 { try!(write!(f, " dbg_tim5_stop"))}
        if self.dbg_tim6_stop() != 0 { try!(write!(f, " dbg_tim6_stop"))}
        if self.dbg_tim7_stop() != 0 { try!(write!(f, " dbg_tim7_stop"))}
        if self.dbg_can2_stop() != 0 { try!(write!(f, " dbg_can2_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}
