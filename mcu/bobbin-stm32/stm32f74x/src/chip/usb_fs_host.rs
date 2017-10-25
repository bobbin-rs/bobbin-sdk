//! USB on the go full speed
#[allow(unused_imports)] use bobbin_common::*;

periph!(USB_FS_HOST, UsbFsHost, 0x50000400);

#[doc="USB on the go full speed"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UsbFsHost(pub usize);
impl UsbFsHost {
    #[doc="Get the *mut pointer for the HCFG register."]
    #[inline] pub fn hcfg_mut(&self) -> *mut Hcfg { 
        (self.0 + 0x0) as *mut Hcfg
    }

    #[doc="Get the *const pointer for the HCFG register."]
    #[inline] pub fn hcfg_ptr(&self) -> *const Hcfg { 
           self.hcfg_mut()
    }

    #[doc="Read the HCFG register."]
    #[inline] pub fn hcfg(&self) -> Hcfg { 
        unsafe {
            read_volatile(self.hcfg_ptr())
        }
    }

    #[doc="Write the HCFG register."]
    #[inline] pub fn set_hcfg<F: FnOnce(Hcfg) -> Hcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcfg_mut(), f(Hcfg(0)));
        }
        self
    }

    #[doc="Modify the HCFG register."]
    #[inline] pub fn with_hcfg<F: FnOnce(Hcfg) -> Hcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcfg_mut(), f(self.hcfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFIR register."]
    #[inline] pub fn hfir_mut(&self) -> *mut Hfir { 
        (self.0 + 0x4) as *mut Hfir
    }

    #[doc="Get the *const pointer for the HFIR register."]
    #[inline] pub fn hfir_ptr(&self) -> *const Hfir { 
           self.hfir_mut()
    }

    #[doc="Read the HFIR register."]
    #[inline] pub fn hfir(&self) -> Hfir { 
        unsafe {
            read_volatile(self.hfir_ptr())
        }
    }

    #[doc="Write the HFIR register."]
    #[inline] pub fn set_hfir<F: FnOnce(Hfir) -> Hfir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfir_mut(), f(Hfir(0)));
        }
        self
    }

    #[doc="Modify the HFIR register."]
    #[inline] pub fn with_hfir<F: FnOnce(Hfir) -> Hfir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfir_mut(), f(self.hfir()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFNUM register."]
    #[inline] pub fn hfnum_mut(&self) -> *mut Hfnum { 
        (self.0 + 0x8) as *mut Hfnum
    }

    #[doc="Get the *const pointer for the HFNUM register."]
    #[inline] pub fn hfnum_ptr(&self) -> *const Hfnum { 
           self.hfnum_mut()
    }

    #[doc="Read the HFNUM register."]
    #[inline] pub fn hfnum(&self) -> Hfnum { 
        unsafe {
            read_volatile(self.hfnum_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HPTXSTS register."]
    #[inline] pub fn hptxsts_mut(&self) -> *mut Hptxsts { 
        (self.0 + 0x10) as *mut Hptxsts
    }

    #[doc="Get the *const pointer for the HPTXSTS register."]
    #[inline] pub fn hptxsts_ptr(&self) -> *const Hptxsts { 
           self.hptxsts_mut()
    }

    #[doc="Read the HPTXSTS register."]
    #[inline] pub fn hptxsts(&self) -> Hptxsts { 
        unsafe {
            read_volatile(self.hptxsts_ptr())
        }
    }

    #[doc="Write the HPTXSTS register."]
    #[inline] pub fn set_hptxsts<F: FnOnce(Hptxsts) -> Hptxsts>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hptxsts_mut(), f(Hptxsts(0)));
        }
        self
    }

    #[doc="Modify the HPTXSTS register."]
    #[inline] pub fn with_hptxsts<F: FnOnce(Hptxsts) -> Hptxsts>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hptxsts_mut(), f(self.hptxsts()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HAINT register."]
    #[inline] pub fn haint_mut(&self) -> *mut Haint { 
        (self.0 + 0x14) as *mut Haint
    }

    #[doc="Get the *const pointer for the HAINT register."]
    #[inline] pub fn haint_ptr(&self) -> *const Haint { 
           self.haint_mut()
    }

    #[doc="Read the HAINT register."]
    #[inline] pub fn haint(&self) -> Haint { 
        unsafe {
            read_volatile(self.haint_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HAINTMSK register."]
    #[inline] pub fn haintmsk_mut(&self) -> *mut Haintmsk { 
        (self.0 + 0x18) as *mut Haintmsk
    }

    #[doc="Get the *const pointer for the HAINTMSK register."]
    #[inline] pub fn haintmsk_ptr(&self) -> *const Haintmsk { 
           self.haintmsk_mut()
    }

    #[doc="Read the HAINTMSK register."]
    #[inline] pub fn haintmsk(&self) -> Haintmsk { 
        unsafe {
            read_volatile(self.haintmsk_ptr())
        }
    }

    #[doc="Write the HAINTMSK register."]
    #[inline] pub fn set_haintmsk<F: FnOnce(Haintmsk) -> Haintmsk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.haintmsk_mut(), f(Haintmsk(0)));
        }
        self
    }

    #[doc="Modify the HAINTMSK register."]
    #[inline] pub fn with_haintmsk<F: FnOnce(Haintmsk) -> Haintmsk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.haintmsk_mut(), f(self.haintmsk()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HPRT register."]
    #[inline] pub fn hprt_mut(&self) -> *mut Hprt { 
        (self.0 + 0x40) as *mut Hprt
    }

    #[doc="Get the *const pointer for the HPRT register."]
    #[inline] pub fn hprt_ptr(&self) -> *const Hprt { 
           self.hprt_mut()
    }

    #[doc="Read the HPRT register."]
    #[inline] pub fn hprt(&self) -> Hprt { 
        unsafe {
            read_volatile(self.hprt_ptr())
        }
    }

    #[doc="Write the HPRT register."]
    #[inline] pub fn set_hprt<F: FnOnce(Hprt) -> Hprt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hprt_mut(), f(Hprt(0)));
        }
        self
    }

    #[doc="Modify the HPRT register."]
    #[inline] pub fn with_hprt<F: FnOnce(Hprt) -> Hprt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hprt_mut(), f(self.hprt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR0 register."]
    #[inline] pub fn hcchar0_mut(&self) -> *mut Hcchar0 { 
        (self.0 + 0x100) as *mut Hcchar0
    }

    #[doc="Get the *const pointer for the HCCHAR0 register."]
    #[inline] pub fn hcchar0_ptr(&self) -> *const Hcchar0 { 
           self.hcchar0_mut()
    }

    #[doc="Read the HCCHAR0 register."]
    #[inline] pub fn hcchar0(&self) -> Hcchar0 { 
        unsafe {
            read_volatile(self.hcchar0_ptr())
        }
    }

    #[doc="Write the HCCHAR0 register."]
    #[inline] pub fn set_hcchar0<F: FnOnce(Hcchar0) -> Hcchar0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar0_mut(), f(Hcchar0(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR0 register."]
    #[inline] pub fn with_hcchar0<F: FnOnce(Hcchar0) -> Hcchar0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar0_mut(), f(self.hcchar0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR1 register."]
    #[inline] pub fn hcchar1_mut(&self) -> *mut Hcchar1 { 
        (self.0 + 0x120) as *mut Hcchar1
    }

    #[doc="Get the *const pointer for the HCCHAR1 register."]
    #[inline] pub fn hcchar1_ptr(&self) -> *const Hcchar1 { 
           self.hcchar1_mut()
    }

    #[doc="Read the HCCHAR1 register."]
    #[inline] pub fn hcchar1(&self) -> Hcchar1 { 
        unsafe {
            read_volatile(self.hcchar1_ptr())
        }
    }

    #[doc="Write the HCCHAR1 register."]
    #[inline] pub fn set_hcchar1<F: FnOnce(Hcchar1) -> Hcchar1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar1_mut(), f(Hcchar1(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR1 register."]
    #[inline] pub fn with_hcchar1<F: FnOnce(Hcchar1) -> Hcchar1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar1_mut(), f(self.hcchar1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR2 register."]
    #[inline] pub fn hcchar2_mut(&self) -> *mut Hcchar2 { 
        (self.0 + 0x140) as *mut Hcchar2
    }

    #[doc="Get the *const pointer for the HCCHAR2 register."]
    #[inline] pub fn hcchar2_ptr(&self) -> *const Hcchar2 { 
           self.hcchar2_mut()
    }

    #[doc="Read the HCCHAR2 register."]
    #[inline] pub fn hcchar2(&self) -> Hcchar2 { 
        unsafe {
            read_volatile(self.hcchar2_ptr())
        }
    }

    #[doc="Write the HCCHAR2 register."]
    #[inline] pub fn set_hcchar2<F: FnOnce(Hcchar2) -> Hcchar2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar2_mut(), f(Hcchar2(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR2 register."]
    #[inline] pub fn with_hcchar2<F: FnOnce(Hcchar2) -> Hcchar2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar2_mut(), f(self.hcchar2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR3 register."]
    #[inline] pub fn hcchar3_mut(&self) -> *mut Hcchar3 { 
        (self.0 + 0x160) as *mut Hcchar3
    }

    #[doc="Get the *const pointer for the HCCHAR3 register."]
    #[inline] pub fn hcchar3_ptr(&self) -> *const Hcchar3 { 
           self.hcchar3_mut()
    }

    #[doc="Read the HCCHAR3 register."]
    #[inline] pub fn hcchar3(&self) -> Hcchar3 { 
        unsafe {
            read_volatile(self.hcchar3_ptr())
        }
    }

    #[doc="Write the HCCHAR3 register."]
    #[inline] pub fn set_hcchar3<F: FnOnce(Hcchar3) -> Hcchar3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar3_mut(), f(Hcchar3(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR3 register."]
    #[inline] pub fn with_hcchar3<F: FnOnce(Hcchar3) -> Hcchar3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar3_mut(), f(self.hcchar3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR4 register."]
    #[inline] pub fn hcchar4_mut(&self) -> *mut Hcchar4 { 
        (self.0 + 0x180) as *mut Hcchar4
    }

    #[doc="Get the *const pointer for the HCCHAR4 register."]
    #[inline] pub fn hcchar4_ptr(&self) -> *const Hcchar4 { 
           self.hcchar4_mut()
    }

    #[doc="Read the HCCHAR4 register."]
    #[inline] pub fn hcchar4(&self) -> Hcchar4 { 
        unsafe {
            read_volatile(self.hcchar4_ptr())
        }
    }

    #[doc="Write the HCCHAR4 register."]
    #[inline] pub fn set_hcchar4<F: FnOnce(Hcchar4) -> Hcchar4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar4_mut(), f(Hcchar4(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR4 register."]
    #[inline] pub fn with_hcchar4<F: FnOnce(Hcchar4) -> Hcchar4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar4_mut(), f(self.hcchar4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR5 register."]
    #[inline] pub fn hcchar5_mut(&self) -> *mut Hcchar5 { 
        (self.0 + 0x1a0) as *mut Hcchar5
    }

    #[doc="Get the *const pointer for the HCCHAR5 register."]
    #[inline] pub fn hcchar5_ptr(&self) -> *const Hcchar5 { 
           self.hcchar5_mut()
    }

    #[doc="Read the HCCHAR5 register."]
    #[inline] pub fn hcchar5(&self) -> Hcchar5 { 
        unsafe {
            read_volatile(self.hcchar5_ptr())
        }
    }

    #[doc="Write the HCCHAR5 register."]
    #[inline] pub fn set_hcchar5<F: FnOnce(Hcchar5) -> Hcchar5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar5_mut(), f(Hcchar5(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR5 register."]
    #[inline] pub fn with_hcchar5<F: FnOnce(Hcchar5) -> Hcchar5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar5_mut(), f(self.hcchar5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR6 register."]
    #[inline] pub fn hcchar6_mut(&self) -> *mut Hcchar6 { 
        (self.0 + 0x1c0) as *mut Hcchar6
    }

    #[doc="Get the *const pointer for the HCCHAR6 register."]
    #[inline] pub fn hcchar6_ptr(&self) -> *const Hcchar6 { 
           self.hcchar6_mut()
    }

    #[doc="Read the HCCHAR6 register."]
    #[inline] pub fn hcchar6(&self) -> Hcchar6 { 
        unsafe {
            read_volatile(self.hcchar6_ptr())
        }
    }

    #[doc="Write the HCCHAR6 register."]
    #[inline] pub fn set_hcchar6<F: FnOnce(Hcchar6) -> Hcchar6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar6_mut(), f(Hcchar6(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR6 register."]
    #[inline] pub fn with_hcchar6<F: FnOnce(Hcchar6) -> Hcchar6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar6_mut(), f(self.hcchar6()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR7 register."]
    #[inline] pub fn hcchar7_mut(&self) -> *mut Hcchar7 { 
        (self.0 + 0x1e0) as *mut Hcchar7
    }

    #[doc="Get the *const pointer for the HCCHAR7 register."]
    #[inline] pub fn hcchar7_ptr(&self) -> *const Hcchar7 { 
           self.hcchar7_mut()
    }

    #[doc="Read the HCCHAR7 register."]
    #[inline] pub fn hcchar7(&self) -> Hcchar7 { 
        unsafe {
            read_volatile(self.hcchar7_ptr())
        }
    }

    #[doc="Write the HCCHAR7 register."]
    #[inline] pub fn set_hcchar7<F: FnOnce(Hcchar7) -> Hcchar7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar7_mut(), f(Hcchar7(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR7 register."]
    #[inline] pub fn with_hcchar7<F: FnOnce(Hcchar7) -> Hcchar7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar7_mut(), f(self.hcchar7()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT0 register."]
    #[inline] pub fn hcint0_mut(&self) -> *mut Hcint0 { 
        (self.0 + 0x108) as *mut Hcint0
    }

    #[doc="Get the *const pointer for the HCINT0 register."]
    #[inline] pub fn hcint0_ptr(&self) -> *const Hcint0 { 
           self.hcint0_mut()
    }

    #[doc="Read the HCINT0 register."]
    #[inline] pub fn hcint0(&self) -> Hcint0 { 
        unsafe {
            read_volatile(self.hcint0_ptr())
        }
    }

    #[doc="Write the HCINT0 register."]
    #[inline] pub fn set_hcint0<F: FnOnce(Hcint0) -> Hcint0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint0_mut(), f(Hcint0(0)));
        }
        self
    }

    #[doc="Modify the HCINT0 register."]
    #[inline] pub fn with_hcint0<F: FnOnce(Hcint0) -> Hcint0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint0_mut(), f(self.hcint0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT1 register."]
    #[inline] pub fn hcint1_mut(&self) -> *mut Hcint1 { 
        (self.0 + 0x128) as *mut Hcint1
    }

    #[doc="Get the *const pointer for the HCINT1 register."]
    #[inline] pub fn hcint1_ptr(&self) -> *const Hcint1 { 
           self.hcint1_mut()
    }

    #[doc="Read the HCINT1 register."]
    #[inline] pub fn hcint1(&self) -> Hcint1 { 
        unsafe {
            read_volatile(self.hcint1_ptr())
        }
    }

    #[doc="Write the HCINT1 register."]
    #[inline] pub fn set_hcint1<F: FnOnce(Hcint1) -> Hcint1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint1_mut(), f(Hcint1(0)));
        }
        self
    }

    #[doc="Modify the HCINT1 register."]
    #[inline] pub fn with_hcint1<F: FnOnce(Hcint1) -> Hcint1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint1_mut(), f(self.hcint1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT2 register."]
    #[inline] pub fn hcint2_mut(&self) -> *mut Hcint2 { 
        (self.0 + 0x148) as *mut Hcint2
    }

    #[doc="Get the *const pointer for the HCINT2 register."]
    #[inline] pub fn hcint2_ptr(&self) -> *const Hcint2 { 
           self.hcint2_mut()
    }

    #[doc="Read the HCINT2 register."]
    #[inline] pub fn hcint2(&self) -> Hcint2 { 
        unsafe {
            read_volatile(self.hcint2_ptr())
        }
    }

    #[doc="Write the HCINT2 register."]
    #[inline] pub fn set_hcint2<F: FnOnce(Hcint2) -> Hcint2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint2_mut(), f(Hcint2(0)));
        }
        self
    }

    #[doc="Modify the HCINT2 register."]
    #[inline] pub fn with_hcint2<F: FnOnce(Hcint2) -> Hcint2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint2_mut(), f(self.hcint2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT3 register."]
    #[inline] pub fn hcint3_mut(&self) -> *mut Hcint3 { 
        (self.0 + 0x168) as *mut Hcint3
    }

    #[doc="Get the *const pointer for the HCINT3 register."]
    #[inline] pub fn hcint3_ptr(&self) -> *const Hcint3 { 
           self.hcint3_mut()
    }

    #[doc="Read the HCINT3 register."]
    #[inline] pub fn hcint3(&self) -> Hcint3 { 
        unsafe {
            read_volatile(self.hcint3_ptr())
        }
    }

    #[doc="Write the HCINT3 register."]
    #[inline] pub fn set_hcint3<F: FnOnce(Hcint3) -> Hcint3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint3_mut(), f(Hcint3(0)));
        }
        self
    }

    #[doc="Modify the HCINT3 register."]
    #[inline] pub fn with_hcint3<F: FnOnce(Hcint3) -> Hcint3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint3_mut(), f(self.hcint3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT4 register."]
    #[inline] pub fn hcint4_mut(&self) -> *mut Hcint4 { 
        (self.0 + 0x188) as *mut Hcint4
    }

    #[doc="Get the *const pointer for the HCINT4 register."]
    #[inline] pub fn hcint4_ptr(&self) -> *const Hcint4 { 
           self.hcint4_mut()
    }

    #[doc="Read the HCINT4 register."]
    #[inline] pub fn hcint4(&self) -> Hcint4 { 
        unsafe {
            read_volatile(self.hcint4_ptr())
        }
    }

    #[doc="Write the HCINT4 register."]
    #[inline] pub fn set_hcint4<F: FnOnce(Hcint4) -> Hcint4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint4_mut(), f(Hcint4(0)));
        }
        self
    }

    #[doc="Modify the HCINT4 register."]
    #[inline] pub fn with_hcint4<F: FnOnce(Hcint4) -> Hcint4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint4_mut(), f(self.hcint4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT5 register."]
    #[inline] pub fn hcint5_mut(&self) -> *mut Hcint5 { 
        (self.0 + 0x1a8) as *mut Hcint5
    }

    #[doc="Get the *const pointer for the HCINT5 register."]
    #[inline] pub fn hcint5_ptr(&self) -> *const Hcint5 { 
           self.hcint5_mut()
    }

    #[doc="Read the HCINT5 register."]
    #[inline] pub fn hcint5(&self) -> Hcint5 { 
        unsafe {
            read_volatile(self.hcint5_ptr())
        }
    }

    #[doc="Write the HCINT5 register."]
    #[inline] pub fn set_hcint5<F: FnOnce(Hcint5) -> Hcint5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint5_mut(), f(Hcint5(0)));
        }
        self
    }

    #[doc="Modify the HCINT5 register."]
    #[inline] pub fn with_hcint5<F: FnOnce(Hcint5) -> Hcint5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint5_mut(), f(self.hcint5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT6 register."]
    #[inline] pub fn hcint6_mut(&self) -> *mut Hcint6 { 
        (self.0 + 0x1c8) as *mut Hcint6
    }

    #[doc="Get the *const pointer for the HCINT6 register."]
    #[inline] pub fn hcint6_ptr(&self) -> *const Hcint6 { 
           self.hcint6_mut()
    }

    #[doc="Read the HCINT6 register."]
    #[inline] pub fn hcint6(&self) -> Hcint6 { 
        unsafe {
            read_volatile(self.hcint6_ptr())
        }
    }

    #[doc="Write the HCINT6 register."]
    #[inline] pub fn set_hcint6<F: FnOnce(Hcint6) -> Hcint6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint6_mut(), f(Hcint6(0)));
        }
        self
    }

    #[doc="Modify the HCINT6 register."]
    #[inline] pub fn with_hcint6<F: FnOnce(Hcint6) -> Hcint6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint6_mut(), f(self.hcint6()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT7 register."]
    #[inline] pub fn hcint7_mut(&self) -> *mut Hcint7 { 
        (self.0 + 0x1e8) as *mut Hcint7
    }

    #[doc="Get the *const pointer for the HCINT7 register."]
    #[inline] pub fn hcint7_ptr(&self) -> *const Hcint7 { 
           self.hcint7_mut()
    }

    #[doc="Read the HCINT7 register."]
    #[inline] pub fn hcint7(&self) -> Hcint7 { 
        unsafe {
            read_volatile(self.hcint7_ptr())
        }
    }

    #[doc="Write the HCINT7 register."]
    #[inline] pub fn set_hcint7<F: FnOnce(Hcint7) -> Hcint7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint7_mut(), f(Hcint7(0)));
        }
        self
    }

    #[doc="Modify the HCINT7 register."]
    #[inline] pub fn with_hcint7<F: FnOnce(Hcint7) -> Hcint7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint7_mut(), f(self.hcint7()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK0 register."]
    #[inline] pub fn hcintmsk0_mut(&self) -> *mut Hcintmsk0 { 
        (self.0 + 0x10c) as *mut Hcintmsk0
    }

    #[doc="Get the *const pointer for the HCINTMSK0 register."]
    #[inline] pub fn hcintmsk0_ptr(&self) -> *const Hcintmsk0 { 
           self.hcintmsk0_mut()
    }

    #[doc="Read the HCINTMSK0 register."]
    #[inline] pub fn hcintmsk0(&self) -> Hcintmsk0 { 
        unsafe {
            read_volatile(self.hcintmsk0_ptr())
        }
    }

    #[doc="Write the HCINTMSK0 register."]
    #[inline] pub fn set_hcintmsk0<F: FnOnce(Hcintmsk0) -> Hcintmsk0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk0_mut(), f(Hcintmsk0(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK0 register."]
    #[inline] pub fn with_hcintmsk0<F: FnOnce(Hcintmsk0) -> Hcintmsk0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk0_mut(), f(self.hcintmsk0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK1 register."]
    #[inline] pub fn hcintmsk1_mut(&self) -> *mut Hcintmsk1 { 
        (self.0 + 0x12c) as *mut Hcintmsk1
    }

    #[doc="Get the *const pointer for the HCINTMSK1 register."]
    #[inline] pub fn hcintmsk1_ptr(&self) -> *const Hcintmsk1 { 
           self.hcintmsk1_mut()
    }

    #[doc="Read the HCINTMSK1 register."]
    #[inline] pub fn hcintmsk1(&self) -> Hcintmsk1 { 
        unsafe {
            read_volatile(self.hcintmsk1_ptr())
        }
    }

    #[doc="Write the HCINTMSK1 register."]
    #[inline] pub fn set_hcintmsk1<F: FnOnce(Hcintmsk1) -> Hcintmsk1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk1_mut(), f(Hcintmsk1(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK1 register."]
    #[inline] pub fn with_hcintmsk1<F: FnOnce(Hcintmsk1) -> Hcintmsk1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk1_mut(), f(self.hcintmsk1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK2 register."]
    #[inline] pub fn hcintmsk2_mut(&self) -> *mut Hcintmsk2 { 
        (self.0 + 0x14c) as *mut Hcintmsk2
    }

    #[doc="Get the *const pointer for the HCINTMSK2 register."]
    #[inline] pub fn hcintmsk2_ptr(&self) -> *const Hcintmsk2 { 
           self.hcintmsk2_mut()
    }

    #[doc="Read the HCINTMSK2 register."]
    #[inline] pub fn hcintmsk2(&self) -> Hcintmsk2 { 
        unsafe {
            read_volatile(self.hcintmsk2_ptr())
        }
    }

    #[doc="Write the HCINTMSK2 register."]
    #[inline] pub fn set_hcintmsk2<F: FnOnce(Hcintmsk2) -> Hcintmsk2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk2_mut(), f(Hcintmsk2(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK2 register."]
    #[inline] pub fn with_hcintmsk2<F: FnOnce(Hcintmsk2) -> Hcintmsk2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk2_mut(), f(self.hcintmsk2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK3 register."]
    #[inline] pub fn hcintmsk3_mut(&self) -> *mut Hcintmsk3 { 
        (self.0 + 0x16c) as *mut Hcintmsk3
    }

    #[doc="Get the *const pointer for the HCINTMSK3 register."]
    #[inline] pub fn hcintmsk3_ptr(&self) -> *const Hcintmsk3 { 
           self.hcintmsk3_mut()
    }

    #[doc="Read the HCINTMSK3 register."]
    #[inline] pub fn hcintmsk3(&self) -> Hcintmsk3 { 
        unsafe {
            read_volatile(self.hcintmsk3_ptr())
        }
    }

    #[doc="Write the HCINTMSK3 register."]
    #[inline] pub fn set_hcintmsk3<F: FnOnce(Hcintmsk3) -> Hcintmsk3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk3_mut(), f(Hcintmsk3(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK3 register."]
    #[inline] pub fn with_hcintmsk3<F: FnOnce(Hcintmsk3) -> Hcintmsk3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk3_mut(), f(self.hcintmsk3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK4 register."]
    #[inline] pub fn hcintmsk4_mut(&self) -> *mut Hcintmsk4 { 
        (self.0 + 0x18c) as *mut Hcintmsk4
    }

    #[doc="Get the *const pointer for the HCINTMSK4 register."]
    #[inline] pub fn hcintmsk4_ptr(&self) -> *const Hcintmsk4 { 
           self.hcintmsk4_mut()
    }

    #[doc="Read the HCINTMSK4 register."]
    #[inline] pub fn hcintmsk4(&self) -> Hcintmsk4 { 
        unsafe {
            read_volatile(self.hcintmsk4_ptr())
        }
    }

    #[doc="Write the HCINTMSK4 register."]
    #[inline] pub fn set_hcintmsk4<F: FnOnce(Hcintmsk4) -> Hcintmsk4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk4_mut(), f(Hcintmsk4(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK4 register."]
    #[inline] pub fn with_hcintmsk4<F: FnOnce(Hcintmsk4) -> Hcintmsk4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk4_mut(), f(self.hcintmsk4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK5 register."]
    #[inline] pub fn hcintmsk5_mut(&self) -> *mut Hcintmsk5 { 
        (self.0 + 0x1ac) as *mut Hcintmsk5
    }

    #[doc="Get the *const pointer for the HCINTMSK5 register."]
    #[inline] pub fn hcintmsk5_ptr(&self) -> *const Hcintmsk5 { 
           self.hcintmsk5_mut()
    }

    #[doc="Read the HCINTMSK5 register."]
    #[inline] pub fn hcintmsk5(&self) -> Hcintmsk5 { 
        unsafe {
            read_volatile(self.hcintmsk5_ptr())
        }
    }

    #[doc="Write the HCINTMSK5 register."]
    #[inline] pub fn set_hcintmsk5<F: FnOnce(Hcintmsk5) -> Hcintmsk5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk5_mut(), f(Hcintmsk5(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK5 register."]
    #[inline] pub fn with_hcintmsk5<F: FnOnce(Hcintmsk5) -> Hcintmsk5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk5_mut(), f(self.hcintmsk5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK6 register."]
    #[inline] pub fn hcintmsk6_mut(&self) -> *mut Hcintmsk6 { 
        (self.0 + 0x1cc) as *mut Hcintmsk6
    }

    #[doc="Get the *const pointer for the HCINTMSK6 register."]
    #[inline] pub fn hcintmsk6_ptr(&self) -> *const Hcintmsk6 { 
           self.hcintmsk6_mut()
    }

    #[doc="Read the HCINTMSK6 register."]
    #[inline] pub fn hcintmsk6(&self) -> Hcintmsk6 { 
        unsafe {
            read_volatile(self.hcintmsk6_ptr())
        }
    }

    #[doc="Write the HCINTMSK6 register."]
    #[inline] pub fn set_hcintmsk6<F: FnOnce(Hcintmsk6) -> Hcintmsk6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk6_mut(), f(Hcintmsk6(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK6 register."]
    #[inline] pub fn with_hcintmsk6<F: FnOnce(Hcintmsk6) -> Hcintmsk6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk6_mut(), f(self.hcintmsk6()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK7 register."]
    #[inline] pub fn hcintmsk7_mut(&self) -> *mut Hcintmsk7 { 
        (self.0 + 0x1ec) as *mut Hcintmsk7
    }

    #[doc="Get the *const pointer for the HCINTMSK7 register."]
    #[inline] pub fn hcintmsk7_ptr(&self) -> *const Hcintmsk7 { 
           self.hcintmsk7_mut()
    }

    #[doc="Read the HCINTMSK7 register."]
    #[inline] pub fn hcintmsk7(&self) -> Hcintmsk7 { 
        unsafe {
            read_volatile(self.hcintmsk7_ptr())
        }
    }

    #[doc="Write the HCINTMSK7 register."]
    #[inline] pub fn set_hcintmsk7<F: FnOnce(Hcintmsk7) -> Hcintmsk7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk7_mut(), f(Hcintmsk7(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK7 register."]
    #[inline] pub fn with_hcintmsk7<F: FnOnce(Hcintmsk7) -> Hcintmsk7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk7_mut(), f(self.hcintmsk7()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ0 register."]
    #[inline] pub fn hctsiz0_mut(&self) -> *mut Hctsiz0 { 
        (self.0 + 0x110) as *mut Hctsiz0
    }

    #[doc="Get the *const pointer for the HCTSIZ0 register."]
    #[inline] pub fn hctsiz0_ptr(&self) -> *const Hctsiz0 { 
           self.hctsiz0_mut()
    }

    #[doc="Read the HCTSIZ0 register."]
    #[inline] pub fn hctsiz0(&self) -> Hctsiz0 { 
        unsafe {
            read_volatile(self.hctsiz0_ptr())
        }
    }

    #[doc="Write the HCTSIZ0 register."]
    #[inline] pub fn set_hctsiz0<F: FnOnce(Hctsiz0) -> Hctsiz0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz0_mut(), f(Hctsiz0(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ0 register."]
    #[inline] pub fn with_hctsiz0<F: FnOnce(Hctsiz0) -> Hctsiz0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz0_mut(), f(self.hctsiz0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ1 register."]
    #[inline] pub fn hctsiz1_mut(&self) -> *mut Hctsiz1 { 
        (self.0 + 0x130) as *mut Hctsiz1
    }

    #[doc="Get the *const pointer for the HCTSIZ1 register."]
    #[inline] pub fn hctsiz1_ptr(&self) -> *const Hctsiz1 { 
           self.hctsiz1_mut()
    }

    #[doc="Read the HCTSIZ1 register."]
    #[inline] pub fn hctsiz1(&self) -> Hctsiz1 { 
        unsafe {
            read_volatile(self.hctsiz1_ptr())
        }
    }

    #[doc="Write the HCTSIZ1 register."]
    #[inline] pub fn set_hctsiz1<F: FnOnce(Hctsiz1) -> Hctsiz1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz1_mut(), f(Hctsiz1(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ1 register."]
    #[inline] pub fn with_hctsiz1<F: FnOnce(Hctsiz1) -> Hctsiz1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz1_mut(), f(self.hctsiz1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ2 register."]
    #[inline] pub fn hctsiz2_mut(&self) -> *mut Hctsiz2 { 
        (self.0 + 0x150) as *mut Hctsiz2
    }

    #[doc="Get the *const pointer for the HCTSIZ2 register."]
    #[inline] pub fn hctsiz2_ptr(&self) -> *const Hctsiz2 { 
           self.hctsiz2_mut()
    }

    #[doc="Read the HCTSIZ2 register."]
    #[inline] pub fn hctsiz2(&self) -> Hctsiz2 { 
        unsafe {
            read_volatile(self.hctsiz2_ptr())
        }
    }

    #[doc="Write the HCTSIZ2 register."]
    #[inline] pub fn set_hctsiz2<F: FnOnce(Hctsiz2) -> Hctsiz2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz2_mut(), f(Hctsiz2(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ2 register."]
    #[inline] pub fn with_hctsiz2<F: FnOnce(Hctsiz2) -> Hctsiz2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz2_mut(), f(self.hctsiz2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ3 register."]
    #[inline] pub fn hctsiz3_mut(&self) -> *mut Hctsiz3 { 
        (self.0 + 0x170) as *mut Hctsiz3
    }

    #[doc="Get the *const pointer for the HCTSIZ3 register."]
    #[inline] pub fn hctsiz3_ptr(&self) -> *const Hctsiz3 { 
           self.hctsiz3_mut()
    }

    #[doc="Read the HCTSIZ3 register."]
    #[inline] pub fn hctsiz3(&self) -> Hctsiz3 { 
        unsafe {
            read_volatile(self.hctsiz3_ptr())
        }
    }

    #[doc="Write the HCTSIZ3 register."]
    #[inline] pub fn set_hctsiz3<F: FnOnce(Hctsiz3) -> Hctsiz3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz3_mut(), f(Hctsiz3(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ3 register."]
    #[inline] pub fn with_hctsiz3<F: FnOnce(Hctsiz3) -> Hctsiz3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz3_mut(), f(self.hctsiz3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ4 register."]
    #[inline] pub fn hctsiz4_mut(&self) -> *mut Hctsiz4 { 
        (self.0 + 0x190) as *mut Hctsiz4
    }

    #[doc="Get the *const pointer for the HCTSIZ4 register."]
    #[inline] pub fn hctsiz4_ptr(&self) -> *const Hctsiz4 { 
           self.hctsiz4_mut()
    }

    #[doc="Read the HCTSIZ4 register."]
    #[inline] pub fn hctsiz4(&self) -> Hctsiz4 { 
        unsafe {
            read_volatile(self.hctsiz4_ptr())
        }
    }

    #[doc="Write the HCTSIZ4 register."]
    #[inline] pub fn set_hctsiz4<F: FnOnce(Hctsiz4) -> Hctsiz4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz4_mut(), f(Hctsiz4(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ4 register."]
    #[inline] pub fn with_hctsiz4<F: FnOnce(Hctsiz4) -> Hctsiz4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz4_mut(), f(self.hctsiz4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ5 register."]
    #[inline] pub fn hctsiz5_mut(&self) -> *mut Hctsiz5 { 
        (self.0 + 0x1b0) as *mut Hctsiz5
    }

    #[doc="Get the *const pointer for the HCTSIZ5 register."]
    #[inline] pub fn hctsiz5_ptr(&self) -> *const Hctsiz5 { 
           self.hctsiz5_mut()
    }

    #[doc="Read the HCTSIZ5 register."]
    #[inline] pub fn hctsiz5(&self) -> Hctsiz5 { 
        unsafe {
            read_volatile(self.hctsiz5_ptr())
        }
    }

    #[doc="Write the HCTSIZ5 register."]
    #[inline] pub fn set_hctsiz5<F: FnOnce(Hctsiz5) -> Hctsiz5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz5_mut(), f(Hctsiz5(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ5 register."]
    #[inline] pub fn with_hctsiz5<F: FnOnce(Hctsiz5) -> Hctsiz5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz5_mut(), f(self.hctsiz5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ6 register."]
    #[inline] pub fn hctsiz6_mut(&self) -> *mut Hctsiz6 { 
        (self.0 + 0x1d0) as *mut Hctsiz6
    }

    #[doc="Get the *const pointer for the HCTSIZ6 register."]
    #[inline] pub fn hctsiz6_ptr(&self) -> *const Hctsiz6 { 
           self.hctsiz6_mut()
    }

    #[doc="Read the HCTSIZ6 register."]
    #[inline] pub fn hctsiz6(&self) -> Hctsiz6 { 
        unsafe {
            read_volatile(self.hctsiz6_ptr())
        }
    }

    #[doc="Write the HCTSIZ6 register."]
    #[inline] pub fn set_hctsiz6<F: FnOnce(Hctsiz6) -> Hctsiz6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz6_mut(), f(Hctsiz6(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ6 register."]
    #[inline] pub fn with_hctsiz6<F: FnOnce(Hctsiz6) -> Hctsiz6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz6_mut(), f(self.hctsiz6()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ7 register."]
    #[inline] pub fn hctsiz7_mut(&self) -> *mut Hctsiz7 { 
        (self.0 + 0x1f0) as *mut Hctsiz7
    }

    #[doc="Get the *const pointer for the HCTSIZ7 register."]
    #[inline] pub fn hctsiz7_ptr(&self) -> *const Hctsiz7 { 
           self.hctsiz7_mut()
    }

    #[doc="Read the HCTSIZ7 register."]
    #[inline] pub fn hctsiz7(&self) -> Hctsiz7 { 
        unsafe {
            read_volatile(self.hctsiz7_ptr())
        }
    }

    #[doc="Write the HCTSIZ7 register."]
    #[inline] pub fn set_hctsiz7<F: FnOnce(Hctsiz7) -> Hctsiz7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz7_mut(), f(Hctsiz7(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ7 register."]
    #[inline] pub fn with_hctsiz7<F: FnOnce(Hctsiz7) -> Hctsiz7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz7_mut(), f(self.hctsiz7()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR8 register."]
    #[inline] pub fn hcchar8_mut(&self) -> *mut Hcchar8 { 
        (self.0 + 0x1f4) as *mut Hcchar8
    }

    #[doc="Get the *const pointer for the HCCHAR8 register."]
    #[inline] pub fn hcchar8_ptr(&self) -> *const Hcchar8 { 
           self.hcchar8_mut()
    }

    #[doc="Read the HCCHAR8 register."]
    #[inline] pub fn hcchar8(&self) -> Hcchar8 { 
        unsafe {
            read_volatile(self.hcchar8_ptr())
        }
    }

    #[doc="Write the HCCHAR8 register."]
    #[inline] pub fn set_hcchar8<F: FnOnce(Hcchar8) -> Hcchar8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar8_mut(), f(Hcchar8(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR8 register."]
    #[inline] pub fn with_hcchar8<F: FnOnce(Hcchar8) -> Hcchar8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar8_mut(), f(self.hcchar8()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT8 register."]
    #[inline] pub fn hcint8_mut(&self) -> *mut Hcint8 { 
        (self.0 + 0x1f8) as *mut Hcint8
    }

    #[doc="Get the *const pointer for the HCINT8 register."]
    #[inline] pub fn hcint8_ptr(&self) -> *const Hcint8 { 
           self.hcint8_mut()
    }

    #[doc="Read the HCINT8 register."]
    #[inline] pub fn hcint8(&self) -> Hcint8 { 
        unsafe {
            read_volatile(self.hcint8_ptr())
        }
    }

    #[doc="Write the HCINT8 register."]
    #[inline] pub fn set_hcint8<F: FnOnce(Hcint8) -> Hcint8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint8_mut(), f(Hcint8(0)));
        }
        self
    }

    #[doc="Modify the HCINT8 register."]
    #[inline] pub fn with_hcint8<F: FnOnce(Hcint8) -> Hcint8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint8_mut(), f(self.hcint8()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK8 register."]
    #[inline] pub fn hcintmsk8_mut(&self) -> *mut Hcintmsk8 { 
        (self.0 + 0x1fc) as *mut Hcintmsk8
    }

    #[doc="Get the *const pointer for the HCINTMSK8 register."]
    #[inline] pub fn hcintmsk8_ptr(&self) -> *const Hcintmsk8 { 
           self.hcintmsk8_mut()
    }

    #[doc="Read the HCINTMSK8 register."]
    #[inline] pub fn hcintmsk8(&self) -> Hcintmsk8 { 
        unsafe {
            read_volatile(self.hcintmsk8_ptr())
        }
    }

    #[doc="Write the HCINTMSK8 register."]
    #[inline] pub fn set_hcintmsk8<F: FnOnce(Hcintmsk8) -> Hcintmsk8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk8_mut(), f(Hcintmsk8(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK8 register."]
    #[inline] pub fn with_hcintmsk8<F: FnOnce(Hcintmsk8) -> Hcintmsk8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk8_mut(), f(self.hcintmsk8()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ8 register."]
    #[inline] pub fn hctsiz8_mut(&self) -> *mut Hctsiz8 { 
        (self.0 + 0x200) as *mut Hctsiz8
    }

    #[doc="Get the *const pointer for the HCTSIZ8 register."]
    #[inline] pub fn hctsiz8_ptr(&self) -> *const Hctsiz8 { 
           self.hctsiz8_mut()
    }

    #[doc="Read the HCTSIZ8 register."]
    #[inline] pub fn hctsiz8(&self) -> Hctsiz8 { 
        unsafe {
            read_volatile(self.hctsiz8_ptr())
        }
    }

    #[doc="Write the HCTSIZ8 register."]
    #[inline] pub fn set_hctsiz8<F: FnOnce(Hctsiz8) -> Hctsiz8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz8_mut(), f(Hctsiz8(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ8 register."]
    #[inline] pub fn with_hctsiz8<F: FnOnce(Hctsiz8) -> Hctsiz8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz8_mut(), f(self.hctsiz8()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR9 register."]
    #[inline] pub fn hcchar9_mut(&self) -> *mut Hcchar9 { 
        (self.0 + 0x204) as *mut Hcchar9
    }

    #[doc="Get the *const pointer for the HCCHAR9 register."]
    #[inline] pub fn hcchar9_ptr(&self) -> *const Hcchar9 { 
           self.hcchar9_mut()
    }

    #[doc="Read the HCCHAR9 register."]
    #[inline] pub fn hcchar9(&self) -> Hcchar9 { 
        unsafe {
            read_volatile(self.hcchar9_ptr())
        }
    }

    #[doc="Write the HCCHAR9 register."]
    #[inline] pub fn set_hcchar9<F: FnOnce(Hcchar9) -> Hcchar9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar9_mut(), f(Hcchar9(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR9 register."]
    #[inline] pub fn with_hcchar9<F: FnOnce(Hcchar9) -> Hcchar9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar9_mut(), f(self.hcchar9()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT9 register."]
    #[inline] pub fn hcint9_mut(&self) -> *mut Hcint9 { 
        (self.0 + 0x208) as *mut Hcint9
    }

    #[doc="Get the *const pointer for the HCINT9 register."]
    #[inline] pub fn hcint9_ptr(&self) -> *const Hcint9 { 
           self.hcint9_mut()
    }

    #[doc="Read the HCINT9 register."]
    #[inline] pub fn hcint9(&self) -> Hcint9 { 
        unsafe {
            read_volatile(self.hcint9_ptr())
        }
    }

    #[doc="Write the HCINT9 register."]
    #[inline] pub fn set_hcint9<F: FnOnce(Hcint9) -> Hcint9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint9_mut(), f(Hcint9(0)));
        }
        self
    }

    #[doc="Modify the HCINT9 register."]
    #[inline] pub fn with_hcint9<F: FnOnce(Hcint9) -> Hcint9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint9_mut(), f(self.hcint9()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK9 register."]
    #[inline] pub fn hcintmsk9_mut(&self) -> *mut Hcintmsk9 { 
        (self.0 + 0x20c) as *mut Hcintmsk9
    }

    #[doc="Get the *const pointer for the HCINTMSK9 register."]
    #[inline] pub fn hcintmsk9_ptr(&self) -> *const Hcintmsk9 { 
           self.hcintmsk9_mut()
    }

    #[doc="Read the HCINTMSK9 register."]
    #[inline] pub fn hcintmsk9(&self) -> Hcintmsk9 { 
        unsafe {
            read_volatile(self.hcintmsk9_ptr())
        }
    }

    #[doc="Write the HCINTMSK9 register."]
    #[inline] pub fn set_hcintmsk9<F: FnOnce(Hcintmsk9) -> Hcintmsk9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk9_mut(), f(Hcintmsk9(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK9 register."]
    #[inline] pub fn with_hcintmsk9<F: FnOnce(Hcintmsk9) -> Hcintmsk9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk9_mut(), f(self.hcintmsk9()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ9 register."]
    #[inline] pub fn hctsiz9_mut(&self) -> *mut Hctsiz9 { 
        (self.0 + 0x210) as *mut Hctsiz9
    }

    #[doc="Get the *const pointer for the HCTSIZ9 register."]
    #[inline] pub fn hctsiz9_ptr(&self) -> *const Hctsiz9 { 
           self.hctsiz9_mut()
    }

    #[doc="Read the HCTSIZ9 register."]
    #[inline] pub fn hctsiz9(&self) -> Hctsiz9 { 
        unsafe {
            read_volatile(self.hctsiz9_ptr())
        }
    }

    #[doc="Write the HCTSIZ9 register."]
    #[inline] pub fn set_hctsiz9<F: FnOnce(Hctsiz9) -> Hctsiz9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz9_mut(), f(Hctsiz9(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ9 register."]
    #[inline] pub fn with_hctsiz9<F: FnOnce(Hctsiz9) -> Hctsiz9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz9_mut(), f(self.hctsiz9()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR10 register."]
    #[inline] pub fn hcchar10_mut(&self) -> *mut Hcchar10 { 
        (self.0 + 0x214) as *mut Hcchar10
    }

    #[doc="Get the *const pointer for the HCCHAR10 register."]
    #[inline] pub fn hcchar10_ptr(&self) -> *const Hcchar10 { 
           self.hcchar10_mut()
    }

    #[doc="Read the HCCHAR10 register."]
    #[inline] pub fn hcchar10(&self) -> Hcchar10 { 
        unsafe {
            read_volatile(self.hcchar10_ptr())
        }
    }

    #[doc="Write the HCCHAR10 register."]
    #[inline] pub fn set_hcchar10<F: FnOnce(Hcchar10) -> Hcchar10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar10_mut(), f(Hcchar10(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR10 register."]
    #[inline] pub fn with_hcchar10<F: FnOnce(Hcchar10) -> Hcchar10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar10_mut(), f(self.hcchar10()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT10 register."]
    #[inline] pub fn hcint10_mut(&self) -> *mut Hcint10 { 
        (self.0 + 0x218) as *mut Hcint10
    }

    #[doc="Get the *const pointer for the HCINT10 register."]
    #[inline] pub fn hcint10_ptr(&self) -> *const Hcint10 { 
           self.hcint10_mut()
    }

    #[doc="Read the HCINT10 register."]
    #[inline] pub fn hcint10(&self) -> Hcint10 { 
        unsafe {
            read_volatile(self.hcint10_ptr())
        }
    }

    #[doc="Write the HCINT10 register."]
    #[inline] pub fn set_hcint10<F: FnOnce(Hcint10) -> Hcint10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint10_mut(), f(Hcint10(0)));
        }
        self
    }

    #[doc="Modify the HCINT10 register."]
    #[inline] pub fn with_hcint10<F: FnOnce(Hcint10) -> Hcint10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint10_mut(), f(self.hcint10()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK10 register."]
    #[inline] pub fn hcintmsk10_mut(&self) -> *mut Hcintmsk10 { 
        (self.0 + 0x21c) as *mut Hcintmsk10
    }

    #[doc="Get the *const pointer for the HCINTMSK10 register."]
    #[inline] pub fn hcintmsk10_ptr(&self) -> *const Hcintmsk10 { 
           self.hcintmsk10_mut()
    }

    #[doc="Read the HCINTMSK10 register."]
    #[inline] pub fn hcintmsk10(&self) -> Hcintmsk10 { 
        unsafe {
            read_volatile(self.hcintmsk10_ptr())
        }
    }

    #[doc="Write the HCINTMSK10 register."]
    #[inline] pub fn set_hcintmsk10<F: FnOnce(Hcintmsk10) -> Hcintmsk10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk10_mut(), f(Hcintmsk10(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK10 register."]
    #[inline] pub fn with_hcintmsk10<F: FnOnce(Hcintmsk10) -> Hcintmsk10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk10_mut(), f(self.hcintmsk10()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ10 register."]
    #[inline] pub fn hctsiz10_mut(&self) -> *mut Hctsiz10 { 
        (self.0 + 0x220) as *mut Hctsiz10
    }

    #[doc="Get the *const pointer for the HCTSIZ10 register."]
    #[inline] pub fn hctsiz10_ptr(&self) -> *const Hctsiz10 { 
           self.hctsiz10_mut()
    }

    #[doc="Read the HCTSIZ10 register."]
    #[inline] pub fn hctsiz10(&self) -> Hctsiz10 { 
        unsafe {
            read_volatile(self.hctsiz10_ptr())
        }
    }

    #[doc="Write the HCTSIZ10 register."]
    #[inline] pub fn set_hctsiz10<F: FnOnce(Hctsiz10) -> Hctsiz10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz10_mut(), f(Hctsiz10(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ10 register."]
    #[inline] pub fn with_hctsiz10<F: FnOnce(Hctsiz10) -> Hctsiz10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz10_mut(), f(self.hctsiz10()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCCHAR11 register."]
    #[inline] pub fn hcchar11_mut(&self) -> *mut Hcchar11 { 
        (self.0 + 0x224) as *mut Hcchar11
    }

    #[doc="Get the *const pointer for the HCCHAR11 register."]
    #[inline] pub fn hcchar11_ptr(&self) -> *const Hcchar11 { 
           self.hcchar11_mut()
    }

    #[doc="Read the HCCHAR11 register."]
    #[inline] pub fn hcchar11(&self) -> Hcchar11 { 
        unsafe {
            read_volatile(self.hcchar11_ptr())
        }
    }

    #[doc="Write the HCCHAR11 register."]
    #[inline] pub fn set_hcchar11<F: FnOnce(Hcchar11) -> Hcchar11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar11_mut(), f(Hcchar11(0)));
        }
        self
    }

    #[doc="Modify the HCCHAR11 register."]
    #[inline] pub fn with_hcchar11<F: FnOnce(Hcchar11) -> Hcchar11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcchar11_mut(), f(self.hcchar11()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINT11 register."]
    #[inline] pub fn hcint11_mut(&self) -> *mut Hcint11 { 
        (self.0 + 0x228) as *mut Hcint11
    }

    #[doc="Get the *const pointer for the HCINT11 register."]
    #[inline] pub fn hcint11_ptr(&self) -> *const Hcint11 { 
           self.hcint11_mut()
    }

    #[doc="Read the HCINT11 register."]
    #[inline] pub fn hcint11(&self) -> Hcint11 { 
        unsafe {
            read_volatile(self.hcint11_ptr())
        }
    }

    #[doc="Write the HCINT11 register."]
    #[inline] pub fn set_hcint11<F: FnOnce(Hcint11) -> Hcint11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint11_mut(), f(Hcint11(0)));
        }
        self
    }

    #[doc="Modify the HCINT11 register."]
    #[inline] pub fn with_hcint11<F: FnOnce(Hcint11) -> Hcint11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcint11_mut(), f(self.hcint11()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCINTMSK11 register."]
    #[inline] pub fn hcintmsk11_mut(&self) -> *mut Hcintmsk11 { 
        (self.0 + 0x22c) as *mut Hcintmsk11
    }

    #[doc="Get the *const pointer for the HCINTMSK11 register."]
    #[inline] pub fn hcintmsk11_ptr(&self) -> *const Hcintmsk11 { 
           self.hcintmsk11_mut()
    }

    #[doc="Read the HCINTMSK11 register."]
    #[inline] pub fn hcintmsk11(&self) -> Hcintmsk11 { 
        unsafe {
            read_volatile(self.hcintmsk11_ptr())
        }
    }

    #[doc="Write the HCINTMSK11 register."]
    #[inline] pub fn set_hcintmsk11<F: FnOnce(Hcintmsk11) -> Hcintmsk11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk11_mut(), f(Hcintmsk11(0)));
        }
        self
    }

    #[doc="Modify the HCINTMSK11 register."]
    #[inline] pub fn with_hcintmsk11<F: FnOnce(Hcintmsk11) -> Hcintmsk11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hcintmsk11_mut(), f(self.hcintmsk11()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HCTSIZ11 register."]
    #[inline] pub fn hctsiz11_mut(&self) -> *mut Hctsiz11 { 
        (self.0 + 0x230) as *mut Hctsiz11
    }

    #[doc="Get the *const pointer for the HCTSIZ11 register."]
    #[inline] pub fn hctsiz11_ptr(&self) -> *const Hctsiz11 { 
           self.hctsiz11_mut()
    }

    #[doc="Read the HCTSIZ11 register."]
    #[inline] pub fn hctsiz11(&self) -> Hctsiz11 { 
        unsafe {
            read_volatile(self.hctsiz11_ptr())
        }
    }

    #[doc="Write the HCTSIZ11 register."]
    #[inline] pub fn set_hctsiz11<F: FnOnce(Hctsiz11) -> Hctsiz11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz11_mut(), f(Hctsiz11(0)));
        }
        self
    }

    #[doc="Modify the HCTSIZ11 register."]
    #[inline] pub fn with_hctsiz11<F: FnOnce(Hctsiz11) -> Hctsiz11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hctsiz11_mut(), f(self.hctsiz11()));
        }
        self
    }

}

#[doc="OTG_FS host configuration register (OTG_FS_HCFG)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcfg(pub u32);
impl Hcfg {
    #[doc="FS/LS PHY clock select"]
    #[inline] pub fn fslspcs(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if FSLSPCS != 0"]
    #[inline] pub fn test_fslspcs(&self) -> bool {
        self.fslspcs() != 0
    }

    #[doc="Sets the FSLSPCS field."]
    #[inline] pub fn set_fslspcs<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="FS- and LS-only support"]
    #[inline] pub fn fslss(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FSLSS != 0"]
    #[inline] pub fn test_fslss(&self) -> bool {
        self.fslss() != 0
    }

    #[doc="Sets the FSLSS field."]
    #[inline] pub fn set_fslss<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Hcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Hcfg(other)
    }
}

impl ::core::fmt::Display for Hcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fslspcs() != 0 { try!(write!(f, " fslspcs=0x{:x}", self.fslspcs()))}
        if self.fslss() != 0 { try!(write!(f, " fslss"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS Host frame interval register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfir(pub u32);
impl Hfir {
    #[doc="Frame interval"]
    #[inline] pub fn frivl(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if FRIVL != 0"]
    #[inline] pub fn test_frivl(&self) -> bool {
        self.frivl() != 0
    }

    #[doc="Sets the FRIVL field."]
    #[inline] pub fn set_frivl<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hfir {
    #[inline]
    fn from(other: u32) -> Self {
         Hfir(other)
    }
}

impl ::core::fmt::Display for Hfir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frivl() != 0 { try!(write!(f, " frivl=0x{:x}", self.frivl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfnum(pub u32);
impl Hfnum {
    #[doc="Frame number"]
    #[inline] pub fn frnum(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if FRNUM != 0"]
    #[inline] pub fn test_frnum(&self) -> bool {
        self.frnum() != 0
    }

    #[doc="Sets the FRNUM field."]
    #[inline] pub fn set_frnum<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Frame time remaining"]
    #[inline] pub fn ftrem(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if FTREM != 0"]
    #[inline] pub fn test_ftrem(&self) -> bool {
        self.ftrem() != 0
    }

    #[doc="Sets the FTREM field."]
    #[inline] pub fn set_ftrem<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Hfnum {
    #[inline]
    fn from(other: u32) -> Self {
         Hfnum(other)
    }
}

impl ::core::fmt::Display for Hfnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frnum() != 0 { try!(write!(f, " frnum=0x{:x}", self.frnum()))}
        if self.ftrem() != 0 { try!(write!(f, " ftrem=0x{:x}", self.ftrem()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hptxsts(pub u32);
impl Hptxsts {
    #[doc="Periodic transmit data FIFO space available"]
    #[inline] pub fn ptxfsavl(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PTXFSAVL != 0"]
    #[inline] pub fn test_ptxfsavl(&self) -> bool {
        self.ptxfsavl() != 0
    }

    #[doc="Sets the PTXFSAVL field."]
    #[inline] pub fn set_ptxfsavl<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Periodic transmit request queue space available"]
    #[inline] pub fn ptxqsav(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if PTXQSAV != 0"]
    #[inline] pub fn test_ptxqsav(&self) -> bool {
        self.ptxqsav() != 0
    }

    #[doc="Sets the PTXQSAV field."]
    #[inline] pub fn set_ptxqsav<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Top of the periodic transmit request queue"]
    #[inline] pub fn ptxqtop(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if PTXQTOP != 0"]
    #[inline] pub fn test_ptxqtop(&self) -> bool {
        self.ptxqtop() != 0
    }

    #[doc="Sets the PTXQTOP field."]
    #[inline] pub fn set_ptxqtop<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Hptxsts {
    #[inline]
    fn from(other: u32) -> Self {
         Hptxsts(other)
    }
}

impl ::core::fmt::Display for Hptxsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hptxsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ptxfsavl() != 0 { try!(write!(f, " ptxfsavl=0x{:x}", self.ptxfsavl()))}
        if self.ptxqsav() != 0 { try!(write!(f, " ptxqsav=0x{:x}", self.ptxqsav()))}
        if self.ptxqtop() != 0 { try!(write!(f, " ptxqtop=0x{:x}", self.ptxqtop()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS Host all channels interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Haint(pub u32);
impl Haint {
    #[doc="Channel interrupts"]
    #[inline] pub fn haint(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if HAINT != 0"]
    #[inline] pub fn test_haint(&self) -> bool {
        self.haint() != 0
    }

    #[doc="Sets the HAINT field."]
    #[inline] pub fn set_haint<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Haint {
    #[inline]
    fn from(other: u32) -> Self {
         Haint(other)
    }
}

impl ::core::fmt::Display for Haint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Haint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.haint() != 0 { try!(write!(f, " haint=0x{:x}", self.haint()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host all channels interrupt mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Haintmsk(pub u32);
impl Haintmsk {
    #[doc="Channel interrupt mask"]
    #[inline] pub fn haintm(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if HAINTM != 0"]
    #[inline] pub fn test_haintm(&self) -> bool {
        self.haintm() != 0
    }

    #[doc="Sets the HAINTM field."]
    #[inline] pub fn set_haintm<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Haintmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Haintmsk(other)
    }
}

impl ::core::fmt::Display for Haintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Haintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.haintm() != 0 { try!(write!(f, " haintm=0x{:x}", self.haintm()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host port control and status register (OTG_FS_HPRT)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hprt(pub u32);
impl Hprt {
    #[doc="Port connect status"]
    #[inline] pub fn pcsts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PCSTS != 0"]
    #[inline] pub fn test_pcsts(&self) -> bool {
        self.pcsts() != 0
    }

    #[doc="Sets the PCSTS field."]
    #[inline] pub fn set_pcsts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Port connect detected"]
    #[inline] pub fn pcdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PCDET != 0"]
    #[inline] pub fn test_pcdet(&self) -> bool {
        self.pcdet() != 0
    }

    #[doc="Sets the PCDET field."]
    #[inline] pub fn set_pcdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port enable"]
    #[inline] pub fn pena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PENA != 0"]
    #[inline] pub fn test_pena(&self) -> bool {
        self.pena() != 0
    }

    #[doc="Sets the PENA field."]
    #[inline] pub fn set_pena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port enable/disable change"]
    #[inline] pub fn penchng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PENCHNG != 0"]
    #[inline] pub fn test_penchng(&self) -> bool {
        self.penchng() != 0
    }

    #[doc="Sets the PENCHNG field."]
    #[inline] pub fn set_penchng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port overcurrent active"]
    #[inline] pub fn poca(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if POCA != 0"]
    #[inline] pub fn test_poca(&self) -> bool {
        self.poca() != 0
    }

    #[doc="Sets the POCA field."]
    #[inline] pub fn set_poca<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port overcurrent change"]
    #[inline] pub fn pocchng(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if POCCHNG != 0"]
    #[inline] pub fn test_pocchng(&self) -> bool {
        self.pocchng() != 0
    }

    #[doc="Sets the POCCHNG field."]
    #[inline] pub fn set_pocchng<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port resume"]
    #[inline] pub fn pres(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PRES != 0"]
    #[inline] pub fn test_pres(&self) -> bool {
        self.pres() != 0
    }

    #[doc="Sets the PRES field."]
    #[inline] pub fn set_pres<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port suspend"]
    #[inline] pub fn psusp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PSUSP != 0"]
    #[inline] pub fn test_psusp(&self) -> bool {
        self.psusp() != 0
    }

    #[doc="Sets the PSUSP field."]
    #[inline] pub fn set_psusp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port reset"]
    #[inline] pub fn prst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PRST != 0"]
    #[inline] pub fn test_prst(&self) -> bool {
        self.prst() != 0
    }

    #[doc="Sets the PRST field."]
    #[inline] pub fn set_prst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port line status"]
    #[inline] pub fn plsts(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if PLSTS != 0"]
    #[inline] pub fn test_plsts(&self) -> bool {
        self.plsts() != 0
    }

    #[doc="Sets the PLSTS field."]
    #[inline] pub fn set_plsts<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port power"]
    #[inline] pub fn ppwr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PPWR != 0"]
    #[inline] pub fn test_ppwr(&self) -> bool {
        self.ppwr() != 0
    }

    #[doc="Sets the PPWR field."]
    #[inline] pub fn set_ppwr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port test control"]
    #[inline] pub fn ptctl(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0xf) as u8) } // [16:13]
    }

    #[doc="Returns true if PTCTL != 0"]
    #[inline] pub fn test_ptctl(&self) -> bool {
        self.ptctl() != 0
    }

    #[doc="Sets the PTCTL field."]
    #[inline] pub fn set_ptctl<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port speed"]
    #[inline] pub fn pspd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if PSPD != 0"]
    #[inline] pub fn test_pspd(&self) -> bool {
        self.pspd() != 0
    }

    #[doc="Sets the PSPD field."]
    #[inline] pub fn set_pspd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Hprt {
    #[inline]
    fn from(other: u32) -> Self {
         Hprt(other)
    }
}

impl ::core::fmt::Display for Hprt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hprt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcsts() != 0 { try!(write!(f, " pcsts"))}
        if self.pcdet() != 0 { try!(write!(f, " pcdet"))}
        if self.pena() != 0 { try!(write!(f, " pena"))}
        if self.penchng() != 0 { try!(write!(f, " penchng"))}
        if self.poca() != 0 { try!(write!(f, " poca"))}
        if self.pocchng() != 0 { try!(write!(f, " pocchng"))}
        if self.pres() != 0 { try!(write!(f, " pres"))}
        if self.psusp() != 0 { try!(write!(f, " psusp"))}
        if self.prst() != 0 { try!(write!(f, " prst"))}
        if self.plsts() != 0 { try!(write!(f, " plsts=0x{:x}", self.plsts()))}
        if self.ppwr() != 0 { try!(write!(f, " ppwr"))}
        if self.ptctl() != 0 { try!(write!(f, " ptctl=0x{:x}", self.ptctl()))}
        if self.pspd() != 0 { try!(write!(f, " pspd=0x{:x}", self.pspd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar0(pub u32);
impl Hcchar0 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar0 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar0(other)
    }
}

impl ::core::fmt::Display for Hcchar0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar1(pub u32);
impl Hcchar1 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar1 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar1(other)
    }
}

impl ::core::fmt::Display for Hcchar1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar2(pub u32);
impl Hcchar2 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar2 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar2(other)
    }
}

impl ::core::fmt::Display for Hcchar2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar3(pub u32);
impl Hcchar3 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar3 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar3(other)
    }
}

impl ::core::fmt::Display for Hcchar3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar4(pub u32);
impl Hcchar4 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar4 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar4(other)
    }
}

impl ::core::fmt::Display for Hcchar4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar5(pub u32);
impl Hcchar5 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar5 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar5(other)
    }
}

impl ::core::fmt::Display for Hcchar5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar6(pub u32);
impl Hcchar6 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar6 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar6(other)
    }
}

impl ::core::fmt::Display for Hcchar6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar7(pub u32);
impl Hcchar7 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar7 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar7(other)
    }
}

impl ::core::fmt::Display for Hcchar7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint0(pub u32);
impl Hcint0 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint0 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint0(other)
    }
}

impl ::core::fmt::Display for Hcint0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint1(pub u32);
impl Hcint1 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint1 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint1(other)
    }
}

impl ::core::fmt::Display for Hcint1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint2(pub u32);
impl Hcint2 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint2 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint2(other)
    }
}

impl ::core::fmt::Display for Hcint2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint3(pub u32);
impl Hcint3 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint3 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint3(other)
    }
}

impl ::core::fmt::Display for Hcint3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint4(pub u32);
impl Hcint4 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint4 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint4(other)
    }
}

impl ::core::fmt::Display for Hcint4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint5(pub u32);
impl Hcint5 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint5 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint5(other)
    }
}

impl ::core::fmt::Display for Hcint5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint6(pub u32);
impl Hcint6 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint6 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint6(other)
    }
}

impl ::core::fmt::Display for Hcint6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint7(pub u32);
impl Hcint7 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint7 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint7(other)
    }
}

impl ::core::fmt::Display for Hcint7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk0(pub u32);
impl Hcintmsk0 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk0 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk0(other)
    }
}

impl ::core::fmt::Display for Hcintmsk0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk1(pub u32);
impl Hcintmsk1 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk1 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk1(other)
    }
}

impl ::core::fmt::Display for Hcintmsk1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk2(pub u32);
impl Hcintmsk2 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk2 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk2(other)
    }
}

impl ::core::fmt::Display for Hcintmsk2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk3(pub u32);
impl Hcintmsk3 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk3 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk3(other)
    }
}

impl ::core::fmt::Display for Hcintmsk3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk4(pub u32);
impl Hcintmsk4 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk4 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk4(other)
    }
}

impl ::core::fmt::Display for Hcintmsk4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk5(pub u32);
impl Hcintmsk5 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk5 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk5(other)
    }
}

impl ::core::fmt::Display for Hcintmsk5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk6(pub u32);
impl Hcintmsk6 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk6 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk6(other)
    }
}

impl ::core::fmt::Display for Hcintmsk6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk7(pub u32);
impl Hcintmsk7 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk7 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk7(other)
    }
}

impl ::core::fmt::Display for Hcintmsk7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-0 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz0(pub u32);
impl Hctsiz0 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz0 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz0(other)
    }
}

impl ::core::fmt::Display for Hctsiz0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-1 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz1(pub u32);
impl Hctsiz1 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz1 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz1(other)
    }
}

impl ::core::fmt::Display for Hctsiz1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-2 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz2(pub u32);
impl Hctsiz2 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz2 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz2(other)
    }
}

impl ::core::fmt::Display for Hctsiz2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-3 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz3(pub u32);
impl Hctsiz3 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz3 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz3(other)
    }
}

impl ::core::fmt::Display for Hctsiz3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-x transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz4(pub u32);
impl Hctsiz4 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz4 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz4(other)
    }
}

impl ::core::fmt::Display for Hctsiz4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-5 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz5(pub u32);
impl Hctsiz5 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz5 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz5(other)
    }
}

impl ::core::fmt::Display for Hctsiz5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-6 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz6(pub u32);
impl Hctsiz6 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz6 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz6(other)
    }
}

impl ::core::fmt::Display for Hctsiz6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-7 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz7(pub u32);
impl Hctsiz7 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz7 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz7(other)
    }
}

impl ::core::fmt::Display for Hctsiz7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-8 characteristics register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar8(pub u32);
impl Hcchar8 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar8 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar8(other)
    }
}

impl ::core::fmt::Display for Hcchar8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-8 interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint8(pub u32);
impl Hcint8 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint8 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint8(other)
    }
}

impl ::core::fmt::Display for Hcint8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-8 mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk8(pub u32);
impl Hcintmsk8 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk8 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk8(other)
    }
}

impl ::core::fmt::Display for Hcintmsk8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-8 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz8(pub u32);
impl Hctsiz8 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz8 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz8(other)
    }
}

impl ::core::fmt::Display for Hctsiz8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-9 characteristics register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar9(pub u32);
impl Hcchar9 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar9 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar9(other)
    }
}

impl ::core::fmt::Display for Hcchar9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-9 interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint9(pub u32);
impl Hcint9 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint9 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint9(other)
    }
}

impl ::core::fmt::Display for Hcint9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-9 mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk9(pub u32);
impl Hcintmsk9 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk9 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk9(other)
    }
}

impl ::core::fmt::Display for Hcintmsk9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-9 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz9(pub u32);
impl Hctsiz9 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz9 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz9(other)
    }
}

impl ::core::fmt::Display for Hctsiz9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-10 characteristics register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar10(pub u32);
impl Hcchar10 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar10 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar10(other)
    }
}

impl ::core::fmt::Display for Hcchar10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-10 interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint10(pub u32);
impl Hcint10 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint10 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint10(other)
    }
}

impl ::core::fmt::Display for Hcint10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-10 mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk10(pub u32);
impl Hcintmsk10 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk10 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk10(other)
    }
}

impl ::core::fmt::Display for Hcintmsk10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-10 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz10(pub u32);
impl Hctsiz10 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz10 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz10(other)
    }
}

impl ::core::fmt::Display for Hctsiz10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-11 characteristics register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcchar11(pub u32);
impl Hcchar11 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Endpoint direction"]
    #[inline] pub fn epdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EPDIR != 0"]
    #[inline] pub fn test_epdir(&self) -> bool {
        self.epdir() != 0
    }

    #[doc="Sets the EPDIR field."]
    #[inline] pub fn set_epdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Low-speed device"]
    #[inline] pub fn lsdev(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSDEV != 0"]
    #[inline] pub fn test_lsdev(&self) -> bool {
        self.lsdev() != 0
    }

    #[doc="Sets the LSDEV field."]
    #[inline] pub fn set_lsdev<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Multicount"]
    #[inline] pub fn mcnt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7f) as u8) } // [28:22]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Odd frame"]
    #[inline] pub fn oddfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ODDFRM != 0"]
    #[inline] pub fn test_oddfrm(&self) -> bool {
        self.oddfrm() != 0
    }

    #[doc="Sets the ODDFRM field."]
    #[inline] pub fn set_oddfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel disable"]
    #[inline] pub fn chdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHDIS != 0"]
    #[inline] pub fn test_chdis(&self) -> bool {
        self.chdis() != 0
    }

    #[doc="Sets the CHDIS field."]
    #[inline] pub fn set_chdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel enable"]
    #[inline] pub fn chena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHENA != 0"]
    #[inline] pub fn test_chena(&self) -> bool {
        self.chena() != 0
    }

    #[doc="Sets the CHENA field."]
    #[inline] pub fn set_chena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Hcchar11 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcchar11(other)
    }
}

impl ::core::fmt::Display for Hcchar11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcchar11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.epdir() != 0 { try!(write!(f, " epdir"))}
        if self.lsdev() != 0 { try!(write!(f, " lsdev"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.oddfrm() != 0 { try!(write!(f, " oddfrm"))}
        if self.chdis() != 0 { try!(write!(f, " chdis"))}
        if self.chena() != 0 { try!(write!(f, " chena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-11 interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcint11(pub u32);
impl Hcint11 {
    #[doc="Transfer completed"]
    #[inline] pub fn xfrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted"]
    #[inline] pub fn chh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHH != 0"]
    #[inline] pub fn test_chh(&self) -> bool {
        self.chh() != 0
    }

    #[doc="Sets the CHH field."]
    #[inline] pub fn set_chh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt"]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transaction error"]
    #[inline] pub fn txerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error"]
    #[inline] pub fn bberr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERR != 0"]
    #[inline] pub fn test_bberr(&self) -> bool {
        self.bberr() != 0
    }

    #[doc="Sets the BBERR field."]
    #[inline] pub fn set_bberr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun"]
    #[inline] pub fn frmor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMOR != 0"]
    #[inline] pub fn test_frmor(&self) -> bool {
        self.frmor() != 0
    }

    #[doc="Sets the FRMOR field."]
    #[inline] pub fn set_frmor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error"]
    #[inline] pub fn dterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERR != 0"]
    #[inline] pub fn test_dterr(&self) -> bool {
        self.dterr() != 0
    }

    #[doc="Sets the DTERR field."]
    #[inline] pub fn set_dterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcint11 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcint11(other)
    }
}

impl ::core::fmt::Display for Hcint11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcint11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        if self.chh() != 0 { try!(write!(f, " chh"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.bberr() != 0 { try!(write!(f, " bberr"))}
        if self.frmor() != 0 { try!(write!(f, " frmor"))}
        if self.dterr() != 0 { try!(write!(f, " dterr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-11 mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hcintmsk11(pub u32);
impl Hcintmsk11 {
    #[doc="Transfer completed mask"]
    #[inline] pub fn xfrcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel halted mask"]
    #[inline] pub fn chhm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHHM != 0"]
    #[inline] pub fn test_chhm(&self) -> bool {
        self.chhm() != 0
    }

    #[doc="Sets the CHHM field."]
    #[inline] pub fn set_chhm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="STALL response received interrupt mask"]
    #[inline] pub fn stallm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STALLM != 0"]
    #[inline] pub fn test_stallm(&self) -> bool {
        self.stallm() != 0
    }

    #[doc="Sets the STALLM field."]
    #[inline] pub fn set_stallm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="NAK response received interrupt mask"]
    #[inline] pub fn nakm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAKM != 0"]
    #[inline] pub fn test_nakm(&self) -> bool {
        self.nakm() != 0
    }

    #[doc="Sets the NAKM field."]
    #[inline] pub fn set_nakm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACK response received/transmitted interrupt mask"]
    #[inline] pub fn ackm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACKM != 0"]
    #[inline] pub fn test_ackm(&self) -> bool {
        self.ackm() != 0
    }

    #[doc="Sets the ACKM field."]
    #[inline] pub fn set_ackm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="response received interrupt mask"]
    #[inline] pub fn nyet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NYET != 0"]
    #[inline] pub fn test_nyet(&self) -> bool {
        self.nyet() != 0
    }

    #[doc="Sets the NYET field."]
    #[inline] pub fn set_nyet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transaction error mask"]
    #[inline] pub fn txerrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXERRM != 0"]
    #[inline] pub fn test_txerrm(&self) -> bool {
        self.txerrm() != 0
    }

    #[doc="Sets the TXERRM field."]
    #[inline] pub fn set_txerrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Babble error mask"]
    #[inline] pub fn bberrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BBERRM != 0"]
    #[inline] pub fn test_bberrm(&self) -> bool {
        self.bberrm() != 0
    }

    #[doc="Sets the BBERRM field."]
    #[inline] pub fn set_bberrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame overrun mask"]
    #[inline] pub fn frmorm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FRMORM != 0"]
    #[inline] pub fn test_frmorm(&self) -> bool {
        self.frmorm() != 0
    }

    #[doc="Sets the FRMORM field."]
    #[inline] pub fn set_frmorm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data toggle error mask"]
    #[inline] pub fn dterrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTERRM != 0"]
    #[inline] pub fn test_dterrm(&self) -> bool {
        self.dterrm() != 0
    }

    #[doc="Sets the DTERRM field."]
    #[inline] pub fn set_dterrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Hcintmsk11 {
    #[inline]
    fn from(other: u32) -> Self {
         Hcintmsk11(other)
    }
}

impl ::core::fmt::Display for Hcintmsk11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hcintmsk11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.chhm() != 0 { try!(write!(f, " chhm"))}
        if self.stallm() != 0 { try!(write!(f, " stallm"))}
        if self.nakm() != 0 { try!(write!(f, " nakm"))}
        if self.ackm() != 0 { try!(write!(f, " ackm"))}
        if self.nyet() != 0 { try!(write!(f, " nyet"))}
        if self.txerrm() != 0 { try!(write!(f, " txerrm"))}
        if self.bberrm() != 0 { try!(write!(f, " bberrm"))}
        if self.frmorm() != 0 { try!(write!(f, " frmorm"))}
        if self.dterrm() != 0 { try!(write!(f, " dterrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS host channel-11 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hctsiz11(pub u32);
impl Hctsiz11 {
    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<bits::U19>>(mut self, value: V) -> Self {
        let value: bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Hctsiz11 {
    #[inline]
    fn from(other: u32) -> Self {
         Hctsiz11(other)
    }
}

impl ::core::fmt::Display for Hctsiz11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hctsiz11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


