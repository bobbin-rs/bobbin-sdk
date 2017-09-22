//! GPIO Common Registers
#[allow(unused_imports)] use bobbin_common::*;

periph!(GPIO_COMMON, GpioCommon, 0x4000a000);

#[doc="GPIO Common Registers"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GpioCommon(pub usize);
impl GpioCommon {
    #[doc="Get the *mut pointer for the EXTIPSELL register."]
    #[inline] pub fn extipsell_mut(&self) -> *mut Extipsell { 
        (self.0 + 0x400) as *mut Extipsell
    }

    #[doc="Get the *const pointer for the EXTIPSELL register."]
    #[inline] pub fn extipsell_ptr(&self) -> *const Extipsell { 
           self.extipsell_mut()
    }

    #[doc="Read the EXTIPSELL register."]
    #[inline] pub fn extipsell(&self) -> Extipsell { 
        unsafe {
            read_volatile(self.extipsell_ptr())
        }
    }

    #[doc="Write the EXTIPSELL register."]
    #[inline] pub fn set_extipsell<F: FnOnce(Extipsell) -> Extipsell>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extipsell_mut(), f(Extipsell(0)));
        }
        self
    }

    #[doc="Modify the EXTIPSELL register."]
    #[inline] pub fn with_extipsell<F: FnOnce(Extipsell) -> Extipsell>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extipsell_mut(), f(self.extipsell()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTIPSELH register."]
    #[inline] pub fn extipselh_mut(&self) -> *mut Extipselh { 
        (self.0 + 0x404) as *mut Extipselh
    }

    #[doc="Get the *const pointer for the EXTIPSELH register."]
    #[inline] pub fn extipselh_ptr(&self) -> *const Extipselh { 
           self.extipselh_mut()
    }

    #[doc="Read the EXTIPSELH register."]
    #[inline] pub fn extipselh(&self) -> Extipselh { 
        unsafe {
            read_volatile(self.extipselh_ptr())
        }
    }

    #[doc="Write the EXTIPSELH register."]
    #[inline] pub fn set_extipselh<F: FnOnce(Extipselh) -> Extipselh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extipselh_mut(), f(Extipselh(0)));
        }
        self
    }

    #[doc="Modify the EXTIPSELH register."]
    #[inline] pub fn with_extipselh<F: FnOnce(Extipselh) -> Extipselh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extipselh_mut(), f(self.extipselh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTIPINSELL register."]
    #[inline] pub fn extipinsell_mut(&self) -> *mut Extipinsell { 
        (self.0 + 0x408) as *mut Extipinsell
    }

    #[doc="Get the *const pointer for the EXTIPINSELL register."]
    #[inline] pub fn extipinsell_ptr(&self) -> *const Extipinsell { 
           self.extipinsell_mut()
    }

    #[doc="Read the EXTIPINSELL register."]
    #[inline] pub fn extipinsell(&self) -> Extipinsell { 
        unsafe {
            read_volatile(self.extipinsell_ptr())
        }
    }

    #[doc="Write the EXTIPINSELL register."]
    #[inline] pub fn set_extipinsell<F: FnOnce(Extipinsell) -> Extipinsell>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extipinsell_mut(), f(Extipinsell(0)));
        }
        self
    }

    #[doc="Modify the EXTIPINSELL register."]
    #[inline] pub fn with_extipinsell<F: FnOnce(Extipinsell) -> Extipinsell>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extipinsell_mut(), f(self.extipinsell()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTIPINSELH register."]
    #[inline] pub fn extipinselh_mut(&self) -> *mut Extipinselh { 
        (self.0 + 0x40c) as *mut Extipinselh
    }

    #[doc="Get the *const pointer for the EXTIPINSELH register."]
    #[inline] pub fn extipinselh_ptr(&self) -> *const Extipinselh { 
           self.extipinselh_mut()
    }

    #[doc="Read the EXTIPINSELH register."]
    #[inline] pub fn extipinselh(&self) -> Extipinselh { 
        unsafe {
            read_volatile(self.extipinselh_ptr())
        }
    }

    #[doc="Write the EXTIPINSELH register."]
    #[inline] pub fn set_extipinselh<F: FnOnce(Extipinselh) -> Extipinselh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extipinselh_mut(), f(Extipinselh(0)));
        }
        self
    }

    #[doc="Modify the EXTIPINSELH register."]
    #[inline] pub fn with_extipinselh<F: FnOnce(Extipinselh) -> Extipinselh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extipinselh_mut(), f(self.extipinselh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTIRISE register."]
    #[inline] pub fn extirise_mut(&self) -> *mut Extirise { 
        (self.0 + 0x410) as *mut Extirise
    }

    #[doc="Get the *const pointer for the EXTIRISE register."]
    #[inline] pub fn extirise_ptr(&self) -> *const Extirise { 
           self.extirise_mut()
    }

    #[doc="Read the EXTIRISE register."]
    #[inline] pub fn extirise(&self) -> Extirise { 
        unsafe {
            read_volatile(self.extirise_ptr())
        }
    }

    #[doc="Write the EXTIRISE register."]
    #[inline] pub fn set_extirise<F: FnOnce(Extirise) -> Extirise>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extirise_mut(), f(Extirise(0)));
        }
        self
    }

    #[doc="Modify the EXTIRISE register."]
    #[inline] pub fn with_extirise<F: FnOnce(Extirise) -> Extirise>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extirise_mut(), f(self.extirise()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTIFALL register."]
    #[inline] pub fn extifall_mut(&self) -> *mut Extifall { 
        (self.0 + 0x414) as *mut Extifall
    }

    #[doc="Get the *const pointer for the EXTIFALL register."]
    #[inline] pub fn extifall_ptr(&self) -> *const Extifall { 
           self.extifall_mut()
    }

    #[doc="Read the EXTIFALL register."]
    #[inline] pub fn extifall(&self) -> Extifall { 
        unsafe {
            read_volatile(self.extifall_ptr())
        }
    }

    #[doc="Write the EXTIFALL register."]
    #[inline] pub fn set_extifall<F: FnOnce(Extifall) -> Extifall>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extifall_mut(), f(Extifall(0)));
        }
        self
    }

    #[doc="Modify the EXTIFALL register."]
    #[inline] pub fn with_extifall<F: FnOnce(Extifall) -> Extifall>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extifall_mut(), f(self.extifall()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTILEVEL register."]
    #[inline] pub fn extilevel_mut(&self) -> *mut Extilevel { 
        (self.0 + 0x418) as *mut Extilevel
    }

    #[doc="Get the *const pointer for the EXTILEVEL register."]
    #[inline] pub fn extilevel_ptr(&self) -> *const Extilevel { 
           self.extilevel_mut()
    }

    #[doc="Read the EXTILEVEL register."]
    #[inline] pub fn extilevel(&self) -> Extilevel { 
        unsafe {
            read_volatile(self.extilevel_ptr())
        }
    }

    #[doc="Write the EXTILEVEL register."]
    #[inline] pub fn set_extilevel<F: FnOnce(Extilevel) -> Extilevel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extilevel_mut(), f(Extilevel(0)));
        }
        self
    }

    #[doc="Modify the EXTILEVEL register."]
    #[inline] pub fn with_extilevel<F: FnOnce(Extilevel) -> Extilevel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extilevel_mut(), f(self.extilevel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IF register."]
    #[inline] pub fn if_mut(&self) -> *mut If { 
        (self.0 + 0x41c) as *mut If
    }

    #[doc="Get the *const pointer for the IF register."]
    #[inline] pub fn if_ptr(&self) -> *const If { 
           self.if_mut()
    }

    #[doc="Read the IF register."]
    #[inline] pub fn _if(&self) -> If { 
        unsafe {
            read_volatile(self.if_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IFS register."]
    #[inline] pub fn ifs_mut(&self) -> *mut Ifs { 
        (self.0 + 0x420) as *mut Ifs
    }

    #[doc="Get the *const pointer for the IFS register."]
    #[inline] pub fn ifs_ptr(&self) -> *const Ifs { 
           self.ifs_mut()
    }

    #[doc="Write the IFS register."]
    #[inline] pub fn set_ifs<F: FnOnce(Ifs) -> Ifs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifs_mut(), f(Ifs(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IFC register."]
    #[inline] pub fn ifc_mut(&self) -> *mut Ifc { 
        (self.0 + 0x424) as *mut Ifc
    }

    #[doc="Get the *const pointer for the IFC register."]
    #[inline] pub fn ifc_ptr(&self) -> *const Ifc { 
           self.ifc_mut()
    }

    #[doc="Write the IFC register."]
    #[inline] pub fn set_ifc<F: FnOnce(Ifc) -> Ifc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifc_mut(), f(Ifc(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IEN register."]
    #[inline] pub fn ien_mut(&self) -> *mut Ien { 
        (self.0 + 0x428) as *mut Ien
    }

    #[doc="Get the *const pointer for the IEN register."]
    #[inline] pub fn ien_ptr(&self) -> *const Ien { 
           self.ien_mut()
    }

    #[doc="Read the IEN register."]
    #[inline] pub fn ien(&self) -> Ien { 
        unsafe {
            read_volatile(self.ien_ptr())
        }
    }

    #[doc="Write the IEN register."]
    #[inline] pub fn set_ien<F: FnOnce(Ien) -> Ien>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ien_mut(), f(Ien(0)));
        }
        self
    }

    #[doc="Modify the IEN register."]
    #[inline] pub fn with_ien<F: FnOnce(Ien) -> Ien>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ien_mut(), f(self.ien()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EM4WUEN register."]
    #[inline] pub fn em4wuen_mut(&self) -> *mut Em4wuen { 
        (self.0 + 0x42c) as *mut Em4wuen
    }

    #[doc="Get the *const pointer for the EM4WUEN register."]
    #[inline] pub fn em4wuen_ptr(&self) -> *const Em4wuen { 
           self.em4wuen_mut()
    }

    #[doc="Read the EM4WUEN register."]
    #[inline] pub fn em4wuen(&self) -> Em4wuen { 
        unsafe {
            read_volatile(self.em4wuen_ptr())
        }
    }

    #[doc="Write the EM4WUEN register."]
    #[inline] pub fn set_em4wuen<F: FnOnce(Em4wuen) -> Em4wuen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.em4wuen_mut(), f(Em4wuen(0)));
        }
        self
    }

    #[doc="Modify the EM4WUEN register."]
    #[inline] pub fn with_em4wuen<F: FnOnce(Em4wuen) -> Em4wuen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.em4wuen_mut(), f(self.em4wuen()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ROUTEPEN register."]
    #[inline] pub fn routepen_mut(&self) -> *mut Routepen { 
        (self.0 + 0x440) as *mut Routepen
    }

    #[doc="Get the *const pointer for the ROUTEPEN register."]
    #[inline] pub fn routepen_ptr(&self) -> *const Routepen { 
           self.routepen_mut()
    }

    #[doc="Read the ROUTEPEN register."]
    #[inline] pub fn routepen(&self) -> Routepen { 
        unsafe {
            read_volatile(self.routepen_ptr())
        }
    }

    #[doc="Write the ROUTEPEN register."]
    #[inline] pub fn set_routepen<F: FnOnce(Routepen) -> Routepen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routepen_mut(), f(Routepen(0)));
        }
        self
    }

    #[doc="Modify the ROUTEPEN register."]
    #[inline] pub fn with_routepen<F: FnOnce(Routepen) -> Routepen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routepen_mut(), f(self.routepen()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ROUTELOC0 register."]
    #[inline] pub fn routeloc0_mut(&self) -> *mut Routeloc0 { 
        (self.0 + 0x444) as *mut Routeloc0
    }

    #[doc="Get the *const pointer for the ROUTELOC0 register."]
    #[inline] pub fn routeloc0_ptr(&self) -> *const Routeloc0 { 
           self.routeloc0_mut()
    }

    #[doc="Read the ROUTELOC0 register."]
    #[inline] pub fn routeloc0(&self) -> Routeloc0 { 
        unsafe {
            read_volatile(self.routeloc0_ptr())
        }
    }

    #[doc="Write the ROUTELOC0 register."]
    #[inline] pub fn set_routeloc0<F: FnOnce(Routeloc0) -> Routeloc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc0_mut(), f(Routeloc0(0)));
        }
        self
    }

    #[doc="Modify the ROUTELOC0 register."]
    #[inline] pub fn with_routeloc0<F: FnOnce(Routeloc0) -> Routeloc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc0_mut(), f(self.routeloc0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ROUTELOC1 register."]
    #[inline] pub fn routeloc1_mut(&self) -> *mut Routeloc1 { 
        (self.0 + 0x448) as *mut Routeloc1
    }

    #[doc="Get the *const pointer for the ROUTELOC1 register."]
    #[inline] pub fn routeloc1_ptr(&self) -> *const Routeloc1 { 
           self.routeloc1_mut()
    }

    #[doc="Read the ROUTELOC1 register."]
    #[inline] pub fn routeloc1(&self) -> Routeloc1 { 
        unsafe {
            read_volatile(self.routeloc1_ptr())
        }
    }

    #[doc="Write the ROUTELOC1 register."]
    #[inline] pub fn set_routeloc1<F: FnOnce(Routeloc1) -> Routeloc1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc1_mut(), f(Routeloc1(0)));
        }
        self
    }

    #[doc="Modify the ROUTELOC1 register."]
    #[inline] pub fn with_routeloc1<F: FnOnce(Routeloc1) -> Routeloc1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.routeloc1_mut(), f(self.routeloc1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INSENSE register."]
    #[inline] pub fn insense_mut(&self) -> *mut Insense { 
        (self.0 + 0x450) as *mut Insense
    }

    #[doc="Get the *const pointer for the INSENSE register."]
    #[inline] pub fn insense_ptr(&self) -> *const Insense { 
           self.insense_mut()
    }

    #[doc="Read the INSENSE register."]
    #[inline] pub fn insense(&self) -> Insense { 
        unsafe {
            read_volatile(self.insense_ptr())
        }
    }

    #[doc="Write the INSENSE register."]
    #[inline] pub fn set_insense<F: FnOnce(Insense) -> Insense>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.insense_mut(), f(Insense(0)));
        }
        self
    }

    #[doc="Modify the INSENSE register."]
    #[inline] pub fn with_insense<F: FnOnce(Insense) -> Insense>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.insense_mut(), f(self.insense()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LOCK register."]
    #[inline] pub fn lock_mut(&self) -> *mut Lock { 
        (self.0 + 0x454) as *mut Lock
    }

    #[doc="Get the *const pointer for the LOCK register."]
    #[inline] pub fn lock_ptr(&self) -> *const Lock { 
           self.lock_mut()
    }

    #[doc="Read the LOCK register."]
    #[inline] pub fn lock(&self) -> Lock { 
        unsafe {
            read_volatile(self.lock_ptr())
        }
    }

    #[doc="Write the LOCK register."]
    #[inline] pub fn set_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lock_mut(), f(Lock(0)));
        }
        self
    }

    #[doc="Modify the LOCK register."]
    #[inline] pub fn with_lock<F: FnOnce(Lock) -> Lock>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lock_mut(), f(self.lock()));
        }
        self
    }

}

#[doc="External Interrupt Port Select Low Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Extipsell(pub u32);
impl Extipsell {
    #[doc="External Interrupt 0 Port Select"]
    #[inline] pub fn extipsel0(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTIPSEL0 != 0"]
    #[inline] pub fn test_extipsel0(&self) -> bool {
        self.extipsel0() != 0
    }

    #[doc="Sets the EXTIPSEL0 field."]
    #[inline] pub fn set_extipsel0<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="External Interrupt 1 Port Select"]
    #[inline] pub fn extipsel1(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTIPSEL1 != 0"]
    #[inline] pub fn test_extipsel1(&self) -> bool {
        self.extipsel1() != 0
    }

    #[doc="Sets the EXTIPSEL1 field."]
    #[inline] pub fn set_extipsel1<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="External Interrupt 2 Port Select"]
    #[inline] pub fn extipsel2(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTIPSEL2 != 0"]
    #[inline] pub fn test_extipsel2(&self) -> bool {
        self.extipsel2() != 0
    }

    #[doc="Sets the EXTIPSEL2 field."]
    #[inline] pub fn set_extipsel2<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="External Interrupt 3 Port Select"]
    #[inline] pub fn extipsel3(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTIPSEL3 != 0"]
    #[inline] pub fn test_extipsel3(&self) -> bool {
        self.extipsel3() != 0
    }

    #[doc="Sets the EXTIPSEL3 field."]
    #[inline] pub fn set_extipsel3<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="External Interrupt 4 Port Select"]
    #[inline] pub fn extipsel4(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if EXTIPSEL4 != 0"]
    #[inline] pub fn test_extipsel4(&self) -> bool {
        self.extipsel4() != 0
    }

    #[doc="Sets the EXTIPSEL4 field."]
    #[inline] pub fn set_extipsel4<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="External Interrupt 5 Port Select"]
    #[inline] pub fn extipsel5(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if EXTIPSEL5 != 0"]
    #[inline] pub fn test_extipsel5(&self) -> bool {
        self.extipsel5() != 0
    }

    #[doc="Sets the EXTIPSEL5 field."]
    #[inline] pub fn set_extipsel5<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="External Interrupt 6 Port Select"]
    #[inline] pub fn extipsel6(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if EXTIPSEL6 != 0"]
    #[inline] pub fn test_extipsel6(&self) -> bool {
        self.extipsel6() != 0
    }

    #[doc="Sets the EXTIPSEL6 field."]
    #[inline] pub fn set_extipsel6<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="External Interrupt 7 Port Select"]
    #[inline] pub fn extipsel7(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if EXTIPSEL7 != 0"]
    #[inline] pub fn test_extipsel7(&self) -> bool {
        self.extipsel7() != 0
    }

    #[doc="Sets the EXTIPSEL7 field."]
    #[inline] pub fn set_extipsel7<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Extipsell {
    #[inline]
    fn from(other: u32) -> Self {
         Extipsell(other)
    }
}

impl ::core::fmt::Display for Extipsell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Extipsell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.extipsel0() != 0 { try!(write!(f, " extipsel0=0x{:x}", self.extipsel0()))}
        if self.extipsel1() != 0 { try!(write!(f, " extipsel1=0x{:x}", self.extipsel1()))}
        if self.extipsel2() != 0 { try!(write!(f, " extipsel2=0x{:x}", self.extipsel2()))}
        if self.extipsel3() != 0 { try!(write!(f, " extipsel3=0x{:x}", self.extipsel3()))}
        if self.extipsel4() != 0 { try!(write!(f, " extipsel4=0x{:x}", self.extipsel4()))}
        if self.extipsel5() != 0 { try!(write!(f, " extipsel5=0x{:x}", self.extipsel5()))}
        if self.extipsel6() != 0 { try!(write!(f, " extipsel6=0x{:x}", self.extipsel6()))}
        if self.extipsel7() != 0 { try!(write!(f, " extipsel7=0x{:x}", self.extipsel7()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External Interrupt Port Select High Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Extipselh(pub u32);
impl Extipselh {
    #[doc="External Interrupt 8 Port Select"]
    #[inline] pub fn extipsel8(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTIPSEL8 != 0"]
    #[inline] pub fn test_extipsel8(&self) -> bool {
        self.extipsel8() != 0
    }

    #[doc="Sets the EXTIPSEL8 field."]
    #[inline] pub fn set_extipsel8<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="External Interrupt 9 Port Select"]
    #[inline] pub fn extipsel9(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTIPSEL9 != 0"]
    #[inline] pub fn test_extipsel9(&self) -> bool {
        self.extipsel9() != 0
    }

    #[doc="Sets the EXTIPSEL9 field."]
    #[inline] pub fn set_extipsel9<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="External Interrupt 10 Port Select"]
    #[inline] pub fn extipsel10(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTIPSEL10 != 0"]
    #[inline] pub fn test_extipsel10(&self) -> bool {
        self.extipsel10() != 0
    }

    #[doc="Sets the EXTIPSEL10 field."]
    #[inline] pub fn set_extipsel10<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="External Interrupt 11 Port Select"]
    #[inline] pub fn extipsel11(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTIPSEL11 != 0"]
    #[inline] pub fn test_extipsel11(&self) -> bool {
        self.extipsel11() != 0
    }

    #[doc="Sets the EXTIPSEL11 field."]
    #[inline] pub fn set_extipsel11<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="External Interrupt 12 Port Select"]
    #[inline] pub fn extipsel12(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if EXTIPSEL12 != 0"]
    #[inline] pub fn test_extipsel12(&self) -> bool {
        self.extipsel12() != 0
    }

    #[doc="Sets the EXTIPSEL12 field."]
    #[inline] pub fn set_extipsel12<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="External Interrupt 13 Port Select"]
    #[inline] pub fn extipsel13(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if EXTIPSEL13 != 0"]
    #[inline] pub fn test_extipsel13(&self) -> bool {
        self.extipsel13() != 0
    }

    #[doc="Sets the EXTIPSEL13 field."]
    #[inline] pub fn set_extipsel13<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="External Interrupt 14 Port Select"]
    #[inline] pub fn extipsel14(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if EXTIPSEL14 != 0"]
    #[inline] pub fn test_extipsel14(&self) -> bool {
        self.extipsel14() != 0
    }

    #[doc="Sets the EXTIPSEL14 field."]
    #[inline] pub fn set_extipsel14<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="External Interrupt 15 Port Select"]
    #[inline] pub fn extipsel15(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if EXTIPSEL15 != 0"]
    #[inline] pub fn test_extipsel15(&self) -> bool {
        self.extipsel15() != 0
    }

    #[doc="Sets the EXTIPSEL15 field."]
    #[inline] pub fn set_extipsel15<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Extipselh {
    #[inline]
    fn from(other: u32) -> Self {
         Extipselh(other)
    }
}

impl ::core::fmt::Display for Extipselh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Extipselh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.extipsel8() != 0 { try!(write!(f, " extipsel8=0x{:x}", self.extipsel8()))}
        if self.extipsel9() != 0 { try!(write!(f, " extipsel9=0x{:x}", self.extipsel9()))}
        if self.extipsel10() != 0 { try!(write!(f, " extipsel10=0x{:x}", self.extipsel10()))}
        if self.extipsel11() != 0 { try!(write!(f, " extipsel11=0x{:x}", self.extipsel11()))}
        if self.extipsel12() != 0 { try!(write!(f, " extipsel12=0x{:x}", self.extipsel12()))}
        if self.extipsel13() != 0 { try!(write!(f, " extipsel13=0x{:x}", self.extipsel13()))}
        if self.extipsel14() != 0 { try!(write!(f, " extipsel14=0x{:x}", self.extipsel14()))}
        if self.extipsel15() != 0 { try!(write!(f, " extipsel15=0x{:x}", self.extipsel15()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External Interrupt Pin Select Low Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Extipinsell(pub u32);
impl Extipinsell {
    #[doc="External Interrupt 0 Pin Select"]
    #[inline] pub fn extipinsel0(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if EXTIPINSEL0 != 0"]
    #[inline] pub fn test_extipinsel0(&self) -> bool {
        self.extipinsel0() != 0
    }

    #[doc="Sets the EXTIPINSEL0 field."]
    #[inline] pub fn set_extipinsel0<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="External Interrupt 1 Pin Select"]
    #[inline] pub fn extipinsel1(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if EXTIPINSEL1 != 0"]
    #[inline] pub fn test_extipinsel1(&self) -> bool {
        self.extipinsel1() != 0
    }

    #[doc="Sets the EXTIPINSEL1 field."]
    #[inline] pub fn set_extipinsel1<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="External Interrupt 2 Pin Select"]
    #[inline] pub fn extipinsel2(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if EXTIPINSEL2 != 0"]
    #[inline] pub fn test_extipinsel2(&self) -> bool {
        self.extipinsel2() != 0
    }

    #[doc="Sets the EXTIPINSEL2 field."]
    #[inline] pub fn set_extipinsel2<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="External Interrupt 3 Pin Select"]
    #[inline] pub fn extipinsel3(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if EXTIPINSEL3 != 0"]
    #[inline] pub fn test_extipinsel3(&self) -> bool {
        self.extipinsel3() != 0
    }

    #[doc="Sets the EXTIPINSEL3 field."]
    #[inline] pub fn set_extipinsel3<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="External Interrupt 4 Pin Select"]
    #[inline] pub fn extipinsel4(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if EXTIPINSEL4 != 0"]
    #[inline] pub fn test_extipinsel4(&self) -> bool {
        self.extipinsel4() != 0
    }

    #[doc="Sets the EXTIPINSEL4 field."]
    #[inline] pub fn set_extipinsel4<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="External Interrupt 5 Pin Select"]
    #[inline] pub fn extipinsel5(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if EXTIPINSEL5 != 0"]
    #[inline] pub fn test_extipinsel5(&self) -> bool {
        self.extipinsel5() != 0
    }

    #[doc="Sets the EXTIPINSEL5 field."]
    #[inline] pub fn set_extipinsel5<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="External Interrupt 6 Pin Select"]
    #[inline] pub fn extipinsel6(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if EXTIPINSEL6 != 0"]
    #[inline] pub fn test_extipinsel6(&self) -> bool {
        self.extipinsel6() != 0
    }

    #[doc="Sets the EXTIPINSEL6 field."]
    #[inline] pub fn set_extipinsel6<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="External Interrupt 7 Pin Select"]
    #[inline] pub fn extipinsel7(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if EXTIPINSEL7 != 0"]
    #[inline] pub fn test_extipinsel7(&self) -> bool {
        self.extipinsel7() != 0
    }

    #[doc="Sets the EXTIPINSEL7 field."]
    #[inline] pub fn set_extipinsel7<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Extipinsell {
    #[inline]
    fn from(other: u32) -> Self {
         Extipinsell(other)
    }
}

impl ::core::fmt::Display for Extipinsell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Extipinsell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.extipinsel0() != 0 { try!(write!(f, " extipinsel0=0x{:x}", self.extipinsel0()))}
        if self.extipinsel1() != 0 { try!(write!(f, " extipinsel1=0x{:x}", self.extipinsel1()))}
        if self.extipinsel2() != 0 { try!(write!(f, " extipinsel2=0x{:x}", self.extipinsel2()))}
        if self.extipinsel3() != 0 { try!(write!(f, " extipinsel3=0x{:x}", self.extipinsel3()))}
        if self.extipinsel4() != 0 { try!(write!(f, " extipinsel4=0x{:x}", self.extipinsel4()))}
        if self.extipinsel5() != 0 { try!(write!(f, " extipinsel5=0x{:x}", self.extipinsel5()))}
        if self.extipinsel6() != 0 { try!(write!(f, " extipinsel6=0x{:x}", self.extipinsel6()))}
        if self.extipinsel7() != 0 { try!(write!(f, " extipinsel7=0x{:x}", self.extipinsel7()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External Interrupt Pin Select High Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Extipinselh(pub u32);
impl Extipinselh {
    #[doc="External Interrupt 8 Pin Select"]
    #[inline] pub fn extipinsel8(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if EXTIPINSEL8 != 0"]
    #[inline] pub fn test_extipinsel8(&self) -> bool {
        self.extipinsel8() != 0
    }

    #[doc="Sets the EXTIPINSEL8 field."]
    #[inline] pub fn set_extipinsel8<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="External Interrupt 9 Pin Select"]
    #[inline] pub fn extipinsel9(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if EXTIPINSEL9 != 0"]
    #[inline] pub fn test_extipinsel9(&self) -> bool {
        self.extipinsel9() != 0
    }

    #[doc="Sets the EXTIPINSEL9 field."]
    #[inline] pub fn set_extipinsel9<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="External Interrupt 10 Pin Select"]
    #[inline] pub fn extipinsel10(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if EXTIPINSEL10 != 0"]
    #[inline] pub fn test_extipinsel10(&self) -> bool {
        self.extipinsel10() != 0
    }

    #[doc="Sets the EXTIPINSEL10 field."]
    #[inline] pub fn set_extipinsel10<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="External Interrupt 11 Pin Select"]
    #[inline] pub fn extipinsel11(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if EXTIPINSEL11 != 0"]
    #[inline] pub fn test_extipinsel11(&self) -> bool {
        self.extipinsel11() != 0
    }

    #[doc="Sets the EXTIPINSEL11 field."]
    #[inline] pub fn set_extipinsel11<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="External Interrupt 12 Pin Select"]
    #[inline] pub fn extipinsel12(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if EXTIPINSEL12 != 0"]
    #[inline] pub fn test_extipinsel12(&self) -> bool {
        self.extipinsel12() != 0
    }

    #[doc="Sets the EXTIPINSEL12 field."]
    #[inline] pub fn set_extipinsel12<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="External Interrupt 13 Pin Select"]
    #[inline] pub fn extipinsel13(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if EXTIPINSEL13 != 0"]
    #[inline] pub fn test_extipinsel13(&self) -> bool {
        self.extipinsel13() != 0
    }

    #[doc="Sets the EXTIPINSEL13 field."]
    #[inline] pub fn set_extipinsel13<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="External Interrupt 14 Pin Select"]
    #[inline] pub fn extipinsel14(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if EXTIPINSEL14 != 0"]
    #[inline] pub fn test_extipinsel14(&self) -> bool {
        self.extipinsel14() != 0
    }

    #[doc="Sets the EXTIPINSEL14 field."]
    #[inline] pub fn set_extipinsel14<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="External Interrupt 15 Pin Select"]
    #[inline] pub fn extipinsel15(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if EXTIPINSEL15 != 0"]
    #[inline] pub fn test_extipinsel15(&self) -> bool {
        self.extipinsel15() != 0
    }

    #[doc="Sets the EXTIPINSEL15 field."]
    #[inline] pub fn set_extipinsel15<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Extipinselh {
    #[inline]
    fn from(other: u32) -> Self {
         Extipinselh(other)
    }
}

impl ::core::fmt::Display for Extipinselh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Extipinselh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.extipinsel8() != 0 { try!(write!(f, " extipinsel8=0x{:x}", self.extipinsel8()))}
        if self.extipinsel9() != 0 { try!(write!(f, " extipinsel9=0x{:x}", self.extipinsel9()))}
        if self.extipinsel10() != 0 { try!(write!(f, " extipinsel10=0x{:x}", self.extipinsel10()))}
        if self.extipinsel11() != 0 { try!(write!(f, " extipinsel11=0x{:x}", self.extipinsel11()))}
        if self.extipinsel12() != 0 { try!(write!(f, " extipinsel12=0x{:x}", self.extipinsel12()))}
        if self.extipinsel13() != 0 { try!(write!(f, " extipinsel13=0x{:x}", self.extipinsel13()))}
        if self.extipinsel14() != 0 { try!(write!(f, " extipinsel14=0x{:x}", self.extipinsel14()))}
        if self.extipinsel15() != 0 { try!(write!(f, " extipinsel15=0x{:x}", self.extipinsel15()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External Interrupt Rising Edge Trigger Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Extirise(pub u32);
impl Extirise {
    #[doc="External Interrupt n Rising Edge Trigger Enable"]
    #[inline] pub fn extirise(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if EXTIRISE != 0"]
    #[inline] pub fn test_extirise(&self) -> bool {
        self.extirise() != 0
    }

    #[doc="Sets the EXTIRISE field."]
    #[inline] pub fn set_extirise<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Extirise {
    #[inline]
    fn from(other: u32) -> Self {
         Extirise(other)
    }
}

impl ::core::fmt::Display for Extirise {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Extirise {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.extirise() != 0 { try!(write!(f, " extirise=0x{:x}", self.extirise()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External Interrupt Falling Edge Trigger Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Extifall(pub u32);
impl Extifall {
    #[doc="External Interrupt n Falling Edge Trigger Enable"]
    #[inline] pub fn extifall(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if EXTIFALL != 0"]
    #[inline] pub fn test_extifall(&self) -> bool {
        self.extifall() != 0
    }

    #[doc="Sets the EXTIFALL field."]
    #[inline] pub fn set_extifall<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Extifall {
    #[inline]
    fn from(other: u32) -> Self {
         Extifall(other)
    }
}

impl ::core::fmt::Display for Extifall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Extifall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.extifall() != 0 { try!(write!(f, " extifall=0x{:x}", self.extifall()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External Interrupt Level Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Extilevel(pub u32);
impl Extilevel {
    #[doc="EM4 Wake Up Level for EM4WU0 Pin"]
    #[inline] pub fn em4wu0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if EM4WU0 != 0"]
    #[inline] pub fn test_em4wu0(&self) -> bool {
        self.em4wu0() != 0
    }

    #[doc="Sets the EM4WU0 field."]
    #[inline] pub fn set_em4wu0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="EM4 Wake Up Level for EM4WU1 Pin"]
    #[inline] pub fn em4wu1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if EM4WU1 != 0"]
    #[inline] pub fn test_em4wu1(&self) -> bool {
        self.em4wu1() != 0
    }

    #[doc="Sets the EM4WU1 field."]
    #[inline] pub fn set_em4wu1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="EM4 Wake Up Level for EM4WU4 Pin"]
    #[inline] pub fn em4wu4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if EM4WU4 != 0"]
    #[inline] pub fn test_em4wu4(&self) -> bool {
        self.em4wu4() != 0
    }

    #[doc="Sets the EM4WU4 field."]
    #[inline] pub fn set_em4wu4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="EM4 Wake Up Level for EM4WU8 Pin"]
    #[inline] pub fn em4wu8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if EM4WU8 != 0"]
    #[inline] pub fn test_em4wu8(&self) -> bool {
        self.em4wu8() != 0
    }

    #[doc="Sets the EM4WU8 field."]
    #[inline] pub fn set_em4wu8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="EM4 Wake Up Level for EM4WU9 Pin"]
    #[inline] pub fn em4wu9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if EM4WU9 != 0"]
    #[inline] pub fn test_em4wu9(&self) -> bool {
        self.em4wu9() != 0
    }

    #[doc="Sets the EM4WU9 field."]
    #[inline] pub fn set_em4wu9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="EM4 Wake Up Level for EM4WU12 Pin"]
    #[inline] pub fn em4wu12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if EM4WU12 != 0"]
    #[inline] pub fn test_em4wu12(&self) -> bool {
        self.em4wu12() != 0
    }

    #[doc="Sets the EM4WU12 field."]
    #[inline] pub fn set_em4wu12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Extilevel {
    #[inline]
    fn from(other: u32) -> Self {
         Extilevel(other)
    }
}

impl ::core::fmt::Display for Extilevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Extilevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.em4wu0() != 0 { try!(write!(f, " em4wu0"))}
        if self.em4wu1() != 0 { try!(write!(f, " em4wu1"))}
        if self.em4wu4() != 0 { try!(write!(f, " em4wu4"))}
        if self.em4wu8() != 0 { try!(write!(f, " em4wu8"))}
        if self.em4wu9() != 0 { try!(write!(f, " em4wu9"))}
        if self.em4wu12() != 0 { try!(write!(f, " em4wu12"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct If(pub u32);
impl If {
    #[doc="External Pin Interrupt Flag"]
    #[inline] pub fn ext(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if EXT != 0"]
    #[inline] pub fn test_ext(&self) -> bool {
        self.ext() != 0
    }

    #[doc="Sets the EXT field."]
    #[inline] pub fn set_ext<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="EM4 wake up Pin Interrupt Flag"]
    #[inline] pub fn em4wu(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if EM4WU != 0"]
    #[inline] pub fn test_em4wu(&self) -> bool {
        self.em4wu() != 0
    }

    #[doc="Sets the EM4WU field."]
    #[inline] pub fn set_em4wu<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for If {
    #[inline]
    fn from(other: u32) -> Self {
         If(other)
    }
}

impl ::core::fmt::Display for If {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for If {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ext() != 0 { try!(write!(f, " ext=0x{:x}", self.ext()))}
        if self.em4wu() != 0 { try!(write!(f, " em4wu=0x{:x}", self.em4wu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Set Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc="Set EXT Interrupt Flag"]
    #[inline] pub fn ext(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if EXT != 0"]
    #[inline] pub fn test_ext(&self) -> bool {
        self.ext() != 0
    }

    #[doc="Sets the EXT field."]
    #[inline] pub fn set_ext<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Set EM4WU Interrupt Flag"]
    #[inline] pub fn em4wu(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if EM4WU != 0"]
    #[inline] pub fn test_em4wu(&self) -> bool {
        self.em4wu() != 0
    }

    #[doc="Sets the EM4WU field."]
    #[inline] pub fn set_em4wu<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Ifs {
    #[inline]
    fn from(other: u32) -> Self {
         Ifs(other)
    }
}

impl ::core::fmt::Display for Ifs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ext() != 0 { try!(write!(f, " ext=0x{:x}", self.ext()))}
        if self.em4wu() != 0 { try!(write!(f, " em4wu=0x{:x}", self.em4wu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Clear Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifc(pub u32);
impl Ifc {
    #[doc="Clear EXT Interrupt Flag"]
    #[inline] pub fn ext(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if EXT != 0"]
    #[inline] pub fn test_ext(&self) -> bool {
        self.ext() != 0
    }

    #[doc="Sets the EXT field."]
    #[inline] pub fn set_ext<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clear EM4WU Interrupt Flag"]
    #[inline] pub fn em4wu(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if EM4WU != 0"]
    #[inline] pub fn test_em4wu(&self) -> bool {
        self.em4wu() != 0
    }

    #[doc="Sets the EM4WU field."]
    #[inline] pub fn set_em4wu<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Ifc {
    #[inline]
    fn from(other: u32) -> Self {
         Ifc(other)
    }
}

impl ::core::fmt::Display for Ifc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ext() != 0 { try!(write!(f, " ext=0x{:x}", self.ext()))}
        if self.em4wu() != 0 { try!(write!(f, " em4wu=0x{:x}", self.em4wu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc="EXT Interrupt Enable"]
    #[inline] pub fn ext(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if EXT != 0"]
    #[inline] pub fn test_ext(&self) -> bool {
        self.ext() != 0
    }

    #[doc="Sets the EXT field."]
    #[inline] pub fn set_ext<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="EM4WU Interrupt Enable"]
    #[inline] pub fn em4wu(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if EM4WU != 0"]
    #[inline] pub fn test_em4wu(&self) -> bool {
        self.em4wu() != 0
    }

    #[doc="Sets the EM4WU field."]
    #[inline] pub fn set_em4wu<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Ien {
    #[inline]
    fn from(other: u32) -> Self {
         Ien(other)
    }
}

impl ::core::fmt::Display for Ien {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ien {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ext() != 0 { try!(write!(f, " ext=0x{:x}", self.ext()))}
        if self.em4wu() != 0 { try!(write!(f, " em4wu=0x{:x}", self.em4wu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EM4 wake up Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Em4wuen(pub u32);
impl Em4wuen {
    #[doc="EM4 wake up enable"]
    #[inline] pub fn em4wuen(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if EM4WUEN != 0"]
    #[inline] pub fn test_em4wuen(&self) -> bool {
        self.em4wuen() != 0
    }

    #[doc="Sets the EM4WUEN field."]
    #[inline] pub fn set_em4wuen<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Em4wuen {
    #[inline]
    fn from(other: u32) -> Self {
         Em4wuen(other)
    }
}

impl ::core::fmt::Display for Em4wuen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Em4wuen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.em4wuen() != 0 { try!(write!(f, " em4wuen=0x{:x}", self.em4wuen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Routing Pin Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc="Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline] pub fn swclktckpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWCLKTCKPEN != 0"]
    #[inline] pub fn test_swclktckpen(&self) -> bool {
        self.swclktckpen() != 0
    }

    #[doc="Sets the SWCLKTCKPEN field."]
    #[inline] pub fn set_swclktckpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline] pub fn swdiotmspen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SWDIOTMSPEN != 0"]
    #[inline] pub fn test_swdiotmspen(&self) -> bool {
        self.swdiotmspen() != 0
    }

    #[doc="Sets the SWDIOTMSPEN field."]
    #[inline] pub fn set_swdiotmspen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="JTAG Test Debug Output Pin Enable"]
    #[inline] pub fn tdopen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TDOPEN != 0"]
    #[inline] pub fn test_tdopen(&self) -> bool {
        self.tdopen() != 0
    }

    #[doc="Sets the TDOPEN field."]
    #[inline] pub fn set_tdopen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="JTAG Test Debug Input Pin Enable"]
    #[inline] pub fn tdipen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TDIPEN != 0"]
    #[inline] pub fn test_tdipen(&self) -> bool {
        self.tdipen() != 0
    }

    #[doc="Sets the TDIPEN field."]
    #[inline] pub fn set_tdipen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Serial Wire Viewer Output Pin Enable"]
    #[inline] pub fn swvpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SWVPEN != 0"]
    #[inline] pub fn test_swvpen(&self) -> bool {
        self.swvpen() != 0
    }

    #[doc="Sets the SWVPEN field."]
    #[inline] pub fn set_swvpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ETM Trace Clock Pin Enable"]
    #[inline] pub fn etmtclkpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if ETMTCLKPEN != 0"]
    #[inline] pub fn test_etmtclkpen(&self) -> bool {
        self.etmtclkpen() != 0
    }

    #[doc="Sets the ETMTCLKPEN field."]
    #[inline] pub fn set_etmtclkpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ETM Trace Data Pin Enable"]
    #[inline] pub fn etmtd0pen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if ETMTD0PEN != 0"]
    #[inline] pub fn test_etmtd0pen(&self) -> bool {
        self.etmtd0pen() != 0
    }

    #[doc="Sets the ETMTD0PEN field."]
    #[inline] pub fn set_etmtd0pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="ETM Trace Data Pin Enable"]
    #[inline] pub fn etmtd1pen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ETMTD1PEN != 0"]
    #[inline] pub fn test_etmtd1pen(&self) -> bool {
        self.etmtd1pen() != 0
    }

    #[doc="Sets the ETMTD1PEN field."]
    #[inline] pub fn set_etmtd1pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="ETM Trace Data Pin Enable"]
    #[inline] pub fn etmtd2pen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if ETMTD2PEN != 0"]
    #[inline] pub fn test_etmtd2pen(&self) -> bool {
        self.etmtd2pen() != 0
    }

    #[doc="Sets the ETMTD2PEN field."]
    #[inline] pub fn set_etmtd2pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="ETM Trace Data Pin Enable"]
    #[inline] pub fn etmtd3pen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if ETMTD3PEN != 0"]
    #[inline] pub fn test_etmtd3pen(&self) -> bool {
        self.etmtd3pen() != 0
    }

    #[doc="Sets the ETMTD3PEN field."]
    #[inline] pub fn set_etmtd3pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl From<u32> for Routepen {
    #[inline]
    fn from(other: u32) -> Self {
         Routepen(other)
    }
}

impl ::core::fmt::Display for Routepen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Routepen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swclktckpen() != 0 { try!(write!(f, " swclktckpen"))}
        if self.swdiotmspen() != 0 { try!(write!(f, " swdiotmspen"))}
        if self.tdopen() != 0 { try!(write!(f, " tdopen"))}
        if self.tdipen() != 0 { try!(write!(f, " tdipen"))}
        if self.swvpen() != 0 { try!(write!(f, " swvpen"))}
        if self.etmtclkpen() != 0 { try!(write!(f, " etmtclkpen"))}
        if self.etmtd0pen() != 0 { try!(write!(f, " etmtd0pen"))}
        if self.etmtd1pen() != 0 { try!(write!(f, " etmtd1pen"))}
        if self.etmtd2pen() != 0 { try!(write!(f, " etmtd2pen"))}
        if self.etmtd3pen() != 0 { try!(write!(f, " etmtd3pen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Routing Location Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Routeloc0(pub u32);
impl Routeloc0 {
    #[doc="I/O Location"]
    #[inline] pub fn swvloc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if SWVLOC != 0"]
    #[inline] pub fn test_swvloc(&self) -> bool {
        self.swvloc() != 0
    }

    #[doc="Sets the SWVLOC field."]
    #[inline] pub fn set_swvloc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Routeloc0 {
    #[inline]
    fn from(other: u32) -> Self {
         Routeloc0(other)
    }
}

impl ::core::fmt::Display for Routeloc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Routeloc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swvloc() != 0 { try!(write!(f, " swvloc=0x{:x}", self.swvloc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Routing Location Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Routeloc1(pub u32);
impl Routeloc1 {
    #[doc="I/O Location"]
    #[inline] pub fn etmtclkloc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if ETMTCLKLOC != 0"]
    #[inline] pub fn test_etmtclkloc(&self) -> bool {
        self.etmtclkloc() != 0
    }

    #[doc="Sets the ETMTCLKLOC field."]
    #[inline] pub fn set_etmtclkloc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I/O Location"]
    #[inline] pub fn etmtd0loc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if ETMTD0LOC != 0"]
    #[inline] pub fn test_etmtd0loc(&self) -> bool {
        self.etmtd0loc() != 0
    }

    #[doc="Sets the ETMTD0LOC field."]
    #[inline] pub fn set_etmtd0loc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I/O Location"]
    #[inline] pub fn etmtd1loc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3f) as u8) } // [19:14]
    }

    #[doc="Returns true if ETMTD1LOC != 0"]
    #[inline] pub fn test_etmtd1loc(&self) -> bool {
        self.etmtd1loc() != 0
    }

    #[doc="Sets the ETMTD1LOC field."]
    #[inline] pub fn set_etmtd1loc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="I/O Location"]
    #[inline] pub fn etmtd2loc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3f) as u8) } // [25:20]
    }

    #[doc="Returns true if ETMTD2LOC != 0"]
    #[inline] pub fn test_etmtd2loc(&self) -> bool {
        self.etmtd2loc() != 0
    }

    #[doc="Sets the ETMTD2LOC field."]
    #[inline] pub fn set_etmtd2loc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="I/O Location"]
    #[inline] pub fn etmtd3loc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
    }

    #[doc="Returns true if ETMTD3LOC != 0"]
    #[inline] pub fn test_etmtd3loc(&self) -> bool {
        self.etmtd3loc() != 0
    }

    #[doc="Sets the ETMTD3LOC field."]
    #[inline] pub fn set_etmtd3loc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Routeloc1 {
    #[inline]
    fn from(other: u32) -> Self {
         Routeloc1(other)
    }
}

impl ::core::fmt::Display for Routeloc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Routeloc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.etmtclkloc() != 0 { try!(write!(f, " etmtclkloc=0x{:x}", self.etmtclkloc()))}
        if self.etmtd0loc() != 0 { try!(write!(f, " etmtd0loc=0x{:x}", self.etmtd0loc()))}
        if self.etmtd1loc() != 0 { try!(write!(f, " etmtd1loc=0x{:x}", self.etmtd1loc()))}
        if self.etmtd2loc() != 0 { try!(write!(f, " etmtd2loc=0x{:x}", self.etmtd2loc()))}
        if self.etmtd3loc() != 0 { try!(write!(f, " etmtd3loc=0x{:x}", self.etmtd3loc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Input Sense Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Insense(pub u32);
impl Insense {
    #[doc="Interrupt Sense Enable"]
    #[inline] pub fn int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INT != 0"]
    #[inline] pub fn test_int(&self) -> bool {
        self.int() != 0
    }

    #[doc="Sets the INT field."]
    #[inline] pub fn set_int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="EM4WU Interrupt Sense Enable"]
    #[inline] pub fn em4wu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EM4WU != 0"]
    #[inline] pub fn test_em4wu(&self) -> bool {
        self.em4wu() != 0
    }

    #[doc="Sets the EM4WU field."]
    #[inline] pub fn set_em4wu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Insense {
    #[inline]
    fn from(other: u32) -> Self {
         Insense(other)
    }
}

impl ::core::fmt::Display for Insense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Insense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.int() != 0 { try!(write!(f, " int"))}
        if self.em4wu() != 0 { try!(write!(f, " em4wu"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration Lock Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc="Configuration Lock Key"]
    #[inline] pub fn lockkey(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if LOCKKEY != 0"]
    #[inline] pub fn test_lockkey(&self) -> bool {
        self.lockkey() != 0
    }

    #[doc="Sets the LOCKKEY field."]
    #[inline] pub fn set_lockkey<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Lock {
    #[inline]
    fn from(other: u32) -> Self {
         Lock(other)
    }
}

impl ::core::fmt::Display for Lock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lockkey() != 0 { try!(write!(f, " lockkey=0x{:x}", self.lockkey()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


