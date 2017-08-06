#[allow(unused_imports)] use bobbin_common::bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="UART0 Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the BDH register."]
  #[inline] pub fn bdh_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x0) as *const u8
  }
#[doc="Get the *mut pointer for the BDH register."]
  #[inline] pub fn bdh_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x0) as *mut u8
  }
#[doc="Read the BDH register."]
  #[inline] pub fn bdh(&self) -> Bdh { 
     unsafe {
        Bdh(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
     }
  }
#[doc="Write the BDH register."]
  #[inline] pub fn set_bdh(&self, value: Bdh) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the BDH register."]
  #[inline] pub fn with_bdh<F: FnOnce(Bdh) -> Bdh>(&self, f: F) -> &Self {
     let tmp = self.bdh();
     self.set_bdh(f(tmp))
  }

#[doc="Get the *const pointer for the BDL register."]
  #[inline] pub fn bdl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1) as *const u8
  }
#[doc="Get the *mut pointer for the BDL register."]
  #[inline] pub fn bdl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1) as *mut u8
  }
#[doc="Read the BDL register."]
  #[inline] pub fn bdl(&self) -> Bdl { 
     unsafe {
        Bdl(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
     }
  }
#[doc="Write the BDL register."]
  #[inline] pub fn set_bdl(&self, value: Bdl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the BDL register."]
  #[inline] pub fn with_bdl<F: FnOnce(Bdl) -> Bdl>(&self, f: F) -> &Self {
     let tmp = self.bdl();
     self.set_bdl(f(tmp))
  }

#[doc="Get the *const pointer for the C1 register."]
  #[inline] pub fn c1_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x2) as *const u8
  }
#[doc="Get the *mut pointer for the C1 register."]
  #[inline] pub fn c1_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x2) as *mut u8
  }
#[doc="Read the C1 register."]
  #[inline] pub fn c1(&self) -> C1 { 
     unsafe {
        C1(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
     }
  }
#[doc="Write the C1 register."]
  #[inline] pub fn set_c1(&self, value: C1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C1 register."]
  #[inline] pub fn with_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
     let tmp = self.c1();
     self.set_c1(f(tmp))
  }

#[doc="Get the *const pointer for the C2 register."]
  #[inline] pub fn c2_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x3) as *const u8
  }
#[doc="Get the *mut pointer for the C2 register."]
  #[inline] pub fn c2_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x3) as *mut u8
  }
#[doc="Read the C2 register."]
  #[inline] pub fn c2(&self) -> C2 { 
     unsafe {
        C2(::core::ptr::read_volatile(((self.0 as usize) + 0x3) as *const u8))
     }
  }
#[doc="Write the C2 register."]
  #[inline] pub fn set_c2(&self, value: C2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C2 register."]
  #[inline] pub fn with_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
     let tmp = self.c2();
     self.set_c2(f(tmp))
  }

#[doc="Get the *const pointer for the S1 register."]
  #[inline] pub fn s1_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
#[doc="Get the *mut pointer for the S1 register."]
  #[inline] pub fn s1_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
#[doc="Read the S1 register."]
  #[inline] pub fn s1(&self) -> S1 { 
     unsafe {
        S1(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
#[doc="Write the S1 register."]
  #[inline] pub fn set_s1(&self, value: S1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the S1 register."]
  #[inline] pub fn with_s1<F: FnOnce(S1) -> S1>(&self, f: F) -> &Self {
     let tmp = self.s1();
     self.set_s1(f(tmp))
  }

#[doc="Get the *const pointer for the S2 register."]
  #[inline] pub fn s2_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
#[doc="Get the *mut pointer for the S2 register."]
  #[inline] pub fn s2_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
#[doc="Read the S2 register."]
  #[inline] pub fn s2(&self) -> S2 { 
     unsafe {
        S2(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
#[doc="Write the S2 register."]
  #[inline] pub fn set_s2(&self, value: S2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the S2 register."]
  #[inline] pub fn with_s2<F: FnOnce(S2) -> S2>(&self, f: F) -> &Self {
     let tmp = self.s2();
     self.set_s2(f(tmp))
  }

#[doc="Get the *const pointer for the C3 register."]
  #[inline] pub fn c3_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
#[doc="Get the *mut pointer for the C3 register."]
  #[inline] pub fn c3_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
#[doc="Read the C3 register."]
  #[inline] pub fn c3(&self) -> C3 { 
     unsafe {
        C3(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
#[doc="Write the C3 register."]
  #[inline] pub fn set_c3(&self, value: C3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C3 register."]
  #[inline] pub fn with_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
     let tmp = self.c3();
     self.set_c3(f(tmp))
  }

#[doc="Get the *const pointer for the D register."]
  #[inline] pub fn d_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x7) as *const u8
  }
#[doc="Get the *mut pointer for the D register."]
  #[inline] pub fn d_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x7) as *mut u8
  }
#[doc="Read the D register."]
  #[inline] pub fn d(&self) -> D { 
     unsafe {
        D(::core::ptr::read_volatile(((self.0 as usize) + 0x7) as *const u8))
     }
  }
#[doc="Write the D register."]
  #[inline] pub fn set_d(&self, value: D) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x7) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the D register."]
  #[inline] pub fn with_d<F: FnOnce(D) -> D>(&self, f: F) -> &Self {
     let tmp = self.d();
     self.set_d(f(tmp))
  }

#[doc="Get the *const pointer for the MA1 register."]
  #[inline] pub fn ma1_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
#[doc="Get the *mut pointer for the MA1 register."]
  #[inline] pub fn ma1_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
#[doc="Read the MA1 register."]
  #[inline] pub fn ma1(&self) -> Ma1 { 
     unsafe {
        Ma1(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
#[doc="Write the MA1 register."]
  #[inline] pub fn set_ma1(&self, value: Ma1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the MA1 register."]
  #[inline] pub fn with_ma1<F: FnOnce(Ma1) -> Ma1>(&self, f: F) -> &Self {
     let tmp = self.ma1();
     self.set_ma1(f(tmp))
  }

#[doc="Get the *const pointer for the MA2 register."]
  #[inline] pub fn ma2_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x9) as *const u8
  }
#[doc="Get the *mut pointer for the MA2 register."]
  #[inline] pub fn ma2_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x9) as *mut u8
  }
#[doc="Read the MA2 register."]
  #[inline] pub fn ma2(&self) -> Ma2 { 
     unsafe {
        Ma2(::core::ptr::read_volatile(((self.0 as usize) + 0x9) as *const u8))
     }
  }
#[doc="Write the MA2 register."]
  #[inline] pub fn set_ma2(&self, value: Ma2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x9) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the MA2 register."]
  #[inline] pub fn with_ma2<F: FnOnce(Ma2) -> Ma2>(&self, f: F) -> &Self {
     let tmp = self.ma2();
     self.set_ma2(f(tmp))
  }

#[doc="Get the *const pointer for the C4 register."]
  #[inline] pub fn c4_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xa) as *const u8
  }
#[doc="Get the *mut pointer for the C4 register."]
  #[inline] pub fn c4_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xa) as *mut u8
  }
#[doc="Read the C4 register."]
  #[inline] pub fn c4(&self) -> C4 { 
     unsafe {
        C4(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u8))
     }
  }
#[doc="Write the C4 register."]
  #[inline] pub fn set_c4(&self, value: C4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C4 register."]
  #[inline] pub fn with_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
     let tmp = self.c4();
     self.set_c4(f(tmp))
  }

#[doc="Get the *const pointer for the C5 register."]
  #[inline] pub fn c5_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xb) as *const u8
  }
#[doc="Get the *mut pointer for the C5 register."]
  #[inline] pub fn c5_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xb) as *mut u8
  }
#[doc="Read the C5 register."]
  #[inline] pub fn c5(&self) -> C5 { 
     unsafe {
        C5(::core::ptr::read_volatile(((self.0 as usize) + 0xb) as *const u8))
     }
  }
#[doc="Write the C5 register."]
  #[inline] pub fn set_c5(&self, value: C5) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the C5 register."]
  #[inline] pub fn with_c5<F: FnOnce(C5) -> C5>(&self, f: F) -> &Self {
     let tmp = self.c5();
     self.set_c5(f(tmp))
  }

}

#[doc="UART Baud Rate Register High"]
#[derive(PartialEq, Eq)]
pub struct Bdh(pub u8);
impl Bdh {
#[doc="Baud Rate Modulo Divisor."]
  #[inline] pub fn sbr(&self) -> bits::U5 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
  }
#[doc="Baud Rate Modulo Divisor."]
  #[inline] pub fn set_sbr<V: Into<bits::U5>>(mut self, value: V) -> Self {
     let value: bits::U5 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Stop Bit Number Select"]
  #[inline] pub fn sbns(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Stop Bit Number Select"]
  #[inline] pub fn set_sbns<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="RX Input Active Edge Interrupt Enable (for RXEDGIF)"]
  #[inline] pub fn rxedgie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="RX Input Active Edge Interrupt Enable (for RXEDGIF)"]
  #[inline] pub fn set_rxedgie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="LIN Break Detect Interrupt Enable (for LBKDIF)"]
  #[inline] pub fn lbkdie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="LIN Break Detect Interrupt Enable (for LBKDIF)"]
  #[inline] pub fn set_lbkdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="UART Baud Rate Register Low"]
#[derive(PartialEq, Eq)]
pub struct Bdl(pub u8);
impl Bdl {
#[doc="Baud Rate Modulo Divisor"]
  #[inline] pub fn sbr(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Baud Rate Modulo Divisor"]
  #[inline] pub fn set_sbr<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u8 = value.into();
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
#[doc="UART Control Register 1"]
#[derive(PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
#[doc="Parity Type"]
  #[inline] pub fn pt(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Parity Type"]
  #[inline] pub fn set_pt<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Parity Enable"]
  #[inline] pub fn pe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Parity Enable"]
  #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Idle Line Type Select"]
  #[inline] pub fn ilt(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Idle Line Type Select"]
  #[inline] pub fn set_ilt<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Receiver Wakeup Method Select"]
  #[inline] pub fn wake(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Receiver Wakeup Method Select"]
  #[inline] pub fn set_wake<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="9-Bit or 8-Bit Mode Select"]
  #[inline] pub fn m(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="9-Bit or 8-Bit Mode Select"]
  #[inline] pub fn set_m<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Receiver Source Select"]
  #[inline] pub fn rsrc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Receiver Source Select"]
  #[inline] pub fn set_rsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Doze Enable"]
  #[inline] pub fn dozeen(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Doze Enable"]
  #[inline] pub fn set_dozeen<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Loop Mode Select"]
  #[inline] pub fn loops(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Loop Mode Select"]
  #[inline] pub fn set_loops<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
      if self.dozeen() != 0 { try!(write!(f, " dozeen"))}
      if self.loops() != 0 { try!(write!(f, " loops"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Control Register 2"]
#[derive(PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
#[doc="Send Break"]
  #[inline] pub fn sbk(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Send Break"]
  #[inline] pub fn set_sbk<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Receiver Wakeup Control"]
  #[inline] pub fn rwu(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Receiver Wakeup Control"]
  #[inline] pub fn set_rwu<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Receiver Enable"]
  #[inline] pub fn re(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Receiver Enable"]
  #[inline] pub fn set_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Transmitter Enable"]
  #[inline] pub fn te(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Transmitter Enable"]
  #[inline] pub fn set_te<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Idle Line Interrupt Enable for IDLE"]
  #[inline] pub fn ilie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Idle Line Interrupt Enable for IDLE"]
  #[inline] pub fn set_ilie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Receiver Interrupt Enable for RDRF"]
  #[inline] pub fn rie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Receiver Interrupt Enable for RDRF"]
  #[inline] pub fn set_rie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Transmission Complete Interrupt Enable for TC"]
  #[inline] pub fn tcie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Transmission Complete Interrupt Enable for TC"]
  #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Transmit Interrupt Enable for TDRE"]
  #[inline] pub fn tie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Transmit Interrupt Enable for TDRE"]
  #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="UART Status Register 1"]
#[derive(PartialEq, Eq)]
pub struct S1(pub u8);
impl S1 {
#[doc="Parity Error Flag"]
  #[inline] pub fn pf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Parity Error Flag"]
  #[inline] pub fn set_pf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Framing Error Flag"]
  #[inline] pub fn fe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Framing Error Flag"]
  #[inline] pub fn set_fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Noise Flag"]
  #[inline] pub fn nf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Noise Flag"]
  #[inline] pub fn set_nf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Receiver Overrun Flag"]
  #[inline] pub fn or(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Receiver Overrun Flag"]
  #[inline] pub fn set_or<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Idle Line Flag"]
  #[inline] pub fn idle(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Idle Line Flag"]
  #[inline] pub fn set_idle<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Receive Data Register Full Flag"]
  #[inline] pub fn rdrf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Receive Data Register Full Flag"]
  #[inline] pub fn set_rdrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Transmission Complete Flag"]
  #[inline] pub fn tc(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Transmission Complete Flag"]
  #[inline] pub fn set_tc<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Transmit Data Register Empty Flag"]
  #[inline] pub fn tdre(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Transmit Data Register Empty Flag"]
  #[inline] pub fn set_tdre<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="UART Status Register 2"]
#[derive(PartialEq, Eq)]
pub struct S2(pub u8);
impl S2 {
#[doc="Receiver Active Flag"]
  #[inline] pub fn raf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Receiver Active Flag"]
  #[inline] pub fn set_raf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="LIN Break Detection Enable"]
  #[inline] pub fn lbkde(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="LIN Break Detection Enable"]
  #[inline] pub fn set_lbkde<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Break Character Generation Length"]
  #[inline] pub fn brk13(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Break Character Generation Length"]
  #[inline] pub fn set_brk13<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Receive Wake Up Idle Detect"]
  #[inline] pub fn rwuid(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Receive Wake Up Idle Detect"]
  #[inline] pub fn set_rwuid<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Receive Data Inversion"]
  #[inline] pub fn rxinv(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Receive Data Inversion"]
  #[inline] pub fn set_rxinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="MSB First"]
  #[inline] pub fn msbf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="MSB First"]
  #[inline] pub fn set_msbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="UART_RX Pin Active Edge Interrupt Flag"]
  #[inline] pub fn rxedgif(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="UART_RX Pin Active Edge Interrupt Flag"]
  #[inline] pub fn set_rxedgif<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="LIN Break Detect Interrupt Flag"]
  #[inline] pub fn lbkdif(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="LIN Break Detect Interrupt Flag"]
  #[inline] pub fn set_lbkdif<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
#[doc="UART Control Register 3"]
#[derive(PartialEq, Eq)]
pub struct C3(pub u8);
impl C3 {
#[doc="Parity Error Interrupt Enable"]
  #[inline] pub fn peie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Parity Error Interrupt Enable"]
  #[inline] pub fn set_peie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Framing Error Interrupt Enable"]
  #[inline] pub fn feie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Framing Error Interrupt Enable"]
  #[inline] pub fn set_feie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Noise Error Interrupt Enable"]
  #[inline] pub fn neie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Noise Error Interrupt Enable"]
  #[inline] pub fn set_neie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Overrun Interrupt Enable"]
  #[inline] pub fn orie(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Overrun Interrupt Enable"]
  #[inline] pub fn set_orie<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Transmit Data Inversion"]
  #[inline] pub fn txinv(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="Transmit Data Inversion"]
  #[inline] pub fn set_txinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="UART_TX Pin Direction in Single-Wire Mode"]
  #[inline] pub fn txdir(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="UART_TX Pin Direction in Single-Wire Mode"]
  #[inline] pub fn set_txdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Receive Bit 9 / Transmit Bit 8"]
  #[inline] pub fn r9t8(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Receive Bit 9 / Transmit Bit 8"]
  #[inline] pub fn set_r9t8<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Receive Bit 8 / Transmit Bit 9"]
  #[inline] pub fn r8t9(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Receive Bit 8 / Transmit Bit 9"]
  #[inline] pub fn set_r8t9<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
      if self.r9t8() != 0 { try!(write!(f, " r9t8"))}
      if self.r8t9() != 0 { try!(write!(f, " r8t9"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Data Register"]
#[derive(PartialEq, Eq)]
pub struct D(pub u8);
impl D {
#[doc="Read receive data buffer or write transmit data buffer."]
  #[inline] pub fn rt(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Read receive data buffer or write transmit data buffer."]
  #[inline] pub fn set_rt<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u8 = value.into();
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
#[doc="UART Match Address Registers 1"]
#[derive(PartialEq, Eq)]
pub struct Ma1(pub u8);
impl Ma1 {
#[doc="Match Address"]
  #[inline] pub fn ma(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Match Address"]
  #[inline] pub fn set_ma<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u8 = value.into();
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
#[doc="UART Match Address Registers 2"]
#[derive(PartialEq, Eq)]
pub struct Ma2(pub u8);
impl Ma2 {
#[doc="Match Address"]
  #[inline] pub fn ma(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Match Address"]
  #[inline] pub fn set_ma<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u8 = value.into();
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
#[doc="UART Control Register 4"]
#[derive(PartialEq, Eq)]
pub struct C4(pub u8);
impl C4 {
#[doc="Over Sampling Ratio"]
  #[inline] pub fn osr(&self) -> bits::U5 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
  }
#[doc="Over Sampling Ratio"]
  #[inline] pub fn set_osr<V: Into<bits::U5>>(mut self, value: V) -> Self {
     let value: bits::U5 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="10-bit Mode select"]
  #[inline] pub fn m10(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="10-bit Mode select"]
  #[inline] pub fn set_m10<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Match Address Mode Enable 2"]
  #[inline] pub fn maen2(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Match Address Mode Enable 2"]
  #[inline] pub fn set_maen2<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Match Address Mode Enable 1"]
  #[inline] pub fn maen1(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Match Address Mode Enable 1"]
  #[inline] pub fn set_maen1<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
      if self.osr() != 0 { try!(write!(f, " osr=0x{:x}", self.osr()))}
      if self.m10() != 0 { try!(write!(f, " m10"))}
      if self.maen2() != 0 { try!(write!(f, " maen2"))}
      if self.maen1() != 0 { try!(write!(f, " maen1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Control Register 5"]
#[derive(PartialEq, Eq)]
pub struct C5(pub u8);
impl C5 {
#[doc="Resynchronization Disable"]
  #[inline] pub fn resyncdis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Resynchronization Disable"]
  #[inline] pub fn set_resyncdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Both Edge Sampling"]
  #[inline] pub fn bothedge(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Both Edge Sampling"]
  #[inline] pub fn set_bothedge<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Receiver Full DMA Enable"]
  #[inline] pub fn rdmae(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Receiver Full DMA Enable"]
  #[inline] pub fn set_rdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Transmitter DMA Enable"]
  #[inline] pub fn tdmae(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Transmitter DMA Enable"]
  #[inline] pub fn set_tdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u8 = value.into();
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
      if self.resyncdis() != 0 { try!(write!(f, " resyncdis"))}
      if self.bothedge() != 0 { try!(write!(f, " bothedge"))}
      if self.rdmae() != 0 { try!(write!(f, " rdmae"))}
      if self.tdmae() != 0 { try!(write!(f, " tdmae"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
