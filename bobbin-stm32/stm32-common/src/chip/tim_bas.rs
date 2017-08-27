#[allow(unused_imports)] use bobbin_common::*;



pub trait TimBasPeriph : Base {
#[doc="Get the *const pointer for the CR1 register."]
   #[inline] fn cr1_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the CR1 register."]
   #[inline] fn cr1_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the CR1 register."]
   #[inline] fn cr1(&self) -> Cr1 { 
      unsafe {
         Cr1(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      }
   }
#[doc="Write the CR1 register."]
   #[inline] fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
      let value = f(Cr1(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR1 register."]
   #[inline] fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cr1(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CR2 register."]
   #[inline] fn cr2_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the CR2 register."]
   #[inline] fn cr2_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the CR2 register."]
   #[inline] fn cr2(&self) -> Cr2 { 
      unsafe {
         Cr2(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      }
   }
#[doc="Write the CR2 register."]
   #[inline] fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
      let value = f(Cr2(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR2 register."]
   #[inline] fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cr2(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DIER register."]
   #[inline] fn dier_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the DIER register."]
   #[inline] fn dier_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Read the DIER register."]
   #[inline] fn dier(&self) -> Dier { 
      unsafe {
         Dier(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      }
   }
#[doc="Write the DIER register."]
   #[inline] fn set_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &Self {
      let value = f(Dier(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DIER register."]
   #[inline] fn with_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dier(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SR register."]
   #[inline] fn sr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Get the *mut pointer for the SR register."]
   #[inline] fn sr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Read the SR register."]
   #[inline] fn sr(&self) -> Sr { 
      unsafe {
         Sr(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      }
   }
#[doc="Write the SR register."]
   #[inline] fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let value = f(Sr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SR register."]
   #[inline] fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Sr(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EGR register."]
   #[inline] fn egr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Get the *mut pointer for the EGR register."]
   #[inline] fn egr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Write the EGR register."]
   #[inline] fn set_egr<F: FnOnce(Egr) -> Egr>(&self, f: F) -> &Self {
      let value = f(Egr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CNT register."]
   #[inline] fn cnt_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x24)
   }
#[doc="Get the *mut pointer for the CNT register."]
   #[inline] fn cnt_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x24)
   }
#[doc="Read the CNT register."]
   #[inline] fn cnt(&self) -> Cnt { 
      unsafe {
         Cnt(::core::ptr::read_volatile((self.base() + 0x24) as *const u32))
      }
   }
#[doc="Write the CNT register."]
   #[inline] fn set_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
      let value = f(Cnt(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CNT register."]
   #[inline] fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cnt(::core::ptr::read_volatile((self.base() + 0x24) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x24) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PSC register."]
   #[inline] fn psc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x28)
   }
#[doc="Get the *mut pointer for the PSC register."]
   #[inline] fn psc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x28)
   }
#[doc="Read the PSC register."]
   #[inline] fn psc(&self) -> Psc { 
      unsafe {
         Psc(::core::ptr::read_volatile((self.base() + 0x28) as *const u32))
      }
   }
#[doc="Write the PSC register."]
   #[inline] fn set_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &Self {
      let value = f(Psc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PSC register."]
   #[inline] fn with_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Psc(::core::ptr::read_volatile((self.base() + 0x28) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x28) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ARR register."]
   #[inline] fn arr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x2c)
   }
#[doc="Get the *mut pointer for the ARR register."]
   #[inline] fn arr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x2c)
   }
#[doc="Read the ARR register."]
   #[inline] fn arr(&self) -> Arr { 
      unsafe {
         Arr(::core::ptr::read_volatile((self.base() + 0x2c) as *const u32))
      }
   }
#[doc="Write the ARR register."]
   #[inline] fn set_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
      let value = f(Arr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x2c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ARR register."]
   #[inline] fn with_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Arr(::core::ptr::read_volatile((self.base() + 0x2c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x2c) as *mut u32, value.0);
      }
      self
   }

}

#[doc="control register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
#[doc="Counter enable"]
   #[inline] pub fn cen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter enable"]
   #[inline] pub fn set_cen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Update disable"]
   #[inline] pub fn udis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Update disable"]
   #[inline] pub fn set_udis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Update request source"]
   #[inline] pub fn urs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Update request source"]
   #[inline] pub fn set_urs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="One-pulse mode"]
   #[inline] pub fn opm(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="One-pulse mode"]
   #[inline] pub fn set_opm<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Auto-reload preload enable"]
   #[inline] pub fn arpe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Auto-reload preload enable"]
   #[inline] pub fn set_arpe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="UIF status bit remapping"]
   #[inline] pub fn uifremap(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="UIF status bit remapping"]
   #[inline] pub fn set_uifremap<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

}
impl ::core::fmt::Display for Cr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cen() != 0 { try!(write!(f, " cen"))}
      if self.udis() != 0 { try!(write!(f, " udis"))}
      if self.urs() != 0 { try!(write!(f, " urs"))}
      if self.opm() != 0 { try!(write!(f, " opm"))}
      if self.arpe() != 0 { try!(write!(f, " arpe"))}
      if self.uifremap() != 0 { try!(write!(f, " uifremap"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="control register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
#[doc="Master mode selection"]
   #[inline] pub fn mms(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
   }
#[doc="Master mode selection"]
   #[inline] pub fn set_mms<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Cr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mms() != 0 { try!(write!(f, " mms=0x{:x}", self.mms()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA/Interrupt enable register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dier(pub u32);
impl Dier {
#[doc="Update DMA request enable"]
   #[inline] pub fn ude(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Update DMA request enable"]
   #[inline] pub fn set_ude<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Update interrupt enable"]
   #[inline] pub fn uie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Update interrupt enable"]
   #[inline] pub fn set_uie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ude() != 0 { try!(write!(f, " ude"))}
      if self.uie() != 0 { try!(write!(f, " uie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="status register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="Update interrupt flag"]
   #[inline] pub fn uif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Update interrupt flag"]
   #[inline] pub fn set_uif<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
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
      if self.uif() != 0 { try!(write!(f, " uif"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="event generation register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Egr(pub u32);
impl Egr {
#[doc="Update generation"]
   #[inline] pub fn ug(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Update generation"]
   #[inline] pub fn set_ug<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Egr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Egr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ug() != 0 { try!(write!(f, " ug"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="counter"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
#[doc="Low counter value"]
   #[inline] pub fn cnt(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Low counter value"]
   #[inline] pub fn set_cnt<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UIF Copy"]
   #[inline] pub fn uifcpy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="UIF Copy"]
   #[inline] pub fn set_uifcpy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
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
      if self.uifcpy() != 0 { try!(write!(f, " uifcpy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="prescaler"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Psc(pub u32);
impl Psc {
#[doc="Prescaler value"]
   #[inline] pub fn psc(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Prescaler value"]
   #[inline] pub fn set_psc<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Psc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Psc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.psc() != 0 { try!(write!(f, " psc=0x{:x}", self.psc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="auto-reload register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Arr(pub u32);
impl Arr {
#[doc="Low Auto-reload value"]
   #[inline] pub fn arr(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Low Auto-reload value"]
   #[inline] pub fn set_arr<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
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
pub trait Channel<T> {
   fn periph(&self) -> T;
   fn index(&self) -> usize;
}

