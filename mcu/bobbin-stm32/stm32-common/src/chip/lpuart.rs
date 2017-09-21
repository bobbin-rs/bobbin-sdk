#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPUART Peripheral"]
pub struct LpuartPeriph(pub usize); 


impl LpuartPeriph {
    #[doc="Get the *mut pointer for the CR1 register."]
    #[inline] pub fn cr1_mut(&self) -> *mut Cr1 { 
        (self.0 + 0x0) as *mut Cr1
    }

    #[doc="Get the *const pointer for the CR1 register."]
    #[inline] pub fn cr1_ptr(&self) -> *const Cr1 { 
           self.cr1_mut()
    }

    #[doc="Read the CR1 register."]
    #[inline] pub fn cr1(&self) -> Cr1 { 
        unsafe {
            read_volatile(self.cr1_ptr())
        }
    }

    #[doc="Write the CR1 register."]
    #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr1_mut(), f(Cr1(0)));
        }
        self
    }

    #[doc="Modify the CR1 register."]
    #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr1_mut(), f(self.cr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR2 register."]
    #[inline] pub fn cr2_mut(&self) -> *mut Cr2 { 
        (self.0 + 0x4) as *mut Cr2
    }

    #[doc="Get the *const pointer for the CR2 register."]
    #[inline] pub fn cr2_ptr(&self) -> *const Cr2 { 
           self.cr2_mut()
    }

    #[doc="Read the CR2 register."]
    #[inline] pub fn cr2(&self) -> Cr2 { 
        unsafe {
            read_volatile(self.cr2_ptr())
        }
    }

    #[doc="Write the CR2 register."]
    #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr2_mut(), f(Cr2(0)));
        }
        self
    }

    #[doc="Modify the CR2 register."]
    #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr2_mut(), f(self.cr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR3 register."]
    #[inline] pub fn cr3_mut(&self) -> *mut Cr3 { 
        (self.0 + 0x8) as *mut Cr3
    }

    #[doc="Get the *const pointer for the CR3 register."]
    #[inline] pub fn cr3_ptr(&self) -> *const Cr3 { 
           self.cr3_mut()
    }

    #[doc="Read the CR3 register."]
    #[inline] pub fn cr3(&self) -> Cr3 { 
        unsafe {
            read_volatile(self.cr3_ptr())
        }
    }

    #[doc="Write the CR3 register."]
    #[inline] pub fn set_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr3_mut(), f(Cr3(0)));
        }
        self
    }

    #[doc="Modify the CR3 register."]
    #[inline] pub fn with_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr3_mut(), f(self.cr3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BRR register."]
    #[inline] pub fn brr_mut(&self) -> *mut Brr { 
        (self.0 + 0xc) as *mut Brr
    }

    #[doc="Get the *const pointer for the BRR register."]
    #[inline] pub fn brr_ptr(&self) -> *const Brr { 
           self.brr_mut()
    }

    #[doc="Read the BRR register."]
    #[inline] pub fn brr(&self) -> Brr { 
        unsafe {
            read_volatile(self.brr_ptr())
        }
    }

    #[doc="Write the BRR register."]
    #[inline] pub fn set_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.brr_mut(), f(Brr(0)));
        }
        self
    }

    #[doc="Modify the BRR register."]
    #[inline] pub fn with_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.brr_mut(), f(self.brr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RQR register."]
    #[inline] pub fn rqr_mut(&self) -> *mut Rqr { 
        (self.0 + 0x18) as *mut Rqr
    }

    #[doc="Get the *const pointer for the RQR register."]
    #[inline] pub fn rqr_ptr(&self) -> *const Rqr { 
           self.rqr_mut()
    }

    #[doc="Write the RQR register."]
    #[inline] pub fn set_rqr<F: FnOnce(Rqr) -> Rqr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rqr_mut(), f(Rqr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut Isr { 
        (self.0 + 0x1c) as *mut Isr
    }

    #[doc="Get the *const pointer for the ISR register."]
    #[inline] pub fn isr_ptr(&self) -> *const Isr { 
           self.isr_mut()
    }

    #[doc="Read the ISR register."]
    #[inline] pub fn isr(&self) -> Isr { 
        unsafe {
            read_volatile(self.isr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        (self.0 + 0x20) as *mut Icr
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
           self.icr_mut()
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icr_mut(), f(Icr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the RDR register."]
    #[inline] pub fn rdr_mut(&self) -> *mut Rdr { 
        (self.0 + 0x24) as *mut Rdr
    }

    #[doc="Get the *const pointer for the RDR register."]
    #[inline] pub fn rdr_ptr(&self) -> *const Rdr { 
           self.rdr_mut()
    }

    #[doc="Read the RDR register."]
    #[inline] pub fn rdr(&self) -> Rdr { 
        unsafe {
            read_volatile(self.rdr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the TDR register."]
    #[inline] pub fn tdr_mut(&self) -> *mut Tdr { 
        (self.0 + 0x28) as *mut Tdr
    }

    #[doc="Get the *const pointer for the TDR register."]
    #[inline] pub fn tdr_ptr(&self) -> *const Tdr { 
           self.tdr_mut()
    }

    #[doc="Read the TDR register."]
    #[inline] pub fn tdr(&self) -> Tdr { 
        unsafe {
            read_volatile(self.tdr_ptr())
        }
    }

    #[doc="Write the TDR register."]
    #[inline] pub fn set_tdr<F: FnOnce(Tdr) -> Tdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdr_mut(), f(Tdr(0)));
        }
        self
    }

    #[doc="Modify the TDR register."]
    #[inline] pub fn with_tdr<F: FnOnce(Tdr) -> Tdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdr_mut(), f(self.tdr()));
        }
        self
    }

}

#[doc="Control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Word length"]
    #[inline] pub fn m1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if M1 != 0"]
    #[inline] pub fn test_m1(&self) -> bool {
        self.m1() != 0
    }

    #[doc="Sets the M1 field."]
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

    #[doc="Returns true if DEAT4 != 0"]
    #[inline] pub fn test_deat4(&self) -> bool {
        self.deat4() != 0
    }

    #[doc="Sets the DEAT4 field."]
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

    #[doc="Returns true if DEAT3 != 0"]
    #[inline] pub fn test_deat3(&self) -> bool {
        self.deat3() != 0
    }

    #[doc="Sets the DEAT3 field."]
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

    #[doc="Returns true if DEAT2 != 0"]
    #[inline] pub fn test_deat2(&self) -> bool {
        self.deat2() != 0
    }

    #[doc="Sets the DEAT2 field."]
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

    #[doc="Returns true if DEAT1 != 0"]
    #[inline] pub fn test_deat1(&self) -> bool {
        self.deat1() != 0
    }

    #[doc="Sets the DEAT1 field."]
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

    #[doc="Returns true if DEAT0 != 0"]
    #[inline] pub fn test_deat0(&self) -> bool {
        self.deat0() != 0
    }

    #[doc="Sets the DEAT0 field."]
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

    #[doc="Returns true if DEDT4 != 0"]
    #[inline] pub fn test_dedt4(&self) -> bool {
        self.dedt4() != 0
    }

    #[doc="Sets the DEDT4 field."]
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

    #[doc="Returns true if DEDT3 != 0"]
    #[inline] pub fn test_dedt3(&self) -> bool {
        self.dedt3() != 0
    }

    #[doc="Sets the DEDT3 field."]
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

    #[doc="Returns true if DEDT2 != 0"]
    #[inline] pub fn test_dedt2(&self) -> bool {
        self.dedt2() != 0
    }

    #[doc="Sets the DEDT2 field."]
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

    #[doc="Returns true if DEDT1 != 0"]
    #[inline] pub fn test_dedt1(&self) -> bool {
        self.dedt1() != 0
    }

    #[doc="Sets the DEDT1 field."]
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

    #[doc="Returns true if DEDT0 != 0"]
    #[inline] pub fn test_dedt0(&self) -> bool {
        self.dedt0() != 0
    }

    #[doc="Sets the DEDT0 field."]
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

    #[doc="Returns true if CMIE != 0"]
    #[inline] pub fn test_cmie(&self) -> bool {
        self.cmie() != 0
    }

    #[doc="Sets the CMIE field."]
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

    #[doc="Returns true if MME != 0"]
    #[inline] pub fn test_mme(&self) -> bool {
        self.mme() != 0
    }

    #[doc="Sets the MME field."]
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

    #[doc="Returns true if M0 != 0"]
    #[inline] pub fn test_m0(&self) -> bool {
        self.m0() != 0
    }

    #[doc="Sets the M0 field."]
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

    #[doc="Returns true if WAKE != 0"]
    #[inline] pub fn test_wake(&self) -> bool {
        self.wake() != 0
    }

    #[doc="Sets the WAKE field."]
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

    #[doc="Returns true if PCE != 0"]
    #[inline] pub fn test_pce(&self) -> bool {
        self.pce() != 0
    }

    #[doc="Sets the PCE field."]
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

    #[doc="Returns true if PS != 0"]
    #[inline] pub fn test_ps(&self) -> bool {
        self.ps() != 0
    }

    #[doc="Sets the PS field."]
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

    #[doc="Returns true if PEIE != 0"]
    #[inline] pub fn test_peie(&self) -> bool {
        self.peie() != 0
    }

    #[doc="Sets the PEIE field."]
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

    #[doc="Returns true if TXEIE != 0"]
    #[inline] pub fn test_txeie(&self) -> bool {
        self.txeie() != 0
    }

    #[doc="Sets the TXEIE field."]
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

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
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

    #[doc="Returns true if RXNEIE != 0"]
    #[inline] pub fn test_rxneie(&self) -> bool {
        self.rxneie() != 0
    }

    #[doc="Sets the RXNEIE field."]
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

    #[doc="Returns true if IDLEIE != 0"]
    #[inline] pub fn test_idleie(&self) -> bool {
        self.idleie() != 0
    }

    #[doc="Sets the IDLEIE field."]
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

    #[doc="Returns true if TE != 0"]
    #[inline] pub fn test_te(&self) -> bool {
        self.te() != 0
    }

    #[doc="Sets the TE field."]
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

    #[doc="Returns true if RE != 0"]
    #[inline] pub fn test_re(&self) -> bool {
        self.re() != 0
    }

    #[doc="Sets the RE field."]
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

    #[doc="Returns true if UESM != 0"]
    #[inline] pub fn test_uesm(&self) -> bool {
        self.uesm() != 0
    }

    #[doc="Sets the UESM field."]
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

    #[doc="Returns true if UE != 0"]
    #[inline] pub fn test_ue(&self) -> bool {
        self.ue() != 0
    }

    #[doc="Sets the UE field."]
    #[inline] pub fn set_ue<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr1(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="Address of the USART node"]
    #[inline] pub fn add4_7(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if ADD4_7 != 0"]
    #[inline] pub fn test_add4_7(&self) -> bool {
        self.add4_7() != 0
    }

    #[doc="Sets the ADD4_7 field."]
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

    #[doc="Returns true if ADD0_3 != 0"]
    #[inline] pub fn test_add0_3(&self) -> bool {
        self.add0_3() != 0
    }

    #[doc="Sets the ADD0_3 field."]
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

    #[doc="Returns true if MSBFIRST != 0"]
    #[inline] pub fn test_msbfirst(&self) -> bool {
        self.msbfirst() != 0
    }

    #[doc="Sets the MSBFIRST field."]
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

    #[doc="Returns true if TAINV != 0"]
    #[inline] pub fn test_tainv(&self) -> bool {
        self.tainv() != 0
    }

    #[doc="Sets the TAINV field."]
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

    #[doc="Returns true if TXINV != 0"]
    #[inline] pub fn test_txinv(&self) -> bool {
        self.txinv() != 0
    }

    #[doc="Sets the TXINV field."]
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

    #[doc="Returns true if RXINV != 0"]
    #[inline] pub fn test_rxinv(&self) -> bool {
        self.rxinv() != 0
    }

    #[doc="Sets the RXINV field."]
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

    #[doc="Returns true if SWAP != 0"]
    #[inline] pub fn test_swap(&self) -> bool {
        self.swap() != 0
    }

    #[doc="Sets the SWAP field."]
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

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
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

    #[doc="Returns true if CLKEN != 0"]
    #[inline] pub fn test_clken(&self) -> bool {
        self.clken() != 0
    }

    #[doc="Sets the CLKEN field."]
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

    #[doc="Returns true if ADDM7 != 0"]
    #[inline] pub fn test_addm7(&self) -> bool {
        self.addm7() != 0
    }

    #[doc="Sets the ADDM7 field."]
    #[inline] pub fn set_addm7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr2(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr3(pub u32);
impl Cr3 {
    #[doc="Wakeup from Stop mode interrupt enable"]
    #[inline] pub fn wufie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if WUFIE != 0"]
    #[inline] pub fn test_wufie(&self) -> bool {
        self.wufie() != 0
    }

    #[doc="Sets the WUFIE field."]
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

    #[doc="Returns true if WUS != 0"]
    #[inline] pub fn test_wus(&self) -> bool {
        self.wus() != 0
    }

    #[doc="Sets the WUS field."]
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

    #[doc="Returns true if DEP != 0"]
    #[inline] pub fn test_dep(&self) -> bool {
        self.dep() != 0
    }

    #[doc="Sets the DEP field."]
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

    #[doc="Returns true if DEM != 0"]
    #[inline] pub fn test_dem(&self) -> bool {
        self.dem() != 0
    }

    #[doc="Sets the DEM field."]
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

    #[doc="Returns true if DDRE != 0"]
    #[inline] pub fn test_ddre(&self) -> bool {
        self.ddre() != 0
    }

    #[doc="Sets the DDRE field."]
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

    #[doc="Returns true if OVRDIS != 0"]
    #[inline] pub fn test_ovrdis(&self) -> bool {
        self.ovrdis() != 0
    }

    #[doc="Sets the OVRDIS field."]
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

    #[doc="Returns true if CTSIE != 0"]
    #[inline] pub fn test_ctsie(&self) -> bool {
        self.ctsie() != 0
    }

    #[doc="Sets the CTSIE field."]
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

    #[doc="Returns true if CTSE != 0"]
    #[inline] pub fn test_ctse(&self) -> bool {
        self.ctse() != 0
    }

    #[doc="Sets the CTSE field."]
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

    #[doc="Returns true if RTSE != 0"]
    #[inline] pub fn test_rtse(&self) -> bool {
        self.rtse() != 0
    }

    #[doc="Sets the RTSE field."]
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

    #[doc="Returns true if DMAT != 0"]
    #[inline] pub fn test_dmat(&self) -> bool {
        self.dmat() != 0
    }

    #[doc="Sets the DMAT field."]
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

    #[doc="Returns true if DMAR != 0"]
    #[inline] pub fn test_dmar(&self) -> bool {
        self.dmar() != 0
    }

    #[doc="Sets the DMAR field."]
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

    #[doc="Returns true if HDSEL != 0"]
    #[inline] pub fn test_hdsel(&self) -> bool {
        self.hdsel() != 0
    }

    #[doc="Sets the HDSEL field."]
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

    #[doc="Returns true if EIE != 0"]
    #[inline] pub fn test_eie(&self) -> bool {
        self.eie() != 0
    }

    #[doc="Sets the EIE field."]
    #[inline] pub fn set_eie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr3(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Brr(pub u32);
impl Brr {
    #[doc="BRR"]
    #[inline] pub fn brr(&self) -> bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfffff) as u32) } // [19:0]
    }

    #[doc="Returns true if BRR != 0"]
    #[inline] pub fn test_brr(&self) -> bool {
        self.brr() != 0
    }

    #[doc="Sets the BRR field."]
    #[inline] pub fn set_brr<V: Into<bits::U20>>(mut self, value: V) -> Self {
        let value: bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Brr {
    #[inline]
    fn from(other: u32) -> Self {
         Brr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rqr(pub u32);
impl Rqr {
    #[doc="Receive data flush request"]
    #[inline] pub fn rxfrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXFRQ != 0"]
    #[inline] pub fn test_rxfrq(&self) -> bool {
        self.rxfrq() != 0
    }

    #[doc="Sets the RXFRQ field."]
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

    #[doc="Returns true if MMRQ != 0"]
    #[inline] pub fn test_mmrq(&self) -> bool {
        self.mmrq() != 0
    }

    #[doc="Sets the MMRQ field."]
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

    #[doc="Returns true if SBKRQ != 0"]
    #[inline] pub fn test_sbkrq(&self) -> bool {
        self.sbkrq() != 0
    }

    #[doc="Sets the SBKRQ field."]
    #[inline] pub fn set_sbkrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Rqr {
    #[inline]
    fn from(other: u32) -> Self {
         Rqr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="REACK"]
    #[inline] pub fn reack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if REACK != 0"]
    #[inline] pub fn test_reack(&self) -> bool {
        self.reack() != 0
    }

    #[doc="Sets the REACK field."]
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

    #[doc="Returns true if TEACK != 0"]
    #[inline] pub fn test_teack(&self) -> bool {
        self.teack() != 0
    }

    #[doc="Sets the TEACK field."]
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

    #[doc="Returns true if WUF != 0"]
    #[inline] pub fn test_wuf(&self) -> bool {
        self.wuf() != 0
    }

    #[doc="Sets the WUF field."]
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

    #[doc="Returns true if RWU != 0"]
    #[inline] pub fn test_rwu(&self) -> bool {
        self.rwu() != 0
    }

    #[doc="Sets the RWU field."]
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

    #[doc="Returns true if SBKF != 0"]
    #[inline] pub fn test_sbkf(&self) -> bool {
        self.sbkf() != 0
    }

    #[doc="Sets the SBKF field."]
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

    #[doc="Returns true if CMF != 0"]
    #[inline] pub fn test_cmf(&self) -> bool {
        self.cmf() != 0
    }

    #[doc="Sets the CMF field."]
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

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
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

    #[doc="Returns true if CTS != 0"]
    #[inline] pub fn test_cts(&self) -> bool {
        self.cts() != 0
    }

    #[doc="Sets the CTS field."]
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

    #[doc="Returns true if CTSIF != 0"]
    #[inline] pub fn test_ctsif(&self) -> bool {
        self.ctsif() != 0
    }

    #[doc="Sets the CTSIF field."]
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

    #[doc="Returns true if TXE != 0"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe() != 0
    }

    #[doc="Sets the TXE field."]
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

    #[doc="Returns true if TC != 0"]
    #[inline] pub fn test_tc(&self) -> bool {
        self.tc() != 0
    }

    #[doc="Sets the TC field."]
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

    #[doc="Returns true if RXNE != 0"]
    #[inline] pub fn test_rxne(&self) -> bool {
        self.rxne() != 0
    }

    #[doc="Sets the RXNE field."]
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

    #[doc="Returns true if IDLE != 0"]
    #[inline] pub fn test_idle(&self) -> bool {
        self.idle() != 0
    }

    #[doc="Sets the IDLE field."]
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

    #[doc="Returns true if ORE != 0"]
    #[inline] pub fn test_ore(&self) -> bool {
        self.ore() != 0
    }

    #[doc="Sets the ORE field."]
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

    #[doc="Returns true if NF != 0"]
    #[inline] pub fn test_nf(&self) -> bool {
        self.nf() != 0
    }

    #[doc="Sets the NF field."]
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

    #[doc="Returns true if FE != 0"]
    #[inline] pub fn test_fe(&self) -> bool {
        self.fe() != 0
    }

    #[doc="Sets the FE field."]
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

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Isr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="Wakeup from Stop mode clear flag"]
    #[inline] pub fn wucf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if WUCF != 0"]
    #[inline] pub fn test_wucf(&self) -> bool {
        self.wucf() != 0
    }

    #[doc="Sets the WUCF field."]
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

    #[doc="Returns true if CMCF != 0"]
    #[inline] pub fn test_cmcf(&self) -> bool {
        self.cmcf() != 0
    }

    #[doc="Sets the CMCF field."]
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

    #[doc="Returns true if CTSCF != 0"]
    #[inline] pub fn test_ctscf(&self) -> bool {
        self.ctscf() != 0
    }

    #[doc="Sets the CTSCF field."]
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

    #[doc="Returns true if TCCF != 0"]
    #[inline] pub fn test_tccf(&self) -> bool {
        self.tccf() != 0
    }

    #[doc="Sets the TCCF field."]
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

    #[doc="Returns true if IDLECF != 0"]
    #[inline] pub fn test_idlecf(&self) -> bool {
        self.idlecf() != 0
    }

    #[doc="Sets the IDLECF field."]
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

    #[doc="Returns true if ORECF != 0"]
    #[inline] pub fn test_orecf(&self) -> bool {
        self.orecf() != 0
    }

    #[doc="Sets the ORECF field."]
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

    #[doc="Returns true if NCF != 0"]
    #[inline] pub fn test_ncf(&self) -> bool {
        self.ncf() != 0
    }

    #[doc="Sets the NCF field."]
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

    #[doc="Returns true if FECF != 0"]
    #[inline] pub fn test_fecf(&self) -> bool {
        self.fecf() != 0
    }

    #[doc="Sets the FECF field."]
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

    #[doc="Returns true if PECF != 0"]
    #[inline] pub fn test_pecf(&self) -> bool {
        self.pecf() != 0
    }

    #[doc="Sets the PECF field."]
    #[inline] pub fn set_pecf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Icr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdr(pub u32);
impl Rdr {
    #[doc="Receive data value"]
    #[inline] pub fn rdr(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if RDR != 0"]
    #[inline] pub fn test_rdr(&self) -> bool {
        self.rdr() != 0
    }

    #[doc="Sets the RDR field."]
    #[inline] pub fn set_rdr<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdr {
    #[inline]
    fn from(other: u32) -> Self {
         Rdr(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdr(pub u32);
impl Tdr {
    #[doc="Transmit data value"]
    #[inline] pub fn tdr(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if TDR != 0"]
    #[inline] pub fn test_tdr(&self) -> bool {
        self.tdr() != 0
    }

    #[doc="Sets the TDR field."]
    #[inline] pub fn set_tdr<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdr {
    #[inline]
    fn from(other: u32) -> Self {
         Tdr(other)
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


