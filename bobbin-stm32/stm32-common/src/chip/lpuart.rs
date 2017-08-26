#[allow(unused_imports)] use bobbin_common::*;



pub trait LpuartPeriph : Base {
#[doc="Get the *const pointer for the CR1 register."]
   #[inline] fn cr1_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the CR1 register."]
   #[inline] fn cr1_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the CR1 register."]
   #[inline] fn cr1(&self) -> Cr1 { 
      unsafe {
         Cr1(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      }
   }
#[doc="Write the CR1 register."]
   #[inline] fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
      let value = f(Cr1(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR1 register."]
   #[inline] fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cr1(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CR2 register."]
   #[inline] fn cr2_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the CR2 register."]
   #[inline] fn cr2_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the CR2 register."]
   #[inline] fn cr2(&self) -> Cr2 { 
      unsafe {
         Cr2(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      }
   }
#[doc="Write the CR2 register."]
   #[inline] fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
      let value = f(Cr2(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR2 register."]
   #[inline] fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cr2(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CR3 register."]
   #[inline] fn cr3_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the CR3 register."]
   #[inline] fn cr3_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the CR3 register."]
   #[inline] fn cr3(&self) -> Cr3 { 
      unsafe {
         Cr3(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      }
   }
#[doc="Write the CR3 register."]
   #[inline] fn set_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
      let value = f(Cr3(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR3 register."]
   #[inline] fn with_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cr3(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the BRR register."]
   #[inline] fn brr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the BRR register."]
   #[inline] fn brr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Read the BRR register."]
   #[inline] fn brr(&self) -> Brr { 
      unsafe {
         Brr(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      }
   }
#[doc="Write the BRR register."]
   #[inline] fn set_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
      let value = f(Brr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the BRR register."]
   #[inline] fn with_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Brr(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RQR register."]
   #[inline] fn rqr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x18)
   }
#[doc="Get the *mut pointer for the RQR register."]
   #[inline] fn rqr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x18)
   }
#[doc="Write the RQR register."]
   #[inline] fn set_rqr<F: FnOnce(Rqr) -> Rqr>(&self, f: F) -> &Self {
      let value = f(Rqr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ISR register."]
   #[inline] fn isr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x1c)
   }
#[doc="Get the *mut pointer for the ISR register."]
   #[inline] fn isr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x1c)
   }
#[doc="Read the ISR register."]
   #[inline] fn isr(&self) -> Isr { 
      unsafe {
         Isr(::core::ptr::read_volatile((self.base() + 0x1c) as *const u32))
      }
   }

#[doc="Get the *const pointer for the ICR register."]
   #[inline] fn icr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x20)
   }
#[doc="Get the *mut pointer for the ICR register."]
   #[inline] fn icr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x20)
   }
#[doc="Write the ICR register."]
   #[inline] fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
      let value = f(Icr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RDR register."]
   #[inline] fn rdr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x24)
   }
#[doc="Get the *mut pointer for the RDR register."]
   #[inline] fn rdr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x24)
   }
#[doc="Read the RDR register."]
   #[inline] fn rdr(&self) -> Rdr { 
      unsafe {
         Rdr(::core::ptr::read_volatile((self.base() + 0x24) as *const u32))
      }
   }

#[doc="Get the *const pointer for the TDR register."]
   #[inline] fn tdr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x28)
   }
#[doc="Get the *mut pointer for the TDR register."]
   #[inline] fn tdr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x28)
   }
#[doc="Read the TDR register."]
   #[inline] fn tdr(&self) -> Tdr { 
      unsafe {
         Tdr(::core::ptr::read_volatile((self.base() + 0x28) as *const u32))
      }
   }
#[doc="Write the TDR register."]
   #[inline] fn set_tdr<F: FnOnce(Tdr) -> Tdr>(&self, f: F) -> &Self {
      let value = f(Tdr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the TDR register."]
   #[inline] fn with_tdr<F: FnOnce(Tdr) -> Tdr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Tdr(::core::ptr::read_volatile((self.base() + 0x28) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x28) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Control register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
#[doc="Word length"]
   #[inline] pub fn m1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="Word length"]
   #[inline] pub fn set_m1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="Driver Enable assertion time"]
   #[inline] pub fn deat4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Driver Enable assertion time"]
   #[inline] pub fn set_deat4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="DEAT3"]
   #[inline] pub fn deat3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="DEAT3"]
   #[inline] pub fn set_deat3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="DEAT2"]
   #[inline] pub fn deat2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="DEAT2"]
   #[inline] pub fn set_deat2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

#[doc="DEAT1"]
   #[inline] pub fn deat1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="DEAT1"]
   #[inline] pub fn set_deat1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="DEAT0"]
   #[inline] pub fn deat0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="DEAT0"]
   #[inline] pub fn set_deat0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="Driver Enable de-assertion time"]
   #[inline] pub fn dedt4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Driver Enable de-assertion time"]
   #[inline] pub fn set_dedt4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="DEDT3"]
   #[inline] pub fn dedt3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="DEDT3"]
   #[inline] pub fn set_dedt3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="DEDT2"]
   #[inline] pub fn dedt2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="DEDT2"]
   #[inline] pub fn set_dedt2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="DEDT1"]
   #[inline] pub fn dedt1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="DEDT1"]
   #[inline] pub fn set_dedt1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="DEDT0"]
   #[inline] pub fn dedt0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="DEDT0"]
   #[inline] pub fn set_dedt0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Character match interrupt enable"]
   #[inline] pub fn cmie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Character match interrupt enable"]
   #[inline] pub fn set_cmie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Mute mode enable"]
   #[inline] pub fn mme(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Mute mode enable"]
   #[inline] pub fn set_mme<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Word length"]
   #[inline] pub fn m0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Word length"]
   #[inline] pub fn set_m0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Receiver wakeup method"]
   #[inline] pub fn wake(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Receiver wakeup method"]
   #[inline] pub fn set_wake<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Parity control enable"]
   #[inline] pub fn pce(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Parity control enable"]
   #[inline] pub fn set_pce<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Parity selection"]
   #[inline] pub fn ps(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Parity selection"]
   #[inline] pub fn set_ps<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="PE interrupt enable"]
   #[inline] pub fn peie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="PE interrupt enable"]
   #[inline] pub fn set_peie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="interrupt enable"]
   #[inline] pub fn txeie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="interrupt enable"]
   #[inline] pub fn set_txeie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Transmission complete interrupt enable"]
   #[inline] pub fn tcie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Transmission complete interrupt enable"]
   #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="RXNE interrupt enable"]
   #[inline] pub fn rxneie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="RXNE interrupt enable"]
   #[inline] pub fn set_rxneie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="IDLE interrupt enable"]
   #[inline] pub fn idleie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="IDLE interrupt enable"]
   #[inline] pub fn set_idleie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Transmitter enable"]
   #[inline] pub fn te(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Transmitter enable"]
   #[inline] pub fn set_te<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Receiver enable"]
   #[inline] pub fn re(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Receiver enable"]
   #[inline] pub fn set_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="USART enable in Stop mode"]
   #[inline] pub fn uesm(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="USART enable in Stop mode"]
   #[inline] pub fn set_uesm<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="USART enable"]
   #[inline] pub fn ue(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="USART enable"]
   #[inline] pub fn set_ue<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.m1() != 0 { try!(write!(f, " m1"))}
      if self.deat4() != 0 { try!(write!(f, " deat4"))}
      if self.deat3() != 0 { try!(write!(f, " deat3"))}
      if self.deat2() != 0 { try!(write!(f, " deat2"))}
      if self.deat1() != 0 { try!(write!(f, " deat1"))}
      if self.deat0() != 0 { try!(write!(f, " deat0"))}
      if self.dedt4() != 0 { try!(write!(f, " dedt4"))}
      if self.dedt3() != 0 { try!(write!(f, " dedt3"))}
      if self.dedt2() != 0 { try!(write!(f, " dedt2"))}
      if self.dedt1() != 0 { try!(write!(f, " dedt1"))}
      if self.dedt0() != 0 { try!(write!(f, " dedt0"))}
      if self.cmie() != 0 { try!(write!(f, " cmie"))}
      if self.mme() != 0 { try!(write!(f, " mme"))}
      if self.m0() != 0 { try!(write!(f, " m0"))}
      if self.wake() != 0 { try!(write!(f, " wake"))}
      if self.pce() != 0 { try!(write!(f, " pce"))}
      if self.ps() != 0 { try!(write!(f, " ps"))}
      if self.peie() != 0 { try!(write!(f, " peie"))}
      if self.txeie() != 0 { try!(write!(f, " txeie"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.rxneie() != 0 { try!(write!(f, " rxneie"))}
      if self.idleie() != 0 { try!(write!(f, " idleie"))}
      if self.te() != 0 { try!(write!(f, " te"))}
      if self.re() != 0 { try!(write!(f, " re"))}
      if self.uesm() != 0 { try!(write!(f, " uesm"))}
      if self.ue() != 0 { try!(write!(f, " ue"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
#[doc="Address of the USART node"]
   #[inline] pub fn add4_7(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
   }
#[doc="Address of the USART node"]
   #[inline] pub fn set_add4_7<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 28);
      self.0 |= value << 28;
      self
   }

#[doc="Address of the USART node"]
   #[inline] pub fn add0_3(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
   }
#[doc="Address of the USART node"]
   #[inline] pub fn set_add0_3<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Most significant bit first"]
   #[inline] pub fn msbfirst(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="Most significant bit first"]
   #[inline] pub fn set_msbfirst<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="Binary data inversion"]
   #[inline] pub fn tainv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="Binary data inversion"]
   #[inline] pub fn set_tainv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="TX pin active level inversion"]
   #[inline] pub fn txinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="TX pin active level inversion"]
   #[inline] pub fn set_txinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="RX pin active level inversion"]
   #[inline] pub fn rxinv(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="RX pin active level inversion"]
   #[inline] pub fn set_rxinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Swap TX/RX pins"]
   #[inline] pub fn swap(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Swap TX/RX pins"]
   #[inline] pub fn set_swap<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="STOP bits"]
   #[inline] pub fn stop(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
   }
#[doc="STOP bits"]
   #[inline] pub fn set_stop<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Clock enable"]
   #[inline] pub fn clken(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Clock enable"]
   #[inline] pub fn set_clken<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="7-bit Address Detection/4-bit Address Detection"]
   #[inline] pub fn addm7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="7-bit Address Detection/4-bit Address Detection"]
   #[inline] pub fn set_addm7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Cr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.add4_7() != 0 { try!(write!(f, " add4_7=0x{:x}", self.add4_7()))}
      if self.add0_3() != 0 { try!(write!(f, " add0_3=0x{:x}", self.add0_3()))}
      if self.msbfirst() != 0 { try!(write!(f, " msbfirst"))}
      if self.tainv() != 0 { try!(write!(f, " tainv"))}
      if self.txinv() != 0 { try!(write!(f, " txinv"))}
      if self.rxinv() != 0 { try!(write!(f, " rxinv"))}
      if self.swap() != 0 { try!(write!(f, " swap"))}
      if self.stop() != 0 { try!(write!(f, " stop=0x{:x}", self.stop()))}
      if self.clken() != 0 { try!(write!(f, " clken"))}
      if self.addm7() != 0 { try!(write!(f, " addm7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control register 3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr3(pub u32);
impl Cr3 {
#[doc="Wakeup from Stop mode interrupt enable"]
   #[inline] pub fn wufie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="Wakeup from Stop mode interrupt enable"]
   #[inline] pub fn set_wufie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="Wakeup from Stop mode interrupt flag selection"]
   #[inline] pub fn wus(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
   }
#[doc="Wakeup from Stop mode interrupt flag selection"]
   #[inline] pub fn set_wus<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="Driver enable polarity selection"]
   #[inline] pub fn dep(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Driver enable polarity selection"]
   #[inline] pub fn set_dep<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Driver enable mode"]
   #[inline] pub fn dem(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Driver enable mode"]
   #[inline] pub fn set_dem<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="DMA Disable on Reception Error"]
   #[inline] pub fn ddre(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="DMA Disable on Reception Error"]
   #[inline] pub fn set_ddre<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Overrun Disable"]
   #[inline] pub fn ovrdis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Overrun Disable"]
   #[inline] pub fn set_ovrdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="CTS interrupt enable"]
   #[inline] pub fn ctsie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="CTS interrupt enable"]
   #[inline] pub fn set_ctsie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="CTS enable"]
   #[inline] pub fn ctse(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="CTS enable"]
   #[inline] pub fn set_ctse<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="RTS enable"]
   #[inline] pub fn rtse(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="RTS enable"]
   #[inline] pub fn set_rtse<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="DMA enable transmitter"]
   #[inline] pub fn dmat(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="DMA enable transmitter"]
   #[inline] pub fn set_dmat<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="DMA enable receiver"]
   #[inline] pub fn dmar(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="DMA enable receiver"]
   #[inline] pub fn set_dmar<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Half-duplex selection"]
   #[inline] pub fn hdsel(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Half-duplex selection"]
   #[inline] pub fn set_hdsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Error interrupt enable"]
   #[inline] pub fn eie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Error interrupt enable"]
   #[inline] pub fn set_eie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wufie() != 0 { try!(write!(f, " wufie"))}
      if self.wus() != 0 { try!(write!(f, " wus=0x{:x}", self.wus()))}
      if self.dep() != 0 { try!(write!(f, " dep"))}
      if self.dem() != 0 { try!(write!(f, " dem"))}
      if self.ddre() != 0 { try!(write!(f, " ddre"))}
      if self.ovrdis() != 0 { try!(write!(f, " ovrdis"))}
      if self.ctsie() != 0 { try!(write!(f, " ctsie"))}
      if self.ctse() != 0 { try!(write!(f, " ctse"))}
      if self.rtse() != 0 { try!(write!(f, " rtse"))}
      if self.dmat() != 0 { try!(write!(f, " dmat"))}
      if self.dmar() != 0 { try!(write!(f, " dmar"))}
      if self.hdsel() != 0 { try!(write!(f, " hdsel"))}
      if self.eie() != 0 { try!(write!(f, " eie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Baud rate register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Brr(pub u32);
impl Brr {
#[doc="BRR"]
   #[inline] pub fn brr(&self) -> bits::U20 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfffff) as u32) } // [19:0]
   }
#[doc="BRR"]
   #[inline] pub fn set_brr<V: Into<bits::U20>>(mut self, value: V) -> Self {
      let value: bits::U20 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Brr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Brr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.brr() != 0 { try!(write!(f, " brr=0x{:x}", self.brr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Request register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rqr(pub u32);
impl Rqr {
#[doc="Receive data flush request"]
   #[inline] pub fn rxfrq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Receive data flush request"]
   #[inline] pub fn set_rxfrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Mute mode request"]
   #[inline] pub fn mmrq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Mute mode request"]
   #[inline] pub fn set_mmrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Send break request"]
   #[inline] pub fn sbkrq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Send break request"]
   #[inline] pub fn set_sbkrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Rqr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rqr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxfrq() != 0 { try!(write!(f, " rxfrq"))}
      if self.mmrq() != 0 { try!(write!(f, " mmrq"))}
      if self.sbkrq() != 0 { try!(write!(f, " sbkrq"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt & status register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
#[doc="REACK"]
   #[inline] pub fn reack(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="REACK"]
   #[inline] pub fn set_reack<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="TEACK"]
   #[inline] pub fn teack(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="TEACK"]
   #[inline] pub fn set_teack<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="WUF"]
   #[inline] pub fn wuf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="WUF"]
   #[inline] pub fn set_wuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="RWU"]
   #[inline] pub fn rwu(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="RWU"]
   #[inline] pub fn set_rwu<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="SBKF"]
   #[inline] pub fn sbkf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="SBKF"]
   #[inline] pub fn set_sbkf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="CMF"]
   #[inline] pub fn cmf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="CMF"]
   #[inline] pub fn set_cmf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="BUSY"]
   #[inline] pub fn busy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="BUSY"]
   #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="CTS"]
   #[inline] pub fn cts(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="CTS"]
   #[inline] pub fn set_cts<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="CTSIF"]
   #[inline] pub fn ctsif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="CTSIF"]
   #[inline] pub fn set_ctsif<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="TXE"]
   #[inline] pub fn txe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="TXE"]
   #[inline] pub fn set_txe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="TC"]
   #[inline] pub fn tc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="TC"]
   #[inline] pub fn set_tc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="RXNE"]
   #[inline] pub fn rxne(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="RXNE"]
   #[inline] pub fn set_rxne<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="IDLE"]
   #[inline] pub fn idle(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="IDLE"]
   #[inline] pub fn set_idle<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="ORE"]
   #[inline] pub fn ore(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="ORE"]
   #[inline] pub fn set_ore<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="NF"]
   #[inline] pub fn nf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="NF"]
   #[inline] pub fn set_nf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="FE"]
   #[inline] pub fn fe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="FE"]
   #[inline] pub fn set_fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="PE"]
   #[inline] pub fn pe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PE"]
   #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.reack() != 0 { try!(write!(f, " reack"))}
      if self.teack() != 0 { try!(write!(f, " teack"))}
      if self.wuf() != 0 { try!(write!(f, " wuf"))}
      if self.rwu() != 0 { try!(write!(f, " rwu"))}
      if self.sbkf() != 0 { try!(write!(f, " sbkf"))}
      if self.cmf() != 0 { try!(write!(f, " cmf"))}
      if self.busy() != 0 { try!(write!(f, " busy"))}
      if self.cts() != 0 { try!(write!(f, " cts"))}
      if self.ctsif() != 0 { try!(write!(f, " ctsif"))}
      if self.txe() != 0 { try!(write!(f, " txe"))}
      if self.tc() != 0 { try!(write!(f, " tc"))}
      if self.rxne() != 0 { try!(write!(f, " rxne"))}
      if self.idle() != 0 { try!(write!(f, " idle"))}
      if self.ore() != 0 { try!(write!(f, " ore"))}
      if self.nf() != 0 { try!(write!(f, " nf"))}
      if self.fe() != 0 { try!(write!(f, " fe"))}
      if self.pe() != 0 { try!(write!(f, " pe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt flag clear register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
#[doc="Wakeup from Stop mode clear flag"]
   #[inline] pub fn wucf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
   }
#[doc="Wakeup from Stop mode clear flag"]
   #[inline] pub fn set_wucf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="Character match clear flag"]
   #[inline] pub fn cmcf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Character match clear flag"]
   #[inline] pub fn set_cmcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="CTS clear flag"]
   #[inline] pub fn ctscf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="CTS clear flag"]
   #[inline] pub fn set_ctscf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Transmission complete clear flag"]
   #[inline] pub fn tccf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Transmission complete clear flag"]
   #[inline] pub fn set_tccf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Idle line detected clear flag"]
   #[inline] pub fn idlecf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Idle line detected clear flag"]
   #[inline] pub fn set_idlecf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Overrun error clear flag"]
   #[inline] pub fn orecf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Overrun error clear flag"]
   #[inline] pub fn set_orecf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Noise detected clear flag"]
   #[inline] pub fn ncf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Noise detected clear flag"]
   #[inline] pub fn set_ncf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Framing error clear flag"]
   #[inline] pub fn fecf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Framing error clear flag"]
   #[inline] pub fn set_fecf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Parity error clear flag"]
   #[inline] pub fn pecf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Parity error clear flag"]
   #[inline] pub fn set_pecf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wucf() != 0 { try!(write!(f, " wucf"))}
      if self.cmcf() != 0 { try!(write!(f, " cmcf"))}
      if self.ctscf() != 0 { try!(write!(f, " ctscf"))}
      if self.tccf() != 0 { try!(write!(f, " tccf"))}
      if self.idlecf() != 0 { try!(write!(f, " idlecf"))}
      if self.orecf() != 0 { try!(write!(f, " orecf"))}
      if self.ncf() != 0 { try!(write!(f, " ncf"))}
      if self.fecf() != 0 { try!(write!(f, " fecf"))}
      if self.pecf() != 0 { try!(write!(f, " pecf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Receive data register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rdr(pub u32);
impl Rdr {
#[doc="Receive data value"]
   #[inline] pub fn rdr(&self) -> bits::U9 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
   }
#[doc="Receive data value"]
   #[inline] pub fn set_rdr<V: Into<bits::U9>>(mut self, value: V) -> Self {
      let value: bits::U9 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1ff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rdr() != 0 { try!(write!(f, " rdr=0x{:x}", self.rdr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Transmit data register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tdr(pub u32);
impl Tdr {
#[doc="Transmit data value"]
   #[inline] pub fn tdr(&self) -> bits::U9 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
   }
#[doc="Transmit data value"]
   #[inline] pub fn set_tdr<V: Into<bits::U9>>(mut self, value: V) -> Self {
      let value: bits::U9 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1ff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Tdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tdr() != 0 { try!(write!(f, " tdr=0x{:x}", self.tdr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
