//! Ethernet: Precision time protocol
#[allow(unused_imports)] use bobbin_common::*;

periph!(ETHERNET_PTP, EthernetPtp, 0x40028700);

#[doc="Ethernet: Precision time protocol"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EthernetPtp(pub usize);
impl EthernetPtp {
    #[doc="Get the *mut pointer for the PTPTSCR register."]
    #[inline] pub fn ptptscr_mut(&self) -> *mut Ptptscr { 
        (self.0 + 0x0) as *mut Ptptscr
    }

    #[doc="Get the *const pointer for the PTPTSCR register."]
    #[inline] pub fn ptptscr_ptr(&self) -> *const Ptptscr { 
           self.ptptscr_mut()
    }

    #[doc="Read the PTPTSCR register."]
    #[inline] pub fn ptptscr(&self) -> Ptptscr { 
        unsafe {
            read_volatile(self.ptptscr_ptr())
        }
    }

    #[doc="Write the PTPTSCR register."]
    #[inline] pub fn set_ptptscr<F: FnOnce(Ptptscr) -> Ptptscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptptscr_mut(), f(Ptptscr(0)));
        }
        self
    }

    #[doc="Modify the PTPTSCR register."]
    #[inline] pub fn with_ptptscr<F: FnOnce(Ptptscr) -> Ptptscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptptscr_mut(), f(self.ptptscr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PTPSSIR register."]
    #[inline] pub fn ptpssir_mut(&self) -> *mut Ptpssir { 
        (self.0 + 0x4) as *mut Ptpssir
    }

    #[doc="Get the *const pointer for the PTPSSIR register."]
    #[inline] pub fn ptpssir_ptr(&self) -> *const Ptpssir { 
           self.ptpssir_mut()
    }

    #[doc="Read the PTPSSIR register."]
    #[inline] pub fn ptpssir(&self) -> Ptpssir { 
        unsafe {
            read_volatile(self.ptpssir_ptr())
        }
    }

    #[doc="Write the PTPSSIR register."]
    #[inline] pub fn set_ptpssir<F: FnOnce(Ptpssir) -> Ptpssir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptpssir_mut(), f(Ptpssir(0)));
        }
        self
    }

    #[doc="Modify the PTPSSIR register."]
    #[inline] pub fn with_ptpssir<F: FnOnce(Ptpssir) -> Ptpssir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptpssir_mut(), f(self.ptpssir()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PTPTSHR register."]
    #[inline] pub fn ptptshr_mut(&self) -> *mut Ptptshr { 
        (self.0 + 0x8) as *mut Ptptshr
    }

    #[doc="Get the *const pointer for the PTPTSHR register."]
    #[inline] pub fn ptptshr_ptr(&self) -> *const Ptptshr { 
           self.ptptshr_mut()
    }

    #[doc="Read the PTPTSHR register."]
    #[inline] pub fn ptptshr(&self) -> Ptptshr { 
        unsafe {
            read_volatile(self.ptptshr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the PTPTSLR register."]
    #[inline] pub fn ptptslr_mut(&self) -> *mut Ptptslr { 
        (self.0 + 0xc) as *mut Ptptslr
    }

    #[doc="Get the *const pointer for the PTPTSLR register."]
    #[inline] pub fn ptptslr_ptr(&self) -> *const Ptptslr { 
           self.ptptslr_mut()
    }

    #[doc="Read the PTPTSLR register."]
    #[inline] pub fn ptptslr(&self) -> Ptptslr { 
        unsafe {
            read_volatile(self.ptptslr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the PTPTSHUR register."]
    #[inline] pub fn ptptshur_mut(&self) -> *mut Ptptshur { 
        (self.0 + 0x10) as *mut Ptptshur
    }

    #[doc="Get the *const pointer for the PTPTSHUR register."]
    #[inline] pub fn ptptshur_ptr(&self) -> *const Ptptshur { 
           self.ptptshur_mut()
    }

    #[doc="Read the PTPTSHUR register."]
    #[inline] pub fn ptptshur(&self) -> Ptptshur { 
        unsafe {
            read_volatile(self.ptptshur_ptr())
        }
    }

    #[doc="Write the PTPTSHUR register."]
    #[inline] pub fn set_ptptshur<F: FnOnce(Ptptshur) -> Ptptshur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptptshur_mut(), f(Ptptshur(0)));
        }
        self
    }

    #[doc="Modify the PTPTSHUR register."]
    #[inline] pub fn with_ptptshur<F: FnOnce(Ptptshur) -> Ptptshur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptptshur_mut(), f(self.ptptshur()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PTPTSLUR register."]
    #[inline] pub fn ptptslur_mut(&self) -> *mut Ptptslur { 
        (self.0 + 0x14) as *mut Ptptslur
    }

    #[doc="Get the *const pointer for the PTPTSLUR register."]
    #[inline] pub fn ptptslur_ptr(&self) -> *const Ptptslur { 
           self.ptptslur_mut()
    }

    #[doc="Read the PTPTSLUR register."]
    #[inline] pub fn ptptslur(&self) -> Ptptslur { 
        unsafe {
            read_volatile(self.ptptslur_ptr())
        }
    }

    #[doc="Write the PTPTSLUR register."]
    #[inline] pub fn set_ptptslur<F: FnOnce(Ptptslur) -> Ptptslur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptptslur_mut(), f(Ptptslur(0)));
        }
        self
    }

    #[doc="Modify the PTPTSLUR register."]
    #[inline] pub fn with_ptptslur<F: FnOnce(Ptptslur) -> Ptptslur>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptptslur_mut(), f(self.ptptslur()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PTPTSAR register."]
    #[inline] pub fn ptptsar_mut(&self) -> *mut Ptptsar { 
        (self.0 + 0x18) as *mut Ptptsar
    }

    #[doc="Get the *const pointer for the PTPTSAR register."]
    #[inline] pub fn ptptsar_ptr(&self) -> *const Ptptsar { 
           self.ptptsar_mut()
    }

    #[doc="Read the PTPTSAR register."]
    #[inline] pub fn ptptsar(&self) -> Ptptsar { 
        unsafe {
            read_volatile(self.ptptsar_ptr())
        }
    }

    #[doc="Write the PTPTSAR register."]
    #[inline] pub fn set_ptptsar<F: FnOnce(Ptptsar) -> Ptptsar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptptsar_mut(), f(Ptptsar(0)));
        }
        self
    }

    #[doc="Modify the PTPTSAR register."]
    #[inline] pub fn with_ptptsar<F: FnOnce(Ptptsar) -> Ptptsar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptptsar_mut(), f(self.ptptsar()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PTPTTHR register."]
    #[inline] pub fn ptptthr_mut(&self) -> *mut Ptptthr { 
        (self.0 + 0x1c) as *mut Ptptthr
    }

    #[doc="Get the *const pointer for the PTPTTHR register."]
    #[inline] pub fn ptptthr_ptr(&self) -> *const Ptptthr { 
           self.ptptthr_mut()
    }

    #[doc="Read the PTPTTHR register."]
    #[inline] pub fn ptptthr(&self) -> Ptptthr { 
        unsafe {
            read_volatile(self.ptptthr_ptr())
        }
    }

    #[doc="Write the PTPTTHR register."]
    #[inline] pub fn set_ptptthr<F: FnOnce(Ptptthr) -> Ptptthr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptptthr_mut(), f(Ptptthr(0)));
        }
        self
    }

    #[doc="Modify the PTPTTHR register."]
    #[inline] pub fn with_ptptthr<F: FnOnce(Ptptthr) -> Ptptthr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptptthr_mut(), f(self.ptptthr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PTPTTLR register."]
    #[inline] pub fn ptpttlr_mut(&self) -> *mut Ptpttlr { 
        (self.0 + 0x20) as *mut Ptpttlr
    }

    #[doc="Get the *const pointer for the PTPTTLR register."]
    #[inline] pub fn ptpttlr_ptr(&self) -> *const Ptpttlr { 
           self.ptpttlr_mut()
    }

    #[doc="Read the PTPTTLR register."]
    #[inline] pub fn ptpttlr(&self) -> Ptpttlr { 
        unsafe {
            read_volatile(self.ptpttlr_ptr())
        }
    }

    #[doc="Write the PTPTTLR register."]
    #[inline] pub fn set_ptpttlr<F: FnOnce(Ptpttlr) -> Ptpttlr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptpttlr_mut(), f(Ptpttlr(0)));
        }
        self
    }

    #[doc="Modify the PTPTTLR register."]
    #[inline] pub fn with_ptpttlr<F: FnOnce(Ptpttlr) -> Ptpttlr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ptpttlr_mut(), f(self.ptpttlr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PTPTSSR register."]
    #[inline] pub fn ptptssr_mut(&self) -> *mut Ptptssr { 
        (self.0 + 0x28) as *mut Ptptssr
    }

    #[doc="Get the *const pointer for the PTPTSSR register."]
    #[inline] pub fn ptptssr_ptr(&self) -> *const Ptptssr { 
           self.ptptssr_mut()
    }

    #[doc="Read the PTPTSSR register."]
    #[inline] pub fn ptptssr(&self) -> Ptptssr { 
        unsafe {
            read_volatile(self.ptptssr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the PTPPPSCR register."]
    #[inline] pub fn ptpppscr_mut(&self) -> *mut Ptpppscr { 
        (self.0 + 0x2c) as *mut Ptpppscr
    }

    #[doc="Get the *const pointer for the PTPPPSCR register."]
    #[inline] pub fn ptpppscr_ptr(&self) -> *const Ptpppscr { 
           self.ptpppscr_mut()
    }

    #[doc="Read the PTPPPSCR register."]
    #[inline] pub fn ptpppscr(&self) -> Ptpppscr { 
        unsafe {
            read_volatile(self.ptpppscr_ptr())
        }
    }

}

#[doc="Ethernet PTP time stamp control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptscr(pub u32);
impl Ptptscr {
    #[doc="no description available"]
    #[inline] pub fn tse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tse(&self) -> bool {
        self.tse() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsfcu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsfcu(&self) -> bool {
        self.tsfcu() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsfcu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsptppsv2e(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsptppsv2e(&self) -> bool {
        self.tsptppsv2e() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsptppsv2e<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssptpoefe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tssptpoefe(&self) -> bool {
        self.tssptpoefe() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tssptpoefe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssipv6fe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tssipv6fe(&self) -> bool {
        self.tssipv6fe() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tssipv6fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssipv4fe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tssipv4fe(&self) -> bool {
        self.tssipv4fe() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tssipv4fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsseme(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsseme(&self) -> bool {
        self.tsseme() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsseme<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssmrme(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tssmrme(&self) -> bool {
        self.tssmrme() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tssmrme<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tscnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tscnt(&self) -> bool {
        self.tscnt() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tscnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tspffmae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tspffmae(&self) -> bool {
        self.tspffmae() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tspffmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssti(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tssti(&self) -> bool {
        self.tssti() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tssti<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsstu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsstu(&self) -> bool {
        self.tsstu() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsstu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsite(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsite(&self) -> bool {
        self.tsite() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsite<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ttsaru(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="no description available"]
    #[inline] pub fn test_ttsaru(&self) -> bool {
        self.ttsaru() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_ttsaru<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssarfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tssarfe(&self) -> bool {
        self.tssarfe() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tssarfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsssr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsssr(&self) -> bool {
        self.tsssr() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsssr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Ptptscr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptscr(other)
    }
}

impl ::core::fmt::Display for Ptptscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tse() != 0 { try!(write!(f, " tse"))}
        if self.tsfcu() != 0 { try!(write!(f, " tsfcu"))}
        if self.tsptppsv2e() != 0 { try!(write!(f, " tsptppsv2e"))}
        if self.tssptpoefe() != 0 { try!(write!(f, " tssptpoefe"))}
        if self.tssipv6fe() != 0 { try!(write!(f, " tssipv6fe"))}
        if self.tssipv4fe() != 0 { try!(write!(f, " tssipv4fe"))}
        if self.tsseme() != 0 { try!(write!(f, " tsseme"))}
        if self.tssmrme() != 0 { try!(write!(f, " tssmrme"))}
        if self.tscnt() != 0 { try!(write!(f, " tscnt=0x{:x}", self.tscnt()))}
        if self.tspffmae() != 0 { try!(write!(f, " tspffmae"))}
        if self.tssti() != 0 { try!(write!(f, " tssti"))}
        if self.tsstu() != 0 { try!(write!(f, " tsstu"))}
        if self.tsite() != 0 { try!(write!(f, " tsite"))}
        if self.ttsaru() != 0 { try!(write!(f, " ttsaru"))}
        if self.tssarfe() != 0 { try!(write!(f, " tssarfe"))}
        if self.tsssr() != 0 { try!(write!(f, " tsssr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP subsecond increment register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptpssir(pub u32);
impl Ptpssir {
    #[doc="no description available"]
    #[inline] pub fn stssi(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="no description available"]
    #[inline] pub fn test_stssi(&self) -> bool {
        self.stssi() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_stssi<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptpssir {
    #[inline]
    fn from(other: u32) -> Self {
         Ptpssir(other)
    }
}

impl ::core::fmt::Display for Ptpssir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptpssir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stssi() != 0 { try!(write!(f, " stssi=0x{:x}", self.stssi()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptshr(pub u32);
impl Ptptshr {
    #[doc="no description available"]
    #[inline] pub fn sts(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="no description available"]
    #[inline] pub fn test_sts(&self) -> bool {
        self.sts() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_sts<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptptshr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptshr(other)
    }
}

impl ::core::fmt::Display for Ptptshr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptshr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptslr(pub u32);
impl Ptptslr {
    #[doc="no description available"]
    #[inline] pub fn stss(&self) -> bits::U31 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fffffff) as u32) } // [30:0]
    }

    #[doc="no description available"]
    #[inline] pub fn test_stss(&self) -> bool {
        self.stss() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_stss<V: Into<bits::U31>>(mut self, value: V) -> Self {
        let value: bits::U31 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fffffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn stpns(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="no description available"]
    #[inline] pub fn test_stpns(&self) -> bool {
        self.stpns() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_stpns<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ptptslr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptslr(other)
    }
}

impl ::core::fmt::Display for Ptptslr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptslr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stss() != 0 { try!(write!(f, " stss=0x{:x}", self.stss()))}
        if self.stpns() != 0 { try!(write!(f, " stpns"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp high update register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptshur(pub u32);
impl Ptptshur {
    #[doc="no description available"]
    #[inline] pub fn tsus(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsus(&self) -> bool {
        self.tsus() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsus<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptptshur {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptshur(other)
    }
}

impl ::core::fmt::Display for Ptptshur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptshur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp low update register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptslur(pub u32);
impl Ptptslur {
    #[doc="no description available"]
    #[inline] pub fn tsuss(&self) -> bits::U31 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fffffff) as u32) } // [30:0]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsuss(&self) -> bool {
        self.tsuss() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsuss<V: Into<bits::U31>>(mut self, value: V) -> Self {
        let value: bits::U31 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fffffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsupns(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsupns(&self) -> bool {
        self.tsupns() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsupns<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ptptslur {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptslur(other)
    }
}

impl ::core::fmt::Display for Ptptslur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptslur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsuss() != 0 { try!(write!(f, " tsuss=0x{:x}", self.tsuss()))}
        if self.tsupns() != 0 { try!(write!(f, " tsupns"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp addend register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptsar(pub u32);
impl Ptptsar {
    #[doc="no description available"]
    #[inline] pub fn tsa(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsa(&self) -> bool {
        self.tsa() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsa<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptptsar {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptsar(other)
    }
}

impl ::core::fmt::Display for Ptptsar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptsar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP target time high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptthr(pub u32);
impl Ptptthr {
    #[doc="0"]
    #[inline] pub fn ttsh(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="0"]
    #[inline] pub fn test_ttsh(&self) -> bool {
        self.ttsh() != 0
    }

    #[doc="0"]
    #[inline] pub fn set_ttsh<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptptthr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptthr(other)
    }
}

impl ::core::fmt::Display for Ptptthr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptthr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP target time low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptpttlr(pub u32);
impl Ptpttlr {
    #[doc="no description available"]
    #[inline] pub fn ttsl(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="no description available"]
    #[inline] pub fn test_ttsl(&self) -> bool {
        self.ttsl() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_ttsl<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptpttlr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptpttlr(other)
    }
}

impl ::core::fmt::Display for Ptpttlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptpttlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptssr(pub u32);
impl Ptptssr {
    #[doc="no description available"]
    #[inline] pub fn tsso(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsso(&self) -> bool {
        self.tsso() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsso<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsttr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="no description available"]
    #[inline] pub fn test_tsttr(&self) -> bool {
        self.tsttr() != 0
    }

    #[doc="no description available"]
    #[inline] pub fn set_tsttr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Ptptssr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptssr(other)
    }
}

impl ::core::fmt::Display for Ptptssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsso() != 0 { try!(write!(f, " tsso"))}
        if self.tsttr() != 0 { try!(write!(f, " tsttr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP PPS control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptpppscr(pub u32);
impl Ptpppscr {
    #[doc="TSSO"]
    #[inline] pub fn tsso(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="TSSO"]
    #[inline] pub fn test_tsso(&self) -> bool {
        self.tsso() != 0
    }

    #[doc="TSSO"]
    #[inline] pub fn set_tsso<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TSTTR"]
    #[inline] pub fn tsttr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="TSTTR"]
    #[inline] pub fn test_tsttr(&self) -> bool {
        self.tsttr() != 0
    }

    #[doc="TSTTR"]
    #[inline] pub fn set_tsttr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Ptpppscr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptpppscr(other)
    }
}

impl ::core::fmt::Display for Ptpppscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptpppscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsso() != 0 { try!(write!(f, " tsso"))}
        if self.tsttr() != 0 { try!(write!(f, " tsttr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


