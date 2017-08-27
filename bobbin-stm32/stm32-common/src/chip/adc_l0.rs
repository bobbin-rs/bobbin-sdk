#[allow(unused_imports)] use bobbin_common::*;



pub trait AdcL0Periph : Base {
#[doc="Get the *const pointer for the ISR register."]
   #[inline] fn isr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the ISR register."]
   #[inline] fn isr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the ISR register."]
   #[inline] fn isr(&self) -> Isr { 
      unsafe {
         Isr(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      }
   }
#[doc="Write the ISR register."]
   #[inline] fn set_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
      let value = f(Isr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ISR register."]
   #[inline] fn with_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Isr(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IER register."]
   #[inline] fn ier_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the IER register."]
   #[inline] fn ier_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the IER register."]
   #[inline] fn ier(&self) -> Ier { 
      unsafe {
         Ier(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      }
   }
#[doc="Write the IER register."]
   #[inline] fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
      let value = f(Ier(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the IER register."]
   #[inline] fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ier(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CR register."]
   #[inline] fn cr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the CR register."]
   #[inline] fn cr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the CR register."]
   #[inline] fn cr(&self) -> Cr { 
      unsafe {
         Cr(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      }
   }
#[doc="Write the CR register."]
   #[inline] fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let value = f(Cr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR register."]
   #[inline] fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cr(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CFGR1 register."]
   #[inline] fn cfgr1_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the CFGR1 register."]
   #[inline] fn cfgr1_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Read the CFGR1 register."]
   #[inline] fn cfgr1(&self) -> Cfgr1 { 
      unsafe {
         Cfgr1(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      }
   }
#[doc="Write the CFGR1 register."]
   #[inline] fn set_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
      let value = f(Cfgr1(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CFGR1 register."]
   #[inline] fn with_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cfgr1(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CFGR2 register."]
   #[inline] fn cfgr2_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Get the *mut pointer for the CFGR2 register."]
   #[inline] fn cfgr2_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Read the CFGR2 register."]
   #[inline] fn cfgr2(&self) -> Cfgr2 { 
      unsafe {
         Cfgr2(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      }
   }
#[doc="Write the CFGR2 register."]
   #[inline] fn set_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
      let value = f(Cfgr2(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CFGR2 register."]
   #[inline] fn with_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cfgr2(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SMPR register."]
   #[inline] fn smpr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Get the *mut pointer for the SMPR register."]
   #[inline] fn smpr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Read the SMPR register."]
   #[inline] fn smpr(&self) -> Smpr { 
      unsafe {
         Smpr(::core::ptr::read_volatile((self.base() + 0x14) as *const u32))
      }
   }
#[doc="Write the SMPR register."]
   #[inline] fn set_smpr<F: FnOnce(Smpr) -> Smpr>(&self, f: F) -> &Self {
      let value = f(Smpr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SMPR register."]
   #[inline] fn with_smpr<F: FnOnce(Smpr) -> Smpr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Smpr(::core::ptr::read_volatile((self.base() + 0x14) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TR register."]
   #[inline] fn tr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x20)
   }
#[doc="Get the *mut pointer for the TR register."]
   #[inline] fn tr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x20)
   }
#[doc="Read the TR register."]
   #[inline] fn tr(&self) -> Tr { 
      unsafe {
         Tr(::core::ptr::read_volatile((self.base() + 0x20) as *const u32))
      }
   }
#[doc="Write the TR register."]
   #[inline] fn set_tr<F: FnOnce(Tr) -> Tr>(&self, f: F) -> &Self {
      let value = f(Tr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TR register."]
   #[inline] fn with_tr<F: FnOnce(Tr) -> Tr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Tr(::core::ptr::read_volatile((self.base() + 0x20) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CHSELR register."]
   #[inline] fn chselr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x28)
   }
#[doc="Get the *mut pointer for the CHSELR register."]
   #[inline] fn chselr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x28)
   }
#[doc="Read the CHSELR register."]
   #[inline] fn chselr(&self) -> Chselr { 
      unsafe {
         Chselr(::core::ptr::read_volatile((self.base() + 0x28) as *const u32))
      }
   }
#[doc="Write the CHSELR register."]
   #[inline] fn set_chselr<F: FnOnce(Chselr) -> Chselr>(&self, f: F) -> &Self {
      let value = f(Chselr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CHSELR register."]
   #[inline] fn with_chselr<F: FnOnce(Chselr) -> Chselr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Chselr(::core::ptr::read_volatile((self.base() + 0x28) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x28) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DR register."]
   #[inline] fn dr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x40)
   }
#[doc="Get the *mut pointer for the DR register."]
   #[inline] fn dr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x40)
   }
#[doc="Read the DR register."]
   #[inline] fn dr(&self) -> Dr { 
      unsafe {
         Dr(::core::ptr::read_volatile((self.base() + 0x40) as *const u32))
      }
   }

#[doc="Get the *const pointer for the CALFACT register."]
   #[inline] fn calfact_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xb4)
   }
#[doc="Get the *mut pointer for the CALFACT register."]
   #[inline] fn calfact_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xb4)
   }
#[doc="Read the CALFACT register."]
   #[inline] fn calfact(&self) -> Calfact { 
      unsafe {
         Calfact(::core::ptr::read_volatile((self.base() + 0xb4) as *const u32))
      }
   }
#[doc="Write the CALFACT register."]
   #[inline] fn set_calfact<F: FnOnce(Calfact) -> Calfact>(&self, f: F) -> &Self {
      let value = f(Calfact(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xb4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CALFACT register."]
   #[inline] fn with_calfact<F: FnOnce(Calfact) -> Calfact>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Calfact(::core::ptr::read_volatile((self.base() + 0xb4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xb4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CCR register."]
   #[inline] fn ccr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x308)
   }
#[doc="Get the *mut pointer for the CCR register."]
   #[inline] fn ccr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x308)
   }
#[doc="Read the CCR register."]
   #[inline] fn ccr(&self) -> Ccr { 
      unsafe {
         Ccr(::core::ptr::read_volatile((self.base() + 0x308) as *const u32))
      }
   }
#[doc="Write the CCR register."]
   #[inline] fn set_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
      let value = f(Ccr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x308) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CCR register."]
   #[inline] fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ccr(::core::ptr::read_volatile((self.base() + 0x308) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x308) as *mut u32, value.0);
      }
      self
   }

}

#[doc="interrupt and status register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
#[doc="ADC ready"]
   #[inline] pub fn adrdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="ADC ready"]
   #[inline] pub fn set_adrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="End of sampling flag"]
   #[inline] pub fn eosmp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="End of sampling flag"]
   #[inline] pub fn set_eosmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="End of conversion flag"]
   #[inline] pub fn eoc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="End of conversion flag"]
   #[inline] pub fn set_eoc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="End of sequence flag"]
   #[inline] pub fn eos(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="End of sequence flag"]
   #[inline] pub fn set_eos<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="ADC overrun"]
   #[inline] pub fn ovr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="ADC overrun"]
   #[inline] pub fn set_ovr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Analog watchdog flag"]
   #[inline] pub fn awd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Analog watchdog flag"]
   #[inline] pub fn set_awd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="End Of Calibration flag"]
   #[inline] pub fn eocal(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="End Of Calibration flag"]
   #[inline] pub fn set_eocal<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
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
      if self.adrdy() != 0 { try!(write!(f, " adrdy"))}
      if self.eosmp() != 0 { try!(write!(f, " eosmp"))}
      if self.eoc() != 0 { try!(write!(f, " eoc"))}
      if self.eos() != 0 { try!(write!(f, " eos"))}
      if self.ovr() != 0 { try!(write!(f, " ovr"))}
      if self.awd() != 0 { try!(write!(f, " awd"))}
      if self.eocal() != 0 { try!(write!(f, " eocal"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="interrupt enable register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
#[doc="ADC ready interrupt enable"]
   #[inline] pub fn adrdyie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="ADC ready interrupt enable"]
   #[inline] pub fn set_adrdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="End of sampling flag interrupt enable"]
   #[inline] pub fn eosmpie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="End of sampling flag interrupt enable"]
   #[inline] pub fn set_eosmpie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="End of conversion interrupt enable"]
   #[inline] pub fn eocie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="End of conversion interrupt enable"]
   #[inline] pub fn set_eocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="End of conversion sequence interrupt enable"]
   #[inline] pub fn eosie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="End of conversion sequence interrupt enable"]
   #[inline] pub fn set_eosie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Overrun interrupt enable"]
   #[inline] pub fn ovrie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Overrun interrupt enable"]
   #[inline] pub fn set_ovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Analog watchdog interrupt enable"]
   #[inline] pub fn awdie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Analog watchdog interrupt enable"]
   #[inline] pub fn set_awdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="End of calibration interrupt enable"]
   #[inline] pub fn eocalie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="End of calibration interrupt enable"]
   #[inline] pub fn set_eocalie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
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
      if self.adrdyie() != 0 { try!(write!(f, " adrdyie"))}
      if self.eosmpie() != 0 { try!(write!(f, " eosmpie"))}
      if self.eocie() != 0 { try!(write!(f, " eocie"))}
      if self.eosie() != 0 { try!(write!(f, " eosie"))}
      if self.ovrie() != 0 { try!(write!(f, " ovrie"))}
      if self.awdie() != 0 { try!(write!(f, " awdie"))}
      if self.eocalie() != 0 { try!(write!(f, " eocalie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="ADC enable command"]
   #[inline] pub fn aden(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="ADC enable command"]
   #[inline] pub fn set_aden<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="ADC disable command"]
   #[inline] pub fn addis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="ADC disable command"]
   #[inline] pub fn set_addis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="ADC start conversion command"]
   #[inline] pub fn adstart(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="ADC start conversion command"]
   #[inline] pub fn set_adstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="ADC stop conversion command"]
   #[inline] pub fn adstp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="ADC stop conversion command"]
   #[inline] pub fn set_adstp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="ADC Voltage Regulator Enable"]
   #[inline] pub fn advregen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="ADC Voltage Regulator Enable"]
   #[inline] pub fn set_advregen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="ADC calibration"]
   #[inline] pub fn adcal(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="ADC calibration"]
   #[inline] pub fn set_adcal<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
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
      if self.aden() != 0 { try!(write!(f, " aden"))}
      if self.addis() != 0 { try!(write!(f, " addis"))}
      if self.adstart() != 0 { try!(write!(f, " adstart"))}
      if self.adstp() != 0 { try!(write!(f, " adstp"))}
      if self.advregen() != 0 { try!(write!(f, " advregen"))}
      if self.adcal() != 0 { try!(write!(f, " adcal"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="configuration register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
#[doc="Analog watchdog channel selection"]
   #[inline] pub fn awdch(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
   }
#[doc="Analog watchdog channel selection"]
   #[inline] pub fn set_awdch<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 26);
      self.0 |= value << 26;
      self
   }

#[doc="Analog watchdog enable"]
   #[inline] pub fn awden(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="Analog watchdog enable"]
   #[inline] pub fn set_awden<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

#[doc="Enable the watchdog on a single channel or on all channels"]
   #[inline] pub fn awdsgl(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="Enable the watchdog on a single channel or on all channels"]
   #[inline] pub fn set_awdsgl<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="Discontinuous mode"]
   #[inline] pub fn discen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Discontinuous mode"]
   #[inline] pub fn set_discen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Auto-off mode"]
   #[inline] pub fn autoff(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Auto-off mode"]
   #[inline] pub fn set_autoff<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Auto-delayed conversion mode"]
   #[inline] pub fn autdly(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Auto-delayed conversion mode"]
   #[inline] pub fn set_autdly<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Single / continuous conversion mode"]
   #[inline] pub fn cont(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Single / continuous conversion mode"]
   #[inline] pub fn set_cont<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Overrun management mode"]
   #[inline] pub fn ovrmod(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Overrun management mode"]
   #[inline] pub fn set_ovrmod<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="External trigger enable and polarity selection"]
   #[inline] pub fn exten(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
   }
#[doc="External trigger enable and polarity selection"]
   #[inline] pub fn set_exten<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="External trigger selection"]
   #[inline] pub fn extsel(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
   }
#[doc="External trigger selection"]
   #[inline] pub fn set_extsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Data alignment"]
   #[inline] pub fn align(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Data alignment"]
   #[inline] pub fn set_align<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Data resolution"]
   #[inline] pub fn res(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
   }
#[doc="Data resolution"]
   #[inline] pub fn set_res<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Scan sequence direction"]
   #[inline] pub fn scandir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Scan sequence direction"]
   #[inline] pub fn set_scandir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Direct memery access configuration"]
   #[inline] pub fn dmacfg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Direct memery access configuration"]
   #[inline] pub fn set_dmacfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Direct memory access enable"]
   #[inline] pub fn dmaen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Direct memory access enable"]
   #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cfgr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfgr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.awdch() != 0 { try!(write!(f, " awdch=0x{:x}", self.awdch()))}
      if self.awden() != 0 { try!(write!(f, " awden"))}
      if self.awdsgl() != 0 { try!(write!(f, " awdsgl"))}
      if self.discen() != 0 { try!(write!(f, " discen"))}
      if self.autoff() != 0 { try!(write!(f, " autoff"))}
      if self.autdly() != 0 { try!(write!(f, " autdly"))}
      if self.cont() != 0 { try!(write!(f, " cont"))}
      if self.ovrmod() != 0 { try!(write!(f, " ovrmod"))}
      if self.exten() != 0 { try!(write!(f, " exten=0x{:x}", self.exten()))}
      if self.extsel() != 0 { try!(write!(f, " extsel=0x{:x}", self.extsel()))}
      if self.align() != 0 { try!(write!(f, " align"))}
      if self.res() != 0 { try!(write!(f, " res=0x{:x}", self.res()))}
      if self.scandir() != 0 { try!(write!(f, " scandir"))}
      if self.dmacfg() != 0 { try!(write!(f, " dmacfg"))}
      if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="configuration register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
#[doc="Oversampler Enable"]
   #[inline] pub fn ovse(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Oversampler Enable"]
   #[inline] pub fn set_ovse<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Oversampling ratio"]
   #[inline] pub fn ovsr(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
   }
#[doc="Oversampling ratio"]
   #[inline] pub fn set_ovsr<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Oversampling shift"]
   #[inline] pub fn ovss(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0xf) as u8) } // [8:5]
   }
#[doc="Oversampling shift"]
   #[inline] pub fn set_ovss<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Triggered Oversampling"]
   #[inline] pub fn tovs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Triggered Oversampling"]
   #[inline] pub fn set_tovs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="ADC clock mode"]
   #[inline] pub fn ckmode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
   }
#[doc="ADC clock mode"]
   #[inline] pub fn set_ckmode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 30);
      self.0 |= value << 30;
      self
   }

}
impl ::core::fmt::Display for Cfgr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfgr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ovse() != 0 { try!(write!(f, " ovse"))}
      if self.ovsr() != 0 { try!(write!(f, " ovsr=0x{:x}", self.ovsr()))}
      if self.ovss() != 0 { try!(write!(f, " ovss=0x{:x}", self.ovss()))}
      if self.tovs() != 0 { try!(write!(f, " tovs"))}
      if self.ckmode() != 0 { try!(write!(f, " ckmode=0x{:x}", self.ckmode()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="sampling time register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Smpr(pub u32);
impl Smpr {
#[doc="Sampling time selection"]
   #[inline] pub fn smpr(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Sampling time selection"]
   #[inline] pub fn set_smpr<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Smpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Smpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.smpr() != 0 { try!(write!(f, " smpr=0x{:x}", self.smpr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="watchdog threshold register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tr(pub u32);
impl Tr {
#[doc="Analog watchdog higher threshold"]
   #[inline] pub fn ht(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
   }
#[doc="Analog watchdog higher threshold"]
   #[inline] pub fn set_ht<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Analog watchdog lower threshold"]
   #[inline] pub fn lt(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="Analog watchdog lower threshold"]
   #[inline] pub fn set_lt<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Tr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.lt() != 0 { try!(write!(f, " lt=0x{:x}", self.lt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="channel selection register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Chselr(pub u32);
impl Chselr {
#[doc="Channel-x selection"]
   #[inline] pub fn chsel<I: Into<bits::R19>>(&self, index: I) -> bits::U1 {
      let index: bits::R19 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Channel-x selection"]
   #[inline] pub fn set_chsel<I: Into<bits::R19>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R19 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Chselr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chselr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chsel(0) != 0 { try!(write!(f, " chsel[0]"))}
      if self.chsel(1) != 0 { try!(write!(f, " chsel[1]"))}
      if self.chsel(2) != 0 { try!(write!(f, " chsel[2]"))}
      if self.chsel(3) != 0 { try!(write!(f, " chsel[3]"))}
      if self.chsel(4) != 0 { try!(write!(f, " chsel[4]"))}
      if self.chsel(5) != 0 { try!(write!(f, " chsel[5]"))}
      if self.chsel(6) != 0 { try!(write!(f, " chsel[6]"))}
      if self.chsel(7) != 0 { try!(write!(f, " chsel[7]"))}
      if self.chsel(8) != 0 { try!(write!(f, " chsel[8]"))}
      if self.chsel(9) != 0 { try!(write!(f, " chsel[9]"))}
      if self.chsel(10) != 0 { try!(write!(f, " chsel[10]"))}
      if self.chsel(11) != 0 { try!(write!(f, " chsel[11]"))}
      if self.chsel(12) != 0 { try!(write!(f, " chsel[12]"))}
      if self.chsel(13) != 0 { try!(write!(f, " chsel[13]"))}
      if self.chsel(14) != 0 { try!(write!(f, " chsel[14]"))}
      if self.chsel(15) != 0 { try!(write!(f, " chsel[15]"))}
      if self.chsel(16) != 0 { try!(write!(f, " chsel[16]"))}
      if self.chsel(17) != 0 { try!(write!(f, " chsel[17]"))}
      if self.chsel(18) != 0 { try!(write!(f, " chsel[18]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="data register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
#[doc="Converted data"]
   #[inline] pub fn data(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Converted data"]
   #[inline] pub fn set_data<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="DATA (12 bit)"]
   #[inline] pub fn data_12(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="DATA (12 bit)"]
   #[inline] pub fn set_data_12<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="DATA (10 bit)"]
   #[inline] pub fn data_10(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="DATA (10 bit)"]
   #[inline] pub fn set_data_10<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="DATA (8 bit)"]
   #[inline] pub fn data_8(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="DATA (8 bit)"]
   #[inline] pub fn set_data_8<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="DATA (6 bit)"]
   #[inline] pub fn data_6(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="DATA (6 bit)"]
   #[inline] pub fn set_data_6<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      if self.data_12() != 0 { try!(write!(f, " data_12=0x{:x}", self.data_12()))}
      if self.data_10() != 0 { try!(write!(f, " data_10=0x{:x}", self.data_10()))}
      if self.data_8() != 0 { try!(write!(f, " data_8=0x{:x}", self.data_8()))}
      if self.data_6() != 0 { try!(write!(f, " data_6=0x{:x}", self.data_6()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Calibration factor"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Calfact(pub u32);
impl Calfact {
#[doc="Calibration factor"]
   #[inline] pub fn calfact(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="Calibration factor"]
   #[inline] pub fn set_calfact<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Calfact {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Calfact {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.calfact() != 0 { try!(write!(f, " calfact=0x{:x}", self.calfact()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC common configuration register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
#[doc="ADC prescaler"]
   #[inline] pub fn presc(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0xf) as u8) } // [21:18]
   }
#[doc="ADC prescaler"]
   #[inline] pub fn set_presc<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 18);
      self.0 |= value << 18;
      self
   }

#[doc="VREFINT enable"]
   #[inline] pub fn vrefen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="VREFINT enable"]
   #[inline] pub fn set_vrefen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="Temperature sensor enable"]
   #[inline] pub fn tsen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="Temperature sensor enable"]
   #[inline] pub fn set_tsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

#[doc="VLCD enable"]
   #[inline] pub fn vlcden(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="VLCD enable"]
   #[inline] pub fn set_vlcden<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Low Frequency Mode enable"]
   #[inline] pub fn lfmen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Low Frequency Mode enable"]
   #[inline] pub fn set_lfmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

}
impl ::core::fmt::Display for Ccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
      if self.vrefen() != 0 { try!(write!(f, " vrefen"))}
      if self.tsen() != 0 { try!(write!(f, " tsen"))}
      if self.vlcden() != 0 { try!(write!(f, " vlcden"))}
      if self.lfmen() != 0 { try!(write!(f, " lfmen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub trait Channel<T> {
   fn periph(&self) -> T;
   fn index(&self) -> usize;
}

