#[allow(unused_imports)] use ::bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USB Peripheral"]
pub struct UsbPeriph(pub usize); 

impl UsbPeriph {
    #[doc="Get the *mut pointer for the EP0R register."]
    #[inline] pub fn ep0r_mut(&self) -> *mut Ep0r { 
        (self.0 + 0x0) as *mut Ep0r
    }

    #[doc="Get the *const pointer for the EP0R register."]
    #[inline] pub fn ep0r_ptr(&self) -> *const Ep0r { 
           self.ep0r_mut()
    }

    #[doc="Read the EP0R register."]
    #[inline] pub fn ep0r(&self) -> Ep0r { 
        unsafe {
            read_volatile(self.ep0r_ptr())
        }
    }

    #[doc="Write the EP0R register."]
    #[inline] pub fn set_ep0r<F: FnOnce(Ep0r) -> Ep0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep0r_mut(), f(Ep0r(0)));
        }
        self
    }

    #[doc="Modify the EP0R register."]
    #[inline] pub fn with_ep0r<F: FnOnce(Ep0r) -> Ep0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep0r_mut(), f(self.ep0r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EP1R register."]
    #[inline] pub fn ep1r_mut(&self) -> *mut Ep1r { 
        (self.0 + 0x4) as *mut Ep1r
    }

    #[doc="Get the *const pointer for the EP1R register."]
    #[inline] pub fn ep1r_ptr(&self) -> *const Ep1r { 
           self.ep1r_mut()
    }

    #[doc="Read the EP1R register."]
    #[inline] pub fn ep1r(&self) -> Ep1r { 
        unsafe {
            read_volatile(self.ep1r_ptr())
        }
    }

    #[doc="Write the EP1R register."]
    #[inline] pub fn set_ep1r<F: FnOnce(Ep1r) -> Ep1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep1r_mut(), f(Ep1r(0)));
        }
        self
    }

    #[doc="Modify the EP1R register."]
    #[inline] pub fn with_ep1r<F: FnOnce(Ep1r) -> Ep1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep1r_mut(), f(self.ep1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EP2R register."]
    #[inline] pub fn ep2r_mut(&self) -> *mut Ep2r { 
        (self.0 + 0x8) as *mut Ep2r
    }

    #[doc="Get the *const pointer for the EP2R register."]
    #[inline] pub fn ep2r_ptr(&self) -> *const Ep2r { 
           self.ep2r_mut()
    }

    #[doc="Read the EP2R register."]
    #[inline] pub fn ep2r(&self) -> Ep2r { 
        unsafe {
            read_volatile(self.ep2r_ptr())
        }
    }

    #[doc="Write the EP2R register."]
    #[inline] pub fn set_ep2r<F: FnOnce(Ep2r) -> Ep2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep2r_mut(), f(Ep2r(0)));
        }
        self
    }

    #[doc="Modify the EP2R register."]
    #[inline] pub fn with_ep2r<F: FnOnce(Ep2r) -> Ep2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep2r_mut(), f(self.ep2r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EP3R register."]
    #[inline] pub fn ep3r_mut(&self) -> *mut Ep3r { 
        (self.0 + 0xc) as *mut Ep3r
    }

    #[doc="Get the *const pointer for the EP3R register."]
    #[inline] pub fn ep3r_ptr(&self) -> *const Ep3r { 
           self.ep3r_mut()
    }

    #[doc="Read the EP3R register."]
    #[inline] pub fn ep3r(&self) -> Ep3r { 
        unsafe {
            read_volatile(self.ep3r_ptr())
        }
    }

    #[doc="Write the EP3R register."]
    #[inline] pub fn set_ep3r<F: FnOnce(Ep3r) -> Ep3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep3r_mut(), f(Ep3r(0)));
        }
        self
    }

    #[doc="Modify the EP3R register."]
    #[inline] pub fn with_ep3r<F: FnOnce(Ep3r) -> Ep3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep3r_mut(), f(self.ep3r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EP4R register."]
    #[inline] pub fn ep4r_mut(&self) -> *mut Ep4r { 
        (self.0 + 0x10) as *mut Ep4r
    }

    #[doc="Get the *const pointer for the EP4R register."]
    #[inline] pub fn ep4r_ptr(&self) -> *const Ep4r { 
           self.ep4r_mut()
    }

    #[doc="Read the EP4R register."]
    #[inline] pub fn ep4r(&self) -> Ep4r { 
        unsafe {
            read_volatile(self.ep4r_ptr())
        }
    }

    #[doc="Write the EP4R register."]
    #[inline] pub fn set_ep4r<F: FnOnce(Ep4r) -> Ep4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep4r_mut(), f(Ep4r(0)));
        }
        self
    }

    #[doc="Modify the EP4R register."]
    #[inline] pub fn with_ep4r<F: FnOnce(Ep4r) -> Ep4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep4r_mut(), f(self.ep4r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EP5R register."]
    #[inline] pub fn ep5r_mut(&self) -> *mut Ep5r { 
        (self.0 + 0x14) as *mut Ep5r
    }

    #[doc="Get the *const pointer for the EP5R register."]
    #[inline] pub fn ep5r_ptr(&self) -> *const Ep5r { 
           self.ep5r_mut()
    }

    #[doc="Read the EP5R register."]
    #[inline] pub fn ep5r(&self) -> Ep5r { 
        unsafe {
            read_volatile(self.ep5r_ptr())
        }
    }

    #[doc="Write the EP5R register."]
    #[inline] pub fn set_ep5r<F: FnOnce(Ep5r) -> Ep5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep5r_mut(), f(Ep5r(0)));
        }
        self
    }

    #[doc="Modify the EP5R register."]
    #[inline] pub fn with_ep5r<F: FnOnce(Ep5r) -> Ep5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep5r_mut(), f(self.ep5r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EP6R register."]
    #[inline] pub fn ep6r_mut(&self) -> *mut Ep6r { 
        (self.0 + 0x18) as *mut Ep6r
    }

    #[doc="Get the *const pointer for the EP6R register."]
    #[inline] pub fn ep6r_ptr(&self) -> *const Ep6r { 
           self.ep6r_mut()
    }

    #[doc="Read the EP6R register."]
    #[inline] pub fn ep6r(&self) -> Ep6r { 
        unsafe {
            read_volatile(self.ep6r_ptr())
        }
    }

    #[doc="Write the EP6R register."]
    #[inline] pub fn set_ep6r<F: FnOnce(Ep6r) -> Ep6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep6r_mut(), f(Ep6r(0)));
        }
        self
    }

    #[doc="Modify the EP6R register."]
    #[inline] pub fn with_ep6r<F: FnOnce(Ep6r) -> Ep6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep6r_mut(), f(self.ep6r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EP7R register."]
    #[inline] pub fn ep7r_mut(&self) -> *mut Ep7r { 
        (self.0 + 0x1c) as *mut Ep7r
    }

    #[doc="Get the *const pointer for the EP7R register."]
    #[inline] pub fn ep7r_ptr(&self) -> *const Ep7r { 
           self.ep7r_mut()
    }

    #[doc="Read the EP7R register."]
    #[inline] pub fn ep7r(&self) -> Ep7r { 
        unsafe {
            read_volatile(self.ep7r_ptr())
        }
    }

    #[doc="Write the EP7R register."]
    #[inline] pub fn set_ep7r<F: FnOnce(Ep7r) -> Ep7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep7r_mut(), f(Ep7r(0)));
        }
        self
    }

    #[doc="Modify the EP7R register."]
    #[inline] pub fn with_ep7r<F: FnOnce(Ep7r) -> Ep7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ep7r_mut(), f(self.ep7r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CNTR register."]
    #[inline] pub fn cntr_mut(&self) -> *mut Cntr { 
        (self.0 + 0x40) as *mut Cntr
    }

    #[doc="Get the *const pointer for the CNTR register."]
    #[inline] pub fn cntr_ptr(&self) -> *const Cntr { 
           self.cntr_mut()
    }

    #[doc="Read the CNTR register."]
    #[inline] pub fn cntr(&self) -> Cntr { 
        unsafe {
            read_volatile(self.cntr_ptr())
        }
    }

    #[doc="Write the CNTR register."]
    #[inline] pub fn set_cntr<F: FnOnce(Cntr) -> Cntr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cntr_mut(), f(Cntr(0)));
        }
        self
    }

    #[doc="Modify the CNTR register."]
    #[inline] pub fn with_cntr<F: FnOnce(Cntr) -> Cntr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cntr_mut(), f(self.cntr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISTR register."]
    #[inline] pub fn istr_mut(&self) -> *mut Istr { 
        (self.0 + 0x44) as *mut Istr
    }

    #[doc="Get the *const pointer for the ISTR register."]
    #[inline] pub fn istr_ptr(&self) -> *const Istr { 
           self.istr_mut()
    }

    #[doc="Read the ISTR register."]
    #[inline] pub fn istr(&self) -> Istr { 
        unsafe {
            read_volatile(self.istr_ptr())
        }
    }

    #[doc="Write the ISTR register."]
    #[inline] pub fn set_istr<F: FnOnce(Istr) -> Istr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.istr_mut(), f(Istr(0)));
        }
        self
    }

    #[doc="Modify the ISTR register."]
    #[inline] pub fn with_istr<F: FnOnce(Istr) -> Istr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.istr_mut(), f(self.istr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FNR register."]
    #[inline] pub fn fnr_mut(&self) -> *mut Fnr { 
        (self.0 + 0x48) as *mut Fnr
    }

    #[doc="Get the *const pointer for the FNR register."]
    #[inline] pub fn fnr_ptr(&self) -> *const Fnr { 
           self.fnr_mut()
    }

    #[doc="Read the FNR register."]
    #[inline] pub fn fnr(&self) -> Fnr { 
        unsafe {
            read_volatile(self.fnr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DADDR register."]
    #[inline] pub fn daddr_mut(&self) -> *mut Daddr { 
        (self.0 + 0x4c) as *mut Daddr
    }

    #[doc="Get the *const pointer for the DADDR register."]
    #[inline] pub fn daddr_ptr(&self) -> *const Daddr { 
           self.daddr_mut()
    }

    #[doc="Read the DADDR register."]
    #[inline] pub fn daddr(&self) -> Daddr { 
        unsafe {
            read_volatile(self.daddr_ptr())
        }
    }

    #[doc="Write the DADDR register."]
    #[inline] pub fn set_daddr<F: FnOnce(Daddr) -> Daddr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.daddr_mut(), f(Daddr(0)));
        }
        self
    }

    #[doc="Modify the DADDR register."]
    #[inline] pub fn with_daddr<F: FnOnce(Daddr) -> Daddr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.daddr_mut(), f(self.daddr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BTABLE register."]
    #[inline] pub fn btable_mut(&self) -> *mut Btable { 
        (self.0 + 0x50) as *mut Btable
    }

    #[doc="Get the *const pointer for the BTABLE register."]
    #[inline] pub fn btable_ptr(&self) -> *const Btable { 
           self.btable_mut()
    }

    #[doc="Read the BTABLE register."]
    #[inline] pub fn btable(&self) -> Btable { 
        unsafe {
            read_volatile(self.btable_ptr())
        }
    }

    #[doc="Write the BTABLE register."]
    #[inline] pub fn set_btable<F: FnOnce(Btable) -> Btable>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.btable_mut(), f(Btable(0)));
        }
        self
    }

    #[doc="Modify the BTABLE register."]
    #[inline] pub fn with_btable<F: FnOnce(Btable) -> Btable>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.btable_mut(), f(self.btable()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LPMCSR register."]
    #[inline] pub fn lpmcsr_mut(&self) -> *mut Lpmcsr { 
        (self.0 + 0x54) as *mut Lpmcsr
    }

    #[doc="Get the *const pointer for the LPMCSR register."]
    #[inline] pub fn lpmcsr_ptr(&self) -> *const Lpmcsr { 
           self.lpmcsr_mut()
    }

    #[doc="Read the LPMCSR register."]
    #[inline] pub fn lpmcsr(&self) -> Lpmcsr { 
        unsafe {
            read_volatile(self.lpmcsr_ptr())
        }
    }

    #[doc="Write the LPMCSR register."]
    #[inline] pub fn set_lpmcsr<F: FnOnce(Lpmcsr) -> Lpmcsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpmcsr_mut(), f(Lpmcsr(0)));
        }
        self
    }

    #[doc="Modify the LPMCSR register."]
    #[inline] pub fn with_lpmcsr<F: FnOnce(Lpmcsr) -> Lpmcsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpmcsr_mut(), f(self.lpmcsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BCDR register."]
    #[inline] pub fn bcdr_mut(&self) -> *mut Bcdr { 
        (self.0 + 0x58) as *mut Bcdr
    }

    #[doc="Get the *const pointer for the BCDR register."]
    #[inline] pub fn bcdr_ptr(&self) -> *const Bcdr { 
           self.bcdr_mut()
    }

    #[doc="Read the BCDR register."]
    #[inline] pub fn bcdr(&self) -> Bcdr { 
        unsafe {
            read_volatile(self.bcdr_ptr())
        }
    }

    #[doc="Write the BCDR register."]
    #[inline] pub fn set_bcdr<F: FnOnce(Bcdr) -> Bcdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bcdr_mut(), f(Bcdr(0)));
        }
        self
    }

    #[doc="Modify the BCDR register."]
    #[inline] pub fn with_bcdr<F: FnOnce(Bcdr) -> Bcdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bcdr_mut(), f(self.bcdr()));
        }
        self
    }

}

#[doc="endpoint 0 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep0r(pub u32);
impl Ep0r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Ep0r {
    #[inline]
    fn from(other: u32) -> Self {
         Ep0r(other)
    }
}

impl ::core::fmt::Display for Ep0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 1 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep1r(pub u32);
impl Ep1r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Ep1r {
    #[inline]
    fn from(other: u32) -> Self {
         Ep1r(other)
    }
}

impl ::core::fmt::Display for Ep1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 2 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep2r(pub u32);
impl Ep2r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Ep2r {
    #[inline]
    fn from(other: u32) -> Self {
         Ep2r(other)
    }
}

impl ::core::fmt::Display for Ep2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 3 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep3r(pub u32);
impl Ep3r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Ep3r {
    #[inline]
    fn from(other: u32) -> Self {
         Ep3r(other)
    }
}

impl ::core::fmt::Display for Ep3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 4 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep4r(pub u32);
impl Ep4r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Ep4r {
    #[inline]
    fn from(other: u32) -> Self {
         Ep4r(other)
    }
}

impl ::core::fmt::Display for Ep4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 5 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep5r(pub u32);
impl Ep5r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Ep5r {
    #[inline]
    fn from(other: u32) -> Self {
         Ep5r(other)
    }
}

impl ::core::fmt::Display for Ep5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 6 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep6r(pub u32);
impl Ep6r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Ep6r {
    #[inline]
    fn from(other: u32) -> Self {
         Ep6r(other)
    }
}

impl ::core::fmt::Display for Ep6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 7 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep7r(pub u32);
impl Ep7r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Ep7r {
    #[inline]
    fn from(other: u32) -> Self {
         Ep7r(other)
    }
}

impl ::core::fmt::Display for Ep7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cntr(pub u32);
impl Cntr {
    #[doc="Force USB Reset"]
    #[inline] pub fn fres(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FRES != 0"]
    #[inline] pub fn test_fres(&self) -> bool {
        self.fres() != 0
    }

    #[doc="Sets the FRES field."]
    #[inline] pub fn set_fres<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Power down"]
    #[inline] pub fn pdwn(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PDWN != 0"]
    #[inline] pub fn test_pdwn(&self) -> bool {
        self.pdwn() != 0
    }

    #[doc="Sets the PDWN field."]
    #[inline] pub fn set_pdwn<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Low-power mode"]
    #[inline] pub fn lpmode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LPMODE != 0"]
    #[inline] pub fn test_lpmode(&self) -> bool {
        self.lpmode() != 0
    }

    #[doc="Sets the LPMODE field."]
    #[inline] pub fn set_lpmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Force suspend"]
    #[inline] pub fn fsusp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FSUSP != 0"]
    #[inline] pub fn test_fsusp(&self) -> bool {
        self.fsusp() != 0
    }

    #[doc="Sets the FSUSP field."]
    #[inline] pub fn set_fsusp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Resume request"]
    #[inline] pub fn resume(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RESUME != 0"]
    #[inline] pub fn test_resume(&self) -> bool {
        self.resume() != 0
    }

    #[doc="Sets the RESUME field."]
    #[inline] pub fn set_resume<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="LPM L1 Resume request"]
    #[inline] pub fn l1resume(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if L1RESUME != 0"]
    #[inline] pub fn test_l1resume(&self) -> bool {
        self.l1resume() != 0
    }

    #[doc="Sets the L1RESUME field."]
    #[inline] pub fn set_l1resume<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="LPM L1 state request interrupt mask"]
    #[inline] pub fn l1reqm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if L1REQM != 0"]
    #[inline] pub fn test_l1reqm(&self) -> bool {
        self.l1reqm() != 0
    }

    #[doc="Sets the L1REQM field."]
    #[inline] pub fn set_l1reqm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Expected start of frame interrupt mask"]
    #[inline] pub fn esofm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ESOFM != 0"]
    #[inline] pub fn test_esofm(&self) -> bool {
        self.esofm() != 0
    }

    #[doc="Sets the ESOFM field."]
    #[inline] pub fn set_esofm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Start of frame interrupt mask"]
    #[inline] pub fn sofm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SOFM != 0"]
    #[inline] pub fn test_sofm(&self) -> bool {
        self.sofm() != 0
    }

    #[doc="Sets the SOFM field."]
    #[inline] pub fn set_sofm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="USB reset interrupt mask"]
    #[inline] pub fn resetm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RESETM != 0"]
    #[inline] pub fn test_resetm(&self) -> bool {
        self.resetm() != 0
    }

    #[doc="Sets the RESETM field."]
    #[inline] pub fn set_resetm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Suspend mode interrupt mask"]
    #[inline] pub fn suspm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SUSPM != 0"]
    #[inline] pub fn test_suspm(&self) -> bool {
        self.suspm() != 0
    }

    #[doc="Sets the SUSPM field."]
    #[inline] pub fn set_suspm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Wakeup interrupt mask"]
    #[inline] pub fn wkupm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WKUPM != 0"]
    #[inline] pub fn test_wkupm(&self) -> bool {
        self.wkupm() != 0
    }

    #[doc="Sets the WKUPM field."]
    #[inline] pub fn set_wkupm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Error interrupt mask"]
    #[inline] pub fn errm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ERRM != 0"]
    #[inline] pub fn test_errm(&self) -> bool {
        self.errm() != 0
    }

    #[doc="Sets the ERRM field."]
    #[inline] pub fn set_errm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Packet memory area over / underrun interrupt mask"]
    #[inline] pub fn pmaovrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PMAOVRM != 0"]
    #[inline] pub fn test_pmaovrm(&self) -> bool {
        self.pmaovrm() != 0
    }

    #[doc="Sets the PMAOVRM field."]
    #[inline] pub fn set_pmaovrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer interrupt mask"]
    #[inline] pub fn ctrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTRM != 0"]
    #[inline] pub fn test_ctrm(&self) -> bool {
        self.ctrm() != 0
    }

    #[doc="Sets the CTRM field."]
    #[inline] pub fn set_ctrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Cntr {
    #[inline]
    fn from(other: u32) -> Self {
         Cntr(other)
    }
}

impl ::core::fmt::Display for Cntr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cntr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fres() != 0 { try!(write!(f, " fres"))}
        if self.pdwn() != 0 { try!(write!(f, " pdwn"))}
        if self.lpmode() != 0 { try!(write!(f, " lpmode"))}
        if self.fsusp() != 0 { try!(write!(f, " fsusp"))}
        if self.resume() != 0 { try!(write!(f, " resume"))}
        if self.l1resume() != 0 { try!(write!(f, " l1resume"))}
        if self.l1reqm() != 0 { try!(write!(f, " l1reqm"))}
        if self.esofm() != 0 { try!(write!(f, " esofm"))}
        if self.sofm() != 0 { try!(write!(f, " sofm"))}
        if self.resetm() != 0 { try!(write!(f, " resetm"))}
        if self.suspm() != 0 { try!(write!(f, " suspm"))}
        if self.wkupm() != 0 { try!(write!(f, " wkupm"))}
        if self.errm() != 0 { try!(write!(f, " errm"))}
        if self.pmaovrm() != 0 { try!(write!(f, " pmaovrm"))}
        if self.ctrm() != 0 { try!(write!(f, " ctrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Istr(pub u32);
impl Istr {
    #[doc="Endpoint Identifier"]
    #[inline] pub fn ep_id(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EP_ID != 0"]
    #[inline] pub fn test_ep_id(&self) -> bool {
        self.ep_id() != 0
    }

    #[doc="Sets the EP_ID field."]
    #[inline] pub fn set_ep_id<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Direction of transaction"]
    #[inline] pub fn dir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="LPM L1 state request"]
    #[inline] pub fn l1req(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if L1REQ != 0"]
    #[inline] pub fn test_l1req(&self) -> bool {
        self.l1req() != 0
    }

    #[doc="Sets the L1REQ field."]
    #[inline] pub fn set_l1req<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Expected start frame"]
    #[inline] pub fn esof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ESOF != 0"]
    #[inline] pub fn test_esof(&self) -> bool {
        self.esof() != 0
    }

    #[doc="Sets the ESOF field."]
    #[inline] pub fn set_esof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="start of frame"]
    #[inline] pub fn sof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SOF != 0"]
    #[inline] pub fn test_sof(&self) -> bool {
        self.sof() != 0
    }

    #[doc="Sets the SOF field."]
    #[inline] pub fn set_sof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="reset request"]
    #[inline] pub fn _reset(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RESET != 0"]
    #[inline] pub fn test_reset(&self) -> bool {
        self._reset() != 0
    }

    #[doc="Sets the RESET field."]
    #[inline] pub fn set_reset<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Suspend mode request"]
    #[inline] pub fn susp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Wakeup"]
    #[inline] pub fn wkup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WKUP != 0"]
    #[inline] pub fn test_wkup(&self) -> bool {
        self.wkup() != 0
    }

    #[doc="Sets the WKUP field."]
    #[inline] pub fn set_wkup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Error"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Packet memory area over / underrun"]
    #[inline] pub fn pmaovr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PMAOVR != 0"]
    #[inline] pub fn test_pmaovr(&self) -> bool {
        self.pmaovr() != 0
    }

    #[doc="Sets the PMAOVR field."]
    #[inline] pub fn set_pmaovr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer"]
    #[inline] pub fn ctr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR != 0"]
    #[inline] pub fn test_ctr(&self) -> bool {
        self.ctr() != 0
    }

    #[doc="Sets the CTR field."]
    #[inline] pub fn set_ctr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Istr {
    #[inline]
    fn from(other: u32) -> Self {
         Istr(other)
    }
}

impl ::core::fmt::Display for Istr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Istr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ep_id() != 0 { try!(write!(f, " ep_id=0x{:x}", self.ep_id()))}
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.l1req() != 0 { try!(write!(f, " l1req"))}
        if self.esof() != 0 { try!(write!(f, " esof"))}
        if self.sof() != 0 { try!(write!(f, " sof"))}
        if self._reset() != 0 { try!(write!(f, " _reset"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        if self.wkup() != 0 { try!(write!(f, " wkup"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.pmaovr() != 0 { try!(write!(f, " pmaovr"))}
        if self.ctr() != 0 { try!(write!(f, " ctr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="frame number register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fnr(pub u32);
impl Fnr {
    #[doc="Frame number"]
    #[inline] pub fn _fn(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if FN != 0"]
    #[inline] pub fn test_fn(&self) -> bool {
        self._fn() != 0
    }

    #[doc="Sets the FN field."]
    #[inline] pub fn set_fn<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Lost SOF"]
    #[inline] pub fn lsof(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="Returns true if LSOF != 0"]
    #[inline] pub fn test_lsof(&self) -> bool {
        self.lsof() != 0
    }

    #[doc="Sets the LSOF field."]
    #[inline] pub fn set_lsof<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Locked"]
    #[inline] pub fn lck(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if LCK != 0"]
    #[inline] pub fn test_lck(&self) -> bool {
        self.lck() != 0
    }

    #[doc="Sets the LCK field."]
    #[inline] pub fn set_lck<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Receive data - line status"]
    #[inline] pub fn rxdm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RXDM != 0"]
    #[inline] pub fn test_rxdm(&self) -> bool {
        self.rxdm() != 0
    }

    #[doc="Sets the RXDM field."]
    #[inline] pub fn set_rxdm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Receive data + line status"]
    #[inline] pub fn rxdp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RXDP != 0"]
    #[inline] pub fn test_rxdp(&self) -> bool {
        self.rxdp() != 0
    }

    #[doc="Sets the RXDP field."]
    #[inline] pub fn set_rxdp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Fnr {
    #[inline]
    fn from(other: u32) -> Self {
         Fnr(other)
    }
}

impl ::core::fmt::Display for Fnr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fnr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self._fn() != 0 { try!(write!(f, " fn=0x{:x}", self._fn()))}
        if self.lsof() != 0 { try!(write!(f, " lsof=0x{:x}", self.lsof()))}
        if self.lck() != 0 { try!(write!(f, " lck"))}
        if self.rxdm() != 0 { try!(write!(f, " rxdm"))}
        if self.rxdp() != 0 { try!(write!(f, " rxdp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Daddr(pub u32);
impl Daddr {
    #[doc="Device address"]
    #[inline] pub fn add(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if ADD != 0"]
    #[inline] pub fn test_add(&self) -> bool {
        self.add() != 0
    }

    #[doc="Sets the ADD field."]
    #[inline] pub fn set_add<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable function"]
    #[inline] pub fn ef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EF != 0"]
    #[inline] pub fn test_ef(&self) -> bool {
        self.ef() != 0
    }

    #[doc="Sets the EF field."]
    #[inline] pub fn set_ef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Daddr {
    #[inline]
    fn from(other: u32) -> Self {
         Daddr(other)
    }
}

impl ::core::fmt::Display for Daddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Daddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.add() != 0 { try!(write!(f, " add=0x{:x}", self.add()))}
        if self.ef() != 0 { try!(write!(f, " ef"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Buffer table address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Btable(pub u32);
impl Btable {
    #[doc="Buffer table"]
    #[inline] pub fn btable(&self) -> bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1fff) as u16) } // [15:3]
    }

    #[doc="Returns true if BTABLE != 0"]
    #[inline] pub fn test_btable(&self) -> bool {
        self.btable() != 0
    }

    #[doc="Sets the BTABLE field."]
    #[inline] pub fn set_btable<V: Into<bits::U13>>(mut self, value: V) -> Self {
        let value: bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Btable {
    #[inline]
    fn from(other: u32) -> Self {
         Btable(other)
    }
}

impl ::core::fmt::Display for Btable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Btable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.btable() != 0 { try!(write!(f, " btable=0x{:x}", self.btable()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPM control and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpmcsr(pub u32);
impl Lpmcsr {
    #[doc="LPM support enable"]
    #[inline] pub fn lpmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPMEN != 0"]
    #[inline] pub fn test_lpmen(&self) -> bool {
        self.lpmen() != 0
    }

    #[doc="Sets the LPMEN field."]
    #[inline] pub fn set_lpmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="LPM Token acknowledge enable"]
    #[inline] pub fn lpmack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LPMACK != 0"]
    #[inline] pub fn test_lpmack(&self) -> bool {
        self.lpmack() != 0
    }

    #[doc="Sets the LPMACK field."]
    #[inline] pub fn set_lpmack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="bRemoteWake value"]
    #[inline] pub fn remwake(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if REMWAKE != 0"]
    #[inline] pub fn test_remwake(&self) -> bool {
        self.remwake() != 0
    }

    #[doc="Sets the REMWAKE field."]
    #[inline] pub fn set_remwake<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="BESL value"]
    #[inline] pub fn besl(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if BESL != 0"]
    #[inline] pub fn test_besl(&self) -> bool {
        self.besl() != 0
    }

    #[doc="Sets the BESL field."]
    #[inline] pub fn set_besl<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Lpmcsr {
    #[inline]
    fn from(other: u32) -> Self {
         Lpmcsr(other)
    }
}

impl ::core::fmt::Display for Lpmcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lpmcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lpmen() != 0 { try!(write!(f, " lpmen"))}
        if self.lpmack() != 0 { try!(write!(f, " lpmack"))}
        if self.remwake() != 0 { try!(write!(f, " remwake"))}
        if self.besl() != 0 { try!(write!(f, " besl=0x{:x}", self.besl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Battery charging detector"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bcdr(pub u32);
impl Bcdr {
    #[doc="Battery charging detector"]
    #[inline] pub fn bcden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BCDEN != 0"]
    #[inline] pub fn test_bcden(&self) -> bool {
        self.bcden() != 0
    }

    #[doc="Sets the BCDEN field."]
    #[inline] pub fn set_bcden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data contact detection"]
    #[inline] pub fn dcden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DCDEN != 0"]
    #[inline] pub fn test_dcden(&self) -> bool {
        self.dcden() != 0
    }

    #[doc="Sets the DCDEN field."]
    #[inline] pub fn set_dcden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Primary detection"]
    #[inline] pub fn pden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PDEN != 0"]
    #[inline] pub fn test_pden(&self) -> bool {
        self.pden() != 0
    }

    #[doc="Sets the PDEN field."]
    #[inline] pub fn set_pden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Secondary detection"]
    #[inline] pub fn sden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SDEN != 0"]
    #[inline] pub fn test_sden(&self) -> bool {
        self.sden() != 0
    }

    #[doc="Sets the SDEN field."]
    #[inline] pub fn set_sden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data contact detection"]
    #[inline] pub fn dcdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DCDET != 0"]
    #[inline] pub fn test_dcdet(&self) -> bool {
        self.dcdet() != 0
    }

    #[doc="Sets the DCDET field."]
    #[inline] pub fn set_dcdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Primary detection"]
    #[inline] pub fn pdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PDET != 0"]
    #[inline] pub fn test_pdet(&self) -> bool {
        self.pdet() != 0
    }

    #[doc="Sets the PDET field."]
    #[inline] pub fn set_pdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Secondary detection"]
    #[inline] pub fn sdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SDET != 0"]
    #[inline] pub fn test_sdet(&self) -> bool {
        self.sdet() != 0
    }

    #[doc="Sets the SDET field."]
    #[inline] pub fn set_sdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DM pull-up detection status"]
    #[inline] pub fn ps2det(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PS2DET != 0"]
    #[inline] pub fn test_ps2det(&self) -> bool {
        self.ps2det() != 0
    }

    #[doc="Sets the PS2DET field."]
    #[inline] pub fn set_ps2det<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DP pull-up control"]
    #[inline] pub fn dppu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DPPU != 0"]
    #[inline] pub fn test_dppu(&self) -> bool {
        self.dppu() != 0
    }

    #[doc="Sets the DPPU field."]
    #[inline] pub fn set_dppu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Bcdr {
    #[inline]
    fn from(other: u32) -> Self {
         Bcdr(other)
    }
}

impl ::core::fmt::Display for Bcdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bcdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bcden() != 0 { try!(write!(f, " bcden"))}
        if self.dcden() != 0 { try!(write!(f, " dcden"))}
        if self.pden() != 0 { try!(write!(f, " pden"))}
        if self.sden() != 0 { try!(write!(f, " sden"))}
        if self.dcdet() != 0 { try!(write!(f, " dcdet"))}
        if self.pdet() != 0 { try!(write!(f, " pdet"))}
        if self.sdet() != 0 { try!(write!(f, " sdet"))}
        if self.ps2det() != 0 { try!(write!(f, " ps2det"))}
        if self.dppu() != 0 { try!(write!(f, " dppu"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

