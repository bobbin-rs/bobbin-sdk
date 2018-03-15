
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DCMI Peripheral"]
pub struct DcmiPeriph(pub usize); 

impl DcmiPeriph {
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

    #[doc="Get the *mut pointer for the RIS register."]
    #[inline] pub fn ris_mut(&self) -> *mut Ris { 
        (self.0 + 0x8) as *mut Ris
    }

    #[doc="Get the *const pointer for the RIS register."]
    #[inline] pub fn ris_ptr(&self) -> *const Ris { 
           self.ris_mut()
    }

    #[doc="Read the RIS register."]
    #[inline] pub fn ris(&self) -> Ris { 
        unsafe {
            read_volatile(self.ris_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        (self.0 + 0xc) as *mut Ier
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

    #[doc="Get the *mut pointer for the MIS register."]
    #[inline] pub fn mis_mut(&self) -> *mut Mis { 
        (self.0 + 0x10) as *mut Mis
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

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        (self.0 + 0x14) as *mut Icr
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

    #[doc="Get the *mut pointer for the ESCR register."]
    #[inline] pub fn escr_mut(&self) -> *mut Escr { 
        (self.0 + 0x18) as *mut Escr
    }

    #[doc="Get the *const pointer for the ESCR register."]
    #[inline] pub fn escr_ptr(&self) -> *const Escr { 
           self.escr_mut()
    }

    #[doc="Read the ESCR register."]
    #[inline] pub fn escr(&self) -> Escr { 
        unsafe {
            read_volatile(self.escr_ptr())
        }
    }

    #[doc="Write the ESCR register."]
    #[inline] pub fn set_escr<F: FnOnce(Escr) -> Escr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.escr_mut(), f(Escr(0)));
        }
        self
    }

    #[doc="Modify the ESCR register."]
    #[inline] pub fn with_escr<F: FnOnce(Escr) -> Escr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.escr_mut(), f(self.escr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ESUR register."]
    #[inline] pub fn esur_mut(&self) -> *mut Esur { 
        (self.0 + 0x1c) as *mut Esur
    }

    #[doc="Get the *const pointer for the ESUR register."]
    #[inline] pub fn esur_ptr(&self) -> *const Esur { 
           self.esur_mut()
    }

    #[doc="Read the ESUR register."]
    #[inline] pub fn esur(&self) -> Esur { 
        unsafe {
            read_volatile(self.esur_ptr())
        }
    }

    #[doc="Write the ESUR register."]
    #[inline] pub fn set_esur<F: FnOnce(Esur) -> Esur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.esur_mut(), f(Esur(0)));
        }
        self
    }

    #[doc="Modify the ESUR register."]
    #[inline] pub fn with_esur<F: FnOnce(Esur) -> Esur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.esur_mut(), f(self.esur()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CWSTRT register."]
    #[inline] pub fn cwstrt_mut(&self) -> *mut Cwstrt { 
        (self.0 + 0x20) as *mut Cwstrt
    }

    #[doc="Get the *const pointer for the CWSTRT register."]
    #[inline] pub fn cwstrt_ptr(&self) -> *const Cwstrt { 
           self.cwstrt_mut()
    }

    #[doc="Read the CWSTRT register."]
    #[inline] pub fn cwstrt(&self) -> Cwstrt { 
        unsafe {
            read_volatile(self.cwstrt_ptr())
        }
    }

    #[doc="Write the CWSTRT register."]
    #[inline] pub fn set_cwstrt<F: FnOnce(Cwstrt) -> Cwstrt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cwstrt_mut(), f(Cwstrt(0)));
        }
        self
    }

    #[doc="Modify the CWSTRT register."]
    #[inline] pub fn with_cwstrt<F: FnOnce(Cwstrt) -> Cwstrt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cwstrt_mut(), f(self.cwstrt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CWSIZE register."]
    #[inline] pub fn cwsize_mut(&self) -> *mut Cwsize { 
        (self.0 + 0x24) as *mut Cwsize
    }

    #[doc="Get the *const pointer for the CWSIZE register."]
    #[inline] pub fn cwsize_ptr(&self) -> *const Cwsize { 
           self.cwsize_mut()
    }

    #[doc="Read the CWSIZE register."]
    #[inline] pub fn cwsize(&self) -> Cwsize { 
        unsafe {
            read_volatile(self.cwsize_ptr())
        }
    }

    #[doc="Write the CWSIZE register."]
    #[inline] pub fn set_cwsize<F: FnOnce(Cwsize) -> Cwsize>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cwsize_mut(), f(Cwsize(0)));
        }
        self
    }

    #[doc="Modify the CWSIZE register."]
    #[inline] pub fn with_cwsize<F: FnOnce(Cwsize) -> Cwsize>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cwsize_mut(), f(self.cwsize()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut Dr { 
        (self.0 + 0x28) as *mut Dr
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

}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Odd / Even Line Select"]
    #[inline] pub fn oels(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if OELS != 0"]
    #[inline] pub fn test_oels(&self) -> bool {
        self.oels() != 0
    }

    #[doc="Sets the OELS field."]
    #[inline] pub fn set_oels<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Line Select Mode"]
    #[inline] pub fn lsm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if LSM != 0"]
    #[inline] pub fn test_lsm(&self) -> bool {
        self.lsm() != 0
    }

    #[doc="Sets the LSM field."]
    #[inline] pub fn set_lsm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Odd / Even Byte Select"]
    #[inline] pub fn oebs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if OEBS != 0"]
    #[inline] pub fn test_oebs(&self) -> bool {
        self.oebs() != 0
    }

    #[doc="Sets the OEBS field."]
    #[inline] pub fn set_oebs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Byte Select Mode"]
    #[inline] pub fn bsm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if BSM != 0"]
    #[inline] pub fn test_bsm(&self) -> bool {
        self.bsm() != 0
    }

    #[doc="Sets the BSM field."]
    #[inline] pub fn set_bsm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DCMI enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Extended data mode"]
    #[inline] pub fn edm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if EDM != 0"]
    #[inline] pub fn test_edm(&self) -> bool {
        self.edm() != 0
    }

    #[doc="Sets the EDM field."]
    #[inline] pub fn set_edm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Frame capture rate control"]
    #[inline] pub fn fcrc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if FCRC != 0"]
    #[inline] pub fn test_fcrc(&self) -> bool {
        self.fcrc() != 0
    }

    #[doc="Sets the FCRC field."]
    #[inline] pub fn set_fcrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Vertical synchronization polarity"]
    #[inline] pub fn vspol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if VSPOL != 0"]
    #[inline] pub fn test_vspol(&self) -> bool {
        self.vspol() != 0
    }

    #[doc="Sets the VSPOL field."]
    #[inline] pub fn set_vspol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Horizontal synchronization polarity"]
    #[inline] pub fn hspol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if HSPOL != 0"]
    #[inline] pub fn test_hspol(&self) -> bool {
        self.hspol() != 0
    }

    #[doc="Sets the HSPOL field."]
    #[inline] pub fn set_hspol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pixel clock polarity"]
    #[inline] pub fn pckpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PCKPOL != 0"]
    #[inline] pub fn test_pckpol(&self) -> bool {
        self.pckpol() != 0
    }

    #[doc="Sets the PCKPOL field."]
    #[inline] pub fn set_pckpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Embedded synchronization select"]
    #[inline] pub fn ess(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ESS != 0"]
    #[inline] pub fn test_ess(&self) -> bool {
        self.ess() != 0
    }

    #[doc="Sets the ESS field."]
    #[inline] pub fn set_ess<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="JPEG format"]
    #[inline] pub fn jpeg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if JPEG != 0"]
    #[inline] pub fn test_jpeg(&self) -> bool {
        self.jpeg() != 0
    }

    #[doc="Sets the JPEG field."]
    #[inline] pub fn set_jpeg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Crop feature"]
    #[inline] pub fn crop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CROP != 0"]
    #[inline] pub fn test_crop(&self) -> bool {
        self.crop() != 0
    }

    #[doc="Sets the CROP field."]
    #[inline] pub fn set_crop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Capture mode"]
    #[inline] pub fn cm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CM != 0"]
    #[inline] pub fn test_cm(&self) -> bool {
        self.cm() != 0
    }

    #[doc="Sets the CM field."]
    #[inline] pub fn set_cm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Capture enable"]
    #[inline] pub fn capture(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CAPTURE != 0"]
    #[inline] pub fn test_capture(&self) -> bool {
        self.capture() != 0
    }

    #[doc="Sets the CAPTURE field."]
    #[inline] pub fn set_capture<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.oels() != 0 { try!(write!(f, " oels"))}
        if self.lsm() != 0 { try!(write!(f, " lsm"))}
        if self.oebs() != 0 { try!(write!(f, " oebs"))}
        if self.bsm() != 0 { try!(write!(f, " bsm=0x{:x}", self.bsm()))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.edm() != 0 { try!(write!(f, " edm=0x{:x}", self.edm()))}
        if self.fcrc() != 0 { try!(write!(f, " fcrc=0x{:x}", self.fcrc()))}
        if self.vspol() != 0 { try!(write!(f, " vspol"))}
        if self.hspol() != 0 { try!(write!(f, " hspol"))}
        if self.pckpol() != 0 { try!(write!(f, " pckpol"))}
        if self.ess() != 0 { try!(write!(f, " ess"))}
        if self.jpeg() != 0 { try!(write!(f, " jpeg"))}
        if self.crop() != 0 { try!(write!(f, " crop"))}
        if self.cm() != 0 { try!(write!(f, " cm"))}
        if self.capture() != 0 { try!(write!(f, " capture"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="FIFO not empty"]
    #[inline] pub fn fne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FNE != 0"]
    #[inline] pub fn test_fne(&self) -> bool {
        self.fne() != 0
    }

    #[doc="Sets the FNE field."]
    #[inline] pub fn set_fne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="VSYNC"]
    #[inline] pub fn vsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if VSYNC != 0"]
    #[inline] pub fn test_vsync(&self) -> bool {
        self.vsync() != 0
    }

    #[doc="Sets the VSYNC field."]
    #[inline] pub fn set_vsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="HSYNC"]
    #[inline] pub fn hsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HSYNC != 0"]
    #[inline] pub fn test_hsync(&self) -> bool {
        self.hsync() != 0
    }

    #[doc="Sets the HSYNC field."]
    #[inline] pub fn set_hsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.fne() != 0 { try!(write!(f, " fne"))}
        if self.vsync() != 0 { try!(write!(f, " vsync"))}
        if self.hsync() != 0 { try!(write!(f, " hsync"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="raw interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
    #[doc="Line raw interrupt status"]
    #[inline] pub fn line_ris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LINE_RIS != 0"]
    #[inline] pub fn test_line_ris(&self) -> bool {
        self.line_ris() != 0
    }

    #[doc="Sets the LINE_RIS field."]
    #[inline] pub fn set_line_ris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="VSYNC raw interrupt status"]
    #[inline] pub fn vsync_ris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if VSYNC_RIS != 0"]
    #[inline] pub fn test_vsync_ris(&self) -> bool {
        self.vsync_ris() != 0
    }

    #[doc="Sets the VSYNC_RIS field."]
    #[inline] pub fn set_vsync_ris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Synchronization error raw interrupt status"]
    #[inline] pub fn err_ris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ERR_RIS != 0"]
    #[inline] pub fn test_err_ris(&self) -> bool {
        self.err_ris() != 0
    }

    #[doc="Sets the ERR_RIS field."]
    #[inline] pub fn set_err_ris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Overrun raw interrupt status"]
    #[inline] pub fn ovr_ris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVR_RIS != 0"]
    #[inline] pub fn test_ovr_ris(&self) -> bool {
        self.ovr_ris() != 0
    }

    #[doc="Sets the OVR_RIS field."]
    #[inline] pub fn set_ovr_ris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Capture complete raw interrupt status"]
    #[inline] pub fn frame_ris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FRAME_RIS != 0"]
    #[inline] pub fn test_frame_ris(&self) -> bool {
        self.frame_ris() != 0
    }

    #[doc="Sets the FRAME_RIS field."]
    #[inline] pub fn set_frame_ris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ris {
    #[inline]
    fn from(other: u32) -> Self {
         Ris(other)
    }
}

impl ::core::fmt::Display for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.line_ris() != 0 { try!(write!(f, " line_ris"))}
        if self.vsync_ris() != 0 { try!(write!(f, " vsync_ris"))}
        if self.err_ris() != 0 { try!(write!(f, " err_ris"))}
        if self.ovr_ris() != 0 { try!(write!(f, " ovr_ris"))}
        if self.frame_ris() != 0 { try!(write!(f, " frame_ris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="Line interrupt enable"]
    #[inline] pub fn line_ie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LINE_IE != 0"]
    #[inline] pub fn test_line_ie(&self) -> bool {
        self.line_ie() != 0
    }

    #[doc="Sets the LINE_IE field."]
    #[inline] pub fn set_line_ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="VSYNC interrupt enable"]
    #[inline] pub fn vsync_ie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if VSYNC_IE != 0"]
    #[inline] pub fn test_vsync_ie(&self) -> bool {
        self.vsync_ie() != 0
    }

    #[doc="Sets the VSYNC_IE field."]
    #[inline] pub fn set_vsync_ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Synchronization error interrupt enable"]
    #[inline] pub fn err_ie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ERR_IE != 0"]
    #[inline] pub fn test_err_ie(&self) -> bool {
        self.err_ie() != 0
    }

    #[doc="Sets the ERR_IE field."]
    #[inline] pub fn set_err_ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Overrun interrupt enable"]
    #[inline] pub fn ovr_ie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVR_IE != 0"]
    #[inline] pub fn test_ovr_ie(&self) -> bool {
        self.ovr_ie() != 0
    }

    #[doc="Sets the OVR_IE field."]
    #[inline] pub fn set_ovr_ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Capture complete interrupt enable"]
    #[inline] pub fn frame_ie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FRAME_IE != 0"]
    #[inline] pub fn test_frame_ie(&self) -> bool {
        self.frame_ie() != 0
    }

    #[doc="Sets the FRAME_IE field."]
    #[inline] pub fn set_frame_ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.line_ie() != 0 { try!(write!(f, " line_ie"))}
        if self.vsync_ie() != 0 { try!(write!(f, " vsync_ie"))}
        if self.err_ie() != 0 { try!(write!(f, " err_ie"))}
        if self.ovr_ie() != 0 { try!(write!(f, " ovr_ie"))}
        if self.frame_ie() != 0 { try!(write!(f, " frame_ie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="masked interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
    #[doc="Line masked interrupt status"]
    #[inline] pub fn line_mis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LINE_MIS != 0"]
    #[inline] pub fn test_line_mis(&self) -> bool {
        self.line_mis() != 0
    }

    #[doc="Sets the LINE_MIS field."]
    #[inline] pub fn set_line_mis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="VSYNC masked interrupt status"]
    #[inline] pub fn vsync_mis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if VSYNC_MIS != 0"]
    #[inline] pub fn test_vsync_mis(&self) -> bool {
        self.vsync_mis() != 0
    }

    #[doc="Sets the VSYNC_MIS field."]
    #[inline] pub fn set_vsync_mis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Synchronization error masked interrupt status"]
    #[inline] pub fn err_mis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ERR_MIS != 0"]
    #[inline] pub fn test_err_mis(&self) -> bool {
        self.err_mis() != 0
    }

    #[doc="Sets the ERR_MIS field."]
    #[inline] pub fn set_err_mis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Overrun masked interrupt status"]
    #[inline] pub fn ovr_mis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVR_MIS != 0"]
    #[inline] pub fn test_ovr_mis(&self) -> bool {
        self.ovr_mis() != 0
    }

    #[doc="Sets the OVR_MIS field."]
    #[inline] pub fn set_ovr_mis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Capture complete masked interrupt status"]
    #[inline] pub fn frame_mis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FRAME_MIS != 0"]
    #[inline] pub fn test_frame_mis(&self) -> bool {
        self.frame_mis() != 0
    }

    #[doc="Sets the FRAME_MIS field."]
    #[inline] pub fn set_frame_mis<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.line_mis() != 0 { try!(write!(f, " line_mis"))}
        if self.vsync_mis() != 0 { try!(write!(f, " vsync_mis"))}
        if self.err_mis() != 0 { try!(write!(f, " err_mis"))}
        if self.ovr_mis() != 0 { try!(write!(f, " ovr_mis"))}
        if self.frame_mis() != 0 { try!(write!(f, " frame_mis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="line interrupt status clear"]
    #[inline] pub fn line_isc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LINE_ISC != 0"]
    #[inline] pub fn test_line_isc(&self) -> bool {
        self.line_isc() != 0
    }

    #[doc="Sets the LINE_ISC field."]
    #[inline] pub fn set_line_isc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Vertical synch interrupt status clear"]
    #[inline] pub fn vsync_isc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if VSYNC_ISC != 0"]
    #[inline] pub fn test_vsync_isc(&self) -> bool {
        self.vsync_isc() != 0
    }

    #[doc="Sets the VSYNC_ISC field."]
    #[inline] pub fn set_vsync_isc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Synchronization error interrupt status clear"]
    #[inline] pub fn err_isc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ERR_ISC != 0"]
    #[inline] pub fn test_err_isc(&self) -> bool {
        self.err_isc() != 0
    }

    #[doc="Sets the ERR_ISC field."]
    #[inline] pub fn set_err_isc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Overrun interrupt status clear"]
    #[inline] pub fn ovr_isc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVR_ISC != 0"]
    #[inline] pub fn test_ovr_isc(&self) -> bool {
        self.ovr_isc() != 0
    }

    #[doc="Sets the OVR_ISC field."]
    #[inline] pub fn set_ovr_isc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Capture complete interrupt status clear"]
    #[inline] pub fn frame_isc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FRAME_ISC != 0"]
    #[inline] pub fn test_frame_isc(&self) -> bool {
        self.frame_isc() != 0
    }

    #[doc="Sets the FRAME_ISC field."]
    #[inline] pub fn set_frame_isc<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.line_isc() != 0 { try!(write!(f, " line_isc"))}
        if self.vsync_isc() != 0 { try!(write!(f, " vsync_isc"))}
        if self.err_isc() != 0 { try!(write!(f, " err_isc"))}
        if self.ovr_isc() != 0 { try!(write!(f, " ovr_isc"))}
        if self.frame_isc() != 0 { try!(write!(f, " frame_isc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="embedded synchronization code register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Escr(pub u32);
impl Escr {
    #[doc="Frame end delimiter code"]
    #[inline] pub fn fec(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if FEC != 0"]
    #[inline] pub fn test_fec(&self) -> bool {
        self.fec() != 0
    }

    #[doc="Sets the FEC field."]
    #[inline] pub fn set_fec<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Line end delimiter code"]
    #[inline] pub fn lec(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if LEC != 0"]
    #[inline] pub fn test_lec(&self) -> bool {
        self.lec() != 0
    }

    #[doc="Sets the LEC field."]
    #[inline] pub fn set_lec<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Line start delimiter code"]
    #[inline] pub fn lsc(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if LSC != 0"]
    #[inline] pub fn test_lsc(&self) -> bool {
        self.lsc() != 0
    }

    #[doc="Sets the LSC field."]
    #[inline] pub fn set_lsc<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame start delimiter code"]
    #[inline] pub fn fsc(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FSC != 0"]
    #[inline] pub fn test_fsc(&self) -> bool {
        self.fsc() != 0
    }

    #[doc="Sets the FSC field."]
    #[inline] pub fn set_fsc<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Escr {
    #[inline]
    fn from(other: u32) -> Self {
         Escr(other)
    }
}

impl ::core::fmt::Display for Escr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Escr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fec() != 0 { try!(write!(f, " fec=0x{:x}", self.fec()))}
        if self.lec() != 0 { try!(write!(f, " lec=0x{:x}", self.lec()))}
        if self.lsc() != 0 { try!(write!(f, " lsc=0x{:x}", self.lsc()))}
        if self.fsc() != 0 { try!(write!(f, " fsc=0x{:x}", self.fsc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="embedded synchronization unmask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Esur(pub u32);
impl Esur {
    #[doc="Frame end delimiter unmask"]
    #[inline] pub fn feu(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if FEU != 0"]
    #[inline] pub fn test_feu(&self) -> bool {
        self.feu() != 0
    }

    #[doc="Sets the FEU field."]
    #[inline] pub fn set_feu<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Line end delimiter unmask"]
    #[inline] pub fn leu(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if LEU != 0"]
    #[inline] pub fn test_leu(&self) -> bool {
        self.leu() != 0
    }

    #[doc="Sets the LEU field."]
    #[inline] pub fn set_leu<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Line start delimiter unmask"]
    #[inline] pub fn lsu(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if LSU != 0"]
    #[inline] pub fn test_lsu(&self) -> bool {
        self.lsu() != 0
    }

    #[doc="Sets the LSU field."]
    #[inline] pub fn set_lsu<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame start delimiter unmask"]
    #[inline] pub fn fsu(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FSU != 0"]
    #[inline] pub fn test_fsu(&self) -> bool {
        self.fsu() != 0
    }

    #[doc="Sets the FSU field."]
    #[inline] pub fn set_fsu<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Esur {
    #[inline]
    fn from(other: u32) -> Self {
         Esur(other)
    }
}

impl ::core::fmt::Display for Esur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Esur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.feu() != 0 { try!(write!(f, " feu=0x{:x}", self.feu()))}
        if self.leu() != 0 { try!(write!(f, " leu=0x{:x}", self.leu()))}
        if self.lsu() != 0 { try!(write!(f, " lsu=0x{:x}", self.lsu()))}
        if self.fsu() != 0 { try!(write!(f, " fsu=0x{:x}", self.fsu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="crop window start"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cwstrt(pub u32);
impl Cwstrt {
    #[doc="Vertical start line count"]
    #[inline] pub fn vst(&self) -> bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1fff) as u16) } // [28:16]
    }

    #[doc="Returns true if VST != 0"]
    #[inline] pub fn test_vst(&self) -> bool {
        self.vst() != 0
    }

    #[doc="Sets the VST field."]
    #[inline] pub fn set_vst<V: Into<bits::U13>>(mut self, value: V) -> Self {
        let value: bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Horizontal offset count"]
    #[inline] pub fn hoffcnt(&self) -> bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if HOFFCNT != 0"]
    #[inline] pub fn test_hoffcnt(&self) -> bool {
        self.hoffcnt() != 0
    }

    #[doc="Sets the HOFFCNT field."]
    #[inline] pub fn set_hoffcnt<V: Into<bits::U14>>(mut self, value: V) -> Self {
        let value: bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cwstrt {
    #[inline]
    fn from(other: u32) -> Self {
         Cwstrt(other)
    }
}

impl ::core::fmt::Display for Cwstrt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cwstrt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vst() != 0 { try!(write!(f, " vst=0x{:x}", self.vst()))}
        if self.hoffcnt() != 0 { try!(write!(f, " hoffcnt=0x{:x}", self.hoffcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="crop window size"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cwsize(pub u32);
impl Cwsize {
    #[doc="Vertical line count"]
    #[inline] pub fn vline(&self) -> bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3fff) as u16) } // [29:16]
    }

    #[doc="Returns true if VLINE != 0"]
    #[inline] pub fn test_vline(&self) -> bool {
        self.vline() != 0
    }

    #[doc="Sets the VLINE field."]
    #[inline] pub fn set_vline<V: Into<bits::U14>>(mut self, value: V) -> Self {
        let value: bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Capture count"]
    #[inline] pub fn capcnt(&self) -> bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3fff) as u16) } // [13:0]
    }

    #[doc="Returns true if CAPCNT != 0"]
    #[inline] pub fn test_capcnt(&self) -> bool {
        self.capcnt() != 0
    }

    #[doc="Sets the CAPCNT field."]
    #[inline] pub fn set_capcnt<V: Into<bits::U14>>(mut self, value: V) -> Self {
        let value: bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cwsize {
    #[inline]
    fn from(other: u32) -> Self {
         Cwsize(other)
    }
}

impl ::core::fmt::Display for Cwsize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cwsize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vline() != 0 { try!(write!(f, " vline=0x{:x}", self.vline()))}
        if self.capcnt() != 0 { try!(write!(f, " capcnt=0x{:x}", self.capcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="Data byte 3"]
    #[inline] pub fn byte3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if BYTE3 != 0"]
    #[inline] pub fn test_byte3(&self) -> bool {
        self.byte3() != 0
    }

    #[doc="Sets the BYTE3 field."]
    #[inline] pub fn set_byte3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Data byte 2"]
    #[inline] pub fn byte2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if BYTE2 != 0"]
    #[inline] pub fn test_byte2(&self) -> bool {
        self.byte2() != 0
    }

    #[doc="Sets the BYTE2 field."]
    #[inline] pub fn set_byte2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Data byte 1"]
    #[inline] pub fn byte1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if BYTE1 != 0"]
    #[inline] pub fn test_byte1(&self) -> bool {
        self.byte1() != 0
    }

    #[doc="Sets the BYTE1 field."]
    #[inline] pub fn set_byte1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Data byte 0"]
    #[inline] pub fn byte0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BYTE0 != 0"]
    #[inline] pub fn test_byte0(&self) -> bool {
        self.byte0() != 0
    }

    #[doc="Sets the BYTE0 field."]
    #[inline] pub fn set_byte0<V: Into<bits::U8>>(mut self, value: V) -> Self {
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
        if self.byte3() != 0 { try!(write!(f, " byte3=0x{:x}", self.byte3()))}
        if self.byte2() != 0 { try!(write!(f, " byte2=0x{:x}", self.byte2()))}
        if self.byte1() != 0 { try!(write!(f, " byte1=0x{:x}", self.byte1()))}
        if self.byte0() != 0 { try!(write!(f, " byte0=0x{:x}", self.byte0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

