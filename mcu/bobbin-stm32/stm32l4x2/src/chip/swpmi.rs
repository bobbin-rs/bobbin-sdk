#[allow(unused_imports)] use bobbin_common::*;

periph!( SWPMI1, Swpmi1, _SWPMI1, SwpmiPeriph, 0x40008800);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SWPMI Peripheral"]
pub struct SwpmiPeriph(pub usize); 



impl SwpmiPeriph {
    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        (self.0 + 0x0) as *mut Cr
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
           self.cr_mut()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            read_volatile(self.cr_ptr())
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(Cr(0)));
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(self.cr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BRR register."]
    #[inline] pub fn brr_mut(&self) -> *mut Brr { 
        (self.0 + 0x4) as *mut Brr
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

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut Isr { 
        (self.0 + 0xc) as *mut Isr
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
        (self.0 + 0x10) as *mut Icr
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

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        (self.0 + 0x14) as *mut Ier
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const Ier { 
           self.ier_mut()
    }

    #[doc="Read the IER register."]
    #[inline] pub fn ier(&self) -> Ier { 
        unsafe {
            read_volatile(self.ier_ptr())
        }
    }

    #[doc="Write the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ier_mut(), f(Ier(0)));
        }
        self
    }

    #[doc="Modify the IER register."]
    #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ier_mut(), f(self.ier()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RFL register."]
    #[inline] pub fn rfl_mut(&self) -> *mut Rfl { 
        (self.0 + 0x18) as *mut Rfl
    }

    #[doc="Get the *const pointer for the RFL register."]
    #[inline] pub fn rfl_ptr(&self) -> *const Rfl { 
           self.rfl_mut()
    }

    #[doc="Read the RFL register."]
    #[inline] pub fn rfl(&self) -> Rfl { 
        unsafe {
            read_volatile(self.rfl_ptr())
        }
    }

    #[doc="Get the *mut pointer for the TDR register."]
    #[inline] pub fn tdr_mut(&self) -> *mut Tdr { 
        (self.0 + 0x1c) as *mut Tdr
    }

    #[doc="Get the *const pointer for the TDR register."]
    #[inline] pub fn tdr_ptr(&self) -> *const Tdr { 
           self.tdr_mut()
    }

    #[doc="Write the TDR register."]
    #[inline] pub fn set_tdr<F: FnOnce(Tdr) -> Tdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdr_mut(), f(Tdr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the RDR register."]
    #[inline] pub fn rdr_mut(&self) -> *mut Rdr { 
        (self.0 + 0x20) as *mut Rdr
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

}

#[doc="SWPMI Configuration/Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Reception DMA enable"]
    #[inline] pub fn rxdma(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXDMA != 0"]
    #[inline] pub fn test_rxdma(&self) -> bool {
        self.rxdma() != 0
    }

    #[doc="Sets the RXDMA field."]
    #[inline] pub fn set_rxdma<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmission DMA enable"]
    #[inline] pub fn txdma(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXDMA != 0"]
    #[inline] pub fn test_txdma(&self) -> bool {
        self.txdma() != 0
    }

    #[doc="Sets the TXDMA field."]
    #[inline] pub fn set_txdma<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Reception buffering mode"]
    #[inline] pub fn rxmode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXMODE != 0"]
    #[inline] pub fn test_rxmode(&self) -> bool {
        self.rxmode() != 0
    }

    #[doc="Sets the RXMODE field."]
    #[inline] pub fn set_rxmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmission buffering mode"]
    #[inline] pub fn txmode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TXMODE != 0"]
    #[inline] pub fn test_txmode(&self) -> bool {
        self.txmode() != 0
    }

    #[doc="Sets the TXMODE field."]
    #[inline] pub fn set_txmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Loopback mode enable"]
    #[inline] pub fn lpbk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LPBK != 0"]
    #[inline] pub fn test_lpbk(&self) -> bool {
        self.lpbk() != 0
    }

    #[doc="Sets the LPBK field."]
    #[inline] pub fn set_lpbk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Single wire protocol master interface enable"]
    #[inline] pub fn swpme(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SWPME != 0"]
    #[inline] pub fn test_swpme(&self) -> bool {
        self.swpme() != 0
    }

    #[doc="Sets the SWPME field."]
    #[inline] pub fn set_swpme<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Single wire protocol master interface deactivate"]
    #[inline] pub fn deact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DEACT != 0"]
    #[inline] pub fn test_deact(&self) -> bool {
        self.deact() != 0
    }

    #[doc="Sets the DEACT field."]
    #[inline] pub fn set_deact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdma() != 0 { try!(write!(f, " rxdma"))}
        if self.txdma() != 0 { try!(write!(f, " txdma"))}
        if self.rxmode() != 0 { try!(write!(f, " rxmode"))}
        if self.txmode() != 0 { try!(write!(f, " txmode"))}
        if self.lpbk() != 0 { try!(write!(f, " lpbk"))}
        if self.swpme() != 0 { try!(write!(f, " swpme"))}
        if self.deact() != 0 { try!(write!(f, " deact"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SWPMI Bitrate register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Brr(pub u32);
impl Brr {
    #[doc="Bitrate prescaler"]
    #[inline] pub fn br(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if BR != 0"]
    #[inline] pub fn test_br(&self) -> bool {
        self.br() != 0
    }

    #[doc="Sets the BR field."]
    #[inline] pub fn set_br<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
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
        if self.br() != 0 { try!(write!(f, " br=0x{:x}", self.br()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SWPMI Interrupt and Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Receive buffer full flag"]
    #[inline] pub fn rxbff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXBFF != 0"]
    #[inline] pub fn test_rxbff(&self) -> bool {
        self.rxbff() != 0
    }

    #[doc="Sets the RXBFF field."]
    #[inline] pub fn set_rxbff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit buffer empty flag"]
    #[inline] pub fn txbef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXBEF != 0"]
    #[inline] pub fn test_txbef(&self) -> bool {
        self.txbef() != 0
    }

    #[doc="Sets the TXBEF field."]
    #[inline] pub fn set_txbef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive CRC error flag"]
    #[inline] pub fn rxberf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXBERF != 0"]
    #[inline] pub fn test_rxberf(&self) -> bool {
        self.rxberf() != 0
    }

    #[doc="Sets the RXBERF field."]
    #[inline] pub fn set_rxberf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive overrun error flag"]
    #[inline] pub fn rxovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXOVRF != 0"]
    #[inline] pub fn test_rxovrf(&self) -> bool {
        self.rxovrf() != 0
    }

    #[doc="Sets the RXOVRF field."]
    #[inline] pub fn set_rxovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit underrun error flag"]
    #[inline] pub fn txunrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXUNRF != 0"]
    #[inline] pub fn test_txunrf(&self) -> bool {
        self.txunrf() != 0
    }

    #[doc="Sets the TXUNRF field."]
    #[inline] pub fn set_txunrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receive data register not empty"]
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

    #[doc="Transmit data register empty"]
    #[inline] pub fn txe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TXE != 0"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe() != 0
    }

    #[doc="Sets the TXE field."]
    #[inline] pub fn set_txe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transfer complete flag"]
    #[inline] pub fn tcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TCF != 0"]
    #[inline] pub fn test_tcf(&self) -> bool {
        self.tcf() != 0
    }

    #[doc="Sets the TCF field."]
    #[inline] pub fn set_tcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Slave resume flag"]
    #[inline] pub fn srf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SRF != 0"]
    #[inline] pub fn test_srf(&self) -> bool {
        self.srf() != 0
    }

    #[doc="Sets the SRF field."]
    #[inline] pub fn set_srf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="SUSPEND flag"]
    #[inline] pub fn susp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="DEACTIVATED flag"]
    #[inline] pub fn deactf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DEACTF != 0"]
    #[inline] pub fn test_deactf(&self) -> bool {
        self.deactf() != 0
    }

    #[doc="Sets the DEACTF field."]
    #[inline] pub fn set_deactf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
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
        if self.rxbff() != 0 { try!(write!(f, " rxbff"))}
        if self.txbef() != 0 { try!(write!(f, " txbef"))}
        if self.rxberf() != 0 { try!(write!(f, " rxberf"))}
        if self.rxovrf() != 0 { try!(write!(f, " rxovrf"))}
        if self.txunrf() != 0 { try!(write!(f, " txunrf"))}
        if self.rxne() != 0 { try!(write!(f, " rxne"))}
        if self.txe() != 0 { try!(write!(f, " txe"))}
        if self.tcf() != 0 { try!(write!(f, " tcf"))}
        if self.srf() != 0 { try!(write!(f, " srf"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        if self.deactf() != 0 { try!(write!(f, " deactf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SWPMI Interrupt Flag Clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="Clear receive buffer full flag"]
    #[inline] pub fn crxbff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CRXBFF != 0"]
    #[inline] pub fn test_crxbff(&self) -> bool {
        self.crxbff() != 0
    }

    #[doc="Sets the CRXBFF field."]
    #[inline] pub fn set_crxbff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clear transmit buffer empty flag"]
    #[inline] pub fn ctxbef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTXBEF != 0"]
    #[inline] pub fn test_ctxbef(&self) -> bool {
        self.ctxbef() != 0
    }

    #[doc="Sets the CTXBEF field."]
    #[inline] pub fn set_ctxbef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear receive CRC error flag"]
    #[inline] pub fn crxberf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CRXBERF != 0"]
    #[inline] pub fn test_crxberf(&self) -> bool {
        self.crxberf() != 0
    }

    #[doc="Sets the CRXBERF field."]
    #[inline] pub fn set_crxberf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clear receive overrun error flag"]
    #[inline] pub fn crxovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CRXOVRF != 0"]
    #[inline] pub fn test_crxovrf(&self) -> bool {
        self.crxovrf() != 0
    }

    #[doc="Sets the CRXOVRF field."]
    #[inline] pub fn set_crxovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear transmit underrun error flag"]
    #[inline] pub fn ctxunrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTXUNRF != 0"]
    #[inline] pub fn test_ctxunrf(&self) -> bool {
        self.ctxunrf() != 0
    }

    #[doc="Sets the CTXUNRF field."]
    #[inline] pub fn set_ctxunrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear transfer complete flag"]
    #[inline] pub fn ctcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTCF != 0"]
    #[inline] pub fn test_ctcf(&self) -> bool {
        self.ctcf() != 0
    }

    #[doc="Sets the CTCF field."]
    #[inline] pub fn set_ctcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Clear slave resume flag"]
    #[inline] pub fn csrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CSRF != 0"]
    #[inline] pub fn test_csrf(&self) -> bool {
        self.csrf() != 0
    }

    #[doc="Sets the CSRF field."]
    #[inline] pub fn set_csrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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
        if self.crxbff() != 0 { try!(write!(f, " crxbff"))}
        if self.ctxbef() != 0 { try!(write!(f, " ctxbef"))}
        if self.crxberf() != 0 { try!(write!(f, " crxberf"))}
        if self.crxovrf() != 0 { try!(write!(f, " crxovrf"))}
        if self.ctxunrf() != 0 { try!(write!(f, " ctxunrf"))}
        if self.ctcf() != 0 { try!(write!(f, " ctcf"))}
        if self.csrf() != 0 { try!(write!(f, " csrf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SWPMI Interrupt Enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="Receive buffer full interrupt enable"]
    #[inline] pub fn rxbfie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXBFIE != 0"]
    #[inline] pub fn test_rxbfie(&self) -> bool {
        self.rxbfie() != 0
    }

    #[doc="Sets the RXBFIE field."]
    #[inline] pub fn set_rxbfie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit buffer empty interrupt enable"]
    #[inline] pub fn txbeie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXBEIE != 0"]
    #[inline] pub fn test_txbeie(&self) -> bool {
        self.txbeie() != 0
    }

    #[doc="Sets the TXBEIE field."]
    #[inline] pub fn set_txbeie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive CRC error interrupt enable"]
    #[inline] pub fn rxberie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXBERIE != 0"]
    #[inline] pub fn test_rxberie(&self) -> bool {
        self.rxberie() != 0
    }

    #[doc="Sets the RXBERIE field."]
    #[inline] pub fn set_rxberie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive overrun error interrupt enable"]
    #[inline] pub fn rxovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXOVRIE != 0"]
    #[inline] pub fn test_rxovrie(&self) -> bool {
        self.rxovrie() != 0
    }

    #[doc="Sets the RXOVRIE field."]
    #[inline] pub fn set_rxovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit underrun error interrupt enable"]
    #[inline] pub fn txunrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXUNRIE != 0"]
    #[inline] pub fn test_txunrie(&self) -> bool {
        self.txunrie() != 0
    }

    #[doc="Sets the TXUNRIE field."]
    #[inline] pub fn set_txunrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receive interrupt enable"]
    #[inline] pub fn rie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RIE != 0"]
    #[inline] pub fn test_rie(&self) -> bool {
        self.rie() != 0
    }

    #[doc="Sets the RIE field."]
    #[inline] pub fn set_rie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmit interrupt enable"]
    #[inline] pub fn tie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIE != 0"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie() != 0
    }

    #[doc="Sets the TIE field."]
    #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit complete interrupt enable"]
    #[inline] pub fn tcie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
    #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Slave resume interrupt enable"]
    #[inline] pub fn srie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SRIE != 0"]
    #[inline] pub fn test_srie(&self) -> bool {
        self.srie() != 0
    }

    #[doc="Sets the SRIE field."]
    #[inline] pub fn set_srie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Ier {
    #[inline]
    fn from(other: u32) -> Self {
         Ier(other)
    }
}

impl ::core::fmt::Display for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxbfie() != 0 { try!(write!(f, " rxbfie"))}
        if self.txbeie() != 0 { try!(write!(f, " txbeie"))}
        if self.rxberie() != 0 { try!(write!(f, " rxberie"))}
        if self.rxovrie() != 0 { try!(write!(f, " rxovrie"))}
        if self.txunrie() != 0 { try!(write!(f, " txunrie"))}
        if self.rie() != 0 { try!(write!(f, " rie"))}
        if self.tie() != 0 { try!(write!(f, " tie"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.srie() != 0 { try!(write!(f, " srie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SWPMI Receive Frame Length register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rfl(pub u32);
impl Rfl {
    #[doc="Receive frame length"]
    #[inline] pub fn rfl(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if RFL != 0"]
    #[inline] pub fn test_rfl(&self) -> bool {
        self.rfl() != 0
    }

    #[doc="Sets the RFL field."]
    #[inline] pub fn set_rfl<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rfl {
    #[inline]
    fn from(other: u32) -> Self {
         Rfl(other)
    }
}

impl ::core::fmt::Display for Rfl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rfl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rfl() != 0 { try!(write!(f, " rfl=0x{:x}", self.rfl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SWPMI Transmit data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdr(pub u32);
impl Tdr {
    #[doc="Transmit data"]
    #[inline] pub fn td(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TD != 0"]
    #[inline] pub fn test_td(&self) -> bool {
        self.td() != 0
    }

    #[doc="Sets the TD field."]
    #[inline] pub fn set_td<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SWPMI Receive data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdr(pub u32);
impl Rdr {
    #[doc="received data"]
    #[inline] pub fn rd(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RD != 0"]
    #[inline] pub fn test_rd(&self) -> bool {
        self.rd() != 0
    }

    #[doc="Sets the RD field."]
    #[inline] pub fn set_rd<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
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
        try!(write!(f, "]"));
        Ok(())
    }
}


