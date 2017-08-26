//! Debug support
#[allow(unused_imports)] use bobbin_common::*;

periph!(DbgPeriph, DBG, Dbg, 0xe0042000);

#[doc="Debug support"]
pub trait DbgPeriph : Base {
#[doc="Get the *const pointer for the IDCODE register."]
   #[inline] fn idcode_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the IDCODE register."]
   #[inline] fn idcode_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the IDCODE register."]
   #[inline] fn idcode(&self) -> Idcode { 
      unsafe {
         Idcode(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      }
   }

#[doc="Get the *const pointer for the CR register."]
   #[inline] fn cr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the CR register."]
   #[inline] fn cr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the CR register."]
   #[inline] fn cr(&self) -> Cr { 
      unsafe {
         Cr(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      }
   }
#[doc="Write the CR register."]
   #[inline] fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let value = f(Cr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR register."]
   #[inline] fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cr(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the APB1_FZ register."]
   #[inline] fn apb1_fz_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the APB1_FZ register."]
   #[inline] fn apb1_fz_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the APB1_FZ register."]
   #[inline] fn apb1_fz(&self) -> Apb1Fz { 
      unsafe {
         Apb1Fz(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      }
   }
#[doc="Write the APB1_FZ register."]
   #[inline] fn set_apb1_fz<F: FnOnce(Apb1Fz) -> Apb1Fz>(&self, f: F) -> &Self {
      let value = f(Apb1Fz(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the APB1_FZ register."]
   #[inline] fn with_apb1_fz<F: FnOnce(Apb1Fz) -> Apb1Fz>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Apb1Fz(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the APB2_FZ register."]
   #[inline] fn apb2_fz_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the APB2_FZ register."]
   #[inline] fn apb2_fz_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Read the APB2_FZ register."]
   #[inline] fn apb2_fz(&self) -> Apb2Fz { 
      unsafe {
         Apb2Fz(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      }
   }
#[doc="Write the APB2_FZ register."]
   #[inline] fn set_apb2_fz<F: FnOnce(Apb2Fz) -> Apb2Fz>(&self, f: F) -> &Self {
      let value = f(Apb2Fz(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the APB2_FZ register."]
   #[inline] fn with_apb2_fz<F: FnOnce(Apb2Fz) -> Apb2Fz>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Apb2Fz(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }

}

#[doc="IDCODE"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Idcode(pub u32);
impl Idcode {
#[doc="DEV_ID"]
   #[inline] pub fn dev_id(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
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
   #[inline] pub fn set_rev_id<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 16);
      self.0 |= value << 16;
      self
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
#[doc="Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="SLEEP"]
   #[inline] pub fn sleep(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="SLEEP"]
   #[inline] pub fn set_sleep<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="STOP"]
   #[inline] pub fn stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="STOP"]
   #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="STANDBY"]
   #[inline] pub fn standby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="STANDBY"]
   #[inline] pub fn set_standby<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
   #[inline] pub fn set_trace_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="I2C2_SMBUS_TIMEOUT"]
   #[inline] pub fn i2c2_smbus_timeout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="I2C2_SMBUS_TIMEOUT"]
   #[inline] pub fn set_i2c2_smbus_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="TIM8_STOP"]
   #[inline] pub fn tim8_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="TIM8_STOP"]
   #[inline] pub fn set_tim8_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="TIM5_STOP"]
   #[inline] pub fn tim5_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="TIM5_STOP"]
   #[inline] pub fn set_tim5_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="TIM6_STOP"]
   #[inline] pub fn tim6_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="TIM6_STOP"]
   #[inline] pub fn set_tim6_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="TIM7_STOP"]
   #[inline] pub fn tim7_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="TIM7_STOP"]
   #[inline] pub fn set_tim7_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
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
      if self.sleep() != 0 { try!(write!(f, " sleep"))}
      if self.stop() != 0 { try!(write!(f, " stop"))}
      if self.standby() != 0 { try!(write!(f, " standby"))}
      if self.trace_ioen() != 0 { try!(write!(f, " trace_ioen"))}
      if self.trace_mode() != 0 { try!(write!(f, " trace_mode=0x{:x}", self.trace_mode()))}
      if self.i2c2_smbus_timeout() != 0 { try!(write!(f, " i2c2_smbus_timeout"))}
      if self.tim8_stop() != 0 { try!(write!(f, " tim8_stop"))}
      if self.tim5_stop() != 0 { try!(write!(f, " tim5_stop"))}
      if self.tim6_stop() != 0 { try!(write!(f, " tim6_stop"))}
      if self.tim7_stop() != 0 { try!(write!(f, " tim7_stop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Debug MCU APB1 Freeze registe"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Apb1Fz(pub u32);
impl Apb1Fz {
#[doc="TIM2_STOP"]
   #[inline] pub fn tim2_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="TIM2_STOP"]
   #[inline] pub fn set_tim2_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="TIM3 _STOP"]
   #[inline] pub fn tim3_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="TIM3 _STOP"]
   #[inline] pub fn set_tim3_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="TIM4_STOP"]
   #[inline] pub fn tim4_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="TIM4_STOP"]
   #[inline] pub fn set_tim4_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="TIM5_STOP"]
   #[inline] pub fn tim5_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="TIM5_STOP"]
   #[inline] pub fn set_tim5_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="TIM6_STOP"]
   #[inline] pub fn tim6_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="TIM6_STOP"]
   #[inline] pub fn set_tim6_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="TIM7_STOP"]
   #[inline] pub fn tim7_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="TIM7_STOP"]
   #[inline] pub fn set_tim7_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="TIM12_STOP"]
   #[inline] pub fn tim12_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="TIM12_STOP"]
   #[inline] pub fn set_tim12_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="TIM13_STOP"]
   #[inline] pub fn tim13_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="TIM13_STOP"]
   #[inline] pub fn set_tim13_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="TIM14_STOP"]
   #[inline] pub fn tim14_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="TIM14_STOP"]
   #[inline] pub fn set_tim14_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="WWDG_STOP"]
   #[inline] pub fn wwdg_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="WWDG_STOP"]
   #[inline] pub fn set_wwdg_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="IWDEG_STOP"]
   #[inline] pub fn iwdeg_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="IWDEG_STOP"]
   #[inline] pub fn set_iwdeg_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="J2C1_SMBUS_TIMEOUT"]
   #[inline] pub fn j2c1_smbus_timeout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="J2C1_SMBUS_TIMEOUT"]
   #[inline] pub fn set_j2c1_smbus_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="J2C2_SMBUS_TIMEOUT"]
   #[inline] pub fn j2c2_smbus_timeout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="J2C2_SMBUS_TIMEOUT"]
   #[inline] pub fn set_j2c2_smbus_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="J2C3SMBUS_TIMEOUT"]
   #[inline] pub fn j2c3smbus_timeout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="J2C3SMBUS_TIMEOUT"]
   #[inline] pub fn set_j2c3smbus_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

#[doc="CAN1_STOP"]
   #[inline] pub fn can1_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="CAN1_STOP"]
   #[inline] pub fn set_can1_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="CAN2_STOP"]
   #[inline] pub fn can2_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
   }
#[doc="CAN2_STOP"]
   #[inline] pub fn set_can2_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 26);
      self.0 |= value << 26;
      self
   }

}
impl ::core::fmt::Display for Apb1Fz {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb1Fz {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tim2_stop() != 0 { try!(write!(f, " tim2_stop"))}
      if self.tim3_stop() != 0 { try!(write!(f, " tim3_stop"))}
      if self.tim4_stop() != 0 { try!(write!(f, " tim4_stop"))}
      if self.tim5_stop() != 0 { try!(write!(f, " tim5_stop"))}
      if self.tim6_stop() != 0 { try!(write!(f, " tim6_stop"))}
      if self.tim7_stop() != 0 { try!(write!(f, " tim7_stop"))}
      if self.tim12_stop() != 0 { try!(write!(f, " tim12_stop"))}
      if self.tim13_stop() != 0 { try!(write!(f, " tim13_stop"))}
      if self.tim14_stop() != 0 { try!(write!(f, " tim14_stop"))}
      if self.wwdg_stop() != 0 { try!(write!(f, " wwdg_stop"))}
      if self.iwdeg_stop() != 0 { try!(write!(f, " iwdeg_stop"))}
      if self.j2c1_smbus_timeout() != 0 { try!(write!(f, " j2c1_smbus_timeout"))}
      if self.j2c2_smbus_timeout() != 0 { try!(write!(f, " j2c2_smbus_timeout"))}
      if self.j2c3smbus_timeout() != 0 { try!(write!(f, " j2c3smbus_timeout"))}
      if self.can1_stop() != 0 { try!(write!(f, " can1_stop"))}
      if self.can2_stop() != 0 { try!(write!(f, " can2_stop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Debug MCU APB2 Freeze registe"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Apb2Fz(pub u32);
impl Apb2Fz {
#[doc="TIM1 counter stopped when core is halted"]
   #[inline] pub fn tim1_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="TIM1 counter stopped when core is halted"]
   #[inline] pub fn set_tim1_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="TIM8 counter stopped when core is halted"]
   #[inline] pub fn tim8_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="TIM8 counter stopped when core is halted"]
   #[inline] pub fn set_tim8_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="TIM9 counter stopped when core is halted"]
   #[inline] pub fn tim9_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="TIM9 counter stopped when core is halted"]
   #[inline] pub fn set_tim9_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="TIM10 counter stopped when core is halted"]
   #[inline] pub fn tim10_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="TIM10 counter stopped when core is halted"]
   #[inline] pub fn set_tim10_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="TIM11 counter stopped when core is halted"]
   #[inline] pub fn tim11_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="TIM11 counter stopped when core is halted"]
   #[inline] pub fn set_tim11_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

}
impl ::core::fmt::Display for Apb2Fz {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Apb2Fz {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tim1_stop() != 0 { try!(write!(f, " tim1_stop"))}
      if self.tim8_stop() != 0 { try!(write!(f, " tim8_stop"))}
      if self.tim9_stop() != 0 { try!(write!(f, " tim9_stop"))}
      if self.tim10_stop() != 0 { try!(write!(f, " tim10_stop"))}
      if self.tim11_stop() != 0 { try!(write!(f, " tim11_stop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

