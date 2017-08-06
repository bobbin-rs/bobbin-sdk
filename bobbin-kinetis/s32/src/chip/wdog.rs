//! Watchdog timer
#[allow(unused_imports)] use bobbin_common::bits;
pub const WDOG: Wdog = Wdog(0x40052000);

#[doc="Watchdog timer"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wdog(pub u32);
impl Wdog {
#[doc="Get the *const pointer for the CS register."]
  #[inline] pub fn cs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CS register."]
  #[inline] pub fn cs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CS register."]
  #[inline] pub fn cs(&self) -> Cs { 
     unsafe {
        Cs(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CS register."]
  #[inline] pub fn set_cs(&self, value: Cs) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CS register."]
  #[inline] pub fn with_cs<F: FnOnce(Cs) -> Cs>(&self, f: F) -> &Self {
     let tmp = self.cs();
     self.set_cs(f(tmp))
  }

#[doc="Get the *const pointer for the CNT register."]
  #[inline] pub fn cnt_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CNT register."]
  #[inline] pub fn cnt_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CNT register."]
  #[inline] pub fn cnt(&self) -> Cnt { 
     unsafe {
        Cnt(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CNT register."]
  #[inline] pub fn set_cnt(&self, value: Cnt) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CNT register."]
  #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
     let tmp = self.cnt();
     self.set_cnt(f(tmp))
  }

#[doc="Get the *const pointer for the TOVAL register."]
  #[inline] pub fn toval_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the TOVAL register."]
  #[inline] pub fn toval_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the TOVAL register."]
  #[inline] pub fn toval(&self) -> Toval { 
     unsafe {
        Toval(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the TOVAL register."]
  #[inline] pub fn set_toval(&self, value: Toval) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TOVAL register."]
  #[inline] pub fn with_toval<F: FnOnce(Toval) -> Toval>(&self, f: F) -> &Self {
     let tmp = self.toval();
     self.set_toval(f(tmp))
  }

#[doc="Get the *const pointer for the WIN register."]
  #[inline] pub fn win_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the WIN register."]
  #[inline] pub fn win_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the WIN register."]
  #[inline] pub fn win(&self) -> Win { 
     unsafe {
        Win(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the WIN register."]
  #[inline] pub fn set_win(&self, value: Win) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the WIN register."]
  #[inline] pub fn with_win<F: FnOnce(Win) -> Win>(&self, f: F) -> &Self {
     let tmp = self.win();
     self.set_win(f(tmp))
  }

}

#[doc="Watchdog Control and Status Register"]
#[derive(PartialEq, Eq)]
pub struct Cs(pub u32);
impl Cs {
#[doc="Stop Enable"]
  #[inline] pub fn stop(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Stop Enable"]
  #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Wait Enable"]
  #[inline] pub fn wait(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Wait Enable"]
  #[inline] pub fn set_wait<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Debug Enable"]
  #[inline] pub fn dbg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="Debug Enable"]
  #[inline] pub fn set_dbg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Watchdog Test"]
  #[inline] pub fn tst(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
  }
#[doc="Watchdog Test"]
  #[inline] pub fn set_tst<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Allow updates"]
  #[inline] pub fn update(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="Allow updates"]
  #[inline] pub fn set_update<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Watchdog Interrupt"]
  #[inline] pub fn int(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="Watchdog Interrupt"]
  #[inline] pub fn set_int<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Watchdog Enable"]
  #[inline] pub fn en(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="Watchdog Enable"]
  #[inline] pub fn set_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Watchdog Clock"]
  #[inline] pub fn clk(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
  }
#[doc="Watchdog Clock"]
  #[inline] pub fn set_clk<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Reconfiguration Success"]
  #[inline] pub fn rcs(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="Reconfiguration Success"]
  #[inline] pub fn set_rcs<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Unlock status"]
  #[inline] pub fn ulk(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="Unlock status"]
  #[inline] pub fn set_ulk<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Watchdog prescaler"]
  #[inline] pub fn pres(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
  }
#[doc="Watchdog prescaler"]
  #[inline] pub fn set_pres<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
  #[inline] pub fn cmd32en(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
  }
#[doc="Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
  #[inline] pub fn set_cmd32en<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Watchdog Interrupt Flag"]
  #[inline] pub fn flg(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
  }
#[doc="Watchdog Interrupt Flag"]
  #[inline] pub fn set_flg<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Watchdog Window"]
  #[inline] pub fn win(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
  }
#[doc="Watchdog Window"]
  #[inline] pub fn set_win<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.stop() != 0 { try!(write!(f, " stop"))}
      if self.wait() != 0 { try!(write!(f, " wait"))}
      if self.dbg() != 0 { try!(write!(f, " dbg"))}
      if self.tst() != 0 { try!(write!(f, " tst=0x{:x}", self.tst()))}
      if self.update() != 0 { try!(write!(f, " update"))}
      if self.int() != 0 { try!(write!(f, " int"))}
      if self.en() != 0 { try!(write!(f, " en"))}
      if self.clk() != 0 { try!(write!(f, " clk=0x{:x}", self.clk()))}
      if self.rcs() != 0 { try!(write!(f, " rcs"))}
      if self.ulk() != 0 { try!(write!(f, " ulk"))}
      if self.pres() != 0 { try!(write!(f, " pres"))}
      if self.cmd32en() != 0 { try!(write!(f, " cmd32en"))}
      if self.flg() != 0 { try!(write!(f, " flg"))}
      if self.win() != 0 { try!(write!(f, " win"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Counter Register"]
#[derive(PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
#[doc="Low byte of the Watchdog Counter"]
  #[inline] pub fn cntlow(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Low byte of the Watchdog Counter"]
  #[inline] pub fn set_cntlow<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="High byte of the Watchdog Counter"]
  #[inline] pub fn cnthigh(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="High byte of the Watchdog Counter"]
  #[inline] pub fn set_cnthigh<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cntlow() != 0 { try!(write!(f, " cntlow=0x{:x}", self.cntlow()))}
      if self.cnthigh() != 0 { try!(write!(f, " cnthigh=0x{:x}", self.cnthigh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timeout Value Register"]
#[derive(PartialEq, Eq)]
pub struct Toval(pub u32);
impl Toval {
#[doc="Low byte of the timeout value"]
  #[inline] pub fn tovallow(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Low byte of the timeout value"]
  #[inline] pub fn set_tovallow<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="High byte of the timeout value"]
  #[inline] pub fn tovalhigh(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="High byte of the timeout value"]
  #[inline] pub fn set_tovalhigh<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Toval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Toval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tovallow() != 0 { try!(write!(f, " tovallow=0x{:x}", self.tovallow()))}
      if self.tovalhigh() != 0 { try!(write!(f, " tovalhigh=0x{:x}", self.tovalhigh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Window Register"]
#[derive(PartialEq, Eq)]
pub struct Win(pub u32);
impl Win {
#[doc="Low byte of Watchdog Window"]
  #[inline] pub fn winlow(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="Low byte of Watchdog Window"]
  #[inline] pub fn set_winlow<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="High byte of Watchdog Window"]
  #[inline] pub fn winhigh(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="High byte of Watchdog Window"]
  #[inline] pub fn set_winhigh<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Win {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Win {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.winlow() != 0 { try!(write!(f, " winlow=0x{:x}", self.winlow()))}
      if self.winhigh() != 0 { try!(write!(f, " winhigh=0x{:x}", self.winhigh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

