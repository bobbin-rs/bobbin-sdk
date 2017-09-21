#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C_V1 Peripheral"]
pub struct I2cPeriph(pub usize); 


impl I2cPeriph {
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

    #[doc="Get the *mut pointer for the OAR1 register."]
    #[inline] pub fn oar1_mut(&self) -> *mut Oar1 { 
        (self.0 + 0x8) as *mut Oar1
    }

    #[doc="Get the *const pointer for the OAR1 register."]
    #[inline] pub fn oar1_ptr(&self) -> *const Oar1 { 
           self.oar1_mut()
    }

    #[doc="Read the OAR1 register."]
    #[inline] pub fn oar1(&self) -> Oar1 { 
        unsafe {
            read_volatile(self.oar1_ptr())
        }
    }

    #[doc="Write the OAR1 register."]
    #[inline] pub fn set_oar1<F: FnOnce(Oar1) -> Oar1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.oar1_mut(), f(Oar1(0)));
        }
        self
    }

    #[doc="Modify the OAR1 register."]
    #[inline] pub fn with_oar1<F: FnOnce(Oar1) -> Oar1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.oar1_mut(), f(self.oar1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OAR2 register."]
    #[inline] pub fn oar2_mut(&self) -> *mut Oar2 { 
        (self.0 + 0xc) as *mut Oar2
    }

    #[doc="Get the *const pointer for the OAR2 register."]
    #[inline] pub fn oar2_ptr(&self) -> *const Oar2 { 
           self.oar2_mut()
    }

    #[doc="Read the OAR2 register."]
    #[inline] pub fn oar2(&self) -> Oar2 { 
        unsafe {
            read_volatile(self.oar2_ptr())
        }
    }

    #[doc="Write the OAR2 register."]
    #[inline] pub fn set_oar2<F: FnOnce(Oar2) -> Oar2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.oar2_mut(), f(Oar2(0)));
        }
        self
    }

    #[doc="Modify the OAR2 register."]
    #[inline] pub fn with_oar2<F: FnOnce(Oar2) -> Oar2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.oar2_mut(), f(self.oar2()));
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

    #[doc="Get the *mut pointer for the SR1 register."]
    #[inline] pub fn sr1_mut(&self) -> *mut Sr1 { 
        (self.0 + 0x14) as *mut Sr1
    }

    #[doc="Get the *const pointer for the SR1 register."]
    #[inline] pub fn sr1_ptr(&self) -> *const Sr1 { 
           self.sr1_mut()
    }

    #[doc="Read the SR1 register."]
    #[inline] pub fn sr1(&self) -> Sr1 { 
        unsafe {
            read_volatile(self.sr1_ptr())
        }
    }

    #[doc="Write the SR1 register."]
    #[inline] pub fn set_sr1<F: FnOnce(Sr1) -> Sr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr1_mut(), f(Sr1(0)));
        }
        self
    }

    #[doc="Modify the SR1 register."]
    #[inline] pub fn with_sr1<F: FnOnce(Sr1) -> Sr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr1_mut(), f(self.sr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SR2 register."]
    #[inline] pub fn sr2_mut(&self) -> *mut Sr2 { 
        (self.0 + 0x18) as *mut Sr2
    }

    #[doc="Get the *const pointer for the SR2 register."]
    #[inline] pub fn sr2_ptr(&self) -> *const Sr2 { 
           self.sr2_mut()
    }

    #[doc="Read the SR2 register."]
    #[inline] pub fn sr2(&self) -> Sr2 { 
        unsafe {
            read_volatile(self.sr2_ptr())
        }
    }

    #[doc="Get the *mut pointer for the CCR register."]
    #[inline] pub fn ccr_mut(&self) -> *mut Ccr { 
        (self.0 + 0x1c) as *mut Ccr
    }

    #[doc="Get the *const pointer for the CCR register."]
    #[inline] pub fn ccr_ptr(&self) -> *const Ccr { 
           self.ccr_mut()
    }

    #[doc="Read the CCR register."]
    #[inline] pub fn ccr(&self) -> Ccr { 
        unsafe {
            read_volatile(self.ccr_ptr())
        }
    }

    #[doc="Write the CCR register."]
    #[inline] pub fn set_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccr_mut(), f(Ccr(0)));
        }
        self
    }

    #[doc="Modify the CCR register."]
    #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccr_mut(), f(self.ccr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TRISE register."]
    #[inline] pub fn trise_mut(&self) -> *mut Trise { 
        (self.0 + 0x20) as *mut Trise
    }

    #[doc="Get the *const pointer for the TRISE register."]
    #[inline] pub fn trise_ptr(&self) -> *const Trise { 
           self.trise_mut()
    }

    #[doc="Read the TRISE register."]
    #[inline] pub fn trise(&self) -> Trise { 
        unsafe {
            read_volatile(self.trise_ptr())
        }
    }

    #[doc="Write the TRISE register."]
    #[inline] pub fn set_trise<F: FnOnce(Trise) -> Trise>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.trise_mut(), f(Trise(0)));
        }
        self
    }

    #[doc="Modify the TRISE register."]
    #[inline] pub fn with_trise<F: FnOnce(Trise) -> Trise>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.trise_mut(), f(self.trise()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLTR register."]
    #[inline] pub fn fltr_mut(&self) -> *mut Fltr { 
        (self.0 + 0x24) as *mut Fltr
    }

    #[doc="Get the *const pointer for the FLTR register."]
    #[inline] pub fn fltr_ptr(&self) -> *const Fltr { 
           self.fltr_mut()
    }

    #[doc="Read the FLTR register."]
    #[inline] pub fn fltr(&self) -> Fltr { 
        unsafe {
            read_volatile(self.fltr_ptr())
        }
    }

    #[doc="Write the FLTR register."]
    #[inline] pub fn set_fltr<F: FnOnce(Fltr) -> Fltr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltr_mut(), f(Fltr(0)));
        }
        self
    }

    #[doc="Modify the FLTR register."]
    #[inline] pub fn with_fltr<F: FnOnce(Fltr) -> Fltr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fltr_mut(), f(self.fltr()));
        }
        self
    }

}

#[doc="Control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Software reset"]
    #[inline] pub fn swrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="SMBus alert"]
    #[inline] pub fn alert(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ALERT != 0"]
    #[inline] pub fn test_alert(&self) -> bool {
        self.alert() != 0
    }

    #[doc="Sets the ALERT field."]
    #[inline] pub fn set_alert<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Packet error checking"]
    #[inline] pub fn pec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PEC != 0"]
    #[inline] pub fn test_pec(&self) -> bool {
        self.pec() != 0
    }

    #[doc="Sets the PEC field."]
    #[inline] pub fn set_pec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Acknowledge/PEC Position (for data reception)"]
    #[inline] pub fn pos(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if POS != 0"]
    #[inline] pub fn test_pos(&self) -> bool {
        self.pos() != 0
    }

    #[doc="Sets the POS field."]
    #[inline] pub fn set_pos<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Acknowledge enable"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Stop generation"]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Start generation"]
    #[inline] pub fn start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock stretching disable (Slave mode)"]
    #[inline] pub fn nostretch(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NOSTRETCH != 0"]
    #[inline] pub fn test_nostretch(&self) -> bool {
        self.nostretch() != 0
    }

    #[doc="Sets the NOSTRETCH field."]
    #[inline] pub fn set_nostretch<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="General call enable"]
    #[inline] pub fn engc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ENGC != 0"]
    #[inline] pub fn test_engc(&self) -> bool {
        self.engc() != 0
    }

    #[doc="Sets the ENGC field."]
    #[inline] pub fn set_engc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PEC enable"]
    #[inline] pub fn enpec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ENPEC != 0"]
    #[inline] pub fn test_enpec(&self) -> bool {
        self.enpec() != 0
    }

    #[doc="Sets the ENPEC field."]
    #[inline] pub fn set_enpec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="ARP enable"]
    #[inline] pub fn enarp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ENARP != 0"]
    #[inline] pub fn test_enarp(&self) -> bool {
        self.enarp() != 0
    }

    #[doc="Sets the ENARP field."]
    #[inline] pub fn set_enarp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SMBus type"]
    #[inline] pub fn smbtype(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SMBTYPE != 0"]
    #[inline] pub fn test_smbtype(&self) -> bool {
        self.smbtype() != 0
    }

    #[doc="Sets the SMBTYPE field."]
    #[inline] pub fn set_smbtype<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="SMBus mode"]
    #[inline] pub fn smbus(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SMBUS != 0"]
    #[inline] pub fn test_smbus(&self) -> bool {
        self.smbus() != 0
    }

    #[doc="Sets the SMBUS field."]
    #[inline] pub fn set_smbus<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Peripheral enable"]
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
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.alert() != 0 { try!(write!(f, " alert"))}
        if self.pec() != 0 { try!(write!(f, " pec"))}
        if self.pos() != 0 { try!(write!(f, " pos"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.nostretch() != 0 { try!(write!(f, " nostretch"))}
        if self.engc() != 0 { try!(write!(f, " engc"))}
        if self.enpec() != 0 { try!(write!(f, " enpec"))}
        if self.enarp() != 0 { try!(write!(f, " enarp"))}
        if self.smbtype() != 0 { try!(write!(f, " smbtype"))}
        if self.smbus() != 0 { try!(write!(f, " smbus"))}
        if self.pe() != 0 { try!(write!(f, " pe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="DMA last transfer"]
    #[inline] pub fn last(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if LAST != 0"]
    #[inline] pub fn test_last(&self) -> bool {
        self.last() != 0
    }

    #[doc="Sets the LAST field."]
    #[inline] pub fn set_last<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DMA requests enable"]
    #[inline] pub fn dmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Buffer interrupt enable"]
    #[inline] pub fn itbufen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ITBUFEN != 0"]
    #[inline] pub fn test_itbufen(&self) -> bool {
        self.itbufen() != 0
    }

    #[doc="Sets the ITBUFEN field."]
    #[inline] pub fn set_itbufen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Event interrupt enable"]
    #[inline] pub fn itevten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ITEVTEN != 0"]
    #[inline] pub fn test_itevten(&self) -> bool {
        self.itevten() != 0
    }

    #[doc="Sets the ITEVTEN field."]
    #[inline] pub fn set_itevten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn iterren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ITERREN != 0"]
    #[inline] pub fn test_iterren(&self) -> bool {
        self.iterren() != 0
    }

    #[doc="Sets the ITERREN field."]
    #[inline] pub fn set_iterren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Peripheral clock frequency"]
    #[inline] pub fn freq(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if FREQ != 0"]
    #[inline] pub fn test_freq(&self) -> bool {
        self.freq() != 0
    }

    #[doc="Sets the FREQ field."]
    #[inline] pub fn set_freq<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
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
        if self.last() != 0 { try!(write!(f, " last"))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.itbufen() != 0 { try!(write!(f, " itbufen"))}
        if self.itevten() != 0 { try!(write!(f, " itevten"))}
        if self.iterren() != 0 { try!(write!(f, " iterren"))}
        if self.freq() != 0 { try!(write!(f, " freq=0x{:x}", self.freq()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Own address register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Oar1(pub u32);
impl Oar1 {
    #[doc="Addressing mode (slave mode)"]
    #[inline] pub fn addmode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ADDMODE != 0"]
    #[inline] pub fn test_addmode(&self) -> bool {
        self.addmode() != 0
    }

    #[doc="Sets the ADDMODE field."]
    #[inline] pub fn set_addmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Interface address"]
    #[inline] pub fn add10(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if ADD10 != 0"]
    #[inline] pub fn test_add10(&self) -> bool {
        self.add10() != 0
    }

    #[doc="Sets the ADD10 field."]
    #[inline] pub fn set_add10<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Interface address"]
    #[inline] pub fn add7(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7f) as u8) } // [7:1]
    }

    #[doc="Returns true if ADD7 != 0"]
    #[inline] pub fn test_add7(&self) -> bool {
        self.add7() != 0
    }

    #[doc="Sets the ADD7 field."]
    #[inline] pub fn set_add7<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Interface address"]
    #[inline] pub fn add0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADD0 != 0"]
    #[inline] pub fn test_add0(&self) -> bool {
        self.add0() != 0
    }

    #[doc="Sets the ADD0 field."]
    #[inline] pub fn set_add0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Oar1 {
    #[inline]
    fn from(other: u32) -> Self {
         Oar1(other)
    }
}

impl ::core::fmt::Display for Oar1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Oar1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addmode() != 0 { try!(write!(f, " addmode"))}
        if self.add10() != 0 { try!(write!(f, " add10=0x{:x}", self.add10()))}
        if self.add7() != 0 { try!(write!(f, " add7=0x{:x}", self.add7()))}
        if self.add0() != 0 { try!(write!(f, " add0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Own address register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Oar2(pub u32);
impl Oar2 {
    #[doc="Interface address"]
    #[inline] pub fn add2(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7f) as u8) } // [7:1]
    }

    #[doc="Returns true if ADD2 != 0"]
    #[inline] pub fn test_add2(&self) -> bool {
        self.add2() != 0
    }

    #[doc="Sets the ADD2 field."]
    #[inline] pub fn set_add2<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Dual addressing mode enable"]
    #[inline] pub fn endual(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENDUAL != 0"]
    #[inline] pub fn test_endual(&self) -> bool {
        self.endual() != 0
    }

    #[doc="Sets the ENDUAL field."]
    #[inline] pub fn set_endual<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Oar2 {
    #[inline]
    fn from(other: u32) -> Self {
         Oar2(other)
    }
}

impl ::core::fmt::Display for Oar2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Oar2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.add2() != 0 { try!(write!(f, " add2=0x{:x}", self.add2()))}
        if self.endual() != 0 { try!(write!(f, " endual"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="8-bit data register"]
    #[inline] pub fn dr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DR != 0"]
    #[inline] pub fn test_dr(&self) -> bool {
        self.dr() != 0
    }

    #[doc="Sets the DR field."]
    #[inline] pub fn set_dr<V: Into<bits::U8>>(mut self, value: V) -> Self {
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
        if self.dr() != 0 { try!(write!(f, " dr=0x{:x}", self.dr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr1(pub u32);
impl Sr1 {
    #[doc="SMBus alert"]
    #[inline] pub fn smbalert(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SMBALERT != 0"]
    #[inline] pub fn test_smbalert(&self) -> bool {
        self.smbalert() != 0
    }

    #[doc="Sets the SMBALERT field."]
    #[inline] pub fn set_smbalert<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Timeout or Tlow error"]
    #[inline] pub fn timeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TIMEOUT != 0"]
    #[inline] pub fn test_timeout(&self) -> bool {
        self.timeout() != 0
    }

    #[doc="Sets the TIMEOUT field."]
    #[inline] pub fn set_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="PEC Error in reception"]
    #[inline] pub fn pecerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PECERR != 0"]
    #[inline] pub fn test_pecerr(&self) -> bool {
        self.pecerr() != 0
    }

    #[doc="Sets the PECERR field."]
    #[inline] pub fn set_pecerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Overrun/Underrun"]
    #[inline] pub fn ovr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if OVR != 0"]
    #[inline] pub fn test_ovr(&self) -> bool {
        self.ovr() != 0
    }

    #[doc="Sets the OVR field."]
    #[inline] pub fn set_ovr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Acknowledge failure"]
    #[inline] pub fn af(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if AF != 0"]
    #[inline] pub fn test_af(&self) -> bool {
        self.af() != 0
    }

    #[doc="Sets the AF field."]
    #[inline] pub fn set_af<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Arbitration lost (master mode)"]
    #[inline] pub fn arlo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ARLO != 0"]
    #[inline] pub fn test_arlo(&self) -> bool {
        self.arlo() != 0
    }

    #[doc="Sets the ARLO field."]
    #[inline] pub fn set_arlo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Bus error"]
    #[inline] pub fn berr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BERR != 0"]
    #[inline] pub fn test_berr(&self) -> bool {
        self.berr() != 0
    }

    #[doc="Sets the BERR field."]
    #[inline] pub fn set_berr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Data register empty (transmitters)"]
    #[inline] pub fn txe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TxE != 0"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe() != 0
    }

    #[doc="Sets the TxE field."]
    #[inline] pub fn set_txe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Data register not empty (receivers)"]
    #[inline] pub fn rxne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RxNE != 0"]
    #[inline] pub fn test_rxne(&self) -> bool {
        self.rxne() != 0
    }

    #[doc="Sets the RxNE field."]
    #[inline] pub fn set_rxne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Stop detection (slave mode)"]
    #[inline] pub fn stopf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if STOPF != 0"]
    #[inline] pub fn test_stopf(&self) -> bool {
        self.stopf() != 0
    }

    #[doc="Sets the STOPF field."]
    #[inline] pub fn set_stopf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="10-bit header sent (Master mode)"]
    #[inline] pub fn add10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ADD10 != 0"]
    #[inline] pub fn test_add10(&self) -> bool {
        self.add10() != 0
    }

    #[doc="Sets the ADD10 field."]
    #[inline] pub fn set_add10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Byte transfer finished"]
    #[inline] pub fn btf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BTF != 0"]
    #[inline] pub fn test_btf(&self) -> bool {
        self.btf() != 0
    }

    #[doc="Sets the BTF field."]
    #[inline] pub fn set_btf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Address sent (master mode)/matched (slave mode)"]
    #[inline] pub fn addr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Start bit (Master mode)"]
    #[inline] pub fn sb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SB != 0"]
    #[inline] pub fn test_sb(&self) -> bool {
        self.sb() != 0
    }

    #[doc="Sets the SB field."]
    #[inline] pub fn set_sb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Sr1(other)
    }
}

impl ::core::fmt::Display for Sr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.smbalert() != 0 { try!(write!(f, " smbalert"))}
        if self.timeout() != 0 { try!(write!(f, " timeout"))}
        if self.pecerr() != 0 { try!(write!(f, " pecerr"))}
        if self.ovr() != 0 { try!(write!(f, " ovr"))}
        if self.af() != 0 { try!(write!(f, " af"))}
        if self.arlo() != 0 { try!(write!(f, " arlo"))}
        if self.berr() != 0 { try!(write!(f, " berr"))}
        if self.txe() != 0 { try!(write!(f, " txe"))}
        if self.rxne() != 0 { try!(write!(f, " rxne"))}
        if self.stopf() != 0 { try!(write!(f, " stopf"))}
        if self.add10() != 0 { try!(write!(f, " add10"))}
        if self.btf() != 0 { try!(write!(f, " btf"))}
        if self.addr() != 0 { try!(write!(f, " addr"))}
        if self.sb() != 0 { try!(write!(f, " sb"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr2(pub u32);
impl Sr2 {
    #[doc="acket error checking register"]
    #[inline] pub fn pec(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if PEC != 0"]
    #[inline] pub fn test_pec(&self) -> bool {
        self.pec() != 0
    }

    #[doc="Sets the PEC field."]
    #[inline] pub fn set_pec<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Dual flag (Slave mode)"]
    #[inline] pub fn dualf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DUALF != 0"]
    #[inline] pub fn test_dualf(&self) -> bool {
        self.dualf() != 0
    }

    #[doc="Sets the DUALF field."]
    #[inline] pub fn set_dualf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="SMBus host header (Slave mode)"]
    #[inline] pub fn smbhost(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SMBHOST != 0"]
    #[inline] pub fn test_smbhost(&self) -> bool {
        self.smbhost() != 0
    }

    #[doc="Sets the SMBHOST field."]
    #[inline] pub fn set_smbhost<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SMBus device default address (Slave mode)"]
    #[inline] pub fn smbdefault(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SMBDEFAULT != 0"]
    #[inline] pub fn test_smbdefault(&self) -> bool {
        self.smbdefault() != 0
    }

    #[doc="Sets the SMBDEFAULT field."]
    #[inline] pub fn set_smbdefault<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="General call address (Slave mode)"]
    #[inline] pub fn gencall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GENCALL != 0"]
    #[inline] pub fn test_gencall(&self) -> bool {
        self.gencall() != 0
    }

    #[doc="Sets the GENCALL field."]
    #[inline] pub fn set_gencall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmitter/receiver"]
    #[inline] pub fn tra(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TRA != 0"]
    #[inline] pub fn test_tra(&self) -> bool {
        self.tra() != 0
    }

    #[doc="Sets the TRA field."]
    #[inline] pub fn set_tra<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Bus busy"]
    #[inline] pub fn busy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Master/slave"]
    #[inline] pub fn msl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MSL != 0"]
    #[inline] pub fn test_msl(&self) -> bool {
        self.msl() != 0
    }

    #[doc="Sets the MSL field."]
    #[inline] pub fn set_msl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Sr2(other)
    }
}

impl ::core::fmt::Display for Sr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pec() != 0 { try!(write!(f, " pec=0x{:x}", self.pec()))}
        if self.dualf() != 0 { try!(write!(f, " dualf"))}
        if self.smbhost() != 0 { try!(write!(f, " smbhost"))}
        if self.smbdefault() != 0 { try!(write!(f, " smbdefault"))}
        if self.gencall() != 0 { try!(write!(f, " gencall"))}
        if self.tra() != 0 { try!(write!(f, " tra"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.msl() != 0 { try!(write!(f, " msl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc="I2C master mode selection"]
    #[inline] pub fn f_s(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if F_S != 0"]
    #[inline] pub fn test_f_s(&self) -> bool {
        self.f_s() != 0
    }

    #[doc="Sets the F_S field."]
    #[inline] pub fn set_f_s<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Fast mode duty cycle"]
    #[inline] pub fn duty(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DUTY != 0"]
    #[inline] pub fn test_duty(&self) -> bool {
        self.duty() != 0
    }

    #[doc="Sets the DUTY field."]
    #[inline] pub fn set_duty<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Clock control register in Fast/Standard mode (Master mode)"]
    #[inline] pub fn ccr(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if CCR != 0"]
    #[inline] pub fn test_ccr(&self) -> bool {
        self.ccr() != 0
    }

    #[doc="Sets the CCR field."]
    #[inline] pub fn set_ccr<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccr {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr(other)
    }
}

impl ::core::fmt::Display for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.f_s() != 0 { try!(write!(f, " f_s"))}
        if self.duty() != 0 { try!(write!(f, " duty"))}
        if self.ccr() != 0 { try!(write!(f, " ccr=0x{:x}", self.ccr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TRISE register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Trise(pub u32);
impl Trise {
    #[doc="Maximum rise time in Fast/Standard mode (Master mode)"]
    #[inline] pub fn trise(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if TRISE != 0"]
    #[inline] pub fn test_trise(&self) -> bool {
        self.trise() != 0
    }

    #[doc="Sets the TRISE field."]
    #[inline] pub fn set_trise<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Trise {
    #[inline]
    fn from(other: u32) -> Self {
         Trise(other)
    }
}

impl ::core::fmt::Display for Trise {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Trise {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trise() != 0 { try!(write!(f, " trise=0x{:x}", self.trise()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C FLTR register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltr(pub u32);
impl Fltr {
    #[doc="Digital noise filter"]
    #[inline] pub fn dnf(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DNF != 0"]
    #[inline] pub fn test_dnf(&self) -> bool {
        self.dnf() != 0
    }

    #[doc="Sets the DNF field."]
    #[inline] pub fn set_dnf<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Analog noise filter OFF"]
    #[inline] pub fn anoff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ANOFF != 0"]
    #[inline] pub fn test_anoff(&self) -> bool {
        self.anoff() != 0
    }

    #[doc="Sets the ANOFF field."]
    #[inline] pub fn set_anoff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Fltr {
    #[inline]
    fn from(other: u32) -> Self {
         Fltr(other)
    }
}

impl ::core::fmt::Display for Fltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dnf() != 0 { try!(write!(f, " dnf=0x{:x}", self.dnf()))}
        if self.anoff() != 0 { try!(write!(f, " anoff"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub struct I2cCh { pub periph: I2cPeriph, pub index: usize }

