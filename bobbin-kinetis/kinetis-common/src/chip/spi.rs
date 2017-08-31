#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SPI Peripheral"]
pub struct SpiPeriph(pub usize); 


impl SpiPeriph {
#[doc="Get the *const pointer for the MCR register."]
   #[inline] pub fn mcr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the MCR register."]
   #[inline] pub fn mcr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the MCR register."]
   #[inline] pub fn mcr(&self) -> Mcr { 
      unsafe {
         Mcr(read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the MCR register."]
   #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
      let value = f(Mcr(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MCR register."]
   #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
      let tmp = self.mcr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the TCR register."]
   #[inline] pub fn tcr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8) as *const u32
   }
#[doc="Get the *mut pointer for the TCR register."]
   #[inline] pub fn tcr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8) as *mut u32
   }
#[doc="Read the TCR register."]
   #[inline] pub fn tcr(&self) -> Tcr { 
      unsafe {
         Tcr(read_volatile((self.0 + 0x8) as *const u32))
      }
   }
#[doc="Write the TCR register."]
   #[inline] pub fn set_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
      let value = f(Tcr(0));
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TCR register."]
   #[inline] pub fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
      let tmp = self.tcr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTAR register."]
   #[inline] pub fn ctar_ptr<I: Into<bits::R2>>(&self, index: I) -> *const u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0xc + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the CTAR register."]
   #[inline] pub fn ctar_mut<I: Into<bits::R2>>(&self, index: I) -> *mut u32 { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0xc + (index << 2)) as *mut u32
   }
#[doc="Read the CTAR register."]
   #[inline] pub fn ctar<I: Into<bits::R2>>(&self, index: I) -> Ctar { 
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Ctar(read_volatile((self.0 + 0xc + (index << 2)) as *const u32))
      }
   }
#[doc="Write the CTAR register."]
   #[inline] pub fn set_ctar<I: Into<bits::R2>, F: FnOnce(Ctar) -> Ctar>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Ctar(0));
      unsafe {
         write_volatile((self.0 + 0xc + (index << 2)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CTAR register."]
   #[inline] pub fn with_ctar<I: Into<bits::R2> + Copy, F: FnOnce(Ctar) -> Ctar>(&self, index: I, f: F) -> &Self {
      let index: bits::R2 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.ctar(index);
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc + (index << 2)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTAR_SLAVE register."]
   #[inline] pub fn ctar_slave_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc) as *const u32
   }
#[doc="Get the *mut pointer for the CTAR_SLAVE register."]
   #[inline] pub fn ctar_slave_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc) as *mut u32
   }
#[doc="Read the CTAR_SLAVE register."]
   #[inline] pub fn ctar_slave(&self) -> CtarSlave { 
      unsafe {
         CtarSlave(read_volatile((self.0 + 0xc) as *const u32))
      }
   }
#[doc="Write the CTAR_SLAVE register."]
   #[inline] pub fn set_ctar_slave<F: FnOnce(CtarSlave) -> CtarSlave>(&self, f: F) -> &Self {
      let value = f(CtarSlave(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CTAR_SLAVE register."]
   #[inline] pub fn with_ctar_slave<F: FnOnce(CtarSlave) -> CtarSlave>(&self, f: F) -> &Self {
      let tmp = self.ctar_slave();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SR register."]
   #[inline] pub fn sr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x2c) as *const u32
   }
#[doc="Get the *mut pointer for the SR register."]
   #[inline] pub fn sr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x2c) as *mut u32
   }
#[doc="Read the SR register."]
   #[inline] pub fn sr(&self) -> Sr { 
      unsafe {
         Sr(read_volatile((self.0 + 0x2c) as *const u32))
      }
   }
#[doc="Write the SR register."]
   #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let value = f(Sr(0));
      unsafe {
         write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SR register."]
   #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let tmp = self.sr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RSER register."]
   #[inline] pub fn rser_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x30) as *const u32
   }
#[doc="Get the *mut pointer for the RSER register."]
   #[inline] pub fn rser_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x30) as *mut u32
   }
#[doc="Read the RSER register."]
   #[inline] pub fn rser(&self) -> Rser { 
      unsafe {
         Rser(read_volatile((self.0 + 0x30) as *const u32))
      }
   }
#[doc="Write the RSER register."]
   #[inline] pub fn set_rser<F: FnOnce(Rser) -> Rser>(&self, f: F) -> &Self {
      let value = f(Rser(0));
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RSER register."]
   #[inline] pub fn with_rser<F: FnOnce(Rser) -> Rser>(&self, f: F) -> &Self {
      let tmp = self.rser();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PUSHR register."]
   #[inline] pub fn pushr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x34) as *const u32
   }
#[doc="Get the *mut pointer for the PUSHR register."]
   #[inline] pub fn pushr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x34) as *mut u32
   }
#[doc="Read the PUSHR register."]
   #[inline] pub fn pushr(&self) -> Pushr { 
      unsafe {
         Pushr(read_volatile((self.0 + 0x34) as *const u32))
      }
   }
#[doc="Write the PUSHR register."]
   #[inline] pub fn set_pushr<F: FnOnce(Pushr) -> Pushr>(&self, f: F) -> &Self {
      let value = f(Pushr(0));
      unsafe {
         write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PUSHR register."]
   #[inline] pub fn with_pushr<F: FnOnce(Pushr) -> Pushr>(&self, f: F) -> &Self {
      let tmp = self.pushr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PUSHR_SLAVE register."]
   #[inline] pub fn pushr_slave_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x34) as *const u32
   }
#[doc="Get the *mut pointer for the PUSHR_SLAVE register."]
   #[inline] pub fn pushr_slave_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x34) as *mut u32
   }
#[doc="Read the PUSHR_SLAVE register."]
   #[inline] pub fn pushr_slave(&self) -> PushrSlave { 
      unsafe {
         PushrSlave(read_volatile((self.0 + 0x34) as *const u32))
      }
   }
#[doc="Write the PUSHR_SLAVE register."]
   #[inline] pub fn set_pushr_slave<F: FnOnce(PushrSlave) -> PushrSlave>(&self, f: F) -> &Self {
      let value = f(PushrSlave(0));
      unsafe {
         write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PUSHR_SLAVE register."]
   #[inline] pub fn with_pushr_slave<F: FnOnce(PushrSlave) -> PushrSlave>(&self, f: F) -> &Self {
      let tmp = self.pushr_slave();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the POPR register."]
   #[inline] pub fn popr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x38) as *const u32
   }
#[doc="Get the *mut pointer for the POPR register."]
   #[inline] pub fn popr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x38) as *mut u32
   }
#[doc="Read the POPR register."]
   #[inline] pub fn popr(&self) -> Popr { 
      unsafe {
         Popr(read_volatile((self.0 + 0x38) as *const u32))
      }
   }

#[doc="Get the *const pointer for the TXFR register."]
   #[inline] pub fn txfr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x3c + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the TXFR register."]
   #[inline] pub fn txfr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x3c + (index << 2)) as *mut u32
   }
#[doc="Read the TXFR register."]
   #[inline] pub fn txfr<I: Into<bits::R4>>(&self, index: I) -> Txfr { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Txfr(read_volatile((self.0 + 0x3c + (index << 2)) as *const u32))
      }
   }

#[doc="Get the *const pointer for the RXFR register."]
   #[inline] pub fn rxfr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x7c + (index << 2)) as *const u32
   }
#[doc="Get the *mut pointer for the RXFR register."]
   #[inline] pub fn rxfr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x7c + (index << 2)) as *mut u32
   }
#[doc="Read the RXFR register."]
   #[inline] pub fn rxfr<I: Into<bits::R4>>(&self, index: I) -> Rxfr { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Rxfr(read_volatile((self.0 + 0x7c + (index << 2)) as *const u32))
      }
   }

}

#[doc="Module Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
#[doc="Halt"]
   #[inline] pub fn halt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Halt"]
   #[inline] pub fn set_halt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Sample Point"]
   #[inline] pub fn smpl_pt(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
   }
#[doc="Sample Point"]
   #[inline] pub fn set_smpl_pt<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Flushes the RX FIFO"]
   #[inline] pub fn clr_rxf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Flushes the RX FIFO"]
   #[inline] pub fn set_clr_rxf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Clear TX FIFO"]
   #[inline] pub fn clr_txf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Clear TX FIFO"]
   #[inline] pub fn set_clr_txf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Disable Receive FIFO"]
   #[inline] pub fn dis_rxf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Disable Receive FIFO"]
   #[inline] pub fn set_dis_rxf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Disable Transmit FIFO"]
   #[inline] pub fn dis_txf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Disable Transmit FIFO"]
   #[inline] pub fn set_dis_txf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Module Disable"]
   #[inline] pub fn mdis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Module Disable"]
   #[inline] pub fn set_mdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Doze Enable"]
   #[inline] pub fn doze(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Doze Enable"]
   #[inline] pub fn set_doze<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Peripheral Chip Select x Inactive State"]
   #[inline] pub fn pcsis(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
   }
#[doc="Peripheral Chip Select x Inactive State"]
   #[inline] pub fn set_pcsis<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Receive FIFO Overflow Overwrite Enable"]
   #[inline] pub fn rooe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="Receive FIFO Overflow Overwrite Enable"]
   #[inline] pub fn set_rooe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Peripheral Chip Select Strobe Enable"]
   #[inline] pub fn pcsse(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Peripheral Chip Select Strobe Enable"]
   #[inline] pub fn set_pcsse<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="Modified Timing Format Enable"]
   #[inline] pub fn mtfe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
   }
#[doc="Modified Timing Format Enable"]
   #[inline] pub fn set_mtfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 26);
      self.0 |= value << 26;
      self
   }

#[doc="Freeze"]
   #[inline] pub fn frz(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
   }
#[doc="Freeze"]
   #[inline] pub fn set_frz<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 27);
      self.0 |= value << 27;
      self
   }

#[doc="SPI Configuration."]
   #[inline] pub fn dconf(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
   }
#[doc="SPI Configuration."]
   #[inline] pub fn set_dconf<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="Continuous SCK Enable"]
   #[inline] pub fn cont_scke(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Continuous SCK Enable"]
   #[inline] pub fn set_cont_scke<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Master/Slave Mode Select"]
   #[inline] pub fn mstr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Master/Slave Mode Select"]
   #[inline] pub fn set_mstr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.halt() != 0 { try!(write!(f, " halt"))}
      if self.smpl_pt() != 0 { try!(write!(f, " smpl_pt=0x{:x}", self.smpl_pt()))}
      if self.clr_rxf() != 0 { try!(write!(f, " clr_rxf"))}
      if self.clr_txf() != 0 { try!(write!(f, " clr_txf"))}
      if self.dis_rxf() != 0 { try!(write!(f, " dis_rxf"))}
      if self.dis_txf() != 0 { try!(write!(f, " dis_txf"))}
      if self.mdis() != 0 { try!(write!(f, " mdis"))}
      if self.doze() != 0 { try!(write!(f, " doze"))}
      if self.pcsis() != 0 { try!(write!(f, " pcsis=0x{:x}", self.pcsis()))}
      if self.rooe() != 0 { try!(write!(f, " rooe"))}
      if self.pcsse() != 0 { try!(write!(f, " pcsse"))}
      if self.mtfe() != 0 { try!(write!(f, " mtfe"))}
      if self.frz() != 0 { try!(write!(f, " frz"))}
      if self.dconf() != 0 { try!(write!(f, " dconf=0x{:x}", self.dconf()))}
      if self.cont_scke() != 0 { try!(write!(f, " cont_scke"))}
      if self.mstr() != 0 { try!(write!(f, " mstr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Transfer Count Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tcr(pub u32);
impl Tcr {
#[doc="SPI Transfer Counter"]
   #[inline] pub fn spi_tcnt(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
   }
#[doc="SPI Transfer Counter"]
   #[inline] pub fn set_spi_tcnt<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Tcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.spi_tcnt() != 0 { try!(write!(f, " spi_tcnt=0x{:x}", self.spi_tcnt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clock and Transfer Attributes Register (In Master Mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctar(pub u32);
impl Ctar {
#[doc="Baud Rate Scaler"]
   #[inline] pub fn br(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Baud Rate Scaler"]
   #[inline] pub fn set_br<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Delay After Transfer Scaler"]
   #[inline] pub fn dt(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
   }
#[doc="Delay After Transfer Scaler"]
   #[inline] pub fn set_dt<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 4);
      self.0 |= value << 4;
      self
   }

#[doc="After SCK Delay Scaler"]
   #[inline] pub fn asc(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
   }
#[doc="After SCK Delay Scaler"]
   #[inline] pub fn set_asc<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 8);
      self.0 |= value << 8;
      self
   }

#[doc="PCS to SCK Delay Scaler"]
   #[inline] pub fn cssck(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
   }
#[doc="PCS to SCK Delay Scaler"]
   #[inline] pub fn set_cssck<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Baud Rate Prescaler"]
   #[inline] pub fn pbr(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
   }
#[doc="Baud Rate Prescaler"]
   #[inline] pub fn set_pbr<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Delay after Transfer Prescaler"]
   #[inline] pub fn pdt(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
   }
#[doc="Delay after Transfer Prescaler"]
   #[inline] pub fn set_pdt<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="After SCK Delay Prescaler"]
   #[inline] pub fn pasc(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
   }
#[doc="After SCK Delay Prescaler"]
   #[inline] pub fn set_pasc<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="PCS to SCK Delay Prescaler"]
   #[inline] pub fn pcssck(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
   }
#[doc="PCS to SCK Delay Prescaler"]
   #[inline] pub fn set_pcssck<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="LSB First"]
   #[inline] pub fn lsbfe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="LSB First"]
   #[inline] pub fn set_lsbfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Phase"]
   #[inline] pub fn cpha(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Clock Phase"]
   #[inline] pub fn set_cpha<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="Clock Polarity"]
   #[inline] pub fn cpol(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
   }
#[doc="Clock Polarity"]
   #[inline] pub fn set_cpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 26);
      self.0 |= value << 26;
      self
   }

#[doc="Frame Size"]
   #[inline] pub fn fmsz(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0xf) as u8) } // [30:27]
   }
#[doc="Frame Size"]
   #[inline] pub fn set_fmsz<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 27);
      self.0 |= value << 27;
      self
   }

#[doc="Double Baud Rate"]
   #[inline] pub fn dbr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Double Baud Rate"]
   #[inline] pub fn set_dbr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Ctar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.br() != 0 { try!(write!(f, " br=0x{:x}", self.br()))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.asc() != 0 { try!(write!(f, " asc=0x{:x}", self.asc()))}
      if self.cssck() != 0 { try!(write!(f, " cssck=0x{:x}", self.cssck()))}
      if self.pbr() != 0 { try!(write!(f, " pbr=0x{:x}", self.pbr()))}
      if self.pdt() != 0 { try!(write!(f, " pdt=0x{:x}", self.pdt()))}
      if self.pasc() != 0 { try!(write!(f, " pasc=0x{:x}", self.pasc()))}
      if self.pcssck() != 0 { try!(write!(f, " pcssck=0x{:x}", self.pcssck()))}
      if self.lsbfe() != 0 { try!(write!(f, " lsbfe"))}
      if self.cpha() != 0 { try!(write!(f, " cpha"))}
      if self.cpol() != 0 { try!(write!(f, " cpol"))}
      if self.fmsz() != 0 { try!(write!(f, " fmsz=0x{:x}", self.fmsz()))}
      if self.dbr() != 0 { try!(write!(f, " dbr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Clock and Transfer Attributes Register (In Slave Mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CtarSlave(pub u32);
impl CtarSlave {
#[doc="Clock Phase"]
   #[inline] pub fn cpha(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Clock Phase"]
   #[inline] pub fn set_cpha<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="Clock Polarity"]
   #[inline] pub fn cpol(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
   }
#[doc="Clock Polarity"]
   #[inline] pub fn set_cpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 26);
      self.0 |= value << 26;
      self
   }

#[doc="Frame Size"]
   #[inline] pub fn fmsz(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1f) as u8) } // [31:27]
   }
#[doc="Frame Size"]
   #[inline] pub fn set_fmsz<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 27);
      self.0 |= value << 27;
      self
   }

}
impl ::core::fmt::Display for CtarSlave {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for CtarSlave {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cpha() != 0 { try!(write!(f, " cpha"))}
      if self.cpol() != 0 { try!(write!(f, " cpol"))}
      if self.fmsz() != 0 { try!(write!(f, " fmsz=0x{:x}", self.fmsz()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="Pop Next Pointer"]
   #[inline] pub fn popnxtptr(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Pop Next Pointer"]
   #[inline] pub fn set_popnxtptr<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="RX FIFO Counter"]
   #[inline] pub fn rxctr(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
   }
#[doc="RX FIFO Counter"]
   #[inline] pub fn set_rxctr<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Transmit Next Pointer"]
   #[inline] pub fn txnxtptr(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
   }
#[doc="Transmit Next Pointer"]
   #[inline] pub fn set_txnxtptr<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 8);
      self.0 |= value << 8;
      self
   }

#[doc="TX FIFO Counter"]
   #[inline] pub fn txctr(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
   }
#[doc="TX FIFO Counter"]
   #[inline] pub fn set_txctr<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Receive FIFO Drain Flag"]
   #[inline] pub fn rfdf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Receive FIFO Drain Flag"]
   #[inline] pub fn set_rfdf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Receive FIFO Overflow Flag"]
   #[inline] pub fn rfof(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="Receive FIFO Overflow Flag"]
   #[inline] pub fn set_rfof<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="Transmit FIFO Fill Flag"]
   #[inline] pub fn tfff(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Transmit FIFO Fill Flag"]
   #[inline] pub fn set_tfff<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="Transmit FIFO Underflow Flag"]
   #[inline] pub fn tfuf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
   }
#[doc="Transmit FIFO Underflow Flag"]
   #[inline] pub fn set_tfuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 27);
      self.0 |= value << 27;
      self
   }

#[doc="End of Queue Flag"]
   #[inline] pub fn eoqf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="End of Queue Flag"]
   #[inline] pub fn set_eoqf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="TX and RX Status"]
   #[inline] pub fn txrxs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="TX and RX Status"]
   #[inline] pub fn set_txrxs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Transfer Complete Flag"]
   #[inline] pub fn tcf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Transfer Complete Flag"]
   #[inline] pub fn set_tcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
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
      if self.popnxtptr() != 0 { try!(write!(f, " popnxtptr=0x{:x}", self.popnxtptr()))}
      if self.rxctr() != 0 { try!(write!(f, " rxctr=0x{:x}", self.rxctr()))}
      if self.txnxtptr() != 0 { try!(write!(f, " txnxtptr=0x{:x}", self.txnxtptr()))}
      if self.txctr() != 0 { try!(write!(f, " txctr=0x{:x}", self.txctr()))}
      if self.rfdf() != 0 { try!(write!(f, " rfdf"))}
      if self.rfof() != 0 { try!(write!(f, " rfof"))}
      if self.tfff() != 0 { try!(write!(f, " tfff"))}
      if self.tfuf() != 0 { try!(write!(f, " tfuf"))}
      if self.eoqf() != 0 { try!(write!(f, " eoqf"))}
      if self.txrxs() != 0 { try!(write!(f, " txrxs"))}
      if self.tcf() != 0 { try!(write!(f, " tcf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA/Interrupt Request Select and Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rser(pub u32);
impl Rser {
#[doc="Receive FIFO Drain DMA or Interrupt Request Select"]
   #[inline] pub fn rfdf_dirs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Receive FIFO Drain DMA or Interrupt Request Select"]
   #[inline] pub fn set_rfdf_dirs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Receive FIFO Drain Request Enable"]
   #[inline] pub fn rfdf_re(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Receive FIFO Drain Request Enable"]
   #[inline] pub fn set_rfdf_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Receive FIFO Overflow Request Enable"]
   #[inline] pub fn rfof_re(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="Receive FIFO Overflow Request Enable"]
   #[inline] pub fn set_rfof_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="Transmit FIFO Fill DMA or Interrupt Request Select"]
   #[inline] pub fn tfff_dirs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="Transmit FIFO Fill DMA or Interrupt Request Select"]
   #[inline] pub fn set_tfff_dirs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Transmit FIFO Fill Request Enable"]
   #[inline] pub fn tfff_re(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Transmit FIFO Fill Request Enable"]
   #[inline] pub fn set_tfff_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="Transmit FIFO Underflow Request Enable"]
   #[inline] pub fn tfuf_re(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
   }
#[doc="Transmit FIFO Underflow Request Enable"]
   #[inline] pub fn set_tfuf_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 27);
      self.0 |= value << 27;
      self
   }

#[doc="Finished Request Enable"]
   #[inline] pub fn eoqf_re(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="Finished Request Enable"]
   #[inline] pub fn set_eoqf_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="Transmission Complete Request Enable"]
   #[inline] pub fn tcf_re(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Transmission Complete Request Enable"]
   #[inline] pub fn set_tcf_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Rser {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rser {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rfdf_dirs() != 0 { try!(write!(f, " rfdf_dirs"))}
      if self.rfdf_re() != 0 { try!(write!(f, " rfdf_re"))}
      if self.rfof_re() != 0 { try!(write!(f, " rfof_re"))}
      if self.tfff_dirs() != 0 { try!(write!(f, " tfff_dirs"))}
      if self.tfff_re() != 0 { try!(write!(f, " tfff_re"))}
      if self.tfuf_re() != 0 { try!(write!(f, " tfuf_re"))}
      if self.eoqf_re() != 0 { try!(write!(f, " eoqf_re"))}
      if self.tcf_re() != 0 { try!(write!(f, " tcf_re"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PUSH TX FIFO Register In Master Mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pushr(pub u32);
impl Pushr {
#[doc="Transmit Data"]
   #[inline] pub fn txdata(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Transmit Data"]
   #[inline] pub fn set_txdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Select which PCS signals are to be asserted for the transfer"]
   #[inline] pub fn pcs(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
   }
#[doc="Select which PCS signals are to be asserted for the transfer"]
   #[inline] pub fn set_pcs<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Clear Transfer Counter"]
   #[inline] pub fn ctcnt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
   }
#[doc="Clear Transfer Counter"]
   #[inline] pub fn set_ctcnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 26);
      self.0 |= value << 26;
      self
   }

#[doc="End Of Queue"]
   #[inline] pub fn eoq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
   }
#[doc="End Of Queue"]
   #[inline] pub fn set_eoq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 27);
      self.0 |= value << 27;
      self
   }

#[doc="Clock and Transfer Attributes Select"]
   #[inline] pub fn ctas(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
   }
#[doc="Clock and Transfer Attributes Select"]
   #[inline] pub fn set_ctas<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="Continuous Peripheral Chip Select Enable"]
   #[inline] pub fn cont(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Continuous Peripheral Chip Select Enable"]
   #[inline] pub fn set_cont<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Pushr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pushr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txdata() != 0 { try!(write!(f, " txdata=0x{:x}", self.txdata()))}
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.ctcnt() != 0 { try!(write!(f, " ctcnt"))}
      if self.eoq() != 0 { try!(write!(f, " eoq"))}
      if self.ctas() != 0 { try!(write!(f, " ctas=0x{:x}", self.ctas()))}
      if self.cont() != 0 { try!(write!(f, " cont"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PUSH TX FIFO Register In Slave Mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PushrSlave(pub u32);
impl PushrSlave {
#[doc="Transmit Data"]
   #[inline] pub fn txdata(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Transmit Data"]
   #[inline] pub fn set_txdata<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for PushrSlave {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for PushrSlave {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="POP RX FIFO Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Popr(pub u32);
impl Popr {
#[doc="Received Data"]
   #[inline] pub fn rxdata(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Received Data"]
   #[inline] pub fn set_rxdata<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Popr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Popr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Transmit FIFO Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txfr(pub u32);
impl Txfr {
#[doc="Transmit Data"]
   #[inline] pub fn txdata(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Transmit Data"]
   #[inline] pub fn set_txdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transmit Command or Transmit Data"]
   #[inline] pub fn txcmd_txdata(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
   }
#[doc="Transmit Command or Transmit Data"]
   #[inline] pub fn set_txcmd_txdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Txfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Txfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txdata() != 0 { try!(write!(f, " txdata=0x{:x}", self.txdata()))}
      if self.txcmd_txdata() != 0 { try!(write!(f, " txcmd_txdata=0x{:x}", self.txcmd_txdata()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Receive FIFO Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxfr(pub u32);
impl Rxfr {
#[doc="Receive Data"]
   #[inline] pub fn rxdata(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Receive Data"]
   #[inline] pub fn set_rxdata<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rxfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rxfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

