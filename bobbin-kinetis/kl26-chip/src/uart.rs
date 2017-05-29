pub const UART1: Uart = Uart(0x4006b000);
pub const UART2: Uart = Uart(0x4006c000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Uart(pub u32);

impl Uart {
  pub unsafe fn bdh(&self) -> Bdh { 
     Bdh(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
  }
  pub unsafe fn set_bdh(&mut self, value: Bdh) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
  }
  pub unsafe fn with_bdh<F: FnOnce(Bdh) -> Bdh>(&mut self, f: F) {
     let tmp = self.bdh();
     self.set_bdh(f(tmp))
  }

  pub unsafe fn bdl(&self) -> Bdl { 
     Bdl(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
  }
  pub unsafe fn set_bdl(&mut self, value: Bdl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
  }
  pub unsafe fn with_bdl<F: FnOnce(Bdl) -> Bdl>(&mut self, f: F) {
     let tmp = self.bdl();
     self.set_bdl(f(tmp))
  }

  pub unsafe fn c1(&self) -> C1 { 
     C1(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
  }
  pub unsafe fn set_c1(&mut self, value: C1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
  }
  pub unsafe fn with_c1<F: FnOnce(C1) -> C1>(&mut self, f: F) {
     let tmp = self.c1();
     self.set_c1(f(tmp))
  }

  pub unsafe fn c2(&self) -> C2 { 
     C2(::core::ptr::read_volatile(((self.0 as usize) + 0x3) as *const u8))
  }
  pub unsafe fn set_c2(&mut self, value: C2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x3) as *mut u8, value.0);
  }
  pub unsafe fn with_c2<F: FnOnce(C2) -> C2>(&mut self, f: F) {
     let tmp = self.c2();
     self.set_c2(f(tmp))
  }

  pub unsafe fn s1(&self) -> S1 { 
     S1(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
  }

  pub unsafe fn s2(&self) -> S2 { 
     S2(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
  }
  pub unsafe fn set_s2(&mut self, value: S2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
  }
  pub unsafe fn with_s2<F: FnOnce(S2) -> S2>(&mut self, f: F) {
     let tmp = self.s2();
     self.set_s2(f(tmp))
  }

  pub unsafe fn c3(&self) -> C3 { 
     C3(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
  }
  pub unsafe fn set_c3(&mut self, value: C3) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
  }
  pub unsafe fn with_c3<F: FnOnce(C3) -> C3>(&mut self, f: F) {
     let tmp = self.c3();
     self.set_c3(f(tmp))
  }

  pub unsafe fn d(&self) -> D { 
     D(::core::ptr::read_volatile(((self.0 as usize) + 0x7) as *const u8))
  }
  pub unsafe fn set_d(&mut self, value: D) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x7) as *mut u8, value.0);
  }
  pub unsafe fn with_d<F: FnOnce(D) -> D>(&mut self, f: F) {
     let tmp = self.d();
     self.set_d(f(tmp))
  }

  pub unsafe fn c4(&self) -> C4 { 
     C4(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
  }
  pub unsafe fn set_c4(&mut self, value: C4) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
  }
  pub unsafe fn with_c4<F: FnOnce(C4) -> C4>(&mut self, f: F) {
     let tmp = self.c4();
     self.set_c4(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Bdh(pub u8);

impl Bdh {
  pub fn sbr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1f // [4:0]
  }
  pub fn set_sbr(mut self, value: u8) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn sbns(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_sbns(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn rxedgie(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_rxedgie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn lbkdie(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_lbkdie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Bdh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Bdh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sbr() != 0 { try!(write!(f, " sbr=0x{:x}", self.sbr()))}
      if self.sbns() != 0 { try!(write!(f, " sbns"))}
      if self.rxedgie() != 0 { try!(write!(f, " rxedgie"))}
      if self.lbkdie() != 0 { try!(write!(f, " lbkdie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Bdl(pub u8);

impl Bdl {
  pub fn sbr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_sbr(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Bdl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Bdl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sbr() != 0 { try!(write!(f, " sbr=0x{:x}", self.sbr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct C1(pub u8);

impl C1 {
  pub fn pt(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_pt(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn pe(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_pe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn ilt(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_ilt(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn wake(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_wake(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn m(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_m(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn rsrc(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_rsrc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn uartswai(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_uartswai(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn loops(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_loops(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for C1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for C1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pt() != 0 { try!(write!(f, " pt"))}
      if self.pe() != 0 { try!(write!(f, " pe"))}
      if self.ilt() != 0 { try!(write!(f, " ilt"))}
      if self.wake() != 0 { try!(write!(f, " wake"))}
      if self.m() != 0 { try!(write!(f, " m"))}
      if self.rsrc() != 0 { try!(write!(f, " rsrc"))}
      if self.uartswai() != 0 { try!(write!(f, " uartswai"))}
      if self.loops() != 0 { try!(write!(f, " loops"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct C2(pub u8);

impl C2 {
  pub fn sbk(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_sbk(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn rwu(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_rwu(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn re(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_re(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn te(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_te(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn ilie(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_ilie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn rie(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_rie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tcie(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_tcie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn tie(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_tie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for C2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for C2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sbk() != 0 { try!(write!(f, " sbk"))}
      if self.rwu() != 0 { try!(write!(f, " rwu"))}
      if self.re() != 0 { try!(write!(f, " re"))}
      if self.te() != 0 { try!(write!(f, " te"))}
      if self.ilie() != 0 { try!(write!(f, " ilie"))}
      if self.rie() != 0 { try!(write!(f, " rie"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct S1(pub u8);

impl S1 {
  pub fn pf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_pf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn fe(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_fe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn nf(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_nf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn or(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_or(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn idle(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_idle(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn rdrf(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_rdrf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tc(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_tc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn tdre(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_tdre(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for S1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for S1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pf() != 0 { try!(write!(f, " pf"))}
      if self.fe() != 0 { try!(write!(f, " fe"))}
      if self.nf() != 0 { try!(write!(f, " nf"))}
      if self.or() != 0 { try!(write!(f, " or"))}
      if self.idle() != 0 { try!(write!(f, " idle"))}
      if self.rdrf() != 0 { try!(write!(f, " rdrf"))}
      if self.tc() != 0 { try!(write!(f, " tc"))}
      if self.tdre() != 0 { try!(write!(f, " tdre"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct S2(pub u8);

impl S2 {
  pub fn raf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_raf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn lbkde(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_lbkde(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn brk13(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_brk13(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn rwuid(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_rwuid(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn rxinv(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_rxinv(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn rxedgif(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_rxedgif(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn lbkdif(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_lbkdif(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for S2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for S2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.raf() != 0 { try!(write!(f, " raf"))}
      if self.lbkde() != 0 { try!(write!(f, " lbkde"))}
      if self.brk13() != 0 { try!(write!(f, " brk13"))}
      if self.rwuid() != 0 { try!(write!(f, " rwuid"))}
      if self.rxinv() != 0 { try!(write!(f, " rxinv"))}
      if self.rxedgif() != 0 { try!(write!(f, " rxedgif"))}
      if self.lbkdif() != 0 { try!(write!(f, " lbkdif"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct C3(pub u8);

impl C3 {
  pub fn peie(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_peie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn feie(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_feie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn neie(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_neie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn orie(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_orie(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn txinv(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_txinv(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn txdir(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_txdir(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn t8(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_t8(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn r8(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_r8(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for C3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for C3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.peie() != 0 { try!(write!(f, " peie"))}
      if self.feie() != 0 { try!(write!(f, " feie"))}
      if self.neie() != 0 { try!(write!(f, " neie"))}
      if self.orie() != 0 { try!(write!(f, " orie"))}
      if self.txinv() != 0 { try!(write!(f, " txinv"))}
      if self.txdir() != 0 { try!(write!(f, " txdir"))}
      if self.t8() != 0 { try!(write!(f, " t8"))}
      if self.r8() != 0 { try!(write!(f, " r8"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct D(pub u8);

impl D {
  pub fn rt(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_rt(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for D {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for D {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rt() != 0 { try!(write!(f, " rt=0x{:x}", self.rt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct C4(pub u8);

impl C4 {
  pub fn rdmas(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_rdmas(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tdmas(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_tdmas(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for C4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for C4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rdmas() != 0 { try!(write!(f, " rdmas"))}
      if self.tdmas() != 0 { try!(write!(f, " tdmas"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

