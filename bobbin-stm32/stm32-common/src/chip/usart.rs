
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USART Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the CR1 register."]
  #[inline] pub fn cr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CR1 register."]
  #[inline] pub fn cr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CR1 register."]
  #[inline] pub fn cr1(&self) -> Cr1 { 
     unsafe {
        Cr1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CR1 register."]
  #[inline] pub fn set_cr1(&self, value: Cr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR1 register."]
  #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
     let tmp = self.cr1();
     self.set_cr1(f(tmp))
  }

#[doc="Get the *const pointer for the CR2 register."]
  #[inline] pub fn cr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CR2 register."]
  #[inline] pub fn cr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CR2 register."]
  #[inline] pub fn cr2(&self) -> Cr2 { 
     unsafe {
        Cr2(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CR2 register."]
  #[inline] pub fn set_cr2(&self, value: Cr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR2 register."]
  #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
     let tmp = self.cr2();
     self.set_cr2(f(tmp))
  }

#[doc="Get the *const pointer for the CR3 register."]
  #[inline] pub fn cr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the CR3 register."]
  #[inline] pub fn cr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the CR3 register."]
  #[inline] pub fn cr3(&self) -> Cr3 { 
     unsafe {
        Cr3(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the CR3 register."]
  #[inline] pub fn set_cr3(&self, value: Cr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR3 register."]
  #[inline] pub fn with_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
     let tmp = self.cr3();
     self.set_cr3(f(tmp))
  }

#[doc="Get the *const pointer for the BRR register."]
  #[inline] pub fn brr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the BRR register."]
  #[inline] pub fn brr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the BRR register."]
  #[inline] pub fn brr(&self) -> Brr { 
     unsafe {
        Brr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the BRR register."]
  #[inline] pub fn set_brr(&self, value: Brr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BRR register."]
  #[inline] pub fn with_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
     let tmp = self.brr();
     self.set_brr(f(tmp))
  }

#[doc="Get the *const pointer for the GTPR register."]
  #[inline] pub fn gtpr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the GTPR register."]
  #[inline] pub fn gtpr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the GTPR register."]
  #[inline] pub fn gtpr(&self) -> Gtpr { 
     unsafe {
        Gtpr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the GTPR register."]
  #[inline] pub fn set_gtpr(&self, value: Gtpr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the GTPR register."]
  #[inline] pub fn with_gtpr<F: FnOnce(Gtpr) -> Gtpr>(&self, f: F) -> &Self {
     let tmp = self.gtpr();
     self.set_gtpr(f(tmp))
  }

#[doc="Get the *const pointer for the RTOR register."]
  #[inline] pub fn rtor_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the RTOR register."]
  #[inline] pub fn rtor_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the RTOR register."]
  #[inline] pub fn rtor(&self) -> Rtor { 
     unsafe {
        Rtor(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the RTOR register."]
  #[inline] pub fn set_rtor(&self, value: Rtor) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RTOR register."]
  #[inline] pub fn with_rtor<F: FnOnce(Rtor) -> Rtor>(&self, f: F) -> &Self {
     let tmp = self.rtor();
     self.set_rtor(f(tmp))
  }

#[doc="Get the *const pointer for the RQR register."]
  #[inline] pub fn rqr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the RQR register."]
  #[inline] pub fn rqr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the RQR register."]
  #[inline] pub fn rqr(&self) -> Rqr { 
     unsafe {
        Rqr(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the RQR register."]
  #[inline] pub fn set_rqr(&self, value: Rqr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RQR register."]
  #[inline] pub fn with_rqr<F: FnOnce(Rqr) -> Rqr>(&self, f: F) -> &Self {
     let tmp = self.rqr();
     self.set_rqr(f(tmp))
  }

#[doc="Get the *const pointer for the ISR register."]
  #[inline] pub fn isr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the ISR register."]
  #[inline] pub fn isr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the ISR register."]
  #[inline] pub fn isr(&self) -> Isr { 
     unsafe {
        Isr(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the ICR register."]
  #[inline] pub fn icr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the ICR register."]
  #[inline] pub fn icr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the ICR register."]
  #[inline] pub fn icr(&self) -> Icr { 
     unsafe {
        Icr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the ICR register."]
  #[inline] pub fn set_icr(&self, value: Icr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ICR register."]
  #[inline] pub fn with_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
     let tmp = self.icr();
     self.set_icr(f(tmp))
  }

#[doc="Get the *const pointer for the RDR register."]
  #[inline] pub fn rdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the RDR register."]
  #[inline] pub fn rdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the RDR register."]
  #[inline] pub fn rdr(&self) -> Rdr { 
     unsafe {
        Rdr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }

#[doc="Get the *const pointer for the TDR register."]
  #[inline] pub fn tdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the TDR register."]
  #[inline] pub fn tdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the TDR register."]
  #[inline] pub fn tdr(&self) -> Tdr { 
     unsafe {
        Tdr(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
#[doc="Write the TDR register."]
  #[inline] pub fn set_tdr(&self, value: Tdr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TDR register."]
  #[inline] pub fn with_tdr<F: FnOnce(Tdr) -> Tdr>(&self, f: F) -> &Self {
     let tmp = self.tdr();
     self.set_tdr(f(tmp))
  }

}

#[doc="Control register 1"]
#[derive(PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
#[doc="End of Block interrupt enable"]
  #[inline] pub fn eobie(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
#[doc="End of Block interrupt enable"]
  #[inline] pub fn set_eobie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Receiver timeout interrupt enable"]
  #[inline] pub fn rtoie(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="Receiver timeout interrupt enable"]
  #[inline] pub fn set_rtoie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Driver Enable assertion time"]
  #[inline] pub fn deat(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1f // [25:21]
  }
#[doc="Driver Enable assertion time"]
  #[inline] pub fn set_deat(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Driver Enable deassertion time"]
  #[inline] pub fn dedt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1f // [20:16]
  }
#[doc="Driver Enable deassertion time"]
  #[inline] pub fn set_dedt(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Oversampling mode"]
  #[inline] pub fn over8(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Oversampling mode"]
  #[inline] pub fn set_over8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Character match interrupt enable"]
  #[inline] pub fn cmie(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="Character match interrupt enable"]
  #[inline] pub fn set_cmie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Mute mode enable"]
  #[inline] pub fn mme(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Mute mode enable"]
  #[inline] pub fn set_mme(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Word length"]
  #[inline] pub fn m(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Word length"]
  #[inline] pub fn set_m(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Receiver wakeup method"]
  #[inline] pub fn wake(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Receiver wakeup method"]
  #[inline] pub fn set_wake(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Parity control enable"]
  #[inline] pub fn pce(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Parity control enable"]
  #[inline] pub fn set_pce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Parity selection"]
  #[inline] pub fn ps(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Parity selection"]
  #[inline] pub fn set_ps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="PE interrupt enable"]
  #[inline] pub fn peie(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="PE interrupt enable"]
  #[inline] pub fn set_peie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="interrupt enable"]
  #[inline] pub fn txeie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="interrupt enable"]
  #[inline] pub fn set_txeie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Transmission complete interrupt enable"]
  #[inline] pub fn tcie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Transmission complete interrupt enable"]
  #[inline] pub fn set_tcie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="RXNE interrupt enable"]
  #[inline] pub fn rxneie(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="RXNE interrupt enable"]
  #[inline] pub fn set_rxneie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="IDLE interrupt enable"]
  #[inline] pub fn idleie(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="IDLE interrupt enable"]
  #[inline] pub fn set_idleie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Transmitter enable"]
  #[inline] pub fn te(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Transmitter enable"]
  #[inline] pub fn set_te(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Receiver enable"]
  #[inline] pub fn re(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Receiver enable"]
  #[inline] pub fn set_re(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="USART enable in Stop mode"]
  #[inline] pub fn uesm(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="USART enable in Stop mode"]
  #[inline] pub fn set_uesm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="USART enable"]
  #[inline] pub fn ue(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="USART enable"]
  #[inline] pub fn set_ue(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
      if self.eobie() != 0 { try!(write!(f, " eobie"))}
      if self.rtoie() != 0 { try!(write!(f, " rtoie"))}
      if self.deat() != 0 { try!(write!(f, " deat=0x{:x}", self.deat()))}
      if self.dedt() != 0 { try!(write!(f, " dedt=0x{:x}", self.dedt()))}
      if self.over8() != 0 { try!(write!(f, " over8"))}
      if self.cmie() != 0 { try!(write!(f, " cmie"))}
      if self.mme() != 0 { try!(write!(f, " mme"))}
      if self.m() != 0 { try!(write!(f, " m"))}
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
#[derive(PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
#[doc="Address of the USART node"]
  #[inline] pub fn add4(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
#[doc="Address of the USART node"]
  #[inline] pub fn set_add4(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Address of the USART node"]
  #[inline] pub fn add0(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
#[doc="Address of the USART node"]
  #[inline] pub fn set_add0(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Receiver timeout enable"]
  #[inline] pub fn rtoen(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="Receiver timeout enable"]
  #[inline] pub fn set_rtoen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="Auto baud rate mode"]
  #[inline] pub fn abrmod(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x3 // [22:21]
  }
#[doc="Auto baud rate mode"]
  #[inline] pub fn set_abrmod(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Auto baud rate enable"]
  #[inline] pub fn abren(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="Auto baud rate enable"]
  #[inline] pub fn set_abren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Most significant bit first"]
  #[inline] pub fn msbfirst(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="Most significant bit first"]
  #[inline] pub fn set_msbfirst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Binary data inversion"]
  #[inline] pub fn datainv(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Binary data inversion"]
  #[inline] pub fn set_datainv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="TX pin active level inversion"]
  #[inline] pub fn txinv(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="TX pin active level inversion"]
  #[inline] pub fn set_txinv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="RX pin active level inversion"]
  #[inline] pub fn rxinv(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="RX pin active level inversion"]
  #[inline] pub fn set_rxinv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Swap TX/RX pins"]
  #[inline] pub fn swap(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Swap TX/RX pins"]
  #[inline] pub fn set_swap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="LIN mode enable"]
  #[inline] pub fn linen(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="LIN mode enable"]
  #[inline] pub fn set_linen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="STOP bits"]
  #[inline] pub fn stop(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
#[doc="STOP bits"]
  #[inline] pub fn set_stop(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Clock enable"]
  #[inline] pub fn clken(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Clock enable"]
  #[inline] pub fn set_clken(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Clock polarity"]
  #[inline] pub fn cpol(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Clock polarity"]
  #[inline] pub fn set_cpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Clock phase"]
  #[inline] pub fn cpha(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Clock phase"]
  #[inline] pub fn set_cpha(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Last bit clock pulse"]
  #[inline] pub fn lbcl(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Last bit clock pulse"]
  #[inline] pub fn set_lbcl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="LIN break detection interrupt enable"]
  #[inline] pub fn lbdie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="LIN break detection interrupt enable"]
  #[inline] pub fn set_lbdie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="LIN break detection length"]
  #[inline] pub fn lbdl(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="LIN break detection length"]
  #[inline] pub fn set_lbdl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="7-bit Address Detection/4-bit Address Detection"]
  #[inline] pub fn addm7(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="7-bit Address Detection/4-bit Address Detection"]
  #[inline] pub fn set_addm7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
      if self.add4() != 0 { try!(write!(f, " add4=0x{:x}", self.add4()))}
      if self.add0() != 0 { try!(write!(f, " add0=0x{:x}", self.add0()))}
      if self.rtoen() != 0 { try!(write!(f, " rtoen"))}
      if self.abrmod() != 0 { try!(write!(f, " abrmod=0x{:x}", self.abrmod()))}
      if self.abren() != 0 { try!(write!(f, " abren"))}
      if self.msbfirst() != 0 { try!(write!(f, " msbfirst"))}
      if self.datainv() != 0 { try!(write!(f, " datainv"))}
      if self.txinv() != 0 { try!(write!(f, " txinv"))}
      if self.rxinv() != 0 { try!(write!(f, " rxinv"))}
      if self.swap() != 0 { try!(write!(f, " swap"))}
      if self.linen() != 0 { try!(write!(f, " linen"))}
      if self.stop() != 0 { try!(write!(f, " stop=0x{:x}", self.stop()))}
      if self.clken() != 0 { try!(write!(f, " clken"))}
      if self.cpol() != 0 { try!(write!(f, " cpol"))}
      if self.cpha() != 0 { try!(write!(f, " cpha"))}
      if self.lbcl() != 0 { try!(write!(f, " lbcl"))}
      if self.lbdie() != 0 { try!(write!(f, " lbdie"))}
      if self.lbdl() != 0 { try!(write!(f, " lbdl"))}
      if self.addm7() != 0 { try!(write!(f, " addm7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control register 3"]
#[derive(PartialEq, Eq)]
pub struct Cr3(pub u32);
impl Cr3 {
#[doc="Wakeup from Stop mode interrupt enable"]
  #[inline] pub fn wufie(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="Wakeup from Stop mode interrupt enable"]
  #[inline] pub fn set_wufie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Wakeup from Stop mode interrupt flag selection"]
  #[inline] pub fn wus(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="Wakeup from Stop mode interrupt flag selection"]
  #[inline] pub fn set_wus(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Smartcard auto-retry count"]
  #[inline] pub fn scarcnt(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7 // [19:17]
  }
#[doc="Smartcard auto-retry count"]
  #[inline] pub fn set_scarcnt(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Driver enable polarity selection"]
  #[inline] pub fn dep(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Driver enable polarity selection"]
  #[inline] pub fn set_dep(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Driver enable mode"]
  #[inline] pub fn dem(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="Driver enable mode"]
  #[inline] pub fn set_dem(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="DMA Disable on Reception Error"]
  #[inline] pub fn ddre(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="DMA Disable on Reception Error"]
  #[inline] pub fn set_ddre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Overrun Disable"]
  #[inline] pub fn ovrdis(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Overrun Disable"]
  #[inline] pub fn set_ovrdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="One sample bit method enable"]
  #[inline] pub fn onebit(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="One sample bit method enable"]
  #[inline] pub fn set_onebit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="CTS interrupt enable"]
  #[inline] pub fn ctsie(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="CTS interrupt enable"]
  #[inline] pub fn set_ctsie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="CTS enable"]
  #[inline] pub fn ctse(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="CTS enable"]
  #[inline] pub fn set_ctse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="RTS enable"]
  #[inline] pub fn rtse(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="RTS enable"]
  #[inline] pub fn set_rtse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="DMA enable transmitter"]
  #[inline] pub fn dmat(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="DMA enable transmitter"]
  #[inline] pub fn set_dmat(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="DMA enable receiver"]
  #[inline] pub fn dmar(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="DMA enable receiver"]
  #[inline] pub fn set_dmar(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Smartcard mode enable"]
  #[inline] pub fn scen(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Smartcard mode enable"]
  #[inline] pub fn set_scen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Smartcard NACK enable"]
  #[inline] pub fn nack(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Smartcard NACK enable"]
  #[inline] pub fn set_nack(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Half-duplex selection"]
  #[inline] pub fn hdsel(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Half-duplex selection"]
  #[inline] pub fn set_hdsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="IrDA low-power"]
  #[inline] pub fn irlp(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="IrDA low-power"]
  #[inline] pub fn set_irlp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="IrDA mode enable"]
  #[inline] pub fn iren(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="IrDA mode enable"]
  #[inline] pub fn set_iren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Error interrupt enable"]
  #[inline] pub fn eie(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Error interrupt enable"]
  #[inline] pub fn set_eie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
      if self.scarcnt() != 0 { try!(write!(f, " scarcnt=0x{:x}", self.scarcnt()))}
      if self.dep() != 0 { try!(write!(f, " dep"))}
      if self.dem() != 0 { try!(write!(f, " dem"))}
      if self.ddre() != 0 { try!(write!(f, " ddre"))}
      if self.ovrdis() != 0 { try!(write!(f, " ovrdis"))}
      if self.onebit() != 0 { try!(write!(f, " onebit"))}
      if self.ctsie() != 0 { try!(write!(f, " ctsie"))}
      if self.ctse() != 0 { try!(write!(f, " ctse"))}
      if self.rtse() != 0 { try!(write!(f, " rtse"))}
      if self.dmat() != 0 { try!(write!(f, " dmat"))}
      if self.dmar() != 0 { try!(write!(f, " dmar"))}
      if self.scen() != 0 { try!(write!(f, " scen"))}
      if self.nack() != 0 { try!(write!(f, " nack"))}
      if self.hdsel() != 0 { try!(write!(f, " hdsel"))}
      if self.irlp() != 0 { try!(write!(f, " irlp"))}
      if self.iren() != 0 { try!(write!(f, " iren"))}
      if self.eie() != 0 { try!(write!(f, " eie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Baud rate register"]
#[derive(PartialEq, Eq)]
pub struct Brr(pub u32);
impl Brr {
#[doc="mantissa of USARTDIV"]
  #[inline] pub fn div_mantissa(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xfff // [15:4]
  }
#[doc="mantissa of USARTDIV"]
  #[inline] pub fn set_div_mantissa(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 4);
     self.0 |= value << 4;
     self
  }

#[doc="fraction of USARTDIV"]
  #[inline] pub fn div_fraction(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="fraction of USARTDIV"]
  #[inline] pub fn set_div_fraction(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
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
      if self.div_mantissa() != 0 { try!(write!(f, " div_mantissa=0x{:x}", self.div_mantissa()))}
      if self.div_fraction() != 0 { try!(write!(f, " div_fraction=0x{:x}", self.div_fraction()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Guard time and prescaler register"]
#[derive(PartialEq, Eq)]
pub struct Gtpr(pub u32);
impl Gtpr {
#[doc="Guard time value"]
  #[inline] pub fn gt(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
#[doc="Guard time value"]
  #[inline] pub fn set_gt(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Prescaler value"]
  #[inline] pub fn psc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Prescaler value"]
  #[inline] pub fn set_psc(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Gtpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Gtpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gt() != 0 { try!(write!(f, " gt=0x{:x}", self.gt()))}
      if self.psc() != 0 { try!(write!(f, " psc=0x{:x}", self.psc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Receiver timeout register"]
#[derive(PartialEq, Eq)]
pub struct Rtor(pub u32);
impl Rtor {
#[doc="Block Length"]
  #[inline] pub fn blen(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
#[doc="Block Length"]
  #[inline] pub fn set_blen(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Receiver timeout value"]
  #[inline] pub fn rto(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffff // [23:0]
  }
#[doc="Receiver timeout value"]
  #[inline] pub fn set_rto(mut self, value: u32) -> Self {
     assert!((value & !0xffffff) == 0);
     self.0 &= !(0xffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Rtor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rtor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.blen() != 0 { try!(write!(f, " blen=0x{:x}", self.blen()))}
      if self.rto() != 0 { try!(write!(f, " rto=0x{:x}", self.rto()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Request register"]
#[derive(PartialEq, Eq)]
pub struct Rqr(pub u32);
impl Rqr {
#[doc="Transmit data flush request"]
  #[inline] pub fn txfrq(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Transmit data flush request"]
  #[inline] pub fn set_txfrq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Receive data flush request"]
  #[inline] pub fn rxfrq(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Receive data flush request"]
  #[inline] pub fn set_rxfrq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Mute mode request"]
  #[inline] pub fn mmrq(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Mute mode request"]
  #[inline] pub fn set_mmrq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Send break request"]
  #[inline] pub fn sbkrq(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Send break request"]
  #[inline] pub fn set_sbkrq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Auto baud rate request"]
  #[inline] pub fn abrrq(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Auto baud rate request"]
  #[inline] pub fn set_abrrq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.txfrq() != 0 { try!(write!(f, " txfrq"))}
      if self.rxfrq() != 0 { try!(write!(f, " rxfrq"))}
      if self.mmrq() != 0 { try!(write!(f, " mmrq"))}
      if self.sbkrq() != 0 { try!(write!(f, " sbkrq"))}
      if self.abrrq() != 0 { try!(write!(f, " abrrq"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt & status register"]
#[derive(PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
#[doc="Receive enable acknowledge flag"]
  #[inline] pub fn reack(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="Receive enable acknowledge flag"]
  #[inline] pub fn set_reack(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Transmit enable acknowledge flag"]
  #[inline] pub fn teack(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="Transmit enable acknowledge flag"]
  #[inline] pub fn set_teack(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Wakeup from Stop mode flag"]
  #[inline] pub fn wuf(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="Wakeup from Stop mode flag"]
  #[inline] pub fn set_wuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Receiver wakeup from Mute mode"]
  #[inline] pub fn rwu(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="Receiver wakeup from Mute mode"]
  #[inline] pub fn set_rwu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Send break flag"]
  #[inline] pub fn sbkf(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Send break flag"]
  #[inline] pub fn set_sbkf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="character match flag"]
  #[inline] pub fn cmf(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="character match flag"]
  #[inline] pub fn set_cmf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Busy flag"]
  #[inline] pub fn busy(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Busy flag"]
  #[inline] pub fn set_busy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Auto baud rate flag"]
  #[inline] pub fn abrf(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Auto baud rate flag"]
  #[inline] pub fn set_abrf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Auto baud rate error"]
  #[inline] pub fn abre(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="Auto baud rate error"]
  #[inline] pub fn set_abre(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="End of block flag"]
  #[inline] pub fn eobf(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="End of block flag"]
  #[inline] pub fn set_eobf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Receiver timeout"]
  #[inline] pub fn rtof(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Receiver timeout"]
  #[inline] pub fn set_rtof(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="CTS flag"]
  #[inline] pub fn cts(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="CTS flag"]
  #[inline] pub fn set_cts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="CTS interrupt flag"]
  #[inline] pub fn ctsif(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="CTS interrupt flag"]
  #[inline] pub fn set_ctsif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="LIN break detection flag"]
  #[inline] pub fn lbdf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="LIN break detection flag"]
  #[inline] pub fn set_lbdf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Transmit data register empty"]
  #[inline] pub fn txe(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Transmit data register empty"]
  #[inline] pub fn set_txe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Transmission complete"]
  #[inline] pub fn tc(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Transmission complete"]
  #[inline] pub fn set_tc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Read data register not empty"]
  #[inline] pub fn rxne(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Read data register not empty"]
  #[inline] pub fn set_rxne(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Idle line detected"]
  #[inline] pub fn idle(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Idle line detected"]
  #[inline] pub fn set_idle(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Overrun error"]
  #[inline] pub fn ore(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Overrun error"]
  #[inline] pub fn set_ore(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Noise detected flag"]
  #[inline] pub fn nf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Noise detected flag"]
  #[inline] pub fn set_nf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Framing error"]
  #[inline] pub fn fe(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Framing error"]
  #[inline] pub fn set_fe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Parity error"]
  #[inline] pub fn pe(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Parity error"]
  #[inline] pub fn set_pe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
      if self.abrf() != 0 { try!(write!(f, " abrf"))}
      if self.abre() != 0 { try!(write!(f, " abre"))}
      if self.eobf() != 0 { try!(write!(f, " eobf"))}
      if self.rtof() != 0 { try!(write!(f, " rtof"))}
      if self.cts() != 0 { try!(write!(f, " cts"))}
      if self.ctsif() != 0 { try!(write!(f, " ctsif"))}
      if self.lbdf() != 0 { try!(write!(f, " lbdf"))}
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
#[derive(PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
#[doc="Wakeup from Stop mode clear flag"]
  #[inline] pub fn wucf(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="Wakeup from Stop mode clear flag"]
  #[inline] pub fn set_wucf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Character match clear flag"]
  #[inline] pub fn cmcf(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="Character match clear flag"]
  #[inline] pub fn set_cmcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="End of timeout clear flag"]
  #[inline] pub fn eobcf(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="End of timeout clear flag"]
  #[inline] pub fn set_eobcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Receiver timeout clear flag"]
  #[inline] pub fn rtocf(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Receiver timeout clear flag"]
  #[inline] pub fn set_rtocf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="CTS clear flag"]
  #[inline] pub fn ctscf(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="CTS clear flag"]
  #[inline] pub fn set_ctscf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="LIN break detection clear flag"]
  #[inline] pub fn lbdcf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="LIN break detection clear flag"]
  #[inline] pub fn set_lbdcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Transmission complete clear flag"]
  #[inline] pub fn tccf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Transmission complete clear flag"]
  #[inline] pub fn set_tccf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Idle line detected clear flag"]
  #[inline] pub fn idlecf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Idle line detected clear flag"]
  #[inline] pub fn set_idlecf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Overrun error clear flag"]
  #[inline] pub fn orecf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Overrun error clear flag"]
  #[inline] pub fn set_orecf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Noise detected clear flag"]
  #[inline] pub fn ncf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Noise detected clear flag"]
  #[inline] pub fn set_ncf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Framing error clear flag"]
  #[inline] pub fn fecf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Framing error clear flag"]
  #[inline] pub fn set_fecf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Parity error clear flag"]
  #[inline] pub fn pecf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Parity error clear flag"]
  #[inline] pub fn set_pecf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
      if self.eobcf() != 0 { try!(write!(f, " eobcf"))}
      if self.rtocf() != 0 { try!(write!(f, " rtocf"))}
      if self.ctscf() != 0 { try!(write!(f, " ctscf"))}
      if self.lbdcf() != 0 { try!(write!(f, " lbdcf"))}
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
#[derive(PartialEq, Eq)]
pub struct Rdr(pub u32);
impl Rdr {
#[doc="Receive data value"]
  #[inline] pub fn rdr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1ff // [8:0]
  }
#[doc="Receive data value"]
  #[inline] pub fn set_rdr(mut self, value: u32) -> Self {
     assert!((value & !0x1ff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Tdr(pub u32);
impl Tdr {
#[doc="Transmit data value"]
  #[inline] pub fn tdr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1ff // [8:0]
  }
#[doc="Transmit data value"]
  #[inline] pub fn set_tdr(mut self, value: u32) -> Self {
     assert!((value & !0x1ff) == 0);
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
