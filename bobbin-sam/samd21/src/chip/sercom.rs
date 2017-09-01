//! Serial Communication Interface
#[allow(unused_imports)] use bobbin_common::*;

periph!( SERCOM0, Sercom0, _SERCOM0, SercomPeriph, 0x42000800);
periph!( SERCOM1, Sercom1, _SERCOM1, SercomPeriph, 0x42000c00);
periph!( SERCOM2, Sercom2, _SERCOM2, SercomPeriph, 0x42001000);
periph!( SERCOM3, Sercom3, _SERCOM3, SercomPeriph, 0x42001400);
periph!( SERCOM4, Sercom4, _SERCOM4, SercomPeriph, 0x42001800);
periph!( SERCOM5, Sercom5, _SERCOM5, SercomPeriph, 0x42001c00);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SERCOM Peripheral"]
pub struct SercomPeriph(pub usize); 

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


impl SercomPeriph {
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
#[allow(unused_imports)] use bobbin_common::*;
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C Master Mode Peripheral"]
   pub struct I2cm(pub usize);
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
         Addr(read_volatile((self.0 + 0x24) as *const u32))
      }
   }

   #[doc="Write the ADDR register."]
   #[inline] pub fn set_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
      let value = f(Addr(0));
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the ADDR register."]
   #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
      let tmp = self.addr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
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
         Baud(read_volatile((self.0 + 0xc) as *const u32))
      }
   }

   #[doc="Write the BAUD register."]
   #[inline] pub fn set_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
      let value = f(Baud(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the BAUD register."]
   #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
      let tmp = self.baud();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
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
         Ctrla(read_volatile((self.0 + 0x0) as *const u32))
      }
   }

   #[doc="Write the CTRLA register."]
   #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let value = f(Ctrla(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CTRLA register."]
   #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let tmp = self.ctrla();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
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
         Ctrlb(read_volatile((self.0 + 0x4) as *const u32))
      }
   }

   #[doc="Write the CTRLB register."]
   #[inline] pub fn set_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
      let value = f(Ctrlb(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CTRLB register."]
   #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
      let tmp = self.ctrlb();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
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
         Data(read_volatile((self.0 + 0x28) as *const u8))
      }
   }

   #[doc="Write the DATA register."]
   #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let value = f(Data(0));
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the DATA register."]
   #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let tmp = self.data();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u8, value.0);
      }
      self
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
         Dbgctrl(read_volatile((self.0 + 0x30) as *const u8))
      }
   }

   #[doc="Write the DBGCTRL register."]
   #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the DBGCTRL register."]
   #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = self.dbgctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u8, value.0);
      }
      self
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
         Intenclr(read_volatile((self.0 + 0x14) as *const u8))
      }
   }

   #[doc="Write the INTENCLR register."]
   #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTENCLR register."]
   #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = self.intenclr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u8, value.0);
      }
      self
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
         Intenset(read_volatile((self.0 + 0x16) as *const u8))
      }
   }

   #[doc="Write the INTENSET register."]
   #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         write_volatile((self.0 + 0x16) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTENSET register."]
   #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = self.intenset();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x16) as *mut u8, value.0);
      }
      self
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
         Intflag(read_volatile((self.0 + 0x18) as *const u8))
      }
   }

   #[doc="Write the INTFLAG register."]
   #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTFLAG register."]
   #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = self.intflag();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u8, value.0);
      }
      self
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
         Status(read_volatile((self.0 + 0x1a) as *const u16))
      }
   }

   #[doc="Write the STATUS register."]
   #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let value = f(Status(0));
      unsafe {
         write_volatile((self.0 + 0x1a) as *mut u16, value.0);
      }
      self
   }

   #[doc="Modify the STATUS register."]
   #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let tmp = self.status();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1a) as *mut u16, value.0);
      }
      self
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
         Syncbusy(read_volatile((self.0 + 0x1c) as *const u32))
      }
   }

}

#[doc="I2CM Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
   #[doc="Address Value"]
   #[inline] pub fn addr(&self) -> bits::U11 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
   }

   #[doc="Address Value"]
   #[inline] pub fn test_addr(&self) -> bool {
      self.addr != 0
   }

   #[doc="Address Value"]
   #[inline] pub fn set_addr<V: Into<bits::U11>>(mut self, value: V) -> Self {
      let value: bits::U11 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7ff << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Length Enable"]
   #[inline] pub fn lenen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }

   #[doc="Length Enable"]
   #[inline] pub fn test_lenen(&self) -> bool {
      self.lenen != 0
   }

   #[doc="Length Enable"]
   #[inline] pub fn set_lenen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

   #[doc="High Speed Mode"]
   #[inline] pub fn hs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }

   #[doc="High Speed Mode"]
   #[inline] pub fn test_hs(&self) -> bool {
      self.hs != 0
   }

   #[doc="High Speed Mode"]
   #[inline] pub fn set_hs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

   #[doc="Ten Bit Addressing Enable"]
   #[inline] pub fn tenbiten(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }

   #[doc="Ten Bit Addressing Enable"]
   #[inline] pub fn test_tenbiten(&self) -> bool {
      self.tenbiten != 0
   }

   #[doc="Ten Bit Addressing Enable"]
   #[inline] pub fn set_tenbiten<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

   #[doc="Length"]
   #[inline] pub fn len(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }

   #[doc="Length"]
   #[inline] pub fn test_len(&self) -> bool {
      self.len != 0
   }

   #[doc="Length"]
   #[inline] pub fn set_len<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Baud(pub u32);
impl Baud {
   #[doc="Baud Rate Value"]
   #[inline] pub fn baud(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }

   #[doc="Baud Rate Value"]
   #[inline] pub fn test_baud(&self) -> bool {
      self.baud != 0
   }

   #[doc="Baud Rate Value"]
   #[inline] pub fn set_baud<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Baud Rate Value Low"]
   #[inline] pub fn baudlow(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }

   #[doc="Baud Rate Value Low"]
   #[inline] pub fn test_baudlow(&self) -> bool {
      self.baudlow != 0
   }

   #[doc="Baud Rate Value Low"]
   #[inline] pub fn set_baudlow<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="High Speed Baud Rate Value"]
   #[inline] pub fn hsbaud(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }

   #[doc="High Speed Baud Rate Value"]
   #[inline] pub fn test_hsbaud(&self) -> bool {
      self.hsbaud != 0
   }

   #[doc="High Speed Baud Rate Value"]
   #[inline] pub fn set_hsbaud<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="High Speed Baud Rate Value Low"]
   #[inline] pub fn hsbaudlow(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }

   #[doc="High Speed Baud Rate Value Low"]
   #[inline] pub fn test_hsbaudlow(&self) -> bool {
      self.hsbaudlow != 0
   }

   #[doc="High Speed Baud Rate Value Low"]
   #[inline] pub fn set_hsbaudlow<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
   #[doc="Software Reset"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Software Reset"]
   #[inline] pub fn test_swrst(&self) -> bool {
      self.swrst != 0
   }

   #[doc="Software Reset"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Enable"]
   #[inline] pub fn test_enable(&self) -> bool {
      self.enable != 0
   }

   #[doc="Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Operating Mode"]
   #[inline] pub fn mode(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
   }

   #[doc="Operating Mode"]
   #[inline] pub fn test_mode(&self) -> bool {
      self.mode != 0
   }

   #[doc="Operating Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Run in Standby"]
   #[inline] pub fn runstdby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Run in Standby"]
   #[inline] pub fn test_runstdby(&self) -> bool {
      self.runstdby != 0
   }

   #[doc="Run in Standby"]
   #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

   #[doc="Pin Usage"]
   #[inline] pub fn pinout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }

   #[doc="Pin Usage"]
   #[inline] pub fn test_pinout(&self) -> bool {
      self.pinout != 0
   }

   #[doc="Pin Usage"]
   #[inline] pub fn set_pinout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="SDA Hold Time"]
   #[inline] pub fn sdahold(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
   }

   #[doc="SDA Hold Time"]
   #[inline] pub fn test_sdahold(&self) -> bool {
      self.sdahold != 0
   }

   #[doc="SDA Hold Time"]
   #[inline] pub fn set_sdahold<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 20);
      self.0 |= value << 20;
      self
   }

   #[doc="Master SCL Low Extend Timeout"]
   #[inline] pub fn mexttoen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }

   #[doc="Master SCL Low Extend Timeout"]
   #[inline] pub fn test_mexttoen(&self) -> bool {
      self.mexttoen != 0
   }

   #[doc="Master SCL Low Extend Timeout"]
   #[inline] pub fn set_mexttoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn sexttoen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn test_sexttoen(&self) -> bool {
      self.sexttoen != 0
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn set_sexttoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

   #[doc="Transfer Speed"]
   #[inline] pub fn speed(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
   }

   #[doc="Transfer Speed"]
   #[inline] pub fn test_speed(&self) -> bool {
      self.speed != 0
   }

   #[doc="Transfer Speed"]
   #[inline] pub fn set_speed<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 24);
      self.0 |= value << 24;
      self
   }

   #[doc="SCL Clock Stretch Mode"]
   #[inline] pub fn sclsm(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
   }

   #[doc="SCL Clock Stretch Mode"]
   #[inline] pub fn test_sclsm(&self) -> bool {
      self.sclsm != 0
   }

   #[doc="SCL Clock Stretch Mode"]
   #[inline] pub fn set_sclsm<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 27);
      self.0 |= value << 27;
      self
   }

   #[doc="Inactive Time-Out"]
   #[inline] pub fn inactout(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
   }

   #[doc="Inactive Time-Out"]
   #[inline] pub fn test_inactout(&self) -> bool {
      self.inactout != 0
   }

   #[doc="Inactive Time-Out"]
   #[inline] pub fn set_inactout<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 28);
      self.0 |= value << 28;
      self
   }

   #[doc="SCL Low Timeout Enable"]
   #[inline] pub fn lowtouten(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }

   #[doc="SCL Low Timeout Enable"]
   #[inline] pub fn test_lowtouten(&self) -> bool {
      self.lowtouten != 0
   }

   #[doc="SCL Low Timeout Enable"]
   #[inline] pub fn set_lowtouten<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
   #[doc="Smart Mode Enable"]
   #[inline] pub fn smen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }

   #[doc="Smart Mode Enable"]
   #[inline] pub fn test_smen(&self) -> bool {
      self.smen != 0
   }

   #[doc="Smart Mode Enable"]
   #[inline] pub fn set_smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="Quick Command Enable"]
   #[inline] pub fn qcen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }

   #[doc="Quick Command Enable"]
   #[inline] pub fn test_qcen(&self) -> bool {
      self.qcen != 0
   }

   #[doc="Quick Command Enable"]
   #[inline] pub fn set_qcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

   #[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
   }

   #[doc="Command"]
   #[inline] pub fn test_cmd(&self) -> bool {
      self.cmd != 0
   }

   #[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="Acknowledge Action"]
   #[inline] pub fn ackact(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }

   #[doc="Acknowledge Action"]
   #[inline] pub fn test_ackact(&self) -> bool {
      self.ackact != 0
   }

   #[doc="Acknowledge Action"]
   #[inline] pub fn set_ackact<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u8);
impl Data {
   #[doc="Data Value"]
   #[inline] pub fn data(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }

   #[doc="Data Value"]
   #[inline] pub fn test_data(&self) -> bool {
      self.data != 0
   }

   #[doc="Data Value"]
   #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
   #[doc="Debug Mode"]
   #[inline] pub fn dbgstop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Debug Mode"]
   #[inline] pub fn test_dbgstop(&self) -> bool {
      self.dbgstop != 0
   }

   #[doc="Debug Mode"]
   #[inline] pub fn set_dbgstop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
   #[doc="Master On Bus Interrupt Disable"]
   #[inline] pub fn mb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Master On Bus Interrupt Disable"]
   #[inline] pub fn test_mb(&self) -> bool {
      self.mb != 0
   }

   #[doc="Master On Bus Interrupt Disable"]
   #[inline] pub fn set_mb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Slave On Bus Interrupt Disable"]
   #[inline] pub fn sb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Slave On Bus Interrupt Disable"]
   #[inline] pub fn test_sb(&self) -> bool {
      self.sb != 0
   }

   #[doc="Slave On Bus Interrupt Disable"]
   #[inline] pub fn set_sb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
   #[doc="Master On Bus Interrupt Enable"]
   #[inline] pub fn mb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Master On Bus Interrupt Enable"]
   #[inline] pub fn test_mb(&self) -> bool {
      self.mb != 0
   }

   #[doc="Master On Bus Interrupt Enable"]
   #[inline] pub fn set_mb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Slave On Bus Interrupt Enable"]
   #[inline] pub fn sb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Slave On Bus Interrupt Enable"]
   #[inline] pub fn test_sb(&self) -> bool {
      self.sb != 0
   }

   #[doc="Slave On Bus Interrupt Enable"]
   #[inline] pub fn set_sb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
   #[doc="Master On Bus Interrupt"]
   #[inline] pub fn mb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Master On Bus Interrupt"]
   #[inline] pub fn test_mb(&self) -> bool {
      self.mb != 0
   }

   #[doc="Master On Bus Interrupt"]
   #[inline] pub fn set_mb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Slave On Bus Interrupt"]
   #[inline] pub fn sb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Slave On Bus Interrupt"]
   #[inline] pub fn test_sb(&self) -> bool {
      self.sb != 0
   }

   #[doc="Slave On Bus Interrupt"]
   #[inline] pub fn set_sb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
   #[doc="Bus Error"]
   #[inline] pub fn buserr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Bus Error"]
   #[inline] pub fn test_buserr(&self) -> bool {
      self.buserr != 0
   }

   #[doc="Bus Error"]
   #[inline] pub fn set_buserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Arbitration Lost"]
   #[inline] pub fn arblost(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Arbitration Lost"]
   #[inline] pub fn test_arblost(&self) -> bool {
      self.arblost != 0
   }

   #[doc="Arbitration Lost"]
   #[inline] pub fn set_arblost<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Received Not Acknowledge"]
   #[inline] pub fn rxnack(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Received Not Acknowledge"]
   #[inline] pub fn test_rxnack(&self) -> bool {
      self.rxnack != 0
   }

   #[doc="Received Not Acknowledge"]
   #[inline] pub fn set_rxnack<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Bus State"]
   #[inline] pub fn busstate(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
   }

   #[doc="Bus State"]
   #[inline] pub fn test_busstate(&self) -> bool {
      self.busstate != 0
   }

   #[doc="Bus State"]
   #[inline] pub fn set_busstate<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x3 << 4);
      self.0 |= value << 4;
      self
   }

   #[doc="SCL Low Timeout"]
   #[inline] pub fn lowtout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }

   #[doc="SCL Low Timeout"]
   #[inline] pub fn test_lowtout(&self) -> bool {
      self.lowtout != 0
   }

   #[doc="SCL Low Timeout"]
   #[inline] pub fn set_lowtout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="Clock Hold"]
   #[inline] pub fn clkhold(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Clock Hold"]
   #[inline] pub fn test_clkhold(&self) -> bool {
      self.clkhold != 0
   }

   #[doc="Clock Hold"]
   #[inline] pub fn set_clkhold<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

   #[doc="Master SCL Low Extend Timeout"]
   #[inline] pub fn mexttout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }

   #[doc="Master SCL Low Extend Timeout"]
   #[inline] pub fn test_mexttout(&self) -> bool {
      self.mexttout != 0
   }

   #[doc="Master SCL Low Extend Timeout"]
   #[inline] pub fn set_mexttout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn sexttout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn test_sexttout(&self) -> bool {
      self.sexttout != 0
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn set_sexttout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

   #[doc="Length Error"]
   #[inline] pub fn lenerr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }

   #[doc="Length Error"]
   #[inline] pub fn test_lenerr(&self) -> bool {
      self.lenerr != 0
   }

   #[doc="Length Error"]
   #[inline] pub fn set_lenerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn test_swrst(&self) -> bool {
      self.swrst != 0
   }

   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn test_enable(&self) -> bool {
      self.enable != 0
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="System Operation Synchronization Busy"]
   #[inline] pub fn sysop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="System Operation Synchronization Busy"]
   #[inline] pub fn test_sysop(&self) -> bool {
      self.sysop != 0
   }

   #[doc="System Operation Synchronization Busy"]
   #[inline] pub fn set_sysop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[allow(unused_imports)] use bobbin_common::*;
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C Slave Mode Peripheral"]
   pub struct I2cs(pub usize);
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
         Addr(read_volatile((self.0 + 0x24) as *const u32))
      }
   }

   #[doc="Write the ADDR register."]
   #[inline] pub fn set_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
      let value = f(Addr(0));
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the ADDR register."]
   #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
      let tmp = self.addr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
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
         Ctrla(read_volatile((self.0 + 0x0) as *const u32))
      }
   }

   #[doc="Write the CTRLA register."]
   #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let value = f(Ctrla(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CTRLA register."]
   #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let tmp = self.ctrla();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
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
         Ctrlb(read_volatile((self.0 + 0x4) as *const u32))
      }
   }

   #[doc="Write the CTRLB register."]
   #[inline] pub fn set_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
      let value = f(Ctrlb(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CTRLB register."]
   #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
      let tmp = self.ctrlb();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
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
         Data(read_volatile((self.0 + 0x28) as *const u8))
      }
   }

   #[doc="Write the DATA register."]
   #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let value = f(Data(0));
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the DATA register."]
   #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let tmp = self.data();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u8, value.0);
      }
      self
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
         Dbgctrl(read_volatile((self.0 + 0x30) as *const u8))
      }
   }

   #[doc="Write the DBGCTRL register."]
   #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the DBGCTRL register."]
   #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = self.dbgctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u8, value.0);
      }
      self
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
         Intenclr(read_volatile((self.0 + 0x14) as *const u8))
      }
   }

   #[doc="Write the INTENCLR register."]
   #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTENCLR register."]
   #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = self.intenclr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u8, value.0);
      }
      self
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
         Intenset(read_volatile((self.0 + 0x16) as *const u8))
      }
   }

   #[doc="Write the INTENSET register."]
   #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         write_volatile((self.0 + 0x16) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTENSET register."]
   #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = self.intenset();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x16) as *mut u8, value.0);
      }
      self
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
         Intflag(read_volatile((self.0 + 0x18) as *const u8))
      }
   }

   #[doc="Write the INTFLAG register."]
   #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTFLAG register."]
   #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = self.intflag();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u8, value.0);
      }
      self
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
         Status(read_volatile((self.0 + 0x1a) as *const u16))
      }
   }

   #[doc="Write the STATUS register."]
   #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let value = f(Status(0));
      unsafe {
         write_volatile((self.0 + 0x1a) as *mut u16, value.0);
      }
      self
   }

   #[doc="Modify the STATUS register."]
   #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let tmp = self.status();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1a) as *mut u16, value.0);
      }
      self
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
         Syncbusy(read_volatile((self.0 + 0x1c) as *const u32))
      }
   }

}

#[doc="I2CS Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
   #[doc="General Call Address Enable"]
   #[inline] pub fn gencen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="General Call Address Enable"]
   #[inline] pub fn test_gencen(&self) -> bool {
      self.gencen != 0
   }

   #[doc="General Call Address Enable"]
   #[inline] pub fn set_gencen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Address Value"]
   #[inline] pub fn addr(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3ff) as u16) } // [10:1]
   }

   #[doc="Address Value"]
   #[inline] pub fn test_addr(&self) -> bool {
      self.addr != 0
   }

   #[doc="Address Value"]
   #[inline] pub fn set_addr<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Ten Bit Addressing Enable"]
   #[inline] pub fn tenbiten(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }

   #[doc="Ten Bit Addressing Enable"]
   #[inline] pub fn test_tenbiten(&self) -> bool {
      self.tenbiten != 0
   }

   #[doc="Ten Bit Addressing Enable"]
   #[inline] pub fn set_tenbiten<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

   #[doc="Address Mask"]
   #[inline] pub fn addrmask(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3ff) as u16) } // [26:17]
   }

   #[doc="Address Mask"]
   #[inline] pub fn test_addrmask(&self) -> bool {
      self.addrmask != 0
   }

   #[doc="Address Mask"]
   #[inline] pub fn set_addrmask<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
   #[doc="Software Reset"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Software Reset"]
   #[inline] pub fn test_swrst(&self) -> bool {
      self.swrst != 0
   }

   #[doc="Software Reset"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Enable"]
   #[inline] pub fn test_enable(&self) -> bool {
      self.enable != 0
   }

   #[doc="Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Operating Mode"]
   #[inline] pub fn mode(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
   }

   #[doc="Operating Mode"]
   #[inline] pub fn test_mode(&self) -> bool {
      self.mode != 0
   }

   #[doc="Operating Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Run during Standby"]
   #[inline] pub fn runstdby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Run during Standby"]
   #[inline] pub fn test_runstdby(&self) -> bool {
      self.runstdby != 0
   }

   #[doc="Run during Standby"]
   #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

   #[doc="Pin Usage"]
   #[inline] pub fn pinout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }

   #[doc="Pin Usage"]
   #[inline] pub fn test_pinout(&self) -> bool {
      self.pinout != 0
   }

   #[doc="Pin Usage"]
   #[inline] pub fn set_pinout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="SDA Hold Time"]
   #[inline] pub fn sdahold(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
   }

   #[doc="SDA Hold Time"]
   #[inline] pub fn test_sdahold(&self) -> bool {
      self.sdahold != 0
   }

   #[doc="SDA Hold Time"]
   #[inline] pub fn set_sdahold<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 20);
      self.0 |= value << 20;
      self
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn sexttoen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn test_sexttoen(&self) -> bool {
      self.sexttoen != 0
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn set_sexttoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

   #[doc="Transfer Speed"]
   #[inline] pub fn speed(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
   }

   #[doc="Transfer Speed"]
   #[inline] pub fn test_speed(&self) -> bool {
      self.speed != 0
   }

   #[doc="Transfer Speed"]
   #[inline] pub fn set_speed<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 24);
      self.0 |= value << 24;
      self
   }

   #[doc="SCL Clock Stretch Mode"]
   #[inline] pub fn sclsm(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
   }

   #[doc="SCL Clock Stretch Mode"]
   #[inline] pub fn test_sclsm(&self) -> bool {
      self.sclsm != 0
   }

   #[doc="SCL Clock Stretch Mode"]
   #[inline] pub fn set_sclsm<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 27);
      self.0 |= value << 27;
      self
   }

   #[doc="SCL Low Timeout Enable"]
   #[inline] pub fn lowtouten(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }

   #[doc="SCL Low Timeout Enable"]
   #[inline] pub fn test_lowtouten(&self) -> bool {
      self.lowtouten != 0
   }

   #[doc="SCL Low Timeout Enable"]
   #[inline] pub fn set_lowtouten<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
   #[doc="Smart Mode Enable"]
   #[inline] pub fn smen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }

   #[doc="Smart Mode Enable"]
   #[inline] pub fn test_smen(&self) -> bool {
      self.smen != 0
   }

   #[doc="Smart Mode Enable"]
   #[inline] pub fn set_smen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="PMBus Group Command"]
   #[inline] pub fn gcmd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }

   #[doc="PMBus Group Command"]
   #[inline] pub fn test_gcmd(&self) -> bool {
      self.gcmd != 0
   }

   #[doc="PMBus Group Command"]
   #[inline] pub fn set_gcmd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

   #[doc="Automatic Address Acknowledge"]
   #[inline] pub fn aacken(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }

   #[doc="Automatic Address Acknowledge"]
   #[inline] pub fn test_aacken(&self) -> bool {
      self.aacken != 0
   }

   #[doc="Automatic Address Acknowledge"]
   #[inline] pub fn set_aacken<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

   #[doc="Address Mode"]
   #[inline] pub fn amode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
   }

   #[doc="Address Mode"]
   #[inline] pub fn test_amode(&self) -> bool {
      self.amode != 0
   }

   #[doc="Address Mode"]
   #[inline] pub fn set_amode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 14);
      self.0 |= value << 14;
      self
   }

   #[doc="Command"]
   #[inline] pub fn cmd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
   }

   #[doc="Command"]
   #[inline] pub fn test_cmd(&self) -> bool {
      self.cmd != 0
   }

   #[doc="Command"]
   #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="Acknowledge Action"]
   #[inline] pub fn ackact(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }

   #[doc="Acknowledge Action"]
   #[inline] pub fn test_ackact(&self) -> bool {
      self.ackact != 0
   }

   #[doc="Acknowledge Action"]
   #[inline] pub fn set_ackact<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u8);
impl Data {
   #[doc="Data Value"]
   #[inline] pub fn data(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }

   #[doc="Data Value"]
   #[inline] pub fn test_data(&self) -> bool {
      self.data != 0
   }

   #[doc="Data Value"]
   #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
   #[doc="Debug Mode"]
   #[inline] pub fn dbgstop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Debug Mode"]
   #[inline] pub fn test_dbgstop(&self) -> bool {
      self.dbgstop != 0
   }

   #[doc="Debug Mode"]
   #[inline] pub fn set_dbgstop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
   #[doc="Stop Received Interrupt Disable"]
   #[inline] pub fn prec(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Stop Received Interrupt Disable"]
   #[inline] pub fn test_prec(&self) -> bool {
      self.prec != 0
   }

   #[doc="Stop Received Interrupt Disable"]
   #[inline] pub fn set_prec<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Address Match Interrupt Disable"]
   #[inline] pub fn amatch(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Address Match Interrupt Disable"]
   #[inline] pub fn test_amatch(&self) -> bool {
      self.amatch != 0
   }

   #[doc="Address Match Interrupt Disable"]
   #[inline] pub fn set_amatch<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Data Interrupt Disable"]
   #[inline] pub fn drdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Data Interrupt Disable"]
   #[inline] pub fn test_drdy(&self) -> bool {
      self.drdy != 0
   }

   #[doc="Data Interrupt Disable"]
   #[inline] pub fn set_drdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
   #[doc="Stop Received Interrupt Enable"]
   #[inline] pub fn prec(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Stop Received Interrupt Enable"]
   #[inline] pub fn test_prec(&self) -> bool {
      self.prec != 0
   }

   #[doc="Stop Received Interrupt Enable"]
   #[inline] pub fn set_prec<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Address Match Interrupt Enable"]
   #[inline] pub fn amatch(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Address Match Interrupt Enable"]
   #[inline] pub fn test_amatch(&self) -> bool {
      self.amatch != 0
   }

   #[doc="Address Match Interrupt Enable"]
   #[inline] pub fn set_amatch<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Data Interrupt Enable"]
   #[inline] pub fn drdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Data Interrupt Enable"]
   #[inline] pub fn test_drdy(&self) -> bool {
      self.drdy != 0
   }

   #[doc="Data Interrupt Enable"]
   #[inline] pub fn set_drdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
   #[doc="Stop Received Interrupt"]
   #[inline] pub fn prec(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Stop Received Interrupt"]
   #[inline] pub fn test_prec(&self) -> bool {
      self.prec != 0
   }

   #[doc="Stop Received Interrupt"]
   #[inline] pub fn set_prec<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Address Match Interrupt"]
   #[inline] pub fn amatch(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Address Match Interrupt"]
   #[inline] pub fn test_amatch(&self) -> bool {
      self.amatch != 0
   }

   #[doc="Address Match Interrupt"]
   #[inline] pub fn set_amatch<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Data Interrupt"]
   #[inline] pub fn drdy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Data Interrupt"]
   #[inline] pub fn test_drdy(&self) -> bool {
      self.drdy != 0
   }

   #[doc="Data Interrupt"]
   #[inline] pub fn set_drdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
   #[doc="Bus Error"]
   #[inline] pub fn buserr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Bus Error"]
   #[inline] pub fn test_buserr(&self) -> bool {
      self.buserr != 0
   }

   #[doc="Bus Error"]
   #[inline] pub fn set_buserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Transmit Collision"]
   #[inline] pub fn coll(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Transmit Collision"]
   #[inline] pub fn test_coll(&self) -> bool {
      self.coll != 0
   }

   #[doc="Transmit Collision"]
   #[inline] pub fn set_coll<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Received Not Acknowledge"]
   #[inline] pub fn rxnack(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Received Not Acknowledge"]
   #[inline] pub fn test_rxnack(&self) -> bool {
      self.rxnack != 0
   }

   #[doc="Received Not Acknowledge"]
   #[inline] pub fn set_rxnack<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Read/Write Direction"]
   #[inline] pub fn dir(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }

   #[doc="Read/Write Direction"]
   #[inline] pub fn test_dir(&self) -> bool {
      self.dir != 0
   }

   #[doc="Read/Write Direction"]
   #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="Repeated Start"]
   #[inline] pub fn sr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }

   #[doc="Repeated Start"]
   #[inline] pub fn test_sr(&self) -> bool {
      self.sr != 0
   }

   #[doc="Repeated Start"]
   #[inline] pub fn set_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

   #[doc="SCL Low Timeout"]
   #[inline] pub fn lowtout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }

   #[doc="SCL Low Timeout"]
   #[inline] pub fn test_lowtout(&self) -> bool {
      self.lowtout != 0
   }

   #[doc="SCL Low Timeout"]
   #[inline] pub fn set_lowtout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="Clock Hold"]
   #[inline] pub fn clkhold(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Clock Hold"]
   #[inline] pub fn test_clkhold(&self) -> bool {
      self.clkhold != 0
   }

   #[doc="Clock Hold"]
   #[inline] pub fn set_clkhold<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn sexttout(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn test_sexttout(&self) -> bool {
      self.sexttout != 0
   }

   #[doc="Slave SCL Low Extend Timeout"]
   #[inline] pub fn set_sexttout<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

   #[doc="High Speed"]
   #[inline] pub fn hs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }

   #[doc="High Speed"]
   #[inline] pub fn test_hs(&self) -> bool {
      self.hs != 0
   }

   #[doc="High Speed"]
   #[inline] pub fn set_hs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn test_swrst(&self) -> bool {
      self.swrst != 0
   }

   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn test_enable(&self) -> bool {
      self.enable != 0
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[allow(unused_imports)] use bobbin_common::*;
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SPI Mode Peripheral"]
   pub struct Spi(pub usize);
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
         Addr(read_volatile((self.0 + 0x24) as *const u32))
      }
   }

   #[doc="Write the ADDR register."]
   #[inline] pub fn set_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
      let value = f(Addr(0));
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the ADDR register."]
   #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
      let tmp = self.addr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
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
         Baud(read_volatile((self.0 + 0xc) as *const u8))
      }
   }

   #[doc="Write the BAUD register."]
   #[inline] pub fn set_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
      let value = f(Baud(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the BAUD register."]
   #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
      let tmp = self.baud();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u8, value.0);
      }
      self
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
         Ctrla(read_volatile((self.0 + 0x0) as *const u32))
      }
   }

   #[doc="Write the CTRLA register."]
   #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let value = f(Ctrla(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CTRLA register."]
   #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let tmp = self.ctrla();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
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
         Ctrlb(read_volatile((self.0 + 0x4) as *const u32))
      }
   }

   #[doc="Write the CTRLB register."]
   #[inline] pub fn set_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
      let value = f(Ctrlb(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CTRLB register."]
   #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
      let tmp = self.ctrlb();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
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
         Data(read_volatile((self.0 + 0x28) as *const u32))
      }
   }

   #[doc="Write the DATA register."]
   #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let value = f(Data(0));
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DATA register."]
   #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let tmp = self.data();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
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
         Dbgctrl(read_volatile((self.0 + 0x30) as *const u8))
      }
   }

   #[doc="Write the DBGCTRL register."]
   #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the DBGCTRL register."]
   #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = self.dbgctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u8, value.0);
      }
      self
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
         Intenclr(read_volatile((self.0 + 0x14) as *const u8))
      }
   }

   #[doc="Write the INTENCLR register."]
   #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTENCLR register."]
   #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = self.intenclr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u8, value.0);
      }
      self
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
         Intenset(read_volatile((self.0 + 0x16) as *const u8))
      }
   }

   #[doc="Write the INTENSET register."]
   #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         write_volatile((self.0 + 0x16) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTENSET register."]
   #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = self.intenset();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x16) as *mut u8, value.0);
      }
      self
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
         Intflag(read_volatile((self.0 + 0x18) as *const u8))
      }
   }

   #[doc="Write the INTFLAG register."]
   #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTFLAG register."]
   #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = self.intflag();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u8, value.0);
      }
      self
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
         Status(read_volatile((self.0 + 0x1a) as *const u16))
      }
   }

   #[doc="Write the STATUS register."]
   #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let value = f(Status(0));
      unsafe {
         write_volatile((self.0 + 0x1a) as *mut u16, value.0);
      }
      self
   }

   #[doc="Modify the STATUS register."]
   #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let tmp = self.status();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1a) as *mut u16, value.0);
      }
      self
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
         Syncbusy(read_volatile((self.0 + 0x1c) as *const u32))
      }
   }

}

#[doc="SPI Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
   #[doc="Address Value"]
   #[inline] pub fn addr(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }

   #[doc="Address Value"]
   #[inline] pub fn test_addr(&self) -> bool {
      self.addr != 0
   }

   #[doc="Address Value"]
   #[inline] pub fn set_addr<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Address Mask"]
   #[inline] pub fn addrmask(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }

   #[doc="Address Mask"]
   #[inline] pub fn test_addrmask(&self) -> bool {
      self.addrmask != 0
   }

   #[doc="Address Mask"]
   #[inline] pub fn set_addrmask<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Baud(pub u8);
impl Baud {
   #[doc="Baud Rate Value"]
   #[inline] pub fn baud(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }

   #[doc="Baud Rate Value"]
   #[inline] pub fn test_baud(&self) -> bool {
      self.baud != 0
   }

   #[doc="Baud Rate Value"]
   #[inline] pub fn set_baud<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
   #[doc="Software Reset"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Software Reset"]
   #[inline] pub fn test_swrst(&self) -> bool {
      self.swrst != 0
   }

   #[doc="Software Reset"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Enable"]
   #[inline] pub fn test_enable(&self) -> bool {
      self.enable != 0
   }

   #[doc="Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Operating Mode"]
   #[inline] pub fn mode(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
   }

   #[doc="Operating Mode"]
   #[inline] pub fn test_mode(&self) -> bool {
      self.mode != 0
   }

   #[doc="Operating Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Run during Standby"]
   #[inline] pub fn runstdby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Run during Standby"]
   #[inline] pub fn test_runstdby(&self) -> bool {
      self.runstdby != 0
   }

   #[doc="Run during Standby"]
   #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

   #[doc="Immediate Buffer Overflow Notification"]
   #[inline] pub fn ibon(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }

   #[doc="Immediate Buffer Overflow Notification"]
   #[inline] pub fn test_ibon(&self) -> bool {
      self.ibon != 0
   }

   #[doc="Immediate Buffer Overflow Notification"]
   #[inline] pub fn set_ibon<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="Data Out Pinout"]
   #[inline] pub fn dopo(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
   }

   #[doc="Data Out Pinout"]
   #[inline] pub fn test_dopo(&self) -> bool {
      self.dopo != 0
   }

   #[doc="Data Out Pinout"]
   #[inline] pub fn set_dopo<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="Data In Pinout"]
   #[inline] pub fn dipo(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
   }

   #[doc="Data In Pinout"]
   #[inline] pub fn test_dipo(&self) -> bool {
      self.dipo != 0
   }

   #[doc="Data In Pinout"]
   #[inline] pub fn set_dipo<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 20);
      self.0 |= value << 20;
      self
   }

   #[doc="Frame Format"]
   #[inline] pub fn form(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
   }

   #[doc="Frame Format"]
   #[inline] pub fn test_form(&self) -> bool {
      self.form != 0
   }

   #[doc="Frame Format"]
   #[inline] pub fn set_form<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 24);
      self.0 |= value << 24;
      self
   }

   #[doc="Clock Phase"]
   #[inline] pub fn cpha(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }

   #[doc="Clock Phase"]
   #[inline] pub fn test_cpha(&self) -> bool {
      self.cpha != 0
   }

   #[doc="Clock Phase"]
   #[inline] pub fn set_cpha<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

   #[doc="Clock Polarity"]
   #[inline] pub fn cpol(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }

   #[doc="Clock Polarity"]
   #[inline] pub fn test_cpol(&self) -> bool {
      self.cpol != 0
   }

   #[doc="Clock Polarity"]
   #[inline] pub fn set_cpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

   #[doc="Data Order"]
   #[inline] pub fn dord(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }

   #[doc="Data Order"]
   #[inline] pub fn test_dord(&self) -> bool {
      self.dord != 0
   }

   #[doc="Data Order"]
   #[inline] pub fn set_dord<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
   #[doc="Character Size"]
   #[inline] pub fn chsize(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }

   #[doc="Character Size"]
   #[inline] pub fn test_chsize(&self) -> bool {
      self.chsize != 0
   }

   #[doc="Character Size"]
   #[inline] pub fn set_chsize<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Data Preload Enable"]
   #[inline] pub fn ploaden(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }

   #[doc="Data Preload Enable"]
   #[inline] pub fn test_ploaden(&self) -> bool {
      self.ploaden != 0
   }

   #[doc="Data Preload Enable"]
   #[inline] pub fn set_ploaden<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="Slave Select Low Detect Enable"]
   #[inline] pub fn ssde(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }

   #[doc="Slave Select Low Detect Enable"]
   #[inline] pub fn test_ssde(&self) -> bool {
      self.ssde != 0
   }

   #[doc="Slave Select Low Detect Enable"]
   #[inline] pub fn set_ssde<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

   #[doc="Master Slave Select Enable"]
   #[inline] pub fn mssen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }

   #[doc="Master Slave Select Enable"]
   #[inline] pub fn test_mssen(&self) -> bool {
      self.mssen != 0
   }

   #[doc="Master Slave Select Enable"]
   #[inline] pub fn set_mssen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

   #[doc="Address Mode"]
   #[inline] pub fn amode(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
   }

   #[doc="Address Mode"]
   #[inline] pub fn test_amode(&self) -> bool {
      self.amode != 0
   }

   #[doc="Address Mode"]
   #[inline] pub fn set_amode<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 14);
      self.0 |= value << 14;
      self
   }

   #[doc="Receiver Enable"]
   #[inline] pub fn rxen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }

   #[doc="Receiver Enable"]
   #[inline] pub fn test_rxen(&self) -> bool {
      self.rxen != 0
   }

   #[doc="Receiver Enable"]
   #[inline] pub fn set_rxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
   #[doc="Data Value"]
   #[inline] pub fn data(&self) -> bits::U9 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
   }

   #[doc="Data Value"]
   #[inline] pub fn test_data(&self) -> bool {
      self.data != 0
   }

   #[doc="Data Value"]
   #[inline] pub fn set_data<V: Into<bits::U9>>(mut self, value: V) -> Self {
      let value: bits::U9 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
   #[doc="Debug Mode"]
   #[inline] pub fn dbgstop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Debug Mode"]
   #[inline] pub fn test_dbgstop(&self) -> bool {
      self.dbgstop != 0
   }

   #[doc="Debug Mode"]
   #[inline] pub fn set_dbgstop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
   #[doc="Data Register Empty Interrupt Disable"]
   #[inline] pub fn dre(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Data Register Empty Interrupt Disable"]
   #[inline] pub fn test_dre(&self) -> bool {
      self.dre != 0
   }

   #[doc="Data Register Empty Interrupt Disable"]
   #[inline] pub fn set_dre<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Transmit Complete Interrupt Disable"]
   #[inline] pub fn txc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Transmit Complete Interrupt Disable"]
   #[inline] pub fn test_txc(&self) -> bool {
      self.txc != 0
   }

   #[doc="Transmit Complete Interrupt Disable"]
   #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Receive Complete Interrupt Disable"]
   #[inline] pub fn rxc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Receive Complete Interrupt Disable"]
   #[inline] pub fn test_rxc(&self) -> bool {
      self.rxc != 0
   }

   #[doc="Receive Complete Interrupt Disable"]
   #[inline] pub fn set_rxc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Slave Select Low Interrupt Disable"]
   #[inline] pub fn ssl(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }

   #[doc="Slave Select Low Interrupt Disable"]
   #[inline] pub fn test_ssl(&self) -> bool {
      self.ssl != 0
   }

   #[doc="Slave Select Low Interrupt Disable"]
   #[inline] pub fn set_ssl<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
   #[doc="Data Register Empty Interrupt Enable"]
   #[inline] pub fn dre(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Data Register Empty Interrupt Enable"]
   #[inline] pub fn test_dre(&self) -> bool {
      self.dre != 0
   }

   #[doc="Data Register Empty Interrupt Enable"]
   #[inline] pub fn set_dre<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Transmit Complete Interrupt Enable"]
   #[inline] pub fn txc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Transmit Complete Interrupt Enable"]
   #[inline] pub fn test_txc(&self) -> bool {
      self.txc != 0
   }

   #[doc="Transmit Complete Interrupt Enable"]
   #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Receive Complete Interrupt Enable"]
   #[inline] pub fn rxc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Receive Complete Interrupt Enable"]
   #[inline] pub fn test_rxc(&self) -> bool {
      self.rxc != 0
   }

   #[doc="Receive Complete Interrupt Enable"]
   #[inline] pub fn set_rxc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Slave Select Low Interrupt Enable"]
   #[inline] pub fn ssl(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }

   #[doc="Slave Select Low Interrupt Enable"]
   #[inline] pub fn test_ssl(&self) -> bool {
      self.ssl != 0
   }

   #[doc="Slave Select Low Interrupt Enable"]
   #[inline] pub fn set_ssl<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
   #[doc="Data Register Empty Interrupt"]
   #[inline] pub fn dre(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Data Register Empty Interrupt"]
   #[inline] pub fn test_dre(&self) -> bool {
      self.dre != 0
   }

   #[doc="Data Register Empty Interrupt"]
   #[inline] pub fn set_dre<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Transmit Complete Interrupt"]
   #[inline] pub fn txc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Transmit Complete Interrupt"]
   #[inline] pub fn test_txc(&self) -> bool {
      self.txc != 0
   }

   #[doc="Transmit Complete Interrupt"]
   #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Receive Complete Interrupt"]
   #[inline] pub fn rxc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Receive Complete Interrupt"]
   #[inline] pub fn test_rxc(&self) -> bool {
      self.rxc != 0
   }

   #[doc="Receive Complete Interrupt"]
   #[inline] pub fn set_rxc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Slave Select Low Interrupt Flag"]
   #[inline] pub fn ssl(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }

   #[doc="Slave Select Low Interrupt Flag"]
   #[inline] pub fn test_ssl(&self) -> bool {
      self.ssl != 0
   }

   #[doc="Slave Select Low Interrupt Flag"]
   #[inline] pub fn set_ssl<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
   #[doc="Buffer Overflow"]
   #[inline] pub fn bufovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Buffer Overflow"]
   #[inline] pub fn test_bufovf(&self) -> bool {
      self.bufovf != 0
   }

   #[doc="Buffer Overflow"]
   #[inline] pub fn set_bufovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn test_swrst(&self) -> bool {
      self.swrst != 0
   }

   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn test_enable(&self) -> bool {
      self.enable != 0
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="CTRLB Synchronization Busy"]
   #[inline] pub fn ctrlb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="CTRLB Synchronization Busy"]
   #[inline] pub fn test_ctrlb(&self) -> bool {
      self.ctrlb != 0
   }

   #[doc="CTRLB Synchronization Busy"]
   #[inline] pub fn set_ctrlb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[allow(unused_imports)] use bobbin_common::*;
   #[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USART Mode Peripheral"]
   pub struct Usart(pub usize);
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
         Baud(read_volatile((self.0 + 0xc) as *const u16))
      }
   }

   #[doc="Write the BAUD register."]
   #[inline] pub fn set_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
      let value = f(Baud(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u16, value.0);
      }
      self
   }

   #[doc="Modify the BAUD register."]
   #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
      let tmp = self.baud();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u16, value.0);
      }
      self
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
         Ctrla(read_volatile((self.0 + 0x0) as *const u32))
      }
   }

   #[doc="Write the CTRLA register."]
   #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let value = f(Ctrla(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CTRLA register."]
   #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
      let tmp = self.ctrla();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
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
         Ctrlb(read_volatile((self.0 + 0x4) as *const u32))
      }
   }

   #[doc="Write the CTRLB register."]
   #[inline] pub fn set_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
      let value = f(Ctrlb(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CTRLB register."]
   #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
      let tmp = self.ctrlb();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
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
         Data(read_volatile((self.0 + 0x28) as *const u16))
      }
   }

   #[doc="Write the DATA register."]
   #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let value = f(Data(0));
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u16, value.0);
      }
      self
   }

   #[doc="Modify the DATA register."]
   #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
      let tmp = self.data();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u16, value.0);
      }
      self
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
         Dbgctrl(read_volatile((self.0 + 0x30) as *const u8))
      }
   }

   #[doc="Write the DBGCTRL register."]
   #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let value = f(Dbgctrl(0));
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the DBGCTRL register."]
   #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
      let tmp = self.dbgctrl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u8, value.0);
      }
      self
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
         Intenclr(read_volatile((self.0 + 0x14) as *const u8))
      }
   }

   #[doc="Write the INTENCLR register."]
   #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let value = f(Intenclr(0));
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTENCLR register."]
   #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
      let tmp = self.intenclr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u8, value.0);
      }
      self
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
         Intenset(read_volatile((self.0 + 0x16) as *const u8))
      }
   }

   #[doc="Write the INTENSET register."]
   #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let value = f(Intenset(0));
      unsafe {
         write_volatile((self.0 + 0x16) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTENSET register."]
   #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
      let tmp = self.intenset();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x16) as *mut u8, value.0);
      }
      self
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
         Intflag(read_volatile((self.0 + 0x18) as *const u8))
      }
   }

   #[doc="Write the INTFLAG register."]
   #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let value = f(Intflag(0));
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the INTFLAG register."]
   #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
      let tmp = self.intflag();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u8, value.0);
      }
      self
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
         Rxpl(read_volatile((self.0 + 0xe) as *const u8))
      }
   }

   #[doc="Write the RXPL register."]
   #[inline] pub fn set_rxpl<F: FnOnce(Rxpl) -> Rxpl>(&self, f: F) -> &Self {
      let value = f(Rxpl(0));
      unsafe {
         write_volatile((self.0 + 0xe) as *mut u8, value.0);
      }
      self
   }

   #[doc="Modify the RXPL register."]
   #[inline] pub fn with_rxpl<F: FnOnce(Rxpl) -> Rxpl>(&self, f: F) -> &Self {
      let tmp = self.rxpl();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xe) as *mut u8, value.0);
      }
      self
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
         Status(read_volatile((self.0 + 0x1a) as *const u16))
      }
   }

   #[doc="Write the STATUS register."]
   #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let value = f(Status(0));
      unsafe {
         write_volatile((self.0 + 0x1a) as *mut u16, value.0);
      }
      self
   }

   #[doc="Modify the STATUS register."]
   #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let tmp = self.status();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1a) as *mut u16, value.0);
      }
      self
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
         Syncbusy(read_volatile((self.0 + 0x1c) as *const u32))
      }
   }

}

#[doc="USART Baud Rate"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Baud(pub u16);
impl Baud {
   #[doc="Baud Rate Value"]
   #[inline] pub fn baud(&self) -> bits::U13 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
   }

   #[doc="Baud Rate Value"]
   #[inline] pub fn test_baud(&self) -> bool {
      self.baud != 0
   }

   #[doc="Baud Rate Value"]
   #[inline] pub fn set_baud<V: Into<bits::U13>>(mut self, value: V) -> Self {
      let value: bits::U13 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1fff << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Fractional Part"]
   #[inline] pub fn fp(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
   }

   #[doc="Fractional Part"]
   #[inline] pub fn test_fp(&self) -> bool {
      self.fp != 0
   }

   #[doc="Fractional Part"]
   #[inline] pub fn set_fp<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u16 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
   #[doc="Software Reset"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Software Reset"]
   #[inline] pub fn test_swrst(&self) -> bool {
      self.swrst != 0
   }

   #[doc="Software Reset"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Enable"]
   #[inline] pub fn test_enable(&self) -> bool {
      self.enable != 0
   }

   #[doc="Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Operating Mode"]
   #[inline] pub fn mode(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
   }

   #[doc="Operating Mode"]
   #[inline] pub fn test_mode(&self) -> bool {
      self.mode != 0
   }

   #[doc="Operating Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Run during Standby"]
   #[inline] pub fn runstdby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Run during Standby"]
   #[inline] pub fn test_runstdby(&self) -> bool {
      self.runstdby != 0
   }

   #[doc="Run during Standby"]
   #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

   #[doc="Immediate Buffer Overflow Notification"]
   #[inline] pub fn ibon(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }

   #[doc="Immediate Buffer Overflow Notification"]
   #[inline] pub fn test_ibon(&self) -> bool {
      self.ibon != 0
   }

   #[doc="Immediate Buffer Overflow Notification"]
   #[inline] pub fn set_ibon<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="Sample"]
   #[inline] pub fn sampr(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
   }

   #[doc="Sample"]
   #[inline] pub fn test_sampr(&self) -> bool {
      self.sampr != 0
   }

   #[doc="Sample"]
   #[inline] pub fn set_sampr<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 13);
      self.0 |= value << 13;
      self
   }

   #[doc="Transmit Data Pinout"]
   #[inline] pub fn txpo(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
   }

   #[doc="Transmit Data Pinout"]
   #[inline] pub fn test_txpo(&self) -> bool {
      self.txpo != 0
   }

   #[doc="Transmit Data Pinout"]
   #[inline] pub fn set_txpo<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="Receive Data Pinout"]
   #[inline] pub fn rxpo(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
   }

   #[doc="Receive Data Pinout"]
   #[inline] pub fn test_rxpo(&self) -> bool {
      self.rxpo != 0
   }

   #[doc="Receive Data Pinout"]
   #[inline] pub fn set_rxpo<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 20);
      self.0 |= value << 20;
      self
   }

   #[doc="Sample Adjustment"]
   #[inline] pub fn sampa(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
   }

   #[doc="Sample Adjustment"]
   #[inline] pub fn test_sampa(&self) -> bool {
      self.sampa != 0
   }

   #[doc="Sample Adjustment"]
   #[inline] pub fn set_sampa<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 22);
      self.0 |= value << 22;
      self
   }

   #[doc="Frame Format"]
   #[inline] pub fn form(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
   }

   #[doc="Frame Format"]
   #[inline] pub fn test_form(&self) -> bool {
      self.form != 0
   }

   #[doc="Frame Format"]
   #[inline] pub fn set_form<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 24);
      self.0 |= value << 24;
      self
   }

   #[doc="Communication Mode"]
   #[inline] pub fn cmode(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }

   #[doc="Communication Mode"]
   #[inline] pub fn test_cmode(&self) -> bool {
      self.cmode != 0
   }

   #[doc="Communication Mode"]
   #[inline] pub fn set_cmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

   #[doc="Clock Polarity"]
   #[inline] pub fn cpol(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }

   #[doc="Clock Polarity"]
   #[inline] pub fn test_cpol(&self) -> bool {
      self.cpol != 0
   }

   #[doc="Clock Polarity"]
   #[inline] pub fn set_cpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

   #[doc="Data Order"]
   #[inline] pub fn dord(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }

   #[doc="Data Order"]
   #[inline] pub fn test_dord(&self) -> bool {
      self.dord != 0
   }

   #[doc="Data Order"]
   #[inline] pub fn set_dord<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u32);
impl Ctrlb {
   #[doc="Character Size"]
   #[inline] pub fn chsize(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }

   #[doc="Character Size"]
   #[inline] pub fn test_chsize(&self) -> bool {
      self.chsize != 0
   }

   #[doc="Character Size"]
   #[inline] pub fn set_chsize<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Stop Bit Mode"]
   #[inline] pub fn sbmode(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }

   #[doc="Stop Bit Mode"]
   #[inline] pub fn test_sbmode(&self) -> bool {
      self.sbmode != 0
   }

   #[doc="Stop Bit Mode"]
   #[inline] pub fn set_sbmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

   #[doc="Collision Detection Enable"]
   #[inline] pub fn colden(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }

   #[doc="Collision Detection Enable"]
   #[inline] pub fn test_colden(&self) -> bool {
      self.colden != 0
   }

   #[doc="Collision Detection Enable"]
   #[inline] pub fn set_colden<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="Start of Frame Detection Enable"]
   #[inline] pub fn sfde(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }

   #[doc="Start of Frame Detection Enable"]
   #[inline] pub fn test_sfde(&self) -> bool {
      self.sfde != 0
   }

   #[doc="Start of Frame Detection Enable"]
   #[inline] pub fn set_sfde<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

   #[doc="Encoding Format"]
   #[inline] pub fn enc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }

   #[doc="Encoding Format"]
   #[inline] pub fn test_enc(&self) -> bool {
      self.enc != 0
   }

   #[doc="Encoding Format"]
   #[inline] pub fn set_enc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

   #[doc="Parity Mode"]
   #[inline] pub fn pmode(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }

   #[doc="Parity Mode"]
   #[inline] pub fn test_pmode(&self) -> bool {
      self.pmode != 0
   }

   #[doc="Parity Mode"]
   #[inline] pub fn set_pmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

   #[doc="Transmitter Enable"]
   #[inline] pub fn txen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }

   #[doc="Transmitter Enable"]
   #[inline] pub fn test_txen(&self) -> bool {
      self.txen != 0
   }

   #[doc="Transmitter Enable"]
   #[inline] pub fn set_txen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

   #[doc="Receiver Enable"]
   #[inline] pub fn rxen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }

   #[doc="Receiver Enable"]
   #[inline] pub fn test_rxen(&self) -> bool {
      self.rxen != 0
   }

   #[doc="Receiver Enable"]
   #[inline] pub fn set_rxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u16);
impl Data {
   #[doc="Data Value"]
   #[inline] pub fn data(&self) -> bits::U9 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
   }

   #[doc="Data Value"]
   #[inline] pub fn test_data(&self) -> bool {
      self.data != 0
   }

   #[doc="Data Value"]
   #[inline] pub fn set_data<V: Into<bits::U9>>(mut self, value: V) -> Self {
      let value: bits::U9 = value.into();
      let value: u16 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
   #[doc="Debug Mode"]
   #[inline] pub fn dbgstop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Debug Mode"]
   #[inline] pub fn test_dbgstop(&self) -> bool {
      self.dbgstop != 0
   }

   #[doc="Debug Mode"]
   #[inline] pub fn set_dbgstop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
   #[doc="Data Register Empty Interrupt Disable"]
   #[inline] pub fn dre(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Data Register Empty Interrupt Disable"]
   #[inline] pub fn test_dre(&self) -> bool {
      self.dre != 0
   }

   #[doc="Data Register Empty Interrupt Disable"]
   #[inline] pub fn set_dre<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Transmit Complete Interrupt Disable"]
   #[inline] pub fn txc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Transmit Complete Interrupt Disable"]
   #[inline] pub fn test_txc(&self) -> bool {
      self.txc != 0
   }

   #[doc="Transmit Complete Interrupt Disable"]
   #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Receive Complete Interrupt Disable"]
   #[inline] pub fn rxc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Receive Complete Interrupt Disable"]
   #[inline] pub fn test_rxc(&self) -> bool {
      self.rxc != 0
   }

   #[doc="Receive Complete Interrupt Disable"]
   #[inline] pub fn set_rxc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Receive Start Interrupt Disable"]
   #[inline] pub fn rxs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }

   #[doc="Receive Start Interrupt Disable"]
   #[inline] pub fn test_rxs(&self) -> bool {
      self.rxs != 0
   }

   #[doc="Receive Start Interrupt Disable"]
   #[inline] pub fn set_rxs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="Clear To Send Input Change Interrupt Disable"]
   #[inline] pub fn ctsic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }

   #[doc="Clear To Send Input Change Interrupt Disable"]
   #[inline] pub fn test_ctsic(&self) -> bool {
      self.ctsic != 0
   }

   #[doc="Clear To Send Input Change Interrupt Disable"]
   #[inline] pub fn set_ctsic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

   #[doc="Break Received Interrupt Disable"]
   #[inline] pub fn rxbrk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }

   #[doc="Break Received Interrupt Disable"]
   #[inline] pub fn test_rxbrk(&self) -> bool {
      self.rxbrk != 0
   }

   #[doc="Break Received Interrupt Disable"]
   #[inline] pub fn set_rxbrk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt Disable"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
   #[doc="Data Register Empty Interrupt Enable"]
   #[inline] pub fn dre(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Data Register Empty Interrupt Enable"]
   #[inline] pub fn test_dre(&self) -> bool {
      self.dre != 0
   }

   #[doc="Data Register Empty Interrupt Enable"]
   #[inline] pub fn set_dre<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Transmit Complete Interrupt Enable"]
   #[inline] pub fn txc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Transmit Complete Interrupt Enable"]
   #[inline] pub fn test_txc(&self) -> bool {
      self.txc != 0
   }

   #[doc="Transmit Complete Interrupt Enable"]
   #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Receive Complete Interrupt Enable"]
   #[inline] pub fn rxc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Receive Complete Interrupt Enable"]
   #[inline] pub fn test_rxc(&self) -> bool {
      self.rxc != 0
   }

   #[doc="Receive Complete Interrupt Enable"]
   #[inline] pub fn set_rxc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Receive Start Interrupt Enable"]
   #[inline] pub fn rxs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }

   #[doc="Receive Start Interrupt Enable"]
   #[inline] pub fn test_rxs(&self) -> bool {
      self.rxs != 0
   }

   #[doc="Receive Start Interrupt Enable"]
   #[inline] pub fn set_rxs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="Clear To Send Input Change Interrupt Enable"]
   #[inline] pub fn ctsic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }

   #[doc="Clear To Send Input Change Interrupt Enable"]
   #[inline] pub fn test_ctsic(&self) -> bool {
      self.ctsic != 0
   }

   #[doc="Clear To Send Input Change Interrupt Enable"]
   #[inline] pub fn set_ctsic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

   #[doc="Break Received Interrupt Enable"]
   #[inline] pub fn rxbrk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }

   #[doc="Break Received Interrupt Enable"]
   #[inline] pub fn test_rxbrk(&self) -> bool {
      self.rxbrk != 0
   }

   #[doc="Break Received Interrupt Enable"]
   #[inline] pub fn set_rxbrk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt Enable"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
   #[doc="Data Register Empty Interrupt"]
   #[inline] pub fn dre(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Data Register Empty Interrupt"]
   #[inline] pub fn test_dre(&self) -> bool {
      self.dre != 0
   }

   #[doc="Data Register Empty Interrupt"]
   #[inline] pub fn set_dre<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Transmit Complete Interrupt"]
   #[inline] pub fn txc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Transmit Complete Interrupt"]
   #[inline] pub fn test_txc(&self) -> bool {
      self.txc != 0
   }

   #[doc="Transmit Complete Interrupt"]
   #[inline] pub fn set_txc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Receive Complete Interrupt"]
   #[inline] pub fn rxc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Receive Complete Interrupt"]
   #[inline] pub fn test_rxc(&self) -> bool {
      self.rxc != 0
   }

   #[doc="Receive Complete Interrupt"]
   #[inline] pub fn set_rxc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Receive Start Interrupt"]
   #[inline] pub fn rxs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }

   #[doc="Receive Start Interrupt"]
   #[inline] pub fn test_rxs(&self) -> bool {
      self.rxs != 0
   }

   #[doc="Receive Start Interrupt"]
   #[inline] pub fn set_rxs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="Clear To Send Input Change Interrupt"]
   #[inline] pub fn ctsic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }

   #[doc="Clear To Send Input Change Interrupt"]
   #[inline] pub fn test_ctsic(&self) -> bool {
      self.ctsic != 0
   }

   #[doc="Clear To Send Input Change Interrupt"]
   #[inline] pub fn set_ctsic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

   #[doc="Break Received Interrupt"]
   #[inline] pub fn rxbrk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }

   #[doc="Break Received Interrupt"]
   #[inline] pub fn test_rxbrk(&self) -> bool {
      self.rxbrk != 0
   }

   #[doc="Break Received Interrupt"]
   #[inline] pub fn set_rxbrk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn error(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn test_error(&self) -> bool {
      self.error != 0
   }

   #[doc="Combined Error Interrupt"]
   #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxpl(pub u8);
impl Rxpl {
   #[doc="Receive Pulse Length"]
   #[inline] pub fn rxpl(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }

   #[doc="Receive Pulse Length"]
   #[inline] pub fn test_rxpl(&self) -> bool {
      self.rxpl != 0
   }

   #[doc="Receive Pulse Length"]
   #[inline] pub fn set_rxpl<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u8 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
   #[doc="Parity Error"]
   #[inline] pub fn perr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Parity Error"]
   #[inline] pub fn test_perr(&self) -> bool {
      self.perr != 0
   }

   #[doc="Parity Error"]
   #[inline] pub fn set_perr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Frame Error"]
   #[inline] pub fn ferr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Frame Error"]
   #[inline] pub fn test_ferr(&self) -> bool {
      self.ferr != 0
   }

   #[doc="Frame Error"]
   #[inline] pub fn set_ferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Buffer Overflow"]
   #[inline] pub fn bufovf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Buffer Overflow"]
   #[inline] pub fn test_bufovf(&self) -> bool {
      self.bufovf != 0
   }

   #[doc="Buffer Overflow"]
   #[inline] pub fn set_bufovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Clear To Send"]
   #[inline] pub fn cts(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }

   #[doc="Clear To Send"]
   #[inline] pub fn test_cts(&self) -> bool {
      self.cts != 0
   }

   #[doc="Clear To Send"]
   #[inline] pub fn set_cts<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

   #[doc="Inconsistent Sync Field"]
   #[inline] pub fn isf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }

   #[doc="Inconsistent Sync Field"]
   #[inline] pub fn test_isf(&self) -> bool {
      self.isf != 0
   }

   #[doc="Inconsistent Sync Field"]
   #[inline] pub fn set_isf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

   #[doc="Collision Detected"]
   #[inline] pub fn coll(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }

   #[doc="Collision Detected"]
   #[inline] pub fn test_coll(&self) -> bool {
      self.coll != 0
   }

   #[doc="Collision Detected"]
   #[inline] pub fn set_coll<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn swrst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn test_swrst(&self) -> bool {
      self.swrst != 0
   }

   #[doc="Software Reset Synchronization Busy"]
   #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn test_enable(&self) -> bool {
      self.enable != 0
   }

   #[doc="SERCOM Enable Synchronization Busy"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="CTRLB Synchronization Busy"]
   #[inline] pub fn ctrlb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="CTRLB Synchronization Busy"]
   #[inline] pub fn test_ctrlb(&self) -> bool {
      self.ctrlb != 0
   }

   #[doc="CTRLB Synchronization Busy"]
   #[inline] pub fn set_ctrlb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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


