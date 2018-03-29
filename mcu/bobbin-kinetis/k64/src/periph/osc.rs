#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Oscillator"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OscPeriph(pub usize);
impl OscPeriph {
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

}

#[doc="OSC Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u8);
impl Cr {
    #[doc="Oscillator 16 pF Capacitor Load Configure"]
    #[inline] pub fn sc16p(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SC16P != 0"]
    #[inline] pub fn test_sc16p(&self) -> bool {
        self.sc16p() != 0
    }

    #[doc="Sets the SC16P field."]
    #[inline] pub fn set_sc16p<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Oscillator 8 pF Capacitor Load Configure"]
    #[inline] pub fn sc8p(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SC8P != 0"]
    #[inline] pub fn test_sc8p(&self) -> bool {
        self.sc8p() != 0
    }

    #[doc="Sets the SC8P field."]
    #[inline] pub fn set_sc8p<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Oscillator 4 pF Capacitor Load Configure"]
    #[inline] pub fn sc4p(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SC4P != 0"]
    #[inline] pub fn test_sc4p(&self) -> bool {
        self.sc4p() != 0
    }

    #[doc="Sets the SC4P field."]
    #[inline] pub fn set_sc4p<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Oscillator 2 pF Capacitor Load Configure"]
    #[inline] pub fn sc2p(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SC2P != 0"]
    #[inline] pub fn test_sc2p(&self) -> bool {
        self.sc2p() != 0
    }

    #[doc="Sets the SC2P field."]
    #[inline] pub fn set_sc2p<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="External Reference Stop Enable"]
    #[inline] pub fn erefsten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if EREFSTEN != 0"]
    #[inline] pub fn test_erefsten(&self) -> bool {
        self.erefsten() != 0
    }

    #[doc="Sets the EREFSTEN field."]
    #[inline] pub fn set_erefsten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="External Reference Enable"]
    #[inline] pub fn erclken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ERCLKEN != 0"]
    #[inline] pub fn test_erclken(&self) -> bool {
        self.erclken() != 0
    }

    #[doc="Sets the ERCLKEN field."]
    #[inline] pub fn set_erclken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Cr {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.sc16p() != 0 { try!(write!(f, " sc16p"))}
        if self.sc8p() != 0 { try!(write!(f, " sc8p"))}
        if self.sc4p() != 0 { try!(write!(f, " sc4p"))}
        if self.sc2p() != 0 { try!(write!(f, " sc2p"))}
        if self.erefsten() != 0 { try!(write!(f, " erefsten"))}
        if self.erclken() != 0 { try!(write!(f, " erclken"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

