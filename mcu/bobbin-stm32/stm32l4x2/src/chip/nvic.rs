#[allow(unused_imports)] use bobbin_common::*;

periph!( NVIC, Nvic, _NVIC, NvicPeriph, 0xe000e000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="NVIC Peripheral"]
pub struct NvicPeriph(pub usize); 



impl NvicPeriph {
    #[doc="Get the *mut pointer for the ICTR register."]
    #[inline] pub fn ictr_mut(&self) -> *mut Ictr { 
        (self.0 + 0x4) as *mut Ictr
    }

    #[doc="Get the *const pointer for the ICTR register."]
    #[inline] pub fn ictr_ptr(&self) -> *const Ictr { 
           self.ictr_mut()
    }

    #[doc="Read the ICTR register."]
    #[inline] pub fn ictr(&self) -> Ictr { 
        unsafe {
            read_volatile(self.ictr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the STIR register."]
    #[inline] pub fn stir_mut(&self) -> *mut Stir { 
        (self.0 + 0xf00) as *mut Stir
    }

    #[doc="Get the *const pointer for the STIR register."]
    #[inline] pub fn stir_ptr(&self) -> *const Stir { 
           self.stir_mut()
    }

    #[doc="Write the STIR register."]
    #[inline] pub fn set_stir<F: FnOnce(Stir) -> Stir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stir_mut(), f(Stir(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISER0 register."]
    #[inline] pub fn iser0_mut(&self) -> *mut Iser0 { 
        (self.0 + 0x100) as *mut Iser0
    }

    #[doc="Get the *const pointer for the ISER0 register."]
    #[inline] pub fn iser0_ptr(&self) -> *const Iser0 { 
           self.iser0_mut()
    }

    #[doc="Read the ISER0 register."]
    #[inline] pub fn iser0(&self) -> Iser0 { 
        unsafe {
            read_volatile(self.iser0_ptr())
        }
    }

    #[doc="Write the ISER0 register."]
    #[inline] pub fn set_iser0<F: FnOnce(Iser0) -> Iser0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iser0_mut(), f(Iser0(0)));
        }
        self
    }

    #[doc="Modify the ISER0 register."]
    #[inline] pub fn with_iser0<F: FnOnce(Iser0) -> Iser0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iser0_mut(), f(self.iser0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISER1 register."]
    #[inline] pub fn iser1_mut(&self) -> *mut Iser1 { 
        (self.0 + 0x104) as *mut Iser1
    }

    #[doc="Get the *const pointer for the ISER1 register."]
    #[inline] pub fn iser1_ptr(&self) -> *const Iser1 { 
           self.iser1_mut()
    }

    #[doc="Read the ISER1 register."]
    #[inline] pub fn iser1(&self) -> Iser1 { 
        unsafe {
            read_volatile(self.iser1_ptr())
        }
    }

    #[doc="Write the ISER1 register."]
    #[inline] pub fn set_iser1<F: FnOnce(Iser1) -> Iser1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iser1_mut(), f(Iser1(0)));
        }
        self
    }

    #[doc="Modify the ISER1 register."]
    #[inline] pub fn with_iser1<F: FnOnce(Iser1) -> Iser1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iser1_mut(), f(self.iser1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISER2 register."]
    #[inline] pub fn iser2_mut(&self) -> *mut Iser2 { 
        (self.0 + 0x108) as *mut Iser2
    }

    #[doc="Get the *const pointer for the ISER2 register."]
    #[inline] pub fn iser2_ptr(&self) -> *const Iser2 { 
           self.iser2_mut()
    }

    #[doc="Read the ISER2 register."]
    #[inline] pub fn iser2(&self) -> Iser2 { 
        unsafe {
            read_volatile(self.iser2_ptr())
        }
    }

    #[doc="Write the ISER2 register."]
    #[inline] pub fn set_iser2<F: FnOnce(Iser2) -> Iser2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iser2_mut(), f(Iser2(0)));
        }
        self
    }

    #[doc="Modify the ISER2 register."]
    #[inline] pub fn with_iser2<F: FnOnce(Iser2) -> Iser2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iser2_mut(), f(self.iser2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICER0 register."]
    #[inline] pub fn icer0_mut(&self) -> *mut Icer0 { 
        (self.0 + 0x180) as *mut Icer0
    }

    #[doc="Get the *const pointer for the ICER0 register."]
    #[inline] pub fn icer0_ptr(&self) -> *const Icer0 { 
           self.icer0_mut()
    }

    #[doc="Read the ICER0 register."]
    #[inline] pub fn icer0(&self) -> Icer0 { 
        unsafe {
            read_volatile(self.icer0_ptr())
        }
    }

    #[doc="Write the ICER0 register."]
    #[inline] pub fn set_icer0<F: FnOnce(Icer0) -> Icer0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icer0_mut(), f(Icer0(0)));
        }
        self
    }

    #[doc="Modify the ICER0 register."]
    #[inline] pub fn with_icer0<F: FnOnce(Icer0) -> Icer0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icer0_mut(), f(self.icer0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICER1 register."]
    #[inline] pub fn icer1_mut(&self) -> *mut Icer1 { 
        (self.0 + 0x184) as *mut Icer1
    }

    #[doc="Get the *const pointer for the ICER1 register."]
    #[inline] pub fn icer1_ptr(&self) -> *const Icer1 { 
           self.icer1_mut()
    }

    #[doc="Read the ICER1 register."]
    #[inline] pub fn icer1(&self) -> Icer1 { 
        unsafe {
            read_volatile(self.icer1_ptr())
        }
    }

    #[doc="Write the ICER1 register."]
    #[inline] pub fn set_icer1<F: FnOnce(Icer1) -> Icer1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icer1_mut(), f(Icer1(0)));
        }
        self
    }

    #[doc="Modify the ICER1 register."]
    #[inline] pub fn with_icer1<F: FnOnce(Icer1) -> Icer1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icer1_mut(), f(self.icer1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICER2 register."]
    #[inline] pub fn icer2_mut(&self) -> *mut Icer2 { 
        (self.0 + 0x188) as *mut Icer2
    }

    #[doc="Get the *const pointer for the ICER2 register."]
    #[inline] pub fn icer2_ptr(&self) -> *const Icer2 { 
           self.icer2_mut()
    }

    #[doc="Read the ICER2 register."]
    #[inline] pub fn icer2(&self) -> Icer2 { 
        unsafe {
            read_volatile(self.icer2_ptr())
        }
    }

    #[doc="Write the ICER2 register."]
    #[inline] pub fn set_icer2<F: FnOnce(Icer2) -> Icer2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icer2_mut(), f(Icer2(0)));
        }
        self
    }

    #[doc="Modify the ICER2 register."]
    #[inline] pub fn with_icer2<F: FnOnce(Icer2) -> Icer2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icer2_mut(), f(self.icer2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISPR0 register."]
    #[inline] pub fn ispr0_mut(&self) -> *mut Ispr0 { 
        (self.0 + 0x200) as *mut Ispr0
    }

    #[doc="Get the *const pointer for the ISPR0 register."]
    #[inline] pub fn ispr0_ptr(&self) -> *const Ispr0 { 
           self.ispr0_mut()
    }

    #[doc="Read the ISPR0 register."]
    #[inline] pub fn ispr0(&self) -> Ispr0 { 
        unsafe {
            read_volatile(self.ispr0_ptr())
        }
    }

    #[doc="Write the ISPR0 register."]
    #[inline] pub fn set_ispr0<F: FnOnce(Ispr0) -> Ispr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ispr0_mut(), f(Ispr0(0)));
        }
        self
    }

    #[doc="Modify the ISPR0 register."]
    #[inline] pub fn with_ispr0<F: FnOnce(Ispr0) -> Ispr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ispr0_mut(), f(self.ispr0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISPR1 register."]
    #[inline] pub fn ispr1_mut(&self) -> *mut Ispr1 { 
        (self.0 + 0x204) as *mut Ispr1
    }

    #[doc="Get the *const pointer for the ISPR1 register."]
    #[inline] pub fn ispr1_ptr(&self) -> *const Ispr1 { 
           self.ispr1_mut()
    }

    #[doc="Read the ISPR1 register."]
    #[inline] pub fn ispr1(&self) -> Ispr1 { 
        unsafe {
            read_volatile(self.ispr1_ptr())
        }
    }

    #[doc="Write the ISPR1 register."]
    #[inline] pub fn set_ispr1<F: FnOnce(Ispr1) -> Ispr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ispr1_mut(), f(Ispr1(0)));
        }
        self
    }

    #[doc="Modify the ISPR1 register."]
    #[inline] pub fn with_ispr1<F: FnOnce(Ispr1) -> Ispr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ispr1_mut(), f(self.ispr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISPR2 register."]
    #[inline] pub fn ispr2_mut(&self) -> *mut Ispr2 { 
        (self.0 + 0x208) as *mut Ispr2
    }

    #[doc="Get the *const pointer for the ISPR2 register."]
    #[inline] pub fn ispr2_ptr(&self) -> *const Ispr2 { 
           self.ispr2_mut()
    }

    #[doc="Read the ISPR2 register."]
    #[inline] pub fn ispr2(&self) -> Ispr2 { 
        unsafe {
            read_volatile(self.ispr2_ptr())
        }
    }

    #[doc="Write the ISPR2 register."]
    #[inline] pub fn set_ispr2<F: FnOnce(Ispr2) -> Ispr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ispr2_mut(), f(Ispr2(0)));
        }
        self
    }

    #[doc="Modify the ISPR2 register."]
    #[inline] pub fn with_ispr2<F: FnOnce(Ispr2) -> Ispr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ispr2_mut(), f(self.ispr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICPR0 register."]
    #[inline] pub fn icpr0_mut(&self) -> *mut Icpr0 { 
        (self.0 + 0x280) as *mut Icpr0
    }

    #[doc="Get the *const pointer for the ICPR0 register."]
    #[inline] pub fn icpr0_ptr(&self) -> *const Icpr0 { 
           self.icpr0_mut()
    }

    #[doc="Read the ICPR0 register."]
    #[inline] pub fn icpr0(&self) -> Icpr0 { 
        unsafe {
            read_volatile(self.icpr0_ptr())
        }
    }

    #[doc="Write the ICPR0 register."]
    #[inline] pub fn set_icpr0<F: FnOnce(Icpr0) -> Icpr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icpr0_mut(), f(Icpr0(0)));
        }
        self
    }

    #[doc="Modify the ICPR0 register."]
    #[inline] pub fn with_icpr0<F: FnOnce(Icpr0) -> Icpr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icpr0_mut(), f(self.icpr0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICPR1 register."]
    #[inline] pub fn icpr1_mut(&self) -> *mut Icpr1 { 
        (self.0 + 0x284) as *mut Icpr1
    }

    #[doc="Get the *const pointer for the ICPR1 register."]
    #[inline] pub fn icpr1_ptr(&self) -> *const Icpr1 { 
           self.icpr1_mut()
    }

    #[doc="Read the ICPR1 register."]
    #[inline] pub fn icpr1(&self) -> Icpr1 { 
        unsafe {
            read_volatile(self.icpr1_ptr())
        }
    }

    #[doc="Write the ICPR1 register."]
    #[inline] pub fn set_icpr1<F: FnOnce(Icpr1) -> Icpr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icpr1_mut(), f(Icpr1(0)));
        }
        self
    }

    #[doc="Modify the ICPR1 register."]
    #[inline] pub fn with_icpr1<F: FnOnce(Icpr1) -> Icpr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icpr1_mut(), f(self.icpr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICPR2 register."]
    #[inline] pub fn icpr2_mut(&self) -> *mut Icpr2 { 
        (self.0 + 0x288) as *mut Icpr2
    }

    #[doc="Get the *const pointer for the ICPR2 register."]
    #[inline] pub fn icpr2_ptr(&self) -> *const Icpr2 { 
           self.icpr2_mut()
    }

    #[doc="Read the ICPR2 register."]
    #[inline] pub fn icpr2(&self) -> Icpr2 { 
        unsafe {
            read_volatile(self.icpr2_ptr())
        }
    }

    #[doc="Write the ICPR2 register."]
    #[inline] pub fn set_icpr2<F: FnOnce(Icpr2) -> Icpr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icpr2_mut(), f(Icpr2(0)));
        }
        self
    }

    #[doc="Modify the ICPR2 register."]
    #[inline] pub fn with_icpr2<F: FnOnce(Icpr2) -> Icpr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icpr2_mut(), f(self.icpr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IABR0 register."]
    #[inline] pub fn iabr0_mut(&self) -> *mut Iabr0 { 
        (self.0 + 0x300) as *mut Iabr0
    }

    #[doc="Get the *const pointer for the IABR0 register."]
    #[inline] pub fn iabr0_ptr(&self) -> *const Iabr0 { 
           self.iabr0_mut()
    }

    #[doc="Read the IABR0 register."]
    #[inline] pub fn iabr0(&self) -> Iabr0 { 
        unsafe {
            read_volatile(self.iabr0_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IABR1 register."]
    #[inline] pub fn iabr1_mut(&self) -> *mut Iabr1 { 
        (self.0 + 0x304) as *mut Iabr1
    }

    #[doc="Get the *const pointer for the IABR1 register."]
    #[inline] pub fn iabr1_ptr(&self) -> *const Iabr1 { 
           self.iabr1_mut()
    }

    #[doc="Read the IABR1 register."]
    #[inline] pub fn iabr1(&self) -> Iabr1 { 
        unsafe {
            read_volatile(self.iabr1_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IABR2 register."]
    #[inline] pub fn iabr2_mut(&self) -> *mut Iabr2 { 
        (self.0 + 0x308) as *mut Iabr2
    }

    #[doc="Get the *const pointer for the IABR2 register."]
    #[inline] pub fn iabr2_ptr(&self) -> *const Iabr2 { 
           self.iabr2_mut()
    }

    #[doc="Read the IABR2 register."]
    #[inline] pub fn iabr2(&self) -> Iabr2 { 
        unsafe {
            read_volatile(self.iabr2_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IPR0 register."]
    #[inline] pub fn ipr0_mut(&self) -> *mut Ipr0 { 
        (self.0 + 0x400) as *mut Ipr0
    }

    #[doc="Get the *const pointer for the IPR0 register."]
    #[inline] pub fn ipr0_ptr(&self) -> *const Ipr0 { 
           self.ipr0_mut()
    }

    #[doc="Read the IPR0 register."]
    #[inline] pub fn ipr0(&self) -> Ipr0 { 
        unsafe {
            read_volatile(self.ipr0_ptr())
        }
    }

    #[doc="Write the IPR0 register."]
    #[inline] pub fn set_ipr0<F: FnOnce(Ipr0) -> Ipr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr0_mut(), f(Ipr0(0)));
        }
        self
    }

    #[doc="Modify the IPR0 register."]
    #[inline] pub fn with_ipr0<F: FnOnce(Ipr0) -> Ipr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr0_mut(), f(self.ipr0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR1 register."]
    #[inline] pub fn ipr1_mut(&self) -> *mut Ipr1 { 
        (self.0 + 0x404) as *mut Ipr1
    }

    #[doc="Get the *const pointer for the IPR1 register."]
    #[inline] pub fn ipr1_ptr(&self) -> *const Ipr1 { 
           self.ipr1_mut()
    }

    #[doc="Read the IPR1 register."]
    #[inline] pub fn ipr1(&self) -> Ipr1 { 
        unsafe {
            read_volatile(self.ipr1_ptr())
        }
    }

    #[doc="Write the IPR1 register."]
    #[inline] pub fn set_ipr1<F: FnOnce(Ipr1) -> Ipr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr1_mut(), f(Ipr1(0)));
        }
        self
    }

    #[doc="Modify the IPR1 register."]
    #[inline] pub fn with_ipr1<F: FnOnce(Ipr1) -> Ipr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr1_mut(), f(self.ipr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR2 register."]
    #[inline] pub fn ipr2_mut(&self) -> *mut Ipr2 { 
        (self.0 + 0x408) as *mut Ipr2
    }

    #[doc="Get the *const pointer for the IPR2 register."]
    #[inline] pub fn ipr2_ptr(&self) -> *const Ipr2 { 
           self.ipr2_mut()
    }

    #[doc="Read the IPR2 register."]
    #[inline] pub fn ipr2(&self) -> Ipr2 { 
        unsafe {
            read_volatile(self.ipr2_ptr())
        }
    }

    #[doc="Write the IPR2 register."]
    #[inline] pub fn set_ipr2<F: FnOnce(Ipr2) -> Ipr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr2_mut(), f(Ipr2(0)));
        }
        self
    }

    #[doc="Modify the IPR2 register."]
    #[inline] pub fn with_ipr2<F: FnOnce(Ipr2) -> Ipr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr2_mut(), f(self.ipr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR3 register."]
    #[inline] pub fn ipr3_mut(&self) -> *mut Ipr3 { 
        (self.0 + 0x40c) as *mut Ipr3
    }

    #[doc="Get the *const pointer for the IPR3 register."]
    #[inline] pub fn ipr3_ptr(&self) -> *const Ipr3 { 
           self.ipr3_mut()
    }

    #[doc="Read the IPR3 register."]
    #[inline] pub fn ipr3(&self) -> Ipr3 { 
        unsafe {
            read_volatile(self.ipr3_ptr())
        }
    }

    #[doc="Write the IPR3 register."]
    #[inline] pub fn set_ipr3<F: FnOnce(Ipr3) -> Ipr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr3_mut(), f(Ipr3(0)));
        }
        self
    }

    #[doc="Modify the IPR3 register."]
    #[inline] pub fn with_ipr3<F: FnOnce(Ipr3) -> Ipr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr3_mut(), f(self.ipr3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR4 register."]
    #[inline] pub fn ipr4_mut(&self) -> *mut Ipr4 { 
        (self.0 + 0x410) as *mut Ipr4
    }

    #[doc="Get the *const pointer for the IPR4 register."]
    #[inline] pub fn ipr4_ptr(&self) -> *const Ipr4 { 
           self.ipr4_mut()
    }

    #[doc="Read the IPR4 register."]
    #[inline] pub fn ipr4(&self) -> Ipr4 { 
        unsafe {
            read_volatile(self.ipr4_ptr())
        }
    }

    #[doc="Write the IPR4 register."]
    #[inline] pub fn set_ipr4<F: FnOnce(Ipr4) -> Ipr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr4_mut(), f(Ipr4(0)));
        }
        self
    }

    #[doc="Modify the IPR4 register."]
    #[inline] pub fn with_ipr4<F: FnOnce(Ipr4) -> Ipr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr4_mut(), f(self.ipr4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR5 register."]
    #[inline] pub fn ipr5_mut(&self) -> *mut Ipr5 { 
        (self.0 + 0x414) as *mut Ipr5
    }

    #[doc="Get the *const pointer for the IPR5 register."]
    #[inline] pub fn ipr5_ptr(&self) -> *const Ipr5 { 
           self.ipr5_mut()
    }

    #[doc="Read the IPR5 register."]
    #[inline] pub fn ipr5(&self) -> Ipr5 { 
        unsafe {
            read_volatile(self.ipr5_ptr())
        }
    }

    #[doc="Write the IPR5 register."]
    #[inline] pub fn set_ipr5<F: FnOnce(Ipr5) -> Ipr5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr5_mut(), f(Ipr5(0)));
        }
        self
    }

    #[doc="Modify the IPR5 register."]
    #[inline] pub fn with_ipr5<F: FnOnce(Ipr5) -> Ipr5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr5_mut(), f(self.ipr5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR6 register."]
    #[inline] pub fn ipr6_mut(&self) -> *mut Ipr6 { 
        (self.0 + 0x418) as *mut Ipr6
    }

    #[doc="Get the *const pointer for the IPR6 register."]
    #[inline] pub fn ipr6_ptr(&self) -> *const Ipr6 { 
           self.ipr6_mut()
    }

    #[doc="Read the IPR6 register."]
    #[inline] pub fn ipr6(&self) -> Ipr6 { 
        unsafe {
            read_volatile(self.ipr6_ptr())
        }
    }

    #[doc="Write the IPR6 register."]
    #[inline] pub fn set_ipr6<F: FnOnce(Ipr6) -> Ipr6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr6_mut(), f(Ipr6(0)));
        }
        self
    }

    #[doc="Modify the IPR6 register."]
    #[inline] pub fn with_ipr6<F: FnOnce(Ipr6) -> Ipr6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr6_mut(), f(self.ipr6()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR7 register."]
    #[inline] pub fn ipr7_mut(&self) -> *mut Ipr7 { 
        (self.0 + 0x41c) as *mut Ipr7
    }

    #[doc="Get the *const pointer for the IPR7 register."]
    #[inline] pub fn ipr7_ptr(&self) -> *const Ipr7 { 
           self.ipr7_mut()
    }

    #[doc="Read the IPR7 register."]
    #[inline] pub fn ipr7(&self) -> Ipr7 { 
        unsafe {
            read_volatile(self.ipr7_ptr())
        }
    }

    #[doc="Write the IPR7 register."]
    #[inline] pub fn set_ipr7<F: FnOnce(Ipr7) -> Ipr7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr7_mut(), f(Ipr7(0)));
        }
        self
    }

    #[doc="Modify the IPR7 register."]
    #[inline] pub fn with_ipr7<F: FnOnce(Ipr7) -> Ipr7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr7_mut(), f(self.ipr7()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR8 register."]
    #[inline] pub fn ipr8_mut(&self) -> *mut Ipr8 { 
        (self.0 + 0x420) as *mut Ipr8
    }

    #[doc="Get the *const pointer for the IPR8 register."]
    #[inline] pub fn ipr8_ptr(&self) -> *const Ipr8 { 
           self.ipr8_mut()
    }

    #[doc="Read the IPR8 register."]
    #[inline] pub fn ipr8(&self) -> Ipr8 { 
        unsafe {
            read_volatile(self.ipr8_ptr())
        }
    }

    #[doc="Write the IPR8 register."]
    #[inline] pub fn set_ipr8<F: FnOnce(Ipr8) -> Ipr8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr8_mut(), f(Ipr8(0)));
        }
        self
    }

    #[doc="Modify the IPR8 register."]
    #[inline] pub fn with_ipr8<F: FnOnce(Ipr8) -> Ipr8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr8_mut(), f(self.ipr8()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR9 register."]
    #[inline] pub fn ipr9_mut(&self) -> *mut Ipr9 { 
        (self.0 + 0x424) as *mut Ipr9
    }

    #[doc="Get the *const pointer for the IPR9 register."]
    #[inline] pub fn ipr9_ptr(&self) -> *const Ipr9 { 
           self.ipr9_mut()
    }

    #[doc="Read the IPR9 register."]
    #[inline] pub fn ipr9(&self) -> Ipr9 { 
        unsafe {
            read_volatile(self.ipr9_ptr())
        }
    }

    #[doc="Write the IPR9 register."]
    #[inline] pub fn set_ipr9<F: FnOnce(Ipr9) -> Ipr9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr9_mut(), f(Ipr9(0)));
        }
        self
    }

    #[doc="Modify the IPR9 register."]
    #[inline] pub fn with_ipr9<F: FnOnce(Ipr9) -> Ipr9>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr9_mut(), f(self.ipr9()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR10 register."]
    #[inline] pub fn ipr10_mut(&self) -> *mut Ipr10 { 
        (self.0 + 0x428) as *mut Ipr10
    }

    #[doc="Get the *const pointer for the IPR10 register."]
    #[inline] pub fn ipr10_ptr(&self) -> *const Ipr10 { 
           self.ipr10_mut()
    }

    #[doc="Read the IPR10 register."]
    #[inline] pub fn ipr10(&self) -> Ipr10 { 
        unsafe {
            read_volatile(self.ipr10_ptr())
        }
    }

    #[doc="Write the IPR10 register."]
    #[inline] pub fn set_ipr10<F: FnOnce(Ipr10) -> Ipr10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr10_mut(), f(Ipr10(0)));
        }
        self
    }

    #[doc="Modify the IPR10 register."]
    #[inline] pub fn with_ipr10<F: FnOnce(Ipr10) -> Ipr10>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr10_mut(), f(self.ipr10()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR11 register."]
    #[inline] pub fn ipr11_mut(&self) -> *mut Ipr11 { 
        (self.0 + 0x42c) as *mut Ipr11
    }

    #[doc="Get the *const pointer for the IPR11 register."]
    #[inline] pub fn ipr11_ptr(&self) -> *const Ipr11 { 
           self.ipr11_mut()
    }

    #[doc="Read the IPR11 register."]
    #[inline] pub fn ipr11(&self) -> Ipr11 { 
        unsafe {
            read_volatile(self.ipr11_ptr())
        }
    }

    #[doc="Write the IPR11 register."]
    #[inline] pub fn set_ipr11<F: FnOnce(Ipr11) -> Ipr11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr11_mut(), f(Ipr11(0)));
        }
        self
    }

    #[doc="Modify the IPR11 register."]
    #[inline] pub fn with_ipr11<F: FnOnce(Ipr11) -> Ipr11>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr11_mut(), f(self.ipr11()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR12 register."]
    #[inline] pub fn ipr12_mut(&self) -> *mut Ipr12 { 
        (self.0 + 0x430) as *mut Ipr12
    }

    #[doc="Get the *const pointer for the IPR12 register."]
    #[inline] pub fn ipr12_ptr(&self) -> *const Ipr12 { 
           self.ipr12_mut()
    }

    #[doc="Read the IPR12 register."]
    #[inline] pub fn ipr12(&self) -> Ipr12 { 
        unsafe {
            read_volatile(self.ipr12_ptr())
        }
    }

    #[doc="Write the IPR12 register."]
    #[inline] pub fn set_ipr12<F: FnOnce(Ipr12) -> Ipr12>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr12_mut(), f(Ipr12(0)));
        }
        self
    }

    #[doc="Modify the IPR12 register."]
    #[inline] pub fn with_ipr12<F: FnOnce(Ipr12) -> Ipr12>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr12_mut(), f(self.ipr12()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR13 register."]
    #[inline] pub fn ipr13_mut(&self) -> *mut Ipr13 { 
        (self.0 + 0x434) as *mut Ipr13
    }

    #[doc="Get the *const pointer for the IPR13 register."]
    #[inline] pub fn ipr13_ptr(&self) -> *const Ipr13 { 
           self.ipr13_mut()
    }

    #[doc="Read the IPR13 register."]
    #[inline] pub fn ipr13(&self) -> Ipr13 { 
        unsafe {
            read_volatile(self.ipr13_ptr())
        }
    }

    #[doc="Write the IPR13 register."]
    #[inline] pub fn set_ipr13<F: FnOnce(Ipr13) -> Ipr13>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr13_mut(), f(Ipr13(0)));
        }
        self
    }

    #[doc="Modify the IPR13 register."]
    #[inline] pub fn with_ipr13<F: FnOnce(Ipr13) -> Ipr13>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr13_mut(), f(self.ipr13()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR14 register."]
    #[inline] pub fn ipr14_mut(&self) -> *mut Ipr14 { 
        (self.0 + 0x438) as *mut Ipr14
    }

    #[doc="Get the *const pointer for the IPR14 register."]
    #[inline] pub fn ipr14_ptr(&self) -> *const Ipr14 { 
           self.ipr14_mut()
    }

    #[doc="Read the IPR14 register."]
    #[inline] pub fn ipr14(&self) -> Ipr14 { 
        unsafe {
            read_volatile(self.ipr14_ptr())
        }
    }

    #[doc="Write the IPR14 register."]
    #[inline] pub fn set_ipr14<F: FnOnce(Ipr14) -> Ipr14>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr14_mut(), f(Ipr14(0)));
        }
        self
    }

    #[doc="Modify the IPR14 register."]
    #[inline] pub fn with_ipr14<F: FnOnce(Ipr14) -> Ipr14>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr14_mut(), f(self.ipr14()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR15 register."]
    #[inline] pub fn ipr15_mut(&self) -> *mut Ipr15 { 
        (self.0 + 0x43c) as *mut Ipr15
    }

    #[doc="Get the *const pointer for the IPR15 register."]
    #[inline] pub fn ipr15_ptr(&self) -> *const Ipr15 { 
           self.ipr15_mut()
    }

    #[doc="Read the IPR15 register."]
    #[inline] pub fn ipr15(&self) -> Ipr15 { 
        unsafe {
            read_volatile(self.ipr15_ptr())
        }
    }

    #[doc="Write the IPR15 register."]
    #[inline] pub fn set_ipr15<F: FnOnce(Ipr15) -> Ipr15>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr15_mut(), f(Ipr15(0)));
        }
        self
    }

    #[doc="Modify the IPR15 register."]
    #[inline] pub fn with_ipr15<F: FnOnce(Ipr15) -> Ipr15>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr15_mut(), f(self.ipr15()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR16 register."]
    #[inline] pub fn ipr16_mut(&self) -> *mut Ipr16 { 
        (self.0 + 0x440) as *mut Ipr16
    }

    #[doc="Get the *const pointer for the IPR16 register."]
    #[inline] pub fn ipr16_ptr(&self) -> *const Ipr16 { 
           self.ipr16_mut()
    }

    #[doc="Read the IPR16 register."]
    #[inline] pub fn ipr16(&self) -> Ipr16 { 
        unsafe {
            read_volatile(self.ipr16_ptr())
        }
    }

    #[doc="Write the IPR16 register."]
    #[inline] pub fn set_ipr16<F: FnOnce(Ipr16) -> Ipr16>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr16_mut(), f(Ipr16(0)));
        }
        self
    }

    #[doc="Modify the IPR16 register."]
    #[inline] pub fn with_ipr16<F: FnOnce(Ipr16) -> Ipr16>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr16_mut(), f(self.ipr16()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR17 register."]
    #[inline] pub fn ipr17_mut(&self) -> *mut Ipr17 { 
        (self.0 + 0x444) as *mut Ipr17
    }

    #[doc="Get the *const pointer for the IPR17 register."]
    #[inline] pub fn ipr17_ptr(&self) -> *const Ipr17 { 
           self.ipr17_mut()
    }

    #[doc="Read the IPR17 register."]
    #[inline] pub fn ipr17(&self) -> Ipr17 { 
        unsafe {
            read_volatile(self.ipr17_ptr())
        }
    }

    #[doc="Write the IPR17 register."]
    #[inline] pub fn set_ipr17<F: FnOnce(Ipr17) -> Ipr17>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr17_mut(), f(Ipr17(0)));
        }
        self
    }

    #[doc="Modify the IPR17 register."]
    #[inline] pub fn with_ipr17<F: FnOnce(Ipr17) -> Ipr17>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr17_mut(), f(self.ipr17()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR18 register."]
    #[inline] pub fn ipr18_mut(&self) -> *mut Ipr18 { 
        (self.0 + 0x448) as *mut Ipr18
    }

    #[doc="Get the *const pointer for the IPR18 register."]
    #[inline] pub fn ipr18_ptr(&self) -> *const Ipr18 { 
           self.ipr18_mut()
    }

    #[doc="Read the IPR18 register."]
    #[inline] pub fn ipr18(&self) -> Ipr18 { 
        unsafe {
            read_volatile(self.ipr18_ptr())
        }
    }

    #[doc="Write the IPR18 register."]
    #[inline] pub fn set_ipr18<F: FnOnce(Ipr18) -> Ipr18>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr18_mut(), f(Ipr18(0)));
        }
        self
    }

    #[doc="Modify the IPR18 register."]
    #[inline] pub fn with_ipr18<F: FnOnce(Ipr18) -> Ipr18>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr18_mut(), f(self.ipr18()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR19 register."]
    #[inline] pub fn ipr19_mut(&self) -> *mut Ipr19 { 
        (self.0 + 0x44c) as *mut Ipr19
    }

    #[doc="Get the *const pointer for the IPR19 register."]
    #[inline] pub fn ipr19_ptr(&self) -> *const Ipr19 { 
           self.ipr19_mut()
    }

    #[doc="Read the IPR19 register."]
    #[inline] pub fn ipr19(&self) -> Ipr19 { 
        unsafe {
            read_volatile(self.ipr19_ptr())
        }
    }

    #[doc="Write the IPR19 register."]
    #[inline] pub fn set_ipr19<F: FnOnce(Ipr19) -> Ipr19>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr19_mut(), f(Ipr19(0)));
        }
        self
    }

    #[doc="Modify the IPR19 register."]
    #[inline] pub fn with_ipr19<F: FnOnce(Ipr19) -> Ipr19>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr19_mut(), f(self.ipr19()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IPR20 register."]
    #[inline] pub fn ipr20_mut(&self) -> *mut Ipr20 { 
        (self.0 + 0x450) as *mut Ipr20
    }

    #[doc="Get the *const pointer for the IPR20 register."]
    #[inline] pub fn ipr20_ptr(&self) -> *const Ipr20 { 
           self.ipr20_mut()
    }

    #[doc="Read the IPR20 register."]
    #[inline] pub fn ipr20(&self) -> Ipr20 { 
        unsafe {
            read_volatile(self.ipr20_ptr())
        }
    }

    #[doc="Write the IPR20 register."]
    #[inline] pub fn set_ipr20<F: FnOnce(Ipr20) -> Ipr20>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr20_mut(), f(Ipr20(0)));
        }
        self
    }

    #[doc="Modify the IPR20 register."]
    #[inline] pub fn with_ipr20<F: FnOnce(Ipr20) -> Ipr20>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ipr20_mut(), f(self.ipr20()));
        }
        self
    }

}

#[doc="Interrupt Controller Type Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ictr(pub u32);
impl Ictr {
    #[doc="Total number of interrupt lines in groups"]
    #[inline] pub fn intlinesnum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if INTLINESNUM != 0"]
    #[inline] pub fn test_intlinesnum(&self) -> bool {
        self.intlinesnum() != 0
    }

    #[doc="Sets the INTLINESNUM field."]
    #[inline] pub fn set_intlinesnum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ictr {
    #[inline]
    fn from(other: u32) -> Self {
         Ictr(other)
    }
}

impl ::core::fmt::Display for Ictr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ictr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.intlinesnum() != 0 { try!(write!(f, " intlinesnum=0x{:x}", self.intlinesnum()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Software Triggered Interrupt Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stir(pub u32);
impl Stir {
    #[doc="interrupt to be triggered"]
    #[inline] pub fn intid(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if INTID != 0"]
    #[inline] pub fn test_intid(&self) -> bool {
        self.intid() != 0
    }

    #[doc="Sets the INTID field."]
    #[inline] pub fn set_intid<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stir {
    #[inline]
    fn from(other: u32) -> Self {
         Stir(other)
    }
}

impl ::core::fmt::Display for Stir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.intid() != 0 { try!(write!(f, " intid=0x{:x}", self.intid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Set-Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iser0(pub u32);
impl Iser0 {
    #[doc="SETENA"]
    #[inline] pub fn setena(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SETENA != 0"]
    #[inline] pub fn test_setena(&self) -> bool {
        self.setena() != 0
    }

    #[doc="Sets the SETENA field."]
    #[inline] pub fn set_setena<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iser0 {
    #[inline]
    fn from(other: u32) -> Self {
         Iser0(other)
    }
}

impl ::core::fmt::Display for Iser0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iser0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Set-Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iser1(pub u32);
impl Iser1 {
    #[doc="SETENA"]
    #[inline] pub fn setena(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SETENA != 0"]
    #[inline] pub fn test_setena(&self) -> bool {
        self.setena() != 0
    }

    #[doc="Sets the SETENA field."]
    #[inline] pub fn set_setena<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iser1 {
    #[inline]
    fn from(other: u32) -> Self {
         Iser1(other)
    }
}

impl ::core::fmt::Display for Iser1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iser1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Set-Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iser2(pub u32);
impl Iser2 {
    #[doc="SETENA"]
    #[inline] pub fn setena(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SETENA != 0"]
    #[inline] pub fn test_setena(&self) -> bool {
        self.setena() != 0
    }

    #[doc="Sets the SETENA field."]
    #[inline] pub fn set_setena<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iser2 {
    #[inline]
    fn from(other: u32) -> Self {
         Iser2(other)
    }
}

impl ::core::fmt::Display for Iser2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iser2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear-Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icer0(pub u32);
impl Icer0 {
    #[doc="CLRENA"]
    #[inline] pub fn clrena(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CLRENA != 0"]
    #[inline] pub fn test_clrena(&self) -> bool {
        self.clrena() != 0
    }

    #[doc="Sets the CLRENA field."]
    #[inline] pub fn set_clrena<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icer0 {
    #[inline]
    fn from(other: u32) -> Self {
         Icer0(other)
    }
}

impl ::core::fmt::Display for Icer0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icer0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear-Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icer1(pub u32);
impl Icer1 {
    #[doc="CLRENA"]
    #[inline] pub fn clrena(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CLRENA != 0"]
    #[inline] pub fn test_clrena(&self) -> bool {
        self.clrena() != 0
    }

    #[doc="Sets the CLRENA field."]
    #[inline] pub fn set_clrena<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icer1 {
    #[inline]
    fn from(other: u32) -> Self {
         Icer1(other)
    }
}

impl ::core::fmt::Display for Icer1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icer1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear-Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icer2(pub u32);
impl Icer2 {
    #[doc="CLRENA"]
    #[inline] pub fn clrena(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CLRENA != 0"]
    #[inline] pub fn test_clrena(&self) -> bool {
        self.clrena() != 0
    }

    #[doc="Sets the CLRENA field."]
    #[inline] pub fn set_clrena<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icer2 {
    #[inline]
    fn from(other: u32) -> Self {
         Icer2(other)
    }
}

impl ::core::fmt::Display for Icer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Set-Pending Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ispr0(pub u32);
impl Ispr0 {
    #[doc="SETPEND"]
    #[inline] pub fn setpend(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SETPEND != 0"]
    #[inline] pub fn test_setpend(&self) -> bool {
        self.setpend() != 0
    }

    #[doc="Sets the SETPEND field."]
    #[inline] pub fn set_setpend<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ispr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Ispr0(other)
    }
}

impl ::core::fmt::Display for Ispr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ispr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Set-Pending Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ispr1(pub u32);
impl Ispr1 {
    #[doc="SETPEND"]
    #[inline] pub fn setpend(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SETPEND != 0"]
    #[inline] pub fn test_setpend(&self) -> bool {
        self.setpend() != 0
    }

    #[doc="Sets the SETPEND field."]
    #[inline] pub fn set_setpend<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ispr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Ispr1(other)
    }
}

impl ::core::fmt::Display for Ispr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ispr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Set-Pending Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ispr2(pub u32);
impl Ispr2 {
    #[doc="SETPEND"]
    #[inline] pub fn setpend(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SETPEND != 0"]
    #[inline] pub fn test_setpend(&self) -> bool {
        self.setpend() != 0
    }

    #[doc="Sets the SETPEND field."]
    #[inline] pub fn set_setpend<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ispr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Ispr2(other)
    }
}

impl ::core::fmt::Display for Ispr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ispr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear-Pending Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icpr0(pub u32);
impl Icpr0 {
    #[doc="CLRPEND"]
    #[inline] pub fn clrpend(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CLRPEND != 0"]
    #[inline] pub fn test_clrpend(&self) -> bool {
        self.clrpend() != 0
    }

    #[doc="Sets the CLRPEND field."]
    #[inline] pub fn set_clrpend<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icpr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Icpr0(other)
    }
}

impl ::core::fmt::Display for Icpr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icpr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear-Pending Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icpr1(pub u32);
impl Icpr1 {
    #[doc="CLRPEND"]
    #[inline] pub fn clrpend(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CLRPEND != 0"]
    #[inline] pub fn test_clrpend(&self) -> bool {
        self.clrpend() != 0
    }

    #[doc="Sets the CLRPEND field."]
    #[inline] pub fn set_clrpend<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icpr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Icpr1(other)
    }
}

impl ::core::fmt::Display for Icpr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icpr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear-Pending Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icpr2(pub u32);
impl Icpr2 {
    #[doc="CLRPEND"]
    #[inline] pub fn clrpend(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CLRPEND != 0"]
    #[inline] pub fn test_clrpend(&self) -> bool {
        self.clrpend() != 0
    }

    #[doc="Sets the CLRPEND field."]
    #[inline] pub fn set_clrpend<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icpr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Icpr2(other)
    }
}

impl ::core::fmt::Display for Icpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Active Bit Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iabr0(pub u32);
impl Iabr0 {
    #[doc="ACTIVE"]
    #[inline] pub fn active(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ACTIVE != 0"]
    #[inline] pub fn test_active(&self) -> bool {
        self.active() != 0
    }

    #[doc="Sets the ACTIVE field."]
    #[inline] pub fn set_active<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iabr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Iabr0(other)
    }
}

impl ::core::fmt::Display for Iabr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iabr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Active Bit Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iabr1(pub u32);
impl Iabr1 {
    #[doc="ACTIVE"]
    #[inline] pub fn active(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ACTIVE != 0"]
    #[inline] pub fn test_active(&self) -> bool {
        self.active() != 0
    }

    #[doc="Sets the ACTIVE field."]
    #[inline] pub fn set_active<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iabr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Iabr1(other)
    }
}

impl ::core::fmt::Display for Iabr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iabr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Active Bit Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iabr2(pub u32);
impl Iabr2 {
    #[doc="ACTIVE"]
    #[inline] pub fn active(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ACTIVE != 0"]
    #[inline] pub fn test_active(&self) -> bool {
        self.active() != 0
    }

    #[doc="Sets the ACTIVE field."]
    #[inline] pub fn set_active<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iabr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Iabr2(other)
    }
}

impl ::core::fmt::Display for Iabr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iabr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr0(pub u32);
impl Ipr0 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr0(other)
    }
}

impl ::core::fmt::Display for Ipr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr1(pub u32);
impl Ipr1 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr1(other)
    }
}

impl ::core::fmt::Display for Ipr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr2(pub u32);
impl Ipr2 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr2(other)
    }
}

impl ::core::fmt::Display for Ipr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr3(pub u32);
impl Ipr3 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr3(other)
    }
}

impl ::core::fmt::Display for Ipr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr4(pub u32);
impl Ipr4 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr4(other)
    }
}

impl ::core::fmt::Display for Ipr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr5(pub u32);
impl Ipr5 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr5 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr5(other)
    }
}

impl ::core::fmt::Display for Ipr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr6(pub u32);
impl Ipr6 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr6 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr6(other)
    }
}

impl ::core::fmt::Display for Ipr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr7(pub u32);
impl Ipr7 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr7 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr7(other)
    }
}

impl ::core::fmt::Display for Ipr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr8(pub u32);
impl Ipr8 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr8 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr8(other)
    }
}

impl ::core::fmt::Display for Ipr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr9(pub u32);
impl Ipr9 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr9 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr9(other)
    }
}

impl ::core::fmt::Display for Ipr9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr10(pub u32);
impl Ipr10 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr10 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr10(other)
    }
}

impl ::core::fmt::Display for Ipr10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr11(pub u32);
impl Ipr11 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr11 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr11(other)
    }
}

impl ::core::fmt::Display for Ipr11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr12(pub u32);
impl Ipr12 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr12 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr12(other)
    }
}

impl ::core::fmt::Display for Ipr12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr13(pub u32);
impl Ipr13 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr13 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr13(other)
    }
}

impl ::core::fmt::Display for Ipr13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr14(pub u32);
impl Ipr14 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr14 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr14(other)
    }
}

impl ::core::fmt::Display for Ipr14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr15(pub u32);
impl Ipr15 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr15 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr15(other)
    }
}

impl ::core::fmt::Display for Ipr15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr16(pub u32);
impl Ipr16 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr16 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr16(other)
    }
}

impl ::core::fmt::Display for Ipr16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr17(pub u32);
impl Ipr17 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr17 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr17(other)
    }
}

impl ::core::fmt::Display for Ipr17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr18(pub u32);
impl Ipr18 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr18 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr18(other)
    }
}

impl ::core::fmt::Display for Ipr18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr19(pub u32);
impl Ipr19 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr19 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr19(other)
    }
}

impl ::core::fmt::Display for Ipr19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Priority Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipr20(pub u32);
impl Ipr20 {
    #[doc="IPR_N0"]
    #[inline] pub fn ipr_n0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IPR_N0 != 0"]
    #[inline] pub fn test_ipr_n0(&self) -> bool {
        self.ipr_n0() != 0
    }

    #[doc="Sets the IPR_N0 field."]
    #[inline] pub fn set_ipr_n0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IPR_N1"]
    #[inline] pub fn ipr_n1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if IPR_N1 != 0"]
    #[inline] pub fn test_ipr_n1(&self) -> bool {
        self.ipr_n1() != 0
    }

    #[doc="Sets the IPR_N1 field."]
    #[inline] pub fn set_ipr_n1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IPR_N2"]
    #[inline] pub fn ipr_n2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if IPR_N2 != 0"]
    #[inline] pub fn test_ipr_n2(&self) -> bool {
        self.ipr_n2() != 0
    }

    #[doc="Sets the IPR_N2 field."]
    #[inline] pub fn set_ipr_n2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IPR_N3"]
    #[inline] pub fn ipr_n3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IPR_N3 != 0"]
    #[inline] pub fn test_ipr_n3(&self) -> bool {
        self.ipr_n3() != 0
    }

    #[doc="Sets the IPR_N3 field."]
    #[inline] pub fn set_ipr_n3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ipr20 {
    #[inline]
    fn from(other: u32) -> Self {
         Ipr20(other)
    }
}

impl ::core::fmt::Display for Ipr20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipr20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ipr_n0() != 0 { try!(write!(f, " ipr_n0=0x{:x}", self.ipr_n0()))}
        if self.ipr_n1() != 0 { try!(write!(f, " ipr_n1=0x{:x}", self.ipr_n1()))}
        if self.ipr_n2() != 0 { try!(write!(f, " ipr_n2=0x{:x}", self.ipr_n2()))}
        if self.ipr_n3() != 0 { try!(write!(f, " ipr_n3=0x{:x}", self.ipr_n3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


