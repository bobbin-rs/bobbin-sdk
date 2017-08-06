#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPSPI Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the VERID register."]
  #[inline] pub fn verid_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the VERID register."]
  #[inline] pub fn verid_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the VERID register."]
  #[inline] pub fn verid(&self) -> Verid { 
     unsafe {
        Verid(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PARAM register."]
  #[inline] pub fn param_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the PARAM register."]
  #[inline] pub fn param_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the PARAM register."]
  #[inline] pub fn param(&self) -> Param { 
     unsafe {
        Param(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }

#[doc="Get the *const pointer for the CR register."]
  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the CR register."]
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the CR register."]
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the CR register."]
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

#[doc="Get the *const pointer for the SR register."]
  #[inline] pub fn sr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the SR register."]
  #[inline] pub fn sr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the SR register."]
  #[inline] pub fn sr(&self) -> Sr { 
     unsafe {
        Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the SR register."]
  #[inline] pub fn set_sr(&self, value: Sr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR register."]
  #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

#[doc="Get the *const pointer for the IER register."]
  #[inline] pub fn ier_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the IER register."]
  #[inline] pub fn ier_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the IER register."]
  #[inline] pub fn ier(&self) -> Ier { 
     unsafe {
        Ier(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the IER register."]
  #[inline] pub fn set_ier(&self, value: Ier) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the IER register."]
  #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
     let tmp = self.ier();
     self.set_ier(f(tmp))
  }

#[doc="Get the *const pointer for the DER register."]
  #[inline] pub fn der_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the DER register."]
  #[inline] pub fn der_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the DER register."]
  #[inline] pub fn der(&self) -> Der { 
     unsafe {
        Der(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the DER register."]
  #[inline] pub fn set_der(&self, value: Der) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DER register."]
  #[inline] pub fn with_der<F: FnOnce(Der) -> Der>(&self, f: F) -> &Self {
     let tmp = self.der();
     self.set_der(f(tmp))
  }

#[doc="Get the *const pointer for the CFGR0 register."]
  #[inline] pub fn cfgr0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the CFGR0 register."]
  #[inline] pub fn cfgr0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the CFGR0 register."]
  #[inline] pub fn cfgr0(&self) -> Cfgr0 { 
     unsafe {
        Cfgr0(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the CFGR0 register."]
  #[inline] pub fn set_cfgr0(&self, value: Cfgr0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFGR0 register."]
  #[inline] pub fn with_cfgr0<F: FnOnce(Cfgr0) -> Cfgr0>(&self, f: F) -> &Self {
     let tmp = self.cfgr0();
     self.set_cfgr0(f(tmp))
  }

#[doc="Get the *const pointer for the CFGR1 register."]
  #[inline] pub fn cfgr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the CFGR1 register."]
  #[inline] pub fn cfgr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the CFGR1 register."]
  #[inline] pub fn cfgr1(&self) -> Cfgr1 { 
     unsafe {
        Cfgr1(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the CFGR1 register."]
  #[inline] pub fn set_cfgr1(&self, value: Cfgr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFGR1 register."]
  #[inline] pub fn with_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
     let tmp = self.cfgr1();
     self.set_cfgr1(f(tmp))
  }

#[doc="Get the *const pointer for the DMR0 register."]
  #[inline] pub fn dmr0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
#[doc="Get the *mut pointer for the DMR0 register."]
  #[inline] pub fn dmr0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
#[doc="Read the DMR0 register."]
  #[inline] pub fn dmr0(&self) -> Dmr0 { 
     unsafe {
        Dmr0(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
#[doc="Write the DMR0 register."]
  #[inline] pub fn set_dmr0(&self, value: Dmr0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DMR0 register."]
  #[inline] pub fn with_dmr0<F: FnOnce(Dmr0) -> Dmr0>(&self, f: F) -> &Self {
     let tmp = self.dmr0();
     self.set_dmr0(f(tmp))
  }

#[doc="Get the *const pointer for the DMR1 register."]
  #[inline] pub fn dmr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
#[doc="Get the *mut pointer for the DMR1 register."]
  #[inline] pub fn dmr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
#[doc="Read the DMR1 register."]
  #[inline] pub fn dmr1(&self) -> Dmr1 { 
     unsafe {
        Dmr1(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }
#[doc="Write the DMR1 register."]
  #[inline] pub fn set_dmr1(&self, value: Dmr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DMR1 register."]
  #[inline] pub fn with_dmr1<F: FnOnce(Dmr1) -> Dmr1>(&self, f: F) -> &Self {
     let tmp = self.dmr1();
     self.set_dmr1(f(tmp))
  }

#[doc="Get the *const pointer for the CCR register."]
  #[inline] pub fn ccr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
#[doc="Get the *mut pointer for the CCR register."]
  #[inline] pub fn ccr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
#[doc="Read the CCR register."]
  #[inline] pub fn ccr(&self) -> Ccr { 
     unsafe {
        Ccr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
#[doc="Write the CCR register."]
  #[inline] pub fn set_ccr(&self, value: Ccr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CCR register."]
  #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
     let tmp = self.ccr();
     self.set_ccr(f(tmp))
  }

#[doc="Get the *const pointer for the FCR register."]
  #[inline] pub fn fcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x58) as *const u32
  }
#[doc="Get the *mut pointer for the FCR register."]
  #[inline] pub fn fcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x58) as *mut u32
  }
#[doc="Read the FCR register."]
  #[inline] pub fn fcr(&self) -> Fcr { 
     unsafe {
        Fcr(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }
#[doc="Write the FCR register."]
  #[inline] pub fn set_fcr(&self, value: Fcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FCR register."]
  #[inline] pub fn with_fcr<F: FnOnce(Fcr) -> Fcr>(&self, f: F) -> &Self {
     let tmp = self.fcr();
     self.set_fcr(f(tmp))
  }

#[doc="Get the *const pointer for the FSR register."]
  #[inline] pub fn fsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x5c) as *const u32
  }
#[doc="Get the *mut pointer for the FSR register."]
  #[inline] pub fn fsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x5c) as *mut u32
  }
#[doc="Read the FSR register."]
  #[inline] pub fn fsr(&self) -> Fsr { 
     unsafe {
        Fsr(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the TCR register."]
  #[inline] pub fn tcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x60) as *const u32
  }
#[doc="Get the *mut pointer for the TCR register."]
  #[inline] pub fn tcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x60) as *mut u32
  }
#[doc="Read the TCR register."]
  #[inline] pub fn tcr(&self) -> Tcr { 
     unsafe {
        Tcr(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
     }
  }
#[doc="Write the TCR register."]
  #[inline] pub fn set_tcr(&self, value: Tcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TCR register."]
  #[inline] pub fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
     let tmp = self.tcr();
     self.set_tcr(f(tmp))
  }

#[doc="Get the *const pointer for the TDR register."]
  #[inline] pub fn tdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x64) as *const u32
  }
#[doc="Get the *mut pointer for the TDR register."]
  #[inline] pub fn tdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x64) as *mut u32
  }
#[doc="Write the TDR register."]
  #[inline] pub fn set_tdr(&self, value: Tdr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the RSR register."]
  #[inline] pub fn rsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x70) as *const u32
  }
#[doc="Get the *mut pointer for the RSR register."]
  #[inline] pub fn rsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x70) as *mut u32
  }
#[doc="Read the RSR register."]
  #[inline] pub fn rsr(&self) -> Rsr { 
     unsafe {
        Rsr(::core::ptr::read_volatile(((self.0 as usize) + 0x70) as *const u32))
     }
  }

#[doc="Get the *const pointer for the RDR register."]
  #[inline] pub fn rdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x74) as *const u32
  }
#[doc="Get the *mut pointer for the RDR register."]
  #[inline] pub fn rdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x74) as *mut u32
  }
#[doc="Read the RDR register."]
  #[inline] pub fn rdr(&self) -> Rdr { 
     unsafe {
        Rdr(::core::ptr::read_volatile(((self.0 as usize) + 0x74) as *const u32))
     }
  }

}

#[doc="Version ID Register"]
#[derive(PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
#[doc="Module Identification Number"]
  #[inline] pub fn feature(&self) -> bits::B16 {
     (((self.0 as u32) >> 0) & 0xffff).into() // [15:0]
  }
#[doc="Module Identification Number"]
  #[inline] pub fn set_feature<V: Into<bits::B16>>(mut self, value: V) -> Self {
     let value: bits::B16 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Minor Version Number"]
  #[inline] pub fn minor(&self) -> bits::B8 {
     (((self.0 as u32) >> 16) & 0xff).into() // [23:16]
  }
#[doc="Minor Version Number"]
  #[inline] pub fn set_minor<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Major Version Number"]
  #[inline] pub fn major(&self) -> bits::B8 {
     (((self.0 as u32) >> 24) & 0xff).into() // [31:24]
  }
#[doc="Major Version Number"]
  #[inline] pub fn set_major<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Verid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Verid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.feature() != 0 { try!(write!(f, " feature=0x{:x}", self.feature()))}
      if self.minor() != 0 { try!(write!(f, " minor=0x{:x}", self.minor()))}
      if self.major() != 0 { try!(write!(f, " major=0x{:x}", self.major()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Parameter Register"]
#[derive(PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
#[doc="Transmit FIFO Size"]
  #[inline] pub fn txfifo(&self) -> bits::B8 {
     (((self.0 as u32) >> 0) & 0xff).into() // [7:0]
  }
#[doc="Transmit FIFO Size"]
  #[inline] pub fn set_txfifo<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Receive FIFO Size"]
  #[inline] pub fn rxfifo(&self) -> bits::B8 {
     (((self.0 as u32) >> 8) & 0xff).into() // [15:8]
  }
#[doc="Receive FIFO Size"]
  #[inline] pub fn set_rxfifo<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Param {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Param {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txfifo() != 0 { try!(write!(f, " txfifo=0x{:x}", self.txfifo()))}
      if self.rxfifo() != 0 { try!(write!(f, " rxfifo=0x{:x}", self.rxfifo()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control Register"]
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Module Enable"]
  #[inline] pub fn men(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Module Enable"]
  #[inline] pub fn set_men<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Software Reset"]
  #[inline] pub fn rst(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Software Reset"]
  #[inline] pub fn set_rst<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Doze mode enable"]
  #[inline] pub fn dozen(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Doze mode enable"]
  #[inline] pub fn set_dozen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Debug Enable"]
  #[inline] pub fn dbgen(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="Debug Enable"]
  #[inline] pub fn set_dbgen<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Reset Transmit FIFO"]
  #[inline] pub fn rtf(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Reset Transmit FIFO"]
  #[inline] pub fn set_rtf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Reset Receive FIFO"]
  #[inline] pub fn rrf(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Reset Receive FIFO"]
  #[inline] pub fn set_rrf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
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
      if self.men() != 0 { try!(write!(f, " men"))}
      if self.rst() != 0 { try!(write!(f, " rst"))}
      if self.dozen() != 0 { try!(write!(f, " dozen"))}
      if self.dbgen() != 0 { try!(write!(f, " dbgen"))}
      if self.rtf() != 0 { try!(write!(f, " rtf"))}
      if self.rrf() != 0 { try!(write!(f, " rrf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status Register"]
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="Transmit Data Flag"]
  #[inline] pub fn tdf(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Transmit Data Flag"]
  #[inline] pub fn set_tdf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Receive Data Flag"]
  #[inline] pub fn rdf(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Receive Data Flag"]
  #[inline] pub fn set_rdf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Word Complete Flag"]
  #[inline] pub fn wcf(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Word Complete Flag"]
  #[inline] pub fn set_wcf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Frame Complete Flag"]
  #[inline] pub fn fcf(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Frame Complete Flag"]
  #[inline] pub fn set_fcf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Transfer Complete Flag"]
  #[inline] pub fn tcf(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="Transfer Complete Flag"]
  #[inline] pub fn set_tcf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Transmit Error Flag"]
  #[inline] pub fn tef(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="Transmit Error Flag"]
  #[inline] pub fn set_tef<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Receive Error Flag"]
  #[inline] pub fn _ref(&self) -> bits::B1 {
     (((self.0 as u32) >> 12) & 0x1).into() // [12]
  }
#[doc="Receive Error Flag"]
  #[inline] pub fn set_ref<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Data Match Flag"]
  #[inline] pub fn dmf(&self) -> bits::B1 {
     (((self.0 as u32) >> 13) & 0x1).into() // [13]
  }
#[doc="Data Match Flag"]
  #[inline] pub fn set_dmf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Module Busy Flag"]
  #[inline] pub fn mbf(&self) -> bits::B1 {
     (((self.0 as u32) >> 24) & 0x1).into() // [24]
  }
#[doc="Module Busy Flag"]
  #[inline] pub fn set_mbf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
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
      if self.tdf() != 0 { try!(write!(f, " tdf"))}
      if self.rdf() != 0 { try!(write!(f, " rdf"))}
      if self.wcf() != 0 { try!(write!(f, " wcf"))}
      if self.fcf() != 0 { try!(write!(f, " fcf"))}
      if self.tcf() != 0 { try!(write!(f, " tcf"))}
      if self.tef() != 0 { try!(write!(f, " tef"))}
      if self._ref() != 0 { try!(write!(f, " _ref"))}
      if self.dmf() != 0 { try!(write!(f, " dmf"))}
      if self.mbf() != 0 { try!(write!(f, " mbf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Enable Register"]
#[derive(PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
#[doc="Transmit Data Interrupt Enable"]
  #[inline] pub fn tdie(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Transmit Data Interrupt Enable"]
  #[inline] pub fn set_tdie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Receive Data Interrupt Enable"]
  #[inline] pub fn rdie(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Receive Data Interrupt Enable"]
  #[inline] pub fn set_rdie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Word Complete Interrupt Enable"]
  #[inline] pub fn wcie(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Word Complete Interrupt Enable"]
  #[inline] pub fn set_wcie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Frame Complete Interrupt Enable"]
  #[inline] pub fn fcie(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Frame Complete Interrupt Enable"]
  #[inline] pub fn set_fcie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Transfer Complete Interrupt Enable"]
  #[inline] pub fn tcie(&self) -> bits::B1 {
     (((self.0 as u32) >> 10) & 0x1).into() // [10]
  }
#[doc="Transfer Complete Interrupt Enable"]
  #[inline] pub fn set_tcie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Transmit Error Interrupt Enable"]
  #[inline] pub fn teie(&self) -> bits::B1 {
     (((self.0 as u32) >> 11) & 0x1).into() // [11]
  }
#[doc="Transmit Error Interrupt Enable"]
  #[inline] pub fn set_teie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Receive Error Interrupt Enable"]
  #[inline] pub fn reie(&self) -> bits::B1 {
     (((self.0 as u32) >> 12) & 0x1).into() // [12]
  }
#[doc="Receive Error Interrupt Enable"]
  #[inline] pub fn set_reie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Data Match Interrupt Enable"]
  #[inline] pub fn dmie(&self) -> bits::B1 {
     (((self.0 as u32) >> 13) & 0x1).into() // [13]
  }
#[doc="Data Match Interrupt Enable"]
  #[inline] pub fn set_dmie<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
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
      if self.tdie() != 0 { try!(write!(f, " tdie"))}
      if self.rdie() != 0 { try!(write!(f, " rdie"))}
      if self.wcie() != 0 { try!(write!(f, " wcie"))}
      if self.fcie() != 0 { try!(write!(f, " fcie"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.teie() != 0 { try!(write!(f, " teie"))}
      if self.reie() != 0 { try!(write!(f, " reie"))}
      if self.dmie() != 0 { try!(write!(f, " dmie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Enable Register"]
#[derive(PartialEq, Eq)]
pub struct Der(pub u32);
impl Der {
#[doc="Transmit Data DMA Enable"]
  #[inline] pub fn tdde(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Transmit Data DMA Enable"]
  #[inline] pub fn set_tdde<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Receive Data DMA Enable"]
  #[inline] pub fn rdde(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Receive Data DMA Enable"]
  #[inline] pub fn set_rdde<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Der {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Der {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tdde() != 0 { try!(write!(f, " tdde"))}
      if self.rdde() != 0 { try!(write!(f, " rdde"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Configuration Register 0"]
#[derive(PartialEq, Eq)]
pub struct Cfgr0(pub u32);
impl Cfgr0 {
#[doc="Host Request Enable"]
  #[inline] pub fn hren(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Host Request Enable"]
  #[inline] pub fn set_hren<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Host Request Polarity"]
  #[inline] pub fn hrpol(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Host Request Polarity"]
  #[inline] pub fn set_hrpol<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Host Request Select"]
  #[inline] pub fn hrsel(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Host Request Select"]
  #[inline] pub fn set_hrsel<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Circular FIFO Enable"]
  #[inline] pub fn cirfifo(&self) -> bits::B1 {
     (((self.0 as u32) >> 8) & 0x1).into() // [8]
  }
#[doc="Circular FIFO Enable"]
  #[inline] pub fn set_cirfifo<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Receive Data Match Only"]
  #[inline] pub fn rdmo(&self) -> bits::B1 {
     (((self.0 as u32) >> 9) & 0x1).into() // [9]
  }
#[doc="Receive Data Match Only"]
  #[inline] pub fn set_rdmo<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

}
impl ::core::fmt::Display for Cfgr0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfgr0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.hren() != 0 { try!(write!(f, " hren"))}
      if self.hrpol() != 0 { try!(write!(f, " hrpol"))}
      if self.hrsel() != 0 { try!(write!(f, " hrsel"))}
      if self.cirfifo() != 0 { try!(write!(f, " cirfifo"))}
      if self.rdmo() != 0 { try!(write!(f, " rdmo"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Configuration Register 1"]
#[derive(PartialEq, Eq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
#[doc="Master Mode"]
  #[inline] pub fn master(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Master Mode"]
  #[inline] pub fn set_master<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Sample Point"]
  #[inline] pub fn sample(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="Sample Point"]
  #[inline] pub fn set_sample<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Automatic PCS"]
  #[inline] pub fn autopcs(&self) -> bits::B1 {
     (((self.0 as u32) >> 2) & 0x1).into() // [2]
  }
#[doc="Automatic PCS"]
  #[inline] pub fn set_autopcs<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="No Stall"]
  #[inline] pub fn nostall(&self) -> bits::B1 {
     (((self.0 as u32) >> 3) & 0x1).into() // [3]
  }
#[doc="No Stall"]
  #[inline] pub fn set_nostall<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Peripheral Chip Select Polarity"]
  #[inline] pub fn pcspol(&self) -> bits::B4 {
     (((self.0 as u32) >> 8) & 0xf).into() // [11:8]
  }
#[doc="Peripheral Chip Select Polarity"]
  #[inline] pub fn set_pcspol<V: Into<bits::B4>>(mut self, value: V) -> Self {
     let value: bits::B4 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Match Configuration"]
  #[inline] pub fn matcfg(&self) -> bits::B3 {
     (((self.0 as u32) >> 16) & 0x7).into() // [18:16]
  }
#[doc="Match Configuration"]
  #[inline] pub fn set_matcfg<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Pin Configuration"]
  #[inline] pub fn pincfg(&self) -> bits::B2 {
     (((self.0 as u32) >> 24) & 0x3).into() // [25:24]
  }
#[doc="Pin Configuration"]
  #[inline] pub fn set_pincfg<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Output Config"]
  #[inline] pub fn outcfg(&self) -> bits::B1 {
     (((self.0 as u32) >> 26) & 0x1).into() // [26]
  }
#[doc="Output Config"]
  #[inline] pub fn set_outcfg<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Peripheral Chip Select Configuration"]
  #[inline] pub fn pcscfg(&self) -> bits::B1 {
     (((self.0 as u32) >> 27) & 0x1).into() // [27]
  }
#[doc="Peripheral Chip Select Configuration"]
  #[inline] pub fn set_pcscfg<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
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
      if self.master() != 0 { try!(write!(f, " master"))}
      if self.sample() != 0 { try!(write!(f, " sample"))}
      if self.autopcs() != 0 { try!(write!(f, " autopcs"))}
      if self.nostall() != 0 { try!(write!(f, " nostall"))}
      if self.pcspol() != 0 { try!(write!(f, " pcspol=0x{:x}", self.pcspol()))}
      if self.matcfg() != 0 { try!(write!(f, " matcfg=0x{:x}", self.matcfg()))}
      if self.pincfg() != 0 { try!(write!(f, " pincfg=0x{:x}", self.pincfg()))}
      if self.outcfg() != 0 { try!(write!(f, " outcfg"))}
      if self.pcscfg() != 0 { try!(write!(f, " pcscfg"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Data Match Register 0"]
#[derive(PartialEq, Eq)]
pub struct Dmr0(pub u32);
impl Dmr0 {
#[doc="Match 0 Value"]
  #[inline] pub fn match0(&self) -> bits::B32 {
     (((self.0 as u32) >> 0) & 0xffffffff).into() // [31:0]
  }
#[doc="Match 0 Value"]
  #[inline] pub fn set_match0<V: Into<bits::B32>>(mut self, value: V) -> Self {
     let value: bits::B32 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmr0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmr0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Data Match Register 1"]
#[derive(PartialEq, Eq)]
pub struct Dmr1(pub u32);
impl Dmr1 {
#[doc="Match 1 Value"]
  #[inline] pub fn match1(&self) -> bits::B32 {
     (((self.0 as u32) >> 0) & 0xffffffff).into() // [31:0]
  }
#[doc="Match 1 Value"]
  #[inline] pub fn set_match1<V: Into<bits::B32>>(mut self, value: V) -> Self {
     let value: bits::B32 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clock Configuration Register"]
#[derive(PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
#[doc="SCK Divider"]
  #[inline] pub fn sckdiv(&self) -> bits::B8 {
     (((self.0 as u32) >> 0) & 0xff).into() // [7:0]
  }
#[doc="SCK Divider"]
  #[inline] pub fn set_sckdiv<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Delay Between Transfers"]
  #[inline] pub fn dbt(&self) -> bits::B8 {
     (((self.0 as u32) >> 8) & 0xff).into() // [15:8]
  }
#[doc="Delay Between Transfers"]
  #[inline] pub fn set_dbt<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="PCS to SCK Delay"]
  #[inline] pub fn pcssck(&self) -> bits::B8 {
     (((self.0 as u32) >> 16) & 0xff).into() // [23:16]
  }
#[doc="PCS to SCK Delay"]
  #[inline] pub fn set_pcssck<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="SCK to PCS Delay"]
  #[inline] pub fn sckpcs(&self) -> bits::B8 {
     (((self.0 as u32) >> 24) & 0xff).into() // [31:24]
  }
#[doc="SCK to PCS Delay"]
  #[inline] pub fn set_sckpcs<V: Into<bits::B8>>(mut self, value: V) -> Self {
     let value: bits::B8 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
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
      if self.sckdiv() != 0 { try!(write!(f, " sckdiv=0x{:x}", self.sckdiv()))}
      if self.dbt() != 0 { try!(write!(f, " dbt=0x{:x}", self.dbt()))}
      if self.pcssck() != 0 { try!(write!(f, " pcssck=0x{:x}", self.pcssck()))}
      if self.sckpcs() != 0 { try!(write!(f, " sckpcs=0x{:x}", self.sckpcs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FIFO Control Register"]
#[derive(PartialEq, Eq)]
pub struct Fcr(pub u32);
impl Fcr {
#[doc="Transmit FIFO Watermark"]
  #[inline] pub fn txwater(&self) -> bits::B2 {
     (((self.0 as u32) >> 0) & 0x3).into() // [1:0]
  }
#[doc="Transmit FIFO Watermark"]
  #[inline] pub fn set_txwater<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Receive FIFO Watermark"]
  #[inline] pub fn rxwater(&self) -> bits::B2 {
     (((self.0 as u32) >> 16) & 0x3).into() // [17:16]
  }
#[doc="Receive FIFO Watermark"]
  #[inline] pub fn set_rxwater<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Fcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txwater() != 0 { try!(write!(f, " txwater=0x{:x}", self.txwater()))}
      if self.rxwater() != 0 { try!(write!(f, " rxwater=0x{:x}", self.rxwater()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FIFO Status Register"]
#[derive(PartialEq, Eq)]
pub struct Fsr(pub u32);
impl Fsr {
#[doc="Transmit FIFO Count"]
  #[inline] pub fn txcount(&self) -> bits::B3 {
     (((self.0 as u32) >> 0) & 0x7).into() // [2:0]
  }
#[doc="Transmit FIFO Count"]
  #[inline] pub fn set_txcount<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Receive FIFO Count"]
  #[inline] pub fn rxcount(&self) -> bits::B3 {
     (((self.0 as u32) >> 16) & 0x7).into() // [18:16]
  }
#[doc="Receive FIFO Count"]
  #[inline] pub fn set_rxcount<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Fsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txcount() != 0 { try!(write!(f, " txcount=0x{:x}", self.txcount()))}
      if self.rxcount() != 0 { try!(write!(f, " rxcount=0x{:x}", self.rxcount()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Transmit Command Register"]
#[derive(PartialEq, Eq)]
pub struct Tcr(pub u32);
impl Tcr {
#[doc="Frame Size"]
  #[inline] pub fn framesz(&self) -> bits::B12 {
     (((self.0 as u32) >> 0) & 0xfff).into() // [11:0]
  }
#[doc="Frame Size"]
  #[inline] pub fn set_framesz<V: Into<bits::B12>>(mut self, value: V) -> Self {
     let value: bits::B12 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transfer Width"]
  #[inline] pub fn width(&self) -> bits::B2 {
     (((self.0 as u32) >> 16) & 0x3).into() // [17:16]
  }
#[doc="Transfer Width"]
  #[inline] pub fn set_width<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Transmit Data Mask"]
  #[inline] pub fn txmsk(&self) -> bits::B1 {
     (((self.0 as u32) >> 18) & 0x1).into() // [18]
  }
#[doc="Transmit Data Mask"]
  #[inline] pub fn set_txmsk<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Receive Data Mask"]
  #[inline] pub fn rxmsk(&self) -> bits::B1 {
     (((self.0 as u32) >> 19) & 0x1).into() // [19]
  }
#[doc="Receive Data Mask"]
  #[inline] pub fn set_rxmsk<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Continuing Command"]
  #[inline] pub fn contc(&self) -> bits::B1 {
     (((self.0 as u32) >> 20) & 0x1).into() // [20]
  }
#[doc="Continuing Command"]
  #[inline] pub fn set_contc<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Continuous Transfer"]
  #[inline] pub fn cont(&self) -> bits::B1 {
     (((self.0 as u32) >> 21) & 0x1).into() // [21]
  }
#[doc="Continuous Transfer"]
  #[inline] pub fn set_cont<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Byte Swap"]
  #[inline] pub fn bysw(&self) -> bits::B1 {
     (((self.0 as u32) >> 22) & 0x1).into() // [22]
  }
#[doc="Byte Swap"]
  #[inline] pub fn set_bysw<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="LSB First"]
  #[inline] pub fn lsbf(&self) -> bits::B1 {
     (((self.0 as u32) >> 23) & 0x1).into() // [23]
  }
#[doc="LSB First"]
  #[inline] pub fn set_lsbf<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Peripheral Chip Select"]
  #[inline] pub fn pcs(&self) -> bits::B2 {
     (((self.0 as u32) >> 24) & 0x3).into() // [25:24]
  }
#[doc="Peripheral Chip Select"]
  #[inline] pub fn set_pcs<V: Into<bits::B2>>(mut self, value: V) -> Self {
     let value: bits::B2 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Prescaler Value"]
  #[inline] pub fn prescale(&self) -> bits::B3 {
     (((self.0 as u32) >> 27) & 0x7).into() // [29:27]
  }
#[doc="Prescaler Value"]
  #[inline] pub fn set_prescale<V: Into<bits::B3>>(mut self, value: V) -> Self {
     let value: bits::B3 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Clock Phase"]
  #[inline] pub fn cpha(&self) -> bits::B1 {
     (((self.0 as u32) >> 30) & 0x1).into() // [30]
  }
#[doc="Clock Phase"]
  #[inline] pub fn set_cpha<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Clock Polarity"]
  #[inline] pub fn cpol(&self) -> bits::B1 {
     (((self.0 as u32) >> 31) & 0x1).into() // [31]
  }
#[doc="Clock Polarity"]
  #[inline] pub fn set_cpol<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Tcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.framesz() != 0 { try!(write!(f, " framesz=0x{:x}", self.framesz()))}
      if self.width() != 0 { try!(write!(f, " width=0x{:x}", self.width()))}
      if self.txmsk() != 0 { try!(write!(f, " txmsk"))}
      if self.rxmsk() != 0 { try!(write!(f, " rxmsk"))}
      if self.contc() != 0 { try!(write!(f, " contc"))}
      if self.cont() != 0 { try!(write!(f, " cont"))}
      if self.bysw() != 0 { try!(write!(f, " bysw"))}
      if self.lsbf() != 0 { try!(write!(f, " lsbf"))}
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.prescale() != 0 { try!(write!(f, " prescale=0x{:x}", self.prescale()))}
      if self.cpha() != 0 { try!(write!(f, " cpha"))}
      if self.cpol() != 0 { try!(write!(f, " cpol"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Transmit Data Register"]
#[derive(PartialEq, Eq)]
pub struct Tdr(pub u32);
impl Tdr {
#[doc="Transmit Data"]
  #[inline] pub fn data(&self) -> bits::B32 {
     (((self.0 as u32) >> 0) & 0xffffffff).into() // [31:0]
  }
#[doc="Transmit Data"]
  #[inline] pub fn set_data<V: Into<bits::B32>>(mut self, value: V) -> Self {
     let value: bits::B32 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Receive Status Register"]
#[derive(PartialEq, Eq)]
pub struct Rsr(pub u32);
impl Rsr {
#[doc="Start Of Frame"]
  #[inline] pub fn sof(&self) -> bits::B1 {
     (((self.0 as u32) >> 0) & 0x1).into() // [0]
  }
#[doc="Start Of Frame"]
  #[inline] pub fn set_sof<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="RX FIFO Empty"]
  #[inline] pub fn rxempty(&self) -> bits::B1 {
     (((self.0 as u32) >> 1) & 0x1).into() // [1]
  }
#[doc="RX FIFO Empty"]
  #[inline] pub fn set_rxempty<V: Into<bits::B1>>(mut self, value: V) -> Self {
     let value: bits::B1 = value.into();
     let value: u32 = value.into();
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Rsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sof() != 0 { try!(write!(f, " sof"))}
      if self.rxempty() != 0 { try!(write!(f, " rxempty"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Receive Data Register"]
#[derive(PartialEq, Eq)]
pub struct Rdr(pub u32);
impl Rdr {
#[doc="Receive Data"]
  #[inline] pub fn data(&self) -> bits::B32 {
     (((self.0 as u32) >> 0) & 0xffffffff).into() // [31:0]
  }
#[doc="Receive Data"]
  #[inline] pub fn set_data<V: Into<bits::B32>>(mut self, value: V) -> Self {
     let value: bits::B32 = value.into();
     let value: u32 = value.into();
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Rdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
