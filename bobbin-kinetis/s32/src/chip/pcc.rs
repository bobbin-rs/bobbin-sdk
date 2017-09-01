//! PCC
#[allow(unused_imports)] use bobbin_common::*;

periph!(PCC, Pcc, 0x40065000);

#[doc="PCC"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcc(pub usize);
impl Pcc {
    #[doc="Get the *mut pointer for the PCCDUMMY0 register."]
    #[inline] pub fn pccdummy0_mut(&self) -> *mut Pccdummy0 { 
        (self.0 + 0x0) as *mut Pccdummy0
    }

    #[doc="Get the *const pointer for the PCCDUMMY0 register."]
    #[inline] pub fn pccdummy0_ptr(&self) -> *const Pccdummy0 { 
           self.pccdummy0_mut()
    }

    #[doc="Read the PCCDUMMY0 register."]
    #[inline] pub fn pccdummy0(&self) -> Pccdummy0 { 
        unsafe {
            read_volatile(self.pccdummy0_ptr())
        }
    }

    #[doc="Write the PCCDUMMY0 register."]
    #[inline] pub fn set_pccdummy0<F: FnOnce(Pccdummy0) -> Pccdummy0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy0_mut(), f(Pccdummy0(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY0 register."]
    #[inline] pub fn with_pccdummy0<F: FnOnce(Pccdummy0) -> Pccdummy0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy0_mut(), f(self.pccdummy0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY1 register."]
    #[inline] pub fn pccdummy1_mut(&self) -> *mut Pccdummy1 { 
        (self.0 + 0x4) as *mut Pccdummy1
    }

    #[doc="Get the *const pointer for the PCCDUMMY1 register."]
    #[inline] pub fn pccdummy1_ptr(&self) -> *const Pccdummy1 { 
           self.pccdummy1_mut()
    }

    #[doc="Read the PCCDUMMY1 register."]
    #[inline] pub fn pccdummy1(&self) -> Pccdummy1 { 
        unsafe {
            read_volatile(self.pccdummy1_ptr())
        }
    }

    #[doc="Write the PCCDUMMY1 register."]
    #[inline] pub fn set_pccdummy1<F: FnOnce(Pccdummy1) -> Pccdummy1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy1_mut(), f(Pccdummy1(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY1 register."]
    #[inline] pub fn with_pccdummy1<F: FnOnce(Pccdummy1) -> Pccdummy1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy1_mut(), f(self.pccdummy1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY2 register."]
    #[inline] pub fn pccdummy2_mut(&self) -> *mut Pccdummy2 { 
        (self.0 + 0x8) as *mut Pccdummy2
    }

    #[doc="Get the *const pointer for the PCCDUMMY2 register."]
    #[inline] pub fn pccdummy2_ptr(&self) -> *const Pccdummy2 { 
           self.pccdummy2_mut()
    }

    #[doc="Read the PCCDUMMY2 register."]
    #[inline] pub fn pccdummy2(&self) -> Pccdummy2 { 
        unsafe {
            read_volatile(self.pccdummy2_ptr())
        }
    }

    #[doc="Write the PCCDUMMY2 register."]
    #[inline] pub fn set_pccdummy2<F: FnOnce(Pccdummy2) -> Pccdummy2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy2_mut(), f(Pccdummy2(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY2 register."]
    #[inline] pub fn with_pccdummy2<F: FnOnce(Pccdummy2) -> Pccdummy2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy2_mut(), f(self.pccdummy2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY3 register."]
    #[inline] pub fn pccdummy3_mut(&self) -> *mut Pccdummy3 { 
        (self.0 + 0xc) as *mut Pccdummy3
    }

    #[doc="Get the *const pointer for the PCCDUMMY3 register."]
    #[inline] pub fn pccdummy3_ptr(&self) -> *const Pccdummy3 { 
           self.pccdummy3_mut()
    }

    #[doc="Read the PCCDUMMY3 register."]
    #[inline] pub fn pccdummy3(&self) -> Pccdummy3 { 
        unsafe {
            read_volatile(self.pccdummy3_ptr())
        }
    }

    #[doc="Write the PCCDUMMY3 register."]
    #[inline] pub fn set_pccdummy3<F: FnOnce(Pccdummy3) -> Pccdummy3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy3_mut(), f(Pccdummy3(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY3 register."]
    #[inline] pub fn with_pccdummy3<F: FnOnce(Pccdummy3) -> Pccdummy3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy3_mut(), f(self.pccdummy3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY4 register."]
    #[inline] pub fn pccdummy4_mut(&self) -> *mut Pccdummy4 { 
        (self.0 + 0x10) as *mut Pccdummy4
    }

    #[doc="Get the *const pointer for the PCCDUMMY4 register."]
    #[inline] pub fn pccdummy4_ptr(&self) -> *const Pccdummy4 { 
           self.pccdummy4_mut()
    }

    #[doc="Read the PCCDUMMY4 register."]
    #[inline] pub fn pccdummy4(&self) -> Pccdummy4 { 
        unsafe {
            read_volatile(self.pccdummy4_ptr())
        }
    }

    #[doc="Write the PCCDUMMY4 register."]
    #[inline] pub fn set_pccdummy4<F: FnOnce(Pccdummy4) -> Pccdummy4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy4_mut(), f(Pccdummy4(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY4 register."]
    #[inline] pub fn with_pccdummy4<F: FnOnce(Pccdummy4) -> Pccdummy4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy4_mut(), f(self.pccdummy4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY5 register."]
    #[inline] pub fn pccdummy5_mut(&self) -> *mut Pccdummy5 { 
        (self.0 + 0x14) as *mut Pccdummy5
    }

    #[doc="Get the *const pointer for the PCCDUMMY5 register."]
    #[inline] pub fn pccdummy5_ptr(&self) -> *const Pccdummy5 { 
           self.pccdummy5_mut()
    }

    #[doc="Read the PCCDUMMY5 register."]
    #[inline] pub fn pccdummy5(&self) -> Pccdummy5 { 
        unsafe {
            read_volatile(self.pccdummy5_ptr())
        }
    }

    #[doc="Write the PCCDUMMY5 register."]
    #[inline] pub fn set_pccdummy5<F: FnOnce(Pccdummy5) -> Pccdummy5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy5_mut(), f(Pccdummy5(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY5 register."]
    #[inline] pub fn with_pccdummy5<F: FnOnce(Pccdummy5) -> Pccdummy5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy5_mut(), f(self.pccdummy5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY6 register."]
    #[inline] pub fn pccdummy6_mut(&self) -> *mut Pccdummy6 { 
        (self.0 + 0x18) as *mut Pccdummy6
    }

    #[doc="Get the *const pointer for the PCCDUMMY6 register."]
    #[inline] pub fn pccdummy6_ptr(&self) -> *const Pccdummy6 { 
           self.pccdummy6_mut()
    }

    #[doc="Read the PCCDUMMY6 register."]
    #[inline] pub fn pccdummy6(&self) -> Pccdummy6 { 
        unsafe {
            read_volatile(self.pccdummy6_ptr())
        }
    }

    #[doc="Write the PCCDUMMY6 register."]
    #[inline] pub fn set_pccdummy6<F: FnOnce(Pccdummy6) -> Pccdummy6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy6_mut(), f(Pccdummy6(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY6 register."]
    #[inline] pub fn with_pccdummy6<F: FnOnce(Pccdummy6) -> Pccdummy6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy6_mut(), f(self.pccdummy6()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY7 register."]
    #[inline] pub fn pccdummy7_mut(&self) -> *mut Pccdummy7 { 
        (self.0 + 0x1c) as *mut Pccdummy7
    }

    #[doc="Get the *const pointer for the PCCDUMMY7 register."]
    #[inline] pub fn pccdummy7_ptr(&self) -> *const Pccdummy7 { 
           self.pccdummy7_mut()
    }

    #[doc="Read the PCCDUMMY7 register."]
    #[inline] pub fn pccdummy7(&self) -> Pccdummy7 { 
        unsafe {
            read_volatile(self.pccdummy7_ptr())
        }
    }

    #[doc="Write the PCCDUMMY7 register."]
    #[inline] pub fn set_pccdummy7<F: FnOnce(Pccdummy7) -> Pccdummy7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy7_mut(), f(Pccdummy7(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY7 register."]
    #[inline] pub fn with_pccdummy7<F: FnOnce(Pccdummy7) -> Pccdummy7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy7_mut(), f(self.pccdummy7()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY8 register."]
    #[inline] pub fn pccdummy8_mut(&self) -> *mut Pccdummy8 { 
        (self.0 + 0x20) as *mut Pccdummy8
    }

    #[doc="Get the *const pointer for the PCCDUMMY8 register."]
    #[inline] pub fn pccdummy8_ptr(&self) -> *const Pccdummy8 { 
           self.pccdummy8_mut()
    }

    #[doc="Read the PCCDUMMY8 register."]
    #[inline] pub fn pccdummy8(&self) -> Pccdummy8 { 
        unsafe {
            read_volatile(self.pccdummy8_ptr())
        }
    }

    #[doc="Write the PCCDUMMY8 register."]
    #[inline] pub fn set_pccdummy8<F: FnOnce(Pccdummy8) -> Pccdummy8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy8_mut(), f(Pccdummy8(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY8 register."]
    #[inline] pub fn with_pccdummy8<F: FnOnce(Pccdummy8) -> Pccdummy8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy8_mut(), f(self.pccdummy8()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY9 register."]
    #[inline] pub fn pccdummy9_mut(&self) -> *mut Pccdummy9 { 
        (self.0 + 0x24) as *mut Pccdummy9
    }

    #[doc="Get the *const pointer for the PCCDUMMY9 register."]
    #[inline] pub fn pccdummy9_ptr(&self) -> *const Pccdummy9 { 
           self.pccdummy9_mut()
    }

    #[doc="Read the PCCDUMMY9 register."]
    #[inline] pub fn pccdummy9(&self) -> Pccdummy9 { 
        unsafe {
            read_volatile(self.pccdummy9_ptr())
        }
    }

    #[doc="Write the PCCDUMMY9 register."]
    #[inline] pub fn set_pccdummy9<F: FnOnce(Pccdummy9) -> Pccdummy9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy9_mut(), f(Pccdummy9(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY9 register."]
    #[inline] pub fn with_pccdummy9<F: FnOnce(Pccdummy9) -> Pccdummy9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy9_mut(), f(self.pccdummy9()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY10 register."]
    #[inline] pub fn pccdummy10_mut(&self) -> *mut Pccdummy10 { 
        (self.0 + 0x28) as *mut Pccdummy10
    }

    #[doc="Get the *const pointer for the PCCDUMMY10 register."]
    #[inline] pub fn pccdummy10_ptr(&self) -> *const Pccdummy10 { 
           self.pccdummy10_mut()
    }

    #[doc="Read the PCCDUMMY10 register."]
    #[inline] pub fn pccdummy10(&self) -> Pccdummy10 { 
        unsafe {
            read_volatile(self.pccdummy10_ptr())
        }
    }

    #[doc="Write the PCCDUMMY10 register."]
    #[inline] pub fn set_pccdummy10<F: FnOnce(Pccdummy10) -> Pccdummy10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy10_mut(), f(Pccdummy10(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY10 register."]
    #[inline] pub fn with_pccdummy10<F: FnOnce(Pccdummy10) -> Pccdummy10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy10_mut(), f(self.pccdummy10()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY11 register."]
    #[inline] pub fn pccdummy11_mut(&self) -> *mut Pccdummy11 { 
        (self.0 + 0x2c) as *mut Pccdummy11
    }

    #[doc="Get the *const pointer for the PCCDUMMY11 register."]
    #[inline] pub fn pccdummy11_ptr(&self) -> *const Pccdummy11 { 
           self.pccdummy11_mut()
    }

    #[doc="Read the PCCDUMMY11 register."]
    #[inline] pub fn pccdummy11(&self) -> Pccdummy11 { 
        unsafe {
            read_volatile(self.pccdummy11_ptr())
        }
    }

    #[doc="Write the PCCDUMMY11 register."]
    #[inline] pub fn set_pccdummy11<F: FnOnce(Pccdummy11) -> Pccdummy11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy11_mut(), f(Pccdummy11(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY11 register."]
    #[inline] pub fn with_pccdummy11<F: FnOnce(Pccdummy11) -> Pccdummy11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy11_mut(), f(self.pccdummy11()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY12 register."]
    #[inline] pub fn pccdummy12_mut(&self) -> *mut Pccdummy12 { 
        (self.0 + 0x30) as *mut Pccdummy12
    }

    #[doc="Get the *const pointer for the PCCDUMMY12 register."]
    #[inline] pub fn pccdummy12_ptr(&self) -> *const Pccdummy12 { 
           self.pccdummy12_mut()
    }

    #[doc="Read the PCCDUMMY12 register."]
    #[inline] pub fn pccdummy12(&self) -> Pccdummy12 { 
        unsafe {
            read_volatile(self.pccdummy12_ptr())
        }
    }

    #[doc="Write the PCCDUMMY12 register."]
    #[inline] pub fn set_pccdummy12<F: FnOnce(Pccdummy12) -> Pccdummy12>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy12_mut(), f(Pccdummy12(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY12 register."]
    #[inline] pub fn with_pccdummy12<F: FnOnce(Pccdummy12) -> Pccdummy12>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy12_mut(), f(self.pccdummy12()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY13 register."]
    #[inline] pub fn pccdummy13_mut(&self) -> *mut Pccdummy13 { 
        (self.0 + 0x34) as *mut Pccdummy13
    }

    #[doc="Get the *const pointer for the PCCDUMMY13 register."]
    #[inline] pub fn pccdummy13_ptr(&self) -> *const Pccdummy13 { 
           self.pccdummy13_mut()
    }

    #[doc="Read the PCCDUMMY13 register."]
    #[inline] pub fn pccdummy13(&self) -> Pccdummy13 { 
        unsafe {
            read_volatile(self.pccdummy13_ptr())
        }
    }

    #[doc="Write the PCCDUMMY13 register."]
    #[inline] pub fn set_pccdummy13<F: FnOnce(Pccdummy13) -> Pccdummy13>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy13_mut(), f(Pccdummy13(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY13 register."]
    #[inline] pub fn with_pccdummy13<F: FnOnce(Pccdummy13) -> Pccdummy13>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy13_mut(), f(self.pccdummy13()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY14 register."]
    #[inline] pub fn pccdummy14_mut(&self) -> *mut Pccdummy14 { 
        (self.0 + 0x38) as *mut Pccdummy14
    }

    #[doc="Get the *const pointer for the PCCDUMMY14 register."]
    #[inline] pub fn pccdummy14_ptr(&self) -> *const Pccdummy14 { 
           self.pccdummy14_mut()
    }

    #[doc="Read the PCCDUMMY14 register."]
    #[inline] pub fn pccdummy14(&self) -> Pccdummy14 { 
        unsafe {
            read_volatile(self.pccdummy14_ptr())
        }
    }

    #[doc="Write the PCCDUMMY14 register."]
    #[inline] pub fn set_pccdummy14<F: FnOnce(Pccdummy14) -> Pccdummy14>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy14_mut(), f(Pccdummy14(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY14 register."]
    #[inline] pub fn with_pccdummy14<F: FnOnce(Pccdummy14) -> Pccdummy14>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy14_mut(), f(self.pccdummy14()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY15 register."]
    #[inline] pub fn pccdummy15_mut(&self) -> *mut Pccdummy15 { 
        (self.0 + 0x3c) as *mut Pccdummy15
    }

    #[doc="Get the *const pointer for the PCCDUMMY15 register."]
    #[inline] pub fn pccdummy15_ptr(&self) -> *const Pccdummy15 { 
           self.pccdummy15_mut()
    }

    #[doc="Read the PCCDUMMY15 register."]
    #[inline] pub fn pccdummy15(&self) -> Pccdummy15 { 
        unsafe {
            read_volatile(self.pccdummy15_ptr())
        }
    }

    #[doc="Write the PCCDUMMY15 register."]
    #[inline] pub fn set_pccdummy15<F: FnOnce(Pccdummy15) -> Pccdummy15>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy15_mut(), f(Pccdummy15(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY15 register."]
    #[inline] pub fn with_pccdummy15<F: FnOnce(Pccdummy15) -> Pccdummy15>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy15_mut(), f(self.pccdummy15()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY16 register."]
    #[inline] pub fn pccdummy16_mut(&self) -> *mut Pccdummy16 { 
        (self.0 + 0x40) as *mut Pccdummy16
    }

    #[doc="Get the *const pointer for the PCCDUMMY16 register."]
    #[inline] pub fn pccdummy16_ptr(&self) -> *const Pccdummy16 { 
           self.pccdummy16_mut()
    }

    #[doc="Read the PCCDUMMY16 register."]
    #[inline] pub fn pccdummy16(&self) -> Pccdummy16 { 
        unsafe {
            read_volatile(self.pccdummy16_ptr())
        }
    }

    #[doc="Write the PCCDUMMY16 register."]
    #[inline] pub fn set_pccdummy16<F: FnOnce(Pccdummy16) -> Pccdummy16>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy16_mut(), f(Pccdummy16(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY16 register."]
    #[inline] pub fn with_pccdummy16<F: FnOnce(Pccdummy16) -> Pccdummy16>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy16_mut(), f(self.pccdummy16()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY17 register."]
    #[inline] pub fn pccdummy17_mut(&self) -> *mut Pccdummy17 { 
        (self.0 + 0x44) as *mut Pccdummy17
    }

    #[doc="Get the *const pointer for the PCCDUMMY17 register."]
    #[inline] pub fn pccdummy17_ptr(&self) -> *const Pccdummy17 { 
           self.pccdummy17_mut()
    }

    #[doc="Read the PCCDUMMY17 register."]
    #[inline] pub fn pccdummy17(&self) -> Pccdummy17 { 
        unsafe {
            read_volatile(self.pccdummy17_ptr())
        }
    }

    #[doc="Write the PCCDUMMY17 register."]
    #[inline] pub fn set_pccdummy17<F: FnOnce(Pccdummy17) -> Pccdummy17>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy17_mut(), f(Pccdummy17(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY17 register."]
    #[inline] pub fn with_pccdummy17<F: FnOnce(Pccdummy17) -> Pccdummy17>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy17_mut(), f(self.pccdummy17()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY18 register."]
    #[inline] pub fn pccdummy18_mut(&self) -> *mut Pccdummy18 { 
        (self.0 + 0x48) as *mut Pccdummy18
    }

    #[doc="Get the *const pointer for the PCCDUMMY18 register."]
    #[inline] pub fn pccdummy18_ptr(&self) -> *const Pccdummy18 { 
           self.pccdummy18_mut()
    }

    #[doc="Read the PCCDUMMY18 register."]
    #[inline] pub fn pccdummy18(&self) -> Pccdummy18 { 
        unsafe {
            read_volatile(self.pccdummy18_ptr())
        }
    }

    #[doc="Write the PCCDUMMY18 register."]
    #[inline] pub fn set_pccdummy18<F: FnOnce(Pccdummy18) -> Pccdummy18>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy18_mut(), f(Pccdummy18(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY18 register."]
    #[inline] pub fn with_pccdummy18<F: FnOnce(Pccdummy18) -> Pccdummy18>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy18_mut(), f(self.pccdummy18()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY19 register."]
    #[inline] pub fn pccdummy19_mut(&self) -> *mut Pccdummy19 { 
        (self.0 + 0x4c) as *mut Pccdummy19
    }

    #[doc="Get the *const pointer for the PCCDUMMY19 register."]
    #[inline] pub fn pccdummy19_ptr(&self) -> *const Pccdummy19 { 
           self.pccdummy19_mut()
    }

    #[doc="Read the PCCDUMMY19 register."]
    #[inline] pub fn pccdummy19(&self) -> Pccdummy19 { 
        unsafe {
            read_volatile(self.pccdummy19_ptr())
        }
    }

    #[doc="Write the PCCDUMMY19 register."]
    #[inline] pub fn set_pccdummy19<F: FnOnce(Pccdummy19) -> Pccdummy19>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy19_mut(), f(Pccdummy19(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY19 register."]
    #[inline] pub fn with_pccdummy19<F: FnOnce(Pccdummy19) -> Pccdummy19>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy19_mut(), f(self.pccdummy19()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY20 register."]
    #[inline] pub fn pccdummy20_mut(&self) -> *mut Pccdummy20 { 
        (self.0 + 0x50) as *mut Pccdummy20
    }

    #[doc="Get the *const pointer for the PCCDUMMY20 register."]
    #[inline] pub fn pccdummy20_ptr(&self) -> *const Pccdummy20 { 
           self.pccdummy20_mut()
    }

    #[doc="Read the PCCDUMMY20 register."]
    #[inline] pub fn pccdummy20(&self) -> Pccdummy20 { 
        unsafe {
            read_volatile(self.pccdummy20_ptr())
        }
    }

    #[doc="Write the PCCDUMMY20 register."]
    #[inline] pub fn set_pccdummy20<F: FnOnce(Pccdummy20) -> Pccdummy20>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy20_mut(), f(Pccdummy20(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY20 register."]
    #[inline] pub fn with_pccdummy20<F: FnOnce(Pccdummy20) -> Pccdummy20>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy20_mut(), f(self.pccdummy20()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY21 register."]
    #[inline] pub fn pccdummy21_mut(&self) -> *mut Pccdummy21 { 
        (self.0 + 0x54) as *mut Pccdummy21
    }

    #[doc="Get the *const pointer for the PCCDUMMY21 register."]
    #[inline] pub fn pccdummy21_ptr(&self) -> *const Pccdummy21 { 
           self.pccdummy21_mut()
    }

    #[doc="Read the PCCDUMMY21 register."]
    #[inline] pub fn pccdummy21(&self) -> Pccdummy21 { 
        unsafe {
            read_volatile(self.pccdummy21_ptr())
        }
    }

    #[doc="Write the PCCDUMMY21 register."]
    #[inline] pub fn set_pccdummy21<F: FnOnce(Pccdummy21) -> Pccdummy21>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy21_mut(), f(Pccdummy21(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY21 register."]
    #[inline] pub fn with_pccdummy21<F: FnOnce(Pccdummy21) -> Pccdummy21>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy21_mut(), f(self.pccdummy21()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY22 register."]
    #[inline] pub fn pccdummy22_mut(&self) -> *mut Pccdummy22 { 
        (self.0 + 0x58) as *mut Pccdummy22
    }

    #[doc="Get the *const pointer for the PCCDUMMY22 register."]
    #[inline] pub fn pccdummy22_ptr(&self) -> *const Pccdummy22 { 
           self.pccdummy22_mut()
    }

    #[doc="Read the PCCDUMMY22 register."]
    #[inline] pub fn pccdummy22(&self) -> Pccdummy22 { 
        unsafe {
            read_volatile(self.pccdummy22_ptr())
        }
    }

    #[doc="Write the PCCDUMMY22 register."]
    #[inline] pub fn set_pccdummy22<F: FnOnce(Pccdummy22) -> Pccdummy22>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy22_mut(), f(Pccdummy22(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY22 register."]
    #[inline] pub fn with_pccdummy22<F: FnOnce(Pccdummy22) -> Pccdummy22>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy22_mut(), f(self.pccdummy22()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY23 register."]
    #[inline] pub fn pccdummy23_mut(&self) -> *mut Pccdummy23 { 
        (self.0 + 0x5c) as *mut Pccdummy23
    }

    #[doc="Get the *const pointer for the PCCDUMMY23 register."]
    #[inline] pub fn pccdummy23_ptr(&self) -> *const Pccdummy23 { 
           self.pccdummy23_mut()
    }

    #[doc="Read the PCCDUMMY23 register."]
    #[inline] pub fn pccdummy23(&self) -> Pccdummy23 { 
        unsafe {
            read_volatile(self.pccdummy23_ptr())
        }
    }

    #[doc="Write the PCCDUMMY23 register."]
    #[inline] pub fn set_pccdummy23<F: FnOnce(Pccdummy23) -> Pccdummy23>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy23_mut(), f(Pccdummy23(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY23 register."]
    #[inline] pub fn with_pccdummy23<F: FnOnce(Pccdummy23) -> Pccdummy23>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy23_mut(), f(self.pccdummy23()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY24 register."]
    #[inline] pub fn pccdummy24_mut(&self) -> *mut Pccdummy24 { 
        (self.0 + 0x60) as *mut Pccdummy24
    }

    #[doc="Get the *const pointer for the PCCDUMMY24 register."]
    #[inline] pub fn pccdummy24_ptr(&self) -> *const Pccdummy24 { 
           self.pccdummy24_mut()
    }

    #[doc="Read the PCCDUMMY24 register."]
    #[inline] pub fn pccdummy24(&self) -> Pccdummy24 { 
        unsafe {
            read_volatile(self.pccdummy24_ptr())
        }
    }

    #[doc="Write the PCCDUMMY24 register."]
    #[inline] pub fn set_pccdummy24<F: FnOnce(Pccdummy24) -> Pccdummy24>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy24_mut(), f(Pccdummy24(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY24 register."]
    #[inline] pub fn with_pccdummy24<F: FnOnce(Pccdummy24) -> Pccdummy24>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy24_mut(), f(self.pccdummy24()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY25 register."]
    #[inline] pub fn pccdummy25_mut(&self) -> *mut Pccdummy25 { 
        (self.0 + 0x64) as *mut Pccdummy25
    }

    #[doc="Get the *const pointer for the PCCDUMMY25 register."]
    #[inline] pub fn pccdummy25_ptr(&self) -> *const Pccdummy25 { 
           self.pccdummy25_mut()
    }

    #[doc="Read the PCCDUMMY25 register."]
    #[inline] pub fn pccdummy25(&self) -> Pccdummy25 { 
        unsafe {
            read_volatile(self.pccdummy25_ptr())
        }
    }

    #[doc="Write the PCCDUMMY25 register."]
    #[inline] pub fn set_pccdummy25<F: FnOnce(Pccdummy25) -> Pccdummy25>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy25_mut(), f(Pccdummy25(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY25 register."]
    #[inline] pub fn with_pccdummy25<F: FnOnce(Pccdummy25) -> Pccdummy25>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy25_mut(), f(self.pccdummy25()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY26 register."]
    #[inline] pub fn pccdummy26_mut(&self) -> *mut Pccdummy26 { 
        (self.0 + 0x68) as *mut Pccdummy26
    }

    #[doc="Get the *const pointer for the PCCDUMMY26 register."]
    #[inline] pub fn pccdummy26_ptr(&self) -> *const Pccdummy26 { 
           self.pccdummy26_mut()
    }

    #[doc="Read the PCCDUMMY26 register."]
    #[inline] pub fn pccdummy26(&self) -> Pccdummy26 { 
        unsafe {
            read_volatile(self.pccdummy26_ptr())
        }
    }

    #[doc="Write the PCCDUMMY26 register."]
    #[inline] pub fn set_pccdummy26<F: FnOnce(Pccdummy26) -> Pccdummy26>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy26_mut(), f(Pccdummy26(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY26 register."]
    #[inline] pub fn with_pccdummy26<F: FnOnce(Pccdummy26) -> Pccdummy26>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy26_mut(), f(self.pccdummy26()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY27 register."]
    #[inline] pub fn pccdummy27_mut(&self) -> *mut Pccdummy27 { 
        (self.0 + 0x6c) as *mut Pccdummy27
    }

    #[doc="Get the *const pointer for the PCCDUMMY27 register."]
    #[inline] pub fn pccdummy27_ptr(&self) -> *const Pccdummy27 { 
           self.pccdummy27_mut()
    }

    #[doc="Read the PCCDUMMY27 register."]
    #[inline] pub fn pccdummy27(&self) -> Pccdummy27 { 
        unsafe {
            read_volatile(self.pccdummy27_ptr())
        }
    }

    #[doc="Write the PCCDUMMY27 register."]
    #[inline] pub fn set_pccdummy27<F: FnOnce(Pccdummy27) -> Pccdummy27>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy27_mut(), f(Pccdummy27(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY27 register."]
    #[inline] pub fn with_pccdummy27<F: FnOnce(Pccdummy27) -> Pccdummy27>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy27_mut(), f(self.pccdummy27()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY28 register."]
    #[inline] pub fn pccdummy28_mut(&self) -> *mut Pccdummy28 { 
        (self.0 + 0x70) as *mut Pccdummy28
    }

    #[doc="Get the *const pointer for the PCCDUMMY28 register."]
    #[inline] pub fn pccdummy28_ptr(&self) -> *const Pccdummy28 { 
           self.pccdummy28_mut()
    }

    #[doc="Read the PCCDUMMY28 register."]
    #[inline] pub fn pccdummy28(&self) -> Pccdummy28 { 
        unsafe {
            read_volatile(self.pccdummy28_ptr())
        }
    }

    #[doc="Write the PCCDUMMY28 register."]
    #[inline] pub fn set_pccdummy28<F: FnOnce(Pccdummy28) -> Pccdummy28>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy28_mut(), f(Pccdummy28(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY28 register."]
    #[inline] pub fn with_pccdummy28<F: FnOnce(Pccdummy28) -> Pccdummy28>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy28_mut(), f(self.pccdummy28()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY29 register."]
    #[inline] pub fn pccdummy29_mut(&self) -> *mut Pccdummy29 { 
        (self.0 + 0x74) as *mut Pccdummy29
    }

    #[doc="Get the *const pointer for the PCCDUMMY29 register."]
    #[inline] pub fn pccdummy29_ptr(&self) -> *const Pccdummy29 { 
           self.pccdummy29_mut()
    }

    #[doc="Read the PCCDUMMY29 register."]
    #[inline] pub fn pccdummy29(&self) -> Pccdummy29 { 
        unsafe {
            read_volatile(self.pccdummy29_ptr())
        }
    }

    #[doc="Write the PCCDUMMY29 register."]
    #[inline] pub fn set_pccdummy29<F: FnOnce(Pccdummy29) -> Pccdummy29>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy29_mut(), f(Pccdummy29(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY29 register."]
    #[inline] pub fn with_pccdummy29<F: FnOnce(Pccdummy29) -> Pccdummy29>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy29_mut(), f(self.pccdummy29()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY30 register."]
    #[inline] pub fn pccdummy30_mut(&self) -> *mut Pccdummy30 { 
        (self.0 + 0x78) as *mut Pccdummy30
    }

    #[doc="Get the *const pointer for the PCCDUMMY30 register."]
    #[inline] pub fn pccdummy30_ptr(&self) -> *const Pccdummy30 { 
           self.pccdummy30_mut()
    }

    #[doc="Read the PCCDUMMY30 register."]
    #[inline] pub fn pccdummy30(&self) -> Pccdummy30 { 
        unsafe {
            read_volatile(self.pccdummy30_ptr())
        }
    }

    #[doc="Write the PCCDUMMY30 register."]
    #[inline] pub fn set_pccdummy30<F: FnOnce(Pccdummy30) -> Pccdummy30>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy30_mut(), f(Pccdummy30(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY30 register."]
    #[inline] pub fn with_pccdummy30<F: FnOnce(Pccdummy30) -> Pccdummy30>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy30_mut(), f(self.pccdummy30()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY31 register."]
    #[inline] pub fn pccdummy31_mut(&self) -> *mut Pccdummy31 { 
        (self.0 + 0x7c) as *mut Pccdummy31
    }

    #[doc="Get the *const pointer for the PCCDUMMY31 register."]
    #[inline] pub fn pccdummy31_ptr(&self) -> *const Pccdummy31 { 
           self.pccdummy31_mut()
    }

    #[doc="Read the PCCDUMMY31 register."]
    #[inline] pub fn pccdummy31(&self) -> Pccdummy31 { 
        unsafe {
            read_volatile(self.pccdummy31_ptr())
        }
    }

    #[doc="Write the PCCDUMMY31 register."]
    #[inline] pub fn set_pccdummy31<F: FnOnce(Pccdummy31) -> Pccdummy31>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy31_mut(), f(Pccdummy31(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY31 register."]
    #[inline] pub fn with_pccdummy31<F: FnOnce(Pccdummy31) -> Pccdummy31>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy31_mut(), f(self.pccdummy31()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FTFC register."]
    #[inline] pub fn ftfc_mut(&self) -> *mut Ftfc { 
        (self.0 + 0x80) as *mut Ftfc
    }

    #[doc="Get the *const pointer for the FTFC register."]
    #[inline] pub fn ftfc_ptr(&self) -> *const Ftfc { 
           self.ftfc_mut()
    }

    #[doc="Read the FTFC register."]
    #[inline] pub fn ftfc(&self) -> Ftfc { 
        unsafe {
            read_volatile(self.ftfc_ptr())
        }
    }

    #[doc="Write the FTFC register."]
    #[inline] pub fn set_ftfc<F: FnOnce(Ftfc) -> Ftfc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftfc_mut(), f(Ftfc(0)));
        }
        self
    }

    #[doc="Modify the FTFC register."]
    #[inline] pub fn with_ftfc<F: FnOnce(Ftfc) -> Ftfc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftfc_mut(), f(self.ftfc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DMAMUX register."]
    #[inline] pub fn dmamux_mut(&self) -> *mut Dmamux { 
        (self.0 + 0x84) as *mut Dmamux
    }

    #[doc="Get the *const pointer for the DMAMUX register."]
    #[inline] pub fn dmamux_ptr(&self) -> *const Dmamux { 
           self.dmamux_mut()
    }

    #[doc="Read the DMAMUX register."]
    #[inline] pub fn dmamux(&self) -> Dmamux { 
        unsafe {
            read_volatile(self.dmamux_ptr())
        }
    }

    #[doc="Write the DMAMUX register."]
    #[inline] pub fn set_dmamux<F: FnOnce(Dmamux) -> Dmamux>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmamux_mut(), f(Dmamux(0)));
        }
        self
    }

    #[doc="Modify the DMAMUX register."]
    #[inline] pub fn with_dmamux<F: FnOnce(Dmamux) -> Dmamux>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmamux_mut(), f(self.dmamux()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY34 register."]
    #[inline] pub fn pccdummy34_mut(&self) -> *mut Pccdummy34 { 
        (self.0 + 0x88) as *mut Pccdummy34
    }

    #[doc="Get the *const pointer for the PCCDUMMY34 register."]
    #[inline] pub fn pccdummy34_ptr(&self) -> *const Pccdummy34 { 
           self.pccdummy34_mut()
    }

    #[doc="Read the PCCDUMMY34 register."]
    #[inline] pub fn pccdummy34(&self) -> Pccdummy34 { 
        unsafe {
            read_volatile(self.pccdummy34_ptr())
        }
    }

    #[doc="Write the PCCDUMMY34 register."]
    #[inline] pub fn set_pccdummy34<F: FnOnce(Pccdummy34) -> Pccdummy34>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy34_mut(), f(Pccdummy34(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY34 register."]
    #[inline] pub fn with_pccdummy34<F: FnOnce(Pccdummy34) -> Pccdummy34>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy34_mut(), f(self.pccdummy34()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY35 register."]
    #[inline] pub fn pccdummy35_mut(&self) -> *mut Pccdummy35 { 
        (self.0 + 0x8c) as *mut Pccdummy35
    }

    #[doc="Get the *const pointer for the PCCDUMMY35 register."]
    #[inline] pub fn pccdummy35_ptr(&self) -> *const Pccdummy35 { 
           self.pccdummy35_mut()
    }

    #[doc="Read the PCCDUMMY35 register."]
    #[inline] pub fn pccdummy35(&self) -> Pccdummy35 { 
        unsafe {
            read_volatile(self.pccdummy35_ptr())
        }
    }

    #[doc="Write the PCCDUMMY35 register."]
    #[inline] pub fn set_pccdummy35<F: FnOnce(Pccdummy35) -> Pccdummy35>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy35_mut(), f(Pccdummy35(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY35 register."]
    #[inline] pub fn with_pccdummy35<F: FnOnce(Pccdummy35) -> Pccdummy35>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy35_mut(), f(self.pccdummy35()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLEXCAN0 register."]
    #[inline] pub fn flexcan0_mut(&self) -> *mut Flexcan0 { 
        (self.0 + 0x90) as *mut Flexcan0
    }

    #[doc="Get the *const pointer for the FLEXCAN0 register."]
    #[inline] pub fn flexcan0_ptr(&self) -> *const Flexcan0 { 
           self.flexcan0_mut()
    }

    #[doc="Read the FLEXCAN0 register."]
    #[inline] pub fn flexcan0(&self) -> Flexcan0 { 
        unsafe {
            read_volatile(self.flexcan0_ptr())
        }
    }

    #[doc="Write the FLEXCAN0 register."]
    #[inline] pub fn set_flexcan0<F: FnOnce(Flexcan0) -> Flexcan0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flexcan0_mut(), f(Flexcan0(0)));
        }
        self
    }

    #[doc="Modify the FLEXCAN0 register."]
    #[inline] pub fn with_flexcan0<F: FnOnce(Flexcan0) -> Flexcan0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flexcan0_mut(), f(self.flexcan0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLEXCAN1 register."]
    #[inline] pub fn flexcan1_mut(&self) -> *mut Flexcan1 { 
        (self.0 + 0x94) as *mut Flexcan1
    }

    #[doc="Get the *const pointer for the FLEXCAN1 register."]
    #[inline] pub fn flexcan1_ptr(&self) -> *const Flexcan1 { 
           self.flexcan1_mut()
    }

    #[doc="Read the FLEXCAN1 register."]
    #[inline] pub fn flexcan1(&self) -> Flexcan1 { 
        unsafe {
            read_volatile(self.flexcan1_ptr())
        }
    }

    #[doc="Write the FLEXCAN1 register."]
    #[inline] pub fn set_flexcan1<F: FnOnce(Flexcan1) -> Flexcan1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flexcan1_mut(), f(Flexcan1(0)));
        }
        self
    }

    #[doc="Modify the FLEXCAN1 register."]
    #[inline] pub fn with_flexcan1<F: FnOnce(Flexcan1) -> Flexcan1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flexcan1_mut(), f(self.flexcan1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FTM3 register."]
    #[inline] pub fn ftm3_mut(&self) -> *mut Ftm3 { 
        (self.0 + 0x98) as *mut Ftm3
    }

    #[doc="Get the *const pointer for the FTM3 register."]
    #[inline] pub fn ftm3_ptr(&self) -> *const Ftm3 { 
           self.ftm3_mut()
    }

    #[doc="Read the FTM3 register."]
    #[inline] pub fn ftm3(&self) -> Ftm3 { 
        unsafe {
            read_volatile(self.ftm3_ptr())
        }
    }

    #[doc="Write the FTM3 register."]
    #[inline] pub fn set_ftm3<F: FnOnce(Ftm3) -> Ftm3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftm3_mut(), f(Ftm3(0)));
        }
        self
    }

    #[doc="Modify the FTM3 register."]
    #[inline] pub fn with_ftm3<F: FnOnce(Ftm3) -> Ftm3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftm3_mut(), f(self.ftm3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADC1 register."]
    #[inline] pub fn adc1_mut(&self) -> *mut Adc1 { 
        (self.0 + 0x9c) as *mut Adc1
    }

    #[doc="Get the *const pointer for the ADC1 register."]
    #[inline] pub fn adc1_ptr(&self) -> *const Adc1 { 
           self.adc1_mut()
    }

    #[doc="Read the ADC1 register."]
    #[inline] pub fn adc1(&self) -> Adc1 { 
        unsafe {
            read_volatile(self.adc1_ptr())
        }
    }

    #[doc="Write the ADC1 register."]
    #[inline] pub fn set_adc1<F: FnOnce(Adc1) -> Adc1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adc1_mut(), f(Adc1(0)));
        }
        self
    }

    #[doc="Modify the ADC1 register."]
    #[inline] pub fn with_adc1<F: FnOnce(Adc1) -> Adc1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adc1_mut(), f(self.adc1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY40 register."]
    #[inline] pub fn pccdummy40_mut(&self) -> *mut Pccdummy40 { 
        (self.0 + 0xa0) as *mut Pccdummy40
    }

    #[doc="Get the *const pointer for the PCCDUMMY40 register."]
    #[inline] pub fn pccdummy40_ptr(&self) -> *const Pccdummy40 { 
           self.pccdummy40_mut()
    }

    #[doc="Read the PCCDUMMY40 register."]
    #[inline] pub fn pccdummy40(&self) -> Pccdummy40 { 
        unsafe {
            read_volatile(self.pccdummy40_ptr())
        }
    }

    #[doc="Write the PCCDUMMY40 register."]
    #[inline] pub fn set_pccdummy40<F: FnOnce(Pccdummy40) -> Pccdummy40>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy40_mut(), f(Pccdummy40(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY40 register."]
    #[inline] pub fn with_pccdummy40<F: FnOnce(Pccdummy40) -> Pccdummy40>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy40_mut(), f(self.pccdummy40()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY41 register."]
    #[inline] pub fn pccdummy41_mut(&self) -> *mut Pccdummy41 { 
        (self.0 + 0xa4) as *mut Pccdummy41
    }

    #[doc="Get the *const pointer for the PCCDUMMY41 register."]
    #[inline] pub fn pccdummy41_ptr(&self) -> *const Pccdummy41 { 
           self.pccdummy41_mut()
    }

    #[doc="Read the PCCDUMMY41 register."]
    #[inline] pub fn pccdummy41(&self) -> Pccdummy41 { 
        unsafe {
            read_volatile(self.pccdummy41_ptr())
        }
    }

    #[doc="Write the PCCDUMMY41 register."]
    #[inline] pub fn set_pccdummy41<F: FnOnce(Pccdummy41) -> Pccdummy41>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy41_mut(), f(Pccdummy41(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY41 register."]
    #[inline] pub fn with_pccdummy41<F: FnOnce(Pccdummy41) -> Pccdummy41>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy41_mut(), f(self.pccdummy41()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY42 register."]
    #[inline] pub fn pccdummy42_mut(&self) -> *mut Pccdummy42 { 
        (self.0 + 0xa8) as *mut Pccdummy42
    }

    #[doc="Get the *const pointer for the PCCDUMMY42 register."]
    #[inline] pub fn pccdummy42_ptr(&self) -> *const Pccdummy42 { 
           self.pccdummy42_mut()
    }

    #[doc="Read the PCCDUMMY42 register."]
    #[inline] pub fn pccdummy42(&self) -> Pccdummy42 { 
        unsafe {
            read_volatile(self.pccdummy42_ptr())
        }
    }

    #[doc="Write the PCCDUMMY42 register."]
    #[inline] pub fn set_pccdummy42<F: FnOnce(Pccdummy42) -> Pccdummy42>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy42_mut(), f(Pccdummy42(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY42 register."]
    #[inline] pub fn with_pccdummy42<F: FnOnce(Pccdummy42) -> Pccdummy42>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy42_mut(), f(self.pccdummy42()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLEXCAN2 register."]
    #[inline] pub fn flexcan2_mut(&self) -> *mut Flexcan2 { 
        (self.0 + 0xac) as *mut Flexcan2
    }

    #[doc="Get the *const pointer for the FLEXCAN2 register."]
    #[inline] pub fn flexcan2_ptr(&self) -> *const Flexcan2 { 
           self.flexcan2_mut()
    }

    #[doc="Read the FLEXCAN2 register."]
    #[inline] pub fn flexcan2(&self) -> Flexcan2 { 
        unsafe {
            read_volatile(self.flexcan2_ptr())
        }
    }

    #[doc="Write the FLEXCAN2 register."]
    #[inline] pub fn set_flexcan2<F: FnOnce(Flexcan2) -> Flexcan2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flexcan2_mut(), f(Flexcan2(0)));
        }
        self
    }

    #[doc="Modify the FLEXCAN2 register."]
    #[inline] pub fn with_flexcan2<F: FnOnce(Flexcan2) -> Flexcan2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flexcan2_mut(), f(self.flexcan2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LPSPI0 register."]
    #[inline] pub fn lpspi0_mut(&self) -> *mut Lpspi0 { 
        (self.0 + 0xb0) as *mut Lpspi0
    }

    #[doc="Get the *const pointer for the LPSPI0 register."]
    #[inline] pub fn lpspi0_ptr(&self) -> *const Lpspi0 { 
           self.lpspi0_mut()
    }

    #[doc="Read the LPSPI0 register."]
    #[inline] pub fn lpspi0(&self) -> Lpspi0 { 
        unsafe {
            read_volatile(self.lpspi0_ptr())
        }
    }

    #[doc="Write the LPSPI0 register."]
    #[inline] pub fn set_lpspi0<F: FnOnce(Lpspi0) -> Lpspi0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpspi0_mut(), f(Lpspi0(0)));
        }
        self
    }

    #[doc="Modify the LPSPI0 register."]
    #[inline] pub fn with_lpspi0<F: FnOnce(Lpspi0) -> Lpspi0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpspi0_mut(), f(self.lpspi0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LPSPI1 register."]
    #[inline] pub fn lpspi1_mut(&self) -> *mut Lpspi1 { 
        (self.0 + 0xb4) as *mut Lpspi1
    }

    #[doc="Get the *const pointer for the LPSPI1 register."]
    #[inline] pub fn lpspi1_ptr(&self) -> *const Lpspi1 { 
           self.lpspi1_mut()
    }

    #[doc="Read the LPSPI1 register."]
    #[inline] pub fn lpspi1(&self) -> Lpspi1 { 
        unsafe {
            read_volatile(self.lpspi1_ptr())
        }
    }

    #[doc="Write the LPSPI1 register."]
    #[inline] pub fn set_lpspi1<F: FnOnce(Lpspi1) -> Lpspi1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpspi1_mut(), f(Lpspi1(0)));
        }
        self
    }

    #[doc="Modify the LPSPI1 register."]
    #[inline] pub fn with_lpspi1<F: FnOnce(Lpspi1) -> Lpspi1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpspi1_mut(), f(self.lpspi1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LPSPI2 register."]
    #[inline] pub fn lpspi2_mut(&self) -> *mut Lpspi2 { 
        (self.0 + 0xb8) as *mut Lpspi2
    }

    #[doc="Get the *const pointer for the LPSPI2 register."]
    #[inline] pub fn lpspi2_ptr(&self) -> *const Lpspi2 { 
           self.lpspi2_mut()
    }

    #[doc="Read the LPSPI2 register."]
    #[inline] pub fn lpspi2(&self) -> Lpspi2 { 
        unsafe {
            read_volatile(self.lpspi2_ptr())
        }
    }

    #[doc="Write the LPSPI2 register."]
    #[inline] pub fn set_lpspi2<F: FnOnce(Lpspi2) -> Lpspi2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpspi2_mut(), f(Lpspi2(0)));
        }
        self
    }

    #[doc="Modify the LPSPI2 register."]
    #[inline] pub fn with_lpspi2<F: FnOnce(Lpspi2) -> Lpspi2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpspi2_mut(), f(self.lpspi2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY47 register."]
    #[inline] pub fn pccdummy47_mut(&self) -> *mut Pccdummy47 { 
        (self.0 + 0xbc) as *mut Pccdummy47
    }

    #[doc="Get the *const pointer for the PCCDUMMY47 register."]
    #[inline] pub fn pccdummy47_ptr(&self) -> *const Pccdummy47 { 
           self.pccdummy47_mut()
    }

    #[doc="Read the PCCDUMMY47 register."]
    #[inline] pub fn pccdummy47(&self) -> Pccdummy47 { 
        unsafe {
            read_volatile(self.pccdummy47_ptr())
        }
    }

    #[doc="Write the PCCDUMMY47 register."]
    #[inline] pub fn set_pccdummy47<F: FnOnce(Pccdummy47) -> Pccdummy47>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy47_mut(), f(Pccdummy47(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY47 register."]
    #[inline] pub fn with_pccdummy47<F: FnOnce(Pccdummy47) -> Pccdummy47>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy47_mut(), f(self.pccdummy47()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY48 register."]
    #[inline] pub fn pccdummy48_mut(&self) -> *mut Pccdummy48 { 
        (self.0 + 0xc0) as *mut Pccdummy48
    }

    #[doc="Get the *const pointer for the PCCDUMMY48 register."]
    #[inline] pub fn pccdummy48_ptr(&self) -> *const Pccdummy48 { 
           self.pccdummy48_mut()
    }

    #[doc="Read the PCCDUMMY48 register."]
    #[inline] pub fn pccdummy48(&self) -> Pccdummy48 { 
        unsafe {
            read_volatile(self.pccdummy48_ptr())
        }
    }

    #[doc="Write the PCCDUMMY48 register."]
    #[inline] pub fn set_pccdummy48<F: FnOnce(Pccdummy48) -> Pccdummy48>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy48_mut(), f(Pccdummy48(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY48 register."]
    #[inline] pub fn with_pccdummy48<F: FnOnce(Pccdummy48) -> Pccdummy48>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy48_mut(), f(self.pccdummy48()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PDB1 register."]
    #[inline] pub fn pdb1_mut(&self) -> *mut Pdb1 { 
        (self.0 + 0xc4) as *mut Pdb1
    }

    #[doc="Get the *const pointer for the PDB1 register."]
    #[inline] pub fn pdb1_ptr(&self) -> *const Pdb1 { 
           self.pdb1_mut()
    }

    #[doc="Read the PDB1 register."]
    #[inline] pub fn pdb1(&self) -> Pdb1 { 
        unsafe {
            read_volatile(self.pdb1_ptr())
        }
    }

    #[doc="Write the PDB1 register."]
    #[inline] pub fn set_pdb1<F: FnOnce(Pdb1) -> Pdb1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdb1_mut(), f(Pdb1(0)));
        }
        self
    }

    #[doc="Modify the PDB1 register."]
    #[inline] pub fn with_pdb1<F: FnOnce(Pdb1) -> Pdb1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdb1_mut(), f(self.pdb1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CRC register."]
    #[inline] pub fn crc_mut(&self) -> *mut Crc { 
        (self.0 + 0xc8) as *mut Crc
    }

    #[doc="Get the *const pointer for the CRC register."]
    #[inline] pub fn crc_ptr(&self) -> *const Crc { 
           self.crc_mut()
    }

    #[doc="Read the CRC register."]
    #[inline] pub fn crc(&self) -> Crc { 
        unsafe {
            read_volatile(self.crc_ptr())
        }
    }

    #[doc="Write the CRC register."]
    #[inline] pub fn set_crc<F: FnOnce(Crc) -> Crc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crc_mut(), f(Crc(0)));
        }
        self
    }

    #[doc="Modify the CRC register."]
    #[inline] pub fn with_crc<F: FnOnce(Crc) -> Crc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.crc_mut(), f(self.crc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY51 register."]
    #[inline] pub fn pccdummy51_mut(&self) -> *mut Pccdummy51 { 
        (self.0 + 0xcc) as *mut Pccdummy51
    }

    #[doc="Get the *const pointer for the PCCDUMMY51 register."]
    #[inline] pub fn pccdummy51_ptr(&self) -> *const Pccdummy51 { 
           self.pccdummy51_mut()
    }

    #[doc="Read the PCCDUMMY51 register."]
    #[inline] pub fn pccdummy51(&self) -> Pccdummy51 { 
        unsafe {
            read_volatile(self.pccdummy51_ptr())
        }
    }

    #[doc="Write the PCCDUMMY51 register."]
    #[inline] pub fn set_pccdummy51<F: FnOnce(Pccdummy51) -> Pccdummy51>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy51_mut(), f(Pccdummy51(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY51 register."]
    #[inline] pub fn with_pccdummy51<F: FnOnce(Pccdummy51) -> Pccdummy51>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy51_mut(), f(self.pccdummy51()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY52 register."]
    #[inline] pub fn pccdummy52_mut(&self) -> *mut Pccdummy52 { 
        (self.0 + 0xd0) as *mut Pccdummy52
    }

    #[doc="Get the *const pointer for the PCCDUMMY52 register."]
    #[inline] pub fn pccdummy52_ptr(&self) -> *const Pccdummy52 { 
           self.pccdummy52_mut()
    }

    #[doc="Read the PCCDUMMY52 register."]
    #[inline] pub fn pccdummy52(&self) -> Pccdummy52 { 
        unsafe {
            read_volatile(self.pccdummy52_ptr())
        }
    }

    #[doc="Write the PCCDUMMY52 register."]
    #[inline] pub fn set_pccdummy52<F: FnOnce(Pccdummy52) -> Pccdummy52>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy52_mut(), f(Pccdummy52(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY52 register."]
    #[inline] pub fn with_pccdummy52<F: FnOnce(Pccdummy52) -> Pccdummy52>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy52_mut(), f(self.pccdummy52()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY53 register."]
    #[inline] pub fn pccdummy53_mut(&self) -> *mut Pccdummy53 { 
        (self.0 + 0xd4) as *mut Pccdummy53
    }

    #[doc="Get the *const pointer for the PCCDUMMY53 register."]
    #[inline] pub fn pccdummy53_ptr(&self) -> *const Pccdummy53 { 
           self.pccdummy53_mut()
    }

    #[doc="Read the PCCDUMMY53 register."]
    #[inline] pub fn pccdummy53(&self) -> Pccdummy53 { 
        unsafe {
            read_volatile(self.pccdummy53_ptr())
        }
    }

    #[doc="Write the PCCDUMMY53 register."]
    #[inline] pub fn set_pccdummy53<F: FnOnce(Pccdummy53) -> Pccdummy53>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy53_mut(), f(Pccdummy53(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY53 register."]
    #[inline] pub fn with_pccdummy53<F: FnOnce(Pccdummy53) -> Pccdummy53>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy53_mut(), f(self.pccdummy53()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PDB0 register."]
    #[inline] pub fn pdb0_mut(&self) -> *mut Pdb0 { 
        (self.0 + 0xd8) as *mut Pdb0
    }

    #[doc="Get the *const pointer for the PDB0 register."]
    #[inline] pub fn pdb0_ptr(&self) -> *const Pdb0 { 
           self.pdb0_mut()
    }

    #[doc="Read the PDB0 register."]
    #[inline] pub fn pdb0(&self) -> Pdb0 { 
        unsafe {
            read_volatile(self.pdb0_ptr())
        }
    }

    #[doc="Write the PDB0 register."]
    #[inline] pub fn set_pdb0<F: FnOnce(Pdb0) -> Pdb0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdb0_mut(), f(Pdb0(0)));
        }
        self
    }

    #[doc="Modify the PDB0 register."]
    #[inline] pub fn with_pdb0<F: FnOnce(Pdb0) -> Pdb0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pdb0_mut(), f(self.pdb0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LPIT register."]
    #[inline] pub fn lpit_mut(&self) -> *mut Lpit { 
        (self.0 + 0xdc) as *mut Lpit
    }

    #[doc="Get the *const pointer for the LPIT register."]
    #[inline] pub fn lpit_ptr(&self) -> *const Lpit { 
           self.lpit_mut()
    }

    #[doc="Read the LPIT register."]
    #[inline] pub fn lpit(&self) -> Lpit { 
        unsafe {
            read_volatile(self.lpit_ptr())
        }
    }

    #[doc="Write the LPIT register."]
    #[inline] pub fn set_lpit<F: FnOnce(Lpit) -> Lpit>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpit_mut(), f(Lpit(0)));
        }
        self
    }

    #[doc="Modify the LPIT register."]
    #[inline] pub fn with_lpit<F: FnOnce(Lpit) -> Lpit>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpit_mut(), f(self.lpit()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FTM0 register."]
    #[inline] pub fn ftm0_mut(&self) -> *mut Ftm0 { 
        (self.0 + 0xe0) as *mut Ftm0
    }

    #[doc="Get the *const pointer for the FTM0 register."]
    #[inline] pub fn ftm0_ptr(&self) -> *const Ftm0 { 
           self.ftm0_mut()
    }

    #[doc="Read the FTM0 register."]
    #[inline] pub fn ftm0(&self) -> Ftm0 { 
        unsafe {
            read_volatile(self.ftm0_ptr())
        }
    }

    #[doc="Write the FTM0 register."]
    #[inline] pub fn set_ftm0<F: FnOnce(Ftm0) -> Ftm0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftm0_mut(), f(Ftm0(0)));
        }
        self
    }

    #[doc="Modify the FTM0 register."]
    #[inline] pub fn with_ftm0<F: FnOnce(Ftm0) -> Ftm0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftm0_mut(), f(self.ftm0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FTM1 register."]
    #[inline] pub fn ftm1_mut(&self) -> *mut Ftm1 { 
        (self.0 + 0xe4) as *mut Ftm1
    }

    #[doc="Get the *const pointer for the FTM1 register."]
    #[inline] pub fn ftm1_ptr(&self) -> *const Ftm1 { 
           self.ftm1_mut()
    }

    #[doc="Read the FTM1 register."]
    #[inline] pub fn ftm1(&self) -> Ftm1 { 
        unsafe {
            read_volatile(self.ftm1_ptr())
        }
    }

    #[doc="Write the FTM1 register."]
    #[inline] pub fn set_ftm1<F: FnOnce(Ftm1) -> Ftm1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftm1_mut(), f(Ftm1(0)));
        }
        self
    }

    #[doc="Modify the FTM1 register."]
    #[inline] pub fn with_ftm1<F: FnOnce(Ftm1) -> Ftm1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftm1_mut(), f(self.ftm1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FTM2 register."]
    #[inline] pub fn ftm2_mut(&self) -> *mut Ftm2 { 
        (self.0 + 0xe8) as *mut Ftm2
    }

    #[doc="Get the *const pointer for the FTM2 register."]
    #[inline] pub fn ftm2_ptr(&self) -> *const Ftm2 { 
           self.ftm2_mut()
    }

    #[doc="Read the FTM2 register."]
    #[inline] pub fn ftm2(&self) -> Ftm2 { 
        unsafe {
            read_volatile(self.ftm2_ptr())
        }
    }

    #[doc="Write the FTM2 register."]
    #[inline] pub fn set_ftm2<F: FnOnce(Ftm2) -> Ftm2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftm2_mut(), f(Ftm2(0)));
        }
        self
    }

    #[doc="Modify the FTM2 register."]
    #[inline] pub fn with_ftm2<F: FnOnce(Ftm2) -> Ftm2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftm2_mut(), f(self.ftm2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADC0 register."]
    #[inline] pub fn adc0_mut(&self) -> *mut Adc0 { 
        (self.0 + 0xec) as *mut Adc0
    }

    #[doc="Get the *const pointer for the ADC0 register."]
    #[inline] pub fn adc0_ptr(&self) -> *const Adc0 { 
           self.adc0_mut()
    }

    #[doc="Read the ADC0 register."]
    #[inline] pub fn adc0(&self) -> Adc0 { 
        unsafe {
            read_volatile(self.adc0_ptr())
        }
    }

    #[doc="Write the ADC0 register."]
    #[inline] pub fn set_adc0<F: FnOnce(Adc0) -> Adc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adc0_mut(), f(Adc0(0)));
        }
        self
    }

    #[doc="Modify the ADC0 register."]
    #[inline] pub fn with_adc0<F: FnOnce(Adc0) -> Adc0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adc0_mut(), f(self.adc0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY60 register."]
    #[inline] pub fn pccdummy60_mut(&self) -> *mut Pccdummy60 { 
        (self.0 + 0xf0) as *mut Pccdummy60
    }

    #[doc="Get the *const pointer for the PCCDUMMY60 register."]
    #[inline] pub fn pccdummy60_ptr(&self) -> *const Pccdummy60 { 
           self.pccdummy60_mut()
    }

    #[doc="Read the PCCDUMMY60 register."]
    #[inline] pub fn pccdummy60(&self) -> Pccdummy60 { 
        unsafe {
            read_volatile(self.pccdummy60_ptr())
        }
    }

    #[doc="Write the PCCDUMMY60 register."]
    #[inline] pub fn set_pccdummy60<F: FnOnce(Pccdummy60) -> Pccdummy60>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy60_mut(), f(Pccdummy60(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY60 register."]
    #[inline] pub fn with_pccdummy60<F: FnOnce(Pccdummy60) -> Pccdummy60>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy60_mut(), f(self.pccdummy60()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RTC register."]
    #[inline] pub fn rtc_mut(&self) -> *mut Rtc { 
        (self.0 + 0xf4) as *mut Rtc
    }

    #[doc="Get the *const pointer for the RTC register."]
    #[inline] pub fn rtc_ptr(&self) -> *const Rtc { 
           self.rtc_mut()
    }

    #[doc="Read the RTC register."]
    #[inline] pub fn rtc(&self) -> Rtc { 
        unsafe {
            read_volatile(self.rtc_ptr())
        }
    }

    #[doc="Write the RTC register."]
    #[inline] pub fn set_rtc<F: FnOnce(Rtc) -> Rtc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rtc_mut(), f(Rtc(0)));
        }
        self
    }

    #[doc="Modify the RTC register."]
    #[inline] pub fn with_rtc<F: FnOnce(Rtc) -> Rtc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rtc_mut(), f(self.rtc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY62 register."]
    #[inline] pub fn pccdummy62_mut(&self) -> *mut Pccdummy62 { 
        (self.0 + 0xf8) as *mut Pccdummy62
    }

    #[doc="Get the *const pointer for the PCCDUMMY62 register."]
    #[inline] pub fn pccdummy62_ptr(&self) -> *const Pccdummy62 { 
           self.pccdummy62_mut()
    }

    #[doc="Read the PCCDUMMY62 register."]
    #[inline] pub fn pccdummy62(&self) -> Pccdummy62 { 
        unsafe {
            read_volatile(self.pccdummy62_ptr())
        }
    }

    #[doc="Write the PCCDUMMY62 register."]
    #[inline] pub fn set_pccdummy62<F: FnOnce(Pccdummy62) -> Pccdummy62>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy62_mut(), f(Pccdummy62(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY62 register."]
    #[inline] pub fn with_pccdummy62<F: FnOnce(Pccdummy62) -> Pccdummy62>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy62_mut(), f(self.pccdummy62()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY63 register."]
    #[inline] pub fn pccdummy63_mut(&self) -> *mut Pccdummy63 { 
        (self.0 + 0xfc) as *mut Pccdummy63
    }

    #[doc="Get the *const pointer for the PCCDUMMY63 register."]
    #[inline] pub fn pccdummy63_ptr(&self) -> *const Pccdummy63 { 
           self.pccdummy63_mut()
    }

    #[doc="Read the PCCDUMMY63 register."]
    #[inline] pub fn pccdummy63(&self) -> Pccdummy63 { 
        unsafe {
            read_volatile(self.pccdummy63_ptr())
        }
    }

    #[doc="Write the PCCDUMMY63 register."]
    #[inline] pub fn set_pccdummy63<F: FnOnce(Pccdummy63) -> Pccdummy63>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy63_mut(), f(Pccdummy63(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY63 register."]
    #[inline] pub fn with_pccdummy63<F: FnOnce(Pccdummy63) -> Pccdummy63>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy63_mut(), f(self.pccdummy63()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LPTMR0 register."]
    #[inline] pub fn lptmr0_mut(&self) -> *mut Lptmr0 { 
        (self.0 + 0x100) as *mut Lptmr0
    }

    #[doc="Get the *const pointer for the LPTMR0 register."]
    #[inline] pub fn lptmr0_ptr(&self) -> *const Lptmr0 { 
           self.lptmr0_mut()
    }

    #[doc="Read the LPTMR0 register."]
    #[inline] pub fn lptmr0(&self) -> Lptmr0 { 
        unsafe {
            read_volatile(self.lptmr0_ptr())
        }
    }

    #[doc="Write the LPTMR0 register."]
    #[inline] pub fn set_lptmr0<F: FnOnce(Lptmr0) -> Lptmr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lptmr0_mut(), f(Lptmr0(0)));
        }
        self
    }

    #[doc="Modify the LPTMR0 register."]
    #[inline] pub fn with_lptmr0<F: FnOnce(Lptmr0) -> Lptmr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lptmr0_mut(), f(self.lptmr0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY65 register."]
    #[inline] pub fn pccdummy65_mut(&self) -> *mut Pccdummy65 { 
        (self.0 + 0x104) as *mut Pccdummy65
    }

    #[doc="Get the *const pointer for the PCCDUMMY65 register."]
    #[inline] pub fn pccdummy65_ptr(&self) -> *const Pccdummy65 { 
           self.pccdummy65_mut()
    }

    #[doc="Read the PCCDUMMY65 register."]
    #[inline] pub fn pccdummy65(&self) -> Pccdummy65 { 
        unsafe {
            read_volatile(self.pccdummy65_ptr())
        }
    }

    #[doc="Write the PCCDUMMY65 register."]
    #[inline] pub fn set_pccdummy65<F: FnOnce(Pccdummy65) -> Pccdummy65>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy65_mut(), f(Pccdummy65(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY65 register."]
    #[inline] pub fn with_pccdummy65<F: FnOnce(Pccdummy65) -> Pccdummy65>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy65_mut(), f(self.pccdummy65()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY66 register."]
    #[inline] pub fn pccdummy66_mut(&self) -> *mut Pccdummy66 { 
        (self.0 + 0x108) as *mut Pccdummy66
    }

    #[doc="Get the *const pointer for the PCCDUMMY66 register."]
    #[inline] pub fn pccdummy66_ptr(&self) -> *const Pccdummy66 { 
           self.pccdummy66_mut()
    }

    #[doc="Read the PCCDUMMY66 register."]
    #[inline] pub fn pccdummy66(&self) -> Pccdummy66 { 
        unsafe {
            read_volatile(self.pccdummy66_ptr())
        }
    }

    #[doc="Write the PCCDUMMY66 register."]
    #[inline] pub fn set_pccdummy66<F: FnOnce(Pccdummy66) -> Pccdummy66>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy66_mut(), f(Pccdummy66(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY66 register."]
    #[inline] pub fn with_pccdummy66<F: FnOnce(Pccdummy66) -> Pccdummy66>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy66_mut(), f(self.pccdummy66()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY67 register."]
    #[inline] pub fn pccdummy67_mut(&self) -> *mut Pccdummy67 { 
        (self.0 + 0x10c) as *mut Pccdummy67
    }

    #[doc="Get the *const pointer for the PCCDUMMY67 register."]
    #[inline] pub fn pccdummy67_ptr(&self) -> *const Pccdummy67 { 
           self.pccdummy67_mut()
    }

    #[doc="Read the PCCDUMMY67 register."]
    #[inline] pub fn pccdummy67(&self) -> Pccdummy67 { 
        unsafe {
            read_volatile(self.pccdummy67_ptr())
        }
    }

    #[doc="Write the PCCDUMMY67 register."]
    #[inline] pub fn set_pccdummy67<F: FnOnce(Pccdummy67) -> Pccdummy67>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy67_mut(), f(Pccdummy67(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY67 register."]
    #[inline] pub fn with_pccdummy67<F: FnOnce(Pccdummy67) -> Pccdummy67>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy67_mut(), f(self.pccdummy67()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY68 register."]
    #[inline] pub fn pccdummy68_mut(&self) -> *mut Pccdummy68 { 
        (self.0 + 0x110) as *mut Pccdummy68
    }

    #[doc="Get the *const pointer for the PCCDUMMY68 register."]
    #[inline] pub fn pccdummy68_ptr(&self) -> *const Pccdummy68 { 
           self.pccdummy68_mut()
    }

    #[doc="Read the PCCDUMMY68 register."]
    #[inline] pub fn pccdummy68(&self) -> Pccdummy68 { 
        unsafe {
            read_volatile(self.pccdummy68_ptr())
        }
    }

    #[doc="Write the PCCDUMMY68 register."]
    #[inline] pub fn set_pccdummy68<F: FnOnce(Pccdummy68) -> Pccdummy68>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy68_mut(), f(Pccdummy68(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY68 register."]
    #[inline] pub fn with_pccdummy68<F: FnOnce(Pccdummy68) -> Pccdummy68>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy68_mut(), f(self.pccdummy68()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY69 register."]
    #[inline] pub fn pccdummy69_mut(&self) -> *mut Pccdummy69 { 
        (self.0 + 0x114) as *mut Pccdummy69
    }

    #[doc="Get the *const pointer for the PCCDUMMY69 register."]
    #[inline] pub fn pccdummy69_ptr(&self) -> *const Pccdummy69 { 
           self.pccdummy69_mut()
    }

    #[doc="Read the PCCDUMMY69 register."]
    #[inline] pub fn pccdummy69(&self) -> Pccdummy69 { 
        unsafe {
            read_volatile(self.pccdummy69_ptr())
        }
    }

    #[doc="Write the PCCDUMMY69 register."]
    #[inline] pub fn set_pccdummy69<F: FnOnce(Pccdummy69) -> Pccdummy69>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy69_mut(), f(Pccdummy69(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY69 register."]
    #[inline] pub fn with_pccdummy69<F: FnOnce(Pccdummy69) -> Pccdummy69>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy69_mut(), f(self.pccdummy69()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY70 register."]
    #[inline] pub fn pccdummy70_mut(&self) -> *mut Pccdummy70 { 
        (self.0 + 0x118) as *mut Pccdummy70
    }

    #[doc="Get the *const pointer for the PCCDUMMY70 register."]
    #[inline] pub fn pccdummy70_ptr(&self) -> *const Pccdummy70 { 
           self.pccdummy70_mut()
    }

    #[doc="Read the PCCDUMMY70 register."]
    #[inline] pub fn pccdummy70(&self) -> Pccdummy70 { 
        unsafe {
            read_volatile(self.pccdummy70_ptr())
        }
    }

    #[doc="Write the PCCDUMMY70 register."]
    #[inline] pub fn set_pccdummy70<F: FnOnce(Pccdummy70) -> Pccdummy70>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy70_mut(), f(Pccdummy70(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY70 register."]
    #[inline] pub fn with_pccdummy70<F: FnOnce(Pccdummy70) -> Pccdummy70>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy70_mut(), f(self.pccdummy70()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY71 register."]
    #[inline] pub fn pccdummy71_mut(&self) -> *mut Pccdummy71 { 
        (self.0 + 0x11c) as *mut Pccdummy71
    }

    #[doc="Get the *const pointer for the PCCDUMMY71 register."]
    #[inline] pub fn pccdummy71_ptr(&self) -> *const Pccdummy71 { 
           self.pccdummy71_mut()
    }

    #[doc="Read the PCCDUMMY71 register."]
    #[inline] pub fn pccdummy71(&self) -> Pccdummy71 { 
        unsafe {
            read_volatile(self.pccdummy71_ptr())
        }
    }

    #[doc="Write the PCCDUMMY71 register."]
    #[inline] pub fn set_pccdummy71<F: FnOnce(Pccdummy71) -> Pccdummy71>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy71_mut(), f(Pccdummy71(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY71 register."]
    #[inline] pub fn with_pccdummy71<F: FnOnce(Pccdummy71) -> Pccdummy71>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy71_mut(), f(self.pccdummy71()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY72 register."]
    #[inline] pub fn pccdummy72_mut(&self) -> *mut Pccdummy72 { 
        (self.0 + 0x120) as *mut Pccdummy72
    }

    #[doc="Get the *const pointer for the PCCDUMMY72 register."]
    #[inline] pub fn pccdummy72_ptr(&self) -> *const Pccdummy72 { 
           self.pccdummy72_mut()
    }

    #[doc="Read the PCCDUMMY72 register."]
    #[inline] pub fn pccdummy72(&self) -> Pccdummy72 { 
        unsafe {
            read_volatile(self.pccdummy72_ptr())
        }
    }

    #[doc="Write the PCCDUMMY72 register."]
    #[inline] pub fn set_pccdummy72<F: FnOnce(Pccdummy72) -> Pccdummy72>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy72_mut(), f(Pccdummy72(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY72 register."]
    #[inline] pub fn with_pccdummy72<F: FnOnce(Pccdummy72) -> Pccdummy72>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy72_mut(), f(self.pccdummy72()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PORTA register."]
    #[inline] pub fn porta_mut(&self) -> *mut Porta { 
        (self.0 + 0x124) as *mut Porta
    }

    #[doc="Get the *const pointer for the PORTA register."]
    #[inline] pub fn porta_ptr(&self) -> *const Porta { 
           self.porta_mut()
    }

    #[doc="Read the PORTA register."]
    #[inline] pub fn porta(&self) -> Porta { 
        unsafe {
            read_volatile(self.porta_ptr())
        }
    }

    #[doc="Write the PORTA register."]
    #[inline] pub fn set_porta<F: FnOnce(Porta) -> Porta>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.porta_mut(), f(Porta(0)));
        }
        self
    }

    #[doc="Modify the PORTA register."]
    #[inline] pub fn with_porta<F: FnOnce(Porta) -> Porta>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.porta_mut(), f(self.porta()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PORTB register."]
    #[inline] pub fn portb_mut(&self) -> *mut Portb { 
        (self.0 + 0x128) as *mut Portb
    }

    #[doc="Get the *const pointer for the PORTB register."]
    #[inline] pub fn portb_ptr(&self) -> *const Portb { 
           self.portb_mut()
    }

    #[doc="Read the PORTB register."]
    #[inline] pub fn portb(&self) -> Portb { 
        unsafe {
            read_volatile(self.portb_ptr())
        }
    }

    #[doc="Write the PORTB register."]
    #[inline] pub fn set_portb<F: FnOnce(Portb) -> Portb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.portb_mut(), f(Portb(0)));
        }
        self
    }

    #[doc="Modify the PORTB register."]
    #[inline] pub fn with_portb<F: FnOnce(Portb) -> Portb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.portb_mut(), f(self.portb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PORTC register."]
    #[inline] pub fn portc_mut(&self) -> *mut Portc { 
        (self.0 + 0x12c) as *mut Portc
    }

    #[doc="Get the *const pointer for the PORTC register."]
    #[inline] pub fn portc_ptr(&self) -> *const Portc { 
           self.portc_mut()
    }

    #[doc="Read the PORTC register."]
    #[inline] pub fn portc(&self) -> Portc { 
        unsafe {
            read_volatile(self.portc_ptr())
        }
    }

    #[doc="Write the PORTC register."]
    #[inline] pub fn set_portc<F: FnOnce(Portc) -> Portc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.portc_mut(), f(Portc(0)));
        }
        self
    }

    #[doc="Modify the PORTC register."]
    #[inline] pub fn with_portc<F: FnOnce(Portc) -> Portc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.portc_mut(), f(self.portc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PORTD register."]
    #[inline] pub fn portd_mut(&self) -> *mut Portd { 
        (self.0 + 0x130) as *mut Portd
    }

    #[doc="Get the *const pointer for the PORTD register."]
    #[inline] pub fn portd_ptr(&self) -> *const Portd { 
           self.portd_mut()
    }

    #[doc="Read the PORTD register."]
    #[inline] pub fn portd(&self) -> Portd { 
        unsafe {
            read_volatile(self.portd_ptr())
        }
    }

    #[doc="Write the PORTD register."]
    #[inline] pub fn set_portd<F: FnOnce(Portd) -> Portd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.portd_mut(), f(Portd(0)));
        }
        self
    }

    #[doc="Modify the PORTD register."]
    #[inline] pub fn with_portd<F: FnOnce(Portd) -> Portd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.portd_mut(), f(self.portd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PORTE register."]
    #[inline] pub fn porte_mut(&self) -> *mut Porte { 
        (self.0 + 0x134) as *mut Porte
    }

    #[doc="Get the *const pointer for the PORTE register."]
    #[inline] pub fn porte_ptr(&self) -> *const Porte { 
           self.porte_mut()
    }

    #[doc="Read the PORTE register."]
    #[inline] pub fn porte(&self) -> Porte { 
        unsafe {
            read_volatile(self.porte_ptr())
        }
    }

    #[doc="Write the PORTE register."]
    #[inline] pub fn set_porte<F: FnOnce(Porte) -> Porte>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.porte_mut(), f(Porte(0)));
        }
        self
    }

    #[doc="Modify the PORTE register."]
    #[inline] pub fn with_porte<F: FnOnce(Porte) -> Porte>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.porte_mut(), f(self.porte()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY78 register."]
    #[inline] pub fn pccdummy78_mut(&self) -> *mut Pccdummy78 { 
        (self.0 + 0x138) as *mut Pccdummy78
    }

    #[doc="Get the *const pointer for the PCCDUMMY78 register."]
    #[inline] pub fn pccdummy78_ptr(&self) -> *const Pccdummy78 { 
           self.pccdummy78_mut()
    }

    #[doc="Read the PCCDUMMY78 register."]
    #[inline] pub fn pccdummy78(&self) -> Pccdummy78 { 
        unsafe {
            read_volatile(self.pccdummy78_ptr())
        }
    }

    #[doc="Write the PCCDUMMY78 register."]
    #[inline] pub fn set_pccdummy78<F: FnOnce(Pccdummy78) -> Pccdummy78>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy78_mut(), f(Pccdummy78(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY78 register."]
    #[inline] pub fn with_pccdummy78<F: FnOnce(Pccdummy78) -> Pccdummy78>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy78_mut(), f(self.pccdummy78()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY79 register."]
    #[inline] pub fn pccdummy79_mut(&self) -> *mut Pccdummy79 { 
        (self.0 + 0x13c) as *mut Pccdummy79
    }

    #[doc="Get the *const pointer for the PCCDUMMY79 register."]
    #[inline] pub fn pccdummy79_ptr(&self) -> *const Pccdummy79 { 
           self.pccdummy79_mut()
    }

    #[doc="Read the PCCDUMMY79 register."]
    #[inline] pub fn pccdummy79(&self) -> Pccdummy79 { 
        unsafe {
            read_volatile(self.pccdummy79_ptr())
        }
    }

    #[doc="Write the PCCDUMMY79 register."]
    #[inline] pub fn set_pccdummy79<F: FnOnce(Pccdummy79) -> Pccdummy79>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy79_mut(), f(Pccdummy79(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY79 register."]
    #[inline] pub fn with_pccdummy79<F: FnOnce(Pccdummy79) -> Pccdummy79>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy79_mut(), f(self.pccdummy79()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY80 register."]
    #[inline] pub fn pccdummy80_mut(&self) -> *mut Pccdummy80 { 
        (self.0 + 0x140) as *mut Pccdummy80
    }

    #[doc="Get the *const pointer for the PCCDUMMY80 register."]
    #[inline] pub fn pccdummy80_ptr(&self) -> *const Pccdummy80 { 
           self.pccdummy80_mut()
    }

    #[doc="Read the PCCDUMMY80 register."]
    #[inline] pub fn pccdummy80(&self) -> Pccdummy80 { 
        unsafe {
            read_volatile(self.pccdummy80_ptr())
        }
    }

    #[doc="Write the PCCDUMMY80 register."]
    #[inline] pub fn set_pccdummy80<F: FnOnce(Pccdummy80) -> Pccdummy80>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy80_mut(), f(Pccdummy80(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY80 register."]
    #[inline] pub fn with_pccdummy80<F: FnOnce(Pccdummy80) -> Pccdummy80>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy80_mut(), f(self.pccdummy80()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY81 register."]
    #[inline] pub fn pccdummy81_mut(&self) -> *mut Pccdummy81 { 
        (self.0 + 0x144) as *mut Pccdummy81
    }

    #[doc="Get the *const pointer for the PCCDUMMY81 register."]
    #[inline] pub fn pccdummy81_ptr(&self) -> *const Pccdummy81 { 
           self.pccdummy81_mut()
    }

    #[doc="Read the PCCDUMMY81 register."]
    #[inline] pub fn pccdummy81(&self) -> Pccdummy81 { 
        unsafe {
            read_volatile(self.pccdummy81_ptr())
        }
    }

    #[doc="Write the PCCDUMMY81 register."]
    #[inline] pub fn set_pccdummy81<F: FnOnce(Pccdummy81) -> Pccdummy81>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy81_mut(), f(Pccdummy81(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY81 register."]
    #[inline] pub fn with_pccdummy81<F: FnOnce(Pccdummy81) -> Pccdummy81>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy81_mut(), f(self.pccdummy81()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY82 register."]
    #[inline] pub fn pccdummy82_mut(&self) -> *mut Pccdummy82 { 
        (self.0 + 0x148) as *mut Pccdummy82
    }

    #[doc="Get the *const pointer for the PCCDUMMY82 register."]
    #[inline] pub fn pccdummy82_ptr(&self) -> *const Pccdummy82 { 
           self.pccdummy82_mut()
    }

    #[doc="Read the PCCDUMMY82 register."]
    #[inline] pub fn pccdummy82(&self) -> Pccdummy82 { 
        unsafe {
            read_volatile(self.pccdummy82_ptr())
        }
    }

    #[doc="Write the PCCDUMMY82 register."]
    #[inline] pub fn set_pccdummy82<F: FnOnce(Pccdummy82) -> Pccdummy82>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy82_mut(), f(Pccdummy82(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY82 register."]
    #[inline] pub fn with_pccdummy82<F: FnOnce(Pccdummy82) -> Pccdummy82>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy82_mut(), f(self.pccdummy82()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY83 register."]
    #[inline] pub fn pccdummy83_mut(&self) -> *mut Pccdummy83 { 
        (self.0 + 0x14c) as *mut Pccdummy83
    }

    #[doc="Get the *const pointer for the PCCDUMMY83 register."]
    #[inline] pub fn pccdummy83_ptr(&self) -> *const Pccdummy83 { 
           self.pccdummy83_mut()
    }

    #[doc="Read the PCCDUMMY83 register."]
    #[inline] pub fn pccdummy83(&self) -> Pccdummy83 { 
        unsafe {
            read_volatile(self.pccdummy83_ptr())
        }
    }

    #[doc="Write the PCCDUMMY83 register."]
    #[inline] pub fn set_pccdummy83<F: FnOnce(Pccdummy83) -> Pccdummy83>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy83_mut(), f(Pccdummy83(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY83 register."]
    #[inline] pub fn with_pccdummy83<F: FnOnce(Pccdummy83) -> Pccdummy83>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy83_mut(), f(self.pccdummy83()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY84 register."]
    #[inline] pub fn pccdummy84_mut(&self) -> *mut Pccdummy84 { 
        (self.0 + 0x150) as *mut Pccdummy84
    }

    #[doc="Get the *const pointer for the PCCDUMMY84 register."]
    #[inline] pub fn pccdummy84_ptr(&self) -> *const Pccdummy84 { 
           self.pccdummy84_mut()
    }

    #[doc="Read the PCCDUMMY84 register."]
    #[inline] pub fn pccdummy84(&self) -> Pccdummy84 { 
        unsafe {
            read_volatile(self.pccdummy84_ptr())
        }
    }

    #[doc="Write the PCCDUMMY84 register."]
    #[inline] pub fn set_pccdummy84<F: FnOnce(Pccdummy84) -> Pccdummy84>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy84_mut(), f(Pccdummy84(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY84 register."]
    #[inline] pub fn with_pccdummy84<F: FnOnce(Pccdummy84) -> Pccdummy84>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy84_mut(), f(self.pccdummy84()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY85 register."]
    #[inline] pub fn pccdummy85_mut(&self) -> *mut Pccdummy85 { 
        (self.0 + 0x154) as *mut Pccdummy85
    }

    #[doc="Get the *const pointer for the PCCDUMMY85 register."]
    #[inline] pub fn pccdummy85_ptr(&self) -> *const Pccdummy85 { 
           self.pccdummy85_mut()
    }

    #[doc="Read the PCCDUMMY85 register."]
    #[inline] pub fn pccdummy85(&self) -> Pccdummy85 { 
        unsafe {
            read_volatile(self.pccdummy85_ptr())
        }
    }

    #[doc="Write the PCCDUMMY85 register."]
    #[inline] pub fn set_pccdummy85<F: FnOnce(Pccdummy85) -> Pccdummy85>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy85_mut(), f(Pccdummy85(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY85 register."]
    #[inline] pub fn with_pccdummy85<F: FnOnce(Pccdummy85) -> Pccdummy85>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy85_mut(), f(self.pccdummy85()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY86 register."]
    #[inline] pub fn pccdummy86_mut(&self) -> *mut Pccdummy86 { 
        (self.0 + 0x158) as *mut Pccdummy86
    }

    #[doc="Get the *const pointer for the PCCDUMMY86 register."]
    #[inline] pub fn pccdummy86_ptr(&self) -> *const Pccdummy86 { 
           self.pccdummy86_mut()
    }

    #[doc="Read the PCCDUMMY86 register."]
    #[inline] pub fn pccdummy86(&self) -> Pccdummy86 { 
        unsafe {
            read_volatile(self.pccdummy86_ptr())
        }
    }

    #[doc="Write the PCCDUMMY86 register."]
    #[inline] pub fn set_pccdummy86<F: FnOnce(Pccdummy86) -> Pccdummy86>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy86_mut(), f(Pccdummy86(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY86 register."]
    #[inline] pub fn with_pccdummy86<F: FnOnce(Pccdummy86) -> Pccdummy86>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy86_mut(), f(self.pccdummy86()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY87 register."]
    #[inline] pub fn pccdummy87_mut(&self) -> *mut Pccdummy87 { 
        (self.0 + 0x15c) as *mut Pccdummy87
    }

    #[doc="Get the *const pointer for the PCCDUMMY87 register."]
    #[inline] pub fn pccdummy87_ptr(&self) -> *const Pccdummy87 { 
           self.pccdummy87_mut()
    }

    #[doc="Read the PCCDUMMY87 register."]
    #[inline] pub fn pccdummy87(&self) -> Pccdummy87 { 
        unsafe {
            read_volatile(self.pccdummy87_ptr())
        }
    }

    #[doc="Write the PCCDUMMY87 register."]
    #[inline] pub fn set_pccdummy87<F: FnOnce(Pccdummy87) -> Pccdummy87>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy87_mut(), f(Pccdummy87(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY87 register."]
    #[inline] pub fn with_pccdummy87<F: FnOnce(Pccdummy87) -> Pccdummy87>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy87_mut(), f(self.pccdummy87()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY88 register."]
    #[inline] pub fn pccdummy88_mut(&self) -> *mut Pccdummy88 { 
        (self.0 + 0x160) as *mut Pccdummy88
    }

    #[doc="Get the *const pointer for the PCCDUMMY88 register."]
    #[inline] pub fn pccdummy88_ptr(&self) -> *const Pccdummy88 { 
           self.pccdummy88_mut()
    }

    #[doc="Read the PCCDUMMY88 register."]
    #[inline] pub fn pccdummy88(&self) -> Pccdummy88 { 
        unsafe {
            read_volatile(self.pccdummy88_ptr())
        }
    }

    #[doc="Write the PCCDUMMY88 register."]
    #[inline] pub fn set_pccdummy88<F: FnOnce(Pccdummy88) -> Pccdummy88>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy88_mut(), f(Pccdummy88(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY88 register."]
    #[inline] pub fn with_pccdummy88<F: FnOnce(Pccdummy88) -> Pccdummy88>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy88_mut(), f(self.pccdummy88()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY89 register."]
    #[inline] pub fn pccdummy89_mut(&self) -> *mut Pccdummy89 { 
        (self.0 + 0x164) as *mut Pccdummy89
    }

    #[doc="Get the *const pointer for the PCCDUMMY89 register."]
    #[inline] pub fn pccdummy89_ptr(&self) -> *const Pccdummy89 { 
           self.pccdummy89_mut()
    }

    #[doc="Read the PCCDUMMY89 register."]
    #[inline] pub fn pccdummy89(&self) -> Pccdummy89 { 
        unsafe {
            read_volatile(self.pccdummy89_ptr())
        }
    }

    #[doc="Write the PCCDUMMY89 register."]
    #[inline] pub fn set_pccdummy89<F: FnOnce(Pccdummy89) -> Pccdummy89>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy89_mut(), f(Pccdummy89(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY89 register."]
    #[inline] pub fn with_pccdummy89<F: FnOnce(Pccdummy89) -> Pccdummy89>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy89_mut(), f(self.pccdummy89()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FLEXIO register."]
    #[inline] pub fn flexio_mut(&self) -> *mut Flexio { 
        (self.0 + 0x168) as *mut Flexio
    }

    #[doc="Get the *const pointer for the FLEXIO register."]
    #[inline] pub fn flexio_ptr(&self) -> *const Flexio { 
           self.flexio_mut()
    }

    #[doc="Read the FLEXIO register."]
    #[inline] pub fn flexio(&self) -> Flexio { 
        unsafe {
            read_volatile(self.flexio_ptr())
        }
    }

    #[doc="Write the FLEXIO register."]
    #[inline] pub fn set_flexio<F: FnOnce(Flexio) -> Flexio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flexio_mut(), f(Flexio(0)));
        }
        self
    }

    #[doc="Modify the FLEXIO register."]
    #[inline] pub fn with_flexio<F: FnOnce(Flexio) -> Flexio>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.flexio_mut(), f(self.flexio()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY91 register."]
    #[inline] pub fn pccdummy91_mut(&self) -> *mut Pccdummy91 { 
        (self.0 + 0x16c) as *mut Pccdummy91
    }

    #[doc="Get the *const pointer for the PCCDUMMY91 register."]
    #[inline] pub fn pccdummy91_ptr(&self) -> *const Pccdummy91 { 
           self.pccdummy91_mut()
    }

    #[doc="Read the PCCDUMMY91 register."]
    #[inline] pub fn pccdummy91(&self) -> Pccdummy91 { 
        unsafe {
            read_volatile(self.pccdummy91_ptr())
        }
    }

    #[doc="Write the PCCDUMMY91 register."]
    #[inline] pub fn set_pccdummy91<F: FnOnce(Pccdummy91) -> Pccdummy91>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy91_mut(), f(Pccdummy91(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY91 register."]
    #[inline] pub fn with_pccdummy91<F: FnOnce(Pccdummy91) -> Pccdummy91>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy91_mut(), f(self.pccdummy91()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY92 register."]
    #[inline] pub fn pccdummy92_mut(&self) -> *mut Pccdummy92 { 
        (self.0 + 0x170) as *mut Pccdummy92
    }

    #[doc="Get the *const pointer for the PCCDUMMY92 register."]
    #[inline] pub fn pccdummy92_ptr(&self) -> *const Pccdummy92 { 
           self.pccdummy92_mut()
    }

    #[doc="Read the PCCDUMMY92 register."]
    #[inline] pub fn pccdummy92(&self) -> Pccdummy92 { 
        unsafe {
            read_volatile(self.pccdummy92_ptr())
        }
    }

    #[doc="Write the PCCDUMMY92 register."]
    #[inline] pub fn set_pccdummy92<F: FnOnce(Pccdummy92) -> Pccdummy92>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy92_mut(), f(Pccdummy92(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY92 register."]
    #[inline] pub fn with_pccdummy92<F: FnOnce(Pccdummy92) -> Pccdummy92>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy92_mut(), f(self.pccdummy92()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY93 register."]
    #[inline] pub fn pccdummy93_mut(&self) -> *mut Pccdummy93 { 
        (self.0 + 0x174) as *mut Pccdummy93
    }

    #[doc="Get the *const pointer for the PCCDUMMY93 register."]
    #[inline] pub fn pccdummy93_ptr(&self) -> *const Pccdummy93 { 
           self.pccdummy93_mut()
    }

    #[doc="Read the PCCDUMMY93 register."]
    #[inline] pub fn pccdummy93(&self) -> Pccdummy93 { 
        unsafe {
            read_volatile(self.pccdummy93_ptr())
        }
    }

    #[doc="Write the PCCDUMMY93 register."]
    #[inline] pub fn set_pccdummy93<F: FnOnce(Pccdummy93) -> Pccdummy93>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy93_mut(), f(Pccdummy93(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY93 register."]
    #[inline] pub fn with_pccdummy93<F: FnOnce(Pccdummy93) -> Pccdummy93>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy93_mut(), f(self.pccdummy93()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY94 register."]
    #[inline] pub fn pccdummy94_mut(&self) -> *mut Pccdummy94 { 
        (self.0 + 0x178) as *mut Pccdummy94
    }

    #[doc="Get the *const pointer for the PCCDUMMY94 register."]
    #[inline] pub fn pccdummy94_ptr(&self) -> *const Pccdummy94 { 
           self.pccdummy94_mut()
    }

    #[doc="Read the PCCDUMMY94 register."]
    #[inline] pub fn pccdummy94(&self) -> Pccdummy94 { 
        unsafe {
            read_volatile(self.pccdummy94_ptr())
        }
    }

    #[doc="Write the PCCDUMMY94 register."]
    #[inline] pub fn set_pccdummy94<F: FnOnce(Pccdummy94) -> Pccdummy94>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy94_mut(), f(Pccdummy94(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY94 register."]
    #[inline] pub fn with_pccdummy94<F: FnOnce(Pccdummy94) -> Pccdummy94>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy94_mut(), f(self.pccdummy94()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY95 register."]
    #[inline] pub fn pccdummy95_mut(&self) -> *mut Pccdummy95 { 
        (self.0 + 0x17c) as *mut Pccdummy95
    }

    #[doc="Get the *const pointer for the PCCDUMMY95 register."]
    #[inline] pub fn pccdummy95_ptr(&self) -> *const Pccdummy95 { 
           self.pccdummy95_mut()
    }

    #[doc="Read the PCCDUMMY95 register."]
    #[inline] pub fn pccdummy95(&self) -> Pccdummy95 { 
        unsafe {
            read_volatile(self.pccdummy95_ptr())
        }
    }

    #[doc="Write the PCCDUMMY95 register."]
    #[inline] pub fn set_pccdummy95<F: FnOnce(Pccdummy95) -> Pccdummy95>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy95_mut(), f(Pccdummy95(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY95 register."]
    #[inline] pub fn with_pccdummy95<F: FnOnce(Pccdummy95) -> Pccdummy95>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy95_mut(), f(self.pccdummy95()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY96 register."]
    #[inline] pub fn pccdummy96_mut(&self) -> *mut Pccdummy96 { 
        (self.0 + 0x180) as *mut Pccdummy96
    }

    #[doc="Get the *const pointer for the PCCDUMMY96 register."]
    #[inline] pub fn pccdummy96_ptr(&self) -> *const Pccdummy96 { 
           self.pccdummy96_mut()
    }

    #[doc="Read the PCCDUMMY96 register."]
    #[inline] pub fn pccdummy96(&self) -> Pccdummy96 { 
        unsafe {
            read_volatile(self.pccdummy96_ptr())
        }
    }

    #[doc="Write the PCCDUMMY96 register."]
    #[inline] pub fn set_pccdummy96<F: FnOnce(Pccdummy96) -> Pccdummy96>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy96_mut(), f(Pccdummy96(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY96 register."]
    #[inline] pub fn with_pccdummy96<F: FnOnce(Pccdummy96) -> Pccdummy96>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy96_mut(), f(self.pccdummy96()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EWM register."]
    #[inline] pub fn ewm_mut(&self) -> *mut Ewm { 
        (self.0 + 0x184) as *mut Ewm
    }

    #[doc="Get the *const pointer for the EWM register."]
    #[inline] pub fn ewm_ptr(&self) -> *const Ewm { 
           self.ewm_mut()
    }

    #[doc="Read the EWM register."]
    #[inline] pub fn ewm(&self) -> Ewm { 
        unsafe {
            read_volatile(self.ewm_ptr())
        }
    }

    #[doc="Write the EWM register."]
    #[inline] pub fn set_ewm<F: FnOnce(Ewm) -> Ewm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ewm_mut(), f(Ewm(0)));
        }
        self
    }

    #[doc="Modify the EWM register."]
    #[inline] pub fn with_ewm<F: FnOnce(Ewm) -> Ewm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ewm_mut(), f(self.ewm()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY98 register."]
    #[inline] pub fn pccdummy98_mut(&self) -> *mut Pccdummy98 { 
        (self.0 + 0x188) as *mut Pccdummy98
    }

    #[doc="Get the *const pointer for the PCCDUMMY98 register."]
    #[inline] pub fn pccdummy98_ptr(&self) -> *const Pccdummy98 { 
           self.pccdummy98_mut()
    }

    #[doc="Read the PCCDUMMY98 register."]
    #[inline] pub fn pccdummy98(&self) -> Pccdummy98 { 
        unsafe {
            read_volatile(self.pccdummy98_ptr())
        }
    }

    #[doc="Write the PCCDUMMY98 register."]
    #[inline] pub fn set_pccdummy98<F: FnOnce(Pccdummy98) -> Pccdummy98>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy98_mut(), f(Pccdummy98(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY98 register."]
    #[inline] pub fn with_pccdummy98<F: FnOnce(Pccdummy98) -> Pccdummy98>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy98_mut(), f(self.pccdummy98()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY99 register."]
    #[inline] pub fn pccdummy99_mut(&self) -> *mut Pccdummy99 { 
        (self.0 + 0x18c) as *mut Pccdummy99
    }

    #[doc="Get the *const pointer for the PCCDUMMY99 register."]
    #[inline] pub fn pccdummy99_ptr(&self) -> *const Pccdummy99 { 
           self.pccdummy99_mut()
    }

    #[doc="Read the PCCDUMMY99 register."]
    #[inline] pub fn pccdummy99(&self) -> Pccdummy99 { 
        unsafe {
            read_volatile(self.pccdummy99_ptr())
        }
    }

    #[doc="Write the PCCDUMMY99 register."]
    #[inline] pub fn set_pccdummy99<F: FnOnce(Pccdummy99) -> Pccdummy99>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy99_mut(), f(Pccdummy99(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY99 register."]
    #[inline] pub fn with_pccdummy99<F: FnOnce(Pccdummy99) -> Pccdummy99>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy99_mut(), f(self.pccdummy99()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY100 register."]
    #[inline] pub fn pccdummy100_mut(&self) -> *mut Pccdummy100 { 
        (self.0 + 0x190) as *mut Pccdummy100
    }

    #[doc="Get the *const pointer for the PCCDUMMY100 register."]
    #[inline] pub fn pccdummy100_ptr(&self) -> *const Pccdummy100 { 
           self.pccdummy100_mut()
    }

    #[doc="Read the PCCDUMMY100 register."]
    #[inline] pub fn pccdummy100(&self) -> Pccdummy100 { 
        unsafe {
            read_volatile(self.pccdummy100_ptr())
        }
    }

    #[doc="Write the PCCDUMMY100 register."]
    #[inline] pub fn set_pccdummy100<F: FnOnce(Pccdummy100) -> Pccdummy100>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy100_mut(), f(Pccdummy100(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY100 register."]
    #[inline] pub fn with_pccdummy100<F: FnOnce(Pccdummy100) -> Pccdummy100>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy100_mut(), f(self.pccdummy100()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY101 register."]
    #[inline] pub fn pccdummy101_mut(&self) -> *mut Pccdummy101 { 
        (self.0 + 0x194) as *mut Pccdummy101
    }

    #[doc="Get the *const pointer for the PCCDUMMY101 register."]
    #[inline] pub fn pccdummy101_ptr(&self) -> *const Pccdummy101 { 
           self.pccdummy101_mut()
    }

    #[doc="Read the PCCDUMMY101 register."]
    #[inline] pub fn pccdummy101(&self) -> Pccdummy101 { 
        unsafe {
            read_volatile(self.pccdummy101_ptr())
        }
    }

    #[doc="Write the PCCDUMMY101 register."]
    #[inline] pub fn set_pccdummy101<F: FnOnce(Pccdummy101) -> Pccdummy101>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy101_mut(), f(Pccdummy101(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY101 register."]
    #[inline] pub fn with_pccdummy101<F: FnOnce(Pccdummy101) -> Pccdummy101>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy101_mut(), f(self.pccdummy101()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LPI2C0 register."]
    #[inline] pub fn lpi2c0_mut(&self) -> *mut Lpi2c0 { 
        (self.0 + 0x198) as *mut Lpi2c0
    }

    #[doc="Get the *const pointer for the LPI2C0 register."]
    #[inline] pub fn lpi2c0_ptr(&self) -> *const Lpi2c0 { 
           self.lpi2c0_mut()
    }

    #[doc="Read the LPI2C0 register."]
    #[inline] pub fn lpi2c0(&self) -> Lpi2c0 { 
        unsafe {
            read_volatile(self.lpi2c0_ptr())
        }
    }

    #[doc="Write the LPI2C0 register."]
    #[inline] pub fn set_lpi2c0<F: FnOnce(Lpi2c0) -> Lpi2c0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpi2c0_mut(), f(Lpi2c0(0)));
        }
        self
    }

    #[doc="Modify the LPI2C0 register."]
    #[inline] pub fn with_lpi2c0<F: FnOnce(Lpi2c0) -> Lpi2c0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpi2c0_mut(), f(self.lpi2c0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY103 register."]
    #[inline] pub fn pccdummy103_mut(&self) -> *mut Pccdummy103 { 
        (self.0 + 0x19c) as *mut Pccdummy103
    }

    #[doc="Get the *const pointer for the PCCDUMMY103 register."]
    #[inline] pub fn pccdummy103_ptr(&self) -> *const Pccdummy103 { 
           self.pccdummy103_mut()
    }

    #[doc="Read the PCCDUMMY103 register."]
    #[inline] pub fn pccdummy103(&self) -> Pccdummy103 { 
        unsafe {
            read_volatile(self.pccdummy103_ptr())
        }
    }

    #[doc="Write the PCCDUMMY103 register."]
    #[inline] pub fn set_pccdummy103<F: FnOnce(Pccdummy103) -> Pccdummy103>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy103_mut(), f(Pccdummy103(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY103 register."]
    #[inline] pub fn with_pccdummy103<F: FnOnce(Pccdummy103) -> Pccdummy103>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy103_mut(), f(self.pccdummy103()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY104 register."]
    #[inline] pub fn pccdummy104_mut(&self) -> *mut Pccdummy104 { 
        (self.0 + 0x1a0) as *mut Pccdummy104
    }

    #[doc="Get the *const pointer for the PCCDUMMY104 register."]
    #[inline] pub fn pccdummy104_ptr(&self) -> *const Pccdummy104 { 
           self.pccdummy104_mut()
    }

    #[doc="Read the PCCDUMMY104 register."]
    #[inline] pub fn pccdummy104(&self) -> Pccdummy104 { 
        unsafe {
            read_volatile(self.pccdummy104_ptr())
        }
    }

    #[doc="Write the PCCDUMMY104 register."]
    #[inline] pub fn set_pccdummy104<F: FnOnce(Pccdummy104) -> Pccdummy104>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy104_mut(), f(Pccdummy104(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY104 register."]
    #[inline] pub fn with_pccdummy104<F: FnOnce(Pccdummy104) -> Pccdummy104>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy104_mut(), f(self.pccdummy104()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY105 register."]
    #[inline] pub fn pccdummy105_mut(&self) -> *mut Pccdummy105 { 
        (self.0 + 0x1a4) as *mut Pccdummy105
    }

    #[doc="Get the *const pointer for the PCCDUMMY105 register."]
    #[inline] pub fn pccdummy105_ptr(&self) -> *const Pccdummy105 { 
           self.pccdummy105_mut()
    }

    #[doc="Read the PCCDUMMY105 register."]
    #[inline] pub fn pccdummy105(&self) -> Pccdummy105 { 
        unsafe {
            read_volatile(self.pccdummy105_ptr())
        }
    }

    #[doc="Write the PCCDUMMY105 register."]
    #[inline] pub fn set_pccdummy105<F: FnOnce(Pccdummy105) -> Pccdummy105>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy105_mut(), f(Pccdummy105(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY105 register."]
    #[inline] pub fn with_pccdummy105<F: FnOnce(Pccdummy105) -> Pccdummy105>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy105_mut(), f(self.pccdummy105()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LPUART0 register."]
    #[inline] pub fn lpuart0_mut(&self) -> *mut Lpuart0 { 
        (self.0 + 0x1a8) as *mut Lpuart0
    }

    #[doc="Get the *const pointer for the LPUART0 register."]
    #[inline] pub fn lpuart0_ptr(&self) -> *const Lpuart0 { 
           self.lpuart0_mut()
    }

    #[doc="Read the LPUART0 register."]
    #[inline] pub fn lpuart0(&self) -> Lpuart0 { 
        unsafe {
            read_volatile(self.lpuart0_ptr())
        }
    }

    #[doc="Write the LPUART0 register."]
    #[inline] pub fn set_lpuart0<F: FnOnce(Lpuart0) -> Lpuart0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpuart0_mut(), f(Lpuart0(0)));
        }
        self
    }

    #[doc="Modify the LPUART0 register."]
    #[inline] pub fn with_lpuart0<F: FnOnce(Lpuart0) -> Lpuart0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpuart0_mut(), f(self.lpuart0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LPUART1 register."]
    #[inline] pub fn lpuart1_mut(&self) -> *mut Lpuart1 { 
        (self.0 + 0x1ac) as *mut Lpuart1
    }

    #[doc="Get the *const pointer for the LPUART1 register."]
    #[inline] pub fn lpuart1_ptr(&self) -> *const Lpuart1 { 
           self.lpuart1_mut()
    }

    #[doc="Read the LPUART1 register."]
    #[inline] pub fn lpuart1(&self) -> Lpuart1 { 
        unsafe {
            read_volatile(self.lpuart1_ptr())
        }
    }

    #[doc="Write the LPUART1 register."]
    #[inline] pub fn set_lpuart1<F: FnOnce(Lpuart1) -> Lpuart1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpuart1_mut(), f(Lpuart1(0)));
        }
        self
    }

    #[doc="Modify the LPUART1 register."]
    #[inline] pub fn with_lpuart1<F: FnOnce(Lpuart1) -> Lpuart1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpuart1_mut(), f(self.lpuart1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LPUART2 register."]
    #[inline] pub fn lpuart2_mut(&self) -> *mut Lpuart2 { 
        (self.0 + 0x1b0) as *mut Lpuart2
    }

    #[doc="Get the *const pointer for the LPUART2 register."]
    #[inline] pub fn lpuart2_ptr(&self) -> *const Lpuart2 { 
           self.lpuart2_mut()
    }

    #[doc="Read the LPUART2 register."]
    #[inline] pub fn lpuart2(&self) -> Lpuart2 { 
        unsafe {
            read_volatile(self.lpuart2_ptr())
        }
    }

    #[doc="Write the LPUART2 register."]
    #[inline] pub fn set_lpuart2<F: FnOnce(Lpuart2) -> Lpuart2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpuart2_mut(), f(Lpuart2(0)));
        }
        self
    }

    #[doc="Modify the LPUART2 register."]
    #[inline] pub fn with_lpuart2<F: FnOnce(Lpuart2) -> Lpuart2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpuart2_mut(), f(self.lpuart2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY109 register."]
    #[inline] pub fn pccdummy109_mut(&self) -> *mut Pccdummy109 { 
        (self.0 + 0x1b4) as *mut Pccdummy109
    }

    #[doc="Get the *const pointer for the PCCDUMMY109 register."]
    #[inline] pub fn pccdummy109_ptr(&self) -> *const Pccdummy109 { 
           self.pccdummy109_mut()
    }

    #[doc="Read the PCCDUMMY109 register."]
    #[inline] pub fn pccdummy109(&self) -> Pccdummy109 { 
        unsafe {
            read_volatile(self.pccdummy109_ptr())
        }
    }

    #[doc="Write the PCCDUMMY109 register."]
    #[inline] pub fn set_pccdummy109<F: FnOnce(Pccdummy109) -> Pccdummy109>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy109_mut(), f(Pccdummy109(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY109 register."]
    #[inline] pub fn with_pccdummy109<F: FnOnce(Pccdummy109) -> Pccdummy109>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy109_mut(), f(self.pccdummy109()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY110 register."]
    #[inline] pub fn pccdummy110_mut(&self) -> *mut Pccdummy110 { 
        (self.0 + 0x1b8) as *mut Pccdummy110
    }

    #[doc="Get the *const pointer for the PCCDUMMY110 register."]
    #[inline] pub fn pccdummy110_ptr(&self) -> *const Pccdummy110 { 
           self.pccdummy110_mut()
    }

    #[doc="Read the PCCDUMMY110 register."]
    #[inline] pub fn pccdummy110(&self) -> Pccdummy110 { 
        unsafe {
            read_volatile(self.pccdummy110_ptr())
        }
    }

    #[doc="Write the PCCDUMMY110 register."]
    #[inline] pub fn set_pccdummy110<F: FnOnce(Pccdummy110) -> Pccdummy110>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy110_mut(), f(Pccdummy110(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY110 register."]
    #[inline] pub fn with_pccdummy110<F: FnOnce(Pccdummy110) -> Pccdummy110>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy110_mut(), f(self.pccdummy110()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY111 register."]
    #[inline] pub fn pccdummy111_mut(&self) -> *mut Pccdummy111 { 
        (self.0 + 0x1bc) as *mut Pccdummy111
    }

    #[doc="Get the *const pointer for the PCCDUMMY111 register."]
    #[inline] pub fn pccdummy111_ptr(&self) -> *const Pccdummy111 { 
           self.pccdummy111_mut()
    }

    #[doc="Read the PCCDUMMY111 register."]
    #[inline] pub fn pccdummy111(&self) -> Pccdummy111 { 
        unsafe {
            read_volatile(self.pccdummy111_ptr())
        }
    }

    #[doc="Write the PCCDUMMY111 register."]
    #[inline] pub fn set_pccdummy111<F: FnOnce(Pccdummy111) -> Pccdummy111>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy111_mut(), f(Pccdummy111(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY111 register."]
    #[inline] pub fn with_pccdummy111<F: FnOnce(Pccdummy111) -> Pccdummy111>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy111_mut(), f(self.pccdummy111()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY112 register."]
    #[inline] pub fn pccdummy112_mut(&self) -> *mut Pccdummy112 { 
        (self.0 + 0x1c0) as *mut Pccdummy112
    }

    #[doc="Get the *const pointer for the PCCDUMMY112 register."]
    #[inline] pub fn pccdummy112_ptr(&self) -> *const Pccdummy112 { 
           self.pccdummy112_mut()
    }

    #[doc="Read the PCCDUMMY112 register."]
    #[inline] pub fn pccdummy112(&self) -> Pccdummy112 { 
        unsafe {
            read_volatile(self.pccdummy112_ptr())
        }
    }

    #[doc="Write the PCCDUMMY112 register."]
    #[inline] pub fn set_pccdummy112<F: FnOnce(Pccdummy112) -> Pccdummy112>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy112_mut(), f(Pccdummy112(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY112 register."]
    #[inline] pub fn with_pccdummy112<F: FnOnce(Pccdummy112) -> Pccdummy112>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy112_mut(), f(self.pccdummy112()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY113 register."]
    #[inline] pub fn pccdummy113_mut(&self) -> *mut Pccdummy113 { 
        (self.0 + 0x1c4) as *mut Pccdummy113
    }

    #[doc="Get the *const pointer for the PCCDUMMY113 register."]
    #[inline] pub fn pccdummy113_ptr(&self) -> *const Pccdummy113 { 
           self.pccdummy113_mut()
    }

    #[doc="Read the PCCDUMMY113 register."]
    #[inline] pub fn pccdummy113(&self) -> Pccdummy113 { 
        unsafe {
            read_volatile(self.pccdummy113_ptr())
        }
    }

    #[doc="Write the PCCDUMMY113 register."]
    #[inline] pub fn set_pccdummy113<F: FnOnce(Pccdummy113) -> Pccdummy113>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy113_mut(), f(Pccdummy113(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY113 register."]
    #[inline] pub fn with_pccdummy113<F: FnOnce(Pccdummy113) -> Pccdummy113>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy113_mut(), f(self.pccdummy113()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCCDUMMY114 register."]
    #[inline] pub fn pccdummy114_mut(&self) -> *mut Pccdummy114 { 
        (self.0 + 0x1c8) as *mut Pccdummy114
    }

    #[doc="Get the *const pointer for the PCCDUMMY114 register."]
    #[inline] pub fn pccdummy114_ptr(&self) -> *const Pccdummy114 { 
           self.pccdummy114_mut()
    }

    #[doc="Read the PCCDUMMY114 register."]
    #[inline] pub fn pccdummy114(&self) -> Pccdummy114 { 
        unsafe {
            read_volatile(self.pccdummy114_ptr())
        }
    }

    #[doc="Write the PCCDUMMY114 register."]
    #[inline] pub fn set_pccdummy114<F: FnOnce(Pccdummy114) -> Pccdummy114>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy114_mut(), f(Pccdummy114(0)));
        }
        self
    }

    #[doc="Modify the PCCDUMMY114 register."]
    #[inline] pub fn with_pccdummy114<F: FnOnce(Pccdummy114) -> Pccdummy114>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pccdummy114_mut(), f(self.pccdummy114()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMP0 register."]
    #[inline] pub fn cmp0_mut(&self) -> *mut Cmp0 { 
        (self.0 + 0x1cc) as *mut Cmp0
    }

    #[doc="Get the *const pointer for the CMP0 register."]
    #[inline] pub fn cmp0_ptr(&self) -> *const Cmp0 { 
           self.cmp0_mut()
    }

    #[doc="Read the CMP0 register."]
    #[inline] pub fn cmp0(&self) -> Cmp0 { 
        unsafe {
            read_volatile(self.cmp0_ptr())
        }
    }

    #[doc="Write the CMP0 register."]
    #[inline] pub fn set_cmp0<F: FnOnce(Cmp0) -> Cmp0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmp0_mut(), f(Cmp0(0)));
        }
        self
    }

    #[doc="Modify the CMP0 register."]
    #[inline] pub fn with_cmp0<F: FnOnce(Cmp0) -> Cmp0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmp0_mut(), f(self.cmp0()));
        }
        self
    }

}

#[doc="PCC Reserved Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy0(pub u32);
impl Pccdummy0 {
}

impl From<u32> for Pccdummy0 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy0(other)
    }
}

impl ::core::fmt::Display for Pccdummy0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy1(pub u32);
impl Pccdummy1 {
}

impl From<u32> for Pccdummy1 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy1(other)
    }
}

impl ::core::fmt::Display for Pccdummy1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy2(pub u32);
impl Pccdummy2 {
}

impl From<u32> for Pccdummy2 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy2(other)
    }
}

impl ::core::fmt::Display for Pccdummy2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy3(pub u32);
impl Pccdummy3 {
}

impl From<u32> for Pccdummy3 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy3(other)
    }
}

impl ::core::fmt::Display for Pccdummy3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy4(pub u32);
impl Pccdummy4 {
}

impl From<u32> for Pccdummy4 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy4(other)
    }
}

impl ::core::fmt::Display for Pccdummy4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 5"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy5(pub u32);
impl Pccdummy5 {
}

impl From<u32> for Pccdummy5 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy5(other)
    }
}

impl ::core::fmt::Display for Pccdummy5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 6"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy6(pub u32);
impl Pccdummy6 {
}

impl From<u32> for Pccdummy6 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy6(other)
    }
}

impl ::core::fmt::Display for Pccdummy6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 7"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy7(pub u32);
impl Pccdummy7 {
}

impl From<u32> for Pccdummy7 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy7(other)
    }
}

impl ::core::fmt::Display for Pccdummy7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 8"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy8(pub u32);
impl Pccdummy8 {
}

impl From<u32> for Pccdummy8 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy8(other)
    }
}

impl ::core::fmt::Display for Pccdummy8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 9"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy9(pub u32);
impl Pccdummy9 {
}

impl From<u32> for Pccdummy9 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy9(other)
    }
}

impl ::core::fmt::Display for Pccdummy9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 10"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy10(pub u32);
impl Pccdummy10 {
}

impl From<u32> for Pccdummy10 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy10(other)
    }
}

impl ::core::fmt::Display for Pccdummy10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 11"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy11(pub u32);
impl Pccdummy11 {
}

impl From<u32> for Pccdummy11 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy11(other)
    }
}

impl ::core::fmt::Display for Pccdummy11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 12"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy12(pub u32);
impl Pccdummy12 {
}

impl From<u32> for Pccdummy12 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy12(other)
    }
}

impl ::core::fmt::Display for Pccdummy12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 13"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy13(pub u32);
impl Pccdummy13 {
}

impl From<u32> for Pccdummy13 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy13(other)
    }
}

impl ::core::fmt::Display for Pccdummy13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 14"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy14(pub u32);
impl Pccdummy14 {
}

impl From<u32> for Pccdummy14 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy14(other)
    }
}

impl ::core::fmt::Display for Pccdummy14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 15"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy15(pub u32);
impl Pccdummy15 {
}

impl From<u32> for Pccdummy15 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy15(other)
    }
}

impl ::core::fmt::Display for Pccdummy15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 16"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy16(pub u32);
impl Pccdummy16 {
}

impl From<u32> for Pccdummy16 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy16(other)
    }
}

impl ::core::fmt::Display for Pccdummy16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 17"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy17(pub u32);
impl Pccdummy17 {
}

impl From<u32> for Pccdummy17 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy17(other)
    }
}

impl ::core::fmt::Display for Pccdummy17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 18"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy18(pub u32);
impl Pccdummy18 {
}

impl From<u32> for Pccdummy18 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy18(other)
    }
}

impl ::core::fmt::Display for Pccdummy18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 19"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy19(pub u32);
impl Pccdummy19 {
}

impl From<u32> for Pccdummy19 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy19(other)
    }
}

impl ::core::fmt::Display for Pccdummy19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 20"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy20(pub u32);
impl Pccdummy20 {
}

impl From<u32> for Pccdummy20 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy20(other)
    }
}

impl ::core::fmt::Display for Pccdummy20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 21"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy21(pub u32);
impl Pccdummy21 {
}

impl From<u32> for Pccdummy21 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy21(other)
    }
}

impl ::core::fmt::Display for Pccdummy21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 22"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy22(pub u32);
impl Pccdummy22 {
}

impl From<u32> for Pccdummy22 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy22(other)
    }
}

impl ::core::fmt::Display for Pccdummy22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 23"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy23(pub u32);
impl Pccdummy23 {
}

impl From<u32> for Pccdummy23 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy23(other)
    }
}

impl ::core::fmt::Display for Pccdummy23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 24"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy24(pub u32);
impl Pccdummy24 {
}

impl From<u32> for Pccdummy24 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy24(other)
    }
}

impl ::core::fmt::Display for Pccdummy24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 25"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy25(pub u32);
impl Pccdummy25 {
}

impl From<u32> for Pccdummy25 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy25(other)
    }
}

impl ::core::fmt::Display for Pccdummy25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 26"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy26(pub u32);
impl Pccdummy26 {
}

impl From<u32> for Pccdummy26 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy26(other)
    }
}

impl ::core::fmt::Display for Pccdummy26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 27"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy27(pub u32);
impl Pccdummy27 {
}

impl From<u32> for Pccdummy27 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy27(other)
    }
}

impl ::core::fmt::Display for Pccdummy27 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy27 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 28"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy28(pub u32);
impl Pccdummy28 {
}

impl From<u32> for Pccdummy28 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy28(other)
    }
}

impl ::core::fmt::Display for Pccdummy28 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy28 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 29"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy29(pub u32);
impl Pccdummy29 {
}

impl From<u32> for Pccdummy29 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy29(other)
    }
}

impl ::core::fmt::Display for Pccdummy29 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy29 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 30"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy30(pub u32);
impl Pccdummy30 {
}

impl From<u32> for Pccdummy30 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy30(other)
    }
}

impl ::core::fmt::Display for Pccdummy30 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy30 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 31"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy31(pub u32);
impl Pccdummy31 {
}

impl From<u32> for Pccdummy31 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy31(other)
    }
}

impl ::core::fmt::Display for Pccdummy31 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy31 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC FTFC Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftfc(pub u32);
impl Ftfc {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ftfc {
    #[inline]
    fn from(other: u32) -> Self {
         Ftfc(other)
    }
}

impl ::core::fmt::Display for Ftfc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ftfc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC DMAMUX Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmamux(pub u32);
impl Dmamux {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Dmamux {
    #[inline]
    fn from(other: u32) -> Self {
         Dmamux(other)
    }
}

impl ::core::fmt::Display for Dmamux {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmamux {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 34"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy34(pub u32);
impl Pccdummy34 {
}

impl From<u32> for Pccdummy34 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy34(other)
    }
}

impl ::core::fmt::Display for Pccdummy34 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy34 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 35"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy35(pub u32);
impl Pccdummy35 {
}

impl From<u32> for Pccdummy35 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy35(other)
    }
}

impl ::core::fmt::Display for Pccdummy35 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy35 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC FlexCAN0 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flexcan0(pub u32);
impl Flexcan0 {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Flexcan0 {
    #[inline]
    fn from(other: u32) -> Self {
         Flexcan0(other)
    }
}

impl ::core::fmt::Display for Flexcan0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Flexcan0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC FlexCAN1 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flexcan1(pub u32);
impl Flexcan1 {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Flexcan1 {
    #[inline]
    fn from(other: u32) -> Self {
         Flexcan1(other)
    }
}

impl ::core::fmt::Display for Flexcan1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Flexcan1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC FTM3 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftm3(pub u32);
impl Ftm3 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ftm3 {
    #[inline]
    fn from(other: u32) -> Self {
         Ftm3(other)
    }
}

impl ::core::fmt::Display for Ftm3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ftm3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC ADC1 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adc1(pub u32);
impl Adc1 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Adc1 {
    #[inline]
    fn from(other: u32) -> Self {
         Adc1(other)
    }
}

impl ::core::fmt::Display for Adc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 40"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy40(pub u32);
impl Pccdummy40 {
}

impl From<u32> for Pccdummy40 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy40(other)
    }
}

impl ::core::fmt::Display for Pccdummy40 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy40 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 41"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy41(pub u32);
impl Pccdummy41 {
}

impl From<u32> for Pccdummy41 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy41(other)
    }
}

impl ::core::fmt::Display for Pccdummy41 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy41 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 42"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy42(pub u32);
impl Pccdummy42 {
}

impl From<u32> for Pccdummy42 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy42(other)
    }
}

impl ::core::fmt::Display for Pccdummy42 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy42 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC FlexCAN2 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flexcan2(pub u32);
impl Flexcan2 {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Flexcan2 {
    #[inline]
    fn from(other: u32) -> Self {
         Flexcan2(other)
    }
}

impl ::core::fmt::Display for Flexcan2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Flexcan2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC LPSPI0 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpspi0(pub u32);
impl Lpspi0 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Lpspi0 {
    #[inline]
    fn from(other: u32) -> Self {
         Lpspi0(other)
    }
}

impl ::core::fmt::Display for Lpspi0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lpspi0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC LPSPI1 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpspi1(pub u32);
impl Lpspi1 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Lpspi1 {
    #[inline]
    fn from(other: u32) -> Self {
         Lpspi1(other)
    }
}

impl ::core::fmt::Display for Lpspi1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lpspi1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC LPSPI2 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpspi2(pub u32);
impl Lpspi2 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Lpspi2 {
    #[inline]
    fn from(other: u32) -> Self {
         Lpspi2(other)
    }
}

impl ::core::fmt::Display for Lpspi2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lpspi2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 47"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy47(pub u32);
impl Pccdummy47 {
}

impl From<u32> for Pccdummy47 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy47(other)
    }
}

impl ::core::fmt::Display for Pccdummy47 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy47 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 48"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy48(pub u32);
impl Pccdummy48 {
}

impl From<u32> for Pccdummy48 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy48(other)
    }
}

impl ::core::fmt::Display for Pccdummy48 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy48 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC PDB1 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdb1(pub u32);
impl Pdb1 {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Pdb1 {
    #[inline]
    fn from(other: u32) -> Self {
         Pdb1(other)
    }
}

impl ::core::fmt::Display for Pdb1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdb1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC CRC Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crc(pub u32);
impl Crc {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Crc {
    #[inline]
    fn from(other: u32) -> Self {
         Crc(other)
    }
}

impl ::core::fmt::Display for Crc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 51"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy51(pub u32);
impl Pccdummy51 {
}

impl From<u32> for Pccdummy51 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy51(other)
    }
}

impl ::core::fmt::Display for Pccdummy51 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy51 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 52"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy52(pub u32);
impl Pccdummy52 {
}

impl From<u32> for Pccdummy52 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy52(other)
    }
}

impl ::core::fmt::Display for Pccdummy52 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy52 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 53"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy53(pub u32);
impl Pccdummy53 {
}

impl From<u32> for Pccdummy53 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy53(other)
    }
}

impl ::core::fmt::Display for Pccdummy53 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy53 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC PDB0 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdb0(pub u32);
impl Pdb0 {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Pdb0 {
    #[inline]
    fn from(other: u32) -> Self {
         Pdb0(other)
    }
}

impl ::core::fmt::Display for Pdb0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdb0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC LPIT Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpit(pub u32);
impl Lpit {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Lpit {
    #[inline]
    fn from(other: u32) -> Self {
         Lpit(other)
    }
}

impl ::core::fmt::Display for Lpit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lpit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC FTM0 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftm0(pub u32);
impl Ftm0 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ftm0 {
    #[inline]
    fn from(other: u32) -> Self {
         Ftm0(other)
    }
}

impl ::core::fmt::Display for Ftm0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ftm0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC FTM1 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftm1(pub u32);
impl Ftm1 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ftm1 {
    #[inline]
    fn from(other: u32) -> Self {
         Ftm1(other)
    }
}

impl ::core::fmt::Display for Ftm1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ftm1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC FTM2 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftm2(pub u32);
impl Ftm2 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ftm2 {
    #[inline]
    fn from(other: u32) -> Self {
         Ftm2(other)
    }
}

impl ::core::fmt::Display for Ftm2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ftm2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC ADC0 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adc0(pub u32);
impl Adc0 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Adc0 {
    #[inline]
    fn from(other: u32) -> Self {
         Adc0(other)
    }
}

impl ::core::fmt::Display for Adc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 60"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy60(pub u32);
impl Pccdummy60 {
}

impl From<u32> for Pccdummy60 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy60(other)
    }
}

impl ::core::fmt::Display for Pccdummy60 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy60 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC RTC Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rtc(pub u32);
impl Rtc {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Rtc {
    #[inline]
    fn from(other: u32) -> Self {
         Rtc(other)
    }
}

impl ::core::fmt::Display for Rtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 62"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy62(pub u32);
impl Pccdummy62 {
}

impl From<u32> for Pccdummy62 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy62(other)
    }
}

impl ::core::fmt::Display for Pccdummy62 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy62 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 63"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy63(pub u32);
impl Pccdummy63 {
}

impl From<u32> for Pccdummy63 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy63(other)
    }
}

impl ::core::fmt::Display for Pccdummy63 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy63 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC LPTMR0 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lptmr0(pub u32);
impl Lptmr0 {
    #[doc="Peripheral Clock Divider Select"]
    #[inline] pub fn pcd(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if PCD != 0"]
    #[inline] pub fn test_pcd(&self) -> bool {
        self.pcd() != 0
    }

    #[doc="Sets the PCD field."]
    #[inline] pub fn set_pcd<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Peripheral Clock Divider Fraction"]
    #[inline] pub fn frac(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FRAC != 0"]
    #[inline] pub fn test_frac(&self) -> bool {
        self.frac() != 0
    }

    #[doc="Sets the FRAC field."]
    #[inline] pub fn set_frac<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Lptmr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Lptmr0(other)
    }
}

impl ::core::fmt::Display for Lptmr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lptmr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcd() != 0 { try!(write!(f, " pcd=0x{:x}", self.pcd()))}
        if self.frac() != 0 { try!(write!(f, " frac"))}
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 65"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy65(pub u32);
impl Pccdummy65 {
}

impl From<u32> for Pccdummy65 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy65(other)
    }
}

impl ::core::fmt::Display for Pccdummy65 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy65 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 66"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy66(pub u32);
impl Pccdummy66 {
}

impl From<u32> for Pccdummy66 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy66(other)
    }
}

impl ::core::fmt::Display for Pccdummy66 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy66 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 67"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy67(pub u32);
impl Pccdummy67 {
}

impl From<u32> for Pccdummy67 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy67(other)
    }
}

impl ::core::fmt::Display for Pccdummy67 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy67 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 68"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy68(pub u32);
impl Pccdummy68 {
}

impl From<u32> for Pccdummy68 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy68(other)
    }
}

impl ::core::fmt::Display for Pccdummy68 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy68 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 69"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy69(pub u32);
impl Pccdummy69 {
}

impl From<u32> for Pccdummy69 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy69(other)
    }
}

impl ::core::fmt::Display for Pccdummy69 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy69 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 70"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy70(pub u32);
impl Pccdummy70 {
}

impl From<u32> for Pccdummy70 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy70(other)
    }
}

impl ::core::fmt::Display for Pccdummy70 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy70 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 71"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy71(pub u32);
impl Pccdummy71 {
}

impl From<u32> for Pccdummy71 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy71(other)
    }
}

impl ::core::fmt::Display for Pccdummy71 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy71 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 72"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy72(pub u32);
impl Pccdummy72 {
}

impl From<u32> for Pccdummy72 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy72(other)
    }
}

impl ::core::fmt::Display for Pccdummy72 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy72 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC PORTA Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Porta(pub u32);
impl Porta {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Porta {
    #[inline]
    fn from(other: u32) -> Self {
         Porta(other)
    }
}

impl ::core::fmt::Display for Porta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Porta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC PORTB Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Portb(pub u32);
impl Portb {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Portb {
    #[inline]
    fn from(other: u32) -> Self {
         Portb(other)
    }
}

impl ::core::fmt::Display for Portb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Portb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC PORTC Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Portc(pub u32);
impl Portc {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Portc {
    #[inline]
    fn from(other: u32) -> Self {
         Portc(other)
    }
}

impl ::core::fmt::Display for Portc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Portc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC PORTD Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Portd(pub u32);
impl Portd {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Portd {
    #[inline]
    fn from(other: u32) -> Self {
         Portd(other)
    }
}

impl ::core::fmt::Display for Portd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Portd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC PORTE Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Porte(pub u32);
impl Porte {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Porte {
    #[inline]
    fn from(other: u32) -> Self {
         Porte(other)
    }
}

impl ::core::fmt::Display for Porte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Porte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 78"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy78(pub u32);
impl Pccdummy78 {
}

impl From<u32> for Pccdummy78 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy78(other)
    }
}

impl ::core::fmt::Display for Pccdummy78 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy78 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 79"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy79(pub u32);
impl Pccdummy79 {
}

impl From<u32> for Pccdummy79 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy79(other)
    }
}

impl ::core::fmt::Display for Pccdummy79 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy79 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 80"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy80(pub u32);
impl Pccdummy80 {
}

impl From<u32> for Pccdummy80 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy80(other)
    }
}

impl ::core::fmt::Display for Pccdummy80 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy80 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 81"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy81(pub u32);
impl Pccdummy81 {
}

impl From<u32> for Pccdummy81 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy81(other)
    }
}

impl ::core::fmt::Display for Pccdummy81 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy81 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 82"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy82(pub u32);
impl Pccdummy82 {
}

impl From<u32> for Pccdummy82 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy82(other)
    }
}

impl ::core::fmt::Display for Pccdummy82 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy82 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 83"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy83(pub u32);
impl Pccdummy83 {
}

impl From<u32> for Pccdummy83 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy83(other)
    }
}

impl ::core::fmt::Display for Pccdummy83 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy83 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 84"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy84(pub u32);
impl Pccdummy84 {
}

impl From<u32> for Pccdummy84 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy84(other)
    }
}

impl ::core::fmt::Display for Pccdummy84 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy84 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 85"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy85(pub u32);
impl Pccdummy85 {
}

impl From<u32> for Pccdummy85 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy85(other)
    }
}

impl ::core::fmt::Display for Pccdummy85 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy85 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 86"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy86(pub u32);
impl Pccdummy86 {
}

impl From<u32> for Pccdummy86 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy86(other)
    }
}

impl ::core::fmt::Display for Pccdummy86 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy86 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 87"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy87(pub u32);
impl Pccdummy87 {
}

impl From<u32> for Pccdummy87 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy87(other)
    }
}

impl ::core::fmt::Display for Pccdummy87 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy87 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 88"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy88(pub u32);
impl Pccdummy88 {
}

impl From<u32> for Pccdummy88 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy88(other)
    }
}

impl ::core::fmt::Display for Pccdummy88 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy88 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 89"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy89(pub u32);
impl Pccdummy89 {
}

impl From<u32> for Pccdummy89 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy89(other)
    }
}

impl ::core::fmt::Display for Pccdummy89 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy89 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC FlexIO Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flexio(pub u32);
impl Flexio {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Flexio {
    #[inline]
    fn from(other: u32) -> Self {
         Flexio(other)
    }
}

impl ::core::fmt::Display for Flexio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Flexio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 91"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy91(pub u32);
impl Pccdummy91 {
}

impl From<u32> for Pccdummy91 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy91(other)
    }
}

impl ::core::fmt::Display for Pccdummy91 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy91 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 92"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy92(pub u32);
impl Pccdummy92 {
}

impl From<u32> for Pccdummy92 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy92(other)
    }
}

impl ::core::fmt::Display for Pccdummy92 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy92 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 93"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy93(pub u32);
impl Pccdummy93 {
}

impl From<u32> for Pccdummy93 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy93(other)
    }
}

impl ::core::fmt::Display for Pccdummy93 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy93 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 94"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy94(pub u32);
impl Pccdummy94 {
}

impl From<u32> for Pccdummy94 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy94(other)
    }
}

impl ::core::fmt::Display for Pccdummy94 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy94 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 95"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy95(pub u32);
impl Pccdummy95 {
}

impl From<u32> for Pccdummy95 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy95(other)
    }
}

impl ::core::fmt::Display for Pccdummy95 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy95 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 96"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy96(pub u32);
impl Pccdummy96 {
}

impl From<u32> for Pccdummy96 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy96(other)
    }
}

impl ::core::fmt::Display for Pccdummy96 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy96 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC EWM Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ewm(pub u32);
impl Ewm {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ewm {
    #[inline]
    fn from(other: u32) -> Self {
         Ewm(other)
    }
}

impl ::core::fmt::Display for Ewm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ewm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 98"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy98(pub u32);
impl Pccdummy98 {
}

impl From<u32> for Pccdummy98 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy98(other)
    }
}

impl ::core::fmt::Display for Pccdummy98 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy98 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 99"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy99(pub u32);
impl Pccdummy99 {
}

impl From<u32> for Pccdummy99 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy99(other)
    }
}

impl ::core::fmt::Display for Pccdummy99 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy99 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 100"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy100(pub u32);
impl Pccdummy100 {
}

impl From<u32> for Pccdummy100 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy100(other)
    }
}

impl ::core::fmt::Display for Pccdummy100 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy100 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 101"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy101(pub u32);
impl Pccdummy101 {
}

impl From<u32> for Pccdummy101 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy101(other)
    }
}

impl ::core::fmt::Display for Pccdummy101 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy101 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC LPI2C0 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpi2c0(pub u32);
impl Lpi2c0 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Lpi2c0 {
    #[inline]
    fn from(other: u32) -> Self {
         Lpi2c0(other)
    }
}

impl ::core::fmt::Display for Lpi2c0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lpi2c0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 103"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy103(pub u32);
impl Pccdummy103 {
}

impl From<u32> for Pccdummy103 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy103(other)
    }
}

impl ::core::fmt::Display for Pccdummy103 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy103 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 104"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy104(pub u32);
impl Pccdummy104 {
}

impl From<u32> for Pccdummy104 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy104(other)
    }
}

impl ::core::fmt::Display for Pccdummy104 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy104 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 105"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy105(pub u32);
impl Pccdummy105 {
}

impl From<u32> for Pccdummy105 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy105(other)
    }
}

impl ::core::fmt::Display for Pccdummy105 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy105 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC LPUART0 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpuart0(pub u32);
impl Lpuart0 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Lpuart0 {
    #[inline]
    fn from(other: u32) -> Self {
         Lpuart0(other)
    }
}

impl ::core::fmt::Display for Lpuart0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lpuart0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC LPUART1 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpuart1(pub u32);
impl Lpuart1 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Lpuart1 {
    #[inline]
    fn from(other: u32) -> Self {
         Lpuart1(other)
    }
}

impl ::core::fmt::Display for Lpuart1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lpuart1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC LPUART2 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpuart2(pub u32);
impl Lpuart2 {
    #[doc="Peripheral Clock Source Select"]
    #[inline] pub fn pcs(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Lpuart2 {
    #[inline]
    fn from(other: u32) -> Self {
         Lpuart2(other)
    }
}

impl ::core::fmt::Display for Lpuart2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lpuart2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 109"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy109(pub u32);
impl Pccdummy109 {
}

impl From<u32> for Pccdummy109 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy109(other)
    }
}

impl ::core::fmt::Display for Pccdummy109 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy109 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 110"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy110(pub u32);
impl Pccdummy110 {
}

impl From<u32> for Pccdummy110 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy110(other)
    }
}

impl ::core::fmt::Display for Pccdummy110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 111"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy111(pub u32);
impl Pccdummy111 {
}

impl From<u32> for Pccdummy111 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy111(other)
    }
}

impl ::core::fmt::Display for Pccdummy111 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy111 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 112"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy112(pub u32);
impl Pccdummy112 {
}

impl From<u32> for Pccdummy112 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy112(other)
    }
}

impl ::core::fmt::Display for Pccdummy112 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy112 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 113"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy113(pub u32);
impl Pccdummy113 {
}

impl From<u32> for Pccdummy113 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy113(other)
    }
}

impl ::core::fmt::Display for Pccdummy113 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy113 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC Reserved Register 114"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pccdummy114(pub u32);
impl Pccdummy114 {
}

impl From<u32> for Pccdummy114 {
    #[inline]
    fn from(other: u32) -> Self {
         Pccdummy114(other)
    }
}

impl ::core::fmt::Display for Pccdummy114 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pccdummy114 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PCC CMP0 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmp0(pub u32);
impl Cmp0 {
    #[doc="Clock Gate Control"]
    #[inline] pub fn cgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CGC != 0"]
    #[inline] pub fn test_cgc(&self) -> bool {
        self.cgc() != 0
    }

    #[doc="Sets the CGC field."]
    #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Present"]
    #[inline] pub fn pr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Cmp0 {
    #[inline]
    fn from(other: u32) -> Self {
         Cmp0(other)
    }
}

impl ::core::fmt::Display for Cmp0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmp0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cgc() != 0 { try!(write!(f, " cgc"))}
        if self.pr() != 0 { try!(write!(f, " pr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub trait Cgc {
    fn cgc(&self) -> u32;
    fn set_cgc(&self, value: u32);
}

impl Pcc {
    #[inline] pub fn cgc<P: Cgc>(&self, p: &P) -> u32 {
        p.cgc()
    }
    #[inline] pub fn set_cgc<P: Cgc>(&self, p: &P, value: u32) {
        p.set_cgc(value)
    }
}

pub trait Pr {
    fn pr(&self) -> u32;
    fn set_pr(&self, value: u32);
}

impl Pcc {
    #[inline] pub fn pr<P: Pr>(&self, p: &P) -> u32 {
        p.pr()
    }
    #[inline] pub fn set_pr<P: Pr>(&self, p: &P, value: u32) {
        p.set_pr(value)
    }
}

pub trait Pcs {
    fn pcs(&self) -> u32;
    fn set_pcs(&self, value: u32);
}

impl Pcc {
    #[inline] pub fn pcs<P: Pcs>(&self, p: &P) -> u32 {
        p.pcs()
    }
    #[inline] pub fn set_pcs<P: Pcs>(&self, p: &P, value: u32) {
        p.set_pcs(value)
    }
}

impl Cgc for super::flexcan::Can0 {
    #[inline] fn cgc(&self) -> u32 { PCC.flexcan0().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_flexcan0(|r| r.set_cgc(value)); }
}

impl Cgc for super::flexcan::Can1 {
    #[inline] fn cgc(&self) -> u32 { PCC.flexcan1().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_flexcan1(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm3 {
    #[inline] fn pcs(&self) -> u32 { PCC.ftm3().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm3(|r| r.set_pcs(value)); }
}

impl Pcs for super::adc::Adc1 {
    #[inline] fn pcs(&self) -> u32 { PCC.adc1().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_adc1(|r| r.set_pcs(value)); }
}

impl Cgc for super::adc::Adc1 {
    #[inline] fn cgc(&self) -> u32 { PCC.adc1().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_adc1(|r| r.set_cgc(value)); }
}

impl Cgc for super::flexcan::Can2 {
    #[inline] fn cgc(&self) -> u32 { PCC.flexcan2().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_flexcan2(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi0 {
    #[inline] fn pcs(&self) -> u32 { PCC.lpspi0().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpspi0(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi0 {
    #[inline] fn cgc(&self) -> u32 { PCC.lpspi0().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpspi0(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi1 {
    #[inline] fn pcs(&self) -> u32 { PCC.lpspi1().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpspi1(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi1 {
    #[inline] fn cgc(&self) -> u32 { PCC.lpspi1().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpspi1(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi2 {
    #[inline] fn pcs(&self) -> u32 { PCC.lpspi2().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpspi2(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi2 {
    #[inline] fn cgc(&self) -> u32 { PCC.lpspi2().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpspi2(|r| r.set_cgc(value)); }
}

impl Cgc for super::crc::Crc {
    #[inline] fn cgc(&self) -> u32 { PCC.crc().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_crc(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpit::Lpit0 {
    #[inline] fn pcs(&self) -> u32 { PCC.lpit().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpit(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpit::Lpit0 {
    #[inline] fn cgc(&self) -> u32 { PCC.lpit().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpit(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm0 {
    #[inline] fn pcs(&self) -> u32 { PCC.ftm0().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm0(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm0 {
    #[inline] fn cgc(&self) -> u32 { PCC.ftm0().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_ftm0(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm1 {
    #[inline] fn pcs(&self) -> u32 { PCC.ftm1().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm1(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm1 {
    #[inline] fn cgc(&self) -> u32 { PCC.ftm1().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_ftm1(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm2 {
    #[inline] fn pcs(&self) -> u32 { PCC.ftm2().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm2(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm2 {
    #[inline] fn cgc(&self) -> u32 { PCC.ftm2().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_ftm2(|r| r.set_cgc(value)); }
}

impl Pcs for super::adc::Adc0 {
    #[inline] fn pcs(&self) -> u32 { PCC.adc0().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_adc0(|r| r.set_pcs(value)); }
}

impl Cgc for super::adc::Adc0 {
    #[inline] fn cgc(&self) -> u32 { PCC.adc0().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_adc0(|r| r.set_cgc(value)); }
}

impl Cgc for super::rtc::Rtc {
    #[inline] fn cgc(&self) -> u32 { PCC.rtc().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_rtc(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Porta {
    #[inline] fn cgc(&self) -> u32 { PCC.porta().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_porta(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portb {
    #[inline] fn cgc(&self) -> u32 { PCC.portb().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_portb(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portc {
    #[inline] fn cgc(&self) -> u32 { PCC.portc().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_portc(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portd {
    #[inline] fn cgc(&self) -> u32 { PCC.portd().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_portd(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Porte {
    #[inline] fn cgc(&self) -> u32 { PCC.porte().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_porte(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart0 {
    #[inline] fn pcs(&self) -> u32 { PCC.lpuart0().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpuart0(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart0 {
    #[inline] fn cgc(&self) -> u32 { PCC.lpuart0().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpuart0(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart1 {
    #[inline] fn pcs(&self) -> u32 { PCC.lpuart1().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpuart1(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart1 {
    #[inline] fn cgc(&self) -> u32 { PCC.lpuart1().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpuart1(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart2 {
    #[inline] fn pcs(&self) -> u32 { PCC.lpuart2().pcs().into() }
    #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpuart2(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart2 {
    #[inline] fn cgc(&self) -> u32 { PCC.lpuart2().cgc().into() }
    #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpuart2(|r| r.set_cgc(value)); }
}


