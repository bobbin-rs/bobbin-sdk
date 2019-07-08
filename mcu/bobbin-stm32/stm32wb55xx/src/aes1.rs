::bobbin_mcu::periph!( AES1, Aes1, AES1_PERIPH, Aes1Periph, AES1_OWNED, AES1_REF_COUNT, 0x50060000, 0x00, 0x20);
::bobbin_mcu::periph!( AES2, Aes2, AES2_PERIPH, Aes1Periph, AES2_OWNED, AES2_REF_COUNT, 0x58001800, 0x01, 0x21);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="AES1 Peripheral"]
pub struct Aes1Periph(pub usize); 

impl Aes1Periph {
    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x0)
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        self.cr_reg().read()
    }

    #[doc="Write the CR register."]
    #[inline] pub fn write_cr(&self, value: Cr) -> &Self { 
        self.cr_reg().write(value);
        self
    }

    #[doc="Set the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().set(f);
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().with(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x4)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Get the DINR Register."]
    #[inline] pub fn dinr_reg(&self) -> ::bobbin_mcu::register::Register<Dinr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dinr, 0x8)
    }

    #[doc="Get the *mut pointer for the DINR register."]
    #[inline] pub fn dinr_mut(&self) -> *mut Dinr { 
        self.dinr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DINR register."]
    #[inline] pub fn dinr_ptr(&self) -> *const Dinr { 
        self.dinr_reg().ptr()
    }

    #[doc="Read the DINR register."]
    #[inline] pub fn dinr(&self) -> Dinr { 
        self.dinr_reg().read()
    }

    #[doc="Write the DINR register."]
    #[inline] pub fn write_dinr(&self, value: Dinr) -> &Self { 
        self.dinr_reg().write(value);
        self
    }

    #[doc="Set the DINR register."]
    #[inline] pub fn set_dinr<F: FnOnce(Dinr) -> Dinr>(&self, f: F) -> &Self {
        self.dinr_reg().set(f);
        self
    }

    #[doc="Modify the DINR register."]
    #[inline] pub fn with_dinr<F: FnOnce(Dinr) -> Dinr>(&self, f: F) -> &Self {
        self.dinr_reg().with(f);
        self
    }

    #[doc="Get the DOUTR Register."]
    #[inline] pub fn doutr_reg(&self) -> ::bobbin_mcu::register::Register<Doutr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Doutr, 0xc)
    }

    #[doc="Get the *mut pointer for the DOUTR register."]
    #[inline] pub fn doutr_mut(&self) -> *mut Doutr { 
        self.doutr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DOUTR register."]
    #[inline] pub fn doutr_ptr(&self) -> *const Doutr { 
        self.doutr_reg().ptr()
    }

    #[doc="Read the DOUTR register."]
    #[inline] pub fn doutr(&self) -> Doutr { 
        self.doutr_reg().read()
    }

    #[doc="Get the KEYR0 Register."]
    #[inline] pub fn keyr0_reg(&self) -> ::bobbin_mcu::register::Register<Keyr0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr0, 0x10)
    }

    #[doc="Get the *mut pointer for the KEYR0 register."]
    #[inline] pub fn keyr0_mut(&self) -> *mut Keyr0 { 
        self.keyr0_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR0 register."]
    #[inline] pub fn keyr0_ptr(&self) -> *const Keyr0 { 
        self.keyr0_reg().ptr()
    }

    #[doc="Read the KEYR0 register."]
    #[inline] pub fn keyr0(&self) -> Keyr0 { 
        self.keyr0_reg().read()
    }

    #[doc="Write the KEYR0 register."]
    #[inline] pub fn write_keyr0(&self, value: Keyr0) -> &Self { 
        self.keyr0_reg().write(value);
        self
    }

    #[doc="Set the KEYR0 register."]
    #[inline] pub fn set_keyr0<F: FnOnce(Keyr0) -> Keyr0>(&self, f: F) -> &Self {
        self.keyr0_reg().set(f);
        self
    }

    #[doc="Modify the KEYR0 register."]
    #[inline] pub fn with_keyr0<F: FnOnce(Keyr0) -> Keyr0>(&self, f: F) -> &Self {
        self.keyr0_reg().with(f);
        self
    }

    #[doc="Get the KEYR1 Register."]
    #[inline] pub fn keyr1_reg(&self) -> ::bobbin_mcu::register::Register<Keyr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr1, 0x14)
    }

    #[doc="Get the *mut pointer for the KEYR1 register."]
    #[inline] pub fn keyr1_mut(&self) -> *mut Keyr1 { 
        self.keyr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR1 register."]
    #[inline] pub fn keyr1_ptr(&self) -> *const Keyr1 { 
        self.keyr1_reg().ptr()
    }

    #[doc="Read the KEYR1 register."]
    #[inline] pub fn keyr1(&self) -> Keyr1 { 
        self.keyr1_reg().read()
    }

    #[doc="Write the KEYR1 register."]
    #[inline] pub fn write_keyr1(&self, value: Keyr1) -> &Self { 
        self.keyr1_reg().write(value);
        self
    }

    #[doc="Set the KEYR1 register."]
    #[inline] pub fn set_keyr1<F: FnOnce(Keyr1) -> Keyr1>(&self, f: F) -> &Self {
        self.keyr1_reg().set(f);
        self
    }

    #[doc="Modify the KEYR1 register."]
    #[inline] pub fn with_keyr1<F: FnOnce(Keyr1) -> Keyr1>(&self, f: F) -> &Self {
        self.keyr1_reg().with(f);
        self
    }

    #[doc="Get the KEYR2 Register."]
    #[inline] pub fn keyr2_reg(&self) -> ::bobbin_mcu::register::Register<Keyr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr2, 0x18)
    }

    #[doc="Get the *mut pointer for the KEYR2 register."]
    #[inline] pub fn keyr2_mut(&self) -> *mut Keyr2 { 
        self.keyr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR2 register."]
    #[inline] pub fn keyr2_ptr(&self) -> *const Keyr2 { 
        self.keyr2_reg().ptr()
    }

    #[doc="Read the KEYR2 register."]
    #[inline] pub fn keyr2(&self) -> Keyr2 { 
        self.keyr2_reg().read()
    }

    #[doc="Write the KEYR2 register."]
    #[inline] pub fn write_keyr2(&self, value: Keyr2) -> &Self { 
        self.keyr2_reg().write(value);
        self
    }

    #[doc="Set the KEYR2 register."]
    #[inline] pub fn set_keyr2<F: FnOnce(Keyr2) -> Keyr2>(&self, f: F) -> &Self {
        self.keyr2_reg().set(f);
        self
    }

    #[doc="Modify the KEYR2 register."]
    #[inline] pub fn with_keyr2<F: FnOnce(Keyr2) -> Keyr2>(&self, f: F) -> &Self {
        self.keyr2_reg().with(f);
        self
    }

    #[doc="Get the KEYR3 Register."]
    #[inline] pub fn keyr3_reg(&self) -> ::bobbin_mcu::register::Register<Keyr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr3, 0x1c)
    }

    #[doc="Get the *mut pointer for the KEYR3 register."]
    #[inline] pub fn keyr3_mut(&self) -> *mut Keyr3 { 
        self.keyr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR3 register."]
    #[inline] pub fn keyr3_ptr(&self) -> *const Keyr3 { 
        self.keyr3_reg().ptr()
    }

    #[doc="Read the KEYR3 register."]
    #[inline] pub fn keyr3(&self) -> Keyr3 { 
        self.keyr3_reg().read()
    }

    #[doc="Write the KEYR3 register."]
    #[inline] pub fn write_keyr3(&self, value: Keyr3) -> &Self { 
        self.keyr3_reg().write(value);
        self
    }

    #[doc="Set the KEYR3 register."]
    #[inline] pub fn set_keyr3<F: FnOnce(Keyr3) -> Keyr3>(&self, f: F) -> &Self {
        self.keyr3_reg().set(f);
        self
    }

    #[doc="Modify the KEYR3 register."]
    #[inline] pub fn with_keyr3<F: FnOnce(Keyr3) -> Keyr3>(&self, f: F) -> &Self {
        self.keyr3_reg().with(f);
        self
    }

    #[doc="Get the IVR0 Register."]
    #[inline] pub fn ivr0_reg(&self) -> ::bobbin_mcu::register::Register<Ivr0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ivr0, 0x20)
    }

    #[doc="Get the *mut pointer for the IVR0 register."]
    #[inline] pub fn ivr0_mut(&self) -> *mut Ivr0 { 
        self.ivr0_reg().ptr()
    }

    #[doc="Get the *const pointer for the IVR0 register."]
    #[inline] pub fn ivr0_ptr(&self) -> *const Ivr0 { 
        self.ivr0_reg().ptr()
    }

    #[doc="Read the IVR0 register."]
    #[inline] pub fn ivr0(&self) -> Ivr0 { 
        self.ivr0_reg().read()
    }

    #[doc="Write the IVR0 register."]
    #[inline] pub fn write_ivr0(&self, value: Ivr0) -> &Self { 
        self.ivr0_reg().write(value);
        self
    }

    #[doc="Set the IVR0 register."]
    #[inline] pub fn set_ivr0<F: FnOnce(Ivr0) -> Ivr0>(&self, f: F) -> &Self {
        self.ivr0_reg().set(f);
        self
    }

    #[doc="Modify the IVR0 register."]
    #[inline] pub fn with_ivr0<F: FnOnce(Ivr0) -> Ivr0>(&self, f: F) -> &Self {
        self.ivr0_reg().with(f);
        self
    }

    #[doc="Get the IVR1 Register."]
    #[inline] pub fn ivr1_reg(&self) -> ::bobbin_mcu::register::Register<Ivr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ivr1, 0x24)
    }

    #[doc="Get the *mut pointer for the IVR1 register."]
    #[inline] pub fn ivr1_mut(&self) -> *mut Ivr1 { 
        self.ivr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the IVR1 register."]
    #[inline] pub fn ivr1_ptr(&self) -> *const Ivr1 { 
        self.ivr1_reg().ptr()
    }

    #[doc="Read the IVR1 register."]
    #[inline] pub fn ivr1(&self) -> Ivr1 { 
        self.ivr1_reg().read()
    }

    #[doc="Write the IVR1 register."]
    #[inline] pub fn write_ivr1(&self, value: Ivr1) -> &Self { 
        self.ivr1_reg().write(value);
        self
    }

    #[doc="Set the IVR1 register."]
    #[inline] pub fn set_ivr1<F: FnOnce(Ivr1) -> Ivr1>(&self, f: F) -> &Self {
        self.ivr1_reg().set(f);
        self
    }

    #[doc="Modify the IVR1 register."]
    #[inline] pub fn with_ivr1<F: FnOnce(Ivr1) -> Ivr1>(&self, f: F) -> &Self {
        self.ivr1_reg().with(f);
        self
    }

    #[doc="Get the IVR2 Register."]
    #[inline] pub fn ivr2_reg(&self) -> ::bobbin_mcu::register::Register<Ivr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ivr2, 0x28)
    }

    #[doc="Get the *mut pointer for the IVR2 register."]
    #[inline] pub fn ivr2_mut(&self) -> *mut Ivr2 { 
        self.ivr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the IVR2 register."]
    #[inline] pub fn ivr2_ptr(&self) -> *const Ivr2 { 
        self.ivr2_reg().ptr()
    }

    #[doc="Read the IVR2 register."]
    #[inline] pub fn ivr2(&self) -> Ivr2 { 
        self.ivr2_reg().read()
    }

    #[doc="Write the IVR2 register."]
    #[inline] pub fn write_ivr2(&self, value: Ivr2) -> &Self { 
        self.ivr2_reg().write(value);
        self
    }

    #[doc="Set the IVR2 register."]
    #[inline] pub fn set_ivr2<F: FnOnce(Ivr2) -> Ivr2>(&self, f: F) -> &Self {
        self.ivr2_reg().set(f);
        self
    }

    #[doc="Modify the IVR2 register."]
    #[inline] pub fn with_ivr2<F: FnOnce(Ivr2) -> Ivr2>(&self, f: F) -> &Self {
        self.ivr2_reg().with(f);
        self
    }

    #[doc="Get the IVR3 Register."]
    #[inline] pub fn ivr3_reg(&self) -> ::bobbin_mcu::register::Register<Ivr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ivr3, 0x2c)
    }

    #[doc="Get the *mut pointer for the IVR3 register."]
    #[inline] pub fn ivr3_mut(&self) -> *mut Ivr3 { 
        self.ivr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the IVR3 register."]
    #[inline] pub fn ivr3_ptr(&self) -> *const Ivr3 { 
        self.ivr3_reg().ptr()
    }

    #[doc="Read the IVR3 register."]
    #[inline] pub fn ivr3(&self) -> Ivr3 { 
        self.ivr3_reg().read()
    }

    #[doc="Write the IVR3 register."]
    #[inline] pub fn write_ivr3(&self, value: Ivr3) -> &Self { 
        self.ivr3_reg().write(value);
        self
    }

    #[doc="Set the IVR3 register."]
    #[inline] pub fn set_ivr3<F: FnOnce(Ivr3) -> Ivr3>(&self, f: F) -> &Self {
        self.ivr3_reg().set(f);
        self
    }

    #[doc="Modify the IVR3 register."]
    #[inline] pub fn with_ivr3<F: FnOnce(Ivr3) -> Ivr3>(&self, f: F) -> &Self {
        self.ivr3_reg().with(f);
        self
    }

    #[doc="Get the KEYR4 Register."]
    #[inline] pub fn keyr4_reg(&self) -> ::bobbin_mcu::register::Register<Keyr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr4, 0x30)
    }

    #[doc="Get the *mut pointer for the KEYR4 register."]
    #[inline] pub fn keyr4_mut(&self) -> *mut Keyr4 { 
        self.keyr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR4 register."]
    #[inline] pub fn keyr4_ptr(&self) -> *const Keyr4 { 
        self.keyr4_reg().ptr()
    }

    #[doc="Read the KEYR4 register."]
    #[inline] pub fn keyr4(&self) -> Keyr4 { 
        self.keyr4_reg().read()
    }

    #[doc="Write the KEYR4 register."]
    #[inline] pub fn write_keyr4(&self, value: Keyr4) -> &Self { 
        self.keyr4_reg().write(value);
        self
    }

    #[doc="Set the KEYR4 register."]
    #[inline] pub fn set_keyr4<F: FnOnce(Keyr4) -> Keyr4>(&self, f: F) -> &Self {
        self.keyr4_reg().set(f);
        self
    }

    #[doc="Modify the KEYR4 register."]
    #[inline] pub fn with_keyr4<F: FnOnce(Keyr4) -> Keyr4>(&self, f: F) -> &Self {
        self.keyr4_reg().with(f);
        self
    }

    #[doc="Get the KEYR5 Register."]
    #[inline] pub fn keyr5_reg(&self) -> ::bobbin_mcu::register::Register<Keyr5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr5, 0x34)
    }

    #[doc="Get the *mut pointer for the KEYR5 register."]
    #[inline] pub fn keyr5_mut(&self) -> *mut Keyr5 { 
        self.keyr5_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR5 register."]
    #[inline] pub fn keyr5_ptr(&self) -> *const Keyr5 { 
        self.keyr5_reg().ptr()
    }

    #[doc="Read the KEYR5 register."]
    #[inline] pub fn keyr5(&self) -> Keyr5 { 
        self.keyr5_reg().read()
    }

    #[doc="Write the KEYR5 register."]
    #[inline] pub fn write_keyr5(&self, value: Keyr5) -> &Self { 
        self.keyr5_reg().write(value);
        self
    }

    #[doc="Set the KEYR5 register."]
    #[inline] pub fn set_keyr5<F: FnOnce(Keyr5) -> Keyr5>(&self, f: F) -> &Self {
        self.keyr5_reg().set(f);
        self
    }

    #[doc="Modify the KEYR5 register."]
    #[inline] pub fn with_keyr5<F: FnOnce(Keyr5) -> Keyr5>(&self, f: F) -> &Self {
        self.keyr5_reg().with(f);
        self
    }

    #[doc="Get the KEYR6 Register."]
    #[inline] pub fn keyr6_reg(&self) -> ::bobbin_mcu::register::Register<Keyr6> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr6, 0x38)
    }

    #[doc="Get the *mut pointer for the KEYR6 register."]
    #[inline] pub fn keyr6_mut(&self) -> *mut Keyr6 { 
        self.keyr6_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR6 register."]
    #[inline] pub fn keyr6_ptr(&self) -> *const Keyr6 { 
        self.keyr6_reg().ptr()
    }

    #[doc="Read the KEYR6 register."]
    #[inline] pub fn keyr6(&self) -> Keyr6 { 
        self.keyr6_reg().read()
    }

    #[doc="Write the KEYR6 register."]
    #[inline] pub fn write_keyr6(&self, value: Keyr6) -> &Self { 
        self.keyr6_reg().write(value);
        self
    }

    #[doc="Set the KEYR6 register."]
    #[inline] pub fn set_keyr6<F: FnOnce(Keyr6) -> Keyr6>(&self, f: F) -> &Self {
        self.keyr6_reg().set(f);
        self
    }

    #[doc="Modify the KEYR6 register."]
    #[inline] pub fn with_keyr6<F: FnOnce(Keyr6) -> Keyr6>(&self, f: F) -> &Self {
        self.keyr6_reg().with(f);
        self
    }

    #[doc="Get the KEYR7 Register."]
    #[inline] pub fn keyr7_reg(&self) -> ::bobbin_mcu::register::Register<Keyr7> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr7, 0x3c)
    }

    #[doc="Get the *mut pointer for the KEYR7 register."]
    #[inline] pub fn keyr7_mut(&self) -> *mut Keyr7 { 
        self.keyr7_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR7 register."]
    #[inline] pub fn keyr7_ptr(&self) -> *const Keyr7 { 
        self.keyr7_reg().ptr()
    }

    #[doc="Read the KEYR7 register."]
    #[inline] pub fn keyr7(&self) -> Keyr7 { 
        self.keyr7_reg().read()
    }

    #[doc="Write the KEYR7 register."]
    #[inline] pub fn write_keyr7(&self, value: Keyr7) -> &Self { 
        self.keyr7_reg().write(value);
        self
    }

    #[doc="Set the KEYR7 register."]
    #[inline] pub fn set_keyr7<F: FnOnce(Keyr7) -> Keyr7>(&self, f: F) -> &Self {
        self.keyr7_reg().set(f);
        self
    }

    #[doc="Modify the KEYR7 register."]
    #[inline] pub fn with_keyr7<F: FnOnce(Keyr7) -> Keyr7>(&self, f: F) -> &Self {
        self.keyr7_reg().with(f);
        self
    }

    #[doc="Get the SUSP0R Register."]
    #[inline] pub fn susp0r_reg(&self) -> ::bobbin_mcu::register::Register<Susp0r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Susp0r, 0x40)
    }

    #[doc="Get the *mut pointer for the SUSP0R register."]
    #[inline] pub fn susp0r_mut(&self) -> *mut Susp0r { 
        self.susp0r_reg().ptr()
    }

    #[doc="Get the *const pointer for the SUSP0R register."]
    #[inline] pub fn susp0r_ptr(&self) -> *const Susp0r { 
        self.susp0r_reg().ptr()
    }

    #[doc="Read the SUSP0R register."]
    #[inline] pub fn susp0r(&self) -> Susp0r { 
        self.susp0r_reg().read()
    }

    #[doc="Write the SUSP0R register."]
    #[inline] pub fn write_susp0r(&self, value: Susp0r) -> &Self { 
        self.susp0r_reg().write(value);
        self
    }

    #[doc="Set the SUSP0R register."]
    #[inline] pub fn set_susp0r<F: FnOnce(Susp0r) -> Susp0r>(&self, f: F) -> &Self {
        self.susp0r_reg().set(f);
        self
    }

    #[doc="Modify the SUSP0R register."]
    #[inline] pub fn with_susp0r<F: FnOnce(Susp0r) -> Susp0r>(&self, f: F) -> &Self {
        self.susp0r_reg().with(f);
        self
    }

    #[doc="Get the SUSP1R Register."]
    #[inline] pub fn susp1r_reg(&self) -> ::bobbin_mcu::register::Register<Susp1r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Susp1r, 0x44)
    }

    #[doc="Get the *mut pointer for the SUSP1R register."]
    #[inline] pub fn susp1r_mut(&self) -> *mut Susp1r { 
        self.susp1r_reg().ptr()
    }

    #[doc="Get the *const pointer for the SUSP1R register."]
    #[inline] pub fn susp1r_ptr(&self) -> *const Susp1r { 
        self.susp1r_reg().ptr()
    }

    #[doc="Read the SUSP1R register."]
    #[inline] pub fn susp1r(&self) -> Susp1r { 
        self.susp1r_reg().read()
    }

    #[doc="Write the SUSP1R register."]
    #[inline] pub fn write_susp1r(&self, value: Susp1r) -> &Self { 
        self.susp1r_reg().write(value);
        self
    }

    #[doc="Set the SUSP1R register."]
    #[inline] pub fn set_susp1r<F: FnOnce(Susp1r) -> Susp1r>(&self, f: F) -> &Self {
        self.susp1r_reg().set(f);
        self
    }

    #[doc="Modify the SUSP1R register."]
    #[inline] pub fn with_susp1r<F: FnOnce(Susp1r) -> Susp1r>(&self, f: F) -> &Self {
        self.susp1r_reg().with(f);
        self
    }

    #[doc="Get the SUSP2R Register."]
    #[inline] pub fn susp2r_reg(&self) -> ::bobbin_mcu::register::Register<Susp2r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Susp2r, 0x48)
    }

    #[doc="Get the *mut pointer for the SUSP2R register."]
    #[inline] pub fn susp2r_mut(&self) -> *mut Susp2r { 
        self.susp2r_reg().ptr()
    }

    #[doc="Get the *const pointer for the SUSP2R register."]
    #[inline] pub fn susp2r_ptr(&self) -> *const Susp2r { 
        self.susp2r_reg().ptr()
    }

    #[doc="Read the SUSP2R register."]
    #[inline] pub fn susp2r(&self) -> Susp2r { 
        self.susp2r_reg().read()
    }

    #[doc="Write the SUSP2R register."]
    #[inline] pub fn write_susp2r(&self, value: Susp2r) -> &Self { 
        self.susp2r_reg().write(value);
        self
    }

    #[doc="Set the SUSP2R register."]
    #[inline] pub fn set_susp2r<F: FnOnce(Susp2r) -> Susp2r>(&self, f: F) -> &Self {
        self.susp2r_reg().set(f);
        self
    }

    #[doc="Modify the SUSP2R register."]
    #[inline] pub fn with_susp2r<F: FnOnce(Susp2r) -> Susp2r>(&self, f: F) -> &Self {
        self.susp2r_reg().with(f);
        self
    }

    #[doc="Get the SUSP3R Register."]
    #[inline] pub fn susp3r_reg(&self) -> ::bobbin_mcu::register::Register<Susp3r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Susp3r, 0x4c)
    }

    #[doc="Get the *mut pointer for the SUSP3R register."]
    #[inline] pub fn susp3r_mut(&self) -> *mut Susp3r { 
        self.susp3r_reg().ptr()
    }

    #[doc="Get the *const pointer for the SUSP3R register."]
    #[inline] pub fn susp3r_ptr(&self) -> *const Susp3r { 
        self.susp3r_reg().ptr()
    }

    #[doc="Read the SUSP3R register."]
    #[inline] pub fn susp3r(&self) -> Susp3r { 
        self.susp3r_reg().read()
    }

    #[doc="Write the SUSP3R register."]
    #[inline] pub fn write_susp3r(&self, value: Susp3r) -> &Self { 
        self.susp3r_reg().write(value);
        self
    }

    #[doc="Set the SUSP3R register."]
    #[inline] pub fn set_susp3r<F: FnOnce(Susp3r) -> Susp3r>(&self, f: F) -> &Self {
        self.susp3r_reg().set(f);
        self
    }

    #[doc="Modify the SUSP3R register."]
    #[inline] pub fn with_susp3r<F: FnOnce(Susp3r) -> Susp3r>(&self, f: F) -> &Self {
        self.susp3r_reg().with(f);
        self
    }

    #[doc="Get the SUSP4R Register."]
    #[inline] pub fn susp4r_reg(&self) -> ::bobbin_mcu::register::Register<Susp4r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Susp4r, 0x50)
    }

    #[doc="Get the *mut pointer for the SUSP4R register."]
    #[inline] pub fn susp4r_mut(&self) -> *mut Susp4r { 
        self.susp4r_reg().ptr()
    }

    #[doc="Get the *const pointer for the SUSP4R register."]
    #[inline] pub fn susp4r_ptr(&self) -> *const Susp4r { 
        self.susp4r_reg().ptr()
    }

    #[doc="Read the SUSP4R register."]
    #[inline] pub fn susp4r(&self) -> Susp4r { 
        self.susp4r_reg().read()
    }

    #[doc="Write the SUSP4R register."]
    #[inline] pub fn write_susp4r(&self, value: Susp4r) -> &Self { 
        self.susp4r_reg().write(value);
        self
    }

    #[doc="Set the SUSP4R register."]
    #[inline] pub fn set_susp4r<F: FnOnce(Susp4r) -> Susp4r>(&self, f: F) -> &Self {
        self.susp4r_reg().set(f);
        self
    }

    #[doc="Modify the SUSP4R register."]
    #[inline] pub fn with_susp4r<F: FnOnce(Susp4r) -> Susp4r>(&self, f: F) -> &Self {
        self.susp4r_reg().with(f);
        self
    }

    #[doc="Get the SUSP5R Register."]
    #[inline] pub fn susp5r_reg(&self) -> ::bobbin_mcu::register::Register<Susp5r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Susp5r, 0x54)
    }

    #[doc="Get the *mut pointer for the SUSP5R register."]
    #[inline] pub fn susp5r_mut(&self) -> *mut Susp5r { 
        self.susp5r_reg().ptr()
    }

    #[doc="Get the *const pointer for the SUSP5R register."]
    #[inline] pub fn susp5r_ptr(&self) -> *const Susp5r { 
        self.susp5r_reg().ptr()
    }

    #[doc="Read the SUSP5R register."]
    #[inline] pub fn susp5r(&self) -> Susp5r { 
        self.susp5r_reg().read()
    }

    #[doc="Write the SUSP5R register."]
    #[inline] pub fn write_susp5r(&self, value: Susp5r) -> &Self { 
        self.susp5r_reg().write(value);
        self
    }

    #[doc="Set the SUSP5R register."]
    #[inline] pub fn set_susp5r<F: FnOnce(Susp5r) -> Susp5r>(&self, f: F) -> &Self {
        self.susp5r_reg().set(f);
        self
    }

    #[doc="Modify the SUSP5R register."]
    #[inline] pub fn with_susp5r<F: FnOnce(Susp5r) -> Susp5r>(&self, f: F) -> &Self {
        self.susp5r_reg().with(f);
        self
    }

    #[doc="Get the SUSP6R Register."]
    #[inline] pub fn susp6r_reg(&self) -> ::bobbin_mcu::register::Register<Susp6r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Susp6r, 0x58)
    }

    #[doc="Get the *mut pointer for the SUSP6R register."]
    #[inline] pub fn susp6r_mut(&self) -> *mut Susp6r { 
        self.susp6r_reg().ptr()
    }

    #[doc="Get the *const pointer for the SUSP6R register."]
    #[inline] pub fn susp6r_ptr(&self) -> *const Susp6r { 
        self.susp6r_reg().ptr()
    }

    #[doc="Read the SUSP6R register."]
    #[inline] pub fn susp6r(&self) -> Susp6r { 
        self.susp6r_reg().read()
    }

    #[doc="Write the SUSP6R register."]
    #[inline] pub fn write_susp6r(&self, value: Susp6r) -> &Self { 
        self.susp6r_reg().write(value);
        self
    }

    #[doc="Set the SUSP6R register."]
    #[inline] pub fn set_susp6r<F: FnOnce(Susp6r) -> Susp6r>(&self, f: F) -> &Self {
        self.susp6r_reg().set(f);
        self
    }

    #[doc="Modify the SUSP6R register."]
    #[inline] pub fn with_susp6r<F: FnOnce(Susp6r) -> Susp6r>(&self, f: F) -> &Self {
        self.susp6r_reg().with(f);
        self
    }

    #[doc="Get the SUSP7R Register."]
    #[inline] pub fn susp7r_reg(&self) -> ::bobbin_mcu::register::Register<Susp7r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Susp7r, 0x5c)
    }

    #[doc="Get the *mut pointer for the SUSP7R register."]
    #[inline] pub fn susp7r_mut(&self) -> *mut Susp7r { 
        self.susp7r_reg().ptr()
    }

    #[doc="Get the *const pointer for the SUSP7R register."]
    #[inline] pub fn susp7r_ptr(&self) -> *const Susp7r { 
        self.susp7r_reg().ptr()
    }

    #[doc="Read the SUSP7R register."]
    #[inline] pub fn susp7r(&self) -> Susp7r { 
        self.susp7r_reg().read()
    }

    #[doc="Write the SUSP7R register."]
    #[inline] pub fn write_susp7r(&self, value: Susp7r) -> &Self { 
        self.susp7r_reg().write(value);
        self
    }

    #[doc="Set the SUSP7R register."]
    #[inline] pub fn set_susp7r<F: FnOnce(Susp7r) -> Susp7r>(&self, f: F) -> &Self {
        self.susp7r_reg().set(f);
        self
    }

    #[doc="Modify the SUSP7R register."]
    #[inline] pub fn with_susp7r<F: FnOnce(Susp7r) -> Susp7r>(&self, f: F) -> &Self {
        self.susp7r_reg().with(f);
        self
    }

    #[doc="Get the HWCFR Register."]
    #[inline] pub fn hwcfr_reg(&self) -> ::bobbin_mcu::register::Register<Hwcfr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hwcfr, 0x3f0)
    }

    #[doc="Get the *mut pointer for the HWCFR register."]
    #[inline] pub fn hwcfr_mut(&self) -> *mut Hwcfr { 
        self.hwcfr_reg().ptr()
    }

    #[doc="Get the *const pointer for the HWCFR register."]
    #[inline] pub fn hwcfr_ptr(&self) -> *const Hwcfr { 
        self.hwcfr_reg().ptr()
    }

    #[doc="Read the HWCFR register."]
    #[inline] pub fn hwcfr(&self) -> Hwcfr { 
        self.hwcfr_reg().read()
    }

    #[doc="Get the VERR Register."]
    #[inline] pub fn verr_reg(&self) -> ::bobbin_mcu::register::Register<Verr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Verr, 0x3f4)
    }

    #[doc="Get the *mut pointer for the VERR register."]
    #[inline] pub fn verr_mut(&self) -> *mut Verr { 
        self.verr_reg().ptr()
    }

    #[doc="Get the *const pointer for the VERR register."]
    #[inline] pub fn verr_ptr(&self) -> *const Verr { 
        self.verr_reg().ptr()
    }

    #[doc="Read the VERR register."]
    #[inline] pub fn verr(&self) -> Verr { 
        self.verr_reg().read()
    }

    #[doc="Get the IPIDR Register."]
    #[inline] pub fn ipidr_reg(&self) -> ::bobbin_mcu::register::Register<Ipidr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ipidr, 0x3f8)
    }

    #[doc="Get the *mut pointer for the IPIDR register."]
    #[inline] pub fn ipidr_mut(&self) -> *mut Ipidr { 
        self.ipidr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IPIDR register."]
    #[inline] pub fn ipidr_ptr(&self) -> *const Ipidr { 
        self.ipidr_reg().ptr()
    }

    #[doc="Read the IPIDR register."]
    #[inline] pub fn ipidr(&self) -> Ipidr { 
        self.ipidr_reg().read()
    }

    #[doc="Get the SIDR Register."]
    #[inline] pub fn sidr_reg(&self) -> ::bobbin_mcu::register::Register<Sidr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sidr, 0x3fc)
    }

    #[doc="Get the *mut pointer for the SIDR register."]
    #[inline] pub fn sidr_mut(&self) -> *mut Sidr { 
        self.sidr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SIDR register."]
    #[inline] pub fn sidr_ptr(&self) -> *const Sidr { 
        self.sidr_reg().ptr()
    }

    #[doc="Read the SIDR register."]
    #[inline] pub fn sidr(&self) -> Sidr { 
        self.sidr_reg().read()
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Number of padding bytes in last block of payload"]
    #[inline] pub fn npblb(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if NPBLB != 0"]
    #[inline] pub fn test_npblb(&self) -> bool {
        self.npblb() != 0
    }

    #[doc="Sets the NPBLB field."]
    #[inline] pub fn set_npblb<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Key size selection"]
    #[inline] pub fn keysize(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if KEYSIZE != 0"]
    #[inline] pub fn test_keysize(&self) -> bool {
        self.keysize() != 0
    }

    #[doc="Sets the KEYSIZE field."]
    #[inline] pub fn set_keysize<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="AES chaining mode Bit2"]
    #[inline] pub fn chmod2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CHMOD2 != 0"]
    #[inline] pub fn test_chmod2(&self) -> bool {
        self.chmod2() != 0
    }

    #[doc="Sets the CHMOD2 field."]
    #[inline] pub fn set_chmod2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected"]
    #[inline] pub fn gcmph(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if GCMPH != 0"]
    #[inline] pub fn test_gcmph(&self) -> bool {
        self.gcmph() != 0
    }

    #[doc="Sets the GCMPH field."]
    #[inline] pub fn set_gcmph<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Enable DMA management of data output phase"]
    #[inline] pub fn dmaouten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DMAOUTEN != 0"]
    #[inline] pub fn test_dmaouten(&self) -> bool {
        self.dmaouten() != 0
    }

    #[doc="Sets the DMAOUTEN field."]
    #[inline] pub fn set_dmaouten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Enable DMA management of data input phase"]
    #[inline] pub fn dmainen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DMAINEN != 0"]
    #[inline] pub fn test_dmainen(&self) -> bool {
        self.dmainen() != 0
    }

    #[doc="Sets the DMAINEN field."]
    #[inline] pub fn set_dmainen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CCF flag interrupt enable"]
    #[inline] pub fn ccfie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CCFIE != 0"]
    #[inline] pub fn test_ccfie(&self) -> bool {
        self.ccfie() != 0
    }

    #[doc="Sets the CCFIE field."]
    #[inline] pub fn set_ccfie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Error clear"]
    #[inline] pub fn errc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ERRC != 0"]
    #[inline] pub fn test_errc(&self) -> bool {
        self.errc() != 0
    }

    #[doc="Sets the ERRC field."]
    #[inline] pub fn set_errc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Computation Complete Flag Clear"]
    #[inline] pub fn ccfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CCFC != 0"]
    #[inline] pub fn test_ccfc(&self) -> bool {
        self.ccfc() != 0
    }

    #[doc="Sets the CCFC field."]
    #[inline] pub fn set_ccfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="AES chaining mode Bit1 Bit0"]
    #[inline] pub fn chmod10(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if CHMOD10 != 0"]
    #[inline] pub fn test_chmod10(&self) -> bool {
        self.chmod10() != 0
    }

    #[doc="Sets the CHMOD10 field."]
    #[inline] pub fn set_chmod10<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="AES operating mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline] pub fn datatype(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if DATATYPE != 0"]
    #[inline] pub fn test_datatype(&self) -> bool {
        self.datatype() != 0
    }

    #[doc="Sets the DATATYPE field."]
    #[inline] pub fn set_datatype<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="AES enable"]
    #[inline] pub fn en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="Sets the EN field."]
    #[inline] pub fn set_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.npblb() != 0 { try!(write!(f, " npblb=0x{:x}", self.npblb()))}
        if self.keysize() != 0 { try!(write!(f, " keysize"))}
        if self.chmod2() != 0 { try!(write!(f, " chmod2"))}
        if self.gcmph() != 0 { try!(write!(f, " gcmph=0x{:x}", self.gcmph()))}
        if self.dmaouten() != 0 { try!(write!(f, " dmaouten"))}
        if self.dmainen() != 0 { try!(write!(f, " dmainen"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.ccfie() != 0 { try!(write!(f, " ccfie"))}
        if self.errc() != 0 { try!(write!(f, " errc"))}
        if self.ccfc() != 0 { try!(write!(f, " ccfc"))}
        if self.chmod10() != 0 { try!(write!(f, " chmod10=0x{:x}", self.chmod10()))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.datatype() != 0 { try!(write!(f, " datatype=0x{:x}", self.datatype()))}
        if self.en() != 0 { try!(write!(f, " en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Busy flag"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Write error flag"]
    #[inline] pub fn wrerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WRERR != 0"]
    #[inline] pub fn test_wrerr(&self) -> bool {
        self.wrerr() != 0
    }

    #[doc="Sets the WRERR field."]
    #[inline] pub fn set_wrerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Read error flag"]
    #[inline] pub fn rderr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RDERR != 0"]
    #[inline] pub fn test_rderr(&self) -> bool {
        self.rderr() != 0
    }

    #[doc="Sets the RDERR field."]
    #[inline] pub fn set_rderr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Computation complete flag"]
    #[inline] pub fn ccf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CCF != 0"]
    #[inline] pub fn test_ccf(&self) -> bool {
        self.ccf() != 0
    }

    #[doc="Sets the CCF field."]
    #[inline] pub fn set_ccf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
        if self.wrerr() != 0 { try!(write!(f, " wrerr"))}
        if self.rderr() != 0 { try!(write!(f, " rderr"))}
        if self.ccf() != 0 { try!(write!(f, " ccf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data input register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dinr(pub u32);
impl Dinr {
    #[doc="Data Input Register"]
    #[inline] pub fn aes_dinr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_DINR != 0"]
    #[inline] pub fn test_aes_dinr(&self) -> bool {
        self.aes_dinr() != 0
    }

    #[doc="Sets the AES_DINR field."]
    #[inline] pub fn set_aes_dinr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dinr {
    #[inline]
    fn from(other: u32) -> Self {
         Dinr(other)
    }
}

impl ::core::fmt::Display for Dinr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dinr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data output register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doutr(pub u32);
impl Doutr {
    #[doc="Data output register"]
    #[inline] pub fn aes_doutr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_DOUTR != 0"]
    #[inline] pub fn test_aes_doutr(&self) -> bool {
        self.aes_doutr() != 0
    }

    #[doc="Sets the AES_DOUTR field."]
    #[inline] pub fn set_aes_doutr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doutr {
    #[inline]
    fn from(other: u32) -> Self {
         Doutr(other)
    }
}

impl ::core::fmt::Display for Doutr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doutr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr0(pub u32);
impl Keyr0 {
    #[doc="Data Output Register (LSB key [31:0])"]
    #[inline] pub fn aes_keyr0(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR0 != 0"]
    #[inline] pub fn test_aes_keyr0(&self) -> bool {
        self.aes_keyr0() != 0
    }

    #[doc="Sets the AES_KEYR0 field."]
    #[inline] pub fn set_aes_keyr0<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr0(other)
    }
}

impl ::core::fmt::Display for Keyr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr1(pub u32);
impl Keyr1 {
    #[doc="AES key register (key [63:32])"]
    #[inline] pub fn aes_keyr1(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR1 != 0"]
    #[inline] pub fn test_aes_keyr1(&self) -> bool {
        self.aes_keyr1() != 0
    }

    #[doc="Sets the AES_KEYR1 field."]
    #[inline] pub fn set_aes_keyr1<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr1(other)
    }
}

impl ::core::fmt::Display for Keyr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr2(pub u32);
impl Keyr2 {
    #[doc="AES key register (key [95:64])"]
    #[inline] pub fn aes_keyr2(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR2 != 0"]
    #[inline] pub fn test_aes_keyr2(&self) -> bool {
        self.aes_keyr2() != 0
    }

    #[doc="Sets the AES_KEYR2 field."]
    #[inline] pub fn set_aes_keyr2<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr2(other)
    }
}

impl ::core::fmt::Display for Keyr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr3(pub u32);
impl Keyr3 {
    #[doc="AES key register (MSB key [127:96])"]
    #[inline] pub fn aes_keyr3(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR3 != 0"]
    #[inline] pub fn test_aes_keyr3(&self) -> bool {
        self.aes_keyr3() != 0
    }

    #[doc="Sets the AES_KEYR3 field."]
    #[inline] pub fn set_aes_keyr3<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr3(other)
    }
}

impl ::core::fmt::Display for Keyr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ivr0(pub u32);
impl Ivr0 {
    #[doc="initialization vector register (LSB IVR [31:0])"]
    #[inline] pub fn aes_ivr0(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_IVR0 != 0"]
    #[inline] pub fn test_aes_ivr0(&self) -> bool {
        self.aes_ivr0() != 0
    }

    #[doc="Sets the AES_IVR0 field."]
    #[inline] pub fn set_aes_ivr0<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ivr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Ivr0(other)
    }
}

impl ::core::fmt::Display for Ivr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ivr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ivr1(pub u32);
impl Ivr1 {
    #[doc="Initialization Vector Register (IVR [63:32])"]
    #[inline] pub fn aes_ivr1(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_IVR1 != 0"]
    #[inline] pub fn test_aes_ivr1(&self) -> bool {
        self.aes_ivr1() != 0
    }

    #[doc="Sets the AES_IVR1 field."]
    #[inline] pub fn set_aes_ivr1<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ivr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Ivr1(other)
    }
}

impl ::core::fmt::Display for Ivr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ivr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ivr2(pub u32);
impl Ivr2 {
    #[doc="Initialization Vector Register (IVR [95:64])"]
    #[inline] pub fn aes_ivr2(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_IVR2 != 0"]
    #[inline] pub fn test_aes_ivr2(&self) -> bool {
        self.aes_ivr2() != 0
    }

    #[doc="Sets the AES_IVR2 field."]
    #[inline] pub fn set_aes_ivr2<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ivr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Ivr2(other)
    }
}

impl ::core::fmt::Display for Ivr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ivr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="initialization vector register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ivr3(pub u32);
impl Ivr3 {
    #[doc="Initialization Vector Register (MSB IVR [127:96])"]
    #[inline] pub fn aes_ivr3(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_IVR3 != 0"]
    #[inline] pub fn test_aes_ivr3(&self) -> bool {
        self.aes_ivr3() != 0
    }

    #[doc="Sets the AES_IVR3 field."]
    #[inline] pub fn set_aes_ivr3<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ivr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Ivr3(other)
    }
}

impl ::core::fmt::Display for Ivr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ivr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr4(pub u32);
impl Keyr4 {
    #[doc="AES key register (MSB key [159:128])"]
    #[inline] pub fn aes_keyr4(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR4 != 0"]
    #[inline] pub fn test_aes_keyr4(&self) -> bool {
        self.aes_keyr4() != 0
    }

    #[doc="Sets the AES_KEYR4 field."]
    #[inline] pub fn set_aes_keyr4<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr4(other)
    }
}

impl ::core::fmt::Display for Keyr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 5"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr5(pub u32);
impl Keyr5 {
    #[doc="AES key register (MSB key [191:160])"]
    #[inline] pub fn aes_keyr5(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR5 != 0"]
    #[inline] pub fn test_aes_keyr5(&self) -> bool {
        self.aes_keyr5() != 0
    }

    #[doc="Sets the AES_KEYR5 field."]
    #[inline] pub fn set_aes_keyr5<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr5 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr5(other)
    }
}

impl ::core::fmt::Display for Keyr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 6"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr6(pub u32);
impl Keyr6 {
    #[doc="AES key register (MSB key [223:192])"]
    #[inline] pub fn aes_keyr6(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR6 != 0"]
    #[inline] pub fn test_aes_keyr6(&self) -> bool {
        self.aes_keyr6() != 0
    }

    #[doc="Sets the AES_KEYR6 field."]
    #[inline] pub fn set_aes_keyr6<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr6 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr6(other)
    }
}

impl ::core::fmt::Display for Keyr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="key register 7"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr7(pub u32);
impl Keyr7 {
    #[doc="AES key register (MSB key [255:224])"]
    #[inline] pub fn aes_keyr7(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_KEYR7 != 0"]
    #[inline] pub fn test_aes_keyr7(&self) -> bool {
        self.aes_keyr7() != 0
    }

    #[doc="Sets the AES_KEYR7 field."]
    #[inline] pub fn set_aes_keyr7<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr7 {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr7(other)
    }
}

impl ::core::fmt::Display for Keyr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES suspend register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Susp0r(pub u32);
impl Susp0r {
    #[doc="AES suspend register 0"]
    #[inline] pub fn aes_susp0r(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_SUSP0R != 0"]
    #[inline] pub fn test_aes_susp0r(&self) -> bool {
        self.aes_susp0r() != 0
    }

    #[doc="Sets the AES_SUSP0R field."]
    #[inline] pub fn set_aes_susp0r<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Susp0r {
    #[inline]
    fn from(other: u32) -> Self {
         Susp0r(other)
    }
}

impl ::core::fmt::Display for Susp0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Susp0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES suspend register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Susp1r(pub u32);
impl Susp1r {
    #[doc="AES suspend register 1"]
    #[inline] pub fn aes_susp1r(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_SUSP1R != 0"]
    #[inline] pub fn test_aes_susp1r(&self) -> bool {
        self.aes_susp1r() != 0
    }

    #[doc="Sets the AES_SUSP1R field."]
    #[inline] pub fn set_aes_susp1r<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Susp1r {
    #[inline]
    fn from(other: u32) -> Self {
         Susp1r(other)
    }
}

impl ::core::fmt::Display for Susp1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Susp1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES suspend register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Susp2r(pub u32);
impl Susp2r {
    #[doc="AES suspend register 2"]
    #[inline] pub fn aes_susp2r(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_SUSP2R != 0"]
    #[inline] pub fn test_aes_susp2r(&self) -> bool {
        self.aes_susp2r() != 0
    }

    #[doc="Sets the AES_SUSP2R field."]
    #[inline] pub fn set_aes_susp2r<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Susp2r {
    #[inline]
    fn from(other: u32) -> Self {
         Susp2r(other)
    }
}

impl ::core::fmt::Display for Susp2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Susp2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES suspend register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Susp3r(pub u32);
impl Susp3r {
    #[doc="AES suspend register 3"]
    #[inline] pub fn aes_susp3r(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_SUSP3R != 0"]
    #[inline] pub fn test_aes_susp3r(&self) -> bool {
        self.aes_susp3r() != 0
    }

    #[doc="Sets the AES_SUSP3R field."]
    #[inline] pub fn set_aes_susp3r<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Susp3r {
    #[inline]
    fn from(other: u32) -> Self {
         Susp3r(other)
    }
}

impl ::core::fmt::Display for Susp3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Susp3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES suspend register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Susp4r(pub u32);
impl Susp4r {
    #[doc="AES suspend register 4"]
    #[inline] pub fn aes_susp4r(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_SUSP4R != 0"]
    #[inline] pub fn test_aes_susp4r(&self) -> bool {
        self.aes_susp4r() != 0
    }

    #[doc="Sets the AES_SUSP4R field."]
    #[inline] pub fn set_aes_susp4r<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Susp4r {
    #[inline]
    fn from(other: u32) -> Self {
         Susp4r(other)
    }
}

impl ::core::fmt::Display for Susp4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Susp4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES suspend register 5"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Susp5r(pub u32);
impl Susp5r {
    #[doc="AES suspend register 5"]
    #[inline] pub fn aes_susp5r(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_SUSP5R != 0"]
    #[inline] pub fn test_aes_susp5r(&self) -> bool {
        self.aes_susp5r() != 0
    }

    #[doc="Sets the AES_SUSP5R field."]
    #[inline] pub fn set_aes_susp5r<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Susp5r {
    #[inline]
    fn from(other: u32) -> Self {
         Susp5r(other)
    }
}

impl ::core::fmt::Display for Susp5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Susp5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES suspend register 6"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Susp6r(pub u32);
impl Susp6r {
    #[doc="AES suspend register 6"]
    #[inline] pub fn aes_susp6r(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_SUSP6R != 0"]
    #[inline] pub fn test_aes_susp6r(&self) -> bool {
        self.aes_susp6r() != 0
    }

    #[doc="Sets the AES_SUSP6R field."]
    #[inline] pub fn set_aes_susp6r<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Susp6r {
    #[inline]
    fn from(other: u32) -> Self {
         Susp6r(other)
    }
}

impl ::core::fmt::Display for Susp6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Susp6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES suspend register 7"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Susp7r(pub u32);
impl Susp7r {
    #[doc="AES suspend register 7"]
    #[inline] pub fn aes_susp7r(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if AES_SUSP7R != 0"]
    #[inline] pub fn test_aes_susp7r(&self) -> bool {
        self.aes_susp7r() != 0
    }

    #[doc="Sets the AES_SUSP7R field."]
    #[inline] pub fn set_aes_susp7r<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Susp7r {
    #[inline]
    fn from(other: u32) -> Self {
         Susp7r(other)
    }
}

impl ::core::fmt::Display for Susp7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Susp7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES hardware configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hwcfr(pub u32);
impl Hwcfr {
    #[doc="HW Generic 4"]
    #[inline] pub fn cfg4(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if CFG4 != 0"]
    #[inline] pub fn test_cfg4(&self) -> bool {
        self.cfg4() != 0
    }

    #[doc="Sets the CFG4 field."]
    #[inline] pub fn set_cfg4<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="HW Generic 3"]
    #[inline] pub fn cfg3(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CFG3 != 0"]
    #[inline] pub fn test_cfg3(&self) -> bool {
        self.cfg3() != 0
    }

    #[doc="Sets the CFG3 field."]
    #[inline] pub fn set_cfg3<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="HW Generic 2"]
    #[inline] pub fn cfg2(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if CFG2 != 0"]
    #[inline] pub fn test_cfg2(&self) -> bool {
        self.cfg2() != 0
    }

    #[doc="Sets the CFG2 field."]
    #[inline] pub fn set_cfg2<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HW Generic 1"]
    #[inline] pub fn cfg1(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CFG1 != 0"]
    #[inline] pub fn test_cfg1(&self) -> bool {
        self.cfg1() != 0
    }

    #[doc="Sets the CFG1 field."]
    #[inline] pub fn set_cfg1<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hwcfr {
    #[inline]
    fn from(other: u32) -> Self {
         Hwcfr(other)
    }
}

impl ::core::fmt::Display for Hwcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hwcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cfg4() != 0 { try!(write!(f, " cfg4=0x{:x}", self.cfg4()))}
        if self.cfg3() != 0 { try!(write!(f, " cfg3=0x{:x}", self.cfg3()))}
        if self.cfg2() != 0 { try!(write!(f, " cfg2=0x{:x}", self.cfg2()))}
        if self.cfg1() != 0 { try!(write!(f, " cfg1=0x{:x}", self.cfg1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES version register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verr(pub u32);
impl Verr {
    #[doc="Major revision"]
    #[inline] pub fn majrev(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if MAJREV != 0"]
    #[inline] pub fn test_majrev(&self) -> bool {
        self.majrev() != 0
    }

    #[doc="Sets the MAJREV field."]
    #[inline] pub fn set_majrev<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Minor revision"]
    #[inline] pub fn minrev(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MINREV != 0"]
    #[inline] pub fn test_minrev(&self) -> bool {
        self.minrev() != 0
    }

    #[doc="Sets the MINREV field."]
    #[inline] pub fn set_minrev<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Verr {
    #[inline]
    fn from(other: u32) -> Self {
         Verr(other)
    }
}

impl ::core::fmt::Display for Verr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Verr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.majrev() != 0 { try!(write!(f, " majrev=0x{:x}", self.majrev()))}
        if self.minrev() != 0 { try!(write!(f, " minrev=0x{:x}", self.minrev()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES identification register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipidr(pub u32);
impl Ipidr {
    #[doc="Identification code"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ipidr {
    #[inline]
    fn from(other: u32) -> Self {
         Ipidr(other)
    }
}

impl ::core::fmt::Display for Ipidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ipidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AES size ID register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sidr(pub u32);
impl Sidr {
    #[doc="Size Identification code"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sidr {
    #[inline]
    fn from(other: u32) -> Self {
         Sidr(other)
    }
}

impl ::core::fmt::Display for Sidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

