pub const MPU: Mpu = Mpu(0xe000ed90);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mpu(pub u32);

impl Mpu {
  pub unsafe fn mpu_type(&self) -> MpuType { 
     MpuType(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_mpu_type(&mut self, value: MpuType) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_mpu_type<F: FnOnce(MpuType) -> MpuType>(&mut self, f: F) {
     let tmp = self.mpu_type();
     self.set_mpu_type(f(tmp))
  }

  pub unsafe fn mpu_ctrl(&self) -> MpuCtrl { 
     MpuCtrl(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_mpu_ctrl(&mut self, value: MpuCtrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_mpu_ctrl<F: FnOnce(MpuCtrl) -> MpuCtrl>(&mut self, f: F) {
     let tmp = self.mpu_ctrl();
     self.set_mpu_ctrl(f(tmp))
  }

  pub unsafe fn mpu_rnr(&self) -> MpuRnr { 
     MpuRnr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_mpu_rnr(&mut self, value: MpuRnr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_mpu_rnr<F: FnOnce(MpuRnr) -> MpuRnr>(&mut self, f: F) {
     let tmp = self.mpu_rnr();
     self.set_mpu_rnr(f(tmp))
  }

  pub unsafe fn mpu_rbar(&self) -> MpuRbar { 
     MpuRbar(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_mpu_rbar(&mut self, value: MpuRbar) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_mpu_rbar<F: FnOnce(MpuRbar) -> MpuRbar>(&mut self, f: F) {
     let tmp = self.mpu_rbar();
     self.set_mpu_rbar(f(tmp))
  }

  pub unsafe fn mpu_rasr(&self) -> MpuRasr { 
     MpuRasr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_mpu_rasr(&mut self, value: MpuRasr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_mpu_rasr<F: FnOnce(MpuRasr) -> MpuRasr>(&mut self, f: F) {
     let tmp = self.mpu_rasr();
     self.set_mpu_rasr(f(tmp))
  }

  pub unsafe fn mpu_rbar_a1(&self) -> MpuRbarA1 { 
     MpuRbarA1(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_mpu_rbar_a1(&mut self, value: MpuRbarA1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_mpu_rbar_a1<F: FnOnce(MpuRbarA1) -> MpuRbarA1>(&mut self, f: F) {
     let tmp = self.mpu_rbar_a1();
     self.set_mpu_rbar_a1(f(tmp))
  }

  pub unsafe fn mpu_rasr_a1(&self) -> MpuRasrA1 { 
     MpuRasrA1(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_mpu_rasr_a1(&mut self, value: MpuRasrA1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_mpu_rasr_a1<F: FnOnce(MpuRasrA1) -> MpuRasrA1>(&mut self, f: F) {
     let tmp = self.mpu_rasr_a1();
     self.set_mpu_rasr_a1(f(tmp))
  }

  pub unsafe fn mpu_rbar_a2(&self) -> MpuRbarA2 { 
     MpuRbarA2(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
  }
  pub unsafe fn set_mpu_rbar_a2(&mut self, value: MpuRbarA2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
  }
  pub unsafe fn with_mpu_rbar_a2<F: FnOnce(MpuRbarA2) -> MpuRbarA2>(&mut self, f: F) {
     let tmp = self.mpu_rbar_a2();
     self.set_mpu_rbar_a2(f(tmp))
  }

  pub unsafe fn mpu_rasr_a2(&self) -> MpuRasrA2 { 
     MpuRasrA2(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_mpu_rasr_a2(&mut self, value: MpuRasrA2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_mpu_rasr_a2<F: FnOnce(MpuRasrA2) -> MpuRasrA2>(&mut self, f: F) {
     let tmp = self.mpu_rasr_a2();
     self.set_mpu_rasr_a2(f(tmp))
  }

  pub unsafe fn mpu_rbar_a3(&self) -> MpuRbarA3 { 
     MpuRbarA3(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
  }
  pub unsafe fn set_mpu_rbar_a3(&mut self, value: MpuRbarA3) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
  }
  pub unsafe fn with_mpu_rbar_a3<F: FnOnce(MpuRbarA3) -> MpuRbarA3>(&mut self, f: F) {
     let tmp = self.mpu_rbar_a3();
     self.set_mpu_rbar_a3(f(tmp))
  }

  pub unsafe fn mpu_rasr_a3(&self) -> MpuRasrA3 { 
     MpuRasrA3(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
  }
  pub unsafe fn set_mpu_rasr_a3(&mut self, value: MpuRasrA3) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
  }
  pub unsafe fn with_mpu_rasr_a3<F: FnOnce(MpuRasrA3) -> MpuRasrA3>(&mut self, f: F) {
     let tmp = self.mpu_rasr_a3();
     self.set_mpu_rasr_a3(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct MpuType(pub u32);

impl MpuType {
  pub fn iregion(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
  pub fn set_iregion(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn dregion(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_dregion(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn separate(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_separate(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for MpuType {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for MpuType {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.iregion() != 0 { try!(write!(f, " iregion=0x{:x}", self.iregion()))}
      if self.dregion() != 0 { try!(write!(f, " dregion=0x{:x}", self.dregion()))}
      if self.separate() != 0 { try!(write!(f, " separate"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct MpuCtrl(pub u32);

impl MpuCtrl {
  pub fn privdefena(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_privdefena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn hfnmiena(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_hfnmiena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for MpuCtrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for MpuCtrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.privdefena() != 0 { try!(write!(f, " privdefena"))}
      if self.hfnmiena() != 0 { try!(write!(f, " hfnmiena"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct MpuRnr(pub u32);

impl MpuRnr {
  pub fn region(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_region(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for MpuRnr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for MpuRnr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.region() != 0 { try!(write!(f, " region=0x{:x}", self.region()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct MpuRbar(pub u32);

impl MpuRbar {
  pub fn addr(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x7ffffff // [31:5]
  }
  pub fn set_addr(mut self, value: u32) -> Self {
     assert!((value & !0x7ffffff) == 0);
     self.0 &= !(0x7ffffff << 5);
     self.0 |= value << 5;
     self
  }

  pub fn valid(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_valid(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn region(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_region(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for MpuRbar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for MpuRbar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self.valid() != 0 { try!(write!(f, " valid"))}
      if self.region() != 0 { try!(write!(f, " region=0x{:x}", self.region()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct MpuRasr(pub u32);

impl MpuRasr {
  pub fn xn(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_xn(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn ap(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_ap(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn tex(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x7 // [21:19]
  }
  pub fn set_tex(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn c(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_c(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn b(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_b(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn s(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_s(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn srd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
  pub fn set_srd(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

  pub fn size(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1f // [5:1]
  }
  pub fn set_size(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 1);
     self.0 |= value << 1;
     self
  }

  pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for MpuRasr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for MpuRasr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.xn() != 0 { try!(write!(f, " xn"))}
      if self.ap() != 0 { try!(write!(f, " ap=0x{:x}", self.ap()))}
      if self.tex() != 0 { try!(write!(f, " tex=0x{:x}", self.tex()))}
      if self.c() != 0 { try!(write!(f, " c"))}
      if self.b() != 0 { try!(write!(f, " b"))}
      if self.s() != 0 { try!(write!(f, " s"))}
      if self.srd() != 0 { try!(write!(f, " srd=0x{:x}", self.srd()))}
      if self.size() != 0 { try!(write!(f, " size=0x{:x}", self.size()))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct MpuRbarA1(pub u32);

impl MpuRbarA1 {
}

impl ::core::fmt::Display for MpuRbarA1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for MpuRbarA1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct MpuRasrA1(pub u32);

impl MpuRasrA1 {
}

impl ::core::fmt::Display for MpuRasrA1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for MpuRasrA1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct MpuRbarA2(pub u32);

impl MpuRbarA2 {
}

impl ::core::fmt::Display for MpuRbarA2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for MpuRbarA2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct MpuRasrA2(pub u32);

impl MpuRasrA2 {
}

impl ::core::fmt::Display for MpuRasrA2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for MpuRasrA2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct MpuRbarA3(pub u32);

impl MpuRbarA3 {
}

impl ::core::fmt::Display for MpuRbarA3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for MpuRbarA3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct MpuRasrA3(pub u32);

impl MpuRasrA3 {
}

impl ::core::fmt::Display for MpuRasrA3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for MpuRasrA3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

