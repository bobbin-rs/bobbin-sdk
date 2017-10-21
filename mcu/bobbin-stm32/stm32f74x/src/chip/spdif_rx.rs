//! Receiver Interface
#[allow(unused_imports)] use bobbin_common::*;

periph!(SPDIF_RX, SpdifRx, 0x40004000);

#[doc="Receiver Interface"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SpdifRx(pub usize);
impl SpdifRx {
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

    #[doc="Get the *mut pointer for the IMR register."]
    #[inline] pub fn imr_mut(&self) -> *mut Imr { 
        (self.0 + 0x4) as *mut Imr
    }

    #[doc="Get the *const pointer for the IMR register."]
    #[inline] pub fn imr_ptr(&self) -> *const Imr { 
           self.imr_mut()
    }

    #[doc="Read the IMR register."]
    #[inline] pub fn imr(&self) -> Imr { 
        unsafe {
            read_volatile(self.imr_ptr())
        }
    }

    #[doc="Write the IMR register."]
    #[inline] pub fn set_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imr_mut(), f(Imr(0)));
        }
        self
    }

    #[doc="Modify the IMR register."]
    #[inline] pub fn with_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imr_mut(), f(self.imr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        (self.0 + 0x8) as *mut Sr
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
           self.sr_mut()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        unsafe {
            read_volatile(self.sr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IFCR register."]
    #[inline] pub fn ifcr_mut(&self) -> *mut Ifcr { 
        (self.0 + 0xc) as *mut Ifcr
    }

    #[doc="Get the *const pointer for the IFCR register."]
    #[inline] pub fn ifcr_ptr(&self) -> *const Ifcr { 
           self.ifcr_mut()
    }

    #[doc="Write the IFCR register."]
    #[inline] pub fn set_ifcr<F: FnOnce(Ifcr) -> Ifcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifcr_mut(), f(Ifcr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut Dr { 
        (self.0 + 0x10) as *mut Dr
    }

    #[doc="Get the *const pointer for the DR register."]
    #[inline] pub fn dr_ptr(&self) -> *const Dr { 
           self.dr_mut()
    }

    #[doc="Read the DR register."]
    #[inline] pub fn dr(&self) -> Dr { 
        unsafe {
            read_volatile(self.dr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut Csr { 
        (self.0 + 0x14) as *mut Csr
    }

    #[doc="Get the *const pointer for the CSR register."]
    #[inline] pub fn csr_ptr(&self) -> *const Csr { 
           self.csr_mut()
    }

    #[doc="Read the CSR register."]
    #[inline] pub fn csr(&self) -> Csr { 
        unsafe {
            read_volatile(self.csr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DIR register."]
    #[inline] pub fn dir_mut(&self) -> *mut Dir { 
        (self.0 + 0x18) as *mut Dir
    }

    #[doc="Get the *const pointer for the DIR register."]
    #[inline] pub fn dir_ptr(&self) -> *const Dir { 
           self.dir_mut()
    }

    #[doc="Read the DIR register."]
    #[inline] pub fn dir(&self) -> Dir { 
        unsafe {
            read_volatile(self.dir_ptr())
        }
    }

}

#[doc="Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Peripheral Block Enable"]
    #[inline] pub fn spdifen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SPDIFEN != 0"]
    #[inline] pub fn test_spdifen(&self) -> bool {
        self.spdifen() != 0
    }

    #[doc="Sets the SPDIFEN field."]
    #[inline] pub fn set_spdifen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receiver DMA ENable for data flow"]
    #[inline] pub fn rxdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXDMAEN != 0"]
    #[inline] pub fn test_rxdmaen(&self) -> bool {
        self.rxdmaen() != 0
    }

    #[doc="Sets the RXDMAEN field."]
    #[inline] pub fn set_rxdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="STerEO Mode"]
    #[inline] pub fn rxsteo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXSTEO != 0"]
    #[inline] pub fn test_rxsteo(&self) -> bool {
        self.rxsteo() != 0
    }

    #[doc="Sets the RXSTEO field."]
    #[inline] pub fn set_rxsteo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="RX Data format"]
    #[inline] pub fn drfmt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if DRFMT != 0"]
    #[inline] pub fn test_drfmt(&self) -> bool {
        self.drfmt() != 0
    }

    #[doc="Sets the DRFMT field."]
    #[inline] pub fn set_drfmt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Mask Parity error bit"]
    #[inline] pub fn pmsk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PMSK != 0"]
    #[inline] pub fn test_pmsk(&self) -> bool {
        self.pmsk() != 0
    }

    #[doc="Sets the PMSK field."]
    #[inline] pub fn set_pmsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Mask of Validity bit"]
    #[inline] pub fn vmsk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if VMSK != 0"]
    #[inline] pub fn test_vmsk(&self) -> bool {
        self.vmsk() != 0
    }

    #[doc="Sets the VMSK field."]
    #[inline] pub fn set_vmsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Mask of channel status and user bits"]
    #[inline] pub fn cumsk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CUMSK != 0"]
    #[inline] pub fn test_cumsk(&self) -> bool {
        self.cumsk() != 0
    }

    #[doc="Sets the CUMSK field."]
    #[inline] pub fn set_cumsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Mask of Preamble Type bits"]
    #[inline] pub fn ptmsk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PTMSK != 0"]
    #[inline] pub fn test_ptmsk(&self) -> bool {
        self.ptmsk() != 0
    }

    #[doc="Sets the PTMSK field."]
    #[inline] pub fn set_ptmsk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Control Buffer DMA ENable for control flow"]
    #[inline] pub fn cbdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CBDMAEN != 0"]
    #[inline] pub fn test_cbdmaen(&self) -> bool {
        self.cbdmaen() != 0
    }

    #[doc="Sets the CBDMAEN field."]
    #[inline] pub fn set_cbdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Channel Selection"]
    #[inline] pub fn chsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CHSEL != 0"]
    #[inline] pub fn test_chsel(&self) -> bool {
        self.chsel() != 0
    }

    #[doc="Sets the CHSEL field."]
    #[inline] pub fn set_chsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Maximum allowed re-tries during synchronization phase"]
    #[inline] pub fn nbtr(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if NBTR != 0"]
    #[inline] pub fn test_nbtr(&self) -> bool {
        self.nbtr() != 0
    }

    #[doc="Sets the NBTR field."]
    #[inline] pub fn set_nbtr<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Wait For Activity"]
    #[inline] pub fn wfa(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if WFA != 0"]
    #[inline] pub fn test_wfa(&self) -> bool {
        self.wfa() != 0
    }

    #[doc="Sets the WFA field."]
    #[inline] pub fn set_wfa<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="input selection"]
    #[inline] pub fn insel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if INSEL != 0"]
    #[inline] pub fn test_insel(&self) -> bool {
        self.insel() != 0
    }

    #[doc="Sets the INSEL field."]
    #[inline] pub fn set_insel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
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
        if self.spdifen() != 0 { try!(write!(f, " spdifen=0x{:x}", self.spdifen()))}
        if self.rxdmaen() != 0 { try!(write!(f, " rxdmaen"))}
        if self.rxsteo() != 0 { try!(write!(f, " rxsteo"))}
        if self.drfmt() != 0 { try!(write!(f, " drfmt=0x{:x}", self.drfmt()))}
        if self.pmsk() != 0 { try!(write!(f, " pmsk"))}
        if self.vmsk() != 0 { try!(write!(f, " vmsk"))}
        if self.cumsk() != 0 { try!(write!(f, " cumsk"))}
        if self.ptmsk() != 0 { try!(write!(f, " ptmsk"))}
        if self.cbdmaen() != 0 { try!(write!(f, " cbdmaen"))}
        if self.chsel() != 0 { try!(write!(f, " chsel"))}
        if self.nbtr() != 0 { try!(write!(f, " nbtr=0x{:x}", self.nbtr()))}
        if self.wfa() != 0 { try!(write!(f, " wfa"))}
        if self.insel() != 0 { try!(write!(f, " insel=0x{:x}", self.insel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc="RXNE interrupt enable"]
    #[inline] pub fn rxneie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXNEIE != 0"]
    #[inline] pub fn test_rxneie(&self) -> bool {
        self.rxneie() != 0
    }

    #[doc="Sets the RXNEIE field."]
    #[inline] pub fn set_rxneie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Control Buffer Ready Interrupt Enable"]
    #[inline] pub fn csrneie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CSRNEIE != 0"]
    #[inline] pub fn test_csrneie(&self) -> bool {
        self.csrneie() != 0
    }

    #[doc="Sets the CSRNEIE field."]
    #[inline] pub fn set_csrneie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Parity error interrupt enable"]
    #[inline] pub fn perrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PERRIE != 0"]
    #[inline] pub fn test_perrie(&self) -> bool {
        self.perrie() != 0
    }

    #[doc="Sets the PERRIE field."]
    #[inline] pub fn set_perrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Overrun error Interrupt Enable"]
    #[inline] pub fn ovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OVRIE != 0"]
    #[inline] pub fn test_ovrie(&self) -> bool {
        self.ovrie() != 0
    }

    #[doc="Sets the OVRIE field."]
    #[inline] pub fn set_ovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Synchronization Block Detected Interrupt Enable"]
    #[inline] pub fn sblkie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SBLKIE != 0"]
    #[inline] pub fn test_sblkie(&self) -> bool {
        self.sblkie() != 0
    }

    #[doc="Sets the SBLKIE field."]
    #[inline] pub fn set_sblkie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Synchronization Done"]
    #[inline] pub fn syncdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SYNCDIE != 0"]
    #[inline] pub fn test_syncdie(&self) -> bool {
        self.syncdie() != 0
    }

    #[doc="Sets the SYNCDIE field."]
    #[inline] pub fn set_syncdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Serial Interface Error Interrupt Enable"]
    #[inline] pub fn ifeie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if IFEIE != 0"]
    #[inline] pub fn test_ifeie(&self) -> bool {
        self.ifeie() != 0
    }

    #[doc="Sets the IFEIE field."]
    #[inline] pub fn set_ifeie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Imr {
    #[inline]
    fn from(other: u32) -> Self {
         Imr(other)
    }
}

impl ::core::fmt::Display for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxneie() != 0 { try!(write!(f, " rxneie"))}
        if self.csrneie() != 0 { try!(write!(f, " csrneie"))}
        if self.perrie() != 0 { try!(write!(f, " perrie"))}
        if self.ovrie() != 0 { try!(write!(f, " ovrie"))}
        if self.sblkie() != 0 { try!(write!(f, " sblkie"))}
        if self.syncdie() != 0 { try!(write!(f, " syncdie"))}
        if self.ifeie() != 0 { try!(write!(f, " ifeie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Read data register not empty"]
    #[inline] pub fn rxne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXNE != 0"]
    #[inline] pub fn test_rxne(&self) -> bool {
        self.rxne() != 0
    }

    #[doc="Sets the RXNE field."]
    #[inline] pub fn set_rxne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Control Buffer register is not empty"]
    #[inline] pub fn csrne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CSRNE != 0"]
    #[inline] pub fn test_csrne(&self) -> bool {
        self.csrne() != 0
    }

    #[doc="Sets the CSRNE field."]
    #[inline] pub fn set_csrne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Parity error"]
    #[inline] pub fn perr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Overrun error"]
    #[inline] pub fn ovr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OVR != 0"]
    #[inline] pub fn test_ovr(&self) -> bool {
        self.ovr() != 0
    }

    #[doc="Sets the OVR field."]
    #[inline] pub fn set_ovr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Synchronization Block Detected"]
    #[inline] pub fn sbd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SBD != 0"]
    #[inline] pub fn test_sbd(&self) -> bool {
        self.sbd() != 0
    }

    #[doc="Sets the SBD field."]
    #[inline] pub fn set_sbd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Synchronization Done"]
    #[inline] pub fn syncd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SYNCD != 0"]
    #[inline] pub fn test_syncd(&self) -> bool {
        self.syncd() != 0
    }

    #[doc="Sets the SYNCD field."]
    #[inline] pub fn set_syncd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Framing error"]
    #[inline] pub fn ferr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Synchronization error"]
    #[inline] pub fn serr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SERR != 0"]
    #[inline] pub fn test_serr(&self) -> bool {
        self.serr() != 0
    }

    #[doc="Sets the SERR field."]
    #[inline] pub fn set_serr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Time-out error"]
    #[inline] pub fn terr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TERR != 0"]
    #[inline] pub fn test_terr(&self) -> bool {
        self.terr() != 0
    }

    #[doc="Sets the TERR field."]
    #[inline] pub fn set_terr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Duration of 5 symbols counted with SPDIF_CLK"]
    #[inline] pub fn width5(&self) -> bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7fff) as u16) } // [30:16]
    }

    #[doc="Returns true if WIDTH5 != 0"]
    #[inline] pub fn test_width5(&self) -> bool {
        self.width5() != 0
    }

    #[doc="Sets the WIDTH5 field."]
    #[inline] pub fn set_width5<V: Into<bits::U15>>(mut self, value: V) -> Self {
        let value: bits::U15 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
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
        if self.rxne() != 0 { try!(write!(f, " rxne"))}
        if self.csrne() != 0 { try!(write!(f, " csrne"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.ovr() != 0 { try!(write!(f, " ovr"))}
        if self.sbd() != 0 { try!(write!(f, " sbd"))}
        if self.syncd() != 0 { try!(write!(f, " syncd"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        if self.serr() != 0 { try!(write!(f, " serr"))}
        if self.terr() != 0 { try!(write!(f, " terr"))}
        if self.width5() != 0 { try!(write!(f, " width5=0x{:x}", self.width5()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifcr(pub u32);
impl Ifcr {
    #[doc="Clears the Parity error flag"]
    #[inline] pub fn perrcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PERRCF != 0"]
    #[inline] pub fn test_perrcf(&self) -> bool {
        self.perrcf() != 0
    }

    #[doc="Sets the PERRCF field."]
    #[inline] pub fn set_perrcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clears the Overrun error flag"]
    #[inline] pub fn ovrcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OVRCF != 0"]
    #[inline] pub fn test_ovrcf(&self) -> bool {
        self.ovrcf() != 0
    }

    #[doc="Sets the OVRCF field."]
    #[inline] pub fn set_ovrcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clears the Synchronization Block Detected flag"]
    #[inline] pub fn sbdcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SBDCF != 0"]
    #[inline] pub fn test_sbdcf(&self) -> bool {
        self.sbdcf() != 0
    }

    #[doc="Sets the SBDCF field."]
    #[inline] pub fn set_sbdcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clears the Synchronization Done flag"]
    #[inline] pub fn syncdcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SYNCDCF != 0"]
    #[inline] pub fn test_syncdcf(&self) -> bool {
        self.syncdcf() != 0
    }

    #[doc="Sets the SYNCDCF field."]
    #[inline] pub fn set_syncdcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Ifcr {
    #[inline]
    fn from(other: u32) -> Self {
         Ifcr(other)
    }
}

impl ::core::fmt::Display for Ifcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.perrcf() != 0 { try!(write!(f, " perrcf"))}
        if self.ovrcf() != 0 { try!(write!(f, " ovrcf"))}
        if self.sbdcf() != 0 { try!(write!(f, " sbdcf"))}
        if self.syncdcf() != 0 { try!(write!(f, " syncdcf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data input register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="Parity Error bit"]
    #[inline] pub fn dr(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if DR != 0"]
    #[inline] pub fn test_dr(&self) -> bool {
        self.dr() != 0
    }

    #[doc="Sets the DR field."]
    #[inline] pub fn set_dr<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Parity Error bit"]
    #[inline] pub fn pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Validity bit"]
    #[inline] pub fn v(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if V != 0"]
    #[inline] pub fn test_v(&self) -> bool {
        self.v() != 0
    }

    #[doc="Sets the V field."]
    #[inline] pub fn set_v<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="User bit"]
    #[inline] pub fn u(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if U != 0"]
    #[inline] pub fn test_u(&self) -> bool {
        self.u() != 0
    }

    #[doc="Sets the U field."]
    #[inline] pub fn set_u<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Channel Status bit"]
    #[inline] pub fn c(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if C != 0"]
    #[inline] pub fn test_c(&self) -> bool {
        self.c() != 0
    }

    #[doc="Sets the C field."]
    #[inline] pub fn set_c<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Preamble Type"]
    #[inline] pub fn pt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if PT != 0"]
    #[inline] pub fn test_pt(&self) -> bool {
        self.pt() != 0
    }

    #[doc="Sets the PT field."]
    #[inline] pub fn set_pt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Dr {
    #[inline]
    fn from(other: u32) -> Self {
         Dr(other)
    }
}

impl ::core::fmt::Display for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dr() != 0 { try!(write!(f, " dr=0x{:x}", self.dr()))}
        if self.pe() != 0 { try!(write!(f, " pe"))}
        if self.v() != 0 { try!(write!(f, " v"))}
        if self.u() != 0 { try!(write!(f, " u"))}
        if self.c() != 0 { try!(write!(f, " c"))}
        if self.pt() != 0 { try!(write!(f, " pt=0x{:x}", self.pt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc="User data information"]
    #[inline] pub fn usr(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if USR != 0"]
    #[inline] pub fn test_usr(&self) -> bool {
        self.usr() != 0
    }

    #[doc="Sets the USR field."]
    #[inline] pub fn set_usr<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel A status information"]
    #[inline] pub fn cs(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CS != 0"]
    #[inline] pub fn test_cs(&self) -> bool {
        self.cs() != 0
    }

    #[doc="Sets the CS field."]
    #[inline] pub fn set_cs<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Start Of Block"]
    #[inline] pub fn sob(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SOB != 0"]
    #[inline] pub fn test_sob(&self) -> bool {
        self.sob() != 0
    }

    #[doc="Sets the SOB field."]
    #[inline] pub fn set_sob<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Csr(other)
    }
}

impl ::core::fmt::Display for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usr() != 0 { try!(write!(f, " usr=0x{:x}", self.usr()))}
        if self.cs() != 0 { try!(write!(f, " cs=0x{:x}", self.cs()))}
        if self.sob() != 0 { try!(write!(f, " sob"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Information register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dir(pub u32);
impl Dir {
    #[doc="Threshold HIGH"]
    #[inline] pub fn thi(&self) -> bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
    }

    #[doc="Returns true if THI != 0"]
    #[inline] pub fn test_thi(&self) -> bool {
        self.thi() != 0
    }

    #[doc="Sets the THI field."]
    #[inline] pub fn set_thi<V: Into<bits::U13>>(mut self, value: V) -> Self {
        let value: bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Threshold LOW"]
    #[inline] pub fn tlo(&self) -> bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1fff) as u16) } // [28:16]
    }

    #[doc="Returns true if TLO != 0"]
    #[inline] pub fn test_tlo(&self) -> bool {
        self.tlo() != 0
    }

    #[doc="Sets the TLO field."]
    #[inline] pub fn set_tlo<V: Into<bits::U13>>(mut self, value: V) -> Self {
        let value: bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Dir {
    #[inline]
    fn from(other: u32) -> Self {
         Dir(other)
    }
}

impl ::core::fmt::Display for Dir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.thi() != 0 { try!(write!(f, " thi=0x{:x}", self.thi()))}
        if self.tlo() != 0 { try!(write!(f, " tlo=0x{:x}", self.tlo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


