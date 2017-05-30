pub const FLASH: Flash = Flash(0x40022000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Flash(pub u32);
impl Flash {
  pub fn acr(&self) -> Acr { 
     unsafe {
       Acr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  pub fn set_acr(&self, value: Acr) -> &Flash {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  pub fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Flash {
     let tmp = self.acr();
     self.set_acr(f(tmp))
  }

  pub fn pecr(&self) -> Pecr { 
     unsafe {
       Pecr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  pub fn set_pecr(&self, value: Pecr) -> &Flash {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  pub fn with_pecr<F: FnOnce(Pecr) -> Pecr>(&self, f: F) -> &Flash {
     let tmp = self.pecr();
     self.set_pecr(f(tmp))
  }

  pub fn set_pdkeyr(&self, value: Pdkeyr) -> &Flash {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

  pub fn set_pekeyr(&self, value: Pekeyr) -> &Flash {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }

  pub fn set_prgkeyr(&self, value: Prgkeyr) -> &Flash {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }

  pub fn set_optkeyr(&self, value: Optkeyr) -> &Flash {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

  pub fn sr(&self) -> Sr { 
     unsafe {
       Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  pub fn set_sr(&self, value: Sr) -> &Flash {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Flash {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

  pub fn obr(&self) -> Obr { 
     unsafe {
       Obr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }

  pub fn wrpr(&self) -> Wrpr { 
     unsafe {
       Wrpr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  pub fn set_wrpr(&self, value: Wrpr) -> &Flash {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  pub fn with_wrpr<F: FnOnce(Wrpr) -> Wrpr>(&self, f: F) -> &Flash {
     let tmp = self.wrpr();
     self.set_wrpr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
  pub fn latency(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_latency(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn prften(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_prften(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn sleep_pd(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_sleep_pd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn run_pd(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_run_pd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn desab_buf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_desab_buf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn pre_read(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_pre_read(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Acr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Acr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.latency() != 0 { try!(write!(f, " latency"))}
      if self.prften() != 0 { try!(write!(f, " prften"))}
      if self.sleep_pd() != 0 { try!(write!(f, " sleep_pd"))}
      if self.run_pd() != 0 { try!(write!(f, " run_pd"))}
      if self.desab_buf() != 0 { try!(write!(f, " desab_buf"))}
      if self.pre_read() != 0 { try!(write!(f, " pre_read"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pecr(pub u32);
impl Pecr {
  pub fn pelock(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_pelock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn prglock(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_prglock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn optlock(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_optlock(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn prog(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_prog(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_data(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn ftdw(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_ftdw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn erase(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_erase(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn fprg(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_fprg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn parallelbank(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_parallelbank(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn eopie(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_eopie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn errie(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_errie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn obl_launch(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_obl_launch(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}
impl ::core::fmt::Display for Pecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pelock() != 0 { try!(write!(f, " pelock"))}
      if self.prglock() != 0 { try!(write!(f, " prglock"))}
      if self.optlock() != 0 { try!(write!(f, " optlock"))}
      if self.prog() != 0 { try!(write!(f, " prog"))}
      if self.data() != 0 { try!(write!(f, " data"))}
      if self.ftdw() != 0 { try!(write!(f, " ftdw"))}
      if self.erase() != 0 { try!(write!(f, " erase"))}
      if self.fprg() != 0 { try!(write!(f, " fprg"))}
      if self.parallelbank() != 0 { try!(write!(f, " parallelbank"))}
      if self.eopie() != 0 { try!(write!(f, " eopie"))}
      if self.errie() != 0 { try!(write!(f, " errie"))}
      if self.obl_launch() != 0 { try!(write!(f, " obl_launch"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pdkeyr(pub u32);
impl Pdkeyr {
  pub fn pdkeyr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_pdkeyr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Pdkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pdkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pekeyr(pub u32);
impl Pekeyr {
  pub fn pekeyr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_pekeyr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Pekeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pekeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Prgkeyr(pub u32);
impl Prgkeyr {
  pub fn prgkeyr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_prgkeyr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Prgkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prgkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Optkeyr(pub u32);
impl Optkeyr {
  pub fn optkeyr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_optkeyr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Optkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Optkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
  pub fn bsy(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_bsy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn eop(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_eop(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn endhv(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_endhv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn ready(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_ready(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn wrperr(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_wrperr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn pgaerr(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_pgaerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn sizerr(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_sizerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn optverr(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_optverr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn rderr(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_rderr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn notzeroerr(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_notzeroerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn fwwerr(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_fwwerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
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
      if self.bsy() != 0 { try!(write!(f, " bsy"))}
      if self.eop() != 0 { try!(write!(f, " eop"))}
      if self.endhv() != 0 { try!(write!(f, " endhv"))}
      if self.ready() != 0 { try!(write!(f, " ready"))}
      if self.wrperr() != 0 { try!(write!(f, " wrperr"))}
      if self.pgaerr() != 0 { try!(write!(f, " pgaerr"))}
      if self.sizerr() != 0 { try!(write!(f, " sizerr"))}
      if self.optverr() != 0 { try!(write!(f, " optverr"))}
      if self.rderr() != 0 { try!(write!(f, " rderr"))}
      if self.notzeroerr() != 0 { try!(write!(f, " notzeroerr"))}
      if self.fwwerr() != 0 { try!(write!(f, " fwwerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Obr(pub u32);
impl Obr {
  pub fn rdprt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_rdprt(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn bor_lev(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_bor_lev(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  pub fn sprmod(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_sprmod(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Obr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Obr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rdprt() != 0 { try!(write!(f, " rdprt=0x{:x}", self.rdprt()))}
      if self.bor_lev() != 0 { try!(write!(f, " bor_lev=0x{:x}", self.bor_lev()))}
      if self.sprmod() != 0 { try!(write!(f, " sprmod"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Wrpr(pub u32);
impl Wrpr {
  pub fn wrp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_wrp(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Wrpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wrpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wrp() != 0 { try!(write!(f, " wrp=0x{:x}", self.wrp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
