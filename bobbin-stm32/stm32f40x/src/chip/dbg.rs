//! Debug support
#[allow(unused_imports)] use bobbin_common::*;

periph!(DBG, Dbg, 0xe0042000);

#[doc="Debug support"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dbg(pub usize);
impl Dbg {
    #[doc="Get the *const pointer for the DBGMCU_IDCODE register."]
    #[inline] pub fn dbgmcu_idcode_ptr(&self) -> *const DbgmcuIdcode { 
        (self.0 + 0x0) as *const DbgmcuIdcode
    }

    #[doc="Get the *mut pointer for the DBGMCU_IDCODE register."]
    #[inline] pub fn dbgmcu_idcode_mut(&self) -> *mut DbgmcuIdcode { 
        (self.0 + 0x0) as *mut DbgmcuIdcode
    }

    #[doc="Read the DBGMCU_IDCODE register."]
    #[inline] pub fn dbgmcu_idcode(&self) -> DbgmcuIdcode { 
        unsafe {
            read_volatile((self.0 + 0x0) as *const DbgmcuIdcode)
        }
    }

    #[doc="Get the *const pointer for the DBGMCU_CR register."]
    #[inline] pub fn dbgmcu_cr_ptr(&self) -> *const DbgmcuCr { 
        (self.0 + 0x4) as *const DbgmcuCr
    }

    #[doc="Get the *mut pointer for the DBGMCU_CR register."]
    #[inline] pub fn dbgmcu_cr_mut(&self) -> *mut DbgmcuCr { 
        (self.0 + 0x4) as *mut DbgmcuCr
    }

    #[doc="Read the DBGMCU_CR register."]
    #[inline] pub fn dbgmcu_cr(&self) -> DbgmcuCr { 
        unsafe {
            read_volatile((self.0 + 0x4) as *const DbgmcuCr)
        }
    }

    #[doc="Write the DBGMCU_CR register."]
    #[inline] pub fn set_dbgmcu_cr<F: FnOnce(DbgmcuCr) -> DbgmcuCr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x4) as *mut DbgmcuCr, f(DbgmcuCr(0)));
        }
        self
    }

    #[doc="Modify the DBGMCU_CR register."]
    #[inline] pub fn with_dbgmcu_cr<F: FnOnce(DbgmcuCr) -> DbgmcuCr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x4) as *mut DbgmcuCr, f(self.dbgmcu_cr()));
        }
        self
    }

    #[doc="Get the *const pointer for the DBGMCU_APB1_FZ register."]
    #[inline] pub fn dbgmcu_apb1_fz_ptr(&self) -> *const DbgmcuApb1Fz { 
        (self.0 + 0x8) as *const DbgmcuApb1Fz
    }

    #[doc="Get the *mut pointer for the DBGMCU_APB1_FZ register."]
    #[inline] pub fn dbgmcu_apb1_fz_mut(&self) -> *mut DbgmcuApb1Fz { 
        (self.0 + 0x8) as *mut DbgmcuApb1Fz
    }

    #[doc="Read the DBGMCU_APB1_FZ register."]
    #[inline] pub fn dbgmcu_apb1_fz(&self) -> DbgmcuApb1Fz { 
        unsafe {
            read_volatile((self.0 + 0x8) as *const DbgmcuApb1Fz)
        }
    }

    #[doc="Write the DBGMCU_APB1_FZ register."]
    #[inline] pub fn set_dbgmcu_apb1_fz<F: FnOnce(DbgmcuApb1Fz) -> DbgmcuApb1Fz>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x8) as *mut DbgmcuApb1Fz, f(DbgmcuApb1Fz(0)));
        }
        self
    }

    #[doc="Modify the DBGMCU_APB1_FZ register."]
    #[inline] pub fn with_dbgmcu_apb1_fz<F: FnOnce(DbgmcuApb1Fz) -> DbgmcuApb1Fz>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0x8) as *mut DbgmcuApb1Fz, f(self.dbgmcu_apb1_fz()));
        }
        self
    }

    #[doc="Get the *const pointer for the DBGMCU_APB2_FZ register."]
    #[inline] pub fn dbgmcu_apb2_fz_ptr(&self) -> *const DbgmcuApb2Fz { 
        (self.0 + 0xc) as *const DbgmcuApb2Fz
    }

    #[doc="Get the *mut pointer for the DBGMCU_APB2_FZ register."]
    #[inline] pub fn dbgmcu_apb2_fz_mut(&self) -> *mut DbgmcuApb2Fz { 
        (self.0 + 0xc) as *mut DbgmcuApb2Fz
    }

    #[doc="Read the DBGMCU_APB2_FZ register."]
    #[inline] pub fn dbgmcu_apb2_fz(&self) -> DbgmcuApb2Fz { 
        unsafe {
            read_volatile((self.0 + 0xc) as *const DbgmcuApb2Fz)
        }
    }

    #[doc="Write the DBGMCU_APB2_FZ register."]
    #[inline] pub fn set_dbgmcu_apb2_fz<F: FnOnce(DbgmcuApb2Fz) -> DbgmcuApb2Fz>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xc) as *mut DbgmcuApb2Fz, f(DbgmcuApb2Fz(0)));
        }
        self
    }

    #[doc="Modify the DBGMCU_APB2_FZ register."]
    #[inline] pub fn with_dbgmcu_apb2_fz<F: FnOnce(DbgmcuApb2Fz) -> DbgmcuApb2Fz>(&self, f: F) -> &Self {
        unsafe {
            write_volatile((self.0 + 0xc) as *mut DbgmcuApb2Fz, f(self.dbgmcu_apb2_fz()));
        }
        self
    }

}

#[doc="IDCODE"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DbgmcuIdcode(pub u32);
impl DbgmcuIdcode {
    #[doc="DEV_ID"]
    #[inline] pub fn dev_id(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="DEV_ID"]
    #[inline] pub fn test_dev_id(&self) -> bool {
        self.dev_id() != 0
    }

    #[doc="DEV_ID"]
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

    #[doc="REV_ID"]
    #[inline] pub fn test_rev_id(&self) -> bool {
        self.rev_id() != 0
    }

    #[doc="REV_ID"]
    #[inline] pub fn set_rev_id<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for DbgmcuIdcode {
    #[inline]
    fn from(other: u32) -> Self {
         DbgmcuIdcode(other)
    }
}

impl ::core::fmt::Display for DbgmcuIdcode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for DbgmcuIdcode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dev_id() != 0 { try!(write!(f, " dev_id=0x{:x}", self.dev_id()))}
        if self.rev_id() != 0 { try!(write!(f, " rev_id=0x{:x}", self.rev_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DbgmcuCr(pub u32);
impl DbgmcuCr {
    #[doc="DBG_SLEEP"]
    #[inline] pub fn dbg_sleep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="DBG_SLEEP"]
    #[inline] pub fn test_dbg_sleep(&self) -> bool {
        self.dbg_sleep() != 0
    }

    #[doc="DBG_SLEEP"]
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

    #[doc="DBG_STOP"]
    #[inline] pub fn test_dbg_stop(&self) -> bool {
        self.dbg_stop() != 0
    }

    #[doc="DBG_STOP"]
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

    #[doc="DBG_STANDBY"]
    #[inline] pub fn test_dbg_standby(&self) -> bool {
        self.dbg_standby() != 0
    }

    #[doc="DBG_STANDBY"]
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

    #[doc="TRACE_IOEN"]
    #[inline] pub fn test_trace_ioen(&self) -> bool {
        self.trace_ioen() != 0
    }

    #[doc="TRACE_IOEN"]
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

    #[doc="TRACE_MODE"]
    #[inline] pub fn test_trace_mode(&self) -> bool {
        self.trace_mode() != 0
    }

    #[doc="TRACE_MODE"]
    #[inline] pub fn set_trace_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline] pub fn dbg_i2c2_smbus_timeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline] pub fn test_dbg_i2c2_smbus_timeout(&self) -> bool {
        self.dbg_i2c2_smbus_timeout() != 0
    }

    #[doc="DBG_I2C2_SMBUS_TIMEOUT"]
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

    #[doc="DBG_TIM8_STOP"]
    #[inline] pub fn test_dbg_tim8_stop(&self) -> bool {
        self.dbg_tim8_stop() != 0
    }

    #[doc="DBG_TIM8_STOP"]
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

    #[doc="DBG_TIM5_STOP"]
    #[inline] pub fn test_dbg_tim5_stop(&self) -> bool {
        self.dbg_tim5_stop() != 0
    }

    #[doc="DBG_TIM5_STOP"]
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

    #[doc="DBG_TIM6_STOP"]
    #[inline] pub fn test_dbg_tim6_stop(&self) -> bool {
        self.dbg_tim6_stop() != 0
    }

    #[doc="DBG_TIM6_STOP"]
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

    #[doc="DBG_TIM7_STOP"]
    #[inline] pub fn test_dbg_tim7_stop(&self) -> bool {
        self.dbg_tim7_stop() != 0
    }

    #[doc="DBG_TIM7_STOP"]
    #[inline] pub fn set_dbg_tim7_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl From<u32> for DbgmcuCr {
    #[inline]
    fn from(other: u32) -> Self {
         DbgmcuCr(other)
    }
}

impl ::core::fmt::Display for DbgmcuCr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for DbgmcuCr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbg_sleep() != 0 { try!(write!(f, " dbg_sleep"))}
        if self.dbg_stop() != 0 { try!(write!(f, " dbg_stop"))}
        if self.dbg_standby() != 0 { try!(write!(f, " dbg_standby"))}
        if self.trace_ioen() != 0 { try!(write!(f, " trace_ioen"))}
        if self.trace_mode() != 0 { try!(write!(f, " trace_mode=0x{:x}", self.trace_mode()))}
        if self.dbg_i2c2_smbus_timeout() != 0 { try!(write!(f, " dbg_i2c2_smbus_timeout"))}
        if self.dbg_tim8_stop() != 0 { try!(write!(f, " dbg_tim8_stop"))}
        if self.dbg_tim5_stop() != 0 { try!(write!(f, " dbg_tim5_stop"))}
        if self.dbg_tim6_stop() != 0 { try!(write!(f, " dbg_tim6_stop"))}
        if self.dbg_tim7_stop() != 0 { try!(write!(f, " dbg_tim7_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug MCU APB1 Freeze registe"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DbgmcuApb1Fz(pub u32);
impl DbgmcuApb1Fz {
    #[doc="DBG_TIM2_STOP"]
    #[inline] pub fn dbg_tim2_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="DBG_TIM2_STOP"]
    #[inline] pub fn test_dbg_tim2_stop(&self) -> bool {
        self.dbg_tim2_stop() != 0
    }

    #[doc="DBG_TIM2_STOP"]
    #[inline] pub fn set_dbg_tim2_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DBG_TIM3 _STOP"]
    #[inline] pub fn dbg_tim3_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="DBG_TIM3 _STOP"]
    #[inline] pub fn test_dbg_tim3_stop(&self) -> bool {
        self.dbg_tim3_stop() != 0
    }

    #[doc="DBG_TIM3 _STOP"]
    #[inline] pub fn set_dbg_tim3_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DBG_TIM4_STOP"]
    #[inline] pub fn dbg_tim4_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="DBG_TIM4_STOP"]
    #[inline] pub fn test_dbg_tim4_stop(&self) -> bool {
        self.dbg_tim4_stop() != 0
    }

    #[doc="DBG_TIM4_STOP"]
    #[inline] pub fn set_dbg_tim4_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DBG_TIM5_STOP"]
    #[inline] pub fn dbg_tim5_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="DBG_TIM5_STOP"]
    #[inline] pub fn test_dbg_tim5_stop(&self) -> bool {
        self.dbg_tim5_stop() != 0
    }

    #[doc="DBG_TIM5_STOP"]
    #[inline] pub fn set_dbg_tim5_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DBG_TIM6_STOP"]
    #[inline] pub fn dbg_tim6_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="DBG_TIM6_STOP"]
    #[inline] pub fn test_dbg_tim6_stop(&self) -> bool {
        self.dbg_tim6_stop() != 0
    }

    #[doc="DBG_TIM6_STOP"]
    #[inline] pub fn set_dbg_tim6_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DBG_TIM7_STOP"]
    #[inline] pub fn dbg_tim7_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="DBG_TIM7_STOP"]
    #[inline] pub fn test_dbg_tim7_stop(&self) -> bool {
        self.dbg_tim7_stop() != 0
    }

    #[doc="DBG_TIM7_STOP"]
    #[inline] pub fn set_dbg_tim7_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="DBG_TIM12_STOP"]
    #[inline] pub fn dbg_tim12_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="DBG_TIM12_STOP"]
    #[inline] pub fn test_dbg_tim12_stop(&self) -> bool {
        self.dbg_tim12_stop() != 0
    }

    #[doc="DBG_TIM12_STOP"]
    #[inline] pub fn set_dbg_tim12_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DBG_TIM13_STOP"]
    #[inline] pub fn dbg_tim13_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="DBG_TIM13_STOP"]
    #[inline] pub fn test_dbg_tim13_stop(&self) -> bool {
        self.dbg_tim13_stop() != 0
    }

    #[doc="DBG_TIM13_STOP"]
    #[inline] pub fn set_dbg_tim13_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DBG_TIM14_STOP"]
    #[inline] pub fn dbg_tim14_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="DBG_TIM14_STOP"]
    #[inline] pub fn test_dbg_tim14_stop(&self) -> bool {
        self.dbg_tim14_stop() != 0
    }

    #[doc="DBG_TIM14_STOP"]
    #[inline] pub fn set_dbg_tim14_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DBG_WWDG_STOP"]
    #[inline] pub fn dbg_wwdg_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="DBG_WWDG_STOP"]
    #[inline] pub fn test_dbg_wwdg_stop(&self) -> bool {
        self.dbg_wwdg_stop() != 0
    }

    #[doc="DBG_WWDG_STOP"]
    #[inline] pub fn set_dbg_wwdg_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="DBG_IWDEG_STOP"]
    #[inline] pub fn dbg_iwdeg_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="DBG_IWDEG_STOP"]
    #[inline] pub fn test_dbg_iwdeg_stop(&self) -> bool {
        self.dbg_iwdeg_stop() != 0
    }

    #[doc="DBG_IWDEG_STOP"]
    #[inline] pub fn set_dbg_iwdeg_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DBG_J2C1_SMBUS_TIMEOUT"]
    #[inline] pub fn dbg_j2c1_smbus_timeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="DBG_J2C1_SMBUS_TIMEOUT"]
    #[inline] pub fn test_dbg_j2c1_smbus_timeout(&self) -> bool {
        self.dbg_j2c1_smbus_timeout() != 0
    }

    #[doc="DBG_J2C1_SMBUS_TIMEOUT"]
    #[inline] pub fn set_dbg_j2c1_smbus_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="DBG_J2C2_SMBUS_TIMEOUT"]
    #[inline] pub fn dbg_j2c2_smbus_timeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="DBG_J2C2_SMBUS_TIMEOUT"]
    #[inline] pub fn test_dbg_j2c2_smbus_timeout(&self) -> bool {
        self.dbg_j2c2_smbus_timeout() != 0
    }

    #[doc="DBG_J2C2_SMBUS_TIMEOUT"]
    #[inline] pub fn set_dbg_j2c2_smbus_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="DBG_J2C3SMBUS_TIMEOUT"]
    #[inline] pub fn dbg_j2c3smbus_timeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="DBG_J2C3SMBUS_TIMEOUT"]
    #[inline] pub fn test_dbg_j2c3smbus_timeout(&self) -> bool {
        self.dbg_j2c3smbus_timeout() != 0
    }

    #[doc="DBG_J2C3SMBUS_TIMEOUT"]
    #[inline] pub fn set_dbg_j2c3smbus_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="DBG_CAN1_STOP"]
    #[inline] pub fn dbg_can1_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="DBG_CAN1_STOP"]
    #[inline] pub fn test_dbg_can1_stop(&self) -> bool {
        self.dbg_can1_stop() != 0
    }

    #[doc="DBG_CAN1_STOP"]
    #[inline] pub fn set_dbg_can1_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DBG_CAN2_STOP"]
    #[inline] pub fn dbg_can2_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="DBG_CAN2_STOP"]
    #[inline] pub fn test_dbg_can2_stop(&self) -> bool {
        self.dbg_can2_stop() != 0
    }

    #[doc="DBG_CAN2_STOP"]
    #[inline] pub fn set_dbg_can2_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for DbgmcuApb1Fz {
    #[inline]
    fn from(other: u32) -> Self {
         DbgmcuApb1Fz(other)
    }
}

impl ::core::fmt::Display for DbgmcuApb1Fz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for DbgmcuApb1Fz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbg_tim2_stop() != 0 { try!(write!(f, " dbg_tim2_stop"))}
        if self.dbg_tim3_stop() != 0 { try!(write!(f, " dbg_tim3_stop"))}
        if self.dbg_tim4_stop() != 0 { try!(write!(f, " dbg_tim4_stop"))}
        if self.dbg_tim5_stop() != 0 { try!(write!(f, " dbg_tim5_stop"))}
        if self.dbg_tim6_stop() != 0 { try!(write!(f, " dbg_tim6_stop"))}
        if self.dbg_tim7_stop() != 0 { try!(write!(f, " dbg_tim7_stop"))}
        if self.dbg_tim12_stop() != 0 { try!(write!(f, " dbg_tim12_stop"))}
        if self.dbg_tim13_stop() != 0 { try!(write!(f, " dbg_tim13_stop"))}
        if self.dbg_tim14_stop() != 0 { try!(write!(f, " dbg_tim14_stop"))}
        if self.dbg_wwdg_stop() != 0 { try!(write!(f, " dbg_wwdg_stop"))}
        if self.dbg_iwdeg_stop() != 0 { try!(write!(f, " dbg_iwdeg_stop"))}
        if self.dbg_j2c1_smbus_timeout() != 0 { try!(write!(f, " dbg_j2c1_smbus_timeout"))}
        if self.dbg_j2c2_smbus_timeout() != 0 { try!(write!(f, " dbg_j2c2_smbus_timeout"))}
        if self.dbg_j2c3smbus_timeout() != 0 { try!(write!(f, " dbg_j2c3smbus_timeout"))}
        if self.dbg_can1_stop() != 0 { try!(write!(f, " dbg_can1_stop"))}
        if self.dbg_can2_stop() != 0 { try!(write!(f, " dbg_can2_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug MCU APB2 Freeze registe"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DbgmcuApb2Fz(pub u32);
impl DbgmcuApb2Fz {
    #[doc="TIM1 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim1_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="TIM1 counter stopped when core is halted"]
    #[inline] pub fn test_dbg_tim1_stop(&self) -> bool {
        self.dbg_tim1_stop() != 0
    }

    #[doc="TIM1 counter stopped when core is halted"]
    #[inline] pub fn set_dbg_tim1_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TIM8 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim8_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="TIM8 counter stopped when core is halted"]
    #[inline] pub fn test_dbg_tim8_stop(&self) -> bool {
        self.dbg_tim8_stop() != 0
    }

    #[doc="TIM8 counter stopped when core is halted"]
    #[inline] pub fn set_dbg_tim8_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TIM9 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim9_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="TIM9 counter stopped when core is halted"]
    #[inline] pub fn test_dbg_tim9_stop(&self) -> bool {
        self.dbg_tim9_stop() != 0
    }

    #[doc="TIM9 counter stopped when core is halted"]
    #[inline] pub fn set_dbg_tim9_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="TIM10 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim10_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="TIM10 counter stopped when core is halted"]
    #[inline] pub fn test_dbg_tim10_stop(&self) -> bool {
        self.dbg_tim10_stop() != 0
    }

    #[doc="TIM10 counter stopped when core is halted"]
    #[inline] pub fn set_dbg_tim10_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM11 counter stopped when core is halted"]
    #[inline] pub fn dbg_tim11_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="TIM11 counter stopped when core is halted"]
    #[inline] pub fn test_dbg_tim11_stop(&self) -> bool {
        self.dbg_tim11_stop() != 0
    }

    #[doc="TIM11 counter stopped when core is halted"]
    #[inline] pub fn set_dbg_tim11_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for DbgmcuApb2Fz {
    #[inline]
    fn from(other: u32) -> Self {
         DbgmcuApb2Fz(other)
    }
}

impl ::core::fmt::Display for DbgmcuApb2Fz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for DbgmcuApb2Fz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbg_tim1_stop() != 0 { try!(write!(f, " dbg_tim1_stop"))}
        if self.dbg_tim8_stop() != 0 { try!(write!(f, " dbg_tim8_stop"))}
        if self.dbg_tim9_stop() != 0 { try!(write!(f, " dbg_tim9_stop"))}
        if self.dbg_tim10_stop() != 0 { try!(write!(f, " dbg_tim10_stop"))}
        if self.dbg_tim11_stop() != 0 { try!(write!(f, " dbg_tim11_stop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


