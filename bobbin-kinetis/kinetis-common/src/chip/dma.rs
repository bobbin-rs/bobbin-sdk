
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMA Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the SAR register."]
  #[inline] pub fn sar_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x100 + (index << 4)) as *const u32
  }
#[doc="Get the *mut pointer for the SAR register."]
  #[inline] pub fn sar_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x100 + (index << 4)) as *mut u32
  }
#[doc="Read the SAR register."]
  #[inline] pub fn sar(&self, index: usize) -> Sar { 
     assert!(index < 4);
     unsafe {
        Sar(::core::ptr::read_volatile(((self.0 as usize) + 0x100 + (index << 4)) as *const u32))
     }
  }
#[doc="Write the SAR register."]
  #[inline] pub fn set_sar(&self, index: usize, value: Sar) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x100 + (index << 4)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SAR register."]
  #[inline] pub fn with_sar<F: FnOnce(Sar) -> Sar>(&self, index: usize, f: F) -> &Self {
     let tmp = self.sar(index);
     self.set_sar(index, f(tmp))
  }

#[doc="Get the *const pointer for the DAR register."]
  #[inline] pub fn dar_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x104 + (index << 4)) as *const u32
  }
#[doc="Get the *mut pointer for the DAR register."]
  #[inline] pub fn dar_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x104 + (index << 4)) as *mut u32
  }
#[doc="Read the DAR register."]
  #[inline] pub fn dar(&self, index: usize) -> Dar { 
     assert!(index < 4);
     unsafe {
        Dar(::core::ptr::read_volatile(((self.0 as usize) + 0x104 + (index << 4)) as *const u32))
     }
  }
#[doc="Write the DAR register."]
  #[inline] pub fn set_dar(&self, index: usize, value: Dar) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x104 + (index << 4)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DAR register."]
  #[inline] pub fn with_dar<F: FnOnce(Dar) -> Dar>(&self, index: usize, f: F) -> &Self {
     let tmp = self.dar(index);
     self.set_dar(index, f(tmp))
  }

#[doc="Get the *const pointer for the DSR_BCR register."]
  #[inline] pub fn dsr_bcr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x108 + (index << 4)) as *const u32
  }
#[doc="Get the *mut pointer for the DSR_BCR register."]
  #[inline] pub fn dsr_bcr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x108 + (index << 4)) as *mut u32
  }
#[doc="Read the DSR_BCR register."]
  #[inline] pub fn dsr_bcr(&self, index: usize) -> DsrBcr { 
     assert!(index < 4);
     unsafe {
        DsrBcr(::core::ptr::read_volatile(((self.0 as usize) + 0x108 + (index << 4)) as *const u32))
     }
  }
#[doc="Write the DSR_BCR register."]
  #[inline] pub fn set_dsr_bcr(&self, index: usize, value: DsrBcr) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x108 + (index << 4)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DSR_BCR register."]
  #[inline] pub fn with_dsr_bcr<F: FnOnce(DsrBcr) -> DsrBcr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.dsr_bcr(index);
     self.set_dsr_bcr(index, f(tmp))
  }

#[doc="Get the *const pointer for the DSR register."]
  #[inline] pub fn dsr_ptr(&self, index: usize) -> *const u8 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x10b + (index << 4)) as *const u8
  }
#[doc="Get the *mut pointer for the DSR register."]
  #[inline] pub fn dsr_mut(&self, index: usize) -> *mut u8 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x10b + (index << 4)) as *mut u8
  }
#[doc="Read the DSR register."]
  #[inline] pub fn dsr(&self, index: usize) -> Dsr { 
     assert!(index < 4);
     unsafe {
        Dsr(::core::ptr::read_volatile(((self.0 as usize) + 0x10b + (index << 4)) as *const u8))
     }
  }
#[doc="Write the DSR register."]
  #[inline] pub fn set_dsr(&self, index: usize, value: Dsr) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10b + (index << 4)) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DSR register."]
  #[inline] pub fn with_dsr<F: FnOnce(Dsr) -> Dsr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.dsr(index);
     self.set_dsr(index, f(tmp))
  }

#[doc="Get the *const pointer for the DCR register."]
  #[inline] pub fn dcr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x10c + (index << 4)) as *const u32
  }
#[doc="Get the *mut pointer for the DCR register."]
  #[inline] pub fn dcr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x10c + (index << 4)) as *mut u32
  }
#[doc="Read the DCR register."]
  #[inline] pub fn dcr(&self, index: usize) -> Dcr { 
     assert!(index < 4);
     unsafe {
        Dcr(::core::ptr::read_volatile(((self.0 as usize) + 0x10c + (index << 4)) as *const u32))
     }
  }
#[doc="Write the DCR register."]
  #[inline] pub fn set_dcr(&self, index: usize, value: Dcr) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10c + (index << 4)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DCR register."]
  #[inline] pub fn with_dcr<F: FnOnce(Dcr) -> Dcr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.dcr(index);
     self.set_dcr(index, f(tmp))
  }

}

#[doc="Source Address Register"]
#[derive(PartialEq, Eq)]
pub struct Sar(pub u32);
impl Sar {
#[doc="Each SAR contains the byte address used by the DMA controller to read data"]
  #[inline] pub fn sar(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Each SAR contains the byte address used by the DMA controller to read data"]
  #[inline] pub fn set_sar(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Sar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Destination Address Register"]
#[derive(PartialEq, Eq)]
pub struct Dar(pub u32);
impl Dar {
#[doc="Each DAR contains the byte address used by the DMA controller to write data"]
  #[inline] pub fn dar(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Each DAR contains the byte address used by the DMA controller to write data"]
  #[inline] pub fn set_dar(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Status Register / Byte Count Register"]
#[derive(PartialEq, Eq)]
pub struct DsrBcr(pub u32);
impl DsrBcr {
#[doc="This field contains the number of bytes yet to be transferred for a given block"]
  #[inline] pub fn bcr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
#[doc="This field contains the number of bytes yet to be transferred for a given block"]
  #[inline] pub fn set_bcr(mut self, value: u32) -> Self {
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transactions Done"]
  #[inline] pub fn done(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
#[doc="Transactions Done"]
  #[inline] pub fn set_done(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Busy"]
  #[inline] pub fn bsy(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="Busy"]
  #[inline] pub fn set_bsy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Request"]
  #[inline] pub fn req(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="Request"]
  #[inline] pub fn set_req(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Bus Error on Destination"]
  #[inline] pub fn bed(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Bus Error on Destination"]
  #[inline] pub fn set_bed(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Bus Error on Source"]
  #[inline] pub fn bes(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="Bus Error on Source"]
  #[inline] pub fn set_bes(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Configuration Error"]
  #[inline] pub fn ce(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Configuration Error"]
  #[inline] pub fn set_ce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

}
impl ::core::fmt::Display for DsrBcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for DsrBcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bcr() != 0 { try!(write!(f, " bcr=0x{:x}", self.bcr()))}
      if self.done() != 0 { try!(write!(f, " done"))}
      if self.bsy() != 0 { try!(write!(f, " bsy"))}
      if self.req() != 0 { try!(write!(f, " req"))}
      if self.bed() != 0 { try!(write!(f, " bed"))}
      if self.bes() != 0 { try!(write!(f, " bes"))}
      if self.ce() != 0 { try!(write!(f, " ce"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA_DSR0 register."]
#[derive(PartialEq, Eq)]
pub struct Dsr(pub u8);
impl Dsr {
}
impl ::core::fmt::Display for Dsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Control Register"]
#[derive(PartialEq, Eq)]
pub struct Dcr(pub u32);
impl Dcr {
#[doc="Link Channel 2"]
  #[inline] pub fn lch2(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
#[doc="Link Channel 2"]
  #[inline] pub fn set_lch2(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Link Channel 1"]
  #[inline] pub fn lch1(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
#[doc="Link Channel 1"]
  #[inline] pub fn set_lch1(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Link Channel Control"]
  #[inline] pub fn linkcc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
#[doc="Link Channel Control"]
  #[inline] pub fn set_linkcc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Disable Request"]
  #[inline] pub fn d_req(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Disable Request"]
  #[inline] pub fn set_d_req(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Destination Address Modulo"]
  #[inline] pub fn dmod(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
#[doc="Destination Address Modulo"]
  #[inline] pub fn set_dmod(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Source Address Modulo"]
  #[inline] pub fn smod(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
#[doc="Source Address Modulo"]
  #[inline] pub fn set_smod(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Start Transfer"]
  #[inline] pub fn start(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Start Transfer"]
  #[inline] pub fn set_start(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Destination Size"]
  #[inline] pub fn dsize(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x3 // [18:17]
  }
#[doc="Destination Size"]
  #[inline] pub fn set_dsize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Destination Increment"]
  #[inline] pub fn dinc(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="Destination Increment"]
  #[inline] pub fn set_dinc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Source Size"]
  #[inline] pub fn ssize(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="Source Size"]
  #[inline] pub fn set_ssize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Source Increment"]
  #[inline] pub fn sinc(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="Source Increment"]
  #[inline] pub fn set_sinc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Enable asynchronous DMA requests"]
  #[inline] pub fn eadreq(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="Enable asynchronous DMA requests"]
  #[inline] pub fn set_eadreq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Auto-align"]
  #[inline] pub fn aa(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Auto-align"]
  #[inline] pub fn set_aa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Cycle Steal"]
  #[inline] pub fn cs(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="Cycle Steal"]
  #[inline] pub fn set_cs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Enable Peripheral Request"]
  #[inline] pub fn erq(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Enable Peripheral Request"]
  #[inline] pub fn set_erq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Enable Interrupt on Completion of Transfer"]
  #[inline] pub fn eint(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Enable Interrupt on Completion of Transfer"]
  #[inline] pub fn set_eint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Dcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lch2() != 0 { try!(write!(f, " lch2=0x{:x}", self.lch2()))}
      if self.lch1() != 0 { try!(write!(f, " lch1=0x{:x}", self.lch1()))}
      if self.linkcc() != 0 { try!(write!(f, " linkcc=0x{:x}", self.linkcc()))}
      if self.d_req() != 0 { try!(write!(f, " d_req"))}
      if self.dmod() != 0 { try!(write!(f, " dmod=0x{:x}", self.dmod()))}
      if self.smod() != 0 { try!(write!(f, " smod=0x{:x}", self.smod()))}
      if self.start() != 0 { try!(write!(f, " start"))}
      if self.dsize() != 0 { try!(write!(f, " dsize=0x{:x}", self.dsize()))}
      if self.dinc() != 0 { try!(write!(f, " dinc"))}
      if self.ssize() != 0 { try!(write!(f, " ssize=0x{:x}", self.ssize()))}
      if self.sinc() != 0 { try!(write!(f, " sinc"))}
      if self.eadreq() != 0 { try!(write!(f, " eadreq"))}
      if self.aa() != 0 { try!(write!(f, " aa"))}
      if self.cs() != 0 { try!(write!(f, " cs"))}
      if self.erq() != 0 { try!(write!(f, " erq"))}
      if self.eint() != 0 { try!(write!(f, " eint"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(Clone, Copy, PartialEq)]
#[doc="DMA Channel"]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

