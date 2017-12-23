#[allow(unused_imports)] use bobbin_common::*;

periph!( DFSDM, Dfsdm, _DFSDM, DfsdmPeriph, 0x40016000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DFSDM Peripheral"]
pub struct DfsdmPeriph(pub usize); 



impl DfsdmPeriph {
    #[doc="Get the *mut pointer for the CHCFG0R1 register."]
    #[inline] pub fn chcfg0r1_mut(&self) -> *mut Chcfg0r1 { 
        (self.0 + 0x0) as *mut Chcfg0r1
    }

    #[doc="Get the *const pointer for the CHCFG0R1 register."]
    #[inline] pub fn chcfg0r1_ptr(&self) -> *const Chcfg0r1 { 
           self.chcfg0r1_mut()
    }

    #[doc="Read the CHCFG0R1 register."]
    #[inline] pub fn chcfg0r1(&self) -> Chcfg0r1 { 
        unsafe {
            read_volatile(self.chcfg0r1_ptr())
        }
    }

    #[doc="Write the CHCFG0R1 register."]
    #[inline] pub fn set_chcfg0r1<F: FnOnce(Chcfg0r1) -> Chcfg0r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg0r1_mut(), f(Chcfg0r1(0)));
        }
        self
    }

    #[doc="Modify the CHCFG0R1 register."]
    #[inline] pub fn with_chcfg0r1<F: FnOnce(Chcfg0r1) -> Chcfg0r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg0r1_mut(), f(self.chcfg0r1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG0R2 register."]
    #[inline] pub fn chcfg0r2_mut(&self) -> *mut Chcfg0r2 { 
        (self.0 + 0x4) as *mut Chcfg0r2
    }

    #[doc="Get the *const pointer for the CHCFG0R2 register."]
    #[inline] pub fn chcfg0r2_ptr(&self) -> *const Chcfg0r2 { 
           self.chcfg0r2_mut()
    }

    #[doc="Read the CHCFG0R2 register."]
    #[inline] pub fn chcfg0r2(&self) -> Chcfg0r2 { 
        unsafe {
            read_volatile(self.chcfg0r2_ptr())
        }
    }

    #[doc="Write the CHCFG0R2 register."]
    #[inline] pub fn set_chcfg0r2<F: FnOnce(Chcfg0r2) -> Chcfg0r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg0r2_mut(), f(Chcfg0r2(0)));
        }
        self
    }

    #[doc="Modify the CHCFG0R2 register."]
    #[inline] pub fn with_chcfg0r2<F: FnOnce(Chcfg0r2) -> Chcfg0r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg0r2_mut(), f(self.chcfg0r2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AWSCD0R register."]
    #[inline] pub fn awscd0r_mut(&self) -> *mut Awscd0r { 
        (self.0 + 0x8) as *mut Awscd0r
    }

    #[doc="Get the *const pointer for the AWSCD0R register."]
    #[inline] pub fn awscd0r_ptr(&self) -> *const Awscd0r { 
           self.awscd0r_mut()
    }

    #[doc="Read the AWSCD0R register."]
    #[inline] pub fn awscd0r(&self) -> Awscd0r { 
        unsafe {
            read_volatile(self.awscd0r_ptr())
        }
    }

    #[doc="Write the AWSCD0R register."]
    #[inline] pub fn set_awscd0r<F: FnOnce(Awscd0r) -> Awscd0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd0r_mut(), f(Awscd0r(0)));
        }
        self
    }

    #[doc="Modify the AWSCD0R register."]
    #[inline] pub fn with_awscd0r<F: FnOnce(Awscd0r) -> Awscd0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd0r_mut(), f(self.awscd0r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHWDAT0R register."]
    #[inline] pub fn chwdat0r_mut(&self) -> *mut Chwdat0r { 
        (self.0 + 0xc) as *mut Chwdat0r
    }

    #[doc="Get the *const pointer for the CHWDAT0R register."]
    #[inline] pub fn chwdat0r_ptr(&self) -> *const Chwdat0r { 
           self.chwdat0r_mut()
    }

    #[doc="Read the CHWDAT0R register."]
    #[inline] pub fn chwdat0r(&self) -> Chwdat0r { 
        unsafe {
            read_volatile(self.chwdat0r_ptr())
        }
    }

    #[doc="Write the CHWDAT0R register."]
    #[inline] pub fn set_chwdat0r<F: FnOnce(Chwdat0r) -> Chwdat0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat0r_mut(), f(Chwdat0r(0)));
        }
        self
    }

    #[doc="Modify the CHWDAT0R register."]
    #[inline] pub fn with_chwdat0r<F: FnOnce(Chwdat0r) -> Chwdat0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat0r_mut(), f(self.chwdat0r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHDATIN0R register."]
    #[inline] pub fn chdatin0r_mut(&self) -> *mut Chdatin0r { 
        (self.0 + 0x10) as *mut Chdatin0r
    }

    #[doc="Get the *const pointer for the CHDATIN0R register."]
    #[inline] pub fn chdatin0r_ptr(&self) -> *const Chdatin0r { 
           self.chdatin0r_mut()
    }

    #[doc="Read the CHDATIN0R register."]
    #[inline] pub fn chdatin0r(&self) -> Chdatin0r { 
        unsafe {
            read_volatile(self.chdatin0r_ptr())
        }
    }

    #[doc="Write the CHDATIN0R register."]
    #[inline] pub fn set_chdatin0r<F: FnOnce(Chdatin0r) -> Chdatin0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin0r_mut(), f(Chdatin0r(0)));
        }
        self
    }

    #[doc="Modify the CHDATIN0R register."]
    #[inline] pub fn with_chdatin0r<F: FnOnce(Chdatin0r) -> Chdatin0r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin0r_mut(), f(self.chdatin0r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG1R1 register."]
    #[inline] pub fn chcfg1r1_mut(&self) -> *mut Chcfg1r1 { 
        (self.0 + 0x20) as *mut Chcfg1r1
    }

    #[doc="Get the *const pointer for the CHCFG1R1 register."]
    #[inline] pub fn chcfg1r1_ptr(&self) -> *const Chcfg1r1 { 
           self.chcfg1r1_mut()
    }

    #[doc="Read the CHCFG1R1 register."]
    #[inline] pub fn chcfg1r1(&self) -> Chcfg1r1 { 
        unsafe {
            read_volatile(self.chcfg1r1_ptr())
        }
    }

    #[doc="Write the CHCFG1R1 register."]
    #[inline] pub fn set_chcfg1r1<F: FnOnce(Chcfg1r1) -> Chcfg1r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg1r1_mut(), f(Chcfg1r1(0)));
        }
        self
    }

    #[doc="Modify the CHCFG1R1 register."]
    #[inline] pub fn with_chcfg1r1<F: FnOnce(Chcfg1r1) -> Chcfg1r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg1r1_mut(), f(self.chcfg1r1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG1R2 register."]
    #[inline] pub fn chcfg1r2_mut(&self) -> *mut Chcfg1r2 { 
        (self.0 + 0x24) as *mut Chcfg1r2
    }

    #[doc="Get the *const pointer for the CHCFG1R2 register."]
    #[inline] pub fn chcfg1r2_ptr(&self) -> *const Chcfg1r2 { 
           self.chcfg1r2_mut()
    }

    #[doc="Read the CHCFG1R2 register."]
    #[inline] pub fn chcfg1r2(&self) -> Chcfg1r2 { 
        unsafe {
            read_volatile(self.chcfg1r2_ptr())
        }
    }

    #[doc="Write the CHCFG1R2 register."]
    #[inline] pub fn set_chcfg1r2<F: FnOnce(Chcfg1r2) -> Chcfg1r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg1r2_mut(), f(Chcfg1r2(0)));
        }
        self
    }

    #[doc="Modify the CHCFG1R2 register."]
    #[inline] pub fn with_chcfg1r2<F: FnOnce(Chcfg1r2) -> Chcfg1r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg1r2_mut(), f(self.chcfg1r2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AWSCD1R register."]
    #[inline] pub fn awscd1r_mut(&self) -> *mut Awscd1r { 
        (self.0 + 0x28) as *mut Awscd1r
    }

    #[doc="Get the *const pointer for the AWSCD1R register."]
    #[inline] pub fn awscd1r_ptr(&self) -> *const Awscd1r { 
           self.awscd1r_mut()
    }

    #[doc="Read the AWSCD1R register."]
    #[inline] pub fn awscd1r(&self) -> Awscd1r { 
        unsafe {
            read_volatile(self.awscd1r_ptr())
        }
    }

    #[doc="Write the AWSCD1R register."]
    #[inline] pub fn set_awscd1r<F: FnOnce(Awscd1r) -> Awscd1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd1r_mut(), f(Awscd1r(0)));
        }
        self
    }

    #[doc="Modify the AWSCD1R register."]
    #[inline] pub fn with_awscd1r<F: FnOnce(Awscd1r) -> Awscd1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd1r_mut(), f(self.awscd1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHWDAT1R register."]
    #[inline] pub fn chwdat1r_mut(&self) -> *mut Chwdat1r { 
        (self.0 + 0x2c) as *mut Chwdat1r
    }

    #[doc="Get the *const pointer for the CHWDAT1R register."]
    #[inline] pub fn chwdat1r_ptr(&self) -> *const Chwdat1r { 
           self.chwdat1r_mut()
    }

    #[doc="Read the CHWDAT1R register."]
    #[inline] pub fn chwdat1r(&self) -> Chwdat1r { 
        unsafe {
            read_volatile(self.chwdat1r_ptr())
        }
    }

    #[doc="Write the CHWDAT1R register."]
    #[inline] pub fn set_chwdat1r<F: FnOnce(Chwdat1r) -> Chwdat1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat1r_mut(), f(Chwdat1r(0)));
        }
        self
    }

    #[doc="Modify the CHWDAT1R register."]
    #[inline] pub fn with_chwdat1r<F: FnOnce(Chwdat1r) -> Chwdat1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat1r_mut(), f(self.chwdat1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHDATIN1R register."]
    #[inline] pub fn chdatin1r_mut(&self) -> *mut Chdatin1r { 
        (self.0 + 0x30) as *mut Chdatin1r
    }

    #[doc="Get the *const pointer for the CHDATIN1R register."]
    #[inline] pub fn chdatin1r_ptr(&self) -> *const Chdatin1r { 
           self.chdatin1r_mut()
    }

    #[doc="Read the CHDATIN1R register."]
    #[inline] pub fn chdatin1r(&self) -> Chdatin1r { 
        unsafe {
            read_volatile(self.chdatin1r_ptr())
        }
    }

    #[doc="Write the CHDATIN1R register."]
    #[inline] pub fn set_chdatin1r<F: FnOnce(Chdatin1r) -> Chdatin1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin1r_mut(), f(Chdatin1r(0)));
        }
        self
    }

    #[doc="Modify the CHDATIN1R register."]
    #[inline] pub fn with_chdatin1r<F: FnOnce(Chdatin1r) -> Chdatin1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin1r_mut(), f(self.chdatin1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG2R1 register."]
    #[inline] pub fn chcfg2r1_mut(&self) -> *mut Chcfg2r1 { 
        (self.0 + 0x40) as *mut Chcfg2r1
    }

    #[doc="Get the *const pointer for the CHCFG2R1 register."]
    #[inline] pub fn chcfg2r1_ptr(&self) -> *const Chcfg2r1 { 
           self.chcfg2r1_mut()
    }

    #[doc="Read the CHCFG2R1 register."]
    #[inline] pub fn chcfg2r1(&self) -> Chcfg2r1 { 
        unsafe {
            read_volatile(self.chcfg2r1_ptr())
        }
    }

    #[doc="Write the CHCFG2R1 register."]
    #[inline] pub fn set_chcfg2r1<F: FnOnce(Chcfg2r1) -> Chcfg2r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg2r1_mut(), f(Chcfg2r1(0)));
        }
        self
    }

    #[doc="Modify the CHCFG2R1 register."]
    #[inline] pub fn with_chcfg2r1<F: FnOnce(Chcfg2r1) -> Chcfg2r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg2r1_mut(), f(self.chcfg2r1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG2R2 register."]
    #[inline] pub fn chcfg2r2_mut(&self) -> *mut Chcfg2r2 { 
        (self.0 + 0x44) as *mut Chcfg2r2
    }

    #[doc="Get the *const pointer for the CHCFG2R2 register."]
    #[inline] pub fn chcfg2r2_ptr(&self) -> *const Chcfg2r2 { 
           self.chcfg2r2_mut()
    }

    #[doc="Read the CHCFG2R2 register."]
    #[inline] pub fn chcfg2r2(&self) -> Chcfg2r2 { 
        unsafe {
            read_volatile(self.chcfg2r2_ptr())
        }
    }

    #[doc="Write the CHCFG2R2 register."]
    #[inline] pub fn set_chcfg2r2<F: FnOnce(Chcfg2r2) -> Chcfg2r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg2r2_mut(), f(Chcfg2r2(0)));
        }
        self
    }

    #[doc="Modify the CHCFG2R2 register."]
    #[inline] pub fn with_chcfg2r2<F: FnOnce(Chcfg2r2) -> Chcfg2r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg2r2_mut(), f(self.chcfg2r2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AWSCD2R register."]
    #[inline] pub fn awscd2r_mut(&self) -> *mut Awscd2r { 
        (self.0 + 0x48) as *mut Awscd2r
    }

    #[doc="Get the *const pointer for the AWSCD2R register."]
    #[inline] pub fn awscd2r_ptr(&self) -> *const Awscd2r { 
           self.awscd2r_mut()
    }

    #[doc="Read the AWSCD2R register."]
    #[inline] pub fn awscd2r(&self) -> Awscd2r { 
        unsafe {
            read_volatile(self.awscd2r_ptr())
        }
    }

    #[doc="Write the AWSCD2R register."]
    #[inline] pub fn set_awscd2r<F: FnOnce(Awscd2r) -> Awscd2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd2r_mut(), f(Awscd2r(0)));
        }
        self
    }

    #[doc="Modify the AWSCD2R register."]
    #[inline] pub fn with_awscd2r<F: FnOnce(Awscd2r) -> Awscd2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd2r_mut(), f(self.awscd2r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHWDAT2R register."]
    #[inline] pub fn chwdat2r_mut(&self) -> *mut Chwdat2r { 
        (self.0 + 0x4c) as *mut Chwdat2r
    }

    #[doc="Get the *const pointer for the CHWDAT2R register."]
    #[inline] pub fn chwdat2r_ptr(&self) -> *const Chwdat2r { 
           self.chwdat2r_mut()
    }

    #[doc="Read the CHWDAT2R register."]
    #[inline] pub fn chwdat2r(&self) -> Chwdat2r { 
        unsafe {
            read_volatile(self.chwdat2r_ptr())
        }
    }

    #[doc="Write the CHWDAT2R register."]
    #[inline] pub fn set_chwdat2r<F: FnOnce(Chwdat2r) -> Chwdat2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat2r_mut(), f(Chwdat2r(0)));
        }
        self
    }

    #[doc="Modify the CHWDAT2R register."]
    #[inline] pub fn with_chwdat2r<F: FnOnce(Chwdat2r) -> Chwdat2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat2r_mut(), f(self.chwdat2r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHDATIN2R register."]
    #[inline] pub fn chdatin2r_mut(&self) -> *mut Chdatin2r { 
        (self.0 + 0x50) as *mut Chdatin2r
    }

    #[doc="Get the *const pointer for the CHDATIN2R register."]
    #[inline] pub fn chdatin2r_ptr(&self) -> *const Chdatin2r { 
           self.chdatin2r_mut()
    }

    #[doc="Read the CHDATIN2R register."]
    #[inline] pub fn chdatin2r(&self) -> Chdatin2r { 
        unsafe {
            read_volatile(self.chdatin2r_ptr())
        }
    }

    #[doc="Write the CHDATIN2R register."]
    #[inline] pub fn set_chdatin2r<F: FnOnce(Chdatin2r) -> Chdatin2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin2r_mut(), f(Chdatin2r(0)));
        }
        self
    }

    #[doc="Modify the CHDATIN2R register."]
    #[inline] pub fn with_chdatin2r<F: FnOnce(Chdatin2r) -> Chdatin2r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin2r_mut(), f(self.chdatin2r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG3R1 register."]
    #[inline] pub fn chcfg3r1_mut(&self) -> *mut Chcfg3r1 { 
        (self.0 + 0x60) as *mut Chcfg3r1
    }

    #[doc="Get the *const pointer for the CHCFG3R1 register."]
    #[inline] pub fn chcfg3r1_ptr(&self) -> *const Chcfg3r1 { 
           self.chcfg3r1_mut()
    }

    #[doc="Read the CHCFG3R1 register."]
    #[inline] pub fn chcfg3r1(&self) -> Chcfg3r1 { 
        unsafe {
            read_volatile(self.chcfg3r1_ptr())
        }
    }

    #[doc="Write the CHCFG3R1 register."]
    #[inline] pub fn set_chcfg3r1<F: FnOnce(Chcfg3r1) -> Chcfg3r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg3r1_mut(), f(Chcfg3r1(0)));
        }
        self
    }

    #[doc="Modify the CHCFG3R1 register."]
    #[inline] pub fn with_chcfg3r1<F: FnOnce(Chcfg3r1) -> Chcfg3r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg3r1_mut(), f(self.chcfg3r1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG3R2 register."]
    #[inline] pub fn chcfg3r2_mut(&self) -> *mut Chcfg3r2 { 
        (self.0 + 0x64) as *mut Chcfg3r2
    }

    #[doc="Get the *const pointer for the CHCFG3R2 register."]
    #[inline] pub fn chcfg3r2_ptr(&self) -> *const Chcfg3r2 { 
           self.chcfg3r2_mut()
    }

    #[doc="Read the CHCFG3R2 register."]
    #[inline] pub fn chcfg3r2(&self) -> Chcfg3r2 { 
        unsafe {
            read_volatile(self.chcfg3r2_ptr())
        }
    }

    #[doc="Write the CHCFG3R2 register."]
    #[inline] pub fn set_chcfg3r2<F: FnOnce(Chcfg3r2) -> Chcfg3r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg3r2_mut(), f(Chcfg3r2(0)));
        }
        self
    }

    #[doc="Modify the CHCFG3R2 register."]
    #[inline] pub fn with_chcfg3r2<F: FnOnce(Chcfg3r2) -> Chcfg3r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg3r2_mut(), f(self.chcfg3r2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AWSCD3R register."]
    #[inline] pub fn awscd3r_mut(&self) -> *mut Awscd3r { 
        (self.0 + 0x68) as *mut Awscd3r
    }

    #[doc="Get the *const pointer for the AWSCD3R register."]
    #[inline] pub fn awscd3r_ptr(&self) -> *const Awscd3r { 
           self.awscd3r_mut()
    }

    #[doc="Read the AWSCD3R register."]
    #[inline] pub fn awscd3r(&self) -> Awscd3r { 
        unsafe {
            read_volatile(self.awscd3r_ptr())
        }
    }

    #[doc="Write the AWSCD3R register."]
    #[inline] pub fn set_awscd3r<F: FnOnce(Awscd3r) -> Awscd3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd3r_mut(), f(Awscd3r(0)));
        }
        self
    }

    #[doc="Modify the AWSCD3R register."]
    #[inline] pub fn with_awscd3r<F: FnOnce(Awscd3r) -> Awscd3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd3r_mut(), f(self.awscd3r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHWDAT3R register."]
    #[inline] pub fn chwdat3r_mut(&self) -> *mut Chwdat3r { 
        (self.0 + 0x6c) as *mut Chwdat3r
    }

    #[doc="Get the *const pointer for the CHWDAT3R register."]
    #[inline] pub fn chwdat3r_ptr(&self) -> *const Chwdat3r { 
           self.chwdat3r_mut()
    }

    #[doc="Read the CHWDAT3R register."]
    #[inline] pub fn chwdat3r(&self) -> Chwdat3r { 
        unsafe {
            read_volatile(self.chwdat3r_ptr())
        }
    }

    #[doc="Write the CHWDAT3R register."]
    #[inline] pub fn set_chwdat3r<F: FnOnce(Chwdat3r) -> Chwdat3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat3r_mut(), f(Chwdat3r(0)));
        }
        self
    }

    #[doc="Modify the CHWDAT3R register."]
    #[inline] pub fn with_chwdat3r<F: FnOnce(Chwdat3r) -> Chwdat3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat3r_mut(), f(self.chwdat3r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHDATIN3R register."]
    #[inline] pub fn chdatin3r_mut(&self) -> *mut Chdatin3r { 
        (self.0 + 0x70) as *mut Chdatin3r
    }

    #[doc="Get the *const pointer for the CHDATIN3R register."]
    #[inline] pub fn chdatin3r_ptr(&self) -> *const Chdatin3r { 
           self.chdatin3r_mut()
    }

    #[doc="Read the CHDATIN3R register."]
    #[inline] pub fn chdatin3r(&self) -> Chdatin3r { 
        unsafe {
            read_volatile(self.chdatin3r_ptr())
        }
    }

    #[doc="Write the CHDATIN3R register."]
    #[inline] pub fn set_chdatin3r<F: FnOnce(Chdatin3r) -> Chdatin3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin3r_mut(), f(Chdatin3r(0)));
        }
        self
    }

    #[doc="Modify the CHDATIN3R register."]
    #[inline] pub fn with_chdatin3r<F: FnOnce(Chdatin3r) -> Chdatin3r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin3r_mut(), f(self.chdatin3r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG4R1 register."]
    #[inline] pub fn chcfg4r1_mut(&self) -> *mut Chcfg4r1 { 
        (self.0 + 0x80) as *mut Chcfg4r1
    }

    #[doc="Get the *const pointer for the CHCFG4R1 register."]
    #[inline] pub fn chcfg4r1_ptr(&self) -> *const Chcfg4r1 { 
           self.chcfg4r1_mut()
    }

    #[doc="Read the CHCFG4R1 register."]
    #[inline] pub fn chcfg4r1(&self) -> Chcfg4r1 { 
        unsafe {
            read_volatile(self.chcfg4r1_ptr())
        }
    }

    #[doc="Write the CHCFG4R1 register."]
    #[inline] pub fn set_chcfg4r1<F: FnOnce(Chcfg4r1) -> Chcfg4r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg4r1_mut(), f(Chcfg4r1(0)));
        }
        self
    }

    #[doc="Modify the CHCFG4R1 register."]
    #[inline] pub fn with_chcfg4r1<F: FnOnce(Chcfg4r1) -> Chcfg4r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg4r1_mut(), f(self.chcfg4r1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG4R2 register."]
    #[inline] pub fn chcfg4r2_mut(&self) -> *mut Chcfg4r2 { 
        (self.0 + 0x84) as *mut Chcfg4r2
    }

    #[doc="Get the *const pointer for the CHCFG4R2 register."]
    #[inline] pub fn chcfg4r2_ptr(&self) -> *const Chcfg4r2 { 
           self.chcfg4r2_mut()
    }

    #[doc="Read the CHCFG4R2 register."]
    #[inline] pub fn chcfg4r2(&self) -> Chcfg4r2 { 
        unsafe {
            read_volatile(self.chcfg4r2_ptr())
        }
    }

    #[doc="Write the CHCFG4R2 register."]
    #[inline] pub fn set_chcfg4r2<F: FnOnce(Chcfg4r2) -> Chcfg4r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg4r2_mut(), f(Chcfg4r2(0)));
        }
        self
    }

    #[doc="Modify the CHCFG4R2 register."]
    #[inline] pub fn with_chcfg4r2<F: FnOnce(Chcfg4r2) -> Chcfg4r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg4r2_mut(), f(self.chcfg4r2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AWSCD4R register."]
    #[inline] pub fn awscd4r_mut(&self) -> *mut Awscd4r { 
        (self.0 + 0x88) as *mut Awscd4r
    }

    #[doc="Get the *const pointer for the AWSCD4R register."]
    #[inline] pub fn awscd4r_ptr(&self) -> *const Awscd4r { 
           self.awscd4r_mut()
    }

    #[doc="Read the AWSCD4R register."]
    #[inline] pub fn awscd4r(&self) -> Awscd4r { 
        unsafe {
            read_volatile(self.awscd4r_ptr())
        }
    }

    #[doc="Write the AWSCD4R register."]
    #[inline] pub fn set_awscd4r<F: FnOnce(Awscd4r) -> Awscd4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd4r_mut(), f(Awscd4r(0)));
        }
        self
    }

    #[doc="Modify the AWSCD4R register."]
    #[inline] pub fn with_awscd4r<F: FnOnce(Awscd4r) -> Awscd4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd4r_mut(), f(self.awscd4r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHWDAT4R register."]
    #[inline] pub fn chwdat4r_mut(&self) -> *mut Chwdat4r { 
        (self.0 + 0x8c) as *mut Chwdat4r
    }

    #[doc="Get the *const pointer for the CHWDAT4R register."]
    #[inline] pub fn chwdat4r_ptr(&self) -> *const Chwdat4r { 
           self.chwdat4r_mut()
    }

    #[doc="Read the CHWDAT4R register."]
    #[inline] pub fn chwdat4r(&self) -> Chwdat4r { 
        unsafe {
            read_volatile(self.chwdat4r_ptr())
        }
    }

    #[doc="Write the CHWDAT4R register."]
    #[inline] pub fn set_chwdat4r<F: FnOnce(Chwdat4r) -> Chwdat4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat4r_mut(), f(Chwdat4r(0)));
        }
        self
    }

    #[doc="Modify the CHWDAT4R register."]
    #[inline] pub fn with_chwdat4r<F: FnOnce(Chwdat4r) -> Chwdat4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat4r_mut(), f(self.chwdat4r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHDATIN4R register."]
    #[inline] pub fn chdatin4r_mut(&self) -> *mut Chdatin4r { 
        (self.0 + 0x90) as *mut Chdatin4r
    }

    #[doc="Get the *const pointer for the CHDATIN4R register."]
    #[inline] pub fn chdatin4r_ptr(&self) -> *const Chdatin4r { 
           self.chdatin4r_mut()
    }

    #[doc="Read the CHDATIN4R register."]
    #[inline] pub fn chdatin4r(&self) -> Chdatin4r { 
        unsafe {
            read_volatile(self.chdatin4r_ptr())
        }
    }

    #[doc="Write the CHDATIN4R register."]
    #[inline] pub fn set_chdatin4r<F: FnOnce(Chdatin4r) -> Chdatin4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin4r_mut(), f(Chdatin4r(0)));
        }
        self
    }

    #[doc="Modify the CHDATIN4R register."]
    #[inline] pub fn with_chdatin4r<F: FnOnce(Chdatin4r) -> Chdatin4r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin4r_mut(), f(self.chdatin4r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG5R1 register."]
    #[inline] pub fn chcfg5r1_mut(&self) -> *mut Chcfg5r1 { 
        (self.0 + 0xa0) as *mut Chcfg5r1
    }

    #[doc="Get the *const pointer for the CHCFG5R1 register."]
    #[inline] pub fn chcfg5r1_ptr(&self) -> *const Chcfg5r1 { 
           self.chcfg5r1_mut()
    }

    #[doc="Read the CHCFG5R1 register."]
    #[inline] pub fn chcfg5r1(&self) -> Chcfg5r1 { 
        unsafe {
            read_volatile(self.chcfg5r1_ptr())
        }
    }

    #[doc="Write the CHCFG5R1 register."]
    #[inline] pub fn set_chcfg5r1<F: FnOnce(Chcfg5r1) -> Chcfg5r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg5r1_mut(), f(Chcfg5r1(0)));
        }
        self
    }

    #[doc="Modify the CHCFG5R1 register."]
    #[inline] pub fn with_chcfg5r1<F: FnOnce(Chcfg5r1) -> Chcfg5r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg5r1_mut(), f(self.chcfg5r1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG5R2 register."]
    #[inline] pub fn chcfg5r2_mut(&self) -> *mut Chcfg5r2 { 
        (self.0 + 0xa4) as *mut Chcfg5r2
    }

    #[doc="Get the *const pointer for the CHCFG5R2 register."]
    #[inline] pub fn chcfg5r2_ptr(&self) -> *const Chcfg5r2 { 
           self.chcfg5r2_mut()
    }

    #[doc="Read the CHCFG5R2 register."]
    #[inline] pub fn chcfg5r2(&self) -> Chcfg5r2 { 
        unsafe {
            read_volatile(self.chcfg5r2_ptr())
        }
    }

    #[doc="Write the CHCFG5R2 register."]
    #[inline] pub fn set_chcfg5r2<F: FnOnce(Chcfg5r2) -> Chcfg5r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg5r2_mut(), f(Chcfg5r2(0)));
        }
        self
    }

    #[doc="Modify the CHCFG5R2 register."]
    #[inline] pub fn with_chcfg5r2<F: FnOnce(Chcfg5r2) -> Chcfg5r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg5r2_mut(), f(self.chcfg5r2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AWSCD5R register."]
    #[inline] pub fn awscd5r_mut(&self) -> *mut Awscd5r { 
        (self.0 + 0xa8) as *mut Awscd5r
    }

    #[doc="Get the *const pointer for the AWSCD5R register."]
    #[inline] pub fn awscd5r_ptr(&self) -> *const Awscd5r { 
           self.awscd5r_mut()
    }

    #[doc="Read the AWSCD5R register."]
    #[inline] pub fn awscd5r(&self) -> Awscd5r { 
        unsafe {
            read_volatile(self.awscd5r_ptr())
        }
    }

    #[doc="Write the AWSCD5R register."]
    #[inline] pub fn set_awscd5r<F: FnOnce(Awscd5r) -> Awscd5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd5r_mut(), f(Awscd5r(0)));
        }
        self
    }

    #[doc="Modify the AWSCD5R register."]
    #[inline] pub fn with_awscd5r<F: FnOnce(Awscd5r) -> Awscd5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd5r_mut(), f(self.awscd5r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHWDAT5R register."]
    #[inline] pub fn chwdat5r_mut(&self) -> *mut Chwdat5r { 
        (self.0 + 0xac) as *mut Chwdat5r
    }

    #[doc="Get the *const pointer for the CHWDAT5R register."]
    #[inline] pub fn chwdat5r_ptr(&self) -> *const Chwdat5r { 
           self.chwdat5r_mut()
    }

    #[doc="Read the CHWDAT5R register."]
    #[inline] pub fn chwdat5r(&self) -> Chwdat5r { 
        unsafe {
            read_volatile(self.chwdat5r_ptr())
        }
    }

    #[doc="Write the CHWDAT5R register."]
    #[inline] pub fn set_chwdat5r<F: FnOnce(Chwdat5r) -> Chwdat5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat5r_mut(), f(Chwdat5r(0)));
        }
        self
    }

    #[doc="Modify the CHWDAT5R register."]
    #[inline] pub fn with_chwdat5r<F: FnOnce(Chwdat5r) -> Chwdat5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat5r_mut(), f(self.chwdat5r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHDATIN5R register."]
    #[inline] pub fn chdatin5r_mut(&self) -> *mut Chdatin5r { 
        (self.0 + 0xb0) as *mut Chdatin5r
    }

    #[doc="Get the *const pointer for the CHDATIN5R register."]
    #[inline] pub fn chdatin5r_ptr(&self) -> *const Chdatin5r { 
           self.chdatin5r_mut()
    }

    #[doc="Read the CHDATIN5R register."]
    #[inline] pub fn chdatin5r(&self) -> Chdatin5r { 
        unsafe {
            read_volatile(self.chdatin5r_ptr())
        }
    }

    #[doc="Write the CHDATIN5R register."]
    #[inline] pub fn set_chdatin5r<F: FnOnce(Chdatin5r) -> Chdatin5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin5r_mut(), f(Chdatin5r(0)));
        }
        self
    }

    #[doc="Modify the CHDATIN5R register."]
    #[inline] pub fn with_chdatin5r<F: FnOnce(Chdatin5r) -> Chdatin5r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin5r_mut(), f(self.chdatin5r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG6R1 register."]
    #[inline] pub fn chcfg6r1_mut(&self) -> *mut Chcfg6r1 { 
        (self.0 + 0xc0) as *mut Chcfg6r1
    }

    #[doc="Get the *const pointer for the CHCFG6R1 register."]
    #[inline] pub fn chcfg6r1_ptr(&self) -> *const Chcfg6r1 { 
           self.chcfg6r1_mut()
    }

    #[doc="Read the CHCFG6R1 register."]
    #[inline] pub fn chcfg6r1(&self) -> Chcfg6r1 { 
        unsafe {
            read_volatile(self.chcfg6r1_ptr())
        }
    }

    #[doc="Write the CHCFG6R1 register."]
    #[inline] pub fn set_chcfg6r1<F: FnOnce(Chcfg6r1) -> Chcfg6r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg6r1_mut(), f(Chcfg6r1(0)));
        }
        self
    }

    #[doc="Modify the CHCFG6R1 register."]
    #[inline] pub fn with_chcfg6r1<F: FnOnce(Chcfg6r1) -> Chcfg6r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg6r1_mut(), f(self.chcfg6r1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG6R2 register."]
    #[inline] pub fn chcfg6r2_mut(&self) -> *mut Chcfg6r2 { 
        (self.0 + 0xc4) as *mut Chcfg6r2
    }

    #[doc="Get the *const pointer for the CHCFG6R2 register."]
    #[inline] pub fn chcfg6r2_ptr(&self) -> *const Chcfg6r2 { 
           self.chcfg6r2_mut()
    }

    #[doc="Read the CHCFG6R2 register."]
    #[inline] pub fn chcfg6r2(&self) -> Chcfg6r2 { 
        unsafe {
            read_volatile(self.chcfg6r2_ptr())
        }
    }

    #[doc="Write the CHCFG6R2 register."]
    #[inline] pub fn set_chcfg6r2<F: FnOnce(Chcfg6r2) -> Chcfg6r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg6r2_mut(), f(Chcfg6r2(0)));
        }
        self
    }

    #[doc="Modify the CHCFG6R2 register."]
    #[inline] pub fn with_chcfg6r2<F: FnOnce(Chcfg6r2) -> Chcfg6r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg6r2_mut(), f(self.chcfg6r2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AWSCD6R register."]
    #[inline] pub fn awscd6r_mut(&self) -> *mut Awscd6r { 
        (self.0 + 0xc8) as *mut Awscd6r
    }

    #[doc="Get the *const pointer for the AWSCD6R register."]
    #[inline] pub fn awscd6r_ptr(&self) -> *const Awscd6r { 
           self.awscd6r_mut()
    }

    #[doc="Read the AWSCD6R register."]
    #[inline] pub fn awscd6r(&self) -> Awscd6r { 
        unsafe {
            read_volatile(self.awscd6r_ptr())
        }
    }

    #[doc="Write the AWSCD6R register."]
    #[inline] pub fn set_awscd6r<F: FnOnce(Awscd6r) -> Awscd6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd6r_mut(), f(Awscd6r(0)));
        }
        self
    }

    #[doc="Modify the AWSCD6R register."]
    #[inline] pub fn with_awscd6r<F: FnOnce(Awscd6r) -> Awscd6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd6r_mut(), f(self.awscd6r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHWDAT6R register."]
    #[inline] pub fn chwdat6r_mut(&self) -> *mut Chwdat6r { 
        (self.0 + 0xcc) as *mut Chwdat6r
    }

    #[doc="Get the *const pointer for the CHWDAT6R register."]
    #[inline] pub fn chwdat6r_ptr(&self) -> *const Chwdat6r { 
           self.chwdat6r_mut()
    }

    #[doc="Read the CHWDAT6R register."]
    #[inline] pub fn chwdat6r(&self) -> Chwdat6r { 
        unsafe {
            read_volatile(self.chwdat6r_ptr())
        }
    }

    #[doc="Write the CHWDAT6R register."]
    #[inline] pub fn set_chwdat6r<F: FnOnce(Chwdat6r) -> Chwdat6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat6r_mut(), f(Chwdat6r(0)));
        }
        self
    }

    #[doc="Modify the CHWDAT6R register."]
    #[inline] pub fn with_chwdat6r<F: FnOnce(Chwdat6r) -> Chwdat6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat6r_mut(), f(self.chwdat6r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHDATIN6R register."]
    #[inline] pub fn chdatin6r_mut(&self) -> *mut Chdatin6r { 
        (self.0 + 0xd0) as *mut Chdatin6r
    }

    #[doc="Get the *const pointer for the CHDATIN6R register."]
    #[inline] pub fn chdatin6r_ptr(&self) -> *const Chdatin6r { 
           self.chdatin6r_mut()
    }

    #[doc="Read the CHDATIN6R register."]
    #[inline] pub fn chdatin6r(&self) -> Chdatin6r { 
        unsafe {
            read_volatile(self.chdatin6r_ptr())
        }
    }

    #[doc="Write the CHDATIN6R register."]
    #[inline] pub fn set_chdatin6r<F: FnOnce(Chdatin6r) -> Chdatin6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin6r_mut(), f(Chdatin6r(0)));
        }
        self
    }

    #[doc="Modify the CHDATIN6R register."]
    #[inline] pub fn with_chdatin6r<F: FnOnce(Chdatin6r) -> Chdatin6r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin6r_mut(), f(self.chdatin6r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG7R1 register."]
    #[inline] pub fn chcfg7r1_mut(&self) -> *mut Chcfg7r1 { 
        (self.0 + 0xe0) as *mut Chcfg7r1
    }

    #[doc="Get the *const pointer for the CHCFG7R1 register."]
    #[inline] pub fn chcfg7r1_ptr(&self) -> *const Chcfg7r1 { 
           self.chcfg7r1_mut()
    }

    #[doc="Read the CHCFG7R1 register."]
    #[inline] pub fn chcfg7r1(&self) -> Chcfg7r1 { 
        unsafe {
            read_volatile(self.chcfg7r1_ptr())
        }
    }

    #[doc="Write the CHCFG7R1 register."]
    #[inline] pub fn set_chcfg7r1<F: FnOnce(Chcfg7r1) -> Chcfg7r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg7r1_mut(), f(Chcfg7r1(0)));
        }
        self
    }

    #[doc="Modify the CHCFG7R1 register."]
    #[inline] pub fn with_chcfg7r1<F: FnOnce(Chcfg7r1) -> Chcfg7r1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg7r1_mut(), f(self.chcfg7r1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHCFG7R2 register."]
    #[inline] pub fn chcfg7r2_mut(&self) -> *mut Chcfg7r2 { 
        (self.0 + 0xe4) as *mut Chcfg7r2
    }

    #[doc="Get the *const pointer for the CHCFG7R2 register."]
    #[inline] pub fn chcfg7r2_ptr(&self) -> *const Chcfg7r2 { 
           self.chcfg7r2_mut()
    }

    #[doc="Read the CHCFG7R2 register."]
    #[inline] pub fn chcfg7r2(&self) -> Chcfg7r2 { 
        unsafe {
            read_volatile(self.chcfg7r2_ptr())
        }
    }

    #[doc="Write the CHCFG7R2 register."]
    #[inline] pub fn set_chcfg7r2<F: FnOnce(Chcfg7r2) -> Chcfg7r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg7r2_mut(), f(Chcfg7r2(0)));
        }
        self
    }

    #[doc="Modify the CHCFG7R2 register."]
    #[inline] pub fn with_chcfg7r2<F: FnOnce(Chcfg7r2) -> Chcfg7r2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chcfg7r2_mut(), f(self.chcfg7r2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AWSCD7R register."]
    #[inline] pub fn awscd7r_mut(&self) -> *mut Awscd7r { 
        (self.0 + 0xe8) as *mut Awscd7r
    }

    #[doc="Get the *const pointer for the AWSCD7R register."]
    #[inline] pub fn awscd7r_ptr(&self) -> *const Awscd7r { 
           self.awscd7r_mut()
    }

    #[doc="Read the AWSCD7R register."]
    #[inline] pub fn awscd7r(&self) -> Awscd7r { 
        unsafe {
            read_volatile(self.awscd7r_ptr())
        }
    }

    #[doc="Write the AWSCD7R register."]
    #[inline] pub fn set_awscd7r<F: FnOnce(Awscd7r) -> Awscd7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd7r_mut(), f(Awscd7r(0)));
        }
        self
    }

    #[doc="Modify the AWSCD7R register."]
    #[inline] pub fn with_awscd7r<F: FnOnce(Awscd7r) -> Awscd7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.awscd7r_mut(), f(self.awscd7r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHWDAT7R register."]
    #[inline] pub fn chwdat7r_mut(&self) -> *mut Chwdat7r { 
        (self.0 + 0xec) as *mut Chwdat7r
    }

    #[doc="Get the *const pointer for the CHWDAT7R register."]
    #[inline] pub fn chwdat7r_ptr(&self) -> *const Chwdat7r { 
           self.chwdat7r_mut()
    }

    #[doc="Read the CHWDAT7R register."]
    #[inline] pub fn chwdat7r(&self) -> Chwdat7r { 
        unsafe {
            read_volatile(self.chwdat7r_ptr())
        }
    }

    #[doc="Write the CHWDAT7R register."]
    #[inline] pub fn set_chwdat7r<F: FnOnce(Chwdat7r) -> Chwdat7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat7r_mut(), f(Chwdat7r(0)));
        }
        self
    }

    #[doc="Modify the CHWDAT7R register."]
    #[inline] pub fn with_chwdat7r<F: FnOnce(Chwdat7r) -> Chwdat7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chwdat7r_mut(), f(self.chwdat7r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHDATIN7R register."]
    #[inline] pub fn chdatin7r_mut(&self) -> *mut Chdatin7r { 
        (self.0 + 0xf0) as *mut Chdatin7r
    }

    #[doc="Get the *const pointer for the CHDATIN7R register."]
    #[inline] pub fn chdatin7r_ptr(&self) -> *const Chdatin7r { 
           self.chdatin7r_mut()
    }

    #[doc="Read the CHDATIN7R register."]
    #[inline] pub fn chdatin7r(&self) -> Chdatin7r { 
        unsafe {
            read_volatile(self.chdatin7r_ptr())
        }
    }

    #[doc="Write the CHDATIN7R register."]
    #[inline] pub fn set_chdatin7r<F: FnOnce(Chdatin7r) -> Chdatin7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin7r_mut(), f(Chdatin7r(0)));
        }
        self
    }

    #[doc="Modify the CHDATIN7R register."]
    #[inline] pub fn with_chdatin7r<F: FnOnce(Chdatin7r) -> Chdatin7r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chdatin7r_mut(), f(self.chdatin7r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM0_CR1 register."]
    #[inline] pub fn dfsdm0_cr1_mut(&self) -> *mut Dfsdm0Cr1 { 
        (self.0 + 0x100) as *mut Dfsdm0Cr1
    }

    #[doc="Get the *const pointer for the DFSDM0_CR1 register."]
    #[inline] pub fn dfsdm0_cr1_ptr(&self) -> *const Dfsdm0Cr1 { 
           self.dfsdm0_cr1_mut()
    }

    #[doc="Read the DFSDM0_CR1 register."]
    #[inline] pub fn dfsdm0_cr1(&self) -> Dfsdm0Cr1 { 
        unsafe {
            read_volatile(self.dfsdm0_cr1_ptr())
        }
    }

    #[doc="Write the DFSDM0_CR1 register."]
    #[inline] pub fn set_dfsdm0_cr1<F: FnOnce(Dfsdm0Cr1) -> Dfsdm0Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_cr1_mut(), f(Dfsdm0Cr1(0)));
        }
        self
    }

    #[doc="Modify the DFSDM0_CR1 register."]
    #[inline] pub fn with_dfsdm0_cr1<F: FnOnce(Dfsdm0Cr1) -> Dfsdm0Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_cr1_mut(), f(self.dfsdm0_cr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM0_CR2 register."]
    #[inline] pub fn dfsdm0_cr2_mut(&self) -> *mut Dfsdm0Cr2 { 
        (self.0 + 0x104) as *mut Dfsdm0Cr2
    }

    #[doc="Get the *const pointer for the DFSDM0_CR2 register."]
    #[inline] pub fn dfsdm0_cr2_ptr(&self) -> *const Dfsdm0Cr2 { 
           self.dfsdm0_cr2_mut()
    }

    #[doc="Read the DFSDM0_CR2 register."]
    #[inline] pub fn dfsdm0_cr2(&self) -> Dfsdm0Cr2 { 
        unsafe {
            read_volatile(self.dfsdm0_cr2_ptr())
        }
    }

    #[doc="Write the DFSDM0_CR2 register."]
    #[inline] pub fn set_dfsdm0_cr2<F: FnOnce(Dfsdm0Cr2) -> Dfsdm0Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_cr2_mut(), f(Dfsdm0Cr2(0)));
        }
        self
    }

    #[doc="Modify the DFSDM0_CR2 register."]
    #[inline] pub fn with_dfsdm0_cr2<F: FnOnce(Dfsdm0Cr2) -> Dfsdm0Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_cr2_mut(), f(self.dfsdm0_cr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM0_ISR register."]
    #[inline] pub fn dfsdm0_isr_mut(&self) -> *mut Dfsdm0Isr { 
        (self.0 + 0x108) as *mut Dfsdm0Isr
    }

    #[doc="Get the *const pointer for the DFSDM0_ISR register."]
    #[inline] pub fn dfsdm0_isr_ptr(&self) -> *const Dfsdm0Isr { 
           self.dfsdm0_isr_mut()
    }

    #[doc="Read the DFSDM0_ISR register."]
    #[inline] pub fn dfsdm0_isr(&self) -> Dfsdm0Isr { 
        unsafe {
            read_volatile(self.dfsdm0_isr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM0_ICR register."]
    #[inline] pub fn dfsdm0_icr_mut(&self) -> *mut Dfsdm0Icr { 
        (self.0 + 0x10c) as *mut Dfsdm0Icr
    }

    #[doc="Get the *const pointer for the DFSDM0_ICR register."]
    #[inline] pub fn dfsdm0_icr_ptr(&self) -> *const Dfsdm0Icr { 
           self.dfsdm0_icr_mut()
    }

    #[doc="Read the DFSDM0_ICR register."]
    #[inline] pub fn dfsdm0_icr(&self) -> Dfsdm0Icr { 
        unsafe {
            read_volatile(self.dfsdm0_icr_ptr())
        }
    }

    #[doc="Write the DFSDM0_ICR register."]
    #[inline] pub fn set_dfsdm0_icr<F: FnOnce(Dfsdm0Icr) -> Dfsdm0Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_icr_mut(), f(Dfsdm0Icr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM0_ICR register."]
    #[inline] pub fn with_dfsdm0_icr<F: FnOnce(Dfsdm0Icr) -> Dfsdm0Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_icr_mut(), f(self.dfsdm0_icr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM0_JCHGR register."]
    #[inline] pub fn dfsdm0_jchgr_mut(&self) -> *mut Dfsdm0Jchgr { 
        (self.0 + 0x110) as *mut Dfsdm0Jchgr
    }

    #[doc="Get the *const pointer for the DFSDM0_JCHGR register."]
    #[inline] pub fn dfsdm0_jchgr_ptr(&self) -> *const Dfsdm0Jchgr { 
           self.dfsdm0_jchgr_mut()
    }

    #[doc="Read the DFSDM0_JCHGR register."]
    #[inline] pub fn dfsdm0_jchgr(&self) -> Dfsdm0Jchgr { 
        unsafe {
            read_volatile(self.dfsdm0_jchgr_ptr())
        }
    }

    #[doc="Write the DFSDM0_JCHGR register."]
    #[inline] pub fn set_dfsdm0_jchgr<F: FnOnce(Dfsdm0Jchgr) -> Dfsdm0Jchgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_jchgr_mut(), f(Dfsdm0Jchgr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM0_JCHGR register."]
    #[inline] pub fn with_dfsdm0_jchgr<F: FnOnce(Dfsdm0Jchgr) -> Dfsdm0Jchgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_jchgr_mut(), f(self.dfsdm0_jchgr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM0_FCR register."]
    #[inline] pub fn dfsdm0_fcr_mut(&self) -> *mut Dfsdm0Fcr { 
        (self.0 + 0x114) as *mut Dfsdm0Fcr
    }

    #[doc="Get the *const pointer for the DFSDM0_FCR register."]
    #[inline] pub fn dfsdm0_fcr_ptr(&self) -> *const Dfsdm0Fcr { 
           self.dfsdm0_fcr_mut()
    }

    #[doc="Read the DFSDM0_FCR register."]
    #[inline] pub fn dfsdm0_fcr(&self) -> Dfsdm0Fcr { 
        unsafe {
            read_volatile(self.dfsdm0_fcr_ptr())
        }
    }

    #[doc="Write the DFSDM0_FCR register."]
    #[inline] pub fn set_dfsdm0_fcr<F: FnOnce(Dfsdm0Fcr) -> Dfsdm0Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_fcr_mut(), f(Dfsdm0Fcr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM0_FCR register."]
    #[inline] pub fn with_dfsdm0_fcr<F: FnOnce(Dfsdm0Fcr) -> Dfsdm0Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_fcr_mut(), f(self.dfsdm0_fcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM0_JDATAR register."]
    #[inline] pub fn dfsdm0_jdatar_mut(&self) -> *mut Dfsdm0Jdatar { 
        (self.0 + 0x118) as *mut Dfsdm0Jdatar
    }

    #[doc="Get the *const pointer for the DFSDM0_JDATAR register."]
    #[inline] pub fn dfsdm0_jdatar_ptr(&self) -> *const Dfsdm0Jdatar { 
           self.dfsdm0_jdatar_mut()
    }

    #[doc="Read the DFSDM0_JDATAR register."]
    #[inline] pub fn dfsdm0_jdatar(&self) -> Dfsdm0Jdatar { 
        unsafe {
            read_volatile(self.dfsdm0_jdatar_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM0_RDATAR register."]
    #[inline] pub fn dfsdm0_rdatar_mut(&self) -> *mut Dfsdm0Rdatar { 
        (self.0 + 0x11c) as *mut Dfsdm0Rdatar
    }

    #[doc="Get the *const pointer for the DFSDM0_RDATAR register."]
    #[inline] pub fn dfsdm0_rdatar_ptr(&self) -> *const Dfsdm0Rdatar { 
           self.dfsdm0_rdatar_mut()
    }

    #[doc="Read the DFSDM0_RDATAR register."]
    #[inline] pub fn dfsdm0_rdatar(&self) -> Dfsdm0Rdatar { 
        unsafe {
            read_volatile(self.dfsdm0_rdatar_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM0_AWHTR register."]
    #[inline] pub fn dfsdm0_awhtr_mut(&self) -> *mut Dfsdm0Awhtr { 
        (self.0 + 0x120) as *mut Dfsdm0Awhtr
    }

    #[doc="Get the *const pointer for the DFSDM0_AWHTR register."]
    #[inline] pub fn dfsdm0_awhtr_ptr(&self) -> *const Dfsdm0Awhtr { 
           self.dfsdm0_awhtr_mut()
    }

    #[doc="Read the DFSDM0_AWHTR register."]
    #[inline] pub fn dfsdm0_awhtr(&self) -> Dfsdm0Awhtr { 
        unsafe {
            read_volatile(self.dfsdm0_awhtr_ptr())
        }
    }

    #[doc="Write the DFSDM0_AWHTR register."]
    #[inline] pub fn set_dfsdm0_awhtr<F: FnOnce(Dfsdm0Awhtr) -> Dfsdm0Awhtr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_awhtr_mut(), f(Dfsdm0Awhtr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM0_AWHTR register."]
    #[inline] pub fn with_dfsdm0_awhtr<F: FnOnce(Dfsdm0Awhtr) -> Dfsdm0Awhtr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_awhtr_mut(), f(self.dfsdm0_awhtr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM0_AWLTR register."]
    #[inline] pub fn dfsdm0_awltr_mut(&self) -> *mut Dfsdm0Awltr { 
        (self.0 + 0x124) as *mut Dfsdm0Awltr
    }

    #[doc="Get the *const pointer for the DFSDM0_AWLTR register."]
    #[inline] pub fn dfsdm0_awltr_ptr(&self) -> *const Dfsdm0Awltr { 
           self.dfsdm0_awltr_mut()
    }

    #[doc="Read the DFSDM0_AWLTR register."]
    #[inline] pub fn dfsdm0_awltr(&self) -> Dfsdm0Awltr { 
        unsafe {
            read_volatile(self.dfsdm0_awltr_ptr())
        }
    }

    #[doc="Write the DFSDM0_AWLTR register."]
    #[inline] pub fn set_dfsdm0_awltr<F: FnOnce(Dfsdm0Awltr) -> Dfsdm0Awltr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_awltr_mut(), f(Dfsdm0Awltr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM0_AWLTR register."]
    #[inline] pub fn with_dfsdm0_awltr<F: FnOnce(Dfsdm0Awltr) -> Dfsdm0Awltr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_awltr_mut(), f(self.dfsdm0_awltr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM0_AWSR register."]
    #[inline] pub fn dfsdm0_awsr_mut(&self) -> *mut Dfsdm0Awsr { 
        (self.0 + 0x128) as *mut Dfsdm0Awsr
    }

    #[doc="Get the *const pointer for the DFSDM0_AWSR register."]
    #[inline] pub fn dfsdm0_awsr_ptr(&self) -> *const Dfsdm0Awsr { 
           self.dfsdm0_awsr_mut()
    }

    #[doc="Read the DFSDM0_AWSR register."]
    #[inline] pub fn dfsdm0_awsr(&self) -> Dfsdm0Awsr { 
        unsafe {
            read_volatile(self.dfsdm0_awsr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM0_AWCFR register."]
    #[inline] pub fn dfsdm0_awcfr_mut(&self) -> *mut Dfsdm0Awcfr { 
        (self.0 + 0x12c) as *mut Dfsdm0Awcfr
    }

    #[doc="Get the *const pointer for the DFSDM0_AWCFR register."]
    #[inline] pub fn dfsdm0_awcfr_ptr(&self) -> *const Dfsdm0Awcfr { 
           self.dfsdm0_awcfr_mut()
    }

    #[doc="Read the DFSDM0_AWCFR register."]
    #[inline] pub fn dfsdm0_awcfr(&self) -> Dfsdm0Awcfr { 
        unsafe {
            read_volatile(self.dfsdm0_awcfr_ptr())
        }
    }

    #[doc="Write the DFSDM0_AWCFR register."]
    #[inline] pub fn set_dfsdm0_awcfr<F: FnOnce(Dfsdm0Awcfr) -> Dfsdm0Awcfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_awcfr_mut(), f(Dfsdm0Awcfr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM0_AWCFR register."]
    #[inline] pub fn with_dfsdm0_awcfr<F: FnOnce(Dfsdm0Awcfr) -> Dfsdm0Awcfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm0_awcfr_mut(), f(self.dfsdm0_awcfr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM0_EXMAX register."]
    #[inline] pub fn dfsdm0_exmax_mut(&self) -> *mut Dfsdm0Exmax { 
        (self.0 + 0x130) as *mut Dfsdm0Exmax
    }

    #[doc="Get the *const pointer for the DFSDM0_EXMAX register."]
    #[inline] pub fn dfsdm0_exmax_ptr(&self) -> *const Dfsdm0Exmax { 
           self.dfsdm0_exmax_mut()
    }

    #[doc="Read the DFSDM0_EXMAX register."]
    #[inline] pub fn dfsdm0_exmax(&self) -> Dfsdm0Exmax { 
        unsafe {
            read_volatile(self.dfsdm0_exmax_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM0_EXMIN register."]
    #[inline] pub fn dfsdm0_exmin_mut(&self) -> *mut Dfsdm0Exmin { 
        (self.0 + 0x134) as *mut Dfsdm0Exmin
    }

    #[doc="Get the *const pointer for the DFSDM0_EXMIN register."]
    #[inline] pub fn dfsdm0_exmin_ptr(&self) -> *const Dfsdm0Exmin { 
           self.dfsdm0_exmin_mut()
    }

    #[doc="Read the DFSDM0_EXMIN register."]
    #[inline] pub fn dfsdm0_exmin(&self) -> Dfsdm0Exmin { 
        unsafe {
            read_volatile(self.dfsdm0_exmin_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM0_CNVTIMR register."]
    #[inline] pub fn dfsdm0_cnvtimr_mut(&self) -> *mut Dfsdm0Cnvtimr { 
        (self.0 + 0x138) as *mut Dfsdm0Cnvtimr
    }

    #[doc="Get the *const pointer for the DFSDM0_CNVTIMR register."]
    #[inline] pub fn dfsdm0_cnvtimr_ptr(&self) -> *const Dfsdm0Cnvtimr { 
           self.dfsdm0_cnvtimr_mut()
    }

    #[doc="Read the DFSDM0_CNVTIMR register."]
    #[inline] pub fn dfsdm0_cnvtimr(&self) -> Dfsdm0Cnvtimr { 
        unsafe {
            read_volatile(self.dfsdm0_cnvtimr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM1_CR1 register."]
    #[inline] pub fn dfsdm1_cr1_mut(&self) -> *mut Dfsdm1Cr1 { 
        (self.0 + 0x200) as *mut Dfsdm1Cr1
    }

    #[doc="Get the *const pointer for the DFSDM1_CR1 register."]
    #[inline] pub fn dfsdm1_cr1_ptr(&self) -> *const Dfsdm1Cr1 { 
           self.dfsdm1_cr1_mut()
    }

    #[doc="Read the DFSDM1_CR1 register."]
    #[inline] pub fn dfsdm1_cr1(&self) -> Dfsdm1Cr1 { 
        unsafe {
            read_volatile(self.dfsdm1_cr1_ptr())
        }
    }

    #[doc="Write the DFSDM1_CR1 register."]
    #[inline] pub fn set_dfsdm1_cr1<F: FnOnce(Dfsdm1Cr1) -> Dfsdm1Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_cr1_mut(), f(Dfsdm1Cr1(0)));
        }
        self
    }

    #[doc="Modify the DFSDM1_CR1 register."]
    #[inline] pub fn with_dfsdm1_cr1<F: FnOnce(Dfsdm1Cr1) -> Dfsdm1Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_cr1_mut(), f(self.dfsdm1_cr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM1_CR2 register."]
    #[inline] pub fn dfsdm1_cr2_mut(&self) -> *mut Dfsdm1Cr2 { 
        (self.0 + 0x204) as *mut Dfsdm1Cr2
    }

    #[doc="Get the *const pointer for the DFSDM1_CR2 register."]
    #[inline] pub fn dfsdm1_cr2_ptr(&self) -> *const Dfsdm1Cr2 { 
           self.dfsdm1_cr2_mut()
    }

    #[doc="Read the DFSDM1_CR2 register."]
    #[inline] pub fn dfsdm1_cr2(&self) -> Dfsdm1Cr2 { 
        unsafe {
            read_volatile(self.dfsdm1_cr2_ptr())
        }
    }

    #[doc="Write the DFSDM1_CR2 register."]
    #[inline] pub fn set_dfsdm1_cr2<F: FnOnce(Dfsdm1Cr2) -> Dfsdm1Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_cr2_mut(), f(Dfsdm1Cr2(0)));
        }
        self
    }

    #[doc="Modify the DFSDM1_CR2 register."]
    #[inline] pub fn with_dfsdm1_cr2<F: FnOnce(Dfsdm1Cr2) -> Dfsdm1Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_cr2_mut(), f(self.dfsdm1_cr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM1_ISR register."]
    #[inline] pub fn dfsdm1_isr_mut(&self) -> *mut Dfsdm1Isr { 
        (self.0 + 0x208) as *mut Dfsdm1Isr
    }

    #[doc="Get the *const pointer for the DFSDM1_ISR register."]
    #[inline] pub fn dfsdm1_isr_ptr(&self) -> *const Dfsdm1Isr { 
           self.dfsdm1_isr_mut()
    }

    #[doc="Read the DFSDM1_ISR register."]
    #[inline] pub fn dfsdm1_isr(&self) -> Dfsdm1Isr { 
        unsafe {
            read_volatile(self.dfsdm1_isr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM1_ICR register."]
    #[inline] pub fn dfsdm1_icr_mut(&self) -> *mut Dfsdm1Icr { 
        (self.0 + 0x20c) as *mut Dfsdm1Icr
    }

    #[doc="Get the *const pointer for the DFSDM1_ICR register."]
    #[inline] pub fn dfsdm1_icr_ptr(&self) -> *const Dfsdm1Icr { 
           self.dfsdm1_icr_mut()
    }

    #[doc="Read the DFSDM1_ICR register."]
    #[inline] pub fn dfsdm1_icr(&self) -> Dfsdm1Icr { 
        unsafe {
            read_volatile(self.dfsdm1_icr_ptr())
        }
    }

    #[doc="Write the DFSDM1_ICR register."]
    #[inline] pub fn set_dfsdm1_icr<F: FnOnce(Dfsdm1Icr) -> Dfsdm1Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_icr_mut(), f(Dfsdm1Icr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM1_ICR register."]
    #[inline] pub fn with_dfsdm1_icr<F: FnOnce(Dfsdm1Icr) -> Dfsdm1Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_icr_mut(), f(self.dfsdm1_icr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM1_JCHGR register."]
    #[inline] pub fn dfsdm1_jchgr_mut(&self) -> *mut Dfsdm1Jchgr { 
        (self.0 + 0x210) as *mut Dfsdm1Jchgr
    }

    #[doc="Get the *const pointer for the DFSDM1_JCHGR register."]
    #[inline] pub fn dfsdm1_jchgr_ptr(&self) -> *const Dfsdm1Jchgr { 
           self.dfsdm1_jchgr_mut()
    }

    #[doc="Read the DFSDM1_JCHGR register."]
    #[inline] pub fn dfsdm1_jchgr(&self) -> Dfsdm1Jchgr { 
        unsafe {
            read_volatile(self.dfsdm1_jchgr_ptr())
        }
    }

    #[doc="Write the DFSDM1_JCHGR register."]
    #[inline] pub fn set_dfsdm1_jchgr<F: FnOnce(Dfsdm1Jchgr) -> Dfsdm1Jchgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_jchgr_mut(), f(Dfsdm1Jchgr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM1_JCHGR register."]
    #[inline] pub fn with_dfsdm1_jchgr<F: FnOnce(Dfsdm1Jchgr) -> Dfsdm1Jchgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_jchgr_mut(), f(self.dfsdm1_jchgr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM1_FCR register."]
    #[inline] pub fn dfsdm1_fcr_mut(&self) -> *mut Dfsdm1Fcr { 
        (self.0 + 0x214) as *mut Dfsdm1Fcr
    }

    #[doc="Get the *const pointer for the DFSDM1_FCR register."]
    #[inline] pub fn dfsdm1_fcr_ptr(&self) -> *const Dfsdm1Fcr { 
           self.dfsdm1_fcr_mut()
    }

    #[doc="Read the DFSDM1_FCR register."]
    #[inline] pub fn dfsdm1_fcr(&self) -> Dfsdm1Fcr { 
        unsafe {
            read_volatile(self.dfsdm1_fcr_ptr())
        }
    }

    #[doc="Write the DFSDM1_FCR register."]
    #[inline] pub fn set_dfsdm1_fcr<F: FnOnce(Dfsdm1Fcr) -> Dfsdm1Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_fcr_mut(), f(Dfsdm1Fcr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM1_FCR register."]
    #[inline] pub fn with_dfsdm1_fcr<F: FnOnce(Dfsdm1Fcr) -> Dfsdm1Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_fcr_mut(), f(self.dfsdm1_fcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM1_JDATAR register."]
    #[inline] pub fn dfsdm1_jdatar_mut(&self) -> *mut Dfsdm1Jdatar { 
        (self.0 + 0x218) as *mut Dfsdm1Jdatar
    }

    #[doc="Get the *const pointer for the DFSDM1_JDATAR register."]
    #[inline] pub fn dfsdm1_jdatar_ptr(&self) -> *const Dfsdm1Jdatar { 
           self.dfsdm1_jdatar_mut()
    }

    #[doc="Read the DFSDM1_JDATAR register."]
    #[inline] pub fn dfsdm1_jdatar(&self) -> Dfsdm1Jdatar { 
        unsafe {
            read_volatile(self.dfsdm1_jdatar_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM1_RDATAR register."]
    #[inline] pub fn dfsdm1_rdatar_mut(&self) -> *mut Dfsdm1Rdatar { 
        (self.0 + 0x21c) as *mut Dfsdm1Rdatar
    }

    #[doc="Get the *const pointer for the DFSDM1_RDATAR register."]
    #[inline] pub fn dfsdm1_rdatar_ptr(&self) -> *const Dfsdm1Rdatar { 
           self.dfsdm1_rdatar_mut()
    }

    #[doc="Read the DFSDM1_RDATAR register."]
    #[inline] pub fn dfsdm1_rdatar(&self) -> Dfsdm1Rdatar { 
        unsafe {
            read_volatile(self.dfsdm1_rdatar_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM1_AWHTR register."]
    #[inline] pub fn dfsdm1_awhtr_mut(&self) -> *mut Dfsdm1Awhtr { 
        (self.0 + 0x220) as *mut Dfsdm1Awhtr
    }

    #[doc="Get the *const pointer for the DFSDM1_AWHTR register."]
    #[inline] pub fn dfsdm1_awhtr_ptr(&self) -> *const Dfsdm1Awhtr { 
           self.dfsdm1_awhtr_mut()
    }

    #[doc="Read the DFSDM1_AWHTR register."]
    #[inline] pub fn dfsdm1_awhtr(&self) -> Dfsdm1Awhtr { 
        unsafe {
            read_volatile(self.dfsdm1_awhtr_ptr())
        }
    }

    #[doc="Write the DFSDM1_AWHTR register."]
    #[inline] pub fn set_dfsdm1_awhtr<F: FnOnce(Dfsdm1Awhtr) -> Dfsdm1Awhtr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_awhtr_mut(), f(Dfsdm1Awhtr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM1_AWHTR register."]
    #[inline] pub fn with_dfsdm1_awhtr<F: FnOnce(Dfsdm1Awhtr) -> Dfsdm1Awhtr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_awhtr_mut(), f(self.dfsdm1_awhtr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM1_AWLTR register."]
    #[inline] pub fn dfsdm1_awltr_mut(&self) -> *mut Dfsdm1Awltr { 
        (self.0 + 0x224) as *mut Dfsdm1Awltr
    }

    #[doc="Get the *const pointer for the DFSDM1_AWLTR register."]
    #[inline] pub fn dfsdm1_awltr_ptr(&self) -> *const Dfsdm1Awltr { 
           self.dfsdm1_awltr_mut()
    }

    #[doc="Read the DFSDM1_AWLTR register."]
    #[inline] pub fn dfsdm1_awltr(&self) -> Dfsdm1Awltr { 
        unsafe {
            read_volatile(self.dfsdm1_awltr_ptr())
        }
    }

    #[doc="Write the DFSDM1_AWLTR register."]
    #[inline] pub fn set_dfsdm1_awltr<F: FnOnce(Dfsdm1Awltr) -> Dfsdm1Awltr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_awltr_mut(), f(Dfsdm1Awltr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM1_AWLTR register."]
    #[inline] pub fn with_dfsdm1_awltr<F: FnOnce(Dfsdm1Awltr) -> Dfsdm1Awltr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_awltr_mut(), f(self.dfsdm1_awltr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM1_AWSR register."]
    #[inline] pub fn dfsdm1_awsr_mut(&self) -> *mut Dfsdm1Awsr { 
        (self.0 + 0x228) as *mut Dfsdm1Awsr
    }

    #[doc="Get the *const pointer for the DFSDM1_AWSR register."]
    #[inline] pub fn dfsdm1_awsr_ptr(&self) -> *const Dfsdm1Awsr { 
           self.dfsdm1_awsr_mut()
    }

    #[doc="Read the DFSDM1_AWSR register."]
    #[inline] pub fn dfsdm1_awsr(&self) -> Dfsdm1Awsr { 
        unsafe {
            read_volatile(self.dfsdm1_awsr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM1_AWCFR register."]
    #[inline] pub fn dfsdm1_awcfr_mut(&self) -> *mut Dfsdm1Awcfr { 
        (self.0 + 0x22c) as *mut Dfsdm1Awcfr
    }

    #[doc="Get the *const pointer for the DFSDM1_AWCFR register."]
    #[inline] pub fn dfsdm1_awcfr_ptr(&self) -> *const Dfsdm1Awcfr { 
           self.dfsdm1_awcfr_mut()
    }

    #[doc="Read the DFSDM1_AWCFR register."]
    #[inline] pub fn dfsdm1_awcfr(&self) -> Dfsdm1Awcfr { 
        unsafe {
            read_volatile(self.dfsdm1_awcfr_ptr())
        }
    }

    #[doc="Write the DFSDM1_AWCFR register."]
    #[inline] pub fn set_dfsdm1_awcfr<F: FnOnce(Dfsdm1Awcfr) -> Dfsdm1Awcfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_awcfr_mut(), f(Dfsdm1Awcfr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM1_AWCFR register."]
    #[inline] pub fn with_dfsdm1_awcfr<F: FnOnce(Dfsdm1Awcfr) -> Dfsdm1Awcfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm1_awcfr_mut(), f(self.dfsdm1_awcfr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM1_EXMAX register."]
    #[inline] pub fn dfsdm1_exmax_mut(&self) -> *mut Dfsdm1Exmax { 
        (self.0 + 0x230) as *mut Dfsdm1Exmax
    }

    #[doc="Get the *const pointer for the DFSDM1_EXMAX register."]
    #[inline] pub fn dfsdm1_exmax_ptr(&self) -> *const Dfsdm1Exmax { 
           self.dfsdm1_exmax_mut()
    }

    #[doc="Read the DFSDM1_EXMAX register."]
    #[inline] pub fn dfsdm1_exmax(&self) -> Dfsdm1Exmax { 
        unsafe {
            read_volatile(self.dfsdm1_exmax_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM1_EXMIN register."]
    #[inline] pub fn dfsdm1_exmin_mut(&self) -> *mut Dfsdm1Exmin { 
        (self.0 + 0x234) as *mut Dfsdm1Exmin
    }

    #[doc="Get the *const pointer for the DFSDM1_EXMIN register."]
    #[inline] pub fn dfsdm1_exmin_ptr(&self) -> *const Dfsdm1Exmin { 
           self.dfsdm1_exmin_mut()
    }

    #[doc="Read the DFSDM1_EXMIN register."]
    #[inline] pub fn dfsdm1_exmin(&self) -> Dfsdm1Exmin { 
        unsafe {
            read_volatile(self.dfsdm1_exmin_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM1_CNVTIMR register."]
    #[inline] pub fn dfsdm1_cnvtimr_mut(&self) -> *mut Dfsdm1Cnvtimr { 
        (self.0 + 0x238) as *mut Dfsdm1Cnvtimr
    }

    #[doc="Get the *const pointer for the DFSDM1_CNVTIMR register."]
    #[inline] pub fn dfsdm1_cnvtimr_ptr(&self) -> *const Dfsdm1Cnvtimr { 
           self.dfsdm1_cnvtimr_mut()
    }

    #[doc="Read the DFSDM1_CNVTIMR register."]
    #[inline] pub fn dfsdm1_cnvtimr(&self) -> Dfsdm1Cnvtimr { 
        unsafe {
            read_volatile(self.dfsdm1_cnvtimr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM2_CR1 register."]
    #[inline] pub fn dfsdm2_cr1_mut(&self) -> *mut Dfsdm2Cr1 { 
        (self.0 + 0x300) as *mut Dfsdm2Cr1
    }

    #[doc="Get the *const pointer for the DFSDM2_CR1 register."]
    #[inline] pub fn dfsdm2_cr1_ptr(&self) -> *const Dfsdm2Cr1 { 
           self.dfsdm2_cr1_mut()
    }

    #[doc="Read the DFSDM2_CR1 register."]
    #[inline] pub fn dfsdm2_cr1(&self) -> Dfsdm2Cr1 { 
        unsafe {
            read_volatile(self.dfsdm2_cr1_ptr())
        }
    }

    #[doc="Write the DFSDM2_CR1 register."]
    #[inline] pub fn set_dfsdm2_cr1<F: FnOnce(Dfsdm2Cr1) -> Dfsdm2Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_cr1_mut(), f(Dfsdm2Cr1(0)));
        }
        self
    }

    #[doc="Modify the DFSDM2_CR1 register."]
    #[inline] pub fn with_dfsdm2_cr1<F: FnOnce(Dfsdm2Cr1) -> Dfsdm2Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_cr1_mut(), f(self.dfsdm2_cr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM2_CR2 register."]
    #[inline] pub fn dfsdm2_cr2_mut(&self) -> *mut Dfsdm2Cr2 { 
        (self.0 + 0x304) as *mut Dfsdm2Cr2
    }

    #[doc="Get the *const pointer for the DFSDM2_CR2 register."]
    #[inline] pub fn dfsdm2_cr2_ptr(&self) -> *const Dfsdm2Cr2 { 
           self.dfsdm2_cr2_mut()
    }

    #[doc="Read the DFSDM2_CR2 register."]
    #[inline] pub fn dfsdm2_cr2(&self) -> Dfsdm2Cr2 { 
        unsafe {
            read_volatile(self.dfsdm2_cr2_ptr())
        }
    }

    #[doc="Write the DFSDM2_CR2 register."]
    #[inline] pub fn set_dfsdm2_cr2<F: FnOnce(Dfsdm2Cr2) -> Dfsdm2Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_cr2_mut(), f(Dfsdm2Cr2(0)));
        }
        self
    }

    #[doc="Modify the DFSDM2_CR2 register."]
    #[inline] pub fn with_dfsdm2_cr2<F: FnOnce(Dfsdm2Cr2) -> Dfsdm2Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_cr2_mut(), f(self.dfsdm2_cr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM2_ISR register."]
    #[inline] pub fn dfsdm2_isr_mut(&self) -> *mut Dfsdm2Isr { 
        (self.0 + 0x308) as *mut Dfsdm2Isr
    }

    #[doc="Get the *const pointer for the DFSDM2_ISR register."]
    #[inline] pub fn dfsdm2_isr_ptr(&self) -> *const Dfsdm2Isr { 
           self.dfsdm2_isr_mut()
    }

    #[doc="Read the DFSDM2_ISR register."]
    #[inline] pub fn dfsdm2_isr(&self) -> Dfsdm2Isr { 
        unsafe {
            read_volatile(self.dfsdm2_isr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM2_ICR register."]
    #[inline] pub fn dfsdm2_icr_mut(&self) -> *mut Dfsdm2Icr { 
        (self.0 + 0x30c) as *mut Dfsdm2Icr
    }

    #[doc="Get the *const pointer for the DFSDM2_ICR register."]
    #[inline] pub fn dfsdm2_icr_ptr(&self) -> *const Dfsdm2Icr { 
           self.dfsdm2_icr_mut()
    }

    #[doc="Read the DFSDM2_ICR register."]
    #[inline] pub fn dfsdm2_icr(&self) -> Dfsdm2Icr { 
        unsafe {
            read_volatile(self.dfsdm2_icr_ptr())
        }
    }

    #[doc="Write the DFSDM2_ICR register."]
    #[inline] pub fn set_dfsdm2_icr<F: FnOnce(Dfsdm2Icr) -> Dfsdm2Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_icr_mut(), f(Dfsdm2Icr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM2_ICR register."]
    #[inline] pub fn with_dfsdm2_icr<F: FnOnce(Dfsdm2Icr) -> Dfsdm2Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_icr_mut(), f(self.dfsdm2_icr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM2_JCHGR register."]
    #[inline] pub fn dfsdm2_jchgr_mut(&self) -> *mut Dfsdm2Jchgr { 
        (self.0 + 0x310) as *mut Dfsdm2Jchgr
    }

    #[doc="Get the *const pointer for the DFSDM2_JCHGR register."]
    #[inline] pub fn dfsdm2_jchgr_ptr(&self) -> *const Dfsdm2Jchgr { 
           self.dfsdm2_jchgr_mut()
    }

    #[doc="Read the DFSDM2_JCHGR register."]
    #[inline] pub fn dfsdm2_jchgr(&self) -> Dfsdm2Jchgr { 
        unsafe {
            read_volatile(self.dfsdm2_jchgr_ptr())
        }
    }

    #[doc="Write the DFSDM2_JCHGR register."]
    #[inline] pub fn set_dfsdm2_jchgr<F: FnOnce(Dfsdm2Jchgr) -> Dfsdm2Jchgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_jchgr_mut(), f(Dfsdm2Jchgr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM2_JCHGR register."]
    #[inline] pub fn with_dfsdm2_jchgr<F: FnOnce(Dfsdm2Jchgr) -> Dfsdm2Jchgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_jchgr_mut(), f(self.dfsdm2_jchgr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM2_FCR register."]
    #[inline] pub fn dfsdm2_fcr_mut(&self) -> *mut Dfsdm2Fcr { 
        (self.0 + 0x314) as *mut Dfsdm2Fcr
    }

    #[doc="Get the *const pointer for the DFSDM2_FCR register."]
    #[inline] pub fn dfsdm2_fcr_ptr(&self) -> *const Dfsdm2Fcr { 
           self.dfsdm2_fcr_mut()
    }

    #[doc="Read the DFSDM2_FCR register."]
    #[inline] pub fn dfsdm2_fcr(&self) -> Dfsdm2Fcr { 
        unsafe {
            read_volatile(self.dfsdm2_fcr_ptr())
        }
    }

    #[doc="Write the DFSDM2_FCR register."]
    #[inline] pub fn set_dfsdm2_fcr<F: FnOnce(Dfsdm2Fcr) -> Dfsdm2Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_fcr_mut(), f(Dfsdm2Fcr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM2_FCR register."]
    #[inline] pub fn with_dfsdm2_fcr<F: FnOnce(Dfsdm2Fcr) -> Dfsdm2Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_fcr_mut(), f(self.dfsdm2_fcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM2_JDATAR register."]
    #[inline] pub fn dfsdm2_jdatar_mut(&self) -> *mut Dfsdm2Jdatar { 
        (self.0 + 0x318) as *mut Dfsdm2Jdatar
    }

    #[doc="Get the *const pointer for the DFSDM2_JDATAR register."]
    #[inline] pub fn dfsdm2_jdatar_ptr(&self) -> *const Dfsdm2Jdatar { 
           self.dfsdm2_jdatar_mut()
    }

    #[doc="Read the DFSDM2_JDATAR register."]
    #[inline] pub fn dfsdm2_jdatar(&self) -> Dfsdm2Jdatar { 
        unsafe {
            read_volatile(self.dfsdm2_jdatar_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM2_RDATAR register."]
    #[inline] pub fn dfsdm2_rdatar_mut(&self) -> *mut Dfsdm2Rdatar { 
        (self.0 + 0x31c) as *mut Dfsdm2Rdatar
    }

    #[doc="Get the *const pointer for the DFSDM2_RDATAR register."]
    #[inline] pub fn dfsdm2_rdatar_ptr(&self) -> *const Dfsdm2Rdatar { 
           self.dfsdm2_rdatar_mut()
    }

    #[doc="Read the DFSDM2_RDATAR register."]
    #[inline] pub fn dfsdm2_rdatar(&self) -> Dfsdm2Rdatar { 
        unsafe {
            read_volatile(self.dfsdm2_rdatar_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM2_AWHTR register."]
    #[inline] pub fn dfsdm2_awhtr_mut(&self) -> *mut Dfsdm2Awhtr { 
        (self.0 + 0x320) as *mut Dfsdm2Awhtr
    }

    #[doc="Get the *const pointer for the DFSDM2_AWHTR register."]
    #[inline] pub fn dfsdm2_awhtr_ptr(&self) -> *const Dfsdm2Awhtr { 
           self.dfsdm2_awhtr_mut()
    }

    #[doc="Read the DFSDM2_AWHTR register."]
    #[inline] pub fn dfsdm2_awhtr(&self) -> Dfsdm2Awhtr { 
        unsafe {
            read_volatile(self.dfsdm2_awhtr_ptr())
        }
    }

    #[doc="Write the DFSDM2_AWHTR register."]
    #[inline] pub fn set_dfsdm2_awhtr<F: FnOnce(Dfsdm2Awhtr) -> Dfsdm2Awhtr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_awhtr_mut(), f(Dfsdm2Awhtr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM2_AWHTR register."]
    #[inline] pub fn with_dfsdm2_awhtr<F: FnOnce(Dfsdm2Awhtr) -> Dfsdm2Awhtr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_awhtr_mut(), f(self.dfsdm2_awhtr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM2_AWLTR register."]
    #[inline] pub fn dfsdm2_awltr_mut(&self) -> *mut Dfsdm2Awltr { 
        (self.0 + 0x324) as *mut Dfsdm2Awltr
    }

    #[doc="Get the *const pointer for the DFSDM2_AWLTR register."]
    #[inline] pub fn dfsdm2_awltr_ptr(&self) -> *const Dfsdm2Awltr { 
           self.dfsdm2_awltr_mut()
    }

    #[doc="Read the DFSDM2_AWLTR register."]
    #[inline] pub fn dfsdm2_awltr(&self) -> Dfsdm2Awltr { 
        unsafe {
            read_volatile(self.dfsdm2_awltr_ptr())
        }
    }

    #[doc="Write the DFSDM2_AWLTR register."]
    #[inline] pub fn set_dfsdm2_awltr<F: FnOnce(Dfsdm2Awltr) -> Dfsdm2Awltr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_awltr_mut(), f(Dfsdm2Awltr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM2_AWLTR register."]
    #[inline] pub fn with_dfsdm2_awltr<F: FnOnce(Dfsdm2Awltr) -> Dfsdm2Awltr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_awltr_mut(), f(self.dfsdm2_awltr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM2_AWSR register."]
    #[inline] pub fn dfsdm2_awsr_mut(&self) -> *mut Dfsdm2Awsr { 
        (self.0 + 0x328) as *mut Dfsdm2Awsr
    }

    #[doc="Get the *const pointer for the DFSDM2_AWSR register."]
    #[inline] pub fn dfsdm2_awsr_ptr(&self) -> *const Dfsdm2Awsr { 
           self.dfsdm2_awsr_mut()
    }

    #[doc="Read the DFSDM2_AWSR register."]
    #[inline] pub fn dfsdm2_awsr(&self) -> Dfsdm2Awsr { 
        unsafe {
            read_volatile(self.dfsdm2_awsr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM2_AWCFR register."]
    #[inline] pub fn dfsdm2_awcfr_mut(&self) -> *mut Dfsdm2Awcfr { 
        (self.0 + 0x32c) as *mut Dfsdm2Awcfr
    }

    #[doc="Get the *const pointer for the DFSDM2_AWCFR register."]
    #[inline] pub fn dfsdm2_awcfr_ptr(&self) -> *const Dfsdm2Awcfr { 
           self.dfsdm2_awcfr_mut()
    }

    #[doc="Read the DFSDM2_AWCFR register."]
    #[inline] pub fn dfsdm2_awcfr(&self) -> Dfsdm2Awcfr { 
        unsafe {
            read_volatile(self.dfsdm2_awcfr_ptr())
        }
    }

    #[doc="Write the DFSDM2_AWCFR register."]
    #[inline] pub fn set_dfsdm2_awcfr<F: FnOnce(Dfsdm2Awcfr) -> Dfsdm2Awcfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_awcfr_mut(), f(Dfsdm2Awcfr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM2_AWCFR register."]
    #[inline] pub fn with_dfsdm2_awcfr<F: FnOnce(Dfsdm2Awcfr) -> Dfsdm2Awcfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm2_awcfr_mut(), f(self.dfsdm2_awcfr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM2_EXMAX register."]
    #[inline] pub fn dfsdm2_exmax_mut(&self) -> *mut Dfsdm2Exmax { 
        (self.0 + 0x330) as *mut Dfsdm2Exmax
    }

    #[doc="Get the *const pointer for the DFSDM2_EXMAX register."]
    #[inline] pub fn dfsdm2_exmax_ptr(&self) -> *const Dfsdm2Exmax { 
           self.dfsdm2_exmax_mut()
    }

    #[doc="Read the DFSDM2_EXMAX register."]
    #[inline] pub fn dfsdm2_exmax(&self) -> Dfsdm2Exmax { 
        unsafe {
            read_volatile(self.dfsdm2_exmax_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM2_EXMIN register."]
    #[inline] pub fn dfsdm2_exmin_mut(&self) -> *mut Dfsdm2Exmin { 
        (self.0 + 0x334) as *mut Dfsdm2Exmin
    }

    #[doc="Get the *const pointer for the DFSDM2_EXMIN register."]
    #[inline] pub fn dfsdm2_exmin_ptr(&self) -> *const Dfsdm2Exmin { 
           self.dfsdm2_exmin_mut()
    }

    #[doc="Read the DFSDM2_EXMIN register."]
    #[inline] pub fn dfsdm2_exmin(&self) -> Dfsdm2Exmin { 
        unsafe {
            read_volatile(self.dfsdm2_exmin_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM2_CNVTIMR register."]
    #[inline] pub fn dfsdm2_cnvtimr_mut(&self) -> *mut Dfsdm2Cnvtimr { 
        (self.0 + 0x338) as *mut Dfsdm2Cnvtimr
    }

    #[doc="Get the *const pointer for the DFSDM2_CNVTIMR register."]
    #[inline] pub fn dfsdm2_cnvtimr_ptr(&self) -> *const Dfsdm2Cnvtimr { 
           self.dfsdm2_cnvtimr_mut()
    }

    #[doc="Read the DFSDM2_CNVTIMR register."]
    #[inline] pub fn dfsdm2_cnvtimr(&self) -> Dfsdm2Cnvtimr { 
        unsafe {
            read_volatile(self.dfsdm2_cnvtimr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM3_CR1 register."]
    #[inline] pub fn dfsdm3_cr1_mut(&self) -> *mut Dfsdm3Cr1 { 
        (self.0 + 0x400) as *mut Dfsdm3Cr1
    }

    #[doc="Get the *const pointer for the DFSDM3_CR1 register."]
    #[inline] pub fn dfsdm3_cr1_ptr(&self) -> *const Dfsdm3Cr1 { 
           self.dfsdm3_cr1_mut()
    }

    #[doc="Read the DFSDM3_CR1 register."]
    #[inline] pub fn dfsdm3_cr1(&self) -> Dfsdm3Cr1 { 
        unsafe {
            read_volatile(self.dfsdm3_cr1_ptr())
        }
    }

    #[doc="Write the DFSDM3_CR1 register."]
    #[inline] pub fn set_dfsdm3_cr1<F: FnOnce(Dfsdm3Cr1) -> Dfsdm3Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_cr1_mut(), f(Dfsdm3Cr1(0)));
        }
        self
    }

    #[doc="Modify the DFSDM3_CR1 register."]
    #[inline] pub fn with_dfsdm3_cr1<F: FnOnce(Dfsdm3Cr1) -> Dfsdm3Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_cr1_mut(), f(self.dfsdm3_cr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM3_CR2 register."]
    #[inline] pub fn dfsdm3_cr2_mut(&self) -> *mut Dfsdm3Cr2 { 
        (self.0 + 0x404) as *mut Dfsdm3Cr2
    }

    #[doc="Get the *const pointer for the DFSDM3_CR2 register."]
    #[inline] pub fn dfsdm3_cr2_ptr(&self) -> *const Dfsdm3Cr2 { 
           self.dfsdm3_cr2_mut()
    }

    #[doc="Read the DFSDM3_CR2 register."]
    #[inline] pub fn dfsdm3_cr2(&self) -> Dfsdm3Cr2 { 
        unsafe {
            read_volatile(self.dfsdm3_cr2_ptr())
        }
    }

    #[doc="Write the DFSDM3_CR2 register."]
    #[inline] pub fn set_dfsdm3_cr2<F: FnOnce(Dfsdm3Cr2) -> Dfsdm3Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_cr2_mut(), f(Dfsdm3Cr2(0)));
        }
        self
    }

    #[doc="Modify the DFSDM3_CR2 register."]
    #[inline] pub fn with_dfsdm3_cr2<F: FnOnce(Dfsdm3Cr2) -> Dfsdm3Cr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_cr2_mut(), f(self.dfsdm3_cr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM3_ISR register."]
    #[inline] pub fn dfsdm3_isr_mut(&self) -> *mut Dfsdm3Isr { 
        (self.0 + 0x408) as *mut Dfsdm3Isr
    }

    #[doc="Get the *const pointer for the DFSDM3_ISR register."]
    #[inline] pub fn dfsdm3_isr_ptr(&self) -> *const Dfsdm3Isr { 
           self.dfsdm3_isr_mut()
    }

    #[doc="Read the DFSDM3_ISR register."]
    #[inline] pub fn dfsdm3_isr(&self) -> Dfsdm3Isr { 
        unsafe {
            read_volatile(self.dfsdm3_isr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM3_ICR register."]
    #[inline] pub fn dfsdm3_icr_mut(&self) -> *mut Dfsdm3Icr { 
        (self.0 + 0x40c) as *mut Dfsdm3Icr
    }

    #[doc="Get the *const pointer for the DFSDM3_ICR register."]
    #[inline] pub fn dfsdm3_icr_ptr(&self) -> *const Dfsdm3Icr { 
           self.dfsdm3_icr_mut()
    }

    #[doc="Read the DFSDM3_ICR register."]
    #[inline] pub fn dfsdm3_icr(&self) -> Dfsdm3Icr { 
        unsafe {
            read_volatile(self.dfsdm3_icr_ptr())
        }
    }

    #[doc="Write the DFSDM3_ICR register."]
    #[inline] pub fn set_dfsdm3_icr<F: FnOnce(Dfsdm3Icr) -> Dfsdm3Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_icr_mut(), f(Dfsdm3Icr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM3_ICR register."]
    #[inline] pub fn with_dfsdm3_icr<F: FnOnce(Dfsdm3Icr) -> Dfsdm3Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_icr_mut(), f(self.dfsdm3_icr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM3_JCHGR register."]
    #[inline] pub fn dfsdm3_jchgr_mut(&self) -> *mut Dfsdm3Jchgr { 
        (self.0 + 0x410) as *mut Dfsdm3Jchgr
    }

    #[doc="Get the *const pointer for the DFSDM3_JCHGR register."]
    #[inline] pub fn dfsdm3_jchgr_ptr(&self) -> *const Dfsdm3Jchgr { 
           self.dfsdm3_jchgr_mut()
    }

    #[doc="Read the DFSDM3_JCHGR register."]
    #[inline] pub fn dfsdm3_jchgr(&self) -> Dfsdm3Jchgr { 
        unsafe {
            read_volatile(self.dfsdm3_jchgr_ptr())
        }
    }

    #[doc="Write the DFSDM3_JCHGR register."]
    #[inline] pub fn set_dfsdm3_jchgr<F: FnOnce(Dfsdm3Jchgr) -> Dfsdm3Jchgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_jchgr_mut(), f(Dfsdm3Jchgr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM3_JCHGR register."]
    #[inline] pub fn with_dfsdm3_jchgr<F: FnOnce(Dfsdm3Jchgr) -> Dfsdm3Jchgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_jchgr_mut(), f(self.dfsdm3_jchgr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM3_FCR register."]
    #[inline] pub fn dfsdm3_fcr_mut(&self) -> *mut Dfsdm3Fcr { 
        (self.0 + 0x414) as *mut Dfsdm3Fcr
    }

    #[doc="Get the *const pointer for the DFSDM3_FCR register."]
    #[inline] pub fn dfsdm3_fcr_ptr(&self) -> *const Dfsdm3Fcr { 
           self.dfsdm3_fcr_mut()
    }

    #[doc="Read the DFSDM3_FCR register."]
    #[inline] pub fn dfsdm3_fcr(&self) -> Dfsdm3Fcr { 
        unsafe {
            read_volatile(self.dfsdm3_fcr_ptr())
        }
    }

    #[doc="Write the DFSDM3_FCR register."]
    #[inline] pub fn set_dfsdm3_fcr<F: FnOnce(Dfsdm3Fcr) -> Dfsdm3Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_fcr_mut(), f(Dfsdm3Fcr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM3_FCR register."]
    #[inline] pub fn with_dfsdm3_fcr<F: FnOnce(Dfsdm3Fcr) -> Dfsdm3Fcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_fcr_mut(), f(self.dfsdm3_fcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM3_JDATAR register."]
    #[inline] pub fn dfsdm3_jdatar_mut(&self) -> *mut Dfsdm3Jdatar { 
        (self.0 + 0x418) as *mut Dfsdm3Jdatar
    }

    #[doc="Get the *const pointer for the DFSDM3_JDATAR register."]
    #[inline] pub fn dfsdm3_jdatar_ptr(&self) -> *const Dfsdm3Jdatar { 
           self.dfsdm3_jdatar_mut()
    }

    #[doc="Read the DFSDM3_JDATAR register."]
    #[inline] pub fn dfsdm3_jdatar(&self) -> Dfsdm3Jdatar { 
        unsafe {
            read_volatile(self.dfsdm3_jdatar_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM3_RDATAR register."]
    #[inline] pub fn dfsdm3_rdatar_mut(&self) -> *mut Dfsdm3Rdatar { 
        (self.0 + 0x41c) as *mut Dfsdm3Rdatar
    }

    #[doc="Get the *const pointer for the DFSDM3_RDATAR register."]
    #[inline] pub fn dfsdm3_rdatar_ptr(&self) -> *const Dfsdm3Rdatar { 
           self.dfsdm3_rdatar_mut()
    }

    #[doc="Read the DFSDM3_RDATAR register."]
    #[inline] pub fn dfsdm3_rdatar(&self) -> Dfsdm3Rdatar { 
        unsafe {
            read_volatile(self.dfsdm3_rdatar_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM3_AWHTR register."]
    #[inline] pub fn dfsdm3_awhtr_mut(&self) -> *mut Dfsdm3Awhtr { 
        (self.0 + 0x420) as *mut Dfsdm3Awhtr
    }

    #[doc="Get the *const pointer for the DFSDM3_AWHTR register."]
    #[inline] pub fn dfsdm3_awhtr_ptr(&self) -> *const Dfsdm3Awhtr { 
           self.dfsdm3_awhtr_mut()
    }

    #[doc="Read the DFSDM3_AWHTR register."]
    #[inline] pub fn dfsdm3_awhtr(&self) -> Dfsdm3Awhtr { 
        unsafe {
            read_volatile(self.dfsdm3_awhtr_ptr())
        }
    }

    #[doc="Write the DFSDM3_AWHTR register."]
    #[inline] pub fn set_dfsdm3_awhtr<F: FnOnce(Dfsdm3Awhtr) -> Dfsdm3Awhtr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_awhtr_mut(), f(Dfsdm3Awhtr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM3_AWHTR register."]
    #[inline] pub fn with_dfsdm3_awhtr<F: FnOnce(Dfsdm3Awhtr) -> Dfsdm3Awhtr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_awhtr_mut(), f(self.dfsdm3_awhtr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM3_AWLTR register."]
    #[inline] pub fn dfsdm3_awltr_mut(&self) -> *mut Dfsdm3Awltr { 
        (self.0 + 0x424) as *mut Dfsdm3Awltr
    }

    #[doc="Get the *const pointer for the DFSDM3_AWLTR register."]
    #[inline] pub fn dfsdm3_awltr_ptr(&self) -> *const Dfsdm3Awltr { 
           self.dfsdm3_awltr_mut()
    }

    #[doc="Read the DFSDM3_AWLTR register."]
    #[inline] pub fn dfsdm3_awltr(&self) -> Dfsdm3Awltr { 
        unsafe {
            read_volatile(self.dfsdm3_awltr_ptr())
        }
    }

    #[doc="Write the DFSDM3_AWLTR register."]
    #[inline] pub fn set_dfsdm3_awltr<F: FnOnce(Dfsdm3Awltr) -> Dfsdm3Awltr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_awltr_mut(), f(Dfsdm3Awltr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM3_AWLTR register."]
    #[inline] pub fn with_dfsdm3_awltr<F: FnOnce(Dfsdm3Awltr) -> Dfsdm3Awltr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_awltr_mut(), f(self.dfsdm3_awltr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM3_AWSR register."]
    #[inline] pub fn dfsdm3_awsr_mut(&self) -> *mut Dfsdm3Awsr { 
        (self.0 + 0x428) as *mut Dfsdm3Awsr
    }

    #[doc="Get the *const pointer for the DFSDM3_AWSR register."]
    #[inline] pub fn dfsdm3_awsr_ptr(&self) -> *const Dfsdm3Awsr { 
           self.dfsdm3_awsr_mut()
    }

    #[doc="Read the DFSDM3_AWSR register."]
    #[inline] pub fn dfsdm3_awsr(&self) -> Dfsdm3Awsr { 
        unsafe {
            read_volatile(self.dfsdm3_awsr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM3_AWCFR register."]
    #[inline] pub fn dfsdm3_awcfr_mut(&self) -> *mut Dfsdm3Awcfr { 
        (self.0 + 0x42c) as *mut Dfsdm3Awcfr
    }

    #[doc="Get the *const pointer for the DFSDM3_AWCFR register."]
    #[inline] pub fn dfsdm3_awcfr_ptr(&self) -> *const Dfsdm3Awcfr { 
           self.dfsdm3_awcfr_mut()
    }

    #[doc="Read the DFSDM3_AWCFR register."]
    #[inline] pub fn dfsdm3_awcfr(&self) -> Dfsdm3Awcfr { 
        unsafe {
            read_volatile(self.dfsdm3_awcfr_ptr())
        }
    }

    #[doc="Write the DFSDM3_AWCFR register."]
    #[inline] pub fn set_dfsdm3_awcfr<F: FnOnce(Dfsdm3Awcfr) -> Dfsdm3Awcfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_awcfr_mut(), f(Dfsdm3Awcfr(0)));
        }
        self
    }

    #[doc="Modify the DFSDM3_AWCFR register."]
    #[inline] pub fn with_dfsdm3_awcfr<F: FnOnce(Dfsdm3Awcfr) -> Dfsdm3Awcfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dfsdm3_awcfr_mut(), f(self.dfsdm3_awcfr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DFSDM3_EXMAX register."]
    #[inline] pub fn dfsdm3_exmax_mut(&self) -> *mut Dfsdm3Exmax { 
        (self.0 + 0x430) as *mut Dfsdm3Exmax
    }

    #[doc="Get the *const pointer for the DFSDM3_EXMAX register."]
    #[inline] pub fn dfsdm3_exmax_ptr(&self) -> *const Dfsdm3Exmax { 
           self.dfsdm3_exmax_mut()
    }

    #[doc="Read the DFSDM3_EXMAX register."]
    #[inline] pub fn dfsdm3_exmax(&self) -> Dfsdm3Exmax { 
        unsafe {
            read_volatile(self.dfsdm3_exmax_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM3_EXMIN register."]
    #[inline] pub fn dfsdm3_exmin_mut(&self) -> *mut Dfsdm3Exmin { 
        (self.0 + 0x434) as *mut Dfsdm3Exmin
    }

    #[doc="Get the *const pointer for the DFSDM3_EXMIN register."]
    #[inline] pub fn dfsdm3_exmin_ptr(&self) -> *const Dfsdm3Exmin { 
           self.dfsdm3_exmin_mut()
    }

    #[doc="Read the DFSDM3_EXMIN register."]
    #[inline] pub fn dfsdm3_exmin(&self) -> Dfsdm3Exmin { 
        unsafe {
            read_volatile(self.dfsdm3_exmin_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DFSDM3_CNVTIMR register."]
    #[inline] pub fn dfsdm3_cnvtimr_mut(&self) -> *mut Dfsdm3Cnvtimr { 
        (self.0 + 0x438) as *mut Dfsdm3Cnvtimr
    }

    #[doc="Get the *const pointer for the DFSDM3_CNVTIMR register."]
    #[inline] pub fn dfsdm3_cnvtimr_ptr(&self) -> *const Dfsdm3Cnvtimr { 
           self.dfsdm3_cnvtimr_mut()
    }

    #[doc="Read the DFSDM3_CNVTIMR register."]
    #[inline] pub fn dfsdm3_cnvtimr(&self) -> Dfsdm3Cnvtimr { 
        unsafe {
            read_volatile(self.dfsdm3_cnvtimr_ptr())
        }
    }

}

#[doc="channel configuration y register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg0r1(pub u32);
impl Chcfg0r1 {
    #[doc="DFSDMEN"]
    #[inline] pub fn dfsdmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if DFSDMEN != 0"]
    #[inline] pub fn test_dfsdmen(&self) -> bool {
        self.dfsdmen() != 0
    }

    #[doc="Sets the DFSDMEN field."]
    #[inline] pub fn set_dfsdmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="CKOUTSRC"]
    #[inline] pub fn ckoutsrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CKOUTSRC != 0"]
    #[inline] pub fn test_ckoutsrc(&self) -> bool {
        self.ckoutsrc() != 0
    }

    #[doc="Sets the CKOUTSRC field."]
    #[inline] pub fn set_ckoutsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="CKOUTDIV"]
    #[inline] pub fn ckoutdiv(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CKOUTDIV != 0"]
    #[inline] pub fn test_ckoutdiv(&self) -> bool {
        self.ckoutdiv() != 0
    }

    #[doc="Sets the CKOUTDIV field."]
    #[inline] pub fn set_ckoutdiv<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATPACK"]
    #[inline] pub fn datpack(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if DATPACK != 0"]
    #[inline] pub fn test_datpack(&self) -> bool {
        self.datpack() != 0
    }

    #[doc="Sets the DATPACK field."]
    #[inline] pub fn set_datpack<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DATMPX"]
    #[inline] pub fn datmpx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if DATMPX != 0"]
    #[inline] pub fn test_datmpx(&self) -> bool {
        self.datmpx() != 0
    }

    #[doc="Sets the DATMPX field."]
    #[inline] pub fn set_datmpx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CHINSEL"]
    #[inline] pub fn chinsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHINSEL != 0"]
    #[inline] pub fn test_chinsel(&self) -> bool {
        self.chinsel() != 0
    }

    #[doc="Sets the CHINSEL field."]
    #[inline] pub fn set_chinsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CHEN"]
    #[inline] pub fn chen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHEN != 0"]
    #[inline] pub fn test_chen(&self) -> bool {
        self.chen() != 0
    }

    #[doc="Sets the CHEN field."]
    #[inline] pub fn set_chen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CKABEN"]
    #[inline] pub fn ckaben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABEN != 0"]
    #[inline] pub fn test_ckaben(&self) -> bool {
        self.ckaben() != 0
    }

    #[doc="Sets the CKABEN field."]
    #[inline] pub fn set_ckaben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SCDEN"]
    #[inline] pub fn scden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDEN != 0"]
    #[inline] pub fn test_scden(&self) -> bool {
        self.scden() != 0
    }

    #[doc="Sets the SCDEN field."]
    #[inline] pub fn set_scden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPICKSEL"]
    #[inline] pub fn spicksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPICKSEL != 0"]
    #[inline] pub fn test_spicksel(&self) -> bool {
        self.spicksel() != 0
    }

    #[doc="Sets the SPICKSEL field."]
    #[inline] pub fn set_spicksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SITP"]
    #[inline] pub fn sitp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SITP != 0"]
    #[inline] pub fn test_sitp(&self) -> bool {
        self.sitp() != 0
    }

    #[doc="Sets the SITP field."]
    #[inline] pub fn set_sitp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chcfg0r1 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg0r1(other)
    }
}

impl ::core::fmt::Display for Chcfg0r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg0r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dfsdmen() != 0 { try!(write!(f, " dfsdmen"))}
        if self.ckoutsrc() != 0 { try!(write!(f, " ckoutsrc"))}
        if self.ckoutdiv() != 0 { try!(write!(f, " ckoutdiv=0x{:x}", self.ckoutdiv()))}
        if self.datpack() != 0 { try!(write!(f, " datpack=0x{:x}", self.datpack()))}
        if self.datmpx() != 0 { try!(write!(f, " datmpx=0x{:x}", self.datmpx()))}
        if self.chinsel() != 0 { try!(write!(f, " chinsel"))}
        if self.chen() != 0 { try!(write!(f, " chen"))}
        if self.ckaben() != 0 { try!(write!(f, " ckaben"))}
        if self.scden() != 0 { try!(write!(f, " scden"))}
        if self.spicksel() != 0 { try!(write!(f, " spicksel=0x{:x}", self.spicksel()))}
        if self.sitp() != 0 { try!(write!(f, " sitp=0x{:x}", self.sitp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel configuration y register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg0r2(pub u32);
impl Chcfg0r2 {
    #[doc="OFFSET"]
    #[inline] pub fn offset(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if OFFSET != 0"]
    #[inline] pub fn test_offset(&self) -> bool {
        self.offset() != 0
    }

    #[doc="Sets the OFFSET field."]
    #[inline] pub fn set_offset<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DTRBS"]
    #[inline] pub fn dtrbs(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if DTRBS != 0"]
    #[inline] pub fn test_dtrbs(&self) -> bool {
        self.dtrbs() != 0
    }

    #[doc="Sets the DTRBS field."]
    #[inline] pub fn set_dtrbs<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Chcfg0r2 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg0r2(other)
    }
}

impl ::core::fmt::Display for Chcfg0r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg0r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset() != 0 { try!(write!(f, " offset=0x{:x}", self.offset()))}
        if self.dtrbs() != 0 { try!(write!(f, " dtrbs=0x{:x}", self.dtrbs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog and short-circuit detector register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awscd0r(pub u32);
impl Awscd0r {
    #[doc="AWFORD"]
    #[inline] pub fn awford(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if AWFORD != 0"]
    #[inline] pub fn test_awford(&self) -> bool {
        self.awford() != 0
    }

    #[doc="Sets the AWFORD field."]
    #[inline] pub fn set_awford<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="AWFOSR"]
    #[inline] pub fn awfosr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if AWFOSR != 0"]
    #[inline] pub fn test_awfosr(&self) -> bool {
        self.awfosr() != 0
    }

    #[doc="Sets the AWFOSR field."]
    #[inline] pub fn set_awfosr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="BKSCD"]
    #[inline] pub fn bkscd(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if BKSCD != 0"]
    #[inline] pub fn test_bkscd(&self) -> bool {
        self.bkscd() != 0
    }

    #[doc="Sets the BKSCD field."]
    #[inline] pub fn set_bkscd<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SCDT"]
    #[inline] pub fn scdt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCDT != 0"]
    #[inline] pub fn test_scdt(&self) -> bool {
        self.scdt() != 0
    }

    #[doc="Sets the SCDT field."]
    #[inline] pub fn set_scdt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Awscd0r {
    #[inline]
    fn from(other: u32) -> Self {
         Awscd0r(other)
    }
}

impl ::core::fmt::Display for Awscd0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awscd0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awford() != 0 { try!(write!(f, " awford=0x{:x}", self.awford()))}
        if self.awfosr() != 0 { try!(write!(f, " awfosr=0x{:x}", self.awfosr()))}
        if self.bkscd() != 0 { try!(write!(f, " bkscd=0x{:x}", self.bkscd()))}
        if self.scdt() != 0 { try!(write!(f, " scdt=0x{:x}", self.scdt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel watchdog filter data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chwdat0r(pub u32);
impl Chwdat0r {
    #[doc="WDATA"]
    #[inline] pub fn wdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WDATA != 0"]
    #[inline] pub fn test_wdata(&self) -> bool {
        self.wdata() != 0
    }

    #[doc="Sets the WDATA field."]
    #[inline] pub fn set_wdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chwdat0r {
    #[inline]
    fn from(other: u32) -> Self {
         Chwdat0r(other)
    }
}

impl ::core::fmt::Display for Chwdat0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chwdat0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdata() != 0 { try!(write!(f, " wdata=0x{:x}", self.wdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel data input register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chdatin0r(pub u32);
impl Chdatin0r {
    #[doc="INDAT1"]
    #[inline] pub fn indat1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if INDAT1 != 0"]
    #[inline] pub fn test_indat1(&self) -> bool {
        self.indat1() != 0
    }

    #[doc="Sets the INDAT1 field."]
    #[inline] pub fn set_indat1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="INDAT0"]
    #[inline] pub fn indat0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INDAT0 != 0"]
    #[inline] pub fn test_indat0(&self) -> bool {
        self.indat0() != 0
    }

    #[doc="Sets the INDAT0 field."]
    #[inline] pub fn set_indat0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chdatin0r {
    #[inline]
    fn from(other: u32) -> Self {
         Chdatin0r(other)
    }
}

impl ::core::fmt::Display for Chdatin0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chdatin0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.indat1() != 0 { try!(write!(f, " indat1=0x{:x}", self.indat1()))}
        if self.indat0() != 0 { try!(write!(f, " indat0=0x{:x}", self.indat0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG1R1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg1r1(pub u32);
impl Chcfg1r1 {
    #[doc="DATPACK"]
    #[inline] pub fn datpack(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if DATPACK != 0"]
    #[inline] pub fn test_datpack(&self) -> bool {
        self.datpack() != 0
    }

    #[doc="Sets the DATPACK field."]
    #[inline] pub fn set_datpack<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DATMPX"]
    #[inline] pub fn datmpx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if DATMPX != 0"]
    #[inline] pub fn test_datmpx(&self) -> bool {
        self.datmpx() != 0
    }

    #[doc="Sets the DATMPX field."]
    #[inline] pub fn set_datmpx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CHINSEL"]
    #[inline] pub fn chinsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHINSEL != 0"]
    #[inline] pub fn test_chinsel(&self) -> bool {
        self.chinsel() != 0
    }

    #[doc="Sets the CHINSEL field."]
    #[inline] pub fn set_chinsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CHEN"]
    #[inline] pub fn chen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHEN != 0"]
    #[inline] pub fn test_chen(&self) -> bool {
        self.chen() != 0
    }

    #[doc="Sets the CHEN field."]
    #[inline] pub fn set_chen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CKABEN"]
    #[inline] pub fn ckaben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABEN != 0"]
    #[inline] pub fn test_ckaben(&self) -> bool {
        self.ckaben() != 0
    }

    #[doc="Sets the CKABEN field."]
    #[inline] pub fn set_ckaben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SCDEN"]
    #[inline] pub fn scden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDEN != 0"]
    #[inline] pub fn test_scden(&self) -> bool {
        self.scden() != 0
    }

    #[doc="Sets the SCDEN field."]
    #[inline] pub fn set_scden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPICKSEL"]
    #[inline] pub fn spicksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPICKSEL != 0"]
    #[inline] pub fn test_spicksel(&self) -> bool {
        self.spicksel() != 0
    }

    #[doc="Sets the SPICKSEL field."]
    #[inline] pub fn set_spicksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SITP"]
    #[inline] pub fn sitp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SITP != 0"]
    #[inline] pub fn test_sitp(&self) -> bool {
        self.sitp() != 0
    }

    #[doc="Sets the SITP field."]
    #[inline] pub fn set_sitp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chcfg1r1 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg1r1(other)
    }
}

impl ::core::fmt::Display for Chcfg1r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg1r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datpack() != 0 { try!(write!(f, " datpack=0x{:x}", self.datpack()))}
        if self.datmpx() != 0 { try!(write!(f, " datmpx=0x{:x}", self.datmpx()))}
        if self.chinsel() != 0 { try!(write!(f, " chinsel"))}
        if self.chen() != 0 { try!(write!(f, " chen"))}
        if self.ckaben() != 0 { try!(write!(f, " ckaben"))}
        if self.scden() != 0 { try!(write!(f, " scden"))}
        if self.spicksel() != 0 { try!(write!(f, " spicksel=0x{:x}", self.spicksel()))}
        if self.sitp() != 0 { try!(write!(f, " sitp=0x{:x}", self.sitp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG1R2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg1r2(pub u32);
impl Chcfg1r2 {
    #[doc="OFFSET"]
    #[inline] pub fn offset(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if OFFSET != 0"]
    #[inline] pub fn test_offset(&self) -> bool {
        self.offset() != 0
    }

    #[doc="Sets the OFFSET field."]
    #[inline] pub fn set_offset<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DTRBS"]
    #[inline] pub fn dtrbs(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if DTRBS != 0"]
    #[inline] pub fn test_dtrbs(&self) -> bool {
        self.dtrbs() != 0
    }

    #[doc="Sets the DTRBS field."]
    #[inline] pub fn set_dtrbs<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Chcfg1r2 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg1r2(other)
    }
}

impl ::core::fmt::Display for Chcfg1r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg1r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset() != 0 { try!(write!(f, " offset=0x{:x}", self.offset()))}
        if self.dtrbs() != 0 { try!(write!(f, " dtrbs=0x{:x}", self.dtrbs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AWSCD1R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awscd1r(pub u32);
impl Awscd1r {
    #[doc="AWFORD"]
    #[inline] pub fn awford(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if AWFORD != 0"]
    #[inline] pub fn test_awford(&self) -> bool {
        self.awford() != 0
    }

    #[doc="Sets the AWFORD field."]
    #[inline] pub fn set_awford<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="AWFOSR"]
    #[inline] pub fn awfosr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if AWFOSR != 0"]
    #[inline] pub fn test_awfosr(&self) -> bool {
        self.awfosr() != 0
    }

    #[doc="Sets the AWFOSR field."]
    #[inline] pub fn set_awfosr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="BKSCD"]
    #[inline] pub fn bkscd(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if BKSCD != 0"]
    #[inline] pub fn test_bkscd(&self) -> bool {
        self.bkscd() != 0
    }

    #[doc="Sets the BKSCD field."]
    #[inline] pub fn set_bkscd<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SCDT"]
    #[inline] pub fn scdt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCDT != 0"]
    #[inline] pub fn test_scdt(&self) -> bool {
        self.scdt() != 0
    }

    #[doc="Sets the SCDT field."]
    #[inline] pub fn set_scdt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Awscd1r {
    #[inline]
    fn from(other: u32) -> Self {
         Awscd1r(other)
    }
}

impl ::core::fmt::Display for Awscd1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awscd1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awford() != 0 { try!(write!(f, " awford=0x{:x}", self.awford()))}
        if self.awfosr() != 0 { try!(write!(f, " awfosr=0x{:x}", self.awfosr()))}
        if self.bkscd() != 0 { try!(write!(f, " bkscd=0x{:x}", self.bkscd()))}
        if self.scdt() != 0 { try!(write!(f, " scdt=0x{:x}", self.scdt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHWDAT1R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chwdat1r(pub u32);
impl Chwdat1r {
    #[doc="WDATA"]
    #[inline] pub fn wdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WDATA != 0"]
    #[inline] pub fn test_wdata(&self) -> bool {
        self.wdata() != 0
    }

    #[doc="Sets the WDATA field."]
    #[inline] pub fn set_wdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chwdat1r {
    #[inline]
    fn from(other: u32) -> Self {
         Chwdat1r(other)
    }
}

impl ::core::fmt::Display for Chwdat1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chwdat1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdata() != 0 { try!(write!(f, " wdata=0x{:x}", self.wdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHDATIN1R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chdatin1r(pub u32);
impl Chdatin1r {
    #[doc="INDAT1"]
    #[inline] pub fn indat1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if INDAT1 != 0"]
    #[inline] pub fn test_indat1(&self) -> bool {
        self.indat1() != 0
    }

    #[doc="Sets the INDAT1 field."]
    #[inline] pub fn set_indat1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="INDAT0"]
    #[inline] pub fn indat0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INDAT0 != 0"]
    #[inline] pub fn test_indat0(&self) -> bool {
        self.indat0() != 0
    }

    #[doc="Sets the INDAT0 field."]
    #[inline] pub fn set_indat0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chdatin1r {
    #[inline]
    fn from(other: u32) -> Self {
         Chdatin1r(other)
    }
}

impl ::core::fmt::Display for Chdatin1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chdatin1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.indat1() != 0 { try!(write!(f, " indat1=0x{:x}", self.indat1()))}
        if self.indat0() != 0 { try!(write!(f, " indat0=0x{:x}", self.indat0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG2R1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg2r1(pub u32);
impl Chcfg2r1 {
    #[doc="DATPACK"]
    #[inline] pub fn datpack(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if DATPACK != 0"]
    #[inline] pub fn test_datpack(&self) -> bool {
        self.datpack() != 0
    }

    #[doc="Sets the DATPACK field."]
    #[inline] pub fn set_datpack<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DATMPX"]
    #[inline] pub fn datmpx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if DATMPX != 0"]
    #[inline] pub fn test_datmpx(&self) -> bool {
        self.datmpx() != 0
    }

    #[doc="Sets the DATMPX field."]
    #[inline] pub fn set_datmpx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CHINSEL"]
    #[inline] pub fn chinsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHINSEL != 0"]
    #[inline] pub fn test_chinsel(&self) -> bool {
        self.chinsel() != 0
    }

    #[doc="Sets the CHINSEL field."]
    #[inline] pub fn set_chinsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CHEN"]
    #[inline] pub fn chen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHEN != 0"]
    #[inline] pub fn test_chen(&self) -> bool {
        self.chen() != 0
    }

    #[doc="Sets the CHEN field."]
    #[inline] pub fn set_chen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CKABEN"]
    #[inline] pub fn ckaben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABEN != 0"]
    #[inline] pub fn test_ckaben(&self) -> bool {
        self.ckaben() != 0
    }

    #[doc="Sets the CKABEN field."]
    #[inline] pub fn set_ckaben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SCDEN"]
    #[inline] pub fn scden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDEN != 0"]
    #[inline] pub fn test_scden(&self) -> bool {
        self.scden() != 0
    }

    #[doc="Sets the SCDEN field."]
    #[inline] pub fn set_scden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPICKSEL"]
    #[inline] pub fn spicksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPICKSEL != 0"]
    #[inline] pub fn test_spicksel(&self) -> bool {
        self.spicksel() != 0
    }

    #[doc="Sets the SPICKSEL field."]
    #[inline] pub fn set_spicksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SITP"]
    #[inline] pub fn sitp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SITP != 0"]
    #[inline] pub fn test_sitp(&self) -> bool {
        self.sitp() != 0
    }

    #[doc="Sets the SITP field."]
    #[inline] pub fn set_sitp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chcfg2r1 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg2r1(other)
    }
}

impl ::core::fmt::Display for Chcfg2r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg2r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datpack() != 0 { try!(write!(f, " datpack=0x{:x}", self.datpack()))}
        if self.datmpx() != 0 { try!(write!(f, " datmpx=0x{:x}", self.datmpx()))}
        if self.chinsel() != 0 { try!(write!(f, " chinsel"))}
        if self.chen() != 0 { try!(write!(f, " chen"))}
        if self.ckaben() != 0 { try!(write!(f, " ckaben"))}
        if self.scden() != 0 { try!(write!(f, " scden"))}
        if self.spicksel() != 0 { try!(write!(f, " spicksel=0x{:x}", self.spicksel()))}
        if self.sitp() != 0 { try!(write!(f, " sitp=0x{:x}", self.sitp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG2R2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg2r2(pub u32);
impl Chcfg2r2 {
    #[doc="OFFSET"]
    #[inline] pub fn offset(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if OFFSET != 0"]
    #[inline] pub fn test_offset(&self) -> bool {
        self.offset() != 0
    }

    #[doc="Sets the OFFSET field."]
    #[inline] pub fn set_offset<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DTRBS"]
    #[inline] pub fn dtrbs(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if DTRBS != 0"]
    #[inline] pub fn test_dtrbs(&self) -> bool {
        self.dtrbs() != 0
    }

    #[doc="Sets the DTRBS field."]
    #[inline] pub fn set_dtrbs<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Chcfg2r2 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg2r2(other)
    }
}

impl ::core::fmt::Display for Chcfg2r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg2r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset() != 0 { try!(write!(f, " offset=0x{:x}", self.offset()))}
        if self.dtrbs() != 0 { try!(write!(f, " dtrbs=0x{:x}", self.dtrbs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AWSCD2R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awscd2r(pub u32);
impl Awscd2r {
    #[doc="AWFORD"]
    #[inline] pub fn awford(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if AWFORD != 0"]
    #[inline] pub fn test_awford(&self) -> bool {
        self.awford() != 0
    }

    #[doc="Sets the AWFORD field."]
    #[inline] pub fn set_awford<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="AWFOSR"]
    #[inline] pub fn awfosr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if AWFOSR != 0"]
    #[inline] pub fn test_awfosr(&self) -> bool {
        self.awfosr() != 0
    }

    #[doc="Sets the AWFOSR field."]
    #[inline] pub fn set_awfosr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="BKSCD"]
    #[inline] pub fn bkscd(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if BKSCD != 0"]
    #[inline] pub fn test_bkscd(&self) -> bool {
        self.bkscd() != 0
    }

    #[doc="Sets the BKSCD field."]
    #[inline] pub fn set_bkscd<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SCDT"]
    #[inline] pub fn scdt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCDT != 0"]
    #[inline] pub fn test_scdt(&self) -> bool {
        self.scdt() != 0
    }

    #[doc="Sets the SCDT field."]
    #[inline] pub fn set_scdt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Awscd2r {
    #[inline]
    fn from(other: u32) -> Self {
         Awscd2r(other)
    }
}

impl ::core::fmt::Display for Awscd2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awscd2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awford() != 0 { try!(write!(f, " awford=0x{:x}", self.awford()))}
        if self.awfosr() != 0 { try!(write!(f, " awfosr=0x{:x}", self.awfosr()))}
        if self.bkscd() != 0 { try!(write!(f, " bkscd=0x{:x}", self.bkscd()))}
        if self.scdt() != 0 { try!(write!(f, " scdt=0x{:x}", self.scdt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHWDAT2R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chwdat2r(pub u32);
impl Chwdat2r {
    #[doc="WDATA"]
    #[inline] pub fn wdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WDATA != 0"]
    #[inline] pub fn test_wdata(&self) -> bool {
        self.wdata() != 0
    }

    #[doc="Sets the WDATA field."]
    #[inline] pub fn set_wdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chwdat2r {
    #[inline]
    fn from(other: u32) -> Self {
         Chwdat2r(other)
    }
}

impl ::core::fmt::Display for Chwdat2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chwdat2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdata() != 0 { try!(write!(f, " wdata=0x{:x}", self.wdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHDATIN2R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chdatin2r(pub u32);
impl Chdatin2r {
    #[doc="INDAT1"]
    #[inline] pub fn indat1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if INDAT1 != 0"]
    #[inline] pub fn test_indat1(&self) -> bool {
        self.indat1() != 0
    }

    #[doc="Sets the INDAT1 field."]
    #[inline] pub fn set_indat1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="INDAT0"]
    #[inline] pub fn indat0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INDAT0 != 0"]
    #[inline] pub fn test_indat0(&self) -> bool {
        self.indat0() != 0
    }

    #[doc="Sets the INDAT0 field."]
    #[inline] pub fn set_indat0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chdatin2r {
    #[inline]
    fn from(other: u32) -> Self {
         Chdatin2r(other)
    }
}

impl ::core::fmt::Display for Chdatin2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chdatin2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.indat1() != 0 { try!(write!(f, " indat1=0x{:x}", self.indat1()))}
        if self.indat0() != 0 { try!(write!(f, " indat0=0x{:x}", self.indat0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG3R1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg3r1(pub u32);
impl Chcfg3r1 {
    #[doc="DATPACK"]
    #[inline] pub fn datpack(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if DATPACK != 0"]
    #[inline] pub fn test_datpack(&self) -> bool {
        self.datpack() != 0
    }

    #[doc="Sets the DATPACK field."]
    #[inline] pub fn set_datpack<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DATMPX"]
    #[inline] pub fn datmpx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if DATMPX != 0"]
    #[inline] pub fn test_datmpx(&self) -> bool {
        self.datmpx() != 0
    }

    #[doc="Sets the DATMPX field."]
    #[inline] pub fn set_datmpx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CHINSEL"]
    #[inline] pub fn chinsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHINSEL != 0"]
    #[inline] pub fn test_chinsel(&self) -> bool {
        self.chinsel() != 0
    }

    #[doc="Sets the CHINSEL field."]
    #[inline] pub fn set_chinsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CHEN"]
    #[inline] pub fn chen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHEN != 0"]
    #[inline] pub fn test_chen(&self) -> bool {
        self.chen() != 0
    }

    #[doc="Sets the CHEN field."]
    #[inline] pub fn set_chen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CKABEN"]
    #[inline] pub fn ckaben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABEN != 0"]
    #[inline] pub fn test_ckaben(&self) -> bool {
        self.ckaben() != 0
    }

    #[doc="Sets the CKABEN field."]
    #[inline] pub fn set_ckaben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SCDEN"]
    #[inline] pub fn scden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDEN != 0"]
    #[inline] pub fn test_scden(&self) -> bool {
        self.scden() != 0
    }

    #[doc="Sets the SCDEN field."]
    #[inline] pub fn set_scden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPICKSEL"]
    #[inline] pub fn spicksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPICKSEL != 0"]
    #[inline] pub fn test_spicksel(&self) -> bool {
        self.spicksel() != 0
    }

    #[doc="Sets the SPICKSEL field."]
    #[inline] pub fn set_spicksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SITP"]
    #[inline] pub fn sitp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SITP != 0"]
    #[inline] pub fn test_sitp(&self) -> bool {
        self.sitp() != 0
    }

    #[doc="Sets the SITP field."]
    #[inline] pub fn set_sitp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chcfg3r1 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg3r1(other)
    }
}

impl ::core::fmt::Display for Chcfg3r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg3r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datpack() != 0 { try!(write!(f, " datpack=0x{:x}", self.datpack()))}
        if self.datmpx() != 0 { try!(write!(f, " datmpx=0x{:x}", self.datmpx()))}
        if self.chinsel() != 0 { try!(write!(f, " chinsel"))}
        if self.chen() != 0 { try!(write!(f, " chen"))}
        if self.ckaben() != 0 { try!(write!(f, " ckaben"))}
        if self.scden() != 0 { try!(write!(f, " scden"))}
        if self.spicksel() != 0 { try!(write!(f, " spicksel=0x{:x}", self.spicksel()))}
        if self.sitp() != 0 { try!(write!(f, " sitp=0x{:x}", self.sitp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG3R2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg3r2(pub u32);
impl Chcfg3r2 {
    #[doc="OFFSET"]
    #[inline] pub fn offset(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if OFFSET != 0"]
    #[inline] pub fn test_offset(&self) -> bool {
        self.offset() != 0
    }

    #[doc="Sets the OFFSET field."]
    #[inline] pub fn set_offset<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DTRBS"]
    #[inline] pub fn dtrbs(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if DTRBS != 0"]
    #[inline] pub fn test_dtrbs(&self) -> bool {
        self.dtrbs() != 0
    }

    #[doc="Sets the DTRBS field."]
    #[inline] pub fn set_dtrbs<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Chcfg3r2 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg3r2(other)
    }
}

impl ::core::fmt::Display for Chcfg3r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg3r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset() != 0 { try!(write!(f, " offset=0x{:x}", self.offset()))}
        if self.dtrbs() != 0 { try!(write!(f, " dtrbs=0x{:x}", self.dtrbs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AWSCD3R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awscd3r(pub u32);
impl Awscd3r {
    #[doc="AWFORD"]
    #[inline] pub fn awford(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if AWFORD != 0"]
    #[inline] pub fn test_awford(&self) -> bool {
        self.awford() != 0
    }

    #[doc="Sets the AWFORD field."]
    #[inline] pub fn set_awford<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="AWFOSR"]
    #[inline] pub fn awfosr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if AWFOSR != 0"]
    #[inline] pub fn test_awfosr(&self) -> bool {
        self.awfosr() != 0
    }

    #[doc="Sets the AWFOSR field."]
    #[inline] pub fn set_awfosr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="BKSCD"]
    #[inline] pub fn bkscd(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if BKSCD != 0"]
    #[inline] pub fn test_bkscd(&self) -> bool {
        self.bkscd() != 0
    }

    #[doc="Sets the BKSCD field."]
    #[inline] pub fn set_bkscd<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SCDT"]
    #[inline] pub fn scdt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCDT != 0"]
    #[inline] pub fn test_scdt(&self) -> bool {
        self.scdt() != 0
    }

    #[doc="Sets the SCDT field."]
    #[inline] pub fn set_scdt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Awscd3r {
    #[inline]
    fn from(other: u32) -> Self {
         Awscd3r(other)
    }
}

impl ::core::fmt::Display for Awscd3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awscd3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awford() != 0 { try!(write!(f, " awford=0x{:x}", self.awford()))}
        if self.awfosr() != 0 { try!(write!(f, " awfosr=0x{:x}", self.awfosr()))}
        if self.bkscd() != 0 { try!(write!(f, " bkscd=0x{:x}", self.bkscd()))}
        if self.scdt() != 0 { try!(write!(f, " scdt=0x{:x}", self.scdt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHWDAT3R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chwdat3r(pub u32);
impl Chwdat3r {
    #[doc="WDATA"]
    #[inline] pub fn wdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WDATA != 0"]
    #[inline] pub fn test_wdata(&self) -> bool {
        self.wdata() != 0
    }

    #[doc="Sets the WDATA field."]
    #[inline] pub fn set_wdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chwdat3r {
    #[inline]
    fn from(other: u32) -> Self {
         Chwdat3r(other)
    }
}

impl ::core::fmt::Display for Chwdat3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chwdat3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdata() != 0 { try!(write!(f, " wdata=0x{:x}", self.wdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHDATIN3R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chdatin3r(pub u32);
impl Chdatin3r {
    #[doc="INDAT1"]
    #[inline] pub fn indat1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if INDAT1 != 0"]
    #[inline] pub fn test_indat1(&self) -> bool {
        self.indat1() != 0
    }

    #[doc="Sets the INDAT1 field."]
    #[inline] pub fn set_indat1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="INDAT0"]
    #[inline] pub fn indat0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INDAT0 != 0"]
    #[inline] pub fn test_indat0(&self) -> bool {
        self.indat0() != 0
    }

    #[doc="Sets the INDAT0 field."]
    #[inline] pub fn set_indat0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chdatin3r {
    #[inline]
    fn from(other: u32) -> Self {
         Chdatin3r(other)
    }
}

impl ::core::fmt::Display for Chdatin3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chdatin3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.indat1() != 0 { try!(write!(f, " indat1=0x{:x}", self.indat1()))}
        if self.indat0() != 0 { try!(write!(f, " indat0=0x{:x}", self.indat0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG4R1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg4r1(pub u32);
impl Chcfg4r1 {
    #[doc="DATPACK"]
    #[inline] pub fn datpack(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if DATPACK != 0"]
    #[inline] pub fn test_datpack(&self) -> bool {
        self.datpack() != 0
    }

    #[doc="Sets the DATPACK field."]
    #[inline] pub fn set_datpack<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DATMPX"]
    #[inline] pub fn datmpx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if DATMPX != 0"]
    #[inline] pub fn test_datmpx(&self) -> bool {
        self.datmpx() != 0
    }

    #[doc="Sets the DATMPX field."]
    #[inline] pub fn set_datmpx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CHINSEL"]
    #[inline] pub fn chinsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHINSEL != 0"]
    #[inline] pub fn test_chinsel(&self) -> bool {
        self.chinsel() != 0
    }

    #[doc="Sets the CHINSEL field."]
    #[inline] pub fn set_chinsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CHEN"]
    #[inline] pub fn chen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHEN != 0"]
    #[inline] pub fn test_chen(&self) -> bool {
        self.chen() != 0
    }

    #[doc="Sets the CHEN field."]
    #[inline] pub fn set_chen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CKABEN"]
    #[inline] pub fn ckaben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABEN != 0"]
    #[inline] pub fn test_ckaben(&self) -> bool {
        self.ckaben() != 0
    }

    #[doc="Sets the CKABEN field."]
    #[inline] pub fn set_ckaben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SCDEN"]
    #[inline] pub fn scden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDEN != 0"]
    #[inline] pub fn test_scden(&self) -> bool {
        self.scden() != 0
    }

    #[doc="Sets the SCDEN field."]
    #[inline] pub fn set_scden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPICKSEL"]
    #[inline] pub fn spicksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPICKSEL != 0"]
    #[inline] pub fn test_spicksel(&self) -> bool {
        self.spicksel() != 0
    }

    #[doc="Sets the SPICKSEL field."]
    #[inline] pub fn set_spicksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SITP"]
    #[inline] pub fn sitp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SITP != 0"]
    #[inline] pub fn test_sitp(&self) -> bool {
        self.sitp() != 0
    }

    #[doc="Sets the SITP field."]
    #[inline] pub fn set_sitp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chcfg4r1 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg4r1(other)
    }
}

impl ::core::fmt::Display for Chcfg4r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg4r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datpack() != 0 { try!(write!(f, " datpack=0x{:x}", self.datpack()))}
        if self.datmpx() != 0 { try!(write!(f, " datmpx=0x{:x}", self.datmpx()))}
        if self.chinsel() != 0 { try!(write!(f, " chinsel"))}
        if self.chen() != 0 { try!(write!(f, " chen"))}
        if self.ckaben() != 0 { try!(write!(f, " ckaben"))}
        if self.scden() != 0 { try!(write!(f, " scden"))}
        if self.spicksel() != 0 { try!(write!(f, " spicksel=0x{:x}", self.spicksel()))}
        if self.sitp() != 0 { try!(write!(f, " sitp=0x{:x}", self.sitp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG4R2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg4r2(pub u32);
impl Chcfg4r2 {
    #[doc="OFFSET"]
    #[inline] pub fn offset(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if OFFSET != 0"]
    #[inline] pub fn test_offset(&self) -> bool {
        self.offset() != 0
    }

    #[doc="Sets the OFFSET field."]
    #[inline] pub fn set_offset<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DTRBS"]
    #[inline] pub fn dtrbs(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if DTRBS != 0"]
    #[inline] pub fn test_dtrbs(&self) -> bool {
        self.dtrbs() != 0
    }

    #[doc="Sets the DTRBS field."]
    #[inline] pub fn set_dtrbs<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Chcfg4r2 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg4r2(other)
    }
}

impl ::core::fmt::Display for Chcfg4r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg4r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset() != 0 { try!(write!(f, " offset=0x{:x}", self.offset()))}
        if self.dtrbs() != 0 { try!(write!(f, " dtrbs=0x{:x}", self.dtrbs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AWSCD4R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awscd4r(pub u32);
impl Awscd4r {
    #[doc="AWFORD"]
    #[inline] pub fn awford(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if AWFORD != 0"]
    #[inline] pub fn test_awford(&self) -> bool {
        self.awford() != 0
    }

    #[doc="Sets the AWFORD field."]
    #[inline] pub fn set_awford<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="AWFOSR"]
    #[inline] pub fn awfosr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if AWFOSR != 0"]
    #[inline] pub fn test_awfosr(&self) -> bool {
        self.awfosr() != 0
    }

    #[doc="Sets the AWFOSR field."]
    #[inline] pub fn set_awfosr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="BKSCD"]
    #[inline] pub fn bkscd(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if BKSCD != 0"]
    #[inline] pub fn test_bkscd(&self) -> bool {
        self.bkscd() != 0
    }

    #[doc="Sets the BKSCD field."]
    #[inline] pub fn set_bkscd<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SCDT"]
    #[inline] pub fn scdt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCDT != 0"]
    #[inline] pub fn test_scdt(&self) -> bool {
        self.scdt() != 0
    }

    #[doc="Sets the SCDT field."]
    #[inline] pub fn set_scdt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Awscd4r {
    #[inline]
    fn from(other: u32) -> Self {
         Awscd4r(other)
    }
}

impl ::core::fmt::Display for Awscd4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awscd4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awford() != 0 { try!(write!(f, " awford=0x{:x}", self.awford()))}
        if self.awfosr() != 0 { try!(write!(f, " awfosr=0x{:x}", self.awfosr()))}
        if self.bkscd() != 0 { try!(write!(f, " bkscd=0x{:x}", self.bkscd()))}
        if self.scdt() != 0 { try!(write!(f, " scdt=0x{:x}", self.scdt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHWDAT4R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chwdat4r(pub u32);
impl Chwdat4r {
    #[doc="WDATA"]
    #[inline] pub fn wdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WDATA != 0"]
    #[inline] pub fn test_wdata(&self) -> bool {
        self.wdata() != 0
    }

    #[doc="Sets the WDATA field."]
    #[inline] pub fn set_wdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chwdat4r {
    #[inline]
    fn from(other: u32) -> Self {
         Chwdat4r(other)
    }
}

impl ::core::fmt::Display for Chwdat4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chwdat4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdata() != 0 { try!(write!(f, " wdata=0x{:x}", self.wdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHDATIN4R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chdatin4r(pub u32);
impl Chdatin4r {
    #[doc="INDAT1"]
    #[inline] pub fn indat1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if INDAT1 != 0"]
    #[inline] pub fn test_indat1(&self) -> bool {
        self.indat1() != 0
    }

    #[doc="Sets the INDAT1 field."]
    #[inline] pub fn set_indat1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="INDAT0"]
    #[inline] pub fn indat0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INDAT0 != 0"]
    #[inline] pub fn test_indat0(&self) -> bool {
        self.indat0() != 0
    }

    #[doc="Sets the INDAT0 field."]
    #[inline] pub fn set_indat0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chdatin4r {
    #[inline]
    fn from(other: u32) -> Self {
         Chdatin4r(other)
    }
}

impl ::core::fmt::Display for Chdatin4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chdatin4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.indat1() != 0 { try!(write!(f, " indat1=0x{:x}", self.indat1()))}
        if self.indat0() != 0 { try!(write!(f, " indat0=0x{:x}", self.indat0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG5R1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg5r1(pub u32);
impl Chcfg5r1 {
    #[doc="DATPACK"]
    #[inline] pub fn datpack(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if DATPACK != 0"]
    #[inline] pub fn test_datpack(&self) -> bool {
        self.datpack() != 0
    }

    #[doc="Sets the DATPACK field."]
    #[inline] pub fn set_datpack<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DATMPX"]
    #[inline] pub fn datmpx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if DATMPX != 0"]
    #[inline] pub fn test_datmpx(&self) -> bool {
        self.datmpx() != 0
    }

    #[doc="Sets the DATMPX field."]
    #[inline] pub fn set_datmpx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CHINSEL"]
    #[inline] pub fn chinsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHINSEL != 0"]
    #[inline] pub fn test_chinsel(&self) -> bool {
        self.chinsel() != 0
    }

    #[doc="Sets the CHINSEL field."]
    #[inline] pub fn set_chinsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CHEN"]
    #[inline] pub fn chen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHEN != 0"]
    #[inline] pub fn test_chen(&self) -> bool {
        self.chen() != 0
    }

    #[doc="Sets the CHEN field."]
    #[inline] pub fn set_chen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CKABEN"]
    #[inline] pub fn ckaben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABEN != 0"]
    #[inline] pub fn test_ckaben(&self) -> bool {
        self.ckaben() != 0
    }

    #[doc="Sets the CKABEN field."]
    #[inline] pub fn set_ckaben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SCDEN"]
    #[inline] pub fn scden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDEN != 0"]
    #[inline] pub fn test_scden(&self) -> bool {
        self.scden() != 0
    }

    #[doc="Sets the SCDEN field."]
    #[inline] pub fn set_scden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPICKSEL"]
    #[inline] pub fn spicksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPICKSEL != 0"]
    #[inline] pub fn test_spicksel(&self) -> bool {
        self.spicksel() != 0
    }

    #[doc="Sets the SPICKSEL field."]
    #[inline] pub fn set_spicksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SITP"]
    #[inline] pub fn sitp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SITP != 0"]
    #[inline] pub fn test_sitp(&self) -> bool {
        self.sitp() != 0
    }

    #[doc="Sets the SITP field."]
    #[inline] pub fn set_sitp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chcfg5r1 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg5r1(other)
    }
}

impl ::core::fmt::Display for Chcfg5r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg5r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datpack() != 0 { try!(write!(f, " datpack=0x{:x}", self.datpack()))}
        if self.datmpx() != 0 { try!(write!(f, " datmpx=0x{:x}", self.datmpx()))}
        if self.chinsel() != 0 { try!(write!(f, " chinsel"))}
        if self.chen() != 0 { try!(write!(f, " chen"))}
        if self.ckaben() != 0 { try!(write!(f, " ckaben"))}
        if self.scden() != 0 { try!(write!(f, " scden"))}
        if self.spicksel() != 0 { try!(write!(f, " spicksel=0x{:x}", self.spicksel()))}
        if self.sitp() != 0 { try!(write!(f, " sitp=0x{:x}", self.sitp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG5R2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg5r2(pub u32);
impl Chcfg5r2 {
    #[doc="OFFSET"]
    #[inline] pub fn offset(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if OFFSET != 0"]
    #[inline] pub fn test_offset(&self) -> bool {
        self.offset() != 0
    }

    #[doc="Sets the OFFSET field."]
    #[inline] pub fn set_offset<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DTRBS"]
    #[inline] pub fn dtrbs(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if DTRBS != 0"]
    #[inline] pub fn test_dtrbs(&self) -> bool {
        self.dtrbs() != 0
    }

    #[doc="Sets the DTRBS field."]
    #[inline] pub fn set_dtrbs<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Chcfg5r2 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg5r2(other)
    }
}

impl ::core::fmt::Display for Chcfg5r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg5r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset() != 0 { try!(write!(f, " offset=0x{:x}", self.offset()))}
        if self.dtrbs() != 0 { try!(write!(f, " dtrbs=0x{:x}", self.dtrbs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AWSCD5R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awscd5r(pub u32);
impl Awscd5r {
    #[doc="AWFORD"]
    #[inline] pub fn awford(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if AWFORD != 0"]
    #[inline] pub fn test_awford(&self) -> bool {
        self.awford() != 0
    }

    #[doc="Sets the AWFORD field."]
    #[inline] pub fn set_awford<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="AWFOSR"]
    #[inline] pub fn awfosr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if AWFOSR != 0"]
    #[inline] pub fn test_awfosr(&self) -> bool {
        self.awfosr() != 0
    }

    #[doc="Sets the AWFOSR field."]
    #[inline] pub fn set_awfosr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="BKSCD"]
    #[inline] pub fn bkscd(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if BKSCD != 0"]
    #[inline] pub fn test_bkscd(&self) -> bool {
        self.bkscd() != 0
    }

    #[doc="Sets the BKSCD field."]
    #[inline] pub fn set_bkscd<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SCDT"]
    #[inline] pub fn scdt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCDT != 0"]
    #[inline] pub fn test_scdt(&self) -> bool {
        self.scdt() != 0
    }

    #[doc="Sets the SCDT field."]
    #[inline] pub fn set_scdt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Awscd5r {
    #[inline]
    fn from(other: u32) -> Self {
         Awscd5r(other)
    }
}

impl ::core::fmt::Display for Awscd5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awscd5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awford() != 0 { try!(write!(f, " awford=0x{:x}", self.awford()))}
        if self.awfosr() != 0 { try!(write!(f, " awfosr=0x{:x}", self.awfosr()))}
        if self.bkscd() != 0 { try!(write!(f, " bkscd=0x{:x}", self.bkscd()))}
        if self.scdt() != 0 { try!(write!(f, " scdt=0x{:x}", self.scdt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHWDAT5R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chwdat5r(pub u32);
impl Chwdat5r {
    #[doc="WDATA"]
    #[inline] pub fn wdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WDATA != 0"]
    #[inline] pub fn test_wdata(&self) -> bool {
        self.wdata() != 0
    }

    #[doc="Sets the WDATA field."]
    #[inline] pub fn set_wdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chwdat5r {
    #[inline]
    fn from(other: u32) -> Self {
         Chwdat5r(other)
    }
}

impl ::core::fmt::Display for Chwdat5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chwdat5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdata() != 0 { try!(write!(f, " wdata=0x{:x}", self.wdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHDATIN5R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chdatin5r(pub u32);
impl Chdatin5r {
    #[doc="INDAT1"]
    #[inline] pub fn indat1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if INDAT1 != 0"]
    #[inline] pub fn test_indat1(&self) -> bool {
        self.indat1() != 0
    }

    #[doc="Sets the INDAT1 field."]
    #[inline] pub fn set_indat1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="INDAT0"]
    #[inline] pub fn indat0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INDAT0 != 0"]
    #[inline] pub fn test_indat0(&self) -> bool {
        self.indat0() != 0
    }

    #[doc="Sets the INDAT0 field."]
    #[inline] pub fn set_indat0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chdatin5r {
    #[inline]
    fn from(other: u32) -> Self {
         Chdatin5r(other)
    }
}

impl ::core::fmt::Display for Chdatin5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chdatin5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.indat1() != 0 { try!(write!(f, " indat1=0x{:x}", self.indat1()))}
        if self.indat0() != 0 { try!(write!(f, " indat0=0x{:x}", self.indat0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG6R1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg6r1(pub u32);
impl Chcfg6r1 {
    #[doc="DATPACK"]
    #[inline] pub fn datpack(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if DATPACK != 0"]
    #[inline] pub fn test_datpack(&self) -> bool {
        self.datpack() != 0
    }

    #[doc="Sets the DATPACK field."]
    #[inline] pub fn set_datpack<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DATMPX"]
    #[inline] pub fn datmpx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if DATMPX != 0"]
    #[inline] pub fn test_datmpx(&self) -> bool {
        self.datmpx() != 0
    }

    #[doc="Sets the DATMPX field."]
    #[inline] pub fn set_datmpx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CHINSEL"]
    #[inline] pub fn chinsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHINSEL != 0"]
    #[inline] pub fn test_chinsel(&self) -> bool {
        self.chinsel() != 0
    }

    #[doc="Sets the CHINSEL field."]
    #[inline] pub fn set_chinsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CHEN"]
    #[inline] pub fn chen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHEN != 0"]
    #[inline] pub fn test_chen(&self) -> bool {
        self.chen() != 0
    }

    #[doc="Sets the CHEN field."]
    #[inline] pub fn set_chen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CKABEN"]
    #[inline] pub fn ckaben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABEN != 0"]
    #[inline] pub fn test_ckaben(&self) -> bool {
        self.ckaben() != 0
    }

    #[doc="Sets the CKABEN field."]
    #[inline] pub fn set_ckaben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SCDEN"]
    #[inline] pub fn scden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDEN != 0"]
    #[inline] pub fn test_scden(&self) -> bool {
        self.scden() != 0
    }

    #[doc="Sets the SCDEN field."]
    #[inline] pub fn set_scden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPICKSEL"]
    #[inline] pub fn spicksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPICKSEL != 0"]
    #[inline] pub fn test_spicksel(&self) -> bool {
        self.spicksel() != 0
    }

    #[doc="Sets the SPICKSEL field."]
    #[inline] pub fn set_spicksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SITP"]
    #[inline] pub fn sitp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SITP != 0"]
    #[inline] pub fn test_sitp(&self) -> bool {
        self.sitp() != 0
    }

    #[doc="Sets the SITP field."]
    #[inline] pub fn set_sitp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chcfg6r1 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg6r1(other)
    }
}

impl ::core::fmt::Display for Chcfg6r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg6r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datpack() != 0 { try!(write!(f, " datpack=0x{:x}", self.datpack()))}
        if self.datmpx() != 0 { try!(write!(f, " datmpx=0x{:x}", self.datmpx()))}
        if self.chinsel() != 0 { try!(write!(f, " chinsel"))}
        if self.chen() != 0 { try!(write!(f, " chen"))}
        if self.ckaben() != 0 { try!(write!(f, " ckaben"))}
        if self.scden() != 0 { try!(write!(f, " scden"))}
        if self.spicksel() != 0 { try!(write!(f, " spicksel=0x{:x}", self.spicksel()))}
        if self.sitp() != 0 { try!(write!(f, " sitp=0x{:x}", self.sitp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG6R2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg6r2(pub u32);
impl Chcfg6r2 {
    #[doc="OFFSET"]
    #[inline] pub fn offset(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if OFFSET != 0"]
    #[inline] pub fn test_offset(&self) -> bool {
        self.offset() != 0
    }

    #[doc="Sets the OFFSET field."]
    #[inline] pub fn set_offset<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DTRBS"]
    #[inline] pub fn dtrbs(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if DTRBS != 0"]
    #[inline] pub fn test_dtrbs(&self) -> bool {
        self.dtrbs() != 0
    }

    #[doc="Sets the DTRBS field."]
    #[inline] pub fn set_dtrbs<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Chcfg6r2 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg6r2(other)
    }
}

impl ::core::fmt::Display for Chcfg6r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg6r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset() != 0 { try!(write!(f, " offset=0x{:x}", self.offset()))}
        if self.dtrbs() != 0 { try!(write!(f, " dtrbs=0x{:x}", self.dtrbs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AWSCD6R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awscd6r(pub u32);
impl Awscd6r {
    #[doc="AWFORD"]
    #[inline] pub fn awford(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if AWFORD != 0"]
    #[inline] pub fn test_awford(&self) -> bool {
        self.awford() != 0
    }

    #[doc="Sets the AWFORD field."]
    #[inline] pub fn set_awford<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="AWFOSR"]
    #[inline] pub fn awfosr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if AWFOSR != 0"]
    #[inline] pub fn test_awfosr(&self) -> bool {
        self.awfosr() != 0
    }

    #[doc="Sets the AWFOSR field."]
    #[inline] pub fn set_awfosr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="BKSCD"]
    #[inline] pub fn bkscd(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if BKSCD != 0"]
    #[inline] pub fn test_bkscd(&self) -> bool {
        self.bkscd() != 0
    }

    #[doc="Sets the BKSCD field."]
    #[inline] pub fn set_bkscd<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SCDT"]
    #[inline] pub fn scdt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCDT != 0"]
    #[inline] pub fn test_scdt(&self) -> bool {
        self.scdt() != 0
    }

    #[doc="Sets the SCDT field."]
    #[inline] pub fn set_scdt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Awscd6r {
    #[inline]
    fn from(other: u32) -> Self {
         Awscd6r(other)
    }
}

impl ::core::fmt::Display for Awscd6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awscd6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awford() != 0 { try!(write!(f, " awford=0x{:x}", self.awford()))}
        if self.awfosr() != 0 { try!(write!(f, " awfosr=0x{:x}", self.awfosr()))}
        if self.bkscd() != 0 { try!(write!(f, " bkscd=0x{:x}", self.bkscd()))}
        if self.scdt() != 0 { try!(write!(f, " scdt=0x{:x}", self.scdt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHWDAT6R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chwdat6r(pub u32);
impl Chwdat6r {
    #[doc="WDATA"]
    #[inline] pub fn wdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WDATA != 0"]
    #[inline] pub fn test_wdata(&self) -> bool {
        self.wdata() != 0
    }

    #[doc="Sets the WDATA field."]
    #[inline] pub fn set_wdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chwdat6r {
    #[inline]
    fn from(other: u32) -> Self {
         Chwdat6r(other)
    }
}

impl ::core::fmt::Display for Chwdat6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chwdat6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdata() != 0 { try!(write!(f, " wdata=0x{:x}", self.wdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHDATIN6R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chdatin6r(pub u32);
impl Chdatin6r {
    #[doc="INDAT1"]
    #[inline] pub fn indat1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if INDAT1 != 0"]
    #[inline] pub fn test_indat1(&self) -> bool {
        self.indat1() != 0
    }

    #[doc="Sets the INDAT1 field."]
    #[inline] pub fn set_indat1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="INDAT0"]
    #[inline] pub fn indat0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INDAT0 != 0"]
    #[inline] pub fn test_indat0(&self) -> bool {
        self.indat0() != 0
    }

    #[doc="Sets the INDAT0 field."]
    #[inline] pub fn set_indat0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chdatin6r {
    #[inline]
    fn from(other: u32) -> Self {
         Chdatin6r(other)
    }
}

impl ::core::fmt::Display for Chdatin6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chdatin6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.indat1() != 0 { try!(write!(f, " indat1=0x{:x}", self.indat1()))}
        if self.indat0() != 0 { try!(write!(f, " indat0=0x{:x}", self.indat0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG7R1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg7r1(pub u32);
impl Chcfg7r1 {
    #[doc="DATPACK"]
    #[inline] pub fn datpack(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if DATPACK != 0"]
    #[inline] pub fn test_datpack(&self) -> bool {
        self.datpack() != 0
    }

    #[doc="Sets the DATPACK field."]
    #[inline] pub fn set_datpack<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DATMPX"]
    #[inline] pub fn datmpx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if DATMPX != 0"]
    #[inline] pub fn test_datmpx(&self) -> bool {
        self.datmpx() != 0
    }

    #[doc="Sets the DATMPX field."]
    #[inline] pub fn set_datmpx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CHINSEL"]
    #[inline] pub fn chinsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHINSEL != 0"]
    #[inline] pub fn test_chinsel(&self) -> bool {
        self.chinsel() != 0
    }

    #[doc="Sets the CHINSEL field."]
    #[inline] pub fn set_chinsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CHEN"]
    #[inline] pub fn chen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHEN != 0"]
    #[inline] pub fn test_chen(&self) -> bool {
        self.chen() != 0
    }

    #[doc="Sets the CHEN field."]
    #[inline] pub fn set_chen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CKABEN"]
    #[inline] pub fn ckaben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABEN != 0"]
    #[inline] pub fn test_ckaben(&self) -> bool {
        self.ckaben() != 0
    }

    #[doc="Sets the CKABEN field."]
    #[inline] pub fn set_ckaben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SCDEN"]
    #[inline] pub fn scden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDEN != 0"]
    #[inline] pub fn test_scden(&self) -> bool {
        self.scden() != 0
    }

    #[doc="Sets the SCDEN field."]
    #[inline] pub fn set_scden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPICKSEL"]
    #[inline] pub fn spicksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPICKSEL != 0"]
    #[inline] pub fn test_spicksel(&self) -> bool {
        self.spicksel() != 0
    }

    #[doc="Sets the SPICKSEL field."]
    #[inline] pub fn set_spicksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SITP"]
    #[inline] pub fn sitp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SITP != 0"]
    #[inline] pub fn test_sitp(&self) -> bool {
        self.sitp() != 0
    }

    #[doc="Sets the SITP field."]
    #[inline] pub fn set_sitp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chcfg7r1 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg7r1(other)
    }
}

impl ::core::fmt::Display for Chcfg7r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg7r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datpack() != 0 { try!(write!(f, " datpack=0x{:x}", self.datpack()))}
        if self.datmpx() != 0 { try!(write!(f, " datmpx=0x{:x}", self.datmpx()))}
        if self.chinsel() != 0 { try!(write!(f, " chinsel"))}
        if self.chen() != 0 { try!(write!(f, " chen"))}
        if self.ckaben() != 0 { try!(write!(f, " ckaben"))}
        if self.scden() != 0 { try!(write!(f, " scden"))}
        if self.spicksel() != 0 { try!(write!(f, " spicksel=0x{:x}", self.spicksel()))}
        if self.sitp() != 0 { try!(write!(f, " sitp=0x{:x}", self.sitp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHCFG7R2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg7r2(pub u32);
impl Chcfg7r2 {
    #[doc="OFFSET"]
    #[inline] pub fn offset(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if OFFSET != 0"]
    #[inline] pub fn test_offset(&self) -> bool {
        self.offset() != 0
    }

    #[doc="Sets the OFFSET field."]
    #[inline] pub fn set_offset<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DTRBS"]
    #[inline] pub fn dtrbs(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if DTRBS != 0"]
    #[inline] pub fn test_dtrbs(&self) -> bool {
        self.dtrbs() != 0
    }

    #[doc="Sets the DTRBS field."]
    #[inline] pub fn set_dtrbs<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Chcfg7r2 {
    #[inline]
    fn from(other: u32) -> Self {
         Chcfg7r2(other)
    }
}

impl ::core::fmt::Display for Chcfg7r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg7r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset() != 0 { try!(write!(f, " offset=0x{:x}", self.offset()))}
        if self.dtrbs() != 0 { try!(write!(f, " dtrbs=0x{:x}", self.dtrbs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AWSCD7R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awscd7r(pub u32);
impl Awscd7r {
    #[doc="AWFORD"]
    #[inline] pub fn awford(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if AWFORD != 0"]
    #[inline] pub fn test_awford(&self) -> bool {
        self.awford() != 0
    }

    #[doc="Sets the AWFORD field."]
    #[inline] pub fn set_awford<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="AWFOSR"]
    #[inline] pub fn awfosr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if AWFOSR != 0"]
    #[inline] pub fn test_awfosr(&self) -> bool {
        self.awfosr() != 0
    }

    #[doc="Sets the AWFOSR field."]
    #[inline] pub fn set_awfosr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="BKSCD"]
    #[inline] pub fn bkscd(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if BKSCD != 0"]
    #[inline] pub fn test_bkscd(&self) -> bool {
        self.bkscd() != 0
    }

    #[doc="Sets the BKSCD field."]
    #[inline] pub fn set_bkscd<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SCDT"]
    #[inline] pub fn scdt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCDT != 0"]
    #[inline] pub fn test_scdt(&self) -> bool {
        self.scdt() != 0
    }

    #[doc="Sets the SCDT field."]
    #[inline] pub fn set_scdt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Awscd7r {
    #[inline]
    fn from(other: u32) -> Self {
         Awscd7r(other)
    }
}

impl ::core::fmt::Display for Awscd7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awscd7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awford() != 0 { try!(write!(f, " awford=0x{:x}", self.awford()))}
        if self.awfosr() != 0 { try!(write!(f, " awfosr=0x{:x}", self.awfosr()))}
        if self.bkscd() != 0 { try!(write!(f, " bkscd=0x{:x}", self.bkscd()))}
        if self.scdt() != 0 { try!(write!(f, " scdt=0x{:x}", self.scdt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHWDAT7R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chwdat7r(pub u32);
impl Chwdat7r {
    #[doc="WDATA"]
    #[inline] pub fn wdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WDATA != 0"]
    #[inline] pub fn test_wdata(&self) -> bool {
        self.wdata() != 0
    }

    #[doc="Sets the WDATA field."]
    #[inline] pub fn set_wdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chwdat7r {
    #[inline]
    fn from(other: u32) -> Self {
         Chwdat7r(other)
    }
}

impl ::core::fmt::Display for Chwdat7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chwdat7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wdata() != 0 { try!(write!(f, " wdata=0x{:x}", self.wdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CHDATIN7R"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chdatin7r(pub u32);
impl Chdatin7r {
    #[doc="INDAT1"]
    #[inline] pub fn indat1(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if INDAT1 != 0"]
    #[inline] pub fn test_indat1(&self) -> bool {
        self.indat1() != 0
    }

    #[doc="Sets the INDAT1 field."]
    #[inline] pub fn set_indat1<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="INDAT0"]
    #[inline] pub fn indat0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INDAT0 != 0"]
    #[inline] pub fn test_indat0(&self) -> bool {
        self.indat0() != 0
    }

    #[doc="Sets the INDAT0 field."]
    #[inline] pub fn set_indat0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chdatin7r {
    #[inline]
    fn from(other: u32) -> Self {
         Chdatin7r(other)
    }
}

impl ::core::fmt::Display for Chdatin7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chdatin7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.indat1() != 0 { try!(write!(f, " indat1=0x{:x}", self.indat1()))}
        if self.indat0() != 0 { try!(write!(f, " indat0=0x{:x}", self.indat0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Cr1(pub u32);
impl Dfsdm0Cr1 {
    #[doc="Analog watchdog fast mode select"]
    #[inline] pub fn awfsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if AWFSEL != 0"]
    #[inline] pub fn test_awfsel(&self) -> bool {
        self.awfsel() != 0
    }

    #[doc="Sets the AWFSEL field."]
    #[inline] pub fn set_awfsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Fast conversion mode selection for regular conversions"]
    #[inline] pub fn fast(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if FAST != 0"]
    #[inline] pub fn test_fast(&self) -> bool {
        self.fast() != 0
    }

    #[doc="Sets the FAST field."]
    #[inline] pub fn set_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Regular channel selection"]
    #[inline] pub fn rch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if RCH != 0"]
    #[inline] pub fn test_rch(&self) -> bool {
        self.rch() != 0
    }

    #[doc="Sets the RCH field."]
    #[inline] pub fn set_rch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DMA channel enabled to read data for the regular conversion"]
    #[inline] pub fn rdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RDMAEN != 0"]
    #[inline] pub fn test_rdmaen(&self) -> bool {
        self.rdmaen() != 0
    }

    #[doc="Sets the RDMAEN field."]
    #[inline] pub fn set_rdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Launch regular conversion synchronously with DFSDM0"]
    #[inline] pub fn rsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RSYNC != 0"]
    #[inline] pub fn test_rsync(&self) -> bool {
        self.rsync() != 0
    }

    #[doc="Sets the RSYNC field."]
    #[inline] pub fn set_rsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Continuous mode selection for regular conversions"]
    #[inline] pub fn rcont(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RCONT != 0"]
    #[inline] pub fn test_rcont(&self) -> bool {
        self.rcont() != 0
    }

    #[doc="Sets the RCONT field."]
    #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Software start of a conversion on the regular channel"]
    #[inline] pub fn rswstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RSWSTART != 0"]
    #[inline] pub fn test_rswstart(&self) -> bool {
        self.rswstart() != 0
    }

    #[doc="Sets the RSWSTART field."]
    #[inline] pub fn set_rswstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Trigger enable and trigger edge selection for injected conversions"]
    #[inline] pub fn jexten(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if JEXTEN != 0"]
    #[inline] pub fn test_jexten(&self) -> bool {
        self.jexten() != 0
    }

    #[doc="Sets the JEXTEN field."]
    #[inline] pub fn set_jexten<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Trigger signal selection for launching injected conversions"]
    #[inline] pub fn jextsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if JEXTSEL != 0"]
    #[inline] pub fn test_jextsel(&self) -> bool {
        self.jextsel() != 0
    }

    #[doc="Sets the JEXTSEL field."]
    #[inline] pub fn set_jextsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA channel enabled to read data for the injected channel group"]
    #[inline] pub fn jdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if JDMAEN != 0"]
    #[inline] pub fn test_jdmaen(&self) -> bool {
        self.jdmaen() != 0
    }

    #[doc="Sets the JDMAEN field."]
    #[inline] pub fn set_jdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Scanning conversion mode for injected conversions"]
    #[inline] pub fn jscan(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if JSCAN != 0"]
    #[inline] pub fn test_jscan(&self) -> bool {
        self.jscan() != 0
    }

    #[doc="Sets the JSCAN field."]
    #[inline] pub fn set_jscan<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline] pub fn jsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if JSYNC != 0"]
    #[inline] pub fn test_jsync(&self) -> bool {
        self.jsync() != 0
    }

    #[doc="Sets the JSYNC field."]
    #[inline] pub fn set_jsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Start a conversion of the injected group of channels"]
    #[inline] pub fn jswstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if JSWSTART != 0"]
    #[inline] pub fn test_jswstart(&self) -> bool {
        self.jswstart() != 0
    }

    #[doc="Sets the JSWSTART field."]
    #[inline] pub fn set_jswstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DFSDM enable"]
    #[inline] pub fn dfen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DFEN != 0"]
    #[inline] pub fn test_dfen(&self) -> bool {
        self.dfen() != 0
    }

    #[doc="Sets the DFEN field."]
    #[inline] pub fn set_dfen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Cr1(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awfsel() != 0 { try!(write!(f, " awfsel"))}
        if self.fast() != 0 { try!(write!(f, " fast"))}
        if self.rch() != 0 { try!(write!(f, " rch=0x{:x}", self.rch()))}
        if self.rdmaen() != 0 { try!(write!(f, " rdmaen"))}
        if self.rsync() != 0 { try!(write!(f, " rsync"))}
        if self.rcont() != 0 { try!(write!(f, " rcont"))}
        if self.rswstart() != 0 { try!(write!(f, " rswstart"))}
        if self.jexten() != 0 { try!(write!(f, " jexten=0x{:x}", self.jexten()))}
        if self.jextsel() != 0 { try!(write!(f, " jextsel=0x{:x}", self.jextsel()))}
        if self.jdmaen() != 0 { try!(write!(f, " jdmaen"))}
        if self.jscan() != 0 { try!(write!(f, " jscan"))}
        if self.jsync() != 0 { try!(write!(f, " jsync"))}
        if self.jswstart() != 0 { try!(write!(f, " jswstart"))}
        if self.dfen() != 0 { try!(write!(f, " dfen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Cr2(pub u32);
impl Dfsdm0Cr2 {
    #[doc="Analog watchdog channel selection"]
    #[inline] pub fn awdch(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if AWDCH != 0"]
    #[inline] pub fn test_awdch(&self) -> bool {
        self.awdch() != 0
    }

    #[doc="Sets the AWDCH field."]
    #[inline] pub fn set_awdch<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Extremes detector channel selection"]
    #[inline] pub fn exch(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if EXCH != 0"]
    #[inline] pub fn test_exch(&self) -> bool {
        self.exch() != 0
    }

    #[doc="Sets the EXCH field."]
    #[inline] pub fn set_exch<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock absence interrupt enable"]
    #[inline] pub fn ckabie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABIE != 0"]
    #[inline] pub fn test_ckabie(&self) -> bool {
        self.ckabie() != 0
    }

    #[doc="Sets the CKABIE field."]
    #[inline] pub fn set_ckabie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Short-circuit detector interrupt enable"]
    #[inline] pub fn scdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDIE != 0"]
    #[inline] pub fn test_scdie(&self) -> bool {
        self.scdie() != 0
    }

    #[doc="Sets the SCDIE field."]
    #[inline] pub fn set_scdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Analog watchdog interrupt enable"]
    #[inline] pub fn awdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AWDIE != 0"]
    #[inline] pub fn test_awdie(&self) -> bool {
        self.awdie() != 0
    }

    #[doc="Sets the AWDIE field."]
    #[inline] pub fn set_awdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular data overrun interrupt enable"]
    #[inline] pub fn rovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ROVRIE != 0"]
    #[inline] pub fn test_rovrie(&self) -> bool {
        self.rovrie() != 0
    }

    #[doc="Sets the ROVRIE field."]
    #[inline] pub fn set_rovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Injected data overrun interrupt enable"]
    #[inline] pub fn jovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JOVRIE != 0"]
    #[inline] pub fn test_jovrie(&self) -> bool {
        self.jovrie() != 0
    }

    #[doc="Sets the JOVRIE field."]
    #[inline] pub fn set_jovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Regular end of conversion interrupt enable"]
    #[inline] pub fn reocie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if REOCIE != 0"]
    #[inline] pub fn test_reocie(&self) -> bool {
        self.reocie() != 0
    }

    #[doc="Sets the REOCIE field."]
    #[inline] pub fn set_reocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Injected end of conversion interrupt enable"]
    #[inline] pub fn jeocie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if JEOCIE != 0"]
    #[inline] pub fn test_jeocie(&self) -> bool {
        self.jeocie() != 0
    }

    #[doc="Sets the JEOCIE field."]
    #[inline] pub fn set_jeocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Cr2(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awdch() != 0 { try!(write!(f, " awdch=0x{:x}", self.awdch()))}
        if self.exch() != 0 { try!(write!(f, " exch=0x{:x}", self.exch()))}
        if self.ckabie() != 0 { try!(write!(f, " ckabie"))}
        if self.scdie() != 0 { try!(write!(f, " scdie"))}
        if self.awdie() != 0 { try!(write!(f, " awdie"))}
        if self.rovrie() != 0 { try!(write!(f, " rovrie"))}
        if self.jovrie() != 0 { try!(write!(f, " jovrie"))}
        if self.reocie() != 0 { try!(write!(f, " reocie"))}
        if self.jeocie() != 0 { try!(write!(f, " jeocie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Isr(pub u32);
impl Dfsdm0Isr {
    #[doc="short-circuit detector flag"]
    #[inline] pub fn scdf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if SCDF != 0"]
    #[inline] pub fn test_scdf(&self) -> bool {
        self.scdf() != 0
    }

    #[doc="Sets the SCDF field."]
    #[inline] pub fn set_scdf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock absence flag"]
    #[inline] pub fn ckabf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CKABF != 0"]
    #[inline] pub fn test_ckabf(&self) -> bool {
        self.ckabf() != 0
    }

    #[doc="Sets the CKABF field."]
    #[inline] pub fn set_ckabf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Regular conversion in progress status"]
    #[inline] pub fn rcip(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RCIP != 0"]
    #[inline] pub fn test_rcip(&self) -> bool {
        self.rcip() != 0
    }

    #[doc="Sets the RCIP field."]
    #[inline] pub fn set_rcip<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Injected conversion in progress status"]
    #[inline] pub fn jcip(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if JCIP != 0"]
    #[inline] pub fn test_jcip(&self) -> bool {
        self.jcip() != 0
    }

    #[doc="Sets the JCIP field."]
    #[inline] pub fn set_jcip<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Analog watchdog"]
    #[inline] pub fn awdf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AWDF != 0"]
    #[inline] pub fn test_awdf(&self) -> bool {
        self.awdf() != 0
    }

    #[doc="Sets the AWDF field."]
    #[inline] pub fn set_awdf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular conversion overrun flag"]
    #[inline] pub fn rovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ROVRF != 0"]
    #[inline] pub fn test_rovrf(&self) -> bool {
        self.rovrf() != 0
    }

    #[doc="Sets the ROVRF field."]
    #[inline] pub fn set_rovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Injected conversion overrun flag"]
    #[inline] pub fn jovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JOVRF != 0"]
    #[inline] pub fn test_jovrf(&self) -> bool {
        self.jovrf() != 0
    }

    #[doc="Sets the JOVRF field."]
    #[inline] pub fn set_jovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End of regular conversion flag"]
    #[inline] pub fn reocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if REOCF != 0"]
    #[inline] pub fn test_reocf(&self) -> bool {
        self.reocf() != 0
    }

    #[doc="Sets the REOCF field."]
    #[inline] pub fn set_reocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End of injected conversion flag"]
    #[inline] pub fn jeocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if JEOCF != 0"]
    #[inline] pub fn test_jeocf(&self) -> bool {
        self.jeocf() != 0
    }

    #[doc="Sets the JEOCF field."]
    #[inline] pub fn set_jeocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Isr(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.scdf() != 0 { try!(write!(f, " scdf=0x{:x}", self.scdf()))}
        if self.ckabf() != 0 { try!(write!(f, " ckabf=0x{:x}", self.ckabf()))}
        if self.rcip() != 0 { try!(write!(f, " rcip"))}
        if self.jcip() != 0 { try!(write!(f, " jcip"))}
        if self.awdf() != 0 { try!(write!(f, " awdf"))}
        if self.rovrf() != 0 { try!(write!(f, " rovrf"))}
        if self.jovrf() != 0 { try!(write!(f, " jovrf"))}
        if self.reocf() != 0 { try!(write!(f, " reocf"))}
        if self.jeocf() != 0 { try!(write!(f, " jeocf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt flag clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Icr(pub u32);
impl Dfsdm0Icr {
    #[doc="Clear the short-circuit detector flag"]
    #[inline] pub fn clrscdf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if CLRSCDF != 0"]
    #[inline] pub fn test_clrscdf(&self) -> bool {
        self.clrscdf() != 0
    }

    #[doc="Sets the CLRSCDF field."]
    #[inline] pub fn set_clrscdf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clear the clock absence flag"]
    #[inline] pub fn clrckabf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CLRCKABF != 0"]
    #[inline] pub fn test_clrckabf(&self) -> bool {
        self.clrckabf() != 0
    }

    #[doc="Sets the CLRCKABF field."]
    #[inline] pub fn set_clrckabf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Clear the regular conversion overrun flag"]
    #[inline] pub fn clrrovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CLRROVRF != 0"]
    #[inline] pub fn test_clrrovrf(&self) -> bool {
        self.clrrovrf() != 0
    }

    #[doc="Sets the CLRROVRF field."]
    #[inline] pub fn set_clrrovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear the injected conversion overrun flag"]
    #[inline] pub fn clrjovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CLRJOVRF != 0"]
    #[inline] pub fn test_clrjovrf(&self) -> bool {
        self.clrjovrf() != 0
    }

    #[doc="Sets the CLRJOVRF field."]
    #[inline] pub fn set_clrjovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Dfsdm0Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Icr(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrscdf() != 0 { try!(write!(f, " clrscdf=0x{:x}", self.clrscdf()))}
        if self.clrckabf() != 0 { try!(write!(f, " clrckabf=0x{:x}", self.clrckabf()))}
        if self.clrrovrf() != 0 { try!(write!(f, " clrrovrf"))}
        if self.clrjovrf() != 0 { try!(write!(f, " clrjovrf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected channel group selection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Jchgr(pub u32);
impl Dfsdm0Jchgr {
    #[doc="Injected channel group selection"]
    #[inline] pub fn jchg(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if JCHG != 0"]
    #[inline] pub fn test_jchg(&self) -> bool {
        self.jchg() != 0
    }

    #[doc="Sets the JCHG field."]
    #[inline] pub fn set_jchg<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Jchgr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Jchgr(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Jchgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Jchgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jchg() != 0 { try!(write!(f, " jchg=0x{:x}", self.jchg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Fcr(pub u32);
impl Dfsdm0Fcr {
    #[doc="Sinc filter order"]
    #[inline] pub fn ford(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
    }

    #[doc="Returns true if FORD != 0"]
    #[inline] pub fn test_ford(&self) -> bool {
        self.ford() != 0
    }

    #[doc="Sets the FORD field."]
    #[inline] pub fn set_ford<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Sinc filter oversampling ratio (decimation rate)"]
    #[inline] pub fn fosr(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if FOSR != 0"]
    #[inline] pub fn test_fosr(&self) -> bool {
        self.fosr() != 0
    }

    #[doc="Sets the FOSR field."]
    #[inline] pub fn set_fosr<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Integrator oversampling ratio (averaging length)"]
    #[inline] pub fn iosr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IOSR != 0"]
    #[inline] pub fn test_iosr(&self) -> bool {
        self.iosr() != 0
    }

    #[doc="Sets the IOSR field."]
    #[inline] pub fn set_iosr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Fcr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Fcr(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ford() != 0 { try!(write!(f, " ford=0x{:x}", self.ford()))}
        if self.fosr() != 0 { try!(write!(f, " fosr=0x{:x}", self.fosr()))}
        if self.iosr() != 0 { try!(write!(f, " iosr=0x{:x}", self.iosr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register for injected group"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Jdatar(pub u32);
impl Dfsdm0Jdatar {
    #[doc="Injected group conversion data"]
    #[inline] pub fn jdata(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if JDATA != 0"]
    #[inline] pub fn test_jdata(&self) -> bool {
        self.jdata() != 0
    }

    #[doc="Sets the JDATA field."]
    #[inline] pub fn set_jdata<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Injected channel most recently converted"]
    #[inline] pub fn jdatach(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if JDATACH != 0"]
    #[inline] pub fn test_jdatach(&self) -> bool {
        self.jdatach() != 0
    }

    #[doc="Sets the JDATACH field."]
    #[inline] pub fn set_jdatach<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Jdatar {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Jdatar(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Jdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Jdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jdata() != 0 { try!(write!(f, " jdata=0x{:x}", self.jdata()))}
        if self.jdatach() != 0 { try!(write!(f, " jdatach=0x{:x}", self.jdatach()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register for the regular channel"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Rdatar(pub u32);
impl Dfsdm0Rdatar {
    #[doc="Regular channel conversion data"]
    #[inline] pub fn rdata(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if RDATA != 0"]
    #[inline] pub fn test_rdata(&self) -> bool {
        self.rdata() != 0
    }

    #[doc="Sets the RDATA field."]
    #[inline] pub fn set_rdata<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Regular channel pending data"]
    #[inline] pub fn rpend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RPEND != 0"]
    #[inline] pub fn test_rpend(&self) -> bool {
        self.rpend() != 0
    }

    #[doc="Sets the RPEND field."]
    #[inline] pub fn set_rpend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular channel most recently converted"]
    #[inline] pub fn rdatach(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if RDATACH != 0"]
    #[inline] pub fn test_rdatach(&self) -> bool {
        self.rdatach() != 0
    }

    #[doc="Sets the RDATACH field."]
    #[inline] pub fn set_rdatach<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Rdatar {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Rdatar(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Rdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Rdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdata() != 0 { try!(write!(f, " rdata=0x{:x}", self.rdata()))}
        if self.rpend() != 0 { try!(write!(f, " rpend"))}
        if self.rdatach() != 0 { try!(write!(f, " rdatach=0x{:x}", self.rdatach()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog high threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Awhtr(pub u32);
impl Dfsdm0Awhtr {
    #[doc="Analog watchdog high threshold"]
    #[inline] pub fn awht(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if AWHT != 0"]
    #[inline] pub fn test_awht(&self) -> bool {
        self.awht() != 0
    }

    #[doc="Sets the AWHT field."]
    #[inline] pub fn set_awht<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Break signal assignment to analog watchdog high threshold event"]
    #[inline] pub fn bkawh(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BKAWH != 0"]
    #[inline] pub fn test_bkawh(&self) -> bool {
        self.bkawh() != 0
    }

    #[doc="Sets the BKAWH field."]
    #[inline] pub fn set_bkawh<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Awhtr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Awhtr(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Awhtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Awhtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awht() != 0 { try!(write!(f, " awht=0x{:x}", self.awht()))}
        if self.bkawh() != 0 { try!(write!(f, " bkawh=0x{:x}", self.bkawh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog low threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Awltr(pub u32);
impl Dfsdm0Awltr {
    #[doc="Analog watchdog low threshold"]
    #[inline] pub fn awlt(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if AWLT != 0"]
    #[inline] pub fn test_awlt(&self) -> bool {
        self.awlt() != 0
    }

    #[doc="Sets the AWLT field."]
    #[inline] pub fn set_awlt<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Break signal assignment to analog watchdog low threshold event"]
    #[inline] pub fn bkawl(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BKAWL != 0"]
    #[inline] pub fn test_bkawl(&self) -> bool {
        self.bkawl() != 0
    }

    #[doc="Sets the BKAWL field."]
    #[inline] pub fn set_bkawl<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Awltr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Awltr(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Awltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Awltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awlt() != 0 { try!(write!(f, " awlt=0x{:x}", self.awlt()))}
        if self.bkawl() != 0 { try!(write!(f, " bkawl=0x{:x}", self.bkawl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Awsr(pub u32);
impl Dfsdm0Awsr {
    #[doc="Analog watchdog high threshold flag"]
    #[inline] pub fn awhtf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if AWHTF != 0"]
    #[inline] pub fn test_awhtf(&self) -> bool {
        self.awhtf() != 0
    }

    #[doc="Sets the AWHTF field."]
    #[inline] pub fn set_awhtf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Analog watchdog low threshold flag"]
    #[inline] pub fn awltf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if AWLTF != 0"]
    #[inline] pub fn test_awltf(&self) -> bool {
        self.awltf() != 0
    }

    #[doc="Sets the AWLTF field."]
    #[inline] pub fn set_awltf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Awsr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Awsr(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Awsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Awsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awhtf() != 0 { try!(write!(f, " awhtf=0x{:x}", self.awhtf()))}
        if self.awltf() != 0 { try!(write!(f, " awltf=0x{:x}", self.awltf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog clear flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Awcfr(pub u32);
impl Dfsdm0Awcfr {
    #[doc="Clear the analog watchdog high threshold flag"]
    #[inline] pub fn clrawhtf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if CLRAWHTF != 0"]
    #[inline] pub fn test_clrawhtf(&self) -> bool {
        self.clrawhtf() != 0
    }

    #[doc="Sets the CLRAWHTF field."]
    #[inline] pub fn set_clrawhtf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clear the analog watchdog low threshold flag"]
    #[inline] pub fn clrawltf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLRAWLTF != 0"]
    #[inline] pub fn test_clrawltf(&self) -> bool {
        self.clrawltf() != 0
    }

    #[doc="Sets the CLRAWLTF field."]
    #[inline] pub fn set_clrawltf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Awcfr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Awcfr(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Awcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Awcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrawhtf() != 0 { try!(write!(f, " clrawhtf=0x{:x}", self.clrawhtf()))}
        if self.clrawltf() != 0 { try!(write!(f, " clrawltf=0x{:x}", self.clrawltf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Extremes detector maximum register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Exmax(pub u32);
impl Dfsdm0Exmax {
    #[doc="Extremes detector maximum value"]
    #[inline] pub fn exmax(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if EXMAX != 0"]
    #[inline] pub fn test_exmax(&self) -> bool {
        self.exmax() != 0
    }

    #[doc="Sets the EXMAX field."]
    #[inline] pub fn set_exmax<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Extremes detector maximum data channel"]
    #[inline] pub fn exmaxch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXMAXCH != 0"]
    #[inline] pub fn test_exmaxch(&self) -> bool {
        self.exmaxch() != 0
    }

    #[doc="Sets the EXMAXCH field."]
    #[inline] pub fn set_exmaxch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Exmax {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Exmax(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Exmax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Exmax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exmax() != 0 { try!(write!(f, " exmax=0x{:x}", self.exmax()))}
        if self.exmaxch() != 0 { try!(write!(f, " exmaxch=0x{:x}", self.exmaxch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Extremes detector minimum register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Exmin(pub u32);
impl Dfsdm0Exmin {
    #[doc="EXMIN"]
    #[inline] pub fn exmin(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if EXMIN != 0"]
    #[inline] pub fn test_exmin(&self) -> bool {
        self.exmin() != 0
    }

    #[doc="Sets the EXMIN field."]
    #[inline] pub fn set_exmin<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Extremes detector minimum data channel"]
    #[inline] pub fn exminch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXMINCH != 0"]
    #[inline] pub fn test_exminch(&self) -> bool {
        self.exminch() != 0
    }

    #[doc="Sets the EXMINCH field."]
    #[inline] pub fn set_exminch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm0Exmin {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Exmin(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Exmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Exmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exmin() != 0 { try!(write!(f, " exmin=0x{:x}", self.exmin()))}
        if self.exminch() != 0 { try!(write!(f, " exminch=0x{:x}", self.exminch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="conversion timer register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm0Cnvtimr(pub u32);
impl Dfsdm0Cnvtimr {
    #[doc="28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN"]
    #[inline] pub fn cnvcnt(&self) -> bits::U28 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfffffff) as u32) } // [31:4]
    }

    #[doc="Returns true if CNVCNT != 0"]
    #[inline] pub fn test_cnvcnt(&self) -> bool {
        self.cnvcnt() != 0
    }

    #[doc="Sets the CNVCNT field."]
    #[inline] pub fn set_cnvcnt<V: Into<bits::U28>>(mut self, value: V) -> Self {
        let value: bits::U28 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Dfsdm0Cnvtimr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm0Cnvtimr(other)
    }
}

impl ::core::fmt::Display for Dfsdm0Cnvtimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm0Cnvtimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnvcnt() != 0 { try!(write!(f, " cnvcnt=0x{:x}", self.cnvcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Cr1(pub u32);
impl Dfsdm1Cr1 {
    #[doc="Analog watchdog fast mode select"]
    #[inline] pub fn awfsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if AWFSEL != 0"]
    #[inline] pub fn test_awfsel(&self) -> bool {
        self.awfsel() != 0
    }

    #[doc="Sets the AWFSEL field."]
    #[inline] pub fn set_awfsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Fast conversion mode selection for regular conversions"]
    #[inline] pub fn fast(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if FAST != 0"]
    #[inline] pub fn test_fast(&self) -> bool {
        self.fast() != 0
    }

    #[doc="Sets the FAST field."]
    #[inline] pub fn set_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Regular channel selection"]
    #[inline] pub fn rch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if RCH != 0"]
    #[inline] pub fn test_rch(&self) -> bool {
        self.rch() != 0
    }

    #[doc="Sets the RCH field."]
    #[inline] pub fn set_rch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DMA channel enabled to read data for the regular conversion"]
    #[inline] pub fn rdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RDMAEN != 0"]
    #[inline] pub fn test_rdmaen(&self) -> bool {
        self.rdmaen() != 0
    }

    #[doc="Sets the RDMAEN field."]
    #[inline] pub fn set_rdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Launch regular conversion synchronously with DFSDM0"]
    #[inline] pub fn rsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RSYNC != 0"]
    #[inline] pub fn test_rsync(&self) -> bool {
        self.rsync() != 0
    }

    #[doc="Sets the RSYNC field."]
    #[inline] pub fn set_rsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Continuous mode selection for regular conversions"]
    #[inline] pub fn rcont(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RCONT != 0"]
    #[inline] pub fn test_rcont(&self) -> bool {
        self.rcont() != 0
    }

    #[doc="Sets the RCONT field."]
    #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Software start of a conversion on the regular channel"]
    #[inline] pub fn rswstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RSWSTART != 0"]
    #[inline] pub fn test_rswstart(&self) -> bool {
        self.rswstart() != 0
    }

    #[doc="Sets the RSWSTART field."]
    #[inline] pub fn set_rswstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Trigger enable and trigger edge selection for injected conversions"]
    #[inline] pub fn jexten(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if JEXTEN != 0"]
    #[inline] pub fn test_jexten(&self) -> bool {
        self.jexten() != 0
    }

    #[doc="Sets the JEXTEN field."]
    #[inline] pub fn set_jexten<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Trigger signal selection for launching injected conversions"]
    #[inline] pub fn jextsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if JEXTSEL != 0"]
    #[inline] pub fn test_jextsel(&self) -> bool {
        self.jextsel() != 0
    }

    #[doc="Sets the JEXTSEL field."]
    #[inline] pub fn set_jextsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA channel enabled to read data for the injected channel group"]
    #[inline] pub fn jdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if JDMAEN != 0"]
    #[inline] pub fn test_jdmaen(&self) -> bool {
        self.jdmaen() != 0
    }

    #[doc="Sets the JDMAEN field."]
    #[inline] pub fn set_jdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Scanning conversion mode for injected conversions"]
    #[inline] pub fn jscan(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if JSCAN != 0"]
    #[inline] pub fn test_jscan(&self) -> bool {
        self.jscan() != 0
    }

    #[doc="Sets the JSCAN field."]
    #[inline] pub fn set_jscan<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline] pub fn jsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if JSYNC != 0"]
    #[inline] pub fn test_jsync(&self) -> bool {
        self.jsync() != 0
    }

    #[doc="Sets the JSYNC field."]
    #[inline] pub fn set_jsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Start a conversion of the injected group of channels"]
    #[inline] pub fn jswstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if JSWSTART != 0"]
    #[inline] pub fn test_jswstart(&self) -> bool {
        self.jswstart() != 0
    }

    #[doc="Sets the JSWSTART field."]
    #[inline] pub fn set_jswstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DFSDM enable"]
    #[inline] pub fn dfen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DFEN != 0"]
    #[inline] pub fn test_dfen(&self) -> bool {
        self.dfen() != 0
    }

    #[doc="Sets the DFEN field."]
    #[inline] pub fn set_dfen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Cr1(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awfsel() != 0 { try!(write!(f, " awfsel"))}
        if self.fast() != 0 { try!(write!(f, " fast"))}
        if self.rch() != 0 { try!(write!(f, " rch=0x{:x}", self.rch()))}
        if self.rdmaen() != 0 { try!(write!(f, " rdmaen"))}
        if self.rsync() != 0 { try!(write!(f, " rsync"))}
        if self.rcont() != 0 { try!(write!(f, " rcont"))}
        if self.rswstart() != 0 { try!(write!(f, " rswstart"))}
        if self.jexten() != 0 { try!(write!(f, " jexten=0x{:x}", self.jexten()))}
        if self.jextsel() != 0 { try!(write!(f, " jextsel=0x{:x}", self.jextsel()))}
        if self.jdmaen() != 0 { try!(write!(f, " jdmaen"))}
        if self.jscan() != 0 { try!(write!(f, " jscan"))}
        if self.jsync() != 0 { try!(write!(f, " jsync"))}
        if self.jswstart() != 0 { try!(write!(f, " jswstart"))}
        if self.dfen() != 0 { try!(write!(f, " dfen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Cr2(pub u32);
impl Dfsdm1Cr2 {
    #[doc="Analog watchdog channel selection"]
    #[inline] pub fn awdch(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if AWDCH != 0"]
    #[inline] pub fn test_awdch(&self) -> bool {
        self.awdch() != 0
    }

    #[doc="Sets the AWDCH field."]
    #[inline] pub fn set_awdch<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Extremes detector channel selection"]
    #[inline] pub fn exch(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if EXCH != 0"]
    #[inline] pub fn test_exch(&self) -> bool {
        self.exch() != 0
    }

    #[doc="Sets the EXCH field."]
    #[inline] pub fn set_exch<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock absence interrupt enable"]
    #[inline] pub fn ckabie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABIE != 0"]
    #[inline] pub fn test_ckabie(&self) -> bool {
        self.ckabie() != 0
    }

    #[doc="Sets the CKABIE field."]
    #[inline] pub fn set_ckabie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Short-circuit detector interrupt enable"]
    #[inline] pub fn scdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDIE != 0"]
    #[inline] pub fn test_scdie(&self) -> bool {
        self.scdie() != 0
    }

    #[doc="Sets the SCDIE field."]
    #[inline] pub fn set_scdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Analog watchdog interrupt enable"]
    #[inline] pub fn awdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AWDIE != 0"]
    #[inline] pub fn test_awdie(&self) -> bool {
        self.awdie() != 0
    }

    #[doc="Sets the AWDIE field."]
    #[inline] pub fn set_awdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular data overrun interrupt enable"]
    #[inline] pub fn rovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ROVRIE != 0"]
    #[inline] pub fn test_rovrie(&self) -> bool {
        self.rovrie() != 0
    }

    #[doc="Sets the ROVRIE field."]
    #[inline] pub fn set_rovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Injected data overrun interrupt enable"]
    #[inline] pub fn jovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JOVRIE != 0"]
    #[inline] pub fn test_jovrie(&self) -> bool {
        self.jovrie() != 0
    }

    #[doc="Sets the JOVRIE field."]
    #[inline] pub fn set_jovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Regular end of conversion interrupt enable"]
    #[inline] pub fn reocie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if REOCIE != 0"]
    #[inline] pub fn test_reocie(&self) -> bool {
        self.reocie() != 0
    }

    #[doc="Sets the REOCIE field."]
    #[inline] pub fn set_reocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Injected end of conversion interrupt enable"]
    #[inline] pub fn jeocie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if JEOCIE != 0"]
    #[inline] pub fn test_jeocie(&self) -> bool {
        self.jeocie() != 0
    }

    #[doc="Sets the JEOCIE field."]
    #[inline] pub fn set_jeocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Cr2(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awdch() != 0 { try!(write!(f, " awdch=0x{:x}", self.awdch()))}
        if self.exch() != 0 { try!(write!(f, " exch=0x{:x}", self.exch()))}
        if self.ckabie() != 0 { try!(write!(f, " ckabie"))}
        if self.scdie() != 0 { try!(write!(f, " scdie"))}
        if self.awdie() != 0 { try!(write!(f, " awdie"))}
        if self.rovrie() != 0 { try!(write!(f, " rovrie"))}
        if self.jovrie() != 0 { try!(write!(f, " jovrie"))}
        if self.reocie() != 0 { try!(write!(f, " reocie"))}
        if self.jeocie() != 0 { try!(write!(f, " jeocie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Isr(pub u32);
impl Dfsdm1Isr {
    #[doc="short-circuit detector flag"]
    #[inline] pub fn scdf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if SCDF != 0"]
    #[inline] pub fn test_scdf(&self) -> bool {
        self.scdf() != 0
    }

    #[doc="Sets the SCDF field."]
    #[inline] pub fn set_scdf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock absence flag"]
    #[inline] pub fn ckabf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CKABF != 0"]
    #[inline] pub fn test_ckabf(&self) -> bool {
        self.ckabf() != 0
    }

    #[doc="Sets the CKABF field."]
    #[inline] pub fn set_ckabf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Regular conversion in progress status"]
    #[inline] pub fn rcip(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RCIP != 0"]
    #[inline] pub fn test_rcip(&self) -> bool {
        self.rcip() != 0
    }

    #[doc="Sets the RCIP field."]
    #[inline] pub fn set_rcip<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Injected conversion in progress status"]
    #[inline] pub fn jcip(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if JCIP != 0"]
    #[inline] pub fn test_jcip(&self) -> bool {
        self.jcip() != 0
    }

    #[doc="Sets the JCIP field."]
    #[inline] pub fn set_jcip<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Analog watchdog"]
    #[inline] pub fn awdf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AWDF != 0"]
    #[inline] pub fn test_awdf(&self) -> bool {
        self.awdf() != 0
    }

    #[doc="Sets the AWDF field."]
    #[inline] pub fn set_awdf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular conversion overrun flag"]
    #[inline] pub fn rovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ROVRF != 0"]
    #[inline] pub fn test_rovrf(&self) -> bool {
        self.rovrf() != 0
    }

    #[doc="Sets the ROVRF field."]
    #[inline] pub fn set_rovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Injected conversion overrun flag"]
    #[inline] pub fn jovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JOVRF != 0"]
    #[inline] pub fn test_jovrf(&self) -> bool {
        self.jovrf() != 0
    }

    #[doc="Sets the JOVRF field."]
    #[inline] pub fn set_jovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End of regular conversion flag"]
    #[inline] pub fn reocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if REOCF != 0"]
    #[inline] pub fn test_reocf(&self) -> bool {
        self.reocf() != 0
    }

    #[doc="Sets the REOCF field."]
    #[inline] pub fn set_reocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End of injected conversion flag"]
    #[inline] pub fn jeocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if JEOCF != 0"]
    #[inline] pub fn test_jeocf(&self) -> bool {
        self.jeocf() != 0
    }

    #[doc="Sets the JEOCF field."]
    #[inline] pub fn set_jeocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Isr(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.scdf() != 0 { try!(write!(f, " scdf=0x{:x}", self.scdf()))}
        if self.ckabf() != 0 { try!(write!(f, " ckabf=0x{:x}", self.ckabf()))}
        if self.rcip() != 0 { try!(write!(f, " rcip"))}
        if self.jcip() != 0 { try!(write!(f, " jcip"))}
        if self.awdf() != 0 { try!(write!(f, " awdf"))}
        if self.rovrf() != 0 { try!(write!(f, " rovrf"))}
        if self.jovrf() != 0 { try!(write!(f, " jovrf"))}
        if self.reocf() != 0 { try!(write!(f, " reocf"))}
        if self.jeocf() != 0 { try!(write!(f, " jeocf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt flag clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Icr(pub u32);
impl Dfsdm1Icr {
    #[doc="Clear the short-circuit detector flag"]
    #[inline] pub fn clrscdf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if CLRSCDF != 0"]
    #[inline] pub fn test_clrscdf(&self) -> bool {
        self.clrscdf() != 0
    }

    #[doc="Sets the CLRSCDF field."]
    #[inline] pub fn set_clrscdf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clear the clock absence flag"]
    #[inline] pub fn clrckabf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CLRCKABF != 0"]
    #[inline] pub fn test_clrckabf(&self) -> bool {
        self.clrckabf() != 0
    }

    #[doc="Sets the CLRCKABF field."]
    #[inline] pub fn set_clrckabf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Clear the regular conversion overrun flag"]
    #[inline] pub fn clrrovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CLRROVRF != 0"]
    #[inline] pub fn test_clrrovrf(&self) -> bool {
        self.clrrovrf() != 0
    }

    #[doc="Sets the CLRROVRF field."]
    #[inline] pub fn set_clrrovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear the injected conversion overrun flag"]
    #[inline] pub fn clrjovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CLRJOVRF != 0"]
    #[inline] pub fn test_clrjovrf(&self) -> bool {
        self.clrjovrf() != 0
    }

    #[doc="Sets the CLRJOVRF field."]
    #[inline] pub fn set_clrjovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Dfsdm1Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Icr(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrscdf() != 0 { try!(write!(f, " clrscdf=0x{:x}", self.clrscdf()))}
        if self.clrckabf() != 0 { try!(write!(f, " clrckabf=0x{:x}", self.clrckabf()))}
        if self.clrrovrf() != 0 { try!(write!(f, " clrrovrf"))}
        if self.clrjovrf() != 0 { try!(write!(f, " clrjovrf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected channel group selection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Jchgr(pub u32);
impl Dfsdm1Jchgr {
    #[doc="Injected channel group selection"]
    #[inline] pub fn jchg(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if JCHG != 0"]
    #[inline] pub fn test_jchg(&self) -> bool {
        self.jchg() != 0
    }

    #[doc="Sets the JCHG field."]
    #[inline] pub fn set_jchg<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Jchgr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Jchgr(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Jchgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Jchgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jchg() != 0 { try!(write!(f, " jchg=0x{:x}", self.jchg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Fcr(pub u32);
impl Dfsdm1Fcr {
    #[doc="Sinc filter order"]
    #[inline] pub fn ford(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
    }

    #[doc="Returns true if FORD != 0"]
    #[inline] pub fn test_ford(&self) -> bool {
        self.ford() != 0
    }

    #[doc="Sets the FORD field."]
    #[inline] pub fn set_ford<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Sinc filter oversampling ratio (decimation rate)"]
    #[inline] pub fn fosr(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if FOSR != 0"]
    #[inline] pub fn test_fosr(&self) -> bool {
        self.fosr() != 0
    }

    #[doc="Sets the FOSR field."]
    #[inline] pub fn set_fosr<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Integrator oversampling ratio (averaging length)"]
    #[inline] pub fn iosr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IOSR != 0"]
    #[inline] pub fn test_iosr(&self) -> bool {
        self.iosr() != 0
    }

    #[doc="Sets the IOSR field."]
    #[inline] pub fn set_iosr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Fcr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Fcr(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ford() != 0 { try!(write!(f, " ford=0x{:x}", self.ford()))}
        if self.fosr() != 0 { try!(write!(f, " fosr=0x{:x}", self.fosr()))}
        if self.iosr() != 0 { try!(write!(f, " iosr=0x{:x}", self.iosr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register for injected group"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Jdatar(pub u32);
impl Dfsdm1Jdatar {
    #[doc="Injected group conversion data"]
    #[inline] pub fn jdata(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if JDATA != 0"]
    #[inline] pub fn test_jdata(&self) -> bool {
        self.jdata() != 0
    }

    #[doc="Sets the JDATA field."]
    #[inline] pub fn set_jdata<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Injected channel most recently converted"]
    #[inline] pub fn jdatach(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if JDATACH != 0"]
    #[inline] pub fn test_jdatach(&self) -> bool {
        self.jdatach() != 0
    }

    #[doc="Sets the JDATACH field."]
    #[inline] pub fn set_jdatach<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Jdatar {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Jdatar(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Jdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Jdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jdata() != 0 { try!(write!(f, " jdata=0x{:x}", self.jdata()))}
        if self.jdatach() != 0 { try!(write!(f, " jdatach=0x{:x}", self.jdatach()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register for the regular channel"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Rdatar(pub u32);
impl Dfsdm1Rdatar {
    #[doc="Regular channel conversion data"]
    #[inline] pub fn rdata(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if RDATA != 0"]
    #[inline] pub fn test_rdata(&self) -> bool {
        self.rdata() != 0
    }

    #[doc="Sets the RDATA field."]
    #[inline] pub fn set_rdata<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Regular channel pending data"]
    #[inline] pub fn rpend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RPEND != 0"]
    #[inline] pub fn test_rpend(&self) -> bool {
        self.rpend() != 0
    }

    #[doc="Sets the RPEND field."]
    #[inline] pub fn set_rpend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular channel most recently converted"]
    #[inline] pub fn rdatach(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if RDATACH != 0"]
    #[inline] pub fn test_rdatach(&self) -> bool {
        self.rdatach() != 0
    }

    #[doc="Sets the RDATACH field."]
    #[inline] pub fn set_rdatach<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Rdatar {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Rdatar(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Rdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Rdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdata() != 0 { try!(write!(f, " rdata=0x{:x}", self.rdata()))}
        if self.rpend() != 0 { try!(write!(f, " rpend"))}
        if self.rdatach() != 0 { try!(write!(f, " rdatach=0x{:x}", self.rdatach()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog high threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Awhtr(pub u32);
impl Dfsdm1Awhtr {
    #[doc="Analog watchdog high threshold"]
    #[inline] pub fn awht(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if AWHT != 0"]
    #[inline] pub fn test_awht(&self) -> bool {
        self.awht() != 0
    }

    #[doc="Sets the AWHT field."]
    #[inline] pub fn set_awht<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Break signal assignment to analog watchdog high threshold event"]
    #[inline] pub fn bkawh(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BKAWH != 0"]
    #[inline] pub fn test_bkawh(&self) -> bool {
        self.bkawh() != 0
    }

    #[doc="Sets the BKAWH field."]
    #[inline] pub fn set_bkawh<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Awhtr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Awhtr(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Awhtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Awhtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awht() != 0 { try!(write!(f, " awht=0x{:x}", self.awht()))}
        if self.bkawh() != 0 { try!(write!(f, " bkawh=0x{:x}", self.bkawh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog low threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Awltr(pub u32);
impl Dfsdm1Awltr {
    #[doc="Analog watchdog low threshold"]
    #[inline] pub fn awlt(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if AWLT != 0"]
    #[inline] pub fn test_awlt(&self) -> bool {
        self.awlt() != 0
    }

    #[doc="Sets the AWLT field."]
    #[inline] pub fn set_awlt<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Break signal assignment to analog watchdog low threshold event"]
    #[inline] pub fn bkawl(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BKAWL != 0"]
    #[inline] pub fn test_bkawl(&self) -> bool {
        self.bkawl() != 0
    }

    #[doc="Sets the BKAWL field."]
    #[inline] pub fn set_bkawl<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Awltr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Awltr(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Awltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Awltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awlt() != 0 { try!(write!(f, " awlt=0x{:x}", self.awlt()))}
        if self.bkawl() != 0 { try!(write!(f, " bkawl=0x{:x}", self.bkawl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Awsr(pub u32);
impl Dfsdm1Awsr {
    #[doc="Analog watchdog high threshold flag"]
    #[inline] pub fn awhtf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if AWHTF != 0"]
    #[inline] pub fn test_awhtf(&self) -> bool {
        self.awhtf() != 0
    }

    #[doc="Sets the AWHTF field."]
    #[inline] pub fn set_awhtf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Analog watchdog low threshold flag"]
    #[inline] pub fn awltf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if AWLTF != 0"]
    #[inline] pub fn test_awltf(&self) -> bool {
        self.awltf() != 0
    }

    #[doc="Sets the AWLTF field."]
    #[inline] pub fn set_awltf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Awsr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Awsr(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Awsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Awsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awhtf() != 0 { try!(write!(f, " awhtf=0x{:x}", self.awhtf()))}
        if self.awltf() != 0 { try!(write!(f, " awltf=0x{:x}", self.awltf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog clear flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Awcfr(pub u32);
impl Dfsdm1Awcfr {
    #[doc="Clear the analog watchdog high threshold flag"]
    #[inline] pub fn clrawhtf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if CLRAWHTF != 0"]
    #[inline] pub fn test_clrawhtf(&self) -> bool {
        self.clrawhtf() != 0
    }

    #[doc="Sets the CLRAWHTF field."]
    #[inline] pub fn set_clrawhtf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clear the analog watchdog low threshold flag"]
    #[inline] pub fn clrawltf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLRAWLTF != 0"]
    #[inline] pub fn test_clrawltf(&self) -> bool {
        self.clrawltf() != 0
    }

    #[doc="Sets the CLRAWLTF field."]
    #[inline] pub fn set_clrawltf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Awcfr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Awcfr(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Awcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Awcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrawhtf() != 0 { try!(write!(f, " clrawhtf=0x{:x}", self.clrawhtf()))}
        if self.clrawltf() != 0 { try!(write!(f, " clrawltf=0x{:x}", self.clrawltf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Extremes detector maximum register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Exmax(pub u32);
impl Dfsdm1Exmax {
    #[doc="Extremes detector maximum value"]
    #[inline] pub fn exmax(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if EXMAX != 0"]
    #[inline] pub fn test_exmax(&self) -> bool {
        self.exmax() != 0
    }

    #[doc="Sets the EXMAX field."]
    #[inline] pub fn set_exmax<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Extremes detector maximum data channel"]
    #[inline] pub fn exmaxch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXMAXCH != 0"]
    #[inline] pub fn test_exmaxch(&self) -> bool {
        self.exmaxch() != 0
    }

    #[doc="Sets the EXMAXCH field."]
    #[inline] pub fn set_exmaxch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Exmax {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Exmax(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Exmax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Exmax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exmax() != 0 { try!(write!(f, " exmax=0x{:x}", self.exmax()))}
        if self.exmaxch() != 0 { try!(write!(f, " exmaxch=0x{:x}", self.exmaxch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Extremes detector minimum register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Exmin(pub u32);
impl Dfsdm1Exmin {
    #[doc="EXMIN"]
    #[inline] pub fn exmin(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if EXMIN != 0"]
    #[inline] pub fn test_exmin(&self) -> bool {
        self.exmin() != 0
    }

    #[doc="Sets the EXMIN field."]
    #[inline] pub fn set_exmin<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Extremes detector minimum data channel"]
    #[inline] pub fn exminch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXMINCH != 0"]
    #[inline] pub fn test_exminch(&self) -> bool {
        self.exminch() != 0
    }

    #[doc="Sets the EXMINCH field."]
    #[inline] pub fn set_exminch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm1Exmin {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Exmin(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Exmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Exmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exmin() != 0 { try!(write!(f, " exmin=0x{:x}", self.exmin()))}
        if self.exminch() != 0 { try!(write!(f, " exminch=0x{:x}", self.exminch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="conversion timer register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm1Cnvtimr(pub u32);
impl Dfsdm1Cnvtimr {
    #[doc="28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN"]
    #[inline] pub fn cnvcnt(&self) -> bits::U28 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfffffff) as u32) } // [31:4]
    }

    #[doc="Returns true if CNVCNT != 0"]
    #[inline] pub fn test_cnvcnt(&self) -> bool {
        self.cnvcnt() != 0
    }

    #[doc="Sets the CNVCNT field."]
    #[inline] pub fn set_cnvcnt<V: Into<bits::U28>>(mut self, value: V) -> Self {
        let value: bits::U28 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Dfsdm1Cnvtimr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm1Cnvtimr(other)
    }
}

impl ::core::fmt::Display for Dfsdm1Cnvtimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm1Cnvtimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnvcnt() != 0 { try!(write!(f, " cnvcnt=0x{:x}", self.cnvcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Cr1(pub u32);
impl Dfsdm2Cr1 {
    #[doc="Analog watchdog fast mode select"]
    #[inline] pub fn awfsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if AWFSEL != 0"]
    #[inline] pub fn test_awfsel(&self) -> bool {
        self.awfsel() != 0
    }

    #[doc="Sets the AWFSEL field."]
    #[inline] pub fn set_awfsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Fast conversion mode selection for regular conversions"]
    #[inline] pub fn fast(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if FAST != 0"]
    #[inline] pub fn test_fast(&self) -> bool {
        self.fast() != 0
    }

    #[doc="Sets the FAST field."]
    #[inline] pub fn set_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Regular channel selection"]
    #[inline] pub fn rch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if RCH != 0"]
    #[inline] pub fn test_rch(&self) -> bool {
        self.rch() != 0
    }

    #[doc="Sets the RCH field."]
    #[inline] pub fn set_rch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DMA channel enabled to read data for the regular conversion"]
    #[inline] pub fn rdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RDMAEN != 0"]
    #[inline] pub fn test_rdmaen(&self) -> bool {
        self.rdmaen() != 0
    }

    #[doc="Sets the RDMAEN field."]
    #[inline] pub fn set_rdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Launch regular conversion synchronously with DFSDM0"]
    #[inline] pub fn rsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RSYNC != 0"]
    #[inline] pub fn test_rsync(&self) -> bool {
        self.rsync() != 0
    }

    #[doc="Sets the RSYNC field."]
    #[inline] pub fn set_rsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Continuous mode selection for regular conversions"]
    #[inline] pub fn rcont(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RCONT != 0"]
    #[inline] pub fn test_rcont(&self) -> bool {
        self.rcont() != 0
    }

    #[doc="Sets the RCONT field."]
    #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Software start of a conversion on the regular channel"]
    #[inline] pub fn rswstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RSWSTART != 0"]
    #[inline] pub fn test_rswstart(&self) -> bool {
        self.rswstart() != 0
    }

    #[doc="Sets the RSWSTART field."]
    #[inline] pub fn set_rswstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Trigger enable and trigger edge selection for injected conversions"]
    #[inline] pub fn jexten(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if JEXTEN != 0"]
    #[inline] pub fn test_jexten(&self) -> bool {
        self.jexten() != 0
    }

    #[doc="Sets the JEXTEN field."]
    #[inline] pub fn set_jexten<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Trigger signal selection for launching injected conversions"]
    #[inline] pub fn jextsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if JEXTSEL != 0"]
    #[inline] pub fn test_jextsel(&self) -> bool {
        self.jextsel() != 0
    }

    #[doc="Sets the JEXTSEL field."]
    #[inline] pub fn set_jextsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA channel enabled to read data for the injected channel group"]
    #[inline] pub fn jdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if JDMAEN != 0"]
    #[inline] pub fn test_jdmaen(&self) -> bool {
        self.jdmaen() != 0
    }

    #[doc="Sets the JDMAEN field."]
    #[inline] pub fn set_jdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Scanning conversion mode for injected conversions"]
    #[inline] pub fn jscan(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if JSCAN != 0"]
    #[inline] pub fn test_jscan(&self) -> bool {
        self.jscan() != 0
    }

    #[doc="Sets the JSCAN field."]
    #[inline] pub fn set_jscan<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline] pub fn jsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if JSYNC != 0"]
    #[inline] pub fn test_jsync(&self) -> bool {
        self.jsync() != 0
    }

    #[doc="Sets the JSYNC field."]
    #[inline] pub fn set_jsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Start a conversion of the injected group of channels"]
    #[inline] pub fn jswstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if JSWSTART != 0"]
    #[inline] pub fn test_jswstart(&self) -> bool {
        self.jswstart() != 0
    }

    #[doc="Sets the JSWSTART field."]
    #[inline] pub fn set_jswstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DFSDM enable"]
    #[inline] pub fn dfen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DFEN != 0"]
    #[inline] pub fn test_dfen(&self) -> bool {
        self.dfen() != 0
    }

    #[doc="Sets the DFEN field."]
    #[inline] pub fn set_dfen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Cr1(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awfsel() != 0 { try!(write!(f, " awfsel"))}
        if self.fast() != 0 { try!(write!(f, " fast"))}
        if self.rch() != 0 { try!(write!(f, " rch=0x{:x}", self.rch()))}
        if self.rdmaen() != 0 { try!(write!(f, " rdmaen"))}
        if self.rsync() != 0 { try!(write!(f, " rsync"))}
        if self.rcont() != 0 { try!(write!(f, " rcont"))}
        if self.rswstart() != 0 { try!(write!(f, " rswstart"))}
        if self.jexten() != 0 { try!(write!(f, " jexten=0x{:x}", self.jexten()))}
        if self.jextsel() != 0 { try!(write!(f, " jextsel=0x{:x}", self.jextsel()))}
        if self.jdmaen() != 0 { try!(write!(f, " jdmaen"))}
        if self.jscan() != 0 { try!(write!(f, " jscan"))}
        if self.jsync() != 0 { try!(write!(f, " jsync"))}
        if self.jswstart() != 0 { try!(write!(f, " jswstart"))}
        if self.dfen() != 0 { try!(write!(f, " dfen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Cr2(pub u32);
impl Dfsdm2Cr2 {
    #[doc="Analog watchdog channel selection"]
    #[inline] pub fn awdch(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if AWDCH != 0"]
    #[inline] pub fn test_awdch(&self) -> bool {
        self.awdch() != 0
    }

    #[doc="Sets the AWDCH field."]
    #[inline] pub fn set_awdch<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Extremes detector channel selection"]
    #[inline] pub fn exch(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if EXCH != 0"]
    #[inline] pub fn test_exch(&self) -> bool {
        self.exch() != 0
    }

    #[doc="Sets the EXCH field."]
    #[inline] pub fn set_exch<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock absence interrupt enable"]
    #[inline] pub fn ckabie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABIE != 0"]
    #[inline] pub fn test_ckabie(&self) -> bool {
        self.ckabie() != 0
    }

    #[doc="Sets the CKABIE field."]
    #[inline] pub fn set_ckabie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Short-circuit detector interrupt enable"]
    #[inline] pub fn scdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDIE != 0"]
    #[inline] pub fn test_scdie(&self) -> bool {
        self.scdie() != 0
    }

    #[doc="Sets the SCDIE field."]
    #[inline] pub fn set_scdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Analog watchdog interrupt enable"]
    #[inline] pub fn awdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AWDIE != 0"]
    #[inline] pub fn test_awdie(&self) -> bool {
        self.awdie() != 0
    }

    #[doc="Sets the AWDIE field."]
    #[inline] pub fn set_awdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular data overrun interrupt enable"]
    #[inline] pub fn rovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ROVRIE != 0"]
    #[inline] pub fn test_rovrie(&self) -> bool {
        self.rovrie() != 0
    }

    #[doc="Sets the ROVRIE field."]
    #[inline] pub fn set_rovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Injected data overrun interrupt enable"]
    #[inline] pub fn jovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JOVRIE != 0"]
    #[inline] pub fn test_jovrie(&self) -> bool {
        self.jovrie() != 0
    }

    #[doc="Sets the JOVRIE field."]
    #[inline] pub fn set_jovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Regular end of conversion interrupt enable"]
    #[inline] pub fn reocie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if REOCIE != 0"]
    #[inline] pub fn test_reocie(&self) -> bool {
        self.reocie() != 0
    }

    #[doc="Sets the REOCIE field."]
    #[inline] pub fn set_reocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Injected end of conversion interrupt enable"]
    #[inline] pub fn jeocie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if JEOCIE != 0"]
    #[inline] pub fn test_jeocie(&self) -> bool {
        self.jeocie() != 0
    }

    #[doc="Sets the JEOCIE field."]
    #[inline] pub fn set_jeocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Cr2(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awdch() != 0 { try!(write!(f, " awdch=0x{:x}", self.awdch()))}
        if self.exch() != 0 { try!(write!(f, " exch=0x{:x}", self.exch()))}
        if self.ckabie() != 0 { try!(write!(f, " ckabie"))}
        if self.scdie() != 0 { try!(write!(f, " scdie"))}
        if self.awdie() != 0 { try!(write!(f, " awdie"))}
        if self.rovrie() != 0 { try!(write!(f, " rovrie"))}
        if self.jovrie() != 0 { try!(write!(f, " jovrie"))}
        if self.reocie() != 0 { try!(write!(f, " reocie"))}
        if self.jeocie() != 0 { try!(write!(f, " jeocie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Isr(pub u32);
impl Dfsdm2Isr {
    #[doc="short-circuit detector flag"]
    #[inline] pub fn scdf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if SCDF != 0"]
    #[inline] pub fn test_scdf(&self) -> bool {
        self.scdf() != 0
    }

    #[doc="Sets the SCDF field."]
    #[inline] pub fn set_scdf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock absence flag"]
    #[inline] pub fn ckabf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CKABF != 0"]
    #[inline] pub fn test_ckabf(&self) -> bool {
        self.ckabf() != 0
    }

    #[doc="Sets the CKABF field."]
    #[inline] pub fn set_ckabf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Regular conversion in progress status"]
    #[inline] pub fn rcip(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RCIP != 0"]
    #[inline] pub fn test_rcip(&self) -> bool {
        self.rcip() != 0
    }

    #[doc="Sets the RCIP field."]
    #[inline] pub fn set_rcip<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Injected conversion in progress status"]
    #[inline] pub fn jcip(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if JCIP != 0"]
    #[inline] pub fn test_jcip(&self) -> bool {
        self.jcip() != 0
    }

    #[doc="Sets the JCIP field."]
    #[inline] pub fn set_jcip<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Analog watchdog"]
    #[inline] pub fn awdf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AWDF != 0"]
    #[inline] pub fn test_awdf(&self) -> bool {
        self.awdf() != 0
    }

    #[doc="Sets the AWDF field."]
    #[inline] pub fn set_awdf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular conversion overrun flag"]
    #[inline] pub fn rovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ROVRF != 0"]
    #[inline] pub fn test_rovrf(&self) -> bool {
        self.rovrf() != 0
    }

    #[doc="Sets the ROVRF field."]
    #[inline] pub fn set_rovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Injected conversion overrun flag"]
    #[inline] pub fn jovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JOVRF != 0"]
    #[inline] pub fn test_jovrf(&self) -> bool {
        self.jovrf() != 0
    }

    #[doc="Sets the JOVRF field."]
    #[inline] pub fn set_jovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End of regular conversion flag"]
    #[inline] pub fn reocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if REOCF != 0"]
    #[inline] pub fn test_reocf(&self) -> bool {
        self.reocf() != 0
    }

    #[doc="Sets the REOCF field."]
    #[inline] pub fn set_reocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End of injected conversion flag"]
    #[inline] pub fn jeocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if JEOCF != 0"]
    #[inline] pub fn test_jeocf(&self) -> bool {
        self.jeocf() != 0
    }

    #[doc="Sets the JEOCF field."]
    #[inline] pub fn set_jeocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Isr(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.scdf() != 0 { try!(write!(f, " scdf=0x{:x}", self.scdf()))}
        if self.ckabf() != 0 { try!(write!(f, " ckabf=0x{:x}", self.ckabf()))}
        if self.rcip() != 0 { try!(write!(f, " rcip"))}
        if self.jcip() != 0 { try!(write!(f, " jcip"))}
        if self.awdf() != 0 { try!(write!(f, " awdf"))}
        if self.rovrf() != 0 { try!(write!(f, " rovrf"))}
        if self.jovrf() != 0 { try!(write!(f, " jovrf"))}
        if self.reocf() != 0 { try!(write!(f, " reocf"))}
        if self.jeocf() != 0 { try!(write!(f, " jeocf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt flag clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Icr(pub u32);
impl Dfsdm2Icr {
    #[doc="Clear the short-circuit detector flag"]
    #[inline] pub fn clrscdf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if CLRSCDF != 0"]
    #[inline] pub fn test_clrscdf(&self) -> bool {
        self.clrscdf() != 0
    }

    #[doc="Sets the CLRSCDF field."]
    #[inline] pub fn set_clrscdf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clear the clock absence flag"]
    #[inline] pub fn clrckabf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CLRCKABF != 0"]
    #[inline] pub fn test_clrckabf(&self) -> bool {
        self.clrckabf() != 0
    }

    #[doc="Sets the CLRCKABF field."]
    #[inline] pub fn set_clrckabf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Clear the regular conversion overrun flag"]
    #[inline] pub fn clrrovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CLRROVRF != 0"]
    #[inline] pub fn test_clrrovrf(&self) -> bool {
        self.clrrovrf() != 0
    }

    #[doc="Sets the CLRROVRF field."]
    #[inline] pub fn set_clrrovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear the injected conversion overrun flag"]
    #[inline] pub fn clrjovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CLRJOVRF != 0"]
    #[inline] pub fn test_clrjovrf(&self) -> bool {
        self.clrjovrf() != 0
    }

    #[doc="Sets the CLRJOVRF field."]
    #[inline] pub fn set_clrjovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Dfsdm2Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Icr(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrscdf() != 0 { try!(write!(f, " clrscdf=0x{:x}", self.clrscdf()))}
        if self.clrckabf() != 0 { try!(write!(f, " clrckabf=0x{:x}", self.clrckabf()))}
        if self.clrrovrf() != 0 { try!(write!(f, " clrrovrf"))}
        if self.clrjovrf() != 0 { try!(write!(f, " clrjovrf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected channel group selection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Jchgr(pub u32);
impl Dfsdm2Jchgr {
    #[doc="Injected channel group selection"]
    #[inline] pub fn jchg(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if JCHG != 0"]
    #[inline] pub fn test_jchg(&self) -> bool {
        self.jchg() != 0
    }

    #[doc="Sets the JCHG field."]
    #[inline] pub fn set_jchg<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Jchgr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Jchgr(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Jchgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Jchgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jchg() != 0 { try!(write!(f, " jchg=0x{:x}", self.jchg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Fcr(pub u32);
impl Dfsdm2Fcr {
    #[doc="Sinc filter order"]
    #[inline] pub fn ford(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
    }

    #[doc="Returns true if FORD != 0"]
    #[inline] pub fn test_ford(&self) -> bool {
        self.ford() != 0
    }

    #[doc="Sets the FORD field."]
    #[inline] pub fn set_ford<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Sinc filter oversampling ratio (decimation rate)"]
    #[inline] pub fn fosr(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if FOSR != 0"]
    #[inline] pub fn test_fosr(&self) -> bool {
        self.fosr() != 0
    }

    #[doc="Sets the FOSR field."]
    #[inline] pub fn set_fosr<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Integrator oversampling ratio (averaging length)"]
    #[inline] pub fn iosr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IOSR != 0"]
    #[inline] pub fn test_iosr(&self) -> bool {
        self.iosr() != 0
    }

    #[doc="Sets the IOSR field."]
    #[inline] pub fn set_iosr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Fcr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Fcr(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ford() != 0 { try!(write!(f, " ford=0x{:x}", self.ford()))}
        if self.fosr() != 0 { try!(write!(f, " fosr=0x{:x}", self.fosr()))}
        if self.iosr() != 0 { try!(write!(f, " iosr=0x{:x}", self.iosr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register for injected group"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Jdatar(pub u32);
impl Dfsdm2Jdatar {
    #[doc="Injected group conversion data"]
    #[inline] pub fn jdata(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if JDATA != 0"]
    #[inline] pub fn test_jdata(&self) -> bool {
        self.jdata() != 0
    }

    #[doc="Sets the JDATA field."]
    #[inline] pub fn set_jdata<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Injected channel most recently converted"]
    #[inline] pub fn jdatach(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if JDATACH != 0"]
    #[inline] pub fn test_jdatach(&self) -> bool {
        self.jdatach() != 0
    }

    #[doc="Sets the JDATACH field."]
    #[inline] pub fn set_jdatach<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Jdatar {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Jdatar(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Jdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Jdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jdata() != 0 { try!(write!(f, " jdata=0x{:x}", self.jdata()))}
        if self.jdatach() != 0 { try!(write!(f, " jdatach=0x{:x}", self.jdatach()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register for the regular channel"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Rdatar(pub u32);
impl Dfsdm2Rdatar {
    #[doc="Regular channel conversion data"]
    #[inline] pub fn rdata(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if RDATA != 0"]
    #[inline] pub fn test_rdata(&self) -> bool {
        self.rdata() != 0
    }

    #[doc="Sets the RDATA field."]
    #[inline] pub fn set_rdata<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Regular channel pending data"]
    #[inline] pub fn rpend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RPEND != 0"]
    #[inline] pub fn test_rpend(&self) -> bool {
        self.rpend() != 0
    }

    #[doc="Sets the RPEND field."]
    #[inline] pub fn set_rpend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular channel most recently converted"]
    #[inline] pub fn rdatach(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if RDATACH != 0"]
    #[inline] pub fn test_rdatach(&self) -> bool {
        self.rdatach() != 0
    }

    #[doc="Sets the RDATACH field."]
    #[inline] pub fn set_rdatach<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Rdatar {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Rdatar(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Rdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Rdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdata() != 0 { try!(write!(f, " rdata=0x{:x}", self.rdata()))}
        if self.rpend() != 0 { try!(write!(f, " rpend"))}
        if self.rdatach() != 0 { try!(write!(f, " rdatach=0x{:x}", self.rdatach()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog high threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Awhtr(pub u32);
impl Dfsdm2Awhtr {
    #[doc="Analog watchdog high threshold"]
    #[inline] pub fn awht(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if AWHT != 0"]
    #[inline] pub fn test_awht(&self) -> bool {
        self.awht() != 0
    }

    #[doc="Sets the AWHT field."]
    #[inline] pub fn set_awht<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Break signal assignment to analog watchdog high threshold event"]
    #[inline] pub fn bkawh(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BKAWH != 0"]
    #[inline] pub fn test_bkawh(&self) -> bool {
        self.bkawh() != 0
    }

    #[doc="Sets the BKAWH field."]
    #[inline] pub fn set_bkawh<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Awhtr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Awhtr(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Awhtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Awhtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awht() != 0 { try!(write!(f, " awht=0x{:x}", self.awht()))}
        if self.bkawh() != 0 { try!(write!(f, " bkawh=0x{:x}", self.bkawh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog low threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Awltr(pub u32);
impl Dfsdm2Awltr {
    #[doc="Analog watchdog low threshold"]
    #[inline] pub fn awlt(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if AWLT != 0"]
    #[inline] pub fn test_awlt(&self) -> bool {
        self.awlt() != 0
    }

    #[doc="Sets the AWLT field."]
    #[inline] pub fn set_awlt<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Break signal assignment to analog watchdog low threshold event"]
    #[inline] pub fn bkawl(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BKAWL != 0"]
    #[inline] pub fn test_bkawl(&self) -> bool {
        self.bkawl() != 0
    }

    #[doc="Sets the BKAWL field."]
    #[inline] pub fn set_bkawl<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Awltr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Awltr(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Awltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Awltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awlt() != 0 { try!(write!(f, " awlt=0x{:x}", self.awlt()))}
        if self.bkawl() != 0 { try!(write!(f, " bkawl=0x{:x}", self.bkawl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Awsr(pub u32);
impl Dfsdm2Awsr {
    #[doc="Analog watchdog high threshold flag"]
    #[inline] pub fn awhtf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if AWHTF != 0"]
    #[inline] pub fn test_awhtf(&self) -> bool {
        self.awhtf() != 0
    }

    #[doc="Sets the AWHTF field."]
    #[inline] pub fn set_awhtf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Analog watchdog low threshold flag"]
    #[inline] pub fn awltf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if AWLTF != 0"]
    #[inline] pub fn test_awltf(&self) -> bool {
        self.awltf() != 0
    }

    #[doc="Sets the AWLTF field."]
    #[inline] pub fn set_awltf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Awsr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Awsr(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Awsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Awsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awhtf() != 0 { try!(write!(f, " awhtf=0x{:x}", self.awhtf()))}
        if self.awltf() != 0 { try!(write!(f, " awltf=0x{:x}", self.awltf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog clear flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Awcfr(pub u32);
impl Dfsdm2Awcfr {
    #[doc="Clear the analog watchdog high threshold flag"]
    #[inline] pub fn clrawhtf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if CLRAWHTF != 0"]
    #[inline] pub fn test_clrawhtf(&self) -> bool {
        self.clrawhtf() != 0
    }

    #[doc="Sets the CLRAWHTF field."]
    #[inline] pub fn set_clrawhtf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clear the analog watchdog low threshold flag"]
    #[inline] pub fn clrawltf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLRAWLTF != 0"]
    #[inline] pub fn test_clrawltf(&self) -> bool {
        self.clrawltf() != 0
    }

    #[doc="Sets the CLRAWLTF field."]
    #[inline] pub fn set_clrawltf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Awcfr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Awcfr(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Awcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Awcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrawhtf() != 0 { try!(write!(f, " clrawhtf=0x{:x}", self.clrawhtf()))}
        if self.clrawltf() != 0 { try!(write!(f, " clrawltf=0x{:x}", self.clrawltf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Extremes detector maximum register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Exmax(pub u32);
impl Dfsdm2Exmax {
    #[doc="Extremes detector maximum value"]
    #[inline] pub fn exmax(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if EXMAX != 0"]
    #[inline] pub fn test_exmax(&self) -> bool {
        self.exmax() != 0
    }

    #[doc="Sets the EXMAX field."]
    #[inline] pub fn set_exmax<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Extremes detector maximum data channel"]
    #[inline] pub fn exmaxch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXMAXCH != 0"]
    #[inline] pub fn test_exmaxch(&self) -> bool {
        self.exmaxch() != 0
    }

    #[doc="Sets the EXMAXCH field."]
    #[inline] pub fn set_exmaxch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Exmax {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Exmax(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Exmax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Exmax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exmax() != 0 { try!(write!(f, " exmax=0x{:x}", self.exmax()))}
        if self.exmaxch() != 0 { try!(write!(f, " exmaxch=0x{:x}", self.exmaxch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Extremes detector minimum register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Exmin(pub u32);
impl Dfsdm2Exmin {
    #[doc="EXMIN"]
    #[inline] pub fn exmin(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if EXMIN != 0"]
    #[inline] pub fn test_exmin(&self) -> bool {
        self.exmin() != 0
    }

    #[doc="Sets the EXMIN field."]
    #[inline] pub fn set_exmin<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Extremes detector minimum data channel"]
    #[inline] pub fn exminch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXMINCH != 0"]
    #[inline] pub fn test_exminch(&self) -> bool {
        self.exminch() != 0
    }

    #[doc="Sets the EXMINCH field."]
    #[inline] pub fn set_exminch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm2Exmin {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Exmin(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Exmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Exmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exmin() != 0 { try!(write!(f, " exmin=0x{:x}", self.exmin()))}
        if self.exminch() != 0 { try!(write!(f, " exminch=0x{:x}", self.exminch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="conversion timer register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm2Cnvtimr(pub u32);
impl Dfsdm2Cnvtimr {
    #[doc="28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN"]
    #[inline] pub fn cnvcnt(&self) -> bits::U28 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfffffff) as u32) } // [31:4]
    }

    #[doc="Returns true if CNVCNT != 0"]
    #[inline] pub fn test_cnvcnt(&self) -> bool {
        self.cnvcnt() != 0
    }

    #[doc="Sets the CNVCNT field."]
    #[inline] pub fn set_cnvcnt<V: Into<bits::U28>>(mut self, value: V) -> Self {
        let value: bits::U28 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Dfsdm2Cnvtimr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm2Cnvtimr(other)
    }
}

impl ::core::fmt::Display for Dfsdm2Cnvtimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm2Cnvtimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnvcnt() != 0 { try!(write!(f, " cnvcnt=0x{:x}", self.cnvcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Cr1(pub u32);
impl Dfsdm3Cr1 {
    #[doc="Analog watchdog fast mode select"]
    #[inline] pub fn awfsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if AWFSEL != 0"]
    #[inline] pub fn test_awfsel(&self) -> bool {
        self.awfsel() != 0
    }

    #[doc="Sets the AWFSEL field."]
    #[inline] pub fn set_awfsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Fast conversion mode selection for regular conversions"]
    #[inline] pub fn fast(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if FAST != 0"]
    #[inline] pub fn test_fast(&self) -> bool {
        self.fast() != 0
    }

    #[doc="Sets the FAST field."]
    #[inline] pub fn set_fast<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Regular channel selection"]
    #[inline] pub fn rch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if RCH != 0"]
    #[inline] pub fn test_rch(&self) -> bool {
        self.rch() != 0
    }

    #[doc="Sets the RCH field."]
    #[inline] pub fn set_rch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DMA channel enabled to read data for the regular conversion"]
    #[inline] pub fn rdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RDMAEN != 0"]
    #[inline] pub fn test_rdmaen(&self) -> bool {
        self.rdmaen() != 0
    }

    #[doc="Sets the RDMAEN field."]
    #[inline] pub fn set_rdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Launch regular conversion synchronously with DFSDM0"]
    #[inline] pub fn rsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RSYNC != 0"]
    #[inline] pub fn test_rsync(&self) -> bool {
        self.rsync() != 0
    }

    #[doc="Sets the RSYNC field."]
    #[inline] pub fn set_rsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Continuous mode selection for regular conversions"]
    #[inline] pub fn rcont(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RCONT != 0"]
    #[inline] pub fn test_rcont(&self) -> bool {
        self.rcont() != 0
    }

    #[doc="Sets the RCONT field."]
    #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Software start of a conversion on the regular channel"]
    #[inline] pub fn rswstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RSWSTART != 0"]
    #[inline] pub fn test_rswstart(&self) -> bool {
        self.rswstart() != 0
    }

    #[doc="Sets the RSWSTART field."]
    #[inline] pub fn set_rswstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Trigger enable and trigger edge selection for injected conversions"]
    #[inline] pub fn jexten(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if JEXTEN != 0"]
    #[inline] pub fn test_jexten(&self) -> bool {
        self.jexten() != 0
    }

    #[doc="Sets the JEXTEN field."]
    #[inline] pub fn set_jexten<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Trigger signal selection for launching injected conversions"]
    #[inline] pub fn jextsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if JEXTSEL != 0"]
    #[inline] pub fn test_jextsel(&self) -> bool {
        self.jextsel() != 0
    }

    #[doc="Sets the JEXTSEL field."]
    #[inline] pub fn set_jextsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA channel enabled to read data for the injected channel group"]
    #[inline] pub fn jdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if JDMAEN != 0"]
    #[inline] pub fn test_jdmaen(&self) -> bool {
        self.jdmaen() != 0
    }

    #[doc="Sets the JDMAEN field."]
    #[inline] pub fn set_jdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Scanning conversion mode for injected conversions"]
    #[inline] pub fn jscan(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if JSCAN != 0"]
    #[inline] pub fn test_jscan(&self) -> bool {
        self.jscan() != 0
    }

    #[doc="Sets the JSCAN field."]
    #[inline] pub fn set_jscan<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline] pub fn jsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if JSYNC != 0"]
    #[inline] pub fn test_jsync(&self) -> bool {
        self.jsync() != 0
    }

    #[doc="Sets the JSYNC field."]
    #[inline] pub fn set_jsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Start a conversion of the injected group of channels"]
    #[inline] pub fn jswstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if JSWSTART != 0"]
    #[inline] pub fn test_jswstart(&self) -> bool {
        self.jswstart() != 0
    }

    #[doc="Sets the JSWSTART field."]
    #[inline] pub fn set_jswstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DFSDM enable"]
    #[inline] pub fn dfen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DFEN != 0"]
    #[inline] pub fn test_dfen(&self) -> bool {
        self.dfen() != 0
    }

    #[doc="Sets the DFEN field."]
    #[inline] pub fn set_dfen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Cr1(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awfsel() != 0 { try!(write!(f, " awfsel"))}
        if self.fast() != 0 { try!(write!(f, " fast"))}
        if self.rch() != 0 { try!(write!(f, " rch=0x{:x}", self.rch()))}
        if self.rdmaen() != 0 { try!(write!(f, " rdmaen"))}
        if self.rsync() != 0 { try!(write!(f, " rsync"))}
        if self.rcont() != 0 { try!(write!(f, " rcont"))}
        if self.rswstart() != 0 { try!(write!(f, " rswstart"))}
        if self.jexten() != 0 { try!(write!(f, " jexten=0x{:x}", self.jexten()))}
        if self.jextsel() != 0 { try!(write!(f, " jextsel=0x{:x}", self.jextsel()))}
        if self.jdmaen() != 0 { try!(write!(f, " jdmaen"))}
        if self.jscan() != 0 { try!(write!(f, " jscan"))}
        if self.jsync() != 0 { try!(write!(f, " jsync"))}
        if self.jswstart() != 0 { try!(write!(f, " jswstart"))}
        if self.dfen() != 0 { try!(write!(f, " dfen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Cr2(pub u32);
impl Dfsdm3Cr2 {
    #[doc="Analog watchdog channel selection"]
    #[inline] pub fn awdch(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if AWDCH != 0"]
    #[inline] pub fn test_awdch(&self) -> bool {
        self.awdch() != 0
    }

    #[doc="Sets the AWDCH field."]
    #[inline] pub fn set_awdch<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Extremes detector channel selection"]
    #[inline] pub fn exch(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if EXCH != 0"]
    #[inline] pub fn test_exch(&self) -> bool {
        self.exch() != 0
    }

    #[doc="Sets the EXCH field."]
    #[inline] pub fn set_exch<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock absence interrupt enable"]
    #[inline] pub fn ckabie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CKABIE != 0"]
    #[inline] pub fn test_ckabie(&self) -> bool {
        self.ckabie() != 0
    }

    #[doc="Sets the CKABIE field."]
    #[inline] pub fn set_ckabie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Short-circuit detector interrupt enable"]
    #[inline] pub fn scdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCDIE != 0"]
    #[inline] pub fn test_scdie(&self) -> bool {
        self.scdie() != 0
    }

    #[doc="Sets the SCDIE field."]
    #[inline] pub fn set_scdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Analog watchdog interrupt enable"]
    #[inline] pub fn awdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AWDIE != 0"]
    #[inline] pub fn test_awdie(&self) -> bool {
        self.awdie() != 0
    }

    #[doc="Sets the AWDIE field."]
    #[inline] pub fn set_awdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular data overrun interrupt enable"]
    #[inline] pub fn rovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ROVRIE != 0"]
    #[inline] pub fn test_rovrie(&self) -> bool {
        self.rovrie() != 0
    }

    #[doc="Sets the ROVRIE field."]
    #[inline] pub fn set_rovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Injected data overrun interrupt enable"]
    #[inline] pub fn jovrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JOVRIE != 0"]
    #[inline] pub fn test_jovrie(&self) -> bool {
        self.jovrie() != 0
    }

    #[doc="Sets the JOVRIE field."]
    #[inline] pub fn set_jovrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Regular end of conversion interrupt enable"]
    #[inline] pub fn reocie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if REOCIE != 0"]
    #[inline] pub fn test_reocie(&self) -> bool {
        self.reocie() != 0
    }

    #[doc="Sets the REOCIE field."]
    #[inline] pub fn set_reocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Injected end of conversion interrupt enable"]
    #[inline] pub fn jeocie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if JEOCIE != 0"]
    #[inline] pub fn test_jeocie(&self) -> bool {
        self.jeocie() != 0
    }

    #[doc="Sets the JEOCIE field."]
    #[inline] pub fn set_jeocie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Cr2(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awdch() != 0 { try!(write!(f, " awdch=0x{:x}", self.awdch()))}
        if self.exch() != 0 { try!(write!(f, " exch=0x{:x}", self.exch()))}
        if self.ckabie() != 0 { try!(write!(f, " ckabie"))}
        if self.scdie() != 0 { try!(write!(f, " scdie"))}
        if self.awdie() != 0 { try!(write!(f, " awdie"))}
        if self.rovrie() != 0 { try!(write!(f, " rovrie"))}
        if self.jovrie() != 0 { try!(write!(f, " jovrie"))}
        if self.reocie() != 0 { try!(write!(f, " reocie"))}
        if self.jeocie() != 0 { try!(write!(f, " jeocie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Isr(pub u32);
impl Dfsdm3Isr {
    #[doc="short-circuit detector flag"]
    #[inline] pub fn scdf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if SCDF != 0"]
    #[inline] pub fn test_scdf(&self) -> bool {
        self.scdf() != 0
    }

    #[doc="Sets the SCDF field."]
    #[inline] pub fn set_scdf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock absence flag"]
    #[inline] pub fn ckabf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CKABF != 0"]
    #[inline] pub fn test_ckabf(&self) -> bool {
        self.ckabf() != 0
    }

    #[doc="Sets the CKABF field."]
    #[inline] pub fn set_ckabf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Regular conversion in progress status"]
    #[inline] pub fn rcip(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RCIP != 0"]
    #[inline] pub fn test_rcip(&self) -> bool {
        self.rcip() != 0
    }

    #[doc="Sets the RCIP field."]
    #[inline] pub fn set_rcip<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Injected conversion in progress status"]
    #[inline] pub fn jcip(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if JCIP != 0"]
    #[inline] pub fn test_jcip(&self) -> bool {
        self.jcip() != 0
    }

    #[doc="Sets the JCIP field."]
    #[inline] pub fn set_jcip<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Analog watchdog"]
    #[inline] pub fn awdf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AWDF != 0"]
    #[inline] pub fn test_awdf(&self) -> bool {
        self.awdf() != 0
    }

    #[doc="Sets the AWDF field."]
    #[inline] pub fn set_awdf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular conversion overrun flag"]
    #[inline] pub fn rovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ROVRF != 0"]
    #[inline] pub fn test_rovrf(&self) -> bool {
        self.rovrf() != 0
    }

    #[doc="Sets the ROVRF field."]
    #[inline] pub fn set_rovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Injected conversion overrun flag"]
    #[inline] pub fn jovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if JOVRF != 0"]
    #[inline] pub fn test_jovrf(&self) -> bool {
        self.jovrf() != 0
    }

    #[doc="Sets the JOVRF field."]
    #[inline] pub fn set_jovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End of regular conversion flag"]
    #[inline] pub fn reocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if REOCF != 0"]
    #[inline] pub fn test_reocf(&self) -> bool {
        self.reocf() != 0
    }

    #[doc="Sets the REOCF field."]
    #[inline] pub fn set_reocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End of injected conversion flag"]
    #[inline] pub fn jeocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if JEOCF != 0"]
    #[inline] pub fn test_jeocf(&self) -> bool {
        self.jeocf() != 0
    }

    #[doc="Sets the JEOCF field."]
    #[inline] pub fn set_jeocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Isr(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.scdf() != 0 { try!(write!(f, " scdf=0x{:x}", self.scdf()))}
        if self.ckabf() != 0 { try!(write!(f, " ckabf=0x{:x}", self.ckabf()))}
        if self.rcip() != 0 { try!(write!(f, " rcip"))}
        if self.jcip() != 0 { try!(write!(f, " jcip"))}
        if self.awdf() != 0 { try!(write!(f, " awdf"))}
        if self.rovrf() != 0 { try!(write!(f, " rovrf"))}
        if self.jovrf() != 0 { try!(write!(f, " jovrf"))}
        if self.reocf() != 0 { try!(write!(f, " reocf"))}
        if self.jeocf() != 0 { try!(write!(f, " jeocf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt flag clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Icr(pub u32);
impl Dfsdm3Icr {
    #[doc="Clear the short-circuit detector flag"]
    #[inline] pub fn clrscdf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if CLRSCDF != 0"]
    #[inline] pub fn test_clrscdf(&self) -> bool {
        self.clrscdf() != 0
    }

    #[doc="Sets the CLRSCDF field."]
    #[inline] pub fn set_clrscdf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clear the clock absence flag"]
    #[inline] pub fn clrckabf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if CLRCKABF != 0"]
    #[inline] pub fn test_clrckabf(&self) -> bool {
        self.clrckabf() != 0
    }

    #[doc="Sets the CLRCKABF field."]
    #[inline] pub fn set_clrckabf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Clear the regular conversion overrun flag"]
    #[inline] pub fn clrrovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CLRROVRF != 0"]
    #[inline] pub fn test_clrrovrf(&self) -> bool {
        self.clrrovrf() != 0
    }

    #[doc="Sets the CLRROVRF field."]
    #[inline] pub fn set_clrrovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clear the injected conversion overrun flag"]
    #[inline] pub fn clrjovrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CLRJOVRF != 0"]
    #[inline] pub fn test_clrjovrf(&self) -> bool {
        self.clrjovrf() != 0
    }

    #[doc="Sets the CLRJOVRF field."]
    #[inline] pub fn set_clrjovrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Dfsdm3Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Icr(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrscdf() != 0 { try!(write!(f, " clrscdf=0x{:x}", self.clrscdf()))}
        if self.clrckabf() != 0 { try!(write!(f, " clrckabf=0x{:x}", self.clrckabf()))}
        if self.clrrovrf() != 0 { try!(write!(f, " clrrovrf"))}
        if self.clrjovrf() != 0 { try!(write!(f, " clrjovrf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected channel group selection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Jchgr(pub u32);
impl Dfsdm3Jchgr {
    #[doc="Injected channel group selection"]
    #[inline] pub fn jchg(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if JCHG != 0"]
    #[inline] pub fn test_jchg(&self) -> bool {
        self.jchg() != 0
    }

    #[doc="Sets the JCHG field."]
    #[inline] pub fn set_jchg<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Jchgr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Jchgr(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Jchgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Jchgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jchg() != 0 { try!(write!(f, " jchg=0x{:x}", self.jchg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Fcr(pub u32);
impl Dfsdm3Fcr {
    #[doc="Sinc filter order"]
    #[inline] pub fn ford(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
    }

    #[doc="Returns true if FORD != 0"]
    #[inline] pub fn test_ford(&self) -> bool {
        self.ford() != 0
    }

    #[doc="Sets the FORD field."]
    #[inline] pub fn set_ford<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Sinc filter oversampling ratio (decimation rate)"]
    #[inline] pub fn fosr(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if FOSR != 0"]
    #[inline] pub fn test_fosr(&self) -> bool {
        self.fosr() != 0
    }

    #[doc="Sets the FOSR field."]
    #[inline] pub fn set_fosr<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Integrator oversampling ratio (averaging length)"]
    #[inline] pub fn iosr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IOSR != 0"]
    #[inline] pub fn test_iosr(&self) -> bool {
        self.iosr() != 0
    }

    #[doc="Sets the IOSR field."]
    #[inline] pub fn set_iosr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Fcr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Fcr(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Fcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ford() != 0 { try!(write!(f, " ford=0x{:x}", self.ford()))}
        if self.fosr() != 0 { try!(write!(f, " fosr=0x{:x}", self.fosr()))}
        if self.iosr() != 0 { try!(write!(f, " iosr=0x{:x}", self.iosr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register for injected group"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Jdatar(pub u32);
impl Dfsdm3Jdatar {
    #[doc="Injected group conversion data"]
    #[inline] pub fn jdata(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if JDATA != 0"]
    #[inline] pub fn test_jdata(&self) -> bool {
        self.jdata() != 0
    }

    #[doc="Sets the JDATA field."]
    #[inline] pub fn set_jdata<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Injected channel most recently converted"]
    #[inline] pub fn jdatach(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if JDATACH != 0"]
    #[inline] pub fn test_jdatach(&self) -> bool {
        self.jdatach() != 0
    }

    #[doc="Sets the JDATACH field."]
    #[inline] pub fn set_jdatach<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Jdatar {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Jdatar(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Jdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Jdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jdata() != 0 { try!(write!(f, " jdata=0x{:x}", self.jdata()))}
        if self.jdatach() != 0 { try!(write!(f, " jdatach=0x{:x}", self.jdatach()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register for the regular channel"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Rdatar(pub u32);
impl Dfsdm3Rdatar {
    #[doc="Regular channel conversion data"]
    #[inline] pub fn rdata(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if RDATA != 0"]
    #[inline] pub fn test_rdata(&self) -> bool {
        self.rdata() != 0
    }

    #[doc="Sets the RDATA field."]
    #[inline] pub fn set_rdata<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Regular channel pending data"]
    #[inline] pub fn rpend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RPEND != 0"]
    #[inline] pub fn test_rpend(&self) -> bool {
        self.rpend() != 0
    }

    #[doc="Sets the RPEND field."]
    #[inline] pub fn set_rpend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Regular channel most recently converted"]
    #[inline] pub fn rdatach(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if RDATACH != 0"]
    #[inline] pub fn test_rdatach(&self) -> bool {
        self.rdatach() != 0
    }

    #[doc="Sets the RDATACH field."]
    #[inline] pub fn set_rdatach<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Rdatar {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Rdatar(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Rdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Rdatar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdata() != 0 { try!(write!(f, " rdata=0x{:x}", self.rdata()))}
        if self.rpend() != 0 { try!(write!(f, " rpend"))}
        if self.rdatach() != 0 { try!(write!(f, " rdatach=0x{:x}", self.rdatach()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog high threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Awhtr(pub u32);
impl Dfsdm3Awhtr {
    #[doc="Analog watchdog high threshold"]
    #[inline] pub fn awht(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if AWHT != 0"]
    #[inline] pub fn test_awht(&self) -> bool {
        self.awht() != 0
    }

    #[doc="Sets the AWHT field."]
    #[inline] pub fn set_awht<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Break signal assignment to analog watchdog high threshold event"]
    #[inline] pub fn bkawh(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BKAWH != 0"]
    #[inline] pub fn test_bkawh(&self) -> bool {
        self.bkawh() != 0
    }

    #[doc="Sets the BKAWH field."]
    #[inline] pub fn set_bkawh<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Awhtr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Awhtr(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Awhtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Awhtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awht() != 0 { try!(write!(f, " awht=0x{:x}", self.awht()))}
        if self.bkawh() != 0 { try!(write!(f, " bkawh=0x{:x}", self.bkawh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog low threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Awltr(pub u32);
impl Dfsdm3Awltr {
    #[doc="Analog watchdog low threshold"]
    #[inline] pub fn awlt(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if AWLT != 0"]
    #[inline] pub fn test_awlt(&self) -> bool {
        self.awlt() != 0
    }

    #[doc="Sets the AWLT field."]
    #[inline] pub fn set_awlt<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Break signal assignment to analog watchdog low threshold event"]
    #[inline] pub fn bkawl(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BKAWL != 0"]
    #[inline] pub fn test_bkawl(&self) -> bool {
        self.bkawl() != 0
    }

    #[doc="Sets the BKAWL field."]
    #[inline] pub fn set_bkawl<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Awltr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Awltr(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Awltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Awltr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awlt() != 0 { try!(write!(f, " awlt=0x{:x}", self.awlt()))}
        if self.bkawl() != 0 { try!(write!(f, " bkawl=0x{:x}", self.bkawl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Awsr(pub u32);
impl Dfsdm3Awsr {
    #[doc="Analog watchdog high threshold flag"]
    #[inline] pub fn awhtf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if AWHTF != 0"]
    #[inline] pub fn test_awhtf(&self) -> bool {
        self.awhtf() != 0
    }

    #[doc="Sets the AWHTF field."]
    #[inline] pub fn set_awhtf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Analog watchdog low threshold flag"]
    #[inline] pub fn awltf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if AWLTF != 0"]
    #[inline] pub fn test_awltf(&self) -> bool {
        self.awltf() != 0
    }

    #[doc="Sets the AWLTF field."]
    #[inline] pub fn set_awltf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Awsr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Awsr(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Awsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Awsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awhtf() != 0 { try!(write!(f, " awhtf=0x{:x}", self.awhtf()))}
        if self.awltf() != 0 { try!(write!(f, " awltf=0x{:x}", self.awltf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="analog watchdog clear flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Awcfr(pub u32);
impl Dfsdm3Awcfr {
    #[doc="Clear the analog watchdog high threshold flag"]
    #[inline] pub fn clrawhtf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if CLRAWHTF != 0"]
    #[inline] pub fn test_clrawhtf(&self) -> bool {
        self.clrawhtf() != 0
    }

    #[doc="Sets the CLRAWHTF field."]
    #[inline] pub fn set_clrawhtf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clear the analog watchdog low threshold flag"]
    #[inline] pub fn clrawltf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLRAWLTF != 0"]
    #[inline] pub fn test_clrawltf(&self) -> bool {
        self.clrawltf() != 0
    }

    #[doc="Sets the CLRAWLTF field."]
    #[inline] pub fn set_clrawltf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Awcfr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Awcfr(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Awcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Awcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clrawhtf() != 0 { try!(write!(f, " clrawhtf=0x{:x}", self.clrawhtf()))}
        if self.clrawltf() != 0 { try!(write!(f, " clrawltf=0x{:x}", self.clrawltf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Extremes detector maximum register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Exmax(pub u32);
impl Dfsdm3Exmax {
    #[doc="Extremes detector maximum value"]
    #[inline] pub fn exmax(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if EXMAX != 0"]
    #[inline] pub fn test_exmax(&self) -> bool {
        self.exmax() != 0
    }

    #[doc="Sets the EXMAX field."]
    #[inline] pub fn set_exmax<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Extremes detector maximum data channel"]
    #[inline] pub fn exmaxch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXMAXCH != 0"]
    #[inline] pub fn test_exmaxch(&self) -> bool {
        self.exmaxch() != 0
    }

    #[doc="Sets the EXMAXCH field."]
    #[inline] pub fn set_exmaxch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Exmax {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Exmax(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Exmax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Exmax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exmax() != 0 { try!(write!(f, " exmax=0x{:x}", self.exmax()))}
        if self.exmaxch() != 0 { try!(write!(f, " exmaxch=0x{:x}", self.exmaxch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Extremes detector minimum register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Exmin(pub u32);
impl Dfsdm3Exmin {
    #[doc="EXMIN"]
    #[inline] pub fn exmin(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffffff) as u32) } // [31:8]
    }

    #[doc="Returns true if EXMIN != 0"]
    #[inline] pub fn test_exmin(&self) -> bool {
        self.exmin() != 0
    }

    #[doc="Sets the EXMIN field."]
    #[inline] pub fn set_exmin<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Extremes detector minimum data channel"]
    #[inline] pub fn exminch(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXMINCH != 0"]
    #[inline] pub fn test_exminch(&self) -> bool {
        self.exminch() != 0
    }

    #[doc="Sets the EXMINCH field."]
    #[inline] pub fn set_exminch<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dfsdm3Exmin {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Exmin(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Exmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Exmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exmin() != 0 { try!(write!(f, " exmin=0x{:x}", self.exmin()))}
        if self.exminch() != 0 { try!(write!(f, " exminch=0x{:x}", self.exminch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="conversion timer register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfsdm3Cnvtimr(pub u32);
impl Dfsdm3Cnvtimr {
    #[doc="28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN"]
    #[inline] pub fn cnvcnt(&self) -> bits::U28 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfffffff) as u32) } // [31:4]
    }

    #[doc="Returns true if CNVCNT != 0"]
    #[inline] pub fn test_cnvcnt(&self) -> bool {
        self.cnvcnt() != 0
    }

    #[doc="Sets the CNVCNT field."]
    #[inline] pub fn set_cnvcnt<V: Into<bits::U28>>(mut self, value: V) -> Self {
        let value: bits::U28 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Dfsdm3Cnvtimr {
    #[inline]
    fn from(other: u32) -> Self {
         Dfsdm3Cnvtimr(other)
    }
}

impl ::core::fmt::Display for Dfsdm3Cnvtimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfsdm3Cnvtimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnvcnt() != 0 { try!(write!(f, " cnvcnt=0x{:x}", self.cnvcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


