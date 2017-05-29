pub const C_ADC12: CAdc = CAdc(0x50000300);
pub const C_ADC34: CAdc = CAdc(0x50000700);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CAdc(pub u32);

impl CAdc {
  pub unsafe fn csr(&self) -> Csr { 
     Csr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }

  pub unsafe fn ccr(&self) -> Ccr { 
     Ccr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_ccr(&mut self, value: Ccr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&mut self, f: F) {
     let tmp = self.ccr();
     self.set_ccr(f(tmp))
  }

  pub unsafe fn cdr(&self) -> Cdr { 
     Cdr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }

}

#[derive(PartialEq, Eq)]
pub struct Csr(pub u32);

impl Csr {
  pub fn addrdy_mst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_addrdy_mst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn eosmp_mst(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_eosmp_mst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn eoc_mst(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_eoc_mst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn eos_mst(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_eos_mst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn ovr_mst(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_ovr_mst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn jeoc_mst(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_jeoc_mst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn jeos_mst(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_jeos_mst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn awd1_mst(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_awd1_mst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn awd2_mst(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_awd2_mst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn awd3_mst(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_awd3_mst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn jqovf_mst(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_jqovf_mst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn adrdy_slv(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_adrdy_slv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn eosmp_slv(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_eosmp_slv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn eoc_slv(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_eoc_slv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn eos_slv(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_eos_slv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn ovr_slv(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_ovr_slv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn jeoc_slv(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_jeoc_slv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn jeos_slv(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_jeos_slv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn awd1_slv(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_awd1_slv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn awd2_slv(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_awd2_slv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn awd3_slv(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_awd3_slv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn jqovf_slv(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_jqovf_slv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addrdy_mst() != 0 { try!(write!(f, " addrdy_mst"))}
      if self.eosmp_mst() != 0 { try!(write!(f, " eosmp_mst"))}
      if self.eoc_mst() != 0 { try!(write!(f, " eoc_mst"))}
      if self.eos_mst() != 0 { try!(write!(f, " eos_mst"))}
      if self.ovr_mst() != 0 { try!(write!(f, " ovr_mst"))}
      if self.jeoc_mst() != 0 { try!(write!(f, " jeoc_mst"))}
      if self.jeos_mst() != 0 { try!(write!(f, " jeos_mst"))}
      if self.awd1_mst() != 0 { try!(write!(f, " awd1_mst"))}
      if self.awd2_mst() != 0 { try!(write!(f, " awd2_mst"))}
      if self.awd3_mst() != 0 { try!(write!(f, " awd3_mst"))}
      if self.jqovf_mst() != 0 { try!(write!(f, " jqovf_mst"))}
      if self.adrdy_slv() != 0 { try!(write!(f, " adrdy_slv"))}
      if self.eosmp_slv() != 0 { try!(write!(f, " eosmp_slv"))}
      if self.eoc_slv() != 0 { try!(write!(f, " eoc_slv"))}
      if self.eos_slv() != 0 { try!(write!(f, " eos_slv"))}
      if self.ovr_slv() != 0 { try!(write!(f, " ovr_slv"))}
      if self.jeoc_slv() != 0 { try!(write!(f, " jeoc_slv"))}
      if self.jeos_slv() != 0 { try!(write!(f, " jeos_slv"))}
      if self.awd1_slv() != 0 { try!(write!(f, " awd1_slv"))}
      if self.awd2_slv() != 0 { try!(write!(f, " awd2_slv"))}
      if self.awd3_slv() != 0 { try!(write!(f, " awd3_slv"))}
      if self.jqovf_slv() != 0 { try!(write!(f, " jqovf_slv"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ccr(pub u32);

impl Ccr {
  pub fn mult(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  pub fn set_mult(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn delay(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_delay(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dmacfg(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_dmacfg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn mdma(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
  pub fn set_mdma(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn ckmode(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  pub fn set_ckmode(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn vrefen(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_vrefen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn tsen(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_tsen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn vbaten(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_vbaten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
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
      if self.mult() != 0 { try!(write!(f, " mult=0x{:x}", self.mult()))}
      if self.delay() != 0 { try!(write!(f, " delay=0x{:x}", self.delay()))}
      if self.dmacfg() != 0 { try!(write!(f, " dmacfg"))}
      if self.mdma() != 0 { try!(write!(f, " mdma=0x{:x}", self.mdma()))}
      if self.ckmode() != 0 { try!(write!(f, " ckmode=0x{:x}", self.ckmode()))}
      if self.vrefen() != 0 { try!(write!(f, " vrefen"))}
      if self.tsen() != 0 { try!(write!(f, " tsen"))}
      if self.vbaten() != 0 { try!(write!(f, " vbaten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cdr(pub u32);

impl Cdr {
  pub fn rdata_slv(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  pub fn set_rdata_slv(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rdata_mst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_rdata_mst(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Cdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rdata_slv() != 0 { try!(write!(f, " rdata_slv=0x{:x}", self.rdata_slv()))}
      if self.rdata_mst() != 0 { try!(write!(f, " rdata_mst=0x{:x}", self.rdata_mst()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

