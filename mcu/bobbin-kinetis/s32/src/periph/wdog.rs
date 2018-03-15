//! Watchdog timer

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Watchdog timer"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WdogPeriph(pub usize);
impl WdogPeriph {
    #[doc="Get the *mut pointer for the CS register."]
    #[inline] pub fn cs_mut(&self) -> *mut Cs { 
        (self.0 + 0x0) as *mut Cs
    }

    #[doc="Get the *const pointer for the CS register."]
    #[inline] pub fn cs_ptr(&self) -> *const Cs { 
           self.cs_mut()
    }

    #[doc="Read the CS register."]
    #[inline] pub fn cs(&self) -> Cs { 
        unsafe {
            read_volatile(self.cs_ptr())
        }
    }

    #[doc="Write the CS register."]
    #[inline] pub fn set_cs<F: FnOnce(Cs) -> Cs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cs_mut(), f(Cs(0)));
        }
        self
    }

    #[doc="Modify the CS register."]
    #[inline] pub fn with_cs<F: FnOnce(Cs) -> Cs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cs_mut(), f(self.cs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CNT register."]
    #[inline] pub fn cnt_mut(&self) -> *mut Cnt { 
        (self.0 + 0x4) as *mut Cnt
    }

    #[doc="Get the *const pointer for the CNT register."]
    #[inline] pub fn cnt_ptr(&self) -> *const Cnt { 
           self.cnt_mut()
    }

    #[doc="Read the CNT register."]
    #[inline] pub fn cnt(&self) -> Cnt { 
        unsafe {
            read_volatile(self.cnt_ptr())
        }
    }

    #[doc="Write the CNT register."]
    #[inline] pub fn set_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cnt_mut(), f(Cnt(0)));
        }
        self
    }

    #[doc="Modify the CNT register."]
    #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cnt_mut(), f(self.cnt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TOVAL register."]
    #[inline] pub fn toval_mut(&self) -> *mut Toval { 
        (self.0 + 0x8) as *mut Toval
    }

    #[doc="Get the *const pointer for the TOVAL register."]
    #[inline] pub fn toval_ptr(&self) -> *const Toval { 
           self.toval_mut()
    }

    #[doc="Read the TOVAL register."]
    #[inline] pub fn toval(&self) -> Toval { 
        unsafe {
            read_volatile(self.toval_ptr())
        }
    }

    #[doc="Write the TOVAL register."]
    #[inline] pub fn set_toval<F: FnOnce(Toval) -> Toval>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.toval_mut(), f(Toval(0)));
        }
        self
    }

    #[doc="Modify the TOVAL register."]
    #[inline] pub fn with_toval<F: FnOnce(Toval) -> Toval>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.toval_mut(), f(self.toval()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WIN register."]
    #[inline] pub fn win_mut(&self) -> *mut Win { 
        (self.0 + 0xc) as *mut Win
    }

    #[doc="Get the *const pointer for the WIN register."]
    #[inline] pub fn win_ptr(&self) -> *const Win { 
           self.win_mut()
    }

    #[doc="Read the WIN register."]
    #[inline] pub fn win(&self) -> Win { 
        unsafe {
            read_volatile(self.win_ptr())
        }
    }

    #[doc="Write the WIN register."]
    #[inline] pub fn set_win<F: FnOnce(Win) -> Win>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.win_mut(), f(Win(0)));
        }
        self
    }

    #[doc="Modify the WIN register."]
    #[inline] pub fn with_win<F: FnOnce(Win) -> Win>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.win_mut(), f(self.win()));
        }
        self
    }

}

#[doc="Watchdog Control and Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cs(pub u32);
impl Cs {
    #[doc="Stop Enable"]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
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

    #[doc="Returns true if WAIT != 0"]
    #[inline] pub fn test_wait(&self) -> bool {
        self.wait() != 0
    }

    #[doc="Sets the WAIT field."]
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

    #[doc="Returns true if DBG != 0"]
    #[inline] pub fn test_dbg(&self) -> bool {
        self.dbg() != 0
    }

    #[doc="Sets the DBG field."]
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

    #[doc="Returns true if TST != 0"]
    #[inline] pub fn test_tst(&self) -> bool {
        self.tst() != 0
    }

    #[doc="Sets the TST field."]
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

    #[doc="Returns true if UPDATE != 0"]
    #[inline] pub fn test_update(&self) -> bool {
        self.update() != 0
    }

    #[doc="Sets the UPDATE field."]
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

    #[doc="Returns true if INT != 0"]
    #[inline] pub fn test_int(&self) -> bool {
        self.int() != 0
    }

    #[doc="Sets the INT field."]
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

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="Sets the EN field."]
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

    #[doc="Returns true if CLK != 0"]
    #[inline] pub fn test_clk(&self) -> bool {
        self.clk() != 0
    }

    #[doc="Sets the CLK field."]
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

    #[doc="Returns true if RCS != 0"]
    #[inline] pub fn test_rcs(&self) -> bool {
        self.rcs() != 0
    }

    #[doc="Sets the RCS field."]
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

    #[doc="Returns true if ULK != 0"]
    #[inline] pub fn test_ulk(&self) -> bool {
        self.ulk() != 0
    }

    #[doc="Sets the ULK field."]
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

    #[doc="Returns true if PRES != 0"]
    #[inline] pub fn test_pres(&self) -> bool {
        self.pres() != 0
    }

    #[doc="Sets the PRES field."]
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

    #[doc="Returns true if CMD32EN != 0"]
    #[inline] pub fn test_cmd32en(&self) -> bool {
        self.cmd32en() != 0
    }

    #[doc="Sets the CMD32EN field."]
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

    #[doc="Returns true if FLG != 0"]
    #[inline] pub fn test_flg(&self) -> bool {
        self.flg() != 0
    }

    #[doc="Sets the FLG field."]
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

    #[doc="Returns true if WIN != 0"]
    #[inline] pub fn test_win(&self) -> bool {
        self.win() != 0
    }

    #[doc="Sets the WIN field."]
    #[inline] pub fn set_win<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Cs {
    #[inline]
    fn from(other: u32) -> Self {
         Cs(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="Low byte of the Watchdog Counter"]
    #[inline] pub fn cntlow(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CNTLOW != 0"]
    #[inline] pub fn test_cntlow(&self) -> bool {
        self.cntlow() != 0
    }

    #[doc="Sets the CNTLOW field."]
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

    #[doc="Returns true if CNTHIGH != 0"]
    #[inline] pub fn test_cnthigh(&self) -> bool {
        self.cnthigh() != 0
    }

    #[doc="Sets the CNTHIGH field."]
    #[inline] pub fn set_cnthigh<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Cnt {
    #[inline]
    fn from(other: u32) -> Self {
         Cnt(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Toval(pub u32);
impl Toval {
    #[doc="Low byte of the timeout value"]
    #[inline] pub fn tovallow(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TOVALLOW != 0"]
    #[inline] pub fn test_tovallow(&self) -> bool {
        self.tovallow() != 0
    }

    #[doc="Sets the TOVALLOW field."]
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

    #[doc="Returns true if TOVALHIGH != 0"]
    #[inline] pub fn test_tovalhigh(&self) -> bool {
        self.tovalhigh() != 0
    }

    #[doc="Sets the TOVALHIGH field."]
    #[inline] pub fn set_tovalhigh<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Toval {
    #[inline]
    fn from(other: u32) -> Self {
         Toval(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Win(pub u32);
impl Win {
    #[doc="Low byte of Watchdog Window"]
    #[inline] pub fn winlow(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if WINLOW != 0"]
    #[inline] pub fn test_winlow(&self) -> bool {
        self.winlow() != 0
    }

    #[doc="Sets the WINLOW field."]
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

    #[doc="Returns true if WINHIGH != 0"]
    #[inline] pub fn test_winhigh(&self) -> bool {
        self.winhigh() != 0
    }

    #[doc="Sets the WINHIGH field."]
    #[inline] pub fn set_winhigh<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Win {
    #[inline]
    fn from(other: u32) -> Self {
         Win(other)
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

