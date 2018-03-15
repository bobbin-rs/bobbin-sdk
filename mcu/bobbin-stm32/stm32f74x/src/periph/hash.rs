//! Hash processor

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Hash processor"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HashPeriph(pub usize);
impl HashPeriph {
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

    #[doc="Get the *mut pointer for the DIN register."]
    #[inline] pub fn din_mut(&self) -> *mut Din { 
        (self.0 + 0x4) as *mut Din
    }

    #[doc="Get the *const pointer for the DIN register."]
    #[inline] pub fn din_ptr(&self) -> *const Din { 
           self.din_mut()
    }

    #[doc="Read the DIN register."]
    #[inline] pub fn din(&self) -> Din { 
        unsafe {
            read_volatile(self.din_ptr())
        }
    }

    #[doc="Write the DIN register."]
    #[inline] pub fn set_din<F: FnOnce(Din) -> Din>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.din_mut(), f(Din(0)));
        }
        self
    }

    #[doc="Modify the DIN register."]
    #[inline] pub fn with_din<F: FnOnce(Din) -> Din>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.din_mut(), f(self.din()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STR register."]
    #[inline] pub fn str_mut(&self) -> *mut Str { 
        (self.0 + 0x8) as *mut Str
    }

    #[doc="Get the *const pointer for the STR register."]
    #[inline] pub fn str_ptr(&self) -> *const Str { 
           self.str_mut()
    }

    #[doc="Read the STR register."]
    #[inline] pub fn str(&self) -> Str { 
        unsafe {
            read_volatile(self.str_ptr())
        }
    }

    #[doc="Write the STR register."]
    #[inline] pub fn set_str<F: FnOnce(Str) -> Str>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.str_mut(), f(Str(0)));
        }
        self
    }

    #[doc="Modify the STR register."]
    #[inline] pub fn with_str<F: FnOnce(Str) -> Str>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.str_mut(), f(self.str()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HR0 register."]
    #[inline] pub fn hr0_mut(&self) -> *mut Hr0 { 
        (self.0 + 0xc) as *mut Hr0
    }

    #[doc="Get the *const pointer for the HR0 register."]
    #[inline] pub fn hr0_ptr(&self) -> *const Hr0 { 
           self.hr0_mut()
    }

    #[doc="Read the HR0 register."]
    #[inline] pub fn hr0(&self) -> Hr0 { 
        unsafe {
            read_volatile(self.hr0_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HR1 register."]
    #[inline] pub fn hr1_mut(&self) -> *mut Hr1 { 
        (self.0 + 0x10) as *mut Hr1
    }

    #[doc="Get the *const pointer for the HR1 register."]
    #[inline] pub fn hr1_ptr(&self) -> *const Hr1 { 
           self.hr1_mut()
    }

    #[doc="Read the HR1 register."]
    #[inline] pub fn hr1(&self) -> Hr1 { 
        unsafe {
            read_volatile(self.hr1_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HR2 register."]
    #[inline] pub fn hr2_mut(&self) -> *mut Hr2 { 
        (self.0 + 0x14) as *mut Hr2
    }

    #[doc="Get the *const pointer for the HR2 register."]
    #[inline] pub fn hr2_ptr(&self) -> *const Hr2 { 
           self.hr2_mut()
    }

    #[doc="Read the HR2 register."]
    #[inline] pub fn hr2(&self) -> Hr2 { 
        unsafe {
            read_volatile(self.hr2_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HR3 register."]
    #[inline] pub fn hr3_mut(&self) -> *mut Hr3 { 
        (self.0 + 0x18) as *mut Hr3
    }

    #[doc="Get the *const pointer for the HR3 register."]
    #[inline] pub fn hr3_ptr(&self) -> *const Hr3 { 
           self.hr3_mut()
    }

    #[doc="Read the HR3 register."]
    #[inline] pub fn hr3(&self) -> Hr3 { 
        unsafe {
            read_volatile(self.hr3_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HR4 register."]
    #[inline] pub fn hr4_mut(&self) -> *mut Hr4 { 
        (self.0 + 0x1c) as *mut Hr4
    }

    #[doc="Get the *const pointer for the HR4 register."]
    #[inline] pub fn hr4_ptr(&self) -> *const Hr4 { 
           self.hr4_mut()
    }

    #[doc="Read the HR4 register."]
    #[inline] pub fn hr4(&self) -> Hr4 { 
        unsafe {
            read_volatile(self.hr4_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IMR register."]
    #[inline] pub fn imr_mut(&self) -> *mut Imr { 
        (self.0 + 0x20) as *mut Imr
    }

    #[doc="Get the *const pointer for the IMR register."]
    #[inline] pub fn imr_ptr(&self) -> *const Imr { 
           self.imr_mut()
    }

    #[doc="Read the IMR register."]
    #[inline] pub fn imr(&self) -> Imr { 
        unsafe {
            read_volatile(self.imr_ptr())
        }
    }

    #[doc="Write the IMR register."]
    #[inline] pub fn set_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imr_mut(), f(Imr(0)));
        }
        self
    }

    #[doc="Modify the IMR register."]
    #[inline] pub fn with_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imr_mut(), f(self.imr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        (self.0 + 0x24) as *mut Sr
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

    #[doc="Write the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr_mut(), f(Sr(0)));
        }
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr_mut(), f(self.sr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR0 register."]
    #[inline] pub fn csr0_mut(&self) -> *mut Csr0 { 
        (self.0 + 0xf8) as *mut Csr0
    }

    #[doc="Get the *const pointer for the CSR0 register."]
    #[inline] pub fn csr0_ptr(&self) -> *const Csr0 { 
           self.csr0_mut()
    }

    #[doc="Read the CSR0 register."]
    #[inline] pub fn csr0(&self) -> Csr0 { 
        unsafe {
            read_volatile(self.csr0_ptr())
        }
    }

    #[doc="Write the CSR0 register."]
    #[inline] pub fn set_csr0<F: FnOnce(Csr0) -> Csr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr0_mut(), f(Csr0(0)));
        }
        self
    }

    #[doc="Modify the CSR0 register."]
    #[inline] pub fn with_csr0<F: FnOnce(Csr0) -> Csr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr0_mut(), f(self.csr0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR1 register."]
    #[inline] pub fn csr1_mut(&self) -> *mut Csr1 { 
        (self.0 + 0xfc) as *mut Csr1
    }

    #[doc="Get the *const pointer for the CSR1 register."]
    #[inline] pub fn csr1_ptr(&self) -> *const Csr1 { 
           self.csr1_mut()
    }

    #[doc="Read the CSR1 register."]
    #[inline] pub fn csr1(&self) -> Csr1 { 
        unsafe {
            read_volatile(self.csr1_ptr())
        }
    }

    #[doc="Write the CSR1 register."]
    #[inline] pub fn set_csr1<F: FnOnce(Csr1) -> Csr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr1_mut(), f(Csr1(0)));
        }
        self
    }

    #[doc="Modify the CSR1 register."]
    #[inline] pub fn with_csr1<F: FnOnce(Csr1) -> Csr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr1_mut(), f(self.csr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR2 register."]
    #[inline] pub fn csr2_mut(&self) -> *mut Csr2 { 
        (self.0 + 0x100) as *mut Csr2
    }

    #[doc="Get the *const pointer for the CSR2 register."]
    #[inline] pub fn csr2_ptr(&self) -> *const Csr2 { 
           self.csr2_mut()
    }

    #[doc="Read the CSR2 register."]
    #[inline] pub fn csr2(&self) -> Csr2 { 
        unsafe {
            read_volatile(self.csr2_ptr())
        }
    }

    #[doc="Write the CSR2 register."]
    #[inline] pub fn set_csr2<F: FnOnce(Csr2) -> Csr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr2_mut(), f(Csr2(0)));
        }
        self
    }

    #[doc="Modify the CSR2 register."]
    #[inline] pub fn with_csr2<F: FnOnce(Csr2) -> Csr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr2_mut(), f(self.csr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR3 register."]
    #[inline] pub fn csr3_mut(&self) -> *mut Csr3 { 
        (self.0 + 0x104) as *mut Csr3
    }

    #[doc="Get the *const pointer for the CSR3 register."]
    #[inline] pub fn csr3_ptr(&self) -> *const Csr3 { 
           self.csr3_mut()
    }

    #[doc="Read the CSR3 register."]
    #[inline] pub fn csr3(&self) -> Csr3 { 
        unsafe {
            read_volatile(self.csr3_ptr())
        }
    }

    #[doc="Write the CSR3 register."]
    #[inline] pub fn set_csr3<F: FnOnce(Csr3) -> Csr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr3_mut(), f(Csr3(0)));
        }
        self
    }

    #[doc="Modify the CSR3 register."]
    #[inline] pub fn with_csr3<F: FnOnce(Csr3) -> Csr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr3_mut(), f(self.csr3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR4 register."]
    #[inline] pub fn csr4_mut(&self) -> *mut Csr4 { 
        (self.0 + 0x108) as *mut Csr4
    }

    #[doc="Get the *const pointer for the CSR4 register."]
    #[inline] pub fn csr4_ptr(&self) -> *const Csr4 { 
           self.csr4_mut()
    }

    #[doc="Read the CSR4 register."]
    #[inline] pub fn csr4(&self) -> Csr4 { 
        unsafe {
            read_volatile(self.csr4_ptr())
        }
    }

    #[doc="Write the CSR4 register."]
    #[inline] pub fn set_csr4<F: FnOnce(Csr4) -> Csr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr4_mut(), f(Csr4(0)));
        }
        self
    }

    #[doc="Modify the CSR4 register."]
    #[inline] pub fn with_csr4<F: FnOnce(Csr4) -> Csr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr4_mut(), f(self.csr4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR5 register."]
    #[inline] pub fn csr5_mut(&self) -> *mut Csr5 { 
        (self.0 + 0x10c) as *mut Csr5
    }

    #[doc="Get the *const pointer for the CSR5 register."]
    #[inline] pub fn csr5_ptr(&self) -> *const Csr5 { 
           self.csr5_mut()
    }

    #[doc="Read the CSR5 register."]
    #[inline] pub fn csr5(&self) -> Csr5 { 
        unsafe {
            read_volatile(self.csr5_ptr())
        }
    }

    #[doc="Write the CSR5 register."]
    #[inline] pub fn set_csr5<F: FnOnce(Csr5) -> Csr5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr5_mut(), f(Csr5(0)));
        }
        self
    }

    #[doc="Modify the CSR5 register."]
    #[inline] pub fn with_csr5<F: FnOnce(Csr5) -> Csr5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr5_mut(), f(self.csr5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR6 register."]
    #[inline] pub fn csr6_mut(&self) -> *mut Csr6 { 
        (self.0 + 0x110) as *mut Csr6
    }

    #[doc="Get the *const pointer for the CSR6 register."]
    #[inline] pub fn csr6_ptr(&self) -> *const Csr6 { 
           self.csr6_mut()
    }

    #[doc="Read the CSR6 register."]
    #[inline] pub fn csr6(&self) -> Csr6 { 
        unsafe {
            read_volatile(self.csr6_ptr())
        }
    }

    #[doc="Write the CSR6 register."]
    #[inline] pub fn set_csr6<F: FnOnce(Csr6) -> Csr6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr6_mut(), f(Csr6(0)));
        }
        self
    }

    #[doc="Modify the CSR6 register."]
    #[inline] pub fn with_csr6<F: FnOnce(Csr6) -> Csr6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr6_mut(), f(self.csr6()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR7 register."]
    #[inline] pub fn csr7_mut(&self) -> *mut Csr7 { 
        (self.0 + 0x114) as *mut Csr7
    }

    #[doc="Get the *const pointer for the CSR7 register."]
    #[inline] pub fn csr7_ptr(&self) -> *const Csr7 { 
           self.csr7_mut()
    }

    #[doc="Read the CSR7 register."]
    #[inline] pub fn csr7(&self) -> Csr7 { 
        unsafe {
            read_volatile(self.csr7_ptr())
        }
    }

    #[doc="Write the CSR7 register."]
    #[inline] pub fn set_csr7<F: FnOnce(Csr7) -> Csr7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr7_mut(), f(Csr7(0)));
        }
        self
    }

    #[doc="Modify the CSR7 register."]
    #[inline] pub fn with_csr7<F: FnOnce(Csr7) -> Csr7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr7_mut(), f(self.csr7()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR8 register."]
    #[inline] pub fn csr8_mut(&self) -> *mut Csr8 { 
        (self.0 + 0x118) as *mut Csr8
    }

    #[doc="Get the *const pointer for the CSR8 register."]
    #[inline] pub fn csr8_ptr(&self) -> *const Csr8 { 
           self.csr8_mut()
    }

    #[doc="Read the CSR8 register."]
    #[inline] pub fn csr8(&self) -> Csr8 { 
        unsafe {
            read_volatile(self.csr8_ptr())
        }
    }

    #[doc="Write the CSR8 register."]
    #[inline] pub fn set_csr8<F: FnOnce(Csr8) -> Csr8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr8_mut(), f(Csr8(0)));
        }
        self
    }

    #[doc="Modify the CSR8 register."]
    #[inline] pub fn with_csr8<F: FnOnce(Csr8) -> Csr8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr8_mut(), f(self.csr8()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR9 register."]
    #[inline] pub fn csr9_mut(&self) -> *mut Csr9 { 
        (self.0 + 0x11c) as *mut Csr9
    }

    #[doc="Get the *const pointer for the CSR9 register."]
    #[inline] pub fn csr9_ptr(&self) -> *const Csr9 { 
           self.csr9_mut()
    }

    #[doc="Read the CSR9 register."]
    #[inline] pub fn csr9(&self) -> Csr9 { 
        unsafe {
            read_volatile(self.csr9_ptr())
        }
    }

    #[doc="Write the CSR9 register."]
    #[inline] pub fn set_csr9<F: FnOnce(Csr9) -> Csr9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr9_mut(), f(Csr9(0)));
        }
        self
    }

    #[doc="Modify the CSR9 register."]
    #[inline] pub fn with_csr9<F: FnOnce(Csr9) -> Csr9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr9_mut(), f(self.csr9()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR10 register."]
    #[inline] pub fn csr10_mut(&self) -> *mut Csr10 { 
        (self.0 + 0x120) as *mut Csr10
    }

    #[doc="Get the *const pointer for the CSR10 register."]
    #[inline] pub fn csr10_ptr(&self) -> *const Csr10 { 
           self.csr10_mut()
    }

    #[doc="Read the CSR10 register."]
    #[inline] pub fn csr10(&self) -> Csr10 { 
        unsafe {
            read_volatile(self.csr10_ptr())
        }
    }

    #[doc="Write the CSR10 register."]
    #[inline] pub fn set_csr10<F: FnOnce(Csr10) -> Csr10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr10_mut(), f(Csr10(0)));
        }
        self
    }

    #[doc="Modify the CSR10 register."]
    #[inline] pub fn with_csr10<F: FnOnce(Csr10) -> Csr10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr10_mut(), f(self.csr10()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR11 register."]
    #[inline] pub fn csr11_mut(&self) -> *mut Csr11 { 
        (self.0 + 0x124) as *mut Csr11
    }

    #[doc="Get the *const pointer for the CSR11 register."]
    #[inline] pub fn csr11_ptr(&self) -> *const Csr11 { 
           self.csr11_mut()
    }

    #[doc="Read the CSR11 register."]
    #[inline] pub fn csr11(&self) -> Csr11 { 
        unsafe {
            read_volatile(self.csr11_ptr())
        }
    }

    #[doc="Write the CSR11 register."]
    #[inline] pub fn set_csr11<F: FnOnce(Csr11) -> Csr11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr11_mut(), f(Csr11(0)));
        }
        self
    }

    #[doc="Modify the CSR11 register."]
    #[inline] pub fn with_csr11<F: FnOnce(Csr11) -> Csr11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr11_mut(), f(self.csr11()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR12 register."]
    #[inline] pub fn csr12_mut(&self) -> *mut Csr12 { 
        (self.0 + 0x128) as *mut Csr12
    }

    #[doc="Get the *const pointer for the CSR12 register."]
    #[inline] pub fn csr12_ptr(&self) -> *const Csr12 { 
           self.csr12_mut()
    }

    #[doc="Read the CSR12 register."]
    #[inline] pub fn csr12(&self) -> Csr12 { 
        unsafe {
            read_volatile(self.csr12_ptr())
        }
    }

    #[doc="Write the CSR12 register."]
    #[inline] pub fn set_csr12<F: FnOnce(Csr12) -> Csr12>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr12_mut(), f(Csr12(0)));
        }
        self
    }

    #[doc="Modify the CSR12 register."]
    #[inline] pub fn with_csr12<F: FnOnce(Csr12) -> Csr12>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr12_mut(), f(self.csr12()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR13 register."]
    #[inline] pub fn csr13_mut(&self) -> *mut Csr13 { 
        (self.0 + 0x12c) as *mut Csr13
    }

    #[doc="Get the *const pointer for the CSR13 register."]
    #[inline] pub fn csr13_ptr(&self) -> *const Csr13 { 
           self.csr13_mut()
    }

    #[doc="Read the CSR13 register."]
    #[inline] pub fn csr13(&self) -> Csr13 { 
        unsafe {
            read_volatile(self.csr13_ptr())
        }
    }

    #[doc="Write the CSR13 register."]
    #[inline] pub fn set_csr13<F: FnOnce(Csr13) -> Csr13>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr13_mut(), f(Csr13(0)));
        }
        self
    }

    #[doc="Modify the CSR13 register."]
    #[inline] pub fn with_csr13<F: FnOnce(Csr13) -> Csr13>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr13_mut(), f(self.csr13()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR14 register."]
    #[inline] pub fn csr14_mut(&self) -> *mut Csr14 { 
        (self.0 + 0x130) as *mut Csr14
    }

    #[doc="Get the *const pointer for the CSR14 register."]
    #[inline] pub fn csr14_ptr(&self) -> *const Csr14 { 
           self.csr14_mut()
    }

    #[doc="Read the CSR14 register."]
    #[inline] pub fn csr14(&self) -> Csr14 { 
        unsafe {
            read_volatile(self.csr14_ptr())
        }
    }

    #[doc="Write the CSR14 register."]
    #[inline] pub fn set_csr14<F: FnOnce(Csr14) -> Csr14>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr14_mut(), f(Csr14(0)));
        }
        self
    }

    #[doc="Modify the CSR14 register."]
    #[inline] pub fn with_csr14<F: FnOnce(Csr14) -> Csr14>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr14_mut(), f(self.csr14()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR15 register."]
    #[inline] pub fn csr15_mut(&self) -> *mut Csr15 { 
        (self.0 + 0x134) as *mut Csr15
    }

    #[doc="Get the *const pointer for the CSR15 register."]
    #[inline] pub fn csr15_ptr(&self) -> *const Csr15 { 
           self.csr15_mut()
    }

    #[doc="Read the CSR15 register."]
    #[inline] pub fn csr15(&self) -> Csr15 { 
        unsafe {
            read_volatile(self.csr15_ptr())
        }
    }

    #[doc="Write the CSR15 register."]
    #[inline] pub fn set_csr15<F: FnOnce(Csr15) -> Csr15>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr15_mut(), f(Csr15(0)));
        }
        self
    }

    #[doc="Modify the CSR15 register."]
    #[inline] pub fn with_csr15<F: FnOnce(Csr15) -> Csr15>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr15_mut(), f(self.csr15()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR16 register."]
    #[inline] pub fn csr16_mut(&self) -> *mut Csr16 { 
        (self.0 + 0x138) as *mut Csr16
    }

    #[doc="Get the *const pointer for the CSR16 register."]
    #[inline] pub fn csr16_ptr(&self) -> *const Csr16 { 
           self.csr16_mut()
    }

    #[doc="Read the CSR16 register."]
    #[inline] pub fn csr16(&self) -> Csr16 { 
        unsafe {
            read_volatile(self.csr16_ptr())
        }
    }

    #[doc="Write the CSR16 register."]
    #[inline] pub fn set_csr16<F: FnOnce(Csr16) -> Csr16>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr16_mut(), f(Csr16(0)));
        }
        self
    }

    #[doc="Modify the CSR16 register."]
    #[inline] pub fn with_csr16<F: FnOnce(Csr16) -> Csr16>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr16_mut(), f(self.csr16()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR17 register."]
    #[inline] pub fn csr17_mut(&self) -> *mut Csr17 { 
        (self.0 + 0x13c) as *mut Csr17
    }

    #[doc="Get the *const pointer for the CSR17 register."]
    #[inline] pub fn csr17_ptr(&self) -> *const Csr17 { 
           self.csr17_mut()
    }

    #[doc="Read the CSR17 register."]
    #[inline] pub fn csr17(&self) -> Csr17 { 
        unsafe {
            read_volatile(self.csr17_ptr())
        }
    }

    #[doc="Write the CSR17 register."]
    #[inline] pub fn set_csr17<F: FnOnce(Csr17) -> Csr17>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr17_mut(), f(Csr17(0)));
        }
        self
    }

    #[doc="Modify the CSR17 register."]
    #[inline] pub fn with_csr17<F: FnOnce(Csr17) -> Csr17>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr17_mut(), f(self.csr17()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR18 register."]
    #[inline] pub fn csr18_mut(&self) -> *mut Csr18 { 
        (self.0 + 0x140) as *mut Csr18
    }

    #[doc="Get the *const pointer for the CSR18 register."]
    #[inline] pub fn csr18_ptr(&self) -> *const Csr18 { 
           self.csr18_mut()
    }

    #[doc="Read the CSR18 register."]
    #[inline] pub fn csr18(&self) -> Csr18 { 
        unsafe {
            read_volatile(self.csr18_ptr())
        }
    }

    #[doc="Write the CSR18 register."]
    #[inline] pub fn set_csr18<F: FnOnce(Csr18) -> Csr18>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr18_mut(), f(Csr18(0)));
        }
        self
    }

    #[doc="Modify the CSR18 register."]
    #[inline] pub fn with_csr18<F: FnOnce(Csr18) -> Csr18>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr18_mut(), f(self.csr18()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR19 register."]
    #[inline] pub fn csr19_mut(&self) -> *mut Csr19 { 
        (self.0 + 0x144) as *mut Csr19
    }

    #[doc="Get the *const pointer for the CSR19 register."]
    #[inline] pub fn csr19_ptr(&self) -> *const Csr19 { 
           self.csr19_mut()
    }

    #[doc="Read the CSR19 register."]
    #[inline] pub fn csr19(&self) -> Csr19 { 
        unsafe {
            read_volatile(self.csr19_ptr())
        }
    }

    #[doc="Write the CSR19 register."]
    #[inline] pub fn set_csr19<F: FnOnce(Csr19) -> Csr19>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr19_mut(), f(Csr19(0)));
        }
        self
    }

    #[doc="Modify the CSR19 register."]
    #[inline] pub fn with_csr19<F: FnOnce(Csr19) -> Csr19>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr19_mut(), f(self.csr19()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR20 register."]
    #[inline] pub fn csr20_mut(&self) -> *mut Csr20 { 
        (self.0 + 0x148) as *mut Csr20
    }

    #[doc="Get the *const pointer for the CSR20 register."]
    #[inline] pub fn csr20_ptr(&self) -> *const Csr20 { 
           self.csr20_mut()
    }

    #[doc="Read the CSR20 register."]
    #[inline] pub fn csr20(&self) -> Csr20 { 
        unsafe {
            read_volatile(self.csr20_ptr())
        }
    }

    #[doc="Write the CSR20 register."]
    #[inline] pub fn set_csr20<F: FnOnce(Csr20) -> Csr20>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr20_mut(), f(Csr20(0)));
        }
        self
    }

    #[doc="Modify the CSR20 register."]
    #[inline] pub fn with_csr20<F: FnOnce(Csr20) -> Csr20>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr20_mut(), f(self.csr20()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR21 register."]
    #[inline] pub fn csr21_mut(&self) -> *mut Csr21 { 
        (self.0 + 0x14c) as *mut Csr21
    }

    #[doc="Get the *const pointer for the CSR21 register."]
    #[inline] pub fn csr21_ptr(&self) -> *const Csr21 { 
           self.csr21_mut()
    }

    #[doc="Read the CSR21 register."]
    #[inline] pub fn csr21(&self) -> Csr21 { 
        unsafe {
            read_volatile(self.csr21_ptr())
        }
    }

    #[doc="Write the CSR21 register."]
    #[inline] pub fn set_csr21<F: FnOnce(Csr21) -> Csr21>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr21_mut(), f(Csr21(0)));
        }
        self
    }

    #[doc="Modify the CSR21 register."]
    #[inline] pub fn with_csr21<F: FnOnce(Csr21) -> Csr21>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr21_mut(), f(self.csr21()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR22 register."]
    #[inline] pub fn csr22_mut(&self) -> *mut Csr22 { 
        (self.0 + 0x150) as *mut Csr22
    }

    #[doc="Get the *const pointer for the CSR22 register."]
    #[inline] pub fn csr22_ptr(&self) -> *const Csr22 { 
           self.csr22_mut()
    }

    #[doc="Read the CSR22 register."]
    #[inline] pub fn csr22(&self) -> Csr22 { 
        unsafe {
            read_volatile(self.csr22_ptr())
        }
    }

    #[doc="Write the CSR22 register."]
    #[inline] pub fn set_csr22<F: FnOnce(Csr22) -> Csr22>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr22_mut(), f(Csr22(0)));
        }
        self
    }

    #[doc="Modify the CSR22 register."]
    #[inline] pub fn with_csr22<F: FnOnce(Csr22) -> Csr22>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr22_mut(), f(self.csr22()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR23 register."]
    #[inline] pub fn csr23_mut(&self) -> *mut Csr23 { 
        (self.0 + 0x154) as *mut Csr23
    }

    #[doc="Get the *const pointer for the CSR23 register."]
    #[inline] pub fn csr23_ptr(&self) -> *const Csr23 { 
           self.csr23_mut()
    }

    #[doc="Read the CSR23 register."]
    #[inline] pub fn csr23(&self) -> Csr23 { 
        unsafe {
            read_volatile(self.csr23_ptr())
        }
    }

    #[doc="Write the CSR23 register."]
    #[inline] pub fn set_csr23<F: FnOnce(Csr23) -> Csr23>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr23_mut(), f(Csr23(0)));
        }
        self
    }

    #[doc="Modify the CSR23 register."]
    #[inline] pub fn with_csr23<F: FnOnce(Csr23) -> Csr23>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr23_mut(), f(self.csr23()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR24 register."]
    #[inline] pub fn csr24_mut(&self) -> *mut Csr24 { 
        (self.0 + 0x158) as *mut Csr24
    }

    #[doc="Get the *const pointer for the CSR24 register."]
    #[inline] pub fn csr24_ptr(&self) -> *const Csr24 { 
           self.csr24_mut()
    }

    #[doc="Read the CSR24 register."]
    #[inline] pub fn csr24(&self) -> Csr24 { 
        unsafe {
            read_volatile(self.csr24_ptr())
        }
    }

    #[doc="Write the CSR24 register."]
    #[inline] pub fn set_csr24<F: FnOnce(Csr24) -> Csr24>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr24_mut(), f(Csr24(0)));
        }
        self
    }

    #[doc="Modify the CSR24 register."]
    #[inline] pub fn with_csr24<F: FnOnce(Csr24) -> Csr24>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr24_mut(), f(self.csr24()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR25 register."]
    #[inline] pub fn csr25_mut(&self) -> *mut Csr25 { 
        (self.0 + 0x15c) as *mut Csr25
    }

    #[doc="Get the *const pointer for the CSR25 register."]
    #[inline] pub fn csr25_ptr(&self) -> *const Csr25 { 
           self.csr25_mut()
    }

    #[doc="Read the CSR25 register."]
    #[inline] pub fn csr25(&self) -> Csr25 { 
        unsafe {
            read_volatile(self.csr25_ptr())
        }
    }

    #[doc="Write the CSR25 register."]
    #[inline] pub fn set_csr25<F: FnOnce(Csr25) -> Csr25>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr25_mut(), f(Csr25(0)));
        }
        self
    }

    #[doc="Modify the CSR25 register."]
    #[inline] pub fn with_csr25<F: FnOnce(Csr25) -> Csr25>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr25_mut(), f(self.csr25()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR26 register."]
    #[inline] pub fn csr26_mut(&self) -> *mut Csr26 { 
        (self.0 + 0x160) as *mut Csr26
    }

    #[doc="Get the *const pointer for the CSR26 register."]
    #[inline] pub fn csr26_ptr(&self) -> *const Csr26 { 
           self.csr26_mut()
    }

    #[doc="Read the CSR26 register."]
    #[inline] pub fn csr26(&self) -> Csr26 { 
        unsafe {
            read_volatile(self.csr26_ptr())
        }
    }

    #[doc="Write the CSR26 register."]
    #[inline] pub fn set_csr26<F: FnOnce(Csr26) -> Csr26>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr26_mut(), f(Csr26(0)));
        }
        self
    }

    #[doc="Modify the CSR26 register."]
    #[inline] pub fn with_csr26<F: FnOnce(Csr26) -> Csr26>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr26_mut(), f(self.csr26()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR27 register."]
    #[inline] pub fn csr27_mut(&self) -> *mut Csr27 { 
        (self.0 + 0x164) as *mut Csr27
    }

    #[doc="Get the *const pointer for the CSR27 register."]
    #[inline] pub fn csr27_ptr(&self) -> *const Csr27 { 
           self.csr27_mut()
    }

    #[doc="Read the CSR27 register."]
    #[inline] pub fn csr27(&self) -> Csr27 { 
        unsafe {
            read_volatile(self.csr27_ptr())
        }
    }

    #[doc="Write the CSR27 register."]
    #[inline] pub fn set_csr27<F: FnOnce(Csr27) -> Csr27>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr27_mut(), f(Csr27(0)));
        }
        self
    }

    #[doc="Modify the CSR27 register."]
    #[inline] pub fn with_csr27<F: FnOnce(Csr27) -> Csr27>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr27_mut(), f(self.csr27()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR28 register."]
    #[inline] pub fn csr28_mut(&self) -> *mut Csr28 { 
        (self.0 + 0x168) as *mut Csr28
    }

    #[doc="Get the *const pointer for the CSR28 register."]
    #[inline] pub fn csr28_ptr(&self) -> *const Csr28 { 
           self.csr28_mut()
    }

    #[doc="Read the CSR28 register."]
    #[inline] pub fn csr28(&self) -> Csr28 { 
        unsafe {
            read_volatile(self.csr28_ptr())
        }
    }

    #[doc="Write the CSR28 register."]
    #[inline] pub fn set_csr28<F: FnOnce(Csr28) -> Csr28>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr28_mut(), f(Csr28(0)));
        }
        self
    }

    #[doc="Modify the CSR28 register."]
    #[inline] pub fn with_csr28<F: FnOnce(Csr28) -> Csr28>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr28_mut(), f(self.csr28()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR29 register."]
    #[inline] pub fn csr29_mut(&self) -> *mut Csr29 { 
        (self.0 + 0x16c) as *mut Csr29
    }

    #[doc="Get the *const pointer for the CSR29 register."]
    #[inline] pub fn csr29_ptr(&self) -> *const Csr29 { 
           self.csr29_mut()
    }

    #[doc="Read the CSR29 register."]
    #[inline] pub fn csr29(&self) -> Csr29 { 
        unsafe {
            read_volatile(self.csr29_ptr())
        }
    }

    #[doc="Write the CSR29 register."]
    #[inline] pub fn set_csr29<F: FnOnce(Csr29) -> Csr29>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr29_mut(), f(Csr29(0)));
        }
        self
    }

    #[doc="Modify the CSR29 register."]
    #[inline] pub fn with_csr29<F: FnOnce(Csr29) -> Csr29>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr29_mut(), f(self.csr29()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR30 register."]
    #[inline] pub fn csr30_mut(&self) -> *mut Csr30 { 
        (self.0 + 0x170) as *mut Csr30
    }

    #[doc="Get the *const pointer for the CSR30 register."]
    #[inline] pub fn csr30_ptr(&self) -> *const Csr30 { 
           self.csr30_mut()
    }

    #[doc="Read the CSR30 register."]
    #[inline] pub fn csr30(&self) -> Csr30 { 
        unsafe {
            read_volatile(self.csr30_ptr())
        }
    }

    #[doc="Write the CSR30 register."]
    #[inline] pub fn set_csr30<F: FnOnce(Csr30) -> Csr30>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr30_mut(), f(Csr30(0)));
        }
        self
    }

    #[doc="Modify the CSR30 register."]
    #[inline] pub fn with_csr30<F: FnOnce(Csr30) -> Csr30>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr30_mut(), f(self.csr30()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR31 register."]
    #[inline] pub fn csr31_mut(&self) -> *mut Csr31 { 
        (self.0 + 0x174) as *mut Csr31
    }

    #[doc="Get the *const pointer for the CSR31 register."]
    #[inline] pub fn csr31_ptr(&self) -> *const Csr31 { 
           self.csr31_mut()
    }

    #[doc="Read the CSR31 register."]
    #[inline] pub fn csr31(&self) -> Csr31 { 
        unsafe {
            read_volatile(self.csr31_ptr())
        }
    }

    #[doc="Write the CSR31 register."]
    #[inline] pub fn set_csr31<F: FnOnce(Csr31) -> Csr31>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr31_mut(), f(Csr31(0)));
        }
        self
    }

    #[doc="Modify the CSR31 register."]
    #[inline] pub fn with_csr31<F: FnOnce(Csr31) -> Csr31>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr31_mut(), f(self.csr31()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR32 register."]
    #[inline] pub fn csr32_mut(&self) -> *mut Csr32 { 
        (self.0 + 0x178) as *mut Csr32
    }

    #[doc="Get the *const pointer for the CSR32 register."]
    #[inline] pub fn csr32_ptr(&self) -> *const Csr32 { 
           self.csr32_mut()
    }

    #[doc="Read the CSR32 register."]
    #[inline] pub fn csr32(&self) -> Csr32 { 
        unsafe {
            read_volatile(self.csr32_ptr())
        }
    }

    #[doc="Write the CSR32 register."]
    #[inline] pub fn set_csr32<F: FnOnce(Csr32) -> Csr32>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr32_mut(), f(Csr32(0)));
        }
        self
    }

    #[doc="Modify the CSR32 register."]
    #[inline] pub fn with_csr32<F: FnOnce(Csr32) -> Csr32>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr32_mut(), f(self.csr32()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR33 register."]
    #[inline] pub fn csr33_mut(&self) -> *mut Csr33 { 
        (self.0 + 0x17c) as *mut Csr33
    }

    #[doc="Get the *const pointer for the CSR33 register."]
    #[inline] pub fn csr33_ptr(&self) -> *const Csr33 { 
           self.csr33_mut()
    }

    #[doc="Read the CSR33 register."]
    #[inline] pub fn csr33(&self) -> Csr33 { 
        unsafe {
            read_volatile(self.csr33_ptr())
        }
    }

    #[doc="Write the CSR33 register."]
    #[inline] pub fn set_csr33<F: FnOnce(Csr33) -> Csr33>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr33_mut(), f(Csr33(0)));
        }
        self
    }

    #[doc="Modify the CSR33 register."]
    #[inline] pub fn with_csr33<F: FnOnce(Csr33) -> Csr33>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr33_mut(), f(self.csr33()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR34 register."]
    #[inline] pub fn csr34_mut(&self) -> *mut Csr34 { 
        (self.0 + 0x180) as *mut Csr34
    }

    #[doc="Get the *const pointer for the CSR34 register."]
    #[inline] pub fn csr34_ptr(&self) -> *const Csr34 { 
           self.csr34_mut()
    }

    #[doc="Read the CSR34 register."]
    #[inline] pub fn csr34(&self) -> Csr34 { 
        unsafe {
            read_volatile(self.csr34_ptr())
        }
    }

    #[doc="Write the CSR34 register."]
    #[inline] pub fn set_csr34<F: FnOnce(Csr34) -> Csr34>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr34_mut(), f(Csr34(0)));
        }
        self
    }

    #[doc="Modify the CSR34 register."]
    #[inline] pub fn with_csr34<F: FnOnce(Csr34) -> Csr34>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr34_mut(), f(self.csr34()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR35 register."]
    #[inline] pub fn csr35_mut(&self) -> *mut Csr35 { 
        (self.0 + 0x184) as *mut Csr35
    }

    #[doc="Get the *const pointer for the CSR35 register."]
    #[inline] pub fn csr35_ptr(&self) -> *const Csr35 { 
           self.csr35_mut()
    }

    #[doc="Read the CSR35 register."]
    #[inline] pub fn csr35(&self) -> Csr35 { 
        unsafe {
            read_volatile(self.csr35_ptr())
        }
    }

    #[doc="Write the CSR35 register."]
    #[inline] pub fn set_csr35<F: FnOnce(Csr35) -> Csr35>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr35_mut(), f(Csr35(0)));
        }
        self
    }

    #[doc="Modify the CSR35 register."]
    #[inline] pub fn with_csr35<F: FnOnce(Csr35) -> Csr35>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr35_mut(), f(self.csr35()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR36 register."]
    #[inline] pub fn csr36_mut(&self) -> *mut Csr36 { 
        (self.0 + 0x188) as *mut Csr36
    }

    #[doc="Get the *const pointer for the CSR36 register."]
    #[inline] pub fn csr36_ptr(&self) -> *const Csr36 { 
           self.csr36_mut()
    }

    #[doc="Read the CSR36 register."]
    #[inline] pub fn csr36(&self) -> Csr36 { 
        unsafe {
            read_volatile(self.csr36_ptr())
        }
    }

    #[doc="Write the CSR36 register."]
    #[inline] pub fn set_csr36<F: FnOnce(Csr36) -> Csr36>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr36_mut(), f(Csr36(0)));
        }
        self
    }

    #[doc="Modify the CSR36 register."]
    #[inline] pub fn with_csr36<F: FnOnce(Csr36) -> Csr36>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr36_mut(), f(self.csr36()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR37 register."]
    #[inline] pub fn csr37_mut(&self) -> *mut Csr37 { 
        (self.0 + 0x18c) as *mut Csr37
    }

    #[doc="Get the *const pointer for the CSR37 register."]
    #[inline] pub fn csr37_ptr(&self) -> *const Csr37 { 
           self.csr37_mut()
    }

    #[doc="Read the CSR37 register."]
    #[inline] pub fn csr37(&self) -> Csr37 { 
        unsafe {
            read_volatile(self.csr37_ptr())
        }
    }

    #[doc="Write the CSR37 register."]
    #[inline] pub fn set_csr37<F: FnOnce(Csr37) -> Csr37>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr37_mut(), f(Csr37(0)));
        }
        self
    }

    #[doc="Modify the CSR37 register."]
    #[inline] pub fn with_csr37<F: FnOnce(Csr37) -> Csr37>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr37_mut(), f(self.csr37()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR38 register."]
    #[inline] pub fn csr38_mut(&self) -> *mut Csr38 { 
        (self.0 + 0x190) as *mut Csr38
    }

    #[doc="Get the *const pointer for the CSR38 register."]
    #[inline] pub fn csr38_ptr(&self) -> *const Csr38 { 
           self.csr38_mut()
    }

    #[doc="Read the CSR38 register."]
    #[inline] pub fn csr38(&self) -> Csr38 { 
        unsafe {
            read_volatile(self.csr38_ptr())
        }
    }

    #[doc="Write the CSR38 register."]
    #[inline] pub fn set_csr38<F: FnOnce(Csr38) -> Csr38>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr38_mut(), f(Csr38(0)));
        }
        self
    }

    #[doc="Modify the CSR38 register."]
    #[inline] pub fn with_csr38<F: FnOnce(Csr38) -> Csr38>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr38_mut(), f(self.csr38()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR39 register."]
    #[inline] pub fn csr39_mut(&self) -> *mut Csr39 { 
        (self.0 + 0x194) as *mut Csr39
    }

    #[doc="Get the *const pointer for the CSR39 register."]
    #[inline] pub fn csr39_ptr(&self) -> *const Csr39 { 
           self.csr39_mut()
    }

    #[doc="Read the CSR39 register."]
    #[inline] pub fn csr39(&self) -> Csr39 { 
        unsafe {
            read_volatile(self.csr39_ptr())
        }
    }

    #[doc="Write the CSR39 register."]
    #[inline] pub fn set_csr39<F: FnOnce(Csr39) -> Csr39>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr39_mut(), f(Csr39(0)));
        }
        self
    }

    #[doc="Modify the CSR39 register."]
    #[inline] pub fn with_csr39<F: FnOnce(Csr39) -> Csr39>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr39_mut(), f(self.csr39()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR40 register."]
    #[inline] pub fn csr40_mut(&self) -> *mut Csr40 { 
        (self.0 + 0x198) as *mut Csr40
    }

    #[doc="Get the *const pointer for the CSR40 register."]
    #[inline] pub fn csr40_ptr(&self) -> *const Csr40 { 
           self.csr40_mut()
    }

    #[doc="Read the CSR40 register."]
    #[inline] pub fn csr40(&self) -> Csr40 { 
        unsafe {
            read_volatile(self.csr40_ptr())
        }
    }

    #[doc="Write the CSR40 register."]
    #[inline] pub fn set_csr40<F: FnOnce(Csr40) -> Csr40>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr40_mut(), f(Csr40(0)));
        }
        self
    }

    #[doc="Modify the CSR40 register."]
    #[inline] pub fn with_csr40<F: FnOnce(Csr40) -> Csr40>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr40_mut(), f(self.csr40()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR41 register."]
    #[inline] pub fn csr41_mut(&self) -> *mut Csr41 { 
        (self.0 + 0x19c) as *mut Csr41
    }

    #[doc="Get the *const pointer for the CSR41 register."]
    #[inline] pub fn csr41_ptr(&self) -> *const Csr41 { 
           self.csr41_mut()
    }

    #[doc="Read the CSR41 register."]
    #[inline] pub fn csr41(&self) -> Csr41 { 
        unsafe {
            read_volatile(self.csr41_ptr())
        }
    }

    #[doc="Write the CSR41 register."]
    #[inline] pub fn set_csr41<F: FnOnce(Csr41) -> Csr41>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr41_mut(), f(Csr41(0)));
        }
        self
    }

    #[doc="Modify the CSR41 register."]
    #[inline] pub fn with_csr41<F: FnOnce(Csr41) -> Csr41>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr41_mut(), f(self.csr41()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR42 register."]
    #[inline] pub fn csr42_mut(&self) -> *mut Csr42 { 
        (self.0 + 0x1a0) as *mut Csr42
    }

    #[doc="Get the *const pointer for the CSR42 register."]
    #[inline] pub fn csr42_ptr(&self) -> *const Csr42 { 
           self.csr42_mut()
    }

    #[doc="Read the CSR42 register."]
    #[inline] pub fn csr42(&self) -> Csr42 { 
        unsafe {
            read_volatile(self.csr42_ptr())
        }
    }

    #[doc="Write the CSR42 register."]
    #[inline] pub fn set_csr42<F: FnOnce(Csr42) -> Csr42>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr42_mut(), f(Csr42(0)));
        }
        self
    }

    #[doc="Modify the CSR42 register."]
    #[inline] pub fn with_csr42<F: FnOnce(Csr42) -> Csr42>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr42_mut(), f(self.csr42()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR43 register."]
    #[inline] pub fn csr43_mut(&self) -> *mut Csr43 { 
        (self.0 + 0x1a4) as *mut Csr43
    }

    #[doc="Get the *const pointer for the CSR43 register."]
    #[inline] pub fn csr43_ptr(&self) -> *const Csr43 { 
           self.csr43_mut()
    }

    #[doc="Read the CSR43 register."]
    #[inline] pub fn csr43(&self) -> Csr43 { 
        unsafe {
            read_volatile(self.csr43_ptr())
        }
    }

    #[doc="Write the CSR43 register."]
    #[inline] pub fn set_csr43<F: FnOnce(Csr43) -> Csr43>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr43_mut(), f(Csr43(0)));
        }
        self
    }

    #[doc="Modify the CSR43 register."]
    #[inline] pub fn with_csr43<F: FnOnce(Csr43) -> Csr43>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr43_mut(), f(self.csr43()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR44 register."]
    #[inline] pub fn csr44_mut(&self) -> *mut Csr44 { 
        (self.0 + 0x1a8) as *mut Csr44
    }

    #[doc="Get the *const pointer for the CSR44 register."]
    #[inline] pub fn csr44_ptr(&self) -> *const Csr44 { 
           self.csr44_mut()
    }

    #[doc="Read the CSR44 register."]
    #[inline] pub fn csr44(&self) -> Csr44 { 
        unsafe {
            read_volatile(self.csr44_ptr())
        }
    }

    #[doc="Write the CSR44 register."]
    #[inline] pub fn set_csr44<F: FnOnce(Csr44) -> Csr44>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr44_mut(), f(Csr44(0)));
        }
        self
    }

    #[doc="Modify the CSR44 register."]
    #[inline] pub fn with_csr44<F: FnOnce(Csr44) -> Csr44>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr44_mut(), f(self.csr44()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR45 register."]
    #[inline] pub fn csr45_mut(&self) -> *mut Csr45 { 
        (self.0 + 0x1ac) as *mut Csr45
    }

    #[doc="Get the *const pointer for the CSR45 register."]
    #[inline] pub fn csr45_ptr(&self) -> *const Csr45 { 
           self.csr45_mut()
    }

    #[doc="Read the CSR45 register."]
    #[inline] pub fn csr45(&self) -> Csr45 { 
        unsafe {
            read_volatile(self.csr45_ptr())
        }
    }

    #[doc="Write the CSR45 register."]
    #[inline] pub fn set_csr45<F: FnOnce(Csr45) -> Csr45>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr45_mut(), f(Csr45(0)));
        }
        self
    }

    #[doc="Modify the CSR45 register."]
    #[inline] pub fn with_csr45<F: FnOnce(Csr45) -> Csr45>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr45_mut(), f(self.csr45()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR46 register."]
    #[inline] pub fn csr46_mut(&self) -> *mut Csr46 { 
        (self.0 + 0x1b0) as *mut Csr46
    }

    #[doc="Get the *const pointer for the CSR46 register."]
    #[inline] pub fn csr46_ptr(&self) -> *const Csr46 { 
           self.csr46_mut()
    }

    #[doc="Read the CSR46 register."]
    #[inline] pub fn csr46(&self) -> Csr46 { 
        unsafe {
            read_volatile(self.csr46_ptr())
        }
    }

    #[doc="Write the CSR46 register."]
    #[inline] pub fn set_csr46<F: FnOnce(Csr46) -> Csr46>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr46_mut(), f(Csr46(0)));
        }
        self
    }

    #[doc="Modify the CSR46 register."]
    #[inline] pub fn with_csr46<F: FnOnce(Csr46) -> Csr46>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr46_mut(), f(self.csr46()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR47 register."]
    #[inline] pub fn csr47_mut(&self) -> *mut Csr47 { 
        (self.0 + 0x1b4) as *mut Csr47
    }

    #[doc="Get the *const pointer for the CSR47 register."]
    #[inline] pub fn csr47_ptr(&self) -> *const Csr47 { 
           self.csr47_mut()
    }

    #[doc="Read the CSR47 register."]
    #[inline] pub fn csr47(&self) -> Csr47 { 
        unsafe {
            read_volatile(self.csr47_ptr())
        }
    }

    #[doc="Write the CSR47 register."]
    #[inline] pub fn set_csr47<F: FnOnce(Csr47) -> Csr47>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr47_mut(), f(Csr47(0)));
        }
        self
    }

    #[doc="Modify the CSR47 register."]
    #[inline] pub fn with_csr47<F: FnOnce(Csr47) -> Csr47>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr47_mut(), f(self.csr47()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR48 register."]
    #[inline] pub fn csr48_mut(&self) -> *mut Csr48 { 
        (self.0 + 0x1b8) as *mut Csr48
    }

    #[doc="Get the *const pointer for the CSR48 register."]
    #[inline] pub fn csr48_ptr(&self) -> *const Csr48 { 
           self.csr48_mut()
    }

    #[doc="Read the CSR48 register."]
    #[inline] pub fn csr48(&self) -> Csr48 { 
        unsafe {
            read_volatile(self.csr48_ptr())
        }
    }

    #[doc="Write the CSR48 register."]
    #[inline] pub fn set_csr48<F: FnOnce(Csr48) -> Csr48>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr48_mut(), f(Csr48(0)));
        }
        self
    }

    #[doc="Modify the CSR48 register."]
    #[inline] pub fn with_csr48<F: FnOnce(Csr48) -> Csr48>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr48_mut(), f(self.csr48()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR49 register."]
    #[inline] pub fn csr49_mut(&self) -> *mut Csr49 { 
        (self.0 + 0x1bc) as *mut Csr49
    }

    #[doc="Get the *const pointer for the CSR49 register."]
    #[inline] pub fn csr49_ptr(&self) -> *const Csr49 { 
           self.csr49_mut()
    }

    #[doc="Read the CSR49 register."]
    #[inline] pub fn csr49(&self) -> Csr49 { 
        unsafe {
            read_volatile(self.csr49_ptr())
        }
    }

    #[doc="Write the CSR49 register."]
    #[inline] pub fn set_csr49<F: FnOnce(Csr49) -> Csr49>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr49_mut(), f(Csr49(0)));
        }
        self
    }

    #[doc="Modify the CSR49 register."]
    #[inline] pub fn with_csr49<F: FnOnce(Csr49) -> Csr49>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr49_mut(), f(self.csr49()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR50 register."]
    #[inline] pub fn csr50_mut(&self) -> *mut Csr50 { 
        (self.0 + 0x1c0) as *mut Csr50
    }

    #[doc="Get the *const pointer for the CSR50 register."]
    #[inline] pub fn csr50_ptr(&self) -> *const Csr50 { 
           self.csr50_mut()
    }

    #[doc="Read the CSR50 register."]
    #[inline] pub fn csr50(&self) -> Csr50 { 
        unsafe {
            read_volatile(self.csr50_ptr())
        }
    }

    #[doc="Write the CSR50 register."]
    #[inline] pub fn set_csr50<F: FnOnce(Csr50) -> Csr50>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr50_mut(), f(Csr50(0)));
        }
        self
    }

    #[doc="Modify the CSR50 register."]
    #[inline] pub fn with_csr50<F: FnOnce(Csr50) -> Csr50>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr50_mut(), f(self.csr50()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR51 register."]
    #[inline] pub fn csr51_mut(&self) -> *mut Csr51 { 
        (self.0 + 0x1c4) as *mut Csr51
    }

    #[doc="Get the *const pointer for the CSR51 register."]
    #[inline] pub fn csr51_ptr(&self) -> *const Csr51 { 
           self.csr51_mut()
    }

    #[doc="Read the CSR51 register."]
    #[inline] pub fn csr51(&self) -> Csr51 { 
        unsafe {
            read_volatile(self.csr51_ptr())
        }
    }

    #[doc="Write the CSR51 register."]
    #[inline] pub fn set_csr51<F: FnOnce(Csr51) -> Csr51>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr51_mut(), f(Csr51(0)));
        }
        self
    }

    #[doc="Modify the CSR51 register."]
    #[inline] pub fn with_csr51<F: FnOnce(Csr51) -> Csr51>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr51_mut(), f(self.csr51()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR52 register."]
    #[inline] pub fn csr52_mut(&self) -> *mut Csr52 { 
        (self.0 + 0x1c8) as *mut Csr52
    }

    #[doc="Get the *const pointer for the CSR52 register."]
    #[inline] pub fn csr52_ptr(&self) -> *const Csr52 { 
           self.csr52_mut()
    }

    #[doc="Read the CSR52 register."]
    #[inline] pub fn csr52(&self) -> Csr52 { 
        unsafe {
            read_volatile(self.csr52_ptr())
        }
    }

    #[doc="Write the CSR52 register."]
    #[inline] pub fn set_csr52<F: FnOnce(Csr52) -> Csr52>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr52_mut(), f(Csr52(0)));
        }
        self
    }

    #[doc="Modify the CSR52 register."]
    #[inline] pub fn with_csr52<F: FnOnce(Csr52) -> Csr52>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr52_mut(), f(self.csr52()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR53 register."]
    #[inline] pub fn csr53_mut(&self) -> *mut Csr53 { 
        (self.0 + 0x1cc) as *mut Csr53
    }

    #[doc="Get the *const pointer for the CSR53 register."]
    #[inline] pub fn csr53_ptr(&self) -> *const Csr53 { 
           self.csr53_mut()
    }

    #[doc="Read the CSR53 register."]
    #[inline] pub fn csr53(&self) -> Csr53 { 
        unsafe {
            read_volatile(self.csr53_ptr())
        }
    }

    #[doc="Write the CSR53 register."]
    #[inline] pub fn set_csr53<F: FnOnce(Csr53) -> Csr53>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr53_mut(), f(Csr53(0)));
        }
        self
    }

    #[doc="Modify the CSR53 register."]
    #[inline] pub fn with_csr53<F: FnOnce(Csr53) -> Csr53>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr53_mut(), f(self.csr53()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HASH_HR0 register."]
    #[inline] pub fn hash_hr0_mut(&self) -> *mut HashHr0 { 
        (self.0 + 0x310) as *mut HashHr0
    }

    #[doc="Get the *const pointer for the HASH_HR0 register."]
    #[inline] pub fn hash_hr0_ptr(&self) -> *const HashHr0 { 
           self.hash_hr0_mut()
    }

    #[doc="Read the HASH_HR0 register."]
    #[inline] pub fn hash_hr0(&self) -> HashHr0 { 
        unsafe {
            read_volatile(self.hash_hr0_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HASH_HR1 register."]
    #[inline] pub fn hash_hr1_mut(&self) -> *mut HashHr1 { 
        (self.0 + 0x314) as *mut HashHr1
    }

    #[doc="Get the *const pointer for the HASH_HR1 register."]
    #[inline] pub fn hash_hr1_ptr(&self) -> *const HashHr1 { 
           self.hash_hr1_mut()
    }

    #[doc="Read the HASH_HR1 register."]
    #[inline] pub fn hash_hr1(&self) -> HashHr1 { 
        unsafe {
            read_volatile(self.hash_hr1_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HASH_HR2 register."]
    #[inline] pub fn hash_hr2_mut(&self) -> *mut HashHr2 { 
        (self.0 + 0x318) as *mut HashHr2
    }

    #[doc="Get the *const pointer for the HASH_HR2 register."]
    #[inline] pub fn hash_hr2_ptr(&self) -> *const HashHr2 { 
           self.hash_hr2_mut()
    }

    #[doc="Read the HASH_HR2 register."]
    #[inline] pub fn hash_hr2(&self) -> HashHr2 { 
        unsafe {
            read_volatile(self.hash_hr2_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HASH_HR3 register."]
    #[inline] pub fn hash_hr3_mut(&self) -> *mut HashHr3 { 
        (self.0 + 0x31c) as *mut HashHr3
    }

    #[doc="Get the *const pointer for the HASH_HR3 register."]
    #[inline] pub fn hash_hr3_ptr(&self) -> *const HashHr3 { 
           self.hash_hr3_mut()
    }

    #[doc="Read the HASH_HR3 register."]
    #[inline] pub fn hash_hr3(&self) -> HashHr3 { 
        unsafe {
            read_volatile(self.hash_hr3_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HASH_HR4 register."]
    #[inline] pub fn hash_hr4_mut(&self) -> *mut HashHr4 { 
        (self.0 + 0x320) as *mut HashHr4
    }

    #[doc="Get the *const pointer for the HASH_HR4 register."]
    #[inline] pub fn hash_hr4_ptr(&self) -> *const HashHr4 { 
           self.hash_hr4_mut()
    }

    #[doc="Read the HASH_HR4 register."]
    #[inline] pub fn hash_hr4(&self) -> HashHr4 { 
        unsafe {
            read_volatile(self.hash_hr4_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HASH_HR5 register."]
    #[inline] pub fn hash_hr5_mut(&self) -> *mut HashHr5 { 
        (self.0 + 0x324) as *mut HashHr5
    }

    #[doc="Get the *const pointer for the HASH_HR5 register."]
    #[inline] pub fn hash_hr5_ptr(&self) -> *const HashHr5 { 
           self.hash_hr5_mut()
    }

    #[doc="Read the HASH_HR5 register."]
    #[inline] pub fn hash_hr5(&self) -> HashHr5 { 
        unsafe {
            read_volatile(self.hash_hr5_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HASH_HR6 register."]
    #[inline] pub fn hash_hr6_mut(&self) -> *mut HashHr6 { 
        (self.0 + 0x328) as *mut HashHr6
    }

    #[doc="Get the *const pointer for the HASH_HR6 register."]
    #[inline] pub fn hash_hr6_ptr(&self) -> *const HashHr6 { 
           self.hash_hr6_mut()
    }

    #[doc="Read the HASH_HR6 register."]
    #[inline] pub fn hash_hr6(&self) -> HashHr6 { 
        unsafe {
            read_volatile(self.hash_hr6_ptr())
        }
    }

    #[doc="Get the *mut pointer for the HASH_HR7 register."]
    #[inline] pub fn hash_hr7_mut(&self) -> *mut HashHr7 { 
        (self.0 + 0x32c) as *mut HashHr7
    }

    #[doc="Get the *const pointer for the HASH_HR7 register."]
    #[inline] pub fn hash_hr7_ptr(&self) -> *const HashHr7 { 
           self.hash_hr7_mut()
    }

    #[doc="Read the HASH_HR7 register."]
    #[inline] pub fn hash_hr7(&self) -> HashHr7 { 
        unsafe {
            read_volatile(self.hash_hr7_ptr())
        }
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Initialize message digest calculation"]
    #[inline] pub fn init(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if INIT != 0"]
    #[inline] pub fn test_init(&self) -> bool {
        self.init() != 0
    }

    #[doc="Sets the INIT field."]
    #[inline] pub fn set_init<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DMA enable"]
    #[inline] pub fn dmae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMAE != 0"]
    #[inline] pub fn test_dmae(&self) -> bool {
        self.dmae() != 0
    }

    #[doc="Sets the DMAE field."]
    #[inline] pub fn set_dmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data type selection"]
    #[inline] pub fn datatype(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if DATATYPE != 0"]
    #[inline] pub fn test_datatype(&self) -> bool {
        self.datatype() != 0
    }

    #[doc="Sets the DATATYPE field."]
    #[inline] pub fn set_datatype<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Mode selection"]
    #[inline] pub fn mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Algorithm selection"]
    #[inline] pub fn algo0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ALGO0 != 0"]
    #[inline] pub fn test_algo0(&self) -> bool {
        self.algo0() != 0
    }

    #[doc="Sets the ALGO0 field."]
    #[inline] pub fn set_algo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Number of words already pushed"]
    #[inline] pub fn nbw(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if NBW != 0"]
    #[inline] pub fn test_nbw(&self) -> bool {
        self.nbw() != 0
    }

    #[doc="Sets the NBW field."]
    #[inline] pub fn set_nbw<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DIN not empty"]
    #[inline] pub fn dinne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DINNE != 0"]
    #[inline] pub fn test_dinne(&self) -> bool {
        self.dinne() != 0
    }

    #[doc="Sets the DINNE field."]
    #[inline] pub fn set_dinne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Multiple DMA Transfers"]
    #[inline] pub fn mdmat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if MDMAT != 0"]
    #[inline] pub fn test_mdmat(&self) -> bool {
        self.mdmat() != 0
    }

    #[doc="Sets the MDMAT field."]
    #[inline] pub fn set_mdmat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Long key selection"]
    #[inline] pub fn lkey(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if LKEY != 0"]
    #[inline] pub fn test_lkey(&self) -> bool {
        self.lkey() != 0
    }

    #[doc="Sets the LKEY field."]
    #[inline] pub fn set_lkey<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ALGO"]
    #[inline] pub fn algo1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ALGO1 != 0"]
    #[inline] pub fn test_algo1(&self) -> bool {
        self.algo1() != 0
    }

    #[doc="Sets the ALGO1 field."]
    #[inline] pub fn set_algo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
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
        if self.init() != 0 { try!(write!(f, " init"))}
        if self.dmae() != 0 { try!(write!(f, " dmae"))}
        if self.datatype() != 0 { try!(write!(f, " datatype=0x{:x}", self.datatype()))}
        if self.mode() != 0 { try!(write!(f, " mode"))}
        if self.algo0() != 0 { try!(write!(f, " algo0"))}
        if self.nbw() != 0 { try!(write!(f, " nbw=0x{:x}", self.nbw()))}
        if self.dinne() != 0 { try!(write!(f, " dinne"))}
        if self.mdmat() != 0 { try!(write!(f, " mdmat"))}
        if self.lkey() != 0 { try!(write!(f, " lkey"))}
        if self.algo1() != 0 { try!(write!(f, " algo1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data input register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Din(pub u32);
impl Din {
    #[doc="Data input"]
    #[inline] pub fn datain(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATAIN != 0"]
    #[inline] pub fn test_datain(&self) -> bool {
        self.datain() != 0
    }

    #[doc="Sets the DATAIN field."]
    #[inline] pub fn set_datain<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Din {
    #[inline]
    fn from(other: u32) -> Self {
         Din(other)
    }
}

impl ::core::fmt::Display for Din {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Din {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="start register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Str(pub u32);
impl Str {
    #[doc="Digest calculation"]
    #[inline] pub fn dcal(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DCAL != 0"]
    #[inline] pub fn test_dcal(&self) -> bool {
        self.dcal() != 0
    }

    #[doc="Sets the DCAL field."]
    #[inline] pub fn set_dcal<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Number of valid bits in the last word of the message"]
    #[inline] pub fn nblw(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if NBLW != 0"]
    #[inline] pub fn test_nblw(&self) -> bool {
        self.nblw() != 0
    }

    #[doc="Sets the NBLW field."]
    #[inline] pub fn set_nblw<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Str {
    #[inline]
    fn from(other: u32) -> Self {
         Str(other)
    }
}

impl ::core::fmt::Display for Str {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Str {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dcal() != 0 { try!(write!(f, " dcal"))}
        if self.nblw() != 0 { try!(write!(f, " nblw=0x{:x}", self.nblw()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="digest registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hr0(pub u32);
impl Hr0 {
    #[doc="H0"]
    #[inline] pub fn h0(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H0 != 0"]
    #[inline] pub fn test_h0(&self) -> bool {
        self.h0() != 0
    }

    #[doc="Sets the H0 field."]
    #[inline] pub fn set_h0<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Hr0(other)
    }
}

impl ::core::fmt::Display for Hr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="digest registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hr1(pub u32);
impl Hr1 {
    #[doc="H1"]
    #[inline] pub fn h1(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H1 != 0"]
    #[inline] pub fn test_h1(&self) -> bool {
        self.h1() != 0
    }

    #[doc="Sets the H1 field."]
    #[inline] pub fn set_h1<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Hr1(other)
    }
}

impl ::core::fmt::Display for Hr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="digest registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hr2(pub u32);
impl Hr2 {
    #[doc="H2"]
    #[inline] pub fn h2(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H2 != 0"]
    #[inline] pub fn test_h2(&self) -> bool {
        self.h2() != 0
    }

    #[doc="Sets the H2 field."]
    #[inline] pub fn set_h2<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Hr2(other)
    }
}

impl ::core::fmt::Display for Hr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="digest registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hr3(pub u32);
impl Hr3 {
    #[doc="H3"]
    #[inline] pub fn h3(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H3 != 0"]
    #[inline] pub fn test_h3(&self) -> bool {
        self.h3() != 0
    }

    #[doc="Sets the H3 field."]
    #[inline] pub fn set_h3<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Hr3(other)
    }
}

impl ::core::fmt::Display for Hr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="digest registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hr4(pub u32);
impl Hr4 {
    #[doc="H4"]
    #[inline] pub fn h4(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H4 != 0"]
    #[inline] pub fn test_h4(&self) -> bool {
        self.h4() != 0
    }

    #[doc="Sets the H4 field."]
    #[inline] pub fn set_h4<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Hr4(other)
    }
}

impl ::core::fmt::Display for Hr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc="Digest calculation completion interrupt enable"]
    #[inline] pub fn dcie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DCIE != 0"]
    #[inline] pub fn test_dcie(&self) -> bool {
        self.dcie() != 0
    }

    #[doc="Sets the DCIE field."]
    #[inline] pub fn set_dcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data input interrupt enable"]
    #[inline] pub fn dinie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DINIE != 0"]
    #[inline] pub fn test_dinie(&self) -> bool {
        self.dinie() != 0
    }

    #[doc="Sets the DINIE field."]
    #[inline] pub fn set_dinie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Imr {
    #[inline]
    fn from(other: u32) -> Self {
         Imr(other)
    }
}

impl ::core::fmt::Display for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dcie() != 0 { try!(write!(f, " dcie"))}
        if self.dinie() != 0 { try!(write!(f, " dinie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Busy bit"]
    #[inline] pub fn busy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DMA Status"]
    #[inline] pub fn dmas(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMAS != 0"]
    #[inline] pub fn test_dmas(&self) -> bool {
        self.dmas() != 0
    }

    #[doc="Sets the DMAS field."]
    #[inline] pub fn set_dmas<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Digest calculation completion interrupt status"]
    #[inline] pub fn dcis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DCIS != 0"]
    #[inline] pub fn test_dcis(&self) -> bool {
        self.dcis() != 0
    }

    #[doc="Sets the DCIS field."]
    #[inline] pub fn set_dcis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data input interrupt status"]
    #[inline] pub fn dinis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DINIS != 0"]
    #[inline] pub fn test_dinis(&self) -> bool {
        self.dinis() != 0
    }

    #[doc="Sets the DINIS field."]
    #[inline] pub fn set_dinis<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.dmas() != 0 { try!(write!(f, " dmas"))}
        if self.dcis() != 0 { try!(write!(f, " dcis"))}
        if self.dinis() != 0 { try!(write!(f, " dinis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr0(pub u32);
impl Csr0 {
    #[doc="CSR0"]
    #[inline] pub fn csr0(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR0 != 0"]
    #[inline] pub fn test_csr0(&self) -> bool {
        self.csr0() != 0
    }

    #[doc="Sets the CSR0 field."]
    #[inline] pub fn set_csr0<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr0(other)
    }
}

impl ::core::fmt::Display for Csr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr1(pub u32);
impl Csr1 {
    #[doc="CSR1"]
    #[inline] pub fn csr1(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR1 != 0"]
    #[inline] pub fn test_csr1(&self) -> bool {
        self.csr1() != 0
    }

    #[doc="Sets the CSR1 field."]
    #[inline] pub fn set_csr1<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr1(other)
    }
}

impl ::core::fmt::Display for Csr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr2(pub u32);
impl Csr2 {
    #[doc="CSR2"]
    #[inline] pub fn csr2(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR2 != 0"]
    #[inline] pub fn test_csr2(&self) -> bool {
        self.csr2() != 0
    }

    #[doc="Sets the CSR2 field."]
    #[inline] pub fn set_csr2<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr2(other)
    }
}

impl ::core::fmt::Display for Csr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr3(pub u32);
impl Csr3 {
    #[doc="CSR3"]
    #[inline] pub fn csr3(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR3 != 0"]
    #[inline] pub fn test_csr3(&self) -> bool {
        self.csr3() != 0
    }

    #[doc="Sets the CSR3 field."]
    #[inline] pub fn set_csr3<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr3(other)
    }
}

impl ::core::fmt::Display for Csr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr4(pub u32);
impl Csr4 {
    #[doc="CSR4"]
    #[inline] pub fn csr4(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR4 != 0"]
    #[inline] pub fn test_csr4(&self) -> bool {
        self.csr4() != 0
    }

    #[doc="Sets the CSR4 field."]
    #[inline] pub fn set_csr4<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr4(other)
    }
}

impl ::core::fmt::Display for Csr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr5(pub u32);
impl Csr5 {
    #[doc="CSR5"]
    #[inline] pub fn csr5(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR5 != 0"]
    #[inline] pub fn test_csr5(&self) -> bool {
        self.csr5() != 0
    }

    #[doc="Sets the CSR5 field."]
    #[inline] pub fn set_csr5<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr5 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr5(other)
    }
}

impl ::core::fmt::Display for Csr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr6(pub u32);
impl Csr6 {
    #[doc="CSR6"]
    #[inline] pub fn csr6(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR6 != 0"]
    #[inline] pub fn test_csr6(&self) -> bool {
        self.csr6() != 0
    }

    #[doc="Sets the CSR6 field."]
    #[inline] pub fn set_csr6<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr6 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr6(other)
    }
}

impl ::core::fmt::Display for Csr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr7(pub u32);
impl Csr7 {
    #[doc="CSR7"]
    #[inline] pub fn csr7(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR7 != 0"]
    #[inline] pub fn test_csr7(&self) -> bool {
        self.csr7() != 0
    }

    #[doc="Sets the CSR7 field."]
    #[inline] pub fn set_csr7<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr7 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr7(other)
    }
}

impl ::core::fmt::Display for Csr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr8(pub u32);
impl Csr8 {
    #[doc="CSR8"]
    #[inline] pub fn csr8(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR8 != 0"]
    #[inline] pub fn test_csr8(&self) -> bool {
        self.csr8() != 0
    }

    #[doc="Sets the CSR8 field."]
    #[inline] pub fn set_csr8<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr8 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr8(other)
    }
}

impl ::core::fmt::Display for Csr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr9(pub u32);
impl Csr9 {
    #[doc="CSR9"]
    #[inline] pub fn csr9(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR9 != 0"]
    #[inline] pub fn test_csr9(&self) -> bool {
        self.csr9() != 0
    }

    #[doc="Sets the CSR9 field."]
    #[inline] pub fn set_csr9<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr9 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr9(other)
    }
}

impl ::core::fmt::Display for Csr9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr10(pub u32);
impl Csr10 {
    #[doc="CSR10"]
    #[inline] pub fn csr10(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR10 != 0"]
    #[inline] pub fn test_csr10(&self) -> bool {
        self.csr10() != 0
    }

    #[doc="Sets the CSR10 field."]
    #[inline] pub fn set_csr10<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr10 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr10(other)
    }
}

impl ::core::fmt::Display for Csr10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr11(pub u32);
impl Csr11 {
    #[doc="CSR11"]
    #[inline] pub fn csr11(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR11 != 0"]
    #[inline] pub fn test_csr11(&self) -> bool {
        self.csr11() != 0
    }

    #[doc="Sets the CSR11 field."]
    #[inline] pub fn set_csr11<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr11 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr11(other)
    }
}

impl ::core::fmt::Display for Csr11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr12(pub u32);
impl Csr12 {
    #[doc="CSR12"]
    #[inline] pub fn csr12(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR12 != 0"]
    #[inline] pub fn test_csr12(&self) -> bool {
        self.csr12() != 0
    }

    #[doc="Sets the CSR12 field."]
    #[inline] pub fn set_csr12<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr12 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr12(other)
    }
}

impl ::core::fmt::Display for Csr12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr13(pub u32);
impl Csr13 {
    #[doc="CSR13"]
    #[inline] pub fn csr13(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR13 != 0"]
    #[inline] pub fn test_csr13(&self) -> bool {
        self.csr13() != 0
    }

    #[doc="Sets the CSR13 field."]
    #[inline] pub fn set_csr13<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr13 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr13(other)
    }
}

impl ::core::fmt::Display for Csr13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr14(pub u32);
impl Csr14 {
    #[doc="CSR14"]
    #[inline] pub fn csr14(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR14 != 0"]
    #[inline] pub fn test_csr14(&self) -> bool {
        self.csr14() != 0
    }

    #[doc="Sets the CSR14 field."]
    #[inline] pub fn set_csr14<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr14 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr14(other)
    }
}

impl ::core::fmt::Display for Csr14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr15(pub u32);
impl Csr15 {
    #[doc="CSR15"]
    #[inline] pub fn csr15(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR15 != 0"]
    #[inline] pub fn test_csr15(&self) -> bool {
        self.csr15() != 0
    }

    #[doc="Sets the CSR15 field."]
    #[inline] pub fn set_csr15<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr15 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr15(other)
    }
}

impl ::core::fmt::Display for Csr15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr16(pub u32);
impl Csr16 {
    #[doc="CSR16"]
    #[inline] pub fn csr16(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR16 != 0"]
    #[inline] pub fn test_csr16(&self) -> bool {
        self.csr16() != 0
    }

    #[doc="Sets the CSR16 field."]
    #[inline] pub fn set_csr16<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr16 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr16(other)
    }
}

impl ::core::fmt::Display for Csr16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr17(pub u32);
impl Csr17 {
    #[doc="CSR17"]
    #[inline] pub fn csr17(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR17 != 0"]
    #[inline] pub fn test_csr17(&self) -> bool {
        self.csr17() != 0
    }

    #[doc="Sets the CSR17 field."]
    #[inline] pub fn set_csr17<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr17 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr17(other)
    }
}

impl ::core::fmt::Display for Csr17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr18(pub u32);
impl Csr18 {
    #[doc="CSR18"]
    #[inline] pub fn csr18(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR18 != 0"]
    #[inline] pub fn test_csr18(&self) -> bool {
        self.csr18() != 0
    }

    #[doc="Sets the CSR18 field."]
    #[inline] pub fn set_csr18<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr18 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr18(other)
    }
}

impl ::core::fmt::Display for Csr18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr19(pub u32);
impl Csr19 {
    #[doc="CSR19"]
    #[inline] pub fn csr19(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR19 != 0"]
    #[inline] pub fn test_csr19(&self) -> bool {
        self.csr19() != 0
    }

    #[doc="Sets the CSR19 field."]
    #[inline] pub fn set_csr19<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr19 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr19(other)
    }
}

impl ::core::fmt::Display for Csr19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr20(pub u32);
impl Csr20 {
    #[doc="CSR20"]
    #[inline] pub fn csr20(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR20 != 0"]
    #[inline] pub fn test_csr20(&self) -> bool {
        self.csr20() != 0
    }

    #[doc="Sets the CSR20 field."]
    #[inline] pub fn set_csr20<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr20 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr20(other)
    }
}

impl ::core::fmt::Display for Csr20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr21(pub u32);
impl Csr21 {
    #[doc="CSR21"]
    #[inline] pub fn csr21(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR21 != 0"]
    #[inline] pub fn test_csr21(&self) -> bool {
        self.csr21() != 0
    }

    #[doc="Sets the CSR21 field."]
    #[inline] pub fn set_csr21<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr21 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr21(other)
    }
}

impl ::core::fmt::Display for Csr21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr22(pub u32);
impl Csr22 {
    #[doc="CSR22"]
    #[inline] pub fn csr22(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR22 != 0"]
    #[inline] pub fn test_csr22(&self) -> bool {
        self.csr22() != 0
    }

    #[doc="Sets the CSR22 field."]
    #[inline] pub fn set_csr22<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr22 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr22(other)
    }
}

impl ::core::fmt::Display for Csr22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr23(pub u32);
impl Csr23 {
    #[doc="CSR23"]
    #[inline] pub fn csr23(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR23 != 0"]
    #[inline] pub fn test_csr23(&self) -> bool {
        self.csr23() != 0
    }

    #[doc="Sets the CSR23 field."]
    #[inline] pub fn set_csr23<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr23 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr23(other)
    }
}

impl ::core::fmt::Display for Csr23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr24(pub u32);
impl Csr24 {
    #[doc="CSR24"]
    #[inline] pub fn csr24(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR24 != 0"]
    #[inline] pub fn test_csr24(&self) -> bool {
        self.csr24() != 0
    }

    #[doc="Sets the CSR24 field."]
    #[inline] pub fn set_csr24<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr24 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr24(other)
    }
}

impl ::core::fmt::Display for Csr24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr25(pub u32);
impl Csr25 {
    #[doc="CSR25"]
    #[inline] pub fn csr25(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR25 != 0"]
    #[inline] pub fn test_csr25(&self) -> bool {
        self.csr25() != 0
    }

    #[doc="Sets the CSR25 field."]
    #[inline] pub fn set_csr25<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr25 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr25(other)
    }
}

impl ::core::fmt::Display for Csr25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr26(pub u32);
impl Csr26 {
    #[doc="CSR26"]
    #[inline] pub fn csr26(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR26 != 0"]
    #[inline] pub fn test_csr26(&self) -> bool {
        self.csr26() != 0
    }

    #[doc="Sets the CSR26 field."]
    #[inline] pub fn set_csr26<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr26 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr26(other)
    }
}

impl ::core::fmt::Display for Csr26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr27(pub u32);
impl Csr27 {
    #[doc="CSR27"]
    #[inline] pub fn csr27(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR27 != 0"]
    #[inline] pub fn test_csr27(&self) -> bool {
        self.csr27() != 0
    }

    #[doc="Sets the CSR27 field."]
    #[inline] pub fn set_csr27<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr27 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr27(other)
    }
}

impl ::core::fmt::Display for Csr27 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr27 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr28(pub u32);
impl Csr28 {
    #[doc="CSR28"]
    #[inline] pub fn csr28(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR28 != 0"]
    #[inline] pub fn test_csr28(&self) -> bool {
        self.csr28() != 0
    }

    #[doc="Sets the CSR28 field."]
    #[inline] pub fn set_csr28<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr28 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr28(other)
    }
}

impl ::core::fmt::Display for Csr28 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr28 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr29(pub u32);
impl Csr29 {
    #[doc="CSR29"]
    #[inline] pub fn csr29(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR29 != 0"]
    #[inline] pub fn test_csr29(&self) -> bool {
        self.csr29() != 0
    }

    #[doc="Sets the CSR29 field."]
    #[inline] pub fn set_csr29<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr29 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr29(other)
    }
}

impl ::core::fmt::Display for Csr29 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr29 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr30(pub u32);
impl Csr30 {
    #[doc="CSR30"]
    #[inline] pub fn csr30(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR30 != 0"]
    #[inline] pub fn test_csr30(&self) -> bool {
        self.csr30() != 0
    }

    #[doc="Sets the CSR30 field."]
    #[inline] pub fn set_csr30<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr30 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr30(other)
    }
}

impl ::core::fmt::Display for Csr30 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr30 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr31(pub u32);
impl Csr31 {
    #[doc="CSR31"]
    #[inline] pub fn csr31(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR31 != 0"]
    #[inline] pub fn test_csr31(&self) -> bool {
        self.csr31() != 0
    }

    #[doc="Sets the CSR31 field."]
    #[inline] pub fn set_csr31<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr31 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr31(other)
    }
}

impl ::core::fmt::Display for Csr31 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr31 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr32(pub u32);
impl Csr32 {
    #[doc="CSR32"]
    #[inline] pub fn csr32(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR32 != 0"]
    #[inline] pub fn test_csr32(&self) -> bool {
        self.csr32() != 0
    }

    #[doc="Sets the CSR32 field."]
    #[inline] pub fn set_csr32<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr32 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr32(other)
    }
}

impl ::core::fmt::Display for Csr32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr33(pub u32);
impl Csr33 {
    #[doc="CSR33"]
    #[inline] pub fn csr33(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR33 != 0"]
    #[inline] pub fn test_csr33(&self) -> bool {
        self.csr33() != 0
    }

    #[doc="Sets the CSR33 field."]
    #[inline] pub fn set_csr33<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr33 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr33(other)
    }
}

impl ::core::fmt::Display for Csr33 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr33 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr34(pub u32);
impl Csr34 {
    #[doc="CSR34"]
    #[inline] pub fn csr34(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR34 != 0"]
    #[inline] pub fn test_csr34(&self) -> bool {
        self.csr34() != 0
    }

    #[doc="Sets the CSR34 field."]
    #[inline] pub fn set_csr34<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr34 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr34(other)
    }
}

impl ::core::fmt::Display for Csr34 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr34 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr35(pub u32);
impl Csr35 {
    #[doc="CSR35"]
    #[inline] pub fn csr35(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR35 != 0"]
    #[inline] pub fn test_csr35(&self) -> bool {
        self.csr35() != 0
    }

    #[doc="Sets the CSR35 field."]
    #[inline] pub fn set_csr35<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr35 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr35(other)
    }
}

impl ::core::fmt::Display for Csr35 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr35 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr36(pub u32);
impl Csr36 {
    #[doc="CSR36"]
    #[inline] pub fn csr36(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR36 != 0"]
    #[inline] pub fn test_csr36(&self) -> bool {
        self.csr36() != 0
    }

    #[doc="Sets the CSR36 field."]
    #[inline] pub fn set_csr36<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr36 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr36(other)
    }
}

impl ::core::fmt::Display for Csr36 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr36 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr37(pub u32);
impl Csr37 {
    #[doc="CSR37"]
    #[inline] pub fn csr37(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR37 != 0"]
    #[inline] pub fn test_csr37(&self) -> bool {
        self.csr37() != 0
    }

    #[doc="Sets the CSR37 field."]
    #[inline] pub fn set_csr37<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr37 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr37(other)
    }
}

impl ::core::fmt::Display for Csr37 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr37 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr38(pub u32);
impl Csr38 {
    #[doc="CSR38"]
    #[inline] pub fn csr38(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR38 != 0"]
    #[inline] pub fn test_csr38(&self) -> bool {
        self.csr38() != 0
    }

    #[doc="Sets the CSR38 field."]
    #[inline] pub fn set_csr38<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr38 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr38(other)
    }
}

impl ::core::fmt::Display for Csr38 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr38 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr39(pub u32);
impl Csr39 {
    #[doc="CSR39"]
    #[inline] pub fn csr39(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR39 != 0"]
    #[inline] pub fn test_csr39(&self) -> bool {
        self.csr39() != 0
    }

    #[doc="Sets the CSR39 field."]
    #[inline] pub fn set_csr39<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr39 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr39(other)
    }
}

impl ::core::fmt::Display for Csr39 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr39 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr40(pub u32);
impl Csr40 {
    #[doc="CSR40"]
    #[inline] pub fn csr40(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR40 != 0"]
    #[inline] pub fn test_csr40(&self) -> bool {
        self.csr40() != 0
    }

    #[doc="Sets the CSR40 field."]
    #[inline] pub fn set_csr40<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr40 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr40(other)
    }
}

impl ::core::fmt::Display for Csr40 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr40 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr41(pub u32);
impl Csr41 {
    #[doc="CSR41"]
    #[inline] pub fn csr41(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR41 != 0"]
    #[inline] pub fn test_csr41(&self) -> bool {
        self.csr41() != 0
    }

    #[doc="Sets the CSR41 field."]
    #[inline] pub fn set_csr41<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr41 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr41(other)
    }
}

impl ::core::fmt::Display for Csr41 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr41 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr42(pub u32);
impl Csr42 {
    #[doc="CSR42"]
    #[inline] pub fn csr42(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR42 != 0"]
    #[inline] pub fn test_csr42(&self) -> bool {
        self.csr42() != 0
    }

    #[doc="Sets the CSR42 field."]
    #[inline] pub fn set_csr42<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr42 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr42(other)
    }
}

impl ::core::fmt::Display for Csr42 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr42 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr43(pub u32);
impl Csr43 {
    #[doc="CSR43"]
    #[inline] pub fn csr43(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR43 != 0"]
    #[inline] pub fn test_csr43(&self) -> bool {
        self.csr43() != 0
    }

    #[doc="Sets the CSR43 field."]
    #[inline] pub fn set_csr43<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr43 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr43(other)
    }
}

impl ::core::fmt::Display for Csr43 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr43 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr44(pub u32);
impl Csr44 {
    #[doc="CSR44"]
    #[inline] pub fn csr44(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR44 != 0"]
    #[inline] pub fn test_csr44(&self) -> bool {
        self.csr44() != 0
    }

    #[doc="Sets the CSR44 field."]
    #[inline] pub fn set_csr44<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr44 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr44(other)
    }
}

impl ::core::fmt::Display for Csr44 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr44 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr45(pub u32);
impl Csr45 {
    #[doc="CSR45"]
    #[inline] pub fn csr45(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR45 != 0"]
    #[inline] pub fn test_csr45(&self) -> bool {
        self.csr45() != 0
    }

    #[doc="Sets the CSR45 field."]
    #[inline] pub fn set_csr45<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr45 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr45(other)
    }
}

impl ::core::fmt::Display for Csr45 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr45 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr46(pub u32);
impl Csr46 {
    #[doc="CSR46"]
    #[inline] pub fn csr46(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR46 != 0"]
    #[inline] pub fn test_csr46(&self) -> bool {
        self.csr46() != 0
    }

    #[doc="Sets the CSR46 field."]
    #[inline] pub fn set_csr46<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr46 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr46(other)
    }
}

impl ::core::fmt::Display for Csr46 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr46 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr47(pub u32);
impl Csr47 {
    #[doc="CSR47"]
    #[inline] pub fn csr47(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR47 != 0"]
    #[inline] pub fn test_csr47(&self) -> bool {
        self.csr47() != 0
    }

    #[doc="Sets the CSR47 field."]
    #[inline] pub fn set_csr47<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr47 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr47(other)
    }
}

impl ::core::fmt::Display for Csr47 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr47 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr48(pub u32);
impl Csr48 {
    #[doc="CSR48"]
    #[inline] pub fn csr48(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR48 != 0"]
    #[inline] pub fn test_csr48(&self) -> bool {
        self.csr48() != 0
    }

    #[doc="Sets the CSR48 field."]
    #[inline] pub fn set_csr48<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr48 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr48(other)
    }
}

impl ::core::fmt::Display for Csr48 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr48 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr49(pub u32);
impl Csr49 {
    #[doc="CSR49"]
    #[inline] pub fn csr49(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR49 != 0"]
    #[inline] pub fn test_csr49(&self) -> bool {
        self.csr49() != 0
    }

    #[doc="Sets the CSR49 field."]
    #[inline] pub fn set_csr49<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr49 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr49(other)
    }
}

impl ::core::fmt::Display for Csr49 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr49 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr50(pub u32);
impl Csr50 {
    #[doc="CSR50"]
    #[inline] pub fn csr50(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR50 != 0"]
    #[inline] pub fn test_csr50(&self) -> bool {
        self.csr50() != 0
    }

    #[doc="Sets the CSR50 field."]
    #[inline] pub fn set_csr50<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr50 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr50(other)
    }
}

impl ::core::fmt::Display for Csr50 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr50 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr51(pub u32);
impl Csr51 {
    #[doc="CSR51"]
    #[inline] pub fn csr51(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR51 != 0"]
    #[inline] pub fn test_csr51(&self) -> bool {
        self.csr51() != 0
    }

    #[doc="Sets the CSR51 field."]
    #[inline] pub fn set_csr51<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr51 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr51(other)
    }
}

impl ::core::fmt::Display for Csr51 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr51 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr52(pub u32);
impl Csr52 {
    #[doc="CSR52"]
    #[inline] pub fn csr52(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR52 != 0"]
    #[inline] pub fn test_csr52(&self) -> bool {
        self.csr52() != 0
    }

    #[doc="Sets the CSR52 field."]
    #[inline] pub fn set_csr52<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr52 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr52(other)
    }
}

impl ::core::fmt::Display for Csr52 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr52 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="context swap registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr53(pub u32);
impl Csr53 {
    #[doc="CSR53"]
    #[inline] pub fn csr53(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CSR53 != 0"]
    #[inline] pub fn test_csr53(&self) -> bool {
        self.csr53() != 0
    }

    #[doc="Sets the CSR53 field."]
    #[inline] pub fn set_csr53<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Csr53 {
    #[inline]
    fn from(other: u32) -> Self {
         Csr53(other)
    }
}

impl ::core::fmt::Display for Csr53 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr53 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HASH digest register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr0(pub u32);
impl HashHr0 {
    #[doc="H0"]
    #[inline] pub fn h0(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H0 != 0"]
    #[inline] pub fn test_h0(&self) -> bool {
        self.h0() != 0
    }

    #[doc="Sets the H0 field."]
    #[inline] pub fn set_h0<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr0 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr0(other)
    }
}

impl ::core::fmt::Display for HashHr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr1(pub u32);
impl HashHr1 {
    #[doc="H1"]
    #[inline] pub fn h1(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H1 != 0"]
    #[inline] pub fn test_h1(&self) -> bool {
        self.h1() != 0
    }

    #[doc="Sets the H1 field."]
    #[inline] pub fn set_h1<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr1 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr1(other)
    }
}

impl ::core::fmt::Display for HashHr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr2(pub u32);
impl HashHr2 {
    #[doc="H2"]
    #[inline] pub fn h2(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H2 != 0"]
    #[inline] pub fn test_h2(&self) -> bool {
        self.h2() != 0
    }

    #[doc="Sets the H2 field."]
    #[inline] pub fn set_h2<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr2 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr2(other)
    }
}

impl ::core::fmt::Display for HashHr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr3(pub u32);
impl HashHr3 {
    #[doc="H3"]
    #[inline] pub fn h3(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H3 != 0"]
    #[inline] pub fn test_h3(&self) -> bool {
        self.h3() != 0
    }

    #[doc="Sets the H3 field."]
    #[inline] pub fn set_h3<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr3 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr3(other)
    }
}

impl ::core::fmt::Display for HashHr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr4(pub u32);
impl HashHr4 {
    #[doc="H4"]
    #[inline] pub fn h4(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H4 != 0"]
    #[inline] pub fn test_h4(&self) -> bool {
        self.h4() != 0
    }

    #[doc="Sets the H4 field."]
    #[inline] pub fn set_h4<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr4 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr4(other)
    }
}

impl ::core::fmt::Display for HashHr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr5(pub u32);
impl HashHr5 {
    #[doc="H5"]
    #[inline] pub fn h5(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H5 != 0"]
    #[inline] pub fn test_h5(&self) -> bool {
        self.h5() != 0
    }

    #[doc="Sets the H5 field."]
    #[inline] pub fn set_h5<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr5 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr5(other)
    }
}

impl ::core::fmt::Display for HashHr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr6(pub u32);
impl HashHr6 {
    #[doc="H6"]
    #[inline] pub fn h6(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H6 != 0"]
    #[inline] pub fn test_h6(&self) -> bool {
        self.h6() != 0
    }

    #[doc="Sets the H6 field."]
    #[inline] pub fn set_h6<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr6 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr6(other)
    }
}

impl ::core::fmt::Display for HashHr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="read-only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HashHr7(pub u32);
impl HashHr7 {
    #[doc="H7"]
    #[inline] pub fn h7(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if H7 != 0"]
    #[inline] pub fn test_h7(&self) -> bool {
        self.h7() != 0
    }

    #[doc="Sets the H7 field."]
    #[inline] pub fn set_h7<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for HashHr7 {
    #[inline]
    fn from(other: u32) -> Self {
         HashHr7(other)
    }
}

impl ::core::fmt::Display for HashHr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HashHr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

