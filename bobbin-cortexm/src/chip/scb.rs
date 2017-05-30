pub const SCB: Scb = Scb(0xe000e000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scb(pub u32);

impl Scb {
  pub unsafe fn actlr(&self) -> Actlr { 
     Actlr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_actlr(&self, value: Actlr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_actlr<F: FnOnce(Actlr) -> Actlr>(&self, f: F) {
     let tmp = self.actlr();
     self.set_actlr(f(tmp))
  }

  pub unsafe fn cpuid(&self) -> Cpuid { 
     Cpuid(::core::ptr::read_volatile(((self.0 as usize) + 0xd00) as *const u32))
  }
  pub unsafe fn set_cpuid(&self, value: Cpuid) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd00) as *mut u32, value.0);
  }
  pub unsafe fn with_cpuid<F: FnOnce(Cpuid) -> Cpuid>(&self, f: F) {
     let tmp = self.cpuid();
     self.set_cpuid(f(tmp))
  }

  pub unsafe fn icsr(&self) -> Icsr { 
     Icsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd04) as *const u32))
  }
  pub unsafe fn set_icsr(&self, value: Icsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd04) as *mut u32, value.0);
  }
  pub unsafe fn with_icsr<F: FnOnce(Icsr) -> Icsr>(&self, f: F) {
     let tmp = self.icsr();
     self.set_icsr(f(tmp))
  }

  pub unsafe fn vtor(&self) -> Vtor { 
     Vtor(::core::ptr::read_volatile(((self.0 as usize) + 0xd08) as *const u32))
  }
  pub unsafe fn set_vtor(&self, value: Vtor) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd08) as *mut u32, value.0);
  }
  pub unsafe fn with_vtor<F: FnOnce(Vtor) -> Vtor>(&self, f: F) {
     let tmp = self.vtor();
     self.set_vtor(f(tmp))
  }

  pub unsafe fn aircr(&self) -> Aircr { 
     Aircr(::core::ptr::read_volatile(((self.0 as usize) + 0xd0c) as *const u32))
  }
  pub unsafe fn set_aircr(&self, value: Aircr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd0c) as *mut u32, value.0);
  }
  pub unsafe fn with_aircr<F: FnOnce(Aircr) -> Aircr>(&self, f: F) {
     let tmp = self.aircr();
     self.set_aircr(f(tmp))
  }

  pub unsafe fn scr(&self) -> Scr { 
     Scr(::core::ptr::read_volatile(((self.0 as usize) + 0xd0c) as *const u32))
  }
  pub unsafe fn set_scr(&self, value: Scr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd0c) as *mut u32, value.0);
  }
  pub unsafe fn with_scr<F: FnOnce(Scr) -> Scr>(&self, f: F) {
     let tmp = self.scr();
     self.set_scr(f(tmp))
  }

  pub unsafe fn ccr(&self) -> Ccr { 
     Ccr(::core::ptr::read_volatile(((self.0 as usize) + 0xd14) as *const u32))
  }
  pub unsafe fn set_ccr(&self, value: Ccr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd14) as *mut u32, value.0);
  }
  pub unsafe fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) {
     let tmp = self.ccr();
     self.set_ccr(f(tmp))
  }

  pub unsafe fn shpr1(&self) -> Shpr1 { 
     Shpr1(::core::ptr::read_volatile(((self.0 as usize) + 0xd18) as *const u32))
  }
  pub unsafe fn set_shpr1(&self, value: Shpr1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd18) as *mut u32, value.0);
  }
  pub unsafe fn with_shpr1<F: FnOnce(Shpr1) -> Shpr1>(&self, f: F) {
     let tmp = self.shpr1();
     self.set_shpr1(f(tmp))
  }

  pub unsafe fn shpr2(&self) -> Shpr2 { 
     Shpr2(::core::ptr::read_volatile(((self.0 as usize) + 0xd1c) as *const u32))
  }
  pub unsafe fn set_shpr2(&self, value: Shpr2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd1c) as *mut u32, value.0);
  }
  pub unsafe fn with_shpr2<F: FnOnce(Shpr2) -> Shpr2>(&self, f: F) {
     let tmp = self.shpr2();
     self.set_shpr2(f(tmp))
  }

  pub unsafe fn shpr3(&self) -> Shpr3 { 
     Shpr3(::core::ptr::read_volatile(((self.0 as usize) + 0xd20) as *const u32))
  }
  pub unsafe fn set_shpr3(&self, value: Shpr3) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd20) as *mut u32, value.0);
  }
  pub unsafe fn with_shpr3<F: FnOnce(Shpr3) -> Shpr3>(&self, f: F) {
     let tmp = self.shpr3();
     self.set_shpr3(f(tmp))
  }

  pub unsafe fn shcsr(&self) -> Shcsr { 
     Shcsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd24) as *const u32))
  }
  pub unsafe fn set_shcsr(&self, value: Shcsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd24) as *mut u32, value.0);
  }
  pub unsafe fn with_shcsr<F: FnOnce(Shcsr) -> Shcsr>(&self, f: F) {
     let tmp = self.shcsr();
     self.set_shcsr(f(tmp))
  }

  pub unsafe fn cfsr(&self) -> Cfsr { 
     Cfsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd28) as *const u32))
  }
  pub unsafe fn set_cfsr(&self, value: Cfsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd28) as *mut u32, value.0);
  }
  pub unsafe fn with_cfsr<F: FnOnce(Cfsr) -> Cfsr>(&self, f: F) {
     let tmp = self.cfsr();
     self.set_cfsr(f(tmp))
  }

  pub unsafe fn mmfsr(&self) -> Mmfsr { 
     Mmfsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd28) as *const u8))
  }
  pub unsafe fn set_mmfsr(&self, value: Mmfsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd28) as *mut u8, value.0);
  }
  pub unsafe fn with_mmfsr<F: FnOnce(Mmfsr) -> Mmfsr>(&self, f: F) {
     let tmp = self.mmfsr();
     self.set_mmfsr(f(tmp))
  }

  pub unsafe fn bfsr(&self) -> Bfsr { 
     Bfsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd29) as *const u8))
  }
  pub unsafe fn set_bfsr(&self, value: Bfsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd29) as *mut u8, value.0);
  }
  pub unsafe fn with_bfsr<F: FnOnce(Bfsr) -> Bfsr>(&self, f: F) {
     let tmp = self.bfsr();
     self.set_bfsr(f(tmp))
  }

  pub unsafe fn ufsr(&self) -> Ufsr { 
     Ufsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd2a) as *const u16))
  }
  pub unsafe fn set_ufsr(&self, value: Ufsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd2a) as *mut u16, value.0);
  }
  pub unsafe fn with_ufsr<F: FnOnce(Ufsr) -> Ufsr>(&self, f: F) {
     let tmp = self.ufsr();
     self.set_ufsr(f(tmp))
  }

  pub unsafe fn hfsr(&self) -> Hfsr { 
     Hfsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd2c) as *const u32))
  }
  pub unsafe fn set_hfsr(&self, value: Hfsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd2c) as *mut u32, value.0);
  }
  pub unsafe fn with_hfsr<F: FnOnce(Hfsr) -> Hfsr>(&self, f: F) {
     let tmp = self.hfsr();
     self.set_hfsr(f(tmp))
  }

  pub unsafe fn mmfar(&self) -> Mmfar { 
     Mmfar(::core::ptr::read_volatile(((self.0 as usize) + 0xd34) as *const u32))
  }
  pub unsafe fn set_mmfar(&self, value: Mmfar) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd34) as *mut u32, value.0);
  }
  pub unsafe fn with_mmfar<F: FnOnce(Mmfar) -> Mmfar>(&self, f: F) {
     let tmp = self.mmfar();
     self.set_mmfar(f(tmp))
  }

  pub unsafe fn bfar(&self) -> Bfar { 
     Bfar(::core::ptr::read_volatile(((self.0 as usize) + 0xd38) as *const u32))
  }
  pub unsafe fn set_bfar(&self, value: Bfar) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd38) as *mut u32, value.0);
  }
  pub unsafe fn with_bfar<F: FnOnce(Bfar) -> Bfar>(&self, f: F) {
     let tmp = self.bfar();
     self.set_bfar(f(tmp))
  }

  pub unsafe fn afsr(&self) -> Afsr { 
     Afsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd3c) as *const u32))
  }
  pub unsafe fn set_afsr(&self, value: Afsr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd3c) as *mut u32, value.0);
  }
  pub unsafe fn with_afsr<F: FnOnce(Afsr) -> Afsr>(&self, f: F) {
     let tmp = self.afsr();
     self.set_afsr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Actlr(pub u32);

impl Actlr {
  pub fn disfold(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_disfold(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn disdefwbuf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_disdefwbuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn dismcycint(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_dismcycint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Actlr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Actlr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.disfold() != 0 { try!(write!(f, " disfold"))}
      if self.disdefwbuf() != 0 { try!(write!(f, " disdefwbuf"))}
      if self.dismcycint() != 0 { try!(write!(f, " dismcycint"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cpuid(pub u32);

impl Cpuid {
  pub fn implementer(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_implementer(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

  pub fn variant(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
  pub fn set_variant(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

  pub fn constant(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_constant(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn partno(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xfff // [15:4]
  }
  pub fn set_partno(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 4);
     self.0 |= value << 4;
     self
  }

  pub fn revision(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_revision(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Cpuid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cpuid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.implementer() != 0 { try!(write!(f, " implementer=0x{:x}", self.implementer()))}
      if self.variant() != 0 { try!(write!(f, " variant=0x{:x}", self.variant()))}
      if self.constant() != 0 { try!(write!(f, " constant=0x{:x}", self.constant()))}
      if self.partno() != 0 { try!(write!(f, " partno=0x{:x}", self.partno()))}
      if self.revision() != 0 { try!(write!(f, " revision=0x{:x}", self.revision()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Icsr(pub u32);

impl Icsr {
  pub fn nmipendset(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_nmipendset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn pendsvset(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_pendsvset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn pendsvclr(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_pendsvclr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn pendstset(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_pendstset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn pendstclr(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_pendstclr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn isrpending(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_isrpending(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn vectpending(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3f // [17:12]
  }
  pub fn set_vectpending(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 12);
     self.0 |= value << 12;
     self
  }

  pub fn rettobase(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_rettobase(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn vectactive(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_vectactive(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Icsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Icsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.nmipendset() != 0 { try!(write!(f, " nmipendset"))}
      if self.pendsvset() != 0 { try!(write!(f, " pendsvset"))}
      if self.pendsvclr() != 0 { try!(write!(f, " pendsvclr"))}
      if self.pendstset() != 0 { try!(write!(f, " pendstset"))}
      if self.pendstclr() != 0 { try!(write!(f, " pendstclr"))}
      if self.isrpending() != 0 { try!(write!(f, " isrpending"))}
      if self.vectpending() != 0 { try!(write!(f, " vectpending=0x{:x}", self.vectpending()))}
      if self.rettobase() != 0 { try!(write!(f, " rettobase"))}
      if self.vectactive() != 0 { try!(write!(f, " vectactive=0x{:x}", self.vectactive()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Vtor(pub u32);

impl Vtor {
  pub fn tbloff(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1ffffff // [31:7]
  }
  pub fn set_tbloff(mut self, value: u32) -> Self {
     assert!((value & !0x1ffffff) == 0);
     self.0 &= !(0x1ffffff << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Vtor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Vtor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tbloff() != 0 { try!(write!(f, " tbloff=0x{:x}", self.tbloff()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Aircr(pub u32);

impl Aircr {
  pub fn vectkey(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_vectkey(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn endianness(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_endianness(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn prigroup(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_prigroup(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn sysresetreq(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_sysresetreq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn vectclractive(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_vectclractive(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn vectreset(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_vectreset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Aircr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Aircr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.vectkey() != 0 { try!(write!(f, " vectkey=0x{:x}", self.vectkey()))}
      if self.endianness() != 0 { try!(write!(f, " endianness"))}
      if self.prigroup() != 0 { try!(write!(f, " prigroup=0x{:x}", self.prigroup()))}
      if self.sysresetreq() != 0 { try!(write!(f, " sysresetreq"))}
      if self.vectclractive() != 0 { try!(write!(f, " vectclractive"))}
      if self.vectreset() != 0 { try!(write!(f, " vectreset"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Scr(pub u32);

impl Scr {
  pub fn sevonpend(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_sevonpend(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn sleepdeep(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_sleepdeep(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn sleeponexit(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_sleeponexit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for Scr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Scr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sevonpend() != 0 { try!(write!(f, " sevonpend"))}
      if self.sleepdeep() != 0 { try!(write!(f, " sleepdeep"))}
      if self.sleeponexit() != 0 { try!(write!(f, " sleeponexit"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ccr(pub u32);

impl Ccr {
  pub fn stkalign(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_stkalign(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn bfhfnmign(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_bfhfnmign(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn div_0_trp(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_div_0_trp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn unalign_trp(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_unalign_trp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn usersetmpend(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_usersetmpend(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn nonbasethrdena(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_nonbasethrdena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.stkalign() != 0 { try!(write!(f, " stkalign"))}
      if self.bfhfnmign() != 0 { try!(write!(f, " bfhfnmign"))}
      if self.div_0_trp() != 0 { try!(write!(f, " div_0_trp"))}
      if self.unalign_trp() != 0 { try!(write!(f, " unalign_trp"))}
      if self.usersetmpend() != 0 { try!(write!(f, " usersetmpend"))}
      if self.nonbasethrdena() != 0 { try!(write!(f, " nonbasethrdena"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Shpr1(pub u32);

impl Shpr1 {
  pub fn pri_6(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_pri_6(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn pri_5(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_pri_5(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn pri_4(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_pri_4(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Shpr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Shpr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pri_6() != 0 { try!(write!(f, " pri_6=0x{:x}", self.pri_6()))}
      if self.pri_5() != 0 { try!(write!(f, " pri_5=0x{:x}", self.pri_5()))}
      if self.pri_4() != 0 { try!(write!(f, " pri_4=0x{:x}", self.pri_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Shpr2(pub u32);

impl Shpr2 {
  pub fn pri_11(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_pri_11(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}

impl ::core::fmt::Display for Shpr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Shpr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pri_11() != 0 { try!(write!(f, " pri_11=0x{:x}", self.pri_11()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Shpr3(pub u32);

impl Shpr3 {
  pub fn pri_15(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
  pub fn set_pri_15(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

  pub fn pri_14(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_pri_14(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Shpr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Shpr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pri_15() != 0 { try!(write!(f, " pri_15=0x{:x}", self.pri_15()))}
      if self.pri_14() != 0 { try!(write!(f, " pri_14=0x{:x}", self.pri_14()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Shcsr(pub u32);

impl Shcsr {
  pub fn usgfaultena(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_usgfaultena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn busfaultena(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_busfaultena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn memfaultena(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_memfaultena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn svcallpended(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_svcallpended(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn busfaultpended(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_busfaultpended(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn memfaultpended(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_memfaultpended(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn usgfaultpended(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_usgfaultpended(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn systickact(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_systickact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn pendsvact(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_pendsvact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn monitoract(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_monitoract(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn svcallact(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_svcallact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn usgfaultact(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_usgfaultact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn busfaultact(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_busfaultact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn memfaultact(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_memfaultact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Shcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Shcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.usgfaultena() != 0 { try!(write!(f, " usgfaultena"))}
      if self.busfaultena() != 0 { try!(write!(f, " busfaultena"))}
      if self.memfaultena() != 0 { try!(write!(f, " memfaultena"))}
      if self.svcallpended() != 0 { try!(write!(f, " svcallpended"))}
      if self.busfaultpended() != 0 { try!(write!(f, " busfaultpended"))}
      if self.memfaultpended() != 0 { try!(write!(f, " memfaultpended"))}
      if self.usgfaultpended() != 0 { try!(write!(f, " usgfaultpended"))}
      if self.systickact() != 0 { try!(write!(f, " systickact"))}
      if self.pendsvact() != 0 { try!(write!(f, " pendsvact"))}
      if self.monitoract() != 0 { try!(write!(f, " monitoract"))}
      if self.svcallact() != 0 { try!(write!(f, " svcallact"))}
      if self.usgfaultact() != 0 { try!(write!(f, " usgfaultact"))}
      if self.busfaultact() != 0 { try!(write!(f, " busfaultact"))}
      if self.memfaultact() != 0 { try!(write!(f, " memfaultact"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cfsr(pub u32);

impl Cfsr {
}

impl ::core::fmt::Display for Cfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mmfsr(pub u8);

impl Mmfsr {
  pub fn mmarvalid(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_mmarvalid(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn mstkerr(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_mstkerr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn munstkerr(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_munstkerr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn daccviol(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_daccviol(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn iaccviol(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_iaccviol(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Mmfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mmfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mmarvalid() != 0 { try!(write!(f, " mmarvalid"))}
      if self.mstkerr() != 0 { try!(write!(f, " mstkerr"))}
      if self.munstkerr() != 0 { try!(write!(f, " munstkerr"))}
      if self.daccviol() != 0 { try!(write!(f, " daccviol"))}
      if self.iaccviol() != 0 { try!(write!(f, " iaccviol"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Bfsr(pub u8);

impl Bfsr {
  pub fn bfarvalid(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_bfarvalid(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn stkerr(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_stkerr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn unstkerr(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_unstkerr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn impreciserr(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_impreciserr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn preciserr(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_preciserr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn ibuserr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_ibuserr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Bfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Bfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bfarvalid() != 0 { try!(write!(f, " bfarvalid"))}
      if self.stkerr() != 0 { try!(write!(f, " stkerr"))}
      if self.unstkerr() != 0 { try!(write!(f, " unstkerr"))}
      if self.impreciserr() != 0 { try!(write!(f, " impreciserr"))}
      if self.preciserr() != 0 { try!(write!(f, " preciserr"))}
      if self.ibuserr() != 0 { try!(write!(f, " ibuserr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ufsr(pub u16);

impl Ufsr {
  pub fn divbyzero(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
  pub fn set_divbyzero(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn unaligned(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  pub fn set_unaligned(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn nocp(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  pub fn set_nocp(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn invpc(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  pub fn set_invpc(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn invstate(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  pub fn set_invstate(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn undefinstr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  pub fn set_undefinstr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ufsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ufsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.divbyzero() != 0 { try!(write!(f, " divbyzero"))}
      if self.unaligned() != 0 { try!(write!(f, " unaligned"))}
      if self.nocp() != 0 { try!(write!(f, " nocp"))}
      if self.invpc() != 0 { try!(write!(f, " invpc"))}
      if self.invstate() != 0 { try!(write!(f, " invstate"))}
      if self.undefinstr() != 0 { try!(write!(f, " undefinstr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Hfsr(pub u32);

impl Hfsr {
  pub fn debugevt(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_debugevt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

  pub fn forced(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_forced(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn vecttbl(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_vecttbl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for Hfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Hfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.debugevt() != 0 { try!(write!(f, " debugevt"))}
      if self.forced() != 0 { try!(write!(f, " forced"))}
      if self.vecttbl() != 0 { try!(write!(f, " vecttbl"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mmfar(pub u32);

impl Mmfar {
  pub fn address(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_address(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Mmfar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mmfar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Bfar(pub u32);

impl Bfar {
  pub fn address(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_address(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Bfar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Bfar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Afsr(pub u32);

impl Afsr {
  pub fn impdef(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_impdef(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Afsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Afsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

