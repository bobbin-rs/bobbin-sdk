#[allow(unused_imports)] use bobbin_common::*;

periph!( UART0, Uart0, _UART0, UartPeriph, 0x4001c000);
periph!( UART1, Uart1, _UART1, UartPeriph, 0x4001d000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="UART Peripheral"]
pub struct UartPeriph(pub usize); 

impl super::sig::Signal<super::sig::Uart0tx> for Uart0 {}
impl super::sig::SignalUartTx<super::sig::Uart0tx> for Uart0 {}
impl super::sig::Signal<super::sig::Uart0rx> for Uart0 {}
impl super::sig::SignalUartRx<super::sig::Uart0rx> for Uart0 {}
impl super::sig::Signal<super::sig::Ua0cts> for Uart0 {}
impl super::sig::SignalUartCts<super::sig::Ua0cts> for Uart0 {}
impl super::sig::Signal<super::sig::Ua0rts> for Uart0 {}
impl super::sig::SignalUartRts<super::sig::Ua0rts> for Uart0 {}

impl super::sig::Signal<super::sig::Uart1tx> for Uart1 {}
impl super::sig::SignalUartTx<super::sig::Uart1tx> for Uart1 {}
impl super::sig::Signal<super::sig::Uart1rx> for Uart1 {}
impl super::sig::SignalUartRx<super::sig::Uart1rx> for Uart1 {}
impl super::sig::Signal<super::sig::Ua1cts> for Uart1 {}
impl super::sig::SignalUartCts<super::sig::Ua1cts> for Uart1 {}
impl super::sig::Signal<super::sig::Ua1rts> for Uart1 {}
impl super::sig::SignalUartRts<super::sig::Ua1rts> for Uart1 {}


impl UartPeriph {
    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut Dr { 
        (self.0 + 0x0) as *mut Dr
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

    #[doc="Write the DR register."]
    #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dr_mut(), f(Dr(0)));
        }
        self
    }

    #[doc="Modify the DR register."]
    #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dr_mut(), f(self.dr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RSR register."]
    #[inline] pub fn rsr_mut(&self) -> *mut Rsr { 
        (self.0 + 0x4) as *mut Rsr
    }

    #[doc="Get the *const pointer for the RSR register."]
    #[inline] pub fn rsr_ptr(&self) -> *const Rsr { 
           self.rsr_mut()
    }

    #[doc="Read the RSR register."]
    #[inline] pub fn rsr(&self) -> Rsr { 
        unsafe {
            read_volatile(self.rsr_ptr())
        }
    }

    #[doc="Write the RSR register."]
    #[inline] pub fn set_rsr<F: FnOnce(Rsr) -> Rsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rsr_mut(), f(Rsr(0)));
        }
        self
    }

    #[doc="Modify the RSR register."]
    #[inline] pub fn with_rsr<F: FnOnce(Rsr) -> Rsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rsr_mut(), f(self.rsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FR register."]
    #[inline] pub fn fr_mut(&self) -> *mut Fr { 
        (self.0 + 0x18) as *mut Fr
    }

    #[doc="Get the *const pointer for the FR register."]
    #[inline] pub fn fr_ptr(&self) -> *const Fr { 
           self.fr_mut()
    }

    #[doc="Read the FR register."]
    #[inline] pub fn fr(&self) -> Fr { 
        unsafe {
            read_volatile(self.fr_ptr())
        }
    }

    #[doc="Write the FR register."]
    #[inline] pub fn set_fr<F: FnOnce(Fr) -> Fr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fr_mut(), f(Fr(0)));
        }
        self
    }

    #[doc="Modify the FR register."]
    #[inline] pub fn with_fr<F: FnOnce(Fr) -> Fr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fr_mut(), f(self.fr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ILPR register."]
    #[inline] pub fn ilpr_mut(&self) -> *mut Ilpr { 
        (self.0 + 0x20) as *mut Ilpr
    }

    #[doc="Get the *const pointer for the ILPR register."]
    #[inline] pub fn ilpr_ptr(&self) -> *const Ilpr { 
           self.ilpr_mut()
    }

    #[doc="Read the ILPR register."]
    #[inline] pub fn ilpr(&self) -> Ilpr { 
        unsafe {
            read_volatile(self.ilpr_ptr())
        }
    }

    #[doc="Write the ILPR register."]
    #[inline] pub fn set_ilpr<F: FnOnce(Ilpr) -> Ilpr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ilpr_mut(), f(Ilpr(0)));
        }
        self
    }

    #[doc="Modify the ILPR register."]
    #[inline] pub fn with_ilpr<F: FnOnce(Ilpr) -> Ilpr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ilpr_mut(), f(self.ilpr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IBRD register."]
    #[inline] pub fn ibrd_mut(&self) -> *mut Ibrd { 
        (self.0 + 0x24) as *mut Ibrd
    }

    #[doc="Get the *const pointer for the IBRD register."]
    #[inline] pub fn ibrd_ptr(&self) -> *const Ibrd { 
           self.ibrd_mut()
    }

    #[doc="Read the IBRD register."]
    #[inline] pub fn ibrd(&self) -> Ibrd { 
        unsafe {
            read_volatile(self.ibrd_ptr())
        }
    }

    #[doc="Write the IBRD register."]
    #[inline] pub fn set_ibrd<F: FnOnce(Ibrd) -> Ibrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ibrd_mut(), f(Ibrd(0)));
        }
        self
    }

    #[doc="Modify the IBRD register."]
    #[inline] pub fn with_ibrd<F: FnOnce(Ibrd) -> Ibrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ibrd_mut(), f(self.ibrd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FBRD register."]
    #[inline] pub fn fbrd_mut(&self) -> *mut Fbrd { 
        (self.0 + 0x28) as *mut Fbrd
    }

    #[doc="Get the *const pointer for the FBRD register."]
    #[inline] pub fn fbrd_ptr(&self) -> *const Fbrd { 
           self.fbrd_mut()
    }

    #[doc="Read the FBRD register."]
    #[inline] pub fn fbrd(&self) -> Fbrd { 
        unsafe {
            read_volatile(self.fbrd_ptr())
        }
    }

    #[doc="Write the FBRD register."]
    #[inline] pub fn set_fbrd<F: FnOnce(Fbrd) -> Fbrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fbrd_mut(), f(Fbrd(0)));
        }
        self
    }

    #[doc="Modify the FBRD register."]
    #[inline] pub fn with_fbrd<F: FnOnce(Fbrd) -> Fbrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fbrd_mut(), f(self.fbrd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LCRH register."]
    #[inline] pub fn lcrh_mut(&self) -> *mut Lcrh { 
        (self.0 + 0x2c) as *mut Lcrh
    }

    #[doc="Get the *const pointer for the LCRH register."]
    #[inline] pub fn lcrh_ptr(&self) -> *const Lcrh { 
           self.lcrh_mut()
    }

    #[doc="Read the LCRH register."]
    #[inline] pub fn lcrh(&self) -> Lcrh { 
        unsafe {
            read_volatile(self.lcrh_ptr())
        }
    }

    #[doc="Write the LCRH register."]
    #[inline] pub fn set_lcrh<F: FnOnce(Lcrh) -> Lcrh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lcrh_mut(), f(Lcrh(0)));
        }
        self
    }

    #[doc="Modify the LCRH register."]
    #[inline] pub fn with_lcrh<F: FnOnce(Lcrh) -> Lcrh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lcrh_mut(), f(self.lcrh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        (self.0 + 0x30) as *mut Cr
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

    #[doc="Get the *mut pointer for the IFLS register."]
    #[inline] pub fn ifls_mut(&self) -> *mut Ifls { 
        (self.0 + 0x34) as *mut Ifls
    }

    #[doc="Get the *const pointer for the IFLS register."]
    #[inline] pub fn ifls_ptr(&self) -> *const Ifls { 
           self.ifls_mut()
    }

    #[doc="Read the IFLS register."]
    #[inline] pub fn ifls(&self) -> Ifls { 
        unsafe {
            read_volatile(self.ifls_ptr())
        }
    }

    #[doc="Write the IFLS register."]
    #[inline] pub fn set_ifls<F: FnOnce(Ifls) -> Ifls>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifls_mut(), f(Ifls(0)));
        }
        self
    }

    #[doc="Modify the IFLS register."]
    #[inline] pub fn with_ifls<F: FnOnce(Ifls) -> Ifls>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifls_mut(), f(self.ifls()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        (self.0 + 0x38) as *mut Ier
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

    #[doc="Get the *mut pointer for the IES register."]
    #[inline] pub fn ies_mut(&self) -> *mut Ies { 
        (self.0 + 0x3c) as *mut Ies
    }

    #[doc="Get the *const pointer for the IES register."]
    #[inline] pub fn ies_ptr(&self) -> *const Ies { 
           self.ies_mut()
    }

    #[doc="Read the IES register."]
    #[inline] pub fn ies(&self) -> Ies { 
        unsafe {
            read_volatile(self.ies_ptr())
        }
    }

    #[doc="Write the IES register."]
    #[inline] pub fn set_ies<F: FnOnce(Ies) -> Ies>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ies_mut(), f(Ies(0)));
        }
        self
    }

    #[doc="Modify the IES register."]
    #[inline] pub fn with_ies<F: FnOnce(Ies) -> Ies>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ies_mut(), f(self.ies()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MIS register."]
    #[inline] pub fn mis_mut(&self) -> *mut Mis { 
        (self.0 + 0x40) as *mut Mis
    }

    #[doc="Get the *const pointer for the MIS register."]
    #[inline] pub fn mis_ptr(&self) -> *const Mis { 
           self.mis_mut()
    }

    #[doc="Read the MIS register."]
    #[inline] pub fn mis(&self) -> Mis { 
        unsafe {
            read_volatile(self.mis_ptr())
        }
    }

    #[doc="Write the MIS register."]
    #[inline] pub fn set_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mis_mut(), f(Mis(0)));
        }
        self
    }

    #[doc="Modify the MIS register."]
    #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mis_mut(), f(self.mis()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IEC register."]
    #[inline] pub fn iec_mut(&self) -> *mut Iec { 
        (self.0 + 0x44) as *mut Iec
    }

    #[doc="Get the *const pointer for the IEC register."]
    #[inline] pub fn iec_ptr(&self) -> *const Iec { 
           self.iec_mut()
    }

    #[doc="Read the IEC register."]
    #[inline] pub fn iec(&self) -> Iec { 
        unsafe {
            read_volatile(self.iec_ptr())
        }
    }

    #[doc="Write the IEC register."]
    #[inline] pub fn set_iec<F: FnOnce(Iec) -> Iec>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iec_mut(), f(Iec(0)));
        }
        self
    }

    #[doc="Modify the IEC register."]
    #[inline] pub fn with_iec<F: FnOnce(Iec) -> Iec>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iec_mut(), f(self.iec()));
        }
        self
    }

}

#[doc="UART Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="This is the overrun error indicator."]
    #[inline] pub fn oedata(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if OEDATA != 0"]
    #[inline] pub fn test_oedata(&self) -> bool {
        self.oedata() != 0
    }

    #[doc="Sets the OEDATA field."]
    #[inline] pub fn set_oedata<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="This is the break error indicator."]
    #[inline] pub fn bedata(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BEDATA != 0"]
    #[inline] pub fn test_bedata(&self) -> bool {
        self.bedata() != 0
    }

    #[doc="Sets the BEDATA field."]
    #[inline] pub fn set_bedata<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This is the parity error indicator."]
    #[inline] pub fn pedata(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PEDATA != 0"]
    #[inline] pub fn test_pedata(&self) -> bool {
        self.pedata() != 0
    }

    #[doc="Sets the PEDATA field."]
    #[inline] pub fn set_pedata<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This is the framing error indicator."]
    #[inline] pub fn fedata(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FEDATA != 0"]
    #[inline] pub fn test_fedata(&self) -> bool {
        self.fedata() != 0
    }

    #[doc="Sets the FEDATA field."]
    #[inline] pub fn set_fedata<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This is the UART data port."]
    #[inline] pub fn data(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
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
        if self.oedata() != 0 { try!(write!(f, " oedata"))}
        if self.bedata() != 0 { try!(write!(f, " bedata"))}
        if self.pedata() != 0 { try!(write!(f, " pedata"))}
        if self.fedata() != 0 { try!(write!(f, " fedata"))}
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rsr(pub u32);
impl Rsr {
    #[doc="This is the overrun error indicator."]
    #[inline] pub fn oestat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OESTAT != 0"]
    #[inline] pub fn test_oestat(&self) -> bool {
        self.oestat() != 0
    }

    #[doc="Sets the OESTAT field."]
    #[inline] pub fn set_oestat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This is the break error indicator."]
    #[inline] pub fn bestat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BESTAT != 0"]
    #[inline] pub fn test_bestat(&self) -> bool {
        self.bestat() != 0
    }

    #[doc="Sets the BESTAT field."]
    #[inline] pub fn set_bestat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This is the parity error indicator."]
    #[inline] pub fn pestat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PESTAT != 0"]
    #[inline] pub fn test_pestat(&self) -> bool {
        self.pestat() != 0
    }

    #[doc="Sets the PESTAT field."]
    #[inline] pub fn set_pestat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This is the framing error indicator."]
    #[inline] pub fn festat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FESTAT != 0"]
    #[inline] pub fn test_festat(&self) -> bool {
        self.festat() != 0
    }

    #[doc="Sets the FESTAT field."]
    #[inline] pub fn set_festat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rsr {
    #[inline]
    fn from(other: u32) -> Self {
         Rsr(other)
    }
}

impl ::core::fmt::Display for Rsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oestat() != 0 { try!(write!(f, " oestat"))}
        if self.bestat() != 0 { try!(write!(f, " bestat"))}
        if self.pestat() != 0 { try!(write!(f, " pestat"))}
        if self.festat() != 0 { try!(write!(f, " festat"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flag Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fr(pub u32);
impl Fr {
    #[doc="This bit holds the transmit BUSY indicator."]
    #[inline] pub fn txbusy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXBUSY != 0"]
    #[inline] pub fn test_txbusy(&self) -> bool {
        self.txbusy() != 0
    }

    #[doc="Sets the TXBUSY field."]
    #[inline] pub fn set_txbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit holds the transmit FIFO empty indicator."]
    #[inline] pub fn txfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFE != 0"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe() != 0
    }

    #[doc="Sets the TXFE field."]
    #[inline] pub fn set_txfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="This bit holds the receive FIFO full indicator."]
    #[inline] pub fn rxff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXFF != 0"]
    #[inline] pub fn test_rxff(&self) -> bool {
        self.rxff() != 0
    }

    #[doc="Sets the RXFF field."]
    #[inline] pub fn set_rxff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This bit holds the transmit FIFO full indicator."]
    #[inline] pub fn txff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXFF != 0"]
    #[inline] pub fn test_txff(&self) -> bool {
        self.txff() != 0
    }

    #[doc="Sets the TXFF field."]
    #[inline] pub fn set_txff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This bit holds the receive FIFO empty indicator."]
    #[inline] pub fn rxfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXFE != 0"]
    #[inline] pub fn test_rxfe(&self) -> bool {
        self.rxfe() != 0
    }

    #[doc="Sets the RXFE field."]
    #[inline] pub fn set_rxfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit holds the busy indicator."]
    #[inline] pub fn busy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit holds the data carrier detect indicator."]
    #[inline] pub fn dcd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCD != 0"]
    #[inline] pub fn test_dcd(&self) -> bool {
        self.dcd() != 0
    }

    #[doc="Sets the DCD field."]
    #[inline] pub fn set_dcd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit holds the data set ready indicator."]
    #[inline] pub fn dsr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DSR != 0"]
    #[inline] pub fn test_dsr(&self) -> bool {
        self.dsr() != 0
    }

    #[doc="Sets the DSR field."]
    #[inline] pub fn set_dsr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit holds the clear to send indicator."]
    #[inline] pub fn cts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTS != 0"]
    #[inline] pub fn test_cts(&self) -> bool {
        self.cts() != 0
    }

    #[doc="Sets the CTS field."]
    #[inline] pub fn set_cts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fr {
    #[inline]
    fn from(other: u32) -> Self {
         Fr(other)
    }
}

impl ::core::fmt::Display for Fr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txbusy() != 0 { try!(write!(f, " txbusy"))}
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.rxff() != 0 { try!(write!(f, " rxff"))}
        if self.txff() != 0 { try!(write!(f, " txff"))}
        if self.rxfe() != 0 { try!(write!(f, " rxfe"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.dcd() != 0 { try!(write!(f, " dcd"))}
        if self.dsr() != 0 { try!(write!(f, " dsr"))}
        if self.cts() != 0 { try!(write!(f, " cts"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IrDA Counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ilpr(pub u32);
impl Ilpr {
    #[doc="These bits hold the IrDA counter divisor."]
    #[inline] pub fn ilpdvsr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ILPDVSR != 0"]
    #[inline] pub fn test_ilpdvsr(&self) -> bool {
        self.ilpdvsr() != 0
    }

    #[doc="Sets the ILPDVSR field."]
    #[inline] pub fn set_ilpdvsr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ilpr {
    #[inline]
    fn from(other: u32) -> Self {
         Ilpr(other)
    }
}

impl ::core::fmt::Display for Ilpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ilpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ilpdvsr() != 0 { try!(write!(f, " ilpdvsr=0x{:x}", self.ilpdvsr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Integer Baud Rate Divisor"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ibrd(pub u32);
impl Ibrd {
    #[doc="These bits hold the baud integer divisor."]
    #[inline] pub fn divint(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DIVINT != 0"]
    #[inline] pub fn test_divint(&self) -> bool {
        self.divint() != 0
    }

    #[doc="Sets the DIVINT field."]
    #[inline] pub fn set_divint<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ibrd {
    #[inline]
    fn from(other: u32) -> Self {
         Ibrd(other)
    }
}

impl ::core::fmt::Display for Ibrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ibrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.divint() != 0 { try!(write!(f, " divint=0x{:x}", self.divint()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fractional Baud Rate Divisor"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fbrd(pub u32);
impl Fbrd {
    #[doc="These bits hold the baud fractional divisor."]
    #[inline] pub fn divfrac(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if DIVFRAC != 0"]
    #[inline] pub fn test_divfrac(&self) -> bool {
        self.divfrac() != 0
    }

    #[doc="Sets the DIVFRAC field."]
    #[inline] pub fn set_divfrac<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fbrd {
    #[inline]
    fn from(other: u32) -> Self {
         Fbrd(other)
    }
}

impl ::core::fmt::Display for Fbrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fbrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.divfrac() != 0 { try!(write!(f, " divfrac=0x{:x}", self.divfrac()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Line Control High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lcrh(pub u32);
impl Lcrh {
    #[doc="This bit holds the stick parity select."]
    #[inline] pub fn sps(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SPS != 0"]
    #[inline] pub fn test_sps(&self) -> bool {
        self.sps() != 0
    }

    #[doc="Sets the SPS field."]
    #[inline] pub fn set_sps<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="These bits hold the write length."]
    #[inline] pub fn wlen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if WLEN != 0"]
    #[inline] pub fn test_wlen(&self) -> bool {
        self.wlen() != 0
    }

    #[doc="Sets the WLEN field."]
    #[inline] pub fn set_wlen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This bit holds the FIFO enable."]
    #[inline] pub fn fen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FEN != 0"]
    #[inline] pub fn test_fen(&self) -> bool {
        self.fen() != 0
    }

    #[doc="Sets the FEN field."]
    #[inline] pub fn set_fen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit holds the two stop bits select."]
    #[inline] pub fn stp2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STP2 != 0"]
    #[inline] pub fn test_stp2(&self) -> bool {
        self.stp2() != 0
    }

    #[doc="Sets the STP2 field."]
    #[inline] pub fn set_stp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit holds the even parity select."]
    #[inline] pub fn eps(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EPS != 0"]
    #[inline] pub fn test_eps(&self) -> bool {
        self.eps() != 0
    }

    #[doc="Sets the EPS field."]
    #[inline] pub fn set_eps<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit holds the parity enable."]
    #[inline] pub fn pen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PEN != 0"]
    #[inline] pub fn test_pen(&self) -> bool {
        self.pen() != 0
    }

    #[doc="Sets the PEN field."]
    #[inline] pub fn set_pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit holds the break set."]
    #[inline] pub fn brk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BRK != 0"]
    #[inline] pub fn test_brk(&self) -> bool {
        self.brk() != 0
    }

    #[doc="Sets the BRK field."]
    #[inline] pub fn set_brk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lcrh {
    #[inline]
    fn from(other: u32) -> Self {
         Lcrh(other)
    }
}

impl ::core::fmt::Display for Lcrh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lcrh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sps() != 0 { try!(write!(f, " sps"))}
        if self.wlen() != 0 { try!(write!(f, " wlen=0x{:x}", self.wlen()))}
        if self.fen() != 0 { try!(write!(f, " fen"))}
        if self.stp2() != 0 { try!(write!(f, " stp2"))}
        if self.eps() != 0 { try!(write!(f, " eps"))}
        if self.pen() != 0 { try!(write!(f, " pen"))}
        if self.brk() != 0 { try!(write!(f, " brk"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="This bit enables CTS hardware flow control."]
    #[inline] pub fn ctsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTSEN != 0"]
    #[inline] pub fn test_ctsen(&self) -> bool {
        self.ctsen() != 0
    }

    #[doc="Sets the CTSEN field."]
    #[inline] pub fn set_ctsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="This bit enables RTS hardware flow control."]
    #[inline] pub fn rtsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RTSEN != 0"]
    #[inline] pub fn test_rtsen(&self) -> bool {
        self.rtsen() != 0
    }

    #[doc="Sets the RTSEN field."]
    #[inline] pub fn set_rtsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="This bit holds modem Out2."]
    #[inline] pub fn out2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if OUT2 != 0"]
    #[inline] pub fn test_out2(&self) -> bool {
        self.out2() != 0
    }

    #[doc="Sets the OUT2 field."]
    #[inline] pub fn set_out2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="This bit holds modem Out1."]
    #[inline] pub fn out1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if OUT1 != 0"]
    #[inline] pub fn test_out1(&self) -> bool {
        self.out1() != 0
    }

    #[doc="Sets the OUT1 field."]
    #[inline] pub fn set_out1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="This bit enables request to send."]
    #[inline] pub fn rts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RTS != 0"]
    #[inline] pub fn test_rts(&self) -> bool {
        self.rts() != 0
    }

    #[doc="Sets the RTS field."]
    #[inline] pub fn set_rts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="This bit enables data transmit ready."]
    #[inline] pub fn dtr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTR != 0"]
    #[inline] pub fn test_dtr(&self) -> bool {
        self.dtr() != 0
    }

    #[doc="Sets the DTR field."]
    #[inline] pub fn set_dtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This bit is the receive enable."]
    #[inline] pub fn rxe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RXE != 0"]
    #[inline] pub fn test_rxe(&self) -> bool {
        self.rxe() != 0
    }

    #[doc="Sets the RXE field."]
    #[inline] pub fn set_rxe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This bit is the transmit enable."]
    #[inline] pub fn txe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXE != 0"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe() != 0
    }

    #[doc="Sets the TXE field."]
    #[inline] pub fn set_txe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit is the loopback enable."]
    #[inline] pub fn lbe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LBE != 0"]
    #[inline] pub fn test_lbe(&self) -> bool {
        self.lbe() != 0
    }

    #[doc="Sets the LBE field."]
    #[inline] pub fn set_lbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="This bitfield is the UART clock select."]
    #[inline] pub fn clksel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if CLKSEL != 0"]
    #[inline] pub fn test_clksel(&self) -> bool {
        self.clksel() != 0
    }

    #[doc="Sets the CLKSEL field."]
    #[inline] pub fn set_clksel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit is the UART clock enable."]
    #[inline] pub fn clken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CLKEN != 0"]
    #[inline] pub fn test_clken(&self) -> bool {
        self.clken() != 0
    }

    #[doc="Sets the CLKEN field."]
    #[inline] pub fn set_clken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit is the SIR low power select."]
    #[inline] pub fn sirlp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SIRLP != 0"]
    #[inline] pub fn test_sirlp(&self) -> bool {
        self.sirlp() != 0
    }

    #[doc="Sets the SIRLP field."]
    #[inline] pub fn set_sirlp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit is the SIR ENDEC enable."]
    #[inline] pub fn siren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SIREN != 0"]
    #[inline] pub fn test_siren(&self) -> bool {
        self.siren() != 0
    }

    #[doc="Sets the SIREN field."]
    #[inline] pub fn set_siren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit is the UART enable."]
    #[inline] pub fn uarten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UARTEN != 0"]
    #[inline] pub fn test_uarten(&self) -> bool {
        self.uarten() != 0
    }

    #[doc="Sets the UARTEN field."]
    #[inline] pub fn set_uarten<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.ctsen() != 0 { try!(write!(f, " ctsen"))}
        if self.rtsen() != 0 { try!(write!(f, " rtsen"))}
        if self.out2() != 0 { try!(write!(f, " out2"))}
        if self.out1() != 0 { try!(write!(f, " out1"))}
        if self.rts() != 0 { try!(write!(f, " rts"))}
        if self.dtr() != 0 { try!(write!(f, " dtr"))}
        if self.rxe() != 0 { try!(write!(f, " rxe"))}
        if self.txe() != 0 { try!(write!(f, " txe"))}
        if self.lbe() != 0 { try!(write!(f, " lbe"))}
        if self.clksel() != 0 { try!(write!(f, " clksel=0x{:x}", self.clksel()))}
        if self.clken() != 0 { try!(write!(f, " clken"))}
        if self.sirlp() != 0 { try!(write!(f, " sirlp"))}
        if self.siren() != 0 { try!(write!(f, " siren"))}
        if self.uarten() != 0 { try!(write!(f, " uarten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO Interrupt Level Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifls(pub u32);
impl Ifls {
    #[doc="These bits hold the receive FIFO interrupt level."]
    #[inline] pub fn rxiflsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if RXIFLSEL != 0"]
    #[inline] pub fn test_rxiflsel(&self) -> bool {
        self.rxiflsel() != 0
    }

    #[doc="Sets the RXIFLSEL field."]
    #[inline] pub fn set_rxiflsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="These bits hold the transmit FIFO interrupt level."]
    #[inline] pub fn txiflsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if TXIFLSEL != 0"]
    #[inline] pub fn test_txiflsel(&self) -> bool {
        self.txiflsel() != 0
    }

    #[doc="Sets the TXIFLSEL field."]
    #[inline] pub fn set_txiflsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ifls {
    #[inline]
    fn from(other: u32) -> Self {
         Ifls(other)
    }
}

impl ::core::fmt::Display for Ifls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxiflsel() != 0 { try!(write!(f, " rxiflsel=0x{:x}", self.rxiflsel()))}
        if self.txiflsel() != 0 { try!(write!(f, " txiflsel=0x{:x}", self.txiflsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="This bit holds the overflow interrupt enable."]
    #[inline] pub fn oeim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OEIM != 0"]
    #[inline] pub fn test_oeim(&self) -> bool {
        self.oeim() != 0
    }

    #[doc="Sets the OEIM field."]
    #[inline] pub fn set_oeim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This bit holds the break error interrupt enable."]
    #[inline] pub fn beim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BEIM != 0"]
    #[inline] pub fn test_beim(&self) -> bool {
        self.beim() != 0
    }

    #[doc="Sets the BEIM field."]
    #[inline] pub fn set_beim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This bit holds the parity error interrupt enable."]
    #[inline] pub fn peim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PEIM != 0"]
    #[inline] pub fn test_peim(&self) -> bool {
        self.peim() != 0
    }

    #[doc="Sets the PEIM field."]
    #[inline] pub fn set_peim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit holds the framing error interrupt enable."]
    #[inline] pub fn feim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FEIM != 0"]
    #[inline] pub fn test_feim(&self) -> bool {
        self.feim() != 0
    }

    #[doc="Sets the FEIM field."]
    #[inline] pub fn set_feim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="This bit holds the receive timeout interrupt enable."]
    #[inline] pub fn rtim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTIM != 0"]
    #[inline] pub fn test_rtim(&self) -> bool {
        self.rtim() != 0
    }

    #[doc="Sets the RTIM field."]
    #[inline] pub fn set_rtim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This bit holds the transmit interrupt enable."]
    #[inline] pub fn txim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXIM != 0"]
    #[inline] pub fn test_txim(&self) -> bool {
        self.txim() != 0
    }

    #[doc="Sets the TXIM field."]
    #[inline] pub fn set_txim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This bit holds the receive interrupt enable."]
    #[inline] pub fn rxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXIM != 0"]
    #[inline] pub fn test_rxim(&self) -> bool {
        self.rxim() != 0
    }

    #[doc="Sets the RXIM field."]
    #[inline] pub fn set_rxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit holds the modem DSR interrupt enable."]
    #[inline] pub fn dsrmim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DSRMIM != 0"]
    #[inline] pub fn test_dsrmim(&self) -> bool {
        self.dsrmim() != 0
    }

    #[doc="Sets the DSRMIM field."]
    #[inline] pub fn set_dsrmim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit holds the modem DCD interrupt enable."]
    #[inline] pub fn dcdmim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCDMIM != 0"]
    #[inline] pub fn test_dcdmim(&self) -> bool {
        self.dcdmim() != 0
    }

    #[doc="Sets the DCDMIM field."]
    #[inline] pub fn set_dcdmim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit holds the modem CTS interrupt enable."]
    #[inline] pub fn ctsmim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTSMIM != 0"]
    #[inline] pub fn test_ctsmim(&self) -> bool {
        self.ctsmim() != 0
    }

    #[doc="Sets the CTSMIM field."]
    #[inline] pub fn set_ctsmim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit holds the modem TXCMP interrupt enable."]
    #[inline] pub fn txcmpmim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXCMPMIM != 0"]
    #[inline] pub fn test_txcmpmim(&self) -> bool {
        self.txcmpmim() != 0
    }

    #[doc="Sets the TXCMPMIM field."]
    #[inline] pub fn set_txcmpmim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.oeim() != 0 { try!(write!(f, " oeim"))}
        if self.beim() != 0 { try!(write!(f, " beim"))}
        if self.peim() != 0 { try!(write!(f, " peim"))}
        if self.feim() != 0 { try!(write!(f, " feim"))}
        if self.rtim() != 0 { try!(write!(f, " rtim"))}
        if self.txim() != 0 { try!(write!(f, " txim"))}
        if self.rxim() != 0 { try!(write!(f, " rxim"))}
        if self.dsrmim() != 0 { try!(write!(f, " dsrmim"))}
        if self.dcdmim() != 0 { try!(write!(f, " dcdmim"))}
        if self.ctsmim() != 0 { try!(write!(f, " ctsmim"))}
        if self.txcmpmim() != 0 { try!(write!(f, " txcmpmim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ies(pub u32);
impl Ies {
    #[doc="This bit holds the overflow interrupt status."]
    #[inline] pub fn oeris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OERIS != 0"]
    #[inline] pub fn test_oeris(&self) -> bool {
        self.oeris() != 0
    }

    #[doc="Sets the OERIS field."]
    #[inline] pub fn set_oeris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This bit holds the break error interrupt status."]
    #[inline] pub fn beris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BERIS != 0"]
    #[inline] pub fn test_beris(&self) -> bool {
        self.beris() != 0
    }

    #[doc="Sets the BERIS field."]
    #[inline] pub fn set_beris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This bit holds the parity error interrupt status."]
    #[inline] pub fn peris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PERIS != 0"]
    #[inline] pub fn test_peris(&self) -> bool {
        self.peris() != 0
    }

    #[doc="Sets the PERIS field."]
    #[inline] pub fn set_peris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit holds the framing error interrupt status."]
    #[inline] pub fn feris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FERIS != 0"]
    #[inline] pub fn test_feris(&self) -> bool {
        self.feris() != 0
    }

    #[doc="Sets the FERIS field."]
    #[inline] pub fn set_feris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="This bit holds the receive timeout interrupt status."]
    #[inline] pub fn rtris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTRIS != 0"]
    #[inline] pub fn test_rtris(&self) -> bool {
        self.rtris() != 0
    }

    #[doc="Sets the RTRIS field."]
    #[inline] pub fn set_rtris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This bit holds the transmit interrupt status."]
    #[inline] pub fn txris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXRIS != 0"]
    #[inline] pub fn test_txris(&self) -> bool {
        self.txris() != 0
    }

    #[doc="Sets the TXRIS field."]
    #[inline] pub fn set_txris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This bit holds the receive interrupt status."]
    #[inline] pub fn rxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXRIS != 0"]
    #[inline] pub fn test_rxris(&self) -> bool {
        self.rxris() != 0
    }

    #[doc="Sets the RXRIS field."]
    #[inline] pub fn set_rxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit holds the modem DSR interrupt status."]
    #[inline] pub fn dsrmris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DSRMRIS != 0"]
    #[inline] pub fn test_dsrmris(&self) -> bool {
        self.dsrmris() != 0
    }

    #[doc="Sets the DSRMRIS field."]
    #[inline] pub fn set_dsrmris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit holds the modem DCD interrupt status."]
    #[inline] pub fn dcdmris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCDMRIS != 0"]
    #[inline] pub fn test_dcdmris(&self) -> bool {
        self.dcdmris() != 0
    }

    #[doc="Sets the DCDMRIS field."]
    #[inline] pub fn set_dcdmris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit holds the modem CTS interrupt status."]
    #[inline] pub fn ctsmris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTSMRIS != 0"]
    #[inline] pub fn test_ctsmris(&self) -> bool {
        self.ctsmris() != 0
    }

    #[doc="Sets the CTSMRIS field."]
    #[inline] pub fn set_ctsmris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit holds the modem TXCMP interrupt status."]
    #[inline] pub fn txcmpmris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXCMPMRIS != 0"]
    #[inline] pub fn test_txcmpmris(&self) -> bool {
        self.txcmpmris() != 0
    }

    #[doc="Sets the TXCMPMRIS field."]
    #[inline] pub fn set_txcmpmris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ies {
    #[inline]
    fn from(other: u32) -> Self {
         Ies(other)
    }
}

impl ::core::fmt::Display for Ies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oeris() != 0 { try!(write!(f, " oeris"))}
        if self.beris() != 0 { try!(write!(f, " beris"))}
        if self.peris() != 0 { try!(write!(f, " peris"))}
        if self.feris() != 0 { try!(write!(f, " feris"))}
        if self.rtris() != 0 { try!(write!(f, " rtris"))}
        if self.txris() != 0 { try!(write!(f, " txris"))}
        if self.rxris() != 0 { try!(write!(f, " rxris"))}
        if self.dsrmris() != 0 { try!(write!(f, " dsrmris"))}
        if self.dcdmris() != 0 { try!(write!(f, " dcdmris"))}
        if self.ctsmris() != 0 { try!(write!(f, " ctsmris"))}
        if self.txcmpmris() != 0 { try!(write!(f, " txcmpmris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Masked Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
    #[doc="This bit holds the overflow interrupt status masked."]
    #[inline] pub fn oemis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OEMIS != 0"]
    #[inline] pub fn test_oemis(&self) -> bool {
        self.oemis() != 0
    }

    #[doc="Sets the OEMIS field."]
    #[inline] pub fn set_oemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This bit holds the break error interrupt status masked."]
    #[inline] pub fn bemis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BEMIS != 0"]
    #[inline] pub fn test_bemis(&self) -> bool {
        self.bemis() != 0
    }

    #[doc="Sets the BEMIS field."]
    #[inline] pub fn set_bemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This bit holds the parity error interrupt status masked."]
    #[inline] pub fn pemis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PEMIS != 0"]
    #[inline] pub fn test_pemis(&self) -> bool {
        self.pemis() != 0
    }

    #[doc="Sets the PEMIS field."]
    #[inline] pub fn set_pemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit holds the framing error interrupt status masked."]
    #[inline] pub fn femis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FEMIS != 0"]
    #[inline] pub fn test_femis(&self) -> bool {
        self.femis() != 0
    }

    #[doc="Sets the FEMIS field."]
    #[inline] pub fn set_femis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="This bit holds the receive timeout interrupt status masked."]
    #[inline] pub fn rtmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTMIS != 0"]
    #[inline] pub fn test_rtmis(&self) -> bool {
        self.rtmis() != 0
    }

    #[doc="Sets the RTMIS field."]
    #[inline] pub fn set_rtmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This bit holds the transmit interrupt status masked."]
    #[inline] pub fn txmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXMIS != 0"]
    #[inline] pub fn test_txmis(&self) -> bool {
        self.txmis() != 0
    }

    #[doc="Sets the TXMIS field."]
    #[inline] pub fn set_txmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This bit holds the receive interrupt status masked."]
    #[inline] pub fn rxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXMIS != 0"]
    #[inline] pub fn test_rxmis(&self) -> bool {
        self.rxmis() != 0
    }

    #[doc="Sets the RXMIS field."]
    #[inline] pub fn set_rxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit holds the modem DSR interrupt status masked."]
    #[inline] pub fn dsrmmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DSRMMIS != 0"]
    #[inline] pub fn test_dsrmmis(&self) -> bool {
        self.dsrmmis() != 0
    }

    #[doc="Sets the DSRMMIS field."]
    #[inline] pub fn set_dsrmmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit holds the modem DCD interrupt status masked."]
    #[inline] pub fn dcdmmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCDMMIS != 0"]
    #[inline] pub fn test_dcdmmis(&self) -> bool {
        self.dcdmmis() != 0
    }

    #[doc="Sets the DCDMMIS field."]
    #[inline] pub fn set_dcdmmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit holds the modem CTS interrupt status masked."]
    #[inline] pub fn ctsmmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTSMMIS != 0"]
    #[inline] pub fn test_ctsmmis(&self) -> bool {
        self.ctsmmis() != 0
    }

    #[doc="Sets the CTSMMIS field."]
    #[inline] pub fn set_ctsmmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit holds the modem TXCMP interrupt status masked."]
    #[inline] pub fn txcmpmmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXCMPMMIS != 0"]
    #[inline] pub fn test_txcmpmmis(&self) -> bool {
        self.txcmpmmis() != 0
    }

    #[doc="Sets the TXCMPMMIS field."]
    #[inline] pub fn set_txcmpmmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mis {
    #[inline]
    fn from(other: u32) -> Self {
         Mis(other)
    }
}

impl ::core::fmt::Display for Mis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oemis() != 0 { try!(write!(f, " oemis"))}
        if self.bemis() != 0 { try!(write!(f, " bemis"))}
        if self.pemis() != 0 { try!(write!(f, " pemis"))}
        if self.femis() != 0 { try!(write!(f, " femis"))}
        if self.rtmis() != 0 { try!(write!(f, " rtmis"))}
        if self.txmis() != 0 { try!(write!(f, " txmis"))}
        if self.rxmis() != 0 { try!(write!(f, " rxmis"))}
        if self.dsrmmis() != 0 { try!(write!(f, " dsrmmis"))}
        if self.dcdmmis() != 0 { try!(write!(f, " dcdmmis"))}
        if self.ctsmmis() != 0 { try!(write!(f, " ctsmmis"))}
        if self.txcmpmmis() != 0 { try!(write!(f, " txcmpmmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iec(pub u32);
impl Iec {
    #[doc="This bit holds the overflow interrupt clear."]
    #[inline] pub fn oeic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OEIC != 0"]
    #[inline] pub fn test_oeic(&self) -> bool {
        self.oeic() != 0
    }

    #[doc="Sets the OEIC field."]
    #[inline] pub fn set_oeic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This bit holds the break error interrupt clear."]
    #[inline] pub fn beic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BEIC != 0"]
    #[inline] pub fn test_beic(&self) -> bool {
        self.beic() != 0
    }

    #[doc="Sets the BEIC field."]
    #[inline] pub fn set_beic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This bit holds the parity error interrupt clear."]
    #[inline] pub fn peic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PEIC != 0"]
    #[inline] pub fn test_peic(&self) -> bool {
        self.peic() != 0
    }

    #[doc="Sets the PEIC field."]
    #[inline] pub fn set_peic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit holds the framing error interrupt clear."]
    #[inline] pub fn feic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FEIC != 0"]
    #[inline] pub fn test_feic(&self) -> bool {
        self.feic() != 0
    }

    #[doc="Sets the FEIC field."]
    #[inline] pub fn set_feic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="This bit holds the receive timeout interrupt clear."]
    #[inline] pub fn rtic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTIC != 0"]
    #[inline] pub fn test_rtic(&self) -> bool {
        self.rtic() != 0
    }

    #[doc="Sets the RTIC field."]
    #[inline] pub fn set_rtic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This bit holds the transmit interrupt clear."]
    #[inline] pub fn txic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXIC != 0"]
    #[inline] pub fn test_txic(&self) -> bool {
        self.txic() != 0
    }

    #[doc="Sets the TXIC field."]
    #[inline] pub fn set_txic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This bit holds the receive interrupt clear."]
    #[inline] pub fn rxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXIC != 0"]
    #[inline] pub fn test_rxic(&self) -> bool {
        self.rxic() != 0
    }

    #[doc="Sets the RXIC field."]
    #[inline] pub fn set_rxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit holds the modem DSR interrupt clear."]
    #[inline] pub fn dsrmic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DSRMIC != 0"]
    #[inline] pub fn test_dsrmic(&self) -> bool {
        self.dsrmic() != 0
    }

    #[doc="Sets the DSRMIC field."]
    #[inline] pub fn set_dsrmic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit holds the modem DCD interrupt clear."]
    #[inline] pub fn dcdmic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCDMIC != 0"]
    #[inline] pub fn test_dcdmic(&self) -> bool {
        self.dcdmic() != 0
    }

    #[doc="Sets the DCDMIC field."]
    #[inline] pub fn set_dcdmic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit holds the modem CTS interrupt clear."]
    #[inline] pub fn ctsmic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTSMIC != 0"]
    #[inline] pub fn test_ctsmic(&self) -> bool {
        self.ctsmic() != 0
    }

    #[doc="Sets the CTSMIC field."]
    #[inline] pub fn set_ctsmic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit holds the modem TXCMP interrupt clear."]
    #[inline] pub fn txcmpmic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXCMPMIC != 0"]
    #[inline] pub fn test_txcmpmic(&self) -> bool {
        self.txcmpmic() != 0
    }

    #[doc="Sets the TXCMPMIC field."]
    #[inline] pub fn set_txcmpmic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iec {
    #[inline]
    fn from(other: u32) -> Self {
         Iec(other)
    }
}

impl ::core::fmt::Display for Iec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oeic() != 0 { try!(write!(f, " oeic"))}
        if self.beic() != 0 { try!(write!(f, " beic"))}
        if self.peic() != 0 { try!(write!(f, " peic"))}
        if self.feic() != 0 { try!(write!(f, " feic"))}
        if self.rtic() != 0 { try!(write!(f, " rtic"))}
        if self.txic() != 0 { try!(write!(f, " txic"))}
        if self.rxic() != 0 { try!(write!(f, " rxic"))}
        if self.dsrmic() != 0 { try!(write!(f, " dsrmic"))}
        if self.dcdmic() != 0 { try!(write!(f, " dcdmic"))}
        if self.ctsmic() != 0 { try!(write!(f, " ctsmic"))}
        if self.txcmpmic() != 0 { try!(write!(f, " txcmpmic"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


