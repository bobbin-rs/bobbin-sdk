#[allow(unused_imports)] use bobbin_common::*;

periph!( IOSLAVE, Ioslave, _IOSLAVE, IoslavePeriph, 0x50000000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="IOSLAVE Peripheral"]
pub struct IoslavePeriph(pub usize); 

impl super::sig::Signal<super::sig::Slscl> for Ioslave {}
impl super::sig::SignalSlScl<super::sig::Slscl> for Ioslave {}
impl super::sig::Signal<super::sig::Slscllb> for Ioslave {}
impl super::sig::SignalSlSclLb<super::sig::Slscllb> for Ioslave {}
impl super::sig::Signal<super::sig::Slsda> for Ioslave {}
impl super::sig::SignalSlSda<super::sig::Slsda> for Ioslave {}
impl super::sig::Signal<super::sig::Slsdalb> for Ioslave {}
impl super::sig::SignalSlSdaLb<super::sig::Slsdalb> for Ioslave {}
impl super::sig::Signal<super::sig::Slsmiso> for Ioslave {}
impl super::sig::SignalSlMiso<super::sig::Slsmiso> for Ioslave {}
impl super::sig::Signal<super::sig::Slsmosi> for Ioslave {}
impl super::sig::SignalSlMosi<super::sig::Slsmosi> for Ioslave {}
impl super::sig::Signal<super::sig::Slswir3> for Ioslave {}
impl super::sig::SignalSlWir3<super::sig::Slswir3> for Ioslave {}
impl super::sig::Signal<super::sig::Slnce> for Ioslave {}
impl super::sig::SignalSlCe<super::sig::Slnce> for Ioslave {}
impl super::sig::Signal<super::sig::Slint> for Ioslave {}
impl super::sig::SignalSlInt<super::sig::Slint> for Ioslave {}
impl super::sig::Signal<super::sig::Slintgp> for Ioslave {}
impl super::sig::SignalSlIntGp<super::sig::Slintgp> for Ioslave {}
impl super::sig::Signal<super::sig::Slmisolb> for Ioslave {}
impl super::sig::SignalSlMisoLb<super::sig::Slmisolb> for Ioslave {}
impl super::sig::Signal<super::sig::Slmosilb> for Ioslave {}
impl super::sig::SignalSlMosiLb<super::sig::Slmosilb> for Ioslave {}
impl super::sig::Signal<super::sig::Slwir3lb> for Ioslave {}
impl super::sig::SignalSlWir3Lb<super::sig::Slwir3lb> for Ioslave {}


impl IoslavePeriph {
    #[doc="Get the *mut pointer for the FIFOPTR register."]
    #[inline] pub fn fifoptr_mut(&self) -> *mut Fifoptr { 
        (self.0 + 0x100) as *mut Fifoptr
    }

    #[doc="Get the *const pointer for the FIFOPTR register."]
    #[inline] pub fn fifoptr_ptr(&self) -> *const Fifoptr { 
           self.fifoptr_mut()
    }

    #[doc="Read the FIFOPTR register."]
    #[inline] pub fn fifoptr(&self) -> Fifoptr { 
        unsafe {
            read_volatile(self.fifoptr_ptr())
        }
    }

    #[doc="Write the FIFOPTR register."]
    #[inline] pub fn set_fifoptr<F: FnOnce(Fifoptr) -> Fifoptr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifoptr_mut(), f(Fifoptr(0)));
        }
        self
    }

    #[doc="Modify the FIFOPTR register."]
    #[inline] pub fn with_fifoptr<F: FnOnce(Fifoptr) -> Fifoptr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifoptr_mut(), f(self.fifoptr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIFOCFG register."]
    #[inline] pub fn fifocfg_mut(&self) -> *mut Fifocfg { 
        (self.0 + 0x104) as *mut Fifocfg
    }

    #[doc="Get the *const pointer for the FIFOCFG register."]
    #[inline] pub fn fifocfg_ptr(&self) -> *const Fifocfg { 
           self.fifocfg_mut()
    }

    #[doc="Read the FIFOCFG register."]
    #[inline] pub fn fifocfg(&self) -> Fifocfg { 
        unsafe {
            read_volatile(self.fifocfg_ptr())
        }
    }

    #[doc="Write the FIFOCFG register."]
    #[inline] pub fn set_fifocfg<F: FnOnce(Fifocfg) -> Fifocfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifocfg_mut(), f(Fifocfg(0)));
        }
        self
    }

    #[doc="Modify the FIFOCFG register."]
    #[inline] pub fn with_fifocfg<F: FnOnce(Fifocfg) -> Fifocfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifocfg_mut(), f(self.fifocfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIFOTHR register."]
    #[inline] pub fn fifothr_mut(&self) -> *mut Fifothr { 
        (self.0 + 0x108) as *mut Fifothr
    }

    #[doc="Get the *const pointer for the FIFOTHR register."]
    #[inline] pub fn fifothr_ptr(&self) -> *const Fifothr { 
           self.fifothr_mut()
    }

    #[doc="Read the FIFOTHR register."]
    #[inline] pub fn fifothr(&self) -> Fifothr { 
        unsafe {
            read_volatile(self.fifothr_ptr())
        }
    }

    #[doc="Write the FIFOTHR register."]
    #[inline] pub fn set_fifothr<F: FnOnce(Fifothr) -> Fifothr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifothr_mut(), f(Fifothr(0)));
        }
        self
    }

    #[doc="Modify the FIFOTHR register."]
    #[inline] pub fn with_fifothr<F: FnOnce(Fifothr) -> Fifothr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifothr_mut(), f(self.fifothr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FUPD register."]
    #[inline] pub fn fupd_mut(&self) -> *mut Fupd { 
        (self.0 + 0x10c) as *mut Fupd
    }

    #[doc="Get the *const pointer for the FUPD register."]
    #[inline] pub fn fupd_ptr(&self) -> *const Fupd { 
           self.fupd_mut()
    }

    #[doc="Read the FUPD register."]
    #[inline] pub fn fupd(&self) -> Fupd { 
        unsafe {
            read_volatile(self.fupd_ptr())
        }
    }

    #[doc="Write the FUPD register."]
    #[inline] pub fn set_fupd<F: FnOnce(Fupd) -> Fupd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fupd_mut(), f(Fupd(0)));
        }
        self
    }

    #[doc="Modify the FUPD register."]
    #[inline] pub fn with_fupd<F: FnOnce(Fupd) -> Fupd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fupd_mut(), f(self.fupd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIFOCTR register."]
    #[inline] pub fn fifoctr_mut(&self) -> *mut Fifoctr { 
        (self.0 + 0x110) as *mut Fifoctr
    }

    #[doc="Get the *const pointer for the FIFOCTR register."]
    #[inline] pub fn fifoctr_ptr(&self) -> *const Fifoctr { 
           self.fifoctr_mut()
    }

    #[doc="Read the FIFOCTR register."]
    #[inline] pub fn fifoctr(&self) -> Fifoctr { 
        unsafe {
            read_volatile(self.fifoctr_ptr())
        }
    }

    #[doc="Write the FIFOCTR register."]
    #[inline] pub fn set_fifoctr<F: FnOnce(Fifoctr) -> Fifoctr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifoctr_mut(), f(Fifoctr(0)));
        }
        self
    }

    #[doc="Modify the FIFOCTR register."]
    #[inline] pub fn with_fifoctr<F: FnOnce(Fifoctr) -> Fifoctr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifoctr_mut(), f(self.fifoctr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIFOINC register."]
    #[inline] pub fn fifoinc_mut(&self) -> *mut Fifoinc { 
        (self.0 + 0x114) as *mut Fifoinc
    }

    #[doc="Get the *const pointer for the FIFOINC register."]
    #[inline] pub fn fifoinc_ptr(&self) -> *const Fifoinc { 
           self.fifoinc_mut()
    }

    #[doc="Read the FIFOINC register."]
    #[inline] pub fn fifoinc(&self) -> Fifoinc { 
        unsafe {
            read_volatile(self.fifoinc_ptr())
        }
    }

    #[doc="Write the FIFOINC register."]
    #[inline] pub fn set_fifoinc<F: FnOnce(Fifoinc) -> Fifoinc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifoinc_mut(), f(Fifoinc(0)));
        }
        self
    }

    #[doc="Modify the FIFOINC register."]
    #[inline] pub fn with_fifoinc<F: FnOnce(Fifoinc) -> Fifoinc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifoinc_mut(), f(self.fifoinc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFG register."]
    #[inline] pub fn cfg_mut(&self) -> *mut Cfg { 
        (self.0 + 0x118) as *mut Cfg
    }

    #[doc="Get the *const pointer for the CFG register."]
    #[inline] pub fn cfg_ptr(&self) -> *const Cfg { 
           self.cfg_mut()
    }

    #[doc="Read the CFG register."]
    #[inline] pub fn cfg(&self) -> Cfg { 
        unsafe {
            read_volatile(self.cfg_ptr())
        }
    }

    #[doc="Write the CFG register."]
    #[inline] pub fn set_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(), f(Cfg(0)));
        }
        self
    }

    #[doc="Modify the CFG register."]
    #[inline] pub fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(), f(self.cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PRENC register."]
    #[inline] pub fn prenc_mut(&self) -> *mut Prenc { 
        (self.0 + 0x11c) as *mut Prenc
    }

    #[doc="Get the *const pointer for the PRENC register."]
    #[inline] pub fn prenc_ptr(&self) -> *const Prenc { 
           self.prenc_mut()
    }

    #[doc="Read the PRENC register."]
    #[inline] pub fn prenc(&self) -> Prenc { 
        unsafe {
            read_volatile(self.prenc_ptr())
        }
    }

    #[doc="Write the PRENC register."]
    #[inline] pub fn set_prenc<F: FnOnce(Prenc) -> Prenc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.prenc_mut(), f(Prenc(0)));
        }
        self
    }

    #[doc="Modify the PRENC register."]
    #[inline] pub fn with_prenc<F: FnOnce(Prenc) -> Prenc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.prenc_mut(), f(self.prenc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IOINTCTL register."]
    #[inline] pub fn iointctl_mut(&self) -> *mut Iointctl { 
        (self.0 + 0x120) as *mut Iointctl
    }

    #[doc="Get the *const pointer for the IOINTCTL register."]
    #[inline] pub fn iointctl_ptr(&self) -> *const Iointctl { 
           self.iointctl_mut()
    }

    #[doc="Read the IOINTCTL register."]
    #[inline] pub fn iointctl(&self) -> Iointctl { 
        unsafe {
            read_volatile(self.iointctl_ptr())
        }
    }

    #[doc="Write the IOINTCTL register."]
    #[inline] pub fn set_iointctl<F: FnOnce(Iointctl) -> Iointctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iointctl_mut(), f(Iointctl(0)));
        }
        self
    }

    #[doc="Modify the IOINTCTL register."]
    #[inline] pub fn with_iointctl<F: FnOnce(Iointctl) -> Iointctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iointctl_mut(), f(self.iointctl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the GENADD register."]
    #[inline] pub fn genadd_mut(&self) -> *mut Genadd { 
        (self.0 + 0x124) as *mut Genadd
    }

    #[doc="Get the *const pointer for the GENADD register."]
    #[inline] pub fn genadd_ptr(&self) -> *const Genadd { 
           self.genadd_mut()
    }

    #[doc="Read the GENADD register."]
    #[inline] pub fn genadd(&self) -> Genadd { 
        unsafe {
            read_volatile(self.genadd_ptr())
        }
    }

    #[doc="Write the GENADD register."]
    #[inline] pub fn set_genadd<F: FnOnce(Genadd) -> Genadd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.genadd_mut(), f(Genadd(0)));
        }
        self
    }

    #[doc="Modify the GENADD register."]
    #[inline] pub fn with_genadd<F: FnOnce(Genadd) -> Genadd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.genadd_mut(), f(self.genadd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTEN register."]
    #[inline] pub fn inten_mut(&self) -> *mut Inten { 
        (self.0 + 0x200) as *mut Inten
    }

    #[doc="Get the *const pointer for the INTEN register."]
    #[inline] pub fn inten_ptr(&self) -> *const Inten { 
           self.inten_mut()
    }

    #[doc="Read the INTEN register."]
    #[inline] pub fn inten(&self) -> Inten { 
        unsafe {
            read_volatile(self.inten_ptr())
        }
    }

    #[doc="Write the INTEN register."]
    #[inline] pub fn set_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(), f(Inten(0)));
        }
        self
    }

    #[doc="Modify the INTEN register."]
    #[inline] pub fn with_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(), f(self.inten()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTSTAT register."]
    #[inline] pub fn intstat_mut(&self) -> *mut Intstat { 
        (self.0 + 0x204) as *mut Intstat
    }

    #[doc="Get the *const pointer for the INTSTAT register."]
    #[inline] pub fn intstat_ptr(&self) -> *const Intstat { 
           self.intstat_mut()
    }

    #[doc="Read the INTSTAT register."]
    #[inline] pub fn intstat(&self) -> Intstat { 
        unsafe {
            read_volatile(self.intstat_ptr())
        }
    }

    #[doc="Write the INTSTAT register."]
    #[inline] pub fn set_intstat<F: FnOnce(Intstat) -> Intstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(), f(Intstat(0)));
        }
        self
    }

    #[doc="Modify the INTSTAT register."]
    #[inline] pub fn with_intstat<F: FnOnce(Intstat) -> Intstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(), f(self.intstat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTCLR register."]
    #[inline] pub fn intclr_mut(&self) -> *mut Intclr { 
        (self.0 + 0x208) as *mut Intclr
    }

    #[doc="Get the *const pointer for the INTCLR register."]
    #[inline] pub fn intclr_ptr(&self) -> *const Intclr { 
           self.intclr_mut()
    }

    #[doc="Read the INTCLR register."]
    #[inline] pub fn intclr(&self) -> Intclr { 
        unsafe {
            read_volatile(self.intclr_ptr())
        }
    }

    #[doc="Write the INTCLR register."]
    #[inline] pub fn set_intclr<F: FnOnce(Intclr) -> Intclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(), f(Intclr(0)));
        }
        self
    }

    #[doc="Modify the INTCLR register."]
    #[inline] pub fn with_intclr<F: FnOnce(Intclr) -> Intclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(), f(self.intclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTSET register."]
    #[inline] pub fn intset_mut(&self) -> *mut Intset { 
        (self.0 + 0x20c) as *mut Intset
    }

    #[doc="Get the *const pointer for the INTSET register."]
    #[inline] pub fn intset_ptr(&self) -> *const Intset { 
           self.intset_mut()
    }

    #[doc="Read the INTSET register."]
    #[inline] pub fn intset(&self) -> Intset { 
        unsafe {
            read_volatile(self.intset_ptr())
        }
    }

    #[doc="Write the INTSET register."]
    #[inline] pub fn set_intset<F: FnOnce(Intset) -> Intset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intset_mut(), f(Intset(0)));
        }
        self
    }

    #[doc="Modify the INTSET register."]
    #[inline] pub fn with_intset<F: FnOnce(Intset) -> Intset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intset_mut(), f(self.intset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the REGACCINTEN register."]
    #[inline] pub fn regaccinten_mut(&self) -> *mut Regaccinten { 
        (self.0 + 0x210) as *mut Regaccinten
    }

    #[doc="Get the *const pointer for the REGACCINTEN register."]
    #[inline] pub fn regaccinten_ptr(&self) -> *const Regaccinten { 
           self.regaccinten_mut()
    }

    #[doc="Read the REGACCINTEN register."]
    #[inline] pub fn regaccinten(&self) -> Regaccinten { 
        unsafe {
            read_volatile(self.regaccinten_ptr())
        }
    }

    #[doc="Write the REGACCINTEN register."]
    #[inline] pub fn set_regaccinten<F: FnOnce(Regaccinten) -> Regaccinten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.regaccinten_mut(), f(Regaccinten(0)));
        }
        self
    }

    #[doc="Modify the REGACCINTEN register."]
    #[inline] pub fn with_regaccinten<F: FnOnce(Regaccinten) -> Regaccinten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.regaccinten_mut(), f(self.regaccinten()));
        }
        self
    }

    #[doc="Get the *mut pointer for the REGACCINTSTAT register."]
    #[inline] pub fn regaccintstat_mut(&self) -> *mut Regaccintstat { 
        (self.0 + 0x214) as *mut Regaccintstat
    }

    #[doc="Get the *const pointer for the REGACCINTSTAT register."]
    #[inline] pub fn regaccintstat_ptr(&self) -> *const Regaccintstat { 
           self.regaccintstat_mut()
    }

    #[doc="Read the REGACCINTSTAT register."]
    #[inline] pub fn regaccintstat(&self) -> Regaccintstat { 
        unsafe {
            read_volatile(self.regaccintstat_ptr())
        }
    }

    #[doc="Write the REGACCINTSTAT register."]
    #[inline] pub fn set_regaccintstat<F: FnOnce(Regaccintstat) -> Regaccintstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.regaccintstat_mut(), f(Regaccintstat(0)));
        }
        self
    }

    #[doc="Modify the REGACCINTSTAT register."]
    #[inline] pub fn with_regaccintstat<F: FnOnce(Regaccintstat) -> Regaccintstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.regaccintstat_mut(), f(self.regaccintstat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the REGACCINTCLR register."]
    #[inline] pub fn regaccintclr_mut(&self) -> *mut Regaccintclr { 
        (self.0 + 0x218) as *mut Regaccintclr
    }

    #[doc="Get the *const pointer for the REGACCINTCLR register."]
    #[inline] pub fn regaccintclr_ptr(&self) -> *const Regaccintclr { 
           self.regaccintclr_mut()
    }

    #[doc="Read the REGACCINTCLR register."]
    #[inline] pub fn regaccintclr(&self) -> Regaccintclr { 
        unsafe {
            read_volatile(self.regaccintclr_ptr())
        }
    }

    #[doc="Write the REGACCINTCLR register."]
    #[inline] pub fn set_regaccintclr<F: FnOnce(Regaccintclr) -> Regaccintclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.regaccintclr_mut(), f(Regaccintclr(0)));
        }
        self
    }

    #[doc="Modify the REGACCINTCLR register."]
    #[inline] pub fn with_regaccintclr<F: FnOnce(Regaccintclr) -> Regaccintclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.regaccintclr_mut(), f(self.regaccintclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the REGACCINTSET register."]
    #[inline] pub fn regaccintset_mut(&self) -> *mut Regaccintset { 
        (self.0 + 0x21c) as *mut Regaccintset
    }

    #[doc="Get the *const pointer for the REGACCINTSET register."]
    #[inline] pub fn regaccintset_ptr(&self) -> *const Regaccintset { 
           self.regaccintset_mut()
    }

    #[doc="Read the REGACCINTSET register."]
    #[inline] pub fn regaccintset(&self) -> Regaccintset { 
        unsafe {
            read_volatile(self.regaccintset_ptr())
        }
    }

    #[doc="Write the REGACCINTSET register."]
    #[inline] pub fn set_regaccintset<F: FnOnce(Regaccintset) -> Regaccintset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.regaccintset_mut(), f(Regaccintset(0)));
        }
        self
    }

    #[doc="Modify the REGACCINTSET register."]
    #[inline] pub fn with_regaccintset<F: FnOnce(Regaccintset) -> Regaccintset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.regaccintset_mut(), f(self.regaccintset()));
        }
        self
    }

}

#[doc="Current FIFO Pointer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifoptr(pub u32);
impl Fifoptr {
    #[doc="The number of bytes currently in the hardware FIFO."]
    #[inline] pub fn fifosiz(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if FIFOSIZ != 0"]
    #[inline] pub fn test_fifosiz(&self) -> bool {
        self.fifosiz() != 0
    }

    #[doc="Sets the FIFOSIZ field."]
    #[inline] pub fn set_fifosiz<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Current FIFO pointer."]
    #[inline] pub fn fifoptr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FIFOPTR != 0"]
    #[inline] pub fn test_fifoptr(&self) -> bool {
        self.fifoptr() != 0
    }

    #[doc="Sets the FIFOPTR field."]
    #[inline] pub fn set_fifoptr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifoptr {
    #[inline]
    fn from(other: u32) -> Self {
         Fifoptr(other)
    }
}

impl ::core::fmt::Display for Fifoptr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifoptr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifosiz() != 0 { try!(write!(f, " fifosiz=0x{:x}", self.fifosiz()))}
        if self.fifoptr() != 0 { try!(write!(f, " fifoptr=0x{:x}", self.fifoptr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifocfg(pub u32);
impl Fifocfg {
    #[doc="Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOOBASE*8-1)"]
    #[inline] pub fn robase(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if ROBASE != 0"]
    #[inline] pub fn test_robase(&self) -> bool {
        self.robase() != 0
    }

    #[doc="Sets the ROBASE field."]
    #[inline] pub fn set_robase<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
    #[inline] pub fn fifomax(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if FIFOMAX != 0"]
    #[inline] pub fn test_fifomax(&self) -> bool {
        self.fifomax() != 0
    }

    #[doc="Sets the FIFOMAX field."]
    #[inline] pub fn set_fifomax<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
    #[inline] pub fn fifobase(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if FIFOBASE != 0"]
    #[inline] pub fn test_fifobase(&self) -> bool {
        self.fifobase() != 0
    }

    #[doc="Sets the FIFOBASE field."]
    #[inline] pub fn set_fifobase<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifocfg {
    #[inline]
    fn from(other: u32) -> Self {
         Fifocfg(other)
    }
}

impl ::core::fmt::Display for Fifocfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifocfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.robase() != 0 { try!(write!(f, " robase=0x{:x}", self.robase()))}
        if self.fifomax() != 0 { try!(write!(f, " fifomax=0x{:x}", self.fifomax()))}
        if self.fifobase() != 0 { try!(write!(f, " fifobase=0x{:x}", self.fifobase()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO Threshold Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifothr(pub u32);
impl Fifothr {
    #[doc="FIFO size interrupt threshold."]
    #[inline] pub fn fifothr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FIFOTHR != 0"]
    #[inline] pub fn test_fifothr(&self) -> bool {
        self.fifothr() != 0
    }

    #[doc="Sets the FIFOTHR field."]
    #[inline] pub fn set_fifothr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifothr {
    #[inline]
    fn from(other: u32) -> Self {
         Fifothr(other)
    }
}

impl ::core::fmt::Display for Fifothr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifothr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifothr() != 0 { try!(write!(f, " fifothr=0x{:x}", self.fifothr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO Update Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fupd(pub u32);
impl Fupd {
    #[doc="This bitfield indicates an IO read is active."]
    #[inline] pub fn ioread(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IOREAD != 0"]
    #[inline] pub fn test_ioread(&self) -> bool {
        self.ioread() != 0
    }

    #[doc="Sets the IOREAD field."]
    #[inline] pub fn set_ioread<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit indicates that a FIFO update is underway."]
    #[inline] pub fn fifoupd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FIFOUPD != 0"]
    #[inline] pub fn test_fifoupd(&self) -> bool {
        self.fifoupd() != 0
    }

    #[doc="Sets the FIFOUPD field."]
    #[inline] pub fn set_fifoupd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fupd {
    #[inline]
    fn from(other: u32) -> Self {
         Fupd(other)
    }
}

impl ::core::fmt::Display for Fupd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fupd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ioread() != 0 { try!(write!(f, " ioread"))}
        if self.fifoupd() != 0 { try!(write!(f, " fifoupd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Overall FIFO Counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifoctr(pub u32);
impl Fifoctr {
    #[doc="Virtual FIFO byte count"]
    #[inline] pub fn fifoctr(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if FIFOCTR != 0"]
    #[inline] pub fn test_fifoctr(&self) -> bool {
        self.fifoctr() != 0
    }

    #[doc="Sets the FIFOCTR field."]
    #[inline] pub fn set_fifoctr<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifoctr {
    #[inline]
    fn from(other: u32) -> Self {
         Fifoctr(other)
    }
}

impl ::core::fmt::Display for Fifoctr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifoctr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifoctr() != 0 { try!(write!(f, " fifoctr=0x{:x}", self.fifoctr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Overall FIFO Counter Increment"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifoinc(pub u32);
impl Fifoinc {
    #[doc="Increment the Overall FIFO Counter by this value on a write"]
    #[inline] pub fn fifoinc(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if FIFOINC != 0"]
    #[inline] pub fn test_fifoinc(&self) -> bool {
        self.fifoinc() != 0
    }

    #[doc="Sets the FIFOINC field."]
    #[inline] pub fn set_fifoinc<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifoinc {
    #[inline]
    fn from(other: u32) -> Self {
         Fifoinc(other)
    }
}

impl ::core::fmt::Display for Fifoinc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifoinc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifoinc() != 0 { try!(write!(f, " fifoinc=0x{:x}", self.fifoinc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Slave Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc="IOSLAVE interface enable."]
    #[inline] pub fn ifcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if IFCEN != 0"]
    #[inline] pub fn test_ifcen(&self) -> bool {
        self.ifcen() != 0
    }

    #[doc="Sets the IFCEN field."]
    #[inline] pub fn set_ifcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="7-bit or 10-bit I2C device address."]
    #[inline] pub fn i2caddr(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xfff) as u16) } // [19:8]
    }

    #[doc="Returns true if I2CADDR != 0"]
    #[inline] pub fn test_i2caddr(&self) -> bool {
        self.i2caddr() != 0
    }

    #[doc="Sets the I2CADDR field."]
    #[inline] pub fn set_i2caddr<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit holds the cycle to initiate an I/O RAM read."]
    #[inline] pub fn startrd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if STARTRD != 0"]
    #[inline] pub fn test_startrd(&self) -> bool {
        self.startrd() != 0
    }

    #[doc="Sets the STARTRD field."]
    #[inline] pub fn set_startrd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit selects the transfer bit ordering."]
    #[inline] pub fn lsb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LSB != 0"]
    #[inline] pub fn test_lsb(&self) -> bool {
        self.lsb() != 0
    }

    #[doc="Sets the LSB field."]
    #[inline] pub fn set_lsb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit selects SPI polarity."]
    #[inline] pub fn spol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit selects the I/O interface."]
    #[inline] pub fn ifcsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IFCSEL != 0"]
    #[inline] pub fn test_ifcsel(&self) -> bool {
        self.ifcsel() != 0
    }

    #[doc="Sets the IFCSEL field."]
    #[inline] pub fn set_ifcsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Cfg(other)
    }
}

impl ::core::fmt::Display for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ifcen() != 0 { try!(write!(f, " ifcen"))}
        if self.i2caddr() != 0 { try!(write!(f, " i2caddr=0x{:x}", self.i2caddr()))}
        if self.startrd() != 0 { try!(write!(f, " startrd"))}
        if self.lsb() != 0 { try!(write!(f, " lsb"))}
        if self.spol() != 0 { try!(write!(f, " spol"))}
        if self.ifcsel() != 0 { try!(write!(f, " ifcsel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Slave Interrupt Priority Encode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prenc(pub u32);
impl Prenc {
    #[doc="These bits hold the priority encode of the REGACC interrupts."]
    #[inline] pub fn prenc(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if PRENC != 0"]
    #[inline] pub fn test_prenc(&self) -> bool {
        self.prenc() != 0
    }

    #[doc="Sets the PRENC field."]
    #[inline] pub fn set_prenc<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Prenc {
    #[inline]
    fn from(other: u32) -> Self {
         Prenc(other)
    }
}

impl ::core::fmt::Display for Prenc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prenc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prenc() != 0 { try!(write!(f, " prenc=0x{:x}", self.prenc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Interrupt Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iointctl(pub u32);
impl Iointctl {
    #[doc="These bits set the IOINT interrupts when written with a 1."]
    #[inline] pub fn iointset(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IOINTSET != 0"]
    #[inline] pub fn test_iointset(&self) -> bool {
        self.iointset() != 0
    }

    #[doc="Sets the IOINTSET field."]
    #[inline] pub fn set_iointset<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="This bit clears all of the IOINT interrupts when written with a 1."]
    #[inline] pub fn iointclr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if IOINTCLR != 0"]
    #[inline] pub fn test_iointclr(&self) -> bool {
        self.iointclr() != 0
    }

    #[doc="Sets the IOINTCLR field."]
    #[inline] pub fn set_iointclr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="These bits read the IOINT interrupts."]
    #[inline] pub fn ioint(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IOINT != 0"]
    #[inline] pub fn test_ioint(&self) -> bool {
        self.ioint() != 0
    }

    #[doc="Sets the IOINT field."]
    #[inline] pub fn set_ioint<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="These read-only bits indicate whether the IOINT interrupts are enabled."]
    #[inline] pub fn iointen(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IOINTEN != 0"]
    #[inline] pub fn test_iointen(&self) -> bool {
        self.iointen() != 0
    }

    #[doc="Sets the IOINTEN field."]
    #[inline] pub fn set_iointen<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iointctl {
    #[inline]
    fn from(other: u32) -> Self {
         Iointctl(other)
    }
}

impl ::core::fmt::Display for Iointctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iointctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iointset() != 0 { try!(write!(f, " iointset=0x{:x}", self.iointset()))}
        if self.iointclr() != 0 { try!(write!(f, " iointclr"))}
        if self.ioint() != 0 { try!(write!(f, " ioint=0x{:x}", self.ioint()))}
        if self.iointen() != 0 { try!(write!(f, " iointen=0x{:x}", self.iointen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="General Address Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Genadd(pub u32);
impl Genadd {
    #[doc="The data supplied on the last General Address reference."]
    #[inline] pub fn gadata(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if GADATA != 0"]
    #[inline] pub fn test_gadata(&self) -> bool {
        self.gadata() != 0
    }

    #[doc="Sets the GADATA field."]
    #[inline] pub fn set_gadata<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Genadd {
    #[inline]
    fn from(other: u32) -> Self {
         Genadd(other)
    }
}

impl ::core::fmt::Display for Genadd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Genadd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gadata() != 0 { try!(write!(f, " gadata=0x{:x}", self.gadata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Slave Interrupts: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc="Transfer complete interrupt, write to register space."]
    #[inline] pub fn xcmpwr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if XCMPWR != 0"]
    #[inline] pub fn test_xcmpwr(&self) -> bool {
        self.xcmpwr() != 0
    }

    #[doc="Sets the XCMPWR field."]
    #[inline] pub fn set_xcmpwr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transfer complete interrupt, write to FIFO space."]
    #[inline] pub fn xcmpwf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if XCMPWF != 0"]
    #[inline] pub fn test_xcmpwf(&self) -> bool {
        self.xcmpwf() != 0
    }

    #[doc="Sets the XCMPWF field."]
    #[inline] pub fn set_xcmpwf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transfer complete interrupt, read from register space."]
    #[inline] pub fn xcmprr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if XCMPRR != 0"]
    #[inline] pub fn test_xcmprr(&self) -> bool {
        self.xcmprr() != 0
    }

    #[doc="Sets the XCMPRR field."]
    #[inline] pub fn set_xcmprr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transfer complete interrupt, read from FIFO space."]
    #[inline] pub fn xcmprf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if XCMPRF != 0"]
    #[inline] pub fn test_xcmprf(&self) -> bool {
        self.xcmprf() != 0
    }

    #[doc="Sets the XCMPRF field."]
    #[inline] pub fn set_xcmprf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C Interrupt Write interrupt."]
    #[inline] pub fn iointw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IOINTW != 0"]
    #[inline] pub fn test_iointw(&self) -> bool {
        self.iointw() != 0
    }

    #[doc="Sets the IOINTW field."]
    #[inline] pub fn set_iointw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C General Address interrupt."]
    #[inline] pub fn genad(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GENAD != 0"]
    #[inline] pub fn test_genad(&self) -> bool {
        self.genad() != 0
    }

    #[doc="Sets the GENAD field."]
    #[inline] pub fn set_genad<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO Read Error interrupt."]
    #[inline] pub fn frderr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FRDERR != 0"]
    #[inline] pub fn test_frderr(&self) -> bool {
        self.frderr() != 0
    }

    #[doc="Sets the FRDERR field."]
    #[inline] pub fn set_frderr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO Underflow interrupt."]
    #[inline] pub fn fundfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FUNDFL != 0"]
    #[inline] pub fn test_fundfl(&self) -> bool {
        self.fundfl() != 0
    }

    #[doc="Sets the FUNDFL field."]
    #[inline] pub fn set_fundfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FIFO Overflow interrupt."]
    #[inline] pub fn fovfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FOVFL != 0"]
    #[inline] pub fn test_fovfl(&self) -> bool {
        self.fovfl() != 0
    }

    #[doc="Sets the FOVFL field."]
    #[inline] pub fn set_fovfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FIFO Size interrupt."]
    #[inline] pub fn fsize(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FSIZE != 0"]
    #[inline] pub fn test_fsize(&self) -> bool {
        self.fsize() != 0
    }

    #[doc="Sets the FSIZE field."]
    #[inline] pub fn set_fsize<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Inten {
    #[inline]
    fn from(other: u32) -> Self {
         Inten(other)
    }
}

impl ::core::fmt::Display for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xcmpwr() != 0 { try!(write!(f, " xcmpwr"))}
        if self.xcmpwf() != 0 { try!(write!(f, " xcmpwf"))}
        if self.xcmprr() != 0 { try!(write!(f, " xcmprr"))}
        if self.xcmprf() != 0 { try!(write!(f, " xcmprf"))}
        if self.iointw() != 0 { try!(write!(f, " iointw"))}
        if self.genad() != 0 { try!(write!(f, " genad"))}
        if self.frderr() != 0 { try!(write!(f, " frderr"))}
        if self.fundfl() != 0 { try!(write!(f, " fundfl"))}
        if self.fovfl() != 0 { try!(write!(f, " fovfl"))}
        if self.fsize() != 0 { try!(write!(f, " fsize"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Slave Interrupts: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc="Transfer complete interrupt, write to register space."]
    #[inline] pub fn xcmpwr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if XCMPWR != 0"]
    #[inline] pub fn test_xcmpwr(&self) -> bool {
        self.xcmpwr() != 0
    }

    #[doc="Sets the XCMPWR field."]
    #[inline] pub fn set_xcmpwr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transfer complete interrupt, write to FIFO space."]
    #[inline] pub fn xcmpwf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if XCMPWF != 0"]
    #[inline] pub fn test_xcmpwf(&self) -> bool {
        self.xcmpwf() != 0
    }

    #[doc="Sets the XCMPWF field."]
    #[inline] pub fn set_xcmpwf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transfer complete interrupt, read from register space."]
    #[inline] pub fn xcmprr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if XCMPRR != 0"]
    #[inline] pub fn test_xcmprr(&self) -> bool {
        self.xcmprr() != 0
    }

    #[doc="Sets the XCMPRR field."]
    #[inline] pub fn set_xcmprr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transfer complete interrupt, read from FIFO space."]
    #[inline] pub fn xcmprf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if XCMPRF != 0"]
    #[inline] pub fn test_xcmprf(&self) -> bool {
        self.xcmprf() != 0
    }

    #[doc="Sets the XCMPRF field."]
    #[inline] pub fn set_xcmprf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C Interrupt Write interrupt."]
    #[inline] pub fn iointw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IOINTW != 0"]
    #[inline] pub fn test_iointw(&self) -> bool {
        self.iointw() != 0
    }

    #[doc="Sets the IOINTW field."]
    #[inline] pub fn set_iointw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C General Address interrupt."]
    #[inline] pub fn genad(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GENAD != 0"]
    #[inline] pub fn test_genad(&self) -> bool {
        self.genad() != 0
    }

    #[doc="Sets the GENAD field."]
    #[inline] pub fn set_genad<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO Read Error interrupt."]
    #[inline] pub fn frderr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FRDERR != 0"]
    #[inline] pub fn test_frderr(&self) -> bool {
        self.frderr() != 0
    }

    #[doc="Sets the FRDERR field."]
    #[inline] pub fn set_frderr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO Underflow interrupt."]
    #[inline] pub fn fundfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FUNDFL != 0"]
    #[inline] pub fn test_fundfl(&self) -> bool {
        self.fundfl() != 0
    }

    #[doc="Sets the FUNDFL field."]
    #[inline] pub fn set_fundfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FIFO Overflow interrupt."]
    #[inline] pub fn fovfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FOVFL != 0"]
    #[inline] pub fn test_fovfl(&self) -> bool {
        self.fovfl() != 0
    }

    #[doc="Sets the FOVFL field."]
    #[inline] pub fn set_fovfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FIFO Size interrupt."]
    #[inline] pub fn fsize(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FSIZE != 0"]
    #[inline] pub fn test_fsize(&self) -> bool {
        self.fsize() != 0
    }

    #[doc="Sets the FSIZE field."]
    #[inline] pub fn set_fsize<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intstat {
    #[inline]
    fn from(other: u32) -> Self {
         Intstat(other)
    }
}

impl ::core::fmt::Display for Intstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xcmpwr() != 0 { try!(write!(f, " xcmpwr"))}
        if self.xcmpwf() != 0 { try!(write!(f, " xcmpwf"))}
        if self.xcmprr() != 0 { try!(write!(f, " xcmprr"))}
        if self.xcmprf() != 0 { try!(write!(f, " xcmprf"))}
        if self.iointw() != 0 { try!(write!(f, " iointw"))}
        if self.genad() != 0 { try!(write!(f, " genad"))}
        if self.frderr() != 0 { try!(write!(f, " frderr"))}
        if self.fundfl() != 0 { try!(write!(f, " fundfl"))}
        if self.fovfl() != 0 { try!(write!(f, " fovfl"))}
        if self.fsize() != 0 { try!(write!(f, " fsize"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Slave Interrupts: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc="Transfer complete interrupt, write to register space."]
    #[inline] pub fn xcmpwr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if XCMPWR != 0"]
    #[inline] pub fn test_xcmpwr(&self) -> bool {
        self.xcmpwr() != 0
    }

    #[doc="Sets the XCMPWR field."]
    #[inline] pub fn set_xcmpwr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transfer complete interrupt, write to FIFO space."]
    #[inline] pub fn xcmpwf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if XCMPWF != 0"]
    #[inline] pub fn test_xcmpwf(&self) -> bool {
        self.xcmpwf() != 0
    }

    #[doc="Sets the XCMPWF field."]
    #[inline] pub fn set_xcmpwf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transfer complete interrupt, read from register space."]
    #[inline] pub fn xcmprr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if XCMPRR != 0"]
    #[inline] pub fn test_xcmprr(&self) -> bool {
        self.xcmprr() != 0
    }

    #[doc="Sets the XCMPRR field."]
    #[inline] pub fn set_xcmprr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transfer complete interrupt, read from FIFO space."]
    #[inline] pub fn xcmprf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if XCMPRF != 0"]
    #[inline] pub fn test_xcmprf(&self) -> bool {
        self.xcmprf() != 0
    }

    #[doc="Sets the XCMPRF field."]
    #[inline] pub fn set_xcmprf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C Interrupt Write interrupt."]
    #[inline] pub fn iointw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IOINTW != 0"]
    #[inline] pub fn test_iointw(&self) -> bool {
        self.iointw() != 0
    }

    #[doc="Sets the IOINTW field."]
    #[inline] pub fn set_iointw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C General Address interrupt."]
    #[inline] pub fn genad(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GENAD != 0"]
    #[inline] pub fn test_genad(&self) -> bool {
        self.genad() != 0
    }

    #[doc="Sets the GENAD field."]
    #[inline] pub fn set_genad<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO Read Error interrupt."]
    #[inline] pub fn frderr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FRDERR != 0"]
    #[inline] pub fn test_frderr(&self) -> bool {
        self.frderr() != 0
    }

    #[doc="Sets the FRDERR field."]
    #[inline] pub fn set_frderr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO Underflow interrupt."]
    #[inline] pub fn fundfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FUNDFL != 0"]
    #[inline] pub fn test_fundfl(&self) -> bool {
        self.fundfl() != 0
    }

    #[doc="Sets the FUNDFL field."]
    #[inline] pub fn set_fundfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FIFO Overflow interrupt."]
    #[inline] pub fn fovfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FOVFL != 0"]
    #[inline] pub fn test_fovfl(&self) -> bool {
        self.fovfl() != 0
    }

    #[doc="Sets the FOVFL field."]
    #[inline] pub fn set_fovfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FIFO Size interrupt."]
    #[inline] pub fn fsize(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FSIZE != 0"]
    #[inline] pub fn test_fsize(&self) -> bool {
        self.fsize() != 0
    }

    #[doc="Sets the FSIZE field."]
    #[inline] pub fn set_fsize<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intclr {
    #[inline]
    fn from(other: u32) -> Self {
         Intclr(other)
    }
}

impl ::core::fmt::Display for Intclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xcmpwr() != 0 { try!(write!(f, " xcmpwr"))}
        if self.xcmpwf() != 0 { try!(write!(f, " xcmpwf"))}
        if self.xcmprr() != 0 { try!(write!(f, " xcmprr"))}
        if self.xcmprf() != 0 { try!(write!(f, " xcmprf"))}
        if self.iointw() != 0 { try!(write!(f, " iointw"))}
        if self.genad() != 0 { try!(write!(f, " genad"))}
        if self.frderr() != 0 { try!(write!(f, " frderr"))}
        if self.fundfl() != 0 { try!(write!(f, " fundfl"))}
        if self.fovfl() != 0 { try!(write!(f, " fovfl"))}
        if self.fsize() != 0 { try!(write!(f, " fsize"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Slave Interrupts: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc="Transfer complete interrupt, write to register space."]
    #[inline] pub fn xcmpwr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if XCMPWR != 0"]
    #[inline] pub fn test_xcmpwr(&self) -> bool {
        self.xcmpwr() != 0
    }

    #[doc="Sets the XCMPWR field."]
    #[inline] pub fn set_xcmpwr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transfer complete interrupt, write to FIFO space."]
    #[inline] pub fn xcmpwf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if XCMPWF != 0"]
    #[inline] pub fn test_xcmpwf(&self) -> bool {
        self.xcmpwf() != 0
    }

    #[doc="Sets the XCMPWF field."]
    #[inline] pub fn set_xcmpwf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transfer complete interrupt, read from register space."]
    #[inline] pub fn xcmprr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if XCMPRR != 0"]
    #[inline] pub fn test_xcmprr(&self) -> bool {
        self.xcmprr() != 0
    }

    #[doc="Sets the XCMPRR field."]
    #[inline] pub fn set_xcmprr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transfer complete interrupt, read from FIFO space."]
    #[inline] pub fn xcmprf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if XCMPRF != 0"]
    #[inline] pub fn test_xcmprf(&self) -> bool {
        self.xcmprf() != 0
    }

    #[doc="Sets the XCMPRF field."]
    #[inline] pub fn set_xcmprf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C Interrupt Write interrupt."]
    #[inline] pub fn iointw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IOINTW != 0"]
    #[inline] pub fn test_iointw(&self) -> bool {
        self.iointw() != 0
    }

    #[doc="Sets the IOINTW field."]
    #[inline] pub fn set_iointw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C General Address interrupt."]
    #[inline] pub fn genad(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if GENAD != 0"]
    #[inline] pub fn test_genad(&self) -> bool {
        self.genad() != 0
    }

    #[doc="Sets the GENAD field."]
    #[inline] pub fn set_genad<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO Read Error interrupt."]
    #[inline] pub fn frderr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FRDERR != 0"]
    #[inline] pub fn test_frderr(&self) -> bool {
        self.frderr() != 0
    }

    #[doc="Sets the FRDERR field."]
    #[inline] pub fn set_frderr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO Underflow interrupt."]
    #[inline] pub fn fundfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FUNDFL != 0"]
    #[inline] pub fn test_fundfl(&self) -> bool {
        self.fundfl() != 0
    }

    #[doc="Sets the FUNDFL field."]
    #[inline] pub fn set_fundfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FIFO Overflow interrupt."]
    #[inline] pub fn fovfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FOVFL != 0"]
    #[inline] pub fn test_fovfl(&self) -> bool {
        self.fovfl() != 0
    }

    #[doc="Sets the FOVFL field."]
    #[inline] pub fn set_fovfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FIFO Size interrupt."]
    #[inline] pub fn fsize(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FSIZE != 0"]
    #[inline] pub fn test_fsize(&self) -> bool {
        self.fsize() != 0
    }

    #[doc="Sets the FSIZE field."]
    #[inline] pub fn set_fsize<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intset {
    #[inline]
    fn from(other: u32) -> Self {
         Intset(other)
    }
}

impl ::core::fmt::Display for Intset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xcmpwr() != 0 { try!(write!(f, " xcmpwr"))}
        if self.xcmpwf() != 0 { try!(write!(f, " xcmpwf"))}
        if self.xcmprr() != 0 { try!(write!(f, " xcmprr"))}
        if self.xcmprf() != 0 { try!(write!(f, " xcmprf"))}
        if self.iointw() != 0 { try!(write!(f, " iointw"))}
        if self.genad() != 0 { try!(write!(f, " genad"))}
        if self.frderr() != 0 { try!(write!(f, " frderr"))}
        if self.fundfl() != 0 { try!(write!(f, " fundfl"))}
        if self.fovfl() != 0 { try!(write!(f, " fovfl"))}
        if self.fsize() != 0 { try!(write!(f, " fsize"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Register Access Interrupts: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Regaccinten(pub u32);
impl Regaccinten {
    #[doc="Register access interrupts."]
    #[inline] pub fn regacc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if REGACC != 0"]
    #[inline] pub fn test_regacc(&self) -> bool {
        self.regacc() != 0
    }

    #[doc="Sets the REGACC field."]
    #[inline] pub fn set_regacc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Regaccinten {
    #[inline]
    fn from(other: u32) -> Self {
         Regaccinten(other)
    }
}

impl ::core::fmt::Display for Regaccinten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Regaccinten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Register Access Interrupts: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Regaccintstat(pub u32);
impl Regaccintstat {
    #[doc="Register access interrupts."]
    #[inline] pub fn regacc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if REGACC != 0"]
    #[inline] pub fn test_regacc(&self) -> bool {
        self.regacc() != 0
    }

    #[doc="Sets the REGACC field."]
    #[inline] pub fn set_regacc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Regaccintstat {
    #[inline]
    fn from(other: u32) -> Self {
         Regaccintstat(other)
    }
}

impl ::core::fmt::Display for Regaccintstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Regaccintstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Register Access Interrupts: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Regaccintclr(pub u32);
impl Regaccintclr {
    #[doc="Register access interrupts."]
    #[inline] pub fn regacc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if REGACC != 0"]
    #[inline] pub fn test_regacc(&self) -> bool {
        self.regacc() != 0
    }

    #[doc="Sets the REGACC field."]
    #[inline] pub fn set_regacc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Regaccintclr {
    #[inline]
    fn from(other: u32) -> Self {
         Regaccintclr(other)
    }
}

impl ::core::fmt::Display for Regaccintclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Regaccintclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Register Access Interrupts: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Regaccintset(pub u32);
impl Regaccintset {
    #[doc="Register access interrupts."]
    #[inline] pub fn regacc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if REGACC != 0"]
    #[inline] pub fn test_regacc(&self) -> bool {
        self.regacc() != 0
    }

    #[doc="Sets the REGACC field."]
    #[inline] pub fn set_regacc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Regaccintset {
    #[inline]
    fn from(other: u32) -> Self {
         Regaccintset(other)
    }
}

impl ::core::fmt::Display for Regaccintset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Regaccintset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}


