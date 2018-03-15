#[allow(unused_imports)] use ::bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="AES Peripheral"]
pub struct AesPeriph(pub usize); 

impl AesPeriph {
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

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        (self.0 + 0x4) as *mut Sr
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

    #[doc="Get the *mut pointer for the DINR register."]
    #[inline] pub fn dinr_mut(&self) -> *mut Dinr { 
        (self.0 + 0x8) as *mut Dinr
    }

    #[doc="Get the *const pointer for the DINR register."]
    #[inline] pub fn dinr_ptr(&self) -> *const Dinr { 
           self.dinr_mut()
    }

    #[doc="Read the DINR register."]
    #[inline] pub fn dinr(&self) -> Dinr { 
        unsafe {
            read_volatile(self.dinr_ptr())
        }
    }

    #[doc="Write the DINR register."]
    #[inline] pub fn set_dinr<F: FnOnce(Dinr) -> Dinr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dinr_mut(), f(Dinr(0)));
        }
        self
    }

    #[doc="Modify the DINR register."]
    #[inline] pub fn with_dinr<F: FnOnce(Dinr) -> Dinr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dinr_mut(), f(self.dinr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DOUTR register."]
    #[inline] pub fn doutr_mut(&self) -> *mut Doutr { 
        (self.0 + 0xc) as *mut Doutr
    }

    #[doc="Get the *const pointer for the DOUTR register."]
    #[inline] pub fn doutr_ptr(&self) -> *const Doutr { 
           self.doutr_mut()
    }

    #[doc="Read the DOUTR register."]
    #[inline] pub fn doutr(&self) -> Doutr { 
        unsafe {
            read_volatile(self.doutr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the KEYR0 register."]
    #[inline] pub fn keyr0_mut(&self) -> *mut Keyr0 { 
        (self.0 + 0x10) as *mut Keyr0
    }

    #[doc="Get the *const pointer for the KEYR0 register."]
    #[inline] pub fn keyr0_ptr(&self) -> *const Keyr0 { 
           self.keyr0_mut()
    }

    #[doc="Read the KEYR0 register."]
    #[inline] pub fn keyr0(&self) -> Keyr0 { 
        unsafe {
            read_volatile(self.keyr0_ptr())
        }
    }

    #[doc="Write the KEYR0 register."]
    #[inline] pub fn set_keyr0<F: FnOnce(Keyr0) -> Keyr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.keyr0_mut(), f(Keyr0(0)));
        }
        self
    }

    #[doc="Modify the KEYR0 register."]
    #[inline] pub fn with_keyr0<F: FnOnce(Keyr0) -> Keyr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.keyr0_mut(), f(self.keyr0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the KEYR1 register."]
    #[inline] pub fn keyr1_mut(&self) -> *mut Keyr1 { 
        (self.0 + 0x14) as *mut Keyr1
    }

    #[doc="Get the *const pointer for the KEYR1 register."]
    #[inline] pub fn keyr1_ptr(&self) -> *const Keyr1 { 
           self.keyr1_mut()
    }

    #[doc="Read the KEYR1 register."]
    #[inline] pub fn keyr1(&self) -> Keyr1 { 
        unsafe {
            read_volatile(self.keyr1_ptr())
        }
    }

    #[doc="Write the KEYR1 register."]
    #[inline] pub fn set_keyr1<F: FnOnce(Keyr1) -> Keyr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.keyr1_mut(), f(Keyr1(0)));
        }
        self
    }

    #[doc="Modify the KEYR1 register."]
    #[inline] pub fn with_keyr1<F: FnOnce(Keyr1) -> Keyr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.keyr1_mut(), f(self.keyr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the KEYR2 register."]
    #[inline] pub fn keyr2_mut(&self) -> *mut Keyr2 { 
        (self.0 + 0x18) as *mut Keyr2
    }

    #[doc="Get the *const pointer for the KEYR2 register."]
    #[inline] pub fn keyr2_ptr(&self) -> *const Keyr2 { 
           self.keyr2_mut()
    }

    #[doc="Read the KEYR2 register."]
    #[inline] pub fn keyr2(&self) -> Keyr2 { 
        unsafe {
            read_volatile(self.keyr2_ptr())
        }
    }

    #[doc="Write the KEYR2 register."]
    #[inline] pub fn set_keyr2<F: FnOnce(Keyr2) -> Keyr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.keyr2_mut(), f(Keyr2(0)));
        }
        self
    }

    #[doc="Modify the KEYR2 register."]
    #[inline] pub fn with_keyr2<F: FnOnce(Keyr2) -> Keyr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.keyr2_mut(), f(self.keyr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the KEYR3 register."]
    #[inline] pub fn keyr3_mut(&self) -> *mut Keyr3 { 
        (self.0 + 0x1c) as *mut Keyr3
    }

    #[doc="Get the *const pointer for the KEYR3 register."]
    #[inline] pub fn keyr3_ptr(&self) -> *const Keyr3 { 
           self.keyr3_mut()
    }

    #[doc="Read the KEYR3 register."]
    #[inline] pub fn keyr3(&self) -> Keyr3 { 
        unsafe {
            read_volatile(self.keyr3_ptr())
        }
    }

    #[doc="Write the KEYR3 register."]
    #[inline] pub fn set_keyr3<F: FnOnce(Keyr3) -> Keyr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.keyr3_mut(), f(Keyr3(0)));
        }
        self
    }

    #[doc="Modify the KEYR3 register."]
    #[inline] pub fn with_keyr3<F: FnOnce(Keyr3) -> Keyr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.keyr3_mut(), f(self.keyr3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IVR0 register."]
    #[inline] pub fn ivr0_mut(&self) -> *mut Ivr0 { 
        (self.0 + 0x20) as *mut Ivr0
    }

    #[doc="Get the *const pointer for the IVR0 register."]
    #[inline] pub fn ivr0_ptr(&self) -> *const Ivr0 { 
           self.ivr0_mut()
    }

    #[doc="Read the IVR0 register."]
    #[inline] pub fn ivr0(&self) -> Ivr0 { 
        unsafe {
            read_volatile(self.ivr0_ptr())
        }
    }

    #[doc="Write the IVR0 register."]
    #[inline] pub fn set_ivr0<F: FnOnce(Ivr0) -> Ivr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ivr0_mut(), f(Ivr0(0)));
        }
        self
    }

    #[doc="Modify the IVR0 register."]
    #[inline] pub fn with_ivr0<F: FnOnce(Ivr0) -> Ivr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ivr0_mut(), f(self.ivr0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IVR1 register."]
    #[inline] pub fn ivr1_mut(&self) -> *mut Ivr1 { 
        (self.0 + 0x24) as *mut Ivr1
    }

    #[doc="Get the *const pointer for the IVR1 register."]
    #[inline] pub fn ivr1_ptr(&self) -> *const Ivr1 { 
           self.ivr1_mut()
    }

    #[doc="Read the IVR1 register."]
    #[inline] pub fn ivr1(&self) -> Ivr1 { 
        unsafe {
            read_volatile(self.ivr1_ptr())
        }
    }

    #[doc="Write the IVR1 register."]
    #[inline] pub fn set_ivr1<F: FnOnce(Ivr1) -> Ivr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ivr1_mut(), f(Ivr1(0)));
        }
        self
    }

    #[doc="Modify the IVR1 register."]
    #[inline] pub fn with_ivr1<F: FnOnce(Ivr1) -> Ivr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ivr1_mut(), f(self.ivr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IVR2 register."]
    #[inline] pub fn ivr2_mut(&self) -> *mut Ivr2 { 
        (self.0 + 0x28) as *mut Ivr2
    }

    #[doc="Get the *const pointer for the IVR2 register."]
    #[inline] pub fn ivr2_ptr(&self) -> *const Ivr2 { 
           self.ivr2_mut()
    }

    #[doc="Read the IVR2 register."]
    #[inline] pub fn ivr2(&self) -> Ivr2 { 
        unsafe {
            read_volatile(self.ivr2_ptr())
        }
    }

    #[doc="Write the IVR2 register."]
    #[inline] pub fn set_ivr2<F: FnOnce(Ivr2) -> Ivr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ivr2_mut(), f(Ivr2(0)));
        }
        self
    }

    #[doc="Modify the IVR2 register."]
    #[inline] pub fn with_ivr2<F: FnOnce(Ivr2) -> Ivr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ivr2_mut(), f(self.ivr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IVR3 register."]
    #[inline] pub fn ivr3_mut(&self) -> *mut Ivr3 { 
        (self.0 + 0x2c) as *mut Ivr3
    }

    #[doc="Get the *const pointer for the IVR3 register."]
    #[inline] pub fn ivr3_ptr(&self) -> *const Ivr3 { 
           self.ivr3_mut()
    }

    #[doc="Read the IVR3 register."]
    #[inline] pub fn ivr3(&self) -> Ivr3 { 
        unsafe {
            read_volatile(self.ivr3_ptr())
        }
    }

    #[doc="Write the IVR3 register."]
    #[inline] pub fn set_ivr3<F: FnOnce(Ivr3) -> Ivr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ivr3_mut(), f(Ivr3(0)));
        }
        self
    }

    #[doc="Modify the IVR3 register."]
    #[inline] pub fn with_ivr3<F: FnOnce(Ivr3) -> Ivr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ivr3_mut(), f(self.ivr3()));
        }
        self
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Enable DMA management of data output phase"]
    #[inline] pub fn dmaouten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DMAOUTEN != 0"]
    #[inline] pub fn test_dmaouten(&self) -> bool {
        self.dmaouten() != 0
    }

    #[doc="Sets the DMAOUTEN field."]
    #[inline] pub fn set_dmaouten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Enable DMA management of data input phase"]
    #[inline] pub fn dmainen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DMAINEN != 0"]
    #[inline] pub fn test_dmainen(&self) -> bool {
        self.dmainen() != 0
    }

    #[doc="Sets the DMAINEN field."]
    #[inline] pub fn set_dmainen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CCF flag interrupt enable"]
    #[inline] pub fn ccfie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CCFIE != 0"]
    #[inline] pub fn test_ccfie(&self) -> bool {
        self.ccfie() != 0
    }

    #[doc="Sets the CCFIE field."]
    #[inline] pub fn set_ccfie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Error clear"]
    #[inline] pub fn errc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ERRC != 0"]
    #[inline] pub fn test_errc(&self) -> bool {
        self.errc() != 0
    }

    #[doc="Sets the ERRC field."]
    #[inline] pub fn set_errc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Computation Complete Flag Clear"]
    #[inline] pub fn ccfc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CCFC != 0"]
    #[inline] pub fn test_ccfc(&self) -> bool {
        self.ccfc() != 0
    }

    #[doc="Sets the CCFC field."]
    #[inline] pub fn set_ccfc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="AES chaining mode"]
    #[inline] pub fn chmod(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if CHMOD != 0"]
    #[inline] pub fn test_chmod(&self) -> bool {
        self.chmod() != 0
    }

    #[doc="Sets the CHMOD field."]
    #[inline] pub fn set_chmod<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="AES operating mode"]
    #[inline] pub fn mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline] pub fn datatype(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if DATATYPE != 0"]
    #[inline] pub fn test_datatype(&self) -> bool {
        self.datatype() != 0
    }

    #[doc="Sets the DATATYPE field."]
    #[inline] pub fn set_datatype<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="AES enable"]
    #[inline] pub fn en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="Sets the EN field."]
    #[inline] pub fn set_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.dmaouten() != 0 { try!(write!(f, " dmaouten"))}
        if self.dmainen() != 0 { try!(write!(f, " dmainen"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.ccfie() != 0 { try!(write!(f, " ccfie"))}
        if self.errc() != 0 { try!(write!(f, " errc"))}
        if self.ccfc() != 0 { try!(write!(f, " ccfc"))}
        if self.chmod() != 0 { try!(write!(f, " chmod=0x{:x}", self.chmod()))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.datatype() != 0 { try!(write!(f, " datatype=0x{:x}", self.datatype()))}
        if self.en() != 0 { try!(write!(f, " en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Write error flag"]
    #[inline] pub fn wrerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WRERR != 0"]
    #[inline] pub fn test_wrerr(&self) -> bool {
        self.wrerr() != 0
    }

    #[doc="Sets the WRERR field."]
    #[inline] pub fn set_wrerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Read error flag"]
    #[inline] pub fn rderr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RDERR != 0"]
    #[inline] pub fn test_rderr(&self) -> bool {
        self.rderr() != 0
    }

    #[doc="Sets the RDERR field."]
    #[inline] pub fn set_rderr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Computation complete flag"]
    #[inline] pub fn ccf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CCF != 0"]
    #[inline] pub fn test_ccf(&self) -> bool {
        self.ccf() != 0
    }

    #[doc="Sets the CCF field."]
    #[inline] pub fn set_ccf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.wrerr() != 0 { try!(write!(f, " wrerr"))}
        if self.rderr() != 0 { try!(write!(f, " rderr"))}
        if self.ccf() != 0 { try!(write!(f, " ccf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data input register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dinr(pub u32);
impl Dinr {
    #[doc="Data Input Register"]
    #[inline] pub fn aes_dinr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_DINR != 0"]
    #[inline] pub fn test_aes_dinr(&self) -> bool {
        self.aes_dinr() != 0
    }

    #[doc="Sets the AES_DINR field."]
    #[inline] pub fn set_aes_dinr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dinr {
    #[inline]
    fn from(other: u32) -> Self {
         Dinr(other)
    }
}

impl ::core::fmt::Display for Dinr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dinr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data output register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doutr(pub u32);
impl Doutr {
    #[doc="Data output register"]
    #[inline] pub fn aes_doutr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_DOUTR != 0"]
    #[inline] pub fn test_aes_doutr(&self) -> bool {
        self.aes_doutr() != 0
    }

    #[doc="Sets the AES_DOUTR field."]
    #[inline] pub fn set_aes_doutr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doutr {
    #[inline]
    fn from(other: u32) -> Self {
         Doutr(other)
    }
}

impl ::core::fmt::Display for Doutr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doutr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr0(pub u32);
impl Keyr0 {
    #[doc="Data Output Register (LSB key [31:0])"]
    #[inline] pub fn aes_keyr0(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR0 != 0"]
    #[inline] pub fn test_aes_keyr0(&self) -> bool {
        self.aes_keyr0() != 0
    }

    #[doc="Sets the AES_KEYR0 field."]
    #[inline] pub fn set_aes_keyr0<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr0(other)
    }
}

impl ::core::fmt::Display for Keyr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr1(pub u32);
impl Keyr1 {
    #[doc="AES key register (key [63:32])"]
    #[inline] pub fn aes_keyr1(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR1 != 0"]
    #[inline] pub fn test_aes_keyr1(&self) -> bool {
        self.aes_keyr1() != 0
    }

    #[doc="Sets the AES_KEYR1 field."]
    #[inline] pub fn set_aes_keyr1<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr1(other)
    }
}

impl ::core::fmt::Display for Keyr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr2(pub u32);
impl Keyr2 {
    #[doc="AES key register (key [95:64])"]
    #[inline] pub fn aes_keyr2(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR2 != 0"]
    #[inline] pub fn test_aes_keyr2(&self) -> bool {
        self.aes_keyr2() != 0
    }

    #[doc="Sets the AES_KEYR2 field."]
    #[inline] pub fn set_aes_keyr2<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr2(other)
    }
}

impl ::core::fmt::Display for Keyr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr3(pub u32);
impl Keyr3 {
    #[doc="AES key register (MSB key [127:96])"]
    #[inline] pub fn aes_keyr3(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR3 != 0"]
    #[inline] pub fn test_aes_keyr3(&self) -> bool {
        self.aes_keyr3() != 0
    }

    #[doc="Sets the AES_KEYR3 field."]
    #[inline] pub fn set_aes_keyr3<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr3(other)
    }
}

impl ::core::fmt::Display for Keyr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ivr0(pub u32);
impl Ivr0 {
    #[doc="initialization vector register (LSB IVR [31:0])"]
    #[inline] pub fn aes_ivr0(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_IVR0 != 0"]
    #[inline] pub fn test_aes_ivr0(&self) -> bool {
        self.aes_ivr0() != 0
    }

    #[doc="Sets the AES_IVR0 field."]
    #[inline] pub fn set_aes_ivr0<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ivr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Ivr0(other)
    }
}

impl ::core::fmt::Display for Ivr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ivr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ivr1(pub u32);
impl Ivr1 {
    #[doc="Initialization Vector Register (IVR [63:32])"]
    #[inline] pub fn aes_ivr1(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_IVR1 != 0"]
    #[inline] pub fn test_aes_ivr1(&self) -> bool {
        self.aes_ivr1() != 0
    }

    #[doc="Sets the AES_IVR1 field."]
    #[inline] pub fn set_aes_ivr1<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ivr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Ivr1(other)
    }
}

impl ::core::fmt::Display for Ivr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ivr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ivr2(pub u32);
impl Ivr2 {
    #[doc="Initialization Vector Register (IVR [95:64])"]
    #[inline] pub fn aes_ivr2(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_IVR2 != 0"]
    #[inline] pub fn test_aes_ivr2(&self) -> bool {
        self.aes_ivr2() != 0
    }

    #[doc="Sets the AES_IVR2 field."]
    #[inline] pub fn set_aes_ivr2<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ivr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Ivr2(other)
    }
}

impl ::core::fmt::Display for Ivr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ivr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ivr3(pub u32);
impl Ivr3 {
    #[doc="Initialization Vector Register (MSB IVR [127:96])"]
    #[inline] pub fn aes_ivr3(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_IVR3 != 0"]
    #[inline] pub fn test_aes_ivr3(&self) -> bool {
        self.aes_ivr3() != 0
    }

    #[doc="Sets the AES_IVR3 field."]
    #[inline] pub fn set_aes_ivr3<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ivr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Ivr3(other)
    }
}

impl ::core::fmt::Display for Ivr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ivr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

