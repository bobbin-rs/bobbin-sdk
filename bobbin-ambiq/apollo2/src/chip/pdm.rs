#[allow(unused_imports)] use bobbin_common::*;

periph!( PDM, Pdm, _PDM, PdmPeriph, 0x50011000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PDM Peripheral"]
pub struct PdmPeriph(pub usize); 



impl PdmPeriph {
    #[doc="Get the *mut pointer for the PCFG register."]
    #[inline] pub fn pcfg_mut(&self) -> *mut Pcfg { 
        (self.0 + 0x0) as *mut Pcfg
    }

    #[doc="Get the *const pointer for the PCFG register."]
    #[inline] pub fn pcfg_ptr(&self) -> *const Pcfg { 
           self.pcfg_mut()
    }

    #[doc="Read the PCFG register."]
    #[inline] pub fn pcfg(&self) -> Pcfg { 
        unsafe {
            read_volatile(self.pcfg_ptr())
        }
    }

    #[doc="Write the PCFG register."]
    #[inline] pub fn set_pcfg<F: FnOnce(Pcfg) -> Pcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pcfg_mut(), f(Pcfg(0)));
        }
        self
    }

    #[doc="Modify the PCFG register."]
    #[inline] pub fn with_pcfg<F: FnOnce(Pcfg) -> Pcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pcfg_mut(), f(self.pcfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the VCFG register."]
    #[inline] pub fn vcfg_mut(&self) -> *mut Vcfg { 
        (self.0 + 0x4) as *mut Vcfg
    }

    #[doc="Get the *const pointer for the VCFG register."]
    #[inline] pub fn vcfg_ptr(&self) -> *const Vcfg { 
           self.vcfg_mut()
    }

    #[doc="Read the VCFG register."]
    #[inline] pub fn vcfg(&self) -> Vcfg { 
        unsafe {
            read_volatile(self.vcfg_ptr())
        }
    }

    #[doc="Write the VCFG register."]
    #[inline] pub fn set_vcfg<F: FnOnce(Vcfg) -> Vcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vcfg_mut(), f(Vcfg(0)));
        }
        self
    }

    #[doc="Modify the VCFG register."]
    #[inline] pub fn with_vcfg<F: FnOnce(Vcfg) -> Vcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vcfg_mut(), f(self.vcfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FR register."]
    #[inline] pub fn fr_mut(&self) -> *mut Fr { 
        (self.0 + 0x8) as *mut Fr
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

    #[doc="Get the *mut pointer for the FRD register."]
    #[inline] pub fn frd_mut(&self) -> *mut Frd { 
        (self.0 + 0xc) as *mut Frd
    }

    #[doc="Get the *const pointer for the FRD register."]
    #[inline] pub fn frd_ptr(&self) -> *const Frd { 
           self.frd_mut()
    }

    #[doc="Read the FRD register."]
    #[inline] pub fn frd(&self) -> Frd { 
        unsafe {
            read_volatile(self.frd_ptr())
        }
    }

    #[doc="Write the FRD register."]
    #[inline] pub fn set_frd<F: FnOnce(Frd) -> Frd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.frd_mut(), f(Frd(0)));
        }
        self
    }

    #[doc="Modify the FRD register."]
    #[inline] pub fn with_frd<F: FnOnce(Frd) -> Frd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.frd_mut(), f(self.frd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLUSH register."]
    #[inline] pub fn flush_mut(&self) -> *mut Flush { 
        (self.0 + 0x10) as *mut Flush
    }

    #[doc="Get the *const pointer for the FLUSH register."]
    #[inline] pub fn flush_ptr(&self) -> *const Flush { 
           self.flush_mut()
    }

    #[doc="Read the FLUSH register."]
    #[inline] pub fn flush(&self) -> Flush { 
        unsafe {
            read_volatile(self.flush_ptr())
        }
    }

    #[doc="Write the FLUSH register."]
    #[inline] pub fn set_flush<F: FnOnce(Flush) -> Flush>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flush_mut(), f(Flush(0)));
        }
        self
    }

    #[doc="Modify the FLUSH register."]
    #[inline] pub fn with_flush<F: FnOnce(Flush) -> Flush>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flush_mut(), f(self.flush()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FTHR register."]
    #[inline] pub fn fthr_mut(&self) -> *mut Fthr { 
        (self.0 + 0x14) as *mut Fthr
    }

    #[doc="Get the *const pointer for the FTHR register."]
    #[inline] pub fn fthr_ptr(&self) -> *const Fthr { 
           self.fthr_mut()
    }

    #[doc="Read the FTHR register."]
    #[inline] pub fn fthr(&self) -> Fthr { 
        unsafe {
            read_volatile(self.fthr_ptr())
        }
    }

    #[doc="Write the FTHR register."]
    #[inline] pub fn set_fthr<F: FnOnce(Fthr) -> Fthr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fthr_mut(), f(Fthr(0)));
        }
        self
    }

    #[doc="Modify the FTHR register."]
    #[inline] pub fn with_fthr<F: FnOnce(Fthr) -> Fthr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fthr_mut(), f(self.fthr()));
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

}

#[doc="PDM Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcfg(pub u32);
impl Pcfg {
    #[doc="Left/right channel swap."]
    #[inline] pub fn lrswap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LRSWAP != 0"]
    #[inline] pub fn test_lrswap(&self) -> bool {
        self.lrswap() != 0
    }

    #[doc="Sets the LRSWAP field."]
    #[inline] pub fn set_lrswap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Right channel PGA gain."]
    #[inline] pub fn pgaright(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0xf) as u8) } // [30:27]
    }

    #[doc="Returns true if PGARIGHT != 0"]
    #[inline] pub fn test_pgaright(&self) -> bool {
        self.pgaright() != 0
    }

    #[doc="Sets the PGARIGHT field."]
    #[inline] pub fn set_pgaright<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Left channel PGA gain."]
    #[inline] pub fn pgaleft(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0xf) as u8) } // [26:23]
    }

    #[doc="Returns true if PGALEFT != 0"]
    #[inline] pub fn test_pgaleft(&self) -> bool {
        self.pgaleft() != 0
    }

    #[doc="Sets the PGALEFT field."]
    #[inline] pub fn set_pgaleft<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="PDM_CLK frequency divisor."]
    #[inline] pub fn mclkdiv(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if MCLKDIV != 0"]
    #[inline] pub fn test_mclkdiv(&self) -> bool {
        self.mclkdiv() != 0
    }

    #[doc="Sets the MCLKDIV field."]
    #[inline] pub fn set_mclkdiv<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SINC decimation rate."]
    #[inline] pub fn sincrate(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x7f) as u8) } // [16:10]
    }

    #[doc="Returns true if SINCRATE != 0"]
    #[inline] pub fn test_sincrate(&self) -> bool {
        self.sincrate() != 0
    }

    #[doc="Sets the SINCRATE field."]
    #[inline] pub fn set_sincrate<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="High pass filter control."]
    #[inline] pub fn adchpd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ADCHPD != 0"]
    #[inline] pub fn test_adchpd(&self) -> bool {
        self.adchpd() != 0
    }

    #[doc="Sets the ADCHPD field."]
    #[inline] pub fn set_adchpd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="High pass filter coefficients."]
    #[inline] pub fn hpcutoff(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0xf) as u8) } // [8:5]
    }

    #[doc="Returns true if HPCUTOFF != 0"]
    #[inline] pub fn test_hpcutoff(&self) -> bool {
        self.hpcutoff() != 0
    }

    #[doc="Sets the HPCUTOFF field."]
    #[inline] pub fn set_hpcutoff<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Number of clocks during gain-setting changes."]
    #[inline] pub fn cycles(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
    }

    #[doc="Returns true if CYCLES != 0"]
    #[inline] pub fn test_cycles(&self) -> bool {
        self.cycles() != 0
    }

    #[doc="Sets the CYCLES field."]
    #[inline] pub fn set_cycles<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Soft mute control."]
    #[inline] pub fn softmute(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SOFTMUTE != 0"]
    #[inline] pub fn test_softmute(&self) -> bool {
        self.softmute() != 0
    }

    #[doc="Sets the SOFTMUTE field."]
    #[inline] pub fn set_softmute<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data Streaming Control."]
    #[inline] pub fn pdmcore(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PDMCORE != 0"]
    #[inline] pub fn test_pdmcore(&self) -> bool {
        self.pdmcore() != 0
    }

    #[doc="Sets the PDMCORE field."]
    #[inline] pub fn set_pdmcore<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Pcfg(other)
    }
}

impl ::core::fmt::Display for Pcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lrswap() != 0 { try!(write!(f, " lrswap"))}
        if self.pgaright() != 0 { try!(write!(f, " pgaright=0x{:x}", self.pgaright()))}
        if self.pgaleft() != 0 { try!(write!(f, " pgaleft=0x{:x}", self.pgaleft()))}
        if self.mclkdiv() != 0 { try!(write!(f, " mclkdiv=0x{:x}", self.mclkdiv()))}
        if self.sincrate() != 0 { try!(write!(f, " sincrate=0x{:x}", self.sincrate()))}
        if self.adchpd() != 0 { try!(write!(f, " adchpd"))}
        if self.hpcutoff() != 0 { try!(write!(f, " hpcutoff=0x{:x}", self.hpcutoff()))}
        if self.cycles() != 0 { try!(write!(f, " cycles=0x{:x}", self.cycles()))}
        if self.softmute() != 0 { try!(write!(f, " softmute"))}
        if self.pdmcore() != 0 { try!(write!(f, " pdmcore"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Voice Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vcfg(pub u32);
impl Vcfg {
    #[doc="Enable the IO clock."]
    #[inline] pub fn ioclken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if IOCLKEN != 0"]
    #[inline] pub fn test_ioclken(&self) -> bool {
        self.ioclken() != 0
    }

    #[doc="Sets the IOCLKEN field."]
    #[inline] pub fn set_ioclken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Reset the IP core."]
    #[inline] pub fn rstb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if RSTB != 0"]
    #[inline] pub fn test_rstb(&self) -> bool {
        self.rstb() != 0
    }

    #[doc="Sets the RSTB field."]
    #[inline] pub fn set_rstb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Select the PDM input clock."]
    #[inline] pub fn pdmclksel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if PDMCLKSEL != 0"]
    #[inline] pub fn test_pdmclksel(&self) -> bool {
        self.pdmclksel() != 0
    }

    #[doc="Sets the PDMCLKSEL field."]
    #[inline] pub fn set_pdmclksel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Enable the serial clock."]
    #[inline] pub fn pdmclk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PDMCLK != 0"]
    #[inline] pub fn test_pdmclk(&self) -> bool {
        self.pdmclk() != 0
    }

    #[doc="Sets the PDMCLK field."]
    #[inline] pub fn set_pdmclk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="I2S interface enable."]
    #[inline] pub fn i2smode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if I2SMODE != 0"]
    #[inline] pub fn test_i2smode(&self) -> bool {
        self.i2smode() != 0
    }

    #[doc="Sets the I2SMODE field."]
    #[inline] pub fn set_i2smode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="I2S BCLK input inversion."]
    #[inline] pub fn bclkinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if BCLKINV != 0"]
    #[inline] pub fn test_bclkinv(&self) -> bool {
        self.bclkinv() != 0
    }

    #[doc="Sets the BCLKINV field."]
    #[inline] pub fn set_bclkinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="PDM clock sampling delay."]
    #[inline] pub fn dmickdel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DMICKDEL != 0"]
    #[inline] pub fn test_dmickdel(&self) -> bool {
        self.dmickdel() != 0
    }

    #[doc="Sets the DMICKDEL field."]
    #[inline] pub fn set_dmickdel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Select PDM input clock source."]
    #[inline] pub fn selap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SELAP != 0"]
    #[inline] pub fn test_selap(&self) -> bool {
        self.selap() != 0
    }

    #[doc="Sets the SELAP field."]
    #[inline] pub fn set_selap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="PCM data packing enable."]
    #[inline] pub fn pcmpack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PCMPACK != 0"]
    #[inline] pub fn test_pcmpack(&self) -> bool {
        self.pcmpack() != 0
    }

    #[doc="Sets the PCMPACK field."]
    #[inline] pub fn set_pcmpack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Set PCM channels."]
    #[inline] pub fn chset(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if CHSET != 0"]
    #[inline] pub fn test_chset(&self) -> bool {
        self.chset() != 0
    }

    #[doc="Sets the CHSET field."]
    #[inline] pub fn set_chset<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Vcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Vcfg(other)
    }
}

impl ::core::fmt::Display for Vcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ioclken() != 0 { try!(write!(f, " ioclken"))}
        if self.rstb() != 0 { try!(write!(f, " rstb"))}
        if self.pdmclksel() != 0 { try!(write!(f, " pdmclksel=0x{:x}", self.pdmclksel()))}
        if self.pdmclk() != 0 { try!(write!(f, " pdmclk"))}
        if self.i2smode() != 0 { try!(write!(f, " i2smode"))}
        if self.bclkinv() != 0 { try!(write!(f, " bclkinv"))}
        if self.dmickdel() != 0 { try!(write!(f, " dmickdel"))}
        if self.selap() != 0 { try!(write!(f, " selap"))}
        if self.pcmpack() != 0 { try!(write!(f, " pcmpack"))}
        if self.chset() != 0 { try!(write!(f, " chset=0x{:x}", self.chset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Voice Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fr(pub u32);
impl Fr {
    #[doc="Valid 32-bit entries currently in the FIFO."]
    #[inline] pub fn fifocnt(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if FIFOCNT != 0"]
    #[inline] pub fn test_fifocnt(&self) -> bool {
        self.fifocnt() != 0
    }

    #[doc="Sets the FIFOCNT field."]
    #[inline] pub fn set_fifocnt<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
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
        if self.fifocnt() != 0 { try!(write!(f, " fifocnt=0x{:x}", self.fifocnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO Read"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Frd(pub u32);
impl Frd {
    #[doc="FIFO read data."]
    #[inline] pub fn fiforead(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if FIFOREAD != 0"]
    #[inline] pub fn test_fiforead(&self) -> bool {
        self.fiforead() != 0
    }

    #[doc="Sets the FIFOREAD field."]
    #[inline] pub fn set_fiforead<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Frd {
    #[inline]
    fn from(other: u32) -> Self {
         Frd(other)
    }
}

impl ::core::fmt::Display for Frd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Frd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO Flush"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flush(pub u32);
impl Flush {
    #[doc="FIFO FLUSH."]
    #[inline] pub fn fifoflush(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FIFOFLUSH != 0"]
    #[inline] pub fn test_fifoflush(&self) -> bool {
        self.fifoflush() != 0
    }

    #[doc="Sets the FIFOFLUSH field."]
    #[inline] pub fn set_fifoflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Flush {
    #[inline]
    fn from(other: u32) -> Self {
         Flush(other)
    }
}

impl ::core::fmt::Display for Flush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Flush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifoflush() != 0 { try!(write!(f, " fifoflush"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fthr(pub u32);
impl Fthr {
    #[doc="FIFO interrupt threshold."]
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

impl From<u32> for Fthr {
    #[inline]
    fn from(other: u32) -> Self {
         Fthr(other)
    }
}

impl ::core::fmt::Display for Fthr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fthr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifothr() != 0 { try!(write!(f, " fifothr=0x{:x}", self.fifothr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Master Interrupts: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc="This is the FIFO underflow interrupt."]
    #[inline] pub fn undfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if UNDFL != 0"]
    #[inline] pub fn test_undfl(&self) -> bool {
        self.undfl() != 0
    }

    #[doc="Sets the UNDFL field."]
    #[inline] pub fn set_undfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This is the FIFO overflow interrupt."]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This is the FIFO threshold interrupt."]
    #[inline] pub fn thr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if THR != 0"]
    #[inline] pub fn test_thr(&self) -> bool {
        self.thr() != 0
    }

    #[doc="Sets the THR field."]
    #[inline] pub fn set_thr<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.undfl() != 0 { try!(write!(f, " undfl"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.thr() != 0 { try!(write!(f, " thr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Master Interrupts: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc="This is the FIFO underflow interrupt."]
    #[inline] pub fn undfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if UNDFL != 0"]
    #[inline] pub fn test_undfl(&self) -> bool {
        self.undfl() != 0
    }

    #[doc="Sets the UNDFL field."]
    #[inline] pub fn set_undfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This is the FIFO overflow interrupt."]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This is the FIFO threshold interrupt."]
    #[inline] pub fn thr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if THR != 0"]
    #[inline] pub fn test_thr(&self) -> bool {
        self.thr() != 0
    }

    #[doc="Sets the THR field."]
    #[inline] pub fn set_thr<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.undfl() != 0 { try!(write!(f, " undfl"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.thr() != 0 { try!(write!(f, " thr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Master Interrupts: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc="This is the FIFO underflow interrupt."]
    #[inline] pub fn undfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if UNDFL != 0"]
    #[inline] pub fn test_undfl(&self) -> bool {
        self.undfl() != 0
    }

    #[doc="Sets the UNDFL field."]
    #[inline] pub fn set_undfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This is the FIFO overflow interrupt."]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This is the FIFO threshold interrupt."]
    #[inline] pub fn thr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if THR != 0"]
    #[inline] pub fn test_thr(&self) -> bool {
        self.thr() != 0
    }

    #[doc="Sets the THR field."]
    #[inline] pub fn set_thr<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.undfl() != 0 { try!(write!(f, " undfl"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.thr() != 0 { try!(write!(f, " thr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Master Interrupts: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc="This is the FIFO underflow interrupt."]
    #[inline] pub fn undfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if UNDFL != 0"]
    #[inline] pub fn test_undfl(&self) -> bool {
        self.undfl() != 0
    }

    #[doc="Sets the UNDFL field."]
    #[inline] pub fn set_undfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This is the FIFO overflow interrupt."]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This is the FIFO threshold interrupt."]
    #[inline] pub fn thr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if THR != 0"]
    #[inline] pub fn test_thr(&self) -> bool {
        self.thr() != 0
    }

    #[doc="Sets the THR field."]
    #[inline] pub fn set_thr<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.undfl() != 0 { try!(write!(f, " undfl"))}
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.thr() != 0 { try!(write!(f, " thr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


