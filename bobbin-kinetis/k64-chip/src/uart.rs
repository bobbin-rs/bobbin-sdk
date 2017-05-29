pub const UART0: Uart = Uart(0x4006a000);
pub const UART1: Uart = Uart(0x4006b000);
pub const UART2: Uart = Uart(0x4006c000);
pub const UART3: Uart = Uart(0x4006d000);
pub const UART4: Uart = Uart(0x400ea000);
pub const UART5: Uart = Uart(0x400eb000);

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

  pub unsafe fn ma1(&self) -> Ma1 { 
     Ma1(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
  }
  pub unsafe fn set_ma1(&mut self, value: Ma1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
  }
  pub unsafe fn with_ma1<F: FnOnce(Ma1) -> Ma1>(&mut self, f: F) {
     let tmp = self.ma1();
     self.set_ma1(f(tmp))
  }

  pub unsafe fn ma2(&self) -> Ma2 { 
     Ma2(::core::ptr::read_volatile(((self.0 as usize) + 0x9) as *const u8))
  }
  pub unsafe fn set_ma2(&mut self, value: Ma2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x9) as *mut u8, value.0);
  }
  pub unsafe fn with_ma2<F: FnOnce(Ma2) -> Ma2>(&mut self, f: F) {
     let tmp = self.ma2();
     self.set_ma2(f(tmp))
  }

  pub unsafe fn c4(&self) -> C4 { 
     C4(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
  }
  pub unsafe fn set_c4(&mut self, value: C4) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
  }
  pub unsafe fn with_c4<F: FnOnce(C4) -> C4>(&mut self, f: F) {
     let tmp = self.c4();
     self.set_c4(f(tmp))
  }

  pub unsafe fn c5(&self) -> C5 { 
     C5(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
  }
  pub unsafe fn set_c5(&mut self, value: C5) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
  }
  pub unsafe fn with_c5<F: FnOnce(C5) -> C5>(&mut self, f: F) {
     let tmp = self.c5();
     self.set_c5(f(tmp))
  }

  pub unsafe fn ed(&self) -> Ed { 
     Ed(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
  }

  pub unsafe fn modem(&self) -> Modem { 
     Modem(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
  }
  pub unsafe fn set_modem(&mut self, value: Modem) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
  }
  pub unsafe fn with_modem<F: FnOnce(Modem) -> Modem>(&mut self, f: F) {
     let tmp = self.modem();
     self.set_modem(f(tmp))
  }

  pub unsafe fn ir(&self) -> Ir { 
     Ir(::core::ptr::read_volatile(((self.0 as usize) + 0xe) as *const u8))
  }
  pub unsafe fn set_ir(&mut self, value: Ir) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xe) as *mut u8, value.0);
  }
  pub unsafe fn with_ir<F: FnOnce(Ir) -> Ir>(&mut self, f: F) {
     let tmp = self.ir();
     self.set_ir(f(tmp))
  }

  pub unsafe fn pfifo(&self) -> Pfifo { 
     Pfifo(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u8))
  }
  pub unsafe fn set_pfifo(&mut self, value: Pfifo) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u8, value.0);
  }
  pub unsafe fn with_pfifo<F: FnOnce(Pfifo) -> Pfifo>(&mut self, f: F) {
     let tmp = self.pfifo();
     self.set_pfifo(f(tmp))
  }

  pub unsafe fn cfifo(&self) -> Cfifo { 
     Cfifo(::core::ptr::read_volatile(((self.0 as usize) + 0x11) as *const u8))
  }
  pub unsafe fn set_cfifo(&mut self, value: Cfifo) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x11) as *mut u8, value.0);
  }
  pub unsafe fn with_cfifo<F: FnOnce(Cfifo) -> Cfifo>(&mut self, f: F) {
     let tmp = self.cfifo();
     self.set_cfifo(f(tmp))
  }

  pub unsafe fn sfifo(&self) -> Sfifo { 
     Sfifo(::core::ptr::read_volatile(((self.0 as usize) + 0x12) as *const u8))
  }
  pub unsafe fn set_sfifo(&mut self, value: Sfifo) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x12) as *mut u8, value.0);
  }
  pub unsafe fn with_sfifo<F: FnOnce(Sfifo) -> Sfifo>(&mut self, f: F) {
     let tmp = self.sfifo();
     self.set_sfifo(f(tmp))
  }

  pub unsafe fn twfifo(&self) -> Twfifo { 
     Twfifo(::core::ptr::read_volatile(((self.0 as usize) + 0x13) as *const u8))
  }
  pub unsafe fn set_twfifo(&mut self, value: Twfifo) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x13) as *mut u8, value.0);
  }
  pub unsafe fn with_twfifo<F: FnOnce(Twfifo) -> Twfifo>(&mut self, f: F) {
     let tmp = self.twfifo();
     self.set_twfifo(f(tmp))
  }

  pub unsafe fn tcfifo(&self) -> Tcfifo { 
     Tcfifo(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u8))
  }

  pub unsafe fn rwfifo(&self) -> Rwfifo { 
     Rwfifo(::core::ptr::read_volatile(((self.0 as usize) + 0x15) as *const u8))
  }
  pub unsafe fn set_rwfifo(&mut self, value: Rwfifo) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x15) as *mut u8, value.0);
  }
  pub unsafe fn with_rwfifo<F: FnOnce(Rwfifo) -> Rwfifo>(&mut self, f: F) {
     let tmp = self.rwfifo();
     self.set_rwfifo(f(tmp))
  }

  pub unsafe fn rcfifo(&self) -> Rcfifo { 
     Rcfifo(::core::ptr::read_volatile(((self.0 as usize) + 0x16) as *const u8))
  }

  pub unsafe fn c7816(&self) -> C7816 { 
     C7816(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u8))
  }
  pub unsafe fn set_c7816(&mut self, value: C7816) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u8, value.0);
  }
  pub unsafe fn with_c7816<F: FnOnce(C7816) -> C7816>(&mut self, f: F) {
     let tmp = self.c7816();
     self.set_c7816(f(tmp))
  }

  pub unsafe fn ie7816(&self) -> Ie7816 { 
     Ie7816(::core::ptr::read_volatile(((self.0 as usize) + 0x19) as *const u8))
  }
  pub unsafe fn set_ie7816(&mut self, value: Ie7816) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x19) as *mut u8, value.0);
  }
  pub unsafe fn with_ie7816<F: FnOnce(Ie7816) -> Ie7816>(&mut self, f: F) {
     let tmp = self.ie7816();
     self.set_ie7816(f(tmp))
  }

  pub unsafe fn is7816(&self) -> Is7816 { 
     Is7816(::core::ptr::read_volatile(((self.0 as usize) + 0x1a) as *const u8))
  }
  pub unsafe fn set_is7816(&mut self, value: Is7816) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1a) as *mut u8, value.0);
  }
  pub unsafe fn with_is7816<F: FnOnce(Is7816) -> Is7816>(&mut self, f: F) {
     let tmp = self.is7816();
     self.set_is7816(f(tmp))
  }

  pub unsafe fn wp7816t0(&self) -> Wp7816t0 { 
     Wp7816t0(::core::ptr::read_volatile(((self.0 as usize) + 0x1b) as *const u8))
  }
  pub unsafe fn set_wp7816t0(&mut self, value: Wp7816t0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1b) as *mut u8, value.0);
  }
  pub unsafe fn with_wp7816t0<F: FnOnce(Wp7816t0) -> Wp7816t0>(&mut self, f: F) {
     let tmp = self.wp7816t0();
     self.set_wp7816t0(f(tmp))
  }

  pub unsafe fn wp7816t1(&self) -> Wp7816t1 { 
     Wp7816t1(::core::ptr::read_volatile(((self.0 as usize) + 0x1b) as *const u8))
  }
  pub unsafe fn set_wp7816t1(&mut self, value: Wp7816t1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1b) as *mut u8, value.0);
  }
  pub unsafe fn with_wp7816t1<F: FnOnce(Wp7816t1) -> Wp7816t1>(&mut self, f: F) {
     let tmp = self.wp7816t1();
     self.set_wp7816t1(f(tmp))
  }

  pub unsafe fn wn7816(&self) -> Wn7816 { 
     Wn7816(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u8))
  }
  pub unsafe fn set_wn7816(&mut self, value: Wn7816) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u8, value.0);
  }
  pub unsafe fn with_wn7816<F: FnOnce(Wn7816) -> Wn7816>(&mut self, f: F) {
     let tmp = self.wn7816();
     self.set_wn7816(f(tmp))
  }

  pub unsafe fn wf7816(&self) -> Wf7816 { 
     Wf7816(::core::ptr::read_volatile(((self.0 as usize) + 0x1d) as *const u8))
  }
  pub unsafe fn set_wf7816(&mut self, value: Wf7816) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1d) as *mut u8, value.0);
  }
  pub unsafe fn with_wf7816<F: FnOnce(Wf7816) -> Wf7816>(&mut self, f: F) {
     let tmp = self.wf7816();
     self.set_wf7816(f(tmp))
  }

  pub unsafe fn et7816(&self) -> Et7816 { 
     Et7816(::core::ptr::read_volatile(((self.0 as usize) + 0x1e) as *const u8))
  }
  pub unsafe fn set_et7816(&mut self, value: Et7816) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1e) as *mut u8, value.0);
  }
  pub unsafe fn with_et7816<F: FnOnce(Et7816) -> Et7816>(&mut self, f: F) {
     let tmp = self.et7816();
     self.set_et7816(f(tmp))
  }

  pub unsafe fn tl7816(&self) -> Tl7816 { 
     Tl7816(::core::ptr::read_volatile(((self.0 as usize) + 0x1f) as *const u8))
  }
  pub unsafe fn set_tl7816(&mut self, value: Tl7816) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1f) as *mut u8, value.0);
  }
  pub unsafe fn with_tl7816<F: FnOnce(Tl7816) -> Tl7816>(&mut self, f: F) {
     let tmp = self.tl7816();
     self.set_tl7816(f(tmp))
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

  pub fn msbf(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_msbf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
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
      if self.msbf() != 0 { try!(write!(f, " msbf"))}
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
pub struct Ma1(pub u8);

impl Ma1 {
  pub fn ma(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_ma(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ma1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ma1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ma() != 0 { try!(write!(f, " ma=0x{:x}", self.ma()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ma2(pub u8);

impl Ma2 {
  pub fn ma(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_ma(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Ma2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ma2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ma() != 0 { try!(write!(f, " ma=0x{:x}", self.ma()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct C4(pub u8);

impl C4 {
  pub fn brfa(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1f // [4:0]
  }
  pub fn set_brfa(mut self, value: u8) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

  pub fn m10(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_m10(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn maen2(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_maen2(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn maen1(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_maen1(mut self, value: u8) -> Self {
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
      if self.brfa() != 0 { try!(write!(f, " brfa=0x{:x}", self.brfa()))}
      if self.m10() != 0 { try!(write!(f, " m10"))}
      if self.maen2() != 0 { try!(write!(f, " maen2"))}
      if self.maen1() != 0 { try!(write!(f, " maen1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct C5(pub u8);

impl C5 {
  pub fn lbkddmas(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_lbkddmas(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn ildmas(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_ildmas(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn rdmas(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_rdmas(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tcdmas(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_tcdmas(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
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

impl ::core::fmt::Display for C5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for C5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lbkddmas() != 0 { try!(write!(f, " lbkddmas"))}
      if self.ildmas() != 0 { try!(write!(f, " ildmas"))}
      if self.rdmas() != 0 { try!(write!(f, " rdmas"))}
      if self.tcdmas() != 0 { try!(write!(f, " tcdmas"))}
      if self.tdmas() != 0 { try!(write!(f, " tdmas"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ed(pub u8);

impl Ed {
  pub fn paritye(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_paritye(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn noisy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_noisy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Ed {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ed {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.paritye() != 0 { try!(write!(f, " paritye"))}
      if self.noisy() != 0 { try!(write!(f, " noisy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Modem(pub u8);

impl Modem {
  pub fn txctse(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_txctse(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn txrtse(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_txrtse(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn txrtspol(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_txrtspol(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn rxrtse(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_rxrtse(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}

impl ::core::fmt::Display for Modem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Modem {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txctse() != 0 { try!(write!(f, " txctse"))}
      if self.txrtse() != 0 { try!(write!(f, " txrtse"))}
      if self.txrtspol() != 0 { try!(write!(f, " txrtspol"))}
      if self.rxrtse() != 0 { try!(write!(f, " rxrtse"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ir(pub u8);

impl Ir {
  pub fn tnp(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3 // [1:0]
  }
  pub fn set_tnp(mut self, value: u8) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn iren(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_iren(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}

impl ::core::fmt::Display for Ir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tnp() != 0 { try!(write!(f, " tnp=0x{:x}", self.tnp()))}
      if self.iren() != 0 { try!(write!(f, " iren"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pfifo(pub u8);

impl Pfifo {
  pub fn rxfifosize(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }
  pub fn set_rxfifosize(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn rxfe(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_rxfe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn txfifosize(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x7 // [6:4]
  }
  pub fn set_txfifosize(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn txfe(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_txfe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Pfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxfifosize() != 0 { try!(write!(f, " rxfifosize=0x{:x}", self.rxfifosize()))}
      if self.rxfe() != 0 { try!(write!(f, " rxfe"))}
      if self.txfifosize() != 0 { try!(write!(f, " txfifosize=0x{:x}", self.txfifosize()))}
      if self.txfe() != 0 { try!(write!(f, " txfe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cfifo(pub u8);

impl Cfifo {
  pub fn rxufe(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_rxufe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn txofe(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_txofe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn rxofe(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_rxofe(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn rxflush(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_rxflush(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn txflush(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_txflush(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Cfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxufe() != 0 { try!(write!(f, " rxufe"))}
      if self.txofe() != 0 { try!(write!(f, " txofe"))}
      if self.rxofe() != 0 { try!(write!(f, " rxofe"))}
      if self.rxflush() != 0 { try!(write!(f, " rxflush"))}
      if self.txflush() != 0 { try!(write!(f, " txflush"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sfifo(pub u8);

impl Sfifo {
  pub fn rxuf(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_rxuf(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn txof(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_txof(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn rxof(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_rxof(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn rxempt(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_rxempt(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn txempt(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_txempt(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Sfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
      if self.txof() != 0 { try!(write!(f, " txof"))}
      if self.rxof() != 0 { try!(write!(f, " rxof"))}
      if self.rxempt() != 0 { try!(write!(f, " rxempt"))}
      if self.txempt() != 0 { try!(write!(f, " txempt"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Twfifo(pub u8);

impl Twfifo {
  pub fn txwater(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_txwater(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Twfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Twfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txwater() != 0 { try!(write!(f, " txwater=0x{:x}", self.txwater()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tcfifo(pub u8);

impl Tcfifo {
  pub fn txcount(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_txcount(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tcfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tcfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txcount() != 0 { try!(write!(f, " txcount=0x{:x}", self.txcount()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rwfifo(pub u8);

impl Rwfifo {
  pub fn rxwater(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_rxwater(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rwfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rwfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxwater() != 0 { try!(write!(f, " rxwater=0x{:x}", self.rxwater()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rcfifo(pub u8);

impl Rcfifo {
  pub fn rxcount(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_rxcount(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rcfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rcfifo {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxcount() != 0 { try!(write!(f, " rxcount=0x{:x}", self.rxcount()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct C7816(pub u8);

impl C7816 {
  pub fn iso_7816e(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_iso_7816e(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn ttype(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_ttype(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn init(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_init(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn anack(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
  pub fn set_anack(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn onack(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_onack(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for C7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for C7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.iso_7816e() != 0 { try!(write!(f, " iso_7816e"))}
      if self.ttype() != 0 { try!(write!(f, " ttype"))}
      if self.init() != 0 { try!(write!(f, " init"))}
      if self.anack() != 0 { try!(write!(f, " anack"))}
      if self.onack() != 0 { try!(write!(f, " onack"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ie7816(pub u8);

impl Ie7816 {
  pub fn rxte(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_rxte(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn txte(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_txte(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn gtve(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_gtve(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn initde(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_initde(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn bwte(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_bwte(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn cwte(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_cwte(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn wte(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_wte(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Ie7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ie7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxte() != 0 { try!(write!(f, " rxte"))}
      if self.txte() != 0 { try!(write!(f, " txte"))}
      if self.gtve() != 0 { try!(write!(f, " gtve"))}
      if self.initde() != 0 { try!(write!(f, " initde"))}
      if self.bwte() != 0 { try!(write!(f, " bwte"))}
      if self.cwte() != 0 { try!(write!(f, " cwte"))}
      if self.wte() != 0 { try!(write!(f, " wte"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Is7816(pub u8);

impl Is7816 {
  pub fn rxt(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  pub fn set_rxt(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn txt(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  pub fn set_txt(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn gtv(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  pub fn set_gtv(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn initd(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
  pub fn set_initd(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn bwt(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
  pub fn set_bwt(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn cwt(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  pub fn set_cwt(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn wt(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  pub fn set_wt(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}

impl ::core::fmt::Display for Is7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Is7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxt() != 0 { try!(write!(f, " rxt"))}
      if self.txt() != 0 { try!(write!(f, " txt"))}
      if self.gtv() != 0 { try!(write!(f, " gtv"))}
      if self.initd() != 0 { try!(write!(f, " initd"))}
      if self.bwt() != 0 { try!(write!(f, " bwt"))}
      if self.cwt() != 0 { try!(write!(f, " cwt"))}
      if self.wt() != 0 { try!(write!(f, " wt"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wp7816t0(pub u8);

impl Wp7816t0 {
  pub fn wi(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_wi(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Wp7816t0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wp7816t0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wi() != 0 { try!(write!(f, " wi=0x{:x}", self.wi()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wp7816t1(pub u8);

impl Wp7816t1 {
  pub fn bwi(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  pub fn set_bwi(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn cwi(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0xf // [7:4]
  }
  pub fn set_cwi(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Wp7816t1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wp7816t1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bwi() != 0 { try!(write!(f, " bwi=0x{:x}", self.bwi()))}
      if self.cwi() != 0 { try!(write!(f, " cwi=0x{:x}", self.cwi()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wn7816(pub u8);

impl Wn7816 {
  pub fn gtn(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_gtn(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Wn7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wn7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gtn() != 0 { try!(write!(f, " gtn=0x{:x}", self.gtn()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wf7816(pub u8);

impl Wf7816 {
  pub fn gtfd(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_gtfd(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Wf7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wf7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gtfd() != 0 { try!(write!(f, " gtfd=0x{:x}", self.gtfd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Et7816(pub u8);

impl Et7816 {
  pub fn rxthreshold(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  pub fn set_rxthreshold(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn txthreshold(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0xf // [7:4]
  }
  pub fn set_txthreshold(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Et7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Et7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxthreshold() != 0 { try!(write!(f, " rxthreshold=0x{:x}", self.rxthreshold()))}
      if self.txthreshold() != 0 { try!(write!(f, " txthreshold=0x{:x}", self.txthreshold()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tl7816(pub u8);

impl Tl7816 {
  pub fn tlen(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  pub fn set_tlen(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Tl7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tl7816 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tlen() != 0 { try!(write!(f, " tlen=0x{:x}", self.tlen()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

