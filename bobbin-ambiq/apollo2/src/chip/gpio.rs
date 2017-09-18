#[allow(unused_imports)] use bobbin_common::*;

periph!( GPIO, Gpio, _GPIO, GpioPeriph, 0x40010000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct GpioPeriph(pub usize); 



impl GpioPeriph {
    #[doc="Get the *mut pointer for the PADREGA register."]
    #[inline] pub fn padrega_mut(&self) -> *mut Padrega { 
        (self.0 + 0x0) as *mut Padrega
    }

    #[doc="Get the *const pointer for the PADREGA register."]
    #[inline] pub fn padrega_ptr(&self) -> *const Padrega { 
           self.padrega_mut()
    }

    #[doc="Read the PADREGA register."]
    #[inline] pub fn padrega(&self) -> Padrega { 
        unsafe {
            read_volatile(self.padrega_ptr())
        }
    }

    #[doc="Write the PADREGA register."]
    #[inline] pub fn set_padrega<F: FnOnce(Padrega) -> Padrega>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padrega_mut(), f(Padrega(0)));
        }
        self
    }

    #[doc="Modify the PADREGA register."]
    #[inline] pub fn with_padrega<F: FnOnce(Padrega) -> Padrega>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padrega_mut(), f(self.padrega()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGB register."]
    #[inline] pub fn padregb_mut(&self) -> *mut Padregb { 
        (self.0 + 0x4) as *mut Padregb
    }

    #[doc="Get the *const pointer for the PADREGB register."]
    #[inline] pub fn padregb_ptr(&self) -> *const Padregb { 
           self.padregb_mut()
    }

    #[doc="Read the PADREGB register."]
    #[inline] pub fn padregb(&self) -> Padregb { 
        unsafe {
            read_volatile(self.padregb_ptr())
        }
    }

    #[doc="Write the PADREGB register."]
    #[inline] pub fn set_padregb<F: FnOnce(Padregb) -> Padregb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregb_mut(), f(Padregb(0)));
        }
        self
    }

    #[doc="Modify the PADREGB register."]
    #[inline] pub fn with_padregb<F: FnOnce(Padregb) -> Padregb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregb_mut(), f(self.padregb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGC register."]
    #[inline] pub fn padregc_mut(&self) -> *mut Padregc { 
        (self.0 + 0x8) as *mut Padregc
    }

    #[doc="Get the *const pointer for the PADREGC register."]
    #[inline] pub fn padregc_ptr(&self) -> *const Padregc { 
           self.padregc_mut()
    }

    #[doc="Read the PADREGC register."]
    #[inline] pub fn padregc(&self) -> Padregc { 
        unsafe {
            read_volatile(self.padregc_ptr())
        }
    }

    #[doc="Write the PADREGC register."]
    #[inline] pub fn set_padregc<F: FnOnce(Padregc) -> Padregc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregc_mut(), f(Padregc(0)));
        }
        self
    }

    #[doc="Modify the PADREGC register."]
    #[inline] pub fn with_padregc<F: FnOnce(Padregc) -> Padregc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregc_mut(), f(self.padregc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGD register."]
    #[inline] pub fn padregd_mut(&self) -> *mut Padregd { 
        (self.0 + 0xc) as *mut Padregd
    }

    #[doc="Get the *const pointer for the PADREGD register."]
    #[inline] pub fn padregd_ptr(&self) -> *const Padregd { 
           self.padregd_mut()
    }

    #[doc="Read the PADREGD register."]
    #[inline] pub fn padregd(&self) -> Padregd { 
        unsafe {
            read_volatile(self.padregd_ptr())
        }
    }

    #[doc="Write the PADREGD register."]
    #[inline] pub fn set_padregd<F: FnOnce(Padregd) -> Padregd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregd_mut(), f(Padregd(0)));
        }
        self
    }

    #[doc="Modify the PADREGD register."]
    #[inline] pub fn with_padregd<F: FnOnce(Padregd) -> Padregd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregd_mut(), f(self.padregd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGE register."]
    #[inline] pub fn padrege_mut(&self) -> *mut Padrege { 
        (self.0 + 0x10) as *mut Padrege
    }

    #[doc="Get the *const pointer for the PADREGE register."]
    #[inline] pub fn padrege_ptr(&self) -> *const Padrege { 
           self.padrege_mut()
    }

    #[doc="Read the PADREGE register."]
    #[inline] pub fn padrege(&self) -> Padrege { 
        unsafe {
            read_volatile(self.padrege_ptr())
        }
    }

    #[doc="Write the PADREGE register."]
    #[inline] pub fn set_padrege<F: FnOnce(Padrege) -> Padrege>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padrege_mut(), f(Padrege(0)));
        }
        self
    }

    #[doc="Modify the PADREGE register."]
    #[inline] pub fn with_padrege<F: FnOnce(Padrege) -> Padrege>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padrege_mut(), f(self.padrege()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGF register."]
    #[inline] pub fn padregf_mut(&self) -> *mut Padregf { 
        (self.0 + 0x14) as *mut Padregf
    }

    #[doc="Get the *const pointer for the PADREGF register."]
    #[inline] pub fn padregf_ptr(&self) -> *const Padregf { 
           self.padregf_mut()
    }

    #[doc="Read the PADREGF register."]
    #[inline] pub fn padregf(&self) -> Padregf { 
        unsafe {
            read_volatile(self.padregf_ptr())
        }
    }

    #[doc="Write the PADREGF register."]
    #[inline] pub fn set_padregf<F: FnOnce(Padregf) -> Padregf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregf_mut(), f(Padregf(0)));
        }
        self
    }

    #[doc="Modify the PADREGF register."]
    #[inline] pub fn with_padregf<F: FnOnce(Padregf) -> Padregf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregf_mut(), f(self.padregf()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGG register."]
    #[inline] pub fn padregg_mut(&self) -> *mut Padregg { 
        (self.0 + 0x18) as *mut Padregg
    }

    #[doc="Get the *const pointer for the PADREGG register."]
    #[inline] pub fn padregg_ptr(&self) -> *const Padregg { 
           self.padregg_mut()
    }

    #[doc="Read the PADREGG register."]
    #[inline] pub fn padregg(&self) -> Padregg { 
        unsafe {
            read_volatile(self.padregg_ptr())
        }
    }

    #[doc="Write the PADREGG register."]
    #[inline] pub fn set_padregg<F: FnOnce(Padregg) -> Padregg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregg_mut(), f(Padregg(0)));
        }
        self
    }

    #[doc="Modify the PADREGG register."]
    #[inline] pub fn with_padregg<F: FnOnce(Padregg) -> Padregg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregg_mut(), f(self.padregg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGH register."]
    #[inline] pub fn padregh_mut(&self) -> *mut Padregh { 
        (self.0 + 0x1c) as *mut Padregh
    }

    #[doc="Get the *const pointer for the PADREGH register."]
    #[inline] pub fn padregh_ptr(&self) -> *const Padregh { 
           self.padregh_mut()
    }

    #[doc="Read the PADREGH register."]
    #[inline] pub fn padregh(&self) -> Padregh { 
        unsafe {
            read_volatile(self.padregh_ptr())
        }
    }

    #[doc="Write the PADREGH register."]
    #[inline] pub fn set_padregh<F: FnOnce(Padregh) -> Padregh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregh_mut(), f(Padregh(0)));
        }
        self
    }

    #[doc="Modify the PADREGH register."]
    #[inline] pub fn with_padregh<F: FnOnce(Padregh) -> Padregh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregh_mut(), f(self.padregh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGI register."]
    #[inline] pub fn padregi_mut(&self) -> *mut Padregi { 
        (self.0 + 0x20) as *mut Padregi
    }

    #[doc="Get the *const pointer for the PADREGI register."]
    #[inline] pub fn padregi_ptr(&self) -> *const Padregi { 
           self.padregi_mut()
    }

    #[doc="Read the PADREGI register."]
    #[inline] pub fn padregi(&self) -> Padregi { 
        unsafe {
            read_volatile(self.padregi_ptr())
        }
    }

    #[doc="Write the PADREGI register."]
    #[inline] pub fn set_padregi<F: FnOnce(Padregi) -> Padregi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregi_mut(), f(Padregi(0)));
        }
        self
    }

    #[doc="Modify the PADREGI register."]
    #[inline] pub fn with_padregi<F: FnOnce(Padregi) -> Padregi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregi_mut(), f(self.padregi()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGJ register."]
    #[inline] pub fn padregj_mut(&self) -> *mut Padregj { 
        (self.0 + 0x24) as *mut Padregj
    }

    #[doc="Get the *const pointer for the PADREGJ register."]
    #[inline] pub fn padregj_ptr(&self) -> *const Padregj { 
           self.padregj_mut()
    }

    #[doc="Read the PADREGJ register."]
    #[inline] pub fn padregj(&self) -> Padregj { 
        unsafe {
            read_volatile(self.padregj_ptr())
        }
    }

    #[doc="Write the PADREGJ register."]
    #[inline] pub fn set_padregj<F: FnOnce(Padregj) -> Padregj>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregj_mut(), f(Padregj(0)));
        }
        self
    }

    #[doc="Modify the PADREGJ register."]
    #[inline] pub fn with_padregj<F: FnOnce(Padregj) -> Padregj>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregj_mut(), f(self.padregj()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGK register."]
    #[inline] pub fn padregk_mut(&self) -> *mut Padregk { 
        (self.0 + 0x28) as *mut Padregk
    }

    #[doc="Get the *const pointer for the PADREGK register."]
    #[inline] pub fn padregk_ptr(&self) -> *const Padregk { 
           self.padregk_mut()
    }

    #[doc="Read the PADREGK register."]
    #[inline] pub fn padregk(&self) -> Padregk { 
        unsafe {
            read_volatile(self.padregk_ptr())
        }
    }

    #[doc="Write the PADREGK register."]
    #[inline] pub fn set_padregk<F: FnOnce(Padregk) -> Padregk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregk_mut(), f(Padregk(0)));
        }
        self
    }

    #[doc="Modify the PADREGK register."]
    #[inline] pub fn with_padregk<F: FnOnce(Padregk) -> Padregk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregk_mut(), f(self.padregk()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGL register."]
    #[inline] pub fn padregl_mut(&self) -> *mut Padregl { 
        (self.0 + 0x2c) as *mut Padregl
    }

    #[doc="Get the *const pointer for the PADREGL register."]
    #[inline] pub fn padregl_ptr(&self) -> *const Padregl { 
           self.padregl_mut()
    }

    #[doc="Read the PADREGL register."]
    #[inline] pub fn padregl(&self) -> Padregl { 
        unsafe {
            read_volatile(self.padregl_ptr())
        }
    }

    #[doc="Write the PADREGL register."]
    #[inline] pub fn set_padregl<F: FnOnce(Padregl) -> Padregl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregl_mut(), f(Padregl(0)));
        }
        self
    }

    #[doc="Modify the PADREGL register."]
    #[inline] pub fn with_padregl<F: FnOnce(Padregl) -> Padregl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregl_mut(), f(self.padregl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADREGM register."]
    #[inline] pub fn padregm_mut(&self) -> *mut Padregm { 
        (self.0 + 0x30) as *mut Padregm
    }

    #[doc="Get the *const pointer for the PADREGM register."]
    #[inline] pub fn padregm_ptr(&self) -> *const Padregm { 
           self.padregm_mut()
    }

    #[doc="Read the PADREGM register."]
    #[inline] pub fn padregm(&self) -> Padregm { 
        unsafe {
            read_volatile(self.padregm_ptr())
        }
    }

    #[doc="Write the PADREGM register."]
    #[inline] pub fn set_padregm<F: FnOnce(Padregm) -> Padregm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregm_mut(), f(Padregm(0)));
        }
        self
    }

    #[doc="Modify the PADREGM register."]
    #[inline] pub fn with_padregm<F: FnOnce(Padregm) -> Padregm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padregm_mut(), f(self.padregm()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGA register."]
    #[inline] pub fn cfga_mut(&self) -> *mut Cfga { 
        (self.0 + 0x40) as *mut Cfga
    }

    #[doc="Get the *const pointer for the CFGA register."]
    #[inline] pub fn cfga_ptr(&self) -> *const Cfga { 
           self.cfga_mut()
    }

    #[doc="Read the CFGA register."]
    #[inline] pub fn cfga(&self) -> Cfga { 
        unsafe {
            read_volatile(self.cfga_ptr())
        }
    }

    #[doc="Write the CFGA register."]
    #[inline] pub fn set_cfga<F: FnOnce(Cfga) -> Cfga>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfga_mut(), f(Cfga(0)));
        }
        self
    }

    #[doc="Modify the CFGA register."]
    #[inline] pub fn with_cfga<F: FnOnce(Cfga) -> Cfga>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfga_mut(), f(self.cfga()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGB register."]
    #[inline] pub fn cfgb_mut(&self) -> *mut Cfgb { 
        (self.0 + 0x44) as *mut Cfgb
    }

    #[doc="Get the *const pointer for the CFGB register."]
    #[inline] pub fn cfgb_ptr(&self) -> *const Cfgb { 
           self.cfgb_mut()
    }

    #[doc="Read the CFGB register."]
    #[inline] pub fn cfgb(&self) -> Cfgb { 
        unsafe {
            read_volatile(self.cfgb_ptr())
        }
    }

    #[doc="Write the CFGB register."]
    #[inline] pub fn set_cfgb<F: FnOnce(Cfgb) -> Cfgb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgb_mut(), f(Cfgb(0)));
        }
        self
    }

    #[doc="Modify the CFGB register."]
    #[inline] pub fn with_cfgb<F: FnOnce(Cfgb) -> Cfgb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgb_mut(), f(self.cfgb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGC register."]
    #[inline] pub fn cfgc_mut(&self) -> *mut Cfgc { 
        (self.0 + 0x48) as *mut Cfgc
    }

    #[doc="Get the *const pointer for the CFGC register."]
    #[inline] pub fn cfgc_ptr(&self) -> *const Cfgc { 
           self.cfgc_mut()
    }

    #[doc="Read the CFGC register."]
    #[inline] pub fn cfgc(&self) -> Cfgc { 
        unsafe {
            read_volatile(self.cfgc_ptr())
        }
    }

    #[doc="Write the CFGC register."]
    #[inline] pub fn set_cfgc<F: FnOnce(Cfgc) -> Cfgc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgc_mut(), f(Cfgc(0)));
        }
        self
    }

    #[doc="Modify the CFGC register."]
    #[inline] pub fn with_cfgc<F: FnOnce(Cfgc) -> Cfgc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgc_mut(), f(self.cfgc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGD register."]
    #[inline] pub fn cfgd_mut(&self) -> *mut Cfgd { 
        (self.0 + 0x4c) as *mut Cfgd
    }

    #[doc="Get the *const pointer for the CFGD register."]
    #[inline] pub fn cfgd_ptr(&self) -> *const Cfgd { 
           self.cfgd_mut()
    }

    #[doc="Read the CFGD register."]
    #[inline] pub fn cfgd(&self) -> Cfgd { 
        unsafe {
            read_volatile(self.cfgd_ptr())
        }
    }

    #[doc="Write the CFGD register."]
    #[inline] pub fn set_cfgd<F: FnOnce(Cfgd) -> Cfgd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgd_mut(), f(Cfgd(0)));
        }
        self
    }

    #[doc="Modify the CFGD register."]
    #[inline] pub fn with_cfgd<F: FnOnce(Cfgd) -> Cfgd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgd_mut(), f(self.cfgd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGE register."]
    #[inline] pub fn cfge_mut(&self) -> *mut Cfge { 
        (self.0 + 0x50) as *mut Cfge
    }

    #[doc="Get the *const pointer for the CFGE register."]
    #[inline] pub fn cfge_ptr(&self) -> *const Cfge { 
           self.cfge_mut()
    }

    #[doc="Read the CFGE register."]
    #[inline] pub fn cfge(&self) -> Cfge { 
        unsafe {
            read_volatile(self.cfge_ptr())
        }
    }

    #[doc="Write the CFGE register."]
    #[inline] pub fn set_cfge<F: FnOnce(Cfge) -> Cfge>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfge_mut(), f(Cfge(0)));
        }
        self
    }

    #[doc="Modify the CFGE register."]
    #[inline] pub fn with_cfge<F: FnOnce(Cfge) -> Cfge>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfge_mut(), f(self.cfge()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGF register."]
    #[inline] pub fn cfgf_mut(&self) -> *mut Cfgf { 
        (self.0 + 0x54) as *mut Cfgf
    }

    #[doc="Get the *const pointer for the CFGF register."]
    #[inline] pub fn cfgf_ptr(&self) -> *const Cfgf { 
           self.cfgf_mut()
    }

    #[doc="Read the CFGF register."]
    #[inline] pub fn cfgf(&self) -> Cfgf { 
        unsafe {
            read_volatile(self.cfgf_ptr())
        }
    }

    #[doc="Write the CFGF register."]
    #[inline] pub fn set_cfgf<F: FnOnce(Cfgf) -> Cfgf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgf_mut(), f(Cfgf(0)));
        }
        self
    }

    #[doc="Modify the CFGF register."]
    #[inline] pub fn with_cfgf<F: FnOnce(Cfgf) -> Cfgf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgf_mut(), f(self.cfgf()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGG register."]
    #[inline] pub fn cfgg_mut(&self) -> *mut Cfgg { 
        (self.0 + 0x58) as *mut Cfgg
    }

    #[doc="Get the *const pointer for the CFGG register."]
    #[inline] pub fn cfgg_ptr(&self) -> *const Cfgg { 
           self.cfgg_mut()
    }

    #[doc="Read the CFGG register."]
    #[inline] pub fn cfgg(&self) -> Cfgg { 
        unsafe {
            read_volatile(self.cfgg_ptr())
        }
    }

    #[doc="Write the CFGG register."]
    #[inline] pub fn set_cfgg<F: FnOnce(Cfgg) -> Cfgg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgg_mut(), f(Cfgg(0)));
        }
        self
    }

    #[doc="Modify the CFGG register."]
    #[inline] pub fn with_cfgg<F: FnOnce(Cfgg) -> Cfgg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgg_mut(), f(self.cfgg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADKEY register."]
    #[inline] pub fn padkey_mut(&self) -> *mut Padkey { 
        (self.0 + 0x60) as *mut Padkey
    }

    #[doc="Get the *const pointer for the PADKEY register."]
    #[inline] pub fn padkey_ptr(&self) -> *const Padkey { 
           self.padkey_mut()
    }

    #[doc="Read the PADKEY register."]
    #[inline] pub fn padkey(&self) -> Padkey { 
        unsafe {
            read_volatile(self.padkey_ptr())
        }
    }

    #[doc="Write the PADKEY register."]
    #[inline] pub fn set_padkey<F: FnOnce(Padkey) -> Padkey>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padkey_mut(), f(Padkey(0)));
        }
        self
    }

    #[doc="Modify the PADKEY register."]
    #[inline] pub fn with_padkey<F: FnOnce(Padkey) -> Padkey>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padkey_mut(), f(self.padkey()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RDA register."]
    #[inline] pub fn rda_mut(&self) -> *mut Rda { 
        (self.0 + 0x80) as *mut Rda
    }

    #[doc="Get the *const pointer for the RDA register."]
    #[inline] pub fn rda_ptr(&self) -> *const Rda { 
           self.rda_mut()
    }

    #[doc="Read the RDA register."]
    #[inline] pub fn rda(&self) -> Rda { 
        unsafe {
            read_volatile(self.rda_ptr())
        }
    }

    #[doc="Write the RDA register."]
    #[inline] pub fn set_rda<F: FnOnce(Rda) -> Rda>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rda_mut(), f(Rda(0)));
        }
        self
    }

    #[doc="Modify the RDA register."]
    #[inline] pub fn with_rda<F: FnOnce(Rda) -> Rda>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rda_mut(), f(self.rda()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RDB register."]
    #[inline] pub fn rdb_mut(&self) -> *mut Rdb { 
        (self.0 + 0x84) as *mut Rdb
    }

    #[doc="Get the *const pointer for the RDB register."]
    #[inline] pub fn rdb_ptr(&self) -> *const Rdb { 
           self.rdb_mut()
    }

    #[doc="Read the RDB register."]
    #[inline] pub fn rdb(&self) -> Rdb { 
        unsafe {
            read_volatile(self.rdb_ptr())
        }
    }

    #[doc="Write the RDB register."]
    #[inline] pub fn set_rdb<F: FnOnce(Rdb) -> Rdb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rdb_mut(), f(Rdb(0)));
        }
        self
    }

    #[doc="Modify the RDB register."]
    #[inline] pub fn with_rdb<F: FnOnce(Rdb) -> Rdb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rdb_mut(), f(self.rdb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WTA register."]
    #[inline] pub fn wta_mut(&self) -> *mut Wta { 
        (self.0 + 0x88) as *mut Wta
    }

    #[doc="Get the *const pointer for the WTA register."]
    #[inline] pub fn wta_ptr(&self) -> *const Wta { 
           self.wta_mut()
    }

    #[doc="Read the WTA register."]
    #[inline] pub fn wta(&self) -> Wta { 
        unsafe {
            read_volatile(self.wta_ptr())
        }
    }

    #[doc="Write the WTA register."]
    #[inline] pub fn set_wta<F: FnOnce(Wta) -> Wta>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wta_mut(), f(Wta(0)));
        }
        self
    }

    #[doc="Modify the WTA register."]
    #[inline] pub fn with_wta<F: FnOnce(Wta) -> Wta>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wta_mut(), f(self.wta()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WTB register."]
    #[inline] pub fn wtb_mut(&self) -> *mut Wtb { 
        (self.0 + 0x8c) as *mut Wtb
    }

    #[doc="Get the *const pointer for the WTB register."]
    #[inline] pub fn wtb_ptr(&self) -> *const Wtb { 
           self.wtb_mut()
    }

    #[doc="Read the WTB register."]
    #[inline] pub fn wtb(&self) -> Wtb { 
        unsafe {
            read_volatile(self.wtb_ptr())
        }
    }

    #[doc="Write the WTB register."]
    #[inline] pub fn set_wtb<F: FnOnce(Wtb) -> Wtb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtb_mut(), f(Wtb(0)));
        }
        self
    }

    #[doc="Modify the WTB register."]
    #[inline] pub fn with_wtb<F: FnOnce(Wtb) -> Wtb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtb_mut(), f(self.wtb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WTSA register."]
    #[inline] pub fn wtsa_mut(&self) -> *mut Wtsa { 
        (self.0 + 0x90) as *mut Wtsa
    }

    #[doc="Get the *const pointer for the WTSA register."]
    #[inline] pub fn wtsa_ptr(&self) -> *const Wtsa { 
           self.wtsa_mut()
    }

    #[doc="Read the WTSA register."]
    #[inline] pub fn wtsa(&self) -> Wtsa { 
        unsafe {
            read_volatile(self.wtsa_ptr())
        }
    }

    #[doc="Write the WTSA register."]
    #[inline] pub fn set_wtsa<F: FnOnce(Wtsa) -> Wtsa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtsa_mut(), f(Wtsa(0)));
        }
        self
    }

    #[doc="Modify the WTSA register."]
    #[inline] pub fn with_wtsa<F: FnOnce(Wtsa) -> Wtsa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtsa_mut(), f(self.wtsa()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WTSB register."]
    #[inline] pub fn wtsb_mut(&self) -> *mut Wtsb { 
        (self.0 + 0x94) as *mut Wtsb
    }

    #[doc="Get the *const pointer for the WTSB register."]
    #[inline] pub fn wtsb_ptr(&self) -> *const Wtsb { 
           self.wtsb_mut()
    }

    #[doc="Read the WTSB register."]
    #[inline] pub fn wtsb(&self) -> Wtsb { 
        unsafe {
            read_volatile(self.wtsb_ptr())
        }
    }

    #[doc="Write the WTSB register."]
    #[inline] pub fn set_wtsb<F: FnOnce(Wtsb) -> Wtsb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtsb_mut(), f(Wtsb(0)));
        }
        self
    }

    #[doc="Modify the WTSB register."]
    #[inline] pub fn with_wtsb<F: FnOnce(Wtsb) -> Wtsb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtsb_mut(), f(self.wtsb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WTCA register."]
    #[inline] pub fn wtca_mut(&self) -> *mut Wtca { 
        (self.0 + 0x98) as *mut Wtca
    }

    #[doc="Get the *const pointer for the WTCA register."]
    #[inline] pub fn wtca_ptr(&self) -> *const Wtca { 
           self.wtca_mut()
    }

    #[doc="Read the WTCA register."]
    #[inline] pub fn wtca(&self) -> Wtca { 
        unsafe {
            read_volatile(self.wtca_ptr())
        }
    }

    #[doc="Write the WTCA register."]
    #[inline] pub fn set_wtca<F: FnOnce(Wtca) -> Wtca>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtca_mut(), f(Wtca(0)));
        }
        self
    }

    #[doc="Modify the WTCA register."]
    #[inline] pub fn with_wtca<F: FnOnce(Wtca) -> Wtca>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtca_mut(), f(self.wtca()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WTCB register."]
    #[inline] pub fn wtcb_mut(&self) -> *mut Wtcb { 
        (self.0 + 0x9c) as *mut Wtcb
    }

    #[doc="Get the *const pointer for the WTCB register."]
    #[inline] pub fn wtcb_ptr(&self) -> *const Wtcb { 
           self.wtcb_mut()
    }

    #[doc="Read the WTCB register."]
    #[inline] pub fn wtcb(&self) -> Wtcb { 
        unsafe {
            read_volatile(self.wtcb_ptr())
        }
    }

    #[doc="Write the WTCB register."]
    #[inline] pub fn set_wtcb<F: FnOnce(Wtcb) -> Wtcb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtcb_mut(), f(Wtcb(0)));
        }
        self
    }

    #[doc="Modify the WTCB register."]
    #[inline] pub fn with_wtcb<F: FnOnce(Wtcb) -> Wtcb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtcb_mut(), f(self.wtcb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ENA register."]
    #[inline] pub fn ena_mut(&self) -> *mut Ena { 
        (self.0 + 0xa0) as *mut Ena
    }

    #[doc="Get the *const pointer for the ENA register."]
    #[inline] pub fn ena_ptr(&self) -> *const Ena { 
           self.ena_mut()
    }

    #[doc="Read the ENA register."]
    #[inline] pub fn ena(&self) -> Ena { 
        unsafe {
            read_volatile(self.ena_ptr())
        }
    }

    #[doc="Write the ENA register."]
    #[inline] pub fn set_ena<F: FnOnce(Ena) -> Ena>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ena_mut(), f(Ena(0)));
        }
        self
    }

    #[doc="Modify the ENA register."]
    #[inline] pub fn with_ena<F: FnOnce(Ena) -> Ena>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ena_mut(), f(self.ena()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ENB register."]
    #[inline] pub fn enb_mut(&self) -> *mut Enb { 
        (self.0 + 0xa4) as *mut Enb
    }

    #[doc="Get the *const pointer for the ENB register."]
    #[inline] pub fn enb_ptr(&self) -> *const Enb { 
           self.enb_mut()
    }

    #[doc="Read the ENB register."]
    #[inline] pub fn enb(&self) -> Enb { 
        unsafe {
            read_volatile(self.enb_ptr())
        }
    }

    #[doc="Write the ENB register."]
    #[inline] pub fn set_enb<F: FnOnce(Enb) -> Enb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.enb_mut(), f(Enb(0)));
        }
        self
    }

    #[doc="Modify the ENB register."]
    #[inline] pub fn with_enb<F: FnOnce(Enb) -> Enb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.enb_mut(), f(self.enb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ENSA register."]
    #[inline] pub fn ensa_mut(&self) -> *mut Ensa { 
        (self.0 + 0xa8) as *mut Ensa
    }

    #[doc="Get the *const pointer for the ENSA register."]
    #[inline] pub fn ensa_ptr(&self) -> *const Ensa { 
           self.ensa_mut()
    }

    #[doc="Read the ENSA register."]
    #[inline] pub fn ensa(&self) -> Ensa { 
        unsafe {
            read_volatile(self.ensa_ptr())
        }
    }

    #[doc="Write the ENSA register."]
    #[inline] pub fn set_ensa<F: FnOnce(Ensa) -> Ensa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ensa_mut(), f(Ensa(0)));
        }
        self
    }

    #[doc="Modify the ENSA register."]
    #[inline] pub fn with_ensa<F: FnOnce(Ensa) -> Ensa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ensa_mut(), f(self.ensa()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ENSB register."]
    #[inline] pub fn ensb_mut(&self) -> *mut Ensb { 
        (self.0 + 0xac) as *mut Ensb
    }

    #[doc="Get the *const pointer for the ENSB register."]
    #[inline] pub fn ensb_ptr(&self) -> *const Ensb { 
           self.ensb_mut()
    }

    #[doc="Read the ENSB register."]
    #[inline] pub fn ensb(&self) -> Ensb { 
        unsafe {
            read_volatile(self.ensb_ptr())
        }
    }

    #[doc="Write the ENSB register."]
    #[inline] pub fn set_ensb<F: FnOnce(Ensb) -> Ensb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ensb_mut(), f(Ensb(0)));
        }
        self
    }

    #[doc="Modify the ENSB register."]
    #[inline] pub fn with_ensb<F: FnOnce(Ensb) -> Ensb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ensb_mut(), f(self.ensb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ENCA register."]
    #[inline] pub fn enca_mut(&self) -> *mut Enca { 
        (self.0 + 0xb4) as *mut Enca
    }

    #[doc="Get the *const pointer for the ENCA register."]
    #[inline] pub fn enca_ptr(&self) -> *const Enca { 
           self.enca_mut()
    }

    #[doc="Read the ENCA register."]
    #[inline] pub fn enca(&self) -> Enca { 
        unsafe {
            read_volatile(self.enca_ptr())
        }
    }

    #[doc="Write the ENCA register."]
    #[inline] pub fn set_enca<F: FnOnce(Enca) -> Enca>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.enca_mut(), f(Enca(0)));
        }
        self
    }

    #[doc="Modify the ENCA register."]
    #[inline] pub fn with_enca<F: FnOnce(Enca) -> Enca>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.enca_mut(), f(self.enca()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ENCB register."]
    #[inline] pub fn encb_mut(&self) -> *mut Encb { 
        (self.0 + 0xb8) as *mut Encb
    }

    #[doc="Get the *const pointer for the ENCB register."]
    #[inline] pub fn encb_ptr(&self) -> *const Encb { 
           self.encb_mut()
    }

    #[doc="Read the ENCB register."]
    #[inline] pub fn encb(&self) -> Encb { 
        unsafe {
            read_volatile(self.encb_ptr())
        }
    }

    #[doc="Write the ENCB register."]
    #[inline] pub fn set_encb<F: FnOnce(Encb) -> Encb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.encb_mut(), f(Encb(0)));
        }
        self
    }

    #[doc="Modify the ENCB register."]
    #[inline] pub fn with_encb<F: FnOnce(Encb) -> Encb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.encb_mut(), f(self.encb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STMRCAP register."]
    #[inline] pub fn stmrcap_mut(&self) -> *mut Stmrcap { 
        (self.0 + 0xbc) as *mut Stmrcap
    }

    #[doc="Get the *const pointer for the STMRCAP register."]
    #[inline] pub fn stmrcap_ptr(&self) -> *const Stmrcap { 
           self.stmrcap_mut()
    }

    #[doc="Read the STMRCAP register."]
    #[inline] pub fn stmrcap(&self) -> Stmrcap { 
        unsafe {
            read_volatile(self.stmrcap_ptr())
        }
    }

    #[doc="Write the STMRCAP register."]
    #[inline] pub fn set_stmrcap<F: FnOnce(Stmrcap) -> Stmrcap>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmrcap_mut(), f(Stmrcap(0)));
        }
        self
    }

    #[doc="Modify the STMRCAP register."]
    #[inline] pub fn with_stmrcap<F: FnOnce(Stmrcap) -> Stmrcap>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmrcap_mut(), f(self.stmrcap()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IOM0IRQ register."]
    #[inline] pub fn iom0irq_mut(&self) -> *mut Iom0irq { 
        (self.0 + 0xc0) as *mut Iom0irq
    }

    #[doc="Get the *const pointer for the IOM0IRQ register."]
    #[inline] pub fn iom0irq_ptr(&self) -> *const Iom0irq { 
           self.iom0irq_mut()
    }

    #[doc="Read the IOM0IRQ register."]
    #[inline] pub fn iom0irq(&self) -> Iom0irq { 
        unsafe {
            read_volatile(self.iom0irq_ptr())
        }
    }

    #[doc="Write the IOM0IRQ register."]
    #[inline] pub fn set_iom0irq<F: FnOnce(Iom0irq) -> Iom0irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom0irq_mut(), f(Iom0irq(0)));
        }
        self
    }

    #[doc="Modify the IOM0IRQ register."]
    #[inline] pub fn with_iom0irq<F: FnOnce(Iom0irq) -> Iom0irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom0irq_mut(), f(self.iom0irq()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IOM1IRQ register."]
    #[inline] pub fn iom1irq_mut(&self) -> *mut Iom1irq { 
        (self.0 + 0xc4) as *mut Iom1irq
    }

    #[doc="Get the *const pointer for the IOM1IRQ register."]
    #[inline] pub fn iom1irq_ptr(&self) -> *const Iom1irq { 
           self.iom1irq_mut()
    }

    #[doc="Read the IOM1IRQ register."]
    #[inline] pub fn iom1irq(&self) -> Iom1irq { 
        unsafe {
            read_volatile(self.iom1irq_ptr())
        }
    }

    #[doc="Write the IOM1IRQ register."]
    #[inline] pub fn set_iom1irq<F: FnOnce(Iom1irq) -> Iom1irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom1irq_mut(), f(Iom1irq(0)));
        }
        self
    }

    #[doc="Modify the IOM1IRQ register."]
    #[inline] pub fn with_iom1irq<F: FnOnce(Iom1irq) -> Iom1irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom1irq_mut(), f(self.iom1irq()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IOM2IRQ register."]
    #[inline] pub fn iom2irq_mut(&self) -> *mut Iom2irq { 
        (self.0 + 0xc8) as *mut Iom2irq
    }

    #[doc="Get the *const pointer for the IOM2IRQ register."]
    #[inline] pub fn iom2irq_ptr(&self) -> *const Iom2irq { 
           self.iom2irq_mut()
    }

    #[doc="Read the IOM2IRQ register."]
    #[inline] pub fn iom2irq(&self) -> Iom2irq { 
        unsafe {
            read_volatile(self.iom2irq_ptr())
        }
    }

    #[doc="Write the IOM2IRQ register."]
    #[inline] pub fn set_iom2irq<F: FnOnce(Iom2irq) -> Iom2irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom2irq_mut(), f(Iom2irq(0)));
        }
        self
    }

    #[doc="Modify the IOM2IRQ register."]
    #[inline] pub fn with_iom2irq<F: FnOnce(Iom2irq) -> Iom2irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom2irq_mut(), f(self.iom2irq()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IOM3IRQ register."]
    #[inline] pub fn iom3irq_mut(&self) -> *mut Iom3irq { 
        (self.0 + 0xcc) as *mut Iom3irq
    }

    #[doc="Get the *const pointer for the IOM3IRQ register."]
    #[inline] pub fn iom3irq_ptr(&self) -> *const Iom3irq { 
           self.iom3irq_mut()
    }

    #[doc="Read the IOM3IRQ register."]
    #[inline] pub fn iom3irq(&self) -> Iom3irq { 
        unsafe {
            read_volatile(self.iom3irq_ptr())
        }
    }

    #[doc="Write the IOM3IRQ register."]
    #[inline] pub fn set_iom3irq<F: FnOnce(Iom3irq) -> Iom3irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom3irq_mut(), f(Iom3irq(0)));
        }
        self
    }

    #[doc="Modify the IOM3IRQ register."]
    #[inline] pub fn with_iom3irq<F: FnOnce(Iom3irq) -> Iom3irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom3irq_mut(), f(self.iom3irq()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IOM4IRQ register."]
    #[inline] pub fn iom4irq_mut(&self) -> *mut Iom4irq { 
        (self.0 + 0xd0) as *mut Iom4irq
    }

    #[doc="Get the *const pointer for the IOM4IRQ register."]
    #[inline] pub fn iom4irq_ptr(&self) -> *const Iom4irq { 
           self.iom4irq_mut()
    }

    #[doc="Read the IOM4IRQ register."]
    #[inline] pub fn iom4irq(&self) -> Iom4irq { 
        unsafe {
            read_volatile(self.iom4irq_ptr())
        }
    }

    #[doc="Write the IOM4IRQ register."]
    #[inline] pub fn set_iom4irq<F: FnOnce(Iom4irq) -> Iom4irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom4irq_mut(), f(Iom4irq(0)));
        }
        self
    }

    #[doc="Modify the IOM4IRQ register."]
    #[inline] pub fn with_iom4irq<F: FnOnce(Iom4irq) -> Iom4irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom4irq_mut(), f(self.iom4irq()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IOM5IRQ register."]
    #[inline] pub fn iom5irq_mut(&self) -> *mut Iom5irq { 
        (self.0 + 0xd4) as *mut Iom5irq
    }

    #[doc="Get the *const pointer for the IOM5IRQ register."]
    #[inline] pub fn iom5irq_ptr(&self) -> *const Iom5irq { 
           self.iom5irq_mut()
    }

    #[doc="Read the IOM5IRQ register."]
    #[inline] pub fn iom5irq(&self) -> Iom5irq { 
        unsafe {
            read_volatile(self.iom5irq_ptr())
        }
    }

    #[doc="Write the IOM5IRQ register."]
    #[inline] pub fn set_iom5irq<F: FnOnce(Iom5irq) -> Iom5irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom5irq_mut(), f(Iom5irq(0)));
        }
        self
    }

    #[doc="Modify the IOM5IRQ register."]
    #[inline] pub fn with_iom5irq<F: FnOnce(Iom5irq) -> Iom5irq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iom5irq_mut(), f(self.iom5irq()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LOOPBACK register."]
    #[inline] pub fn loopback_mut(&self) -> *mut Loopback { 
        (self.0 + 0xd8) as *mut Loopback
    }

    #[doc="Get the *const pointer for the LOOPBACK register."]
    #[inline] pub fn loopback_ptr(&self) -> *const Loopback { 
           self.loopback_mut()
    }

    #[doc="Read the LOOPBACK register."]
    #[inline] pub fn loopback(&self) -> Loopback { 
        unsafe {
            read_volatile(self.loopback_ptr())
        }
    }

    #[doc="Write the LOOPBACK register."]
    #[inline] pub fn set_loopback<F: FnOnce(Loopback) -> Loopback>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.loopback_mut(), f(Loopback(0)));
        }
        self
    }

    #[doc="Modify the LOOPBACK register."]
    #[inline] pub fn with_loopback<F: FnOnce(Loopback) -> Loopback>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.loopback_mut(), f(self.loopback()));
        }
        self
    }

    #[doc="Get the *mut pointer for the GPIOOBS register."]
    #[inline] pub fn gpioobs_mut(&self) -> *mut Gpioobs { 
        (self.0 + 0xdc) as *mut Gpioobs
    }

    #[doc="Get the *const pointer for the GPIOOBS register."]
    #[inline] pub fn gpioobs_ptr(&self) -> *const Gpioobs { 
           self.gpioobs_mut()
    }

    #[doc="Read the GPIOOBS register."]
    #[inline] pub fn gpioobs(&self) -> Gpioobs { 
        unsafe {
            read_volatile(self.gpioobs_ptr())
        }
    }

    #[doc="Write the GPIOOBS register."]
    #[inline] pub fn set_gpioobs<F: FnOnce(Gpioobs) -> Gpioobs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.gpioobs_mut(), f(Gpioobs(0)));
        }
        self
    }

    #[doc="Modify the GPIOOBS register."]
    #[inline] pub fn with_gpioobs<F: FnOnce(Gpioobs) -> Gpioobs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.gpioobs_mut(), f(self.gpioobs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGA register."]
    #[inline] pub fn altpadcfga_mut(&self) -> *mut Altpadcfga { 
        (self.0 + 0xe0) as *mut Altpadcfga
    }

    #[doc="Get the *const pointer for the ALTPADCFGA register."]
    #[inline] pub fn altpadcfga_ptr(&self) -> *const Altpadcfga { 
           self.altpadcfga_mut()
    }

    #[doc="Read the ALTPADCFGA register."]
    #[inline] pub fn altpadcfga(&self) -> Altpadcfga { 
        unsafe {
            read_volatile(self.altpadcfga_ptr())
        }
    }

    #[doc="Write the ALTPADCFGA register."]
    #[inline] pub fn set_altpadcfga<F: FnOnce(Altpadcfga) -> Altpadcfga>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfga_mut(), f(Altpadcfga(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGA register."]
    #[inline] pub fn with_altpadcfga<F: FnOnce(Altpadcfga) -> Altpadcfga>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfga_mut(), f(self.altpadcfga()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGB register."]
    #[inline] pub fn altpadcfgb_mut(&self) -> *mut Altpadcfgb { 
        (self.0 + 0xe4) as *mut Altpadcfgb
    }

    #[doc="Get the *const pointer for the ALTPADCFGB register."]
    #[inline] pub fn altpadcfgb_ptr(&self) -> *const Altpadcfgb { 
           self.altpadcfgb_mut()
    }

    #[doc="Read the ALTPADCFGB register."]
    #[inline] pub fn altpadcfgb(&self) -> Altpadcfgb { 
        unsafe {
            read_volatile(self.altpadcfgb_ptr())
        }
    }

    #[doc="Write the ALTPADCFGB register."]
    #[inline] pub fn set_altpadcfgb<F: FnOnce(Altpadcfgb) -> Altpadcfgb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgb_mut(), f(Altpadcfgb(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGB register."]
    #[inline] pub fn with_altpadcfgb<F: FnOnce(Altpadcfgb) -> Altpadcfgb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgb_mut(), f(self.altpadcfgb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGC register."]
    #[inline] pub fn altpadcfgc_mut(&self) -> *mut Altpadcfgc { 
        (self.0 + 0xe8) as *mut Altpadcfgc
    }

    #[doc="Get the *const pointer for the ALTPADCFGC register."]
    #[inline] pub fn altpadcfgc_ptr(&self) -> *const Altpadcfgc { 
           self.altpadcfgc_mut()
    }

    #[doc="Read the ALTPADCFGC register."]
    #[inline] pub fn altpadcfgc(&self) -> Altpadcfgc { 
        unsafe {
            read_volatile(self.altpadcfgc_ptr())
        }
    }

    #[doc="Write the ALTPADCFGC register."]
    #[inline] pub fn set_altpadcfgc<F: FnOnce(Altpadcfgc) -> Altpadcfgc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgc_mut(), f(Altpadcfgc(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGC register."]
    #[inline] pub fn with_altpadcfgc<F: FnOnce(Altpadcfgc) -> Altpadcfgc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgc_mut(), f(self.altpadcfgc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGD register."]
    #[inline] pub fn altpadcfgd_mut(&self) -> *mut Altpadcfgd { 
        (self.0 + 0xec) as *mut Altpadcfgd
    }

    #[doc="Get the *const pointer for the ALTPADCFGD register."]
    #[inline] pub fn altpadcfgd_ptr(&self) -> *const Altpadcfgd { 
           self.altpadcfgd_mut()
    }

    #[doc="Read the ALTPADCFGD register."]
    #[inline] pub fn altpadcfgd(&self) -> Altpadcfgd { 
        unsafe {
            read_volatile(self.altpadcfgd_ptr())
        }
    }

    #[doc="Write the ALTPADCFGD register."]
    #[inline] pub fn set_altpadcfgd<F: FnOnce(Altpadcfgd) -> Altpadcfgd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgd_mut(), f(Altpadcfgd(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGD register."]
    #[inline] pub fn with_altpadcfgd<F: FnOnce(Altpadcfgd) -> Altpadcfgd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgd_mut(), f(self.altpadcfgd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGE register."]
    #[inline] pub fn altpadcfge_mut(&self) -> *mut Altpadcfge { 
        (self.0 + 0xf0) as *mut Altpadcfge
    }

    #[doc="Get the *const pointer for the ALTPADCFGE register."]
    #[inline] pub fn altpadcfge_ptr(&self) -> *const Altpadcfge { 
           self.altpadcfge_mut()
    }

    #[doc="Read the ALTPADCFGE register."]
    #[inline] pub fn altpadcfge(&self) -> Altpadcfge { 
        unsafe {
            read_volatile(self.altpadcfge_ptr())
        }
    }

    #[doc="Write the ALTPADCFGE register."]
    #[inline] pub fn set_altpadcfge<F: FnOnce(Altpadcfge) -> Altpadcfge>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfge_mut(), f(Altpadcfge(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGE register."]
    #[inline] pub fn with_altpadcfge<F: FnOnce(Altpadcfge) -> Altpadcfge>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfge_mut(), f(self.altpadcfge()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGF register."]
    #[inline] pub fn altpadcfgf_mut(&self) -> *mut Altpadcfgf { 
        (self.0 + 0xf4) as *mut Altpadcfgf
    }

    #[doc="Get the *const pointer for the ALTPADCFGF register."]
    #[inline] pub fn altpadcfgf_ptr(&self) -> *const Altpadcfgf { 
           self.altpadcfgf_mut()
    }

    #[doc="Read the ALTPADCFGF register."]
    #[inline] pub fn altpadcfgf(&self) -> Altpadcfgf { 
        unsafe {
            read_volatile(self.altpadcfgf_ptr())
        }
    }

    #[doc="Write the ALTPADCFGF register."]
    #[inline] pub fn set_altpadcfgf<F: FnOnce(Altpadcfgf) -> Altpadcfgf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgf_mut(), f(Altpadcfgf(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGF register."]
    #[inline] pub fn with_altpadcfgf<F: FnOnce(Altpadcfgf) -> Altpadcfgf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgf_mut(), f(self.altpadcfgf()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGG register."]
    #[inline] pub fn altpadcfgg_mut(&self) -> *mut Altpadcfgg { 
        (self.0 + 0xf8) as *mut Altpadcfgg
    }

    #[doc="Get the *const pointer for the ALTPADCFGG register."]
    #[inline] pub fn altpadcfgg_ptr(&self) -> *const Altpadcfgg { 
           self.altpadcfgg_mut()
    }

    #[doc="Read the ALTPADCFGG register."]
    #[inline] pub fn altpadcfgg(&self) -> Altpadcfgg { 
        unsafe {
            read_volatile(self.altpadcfgg_ptr())
        }
    }

    #[doc="Write the ALTPADCFGG register."]
    #[inline] pub fn set_altpadcfgg<F: FnOnce(Altpadcfgg) -> Altpadcfgg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgg_mut(), f(Altpadcfgg(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGG register."]
    #[inline] pub fn with_altpadcfgg<F: FnOnce(Altpadcfgg) -> Altpadcfgg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgg_mut(), f(self.altpadcfgg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGH register."]
    #[inline] pub fn altpadcfgh_mut(&self) -> *mut Altpadcfgh { 
        (self.0 + 0xfc) as *mut Altpadcfgh
    }

    #[doc="Get the *const pointer for the ALTPADCFGH register."]
    #[inline] pub fn altpadcfgh_ptr(&self) -> *const Altpadcfgh { 
           self.altpadcfgh_mut()
    }

    #[doc="Read the ALTPADCFGH register."]
    #[inline] pub fn altpadcfgh(&self) -> Altpadcfgh { 
        unsafe {
            read_volatile(self.altpadcfgh_ptr())
        }
    }

    #[doc="Write the ALTPADCFGH register."]
    #[inline] pub fn set_altpadcfgh<F: FnOnce(Altpadcfgh) -> Altpadcfgh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgh_mut(), f(Altpadcfgh(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGH register."]
    #[inline] pub fn with_altpadcfgh<F: FnOnce(Altpadcfgh) -> Altpadcfgh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgh_mut(), f(self.altpadcfgh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGI register."]
    #[inline] pub fn altpadcfgi_mut(&self) -> *mut Altpadcfgi { 
        (self.0 + 0x100) as *mut Altpadcfgi
    }

    #[doc="Get the *const pointer for the ALTPADCFGI register."]
    #[inline] pub fn altpadcfgi_ptr(&self) -> *const Altpadcfgi { 
           self.altpadcfgi_mut()
    }

    #[doc="Read the ALTPADCFGI register."]
    #[inline] pub fn altpadcfgi(&self) -> Altpadcfgi { 
        unsafe {
            read_volatile(self.altpadcfgi_ptr())
        }
    }

    #[doc="Write the ALTPADCFGI register."]
    #[inline] pub fn set_altpadcfgi<F: FnOnce(Altpadcfgi) -> Altpadcfgi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgi_mut(), f(Altpadcfgi(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGI register."]
    #[inline] pub fn with_altpadcfgi<F: FnOnce(Altpadcfgi) -> Altpadcfgi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgi_mut(), f(self.altpadcfgi()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGJ register."]
    #[inline] pub fn altpadcfgj_mut(&self) -> *mut Altpadcfgj { 
        (self.0 + 0x104) as *mut Altpadcfgj
    }

    #[doc="Get the *const pointer for the ALTPADCFGJ register."]
    #[inline] pub fn altpadcfgj_ptr(&self) -> *const Altpadcfgj { 
           self.altpadcfgj_mut()
    }

    #[doc="Read the ALTPADCFGJ register."]
    #[inline] pub fn altpadcfgj(&self) -> Altpadcfgj { 
        unsafe {
            read_volatile(self.altpadcfgj_ptr())
        }
    }

    #[doc="Write the ALTPADCFGJ register."]
    #[inline] pub fn set_altpadcfgj<F: FnOnce(Altpadcfgj) -> Altpadcfgj>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgj_mut(), f(Altpadcfgj(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGJ register."]
    #[inline] pub fn with_altpadcfgj<F: FnOnce(Altpadcfgj) -> Altpadcfgj>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgj_mut(), f(self.altpadcfgj()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGK register."]
    #[inline] pub fn altpadcfgk_mut(&self) -> *mut Altpadcfgk { 
        (self.0 + 0x108) as *mut Altpadcfgk
    }

    #[doc="Get the *const pointer for the ALTPADCFGK register."]
    #[inline] pub fn altpadcfgk_ptr(&self) -> *const Altpadcfgk { 
           self.altpadcfgk_mut()
    }

    #[doc="Read the ALTPADCFGK register."]
    #[inline] pub fn altpadcfgk(&self) -> Altpadcfgk { 
        unsafe {
            read_volatile(self.altpadcfgk_ptr())
        }
    }

    #[doc="Write the ALTPADCFGK register."]
    #[inline] pub fn set_altpadcfgk<F: FnOnce(Altpadcfgk) -> Altpadcfgk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgk_mut(), f(Altpadcfgk(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGK register."]
    #[inline] pub fn with_altpadcfgk<F: FnOnce(Altpadcfgk) -> Altpadcfgk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgk_mut(), f(self.altpadcfgk()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGL register."]
    #[inline] pub fn altpadcfgl_mut(&self) -> *mut Altpadcfgl { 
        (self.0 + 0x10c) as *mut Altpadcfgl
    }

    #[doc="Get the *const pointer for the ALTPADCFGL register."]
    #[inline] pub fn altpadcfgl_ptr(&self) -> *const Altpadcfgl { 
           self.altpadcfgl_mut()
    }

    #[doc="Read the ALTPADCFGL register."]
    #[inline] pub fn altpadcfgl(&self) -> Altpadcfgl { 
        unsafe {
            read_volatile(self.altpadcfgl_ptr())
        }
    }

    #[doc="Write the ALTPADCFGL register."]
    #[inline] pub fn set_altpadcfgl<F: FnOnce(Altpadcfgl) -> Altpadcfgl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgl_mut(), f(Altpadcfgl(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGL register."]
    #[inline] pub fn with_altpadcfgl<F: FnOnce(Altpadcfgl) -> Altpadcfgl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgl_mut(), f(self.altpadcfgl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGM register."]
    #[inline] pub fn altpadcfgm_mut(&self) -> *mut Altpadcfgm { 
        (self.0 + 0x110) as *mut Altpadcfgm
    }

    #[doc="Get the *const pointer for the ALTPADCFGM register."]
    #[inline] pub fn altpadcfgm_ptr(&self) -> *const Altpadcfgm { 
           self.altpadcfgm_mut()
    }

    #[doc="Read the ALTPADCFGM register."]
    #[inline] pub fn altpadcfgm(&self) -> Altpadcfgm { 
        unsafe {
            read_volatile(self.altpadcfgm_ptr())
        }
    }

    #[doc="Write the ALTPADCFGM register."]
    #[inline] pub fn set_altpadcfgm<F: FnOnce(Altpadcfgm) -> Altpadcfgm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgm_mut(), f(Altpadcfgm(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGM register."]
    #[inline] pub fn with_altpadcfgm<F: FnOnce(Altpadcfgm) -> Altpadcfgm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgm_mut(), f(self.altpadcfgm()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INT0EN register."]
    #[inline] pub fn int0en_mut(&self) -> *mut Int0en { 
        (self.0 + 0x200) as *mut Int0en
    }

    #[doc="Get the *const pointer for the INT0EN register."]
    #[inline] pub fn int0en_ptr(&self) -> *const Int0en { 
           self.int0en_mut()
    }

    #[doc="Read the INT0EN register."]
    #[inline] pub fn int0en(&self) -> Int0en { 
        unsafe {
            read_volatile(self.int0en_ptr())
        }
    }

    #[doc="Write the INT0EN register."]
    #[inline] pub fn set_int0en<F: FnOnce(Int0en) -> Int0en>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int0en_mut(), f(Int0en(0)));
        }
        self
    }

    #[doc="Modify the INT0EN register."]
    #[inline] pub fn with_int0en<F: FnOnce(Int0en) -> Int0en>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int0en_mut(), f(self.int0en()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INT0STAT register."]
    #[inline] pub fn int0stat_mut(&self) -> *mut Int0stat { 
        (self.0 + 0x204) as *mut Int0stat
    }

    #[doc="Get the *const pointer for the INT0STAT register."]
    #[inline] pub fn int0stat_ptr(&self) -> *const Int0stat { 
           self.int0stat_mut()
    }

    #[doc="Read the INT0STAT register."]
    #[inline] pub fn int0stat(&self) -> Int0stat { 
        unsafe {
            read_volatile(self.int0stat_ptr())
        }
    }

    #[doc="Write the INT0STAT register."]
    #[inline] pub fn set_int0stat<F: FnOnce(Int0stat) -> Int0stat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int0stat_mut(), f(Int0stat(0)));
        }
        self
    }

    #[doc="Modify the INT0STAT register."]
    #[inline] pub fn with_int0stat<F: FnOnce(Int0stat) -> Int0stat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int0stat_mut(), f(self.int0stat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INT0CLR register."]
    #[inline] pub fn int0clr_mut(&self) -> *mut Int0clr { 
        (self.0 + 0x208) as *mut Int0clr
    }

    #[doc="Get the *const pointer for the INT0CLR register."]
    #[inline] pub fn int0clr_ptr(&self) -> *const Int0clr { 
           self.int0clr_mut()
    }

    #[doc="Read the INT0CLR register."]
    #[inline] pub fn int0clr(&self) -> Int0clr { 
        unsafe {
            read_volatile(self.int0clr_ptr())
        }
    }

    #[doc="Write the INT0CLR register."]
    #[inline] pub fn set_int0clr<F: FnOnce(Int0clr) -> Int0clr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int0clr_mut(), f(Int0clr(0)));
        }
        self
    }

    #[doc="Modify the INT0CLR register."]
    #[inline] pub fn with_int0clr<F: FnOnce(Int0clr) -> Int0clr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int0clr_mut(), f(self.int0clr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INT0SET register."]
    #[inline] pub fn int0set_mut(&self) -> *mut Int0set { 
        (self.0 + 0x20c) as *mut Int0set
    }

    #[doc="Get the *const pointer for the INT0SET register."]
    #[inline] pub fn int0set_ptr(&self) -> *const Int0set { 
           self.int0set_mut()
    }

    #[doc="Read the INT0SET register."]
    #[inline] pub fn int0set(&self) -> Int0set { 
        unsafe {
            read_volatile(self.int0set_ptr())
        }
    }

    #[doc="Write the INT0SET register."]
    #[inline] pub fn set_int0set<F: FnOnce(Int0set) -> Int0set>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int0set_mut(), f(Int0set(0)));
        }
        self
    }

    #[doc="Modify the INT0SET register."]
    #[inline] pub fn with_int0set<F: FnOnce(Int0set) -> Int0set>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int0set_mut(), f(self.int0set()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INT1EN register."]
    #[inline] pub fn int1en_mut(&self) -> *mut Int1en { 
        (self.0 + 0x210) as *mut Int1en
    }

    #[doc="Get the *const pointer for the INT1EN register."]
    #[inline] pub fn int1en_ptr(&self) -> *const Int1en { 
           self.int1en_mut()
    }

    #[doc="Read the INT1EN register."]
    #[inline] pub fn int1en(&self) -> Int1en { 
        unsafe {
            read_volatile(self.int1en_ptr())
        }
    }

    #[doc="Write the INT1EN register."]
    #[inline] pub fn set_int1en<F: FnOnce(Int1en) -> Int1en>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int1en_mut(), f(Int1en(0)));
        }
        self
    }

    #[doc="Modify the INT1EN register."]
    #[inline] pub fn with_int1en<F: FnOnce(Int1en) -> Int1en>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int1en_mut(), f(self.int1en()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INT1STAT register."]
    #[inline] pub fn int1stat_mut(&self) -> *mut Int1stat { 
        (self.0 + 0x214) as *mut Int1stat
    }

    #[doc="Get the *const pointer for the INT1STAT register."]
    #[inline] pub fn int1stat_ptr(&self) -> *const Int1stat { 
           self.int1stat_mut()
    }

    #[doc="Read the INT1STAT register."]
    #[inline] pub fn int1stat(&self) -> Int1stat { 
        unsafe {
            read_volatile(self.int1stat_ptr())
        }
    }

    #[doc="Write the INT1STAT register."]
    #[inline] pub fn set_int1stat<F: FnOnce(Int1stat) -> Int1stat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int1stat_mut(), f(Int1stat(0)));
        }
        self
    }

    #[doc="Modify the INT1STAT register."]
    #[inline] pub fn with_int1stat<F: FnOnce(Int1stat) -> Int1stat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int1stat_mut(), f(self.int1stat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INT1CLR register."]
    #[inline] pub fn int1clr_mut(&self) -> *mut Int1clr { 
        (self.0 + 0x218) as *mut Int1clr
    }

    #[doc="Get the *const pointer for the INT1CLR register."]
    #[inline] pub fn int1clr_ptr(&self) -> *const Int1clr { 
           self.int1clr_mut()
    }

    #[doc="Read the INT1CLR register."]
    #[inline] pub fn int1clr(&self) -> Int1clr { 
        unsafe {
            read_volatile(self.int1clr_ptr())
        }
    }

    #[doc="Write the INT1CLR register."]
    #[inline] pub fn set_int1clr<F: FnOnce(Int1clr) -> Int1clr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int1clr_mut(), f(Int1clr(0)));
        }
        self
    }

    #[doc="Modify the INT1CLR register."]
    #[inline] pub fn with_int1clr<F: FnOnce(Int1clr) -> Int1clr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int1clr_mut(), f(self.int1clr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INT1SET register."]
    #[inline] pub fn int1set_mut(&self) -> *mut Int1set { 
        (self.0 + 0x21c) as *mut Int1set
    }

    #[doc="Get the *const pointer for the INT1SET register."]
    #[inline] pub fn int1set_ptr(&self) -> *const Int1set { 
           self.int1set_mut()
    }

    #[doc="Read the INT1SET register."]
    #[inline] pub fn int1set(&self) -> Int1set { 
        unsafe {
            read_volatile(self.int1set_ptr())
        }
    }

    #[doc="Write the INT1SET register."]
    #[inline] pub fn set_int1set<F: FnOnce(Int1set) -> Int1set>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int1set_mut(), f(Int1set(0)));
        }
        self
    }

    #[doc="Modify the INT1SET register."]
    #[inline] pub fn with_int1set<F: FnOnce(Int1set) -> Int1set>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.int1set_mut(), f(self.int1set()));
        }
        self
    }

}

#[doc="Pad Configuration Register A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padrega(pub u32);
impl Padrega {
    #[doc="Pad 3 function select"]
    #[inline] pub fn pad3fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD3FNCSEL != 0"]
    #[inline] pub fn test_pad3fncsel(&self) -> bool {
        self.pad3fncsel() != 0
    }

    #[doc="Sets the PAD3FNCSEL field."]
    #[inline] pub fn set_pad3fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 3 drive strength."]
    #[inline] pub fn pad3strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD3STRNG != 0"]
    #[inline] pub fn test_pad3strng(&self) -> bool {
        self.pad3strng() != 0
    }

    #[doc="Sets the PAD3STRNG field."]
    #[inline] pub fn set_pad3strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 3 input enable."]
    #[inline] pub fn pad3inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD3INPEN != 0"]
    #[inline] pub fn test_pad3inpen(&self) -> bool {
        self.pad3inpen() != 0
    }

    #[doc="Sets the PAD3INPEN field."]
    #[inline] pub fn set_pad3inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 3 pullup enable"]
    #[inline] pub fn pad3pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD3PULL != 0"]
    #[inline] pub fn test_pad3pull(&self) -> bool {
        self.pad3pull() != 0
    }

    #[doc="Sets the PAD3PULL field."]
    #[inline] pub fn set_pad3pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 2 function select"]
    #[inline] pub fn pad2fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD2FNCSEL != 0"]
    #[inline] pub fn test_pad2fncsel(&self) -> bool {
        self.pad2fncsel() != 0
    }

    #[doc="Sets the PAD2FNCSEL field."]
    #[inline] pub fn set_pad2fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 2 drive strength"]
    #[inline] pub fn pad2strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD2STRNG != 0"]
    #[inline] pub fn test_pad2strng(&self) -> bool {
        self.pad2strng() != 0
    }

    #[doc="Sets the PAD2STRNG field."]
    #[inline] pub fn set_pad2strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 2 input enable"]
    #[inline] pub fn pad2inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD2INPEN != 0"]
    #[inline] pub fn test_pad2inpen(&self) -> bool {
        self.pad2inpen() != 0
    }

    #[doc="Sets the PAD2INPEN field."]
    #[inline] pub fn set_pad2inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 2 pullup enable"]
    #[inline] pub fn pad2pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD2PULL != 0"]
    #[inline] pub fn test_pad2pull(&self) -> bool {
        self.pad2pull() != 0
    }

    #[doc="Sets the PAD2PULL field."]
    #[inline] pub fn set_pad2pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 1 pullup resistor selection."]
    #[inline] pub fn pad1rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if PAD1RSEL != 0"]
    #[inline] pub fn test_pad1rsel(&self) -> bool {
        self.pad1rsel() != 0
    }

    #[doc="Sets the PAD1RSEL field."]
    #[inline] pub fn set_pad1rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Pad 1 function select"]
    #[inline] pub fn pad1fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD1FNCSEL != 0"]
    #[inline] pub fn test_pad1fncsel(&self) -> bool {
        self.pad1fncsel() != 0
    }

    #[doc="Sets the PAD1FNCSEL field."]
    #[inline] pub fn set_pad1fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 1 drive strength"]
    #[inline] pub fn pad1strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD1STRNG != 0"]
    #[inline] pub fn test_pad1strng(&self) -> bool {
        self.pad1strng() != 0
    }

    #[doc="Sets the PAD1STRNG field."]
    #[inline] pub fn set_pad1strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 1 input enable"]
    #[inline] pub fn pad1inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD1INPEN != 0"]
    #[inline] pub fn test_pad1inpen(&self) -> bool {
        self.pad1inpen() != 0
    }

    #[doc="Sets the PAD1INPEN field."]
    #[inline] pub fn set_pad1inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 1 pullup enable"]
    #[inline] pub fn pad1pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD1PULL != 0"]
    #[inline] pub fn test_pad1pull(&self) -> bool {
        self.pad1pull() != 0
    }

    #[doc="Sets the PAD1PULL field."]
    #[inline] pub fn set_pad1pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 0 pullup resistor selection."]
    #[inline] pub fn pad0rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if PAD0RSEL != 0"]
    #[inline] pub fn test_pad0rsel(&self) -> bool {
        self.pad0rsel() != 0
    }

    #[doc="Sets the PAD0RSEL field."]
    #[inline] pub fn set_pad0rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pad 0 function select"]
    #[inline] pub fn pad0fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD0FNCSEL != 0"]
    #[inline] pub fn test_pad0fncsel(&self) -> bool {
        self.pad0fncsel() != 0
    }

    #[doc="Sets the PAD0FNCSEL field."]
    #[inline] pub fn set_pad0fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 0 drive strength"]
    #[inline] pub fn pad0strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD0STRNG != 0"]
    #[inline] pub fn test_pad0strng(&self) -> bool {
        self.pad0strng() != 0
    }

    #[doc="Sets the PAD0STRNG field."]
    #[inline] pub fn set_pad0strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 0 input enable"]
    #[inline] pub fn pad0inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD0INPEN != 0"]
    #[inline] pub fn test_pad0inpen(&self) -> bool {
        self.pad0inpen() != 0
    }

    #[doc="Sets the PAD0INPEN field."]
    #[inline] pub fn set_pad0inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 0 pullup enable"]
    #[inline] pub fn pad0pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD0PULL != 0"]
    #[inline] pub fn test_pad0pull(&self) -> bool {
        self.pad0pull() != 0
    }

    #[doc="Sets the PAD0PULL field."]
    #[inline] pub fn set_pad0pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padrega {
    #[inline]
    fn from(other: u32) -> Self {
         Padrega(other)
    }
}

impl ::core::fmt::Display for Padrega {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padrega {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad3fncsel() != 0 { try!(write!(f, " pad3fncsel=0x{:x}", self.pad3fncsel()))}
        if self.pad3strng() != 0 { try!(write!(f, " pad3strng"))}
        if self.pad3inpen() != 0 { try!(write!(f, " pad3inpen"))}
        if self.pad3pull() != 0 { try!(write!(f, " pad3pull"))}
        if self.pad2fncsel() != 0 { try!(write!(f, " pad2fncsel=0x{:x}", self.pad2fncsel()))}
        if self.pad2strng() != 0 { try!(write!(f, " pad2strng"))}
        if self.pad2inpen() != 0 { try!(write!(f, " pad2inpen"))}
        if self.pad2pull() != 0 { try!(write!(f, " pad2pull"))}
        if self.pad1rsel() != 0 { try!(write!(f, " pad1rsel=0x{:x}", self.pad1rsel()))}
        if self.pad1fncsel() != 0 { try!(write!(f, " pad1fncsel=0x{:x}", self.pad1fncsel()))}
        if self.pad1strng() != 0 { try!(write!(f, " pad1strng"))}
        if self.pad1inpen() != 0 { try!(write!(f, " pad1inpen"))}
        if self.pad1pull() != 0 { try!(write!(f, " pad1pull"))}
        if self.pad0rsel() != 0 { try!(write!(f, " pad0rsel=0x{:x}", self.pad0rsel()))}
        if self.pad0fncsel() != 0 { try!(write!(f, " pad0fncsel=0x{:x}", self.pad0fncsel()))}
        if self.pad0strng() != 0 { try!(write!(f, " pad0strng"))}
        if self.pad0inpen() != 0 { try!(write!(f, " pad0inpen"))}
        if self.pad0pull() != 0 { try!(write!(f, " pad0pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padregb(pub u32);
impl Padregb {
    #[doc="Pad 7 function select"]
    #[inline] pub fn pad7fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD7FNCSEL != 0"]
    #[inline] pub fn test_pad7fncsel(&self) -> bool {
        self.pad7fncsel() != 0
    }

    #[doc="Sets the PAD7FNCSEL field."]
    #[inline] pub fn set_pad7fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 7 drive strentgh"]
    #[inline] pub fn pad7strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD7STRNG != 0"]
    #[inline] pub fn test_pad7strng(&self) -> bool {
        self.pad7strng() != 0
    }

    #[doc="Sets the PAD7STRNG field."]
    #[inline] pub fn set_pad7strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 7 input enable"]
    #[inline] pub fn pad7inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD7INPEN != 0"]
    #[inline] pub fn test_pad7inpen(&self) -> bool {
        self.pad7inpen() != 0
    }

    #[doc="Sets the PAD7INPEN field."]
    #[inline] pub fn set_pad7inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 7 pullup enable"]
    #[inline] pub fn pad7pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD7PULL != 0"]
    #[inline] pub fn test_pad7pull(&self) -> bool {
        self.pad7pull() != 0
    }

    #[doc="Sets the PAD7PULL field."]
    #[inline] pub fn set_pad7pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 6 pullup resistor selection."]
    #[inline] pub fn pad6rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if PAD6RSEL != 0"]
    #[inline] pub fn test_pad6rsel(&self) -> bool {
        self.pad6rsel() != 0
    }

    #[doc="Sets the PAD6RSEL field."]
    #[inline] pub fn set_pad6rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Pad 6 function select"]
    #[inline] pub fn pad6fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD6FNCSEL != 0"]
    #[inline] pub fn test_pad6fncsel(&self) -> bool {
        self.pad6fncsel() != 0
    }

    #[doc="Sets the PAD6FNCSEL field."]
    #[inline] pub fn set_pad6fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 6 drive strength"]
    #[inline] pub fn pad6strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD6STRNG != 0"]
    #[inline] pub fn test_pad6strng(&self) -> bool {
        self.pad6strng() != 0
    }

    #[doc="Sets the PAD6STRNG field."]
    #[inline] pub fn set_pad6strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 6 input enable"]
    #[inline] pub fn pad6inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD6INPEN != 0"]
    #[inline] pub fn test_pad6inpen(&self) -> bool {
        self.pad6inpen() != 0
    }

    #[doc="Sets the PAD6INPEN field."]
    #[inline] pub fn set_pad6inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 6 pullup enable"]
    #[inline] pub fn pad6pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD6PULL != 0"]
    #[inline] pub fn test_pad6pull(&self) -> bool {
        self.pad6pull() != 0
    }

    #[doc="Sets the PAD6PULL field."]
    #[inline] pub fn set_pad6pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 5 pullup resistor selection."]
    #[inline] pub fn pad5rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if PAD5RSEL != 0"]
    #[inline] pub fn test_pad5rsel(&self) -> bool {
        self.pad5rsel() != 0
    }

    #[doc="Sets the PAD5RSEL field."]
    #[inline] pub fn set_pad5rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Pad 5 function select"]
    #[inline] pub fn pad5fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD5FNCSEL != 0"]
    #[inline] pub fn test_pad5fncsel(&self) -> bool {
        self.pad5fncsel() != 0
    }

    #[doc="Sets the PAD5FNCSEL field."]
    #[inline] pub fn set_pad5fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 5 drive strength"]
    #[inline] pub fn pad5strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD5STRNG != 0"]
    #[inline] pub fn test_pad5strng(&self) -> bool {
        self.pad5strng() != 0
    }

    #[doc="Sets the PAD5STRNG field."]
    #[inline] pub fn set_pad5strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 5 input enable"]
    #[inline] pub fn pad5inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD5INPEN != 0"]
    #[inline] pub fn test_pad5inpen(&self) -> bool {
        self.pad5inpen() != 0
    }

    #[doc="Sets the PAD5INPEN field."]
    #[inline] pub fn set_pad5inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 5 pullup enable"]
    #[inline] pub fn pad5pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD5PULL != 0"]
    #[inline] pub fn test_pad5pull(&self) -> bool {
        self.pad5pull() != 0
    }

    #[doc="Sets the PAD5PULL field."]
    #[inline] pub fn set_pad5pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 4 VSS power switch enable"]
    #[inline] pub fn pad4pwrdn(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PAD4PWRDN != 0"]
    #[inline] pub fn test_pad4pwrdn(&self) -> bool {
        self.pad4pwrdn() != 0
    }

    #[doc="Sets the PAD4PWRDN field."]
    #[inline] pub fn set_pad4pwrdn<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Pad 4 function select"]
    #[inline] pub fn pad4fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD4FNCSEL != 0"]
    #[inline] pub fn test_pad4fncsel(&self) -> bool {
        self.pad4fncsel() != 0
    }

    #[doc="Sets the PAD4FNCSEL field."]
    #[inline] pub fn set_pad4fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 4 drive strength"]
    #[inline] pub fn pad4strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD4STRNG != 0"]
    #[inline] pub fn test_pad4strng(&self) -> bool {
        self.pad4strng() != 0
    }

    #[doc="Sets the PAD4STRNG field."]
    #[inline] pub fn set_pad4strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 4 input enable"]
    #[inline] pub fn pad4inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD4INPEN != 0"]
    #[inline] pub fn test_pad4inpen(&self) -> bool {
        self.pad4inpen() != 0
    }

    #[doc="Sets the PAD4INPEN field."]
    #[inline] pub fn set_pad4inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 4 pullup enable"]
    #[inline] pub fn pad4pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD4PULL != 0"]
    #[inline] pub fn test_pad4pull(&self) -> bool {
        self.pad4pull() != 0
    }

    #[doc="Sets the PAD4PULL field."]
    #[inline] pub fn set_pad4pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padregb {
    #[inline]
    fn from(other: u32) -> Self {
         Padregb(other)
    }
}

impl ::core::fmt::Display for Padregb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padregb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad7fncsel() != 0 { try!(write!(f, " pad7fncsel=0x{:x}", self.pad7fncsel()))}
        if self.pad7strng() != 0 { try!(write!(f, " pad7strng"))}
        if self.pad7inpen() != 0 { try!(write!(f, " pad7inpen"))}
        if self.pad7pull() != 0 { try!(write!(f, " pad7pull"))}
        if self.pad6rsel() != 0 { try!(write!(f, " pad6rsel=0x{:x}", self.pad6rsel()))}
        if self.pad6fncsel() != 0 { try!(write!(f, " pad6fncsel=0x{:x}", self.pad6fncsel()))}
        if self.pad6strng() != 0 { try!(write!(f, " pad6strng"))}
        if self.pad6inpen() != 0 { try!(write!(f, " pad6inpen"))}
        if self.pad6pull() != 0 { try!(write!(f, " pad6pull"))}
        if self.pad5rsel() != 0 { try!(write!(f, " pad5rsel=0x{:x}", self.pad5rsel()))}
        if self.pad5fncsel() != 0 { try!(write!(f, " pad5fncsel=0x{:x}", self.pad5fncsel()))}
        if self.pad5strng() != 0 { try!(write!(f, " pad5strng"))}
        if self.pad5inpen() != 0 { try!(write!(f, " pad5inpen"))}
        if self.pad5pull() != 0 { try!(write!(f, " pad5pull"))}
        if self.pad4pwrdn() != 0 { try!(write!(f, " pad4pwrdn"))}
        if self.pad4fncsel() != 0 { try!(write!(f, " pad4fncsel=0x{:x}", self.pad4fncsel()))}
        if self.pad4strng() != 0 { try!(write!(f, " pad4strng"))}
        if self.pad4inpen() != 0 { try!(write!(f, " pad4inpen"))}
        if self.pad4pull() != 0 { try!(write!(f, " pad4pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padregc(pub u32);
impl Padregc {
    #[doc="Pad 11 function select"]
    #[inline] pub fn pad11fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD11FNCSEL != 0"]
    #[inline] pub fn test_pad11fncsel(&self) -> bool {
        self.pad11fncsel() != 0
    }

    #[doc="Sets the PAD11FNCSEL field."]
    #[inline] pub fn set_pad11fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 11 drive strentgh"]
    #[inline] pub fn pad11strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD11STRNG != 0"]
    #[inline] pub fn test_pad11strng(&self) -> bool {
        self.pad11strng() != 0
    }

    #[doc="Sets the PAD11STRNG field."]
    #[inline] pub fn set_pad11strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 11 input enable"]
    #[inline] pub fn pad11inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD11INPEN != 0"]
    #[inline] pub fn test_pad11inpen(&self) -> bool {
        self.pad11inpen() != 0
    }

    #[doc="Sets the PAD11INPEN field."]
    #[inline] pub fn set_pad11inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 11 pullup enable"]
    #[inline] pub fn pad11pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD11PULL != 0"]
    #[inline] pub fn test_pad11pull(&self) -> bool {
        self.pad11pull() != 0
    }

    #[doc="Sets the PAD11PULL field."]
    #[inline] pub fn set_pad11pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 10 function select"]
    #[inline] pub fn pad10fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD10FNCSEL != 0"]
    #[inline] pub fn test_pad10fncsel(&self) -> bool {
        self.pad10fncsel() != 0
    }

    #[doc="Sets the PAD10FNCSEL field."]
    #[inline] pub fn set_pad10fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 10 drive strength"]
    #[inline] pub fn pad10strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD10STRNG != 0"]
    #[inline] pub fn test_pad10strng(&self) -> bool {
        self.pad10strng() != 0
    }

    #[doc="Sets the PAD10STRNG field."]
    #[inline] pub fn set_pad10strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 10 input enable"]
    #[inline] pub fn pad10inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD10INPEN != 0"]
    #[inline] pub fn test_pad10inpen(&self) -> bool {
        self.pad10inpen() != 0
    }

    #[doc="Sets the PAD10INPEN field."]
    #[inline] pub fn set_pad10inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 10 pullup enable"]
    #[inline] pub fn pad10pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD10PULL != 0"]
    #[inline] pub fn test_pad10pull(&self) -> bool {
        self.pad10pull() != 0
    }

    #[doc="Sets the PAD10PULL field."]
    #[inline] pub fn set_pad10pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 9 pullup resistor selection"]
    #[inline] pub fn pad9rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if PAD9RSEL != 0"]
    #[inline] pub fn test_pad9rsel(&self) -> bool {
        self.pad9rsel() != 0
    }

    #[doc="Sets the PAD9RSEL field."]
    #[inline] pub fn set_pad9rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Pad 9 function select"]
    #[inline] pub fn pad9fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD9FNCSEL != 0"]
    #[inline] pub fn test_pad9fncsel(&self) -> bool {
        self.pad9fncsel() != 0
    }

    #[doc="Sets the PAD9FNCSEL field."]
    #[inline] pub fn set_pad9fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 9 drive strength"]
    #[inline] pub fn pad9strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD9STRNG != 0"]
    #[inline] pub fn test_pad9strng(&self) -> bool {
        self.pad9strng() != 0
    }

    #[doc="Sets the PAD9STRNG field."]
    #[inline] pub fn set_pad9strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 9 input enable"]
    #[inline] pub fn pad9inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD9INPEN != 0"]
    #[inline] pub fn test_pad9inpen(&self) -> bool {
        self.pad9inpen() != 0
    }

    #[doc="Sets the PAD9INPEN field."]
    #[inline] pub fn set_pad9inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 9 pullup enable"]
    #[inline] pub fn pad9pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD9PULL != 0"]
    #[inline] pub fn test_pad9pull(&self) -> bool {
        self.pad9pull() != 0
    }

    #[doc="Sets the PAD9PULL field."]
    #[inline] pub fn set_pad9pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 8 pullup resistor selection."]
    #[inline] pub fn pad8rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if PAD8RSEL != 0"]
    #[inline] pub fn test_pad8rsel(&self) -> bool {
        self.pad8rsel() != 0
    }

    #[doc="Sets the PAD8RSEL field."]
    #[inline] pub fn set_pad8rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pad 8 function select"]
    #[inline] pub fn pad8fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD8FNCSEL != 0"]
    #[inline] pub fn test_pad8fncsel(&self) -> bool {
        self.pad8fncsel() != 0
    }

    #[doc="Sets the PAD8FNCSEL field."]
    #[inline] pub fn set_pad8fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 8 drive strength"]
    #[inline] pub fn pad8strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD8STRNG != 0"]
    #[inline] pub fn test_pad8strng(&self) -> bool {
        self.pad8strng() != 0
    }

    #[doc="Sets the PAD8STRNG field."]
    #[inline] pub fn set_pad8strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 8 input enable"]
    #[inline] pub fn pad8inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD8INPEN != 0"]
    #[inline] pub fn test_pad8inpen(&self) -> bool {
        self.pad8inpen() != 0
    }

    #[doc="Sets the PAD8INPEN field."]
    #[inline] pub fn set_pad8inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 8 pullup enable"]
    #[inline] pub fn pad8pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD8PULL != 0"]
    #[inline] pub fn test_pad8pull(&self) -> bool {
        self.pad8pull() != 0
    }

    #[doc="Sets the PAD8PULL field."]
    #[inline] pub fn set_pad8pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padregc {
    #[inline]
    fn from(other: u32) -> Self {
         Padregc(other)
    }
}

impl ::core::fmt::Display for Padregc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padregc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad11fncsel() != 0 { try!(write!(f, " pad11fncsel=0x{:x}", self.pad11fncsel()))}
        if self.pad11strng() != 0 { try!(write!(f, " pad11strng"))}
        if self.pad11inpen() != 0 { try!(write!(f, " pad11inpen"))}
        if self.pad11pull() != 0 { try!(write!(f, " pad11pull"))}
        if self.pad10fncsel() != 0 { try!(write!(f, " pad10fncsel=0x{:x}", self.pad10fncsel()))}
        if self.pad10strng() != 0 { try!(write!(f, " pad10strng"))}
        if self.pad10inpen() != 0 { try!(write!(f, " pad10inpen"))}
        if self.pad10pull() != 0 { try!(write!(f, " pad10pull"))}
        if self.pad9rsel() != 0 { try!(write!(f, " pad9rsel=0x{:x}", self.pad9rsel()))}
        if self.pad9fncsel() != 0 { try!(write!(f, " pad9fncsel=0x{:x}", self.pad9fncsel()))}
        if self.pad9strng() != 0 { try!(write!(f, " pad9strng"))}
        if self.pad9inpen() != 0 { try!(write!(f, " pad9inpen"))}
        if self.pad9pull() != 0 { try!(write!(f, " pad9pull"))}
        if self.pad8rsel() != 0 { try!(write!(f, " pad8rsel=0x{:x}", self.pad8rsel()))}
        if self.pad8fncsel() != 0 { try!(write!(f, " pad8fncsel=0x{:x}", self.pad8fncsel()))}
        if self.pad8strng() != 0 { try!(write!(f, " pad8strng"))}
        if self.pad8inpen() != 0 { try!(write!(f, " pad8inpen"))}
        if self.pad8pull() != 0 { try!(write!(f, " pad8pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register D"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padregd(pub u32);
impl Padregd {
    #[doc="Pad 15 function select"]
    #[inline] pub fn pad15fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD15FNCSEL != 0"]
    #[inline] pub fn test_pad15fncsel(&self) -> bool {
        self.pad15fncsel() != 0
    }

    #[doc="Sets the PAD15FNCSEL field."]
    #[inline] pub fn set_pad15fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 15 drive strentgh"]
    #[inline] pub fn pad15strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD15STRNG != 0"]
    #[inline] pub fn test_pad15strng(&self) -> bool {
        self.pad15strng() != 0
    }

    #[doc="Sets the PAD15STRNG field."]
    #[inline] pub fn set_pad15strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 15 input enable"]
    #[inline] pub fn pad15inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD15INPEN != 0"]
    #[inline] pub fn test_pad15inpen(&self) -> bool {
        self.pad15inpen() != 0
    }

    #[doc="Sets the PAD15INPEN field."]
    #[inline] pub fn set_pad15inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 15 pullup enable"]
    #[inline] pub fn pad15pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD15PULL != 0"]
    #[inline] pub fn test_pad15pull(&self) -> bool {
        self.pad15pull() != 0
    }

    #[doc="Sets the PAD15PULL field."]
    #[inline] pub fn set_pad15pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 14 function select"]
    #[inline] pub fn pad14fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD14FNCSEL != 0"]
    #[inline] pub fn test_pad14fncsel(&self) -> bool {
        self.pad14fncsel() != 0
    }

    #[doc="Sets the PAD14FNCSEL field."]
    #[inline] pub fn set_pad14fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 14 drive strength"]
    #[inline] pub fn pad14strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD14STRNG != 0"]
    #[inline] pub fn test_pad14strng(&self) -> bool {
        self.pad14strng() != 0
    }

    #[doc="Sets the PAD14STRNG field."]
    #[inline] pub fn set_pad14strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 14 input enable"]
    #[inline] pub fn pad14inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD14INPEN != 0"]
    #[inline] pub fn test_pad14inpen(&self) -> bool {
        self.pad14inpen() != 0
    }

    #[doc="Sets the PAD14INPEN field."]
    #[inline] pub fn set_pad14inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 14 pullup enable"]
    #[inline] pub fn pad14pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD14PULL != 0"]
    #[inline] pub fn test_pad14pull(&self) -> bool {
        self.pad14pull() != 0
    }

    #[doc="Sets the PAD14PULL field."]
    #[inline] pub fn set_pad14pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 13 function select"]
    #[inline] pub fn pad13fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD13FNCSEL != 0"]
    #[inline] pub fn test_pad13fncsel(&self) -> bool {
        self.pad13fncsel() != 0
    }

    #[doc="Sets the PAD13FNCSEL field."]
    #[inline] pub fn set_pad13fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 13 drive strength"]
    #[inline] pub fn pad13strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD13STRNG != 0"]
    #[inline] pub fn test_pad13strng(&self) -> bool {
        self.pad13strng() != 0
    }

    #[doc="Sets the PAD13STRNG field."]
    #[inline] pub fn set_pad13strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 13 input enable"]
    #[inline] pub fn pad13inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD13INPEN != 0"]
    #[inline] pub fn test_pad13inpen(&self) -> bool {
        self.pad13inpen() != 0
    }

    #[doc="Sets the PAD13INPEN field."]
    #[inline] pub fn set_pad13inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 13 pullup enable"]
    #[inline] pub fn pad13pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD13PULL != 0"]
    #[inline] pub fn test_pad13pull(&self) -> bool {
        self.pad13pull() != 0
    }

    #[doc="Sets the PAD13PULL field."]
    #[inline] pub fn set_pad13pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 12 function select"]
    #[inline] pub fn pad12fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD12FNCSEL != 0"]
    #[inline] pub fn test_pad12fncsel(&self) -> bool {
        self.pad12fncsel() != 0
    }

    #[doc="Sets the PAD12FNCSEL field."]
    #[inline] pub fn set_pad12fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 12 drive strength"]
    #[inline] pub fn pad12strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD12STRNG != 0"]
    #[inline] pub fn test_pad12strng(&self) -> bool {
        self.pad12strng() != 0
    }

    #[doc="Sets the PAD12STRNG field."]
    #[inline] pub fn set_pad12strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 12 input enable"]
    #[inline] pub fn pad12inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD12INPEN != 0"]
    #[inline] pub fn test_pad12inpen(&self) -> bool {
        self.pad12inpen() != 0
    }

    #[doc="Sets the PAD12INPEN field."]
    #[inline] pub fn set_pad12inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 12 pullup enable"]
    #[inline] pub fn pad12pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD12PULL != 0"]
    #[inline] pub fn test_pad12pull(&self) -> bool {
        self.pad12pull() != 0
    }

    #[doc="Sets the PAD12PULL field."]
    #[inline] pub fn set_pad12pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padregd {
    #[inline]
    fn from(other: u32) -> Self {
         Padregd(other)
    }
}

impl ::core::fmt::Display for Padregd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padregd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad15fncsel() != 0 { try!(write!(f, " pad15fncsel=0x{:x}", self.pad15fncsel()))}
        if self.pad15strng() != 0 { try!(write!(f, " pad15strng"))}
        if self.pad15inpen() != 0 { try!(write!(f, " pad15inpen"))}
        if self.pad15pull() != 0 { try!(write!(f, " pad15pull"))}
        if self.pad14fncsel() != 0 { try!(write!(f, " pad14fncsel=0x{:x}", self.pad14fncsel()))}
        if self.pad14strng() != 0 { try!(write!(f, " pad14strng"))}
        if self.pad14inpen() != 0 { try!(write!(f, " pad14inpen"))}
        if self.pad14pull() != 0 { try!(write!(f, " pad14pull"))}
        if self.pad13fncsel() != 0 { try!(write!(f, " pad13fncsel=0x{:x}", self.pad13fncsel()))}
        if self.pad13strng() != 0 { try!(write!(f, " pad13strng"))}
        if self.pad13inpen() != 0 { try!(write!(f, " pad13inpen"))}
        if self.pad13pull() != 0 { try!(write!(f, " pad13pull"))}
        if self.pad12fncsel() != 0 { try!(write!(f, " pad12fncsel=0x{:x}", self.pad12fncsel()))}
        if self.pad12strng() != 0 { try!(write!(f, " pad12strng"))}
        if self.pad12inpen() != 0 { try!(write!(f, " pad12inpen"))}
        if self.pad12pull() != 0 { try!(write!(f, " pad12pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register E"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padrege(pub u32);
impl Padrege {
    #[doc="Pad 19 function select"]
    #[inline] pub fn pad19fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD19FNCSEL != 0"]
    #[inline] pub fn test_pad19fncsel(&self) -> bool {
        self.pad19fncsel() != 0
    }

    #[doc="Sets the PAD19FNCSEL field."]
    #[inline] pub fn set_pad19fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 19 drive strentgh"]
    #[inline] pub fn pad19strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD19STRNG != 0"]
    #[inline] pub fn test_pad19strng(&self) -> bool {
        self.pad19strng() != 0
    }

    #[doc="Sets the PAD19STRNG field."]
    #[inline] pub fn set_pad19strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 19 input enable"]
    #[inline] pub fn pad19inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD19INPEN != 0"]
    #[inline] pub fn test_pad19inpen(&self) -> bool {
        self.pad19inpen() != 0
    }

    #[doc="Sets the PAD19INPEN field."]
    #[inline] pub fn set_pad19inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 19 pullup enable"]
    #[inline] pub fn pad19pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD19PULL != 0"]
    #[inline] pub fn test_pad19pull(&self) -> bool {
        self.pad19pull() != 0
    }

    #[doc="Sets the PAD19PULL field."]
    #[inline] pub fn set_pad19pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 18 function select"]
    #[inline] pub fn pad18fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD18FNCSEL != 0"]
    #[inline] pub fn test_pad18fncsel(&self) -> bool {
        self.pad18fncsel() != 0
    }

    #[doc="Sets the PAD18FNCSEL field."]
    #[inline] pub fn set_pad18fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 18 drive strength"]
    #[inline] pub fn pad18strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD18STRNG != 0"]
    #[inline] pub fn test_pad18strng(&self) -> bool {
        self.pad18strng() != 0
    }

    #[doc="Sets the PAD18STRNG field."]
    #[inline] pub fn set_pad18strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 18 input enable"]
    #[inline] pub fn pad18inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD18INPEN != 0"]
    #[inline] pub fn test_pad18inpen(&self) -> bool {
        self.pad18inpen() != 0
    }

    #[doc="Sets the PAD18INPEN field."]
    #[inline] pub fn set_pad18inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 18 pullup enable"]
    #[inline] pub fn pad18pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD18PULL != 0"]
    #[inline] pub fn test_pad18pull(&self) -> bool {
        self.pad18pull() != 0
    }

    #[doc="Sets the PAD18PULL field."]
    #[inline] pub fn set_pad18pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 17 function select"]
    #[inline] pub fn pad17fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD17FNCSEL != 0"]
    #[inline] pub fn test_pad17fncsel(&self) -> bool {
        self.pad17fncsel() != 0
    }

    #[doc="Sets the PAD17FNCSEL field."]
    #[inline] pub fn set_pad17fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 17 drive strength"]
    #[inline] pub fn pad17strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD17STRNG != 0"]
    #[inline] pub fn test_pad17strng(&self) -> bool {
        self.pad17strng() != 0
    }

    #[doc="Sets the PAD17STRNG field."]
    #[inline] pub fn set_pad17strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 17 input enable"]
    #[inline] pub fn pad17inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD17INPEN != 0"]
    #[inline] pub fn test_pad17inpen(&self) -> bool {
        self.pad17inpen() != 0
    }

    #[doc="Sets the PAD17INPEN field."]
    #[inline] pub fn set_pad17inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 17 pullup enable"]
    #[inline] pub fn pad17pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD17PULL != 0"]
    #[inline] pub fn test_pad17pull(&self) -> bool {
        self.pad17pull() != 0
    }

    #[doc="Sets the PAD17PULL field."]
    #[inline] pub fn set_pad17pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 16 function select"]
    #[inline] pub fn pad16fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD16FNCSEL != 0"]
    #[inline] pub fn test_pad16fncsel(&self) -> bool {
        self.pad16fncsel() != 0
    }

    #[doc="Sets the PAD16FNCSEL field."]
    #[inline] pub fn set_pad16fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 16 drive strength"]
    #[inline] pub fn pad16strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD16STRNG != 0"]
    #[inline] pub fn test_pad16strng(&self) -> bool {
        self.pad16strng() != 0
    }

    #[doc="Sets the PAD16STRNG field."]
    #[inline] pub fn set_pad16strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 16 input enable"]
    #[inline] pub fn pad16inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD16INPEN != 0"]
    #[inline] pub fn test_pad16inpen(&self) -> bool {
        self.pad16inpen() != 0
    }

    #[doc="Sets the PAD16INPEN field."]
    #[inline] pub fn set_pad16inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 16 pullup enable"]
    #[inline] pub fn pad16pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD16PULL != 0"]
    #[inline] pub fn test_pad16pull(&self) -> bool {
        self.pad16pull() != 0
    }

    #[doc="Sets the PAD16PULL field."]
    #[inline] pub fn set_pad16pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padrege {
    #[inline]
    fn from(other: u32) -> Self {
         Padrege(other)
    }
}

impl ::core::fmt::Display for Padrege {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padrege {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad19fncsel() != 0 { try!(write!(f, " pad19fncsel=0x{:x}", self.pad19fncsel()))}
        if self.pad19strng() != 0 { try!(write!(f, " pad19strng"))}
        if self.pad19inpen() != 0 { try!(write!(f, " pad19inpen"))}
        if self.pad19pull() != 0 { try!(write!(f, " pad19pull"))}
        if self.pad18fncsel() != 0 { try!(write!(f, " pad18fncsel=0x{:x}", self.pad18fncsel()))}
        if self.pad18strng() != 0 { try!(write!(f, " pad18strng"))}
        if self.pad18inpen() != 0 { try!(write!(f, " pad18inpen"))}
        if self.pad18pull() != 0 { try!(write!(f, " pad18pull"))}
        if self.pad17fncsel() != 0 { try!(write!(f, " pad17fncsel=0x{:x}", self.pad17fncsel()))}
        if self.pad17strng() != 0 { try!(write!(f, " pad17strng"))}
        if self.pad17inpen() != 0 { try!(write!(f, " pad17inpen"))}
        if self.pad17pull() != 0 { try!(write!(f, " pad17pull"))}
        if self.pad16fncsel() != 0 { try!(write!(f, " pad16fncsel=0x{:x}", self.pad16fncsel()))}
        if self.pad16strng() != 0 { try!(write!(f, " pad16strng"))}
        if self.pad16inpen() != 0 { try!(write!(f, " pad16inpen"))}
        if self.pad16pull() != 0 { try!(write!(f, " pad16pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register F"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padregf(pub u32);
impl Padregf {
    #[doc="Pad 23 function select"]
    #[inline] pub fn pad23fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD23FNCSEL != 0"]
    #[inline] pub fn test_pad23fncsel(&self) -> bool {
        self.pad23fncsel() != 0
    }

    #[doc="Sets the PAD23FNCSEL field."]
    #[inline] pub fn set_pad23fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 23 drive strentgh"]
    #[inline] pub fn pad23strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD23STRNG != 0"]
    #[inline] pub fn test_pad23strng(&self) -> bool {
        self.pad23strng() != 0
    }

    #[doc="Sets the PAD23STRNG field."]
    #[inline] pub fn set_pad23strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 23 input enable"]
    #[inline] pub fn pad23inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD23INPEN != 0"]
    #[inline] pub fn test_pad23inpen(&self) -> bool {
        self.pad23inpen() != 0
    }

    #[doc="Sets the PAD23INPEN field."]
    #[inline] pub fn set_pad23inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 23 pullup enable"]
    #[inline] pub fn pad23pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD23PULL != 0"]
    #[inline] pub fn test_pad23pull(&self) -> bool {
        self.pad23pull() != 0
    }

    #[doc="Sets the PAD23PULL field."]
    #[inline] pub fn set_pad23pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 22 upper power switch enable"]
    #[inline] pub fn pad22pwrup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PAD22PWRUP != 0"]
    #[inline] pub fn test_pad22pwrup(&self) -> bool {
        self.pad22pwrup() != 0
    }

    #[doc="Sets the PAD22PWRUP field."]
    #[inline] pub fn set_pad22pwrup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Pad 22 function select"]
    #[inline] pub fn pad22fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD22FNCSEL != 0"]
    #[inline] pub fn test_pad22fncsel(&self) -> bool {
        self.pad22fncsel() != 0
    }

    #[doc="Sets the PAD22FNCSEL field."]
    #[inline] pub fn set_pad22fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 22 drive strength"]
    #[inline] pub fn pad22strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD22STRNG != 0"]
    #[inline] pub fn test_pad22strng(&self) -> bool {
        self.pad22strng() != 0
    }

    #[doc="Sets the PAD22STRNG field."]
    #[inline] pub fn set_pad22strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 22 input enable"]
    #[inline] pub fn pad22inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD22INPEN != 0"]
    #[inline] pub fn test_pad22inpen(&self) -> bool {
        self.pad22inpen() != 0
    }

    #[doc="Sets the PAD22INPEN field."]
    #[inline] pub fn set_pad22inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 22 pullup enable"]
    #[inline] pub fn pad22pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD22PULL != 0"]
    #[inline] pub fn test_pad22pull(&self) -> bool {
        self.pad22pull() != 0
    }

    #[doc="Sets the PAD22PULL field."]
    #[inline] pub fn set_pad22pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 21 function select"]
    #[inline] pub fn pad21fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD21FNCSEL != 0"]
    #[inline] pub fn test_pad21fncsel(&self) -> bool {
        self.pad21fncsel() != 0
    }

    #[doc="Sets the PAD21FNCSEL field."]
    #[inline] pub fn set_pad21fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 21 drive strength"]
    #[inline] pub fn pad21strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD21STRNG != 0"]
    #[inline] pub fn test_pad21strng(&self) -> bool {
        self.pad21strng() != 0
    }

    #[doc="Sets the PAD21STRNG field."]
    #[inline] pub fn set_pad21strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 21 input enable"]
    #[inline] pub fn pad21inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD21INPEN != 0"]
    #[inline] pub fn test_pad21inpen(&self) -> bool {
        self.pad21inpen() != 0
    }

    #[doc="Sets the PAD21INPEN field."]
    #[inline] pub fn set_pad21inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 21 pullup enable"]
    #[inline] pub fn pad21pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD21PULL != 0"]
    #[inline] pub fn test_pad21pull(&self) -> bool {
        self.pad21pull() != 0
    }

    #[doc="Sets the PAD21PULL field."]
    #[inline] pub fn set_pad21pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 20 function select"]
    #[inline] pub fn pad20fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD20FNCSEL != 0"]
    #[inline] pub fn test_pad20fncsel(&self) -> bool {
        self.pad20fncsel() != 0
    }

    #[doc="Sets the PAD20FNCSEL field."]
    #[inline] pub fn set_pad20fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 20 drive strength"]
    #[inline] pub fn pad20strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD20STRNG != 0"]
    #[inline] pub fn test_pad20strng(&self) -> bool {
        self.pad20strng() != 0
    }

    #[doc="Sets the PAD20STRNG field."]
    #[inline] pub fn set_pad20strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 20 input enable"]
    #[inline] pub fn pad20inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD20INPEN != 0"]
    #[inline] pub fn test_pad20inpen(&self) -> bool {
        self.pad20inpen() != 0
    }

    #[doc="Sets the PAD20INPEN field."]
    #[inline] pub fn set_pad20inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 20 pulldown enable"]
    #[inline] pub fn pad20pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD20PULL != 0"]
    #[inline] pub fn test_pad20pull(&self) -> bool {
        self.pad20pull() != 0
    }

    #[doc="Sets the PAD20PULL field."]
    #[inline] pub fn set_pad20pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padregf {
    #[inline]
    fn from(other: u32) -> Self {
         Padregf(other)
    }
}

impl ::core::fmt::Display for Padregf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padregf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad23fncsel() != 0 { try!(write!(f, " pad23fncsel=0x{:x}", self.pad23fncsel()))}
        if self.pad23strng() != 0 { try!(write!(f, " pad23strng"))}
        if self.pad23inpen() != 0 { try!(write!(f, " pad23inpen"))}
        if self.pad23pull() != 0 { try!(write!(f, " pad23pull"))}
        if self.pad22pwrup() != 0 { try!(write!(f, " pad22pwrup"))}
        if self.pad22fncsel() != 0 { try!(write!(f, " pad22fncsel=0x{:x}", self.pad22fncsel()))}
        if self.pad22strng() != 0 { try!(write!(f, " pad22strng"))}
        if self.pad22inpen() != 0 { try!(write!(f, " pad22inpen"))}
        if self.pad22pull() != 0 { try!(write!(f, " pad22pull"))}
        if self.pad21fncsel() != 0 { try!(write!(f, " pad21fncsel=0x{:x}", self.pad21fncsel()))}
        if self.pad21strng() != 0 { try!(write!(f, " pad21strng"))}
        if self.pad21inpen() != 0 { try!(write!(f, " pad21inpen"))}
        if self.pad21pull() != 0 { try!(write!(f, " pad21pull"))}
        if self.pad20fncsel() != 0 { try!(write!(f, " pad20fncsel=0x{:x}", self.pad20fncsel()))}
        if self.pad20strng() != 0 { try!(write!(f, " pad20strng"))}
        if self.pad20inpen() != 0 { try!(write!(f, " pad20inpen"))}
        if self.pad20pull() != 0 { try!(write!(f, " pad20pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register G"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padregg(pub u32);
impl Padregg {
    #[doc="Pad 27 pullup resistor selection."]
    #[inline] pub fn pad27rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if PAD27RSEL != 0"]
    #[inline] pub fn test_pad27rsel(&self) -> bool {
        self.pad27rsel() != 0
    }

    #[doc="Sets the PAD27RSEL field."]
    #[inline] pub fn set_pad27rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Pad 27 function select"]
    #[inline] pub fn pad27fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD27FNCSEL != 0"]
    #[inline] pub fn test_pad27fncsel(&self) -> bool {
        self.pad27fncsel() != 0
    }

    #[doc="Sets the PAD27FNCSEL field."]
    #[inline] pub fn set_pad27fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 27 drive strentgh"]
    #[inline] pub fn pad27strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD27STRNG != 0"]
    #[inline] pub fn test_pad27strng(&self) -> bool {
        self.pad27strng() != 0
    }

    #[doc="Sets the PAD27STRNG field."]
    #[inline] pub fn set_pad27strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 27 input enable"]
    #[inline] pub fn pad27inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD27INPEN != 0"]
    #[inline] pub fn test_pad27inpen(&self) -> bool {
        self.pad27inpen() != 0
    }

    #[doc="Sets the PAD27INPEN field."]
    #[inline] pub fn set_pad27inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 27 pullup enable"]
    #[inline] pub fn pad27pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD27PULL != 0"]
    #[inline] pub fn test_pad27pull(&self) -> bool {
        self.pad27pull() != 0
    }

    #[doc="Sets the PAD27PULL field."]
    #[inline] pub fn set_pad27pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 26 function select"]
    #[inline] pub fn pad26fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD26FNCSEL != 0"]
    #[inline] pub fn test_pad26fncsel(&self) -> bool {
        self.pad26fncsel() != 0
    }

    #[doc="Sets the PAD26FNCSEL field."]
    #[inline] pub fn set_pad26fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 26 drive strength"]
    #[inline] pub fn pad26strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD26STRNG != 0"]
    #[inline] pub fn test_pad26strng(&self) -> bool {
        self.pad26strng() != 0
    }

    #[doc="Sets the PAD26STRNG field."]
    #[inline] pub fn set_pad26strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 26 input enable"]
    #[inline] pub fn pad26inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD26INPEN != 0"]
    #[inline] pub fn test_pad26inpen(&self) -> bool {
        self.pad26inpen() != 0
    }

    #[doc="Sets the PAD26INPEN field."]
    #[inline] pub fn set_pad26inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 26 pullup enable"]
    #[inline] pub fn pad26pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD26PULL != 0"]
    #[inline] pub fn test_pad26pull(&self) -> bool {
        self.pad26pull() != 0
    }

    #[doc="Sets the PAD26PULL field."]
    #[inline] pub fn set_pad26pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 25 pullup resistor selection."]
    #[inline] pub fn pad25rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if PAD25RSEL != 0"]
    #[inline] pub fn test_pad25rsel(&self) -> bool {
        self.pad25rsel() != 0
    }

    #[doc="Sets the PAD25RSEL field."]
    #[inline] pub fn set_pad25rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Pad 25 function select"]
    #[inline] pub fn pad25fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD25FNCSEL != 0"]
    #[inline] pub fn test_pad25fncsel(&self) -> bool {
        self.pad25fncsel() != 0
    }

    #[doc="Sets the PAD25FNCSEL field."]
    #[inline] pub fn set_pad25fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 25 drive strength"]
    #[inline] pub fn pad25strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD25STRNG != 0"]
    #[inline] pub fn test_pad25strng(&self) -> bool {
        self.pad25strng() != 0
    }

    #[doc="Sets the PAD25STRNG field."]
    #[inline] pub fn set_pad25strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 25 input enable"]
    #[inline] pub fn pad25inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD25INPEN != 0"]
    #[inline] pub fn test_pad25inpen(&self) -> bool {
        self.pad25inpen() != 0
    }

    #[doc="Sets the PAD25INPEN field."]
    #[inline] pub fn set_pad25inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 25 pullup enable"]
    #[inline] pub fn pad25pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD25PULL != 0"]
    #[inline] pub fn test_pad25pull(&self) -> bool {
        self.pad25pull() != 0
    }

    #[doc="Sets the PAD25PULL field."]
    #[inline] pub fn set_pad25pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 24 function select"]
    #[inline] pub fn pad24fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD24FNCSEL != 0"]
    #[inline] pub fn test_pad24fncsel(&self) -> bool {
        self.pad24fncsel() != 0
    }

    #[doc="Sets the PAD24FNCSEL field."]
    #[inline] pub fn set_pad24fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 24 drive strength"]
    #[inline] pub fn pad24strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD24STRNG != 0"]
    #[inline] pub fn test_pad24strng(&self) -> bool {
        self.pad24strng() != 0
    }

    #[doc="Sets the PAD24STRNG field."]
    #[inline] pub fn set_pad24strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 24 input enable"]
    #[inline] pub fn pad24inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD24INPEN != 0"]
    #[inline] pub fn test_pad24inpen(&self) -> bool {
        self.pad24inpen() != 0
    }

    #[doc="Sets the PAD24INPEN field."]
    #[inline] pub fn set_pad24inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 24 pullup enable"]
    #[inline] pub fn pad24pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD24PULL != 0"]
    #[inline] pub fn test_pad24pull(&self) -> bool {
        self.pad24pull() != 0
    }

    #[doc="Sets the PAD24PULL field."]
    #[inline] pub fn set_pad24pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padregg {
    #[inline]
    fn from(other: u32) -> Self {
         Padregg(other)
    }
}

impl ::core::fmt::Display for Padregg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padregg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad27rsel() != 0 { try!(write!(f, " pad27rsel=0x{:x}", self.pad27rsel()))}
        if self.pad27fncsel() != 0 { try!(write!(f, " pad27fncsel=0x{:x}", self.pad27fncsel()))}
        if self.pad27strng() != 0 { try!(write!(f, " pad27strng"))}
        if self.pad27inpen() != 0 { try!(write!(f, " pad27inpen"))}
        if self.pad27pull() != 0 { try!(write!(f, " pad27pull"))}
        if self.pad26fncsel() != 0 { try!(write!(f, " pad26fncsel=0x{:x}", self.pad26fncsel()))}
        if self.pad26strng() != 0 { try!(write!(f, " pad26strng"))}
        if self.pad26inpen() != 0 { try!(write!(f, " pad26inpen"))}
        if self.pad26pull() != 0 { try!(write!(f, " pad26pull"))}
        if self.pad25rsel() != 0 { try!(write!(f, " pad25rsel=0x{:x}", self.pad25rsel()))}
        if self.pad25fncsel() != 0 { try!(write!(f, " pad25fncsel=0x{:x}", self.pad25fncsel()))}
        if self.pad25strng() != 0 { try!(write!(f, " pad25strng"))}
        if self.pad25inpen() != 0 { try!(write!(f, " pad25inpen"))}
        if self.pad25pull() != 0 { try!(write!(f, " pad25pull"))}
        if self.pad24fncsel() != 0 { try!(write!(f, " pad24fncsel=0x{:x}", self.pad24fncsel()))}
        if self.pad24strng() != 0 { try!(write!(f, " pad24strng"))}
        if self.pad24inpen() != 0 { try!(write!(f, " pad24inpen"))}
        if self.pad24pull() != 0 { try!(write!(f, " pad24pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register H"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padregh(pub u32);
impl Padregh {
    #[doc="Pad 31 function select"]
    #[inline] pub fn pad31fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD31FNCSEL != 0"]
    #[inline] pub fn test_pad31fncsel(&self) -> bool {
        self.pad31fncsel() != 0
    }

    #[doc="Sets the PAD31FNCSEL field."]
    #[inline] pub fn set_pad31fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 31 drive strentgh"]
    #[inline] pub fn pad31strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD31STRNG != 0"]
    #[inline] pub fn test_pad31strng(&self) -> bool {
        self.pad31strng() != 0
    }

    #[doc="Sets the PAD31STRNG field."]
    #[inline] pub fn set_pad31strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 31 input enable"]
    #[inline] pub fn pad31inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD31INPEN != 0"]
    #[inline] pub fn test_pad31inpen(&self) -> bool {
        self.pad31inpen() != 0
    }

    #[doc="Sets the PAD31INPEN field."]
    #[inline] pub fn set_pad31inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 31 pullup enable"]
    #[inline] pub fn pad31pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD31PULL != 0"]
    #[inline] pub fn test_pad31pull(&self) -> bool {
        self.pad31pull() != 0
    }

    #[doc="Sets the PAD31PULL field."]
    #[inline] pub fn set_pad31pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 30 function select"]
    #[inline] pub fn pad30fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD30FNCSEL != 0"]
    #[inline] pub fn test_pad30fncsel(&self) -> bool {
        self.pad30fncsel() != 0
    }

    #[doc="Sets the PAD30FNCSEL field."]
    #[inline] pub fn set_pad30fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 30 drive strength"]
    #[inline] pub fn pad30strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD30STRNG != 0"]
    #[inline] pub fn test_pad30strng(&self) -> bool {
        self.pad30strng() != 0
    }

    #[doc="Sets the PAD30STRNG field."]
    #[inline] pub fn set_pad30strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 30 input enable"]
    #[inline] pub fn pad30inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD30INPEN != 0"]
    #[inline] pub fn test_pad30inpen(&self) -> bool {
        self.pad30inpen() != 0
    }

    #[doc="Sets the PAD30INPEN field."]
    #[inline] pub fn set_pad30inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 30 pullup enable"]
    #[inline] pub fn pad30pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD30PULL != 0"]
    #[inline] pub fn test_pad30pull(&self) -> bool {
        self.pad30pull() != 0
    }

    #[doc="Sets the PAD30PULL field."]
    #[inline] pub fn set_pad30pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 29 function select"]
    #[inline] pub fn pad29fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD29FNCSEL != 0"]
    #[inline] pub fn test_pad29fncsel(&self) -> bool {
        self.pad29fncsel() != 0
    }

    #[doc="Sets the PAD29FNCSEL field."]
    #[inline] pub fn set_pad29fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 29 drive strength"]
    #[inline] pub fn pad29strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD29STRNG != 0"]
    #[inline] pub fn test_pad29strng(&self) -> bool {
        self.pad29strng() != 0
    }

    #[doc="Sets the PAD29STRNG field."]
    #[inline] pub fn set_pad29strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 29 input enable"]
    #[inline] pub fn pad29inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD29INPEN != 0"]
    #[inline] pub fn test_pad29inpen(&self) -> bool {
        self.pad29inpen() != 0
    }

    #[doc="Sets the PAD29INPEN field."]
    #[inline] pub fn set_pad29inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 29 pullup enable"]
    #[inline] pub fn pad29pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD29PULL != 0"]
    #[inline] pub fn test_pad29pull(&self) -> bool {
        self.pad29pull() != 0
    }

    #[doc="Sets the PAD29PULL field."]
    #[inline] pub fn set_pad29pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 28 function select"]
    #[inline] pub fn pad28fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD28FNCSEL != 0"]
    #[inline] pub fn test_pad28fncsel(&self) -> bool {
        self.pad28fncsel() != 0
    }

    #[doc="Sets the PAD28FNCSEL field."]
    #[inline] pub fn set_pad28fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 28 drive strength"]
    #[inline] pub fn pad28strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD28STRNG != 0"]
    #[inline] pub fn test_pad28strng(&self) -> bool {
        self.pad28strng() != 0
    }

    #[doc="Sets the PAD28STRNG field."]
    #[inline] pub fn set_pad28strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 28 input enable"]
    #[inline] pub fn pad28inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD28INPEN != 0"]
    #[inline] pub fn test_pad28inpen(&self) -> bool {
        self.pad28inpen() != 0
    }

    #[doc="Sets the PAD28INPEN field."]
    #[inline] pub fn set_pad28inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 28 pullup enable"]
    #[inline] pub fn pad28pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD28PULL != 0"]
    #[inline] pub fn test_pad28pull(&self) -> bool {
        self.pad28pull() != 0
    }

    #[doc="Sets the PAD28PULL field."]
    #[inline] pub fn set_pad28pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padregh {
    #[inline]
    fn from(other: u32) -> Self {
         Padregh(other)
    }
}

impl ::core::fmt::Display for Padregh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padregh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad31fncsel() != 0 { try!(write!(f, " pad31fncsel=0x{:x}", self.pad31fncsel()))}
        if self.pad31strng() != 0 { try!(write!(f, " pad31strng"))}
        if self.pad31inpen() != 0 { try!(write!(f, " pad31inpen"))}
        if self.pad31pull() != 0 { try!(write!(f, " pad31pull"))}
        if self.pad30fncsel() != 0 { try!(write!(f, " pad30fncsel=0x{:x}", self.pad30fncsel()))}
        if self.pad30strng() != 0 { try!(write!(f, " pad30strng"))}
        if self.pad30inpen() != 0 { try!(write!(f, " pad30inpen"))}
        if self.pad30pull() != 0 { try!(write!(f, " pad30pull"))}
        if self.pad29fncsel() != 0 { try!(write!(f, " pad29fncsel=0x{:x}", self.pad29fncsel()))}
        if self.pad29strng() != 0 { try!(write!(f, " pad29strng"))}
        if self.pad29inpen() != 0 { try!(write!(f, " pad29inpen"))}
        if self.pad29pull() != 0 { try!(write!(f, " pad29pull"))}
        if self.pad28fncsel() != 0 { try!(write!(f, " pad28fncsel=0x{:x}", self.pad28fncsel()))}
        if self.pad28strng() != 0 { try!(write!(f, " pad28strng"))}
        if self.pad28inpen() != 0 { try!(write!(f, " pad28inpen"))}
        if self.pad28pull() != 0 { try!(write!(f, " pad28pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register I"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padregi(pub u32);
impl Padregi {
    #[doc="Pad 35 function select"]
    #[inline] pub fn pad35fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD35FNCSEL != 0"]
    #[inline] pub fn test_pad35fncsel(&self) -> bool {
        self.pad35fncsel() != 0
    }

    #[doc="Sets the PAD35FNCSEL field."]
    #[inline] pub fn set_pad35fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 35 drive strentgh"]
    #[inline] pub fn pad35strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD35STRNG != 0"]
    #[inline] pub fn test_pad35strng(&self) -> bool {
        self.pad35strng() != 0
    }

    #[doc="Sets the PAD35STRNG field."]
    #[inline] pub fn set_pad35strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 35 input enable"]
    #[inline] pub fn pad35inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD35INPEN != 0"]
    #[inline] pub fn test_pad35inpen(&self) -> bool {
        self.pad35inpen() != 0
    }

    #[doc="Sets the PAD35INPEN field."]
    #[inline] pub fn set_pad35inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 35 pullup enable"]
    #[inline] pub fn pad35pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD35PULL != 0"]
    #[inline] pub fn test_pad35pull(&self) -> bool {
        self.pad35pull() != 0
    }

    #[doc="Sets the PAD35PULL field."]
    #[inline] pub fn set_pad35pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 34 function select"]
    #[inline] pub fn pad34fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD34FNCSEL != 0"]
    #[inline] pub fn test_pad34fncsel(&self) -> bool {
        self.pad34fncsel() != 0
    }

    #[doc="Sets the PAD34FNCSEL field."]
    #[inline] pub fn set_pad34fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 34 drive strength"]
    #[inline] pub fn pad34strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD34STRNG != 0"]
    #[inline] pub fn test_pad34strng(&self) -> bool {
        self.pad34strng() != 0
    }

    #[doc="Sets the PAD34STRNG field."]
    #[inline] pub fn set_pad34strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 34 input enable"]
    #[inline] pub fn pad34inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD34INPEN != 0"]
    #[inline] pub fn test_pad34inpen(&self) -> bool {
        self.pad34inpen() != 0
    }

    #[doc="Sets the PAD34INPEN field."]
    #[inline] pub fn set_pad34inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 34 pullup enable"]
    #[inline] pub fn pad34pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD34PULL != 0"]
    #[inline] pub fn test_pad34pull(&self) -> bool {
        self.pad34pull() != 0
    }

    #[doc="Sets the PAD34PULL field."]
    #[inline] pub fn set_pad34pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 33 function select"]
    #[inline] pub fn pad33fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD33FNCSEL != 0"]
    #[inline] pub fn test_pad33fncsel(&self) -> bool {
        self.pad33fncsel() != 0
    }

    #[doc="Sets the PAD33FNCSEL field."]
    #[inline] pub fn set_pad33fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 33 drive strength"]
    #[inline] pub fn pad33strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD33STRNG != 0"]
    #[inline] pub fn test_pad33strng(&self) -> bool {
        self.pad33strng() != 0
    }

    #[doc="Sets the PAD33STRNG field."]
    #[inline] pub fn set_pad33strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 33 input enable"]
    #[inline] pub fn pad33inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD33INPEN != 0"]
    #[inline] pub fn test_pad33inpen(&self) -> bool {
        self.pad33inpen() != 0
    }

    #[doc="Sets the PAD33INPEN field."]
    #[inline] pub fn set_pad33inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 33 pullup enable"]
    #[inline] pub fn pad33pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD33PULL != 0"]
    #[inline] pub fn test_pad33pull(&self) -> bool {
        self.pad33pull() != 0
    }

    #[doc="Sets the PAD33PULL field."]
    #[inline] pub fn set_pad33pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 32 function select"]
    #[inline] pub fn pad32fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD32FNCSEL != 0"]
    #[inline] pub fn test_pad32fncsel(&self) -> bool {
        self.pad32fncsel() != 0
    }

    #[doc="Sets the PAD32FNCSEL field."]
    #[inline] pub fn set_pad32fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 32 drive strength"]
    #[inline] pub fn pad32strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD32STRNG != 0"]
    #[inline] pub fn test_pad32strng(&self) -> bool {
        self.pad32strng() != 0
    }

    #[doc="Sets the PAD32STRNG field."]
    #[inline] pub fn set_pad32strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 32 input enable"]
    #[inline] pub fn pad32inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD32INPEN != 0"]
    #[inline] pub fn test_pad32inpen(&self) -> bool {
        self.pad32inpen() != 0
    }

    #[doc="Sets the PAD32INPEN field."]
    #[inline] pub fn set_pad32inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 32 pullup enable"]
    #[inline] pub fn pad32pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD32PULL != 0"]
    #[inline] pub fn test_pad32pull(&self) -> bool {
        self.pad32pull() != 0
    }

    #[doc="Sets the PAD32PULL field."]
    #[inline] pub fn set_pad32pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padregi {
    #[inline]
    fn from(other: u32) -> Self {
         Padregi(other)
    }
}

impl ::core::fmt::Display for Padregi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padregi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad35fncsel() != 0 { try!(write!(f, " pad35fncsel=0x{:x}", self.pad35fncsel()))}
        if self.pad35strng() != 0 { try!(write!(f, " pad35strng"))}
        if self.pad35inpen() != 0 { try!(write!(f, " pad35inpen"))}
        if self.pad35pull() != 0 { try!(write!(f, " pad35pull"))}
        if self.pad34fncsel() != 0 { try!(write!(f, " pad34fncsel=0x{:x}", self.pad34fncsel()))}
        if self.pad34strng() != 0 { try!(write!(f, " pad34strng"))}
        if self.pad34inpen() != 0 { try!(write!(f, " pad34inpen"))}
        if self.pad34pull() != 0 { try!(write!(f, " pad34pull"))}
        if self.pad33fncsel() != 0 { try!(write!(f, " pad33fncsel=0x{:x}", self.pad33fncsel()))}
        if self.pad33strng() != 0 { try!(write!(f, " pad33strng"))}
        if self.pad33inpen() != 0 { try!(write!(f, " pad33inpen"))}
        if self.pad33pull() != 0 { try!(write!(f, " pad33pull"))}
        if self.pad32fncsel() != 0 { try!(write!(f, " pad32fncsel=0x{:x}", self.pad32fncsel()))}
        if self.pad32strng() != 0 { try!(write!(f, " pad32strng"))}
        if self.pad32inpen() != 0 { try!(write!(f, " pad32inpen"))}
        if self.pad32pull() != 0 { try!(write!(f, " pad32pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register J"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padregj(pub u32);
impl Padregj {
    #[doc="Pad 39 pullup resistor selection."]
    #[inline] pub fn pad39rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if PAD39RSEL != 0"]
    #[inline] pub fn test_pad39rsel(&self) -> bool {
        self.pad39rsel() != 0
    }

    #[doc="Sets the PAD39RSEL field."]
    #[inline] pub fn set_pad39rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Pad 39 function select"]
    #[inline] pub fn pad39fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD39FNCSEL != 0"]
    #[inline] pub fn test_pad39fncsel(&self) -> bool {
        self.pad39fncsel() != 0
    }

    #[doc="Sets the PAD39FNCSEL field."]
    #[inline] pub fn set_pad39fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 39 drive strentgh"]
    #[inline] pub fn pad39strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD39STRNG != 0"]
    #[inline] pub fn test_pad39strng(&self) -> bool {
        self.pad39strng() != 0
    }

    #[doc="Sets the PAD39STRNG field."]
    #[inline] pub fn set_pad39strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 39 input enable"]
    #[inline] pub fn pad39inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD39INPEN != 0"]
    #[inline] pub fn test_pad39inpen(&self) -> bool {
        self.pad39inpen() != 0
    }

    #[doc="Sets the PAD39INPEN field."]
    #[inline] pub fn set_pad39inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 39 pullup enable"]
    #[inline] pub fn pad39pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD39PULL != 0"]
    #[inline] pub fn test_pad39pull(&self) -> bool {
        self.pad39pull() != 0
    }

    #[doc="Sets the PAD39PULL field."]
    #[inline] pub fn set_pad39pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 38 function select"]
    #[inline] pub fn pad38fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD38FNCSEL != 0"]
    #[inline] pub fn test_pad38fncsel(&self) -> bool {
        self.pad38fncsel() != 0
    }

    #[doc="Sets the PAD38FNCSEL field."]
    #[inline] pub fn set_pad38fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 38 drive strength"]
    #[inline] pub fn pad38strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD38STRNG != 0"]
    #[inline] pub fn test_pad38strng(&self) -> bool {
        self.pad38strng() != 0
    }

    #[doc="Sets the PAD38STRNG field."]
    #[inline] pub fn set_pad38strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 38 input enable"]
    #[inline] pub fn pad38inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD38INPEN != 0"]
    #[inline] pub fn test_pad38inpen(&self) -> bool {
        self.pad38inpen() != 0
    }

    #[doc="Sets the PAD38INPEN field."]
    #[inline] pub fn set_pad38inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 38 pullup enable"]
    #[inline] pub fn pad38pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD38PULL != 0"]
    #[inline] pub fn test_pad38pull(&self) -> bool {
        self.pad38pull() != 0
    }

    #[doc="Sets the PAD38PULL field."]
    #[inline] pub fn set_pad38pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 37 function select"]
    #[inline] pub fn pad37fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD37FNCSEL != 0"]
    #[inline] pub fn test_pad37fncsel(&self) -> bool {
        self.pad37fncsel() != 0
    }

    #[doc="Sets the PAD37FNCSEL field."]
    #[inline] pub fn set_pad37fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 37 drive strength"]
    #[inline] pub fn pad37strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD37STRNG != 0"]
    #[inline] pub fn test_pad37strng(&self) -> bool {
        self.pad37strng() != 0
    }

    #[doc="Sets the PAD37STRNG field."]
    #[inline] pub fn set_pad37strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 37 input enable"]
    #[inline] pub fn pad37inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD37INPEN != 0"]
    #[inline] pub fn test_pad37inpen(&self) -> bool {
        self.pad37inpen() != 0
    }

    #[doc="Sets the PAD37INPEN field."]
    #[inline] pub fn set_pad37inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 37 pullup enable"]
    #[inline] pub fn pad37pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD37PULL != 0"]
    #[inline] pub fn test_pad37pull(&self) -> bool {
        self.pad37pull() != 0
    }

    #[doc="Sets the PAD37PULL field."]
    #[inline] pub fn set_pad37pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 36 function select"]
    #[inline] pub fn pad36fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD36FNCSEL != 0"]
    #[inline] pub fn test_pad36fncsel(&self) -> bool {
        self.pad36fncsel() != 0
    }

    #[doc="Sets the PAD36FNCSEL field."]
    #[inline] pub fn set_pad36fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 36 drive strength"]
    #[inline] pub fn pad36strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD36STRNG != 0"]
    #[inline] pub fn test_pad36strng(&self) -> bool {
        self.pad36strng() != 0
    }

    #[doc="Sets the PAD36STRNG field."]
    #[inline] pub fn set_pad36strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 36 input enable"]
    #[inline] pub fn pad36inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD36INPEN != 0"]
    #[inline] pub fn test_pad36inpen(&self) -> bool {
        self.pad36inpen() != 0
    }

    #[doc="Sets the PAD36INPEN field."]
    #[inline] pub fn set_pad36inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 36 pullup enable"]
    #[inline] pub fn pad36pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD36PULL != 0"]
    #[inline] pub fn test_pad36pull(&self) -> bool {
        self.pad36pull() != 0
    }

    #[doc="Sets the PAD36PULL field."]
    #[inline] pub fn set_pad36pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padregj {
    #[inline]
    fn from(other: u32) -> Self {
         Padregj(other)
    }
}

impl ::core::fmt::Display for Padregj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padregj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad39rsel() != 0 { try!(write!(f, " pad39rsel=0x{:x}", self.pad39rsel()))}
        if self.pad39fncsel() != 0 { try!(write!(f, " pad39fncsel=0x{:x}", self.pad39fncsel()))}
        if self.pad39strng() != 0 { try!(write!(f, " pad39strng"))}
        if self.pad39inpen() != 0 { try!(write!(f, " pad39inpen"))}
        if self.pad39pull() != 0 { try!(write!(f, " pad39pull"))}
        if self.pad38fncsel() != 0 { try!(write!(f, " pad38fncsel=0x{:x}", self.pad38fncsel()))}
        if self.pad38strng() != 0 { try!(write!(f, " pad38strng"))}
        if self.pad38inpen() != 0 { try!(write!(f, " pad38inpen"))}
        if self.pad38pull() != 0 { try!(write!(f, " pad38pull"))}
        if self.pad37fncsel() != 0 { try!(write!(f, " pad37fncsel=0x{:x}", self.pad37fncsel()))}
        if self.pad37strng() != 0 { try!(write!(f, " pad37strng"))}
        if self.pad37inpen() != 0 { try!(write!(f, " pad37inpen"))}
        if self.pad37pull() != 0 { try!(write!(f, " pad37pull"))}
        if self.pad36fncsel() != 0 { try!(write!(f, " pad36fncsel=0x{:x}", self.pad36fncsel()))}
        if self.pad36strng() != 0 { try!(write!(f, " pad36strng"))}
        if self.pad36inpen() != 0 { try!(write!(f, " pad36inpen"))}
        if self.pad36pull() != 0 { try!(write!(f, " pad36pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register K"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padregk(pub u32);
impl Padregk {
    #[doc="Pad 43 pullup resistor selection."]
    #[inline] pub fn pad43rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if PAD43RSEL != 0"]
    #[inline] pub fn test_pad43rsel(&self) -> bool {
        self.pad43rsel() != 0
    }

    #[doc="Sets the PAD43RSEL field."]
    #[inline] pub fn set_pad43rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Pad 43 function select"]
    #[inline] pub fn pad43fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD43FNCSEL != 0"]
    #[inline] pub fn test_pad43fncsel(&self) -> bool {
        self.pad43fncsel() != 0
    }

    #[doc="Sets the PAD43FNCSEL field."]
    #[inline] pub fn set_pad43fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 43 drive strentgh"]
    #[inline] pub fn pad43strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD43STRNG != 0"]
    #[inline] pub fn test_pad43strng(&self) -> bool {
        self.pad43strng() != 0
    }

    #[doc="Sets the PAD43STRNG field."]
    #[inline] pub fn set_pad43strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 43 input enable"]
    #[inline] pub fn pad43inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD43INPEN != 0"]
    #[inline] pub fn test_pad43inpen(&self) -> bool {
        self.pad43inpen() != 0
    }

    #[doc="Sets the PAD43INPEN field."]
    #[inline] pub fn set_pad43inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 43 pullup enable"]
    #[inline] pub fn pad43pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD43PULL != 0"]
    #[inline] pub fn test_pad43pull(&self) -> bool {
        self.pad43pull() != 0
    }

    #[doc="Sets the PAD43PULL field."]
    #[inline] pub fn set_pad43pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 42 pullup resistor selection."]
    #[inline] pub fn pad42rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if PAD42RSEL != 0"]
    #[inline] pub fn test_pad42rsel(&self) -> bool {
        self.pad42rsel() != 0
    }

    #[doc="Sets the PAD42RSEL field."]
    #[inline] pub fn set_pad42rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Pad 42 function select"]
    #[inline] pub fn pad42fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD42FNCSEL != 0"]
    #[inline] pub fn test_pad42fncsel(&self) -> bool {
        self.pad42fncsel() != 0
    }

    #[doc="Sets the PAD42FNCSEL field."]
    #[inline] pub fn set_pad42fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 42 drive strength"]
    #[inline] pub fn pad42strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD42STRNG != 0"]
    #[inline] pub fn test_pad42strng(&self) -> bool {
        self.pad42strng() != 0
    }

    #[doc="Sets the PAD42STRNG field."]
    #[inline] pub fn set_pad42strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 42 input enable"]
    #[inline] pub fn pad42inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD42INPEN != 0"]
    #[inline] pub fn test_pad42inpen(&self) -> bool {
        self.pad42inpen() != 0
    }

    #[doc="Sets the PAD42INPEN field."]
    #[inline] pub fn set_pad42inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 42 pullup enable"]
    #[inline] pub fn pad42pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD42PULL != 0"]
    #[inline] pub fn test_pad42pull(&self) -> bool {
        self.pad42pull() != 0
    }

    #[doc="Sets the PAD42PULL field."]
    #[inline] pub fn set_pad42pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 41 upper power switch enable"]
    #[inline] pub fn pad41pwrup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PAD41PWRUP != 0"]
    #[inline] pub fn test_pad41pwrup(&self) -> bool {
        self.pad41pwrup() != 0
    }

    #[doc="Sets the PAD41PWRUP field."]
    #[inline] pub fn set_pad41pwrup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Pad 41 function select"]
    #[inline] pub fn pad41fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD41FNCSEL != 0"]
    #[inline] pub fn test_pad41fncsel(&self) -> bool {
        self.pad41fncsel() != 0
    }

    #[doc="Sets the PAD41FNCSEL field."]
    #[inline] pub fn set_pad41fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 41 drive strength"]
    #[inline] pub fn pad41strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD41STRNG != 0"]
    #[inline] pub fn test_pad41strng(&self) -> bool {
        self.pad41strng() != 0
    }

    #[doc="Sets the PAD41STRNG field."]
    #[inline] pub fn set_pad41strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 41 input enable"]
    #[inline] pub fn pad41inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD41INPEN != 0"]
    #[inline] pub fn test_pad41inpen(&self) -> bool {
        self.pad41inpen() != 0
    }

    #[doc="Sets the PAD41INPEN field."]
    #[inline] pub fn set_pad41inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 41 pullup enable"]
    #[inline] pub fn pad41pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD41PULL != 0"]
    #[inline] pub fn test_pad41pull(&self) -> bool {
        self.pad41pull() != 0
    }

    #[doc="Sets the PAD41PULL field."]
    #[inline] pub fn set_pad41pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 40 pullup resistor selection."]
    #[inline] pub fn pad40rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if PAD40RSEL != 0"]
    #[inline] pub fn test_pad40rsel(&self) -> bool {
        self.pad40rsel() != 0
    }

    #[doc="Sets the PAD40RSEL field."]
    #[inline] pub fn set_pad40rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pad 40 function select"]
    #[inline] pub fn pad40fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD40FNCSEL != 0"]
    #[inline] pub fn test_pad40fncsel(&self) -> bool {
        self.pad40fncsel() != 0
    }

    #[doc="Sets the PAD40FNCSEL field."]
    #[inline] pub fn set_pad40fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 40 drive strength"]
    #[inline] pub fn pad40strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD40STRNG != 0"]
    #[inline] pub fn test_pad40strng(&self) -> bool {
        self.pad40strng() != 0
    }

    #[doc="Sets the PAD40STRNG field."]
    #[inline] pub fn set_pad40strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 40 input enable"]
    #[inline] pub fn pad40inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD40INPEN != 0"]
    #[inline] pub fn test_pad40inpen(&self) -> bool {
        self.pad40inpen() != 0
    }

    #[doc="Sets the PAD40INPEN field."]
    #[inline] pub fn set_pad40inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 40 pullup enable"]
    #[inline] pub fn pad40pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD40PULL != 0"]
    #[inline] pub fn test_pad40pull(&self) -> bool {
        self.pad40pull() != 0
    }

    #[doc="Sets the PAD40PULL field."]
    #[inline] pub fn set_pad40pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padregk {
    #[inline]
    fn from(other: u32) -> Self {
         Padregk(other)
    }
}

impl ::core::fmt::Display for Padregk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padregk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad43rsel() != 0 { try!(write!(f, " pad43rsel=0x{:x}", self.pad43rsel()))}
        if self.pad43fncsel() != 0 { try!(write!(f, " pad43fncsel=0x{:x}", self.pad43fncsel()))}
        if self.pad43strng() != 0 { try!(write!(f, " pad43strng"))}
        if self.pad43inpen() != 0 { try!(write!(f, " pad43inpen"))}
        if self.pad43pull() != 0 { try!(write!(f, " pad43pull"))}
        if self.pad42rsel() != 0 { try!(write!(f, " pad42rsel=0x{:x}", self.pad42rsel()))}
        if self.pad42fncsel() != 0 { try!(write!(f, " pad42fncsel=0x{:x}", self.pad42fncsel()))}
        if self.pad42strng() != 0 { try!(write!(f, " pad42strng"))}
        if self.pad42inpen() != 0 { try!(write!(f, " pad42inpen"))}
        if self.pad42pull() != 0 { try!(write!(f, " pad42pull"))}
        if self.pad41pwrup() != 0 { try!(write!(f, " pad41pwrup"))}
        if self.pad41fncsel() != 0 { try!(write!(f, " pad41fncsel=0x{:x}", self.pad41fncsel()))}
        if self.pad41strng() != 0 { try!(write!(f, " pad41strng"))}
        if self.pad41inpen() != 0 { try!(write!(f, " pad41inpen"))}
        if self.pad41pull() != 0 { try!(write!(f, " pad41pull"))}
        if self.pad40rsel() != 0 { try!(write!(f, " pad40rsel=0x{:x}", self.pad40rsel()))}
        if self.pad40fncsel() != 0 { try!(write!(f, " pad40fncsel=0x{:x}", self.pad40fncsel()))}
        if self.pad40strng() != 0 { try!(write!(f, " pad40strng"))}
        if self.pad40inpen() != 0 { try!(write!(f, " pad40inpen"))}
        if self.pad40pull() != 0 { try!(write!(f, " pad40pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register L"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padregl(pub u32);
impl Padregl {
    #[doc="Pad 47 function select"]
    #[inline] pub fn pad47fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PAD47FNCSEL != 0"]
    #[inline] pub fn test_pad47fncsel(&self) -> bool {
        self.pad47fncsel() != 0
    }

    #[doc="Sets the PAD47FNCSEL field."]
    #[inline] pub fn set_pad47fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pad 47 drive strentgh"]
    #[inline] pub fn pad47strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PAD47STRNG != 0"]
    #[inline] pub fn test_pad47strng(&self) -> bool {
        self.pad47strng() != 0
    }

    #[doc="Sets the PAD47STRNG field."]
    #[inline] pub fn set_pad47strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pad 47 input enable"]
    #[inline] pub fn pad47inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PAD47INPEN != 0"]
    #[inline] pub fn test_pad47inpen(&self) -> bool {
        self.pad47inpen() != 0
    }

    #[doc="Sets the PAD47INPEN field."]
    #[inline] pub fn set_pad47inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pad 47 pullup enable"]
    #[inline] pub fn pad47pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD47PULL != 0"]
    #[inline] pub fn test_pad47pull(&self) -> bool {
        self.pad47pull() != 0
    }

    #[doc="Sets the PAD47PULL field."]
    #[inline] pub fn set_pad47pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 46 function select"]
    #[inline] pub fn pad46fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if PAD46FNCSEL != 0"]
    #[inline] pub fn test_pad46fncsel(&self) -> bool {
        self.pad46fncsel() != 0
    }

    #[doc="Sets the PAD46FNCSEL field."]
    #[inline] pub fn set_pad46fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pad 46 drive strength"]
    #[inline] pub fn pad46strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PAD46STRNG != 0"]
    #[inline] pub fn test_pad46strng(&self) -> bool {
        self.pad46strng() != 0
    }

    #[doc="Sets the PAD46STRNG field."]
    #[inline] pub fn set_pad46strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pad 46 input enable"]
    #[inline] pub fn pad46inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PAD46INPEN != 0"]
    #[inline] pub fn test_pad46inpen(&self) -> bool {
        self.pad46inpen() != 0
    }

    #[doc="Sets the PAD46INPEN field."]
    #[inline] pub fn set_pad46inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pad 46 pullup enable"]
    #[inline] pub fn pad46pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD46PULL != 0"]
    #[inline] pub fn test_pad46pull(&self) -> bool {
        self.pad46pull() != 0
    }

    #[doc="Sets the PAD46PULL field."]
    #[inline] pub fn set_pad46pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 45 function select"]
    #[inline] pub fn pad45fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD45FNCSEL != 0"]
    #[inline] pub fn test_pad45fncsel(&self) -> bool {
        self.pad45fncsel() != 0
    }

    #[doc="Sets the PAD45FNCSEL field."]
    #[inline] pub fn set_pad45fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 45 drive strength"]
    #[inline] pub fn pad45strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD45STRNG != 0"]
    #[inline] pub fn test_pad45strng(&self) -> bool {
        self.pad45strng() != 0
    }

    #[doc="Sets the PAD45STRNG field."]
    #[inline] pub fn set_pad45strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 45 input enable"]
    #[inline] pub fn pad45inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD45INPEN != 0"]
    #[inline] pub fn test_pad45inpen(&self) -> bool {
        self.pad45inpen() != 0
    }

    #[doc="Sets the PAD45INPEN field."]
    #[inline] pub fn set_pad45inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 45 pullup enable"]
    #[inline] pub fn pad45pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD45PULL != 0"]
    #[inline] pub fn test_pad45pull(&self) -> bool {
        self.pad45pull() != 0
    }

    #[doc="Sets the PAD45PULL field."]
    #[inline] pub fn set_pad45pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 44 function select"]
    #[inline] pub fn pad44fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD44FNCSEL != 0"]
    #[inline] pub fn test_pad44fncsel(&self) -> bool {
        self.pad44fncsel() != 0
    }

    #[doc="Sets the PAD44FNCSEL field."]
    #[inline] pub fn set_pad44fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 44 drive strength"]
    #[inline] pub fn pad44strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD44STRNG != 0"]
    #[inline] pub fn test_pad44strng(&self) -> bool {
        self.pad44strng() != 0
    }

    #[doc="Sets the PAD44STRNG field."]
    #[inline] pub fn set_pad44strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 44 input enable"]
    #[inline] pub fn pad44inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD44INPEN != 0"]
    #[inline] pub fn test_pad44inpen(&self) -> bool {
        self.pad44inpen() != 0
    }

    #[doc="Sets the PAD44INPEN field."]
    #[inline] pub fn set_pad44inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 44 pullup enable"]
    #[inline] pub fn pad44pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD44PULL != 0"]
    #[inline] pub fn test_pad44pull(&self) -> bool {
        self.pad44pull() != 0
    }

    #[doc="Sets the PAD44PULL field."]
    #[inline] pub fn set_pad44pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padregl {
    #[inline]
    fn from(other: u32) -> Self {
         Padregl(other)
    }
}

impl ::core::fmt::Display for Padregl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padregl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad47fncsel() != 0 { try!(write!(f, " pad47fncsel=0x{:x}", self.pad47fncsel()))}
        if self.pad47strng() != 0 { try!(write!(f, " pad47strng"))}
        if self.pad47inpen() != 0 { try!(write!(f, " pad47inpen"))}
        if self.pad47pull() != 0 { try!(write!(f, " pad47pull"))}
        if self.pad46fncsel() != 0 { try!(write!(f, " pad46fncsel=0x{:x}", self.pad46fncsel()))}
        if self.pad46strng() != 0 { try!(write!(f, " pad46strng"))}
        if self.pad46inpen() != 0 { try!(write!(f, " pad46inpen"))}
        if self.pad46pull() != 0 { try!(write!(f, " pad46pull"))}
        if self.pad45fncsel() != 0 { try!(write!(f, " pad45fncsel=0x{:x}", self.pad45fncsel()))}
        if self.pad45strng() != 0 { try!(write!(f, " pad45strng"))}
        if self.pad45inpen() != 0 { try!(write!(f, " pad45inpen"))}
        if self.pad45pull() != 0 { try!(write!(f, " pad45pull"))}
        if self.pad44fncsel() != 0 { try!(write!(f, " pad44fncsel=0x{:x}", self.pad44fncsel()))}
        if self.pad44strng() != 0 { try!(write!(f, " pad44strng"))}
        if self.pad44inpen() != 0 { try!(write!(f, " pad44inpen"))}
        if self.pad44pull() != 0 { try!(write!(f, " pad44pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pad Configuration Register M"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padregm(pub u32);
impl Padregm {
    #[doc="Pad 49 pullup resistor selection."]
    #[inline] pub fn pad49rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if PAD49RSEL != 0"]
    #[inline] pub fn test_pad49rsel(&self) -> bool {
        self.pad49rsel() != 0
    }

    #[doc="Sets the PAD49RSEL field."]
    #[inline] pub fn set_pad49rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Pad 49 function select"]
    #[inline] pub fn pad49fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PAD49FNCSEL != 0"]
    #[inline] pub fn test_pad49fncsel(&self) -> bool {
        self.pad49fncsel() != 0
    }

    #[doc="Sets the PAD49FNCSEL field."]
    #[inline] pub fn set_pad49fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pad 49 drive strength"]
    #[inline] pub fn pad49strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PAD49STRNG != 0"]
    #[inline] pub fn test_pad49strng(&self) -> bool {
        self.pad49strng() != 0
    }

    #[doc="Sets the PAD49STRNG field."]
    #[inline] pub fn set_pad49strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pad 49 input enable"]
    #[inline] pub fn pad49inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PAD49INPEN != 0"]
    #[inline] pub fn test_pad49inpen(&self) -> bool {
        self.pad49inpen() != 0
    }

    #[doc="Sets the PAD49INPEN field."]
    #[inline] pub fn set_pad49inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pad 49 pullup enable"]
    #[inline] pub fn pad49pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD49PULL != 0"]
    #[inline] pub fn test_pad49pull(&self) -> bool {
        self.pad49pull() != 0
    }

    #[doc="Sets the PAD49PULL field."]
    #[inline] pub fn set_pad49pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 48 pullup resistor selection."]
    #[inline] pub fn pad48rsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if PAD48RSEL != 0"]
    #[inline] pub fn test_pad48rsel(&self) -> bool {
        self.pad48rsel() != 0
    }

    #[doc="Sets the PAD48RSEL field."]
    #[inline] pub fn set_pad48rsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pad 48 function select"]
    #[inline] pub fn pad48fncsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PAD48FNCSEL != 0"]
    #[inline] pub fn test_pad48fncsel(&self) -> bool {
        self.pad48fncsel() != 0
    }

    #[doc="Sets the PAD48FNCSEL field."]
    #[inline] pub fn set_pad48fncsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pad 48 drive strength"]
    #[inline] pub fn pad48strng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAD48STRNG != 0"]
    #[inline] pub fn test_pad48strng(&self) -> bool {
        self.pad48strng() != 0
    }

    #[doc="Sets the PAD48STRNG field."]
    #[inline] pub fn set_pad48strng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pad 48 input enable"]
    #[inline] pub fn pad48inpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAD48INPEN != 0"]
    #[inline] pub fn test_pad48inpen(&self) -> bool {
        self.pad48inpen() != 0
    }

    #[doc="Sets the PAD48INPEN field."]
    #[inline] pub fn set_pad48inpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pad 48 pullup enable"]
    #[inline] pub fn pad48pull(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD48PULL != 0"]
    #[inline] pub fn test_pad48pull(&self) -> bool {
        self.pad48pull() != 0
    }

    #[doc="Sets the PAD48PULL field."]
    #[inline] pub fn set_pad48pull<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padregm {
    #[inline]
    fn from(other: u32) -> Self {
         Padregm(other)
    }
}

impl ::core::fmt::Display for Padregm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padregm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad49rsel() != 0 { try!(write!(f, " pad49rsel=0x{:x}", self.pad49rsel()))}
        if self.pad49fncsel() != 0 { try!(write!(f, " pad49fncsel=0x{:x}", self.pad49fncsel()))}
        if self.pad49strng() != 0 { try!(write!(f, " pad49strng"))}
        if self.pad49inpen() != 0 { try!(write!(f, " pad49inpen"))}
        if self.pad49pull() != 0 { try!(write!(f, " pad49pull"))}
        if self.pad48rsel() != 0 { try!(write!(f, " pad48rsel=0x{:x}", self.pad48rsel()))}
        if self.pad48fncsel() != 0 { try!(write!(f, " pad48fncsel=0x{:x}", self.pad48fncsel()))}
        if self.pad48strng() != 0 { try!(write!(f, " pad48strng"))}
        if self.pad48inpen() != 0 { try!(write!(f, " pad48inpen"))}
        if self.pad48pull() != 0 { try!(write!(f, " pad48pull"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Configuration Register A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfga(pub u32);
impl Cfga {
    #[doc="GPIO7 interrupt direction."]
    #[inline] pub fn gpio7intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GPIO7INTD != 0"]
    #[inline] pub fn test_gpio7intd(&self) -> bool {
        self.gpio7intd() != 0
    }

    #[doc="Sets the GPIO7INTD field."]
    #[inline] pub fn set_gpio7intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="GPIO7 output configuration."]
    #[inline] pub fn gpio7outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if GPIO7OUTCFG != 0"]
    #[inline] pub fn test_gpio7outcfg(&self) -> bool {
        self.gpio7outcfg() != 0
    }

    #[doc="Sets the GPIO7OUTCFG field."]
    #[inline] pub fn set_gpio7outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="GPIO7 input enable."]
    #[inline] pub fn gpio7incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if GPIO7INCFG != 0"]
    #[inline] pub fn test_gpio7incfg(&self) -> bool {
        self.gpio7incfg() != 0
    }

    #[doc="Sets the GPIO7INCFG field."]
    #[inline] pub fn set_gpio7incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="GPIO6 interrupt direction."]
    #[inline] pub fn gpio6intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if GPIO6INTD != 0"]
    #[inline] pub fn test_gpio6intd(&self) -> bool {
        self.gpio6intd() != 0
    }

    #[doc="Sets the GPIO6INTD field."]
    #[inline] pub fn set_gpio6intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="GPIO6 output configuration."]
    #[inline] pub fn gpio6outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if GPIO6OUTCFG != 0"]
    #[inline] pub fn test_gpio6outcfg(&self) -> bool {
        self.gpio6outcfg() != 0
    }

    #[doc="Sets the GPIO6OUTCFG field."]
    #[inline] pub fn set_gpio6outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="GPIO6 input enable."]
    #[inline] pub fn gpio6incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if GPIO6INCFG != 0"]
    #[inline] pub fn test_gpio6incfg(&self) -> bool {
        self.gpio6incfg() != 0
    }

    #[doc="Sets the GPIO6INCFG field."]
    #[inline] pub fn set_gpio6incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="GPIO5 interrupt direction."]
    #[inline] pub fn gpio5intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if GPIO5INTD != 0"]
    #[inline] pub fn test_gpio5intd(&self) -> bool {
        self.gpio5intd() != 0
    }

    #[doc="Sets the GPIO5INTD field."]
    #[inline] pub fn set_gpio5intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="GPIO5 output configuration."]
    #[inline] pub fn gpio5outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if GPIO5OUTCFG != 0"]
    #[inline] pub fn test_gpio5outcfg(&self) -> bool {
        self.gpio5outcfg() != 0
    }

    #[doc="Sets the GPIO5OUTCFG field."]
    #[inline] pub fn set_gpio5outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="GPIO5 input enable."]
    #[inline] pub fn gpio5incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if GPIO5INCFG != 0"]
    #[inline] pub fn test_gpio5incfg(&self) -> bool {
        self.gpio5incfg() != 0
    }

    #[doc="Sets the GPIO5INCFG field."]
    #[inline] pub fn set_gpio5incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="GPIO4 interrupt direction."]
    #[inline] pub fn gpio4intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GPIO4INTD != 0"]
    #[inline] pub fn test_gpio4intd(&self) -> bool {
        self.gpio4intd() != 0
    }

    #[doc="Sets the GPIO4INTD field."]
    #[inline] pub fn set_gpio4intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="GPIO4 output configuration."]
    #[inline] pub fn gpio4outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if GPIO4OUTCFG != 0"]
    #[inline] pub fn test_gpio4outcfg(&self) -> bool {
        self.gpio4outcfg() != 0
    }

    #[doc="Sets the GPIO4OUTCFG field."]
    #[inline] pub fn set_gpio4outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO4 input enable."]
    #[inline] pub fn gpio4incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO4INCFG != 0"]
    #[inline] pub fn test_gpio4incfg(&self) -> bool {
        self.gpio4incfg() != 0
    }

    #[doc="Sets the GPIO4INCFG field."]
    #[inline] pub fn set_gpio4incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO3 interrupt direction."]
    #[inline] pub fn gpio3intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO3INTD != 0"]
    #[inline] pub fn test_gpio3intd(&self) -> bool {
        self.gpio3intd() != 0
    }

    #[doc="Sets the GPIO3INTD field."]
    #[inline] pub fn set_gpio3intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO3 output configuration."]
    #[inline] pub fn gpio3outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if GPIO3OUTCFG != 0"]
    #[inline] pub fn test_gpio3outcfg(&self) -> bool {
        self.gpio3outcfg() != 0
    }

    #[doc="Sets the GPIO3OUTCFG field."]
    #[inline] pub fn set_gpio3outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO3 input enable."]
    #[inline] pub fn gpio3incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO3INCFG != 0"]
    #[inline] pub fn test_gpio3incfg(&self) -> bool {
        self.gpio3incfg() != 0
    }

    #[doc="Sets the GPIO3INCFG field."]
    #[inline] pub fn set_gpio3incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO2 interrupt direction."]
    #[inline] pub fn gpio2intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO2INTD != 0"]
    #[inline] pub fn test_gpio2intd(&self) -> bool {
        self.gpio2intd() != 0
    }

    #[doc="Sets the GPIO2INTD field."]
    #[inline] pub fn set_gpio2intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO2 output configuration."]
    #[inline] pub fn gpio2outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if GPIO2OUTCFG != 0"]
    #[inline] pub fn test_gpio2outcfg(&self) -> bool {
        self.gpio2outcfg() != 0
    }

    #[doc="Sets the GPIO2OUTCFG field."]
    #[inline] pub fn set_gpio2outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO2 input enable."]
    #[inline] pub fn gpio2incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO2INCFG != 0"]
    #[inline] pub fn test_gpio2incfg(&self) -> bool {
        self.gpio2incfg() != 0
    }

    #[doc="Sets the GPIO2INCFG field."]
    #[inline] pub fn set_gpio2incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO1 interrupt direction."]
    #[inline] pub fn gpio1intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO1INTD != 0"]
    #[inline] pub fn test_gpio1intd(&self) -> bool {
        self.gpio1intd() != 0
    }

    #[doc="Sets the GPIO1INTD field."]
    #[inline] pub fn set_gpio1intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO1 output configuration."]
    #[inline] pub fn gpio1outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if GPIO1OUTCFG != 0"]
    #[inline] pub fn test_gpio1outcfg(&self) -> bool {
        self.gpio1outcfg() != 0
    }

    #[doc="Sets the GPIO1OUTCFG field."]
    #[inline] pub fn set_gpio1outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO1 input enable."]
    #[inline] pub fn gpio1incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO1INCFG != 0"]
    #[inline] pub fn test_gpio1incfg(&self) -> bool {
        self.gpio1incfg() != 0
    }

    #[doc="Sets the GPIO1INCFG field."]
    #[inline] pub fn set_gpio1incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO0 interrupt direction."]
    #[inline] pub fn gpio0intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO0INTD != 0"]
    #[inline] pub fn test_gpio0intd(&self) -> bool {
        self.gpio0intd() != 0
    }

    #[doc="Sets the GPIO0INTD field."]
    #[inline] pub fn set_gpio0intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO0 output configuration."]
    #[inline] pub fn gpio0outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if GPIO0OUTCFG != 0"]
    #[inline] pub fn test_gpio0outcfg(&self) -> bool {
        self.gpio0outcfg() != 0
    }

    #[doc="Sets the GPIO0OUTCFG field."]
    #[inline] pub fn set_gpio0outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO0 input enable."]
    #[inline] pub fn gpio0incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO0INCFG != 0"]
    #[inline] pub fn test_gpio0incfg(&self) -> bool {
        self.gpio0incfg() != 0
    }

    #[doc="Sets the GPIO0INCFG field."]
    #[inline] pub fn set_gpio0incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfga {
    #[inline]
    fn from(other: u32) -> Self {
         Cfga(other)
    }
}

impl ::core::fmt::Display for Cfga {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfga {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio7intd() != 0 { try!(write!(f, " gpio7intd"))}
        if self.gpio7outcfg() != 0 { try!(write!(f, " gpio7outcfg=0x{:x}", self.gpio7outcfg()))}
        if self.gpio7incfg() != 0 { try!(write!(f, " gpio7incfg"))}
        if self.gpio6intd() != 0 { try!(write!(f, " gpio6intd"))}
        if self.gpio6outcfg() != 0 { try!(write!(f, " gpio6outcfg=0x{:x}", self.gpio6outcfg()))}
        if self.gpio6incfg() != 0 { try!(write!(f, " gpio6incfg"))}
        if self.gpio5intd() != 0 { try!(write!(f, " gpio5intd"))}
        if self.gpio5outcfg() != 0 { try!(write!(f, " gpio5outcfg=0x{:x}", self.gpio5outcfg()))}
        if self.gpio5incfg() != 0 { try!(write!(f, " gpio5incfg"))}
        if self.gpio4intd() != 0 { try!(write!(f, " gpio4intd"))}
        if self.gpio4outcfg() != 0 { try!(write!(f, " gpio4outcfg=0x{:x}", self.gpio4outcfg()))}
        if self.gpio4incfg() != 0 { try!(write!(f, " gpio4incfg"))}
        if self.gpio3intd() != 0 { try!(write!(f, " gpio3intd"))}
        if self.gpio3outcfg() != 0 { try!(write!(f, " gpio3outcfg=0x{:x}", self.gpio3outcfg()))}
        if self.gpio3incfg() != 0 { try!(write!(f, " gpio3incfg"))}
        if self.gpio2intd() != 0 { try!(write!(f, " gpio2intd"))}
        if self.gpio2outcfg() != 0 { try!(write!(f, " gpio2outcfg=0x{:x}", self.gpio2outcfg()))}
        if self.gpio2incfg() != 0 { try!(write!(f, " gpio2incfg"))}
        if self.gpio1intd() != 0 { try!(write!(f, " gpio1intd"))}
        if self.gpio1outcfg() != 0 { try!(write!(f, " gpio1outcfg=0x{:x}", self.gpio1outcfg()))}
        if self.gpio1incfg() != 0 { try!(write!(f, " gpio1incfg"))}
        if self.gpio0intd() != 0 { try!(write!(f, " gpio0intd"))}
        if self.gpio0outcfg() != 0 { try!(write!(f, " gpio0outcfg=0x{:x}", self.gpio0outcfg()))}
        if self.gpio0incfg() != 0 { try!(write!(f, " gpio0incfg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Configuration Register B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgb(pub u32);
impl Cfgb {
    #[doc="GPIO15 interrupt direction."]
    #[inline] pub fn gpio15intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GPIO15INTD != 0"]
    #[inline] pub fn test_gpio15intd(&self) -> bool {
        self.gpio15intd() != 0
    }

    #[doc="Sets the GPIO15INTD field."]
    #[inline] pub fn set_gpio15intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="GPIO15 output configuration."]
    #[inline] pub fn gpio15outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if GPIO15OUTCFG != 0"]
    #[inline] pub fn test_gpio15outcfg(&self) -> bool {
        self.gpio15outcfg() != 0
    }

    #[doc="Sets the GPIO15OUTCFG field."]
    #[inline] pub fn set_gpio15outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="GPIO15 input enable."]
    #[inline] pub fn gpio15incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if GPIO15INCFG != 0"]
    #[inline] pub fn test_gpio15incfg(&self) -> bool {
        self.gpio15incfg() != 0
    }

    #[doc="Sets the GPIO15INCFG field."]
    #[inline] pub fn set_gpio15incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="GPIO14 interrupt direction."]
    #[inline] pub fn gpio14intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if GPIO14INTD != 0"]
    #[inline] pub fn test_gpio14intd(&self) -> bool {
        self.gpio14intd() != 0
    }

    #[doc="Sets the GPIO14INTD field."]
    #[inline] pub fn set_gpio14intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="GPIO14 output configuration."]
    #[inline] pub fn gpio14outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if GPIO14OUTCFG != 0"]
    #[inline] pub fn test_gpio14outcfg(&self) -> bool {
        self.gpio14outcfg() != 0
    }

    #[doc="Sets the GPIO14OUTCFG field."]
    #[inline] pub fn set_gpio14outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="GPIO14 input enable."]
    #[inline] pub fn gpio14incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if GPIO14INCFG != 0"]
    #[inline] pub fn test_gpio14incfg(&self) -> bool {
        self.gpio14incfg() != 0
    }

    #[doc="Sets the GPIO14INCFG field."]
    #[inline] pub fn set_gpio14incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="GPIO13 interrupt direction."]
    #[inline] pub fn gpio13intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if GPIO13INTD != 0"]
    #[inline] pub fn test_gpio13intd(&self) -> bool {
        self.gpio13intd() != 0
    }

    #[doc="Sets the GPIO13INTD field."]
    #[inline] pub fn set_gpio13intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="GPIO13 output configuration."]
    #[inline] pub fn gpio13outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if GPIO13OUTCFG != 0"]
    #[inline] pub fn test_gpio13outcfg(&self) -> bool {
        self.gpio13outcfg() != 0
    }

    #[doc="Sets the GPIO13OUTCFG field."]
    #[inline] pub fn set_gpio13outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="GPIO13 input enable."]
    #[inline] pub fn gpio13incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if GPIO13INCFG != 0"]
    #[inline] pub fn test_gpio13incfg(&self) -> bool {
        self.gpio13incfg() != 0
    }

    #[doc="Sets the GPIO13INCFG field."]
    #[inline] pub fn set_gpio13incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="GPIO12 interrupt direction."]
    #[inline] pub fn gpio12intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GPIO12INTD != 0"]
    #[inline] pub fn test_gpio12intd(&self) -> bool {
        self.gpio12intd() != 0
    }

    #[doc="Sets the GPIO12INTD field."]
    #[inline] pub fn set_gpio12intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="GPIO12 output configuration."]
    #[inline] pub fn gpio12outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if GPIO12OUTCFG != 0"]
    #[inline] pub fn test_gpio12outcfg(&self) -> bool {
        self.gpio12outcfg() != 0
    }

    #[doc="Sets the GPIO12OUTCFG field."]
    #[inline] pub fn set_gpio12outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO12 input enable."]
    #[inline] pub fn gpio12incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO12INCFG != 0"]
    #[inline] pub fn test_gpio12incfg(&self) -> bool {
        self.gpio12incfg() != 0
    }

    #[doc="Sets the GPIO12INCFG field."]
    #[inline] pub fn set_gpio12incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO11 interrupt direction."]
    #[inline] pub fn gpio11intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO11INTD != 0"]
    #[inline] pub fn test_gpio11intd(&self) -> bool {
        self.gpio11intd() != 0
    }

    #[doc="Sets the GPIO11INTD field."]
    #[inline] pub fn set_gpio11intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO11 output configuration."]
    #[inline] pub fn gpio11outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if GPIO11OUTCFG != 0"]
    #[inline] pub fn test_gpio11outcfg(&self) -> bool {
        self.gpio11outcfg() != 0
    }

    #[doc="Sets the GPIO11OUTCFG field."]
    #[inline] pub fn set_gpio11outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO11 input enable."]
    #[inline] pub fn gpio11incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO11INCFG != 0"]
    #[inline] pub fn test_gpio11incfg(&self) -> bool {
        self.gpio11incfg() != 0
    }

    #[doc="Sets the GPIO11INCFG field."]
    #[inline] pub fn set_gpio11incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO10 interrupt direction."]
    #[inline] pub fn gpio10intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO10INTD != 0"]
    #[inline] pub fn test_gpio10intd(&self) -> bool {
        self.gpio10intd() != 0
    }

    #[doc="Sets the GPIO10INTD field."]
    #[inline] pub fn set_gpio10intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO10 output configuration."]
    #[inline] pub fn gpio10outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if GPIO10OUTCFG != 0"]
    #[inline] pub fn test_gpio10outcfg(&self) -> bool {
        self.gpio10outcfg() != 0
    }

    #[doc="Sets the GPIO10OUTCFG field."]
    #[inline] pub fn set_gpio10outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO10 input enable."]
    #[inline] pub fn gpio10incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO10INCFG != 0"]
    #[inline] pub fn test_gpio10incfg(&self) -> bool {
        self.gpio10incfg() != 0
    }

    #[doc="Sets the GPIO10INCFG field."]
    #[inline] pub fn set_gpio10incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO9 interrupt direction."]
    #[inline] pub fn gpio9intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO9INTD != 0"]
    #[inline] pub fn test_gpio9intd(&self) -> bool {
        self.gpio9intd() != 0
    }

    #[doc="Sets the GPIO9INTD field."]
    #[inline] pub fn set_gpio9intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO9 output configuration."]
    #[inline] pub fn gpio9outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if GPIO9OUTCFG != 0"]
    #[inline] pub fn test_gpio9outcfg(&self) -> bool {
        self.gpio9outcfg() != 0
    }

    #[doc="Sets the GPIO9OUTCFG field."]
    #[inline] pub fn set_gpio9outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO9 input enable."]
    #[inline] pub fn gpio9incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO9INCFG != 0"]
    #[inline] pub fn test_gpio9incfg(&self) -> bool {
        self.gpio9incfg() != 0
    }

    #[doc="Sets the GPIO9INCFG field."]
    #[inline] pub fn set_gpio9incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO8 interrupt direction."]
    #[inline] pub fn gpio8intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO8INTD != 0"]
    #[inline] pub fn test_gpio8intd(&self) -> bool {
        self.gpio8intd() != 0
    }

    #[doc="Sets the GPIO8INTD field."]
    #[inline] pub fn set_gpio8intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO8 output configuration."]
    #[inline] pub fn gpio8outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if GPIO8OUTCFG != 0"]
    #[inline] pub fn test_gpio8outcfg(&self) -> bool {
        self.gpio8outcfg() != 0
    }

    #[doc="Sets the GPIO8OUTCFG field."]
    #[inline] pub fn set_gpio8outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO8 input enable."]
    #[inline] pub fn gpio8incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO8INCFG != 0"]
    #[inline] pub fn test_gpio8incfg(&self) -> bool {
        self.gpio8incfg() != 0
    }

    #[doc="Sets the GPIO8INCFG field."]
    #[inline] pub fn set_gpio8incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgb {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgb(other)
    }
}

impl ::core::fmt::Display for Cfgb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio15intd() != 0 { try!(write!(f, " gpio15intd"))}
        if self.gpio15outcfg() != 0 { try!(write!(f, " gpio15outcfg=0x{:x}", self.gpio15outcfg()))}
        if self.gpio15incfg() != 0 { try!(write!(f, " gpio15incfg"))}
        if self.gpio14intd() != 0 { try!(write!(f, " gpio14intd"))}
        if self.gpio14outcfg() != 0 { try!(write!(f, " gpio14outcfg=0x{:x}", self.gpio14outcfg()))}
        if self.gpio14incfg() != 0 { try!(write!(f, " gpio14incfg"))}
        if self.gpio13intd() != 0 { try!(write!(f, " gpio13intd"))}
        if self.gpio13outcfg() != 0 { try!(write!(f, " gpio13outcfg=0x{:x}", self.gpio13outcfg()))}
        if self.gpio13incfg() != 0 { try!(write!(f, " gpio13incfg"))}
        if self.gpio12intd() != 0 { try!(write!(f, " gpio12intd"))}
        if self.gpio12outcfg() != 0 { try!(write!(f, " gpio12outcfg=0x{:x}", self.gpio12outcfg()))}
        if self.gpio12incfg() != 0 { try!(write!(f, " gpio12incfg"))}
        if self.gpio11intd() != 0 { try!(write!(f, " gpio11intd"))}
        if self.gpio11outcfg() != 0 { try!(write!(f, " gpio11outcfg=0x{:x}", self.gpio11outcfg()))}
        if self.gpio11incfg() != 0 { try!(write!(f, " gpio11incfg"))}
        if self.gpio10intd() != 0 { try!(write!(f, " gpio10intd"))}
        if self.gpio10outcfg() != 0 { try!(write!(f, " gpio10outcfg=0x{:x}", self.gpio10outcfg()))}
        if self.gpio10incfg() != 0 { try!(write!(f, " gpio10incfg"))}
        if self.gpio9intd() != 0 { try!(write!(f, " gpio9intd"))}
        if self.gpio9outcfg() != 0 { try!(write!(f, " gpio9outcfg=0x{:x}", self.gpio9outcfg()))}
        if self.gpio9incfg() != 0 { try!(write!(f, " gpio9incfg"))}
        if self.gpio8intd() != 0 { try!(write!(f, " gpio8intd"))}
        if self.gpio8outcfg() != 0 { try!(write!(f, " gpio8outcfg=0x{:x}", self.gpio8outcfg()))}
        if self.gpio8incfg() != 0 { try!(write!(f, " gpio8incfg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Configuration Register C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgc(pub u32);
impl Cfgc {
    #[doc="GPIO23 interrupt direction."]
    #[inline] pub fn gpio23intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GPIO23INTD != 0"]
    #[inline] pub fn test_gpio23intd(&self) -> bool {
        self.gpio23intd() != 0
    }

    #[doc="Sets the GPIO23INTD field."]
    #[inline] pub fn set_gpio23intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="GPIO23 output configuration."]
    #[inline] pub fn gpio23outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if GPIO23OUTCFG != 0"]
    #[inline] pub fn test_gpio23outcfg(&self) -> bool {
        self.gpio23outcfg() != 0
    }

    #[doc="Sets the GPIO23OUTCFG field."]
    #[inline] pub fn set_gpio23outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="GPIO23 input enable."]
    #[inline] pub fn gpio23incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if GPIO23INCFG != 0"]
    #[inline] pub fn test_gpio23incfg(&self) -> bool {
        self.gpio23incfg() != 0
    }

    #[doc="Sets the GPIO23INCFG field."]
    #[inline] pub fn set_gpio23incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="GPIO22 interrupt direction."]
    #[inline] pub fn gpio22intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if GPIO22INTD != 0"]
    #[inline] pub fn test_gpio22intd(&self) -> bool {
        self.gpio22intd() != 0
    }

    #[doc="Sets the GPIO22INTD field."]
    #[inline] pub fn set_gpio22intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="GPIO22 output configuration."]
    #[inline] pub fn gpio22outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if GPIO22OUTCFG != 0"]
    #[inline] pub fn test_gpio22outcfg(&self) -> bool {
        self.gpio22outcfg() != 0
    }

    #[doc="Sets the GPIO22OUTCFG field."]
    #[inline] pub fn set_gpio22outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="GPIO22 input enable."]
    #[inline] pub fn gpio22incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if GPIO22INCFG != 0"]
    #[inline] pub fn test_gpio22incfg(&self) -> bool {
        self.gpio22incfg() != 0
    }

    #[doc="Sets the GPIO22INCFG field."]
    #[inline] pub fn set_gpio22incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="GPIO21 interrupt direction."]
    #[inline] pub fn gpio21intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if GPIO21INTD != 0"]
    #[inline] pub fn test_gpio21intd(&self) -> bool {
        self.gpio21intd() != 0
    }

    #[doc="Sets the GPIO21INTD field."]
    #[inline] pub fn set_gpio21intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="GPIO21 output configuration."]
    #[inline] pub fn gpio21outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if GPIO21OUTCFG != 0"]
    #[inline] pub fn test_gpio21outcfg(&self) -> bool {
        self.gpio21outcfg() != 0
    }

    #[doc="Sets the GPIO21OUTCFG field."]
    #[inline] pub fn set_gpio21outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="GPIO21 input enable."]
    #[inline] pub fn gpio21incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if GPIO21INCFG != 0"]
    #[inline] pub fn test_gpio21incfg(&self) -> bool {
        self.gpio21incfg() != 0
    }

    #[doc="Sets the GPIO21INCFG field."]
    #[inline] pub fn set_gpio21incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="GPIO20 interrupt direction."]
    #[inline] pub fn gpio20intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GPIO20INTD != 0"]
    #[inline] pub fn test_gpio20intd(&self) -> bool {
        self.gpio20intd() != 0
    }

    #[doc="Sets the GPIO20INTD field."]
    #[inline] pub fn set_gpio20intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="GPIO20 output configuration."]
    #[inline] pub fn gpio20outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if GPIO20OUTCFG != 0"]
    #[inline] pub fn test_gpio20outcfg(&self) -> bool {
        self.gpio20outcfg() != 0
    }

    #[doc="Sets the GPIO20OUTCFG field."]
    #[inline] pub fn set_gpio20outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO20 input enable."]
    #[inline] pub fn gpio20incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO20INCFG != 0"]
    #[inline] pub fn test_gpio20incfg(&self) -> bool {
        self.gpio20incfg() != 0
    }

    #[doc="Sets the GPIO20INCFG field."]
    #[inline] pub fn set_gpio20incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO19 interrupt direction."]
    #[inline] pub fn gpio19intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO19INTD != 0"]
    #[inline] pub fn test_gpio19intd(&self) -> bool {
        self.gpio19intd() != 0
    }

    #[doc="Sets the GPIO19INTD field."]
    #[inline] pub fn set_gpio19intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO19 output configuration."]
    #[inline] pub fn gpio19outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if GPIO19OUTCFG != 0"]
    #[inline] pub fn test_gpio19outcfg(&self) -> bool {
        self.gpio19outcfg() != 0
    }

    #[doc="Sets the GPIO19OUTCFG field."]
    #[inline] pub fn set_gpio19outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO19 input enable."]
    #[inline] pub fn gpio19incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO19INCFG != 0"]
    #[inline] pub fn test_gpio19incfg(&self) -> bool {
        self.gpio19incfg() != 0
    }

    #[doc="Sets the GPIO19INCFG field."]
    #[inline] pub fn set_gpio19incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO18 interrupt direction."]
    #[inline] pub fn gpio18intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO18INTD != 0"]
    #[inline] pub fn test_gpio18intd(&self) -> bool {
        self.gpio18intd() != 0
    }

    #[doc="Sets the GPIO18INTD field."]
    #[inline] pub fn set_gpio18intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO18 output configuration."]
    #[inline] pub fn gpio18outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if GPIO18OUTCFG != 0"]
    #[inline] pub fn test_gpio18outcfg(&self) -> bool {
        self.gpio18outcfg() != 0
    }

    #[doc="Sets the GPIO18OUTCFG field."]
    #[inline] pub fn set_gpio18outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO18 input enable."]
    #[inline] pub fn gpio18incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO18INCFG != 0"]
    #[inline] pub fn test_gpio18incfg(&self) -> bool {
        self.gpio18incfg() != 0
    }

    #[doc="Sets the GPIO18INCFG field."]
    #[inline] pub fn set_gpio18incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO17 interrupt direction."]
    #[inline] pub fn gpio17intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO17INTD != 0"]
    #[inline] pub fn test_gpio17intd(&self) -> bool {
        self.gpio17intd() != 0
    }

    #[doc="Sets the GPIO17INTD field."]
    #[inline] pub fn set_gpio17intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO17 output configuration."]
    #[inline] pub fn gpio17outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if GPIO17OUTCFG != 0"]
    #[inline] pub fn test_gpio17outcfg(&self) -> bool {
        self.gpio17outcfg() != 0
    }

    #[doc="Sets the GPIO17OUTCFG field."]
    #[inline] pub fn set_gpio17outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO17 input enable."]
    #[inline] pub fn gpio17incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO17INCFG != 0"]
    #[inline] pub fn test_gpio17incfg(&self) -> bool {
        self.gpio17incfg() != 0
    }

    #[doc="Sets the GPIO17INCFG field."]
    #[inline] pub fn set_gpio17incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO16 interrupt direction."]
    #[inline] pub fn gpio16intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO16INTD != 0"]
    #[inline] pub fn test_gpio16intd(&self) -> bool {
        self.gpio16intd() != 0
    }

    #[doc="Sets the GPIO16INTD field."]
    #[inline] pub fn set_gpio16intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO16 output configuration."]
    #[inline] pub fn gpio16outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if GPIO16OUTCFG != 0"]
    #[inline] pub fn test_gpio16outcfg(&self) -> bool {
        self.gpio16outcfg() != 0
    }

    #[doc="Sets the GPIO16OUTCFG field."]
    #[inline] pub fn set_gpio16outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO16 input enable."]
    #[inline] pub fn gpio16incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO16INCFG != 0"]
    #[inline] pub fn test_gpio16incfg(&self) -> bool {
        self.gpio16incfg() != 0
    }

    #[doc="Sets the GPIO16INCFG field."]
    #[inline] pub fn set_gpio16incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgc {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgc(other)
    }
}

impl ::core::fmt::Display for Cfgc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio23intd() != 0 { try!(write!(f, " gpio23intd"))}
        if self.gpio23outcfg() != 0 { try!(write!(f, " gpio23outcfg=0x{:x}", self.gpio23outcfg()))}
        if self.gpio23incfg() != 0 { try!(write!(f, " gpio23incfg"))}
        if self.gpio22intd() != 0 { try!(write!(f, " gpio22intd"))}
        if self.gpio22outcfg() != 0 { try!(write!(f, " gpio22outcfg=0x{:x}", self.gpio22outcfg()))}
        if self.gpio22incfg() != 0 { try!(write!(f, " gpio22incfg"))}
        if self.gpio21intd() != 0 { try!(write!(f, " gpio21intd"))}
        if self.gpio21outcfg() != 0 { try!(write!(f, " gpio21outcfg=0x{:x}", self.gpio21outcfg()))}
        if self.gpio21incfg() != 0 { try!(write!(f, " gpio21incfg"))}
        if self.gpio20intd() != 0 { try!(write!(f, " gpio20intd"))}
        if self.gpio20outcfg() != 0 { try!(write!(f, " gpio20outcfg=0x{:x}", self.gpio20outcfg()))}
        if self.gpio20incfg() != 0 { try!(write!(f, " gpio20incfg"))}
        if self.gpio19intd() != 0 { try!(write!(f, " gpio19intd"))}
        if self.gpio19outcfg() != 0 { try!(write!(f, " gpio19outcfg=0x{:x}", self.gpio19outcfg()))}
        if self.gpio19incfg() != 0 { try!(write!(f, " gpio19incfg"))}
        if self.gpio18intd() != 0 { try!(write!(f, " gpio18intd"))}
        if self.gpio18outcfg() != 0 { try!(write!(f, " gpio18outcfg=0x{:x}", self.gpio18outcfg()))}
        if self.gpio18incfg() != 0 { try!(write!(f, " gpio18incfg"))}
        if self.gpio17intd() != 0 { try!(write!(f, " gpio17intd"))}
        if self.gpio17outcfg() != 0 { try!(write!(f, " gpio17outcfg=0x{:x}", self.gpio17outcfg()))}
        if self.gpio17incfg() != 0 { try!(write!(f, " gpio17incfg"))}
        if self.gpio16intd() != 0 { try!(write!(f, " gpio16intd"))}
        if self.gpio16outcfg() != 0 { try!(write!(f, " gpio16outcfg=0x{:x}", self.gpio16outcfg()))}
        if self.gpio16incfg() != 0 { try!(write!(f, " gpio16incfg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Configuration Register D"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgd(pub u32);
impl Cfgd {
    #[doc="GPIO31 interrupt direction."]
    #[inline] pub fn gpio31intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GPIO31INTD != 0"]
    #[inline] pub fn test_gpio31intd(&self) -> bool {
        self.gpio31intd() != 0
    }

    #[doc="Sets the GPIO31INTD field."]
    #[inline] pub fn set_gpio31intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="GPIO31 output configuration."]
    #[inline] pub fn gpio31outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if GPIO31OUTCFG != 0"]
    #[inline] pub fn test_gpio31outcfg(&self) -> bool {
        self.gpio31outcfg() != 0
    }

    #[doc="Sets the GPIO31OUTCFG field."]
    #[inline] pub fn set_gpio31outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="GPIO31 input enable."]
    #[inline] pub fn gpio31incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if GPIO31INCFG != 0"]
    #[inline] pub fn test_gpio31incfg(&self) -> bool {
        self.gpio31incfg() != 0
    }

    #[doc="Sets the GPIO31INCFG field."]
    #[inline] pub fn set_gpio31incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="GPIO30 interrupt direction."]
    #[inline] pub fn gpio30intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if GPIO30INTD != 0"]
    #[inline] pub fn test_gpio30intd(&self) -> bool {
        self.gpio30intd() != 0
    }

    #[doc="Sets the GPIO30INTD field."]
    #[inline] pub fn set_gpio30intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="GPIO30 output configuration."]
    #[inline] pub fn gpio30outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if GPIO30OUTCFG != 0"]
    #[inline] pub fn test_gpio30outcfg(&self) -> bool {
        self.gpio30outcfg() != 0
    }

    #[doc="Sets the GPIO30OUTCFG field."]
    #[inline] pub fn set_gpio30outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="GPIO30 input enable."]
    #[inline] pub fn gpio30incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if GPIO30INCFG != 0"]
    #[inline] pub fn test_gpio30incfg(&self) -> bool {
        self.gpio30incfg() != 0
    }

    #[doc="Sets the GPIO30INCFG field."]
    #[inline] pub fn set_gpio30incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="GPIO29 interrupt direction."]
    #[inline] pub fn gpio29intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if GPIO29INTD != 0"]
    #[inline] pub fn test_gpio29intd(&self) -> bool {
        self.gpio29intd() != 0
    }

    #[doc="Sets the GPIO29INTD field."]
    #[inline] pub fn set_gpio29intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="GPIO29 output configuration."]
    #[inline] pub fn gpio29outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if GPIO29OUTCFG != 0"]
    #[inline] pub fn test_gpio29outcfg(&self) -> bool {
        self.gpio29outcfg() != 0
    }

    #[doc="Sets the GPIO29OUTCFG field."]
    #[inline] pub fn set_gpio29outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="GPIO29 input enable."]
    #[inline] pub fn gpio29incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if GPIO29INCFG != 0"]
    #[inline] pub fn test_gpio29incfg(&self) -> bool {
        self.gpio29incfg() != 0
    }

    #[doc="Sets the GPIO29INCFG field."]
    #[inline] pub fn set_gpio29incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="GPIO28 interrupt direction."]
    #[inline] pub fn gpio28intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GPIO28INTD != 0"]
    #[inline] pub fn test_gpio28intd(&self) -> bool {
        self.gpio28intd() != 0
    }

    #[doc="Sets the GPIO28INTD field."]
    #[inline] pub fn set_gpio28intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="GPIO28 output configuration."]
    #[inline] pub fn gpio28outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if GPIO28OUTCFG != 0"]
    #[inline] pub fn test_gpio28outcfg(&self) -> bool {
        self.gpio28outcfg() != 0
    }

    #[doc="Sets the GPIO28OUTCFG field."]
    #[inline] pub fn set_gpio28outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO28 input enable."]
    #[inline] pub fn gpio28incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO28INCFG != 0"]
    #[inline] pub fn test_gpio28incfg(&self) -> bool {
        self.gpio28incfg() != 0
    }

    #[doc="Sets the GPIO28INCFG field."]
    #[inline] pub fn set_gpio28incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO27 interrupt direction."]
    #[inline] pub fn gpio27intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO27INTD != 0"]
    #[inline] pub fn test_gpio27intd(&self) -> bool {
        self.gpio27intd() != 0
    }

    #[doc="Sets the GPIO27INTD field."]
    #[inline] pub fn set_gpio27intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO27 output configuration."]
    #[inline] pub fn gpio27outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if GPIO27OUTCFG != 0"]
    #[inline] pub fn test_gpio27outcfg(&self) -> bool {
        self.gpio27outcfg() != 0
    }

    #[doc="Sets the GPIO27OUTCFG field."]
    #[inline] pub fn set_gpio27outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO27 input enable."]
    #[inline] pub fn gpio27incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO27INCFG != 0"]
    #[inline] pub fn test_gpio27incfg(&self) -> bool {
        self.gpio27incfg() != 0
    }

    #[doc="Sets the GPIO27INCFG field."]
    #[inline] pub fn set_gpio27incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO26 interrupt direction."]
    #[inline] pub fn gpio26intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO26INTD != 0"]
    #[inline] pub fn test_gpio26intd(&self) -> bool {
        self.gpio26intd() != 0
    }

    #[doc="Sets the GPIO26INTD field."]
    #[inline] pub fn set_gpio26intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO26 output configuration."]
    #[inline] pub fn gpio26outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if GPIO26OUTCFG != 0"]
    #[inline] pub fn test_gpio26outcfg(&self) -> bool {
        self.gpio26outcfg() != 0
    }

    #[doc="Sets the GPIO26OUTCFG field."]
    #[inline] pub fn set_gpio26outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO26 input enable."]
    #[inline] pub fn gpio26incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO26INCFG != 0"]
    #[inline] pub fn test_gpio26incfg(&self) -> bool {
        self.gpio26incfg() != 0
    }

    #[doc="Sets the GPIO26INCFG field."]
    #[inline] pub fn set_gpio26incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO25 interrupt direction."]
    #[inline] pub fn gpio25intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO25INTD != 0"]
    #[inline] pub fn test_gpio25intd(&self) -> bool {
        self.gpio25intd() != 0
    }

    #[doc="Sets the GPIO25INTD field."]
    #[inline] pub fn set_gpio25intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO25 output configuration."]
    #[inline] pub fn gpio25outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if GPIO25OUTCFG != 0"]
    #[inline] pub fn test_gpio25outcfg(&self) -> bool {
        self.gpio25outcfg() != 0
    }

    #[doc="Sets the GPIO25OUTCFG field."]
    #[inline] pub fn set_gpio25outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO25 input enable."]
    #[inline] pub fn gpio25incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO25INCFG != 0"]
    #[inline] pub fn test_gpio25incfg(&self) -> bool {
        self.gpio25incfg() != 0
    }

    #[doc="Sets the GPIO25INCFG field."]
    #[inline] pub fn set_gpio25incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO24 interrupt direction."]
    #[inline] pub fn gpio24intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO24INTD != 0"]
    #[inline] pub fn test_gpio24intd(&self) -> bool {
        self.gpio24intd() != 0
    }

    #[doc="Sets the GPIO24INTD field."]
    #[inline] pub fn set_gpio24intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO24 output configuration."]
    #[inline] pub fn gpio24outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if GPIO24OUTCFG != 0"]
    #[inline] pub fn test_gpio24outcfg(&self) -> bool {
        self.gpio24outcfg() != 0
    }

    #[doc="Sets the GPIO24OUTCFG field."]
    #[inline] pub fn set_gpio24outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO24 input enable."]
    #[inline] pub fn gpio24incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO24INCFG != 0"]
    #[inline] pub fn test_gpio24incfg(&self) -> bool {
        self.gpio24incfg() != 0
    }

    #[doc="Sets the GPIO24INCFG field."]
    #[inline] pub fn set_gpio24incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgd {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgd(other)
    }
}

impl ::core::fmt::Display for Cfgd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio31intd() != 0 { try!(write!(f, " gpio31intd"))}
        if self.gpio31outcfg() != 0 { try!(write!(f, " gpio31outcfg=0x{:x}", self.gpio31outcfg()))}
        if self.gpio31incfg() != 0 { try!(write!(f, " gpio31incfg"))}
        if self.gpio30intd() != 0 { try!(write!(f, " gpio30intd"))}
        if self.gpio30outcfg() != 0 { try!(write!(f, " gpio30outcfg=0x{:x}", self.gpio30outcfg()))}
        if self.gpio30incfg() != 0 { try!(write!(f, " gpio30incfg"))}
        if self.gpio29intd() != 0 { try!(write!(f, " gpio29intd"))}
        if self.gpio29outcfg() != 0 { try!(write!(f, " gpio29outcfg=0x{:x}", self.gpio29outcfg()))}
        if self.gpio29incfg() != 0 { try!(write!(f, " gpio29incfg"))}
        if self.gpio28intd() != 0 { try!(write!(f, " gpio28intd"))}
        if self.gpio28outcfg() != 0 { try!(write!(f, " gpio28outcfg=0x{:x}", self.gpio28outcfg()))}
        if self.gpio28incfg() != 0 { try!(write!(f, " gpio28incfg"))}
        if self.gpio27intd() != 0 { try!(write!(f, " gpio27intd"))}
        if self.gpio27outcfg() != 0 { try!(write!(f, " gpio27outcfg=0x{:x}", self.gpio27outcfg()))}
        if self.gpio27incfg() != 0 { try!(write!(f, " gpio27incfg"))}
        if self.gpio26intd() != 0 { try!(write!(f, " gpio26intd"))}
        if self.gpio26outcfg() != 0 { try!(write!(f, " gpio26outcfg=0x{:x}", self.gpio26outcfg()))}
        if self.gpio26incfg() != 0 { try!(write!(f, " gpio26incfg"))}
        if self.gpio25intd() != 0 { try!(write!(f, " gpio25intd"))}
        if self.gpio25outcfg() != 0 { try!(write!(f, " gpio25outcfg=0x{:x}", self.gpio25outcfg()))}
        if self.gpio25incfg() != 0 { try!(write!(f, " gpio25incfg"))}
        if self.gpio24intd() != 0 { try!(write!(f, " gpio24intd"))}
        if self.gpio24outcfg() != 0 { try!(write!(f, " gpio24outcfg=0x{:x}", self.gpio24outcfg()))}
        if self.gpio24incfg() != 0 { try!(write!(f, " gpio24incfg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Configuration Register E"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfge(pub u32);
impl Cfge {
    #[doc="GPIO39 interrupt direction."]
    #[inline] pub fn gpio39intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GPIO39INTD != 0"]
    #[inline] pub fn test_gpio39intd(&self) -> bool {
        self.gpio39intd() != 0
    }

    #[doc="Sets the GPIO39INTD field."]
    #[inline] pub fn set_gpio39intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="GPIO39 output configuration."]
    #[inline] pub fn gpio39outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if GPIO39OUTCFG != 0"]
    #[inline] pub fn test_gpio39outcfg(&self) -> bool {
        self.gpio39outcfg() != 0
    }

    #[doc="Sets the GPIO39OUTCFG field."]
    #[inline] pub fn set_gpio39outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="GPIO39 input enable."]
    #[inline] pub fn gpio39incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if GPIO39INCFG != 0"]
    #[inline] pub fn test_gpio39incfg(&self) -> bool {
        self.gpio39incfg() != 0
    }

    #[doc="Sets the GPIO39INCFG field."]
    #[inline] pub fn set_gpio39incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="GPIO38 interrupt direction."]
    #[inline] pub fn gpio38intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if GPIO38INTD != 0"]
    #[inline] pub fn test_gpio38intd(&self) -> bool {
        self.gpio38intd() != 0
    }

    #[doc="Sets the GPIO38INTD field."]
    #[inline] pub fn set_gpio38intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="GPIO38 output configuration."]
    #[inline] pub fn gpio38outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if GPIO38OUTCFG != 0"]
    #[inline] pub fn test_gpio38outcfg(&self) -> bool {
        self.gpio38outcfg() != 0
    }

    #[doc="Sets the GPIO38OUTCFG field."]
    #[inline] pub fn set_gpio38outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="GPIO38 input enable."]
    #[inline] pub fn gpio38incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if GPIO38INCFG != 0"]
    #[inline] pub fn test_gpio38incfg(&self) -> bool {
        self.gpio38incfg() != 0
    }

    #[doc="Sets the GPIO38INCFG field."]
    #[inline] pub fn set_gpio38incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="GPIO37 interrupt direction."]
    #[inline] pub fn gpio37intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if GPIO37INTD != 0"]
    #[inline] pub fn test_gpio37intd(&self) -> bool {
        self.gpio37intd() != 0
    }

    #[doc="Sets the GPIO37INTD field."]
    #[inline] pub fn set_gpio37intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="GPIO37 output configuration."]
    #[inline] pub fn gpio37outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if GPIO37OUTCFG != 0"]
    #[inline] pub fn test_gpio37outcfg(&self) -> bool {
        self.gpio37outcfg() != 0
    }

    #[doc="Sets the GPIO37OUTCFG field."]
    #[inline] pub fn set_gpio37outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="GPIO37 input enable."]
    #[inline] pub fn gpio37incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if GPIO37INCFG != 0"]
    #[inline] pub fn test_gpio37incfg(&self) -> bool {
        self.gpio37incfg() != 0
    }

    #[doc="Sets the GPIO37INCFG field."]
    #[inline] pub fn set_gpio37incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="GPIO36 interrupt direction."]
    #[inline] pub fn gpio36intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GPIO36INTD != 0"]
    #[inline] pub fn test_gpio36intd(&self) -> bool {
        self.gpio36intd() != 0
    }

    #[doc="Sets the GPIO36INTD field."]
    #[inline] pub fn set_gpio36intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="GPIO36 output configuration."]
    #[inline] pub fn gpio36outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if GPIO36OUTCFG != 0"]
    #[inline] pub fn test_gpio36outcfg(&self) -> bool {
        self.gpio36outcfg() != 0
    }

    #[doc="Sets the GPIO36OUTCFG field."]
    #[inline] pub fn set_gpio36outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO36 input enable."]
    #[inline] pub fn gpio36incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO36INCFG != 0"]
    #[inline] pub fn test_gpio36incfg(&self) -> bool {
        self.gpio36incfg() != 0
    }

    #[doc="Sets the GPIO36INCFG field."]
    #[inline] pub fn set_gpio36incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO35 interrupt direction."]
    #[inline] pub fn gpio35intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO35INTD != 0"]
    #[inline] pub fn test_gpio35intd(&self) -> bool {
        self.gpio35intd() != 0
    }

    #[doc="Sets the GPIO35INTD field."]
    #[inline] pub fn set_gpio35intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO35 output configuration."]
    #[inline] pub fn gpio35outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if GPIO35OUTCFG != 0"]
    #[inline] pub fn test_gpio35outcfg(&self) -> bool {
        self.gpio35outcfg() != 0
    }

    #[doc="Sets the GPIO35OUTCFG field."]
    #[inline] pub fn set_gpio35outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO35 input enable."]
    #[inline] pub fn gpio35incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO35INCFG != 0"]
    #[inline] pub fn test_gpio35incfg(&self) -> bool {
        self.gpio35incfg() != 0
    }

    #[doc="Sets the GPIO35INCFG field."]
    #[inline] pub fn set_gpio35incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO34 interrupt direction."]
    #[inline] pub fn gpio34intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO34INTD != 0"]
    #[inline] pub fn test_gpio34intd(&self) -> bool {
        self.gpio34intd() != 0
    }

    #[doc="Sets the GPIO34INTD field."]
    #[inline] pub fn set_gpio34intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO34 output configuration."]
    #[inline] pub fn gpio34outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if GPIO34OUTCFG != 0"]
    #[inline] pub fn test_gpio34outcfg(&self) -> bool {
        self.gpio34outcfg() != 0
    }

    #[doc="Sets the GPIO34OUTCFG field."]
    #[inline] pub fn set_gpio34outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO34 input enable."]
    #[inline] pub fn gpio34incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO34INCFG != 0"]
    #[inline] pub fn test_gpio34incfg(&self) -> bool {
        self.gpio34incfg() != 0
    }

    #[doc="Sets the GPIO34INCFG field."]
    #[inline] pub fn set_gpio34incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO33 interrupt direction."]
    #[inline] pub fn gpio33intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO33INTD != 0"]
    #[inline] pub fn test_gpio33intd(&self) -> bool {
        self.gpio33intd() != 0
    }

    #[doc="Sets the GPIO33INTD field."]
    #[inline] pub fn set_gpio33intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO33 output configuration."]
    #[inline] pub fn gpio33outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if GPIO33OUTCFG != 0"]
    #[inline] pub fn test_gpio33outcfg(&self) -> bool {
        self.gpio33outcfg() != 0
    }

    #[doc="Sets the GPIO33OUTCFG field."]
    #[inline] pub fn set_gpio33outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO33 input enable."]
    #[inline] pub fn gpio33incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO33INCFG != 0"]
    #[inline] pub fn test_gpio33incfg(&self) -> bool {
        self.gpio33incfg() != 0
    }

    #[doc="Sets the GPIO33INCFG field."]
    #[inline] pub fn set_gpio33incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO32 interrupt direction."]
    #[inline] pub fn gpio32intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO32INTD != 0"]
    #[inline] pub fn test_gpio32intd(&self) -> bool {
        self.gpio32intd() != 0
    }

    #[doc="Sets the GPIO32INTD field."]
    #[inline] pub fn set_gpio32intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO32 output configuration."]
    #[inline] pub fn gpio32outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if GPIO32OUTCFG != 0"]
    #[inline] pub fn test_gpio32outcfg(&self) -> bool {
        self.gpio32outcfg() != 0
    }

    #[doc="Sets the GPIO32OUTCFG field."]
    #[inline] pub fn set_gpio32outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO32 input enable."]
    #[inline] pub fn gpio32incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO32INCFG != 0"]
    #[inline] pub fn test_gpio32incfg(&self) -> bool {
        self.gpio32incfg() != 0
    }

    #[doc="Sets the GPIO32INCFG field."]
    #[inline] pub fn set_gpio32incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfge {
    #[inline]
    fn from(other: u32) -> Self {
         Cfge(other)
    }
}

impl ::core::fmt::Display for Cfge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio39intd() != 0 { try!(write!(f, " gpio39intd"))}
        if self.gpio39outcfg() != 0 { try!(write!(f, " gpio39outcfg=0x{:x}", self.gpio39outcfg()))}
        if self.gpio39incfg() != 0 { try!(write!(f, " gpio39incfg"))}
        if self.gpio38intd() != 0 { try!(write!(f, " gpio38intd"))}
        if self.gpio38outcfg() != 0 { try!(write!(f, " gpio38outcfg=0x{:x}", self.gpio38outcfg()))}
        if self.gpio38incfg() != 0 { try!(write!(f, " gpio38incfg"))}
        if self.gpio37intd() != 0 { try!(write!(f, " gpio37intd"))}
        if self.gpio37outcfg() != 0 { try!(write!(f, " gpio37outcfg=0x{:x}", self.gpio37outcfg()))}
        if self.gpio37incfg() != 0 { try!(write!(f, " gpio37incfg"))}
        if self.gpio36intd() != 0 { try!(write!(f, " gpio36intd"))}
        if self.gpio36outcfg() != 0 { try!(write!(f, " gpio36outcfg=0x{:x}", self.gpio36outcfg()))}
        if self.gpio36incfg() != 0 { try!(write!(f, " gpio36incfg"))}
        if self.gpio35intd() != 0 { try!(write!(f, " gpio35intd"))}
        if self.gpio35outcfg() != 0 { try!(write!(f, " gpio35outcfg=0x{:x}", self.gpio35outcfg()))}
        if self.gpio35incfg() != 0 { try!(write!(f, " gpio35incfg"))}
        if self.gpio34intd() != 0 { try!(write!(f, " gpio34intd"))}
        if self.gpio34outcfg() != 0 { try!(write!(f, " gpio34outcfg=0x{:x}", self.gpio34outcfg()))}
        if self.gpio34incfg() != 0 { try!(write!(f, " gpio34incfg"))}
        if self.gpio33intd() != 0 { try!(write!(f, " gpio33intd"))}
        if self.gpio33outcfg() != 0 { try!(write!(f, " gpio33outcfg=0x{:x}", self.gpio33outcfg()))}
        if self.gpio33incfg() != 0 { try!(write!(f, " gpio33incfg"))}
        if self.gpio32intd() != 0 { try!(write!(f, " gpio32intd"))}
        if self.gpio32outcfg() != 0 { try!(write!(f, " gpio32outcfg=0x{:x}", self.gpio32outcfg()))}
        if self.gpio32incfg() != 0 { try!(write!(f, " gpio32incfg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Configuration Register F"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgf(pub u32);
impl Cfgf {
    #[doc="GPIO47 interrupt direction."]
    #[inline] pub fn gpio47intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GPIO47INTD != 0"]
    #[inline] pub fn test_gpio47intd(&self) -> bool {
        self.gpio47intd() != 0
    }

    #[doc="Sets the GPIO47INTD field."]
    #[inline] pub fn set_gpio47intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="GPIO47 output configuration."]
    #[inline] pub fn gpio47outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if GPIO47OUTCFG != 0"]
    #[inline] pub fn test_gpio47outcfg(&self) -> bool {
        self.gpio47outcfg() != 0
    }

    #[doc="Sets the GPIO47OUTCFG field."]
    #[inline] pub fn set_gpio47outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="GPIO47 input enable."]
    #[inline] pub fn gpio47incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if GPIO47INCFG != 0"]
    #[inline] pub fn test_gpio47incfg(&self) -> bool {
        self.gpio47incfg() != 0
    }

    #[doc="Sets the GPIO47INCFG field."]
    #[inline] pub fn set_gpio47incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="GPIO46 interrupt direction."]
    #[inline] pub fn gpio46intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if GPIO46INTD != 0"]
    #[inline] pub fn test_gpio46intd(&self) -> bool {
        self.gpio46intd() != 0
    }

    #[doc="Sets the GPIO46INTD field."]
    #[inline] pub fn set_gpio46intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="GPIO46 output configuration."]
    #[inline] pub fn gpio46outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x3) as u8) } // [26:25]
    }

    #[doc="Returns true if GPIO46OUTCFG != 0"]
    #[inline] pub fn test_gpio46outcfg(&self) -> bool {
        self.gpio46outcfg() != 0
    }

    #[doc="Sets the GPIO46OUTCFG field."]
    #[inline] pub fn set_gpio46outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="GPIO46 input enable."]
    #[inline] pub fn gpio46incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if GPIO46INCFG != 0"]
    #[inline] pub fn test_gpio46incfg(&self) -> bool {
        self.gpio46incfg() != 0
    }

    #[doc="Sets the GPIO46INCFG field."]
    #[inline] pub fn set_gpio46incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="GPIO45 interrupt direction."]
    #[inline] pub fn gpio45intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if GPIO45INTD != 0"]
    #[inline] pub fn test_gpio45intd(&self) -> bool {
        self.gpio45intd() != 0
    }

    #[doc="Sets the GPIO45INTD field."]
    #[inline] pub fn set_gpio45intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="GPIO45 output configuration."]
    #[inline] pub fn gpio45outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if GPIO45OUTCFG != 0"]
    #[inline] pub fn test_gpio45outcfg(&self) -> bool {
        self.gpio45outcfg() != 0
    }

    #[doc="Sets the GPIO45OUTCFG field."]
    #[inline] pub fn set_gpio45outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="GPIO45 input enable."]
    #[inline] pub fn gpio45incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if GPIO45INCFG != 0"]
    #[inline] pub fn test_gpio45incfg(&self) -> bool {
        self.gpio45incfg() != 0
    }

    #[doc="Sets the GPIO45INCFG field."]
    #[inline] pub fn set_gpio45incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="GPIO44 interrupt direction."]
    #[inline] pub fn gpio44intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GPIO44INTD != 0"]
    #[inline] pub fn test_gpio44intd(&self) -> bool {
        self.gpio44intd() != 0
    }

    #[doc="Sets the GPIO44INTD field."]
    #[inline] pub fn set_gpio44intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="GPIO44 output configuration."]
    #[inline] pub fn gpio44outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if GPIO44OUTCFG != 0"]
    #[inline] pub fn test_gpio44outcfg(&self) -> bool {
        self.gpio44outcfg() != 0
    }

    #[doc="Sets the GPIO44OUTCFG field."]
    #[inline] pub fn set_gpio44outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO44 input enable."]
    #[inline] pub fn gpio44incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO44INCFG != 0"]
    #[inline] pub fn test_gpio44incfg(&self) -> bool {
        self.gpio44incfg() != 0
    }

    #[doc="Sets the GPIO44INCFG field."]
    #[inline] pub fn set_gpio44incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO43 interrupt direction."]
    #[inline] pub fn gpio43intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO43INTD != 0"]
    #[inline] pub fn test_gpio43intd(&self) -> bool {
        self.gpio43intd() != 0
    }

    #[doc="Sets the GPIO43INTD field."]
    #[inline] pub fn set_gpio43intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO43 output configuration."]
    #[inline] pub fn gpio43outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if GPIO43OUTCFG != 0"]
    #[inline] pub fn test_gpio43outcfg(&self) -> bool {
        self.gpio43outcfg() != 0
    }

    #[doc="Sets the GPIO43OUTCFG field."]
    #[inline] pub fn set_gpio43outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO43 input enable."]
    #[inline] pub fn gpio43incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO43INCFG != 0"]
    #[inline] pub fn test_gpio43incfg(&self) -> bool {
        self.gpio43incfg() != 0
    }

    #[doc="Sets the GPIO43INCFG field."]
    #[inline] pub fn set_gpio43incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO42 interrupt direction."]
    #[inline] pub fn gpio42intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO42INTD != 0"]
    #[inline] pub fn test_gpio42intd(&self) -> bool {
        self.gpio42intd() != 0
    }

    #[doc="Sets the GPIO42INTD field."]
    #[inline] pub fn set_gpio42intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO42 output configuration."]
    #[inline] pub fn gpio42outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if GPIO42OUTCFG != 0"]
    #[inline] pub fn test_gpio42outcfg(&self) -> bool {
        self.gpio42outcfg() != 0
    }

    #[doc="Sets the GPIO42OUTCFG field."]
    #[inline] pub fn set_gpio42outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO42 input enable."]
    #[inline] pub fn gpio42incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO42INCFG != 0"]
    #[inline] pub fn test_gpio42incfg(&self) -> bool {
        self.gpio42incfg() != 0
    }

    #[doc="Sets the GPIO42INCFG field."]
    #[inline] pub fn set_gpio42incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO41 interrupt direction."]
    #[inline] pub fn gpio41intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO41INTD != 0"]
    #[inline] pub fn test_gpio41intd(&self) -> bool {
        self.gpio41intd() != 0
    }

    #[doc="Sets the GPIO41INTD field."]
    #[inline] pub fn set_gpio41intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO41 output configuration."]
    #[inline] pub fn gpio41outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if GPIO41OUTCFG != 0"]
    #[inline] pub fn test_gpio41outcfg(&self) -> bool {
        self.gpio41outcfg() != 0
    }

    #[doc="Sets the GPIO41OUTCFG field."]
    #[inline] pub fn set_gpio41outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO41 input enable."]
    #[inline] pub fn gpio41incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO41INCFG != 0"]
    #[inline] pub fn test_gpio41incfg(&self) -> bool {
        self.gpio41incfg() != 0
    }

    #[doc="Sets the GPIO41INCFG field."]
    #[inline] pub fn set_gpio41incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO40 interrupt direction."]
    #[inline] pub fn gpio40intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO40INTD != 0"]
    #[inline] pub fn test_gpio40intd(&self) -> bool {
        self.gpio40intd() != 0
    }

    #[doc="Sets the GPIO40INTD field."]
    #[inline] pub fn set_gpio40intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO40 output configuration."]
    #[inline] pub fn gpio40outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if GPIO40OUTCFG != 0"]
    #[inline] pub fn test_gpio40outcfg(&self) -> bool {
        self.gpio40outcfg() != 0
    }

    #[doc="Sets the GPIO40OUTCFG field."]
    #[inline] pub fn set_gpio40outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO40 input enable."]
    #[inline] pub fn gpio40incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO40INCFG != 0"]
    #[inline] pub fn test_gpio40incfg(&self) -> bool {
        self.gpio40incfg() != 0
    }

    #[doc="Sets the GPIO40INCFG field."]
    #[inline] pub fn set_gpio40incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgf {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgf(other)
    }
}

impl ::core::fmt::Display for Cfgf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio47intd() != 0 { try!(write!(f, " gpio47intd"))}
        if self.gpio47outcfg() != 0 { try!(write!(f, " gpio47outcfg=0x{:x}", self.gpio47outcfg()))}
        if self.gpio47incfg() != 0 { try!(write!(f, " gpio47incfg"))}
        if self.gpio46intd() != 0 { try!(write!(f, " gpio46intd"))}
        if self.gpio46outcfg() != 0 { try!(write!(f, " gpio46outcfg=0x{:x}", self.gpio46outcfg()))}
        if self.gpio46incfg() != 0 { try!(write!(f, " gpio46incfg"))}
        if self.gpio45intd() != 0 { try!(write!(f, " gpio45intd"))}
        if self.gpio45outcfg() != 0 { try!(write!(f, " gpio45outcfg=0x{:x}", self.gpio45outcfg()))}
        if self.gpio45incfg() != 0 { try!(write!(f, " gpio45incfg"))}
        if self.gpio44intd() != 0 { try!(write!(f, " gpio44intd"))}
        if self.gpio44outcfg() != 0 { try!(write!(f, " gpio44outcfg=0x{:x}", self.gpio44outcfg()))}
        if self.gpio44incfg() != 0 { try!(write!(f, " gpio44incfg"))}
        if self.gpio43intd() != 0 { try!(write!(f, " gpio43intd"))}
        if self.gpio43outcfg() != 0 { try!(write!(f, " gpio43outcfg=0x{:x}", self.gpio43outcfg()))}
        if self.gpio43incfg() != 0 { try!(write!(f, " gpio43incfg"))}
        if self.gpio42intd() != 0 { try!(write!(f, " gpio42intd"))}
        if self.gpio42outcfg() != 0 { try!(write!(f, " gpio42outcfg=0x{:x}", self.gpio42outcfg()))}
        if self.gpio42incfg() != 0 { try!(write!(f, " gpio42incfg"))}
        if self.gpio41intd() != 0 { try!(write!(f, " gpio41intd"))}
        if self.gpio41outcfg() != 0 { try!(write!(f, " gpio41outcfg=0x{:x}", self.gpio41outcfg()))}
        if self.gpio41incfg() != 0 { try!(write!(f, " gpio41incfg"))}
        if self.gpio40intd() != 0 { try!(write!(f, " gpio40intd"))}
        if self.gpio40outcfg() != 0 { try!(write!(f, " gpio40outcfg=0x{:x}", self.gpio40outcfg()))}
        if self.gpio40incfg() != 0 { try!(write!(f, " gpio40incfg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Configuration Register G"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgg(pub u32);
impl Cfgg {
    #[doc="GPIO49 interrupt direction."]
    #[inline] pub fn gpio49intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO49INTD != 0"]
    #[inline] pub fn test_gpio49intd(&self) -> bool {
        self.gpio49intd() != 0
    }

    #[doc="Sets the GPIO49INTD field."]
    #[inline] pub fn set_gpio49intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO49 output configuration."]
    #[inline] pub fn gpio49outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if GPIO49OUTCFG != 0"]
    #[inline] pub fn test_gpio49outcfg(&self) -> bool {
        self.gpio49outcfg() != 0
    }

    #[doc="Sets the GPIO49OUTCFG field."]
    #[inline] pub fn set_gpio49outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO49 input enable."]
    #[inline] pub fn gpio49incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO49INCFG != 0"]
    #[inline] pub fn test_gpio49incfg(&self) -> bool {
        self.gpio49incfg() != 0
    }

    #[doc="Sets the GPIO49INCFG field."]
    #[inline] pub fn set_gpio49incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO48 interrupt direction."]
    #[inline] pub fn gpio48intd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO48INTD != 0"]
    #[inline] pub fn test_gpio48intd(&self) -> bool {
        self.gpio48intd() != 0
    }

    #[doc="Sets the GPIO48INTD field."]
    #[inline] pub fn set_gpio48intd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO48 output configuration."]
    #[inline] pub fn gpio48outcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if GPIO48OUTCFG != 0"]
    #[inline] pub fn test_gpio48outcfg(&self) -> bool {
        self.gpio48outcfg() != 0
    }

    #[doc="Sets the GPIO48OUTCFG field."]
    #[inline] pub fn set_gpio48outcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO48 input enable."]
    #[inline] pub fn gpio48incfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO48INCFG != 0"]
    #[inline] pub fn test_gpio48incfg(&self) -> bool {
        self.gpio48incfg() != 0
    }

    #[doc="Sets the GPIO48INCFG field."]
    #[inline] pub fn set_gpio48incfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgg {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgg(other)
    }
}

impl ::core::fmt::Display for Cfgg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio49intd() != 0 { try!(write!(f, " gpio49intd"))}
        if self.gpio49outcfg() != 0 { try!(write!(f, " gpio49outcfg=0x{:x}", self.gpio49outcfg()))}
        if self.gpio49incfg() != 0 { try!(write!(f, " gpio49incfg"))}
        if self.gpio48intd() != 0 { try!(write!(f, " gpio48intd"))}
        if self.gpio48outcfg() != 0 { try!(write!(f, " gpio48outcfg=0x{:x}", self.gpio48outcfg()))}
        if self.gpio48incfg() != 0 { try!(write!(f, " gpio48incfg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Key Register for all pad configuration registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padkey(pub u32);
impl Padkey {
    #[doc="Key register value."]
    #[inline] pub fn padkey(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PADKEY != 0"]
    #[inline] pub fn test_padkey(&self) -> bool {
        self.padkey() != 0
    }

    #[doc="Sets the PADKEY field."]
    #[inline] pub fn set_padkey<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padkey {
    #[inline]
    fn from(other: u32) -> Self {
         Padkey(other)
    }
}

impl ::core::fmt::Display for Padkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Input Register A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rda(pub u32);
impl Rda {
    #[doc="GPIO31-0 read data."]
    #[inline] pub fn rda(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RDA != 0"]
    #[inline] pub fn test_rda(&self) -> bool {
        self.rda() != 0
    }

    #[doc="Sets the RDA field."]
    #[inline] pub fn set_rda<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rda {
    #[inline]
    fn from(other: u32) -> Self {
         Rda(other)
    }
}

impl ::core::fmt::Display for Rda {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rda {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Input Register B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdb(pub u32);
impl Rdb {
    #[doc="GPIO49-32 read data."]
    #[inline] pub fn rdb(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ffff) as u32) } // [17:0]
    }

    #[doc="Returns true if RDB != 0"]
    #[inline] pub fn test_rdb(&self) -> bool {
        self.rdb() != 0
    }

    #[doc="Sets the RDB field."]
    #[inline] pub fn set_rdb<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdb {
    #[inline]
    fn from(other: u32) -> Self {
         Rdb(other)
    }
}

impl ::core::fmt::Display for Rdb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdb() != 0 { try!(write!(f, " rdb=0x{:x}", self.rdb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Output Register A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wta(pub u32);
impl Wta {
    #[doc="GPIO31-0 write data."]
    #[inline] pub fn wta(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if WTA != 0"]
    #[inline] pub fn test_wta(&self) -> bool {
        self.wta() != 0
    }

    #[doc="Sets the WTA field."]
    #[inline] pub fn set_wta<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wta {
    #[inline]
    fn from(other: u32) -> Self {
         Wta(other)
    }
}

impl ::core::fmt::Display for Wta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Output Register B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wtb(pub u32);
impl Wtb {
    #[doc="GPIO49-32 write data."]
    #[inline] pub fn wtb(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ffff) as u32) } // [17:0]
    }

    #[doc="Returns true if WTB != 0"]
    #[inline] pub fn test_wtb(&self) -> bool {
        self.wtb() != 0
    }

    #[doc="Sets the WTB field."]
    #[inline] pub fn set_wtb<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wtb {
    #[inline]
    fn from(other: u32) -> Self {
         Wtb(other)
    }
}

impl ::core::fmt::Display for Wtb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wtb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wtb() != 0 { try!(write!(f, " wtb=0x{:x}", self.wtb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Output Register A Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wtsa(pub u32);
impl Wtsa {
    #[doc="Set the GPIO31-0 write data."]
    #[inline] pub fn wtsa(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if WTSA != 0"]
    #[inline] pub fn test_wtsa(&self) -> bool {
        self.wtsa() != 0
    }

    #[doc="Sets the WTSA field."]
    #[inline] pub fn set_wtsa<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wtsa {
    #[inline]
    fn from(other: u32) -> Self {
         Wtsa(other)
    }
}

impl ::core::fmt::Display for Wtsa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wtsa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Output Register B Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wtsb(pub u32);
impl Wtsb {
    #[doc="Set the GPIO49-32 write data."]
    #[inline] pub fn wtsb(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ffff) as u32) } // [17:0]
    }

    #[doc="Returns true if WTSB != 0"]
    #[inline] pub fn test_wtsb(&self) -> bool {
        self.wtsb() != 0
    }

    #[doc="Sets the WTSB field."]
    #[inline] pub fn set_wtsb<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wtsb {
    #[inline]
    fn from(other: u32) -> Self {
         Wtsb(other)
    }
}

impl ::core::fmt::Display for Wtsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wtsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wtsb() != 0 { try!(write!(f, " wtsb=0x{:x}", self.wtsb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Output Register A Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wtca(pub u32);
impl Wtca {
    #[doc="Clear the GPIO31-0 write data."]
    #[inline] pub fn wtca(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if WTCA != 0"]
    #[inline] pub fn test_wtca(&self) -> bool {
        self.wtca() != 0
    }

    #[doc="Sets the WTCA field."]
    #[inline] pub fn set_wtca<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wtca {
    #[inline]
    fn from(other: u32) -> Self {
         Wtca(other)
    }
}

impl ::core::fmt::Display for Wtca {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wtca {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Output Register B Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wtcb(pub u32);
impl Wtcb {
    #[doc="Clear the GPIO49-32 write data."]
    #[inline] pub fn wtcb(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ffff) as u32) } // [17:0]
    }

    #[doc="Returns true if WTCB != 0"]
    #[inline] pub fn test_wtcb(&self) -> bool {
        self.wtcb() != 0
    }

    #[doc="Sets the WTCB field."]
    #[inline] pub fn set_wtcb<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wtcb {
    #[inline]
    fn from(other: u32) -> Self {
         Wtcb(other)
    }
}

impl ::core::fmt::Display for Wtcb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wtcb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wtcb() != 0 { try!(write!(f, " wtcb=0x{:x}", self.wtcb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Enable Register A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ena(pub u32);
impl Ena {
    #[doc="GPIO31-0 output enables"]
    #[inline] pub fn ena(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ENA != 0"]
    #[inline] pub fn test_ena(&self) -> bool {
        self.ena() != 0
    }

    #[doc="Sets the ENA field."]
    #[inline] pub fn set_ena<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ena {
    #[inline]
    fn from(other: u32) -> Self {
         Ena(other)
    }
}

impl ::core::fmt::Display for Ena {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ena {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Enable Register B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Enb(pub u32);
impl Enb {
    #[doc="GPIO49-32 output enables"]
    #[inline] pub fn enb(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ffff) as u32) } // [17:0]
    }

    #[doc="Returns true if ENB != 0"]
    #[inline] pub fn test_enb(&self) -> bool {
        self.enb() != 0
    }

    #[doc="Sets the ENB field."]
    #[inline] pub fn set_enb<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Enb {
    #[inline]
    fn from(other: u32) -> Self {
         Enb(other)
    }
}

impl ::core::fmt::Display for Enb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Enb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enb() != 0 { try!(write!(f, " enb=0x{:x}", self.enb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Enable Register A Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ensa(pub u32);
impl Ensa {
    #[doc="Set the GPIO31-0 output enables"]
    #[inline] pub fn ensa(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ENSA != 0"]
    #[inline] pub fn test_ensa(&self) -> bool {
        self.ensa() != 0
    }

    #[doc="Sets the ENSA field."]
    #[inline] pub fn set_ensa<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ensa {
    #[inline]
    fn from(other: u32) -> Self {
         Ensa(other)
    }
}

impl ::core::fmt::Display for Ensa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ensa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Enable Register B Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ensb(pub u32);
impl Ensb {
    #[doc="Set the GPIO49-32 output enables"]
    #[inline] pub fn ensb(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ffff) as u32) } // [17:0]
    }

    #[doc="Returns true if ENSB != 0"]
    #[inline] pub fn test_ensb(&self) -> bool {
        self.ensb() != 0
    }

    #[doc="Sets the ENSB field."]
    #[inline] pub fn set_ensb<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ensb {
    #[inline]
    fn from(other: u32) -> Self {
         Ensb(other)
    }
}

impl ::core::fmt::Display for Ensb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ensb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ensb() != 0 { try!(write!(f, " ensb=0x{:x}", self.ensb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Enable Register A Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Enca(pub u32);
impl Enca {
    #[doc="Clear the GPIO31-0 output enables"]
    #[inline] pub fn enca(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ENCA != 0"]
    #[inline] pub fn test_enca(&self) -> bool {
        self.enca() != 0
    }

    #[doc="Sets the ENCA field."]
    #[inline] pub fn set_enca<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Enca {
    #[inline]
    fn from(other: u32) -> Self {
         Enca(other)
    }
}

impl ::core::fmt::Display for Enca {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Enca {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Enable Register B Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Encb(pub u32);
impl Encb {
    #[doc="Clear the GPIO49-32 output enables"]
    #[inline] pub fn encb(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ffff) as u32) } // [17:0]
    }

    #[doc="Returns true if ENCB != 0"]
    #[inline] pub fn test_encb(&self) -> bool {
        self.encb() != 0
    }

    #[doc="Sets the ENCB field."]
    #[inline] pub fn set_encb<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Encb {
    #[inline]
    fn from(other: u32) -> Self {
         Encb(other)
    }
}

impl ::core::fmt::Display for Encb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Encb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.encb() != 0 { try!(write!(f, " encb=0x{:x}", self.encb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Capture Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stmrcap(pub u32);
impl Stmrcap {
    #[doc="STIMER Capture 3 Polarity."]
    #[inline] pub fn stpol3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if STPOL3 != 0"]
    #[inline] pub fn test_stpol3(&self) -> bool {
        self.stpol3() != 0
    }

    #[doc="Sets the STPOL3 field."]
    #[inline] pub fn set_stpol3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="STIMER Capture 3 Select."]
    #[inline] pub fn stsel3(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if STSEL3 != 0"]
    #[inline] pub fn test_stsel3(&self) -> bool {
        self.stsel3() != 0
    }

    #[doc="Sets the STSEL3 field."]
    #[inline] pub fn set_stsel3<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="STIMER Capture 2 Polarity."]
    #[inline] pub fn stpol2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if STPOL2 != 0"]
    #[inline] pub fn test_stpol2(&self) -> bool {
        self.stpol2() != 0
    }

    #[doc="Sets the STPOL2 field."]
    #[inline] pub fn set_stpol2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="STIMER Capture 2 Select."]
    #[inline] pub fn stsel2(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
    }

    #[doc="Returns true if STSEL2 != 0"]
    #[inline] pub fn test_stsel2(&self) -> bool {
        self.stsel2() != 0
    }

    #[doc="Sets the STSEL2 field."]
    #[inline] pub fn set_stsel2<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="STIMER Capture 1 Polarity."]
    #[inline] pub fn stpol1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if STPOL1 != 0"]
    #[inline] pub fn test_stpol1(&self) -> bool {
        self.stpol1() != 0
    }

    #[doc="Sets the STPOL1 field."]
    #[inline] pub fn set_stpol1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="STIMER Capture 1 Select."]
    #[inline] pub fn stsel1(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if STSEL1 != 0"]
    #[inline] pub fn test_stsel1(&self) -> bool {
        self.stsel1() != 0
    }

    #[doc="Sets the STSEL1 field."]
    #[inline] pub fn set_stsel1<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="STIMER Capture 0 Polarity."]
    #[inline] pub fn stpol0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if STPOL0 != 0"]
    #[inline] pub fn test_stpol0(&self) -> bool {
        self.stpol0() != 0
    }

    #[doc="Sets the STPOL0 field."]
    #[inline] pub fn set_stpol0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="STIMER Capture 0 Select."]
    #[inline] pub fn stsel0(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if STSEL0 != 0"]
    #[inline] pub fn test_stsel0(&self) -> bool {
        self.stsel0() != 0
    }

    #[doc="Sets the STSEL0 field."]
    #[inline] pub fn set_stsel0<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stmrcap {
    #[inline]
    fn from(other: u32) -> Self {
         Stmrcap(other)
    }
}

impl ::core::fmt::Display for Stmrcap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stmrcap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stpol3() != 0 { try!(write!(f, " stpol3"))}
        if self.stsel3() != 0 { try!(write!(f, " stsel3=0x{:x}", self.stsel3()))}
        if self.stpol2() != 0 { try!(write!(f, " stpol2"))}
        if self.stsel2() != 0 { try!(write!(f, " stsel2=0x{:x}", self.stsel2()))}
        if self.stpol1() != 0 { try!(write!(f, " stpol1"))}
        if self.stsel1() != 0 { try!(write!(f, " stsel1=0x{:x}", self.stsel1()))}
        if self.stpol0() != 0 { try!(write!(f, " stpol0"))}
        if self.stsel0() != 0 { try!(write!(f, " stsel0=0x{:x}", self.stsel0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IOM0 Flow Control IRQ Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iom0irq(pub u32);
impl Iom0irq {
    #[doc="IOMSTR0 IRQ pad select."]
    #[inline] pub fn iom0irq(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if IOM0IRQ != 0"]
    #[inline] pub fn test_iom0irq(&self) -> bool {
        self.iom0irq() != 0
    }

    #[doc="Sets the IOM0IRQ field."]
    #[inline] pub fn set_iom0irq<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iom0irq {
    #[inline]
    fn from(other: u32) -> Self {
         Iom0irq(other)
    }
}

impl ::core::fmt::Display for Iom0irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iom0irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iom0irq() != 0 { try!(write!(f, " iom0irq=0x{:x}", self.iom0irq()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IOM1 Flow Control IRQ Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iom1irq(pub u32);
impl Iom1irq {
    #[doc="IOMSTR1 IRQ pad select."]
    #[inline] pub fn iom1irq(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if IOM1IRQ != 0"]
    #[inline] pub fn test_iom1irq(&self) -> bool {
        self.iom1irq() != 0
    }

    #[doc="Sets the IOM1IRQ field."]
    #[inline] pub fn set_iom1irq<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iom1irq {
    #[inline]
    fn from(other: u32) -> Self {
         Iom1irq(other)
    }
}

impl ::core::fmt::Display for Iom1irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iom1irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iom1irq() != 0 { try!(write!(f, " iom1irq=0x{:x}", self.iom1irq()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IOM2 Flow Control IRQ Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iom2irq(pub u32);
impl Iom2irq {
    #[doc="IOMSTR2 IRQ pad select."]
    #[inline] pub fn iom2irq(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if IOM2IRQ != 0"]
    #[inline] pub fn test_iom2irq(&self) -> bool {
        self.iom2irq() != 0
    }

    #[doc="Sets the IOM2IRQ field."]
    #[inline] pub fn set_iom2irq<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iom2irq {
    #[inline]
    fn from(other: u32) -> Self {
         Iom2irq(other)
    }
}

impl ::core::fmt::Display for Iom2irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iom2irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iom2irq() != 0 { try!(write!(f, " iom2irq=0x{:x}", self.iom2irq()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IOM3 Flow Control IRQ Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iom3irq(pub u32);
impl Iom3irq {
    #[doc="IOMSTR3 IRQ pad select."]
    #[inline] pub fn iom3irq(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if IOM3IRQ != 0"]
    #[inline] pub fn test_iom3irq(&self) -> bool {
        self.iom3irq() != 0
    }

    #[doc="Sets the IOM3IRQ field."]
    #[inline] pub fn set_iom3irq<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iom3irq {
    #[inline]
    fn from(other: u32) -> Self {
         Iom3irq(other)
    }
}

impl ::core::fmt::Display for Iom3irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iom3irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iom3irq() != 0 { try!(write!(f, " iom3irq=0x{:x}", self.iom3irq()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IOM4 Flow Control IRQ Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iom4irq(pub u32);
impl Iom4irq {
    #[doc="IOMSTR4 IRQ pad select."]
    #[inline] pub fn iom4irq(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if IOM4IRQ != 0"]
    #[inline] pub fn test_iom4irq(&self) -> bool {
        self.iom4irq() != 0
    }

    #[doc="Sets the IOM4IRQ field."]
    #[inline] pub fn set_iom4irq<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iom4irq {
    #[inline]
    fn from(other: u32) -> Self {
         Iom4irq(other)
    }
}

impl ::core::fmt::Display for Iom4irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iom4irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iom4irq() != 0 { try!(write!(f, " iom4irq=0x{:x}", self.iom4irq()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IOM5 Flow Control IRQ Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iom5irq(pub u32);
impl Iom5irq {
    #[doc="IOMSTR5 IRQ pad select."]
    #[inline] pub fn iom5irq(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if IOM5IRQ != 0"]
    #[inline] pub fn test_iom5irq(&self) -> bool {
        self.iom5irq() != 0
    }

    #[doc="Sets the IOM5IRQ field."]
    #[inline] pub fn set_iom5irq<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iom5irq {
    #[inline]
    fn from(other: u32) -> Self {
         Iom5irq(other)
    }
}

impl ::core::fmt::Display for Iom5irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iom5irq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iom5irq() != 0 { try!(write!(f, " iom5irq=0x{:x}", self.iom5irq()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IOM to IOS Loopback Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Loopback(pub u32);
impl Loopback {
    #[doc="IOM to IOS loopback control."]
    #[inline] pub fn loopback(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LOOPBACK != 0"]
    #[inline] pub fn test_loopback(&self) -> bool {
        self.loopback() != 0
    }

    #[doc="Sets the LOOPBACK field."]
    #[inline] pub fn set_loopback<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Loopback {
    #[inline]
    fn from(other: u32) -> Self {
         Loopback(other)
    }
}

impl ::core::fmt::Display for Loopback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Loopback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.loopback() != 0 { try!(write!(f, " loopback=0x{:x}", self.loopback()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Observation Mode Sample register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpioobs(pub u32);
impl Gpioobs {
    #[doc="Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only"]
    #[inline] pub fn obs_data(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if OBS_DATA != 0"]
    #[inline] pub fn test_obs_data(&self) -> bool {
        self.obs_data() != 0
    }

    #[doc="Sets the OBS_DATA field."]
    #[inline] pub fn set_obs_data<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Gpioobs {
    #[inline]
    fn from(other: u32) -> Self {
         Gpioobs(other)
    }
}

impl ::core::fmt::Display for Gpioobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gpioobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.obs_data() != 0 { try!(write!(f, " obs_data=0x{:x}", self.obs_data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg0 (Pads 3,2,1,0)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfga(pub u32);
impl Altpadcfga {
    #[doc="Pad 3 slew rate selection."]
    #[inline] pub fn pad3_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD3_SR != 0"]
    #[inline] pub fn test_pad3_sr(&self) -> bool {
        self.pad3_sr() != 0
    }

    #[doc="Sets the PAD3_SR field."]
    #[inline] pub fn set_pad3_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 3 high order drive strength selection. Used in conjunction with PAD3STRNG field to set the pad drive strength."]
    #[inline] pub fn pad3_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD3_DS1 != 0"]
    #[inline] pub fn test_pad3_ds1(&self) -> bool {
        self.pad3_ds1() != 0
    }

    #[doc="Sets the PAD3_DS1 field."]
    #[inline] pub fn set_pad3_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 2 slew rate selection."]
    #[inline] pub fn pad2_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD2_SR != 0"]
    #[inline] pub fn test_pad2_sr(&self) -> bool {
        self.pad2_sr() != 0
    }

    #[doc="Sets the PAD2_SR field."]
    #[inline] pub fn set_pad2_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 2 high order drive strength selection. Used in conjunction with PAD2STRNG field to set the pad drive strength."]
    #[inline] pub fn pad2_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD2_DS1 != 0"]
    #[inline] pub fn test_pad2_ds1(&self) -> bool {
        self.pad2_ds1() != 0
    }

    #[doc="Sets the PAD2_DS1 field."]
    #[inline] pub fn set_pad2_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 1 slew rate selection."]
    #[inline] pub fn pad1_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD1_SR != 0"]
    #[inline] pub fn test_pad1_sr(&self) -> bool {
        self.pad1_sr() != 0
    }

    #[doc="Sets the PAD1_SR field."]
    #[inline] pub fn set_pad1_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 1 high order drive strength selection. Used in conjunction with PAD1STRNG field to set the pad drive strength."]
    #[inline] pub fn pad1_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD1_DS1 != 0"]
    #[inline] pub fn test_pad1_ds1(&self) -> bool {
        self.pad1_ds1() != 0
    }

    #[doc="Sets the PAD1_DS1 field."]
    #[inline] pub fn set_pad1_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 0 slew rate selection."]
    #[inline] pub fn pad0_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD0_SR != 0"]
    #[inline] pub fn test_pad0_sr(&self) -> bool {
        self.pad0_sr() != 0
    }

    #[doc="Sets the PAD0_SR field."]
    #[inline] pub fn set_pad0_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 0 high order drive strength selection. Used in conjunction with PAD0STRNG field to set the pad drive strength."]
    #[inline] pub fn pad0_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD0_DS1 != 0"]
    #[inline] pub fn test_pad0_ds1(&self) -> bool {
        self.pad0_ds1() != 0
    }

    #[doc="Sets the PAD0_DS1 field."]
    #[inline] pub fn set_pad0_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfga {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfga(other)
    }
}

impl ::core::fmt::Display for Altpadcfga {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfga {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad3_sr() != 0 { try!(write!(f, " pad3_sr"))}
        if self.pad3_ds1() != 0 { try!(write!(f, " pad3_ds1"))}
        if self.pad2_sr() != 0 { try!(write!(f, " pad2_sr"))}
        if self.pad2_ds1() != 0 { try!(write!(f, " pad2_ds1"))}
        if self.pad1_sr() != 0 { try!(write!(f, " pad1_sr"))}
        if self.pad1_ds1() != 0 { try!(write!(f, " pad1_ds1"))}
        if self.pad0_sr() != 0 { try!(write!(f, " pad0_sr"))}
        if self.pad0_ds1() != 0 { try!(write!(f, " pad0_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg1 (Pads 7,6,5,4)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgb(pub u32);
impl Altpadcfgb {
    #[doc="Pad 7 slew rate selection."]
    #[inline] pub fn pad7_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD7_SR != 0"]
    #[inline] pub fn test_pad7_sr(&self) -> bool {
        self.pad7_sr() != 0
    }

    #[doc="Sets the PAD7_SR field."]
    #[inline] pub fn set_pad7_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 7 high order drive strength selection. Used in conjunction with PAD7STRNG field to set the pad drive strength."]
    #[inline] pub fn pad7_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD7_DS1 != 0"]
    #[inline] pub fn test_pad7_ds1(&self) -> bool {
        self.pad7_ds1() != 0
    }

    #[doc="Sets the PAD7_DS1 field."]
    #[inline] pub fn set_pad7_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 6 slew rate selection."]
    #[inline] pub fn pad6_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD6_SR != 0"]
    #[inline] pub fn test_pad6_sr(&self) -> bool {
        self.pad6_sr() != 0
    }

    #[doc="Sets the PAD6_SR field."]
    #[inline] pub fn set_pad6_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 6 high order drive strength selection. Used in conjunction with PAD6STRNG field to set the pad drive strength."]
    #[inline] pub fn pad6_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD6_DS1 != 0"]
    #[inline] pub fn test_pad6_ds1(&self) -> bool {
        self.pad6_ds1() != 0
    }

    #[doc="Sets the PAD6_DS1 field."]
    #[inline] pub fn set_pad6_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 5 slew rate selection."]
    #[inline] pub fn pad5_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD5_SR != 0"]
    #[inline] pub fn test_pad5_sr(&self) -> bool {
        self.pad5_sr() != 0
    }

    #[doc="Sets the PAD5_SR field."]
    #[inline] pub fn set_pad5_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 5 high order drive strength selection. Used in conjunction with PAD5STRNG field to set the pad drive strength."]
    #[inline] pub fn pad5_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD5_DS1 != 0"]
    #[inline] pub fn test_pad5_ds1(&self) -> bool {
        self.pad5_ds1() != 0
    }

    #[doc="Sets the PAD5_DS1 field."]
    #[inline] pub fn set_pad5_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 4 slew rate selection."]
    #[inline] pub fn pad4_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD4_SR != 0"]
    #[inline] pub fn test_pad4_sr(&self) -> bool {
        self.pad4_sr() != 0
    }

    #[doc="Sets the PAD4_SR field."]
    #[inline] pub fn set_pad4_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 4 high order drive strength selection. Used in conjunction with PAD4STRNG field to set the pad drive strength."]
    #[inline] pub fn pad4_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD4_DS1 != 0"]
    #[inline] pub fn test_pad4_ds1(&self) -> bool {
        self.pad4_ds1() != 0
    }

    #[doc="Sets the PAD4_DS1 field."]
    #[inline] pub fn set_pad4_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgb {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgb(other)
    }
}

impl ::core::fmt::Display for Altpadcfgb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad7_sr() != 0 { try!(write!(f, " pad7_sr"))}
        if self.pad7_ds1() != 0 { try!(write!(f, " pad7_ds1"))}
        if self.pad6_sr() != 0 { try!(write!(f, " pad6_sr"))}
        if self.pad6_ds1() != 0 { try!(write!(f, " pad6_ds1"))}
        if self.pad5_sr() != 0 { try!(write!(f, " pad5_sr"))}
        if self.pad5_ds1() != 0 { try!(write!(f, " pad5_ds1"))}
        if self.pad4_sr() != 0 { try!(write!(f, " pad4_sr"))}
        if self.pad4_ds1() != 0 { try!(write!(f, " pad4_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg2 (Pads 11,10,9,8)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgc(pub u32);
impl Altpadcfgc {
    #[doc="Pad 11 slew rate selection."]
    #[inline] pub fn pad11_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD11_SR != 0"]
    #[inline] pub fn test_pad11_sr(&self) -> bool {
        self.pad11_sr() != 0
    }

    #[doc="Sets the PAD11_SR field."]
    #[inline] pub fn set_pad11_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 11 high order drive strength selection. Used in conjunction with PAD11STRNG field to set the pad drive strength."]
    #[inline] pub fn pad11_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD11_DS1 != 0"]
    #[inline] pub fn test_pad11_ds1(&self) -> bool {
        self.pad11_ds1() != 0
    }

    #[doc="Sets the PAD11_DS1 field."]
    #[inline] pub fn set_pad11_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 10 slew rate selection."]
    #[inline] pub fn pad10_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD10_SR != 0"]
    #[inline] pub fn test_pad10_sr(&self) -> bool {
        self.pad10_sr() != 0
    }

    #[doc="Sets the PAD10_SR field."]
    #[inline] pub fn set_pad10_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 10 high order drive strength selection. Used in conjunction with PAD10STRNG field to set the pad drive strength."]
    #[inline] pub fn pad10_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD10_DS1 != 0"]
    #[inline] pub fn test_pad10_ds1(&self) -> bool {
        self.pad10_ds1() != 0
    }

    #[doc="Sets the PAD10_DS1 field."]
    #[inline] pub fn set_pad10_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 9 slew rate selection."]
    #[inline] pub fn pad9_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD9_SR != 0"]
    #[inline] pub fn test_pad9_sr(&self) -> bool {
        self.pad9_sr() != 0
    }

    #[doc="Sets the PAD9_SR field."]
    #[inline] pub fn set_pad9_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 9 high order drive strength selection. Used in conjunction with PAD9STRNG field to set the pad drive strength."]
    #[inline] pub fn pad9_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD9_DS1 != 0"]
    #[inline] pub fn test_pad9_ds1(&self) -> bool {
        self.pad9_ds1() != 0
    }

    #[doc="Sets the PAD9_DS1 field."]
    #[inline] pub fn set_pad9_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 8 slew rate selection."]
    #[inline] pub fn pad8_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD8_SR != 0"]
    #[inline] pub fn test_pad8_sr(&self) -> bool {
        self.pad8_sr() != 0
    }

    #[doc="Sets the PAD8_SR field."]
    #[inline] pub fn set_pad8_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 8 high order drive strength selection. Used in conjunction with PAD8STRNG field to set the pad drive strength."]
    #[inline] pub fn pad8_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD8_DS1 != 0"]
    #[inline] pub fn test_pad8_ds1(&self) -> bool {
        self.pad8_ds1() != 0
    }

    #[doc="Sets the PAD8_DS1 field."]
    #[inline] pub fn set_pad8_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgc {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgc(other)
    }
}

impl ::core::fmt::Display for Altpadcfgc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad11_sr() != 0 { try!(write!(f, " pad11_sr"))}
        if self.pad11_ds1() != 0 { try!(write!(f, " pad11_ds1"))}
        if self.pad10_sr() != 0 { try!(write!(f, " pad10_sr"))}
        if self.pad10_ds1() != 0 { try!(write!(f, " pad10_ds1"))}
        if self.pad9_sr() != 0 { try!(write!(f, " pad9_sr"))}
        if self.pad9_ds1() != 0 { try!(write!(f, " pad9_ds1"))}
        if self.pad8_sr() != 0 { try!(write!(f, " pad8_sr"))}
        if self.pad8_ds1() != 0 { try!(write!(f, " pad8_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg3 (Pads 15,14,13,12)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgd(pub u32);
impl Altpadcfgd {
    #[doc="Pad 15 slew rate selection."]
    #[inline] pub fn pad15_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD15_SR != 0"]
    #[inline] pub fn test_pad15_sr(&self) -> bool {
        self.pad15_sr() != 0
    }

    #[doc="Sets the PAD15_SR field."]
    #[inline] pub fn set_pad15_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
    #[inline] pub fn pad15_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD15_DS1 != 0"]
    #[inline] pub fn test_pad15_ds1(&self) -> bool {
        self.pad15_ds1() != 0
    }

    #[doc="Sets the PAD15_DS1 field."]
    #[inline] pub fn set_pad15_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 14 slew rate selection."]
    #[inline] pub fn pad14_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD14_SR != 0"]
    #[inline] pub fn test_pad14_sr(&self) -> bool {
        self.pad14_sr() != 0
    }

    #[doc="Sets the PAD14_SR field."]
    #[inline] pub fn set_pad14_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
    #[inline] pub fn pad14_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD14_DS1 != 0"]
    #[inline] pub fn test_pad14_ds1(&self) -> bool {
        self.pad14_ds1() != 0
    }

    #[doc="Sets the PAD14_DS1 field."]
    #[inline] pub fn set_pad14_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 13 slew rate selection."]
    #[inline] pub fn pad13_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD13_SR != 0"]
    #[inline] pub fn test_pad13_sr(&self) -> bool {
        self.pad13_sr() != 0
    }

    #[doc="Sets the PAD13_SR field."]
    #[inline] pub fn set_pad13_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
    #[inline] pub fn pad13_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD13_DS1 != 0"]
    #[inline] pub fn test_pad13_ds1(&self) -> bool {
        self.pad13_ds1() != 0
    }

    #[doc="Sets the PAD13_DS1 field."]
    #[inline] pub fn set_pad13_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 12 slew rate selection."]
    #[inline] pub fn pad12_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD12_SR != 0"]
    #[inline] pub fn test_pad12_sr(&self) -> bool {
        self.pad12_sr() != 0
    }

    #[doc="Sets the PAD12_SR field."]
    #[inline] pub fn set_pad12_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
    #[inline] pub fn pad12_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD12_DS1 != 0"]
    #[inline] pub fn test_pad12_ds1(&self) -> bool {
        self.pad12_ds1() != 0
    }

    #[doc="Sets the PAD12_DS1 field."]
    #[inline] pub fn set_pad12_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgd {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgd(other)
    }
}

impl ::core::fmt::Display for Altpadcfgd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad15_sr() != 0 { try!(write!(f, " pad15_sr"))}
        if self.pad15_ds1() != 0 { try!(write!(f, " pad15_ds1"))}
        if self.pad14_sr() != 0 { try!(write!(f, " pad14_sr"))}
        if self.pad14_ds1() != 0 { try!(write!(f, " pad14_ds1"))}
        if self.pad13_sr() != 0 { try!(write!(f, " pad13_sr"))}
        if self.pad13_ds1() != 0 { try!(write!(f, " pad13_ds1"))}
        if self.pad12_sr() != 0 { try!(write!(f, " pad12_sr"))}
        if self.pad12_ds1() != 0 { try!(write!(f, " pad12_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg4 (Pads 19,18,17,16)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfge(pub u32);
impl Altpadcfge {
    #[doc="Pad 19 slew rate selection."]
    #[inline] pub fn pad19_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD19_SR != 0"]
    #[inline] pub fn test_pad19_sr(&self) -> bool {
        self.pad19_sr() != 0
    }

    #[doc="Sets the PAD19_SR field."]
    #[inline] pub fn set_pad19_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
    #[inline] pub fn pad19_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD19_DS1 != 0"]
    #[inline] pub fn test_pad19_ds1(&self) -> bool {
        self.pad19_ds1() != 0
    }

    #[doc="Sets the PAD19_DS1 field."]
    #[inline] pub fn set_pad19_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 18 slew rate selection."]
    #[inline] pub fn pad18_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD18_SR != 0"]
    #[inline] pub fn test_pad18_sr(&self) -> bool {
        self.pad18_sr() != 0
    }

    #[doc="Sets the PAD18_SR field."]
    #[inline] pub fn set_pad18_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
    #[inline] pub fn pad18_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD18_DS1 != 0"]
    #[inline] pub fn test_pad18_ds1(&self) -> bool {
        self.pad18_ds1() != 0
    }

    #[doc="Sets the PAD18_DS1 field."]
    #[inline] pub fn set_pad18_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 17 slew rate selection."]
    #[inline] pub fn pad17_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD17_SR != 0"]
    #[inline] pub fn test_pad17_sr(&self) -> bool {
        self.pad17_sr() != 0
    }

    #[doc="Sets the PAD17_SR field."]
    #[inline] pub fn set_pad17_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
    #[inline] pub fn pad17_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD17_DS1 != 0"]
    #[inline] pub fn test_pad17_ds1(&self) -> bool {
        self.pad17_ds1() != 0
    }

    #[doc="Sets the PAD17_DS1 field."]
    #[inline] pub fn set_pad17_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 16 slew rate selection."]
    #[inline] pub fn pad16_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD16_SR != 0"]
    #[inline] pub fn test_pad16_sr(&self) -> bool {
        self.pad16_sr() != 0
    }

    #[doc="Sets the PAD16_SR field."]
    #[inline] pub fn set_pad16_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
    #[inline] pub fn pad16_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD16_DS1 != 0"]
    #[inline] pub fn test_pad16_ds1(&self) -> bool {
        self.pad16_ds1() != 0
    }

    #[doc="Sets the PAD16_DS1 field."]
    #[inline] pub fn set_pad16_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfge {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfge(other)
    }
}

impl ::core::fmt::Display for Altpadcfge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad19_sr() != 0 { try!(write!(f, " pad19_sr"))}
        if self.pad19_ds1() != 0 { try!(write!(f, " pad19_ds1"))}
        if self.pad18_sr() != 0 { try!(write!(f, " pad18_sr"))}
        if self.pad18_ds1() != 0 { try!(write!(f, " pad18_ds1"))}
        if self.pad17_sr() != 0 { try!(write!(f, " pad17_sr"))}
        if self.pad17_ds1() != 0 { try!(write!(f, " pad17_ds1"))}
        if self.pad16_sr() != 0 { try!(write!(f, " pad16_sr"))}
        if self.pad16_ds1() != 0 { try!(write!(f, " pad16_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg5 (Pads 23,22,21,20)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgf(pub u32);
impl Altpadcfgf {
    #[doc="Pad 23 slew rate selection."]
    #[inline] pub fn pad23_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD23_SR != 0"]
    #[inline] pub fn test_pad23_sr(&self) -> bool {
        self.pad23_sr() != 0
    }

    #[doc="Sets the PAD23_SR field."]
    #[inline] pub fn set_pad23_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
    #[inline] pub fn pad23_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD23_DS1 != 0"]
    #[inline] pub fn test_pad23_ds1(&self) -> bool {
        self.pad23_ds1() != 0
    }

    #[doc="Sets the PAD23_DS1 field."]
    #[inline] pub fn set_pad23_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 22 slew rate selection."]
    #[inline] pub fn pad22_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD22_SR != 0"]
    #[inline] pub fn test_pad22_sr(&self) -> bool {
        self.pad22_sr() != 0
    }

    #[doc="Sets the PAD22_SR field."]
    #[inline] pub fn set_pad22_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
    #[inline] pub fn pad22_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD22_DS1 != 0"]
    #[inline] pub fn test_pad22_ds1(&self) -> bool {
        self.pad22_ds1() != 0
    }

    #[doc="Sets the PAD22_DS1 field."]
    #[inline] pub fn set_pad22_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 21 slew rate selection."]
    #[inline] pub fn pad21_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD21_SR != 0"]
    #[inline] pub fn test_pad21_sr(&self) -> bool {
        self.pad21_sr() != 0
    }

    #[doc="Sets the PAD21_SR field."]
    #[inline] pub fn set_pad21_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
    #[inline] pub fn pad21_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD21_DS1 != 0"]
    #[inline] pub fn test_pad21_ds1(&self) -> bool {
        self.pad21_ds1() != 0
    }

    #[doc="Sets the PAD21_DS1 field."]
    #[inline] pub fn set_pad21_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 20 slew rate selection."]
    #[inline] pub fn pad20_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD20_SR != 0"]
    #[inline] pub fn test_pad20_sr(&self) -> bool {
        self.pad20_sr() != 0
    }

    #[doc="Sets the PAD20_SR field."]
    #[inline] pub fn set_pad20_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
    #[inline] pub fn pad20_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD20_DS1 != 0"]
    #[inline] pub fn test_pad20_ds1(&self) -> bool {
        self.pad20_ds1() != 0
    }

    #[doc="Sets the PAD20_DS1 field."]
    #[inline] pub fn set_pad20_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgf {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgf(other)
    }
}

impl ::core::fmt::Display for Altpadcfgf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad23_sr() != 0 { try!(write!(f, " pad23_sr"))}
        if self.pad23_ds1() != 0 { try!(write!(f, " pad23_ds1"))}
        if self.pad22_sr() != 0 { try!(write!(f, " pad22_sr"))}
        if self.pad22_ds1() != 0 { try!(write!(f, " pad22_ds1"))}
        if self.pad21_sr() != 0 { try!(write!(f, " pad21_sr"))}
        if self.pad21_ds1() != 0 { try!(write!(f, " pad21_ds1"))}
        if self.pad20_sr() != 0 { try!(write!(f, " pad20_sr"))}
        if self.pad20_ds1() != 0 { try!(write!(f, " pad20_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg6 (Pads 27,26,25,24)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgg(pub u32);
impl Altpadcfgg {
    #[doc="Pad 27 slew rate selection."]
    #[inline] pub fn pad27_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD27_SR != 0"]
    #[inline] pub fn test_pad27_sr(&self) -> bool {
        self.pad27_sr() != 0
    }

    #[doc="Sets the PAD27_SR field."]
    #[inline] pub fn set_pad27_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
    #[inline] pub fn pad27_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD27_DS1 != 0"]
    #[inline] pub fn test_pad27_ds1(&self) -> bool {
        self.pad27_ds1() != 0
    }

    #[doc="Sets the PAD27_DS1 field."]
    #[inline] pub fn set_pad27_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 26 slew rate selection."]
    #[inline] pub fn pad26_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD26_SR != 0"]
    #[inline] pub fn test_pad26_sr(&self) -> bool {
        self.pad26_sr() != 0
    }

    #[doc="Sets the PAD26_SR field."]
    #[inline] pub fn set_pad26_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
    #[inline] pub fn pad26_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD26_DS1 != 0"]
    #[inline] pub fn test_pad26_ds1(&self) -> bool {
        self.pad26_ds1() != 0
    }

    #[doc="Sets the PAD26_DS1 field."]
    #[inline] pub fn set_pad26_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 25 slew rate selection."]
    #[inline] pub fn pad25_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD25_SR != 0"]
    #[inline] pub fn test_pad25_sr(&self) -> bool {
        self.pad25_sr() != 0
    }

    #[doc="Sets the PAD25_SR field."]
    #[inline] pub fn set_pad25_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
    #[inline] pub fn pad25_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD25_DS1 != 0"]
    #[inline] pub fn test_pad25_ds1(&self) -> bool {
        self.pad25_ds1() != 0
    }

    #[doc="Sets the PAD25_DS1 field."]
    #[inline] pub fn set_pad25_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 24 slew rate selection."]
    #[inline] pub fn pad24_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD24_SR != 0"]
    #[inline] pub fn test_pad24_sr(&self) -> bool {
        self.pad24_sr() != 0
    }

    #[doc="Sets the PAD24_SR field."]
    #[inline] pub fn set_pad24_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
    #[inline] pub fn pad24_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD24_DS1 != 0"]
    #[inline] pub fn test_pad24_ds1(&self) -> bool {
        self.pad24_ds1() != 0
    }

    #[doc="Sets the PAD24_DS1 field."]
    #[inline] pub fn set_pad24_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgg {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgg(other)
    }
}

impl ::core::fmt::Display for Altpadcfgg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad27_sr() != 0 { try!(write!(f, " pad27_sr"))}
        if self.pad27_ds1() != 0 { try!(write!(f, " pad27_ds1"))}
        if self.pad26_sr() != 0 { try!(write!(f, " pad26_sr"))}
        if self.pad26_ds1() != 0 { try!(write!(f, " pad26_ds1"))}
        if self.pad25_sr() != 0 { try!(write!(f, " pad25_sr"))}
        if self.pad25_ds1() != 0 { try!(write!(f, " pad25_ds1"))}
        if self.pad24_sr() != 0 { try!(write!(f, " pad24_sr"))}
        if self.pad24_ds1() != 0 { try!(write!(f, " pad24_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg7 (Pads 31,30,29,28)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgh(pub u32);
impl Altpadcfgh {
    #[doc="Pad 31 slew rate selection."]
    #[inline] pub fn pad31_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD31_SR != 0"]
    #[inline] pub fn test_pad31_sr(&self) -> bool {
        self.pad31_sr() != 0
    }

    #[doc="Sets the PAD31_SR field."]
    #[inline] pub fn set_pad31_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 31 high order drive strength selection. Used in conjunction with PAD31STRNG field to set the pad drive strength."]
    #[inline] pub fn pad31_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD31_DS1 != 0"]
    #[inline] pub fn test_pad31_ds1(&self) -> bool {
        self.pad31_ds1() != 0
    }

    #[doc="Sets the PAD31_DS1 field."]
    #[inline] pub fn set_pad31_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 30 slew rate selection."]
    #[inline] pub fn pad30_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD30_SR != 0"]
    #[inline] pub fn test_pad30_sr(&self) -> bool {
        self.pad30_sr() != 0
    }

    #[doc="Sets the PAD30_SR field."]
    #[inline] pub fn set_pad30_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 30 high order drive strength selection. Used in conjunction with PAD30STRNG field to set the pad drive strength."]
    #[inline] pub fn pad30_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD30_DS1 != 0"]
    #[inline] pub fn test_pad30_ds1(&self) -> bool {
        self.pad30_ds1() != 0
    }

    #[doc="Sets the PAD30_DS1 field."]
    #[inline] pub fn set_pad30_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 29 slew rate selection."]
    #[inline] pub fn pad29_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD29_SR != 0"]
    #[inline] pub fn test_pad29_sr(&self) -> bool {
        self.pad29_sr() != 0
    }

    #[doc="Sets the PAD29_SR field."]
    #[inline] pub fn set_pad29_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 29 high order drive strength selection. Used in conjunction with PAD29STRNG field to set the pad drive strength."]
    #[inline] pub fn pad29_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD29_DS1 != 0"]
    #[inline] pub fn test_pad29_ds1(&self) -> bool {
        self.pad29_ds1() != 0
    }

    #[doc="Sets the PAD29_DS1 field."]
    #[inline] pub fn set_pad29_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 28 slew rate selection."]
    #[inline] pub fn pad28_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD28_SR != 0"]
    #[inline] pub fn test_pad28_sr(&self) -> bool {
        self.pad28_sr() != 0
    }

    #[doc="Sets the PAD28_SR field."]
    #[inline] pub fn set_pad28_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 28 high order drive strength selection. Used in conjunction with PAD28STRNG field to set the pad drive strength."]
    #[inline] pub fn pad28_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD28_DS1 != 0"]
    #[inline] pub fn test_pad28_ds1(&self) -> bool {
        self.pad28_ds1() != 0
    }

    #[doc="Sets the PAD28_DS1 field."]
    #[inline] pub fn set_pad28_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgh {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgh(other)
    }
}

impl ::core::fmt::Display for Altpadcfgh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad31_sr() != 0 { try!(write!(f, " pad31_sr"))}
        if self.pad31_ds1() != 0 { try!(write!(f, " pad31_ds1"))}
        if self.pad30_sr() != 0 { try!(write!(f, " pad30_sr"))}
        if self.pad30_ds1() != 0 { try!(write!(f, " pad30_ds1"))}
        if self.pad29_sr() != 0 { try!(write!(f, " pad29_sr"))}
        if self.pad29_ds1() != 0 { try!(write!(f, " pad29_ds1"))}
        if self.pad28_sr() != 0 { try!(write!(f, " pad28_sr"))}
        if self.pad28_ds1() != 0 { try!(write!(f, " pad28_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg8 (Pads 35,34,33,32)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgi(pub u32);
impl Altpadcfgi {
    #[doc="Pad 35 slew rate selection."]
    #[inline] pub fn pad35_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD35_SR != 0"]
    #[inline] pub fn test_pad35_sr(&self) -> bool {
        self.pad35_sr() != 0
    }

    #[doc="Sets the PAD35_SR field."]
    #[inline] pub fn set_pad35_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 35 high order drive strength selection. Used in conjunction with PAD35STRNG field to set the pad drive strength."]
    #[inline] pub fn pad35_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD35_DS1 != 0"]
    #[inline] pub fn test_pad35_ds1(&self) -> bool {
        self.pad35_ds1() != 0
    }

    #[doc="Sets the PAD35_DS1 field."]
    #[inline] pub fn set_pad35_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 34 slew rate selection."]
    #[inline] pub fn pad34_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD34_SR != 0"]
    #[inline] pub fn test_pad34_sr(&self) -> bool {
        self.pad34_sr() != 0
    }

    #[doc="Sets the PAD34_SR field."]
    #[inline] pub fn set_pad34_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 34 high order drive strength selection. Used in conjunction with PAD34STRNG field to set the pad drive strength."]
    #[inline] pub fn pad34_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD34_DS1 != 0"]
    #[inline] pub fn test_pad34_ds1(&self) -> bool {
        self.pad34_ds1() != 0
    }

    #[doc="Sets the PAD34_DS1 field."]
    #[inline] pub fn set_pad34_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 33 slew rate selection."]
    #[inline] pub fn pad33_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD33_SR != 0"]
    #[inline] pub fn test_pad33_sr(&self) -> bool {
        self.pad33_sr() != 0
    }

    #[doc="Sets the PAD33_SR field."]
    #[inline] pub fn set_pad33_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 33 high order drive strength selection. Used in conjunction with PAD33STRNG field to set the pad drive strength."]
    #[inline] pub fn pad33_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD33_DS1 != 0"]
    #[inline] pub fn test_pad33_ds1(&self) -> bool {
        self.pad33_ds1() != 0
    }

    #[doc="Sets the PAD33_DS1 field."]
    #[inline] pub fn set_pad33_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 32 slew rate selection."]
    #[inline] pub fn pad32_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD32_SR != 0"]
    #[inline] pub fn test_pad32_sr(&self) -> bool {
        self.pad32_sr() != 0
    }

    #[doc="Sets the PAD32_SR field."]
    #[inline] pub fn set_pad32_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 32 high order drive strength selection. Used in conjunction with PAD32STRNG field to set the pad drive strength."]
    #[inline] pub fn pad32_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD32_DS1 != 0"]
    #[inline] pub fn test_pad32_ds1(&self) -> bool {
        self.pad32_ds1() != 0
    }

    #[doc="Sets the PAD32_DS1 field."]
    #[inline] pub fn set_pad32_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgi {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgi(other)
    }
}

impl ::core::fmt::Display for Altpadcfgi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad35_sr() != 0 { try!(write!(f, " pad35_sr"))}
        if self.pad35_ds1() != 0 { try!(write!(f, " pad35_ds1"))}
        if self.pad34_sr() != 0 { try!(write!(f, " pad34_sr"))}
        if self.pad34_ds1() != 0 { try!(write!(f, " pad34_ds1"))}
        if self.pad33_sr() != 0 { try!(write!(f, " pad33_sr"))}
        if self.pad33_ds1() != 0 { try!(write!(f, " pad33_ds1"))}
        if self.pad32_sr() != 0 { try!(write!(f, " pad32_sr"))}
        if self.pad32_ds1() != 0 { try!(write!(f, " pad32_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg9 (Pads 39,38,37,36)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgj(pub u32);
impl Altpadcfgj {
    #[doc="Pad 39 slew rate selection."]
    #[inline] pub fn pad39_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD39_SR != 0"]
    #[inline] pub fn test_pad39_sr(&self) -> bool {
        self.pad39_sr() != 0
    }

    #[doc="Sets the PAD39_SR field."]
    #[inline] pub fn set_pad39_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
    #[inline] pub fn pad39_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD39_DS1 != 0"]
    #[inline] pub fn test_pad39_ds1(&self) -> bool {
        self.pad39_ds1() != 0
    }

    #[doc="Sets the PAD39_DS1 field."]
    #[inline] pub fn set_pad39_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 38 slew rate selection."]
    #[inline] pub fn pad38_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD38_SR != 0"]
    #[inline] pub fn test_pad38_sr(&self) -> bool {
        self.pad38_sr() != 0
    }

    #[doc="Sets the PAD38_SR field."]
    #[inline] pub fn set_pad38_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
    #[inline] pub fn pad38_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD38_DS1 != 0"]
    #[inline] pub fn test_pad38_ds1(&self) -> bool {
        self.pad38_ds1() != 0
    }

    #[doc="Sets the PAD38_DS1 field."]
    #[inline] pub fn set_pad38_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 37 slew rate selection."]
    #[inline] pub fn pad37_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD37_SR != 0"]
    #[inline] pub fn test_pad37_sr(&self) -> bool {
        self.pad37_sr() != 0
    }

    #[doc="Sets the PAD37_SR field."]
    #[inline] pub fn set_pad37_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
    #[inline] pub fn pad37_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD37_DS1 != 0"]
    #[inline] pub fn test_pad37_ds1(&self) -> bool {
        self.pad37_ds1() != 0
    }

    #[doc="Sets the PAD37_DS1 field."]
    #[inline] pub fn set_pad37_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 36 slew rate selection."]
    #[inline] pub fn pad36_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD36_SR != 0"]
    #[inline] pub fn test_pad36_sr(&self) -> bool {
        self.pad36_sr() != 0
    }

    #[doc="Sets the PAD36_SR field."]
    #[inline] pub fn set_pad36_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
    #[inline] pub fn pad36_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD36_DS1 != 0"]
    #[inline] pub fn test_pad36_ds1(&self) -> bool {
        self.pad36_ds1() != 0
    }

    #[doc="Sets the PAD36_DS1 field."]
    #[inline] pub fn set_pad36_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgj {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgj(other)
    }
}

impl ::core::fmt::Display for Altpadcfgj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad39_sr() != 0 { try!(write!(f, " pad39_sr"))}
        if self.pad39_ds1() != 0 { try!(write!(f, " pad39_ds1"))}
        if self.pad38_sr() != 0 { try!(write!(f, " pad38_sr"))}
        if self.pad38_ds1() != 0 { try!(write!(f, " pad38_ds1"))}
        if self.pad37_sr() != 0 { try!(write!(f, " pad37_sr"))}
        if self.pad37_ds1() != 0 { try!(write!(f, " pad37_ds1"))}
        if self.pad36_sr() != 0 { try!(write!(f, " pad36_sr"))}
        if self.pad36_ds1() != 0 { try!(write!(f, " pad36_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg10 (Pads 43,42,41,40)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgk(pub u32);
impl Altpadcfgk {
    #[doc="Pad 43 slew rate selection."]
    #[inline] pub fn pad43_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD43_SR != 0"]
    #[inline] pub fn test_pad43_sr(&self) -> bool {
        self.pad43_sr() != 0
    }

    #[doc="Sets the PAD43_SR field."]
    #[inline] pub fn set_pad43_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 43 high order drive strength selection. Used in conjunction with PAD43STRNG field to set the pad drive strength."]
    #[inline] pub fn pad43_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD43_DS1 != 0"]
    #[inline] pub fn test_pad43_ds1(&self) -> bool {
        self.pad43_ds1() != 0
    }

    #[doc="Sets the PAD43_DS1 field."]
    #[inline] pub fn set_pad43_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 42 slew rate selection."]
    #[inline] pub fn pad42_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD42_SR != 0"]
    #[inline] pub fn test_pad42_sr(&self) -> bool {
        self.pad42_sr() != 0
    }

    #[doc="Sets the PAD42_SR field."]
    #[inline] pub fn set_pad42_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 42 high order drive strength selection. Used in conjunction with PAD42STRNG field to set the pad drive strength."]
    #[inline] pub fn pad42_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD42_DS1 != 0"]
    #[inline] pub fn test_pad42_ds1(&self) -> bool {
        self.pad42_ds1() != 0
    }

    #[doc="Sets the PAD42_DS1 field."]
    #[inline] pub fn set_pad42_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 41 slew rate selection."]
    #[inline] pub fn pad41_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD41_SR != 0"]
    #[inline] pub fn test_pad41_sr(&self) -> bool {
        self.pad41_sr() != 0
    }

    #[doc="Sets the PAD41_SR field."]
    #[inline] pub fn set_pad41_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 41 high order drive strength selection. Used in conjunction with PAD41STRNG field to set the pad drive strength."]
    #[inline] pub fn pad41_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD41_DS1 != 0"]
    #[inline] pub fn test_pad41_ds1(&self) -> bool {
        self.pad41_ds1() != 0
    }

    #[doc="Sets the PAD41_DS1 field."]
    #[inline] pub fn set_pad41_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 40 slew rate selection."]
    #[inline] pub fn pad40_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD40_SR != 0"]
    #[inline] pub fn test_pad40_sr(&self) -> bool {
        self.pad40_sr() != 0
    }

    #[doc="Sets the PAD40_SR field."]
    #[inline] pub fn set_pad40_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 40 high order drive strength selection. Used in conjunction with PAD40STRNG field to set the pad drive strength."]
    #[inline] pub fn pad40_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD40_DS1 != 0"]
    #[inline] pub fn test_pad40_ds1(&self) -> bool {
        self.pad40_ds1() != 0
    }

    #[doc="Sets the PAD40_DS1 field."]
    #[inline] pub fn set_pad40_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgk {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgk(other)
    }
}

impl ::core::fmt::Display for Altpadcfgk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad43_sr() != 0 { try!(write!(f, " pad43_sr"))}
        if self.pad43_ds1() != 0 { try!(write!(f, " pad43_ds1"))}
        if self.pad42_sr() != 0 { try!(write!(f, " pad42_sr"))}
        if self.pad42_ds1() != 0 { try!(write!(f, " pad42_ds1"))}
        if self.pad41_sr() != 0 { try!(write!(f, " pad41_sr"))}
        if self.pad41_ds1() != 0 { try!(write!(f, " pad41_ds1"))}
        if self.pad40_sr() != 0 { try!(write!(f, " pad40_sr"))}
        if self.pad40_ds1() != 0 { try!(write!(f, " pad40_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg11 (Pads 47,46,45,44)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgl(pub u32);
impl Altpadcfgl {
    #[doc="Pad 47 slew rate selection."]
    #[inline] pub fn pad47_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD47_SR != 0"]
    #[inline] pub fn test_pad47_sr(&self) -> bool {
        self.pad47_sr() != 0
    }

    #[doc="Sets the PAD47_SR field."]
    #[inline] pub fn set_pad47_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 47 high order drive strength selection. Used in conjunction with PAD47STRNG field to set the pad drive strength."]
    #[inline] pub fn pad47_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD47_DS1 != 0"]
    #[inline] pub fn test_pad47_ds1(&self) -> bool {
        self.pad47_ds1() != 0
    }

    #[doc="Sets the PAD47_DS1 field."]
    #[inline] pub fn set_pad47_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 46 slew rate selection."]
    #[inline] pub fn pad46_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD46_SR != 0"]
    #[inline] pub fn test_pad46_sr(&self) -> bool {
        self.pad46_sr() != 0
    }

    #[doc="Sets the PAD46_SR field."]
    #[inline] pub fn set_pad46_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 46 high order drive strength selection. Used in conjunction with PAD46STRNG field to set the pad drive strength."]
    #[inline] pub fn pad46_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD46_DS1 != 0"]
    #[inline] pub fn test_pad46_ds1(&self) -> bool {
        self.pad46_ds1() != 0
    }

    #[doc="Sets the PAD46_DS1 field."]
    #[inline] pub fn set_pad46_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 45 slew rate selection."]
    #[inline] pub fn pad45_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD45_SR != 0"]
    #[inline] pub fn test_pad45_sr(&self) -> bool {
        self.pad45_sr() != 0
    }

    #[doc="Sets the PAD45_SR field."]
    #[inline] pub fn set_pad45_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 45 high order drive strength selection. Used in conjunction with PAD45STRNG field to set the pad drive strength."]
    #[inline] pub fn pad45_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD45_DS1 != 0"]
    #[inline] pub fn test_pad45_ds1(&self) -> bool {
        self.pad45_ds1() != 0
    }

    #[doc="Sets the PAD45_DS1 field."]
    #[inline] pub fn set_pad45_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 44 slew rate selection."]
    #[inline] pub fn pad44_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD44_SR != 0"]
    #[inline] pub fn test_pad44_sr(&self) -> bool {
        self.pad44_sr() != 0
    }

    #[doc="Sets the PAD44_SR field."]
    #[inline] pub fn set_pad44_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 44 high order drive strength selection. Used in conjunction with PAD44STRNG field to set the pad drive strength."]
    #[inline] pub fn pad44_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD44_DS1 != 0"]
    #[inline] pub fn test_pad44_ds1(&self) -> bool {
        self.pad44_ds1() != 0
    }

    #[doc="Sets the PAD44_DS1 field."]
    #[inline] pub fn set_pad44_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgl {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgl(other)
    }
}

impl ::core::fmt::Display for Altpadcfgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad47_sr() != 0 { try!(write!(f, " pad47_sr"))}
        if self.pad47_ds1() != 0 { try!(write!(f, " pad47_ds1"))}
        if self.pad46_sr() != 0 { try!(write!(f, " pad46_sr"))}
        if self.pad46_ds1() != 0 { try!(write!(f, " pad46_ds1"))}
        if self.pad45_sr() != 0 { try!(write!(f, " pad45_sr"))}
        if self.pad45_ds1() != 0 { try!(write!(f, " pad45_ds1"))}
        if self.pad44_sr() != 0 { try!(write!(f, " pad44_sr"))}
        if self.pad44_ds1() != 0 { try!(write!(f, " pad44_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg12 (Pads 49,48)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgm(pub u32);
impl Altpadcfgm {
    #[doc="Pad 49 slew rate selection."]
    #[inline] pub fn pad49_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD49_SR != 0"]
    #[inline] pub fn test_pad49_sr(&self) -> bool {
        self.pad49_sr() != 0
    }

    #[doc="Sets the PAD49_SR field."]
    #[inline] pub fn set_pad49_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 49 high order drive strength selection. Used in conjunction with PAD49STRNG field to set the pad drive strength."]
    #[inline] pub fn pad49_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD49_DS1 != 0"]
    #[inline] pub fn test_pad49_ds1(&self) -> bool {
        self.pad49_ds1() != 0
    }

    #[doc="Sets the PAD49_DS1 field."]
    #[inline] pub fn set_pad49_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 48 slew rate selection."]
    #[inline] pub fn pad48_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD48_SR != 0"]
    #[inline] pub fn test_pad48_sr(&self) -> bool {
        self.pad48_sr() != 0
    }

    #[doc="Sets the PAD48_SR field."]
    #[inline] pub fn set_pad48_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 48 high order drive strength selection. Used in conjunction with PAD48STRNG field to set the pad drive strength."]
    #[inline] pub fn pad48_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD48_DS1 != 0"]
    #[inline] pub fn test_pad48_ds1(&self) -> bool {
        self.pad48_ds1() != 0
    }

    #[doc="Sets the PAD48_DS1 field."]
    #[inline] pub fn set_pad48_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgm {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgm(other)
    }
}

impl ::core::fmt::Display for Altpadcfgm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad49_sr() != 0 { try!(write!(f, " pad49_sr"))}
        if self.pad49_ds1() != 0 { try!(write!(f, " pad49_ds1"))}
        if self.pad48_sr() != 0 { try!(write!(f, " pad48_sr"))}
        if self.pad48_ds1() != 0 { try!(write!(f, " pad48_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers 31-0: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Int0en(pub u32);
impl Int0en {
    #[doc="GPIO31 interrupt."]
    #[inline] pub fn gpio31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GPIO31 != 0"]
    #[inline] pub fn test_gpio31(&self) -> bool {
        self.gpio31() != 0
    }

    #[doc="Sets the GPIO31 field."]
    #[inline] pub fn set_gpio31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="GPIO30 interrupt."]
    #[inline] pub fn gpio30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if GPIO30 != 0"]
    #[inline] pub fn test_gpio30(&self) -> bool {
        self.gpio30() != 0
    }

    #[doc="Sets the GPIO30 field."]
    #[inline] pub fn set_gpio30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="GPIO29 interrupt."]
    #[inline] pub fn gpio29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if GPIO29 != 0"]
    #[inline] pub fn test_gpio29(&self) -> bool {
        self.gpio29() != 0
    }

    #[doc="Sets the GPIO29 field."]
    #[inline] pub fn set_gpio29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="GPIO28 interrupt."]
    #[inline] pub fn gpio28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if GPIO28 != 0"]
    #[inline] pub fn test_gpio28(&self) -> bool {
        self.gpio28() != 0
    }

    #[doc="Sets the GPIO28 field."]
    #[inline] pub fn set_gpio28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="GPIO27 interrupt."]
    #[inline] pub fn gpio27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if GPIO27 != 0"]
    #[inline] pub fn test_gpio27(&self) -> bool {
        self.gpio27() != 0
    }

    #[doc="Sets the GPIO27 field."]
    #[inline] pub fn set_gpio27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="GPIO26 interrupt."]
    #[inline] pub fn gpio26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if GPIO26 != 0"]
    #[inline] pub fn test_gpio26(&self) -> bool {
        self.gpio26() != 0
    }

    #[doc="Sets the GPIO26 field."]
    #[inline] pub fn set_gpio26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="GPIO25 interrupt."]
    #[inline] pub fn gpio25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if GPIO25 != 0"]
    #[inline] pub fn test_gpio25(&self) -> bool {
        self.gpio25() != 0
    }

    #[doc="Sets the GPIO25 field."]
    #[inline] pub fn set_gpio25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="GPIO24 interrupt."]
    #[inline] pub fn gpio24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if GPIO24 != 0"]
    #[inline] pub fn test_gpio24(&self) -> bool {
        self.gpio24() != 0
    }

    #[doc="Sets the GPIO24 field."]
    #[inline] pub fn set_gpio24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="GPIO23 interrupt."]
    #[inline] pub fn gpio23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if GPIO23 != 0"]
    #[inline] pub fn test_gpio23(&self) -> bool {
        self.gpio23() != 0
    }

    #[doc="Sets the GPIO23 field."]
    #[inline] pub fn set_gpio23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="GPIO22 interrupt."]
    #[inline] pub fn gpio22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if GPIO22 != 0"]
    #[inline] pub fn test_gpio22(&self) -> bool {
        self.gpio22() != 0
    }

    #[doc="Sets the GPIO22 field."]
    #[inline] pub fn set_gpio22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="GPIO21 interrupt."]
    #[inline] pub fn gpio21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if GPIO21 != 0"]
    #[inline] pub fn test_gpio21(&self) -> bool {
        self.gpio21() != 0
    }

    #[doc="Sets the GPIO21 field."]
    #[inline] pub fn set_gpio21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="GPIO20 interrupt."]
    #[inline] pub fn gpio20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if GPIO20 != 0"]
    #[inline] pub fn test_gpio20(&self) -> bool {
        self.gpio20() != 0
    }

    #[doc="Sets the GPIO20 field."]
    #[inline] pub fn set_gpio20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="GPIO19 interrupt."]
    #[inline] pub fn gpio19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GPIO19 != 0"]
    #[inline] pub fn test_gpio19(&self) -> bool {
        self.gpio19() != 0
    }

    #[doc="Sets the GPIO19 field."]
    #[inline] pub fn set_gpio19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="GPIO18interrupt."]
    #[inline] pub fn gpio18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if GPIO18 != 0"]
    #[inline] pub fn test_gpio18(&self) -> bool {
        self.gpio18() != 0
    }

    #[doc="Sets the GPIO18 field."]
    #[inline] pub fn set_gpio18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="GPIO17 interrupt."]
    #[inline] pub fn gpio17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if GPIO17 != 0"]
    #[inline] pub fn test_gpio17(&self) -> bool {
        self.gpio17() != 0
    }

    #[doc="Sets the GPIO17 field."]
    #[inline] pub fn set_gpio17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO16 interrupt."]
    #[inline] pub fn gpio16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO16 != 0"]
    #[inline] pub fn test_gpio16(&self) -> bool {
        self.gpio16() != 0
    }

    #[doc="Sets the GPIO16 field."]
    #[inline] pub fn set_gpio16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO15 interrupt."]
    #[inline] pub fn gpio15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO15 != 0"]
    #[inline] pub fn test_gpio15(&self) -> bool {
        self.gpio15() != 0
    }

    #[doc="Sets the GPIO15 field."]
    #[inline] pub fn set_gpio15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO14 interrupt."]
    #[inline] pub fn gpio14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GPIO14 != 0"]
    #[inline] pub fn test_gpio14(&self) -> bool {
        self.gpio14() != 0
    }

    #[doc="Sets the GPIO14 field."]
    #[inline] pub fn set_gpio14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="GPIO13 interrupt."]
    #[inline] pub fn gpio13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if GPIO13 != 0"]
    #[inline] pub fn test_gpio13(&self) -> bool {
        self.gpio13() != 0
    }

    #[doc="Sets the GPIO13 field."]
    #[inline] pub fn set_gpio13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO12 interrupt."]
    #[inline] pub fn gpio12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO12 != 0"]
    #[inline] pub fn test_gpio12(&self) -> bool {
        self.gpio12() != 0
    }

    #[doc="Sets the GPIO12 field."]
    #[inline] pub fn set_gpio12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO11 interrupt."]
    #[inline] pub fn gpio11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO11 != 0"]
    #[inline] pub fn test_gpio11(&self) -> bool {
        self.gpio11() != 0
    }

    #[doc="Sets the GPIO11 field."]
    #[inline] pub fn set_gpio11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO10 interrupt."]
    #[inline] pub fn gpio10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GPIO10 != 0"]
    #[inline] pub fn test_gpio10(&self) -> bool {
        self.gpio10() != 0
    }

    #[doc="Sets the GPIO10 field."]
    #[inline] pub fn set_gpio10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO9 interrupt."]
    #[inline] pub fn gpio9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GPIO9 != 0"]
    #[inline] pub fn test_gpio9(&self) -> bool {
        self.gpio9() != 0
    }

    #[doc="Sets the GPIO9 field."]
    #[inline] pub fn set_gpio9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO8 interrupt."]
    #[inline] pub fn gpio8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO8 != 0"]
    #[inline] pub fn test_gpio8(&self) -> bool {
        self.gpio8() != 0
    }

    #[doc="Sets the GPIO8 field."]
    #[inline] pub fn set_gpio8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO7 interrupt."]
    #[inline] pub fn gpio7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO7 != 0"]
    #[inline] pub fn test_gpio7(&self) -> bool {
        self.gpio7() != 0
    }

    #[doc="Sets the GPIO7 field."]
    #[inline] pub fn set_gpio7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO6 interrupt."]
    #[inline] pub fn gpio6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIO6 != 0"]
    #[inline] pub fn test_gpio6(&self) -> bool {
        self.gpio6() != 0
    }

    #[doc="Sets the GPIO6 field."]
    #[inline] pub fn set_gpio6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO5 interrupt."]
    #[inline] pub fn gpio5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIO5 != 0"]
    #[inline] pub fn test_gpio5(&self) -> bool {
        self.gpio5() != 0
    }

    #[doc="Sets the GPIO5 field."]
    #[inline] pub fn set_gpio5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO4 interrupt."]
    #[inline] pub fn gpio4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO4 != 0"]
    #[inline] pub fn test_gpio4(&self) -> bool {
        self.gpio4() != 0
    }

    #[doc="Sets the GPIO4 field."]
    #[inline] pub fn set_gpio4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO3 interrupt."]
    #[inline] pub fn gpio3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO3 != 0"]
    #[inline] pub fn test_gpio3(&self) -> bool {
        self.gpio3() != 0
    }

    #[doc="Sets the GPIO3 field."]
    #[inline] pub fn set_gpio3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO2 interrupt."]
    #[inline] pub fn gpio2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIO2 != 0"]
    #[inline] pub fn test_gpio2(&self) -> bool {
        self.gpio2() != 0
    }

    #[doc="Sets the GPIO2 field."]
    #[inline] pub fn set_gpio2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO1 interrupt."]
    #[inline] pub fn gpio1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIO1 != 0"]
    #[inline] pub fn test_gpio1(&self) -> bool {
        self.gpio1() != 0
    }

    #[doc="Sets the GPIO1 field."]
    #[inline] pub fn set_gpio1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO0 interrupt."]
    #[inline] pub fn gpio0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO0 != 0"]
    #[inline] pub fn test_gpio0(&self) -> bool {
        self.gpio0() != 0
    }

    #[doc="Sets the GPIO0 field."]
    #[inline] pub fn set_gpio0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Int0en {
    #[inline]
    fn from(other: u32) -> Self {
         Int0en(other)
    }
}

impl ::core::fmt::Display for Int0en {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Int0en {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio31() != 0 { try!(write!(f, " gpio31"))}
        if self.gpio30() != 0 { try!(write!(f, " gpio30"))}
        if self.gpio29() != 0 { try!(write!(f, " gpio29"))}
        if self.gpio28() != 0 { try!(write!(f, " gpio28"))}
        if self.gpio27() != 0 { try!(write!(f, " gpio27"))}
        if self.gpio26() != 0 { try!(write!(f, " gpio26"))}
        if self.gpio25() != 0 { try!(write!(f, " gpio25"))}
        if self.gpio24() != 0 { try!(write!(f, " gpio24"))}
        if self.gpio23() != 0 { try!(write!(f, " gpio23"))}
        if self.gpio22() != 0 { try!(write!(f, " gpio22"))}
        if self.gpio21() != 0 { try!(write!(f, " gpio21"))}
        if self.gpio20() != 0 { try!(write!(f, " gpio20"))}
        if self.gpio19() != 0 { try!(write!(f, " gpio19"))}
        if self.gpio18() != 0 { try!(write!(f, " gpio18"))}
        if self.gpio17() != 0 { try!(write!(f, " gpio17"))}
        if self.gpio16() != 0 { try!(write!(f, " gpio16"))}
        if self.gpio15() != 0 { try!(write!(f, " gpio15"))}
        if self.gpio14() != 0 { try!(write!(f, " gpio14"))}
        if self.gpio13() != 0 { try!(write!(f, " gpio13"))}
        if self.gpio12() != 0 { try!(write!(f, " gpio12"))}
        if self.gpio11() != 0 { try!(write!(f, " gpio11"))}
        if self.gpio10() != 0 { try!(write!(f, " gpio10"))}
        if self.gpio9() != 0 { try!(write!(f, " gpio9"))}
        if self.gpio8() != 0 { try!(write!(f, " gpio8"))}
        if self.gpio7() != 0 { try!(write!(f, " gpio7"))}
        if self.gpio6() != 0 { try!(write!(f, " gpio6"))}
        if self.gpio5() != 0 { try!(write!(f, " gpio5"))}
        if self.gpio4() != 0 { try!(write!(f, " gpio4"))}
        if self.gpio3() != 0 { try!(write!(f, " gpio3"))}
        if self.gpio2() != 0 { try!(write!(f, " gpio2"))}
        if self.gpio1() != 0 { try!(write!(f, " gpio1"))}
        if self.gpio0() != 0 { try!(write!(f, " gpio0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers 31-0: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Int0stat(pub u32);
impl Int0stat {
    #[doc="GPIO31 interrupt."]
    #[inline] pub fn gpio31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GPIO31 != 0"]
    #[inline] pub fn test_gpio31(&self) -> bool {
        self.gpio31() != 0
    }

    #[doc="Sets the GPIO31 field."]
    #[inline] pub fn set_gpio31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="GPIO30 interrupt."]
    #[inline] pub fn gpio30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if GPIO30 != 0"]
    #[inline] pub fn test_gpio30(&self) -> bool {
        self.gpio30() != 0
    }

    #[doc="Sets the GPIO30 field."]
    #[inline] pub fn set_gpio30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="GPIO29 interrupt."]
    #[inline] pub fn gpio29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if GPIO29 != 0"]
    #[inline] pub fn test_gpio29(&self) -> bool {
        self.gpio29() != 0
    }

    #[doc="Sets the GPIO29 field."]
    #[inline] pub fn set_gpio29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="GPIO28 interrupt."]
    #[inline] pub fn gpio28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if GPIO28 != 0"]
    #[inline] pub fn test_gpio28(&self) -> bool {
        self.gpio28() != 0
    }

    #[doc="Sets the GPIO28 field."]
    #[inline] pub fn set_gpio28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="GPIO27 interrupt."]
    #[inline] pub fn gpio27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if GPIO27 != 0"]
    #[inline] pub fn test_gpio27(&self) -> bool {
        self.gpio27() != 0
    }

    #[doc="Sets the GPIO27 field."]
    #[inline] pub fn set_gpio27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="GPIO26 interrupt."]
    #[inline] pub fn gpio26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if GPIO26 != 0"]
    #[inline] pub fn test_gpio26(&self) -> bool {
        self.gpio26() != 0
    }

    #[doc="Sets the GPIO26 field."]
    #[inline] pub fn set_gpio26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="GPIO25 interrupt."]
    #[inline] pub fn gpio25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if GPIO25 != 0"]
    #[inline] pub fn test_gpio25(&self) -> bool {
        self.gpio25() != 0
    }

    #[doc="Sets the GPIO25 field."]
    #[inline] pub fn set_gpio25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="GPIO24 interrupt."]
    #[inline] pub fn gpio24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if GPIO24 != 0"]
    #[inline] pub fn test_gpio24(&self) -> bool {
        self.gpio24() != 0
    }

    #[doc="Sets the GPIO24 field."]
    #[inline] pub fn set_gpio24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="GPIO23 interrupt."]
    #[inline] pub fn gpio23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if GPIO23 != 0"]
    #[inline] pub fn test_gpio23(&self) -> bool {
        self.gpio23() != 0
    }

    #[doc="Sets the GPIO23 field."]
    #[inline] pub fn set_gpio23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="GPIO22 interrupt."]
    #[inline] pub fn gpio22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if GPIO22 != 0"]
    #[inline] pub fn test_gpio22(&self) -> bool {
        self.gpio22() != 0
    }

    #[doc="Sets the GPIO22 field."]
    #[inline] pub fn set_gpio22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="GPIO21 interrupt."]
    #[inline] pub fn gpio21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if GPIO21 != 0"]
    #[inline] pub fn test_gpio21(&self) -> bool {
        self.gpio21() != 0
    }

    #[doc="Sets the GPIO21 field."]
    #[inline] pub fn set_gpio21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="GPIO20 interrupt."]
    #[inline] pub fn gpio20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if GPIO20 != 0"]
    #[inline] pub fn test_gpio20(&self) -> bool {
        self.gpio20() != 0
    }

    #[doc="Sets the GPIO20 field."]
    #[inline] pub fn set_gpio20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="GPIO19 interrupt."]
    #[inline] pub fn gpio19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GPIO19 != 0"]
    #[inline] pub fn test_gpio19(&self) -> bool {
        self.gpio19() != 0
    }

    #[doc="Sets the GPIO19 field."]
    #[inline] pub fn set_gpio19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="GPIO18interrupt."]
    #[inline] pub fn gpio18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if GPIO18 != 0"]
    #[inline] pub fn test_gpio18(&self) -> bool {
        self.gpio18() != 0
    }

    #[doc="Sets the GPIO18 field."]
    #[inline] pub fn set_gpio18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="GPIO17 interrupt."]
    #[inline] pub fn gpio17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if GPIO17 != 0"]
    #[inline] pub fn test_gpio17(&self) -> bool {
        self.gpio17() != 0
    }

    #[doc="Sets the GPIO17 field."]
    #[inline] pub fn set_gpio17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO16 interrupt."]
    #[inline] pub fn gpio16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO16 != 0"]
    #[inline] pub fn test_gpio16(&self) -> bool {
        self.gpio16() != 0
    }

    #[doc="Sets the GPIO16 field."]
    #[inline] pub fn set_gpio16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO15 interrupt."]
    #[inline] pub fn gpio15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO15 != 0"]
    #[inline] pub fn test_gpio15(&self) -> bool {
        self.gpio15() != 0
    }

    #[doc="Sets the GPIO15 field."]
    #[inline] pub fn set_gpio15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO14 interrupt."]
    #[inline] pub fn gpio14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GPIO14 != 0"]
    #[inline] pub fn test_gpio14(&self) -> bool {
        self.gpio14() != 0
    }

    #[doc="Sets the GPIO14 field."]
    #[inline] pub fn set_gpio14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="GPIO13 interrupt."]
    #[inline] pub fn gpio13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if GPIO13 != 0"]
    #[inline] pub fn test_gpio13(&self) -> bool {
        self.gpio13() != 0
    }

    #[doc="Sets the GPIO13 field."]
    #[inline] pub fn set_gpio13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO12 interrupt."]
    #[inline] pub fn gpio12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO12 != 0"]
    #[inline] pub fn test_gpio12(&self) -> bool {
        self.gpio12() != 0
    }

    #[doc="Sets the GPIO12 field."]
    #[inline] pub fn set_gpio12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO11 interrupt."]
    #[inline] pub fn gpio11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO11 != 0"]
    #[inline] pub fn test_gpio11(&self) -> bool {
        self.gpio11() != 0
    }

    #[doc="Sets the GPIO11 field."]
    #[inline] pub fn set_gpio11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO10 interrupt."]
    #[inline] pub fn gpio10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GPIO10 != 0"]
    #[inline] pub fn test_gpio10(&self) -> bool {
        self.gpio10() != 0
    }

    #[doc="Sets the GPIO10 field."]
    #[inline] pub fn set_gpio10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO9 interrupt."]
    #[inline] pub fn gpio9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GPIO9 != 0"]
    #[inline] pub fn test_gpio9(&self) -> bool {
        self.gpio9() != 0
    }

    #[doc="Sets the GPIO9 field."]
    #[inline] pub fn set_gpio9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO8 interrupt."]
    #[inline] pub fn gpio8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO8 != 0"]
    #[inline] pub fn test_gpio8(&self) -> bool {
        self.gpio8() != 0
    }

    #[doc="Sets the GPIO8 field."]
    #[inline] pub fn set_gpio8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO7 interrupt."]
    #[inline] pub fn gpio7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO7 != 0"]
    #[inline] pub fn test_gpio7(&self) -> bool {
        self.gpio7() != 0
    }

    #[doc="Sets the GPIO7 field."]
    #[inline] pub fn set_gpio7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO6 interrupt."]
    #[inline] pub fn gpio6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIO6 != 0"]
    #[inline] pub fn test_gpio6(&self) -> bool {
        self.gpio6() != 0
    }

    #[doc="Sets the GPIO6 field."]
    #[inline] pub fn set_gpio6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO5 interrupt."]
    #[inline] pub fn gpio5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIO5 != 0"]
    #[inline] pub fn test_gpio5(&self) -> bool {
        self.gpio5() != 0
    }

    #[doc="Sets the GPIO5 field."]
    #[inline] pub fn set_gpio5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO4 interrupt."]
    #[inline] pub fn gpio4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO4 != 0"]
    #[inline] pub fn test_gpio4(&self) -> bool {
        self.gpio4() != 0
    }

    #[doc="Sets the GPIO4 field."]
    #[inline] pub fn set_gpio4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO3 interrupt."]
    #[inline] pub fn gpio3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO3 != 0"]
    #[inline] pub fn test_gpio3(&self) -> bool {
        self.gpio3() != 0
    }

    #[doc="Sets the GPIO3 field."]
    #[inline] pub fn set_gpio3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO2 interrupt."]
    #[inline] pub fn gpio2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIO2 != 0"]
    #[inline] pub fn test_gpio2(&self) -> bool {
        self.gpio2() != 0
    }

    #[doc="Sets the GPIO2 field."]
    #[inline] pub fn set_gpio2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO1 interrupt."]
    #[inline] pub fn gpio1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIO1 != 0"]
    #[inline] pub fn test_gpio1(&self) -> bool {
        self.gpio1() != 0
    }

    #[doc="Sets the GPIO1 field."]
    #[inline] pub fn set_gpio1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO0 interrupt."]
    #[inline] pub fn gpio0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO0 != 0"]
    #[inline] pub fn test_gpio0(&self) -> bool {
        self.gpio0() != 0
    }

    #[doc="Sets the GPIO0 field."]
    #[inline] pub fn set_gpio0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Int0stat {
    #[inline]
    fn from(other: u32) -> Self {
         Int0stat(other)
    }
}

impl ::core::fmt::Display for Int0stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Int0stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio31() != 0 { try!(write!(f, " gpio31"))}
        if self.gpio30() != 0 { try!(write!(f, " gpio30"))}
        if self.gpio29() != 0 { try!(write!(f, " gpio29"))}
        if self.gpio28() != 0 { try!(write!(f, " gpio28"))}
        if self.gpio27() != 0 { try!(write!(f, " gpio27"))}
        if self.gpio26() != 0 { try!(write!(f, " gpio26"))}
        if self.gpio25() != 0 { try!(write!(f, " gpio25"))}
        if self.gpio24() != 0 { try!(write!(f, " gpio24"))}
        if self.gpio23() != 0 { try!(write!(f, " gpio23"))}
        if self.gpio22() != 0 { try!(write!(f, " gpio22"))}
        if self.gpio21() != 0 { try!(write!(f, " gpio21"))}
        if self.gpio20() != 0 { try!(write!(f, " gpio20"))}
        if self.gpio19() != 0 { try!(write!(f, " gpio19"))}
        if self.gpio18() != 0 { try!(write!(f, " gpio18"))}
        if self.gpio17() != 0 { try!(write!(f, " gpio17"))}
        if self.gpio16() != 0 { try!(write!(f, " gpio16"))}
        if self.gpio15() != 0 { try!(write!(f, " gpio15"))}
        if self.gpio14() != 0 { try!(write!(f, " gpio14"))}
        if self.gpio13() != 0 { try!(write!(f, " gpio13"))}
        if self.gpio12() != 0 { try!(write!(f, " gpio12"))}
        if self.gpio11() != 0 { try!(write!(f, " gpio11"))}
        if self.gpio10() != 0 { try!(write!(f, " gpio10"))}
        if self.gpio9() != 0 { try!(write!(f, " gpio9"))}
        if self.gpio8() != 0 { try!(write!(f, " gpio8"))}
        if self.gpio7() != 0 { try!(write!(f, " gpio7"))}
        if self.gpio6() != 0 { try!(write!(f, " gpio6"))}
        if self.gpio5() != 0 { try!(write!(f, " gpio5"))}
        if self.gpio4() != 0 { try!(write!(f, " gpio4"))}
        if self.gpio3() != 0 { try!(write!(f, " gpio3"))}
        if self.gpio2() != 0 { try!(write!(f, " gpio2"))}
        if self.gpio1() != 0 { try!(write!(f, " gpio1"))}
        if self.gpio0() != 0 { try!(write!(f, " gpio0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers 31-0: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Int0clr(pub u32);
impl Int0clr {
    #[doc="GPIO31 interrupt."]
    #[inline] pub fn gpio31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GPIO31 != 0"]
    #[inline] pub fn test_gpio31(&self) -> bool {
        self.gpio31() != 0
    }

    #[doc="Sets the GPIO31 field."]
    #[inline] pub fn set_gpio31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="GPIO30 interrupt."]
    #[inline] pub fn gpio30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if GPIO30 != 0"]
    #[inline] pub fn test_gpio30(&self) -> bool {
        self.gpio30() != 0
    }

    #[doc="Sets the GPIO30 field."]
    #[inline] pub fn set_gpio30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="GPIO29 interrupt."]
    #[inline] pub fn gpio29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if GPIO29 != 0"]
    #[inline] pub fn test_gpio29(&self) -> bool {
        self.gpio29() != 0
    }

    #[doc="Sets the GPIO29 field."]
    #[inline] pub fn set_gpio29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="GPIO28 interrupt."]
    #[inline] pub fn gpio28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if GPIO28 != 0"]
    #[inline] pub fn test_gpio28(&self) -> bool {
        self.gpio28() != 0
    }

    #[doc="Sets the GPIO28 field."]
    #[inline] pub fn set_gpio28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="GPIO27 interrupt."]
    #[inline] pub fn gpio27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if GPIO27 != 0"]
    #[inline] pub fn test_gpio27(&self) -> bool {
        self.gpio27() != 0
    }

    #[doc="Sets the GPIO27 field."]
    #[inline] pub fn set_gpio27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="GPIO26 interrupt."]
    #[inline] pub fn gpio26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if GPIO26 != 0"]
    #[inline] pub fn test_gpio26(&self) -> bool {
        self.gpio26() != 0
    }

    #[doc="Sets the GPIO26 field."]
    #[inline] pub fn set_gpio26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="GPIO25 interrupt."]
    #[inline] pub fn gpio25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if GPIO25 != 0"]
    #[inline] pub fn test_gpio25(&self) -> bool {
        self.gpio25() != 0
    }

    #[doc="Sets the GPIO25 field."]
    #[inline] pub fn set_gpio25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="GPIO24 interrupt."]
    #[inline] pub fn gpio24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if GPIO24 != 0"]
    #[inline] pub fn test_gpio24(&self) -> bool {
        self.gpio24() != 0
    }

    #[doc="Sets the GPIO24 field."]
    #[inline] pub fn set_gpio24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="GPIO23 interrupt."]
    #[inline] pub fn gpio23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if GPIO23 != 0"]
    #[inline] pub fn test_gpio23(&self) -> bool {
        self.gpio23() != 0
    }

    #[doc="Sets the GPIO23 field."]
    #[inline] pub fn set_gpio23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="GPIO22 interrupt."]
    #[inline] pub fn gpio22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if GPIO22 != 0"]
    #[inline] pub fn test_gpio22(&self) -> bool {
        self.gpio22() != 0
    }

    #[doc="Sets the GPIO22 field."]
    #[inline] pub fn set_gpio22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="GPIO21 interrupt."]
    #[inline] pub fn gpio21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if GPIO21 != 0"]
    #[inline] pub fn test_gpio21(&self) -> bool {
        self.gpio21() != 0
    }

    #[doc="Sets the GPIO21 field."]
    #[inline] pub fn set_gpio21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="GPIO20 interrupt."]
    #[inline] pub fn gpio20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if GPIO20 != 0"]
    #[inline] pub fn test_gpio20(&self) -> bool {
        self.gpio20() != 0
    }

    #[doc="Sets the GPIO20 field."]
    #[inline] pub fn set_gpio20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="GPIO19 interrupt."]
    #[inline] pub fn gpio19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GPIO19 != 0"]
    #[inline] pub fn test_gpio19(&self) -> bool {
        self.gpio19() != 0
    }

    #[doc="Sets the GPIO19 field."]
    #[inline] pub fn set_gpio19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="GPIO18interrupt."]
    #[inline] pub fn gpio18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if GPIO18 != 0"]
    #[inline] pub fn test_gpio18(&self) -> bool {
        self.gpio18() != 0
    }

    #[doc="Sets the GPIO18 field."]
    #[inline] pub fn set_gpio18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="GPIO17 interrupt."]
    #[inline] pub fn gpio17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if GPIO17 != 0"]
    #[inline] pub fn test_gpio17(&self) -> bool {
        self.gpio17() != 0
    }

    #[doc="Sets the GPIO17 field."]
    #[inline] pub fn set_gpio17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO16 interrupt."]
    #[inline] pub fn gpio16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO16 != 0"]
    #[inline] pub fn test_gpio16(&self) -> bool {
        self.gpio16() != 0
    }

    #[doc="Sets the GPIO16 field."]
    #[inline] pub fn set_gpio16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO15 interrupt."]
    #[inline] pub fn gpio15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO15 != 0"]
    #[inline] pub fn test_gpio15(&self) -> bool {
        self.gpio15() != 0
    }

    #[doc="Sets the GPIO15 field."]
    #[inline] pub fn set_gpio15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO14 interrupt."]
    #[inline] pub fn gpio14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GPIO14 != 0"]
    #[inline] pub fn test_gpio14(&self) -> bool {
        self.gpio14() != 0
    }

    #[doc="Sets the GPIO14 field."]
    #[inline] pub fn set_gpio14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="GPIO13 interrupt."]
    #[inline] pub fn gpio13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if GPIO13 != 0"]
    #[inline] pub fn test_gpio13(&self) -> bool {
        self.gpio13() != 0
    }

    #[doc="Sets the GPIO13 field."]
    #[inline] pub fn set_gpio13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO12 interrupt."]
    #[inline] pub fn gpio12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO12 != 0"]
    #[inline] pub fn test_gpio12(&self) -> bool {
        self.gpio12() != 0
    }

    #[doc="Sets the GPIO12 field."]
    #[inline] pub fn set_gpio12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO11 interrupt."]
    #[inline] pub fn gpio11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO11 != 0"]
    #[inline] pub fn test_gpio11(&self) -> bool {
        self.gpio11() != 0
    }

    #[doc="Sets the GPIO11 field."]
    #[inline] pub fn set_gpio11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO10 interrupt."]
    #[inline] pub fn gpio10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GPIO10 != 0"]
    #[inline] pub fn test_gpio10(&self) -> bool {
        self.gpio10() != 0
    }

    #[doc="Sets the GPIO10 field."]
    #[inline] pub fn set_gpio10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO9 interrupt."]
    #[inline] pub fn gpio9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GPIO9 != 0"]
    #[inline] pub fn test_gpio9(&self) -> bool {
        self.gpio9() != 0
    }

    #[doc="Sets the GPIO9 field."]
    #[inline] pub fn set_gpio9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO8 interrupt."]
    #[inline] pub fn gpio8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO8 != 0"]
    #[inline] pub fn test_gpio8(&self) -> bool {
        self.gpio8() != 0
    }

    #[doc="Sets the GPIO8 field."]
    #[inline] pub fn set_gpio8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO7 interrupt."]
    #[inline] pub fn gpio7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO7 != 0"]
    #[inline] pub fn test_gpio7(&self) -> bool {
        self.gpio7() != 0
    }

    #[doc="Sets the GPIO7 field."]
    #[inline] pub fn set_gpio7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO6 interrupt."]
    #[inline] pub fn gpio6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIO6 != 0"]
    #[inline] pub fn test_gpio6(&self) -> bool {
        self.gpio6() != 0
    }

    #[doc="Sets the GPIO6 field."]
    #[inline] pub fn set_gpio6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO5 interrupt."]
    #[inline] pub fn gpio5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIO5 != 0"]
    #[inline] pub fn test_gpio5(&self) -> bool {
        self.gpio5() != 0
    }

    #[doc="Sets the GPIO5 field."]
    #[inline] pub fn set_gpio5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO4 interrupt."]
    #[inline] pub fn gpio4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO4 != 0"]
    #[inline] pub fn test_gpio4(&self) -> bool {
        self.gpio4() != 0
    }

    #[doc="Sets the GPIO4 field."]
    #[inline] pub fn set_gpio4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO3 interrupt."]
    #[inline] pub fn gpio3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO3 != 0"]
    #[inline] pub fn test_gpio3(&self) -> bool {
        self.gpio3() != 0
    }

    #[doc="Sets the GPIO3 field."]
    #[inline] pub fn set_gpio3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO2 interrupt."]
    #[inline] pub fn gpio2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIO2 != 0"]
    #[inline] pub fn test_gpio2(&self) -> bool {
        self.gpio2() != 0
    }

    #[doc="Sets the GPIO2 field."]
    #[inline] pub fn set_gpio2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO1 interrupt."]
    #[inline] pub fn gpio1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIO1 != 0"]
    #[inline] pub fn test_gpio1(&self) -> bool {
        self.gpio1() != 0
    }

    #[doc="Sets the GPIO1 field."]
    #[inline] pub fn set_gpio1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO0 interrupt."]
    #[inline] pub fn gpio0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO0 != 0"]
    #[inline] pub fn test_gpio0(&self) -> bool {
        self.gpio0() != 0
    }

    #[doc="Sets the GPIO0 field."]
    #[inline] pub fn set_gpio0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Int0clr {
    #[inline]
    fn from(other: u32) -> Self {
         Int0clr(other)
    }
}

impl ::core::fmt::Display for Int0clr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Int0clr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio31() != 0 { try!(write!(f, " gpio31"))}
        if self.gpio30() != 0 { try!(write!(f, " gpio30"))}
        if self.gpio29() != 0 { try!(write!(f, " gpio29"))}
        if self.gpio28() != 0 { try!(write!(f, " gpio28"))}
        if self.gpio27() != 0 { try!(write!(f, " gpio27"))}
        if self.gpio26() != 0 { try!(write!(f, " gpio26"))}
        if self.gpio25() != 0 { try!(write!(f, " gpio25"))}
        if self.gpio24() != 0 { try!(write!(f, " gpio24"))}
        if self.gpio23() != 0 { try!(write!(f, " gpio23"))}
        if self.gpio22() != 0 { try!(write!(f, " gpio22"))}
        if self.gpio21() != 0 { try!(write!(f, " gpio21"))}
        if self.gpio20() != 0 { try!(write!(f, " gpio20"))}
        if self.gpio19() != 0 { try!(write!(f, " gpio19"))}
        if self.gpio18() != 0 { try!(write!(f, " gpio18"))}
        if self.gpio17() != 0 { try!(write!(f, " gpio17"))}
        if self.gpio16() != 0 { try!(write!(f, " gpio16"))}
        if self.gpio15() != 0 { try!(write!(f, " gpio15"))}
        if self.gpio14() != 0 { try!(write!(f, " gpio14"))}
        if self.gpio13() != 0 { try!(write!(f, " gpio13"))}
        if self.gpio12() != 0 { try!(write!(f, " gpio12"))}
        if self.gpio11() != 0 { try!(write!(f, " gpio11"))}
        if self.gpio10() != 0 { try!(write!(f, " gpio10"))}
        if self.gpio9() != 0 { try!(write!(f, " gpio9"))}
        if self.gpio8() != 0 { try!(write!(f, " gpio8"))}
        if self.gpio7() != 0 { try!(write!(f, " gpio7"))}
        if self.gpio6() != 0 { try!(write!(f, " gpio6"))}
        if self.gpio5() != 0 { try!(write!(f, " gpio5"))}
        if self.gpio4() != 0 { try!(write!(f, " gpio4"))}
        if self.gpio3() != 0 { try!(write!(f, " gpio3"))}
        if self.gpio2() != 0 { try!(write!(f, " gpio2"))}
        if self.gpio1() != 0 { try!(write!(f, " gpio1"))}
        if self.gpio0() != 0 { try!(write!(f, " gpio0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers 31-0: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Int0set(pub u32);
impl Int0set {
    #[doc="GPIO31 interrupt."]
    #[inline] pub fn gpio31(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GPIO31 != 0"]
    #[inline] pub fn test_gpio31(&self) -> bool {
        self.gpio31() != 0
    }

    #[doc="Sets the GPIO31 field."]
    #[inline] pub fn set_gpio31<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="GPIO30 interrupt."]
    #[inline] pub fn gpio30(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if GPIO30 != 0"]
    #[inline] pub fn test_gpio30(&self) -> bool {
        self.gpio30() != 0
    }

    #[doc="Sets the GPIO30 field."]
    #[inline] pub fn set_gpio30<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="GPIO29 interrupt."]
    #[inline] pub fn gpio29(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if GPIO29 != 0"]
    #[inline] pub fn test_gpio29(&self) -> bool {
        self.gpio29() != 0
    }

    #[doc="Sets the GPIO29 field."]
    #[inline] pub fn set_gpio29<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="GPIO28 interrupt."]
    #[inline] pub fn gpio28(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if GPIO28 != 0"]
    #[inline] pub fn test_gpio28(&self) -> bool {
        self.gpio28() != 0
    }

    #[doc="Sets the GPIO28 field."]
    #[inline] pub fn set_gpio28<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="GPIO27 interrupt."]
    #[inline] pub fn gpio27(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if GPIO27 != 0"]
    #[inline] pub fn test_gpio27(&self) -> bool {
        self.gpio27() != 0
    }

    #[doc="Sets the GPIO27 field."]
    #[inline] pub fn set_gpio27<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="GPIO26 interrupt."]
    #[inline] pub fn gpio26(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if GPIO26 != 0"]
    #[inline] pub fn test_gpio26(&self) -> bool {
        self.gpio26() != 0
    }

    #[doc="Sets the GPIO26 field."]
    #[inline] pub fn set_gpio26<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="GPIO25 interrupt."]
    #[inline] pub fn gpio25(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if GPIO25 != 0"]
    #[inline] pub fn test_gpio25(&self) -> bool {
        self.gpio25() != 0
    }

    #[doc="Sets the GPIO25 field."]
    #[inline] pub fn set_gpio25<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="GPIO24 interrupt."]
    #[inline] pub fn gpio24(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if GPIO24 != 0"]
    #[inline] pub fn test_gpio24(&self) -> bool {
        self.gpio24() != 0
    }

    #[doc="Sets the GPIO24 field."]
    #[inline] pub fn set_gpio24<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="GPIO23 interrupt."]
    #[inline] pub fn gpio23(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if GPIO23 != 0"]
    #[inline] pub fn test_gpio23(&self) -> bool {
        self.gpio23() != 0
    }

    #[doc="Sets the GPIO23 field."]
    #[inline] pub fn set_gpio23<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="GPIO22 interrupt."]
    #[inline] pub fn gpio22(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if GPIO22 != 0"]
    #[inline] pub fn test_gpio22(&self) -> bool {
        self.gpio22() != 0
    }

    #[doc="Sets the GPIO22 field."]
    #[inline] pub fn set_gpio22<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="GPIO21 interrupt."]
    #[inline] pub fn gpio21(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if GPIO21 != 0"]
    #[inline] pub fn test_gpio21(&self) -> bool {
        self.gpio21() != 0
    }

    #[doc="Sets the GPIO21 field."]
    #[inline] pub fn set_gpio21<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="GPIO20 interrupt."]
    #[inline] pub fn gpio20(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if GPIO20 != 0"]
    #[inline] pub fn test_gpio20(&self) -> bool {
        self.gpio20() != 0
    }

    #[doc="Sets the GPIO20 field."]
    #[inline] pub fn set_gpio20<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="GPIO19 interrupt."]
    #[inline] pub fn gpio19(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if GPIO19 != 0"]
    #[inline] pub fn test_gpio19(&self) -> bool {
        self.gpio19() != 0
    }

    #[doc="Sets the GPIO19 field."]
    #[inline] pub fn set_gpio19<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="GPIO18interrupt."]
    #[inline] pub fn gpio18(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if GPIO18 != 0"]
    #[inline] pub fn test_gpio18(&self) -> bool {
        self.gpio18() != 0
    }

    #[doc="Sets the GPIO18 field."]
    #[inline] pub fn set_gpio18<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="GPIO17 interrupt."]
    #[inline] pub fn gpio17(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if GPIO17 != 0"]
    #[inline] pub fn test_gpio17(&self) -> bool {
        self.gpio17() != 0
    }

    #[doc="Sets the GPIO17 field."]
    #[inline] pub fn set_gpio17<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO16 interrupt."]
    #[inline] pub fn gpio16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO16 != 0"]
    #[inline] pub fn test_gpio16(&self) -> bool {
        self.gpio16() != 0
    }

    #[doc="Sets the GPIO16 field."]
    #[inline] pub fn set_gpio16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO15 interrupt."]
    #[inline] pub fn gpio15(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO15 != 0"]
    #[inline] pub fn test_gpio15(&self) -> bool {
        self.gpio15() != 0
    }

    #[doc="Sets the GPIO15 field."]
    #[inline] pub fn set_gpio15<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO14 interrupt."]
    #[inline] pub fn gpio14(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GPIO14 != 0"]
    #[inline] pub fn test_gpio14(&self) -> bool {
        self.gpio14() != 0
    }

    #[doc="Sets the GPIO14 field."]
    #[inline] pub fn set_gpio14<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="GPIO13 interrupt."]
    #[inline] pub fn gpio13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if GPIO13 != 0"]
    #[inline] pub fn test_gpio13(&self) -> bool {
        self.gpio13() != 0
    }

    #[doc="Sets the GPIO13 field."]
    #[inline] pub fn set_gpio13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO12 interrupt."]
    #[inline] pub fn gpio12(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO12 != 0"]
    #[inline] pub fn test_gpio12(&self) -> bool {
        self.gpio12() != 0
    }

    #[doc="Sets the GPIO12 field."]
    #[inline] pub fn set_gpio12<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO11 interrupt."]
    #[inline] pub fn gpio11(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO11 != 0"]
    #[inline] pub fn test_gpio11(&self) -> bool {
        self.gpio11() != 0
    }

    #[doc="Sets the GPIO11 field."]
    #[inline] pub fn set_gpio11<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO10 interrupt."]
    #[inline] pub fn gpio10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GPIO10 != 0"]
    #[inline] pub fn test_gpio10(&self) -> bool {
        self.gpio10() != 0
    }

    #[doc="Sets the GPIO10 field."]
    #[inline] pub fn set_gpio10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO9 interrupt."]
    #[inline] pub fn gpio9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GPIO9 != 0"]
    #[inline] pub fn test_gpio9(&self) -> bool {
        self.gpio9() != 0
    }

    #[doc="Sets the GPIO9 field."]
    #[inline] pub fn set_gpio9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO8 interrupt."]
    #[inline] pub fn gpio8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO8 != 0"]
    #[inline] pub fn test_gpio8(&self) -> bool {
        self.gpio8() != 0
    }

    #[doc="Sets the GPIO8 field."]
    #[inline] pub fn set_gpio8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO7 interrupt."]
    #[inline] pub fn gpio7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO7 != 0"]
    #[inline] pub fn test_gpio7(&self) -> bool {
        self.gpio7() != 0
    }

    #[doc="Sets the GPIO7 field."]
    #[inline] pub fn set_gpio7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO6 interrupt."]
    #[inline] pub fn gpio6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIO6 != 0"]
    #[inline] pub fn test_gpio6(&self) -> bool {
        self.gpio6() != 0
    }

    #[doc="Sets the GPIO6 field."]
    #[inline] pub fn set_gpio6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO5 interrupt."]
    #[inline] pub fn gpio5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIO5 != 0"]
    #[inline] pub fn test_gpio5(&self) -> bool {
        self.gpio5() != 0
    }

    #[doc="Sets the GPIO5 field."]
    #[inline] pub fn set_gpio5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO4 interrupt."]
    #[inline] pub fn gpio4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO4 != 0"]
    #[inline] pub fn test_gpio4(&self) -> bool {
        self.gpio4() != 0
    }

    #[doc="Sets the GPIO4 field."]
    #[inline] pub fn set_gpio4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO3 interrupt."]
    #[inline] pub fn gpio3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO3 != 0"]
    #[inline] pub fn test_gpio3(&self) -> bool {
        self.gpio3() != 0
    }

    #[doc="Sets the GPIO3 field."]
    #[inline] pub fn set_gpio3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO2 interrupt."]
    #[inline] pub fn gpio2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIO2 != 0"]
    #[inline] pub fn test_gpio2(&self) -> bool {
        self.gpio2() != 0
    }

    #[doc="Sets the GPIO2 field."]
    #[inline] pub fn set_gpio2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO1 interrupt."]
    #[inline] pub fn gpio1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIO1 != 0"]
    #[inline] pub fn test_gpio1(&self) -> bool {
        self.gpio1() != 0
    }

    #[doc="Sets the GPIO1 field."]
    #[inline] pub fn set_gpio1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO0 interrupt."]
    #[inline] pub fn gpio0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO0 != 0"]
    #[inline] pub fn test_gpio0(&self) -> bool {
        self.gpio0() != 0
    }

    #[doc="Sets the GPIO0 field."]
    #[inline] pub fn set_gpio0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Int0set {
    #[inline]
    fn from(other: u32) -> Self {
         Int0set(other)
    }
}

impl ::core::fmt::Display for Int0set {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Int0set {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio31() != 0 { try!(write!(f, " gpio31"))}
        if self.gpio30() != 0 { try!(write!(f, " gpio30"))}
        if self.gpio29() != 0 { try!(write!(f, " gpio29"))}
        if self.gpio28() != 0 { try!(write!(f, " gpio28"))}
        if self.gpio27() != 0 { try!(write!(f, " gpio27"))}
        if self.gpio26() != 0 { try!(write!(f, " gpio26"))}
        if self.gpio25() != 0 { try!(write!(f, " gpio25"))}
        if self.gpio24() != 0 { try!(write!(f, " gpio24"))}
        if self.gpio23() != 0 { try!(write!(f, " gpio23"))}
        if self.gpio22() != 0 { try!(write!(f, " gpio22"))}
        if self.gpio21() != 0 { try!(write!(f, " gpio21"))}
        if self.gpio20() != 0 { try!(write!(f, " gpio20"))}
        if self.gpio19() != 0 { try!(write!(f, " gpio19"))}
        if self.gpio18() != 0 { try!(write!(f, " gpio18"))}
        if self.gpio17() != 0 { try!(write!(f, " gpio17"))}
        if self.gpio16() != 0 { try!(write!(f, " gpio16"))}
        if self.gpio15() != 0 { try!(write!(f, " gpio15"))}
        if self.gpio14() != 0 { try!(write!(f, " gpio14"))}
        if self.gpio13() != 0 { try!(write!(f, " gpio13"))}
        if self.gpio12() != 0 { try!(write!(f, " gpio12"))}
        if self.gpio11() != 0 { try!(write!(f, " gpio11"))}
        if self.gpio10() != 0 { try!(write!(f, " gpio10"))}
        if self.gpio9() != 0 { try!(write!(f, " gpio9"))}
        if self.gpio8() != 0 { try!(write!(f, " gpio8"))}
        if self.gpio7() != 0 { try!(write!(f, " gpio7"))}
        if self.gpio6() != 0 { try!(write!(f, " gpio6"))}
        if self.gpio5() != 0 { try!(write!(f, " gpio5"))}
        if self.gpio4() != 0 { try!(write!(f, " gpio4"))}
        if self.gpio3() != 0 { try!(write!(f, " gpio3"))}
        if self.gpio2() != 0 { try!(write!(f, " gpio2"))}
        if self.gpio1() != 0 { try!(write!(f, " gpio1"))}
        if self.gpio0() != 0 { try!(write!(f, " gpio0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers 49-32: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Int1en(pub u32);
impl Int1en {
    #[doc="GPIO49 interrupt."]
    #[inline] pub fn gpio49(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if GPIO49 != 0"]
    #[inline] pub fn test_gpio49(&self) -> bool {
        self.gpio49() != 0
    }

    #[doc="Sets the GPIO49 field."]
    #[inline] pub fn set_gpio49<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO48 interrupt."]
    #[inline] pub fn gpio48(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO48 != 0"]
    #[inline] pub fn test_gpio48(&self) -> bool {
        self.gpio48() != 0
    }

    #[doc="Sets the GPIO48 field."]
    #[inline] pub fn set_gpio48<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO47 interrupt."]
    #[inline] pub fn gpio47(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO47 != 0"]
    #[inline] pub fn test_gpio47(&self) -> bool {
        self.gpio47() != 0
    }

    #[doc="Sets the GPIO47 field."]
    #[inline] pub fn set_gpio47<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO46 interrupt."]
    #[inline] pub fn gpio46(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GPIO46 != 0"]
    #[inline] pub fn test_gpio46(&self) -> bool {
        self.gpio46() != 0
    }

    #[doc="Sets the GPIO46 field."]
    #[inline] pub fn set_gpio46<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="GPIO45 interrupt."]
    #[inline] pub fn gpio45(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if GPIO45 != 0"]
    #[inline] pub fn test_gpio45(&self) -> bool {
        self.gpio45() != 0
    }

    #[doc="Sets the GPIO45 field."]
    #[inline] pub fn set_gpio45<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO44 interrupt."]
    #[inline] pub fn gpio44(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO44 != 0"]
    #[inline] pub fn test_gpio44(&self) -> bool {
        self.gpio44() != 0
    }

    #[doc="Sets the GPIO44 field."]
    #[inline] pub fn set_gpio44<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO43 interrupt."]
    #[inline] pub fn gpio43(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO43 != 0"]
    #[inline] pub fn test_gpio43(&self) -> bool {
        self.gpio43() != 0
    }

    #[doc="Sets the GPIO43 field."]
    #[inline] pub fn set_gpio43<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO42 interrupt."]
    #[inline] pub fn gpio42(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GPIO42 != 0"]
    #[inline] pub fn test_gpio42(&self) -> bool {
        self.gpio42() != 0
    }

    #[doc="Sets the GPIO42 field."]
    #[inline] pub fn set_gpio42<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO41 interrupt."]
    #[inline] pub fn gpio41(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GPIO41 != 0"]
    #[inline] pub fn test_gpio41(&self) -> bool {
        self.gpio41() != 0
    }

    #[doc="Sets the GPIO41 field."]
    #[inline] pub fn set_gpio41<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO40 interrupt."]
    #[inline] pub fn gpio40(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO40 != 0"]
    #[inline] pub fn test_gpio40(&self) -> bool {
        self.gpio40() != 0
    }

    #[doc="Sets the GPIO40 field."]
    #[inline] pub fn set_gpio40<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO39 interrupt."]
    #[inline] pub fn gpio39(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO39 != 0"]
    #[inline] pub fn test_gpio39(&self) -> bool {
        self.gpio39() != 0
    }

    #[doc="Sets the GPIO39 field."]
    #[inline] pub fn set_gpio39<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO38 interrupt."]
    #[inline] pub fn gpio38(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIO38 != 0"]
    #[inline] pub fn test_gpio38(&self) -> bool {
        self.gpio38() != 0
    }

    #[doc="Sets the GPIO38 field."]
    #[inline] pub fn set_gpio38<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO37 interrupt."]
    #[inline] pub fn gpio37(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIO37 != 0"]
    #[inline] pub fn test_gpio37(&self) -> bool {
        self.gpio37() != 0
    }

    #[doc="Sets the GPIO37 field."]
    #[inline] pub fn set_gpio37<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO36 interrupt."]
    #[inline] pub fn gpio36(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO36 != 0"]
    #[inline] pub fn test_gpio36(&self) -> bool {
        self.gpio36() != 0
    }

    #[doc="Sets the GPIO36 field."]
    #[inline] pub fn set_gpio36<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO35 interrupt."]
    #[inline] pub fn gpio35(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO35 != 0"]
    #[inline] pub fn test_gpio35(&self) -> bool {
        self.gpio35() != 0
    }

    #[doc="Sets the GPIO35 field."]
    #[inline] pub fn set_gpio35<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO34 interrupt."]
    #[inline] pub fn gpio34(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIO34 != 0"]
    #[inline] pub fn test_gpio34(&self) -> bool {
        self.gpio34() != 0
    }

    #[doc="Sets the GPIO34 field."]
    #[inline] pub fn set_gpio34<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO33 interrupt."]
    #[inline] pub fn gpio33(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIO33 != 0"]
    #[inline] pub fn test_gpio33(&self) -> bool {
        self.gpio33() != 0
    }

    #[doc="Sets the GPIO33 field."]
    #[inline] pub fn set_gpio33<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO32 interrupt."]
    #[inline] pub fn gpio32(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO32 != 0"]
    #[inline] pub fn test_gpio32(&self) -> bool {
        self.gpio32() != 0
    }

    #[doc="Sets the GPIO32 field."]
    #[inline] pub fn set_gpio32<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Int1en {
    #[inline]
    fn from(other: u32) -> Self {
         Int1en(other)
    }
}

impl ::core::fmt::Display for Int1en {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Int1en {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio49() != 0 { try!(write!(f, " gpio49"))}
        if self.gpio48() != 0 { try!(write!(f, " gpio48"))}
        if self.gpio47() != 0 { try!(write!(f, " gpio47"))}
        if self.gpio46() != 0 { try!(write!(f, " gpio46"))}
        if self.gpio45() != 0 { try!(write!(f, " gpio45"))}
        if self.gpio44() != 0 { try!(write!(f, " gpio44"))}
        if self.gpio43() != 0 { try!(write!(f, " gpio43"))}
        if self.gpio42() != 0 { try!(write!(f, " gpio42"))}
        if self.gpio41() != 0 { try!(write!(f, " gpio41"))}
        if self.gpio40() != 0 { try!(write!(f, " gpio40"))}
        if self.gpio39() != 0 { try!(write!(f, " gpio39"))}
        if self.gpio38() != 0 { try!(write!(f, " gpio38"))}
        if self.gpio37() != 0 { try!(write!(f, " gpio37"))}
        if self.gpio36() != 0 { try!(write!(f, " gpio36"))}
        if self.gpio35() != 0 { try!(write!(f, " gpio35"))}
        if self.gpio34() != 0 { try!(write!(f, " gpio34"))}
        if self.gpio33() != 0 { try!(write!(f, " gpio33"))}
        if self.gpio32() != 0 { try!(write!(f, " gpio32"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers 49-32: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Int1stat(pub u32);
impl Int1stat {
    #[doc="GPIO49 interrupt."]
    #[inline] pub fn gpio49(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if GPIO49 != 0"]
    #[inline] pub fn test_gpio49(&self) -> bool {
        self.gpio49() != 0
    }

    #[doc="Sets the GPIO49 field."]
    #[inline] pub fn set_gpio49<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO48 interrupt."]
    #[inline] pub fn gpio48(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO48 != 0"]
    #[inline] pub fn test_gpio48(&self) -> bool {
        self.gpio48() != 0
    }

    #[doc="Sets the GPIO48 field."]
    #[inline] pub fn set_gpio48<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO47 interrupt."]
    #[inline] pub fn gpio47(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO47 != 0"]
    #[inline] pub fn test_gpio47(&self) -> bool {
        self.gpio47() != 0
    }

    #[doc="Sets the GPIO47 field."]
    #[inline] pub fn set_gpio47<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO46 interrupt."]
    #[inline] pub fn gpio46(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GPIO46 != 0"]
    #[inline] pub fn test_gpio46(&self) -> bool {
        self.gpio46() != 0
    }

    #[doc="Sets the GPIO46 field."]
    #[inline] pub fn set_gpio46<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="GPIO45 interrupt."]
    #[inline] pub fn gpio45(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if GPIO45 != 0"]
    #[inline] pub fn test_gpio45(&self) -> bool {
        self.gpio45() != 0
    }

    #[doc="Sets the GPIO45 field."]
    #[inline] pub fn set_gpio45<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO44 interrupt."]
    #[inline] pub fn gpio44(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO44 != 0"]
    #[inline] pub fn test_gpio44(&self) -> bool {
        self.gpio44() != 0
    }

    #[doc="Sets the GPIO44 field."]
    #[inline] pub fn set_gpio44<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO43 interrupt."]
    #[inline] pub fn gpio43(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO43 != 0"]
    #[inline] pub fn test_gpio43(&self) -> bool {
        self.gpio43() != 0
    }

    #[doc="Sets the GPIO43 field."]
    #[inline] pub fn set_gpio43<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO42 interrupt."]
    #[inline] pub fn gpio42(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GPIO42 != 0"]
    #[inline] pub fn test_gpio42(&self) -> bool {
        self.gpio42() != 0
    }

    #[doc="Sets the GPIO42 field."]
    #[inline] pub fn set_gpio42<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO41 interrupt."]
    #[inline] pub fn gpio41(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GPIO41 != 0"]
    #[inline] pub fn test_gpio41(&self) -> bool {
        self.gpio41() != 0
    }

    #[doc="Sets the GPIO41 field."]
    #[inline] pub fn set_gpio41<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO40 interrupt."]
    #[inline] pub fn gpio40(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO40 != 0"]
    #[inline] pub fn test_gpio40(&self) -> bool {
        self.gpio40() != 0
    }

    #[doc="Sets the GPIO40 field."]
    #[inline] pub fn set_gpio40<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO39 interrupt."]
    #[inline] pub fn gpio39(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO39 != 0"]
    #[inline] pub fn test_gpio39(&self) -> bool {
        self.gpio39() != 0
    }

    #[doc="Sets the GPIO39 field."]
    #[inline] pub fn set_gpio39<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO38 interrupt."]
    #[inline] pub fn gpio38(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIO38 != 0"]
    #[inline] pub fn test_gpio38(&self) -> bool {
        self.gpio38() != 0
    }

    #[doc="Sets the GPIO38 field."]
    #[inline] pub fn set_gpio38<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO37 interrupt."]
    #[inline] pub fn gpio37(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIO37 != 0"]
    #[inline] pub fn test_gpio37(&self) -> bool {
        self.gpio37() != 0
    }

    #[doc="Sets the GPIO37 field."]
    #[inline] pub fn set_gpio37<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO36 interrupt."]
    #[inline] pub fn gpio36(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO36 != 0"]
    #[inline] pub fn test_gpio36(&self) -> bool {
        self.gpio36() != 0
    }

    #[doc="Sets the GPIO36 field."]
    #[inline] pub fn set_gpio36<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO35 interrupt."]
    #[inline] pub fn gpio35(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO35 != 0"]
    #[inline] pub fn test_gpio35(&self) -> bool {
        self.gpio35() != 0
    }

    #[doc="Sets the GPIO35 field."]
    #[inline] pub fn set_gpio35<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO34 interrupt."]
    #[inline] pub fn gpio34(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIO34 != 0"]
    #[inline] pub fn test_gpio34(&self) -> bool {
        self.gpio34() != 0
    }

    #[doc="Sets the GPIO34 field."]
    #[inline] pub fn set_gpio34<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO33 interrupt."]
    #[inline] pub fn gpio33(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIO33 != 0"]
    #[inline] pub fn test_gpio33(&self) -> bool {
        self.gpio33() != 0
    }

    #[doc="Sets the GPIO33 field."]
    #[inline] pub fn set_gpio33<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO32 interrupt."]
    #[inline] pub fn gpio32(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO32 != 0"]
    #[inline] pub fn test_gpio32(&self) -> bool {
        self.gpio32() != 0
    }

    #[doc="Sets the GPIO32 field."]
    #[inline] pub fn set_gpio32<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Int1stat {
    #[inline]
    fn from(other: u32) -> Self {
         Int1stat(other)
    }
}

impl ::core::fmt::Display for Int1stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Int1stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio49() != 0 { try!(write!(f, " gpio49"))}
        if self.gpio48() != 0 { try!(write!(f, " gpio48"))}
        if self.gpio47() != 0 { try!(write!(f, " gpio47"))}
        if self.gpio46() != 0 { try!(write!(f, " gpio46"))}
        if self.gpio45() != 0 { try!(write!(f, " gpio45"))}
        if self.gpio44() != 0 { try!(write!(f, " gpio44"))}
        if self.gpio43() != 0 { try!(write!(f, " gpio43"))}
        if self.gpio42() != 0 { try!(write!(f, " gpio42"))}
        if self.gpio41() != 0 { try!(write!(f, " gpio41"))}
        if self.gpio40() != 0 { try!(write!(f, " gpio40"))}
        if self.gpio39() != 0 { try!(write!(f, " gpio39"))}
        if self.gpio38() != 0 { try!(write!(f, " gpio38"))}
        if self.gpio37() != 0 { try!(write!(f, " gpio37"))}
        if self.gpio36() != 0 { try!(write!(f, " gpio36"))}
        if self.gpio35() != 0 { try!(write!(f, " gpio35"))}
        if self.gpio34() != 0 { try!(write!(f, " gpio34"))}
        if self.gpio33() != 0 { try!(write!(f, " gpio33"))}
        if self.gpio32() != 0 { try!(write!(f, " gpio32"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers 49-32: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Int1clr(pub u32);
impl Int1clr {
    #[doc="GPIO49 interrupt."]
    #[inline] pub fn gpio49(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if GPIO49 != 0"]
    #[inline] pub fn test_gpio49(&self) -> bool {
        self.gpio49() != 0
    }

    #[doc="Sets the GPIO49 field."]
    #[inline] pub fn set_gpio49<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO48 interrupt."]
    #[inline] pub fn gpio48(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO48 != 0"]
    #[inline] pub fn test_gpio48(&self) -> bool {
        self.gpio48() != 0
    }

    #[doc="Sets the GPIO48 field."]
    #[inline] pub fn set_gpio48<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO47 interrupt."]
    #[inline] pub fn gpio47(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO47 != 0"]
    #[inline] pub fn test_gpio47(&self) -> bool {
        self.gpio47() != 0
    }

    #[doc="Sets the GPIO47 field."]
    #[inline] pub fn set_gpio47<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO46 interrupt."]
    #[inline] pub fn gpio46(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GPIO46 != 0"]
    #[inline] pub fn test_gpio46(&self) -> bool {
        self.gpio46() != 0
    }

    #[doc="Sets the GPIO46 field."]
    #[inline] pub fn set_gpio46<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="GPIO45 interrupt."]
    #[inline] pub fn gpio45(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if GPIO45 != 0"]
    #[inline] pub fn test_gpio45(&self) -> bool {
        self.gpio45() != 0
    }

    #[doc="Sets the GPIO45 field."]
    #[inline] pub fn set_gpio45<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO44 interrupt."]
    #[inline] pub fn gpio44(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO44 != 0"]
    #[inline] pub fn test_gpio44(&self) -> bool {
        self.gpio44() != 0
    }

    #[doc="Sets the GPIO44 field."]
    #[inline] pub fn set_gpio44<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO43 interrupt."]
    #[inline] pub fn gpio43(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO43 != 0"]
    #[inline] pub fn test_gpio43(&self) -> bool {
        self.gpio43() != 0
    }

    #[doc="Sets the GPIO43 field."]
    #[inline] pub fn set_gpio43<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO42 interrupt."]
    #[inline] pub fn gpio42(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GPIO42 != 0"]
    #[inline] pub fn test_gpio42(&self) -> bool {
        self.gpio42() != 0
    }

    #[doc="Sets the GPIO42 field."]
    #[inline] pub fn set_gpio42<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO41 interrupt."]
    #[inline] pub fn gpio41(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GPIO41 != 0"]
    #[inline] pub fn test_gpio41(&self) -> bool {
        self.gpio41() != 0
    }

    #[doc="Sets the GPIO41 field."]
    #[inline] pub fn set_gpio41<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO40 interrupt."]
    #[inline] pub fn gpio40(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO40 != 0"]
    #[inline] pub fn test_gpio40(&self) -> bool {
        self.gpio40() != 0
    }

    #[doc="Sets the GPIO40 field."]
    #[inline] pub fn set_gpio40<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO39 interrupt."]
    #[inline] pub fn gpio39(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO39 != 0"]
    #[inline] pub fn test_gpio39(&self) -> bool {
        self.gpio39() != 0
    }

    #[doc="Sets the GPIO39 field."]
    #[inline] pub fn set_gpio39<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO38 interrupt."]
    #[inline] pub fn gpio38(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIO38 != 0"]
    #[inline] pub fn test_gpio38(&self) -> bool {
        self.gpio38() != 0
    }

    #[doc="Sets the GPIO38 field."]
    #[inline] pub fn set_gpio38<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO37 interrupt."]
    #[inline] pub fn gpio37(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIO37 != 0"]
    #[inline] pub fn test_gpio37(&self) -> bool {
        self.gpio37() != 0
    }

    #[doc="Sets the GPIO37 field."]
    #[inline] pub fn set_gpio37<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO36 interrupt."]
    #[inline] pub fn gpio36(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO36 != 0"]
    #[inline] pub fn test_gpio36(&self) -> bool {
        self.gpio36() != 0
    }

    #[doc="Sets the GPIO36 field."]
    #[inline] pub fn set_gpio36<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO35 interrupt."]
    #[inline] pub fn gpio35(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO35 != 0"]
    #[inline] pub fn test_gpio35(&self) -> bool {
        self.gpio35() != 0
    }

    #[doc="Sets the GPIO35 field."]
    #[inline] pub fn set_gpio35<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO34 interrupt."]
    #[inline] pub fn gpio34(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIO34 != 0"]
    #[inline] pub fn test_gpio34(&self) -> bool {
        self.gpio34() != 0
    }

    #[doc="Sets the GPIO34 field."]
    #[inline] pub fn set_gpio34<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO33 interrupt."]
    #[inline] pub fn gpio33(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIO33 != 0"]
    #[inline] pub fn test_gpio33(&self) -> bool {
        self.gpio33() != 0
    }

    #[doc="Sets the GPIO33 field."]
    #[inline] pub fn set_gpio33<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO32 interrupt."]
    #[inline] pub fn gpio32(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO32 != 0"]
    #[inline] pub fn test_gpio32(&self) -> bool {
        self.gpio32() != 0
    }

    #[doc="Sets the GPIO32 field."]
    #[inline] pub fn set_gpio32<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Int1clr {
    #[inline]
    fn from(other: u32) -> Self {
         Int1clr(other)
    }
}

impl ::core::fmt::Display for Int1clr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Int1clr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio49() != 0 { try!(write!(f, " gpio49"))}
        if self.gpio48() != 0 { try!(write!(f, " gpio48"))}
        if self.gpio47() != 0 { try!(write!(f, " gpio47"))}
        if self.gpio46() != 0 { try!(write!(f, " gpio46"))}
        if self.gpio45() != 0 { try!(write!(f, " gpio45"))}
        if self.gpio44() != 0 { try!(write!(f, " gpio44"))}
        if self.gpio43() != 0 { try!(write!(f, " gpio43"))}
        if self.gpio42() != 0 { try!(write!(f, " gpio42"))}
        if self.gpio41() != 0 { try!(write!(f, " gpio41"))}
        if self.gpio40() != 0 { try!(write!(f, " gpio40"))}
        if self.gpio39() != 0 { try!(write!(f, " gpio39"))}
        if self.gpio38() != 0 { try!(write!(f, " gpio38"))}
        if self.gpio37() != 0 { try!(write!(f, " gpio37"))}
        if self.gpio36() != 0 { try!(write!(f, " gpio36"))}
        if self.gpio35() != 0 { try!(write!(f, " gpio35"))}
        if self.gpio34() != 0 { try!(write!(f, " gpio34"))}
        if self.gpio33() != 0 { try!(write!(f, " gpio33"))}
        if self.gpio32() != 0 { try!(write!(f, " gpio32"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers 49-32: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Int1set(pub u32);
impl Int1set {
    #[doc="GPIO49 interrupt."]
    #[inline] pub fn gpio49(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if GPIO49 != 0"]
    #[inline] pub fn test_gpio49(&self) -> bool {
        self.gpio49() != 0
    }

    #[doc="Sets the GPIO49 field."]
    #[inline] pub fn set_gpio49<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="GPIO48 interrupt."]
    #[inline] pub fn gpio48(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GPIO48 != 0"]
    #[inline] pub fn test_gpio48(&self) -> bool {
        self.gpio48() != 0
    }

    #[doc="Sets the GPIO48 field."]
    #[inline] pub fn set_gpio48<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="GPIO47 interrupt."]
    #[inline] pub fn gpio47(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if GPIO47 != 0"]
    #[inline] pub fn test_gpio47(&self) -> bool {
        self.gpio47() != 0
    }

    #[doc="Sets the GPIO47 field."]
    #[inline] pub fn set_gpio47<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="GPIO46 interrupt."]
    #[inline] pub fn gpio46(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GPIO46 != 0"]
    #[inline] pub fn test_gpio46(&self) -> bool {
        self.gpio46() != 0
    }

    #[doc="Sets the GPIO46 field."]
    #[inline] pub fn set_gpio46<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="GPIO45 interrupt."]
    #[inline] pub fn gpio45(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if GPIO45 != 0"]
    #[inline] pub fn test_gpio45(&self) -> bool {
        self.gpio45() != 0
    }

    #[doc="Sets the GPIO45 field."]
    #[inline] pub fn set_gpio45<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="GPIO44 interrupt."]
    #[inline] pub fn gpio44(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if GPIO44 != 0"]
    #[inline] pub fn test_gpio44(&self) -> bool {
        self.gpio44() != 0
    }

    #[doc="Sets the GPIO44 field."]
    #[inline] pub fn set_gpio44<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="GPIO43 interrupt."]
    #[inline] pub fn gpio43(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if GPIO43 != 0"]
    #[inline] pub fn test_gpio43(&self) -> bool {
        self.gpio43() != 0
    }

    #[doc="Sets the GPIO43 field."]
    #[inline] pub fn set_gpio43<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="GPIO42 interrupt."]
    #[inline] pub fn gpio42(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GPIO42 != 0"]
    #[inline] pub fn test_gpio42(&self) -> bool {
        self.gpio42() != 0
    }

    #[doc="Sets the GPIO42 field."]
    #[inline] pub fn set_gpio42<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPIO41 interrupt."]
    #[inline] pub fn gpio41(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GPIO41 != 0"]
    #[inline] pub fn test_gpio41(&self) -> bool {
        self.gpio41() != 0
    }

    #[doc="Sets the GPIO41 field."]
    #[inline] pub fn set_gpio41<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPIO40 interrupt."]
    #[inline] pub fn gpio40(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GPIO40 != 0"]
    #[inline] pub fn test_gpio40(&self) -> bool {
        self.gpio40() != 0
    }

    #[doc="Sets the GPIO40 field."]
    #[inline] pub fn set_gpio40<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPIO39 interrupt."]
    #[inline] pub fn gpio39(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GPIO39 != 0"]
    #[inline] pub fn test_gpio39(&self) -> bool {
        self.gpio39() != 0
    }

    #[doc="Sets the GPIO39 field."]
    #[inline] pub fn set_gpio39<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPIO38 interrupt."]
    #[inline] pub fn gpio38(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GPIO38 != 0"]
    #[inline] pub fn test_gpio38(&self) -> bool {
        self.gpio38() != 0
    }

    #[doc="Sets the GPIO38 field."]
    #[inline] pub fn set_gpio38<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPIO37 interrupt."]
    #[inline] pub fn gpio37(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if GPIO37 != 0"]
    #[inline] pub fn test_gpio37(&self) -> bool {
        self.gpio37() != 0
    }

    #[doc="Sets the GPIO37 field."]
    #[inline] pub fn set_gpio37<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPIO36 interrupt."]
    #[inline] pub fn gpio36(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GPIO36 != 0"]
    #[inline] pub fn test_gpio36(&self) -> bool {
        self.gpio36() != 0
    }

    #[doc="Sets the GPIO36 field."]
    #[inline] pub fn set_gpio36<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPIO35 interrupt."]
    #[inline] pub fn gpio35(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIO35 != 0"]
    #[inline] pub fn test_gpio35(&self) -> bool {
        self.gpio35() != 0
    }

    #[doc="Sets the GPIO35 field."]
    #[inline] pub fn set_gpio35<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPIO34 interrupt."]
    #[inline] pub fn gpio34(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GPIO34 != 0"]
    #[inline] pub fn test_gpio34(&self) -> bool {
        self.gpio34() != 0
    }

    #[doc="Sets the GPIO34 field."]
    #[inline] pub fn set_gpio34<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPIO33 interrupt."]
    #[inline] pub fn gpio33(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if GPIO33 != 0"]
    #[inline] pub fn test_gpio33(&self) -> bool {
        self.gpio33() != 0
    }

    #[doc="Sets the GPIO33 field."]
    #[inline] pub fn set_gpio33<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="GPIO32 interrupt."]
    #[inline] pub fn gpio32(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO32 != 0"]
    #[inline] pub fn test_gpio32(&self) -> bool {
        self.gpio32() != 0
    }

    #[doc="Sets the GPIO32 field."]
    #[inline] pub fn set_gpio32<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Int1set {
    #[inline]
    fn from(other: u32) -> Self {
         Int1set(other)
    }
}

impl ::core::fmt::Display for Int1set {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Int1set {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio49() != 0 { try!(write!(f, " gpio49"))}
        if self.gpio48() != 0 { try!(write!(f, " gpio48"))}
        if self.gpio47() != 0 { try!(write!(f, " gpio47"))}
        if self.gpio46() != 0 { try!(write!(f, " gpio46"))}
        if self.gpio45() != 0 { try!(write!(f, " gpio45"))}
        if self.gpio44() != 0 { try!(write!(f, " gpio44"))}
        if self.gpio43() != 0 { try!(write!(f, " gpio43"))}
        if self.gpio42() != 0 { try!(write!(f, " gpio42"))}
        if self.gpio41() != 0 { try!(write!(f, " gpio41"))}
        if self.gpio40() != 0 { try!(write!(f, " gpio40"))}
        if self.gpio39() != 0 { try!(write!(f, " gpio39"))}
        if self.gpio38() != 0 { try!(write!(f, " gpio38"))}
        if self.gpio37() != 0 { try!(write!(f, " gpio37"))}
        if self.gpio36() != 0 { try!(write!(f, " gpio36"))}
        if self.gpio35() != 0 { try!(write!(f, " gpio35"))}
        if self.gpio34() != 0 { try!(write!(f, " gpio34"))}
        if self.gpio33() != 0 { try!(write!(f, " gpio33"))}
        if self.gpio32() != 0 { try!(write!(f, " gpio32"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


