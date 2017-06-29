pub const SERCOM0: Sercom0 = Periph(0x42000800, Sercom0Id {});
pub const SERCOM1: Sercom1 = Periph(0x42000c00, Sercom1Id {});
pub const SERCOM2: Sercom2 = Periph(0x42001000, Sercom2Id {});
pub const SERCOM3: Sercom3 = Periph(0x42001400, Sercom3Id {});
pub const SERCOM4: Sercom4 = Periph(0x42001800, Sercom4Id {});
pub const SERCOM5: Sercom5 = Periph(0x42001c00, Sercom5Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SERCOM Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Sercom0Id {}
pub type Sercom0 = Periph<Sercom0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Sercom1Id {}
pub type Sercom1 = Periph<Sercom1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Sercom2Id {}
pub type Sercom2 = Periph<Sercom2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Sercom3Id {}
pub type Sercom3 = Periph<Sercom3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Sercom4Id {}
pub type Sercom4 = Periph<Sercom4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Sercom5Id {}
pub type Sercom5 = Periph<Sercom5Id>;

impl super::sig::Signal<super::sig::Sercom0Pad0> for Sercom0 {}
impl super::sig::SignalPad0<super::sig::Sercom0Pad0> for Sercom0 {}
impl super::sig::Signal<super::sig::Sercom0Pad1> for Sercom0 {}
impl super::sig::SignalPad1<super::sig::Sercom0Pad1> for Sercom0 {}
impl super::sig::Signal<super::sig::Sercom0Pad2> for Sercom0 {}
impl super::sig::SignalPad2<super::sig::Sercom0Pad2> for Sercom0 {}
impl super::sig::Signal<super::sig::Sercom0Pad3> for Sercom0 {}
impl super::sig::SignalPad3<super::sig::Sercom0Pad3> for Sercom0 {}

impl super::sig::Signal<super::sig::Sercom1Pad0> for Sercom1 {}
impl super::sig::SignalPad0<super::sig::Sercom1Pad0> for Sercom1 {}
impl super::sig::Signal<super::sig::Sercom1Pad1> for Sercom1 {}
impl super::sig::SignalPad1<super::sig::Sercom1Pad1> for Sercom1 {}
impl super::sig::Signal<super::sig::Sercom1Pad2> for Sercom1 {}
impl super::sig::SignalPad2<super::sig::Sercom1Pad2> for Sercom1 {}
impl super::sig::Signal<super::sig::Sercom1Pad3> for Sercom1 {}
impl super::sig::SignalPad3<super::sig::Sercom1Pad3> for Sercom1 {}

impl super::sig::Signal<super::sig::Sercom2Pad0> for Sercom2 {}
impl super::sig::SignalPad0<super::sig::Sercom2Pad0> for Sercom2 {}
impl super::sig::Signal<super::sig::Sercom2Pad1> for Sercom2 {}
impl super::sig::SignalPad1<super::sig::Sercom2Pad1> for Sercom2 {}
impl super::sig::Signal<super::sig::Sercom2Pad2> for Sercom2 {}
impl super::sig::SignalPad2<super::sig::Sercom2Pad2> for Sercom2 {}
impl super::sig::Signal<super::sig::Sercom2Pad3> for Sercom2 {}
impl super::sig::SignalPad3<super::sig::Sercom2Pad3> for Sercom2 {}

impl super::sig::Signal<super::sig::Sercom3Pad0> for Sercom3 {}
impl super::sig::SignalPad0<super::sig::Sercom3Pad0> for Sercom3 {}
impl super::sig::Signal<super::sig::Sercom3Pad1> for Sercom3 {}
impl super::sig::SignalPad1<super::sig::Sercom3Pad1> for Sercom3 {}
impl super::sig::Signal<super::sig::Sercom3Pad2> for Sercom3 {}
impl super::sig::SignalPad2<super::sig::Sercom3Pad2> for Sercom3 {}
impl super::sig::Signal<super::sig::Sercom3Pad3> for Sercom3 {}
impl super::sig::SignalPad3<super::sig::Sercom3Pad3> for Sercom3 {}

impl super::sig::Signal<super::sig::Sercom4Pad0> for Sercom4 {}
impl super::sig::SignalPad0<super::sig::Sercom4Pad0> for Sercom4 {}
impl super::sig::Signal<super::sig::Sercom4Pad1> for Sercom4 {}
impl super::sig::SignalPad1<super::sig::Sercom4Pad1> for Sercom4 {}
impl super::sig::Signal<super::sig::Sercom4Pad2> for Sercom4 {}
impl super::sig::SignalPad2<super::sig::Sercom4Pad2> for Sercom4 {}
impl super::sig::Signal<super::sig::Sercom4Pad3> for Sercom4 {}
impl super::sig::SignalPad3<super::sig::Sercom4Pad3> for Sercom4 {}

impl super::sig::Signal<super::sig::Sercom5Pad0> for Sercom5 {}
impl super::sig::SignalPad0<super::sig::Sercom5Pad0> for Sercom5 {}
impl super::sig::Signal<super::sig::Sercom5Pad1> for Sercom5 {}
impl super::sig::SignalPad1<super::sig::Sercom5Pad1> for Sercom5 {}
impl super::sig::Signal<super::sig::Sercom5Pad2> for Sercom5 {}
impl super::sig::SignalPad2<super::sig::Sercom5Pad2> for Sercom5 {}
impl super::sig::Signal<super::sig::Sercom5Pad3> for Sercom5 {}
impl super::sig::SignalPad3<super::sig::Sercom5Pad3> for Sercom5 {}


impl<T> Periph<T> {
#[doc="Get I2C Master Mode Peripheral"]
   #[inline] pub fn i2cm(&self) -> i2cm::I2cm {
      i2cm::I2cm(self.0 + 0x0)
   }
#[doc="Get I2C Slave Mode Peripheral"]
   #[inline] pub fn i2cs(&self) -> i2cs::I2cs {
      i2cs::I2cs(self.0 + 0x0)
   }
#[doc="Get SPI Mode Peripheral"]
   #[inline] pub fn spi(&self) -> spi::Spi {
      spi::Spi(self.0 + 0x0)
   }
#[doc="Get USART Mode Peripheral"]
   #[inline] pub fn usart(&self) -> usart::Usart {
      usart::Usart(self.0 + 0x0)
   }
}
#[doc="I2C Master Mode Cluster"]
pub mod i2cm {
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C Master Mode Peripheral"]
   pub struct I2cm(pub u32);
impl I2cm {
#[doc="Get the *const pointer for the ADDR register."]
  #[inline] pub fn addr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the ADDR register."]
  #[inline] pub fn addr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the ADDR register."]
  #[inline] pub fn addr(&self) -> Addr { 
     unsafe {
        Addr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the ADDR register."]
  #[inline] pub fn set_addr(&self, value: Addr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ADDR register."]
  #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
     let tmp = self.addr();
     self.set_addr(f(tmp))
  }

#[doc="Get the *const pointer for the BAUD register."]
  #[inline] pub fn baud_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the BAUD register."]
  #[inline] pub fn baud_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the BAUD register."]
  #[inline] pub fn baud(&self) -> Baud { 
     unsafe {
        Baud(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the BAUD register."]
  #[inline] pub fn set_baud(&self, value: Baud) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BAUD register."]
  #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
     let tmp = self.baud();
     self.set_baud(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLA register."]
  #[inline] pub fn ctrla_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CTRLA register."]
  #[inline] pub fn ctrla_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CTRLA register."]
  #[inline] pub fn ctrla(&self) -> Ctrla { 
     unsafe {
        Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CTRLA register."]
  #[inline] pub fn set_ctrla(&self, value: Ctrla) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTRLA register."]
  #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLB register."]
  #[inline] pub fn ctrlb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CTRLB register."]
  #[inline] pub fn ctrlb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CTRLB register."]
  #[inline] pub fn ctrlb(&self) -> Ctrlb { 
     unsafe {
        Ctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CTRLB register."]
  #[inline] pub fn set_ctrlb(&self, value: Ctrlb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTRLB register."]
  #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
     let tmp = self.ctrlb();
     self.set_ctrlb(f(tmp))
  }

#[doc="Get the *const pointer for the DATA register."]
  #[inline] pub fn data_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x28) as *const u8
  }
#[doc="Get the *mut pointer for the DATA register."]
  #[inline] pub fn data_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x28) as *mut u8
  }
#[doc="Read the DATA register."]
  #[inline] pub fn data(&self) -> Data { 
     unsafe {
        Data(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u8))
     }
  }
#[doc="Write the DATA register."]
  #[inline] pub fn set_data(&self, value: Data) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DATA register."]
  #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
     let tmp = self.data();
     self.set_data(f(tmp))
  }

#[doc="Get the *const pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x30) as *const u8
  }
#[doc="Get the *mut pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x30) as *mut u8
  }
#[doc="Read the DBGCTRL register."]
  #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
        Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u8))
     }
  }
#[doc="Write the DBGCTRL register."]
  #[inline] pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DBGCTRL register."]
  #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

#[doc="Get the *const pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x14) as *const u8
  }
#[doc="Get the *mut pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x14) as *mut u8
  }
#[doc="Read the INTENCLR register."]
  #[inline] pub fn intenclr(&self) -> Intenclr { 
     unsafe {
        Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u8))
     }
  }
#[doc="Write the INTENCLR register."]
  #[inline] pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENCLR register."]
  #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

#[doc="Get the *const pointer for the INTENSET register."]
  #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x16) as *const u8
  }
#[doc="Get the *mut pointer for the INTENSET register."]
  #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x16) as *mut u8
  }
#[doc="Read the INTENSET register."]
  #[inline] pub fn intenset(&self) -> Intenset { 
     unsafe {
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x16) as *const u8))
     }
  }
#[doc="Write the INTENSET register."]
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x16) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENSET register."]
  #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

#[doc="Get the *const pointer for the INTFLAG register."]
  #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x18) as *const u8
  }
#[doc="Get the *mut pointer for the INTFLAG register."]
  #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x18) as *mut u8
  }
#[doc="Read the INTFLAG register."]
  #[inline] pub fn intflag(&self) -> Intflag { 
     unsafe {
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u8))
     }
  }
#[doc="Write the INTFLAG register."]
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTFLAG register."]
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

#[doc="Get the *const pointer for the STATUS register."]
  #[inline] pub fn status_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x1a) as *const u16
  }
#[doc="Get the *mut pointer for the STATUS register."]
  #[inline] pub fn status_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x1a) as *mut u16
  }
#[doc="Read the STATUS register."]
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x1a) as *const u16))
     }
  }
#[doc="Write the STATUS register."]
  #[inline] pub fn set_status(&self, value: Status) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the STATUS register."]
  #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

#[doc="Get the *const pointer for the SYNCBUSY register."]
  #[inline] pub fn syncbusy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the SYNCBUSY register."]
  #[inline] pub fn syncbusy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the SYNCBUSY register."]
  #[inline] pub fn syncbusy(&self) -> Syncbusy { 
     unsafe {
        Syncbusy(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }

}

#[doc="I2CM Address"]
#[derive(PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
#[doc="Address Value"]
  #[inline] pub fn addr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7ff // [10:0]
  }
#[doc="Address Value"]
  #[inline] pub fn set_addr(mut self, value: u32) -> Self {
     assert!((value & !0x7ff) == 0);
     self.0 &= !(0x7ff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Length Enable"]
  #[inline] pub fn lenen(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Length Enable"]
  #[inline] pub fn set_lenen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="High Speed Mode"]
  #[inline] pub fn hs(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="High Speed Mode"]
  #[inline] pub fn set_hs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Ten Bit Addressing Enable"]
  #[inline] pub fn tenbiten(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Ten Bit Addressing Enable"]
  #[inline] pub fn set_tenbiten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Length"]
  #[inline] pub fn len(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
#[doc="Length"]
  #[inline] pub fn set_len(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Addr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Addr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self.lenen() != 0 { try!(write!(f, " lenen"))}
      if self.hs() != 0 { try!(write!(f, " hs"))}
      if self.tenbiten() != 0 { try!(write!(f, " tenbiten"))}
      if self.len() != 0 { try!(write!(f, " len=0x{:x}", self.len()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CM Baud Rate"]
#[derive(PartialEq, Eq)]
pub struct Baud(pub u32);
impl Baud {
#[doc="Baud Rate Value"]
  #[inline] pub fn baud(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Baud Rate Value"]
  #[inline] pub fn set_baud(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Baud Rate Value Low"]
  #[inline] pub fn baudlow(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
#[doc="Baud Rate Value Low"]
  #[inline] pub fn set_baudlow(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="High Speed Baud Rate Value"]
  #[inline] pub fn hsbaud(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
#[doc="High Speed Baud Rate Value"]
  #[inline] pub fn set_hsbaud(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="High Speed Baud Rate Value Low"]
  #[inline] pub fn hsbaudlow(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
#[doc="High Speed Baud Rate Value Low"]
  #[inline] pub fn set_hsbaudlow(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Baud {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Baud {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.baud() != 0 { try!(write!(f, " baud=0x{:x}", self.baud()))}
      if self.baudlow() != 0 { try!(write!(f, " baudlow=0x{:x}", self.baudlow()))}
      if self.hsbaud() != 0 { try!(write!(f, " hsbaud=0x{:x}", self.hsbaud()))}
      if self.hsbaudlow() != 0 { try!(write!(f, " hsbaudlow=0x{:x}", self.hsbaudlow()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CM Control A"]
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
#[doc="Software Reset"]
  #[inline] pub fn swrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset"]
  #[inline] pub fn set_swrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Enable"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Enable"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Operating Mode"]
  #[inline] pub fn mode(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x7 // [4:2]
  }
#[doc="Operating Mode"]
  #[inline] pub fn set_mode(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Pin Usage"]
  #[inline] pub fn pinout(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Pin Usage"]
  #[inline] pub fn set_pinout(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="SDA Hold Time"]
  #[inline] pub fn sdahold(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="SDA Hold Time"]
  #[inline] pub fn set_sdahold(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Master SCL Low Extend Timeout"]
  #[inline] pub fn mexttoen(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="Master SCL Low Extend Timeout"]
  #[inline] pub fn set_mexttoen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Slave SCL Low Extend Timeout"]
  #[inline] pub fn sexttoen(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="Slave SCL Low Extend Timeout"]
  #[inline] pub fn set_sexttoen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Transfer Speed"]
  #[inline] pub fn speed(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3 // [25:24]
  }
#[doc="Transfer Speed"]
  #[inline] pub fn set_speed(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="SCL Clock Stretch Mode"]
  #[inline] pub fn sclsm(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
#[doc="SCL Clock Stretch Mode"]
  #[inline] pub fn set_sclsm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Inactive Time-Out"]
  #[inline] pub fn inactout(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
#[doc="Inactive Time-Out"]
  #[inline] pub fn set_inactout(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="SCL Low Timeout Enable"]
  #[inline] pub fn lowtouten(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="SCL Low Timeout Enable"]
  #[inline] pub fn set_lowtouten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

}
impl ::core::fmt::Display for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.pinout() != 0 { try!(write!(f, " pinout"))}
      if self.sdahold() != 0 { try!(write!(f, " sdahold=0x{:x}", self.sdahold()))}
      if self.mexttoen() != 0 { try!(write!(f, " mexttoen"))}
      if self.sexttoen() != 0 { try!(write!(f, " sexttoen"))}
      if self.speed() != 0 { try!(write!(f, " speed=0x{:x}", self.speed()))}
      if self.sclsm() != 0 { try!(write!(f, " sclsm"))}
      if self.inactout() != 0 { try!(write!(f, " inactout=0x{:x}", self.inactout()))}
      if self.lowtouten() != 0 { try!(write!(f, " lowtouten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CM Control B"]
#[derive(PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
#[doc="Smart Mode Enable"]
  #[inline] pub fn smen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Smart Mode Enable"]
  #[inline] pub fn set_smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Quick Command Enable"]
  #[inline] pub fn qcen(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Quick Command Enable"]
  #[inline] pub fn set_qcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Command"]
  #[inline] pub fn cmd(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
#[doc="Command"]
  #[inline] pub fn set_cmd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Acknowledge Action"]
  #[inline] pub fn ackact(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Acknowledge Action"]
  #[inline] pub fn set_ackact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}
impl ::core::fmt::Display for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.smen() != 0 { try!(write!(f, " smen"))}
      if self.qcen() != 0 { try!(write!(f, " qcen"))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      if self.ackact() != 0 { try!(write!(f, " ackact"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CM Data"]
#[derive(PartialEq, Eq)]
pub struct Data(pub u8);
impl Data {
#[doc="Data Value"]
  #[inline] pub fn data(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
#[doc="Data Value"]
  #[inline] pub fn set_data(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CM Debug Control"]
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Mode"]
  #[inline] pub fn dbgstop(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Debug Mode"]
  #[inline] pub fn set_dbgstop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgstop() != 0 { try!(write!(f, " dbgstop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CM Interrupt Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Master On Bus Interrupt Disable"]
  #[inline] pub fn mb(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Master On Bus Interrupt Disable"]
  #[inline] pub fn set_mb(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Slave On Bus Interrupt Disable"]
  #[inline] pub fn sb(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Slave On Bus Interrupt Disable"]
  #[inline] pub fn set_sb(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Combined Error Interrupt Disable"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt Disable"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mb() != 0 { try!(write!(f, " mb"))}
      if self.sb() != 0 { try!(write!(f, " sb"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CM Interrupt Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Master On Bus Interrupt Enable"]
  #[inline] pub fn mb(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Master On Bus Interrupt Enable"]
  #[inline] pub fn set_mb(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Slave On Bus Interrupt Enable"]
  #[inline] pub fn sb(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Slave On Bus Interrupt Enable"]
  #[inline] pub fn set_sb(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Combined Error Interrupt Enable"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt Enable"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mb() != 0 { try!(write!(f, " mb"))}
      if self.sb() != 0 { try!(write!(f, " sb"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CM Interrupt Flag Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Master On Bus Interrupt"]
  #[inline] pub fn mb(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Master On Bus Interrupt"]
  #[inline] pub fn set_mb(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Slave On Bus Interrupt"]
  #[inline] pub fn sb(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Slave On Bus Interrupt"]
  #[inline] pub fn set_sb(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Combined Error Interrupt"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mb() != 0 { try!(write!(f, " mb"))}
      if self.sb() != 0 { try!(write!(f, " sb"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CM Status"]
#[derive(PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
#[doc="Bus Error"]
  #[inline] pub fn buserr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
#[doc="Bus Error"]
  #[inline] pub fn set_buserr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Arbitration Lost"]
  #[inline] pub fn arblost(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="Arbitration Lost"]
  #[inline] pub fn set_arblost(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Received Not Acknowledge"]
  #[inline] pub fn rxnack(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
#[doc="Received Not Acknowledge"]
  #[inline] pub fn set_rxnack(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Bus State"]
  #[inline] pub fn busstate(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x3 // [5:4]
  }
#[doc="Bus State"]
  #[inline] pub fn set_busstate(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SCL Low Timeout"]
  #[inline] pub fn lowtout(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
#[doc="SCL Low Timeout"]
  #[inline] pub fn set_lowtout(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Clock Hold"]
  #[inline] pub fn clkhold(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
#[doc="Clock Hold"]
  #[inline] pub fn set_clkhold(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Master SCL Low Extend Timeout"]
  #[inline] pub fn mexttout(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
#[doc="Master SCL Low Extend Timeout"]
  #[inline] pub fn set_mexttout(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Slave SCL Low Extend Timeout"]
  #[inline] pub fn sexttout(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
#[doc="Slave SCL Low Extend Timeout"]
  #[inline] pub fn set_sexttout(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Length Error"]
  #[inline] pub fn lenerr(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
#[doc="Length Error"]
  #[inline] pub fn set_lenerr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.buserr() != 0 { try!(write!(f, " buserr"))}
      if self.arblost() != 0 { try!(write!(f, " arblost"))}
      if self.rxnack() != 0 { try!(write!(f, " rxnack"))}
      if self.busstate() != 0 { try!(write!(f, " busstate=0x{:x}", self.busstate()))}
      if self.lowtout() != 0 { try!(write!(f, " lowtout"))}
      if self.clkhold() != 0 { try!(write!(f, " clkhold"))}
      if self.mexttout() != 0 { try!(write!(f, " mexttout"))}
      if self.sexttout() != 0 { try!(write!(f, " sexttout"))}
      if self.lenerr() != 0 { try!(write!(f, " lenerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CM Syncbusy"]
#[derive(PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
#[doc="Software Reset Synchronization Busy"]
  #[inline] pub fn swrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset Synchronization Busy"]
  #[inline] pub fn set_swrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SERCOM Enable Synchronization Busy"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="SERCOM Enable Synchronization Busy"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="System Operation Synchronization Busy"]
  #[inline] pub fn sysop(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="System Operation Synchronization Busy"]
  #[inline] pub fn set_sysop(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Syncbusy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Syncbusy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.sysop() != 0 { try!(write!(f, " sysop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of i2cm
#[doc="I2C Slave Mode Cluster"]
pub mod i2cs {
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C Slave Mode Peripheral"]
   pub struct I2cs(pub u32);
impl I2cs {
#[doc="Get the *const pointer for the ADDR register."]
  #[inline] pub fn addr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the ADDR register."]
  #[inline] pub fn addr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the ADDR register."]
  #[inline] pub fn addr(&self) -> Addr { 
     unsafe {
        Addr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the ADDR register."]
  #[inline] pub fn set_addr(&self, value: Addr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ADDR register."]
  #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
     let tmp = self.addr();
     self.set_addr(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLA register."]
  #[inline] pub fn ctrla_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CTRLA register."]
  #[inline] pub fn ctrla_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CTRLA register."]
  #[inline] pub fn ctrla(&self) -> Ctrla { 
     unsafe {
        Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CTRLA register."]
  #[inline] pub fn set_ctrla(&self, value: Ctrla) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTRLA register."]
  #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLB register."]
  #[inline] pub fn ctrlb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CTRLB register."]
  #[inline] pub fn ctrlb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CTRLB register."]
  #[inline] pub fn ctrlb(&self) -> Ctrlb { 
     unsafe {
        Ctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CTRLB register."]
  #[inline] pub fn set_ctrlb(&self, value: Ctrlb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTRLB register."]
  #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
     let tmp = self.ctrlb();
     self.set_ctrlb(f(tmp))
  }

#[doc="Get the *const pointer for the DATA register."]
  #[inline] pub fn data_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x28) as *const u8
  }
#[doc="Get the *mut pointer for the DATA register."]
  #[inline] pub fn data_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x28) as *mut u8
  }
#[doc="Read the DATA register."]
  #[inline] pub fn data(&self) -> Data { 
     unsafe {
        Data(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u8))
     }
  }
#[doc="Write the DATA register."]
  #[inline] pub fn set_data(&self, value: Data) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DATA register."]
  #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
     let tmp = self.data();
     self.set_data(f(tmp))
  }

#[doc="Get the *const pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x30) as *const u8
  }
#[doc="Get the *mut pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x30) as *mut u8
  }
#[doc="Read the DBGCTRL register."]
  #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
        Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u8))
     }
  }
#[doc="Write the DBGCTRL register."]
  #[inline] pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DBGCTRL register."]
  #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

#[doc="Get the *const pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x14) as *const u8
  }
#[doc="Get the *mut pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x14) as *mut u8
  }
#[doc="Read the INTENCLR register."]
  #[inline] pub fn intenclr(&self) -> Intenclr { 
     unsafe {
        Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u8))
     }
  }
#[doc="Write the INTENCLR register."]
  #[inline] pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENCLR register."]
  #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

#[doc="Get the *const pointer for the INTENSET register."]
  #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x16) as *const u8
  }
#[doc="Get the *mut pointer for the INTENSET register."]
  #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x16) as *mut u8
  }
#[doc="Read the INTENSET register."]
  #[inline] pub fn intenset(&self) -> Intenset { 
     unsafe {
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x16) as *const u8))
     }
  }
#[doc="Write the INTENSET register."]
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x16) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENSET register."]
  #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

#[doc="Get the *const pointer for the INTFLAG register."]
  #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x18) as *const u8
  }
#[doc="Get the *mut pointer for the INTFLAG register."]
  #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x18) as *mut u8
  }
#[doc="Read the INTFLAG register."]
  #[inline] pub fn intflag(&self) -> Intflag { 
     unsafe {
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u8))
     }
  }
#[doc="Write the INTFLAG register."]
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTFLAG register."]
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

#[doc="Get the *const pointer for the STATUS register."]
  #[inline] pub fn status_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x1a) as *const u16
  }
#[doc="Get the *mut pointer for the STATUS register."]
  #[inline] pub fn status_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x1a) as *mut u16
  }
#[doc="Read the STATUS register."]
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x1a) as *const u16))
     }
  }
#[doc="Write the STATUS register."]
  #[inline] pub fn set_status(&self, value: Status) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the STATUS register."]
  #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

#[doc="Get the *const pointer for the SYNCBUSY register."]
  #[inline] pub fn syncbusy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the SYNCBUSY register."]
  #[inline] pub fn syncbusy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the SYNCBUSY register."]
  #[inline] pub fn syncbusy(&self) -> Syncbusy { 
     unsafe {
        Syncbusy(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }

}

#[doc="I2CS Address"]
#[derive(PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
#[doc="General Call Address Enable"]
  #[inline] pub fn gencen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="General Call Address Enable"]
  #[inline] pub fn set_gencen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Address Value"]
  #[inline] pub fn addr(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3ff // [10:1]
  }
#[doc="Address Value"]
  #[inline] pub fn set_addr(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Ten Bit Addressing Enable"]
  #[inline] pub fn tenbiten(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Ten Bit Addressing Enable"]
  #[inline] pub fn set_tenbiten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Address Mask"]
  #[inline] pub fn addrmask(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x3ff // [26:17]
  }
#[doc="Address Mask"]
  #[inline] pub fn set_addrmask(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 17);
     self.0 |= value << 17;
     self
  }

}
impl ::core::fmt::Display for Addr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Addr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gencen() != 0 { try!(write!(f, " gencen"))}
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self.tenbiten() != 0 { try!(write!(f, " tenbiten"))}
      if self.addrmask() != 0 { try!(write!(f, " addrmask=0x{:x}", self.addrmask()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CS Control A"]
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
#[doc="Software Reset"]
  #[inline] pub fn swrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset"]
  #[inline] pub fn set_swrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Enable"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Enable"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Operating Mode"]
  #[inline] pub fn mode(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x7 // [4:2]
  }
#[doc="Operating Mode"]
  #[inline] pub fn set_mode(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Run during Standby"]
  #[inline] pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Run during Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Pin Usage"]
  #[inline] pub fn pinout(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Pin Usage"]
  #[inline] pub fn set_pinout(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="SDA Hold Time"]
  #[inline] pub fn sdahold(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="SDA Hold Time"]
  #[inline] pub fn set_sdahold(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Slave SCL Low Extend Timeout"]
  #[inline] pub fn sexttoen(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="Slave SCL Low Extend Timeout"]
  #[inline] pub fn set_sexttoen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Transfer Speed"]
  #[inline] pub fn speed(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3 // [25:24]
  }
#[doc="Transfer Speed"]
  #[inline] pub fn set_speed(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="SCL Clock Stretch Mode"]
  #[inline] pub fn sclsm(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
#[doc="SCL Clock Stretch Mode"]
  #[inline] pub fn set_sclsm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="SCL Low Timeout Enable"]
  #[inline] pub fn lowtouten(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="SCL Low Timeout Enable"]
  #[inline] pub fn set_lowtouten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

}
impl ::core::fmt::Display for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.pinout() != 0 { try!(write!(f, " pinout"))}
      if self.sdahold() != 0 { try!(write!(f, " sdahold=0x{:x}", self.sdahold()))}
      if self.sexttoen() != 0 { try!(write!(f, " sexttoen"))}
      if self.speed() != 0 { try!(write!(f, " speed=0x{:x}", self.speed()))}
      if self.sclsm() != 0 { try!(write!(f, " sclsm"))}
      if self.lowtouten() != 0 { try!(write!(f, " lowtouten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CS Control B"]
#[derive(PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
#[doc="Smart Mode Enable"]
  #[inline] pub fn smen(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Smart Mode Enable"]
  #[inline] pub fn set_smen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="PMBus Group Command"]
  #[inline] pub fn gcmd(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="PMBus Group Command"]
  #[inline] pub fn set_gcmd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Automatic Address Acknowledge"]
  #[inline] pub fn aacken(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Automatic Address Acknowledge"]
  #[inline] pub fn set_aacken(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Address Mode"]
  #[inline] pub fn amode(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
#[doc="Address Mode"]
  #[inline] pub fn set_amode(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Command"]
  #[inline] pub fn cmd(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
#[doc="Command"]
  #[inline] pub fn set_cmd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Acknowledge Action"]
  #[inline] pub fn ackact(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Acknowledge Action"]
  #[inline] pub fn set_ackact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}
impl ::core::fmt::Display for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.smen() != 0 { try!(write!(f, " smen"))}
      if self.gcmd() != 0 { try!(write!(f, " gcmd"))}
      if self.aacken() != 0 { try!(write!(f, " aacken"))}
      if self.amode() != 0 { try!(write!(f, " amode=0x{:x}", self.amode()))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      if self.ackact() != 0 { try!(write!(f, " ackact"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CS Data"]
#[derive(PartialEq, Eq)]
pub struct Data(pub u8);
impl Data {
#[doc="Data Value"]
  #[inline] pub fn data(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
#[doc="Data Value"]
  #[inline] pub fn set_data(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CS Debug Control"]
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Mode"]
  #[inline] pub fn dbgstop(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Debug Mode"]
  #[inline] pub fn set_dbgstop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgstop() != 0 { try!(write!(f, " dbgstop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CS Interrupt Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Stop Received Interrupt Disable"]
  #[inline] pub fn prec(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Stop Received Interrupt Disable"]
  #[inline] pub fn set_prec(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Address Match Interrupt Disable"]
  #[inline] pub fn amatch(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Address Match Interrupt Disable"]
  #[inline] pub fn set_amatch(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Data Interrupt Disable"]
  #[inline] pub fn drdy(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Data Interrupt Disable"]
  #[inline] pub fn set_drdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Combined Error Interrupt Disable"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt Disable"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prec() != 0 { try!(write!(f, " prec"))}
      if self.amatch() != 0 { try!(write!(f, " amatch"))}
      if self.drdy() != 0 { try!(write!(f, " drdy"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CS Interrupt Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Stop Received Interrupt Enable"]
  #[inline] pub fn prec(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Stop Received Interrupt Enable"]
  #[inline] pub fn set_prec(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Address Match Interrupt Enable"]
  #[inline] pub fn amatch(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Address Match Interrupt Enable"]
  #[inline] pub fn set_amatch(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Data Interrupt Enable"]
  #[inline] pub fn drdy(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Data Interrupt Enable"]
  #[inline] pub fn set_drdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Combined Error Interrupt Enable"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt Enable"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prec() != 0 { try!(write!(f, " prec"))}
      if self.amatch() != 0 { try!(write!(f, " amatch"))}
      if self.drdy() != 0 { try!(write!(f, " drdy"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CS Interrupt Flag Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Stop Received Interrupt"]
  #[inline] pub fn prec(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Stop Received Interrupt"]
  #[inline] pub fn set_prec(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Address Match Interrupt"]
  #[inline] pub fn amatch(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Address Match Interrupt"]
  #[inline] pub fn set_amatch(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Data Interrupt"]
  #[inline] pub fn drdy(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Data Interrupt"]
  #[inline] pub fn set_drdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Combined Error Interrupt"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prec() != 0 { try!(write!(f, " prec"))}
      if self.amatch() != 0 { try!(write!(f, " amatch"))}
      if self.drdy() != 0 { try!(write!(f, " drdy"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CS Status"]
#[derive(PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
#[doc="Bus Error"]
  #[inline] pub fn buserr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
#[doc="Bus Error"]
  #[inline] pub fn set_buserr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transmit Collision"]
  #[inline] pub fn coll(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="Transmit Collision"]
  #[inline] pub fn set_coll(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Received Not Acknowledge"]
  #[inline] pub fn rxnack(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
#[doc="Received Not Acknowledge"]
  #[inline] pub fn set_rxnack(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Read/Write Direction"]
  #[inline] pub fn dir(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
#[doc="Read/Write Direction"]
  #[inline] pub fn set_dir(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Repeated Start"]
  #[inline] pub fn sr(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
#[doc="Repeated Start"]
  #[inline] pub fn set_sr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SCL Low Timeout"]
  #[inline] pub fn lowtout(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
#[doc="SCL Low Timeout"]
  #[inline] pub fn set_lowtout(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Clock Hold"]
  #[inline] pub fn clkhold(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
#[doc="Clock Hold"]
  #[inline] pub fn set_clkhold(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Slave SCL Low Extend Timeout"]
  #[inline] pub fn sexttout(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
#[doc="Slave SCL Low Extend Timeout"]
  #[inline] pub fn set_sexttout(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="High Speed"]
  #[inline] pub fn hs(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
#[doc="High Speed"]
  #[inline] pub fn set_hs(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.buserr() != 0 { try!(write!(f, " buserr"))}
      if self.coll() != 0 { try!(write!(f, " coll"))}
      if self.rxnack() != 0 { try!(write!(f, " rxnack"))}
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.sr() != 0 { try!(write!(f, " sr"))}
      if self.lowtout() != 0 { try!(write!(f, " lowtout"))}
      if self.clkhold() != 0 { try!(write!(f, " clkhold"))}
      if self.sexttout() != 0 { try!(write!(f, " sexttout"))}
      if self.hs() != 0 { try!(write!(f, " hs"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="I2CS Syncbusy"]
#[derive(PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
#[doc="Software Reset Synchronization Busy"]
  #[inline] pub fn swrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset Synchronization Busy"]
  #[inline] pub fn set_swrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SERCOM Enable Synchronization Busy"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="SERCOM Enable Synchronization Busy"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Syncbusy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Syncbusy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of i2cs
#[doc="SPI Mode Cluster"]
pub mod spi {
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SPI Mode Peripheral"]
   pub struct Spi(pub u32);
impl Spi {
#[doc="Get the *const pointer for the ADDR register."]
  #[inline] pub fn addr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the ADDR register."]
  #[inline] pub fn addr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the ADDR register."]
  #[inline] pub fn addr(&self) -> Addr { 
     unsafe {
        Addr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the ADDR register."]
  #[inline] pub fn set_addr(&self, value: Addr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ADDR register."]
  #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
     let tmp = self.addr();
     self.set_addr(f(tmp))
  }

#[doc="Get the *const pointer for the BAUD register."]
  #[inline] pub fn baud_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
#[doc="Get the *mut pointer for the BAUD register."]
  #[inline] pub fn baud_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
#[doc="Read the BAUD register."]
  #[inline] pub fn baud(&self) -> Baud { 
     unsafe {
        Baud(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
#[doc="Write the BAUD register."]
  #[inline] pub fn set_baud(&self, value: Baud) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the BAUD register."]
  #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
     let tmp = self.baud();
     self.set_baud(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLA register."]
  #[inline] pub fn ctrla_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CTRLA register."]
  #[inline] pub fn ctrla_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CTRLA register."]
  #[inline] pub fn ctrla(&self) -> Ctrla { 
     unsafe {
        Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CTRLA register."]
  #[inline] pub fn set_ctrla(&self, value: Ctrla) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTRLA register."]
  #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLB register."]
  #[inline] pub fn ctrlb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CTRLB register."]
  #[inline] pub fn ctrlb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CTRLB register."]
  #[inline] pub fn ctrlb(&self) -> Ctrlb { 
     unsafe {
        Ctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CTRLB register."]
  #[inline] pub fn set_ctrlb(&self, value: Ctrlb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTRLB register."]
  #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
     let tmp = self.ctrlb();
     self.set_ctrlb(f(tmp))
  }

#[doc="Get the *const pointer for the DATA register."]
  #[inline] pub fn data_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the DATA register."]
  #[inline] pub fn data_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the DATA register."]
  #[inline] pub fn data(&self) -> Data { 
     unsafe {
        Data(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
#[doc="Write the DATA register."]
  #[inline] pub fn set_data(&self, value: Data) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DATA register."]
  #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
     let tmp = self.data();
     self.set_data(f(tmp))
  }

#[doc="Get the *const pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x30) as *const u8
  }
#[doc="Get the *mut pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x30) as *mut u8
  }
#[doc="Read the DBGCTRL register."]
  #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
        Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u8))
     }
  }
#[doc="Write the DBGCTRL register."]
  #[inline] pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DBGCTRL register."]
  #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

#[doc="Get the *const pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x14) as *const u8
  }
#[doc="Get the *mut pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x14) as *mut u8
  }
#[doc="Read the INTENCLR register."]
  #[inline] pub fn intenclr(&self) -> Intenclr { 
     unsafe {
        Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u8))
     }
  }
#[doc="Write the INTENCLR register."]
  #[inline] pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENCLR register."]
  #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

#[doc="Get the *const pointer for the INTENSET register."]
  #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x16) as *const u8
  }
#[doc="Get the *mut pointer for the INTENSET register."]
  #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x16) as *mut u8
  }
#[doc="Read the INTENSET register."]
  #[inline] pub fn intenset(&self) -> Intenset { 
     unsafe {
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x16) as *const u8))
     }
  }
#[doc="Write the INTENSET register."]
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x16) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENSET register."]
  #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

#[doc="Get the *const pointer for the INTFLAG register."]
  #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x18) as *const u8
  }
#[doc="Get the *mut pointer for the INTFLAG register."]
  #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x18) as *mut u8
  }
#[doc="Read the INTFLAG register."]
  #[inline] pub fn intflag(&self) -> Intflag { 
     unsafe {
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u8))
     }
  }
#[doc="Write the INTFLAG register."]
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTFLAG register."]
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

#[doc="Get the *const pointer for the STATUS register."]
  #[inline] pub fn status_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x1a) as *const u16
  }
#[doc="Get the *mut pointer for the STATUS register."]
  #[inline] pub fn status_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x1a) as *mut u16
  }
#[doc="Read the STATUS register."]
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x1a) as *const u16))
     }
  }
#[doc="Write the STATUS register."]
  #[inline] pub fn set_status(&self, value: Status) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the STATUS register."]
  #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

#[doc="Get the *const pointer for the SYNCBUSY register."]
  #[inline] pub fn syncbusy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the SYNCBUSY register."]
  #[inline] pub fn syncbusy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the SYNCBUSY register."]
  #[inline] pub fn syncbusy(&self) -> Syncbusy { 
     unsafe {
        Syncbusy(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }

}

#[doc="SPI Address"]
#[derive(PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
#[doc="Address Value"]
  #[inline] pub fn addr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Address Value"]
  #[inline] pub fn set_addr(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Address Mask"]
  #[inline] pub fn addrmask(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
#[doc="Address Mask"]
  #[inline] pub fn set_addrmask(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Addr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Addr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self.addrmask() != 0 { try!(write!(f, " addrmask=0x{:x}", self.addrmask()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SPI Baud Rate"]
#[derive(PartialEq, Eq)]
pub struct Baud(pub u8);
impl Baud {
#[doc="Baud Rate Value"]
  #[inline] pub fn baud(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
#[doc="Baud Rate Value"]
  #[inline] pub fn set_baud(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Baud {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Baud {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.baud() != 0 { try!(write!(f, " baud=0x{:x}", self.baud()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SPI Control A"]
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
#[doc="Software Reset"]
  #[inline] pub fn swrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset"]
  #[inline] pub fn set_swrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Enable"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Enable"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Operating Mode"]
  #[inline] pub fn mode(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x7 // [4:2]
  }
#[doc="Operating Mode"]
  #[inline] pub fn set_mode(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Run during Standby"]
  #[inline] pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Run during Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Immediate Buffer Overflow Notification"]
  #[inline] pub fn ibon(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Immediate Buffer Overflow Notification"]
  #[inline] pub fn set_ibon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Data Out Pinout"]
  #[inline] pub fn dopo(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
#[doc="Data Out Pinout"]
  #[inline] pub fn set_dopo(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Data In Pinout"]
  #[inline] pub fn dipo(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="Data In Pinout"]
  #[inline] pub fn set_dipo(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Frame Format"]
  #[inline] pub fn form(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
#[doc="Frame Format"]
  #[inline] pub fn set_form(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Phase"]
  #[inline] pub fn cpha(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Clock Phase"]
  #[inline] pub fn set_cpha(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Clock Polarity"]
  #[inline] pub fn cpol(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="Clock Polarity"]
  #[inline] pub fn set_cpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Data Order"]
  #[inline] pub fn dord(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Data Order"]
  #[inline] pub fn set_dord(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

}
impl ::core::fmt::Display for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ibon() != 0 { try!(write!(f, " ibon"))}
      if self.dopo() != 0 { try!(write!(f, " dopo=0x{:x}", self.dopo()))}
      if self.dipo() != 0 { try!(write!(f, " dipo=0x{:x}", self.dipo()))}
      if self.form() != 0 { try!(write!(f, " form=0x{:x}", self.form()))}
      if self.cpha() != 0 { try!(write!(f, " cpha"))}
      if self.cpol() != 0 { try!(write!(f, " cpol"))}
      if self.dord() != 0 { try!(write!(f, " dord"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SPI Control B"]
#[derive(PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
#[doc="Character Size"]
  #[inline] pub fn chsize(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
#[doc="Character Size"]
  #[inline] pub fn set_chsize(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Data Preload Enable"]
  #[inline] pub fn ploaden(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Data Preload Enable"]
  #[inline] pub fn set_ploaden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Slave Select Low Detect Enable"]
  #[inline] pub fn ssde(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Slave Select Low Detect Enable"]
  #[inline] pub fn set_ssde(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Master Slave Select Enable"]
  #[inline] pub fn mssen(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Master Slave Select Enable"]
  #[inline] pub fn set_mssen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Address Mode"]
  #[inline] pub fn amode(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
#[doc="Address Mode"]
  #[inline] pub fn set_amode(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Receiver Enable"]
  #[inline] pub fn rxen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="Receiver Enable"]
  #[inline] pub fn set_rxen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

}
impl ::core::fmt::Display for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chsize() != 0 { try!(write!(f, " chsize=0x{:x}", self.chsize()))}
      if self.ploaden() != 0 { try!(write!(f, " ploaden"))}
      if self.ssde() != 0 { try!(write!(f, " ssde"))}
      if self.mssen() != 0 { try!(write!(f, " mssen"))}
      if self.amode() != 0 { try!(write!(f, " amode=0x{:x}", self.amode()))}
      if self.rxen() != 0 { try!(write!(f, " rxen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SPI Data"]
#[derive(PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
#[doc="Data Value"]
  #[inline] pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1ff // [8:0]
  }
#[doc="Data Value"]
  #[inline] pub fn set_data(mut self, value: u32) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SPI Debug Control"]
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Mode"]
  #[inline] pub fn dbgstop(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Debug Mode"]
  #[inline] pub fn set_dbgstop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgstop() != 0 { try!(write!(f, " dbgstop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SPI Interrupt Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Data Register Empty Interrupt Disable"]
  #[inline] pub fn dre(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Data Register Empty Interrupt Disable"]
  #[inline] pub fn set_dre(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transmit Complete Interrupt Disable"]
  #[inline] pub fn txc(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Transmit Complete Interrupt Disable"]
  #[inline] pub fn set_txc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Receive Complete Interrupt Disable"]
  #[inline] pub fn rxc(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Receive Complete Interrupt Disable"]
  #[inline] pub fn set_rxc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Slave Select Low Interrupt Disable"]
  #[inline] pub fn ssl(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Slave Select Low Interrupt Disable"]
  #[inline] pub fn set_ssl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Combined Error Interrupt Disable"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt Disable"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dre() != 0 { try!(write!(f, " dre"))}
      if self.txc() != 0 { try!(write!(f, " txc"))}
      if self.rxc() != 0 { try!(write!(f, " rxc"))}
      if self.ssl() != 0 { try!(write!(f, " ssl"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SPI Interrupt Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Data Register Empty Interrupt Enable"]
  #[inline] pub fn dre(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Data Register Empty Interrupt Enable"]
  #[inline] pub fn set_dre(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transmit Complete Interrupt Enable"]
  #[inline] pub fn txc(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Transmit Complete Interrupt Enable"]
  #[inline] pub fn set_txc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Receive Complete Interrupt Enable"]
  #[inline] pub fn rxc(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Receive Complete Interrupt Enable"]
  #[inline] pub fn set_rxc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Slave Select Low Interrupt Enable"]
  #[inline] pub fn ssl(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Slave Select Low Interrupt Enable"]
  #[inline] pub fn set_ssl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Combined Error Interrupt Enable"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt Enable"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dre() != 0 { try!(write!(f, " dre"))}
      if self.txc() != 0 { try!(write!(f, " txc"))}
      if self.rxc() != 0 { try!(write!(f, " rxc"))}
      if self.ssl() != 0 { try!(write!(f, " ssl"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SPI Interrupt Flag Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Data Register Empty Interrupt"]
  #[inline] pub fn dre(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Data Register Empty Interrupt"]
  #[inline] pub fn set_dre(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transmit Complete Interrupt"]
  #[inline] pub fn txc(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Transmit Complete Interrupt"]
  #[inline] pub fn set_txc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Receive Complete Interrupt"]
  #[inline] pub fn rxc(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Receive Complete Interrupt"]
  #[inline] pub fn set_rxc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Slave Select Low Interrupt Flag"]
  #[inline] pub fn ssl(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Slave Select Low Interrupt Flag"]
  #[inline] pub fn set_ssl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Combined Error Interrupt"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dre() != 0 { try!(write!(f, " dre"))}
      if self.txc() != 0 { try!(write!(f, " txc"))}
      if self.rxc() != 0 { try!(write!(f, " rxc"))}
      if self.ssl() != 0 { try!(write!(f, " ssl"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SPI Status"]
#[derive(PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
#[doc="Buffer Overflow"]
  #[inline] pub fn bufovf(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
#[doc="Buffer Overflow"]
  #[inline] pub fn set_bufovf(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bufovf() != 0 { try!(write!(f, " bufovf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SPI Syncbusy"]
#[derive(PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
#[doc="Software Reset Synchronization Busy"]
  #[inline] pub fn swrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset Synchronization Busy"]
  #[inline] pub fn set_swrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SERCOM Enable Synchronization Busy"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="SERCOM Enable Synchronization Busy"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="CTRLB Synchronization Busy"]
  #[inline] pub fn ctrlb(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="CTRLB Synchronization Busy"]
  #[inline] pub fn set_ctrlb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Syncbusy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Syncbusy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.ctrlb() != 0 { try!(write!(f, " ctrlb"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of spi
#[doc="USART Mode Cluster"]
pub mod usart {
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USART Mode Peripheral"]
   pub struct Usart(pub u32);
impl Usart {
#[doc="Get the *const pointer for the BAUD register."]
  #[inline] pub fn baud_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xc) as *const u16
  }
#[doc="Get the *mut pointer for the BAUD register."]
  #[inline] pub fn baud_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xc) as *mut u16
  }
#[doc="Read the BAUD register."]
  #[inline] pub fn baud(&self) -> Baud { 
     unsafe {
        Baud(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u16))
     }
  }
#[doc="Write the BAUD register."]
  #[inline] pub fn set_baud(&self, value: Baud) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the BAUD register."]
  #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
     let tmp = self.baud();
     self.set_baud(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLA register."]
  #[inline] pub fn ctrla_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CTRLA register."]
  #[inline] pub fn ctrla_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CTRLA register."]
  #[inline] pub fn ctrla(&self) -> Ctrla { 
     unsafe {
        Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CTRLA register."]
  #[inline] pub fn set_ctrla(&self, value: Ctrla) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTRLA register."]
  #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
  }

#[doc="Get the *const pointer for the CTRLB register."]
  #[inline] pub fn ctrlb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CTRLB register."]
  #[inline] pub fn ctrlb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CTRLB register."]
  #[inline] pub fn ctrlb(&self) -> Ctrlb { 
     unsafe {
        Ctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CTRLB register."]
  #[inline] pub fn set_ctrlb(&self, value: Ctrlb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTRLB register."]
  #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
     let tmp = self.ctrlb();
     self.set_ctrlb(f(tmp))
  }

#[doc="Get the *const pointer for the DATA register."]
  #[inline] pub fn data_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x28) as *const u16
  }
#[doc="Get the *mut pointer for the DATA register."]
  #[inline] pub fn data_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x28) as *mut u16
  }
#[doc="Read the DATA register."]
  #[inline] pub fn data(&self) -> Data { 
     unsafe {
        Data(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u16))
     }
  }
#[doc="Write the DATA register."]
  #[inline] pub fn set_data(&self, value: Data) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the DATA register."]
  #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
     let tmp = self.data();
     self.set_data(f(tmp))
  }

#[doc="Get the *const pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x30) as *const u8
  }
#[doc="Get the *mut pointer for the DBGCTRL register."]
  #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x30) as *mut u8
  }
#[doc="Read the DBGCTRL register."]
  #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
        Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u8))
     }
  }
#[doc="Write the DBGCTRL register."]
  #[inline] pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DBGCTRL register."]
  #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

#[doc="Get the *const pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x14) as *const u8
  }
#[doc="Get the *mut pointer for the INTENCLR register."]
  #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x14) as *mut u8
  }
#[doc="Read the INTENCLR register."]
  #[inline] pub fn intenclr(&self) -> Intenclr { 
     unsafe {
        Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u8))
     }
  }
#[doc="Write the INTENCLR register."]
  #[inline] pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENCLR register."]
  #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

#[doc="Get the *const pointer for the INTENSET register."]
  #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x16) as *const u8
  }
#[doc="Get the *mut pointer for the INTENSET register."]
  #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x16) as *mut u8
  }
#[doc="Read the INTENSET register."]
  #[inline] pub fn intenset(&self) -> Intenset { 
     unsafe {
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x16) as *const u8))
     }
  }
#[doc="Write the INTENSET register."]
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x16) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENSET register."]
  #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

#[doc="Get the *const pointer for the INTFLAG register."]
  #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x18) as *const u8
  }
#[doc="Get the *mut pointer for the INTFLAG register."]
  #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x18) as *mut u8
  }
#[doc="Read the INTFLAG register."]
  #[inline] pub fn intflag(&self) -> Intflag { 
     unsafe {
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u8))
     }
  }
#[doc="Write the INTFLAG register."]
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTFLAG register."]
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

#[doc="Get the *const pointer for the RXPL register."]
  #[inline] pub fn rxpl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xe) as *const u8
  }
#[doc="Get the *mut pointer for the RXPL register."]
  #[inline] pub fn rxpl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xe) as *mut u8
  }
#[doc="Read the RXPL register."]
  #[inline] pub fn rxpl(&self) -> Rxpl { 
     unsafe {
        Rxpl(::core::ptr::read_volatile(((self.0 as usize) + 0xe) as *const u8))
     }
  }
#[doc="Write the RXPL register."]
  #[inline] pub fn set_rxpl(&self, value: Rxpl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the RXPL register."]
  #[inline] pub fn with_rxpl<F: FnOnce(Rxpl) -> Rxpl>(&self, f: F) -> &Self {
     let tmp = self.rxpl();
     self.set_rxpl(f(tmp))
  }

#[doc="Get the *const pointer for the STATUS register."]
  #[inline] pub fn status_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x1a) as *const u16
  }
#[doc="Get the *mut pointer for the STATUS register."]
  #[inline] pub fn status_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x1a) as *mut u16
  }
#[doc="Read the STATUS register."]
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x1a) as *const u16))
     }
  }
#[doc="Write the STATUS register."]
  #[inline] pub fn set_status(&self, value: Status) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the STATUS register."]
  #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

#[doc="Get the *const pointer for the SYNCBUSY register."]
  #[inline] pub fn syncbusy_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the SYNCBUSY register."]
  #[inline] pub fn syncbusy_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the SYNCBUSY register."]
  #[inline] pub fn syncbusy(&self) -> Syncbusy { 
     unsafe {
        Syncbusy(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }

}

#[doc="USART Baud Rate"]
#[derive(PartialEq, Eq)]
pub struct Baud(pub u16);
impl Baud {
#[doc="Baud Rate Value"]
  #[inline] pub fn baud(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1fff // [12:0]
  }
#[doc="Baud Rate Value"]
  #[inline] pub fn set_baud(mut self, value: u16) -> Self {
     assert!((value & !0x1fff) == 0);
     self.0 &= !(0x1fff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Fractional Part"]
  #[inline] pub fn fp(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x7 // [15:13]
  }
#[doc="Fractional Part"]
  #[inline] pub fn set_fp(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Baud {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Baud {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.baud() != 0 { try!(write!(f, " baud=0x{:x}", self.baud()))}
      if self.fp() != 0 { try!(write!(f, " fp=0x{:x}", self.fp()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USART Control A"]
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
#[doc="Software Reset"]
  #[inline] pub fn swrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset"]
  #[inline] pub fn set_swrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Enable"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Enable"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Operating Mode"]
  #[inline] pub fn mode(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x7 // [4:2]
  }
#[doc="Operating Mode"]
  #[inline] pub fn set_mode(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Run during Standby"]
  #[inline] pub fn runstdby(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Run during Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Immediate Buffer Overflow Notification"]
  #[inline] pub fn ibon(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Immediate Buffer Overflow Notification"]
  #[inline] pub fn set_ibon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Sample"]
  #[inline] pub fn sampr(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
#[doc="Sample"]
  #[inline] pub fn set_sampr(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Transmit Data Pinout"]
  #[inline] pub fn txpo(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
#[doc="Transmit Data Pinout"]
  #[inline] pub fn set_txpo(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Receive Data Pinout"]
  #[inline] pub fn rxpo(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="Receive Data Pinout"]
  #[inline] pub fn set_rxpo(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Sample Adjustment"]
  #[inline] pub fn sampa(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x3 // [23:22]
  }
#[doc="Sample Adjustment"]
  #[inline] pub fn set_sampa(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Frame Format"]
  #[inline] pub fn form(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
#[doc="Frame Format"]
  #[inline] pub fn set_form(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Communication Mode"]
  #[inline] pub fn cmode(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Communication Mode"]
  #[inline] pub fn set_cmode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Clock Polarity"]
  #[inline] pub fn cpol(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
#[doc="Clock Polarity"]
  #[inline] pub fn set_cpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

#[doc="Data Order"]
  #[inline] pub fn dord(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Data Order"]
  #[inline] pub fn set_dord(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

}
impl ::core::fmt::Display for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
      if self.ibon() != 0 { try!(write!(f, " ibon"))}
      if self.sampr() != 0 { try!(write!(f, " sampr=0x{:x}", self.sampr()))}
      if self.txpo() != 0 { try!(write!(f, " txpo=0x{:x}", self.txpo()))}
      if self.rxpo() != 0 { try!(write!(f, " rxpo=0x{:x}", self.rxpo()))}
      if self.sampa() != 0 { try!(write!(f, " sampa=0x{:x}", self.sampa()))}
      if self.form() != 0 { try!(write!(f, " form=0x{:x}", self.form()))}
      if self.cmode() != 0 { try!(write!(f, " cmode"))}
      if self.cpol() != 0 { try!(write!(f, " cpol"))}
      if self.dord() != 0 { try!(write!(f, " dord"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USART Control B"]
#[derive(PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
#[doc="Character Size"]
  #[inline] pub fn chsize(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
#[doc="Character Size"]
  #[inline] pub fn set_chsize(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Stop Bit Mode"]
  #[inline] pub fn sbmode(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Stop Bit Mode"]
  #[inline] pub fn set_sbmode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Collision Detection Enable"]
  #[inline] pub fn colden(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Collision Detection Enable"]
  #[inline] pub fn set_colden(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Start of Frame Detection Enable"]
  #[inline] pub fn sfde(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Start of Frame Detection Enable"]
  #[inline] pub fn set_sfde(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Encoding Format"]
  #[inline] pub fn enc(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Encoding Format"]
  #[inline] pub fn set_enc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Parity Mode"]
  #[inline] pub fn pmode(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Parity Mode"]
  #[inline] pub fn set_pmode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Transmitter Enable"]
  #[inline] pub fn txen(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Transmitter Enable"]
  #[inline] pub fn set_txen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Receiver Enable"]
  #[inline] pub fn rxen(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="Receiver Enable"]
  #[inline] pub fn set_rxen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

}
impl ::core::fmt::Display for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chsize() != 0 { try!(write!(f, " chsize=0x{:x}", self.chsize()))}
      if self.sbmode() != 0 { try!(write!(f, " sbmode"))}
      if self.colden() != 0 { try!(write!(f, " colden"))}
      if self.sfde() != 0 { try!(write!(f, " sfde"))}
      if self.enc() != 0 { try!(write!(f, " enc"))}
      if self.pmode() != 0 { try!(write!(f, " pmode"))}
      if self.txen() != 0 { try!(write!(f, " txen"))}
      if self.rxen() != 0 { try!(write!(f, " rxen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USART Data"]
#[derive(PartialEq, Eq)]
pub struct Data(pub u16);
impl Data {
#[doc="Data Value"]
  #[inline] pub fn data(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1ff // [8:0]
  }
#[doc="Data Value"]
  #[inline] pub fn set_data(mut self, value: u16) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USART Debug Control"]
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Mode"]
  #[inline] pub fn dbgstop(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Debug Mode"]
  #[inline] pub fn set_dbgstop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgstop() != 0 { try!(write!(f, " dbgstop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USART Interrupt Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Data Register Empty Interrupt Disable"]
  #[inline] pub fn dre(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Data Register Empty Interrupt Disable"]
  #[inline] pub fn set_dre(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transmit Complete Interrupt Disable"]
  #[inline] pub fn txc(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Transmit Complete Interrupt Disable"]
  #[inline] pub fn set_txc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Receive Complete Interrupt Disable"]
  #[inline] pub fn rxc(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Receive Complete Interrupt Disable"]
  #[inline] pub fn set_rxc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Receive Start Interrupt Disable"]
  #[inline] pub fn rxs(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Receive Start Interrupt Disable"]
  #[inline] pub fn set_rxs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Clear To Send Input Change Interrupt Disable"]
  #[inline] pub fn ctsic(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Clear To Send Input Change Interrupt Disable"]
  #[inline] pub fn set_ctsic(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Break Received Interrupt Disable"]
  #[inline] pub fn rxbrk(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Break Received Interrupt Disable"]
  #[inline] pub fn set_rxbrk(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Combined Error Interrupt Disable"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt Disable"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dre() != 0 { try!(write!(f, " dre"))}
      if self.txc() != 0 { try!(write!(f, " txc"))}
      if self.rxc() != 0 { try!(write!(f, " rxc"))}
      if self.rxs() != 0 { try!(write!(f, " rxs"))}
      if self.ctsic() != 0 { try!(write!(f, " ctsic"))}
      if self.rxbrk() != 0 { try!(write!(f, " rxbrk"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USART Interrupt Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Data Register Empty Interrupt Enable"]
  #[inline] pub fn dre(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Data Register Empty Interrupt Enable"]
  #[inline] pub fn set_dre(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transmit Complete Interrupt Enable"]
  #[inline] pub fn txc(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Transmit Complete Interrupt Enable"]
  #[inline] pub fn set_txc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Receive Complete Interrupt Enable"]
  #[inline] pub fn rxc(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Receive Complete Interrupt Enable"]
  #[inline] pub fn set_rxc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Receive Start Interrupt Enable"]
  #[inline] pub fn rxs(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Receive Start Interrupt Enable"]
  #[inline] pub fn set_rxs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Clear To Send Input Change Interrupt Enable"]
  #[inline] pub fn ctsic(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Clear To Send Input Change Interrupt Enable"]
  #[inline] pub fn set_ctsic(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Break Received Interrupt Enable"]
  #[inline] pub fn rxbrk(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Break Received Interrupt Enable"]
  #[inline] pub fn set_rxbrk(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Combined Error Interrupt Enable"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt Enable"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dre() != 0 { try!(write!(f, " dre"))}
      if self.txc() != 0 { try!(write!(f, " txc"))}
      if self.rxc() != 0 { try!(write!(f, " rxc"))}
      if self.rxs() != 0 { try!(write!(f, " rxs"))}
      if self.ctsic() != 0 { try!(write!(f, " ctsic"))}
      if self.rxbrk() != 0 { try!(write!(f, " rxbrk"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USART Interrupt Flag Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Data Register Empty Interrupt"]
  #[inline] pub fn dre(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Data Register Empty Interrupt"]
  #[inline] pub fn set_dre(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transmit Complete Interrupt"]
  #[inline] pub fn txc(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Transmit Complete Interrupt"]
  #[inline] pub fn set_txc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Receive Complete Interrupt"]
  #[inline] pub fn rxc(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Receive Complete Interrupt"]
  #[inline] pub fn set_rxc(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Receive Start Interrupt"]
  #[inline] pub fn rxs(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Receive Start Interrupt"]
  #[inline] pub fn set_rxs(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Clear To Send Input Change Interrupt"]
  #[inline] pub fn ctsic(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Clear To Send Input Change Interrupt"]
  #[inline] pub fn set_ctsic(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Break Received Interrupt"]
  #[inline] pub fn rxbrk(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Break Received Interrupt"]
  #[inline] pub fn set_rxbrk(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Combined Error Interrupt"]
  #[inline] pub fn error(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Combined Error Interrupt"]
  #[inline] pub fn set_error(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dre() != 0 { try!(write!(f, " dre"))}
      if self.txc() != 0 { try!(write!(f, " txc"))}
      if self.rxc() != 0 { try!(write!(f, " rxc"))}
      if self.rxs() != 0 { try!(write!(f, " rxs"))}
      if self.ctsic() != 0 { try!(write!(f, " ctsic"))}
      if self.rxbrk() != 0 { try!(write!(f, " rxbrk"))}
      if self.error() != 0 { try!(write!(f, " error"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USART Receive Pulse Length"]
#[derive(PartialEq, Eq)]
pub struct Rxpl(pub u8);
impl Rxpl {
#[doc="Receive Pulse Length"]
  #[inline] pub fn rxpl(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
#[doc="Receive Pulse Length"]
  #[inline] pub fn set_rxpl(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Rxpl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rxpl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxpl() != 0 { try!(write!(f, " rxpl=0x{:x}", self.rxpl()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USART Status"]
#[derive(PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
#[doc="Parity Error"]
  #[inline] pub fn perr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
#[doc="Parity Error"]
  #[inline] pub fn set_perr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Frame Error"]
  #[inline] pub fn ferr(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="Frame Error"]
  #[inline] pub fn set_ferr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Buffer Overflow"]
  #[inline] pub fn bufovf(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
#[doc="Buffer Overflow"]
  #[inline] pub fn set_bufovf(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Clear To Send"]
  #[inline] pub fn cts(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
#[doc="Clear To Send"]
  #[inline] pub fn set_cts(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Inconsistent Sync Field"]
  #[inline] pub fn isf(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
#[doc="Inconsistent Sync Field"]
  #[inline] pub fn set_isf(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Collision Detected"]
  #[inline] pub fn coll(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
#[doc="Collision Detected"]
  #[inline] pub fn set_coll(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.perr() != 0 { try!(write!(f, " perr"))}
      if self.ferr() != 0 { try!(write!(f, " ferr"))}
      if self.bufovf() != 0 { try!(write!(f, " bufovf"))}
      if self.cts() != 0 { try!(write!(f, " cts"))}
      if self.isf() != 0 { try!(write!(f, " isf"))}
      if self.coll() != 0 { try!(write!(f, " coll"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USART Syncbusy"]
#[derive(PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
#[doc="Software Reset Synchronization Busy"]
  #[inline] pub fn swrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset Synchronization Busy"]
  #[inline] pub fn set_swrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SERCOM Enable Synchronization Busy"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="SERCOM Enable Synchronization Busy"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="CTRLB Synchronization Busy"]
  #[inline] pub fn ctrlb(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="CTRLB Synchronization Busy"]
  #[inline] pub fn set_ctrlb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Syncbusy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Syncbusy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.ctrlb() != 0 { try!(write!(f, " ctrlb"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
}
// End of usart
