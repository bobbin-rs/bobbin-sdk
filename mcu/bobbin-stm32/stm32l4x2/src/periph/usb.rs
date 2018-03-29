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

