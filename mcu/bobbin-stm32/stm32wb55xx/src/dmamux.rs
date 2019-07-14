::bobbin_mcu::periph!( DMAMUX1, Dmamux1, DMAMUX1_PERIPH, DmamuxPeriph, DMAMUX1_OWNED, DMAMUX1_REF_COUNT, 0x40020800, 0x00, 0x1b);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMAMUX Peripheral"]
pub struct DmamuxPeriph(pub usize); 

impl DmamuxPeriph {
    #[doc="Get the C0CR Register."]
    #[inline] pub fn c0cr_reg(&self) -> ::bobbin_mcu::register::Register<C0cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C0cr, 0x0)
    }

    #[doc="Get the *mut pointer for the C0CR register."]
    #[inline] pub fn c0cr_mut(&self) -> *mut C0cr { 
        self.c0cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C0CR register."]
    #[inline] pub fn c0cr_ptr(&self) -> *const C0cr { 
        self.c0cr_reg().ptr()
    }

    #[doc="Read the C0CR register."]
    #[inline] pub fn c0cr(&self) -> C0cr { 
        self.c0cr_reg().read()
    }

    #[doc="Write the C0CR register."]
    #[inline] pub fn write_c0cr(&self, value: C0cr) -> &Self { 
        self.c0cr_reg().write(value);
        self
    }

    #[doc="Set the C0CR register."]
    #[inline] pub fn set_c0cr<F: FnOnce(C0cr) -> C0cr>(&self, f: F) -> &Self {
        self.c0cr_reg().set(f);
        self
    }

    #[doc="Modify the C0CR register."]
    #[inline] pub fn with_c0cr<F: FnOnce(C0cr) -> C0cr>(&self, f: F) -> &Self {
        self.c0cr_reg().with(f);
        self
    }

    #[doc="Get the C1CR Register."]
    #[inline] pub fn c1cr_reg(&self) -> ::bobbin_mcu::register::Register<C1cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1cr, 0x4)
    }

    #[doc="Get the *mut pointer for the C1CR register."]
    #[inline] pub fn c1cr_mut(&self) -> *mut C1cr { 
        self.c1cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1CR register."]
    #[inline] pub fn c1cr_ptr(&self) -> *const C1cr { 
        self.c1cr_reg().ptr()
    }

    #[doc="Read the C1CR register."]
    #[inline] pub fn c1cr(&self) -> C1cr { 
        self.c1cr_reg().read()
    }

    #[doc="Write the C1CR register."]
    #[inline] pub fn write_c1cr(&self, value: C1cr) -> &Self { 
        self.c1cr_reg().write(value);
        self
    }

    #[doc="Set the C1CR register."]
    #[inline] pub fn set_c1cr<F: FnOnce(C1cr) -> C1cr>(&self, f: F) -> &Self {
        self.c1cr_reg().set(f);
        self
    }

    #[doc="Modify the C1CR register."]
    #[inline] pub fn with_c1cr<F: FnOnce(C1cr) -> C1cr>(&self, f: F) -> &Self {
        self.c1cr_reg().with(f);
        self
    }

    #[doc="Get the C2CR Register."]
    #[inline] pub fn c2cr_reg(&self) -> ::bobbin_mcu::register::Register<C2cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2cr, 0x8)
    }

    #[doc="Get the *mut pointer for the C2CR register."]
    #[inline] pub fn c2cr_mut(&self) -> *mut C2cr { 
        self.c2cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2CR register."]
    #[inline] pub fn c2cr_ptr(&self) -> *const C2cr { 
        self.c2cr_reg().ptr()
    }

    #[doc="Read the C2CR register."]
    #[inline] pub fn c2cr(&self) -> C2cr { 
        self.c2cr_reg().read()
    }

    #[doc="Write the C2CR register."]
    #[inline] pub fn write_c2cr(&self, value: C2cr) -> &Self { 
        self.c2cr_reg().write(value);
        self
    }

    #[doc="Set the C2CR register."]
    #[inline] pub fn set_c2cr<F: FnOnce(C2cr) -> C2cr>(&self, f: F) -> &Self {
        self.c2cr_reg().set(f);
        self
    }

    #[doc="Modify the C2CR register."]
    #[inline] pub fn with_c2cr<F: FnOnce(C2cr) -> C2cr>(&self, f: F) -> &Self {
        self.c2cr_reg().with(f);
        self
    }

    #[doc="Get the C3CR Register."]
    #[inline] pub fn c3cr_reg(&self) -> ::bobbin_mcu::register::Register<C3cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C3cr, 0xc)
    }

    #[doc="Get the *mut pointer for the C3CR register."]
    #[inline] pub fn c3cr_mut(&self) -> *mut C3cr { 
        self.c3cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C3CR register."]
    #[inline] pub fn c3cr_ptr(&self) -> *const C3cr { 
        self.c3cr_reg().ptr()
    }

    #[doc="Read the C3CR register."]
    #[inline] pub fn c3cr(&self) -> C3cr { 
        self.c3cr_reg().read()
    }

    #[doc="Write the C3CR register."]
    #[inline] pub fn write_c3cr(&self, value: C3cr) -> &Self { 
        self.c3cr_reg().write(value);
        self
    }

    #[doc="Set the C3CR register."]
    #[inline] pub fn set_c3cr<F: FnOnce(C3cr) -> C3cr>(&self, f: F) -> &Self {
        self.c3cr_reg().set(f);
        self
    }

    #[doc="Modify the C3CR register."]
    #[inline] pub fn with_c3cr<F: FnOnce(C3cr) -> C3cr>(&self, f: F) -> &Self {
        self.c3cr_reg().with(f);
        self
    }

    #[doc="Get the C4CR Register."]
    #[inline] pub fn c4cr_reg(&self) -> ::bobbin_mcu::register::Register<C4cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C4cr, 0x10)
    }

    #[doc="Get the *mut pointer for the C4CR register."]
    #[inline] pub fn c4cr_mut(&self) -> *mut C4cr { 
        self.c4cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C4CR register."]
    #[inline] pub fn c4cr_ptr(&self) -> *const C4cr { 
        self.c4cr_reg().ptr()
    }

    #[doc="Read the C4CR register."]
    #[inline] pub fn c4cr(&self) -> C4cr { 
        self.c4cr_reg().read()
    }

    #[doc="Write the C4CR register."]
    #[inline] pub fn write_c4cr(&self, value: C4cr) -> &Self { 
        self.c4cr_reg().write(value);
        self
    }

    #[doc="Set the C4CR register."]
    #[inline] pub fn set_c4cr<F: FnOnce(C4cr) -> C4cr>(&self, f: F) -> &Self {
        self.c4cr_reg().set(f);
        self
    }

    #[doc="Modify the C4CR register."]
    #[inline] pub fn with_c4cr<F: FnOnce(C4cr) -> C4cr>(&self, f: F) -> &Self {
        self.c4cr_reg().with(f);
        self
    }

    #[doc="Get the C5CR Register."]
    #[inline] pub fn c5cr_reg(&self) -> ::bobbin_mcu::register::Register<C5cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C5cr, 0x14)
    }

    #[doc="Get the *mut pointer for the C5CR register."]
    #[inline] pub fn c5cr_mut(&self) -> *mut C5cr { 
        self.c5cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C5CR register."]
    #[inline] pub fn c5cr_ptr(&self) -> *const C5cr { 
        self.c5cr_reg().ptr()
    }

    #[doc="Read the C5CR register."]
    #[inline] pub fn c5cr(&self) -> C5cr { 
        self.c5cr_reg().read()
    }

    #[doc="Write the C5CR register."]
    #[inline] pub fn write_c5cr(&self, value: C5cr) -> &Self { 
        self.c5cr_reg().write(value);
        self
    }

    #[doc="Set the C5CR register."]
    #[inline] pub fn set_c5cr<F: FnOnce(C5cr) -> C5cr>(&self, f: F) -> &Self {
        self.c5cr_reg().set(f);
        self
    }

    #[doc="Modify the C5CR register."]
    #[inline] pub fn with_c5cr<F: FnOnce(C5cr) -> C5cr>(&self, f: F) -> &Self {
        self.c5cr_reg().with(f);
        self
    }

    #[doc="Get the C6CR Register."]
    #[inline] pub fn c6cr_reg(&self) -> ::bobbin_mcu::register::Register<C6cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C6cr, 0x18)
    }

    #[doc="Get the *mut pointer for the C6CR register."]
    #[inline] pub fn c6cr_mut(&self) -> *mut C6cr { 
        self.c6cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C6CR register."]
    #[inline] pub fn c6cr_ptr(&self) -> *const C6cr { 
        self.c6cr_reg().ptr()
    }

    #[doc="Read the C6CR register."]
    #[inline] pub fn c6cr(&self) -> C6cr { 
        self.c6cr_reg().read()
    }

    #[doc="Write the C6CR register."]
    #[inline] pub fn write_c6cr(&self, value: C6cr) -> &Self { 
        self.c6cr_reg().write(value);
        self
    }

    #[doc="Set the C6CR register."]
    #[inline] pub fn set_c6cr<F: FnOnce(C6cr) -> C6cr>(&self, f: F) -> &Self {
        self.c6cr_reg().set(f);
        self
    }

    #[doc="Modify the C6CR register."]
    #[inline] pub fn with_c6cr<F: FnOnce(C6cr) -> C6cr>(&self, f: F) -> &Self {
        self.c6cr_reg().with(f);
        self
    }

    #[doc="Get the C7CR Register."]
    #[inline] pub fn c7cr_reg(&self) -> ::bobbin_mcu::register::Register<C7cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C7cr, 0x1c)
    }

    #[doc="Get the *mut pointer for the C7CR register."]
    #[inline] pub fn c7cr_mut(&self) -> *mut C7cr { 
        self.c7cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C7CR register."]
    #[inline] pub fn c7cr_ptr(&self) -> *const C7cr { 
        self.c7cr_reg().ptr()
    }

    #[doc="Read the C7CR register."]
    #[inline] pub fn c7cr(&self) -> C7cr { 
        self.c7cr_reg().read()
    }

    #[doc="Write the C7CR register."]
    #[inline] pub fn write_c7cr(&self, value: C7cr) -> &Self { 
        self.c7cr_reg().write(value);
        self
    }

    #[doc="Set the C7CR register."]
    #[inline] pub fn set_c7cr<F: FnOnce(C7cr) -> C7cr>(&self, f: F) -> &Self {
        self.c7cr_reg().set(f);
        self
    }

    #[doc="Modify the C7CR register."]
    #[inline] pub fn with_c7cr<F: FnOnce(C7cr) -> C7cr>(&self, f: F) -> &Self {
        self.c7cr_reg().with(f);
        self
    }

    #[doc="Get the C8CR Register."]
    #[inline] pub fn c8cr_reg(&self) -> ::bobbin_mcu::register::Register<C8cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C8cr, 0x20)
    }

    #[doc="Get the *mut pointer for the C8CR register."]
    #[inline] pub fn c8cr_mut(&self) -> *mut C8cr { 
        self.c8cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C8CR register."]
    #[inline] pub fn c8cr_ptr(&self) -> *const C8cr { 
        self.c8cr_reg().ptr()
    }

    #[doc="Read the C8CR register."]
    #[inline] pub fn c8cr(&self) -> C8cr { 
        self.c8cr_reg().read()
    }

    #[doc="Write the C8CR register."]
    #[inline] pub fn write_c8cr(&self, value: C8cr) -> &Self { 
        self.c8cr_reg().write(value);
        self
    }

    #[doc="Set the C8CR register."]
    #[inline] pub fn set_c8cr<F: FnOnce(C8cr) -> C8cr>(&self, f: F) -> &Self {
        self.c8cr_reg().set(f);
        self
    }

    #[doc="Modify the C8CR register."]
    #[inline] pub fn with_c8cr<F: FnOnce(C8cr) -> C8cr>(&self, f: F) -> &Self {
        self.c8cr_reg().with(f);
        self
    }

    #[doc="Get the C9CR Register."]
    #[inline] pub fn c9cr_reg(&self) -> ::bobbin_mcu::register::Register<C9cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C9cr, 0x24)
    }

    #[doc="Get the *mut pointer for the C9CR register."]
    #[inline] pub fn c9cr_mut(&self) -> *mut C9cr { 
        self.c9cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C9CR register."]
    #[inline] pub fn c9cr_ptr(&self) -> *const C9cr { 
        self.c9cr_reg().ptr()
    }

    #[doc="Read the C9CR register."]
    #[inline] pub fn c9cr(&self) -> C9cr { 
        self.c9cr_reg().read()
    }

    #[doc="Write the C9CR register."]
    #[inline] pub fn write_c9cr(&self, value: C9cr) -> &Self { 
        self.c9cr_reg().write(value);
        self
    }

    #[doc="Set the C9CR register."]
    #[inline] pub fn set_c9cr<F: FnOnce(C9cr) -> C9cr>(&self, f: F) -> &Self {
        self.c9cr_reg().set(f);
        self
    }

    #[doc="Modify the C9CR register."]
    #[inline] pub fn with_c9cr<F: FnOnce(C9cr) -> C9cr>(&self, f: F) -> &Self {
        self.c9cr_reg().with(f);
        self
    }

    #[doc="Get the C10CR Register."]
    #[inline] pub fn c10cr_reg(&self) -> ::bobbin_mcu::register::Register<C10cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C10cr, 0x28)
    }

    #[doc="Get the *mut pointer for the C10CR register."]
    #[inline] pub fn c10cr_mut(&self) -> *mut C10cr { 
        self.c10cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C10CR register."]
    #[inline] pub fn c10cr_ptr(&self) -> *const C10cr { 
        self.c10cr_reg().ptr()
    }

    #[doc="Read the C10CR register."]
    #[inline] pub fn c10cr(&self) -> C10cr { 
        self.c10cr_reg().read()
    }

    #[doc="Write the C10CR register."]
    #[inline] pub fn write_c10cr(&self, value: C10cr) -> &Self { 
        self.c10cr_reg().write(value);
        self
    }

    #[doc="Set the C10CR register."]
    #[inline] pub fn set_c10cr<F: FnOnce(C10cr) -> C10cr>(&self, f: F) -> &Self {
        self.c10cr_reg().set(f);
        self
    }

    #[doc="Modify the C10CR register."]
    #[inline] pub fn with_c10cr<F: FnOnce(C10cr) -> C10cr>(&self, f: F) -> &Self {
        self.c10cr_reg().with(f);
        self
    }

    #[doc="Get the C11CR Register."]
    #[inline] pub fn c11cr_reg(&self) -> ::bobbin_mcu::register::Register<C11cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C11cr, 0x2c)
    }

    #[doc="Get the *mut pointer for the C11CR register."]
    #[inline] pub fn c11cr_mut(&self) -> *mut C11cr { 
        self.c11cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C11CR register."]
    #[inline] pub fn c11cr_ptr(&self) -> *const C11cr { 
        self.c11cr_reg().ptr()
    }

    #[doc="Read the C11CR register."]
    #[inline] pub fn c11cr(&self) -> C11cr { 
        self.c11cr_reg().read()
    }

    #[doc="Write the C11CR register."]
    #[inline] pub fn write_c11cr(&self, value: C11cr) -> &Self { 
        self.c11cr_reg().write(value);
        self
    }

    #[doc="Set the C11CR register."]
    #[inline] pub fn set_c11cr<F: FnOnce(C11cr) -> C11cr>(&self, f: F) -> &Self {
        self.c11cr_reg().set(f);
        self
    }

    #[doc="Modify the C11CR register."]
    #[inline] pub fn with_c11cr<F: FnOnce(C11cr) -> C11cr>(&self, f: F) -> &Self {
        self.c11cr_reg().with(f);
        self
    }

    #[doc="Get the C12CR Register."]
    #[inline] pub fn c12cr_reg(&self) -> ::bobbin_mcu::register::Register<C12cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C12cr, 0x30)
    }

    #[doc="Get the *mut pointer for the C12CR register."]
    #[inline] pub fn c12cr_mut(&self) -> *mut C12cr { 
        self.c12cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C12CR register."]
    #[inline] pub fn c12cr_ptr(&self) -> *const C12cr { 
        self.c12cr_reg().ptr()
    }

    #[doc="Read the C12CR register."]
    #[inline] pub fn c12cr(&self) -> C12cr { 
        self.c12cr_reg().read()
    }

    #[doc="Write the C12CR register."]
    #[inline] pub fn write_c12cr(&self, value: C12cr) -> &Self { 
        self.c12cr_reg().write(value);
        self
    }

    #[doc="Set the C12CR register."]
    #[inline] pub fn set_c12cr<F: FnOnce(C12cr) -> C12cr>(&self, f: F) -> &Self {
        self.c12cr_reg().set(f);
        self
    }

    #[doc="Modify the C12CR register."]
    #[inline] pub fn with_c12cr<F: FnOnce(C12cr) -> C12cr>(&self, f: F) -> &Self {
        self.c12cr_reg().with(f);
        self
    }

    #[doc="Get the C13CR Register."]
    #[inline] pub fn c13cr_reg(&self) -> ::bobbin_mcu::register::Register<C13cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C13cr, 0x34)
    }

    #[doc="Get the *mut pointer for the C13CR register."]
    #[inline] pub fn c13cr_mut(&self) -> *mut C13cr { 
        self.c13cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C13CR register."]
    #[inline] pub fn c13cr_ptr(&self) -> *const C13cr { 
        self.c13cr_reg().ptr()
    }

    #[doc="Read the C13CR register."]
    #[inline] pub fn c13cr(&self) -> C13cr { 
        self.c13cr_reg().read()
    }

    #[doc="Write the C13CR register."]
    #[inline] pub fn write_c13cr(&self, value: C13cr) -> &Self { 
        self.c13cr_reg().write(value);
        self
    }

    #[doc="Set the C13CR register."]
    #[inline] pub fn set_c13cr<F: FnOnce(C13cr) -> C13cr>(&self, f: F) -> &Self {
        self.c13cr_reg().set(f);
        self
    }

    #[doc="Modify the C13CR register."]
    #[inline] pub fn with_c13cr<F: FnOnce(C13cr) -> C13cr>(&self, f: F) -> &Self {
        self.c13cr_reg().with(f);
        self
    }

    #[doc="Get the CSR Register."]
    #[inline] pub fn csr_reg(&self) -> ::bobbin_mcu::register::Register<Csr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr, 0x80)
    }

    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut Csr { 
        self.csr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR register."]
    #[inline] pub fn csr_ptr(&self) -> *const Csr { 
        self.csr_reg().ptr()
    }

    #[doc="Read the CSR register."]
    #[inline] pub fn csr(&self) -> Csr { 
        self.csr_reg().read()
    }

    #[doc="Get the CFR Register."]
    #[inline] pub fn cfr_reg(&self) -> ::bobbin_mcu::register::Register<Cfr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfr, 0x84)
    }

    #[doc="Get the *mut pointer for the CFR register."]
    #[inline] pub fn cfr_mut(&self) -> *mut Cfr { 
        self.cfr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFR register."]
    #[inline] pub fn cfr_ptr(&self) -> *const Cfr { 
        self.cfr_reg().ptr()
    }

    #[doc="Write the CFR register."]
    #[inline] pub fn write_cfr(&self, value: Cfr) -> &Self { 
        self.cfr_reg().write(value);
        self
    }

    #[doc="Set the CFR register."]
    #[inline] pub fn set_cfr<F: FnOnce(Cfr) -> Cfr>(&self, f: F) -> &Self {
        self.cfr_reg().set(f);
        self
    }

    #[doc="Get the RG0CR Register."]
    #[inline] pub fn rg0cr_reg(&self) -> ::bobbin_mcu::register::Register<Rg0cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rg0cr, 0x100)
    }

    #[doc="Get the *mut pointer for the RG0CR register."]
    #[inline] pub fn rg0cr_mut(&self) -> *mut Rg0cr { 
        self.rg0cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RG0CR register."]
    #[inline] pub fn rg0cr_ptr(&self) -> *const Rg0cr { 
        self.rg0cr_reg().ptr()
    }

    #[doc="Read the RG0CR register."]
    #[inline] pub fn rg0cr(&self) -> Rg0cr { 
        self.rg0cr_reg().read()
    }

    #[doc="Write the RG0CR register."]
    #[inline] pub fn write_rg0cr(&self, value: Rg0cr) -> &Self { 
        self.rg0cr_reg().write(value);
        self
    }

    #[doc="Set the RG0CR register."]
    #[inline] pub fn set_rg0cr<F: FnOnce(Rg0cr) -> Rg0cr>(&self, f: F) -> &Self {
        self.rg0cr_reg().set(f);
        self
    }

    #[doc="Modify the RG0CR register."]
    #[inline] pub fn with_rg0cr<F: FnOnce(Rg0cr) -> Rg0cr>(&self, f: F) -> &Self {
        self.rg0cr_reg().with(f);
        self
    }

    #[doc="Get the RG1CR Register."]
    #[inline] pub fn rg1cr_reg(&self) -> ::bobbin_mcu::register::Register<Rg1cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rg1cr, 0x104)
    }

    #[doc="Get the *mut pointer for the RG1CR register."]
    #[inline] pub fn rg1cr_mut(&self) -> *mut Rg1cr { 
        self.rg1cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RG1CR register."]
    #[inline] pub fn rg1cr_ptr(&self) -> *const Rg1cr { 
        self.rg1cr_reg().ptr()
    }

    #[doc="Read the RG1CR register."]
    #[inline] pub fn rg1cr(&self) -> Rg1cr { 
        self.rg1cr_reg().read()
    }

    #[doc="Write the RG1CR register."]
    #[inline] pub fn write_rg1cr(&self, value: Rg1cr) -> &Self { 
        self.rg1cr_reg().write(value);
        self
    }

    #[doc="Set the RG1CR register."]
    #[inline] pub fn set_rg1cr<F: FnOnce(Rg1cr) -> Rg1cr>(&self, f: F) -> &Self {
        self.rg1cr_reg().set(f);
        self
    }

    #[doc="Modify the RG1CR register."]
    #[inline] pub fn with_rg1cr<F: FnOnce(Rg1cr) -> Rg1cr>(&self, f: F) -> &Self {
        self.rg1cr_reg().with(f);
        self
    }

    #[doc="Get the RG2CR Register."]
    #[inline] pub fn rg2cr_reg(&self) -> ::bobbin_mcu::register::Register<Rg2cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rg2cr, 0x108)
    }

    #[doc="Get the *mut pointer for the RG2CR register."]
    #[inline] pub fn rg2cr_mut(&self) -> *mut Rg2cr { 
        self.rg2cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RG2CR register."]
    #[inline] pub fn rg2cr_ptr(&self) -> *const Rg2cr { 
        self.rg2cr_reg().ptr()
    }

    #[doc="Read the RG2CR register."]
    #[inline] pub fn rg2cr(&self) -> Rg2cr { 
        self.rg2cr_reg().read()
    }

    #[doc="Write the RG2CR register."]
    #[inline] pub fn write_rg2cr(&self, value: Rg2cr) -> &Self { 
        self.rg2cr_reg().write(value);
        self
    }

    #[doc="Set the RG2CR register."]
    #[inline] pub fn set_rg2cr<F: FnOnce(Rg2cr) -> Rg2cr>(&self, f: F) -> &Self {
        self.rg2cr_reg().set(f);
        self
    }

    #[doc="Modify the RG2CR register."]
    #[inline] pub fn with_rg2cr<F: FnOnce(Rg2cr) -> Rg2cr>(&self, f: F) -> &Self {
        self.rg2cr_reg().with(f);
        self
    }

    #[doc="Get the RG3CR Register."]
    #[inline] pub fn rg3cr_reg(&self) -> ::bobbin_mcu::register::Register<Rg3cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rg3cr, 0x10c)
    }

    #[doc="Get the *mut pointer for the RG3CR register."]
    #[inline] pub fn rg3cr_mut(&self) -> *mut Rg3cr { 
        self.rg3cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RG3CR register."]
    #[inline] pub fn rg3cr_ptr(&self) -> *const Rg3cr { 
        self.rg3cr_reg().ptr()
    }

    #[doc="Read the RG3CR register."]
    #[inline] pub fn rg3cr(&self) -> Rg3cr { 
        self.rg3cr_reg().read()
    }

    #[doc="Write the RG3CR register."]
    #[inline] pub fn write_rg3cr(&self, value: Rg3cr) -> &Self { 
        self.rg3cr_reg().write(value);
        self
    }

    #[doc="Set the RG3CR register."]
    #[inline] pub fn set_rg3cr<F: FnOnce(Rg3cr) -> Rg3cr>(&self, f: F) -> &Self {
        self.rg3cr_reg().set(f);
        self
    }

    #[doc="Modify the RG3CR register."]
    #[inline] pub fn with_rg3cr<F: FnOnce(Rg3cr) -> Rg3cr>(&self, f: F) -> &Self {
        self.rg3cr_reg().with(f);
        self
    }

    #[doc="Get the RGSR Register."]
    #[inline] pub fn rgsr_reg(&self) -> ::bobbin_mcu::register::Register<Rgsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rgsr, 0x140)
    }

    #[doc="Get the *mut pointer for the RGSR register."]
    #[inline] pub fn rgsr_mut(&self) -> *mut Rgsr { 
        self.rgsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RGSR register."]
    #[inline] pub fn rgsr_ptr(&self) -> *const Rgsr { 
        self.rgsr_reg().ptr()
    }

    #[doc="Read the RGSR register."]
    #[inline] pub fn rgsr(&self) -> Rgsr { 
        self.rgsr_reg().read()
    }

    #[doc="Get the RGCFR Register."]
    #[inline] pub fn rgcfr_reg(&self) -> ::bobbin_mcu::register::Register<Rgcfr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rgcfr, 0x144)
    }

    #[doc="Get the *mut pointer for the RGCFR register."]
    #[inline] pub fn rgcfr_mut(&self) -> *mut Rgcfr { 
        self.rgcfr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RGCFR register."]
    #[inline] pub fn rgcfr_ptr(&self) -> *const Rgcfr { 
        self.rgcfr_reg().ptr()
    }

    #[doc="Read the RGCFR register."]
    #[inline] pub fn rgcfr(&self) -> Rgcfr { 
        self.rgcfr_reg().read()
    }

}

#[doc="DMA Multiplexer Channel 0 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C0cr(pub u32);
impl C0cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C0cr {
    #[inline]
    fn from(other: u32) -> Self {
         C0cr(other)
    }
}

impl ::core::fmt::Display for C0cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C0cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 1 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1cr(pub u32);
impl C1cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C1cr {
    #[inline]
    fn from(other: u32) -> Self {
         C1cr(other)
    }
}

impl ::core::fmt::Display for C1cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 2 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2cr(pub u32);
impl C2cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2cr {
    #[inline]
    fn from(other: u32) -> Self {
         C2cr(other)
    }
}

impl ::core::fmt::Display for C2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 3 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C3cr(pub u32);
impl C3cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C3cr {
    #[inline]
    fn from(other: u32) -> Self {
         C3cr(other)
    }
}

impl ::core::fmt::Display for C3cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C3cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 4 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C4cr(pub u32);
impl C4cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C4cr {
    #[inline]
    fn from(other: u32) -> Self {
         C4cr(other)
    }
}

impl ::core::fmt::Display for C4cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C4cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 5 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C5cr(pub u32);
impl C5cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C5cr {
    #[inline]
    fn from(other: u32) -> Self {
         C5cr(other)
    }
}

impl ::core::fmt::Display for C5cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C5cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 6 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C6cr(pub u32);
impl C6cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C6cr {
    #[inline]
    fn from(other: u32) -> Self {
         C6cr(other)
    }
}

impl ::core::fmt::Display for C6cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C6cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 7 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C7cr(pub u32);
impl C7cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C7cr {
    #[inline]
    fn from(other: u32) -> Self {
         C7cr(other)
    }
}

impl ::core::fmt::Display for C7cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C7cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 8 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C8cr(pub u32);
impl C8cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C8cr {
    #[inline]
    fn from(other: u32) -> Self {
         C8cr(other)
    }
}

impl ::core::fmt::Display for C8cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C8cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 9 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C9cr(pub u32);
impl C9cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C9cr {
    #[inline]
    fn from(other: u32) -> Self {
         C9cr(other)
    }
}

impl ::core::fmt::Display for C9cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C9cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 10 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C10cr(pub u32);
impl C10cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C10cr {
    #[inline]
    fn from(other: u32) -> Self {
         C10cr(other)
    }
}

impl ::core::fmt::Display for C10cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C10cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 11 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C11cr(pub u32);
impl C11cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C11cr {
    #[inline]
    fn from(other: u32) -> Self {
         C11cr(other)
    }
}

impl ::core::fmt::Display for C11cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C11cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 12 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C12cr(pub u32);
impl C12cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C12cr {
    #[inline]
    fn from(other: u32) -> Self {
         C12cr(other)
    }
}

impl ::core::fmt::Display for C12cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C12cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel 13 Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C13cr(pub u32);
impl C13cr {
    #[doc="SYNC_ID"]
    #[inline] pub fn sync_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SYNC_ID != 0"]
    #[inline] pub fn test_sync_id(&self) -> bool {
        self.sync_id() != 0
    }

    #[doc="Sets the SYNC_ID field."]
    #[inline] pub fn set_sync_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Nb request"]
    #[inline] pub fn nbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if NBREQ != 0"]
    #[inline] pub fn test_nbreq(&self) -> bool {
        self.nbreq() != 0
    }

    #[doc="Sets the NBREQ field."]
    #[inline] pub fn set_nbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Sync polarity"]
    #[inline] pub fn spol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SE != 0"]
    #[inline] pub fn test_se(&self) -> bool {
        self.se() != 0
    }

    #[doc="Sets the SE field."]
    #[inline] pub fn set_se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Event Generation Enable"]
    #[inline] pub fn ege(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EGE != 0"]
    #[inline] pub fn test_ege(&self) -> bool {
        self.ege() != 0
    }

    #[doc="Sets the EGE field."]
    #[inline] pub fn set_ege<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Interrupt Enable"]
    #[inline] pub fn soie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOIE != 0"]
    #[inline] pub fn test_soie(&self) -> bool {
        self.soie() != 0
    }

    #[doc="Sets the SOIE field."]
    #[inline] pub fn set_soie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Request ID"]
    #[inline] pub fn dmareq_id(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DMAREQ_ID != 0"]
    #[inline] pub fn test_dmareq_id(&self) -> bool {
        self.dmareq_id() != 0
    }

    #[doc="Sets the DMAREQ_ID field."]
    #[inline] pub fn set_dmareq_id<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C13cr {
    #[inline]
    fn from(other: u32) -> Self {
         C13cr(other)
    }
}

impl ::core::fmt::Display for C13cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C13cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_id() != 0 { try!(write!(f, " sync_id=0x{:x}", self.sync_id()))}
        if self.nbreq() != 0 { try!(write!(f, " nbreq=0x{:x}", self.nbreq()))}
        if self.spol() != 0 { try!(write!(f, " spol=0x{:x}", self.spol()))}
        if self.se() != 0 { try!(write!(f, " se"))}
        if self.ege() != 0 { try!(write!(f, " ege"))}
        if self.soie() != 0 { try!(write!(f, " soie"))}
        if self.dmareq_id() != 0 { try!(write!(f, " dmareq_id=0x{:x}", self.dmareq_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Multiplexer Channel Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc="Synchronization Overrun Flag 0"]
    #[inline] pub fn sof0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SOF0 != 0"]
    #[inline] pub fn test_sof0(&self) -> bool {
        self.sof0() != 0
    }

    #[doc="Sets the SOF0 field."]
    #[inline] pub fn set_sof0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Synchronization Overrun Flag 1"]
    #[inline] pub fn sof1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SOF1 != 0"]
    #[inline] pub fn test_sof1(&self) -> bool {
        self.sof1() != 0
    }

    #[doc="Sets the SOF1 field."]
    #[inline] pub fn set_sof1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Overrun Flag 2"]
    #[inline] pub fn sof2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SOF2 != 0"]
    #[inline] pub fn test_sof2(&self) -> bool {
        self.sof2() != 0
    }

    #[doc="Sets the SOF2 field."]
    #[inline] pub fn set_sof2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Synchronization Overrun Flag 3"]
    #[inline] pub fn sof3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SOF3 != 0"]
    #[inline] pub fn test_sof3(&self) -> bool {
        self.sof3() != 0
    }

    #[doc="Sets the SOF3 field."]
    #[inline] pub fn set_sof3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Synchronization Overrun Flag 4"]
    #[inline] pub fn sof4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SOF4 != 0"]
    #[inline] pub fn test_sof4(&self) -> bool {
        self.sof4() != 0
    }

    #[doc="Sets the SOF4 field."]
    #[inline] pub fn set_sof4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Synchronization Overrun Flag 5"]
    #[inline] pub fn sof5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SOF5 != 0"]
    #[inline] pub fn test_sof5(&self) -> bool {
        self.sof5() != 0
    }

    #[doc="Sets the SOF5 field."]
    #[inline] pub fn set_sof5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Synchronization Overrun Flag 6"]
    #[inline] pub fn sof6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SOF6 != 0"]
    #[inline] pub fn test_sof6(&self) -> bool {
        self.sof6() != 0
    }

    #[doc="Sets the SOF6 field."]
    #[inline] pub fn set_sof6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Synchronization Overrun Flag 7"]
    #[inline] pub fn sof7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SOF7 != 0"]
    #[inline] pub fn test_sof7(&self) -> bool {
        self.sof7() != 0
    }

    #[doc="Sets the SOF7 field."]
    #[inline] pub fn set_sof7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Synchronization Overrun Flag 8"]
    #[inline] pub fn sof8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOF8 != 0"]
    #[inline] pub fn test_sof8(&self) -> bool {
        self.sof8() != 0
    }

    #[doc="Sets the SOF8 field."]
    #[inline] pub fn set_sof8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Synchronization Overrun Flag 9"]
    #[inline] pub fn sof9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SOF9 != 0"]
    #[inline] pub fn test_sof9(&self) -> bool {
        self.sof9() != 0
    }

    #[doc="Sets the SOF9 field."]
    #[inline] pub fn set_sof9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Overrun Flag 10"]
    #[inline] pub fn sof10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SOF10 != 0"]
    #[inline] pub fn test_sof10(&self) -> bool {
        self.sof10() != 0
    }

    #[doc="Sets the SOF10 field."]
    #[inline] pub fn set_sof10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Synchronization Overrun Flag 11"]
    #[inline] pub fn sof11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SOF11 != 0"]
    #[inline] pub fn test_sof11(&self) -> bool {
        self.sof11() != 0
    }

    #[doc="Sets the SOF11 field."]
    #[inline] pub fn set_sof11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Synchronization Overrun Flag 12"]
    #[inline] pub fn sof12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SOF12 != 0"]
    #[inline] pub fn test_sof12(&self) -> bool {
        self.sof12() != 0
    }

    #[doc="Sets the SOF12 field."]
    #[inline] pub fn set_sof12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Synchronization Overrun Flag 13"]
    #[inline] pub fn sof13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SOF13 != 0"]
    #[inline] pub fn test_sof13(&self) -> bool {
        self.sof13() != 0
    }

    #[doc="Sets the SOF13 field."]
    #[inline] pub fn set_sof13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Csr(other)
    }
}

impl ::core::fmt::Display for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sof0() != 0 { try!(write!(f, " sof0"))}
        if self.sof1() != 0 { try!(write!(f, " sof1"))}
        if self.sof2() != 0 { try!(write!(f, " sof2"))}
        if self.sof3() != 0 { try!(write!(f, " sof3"))}
        if self.sof4() != 0 { try!(write!(f, " sof4"))}
        if self.sof5() != 0 { try!(write!(f, " sof5"))}
        if self.sof6() != 0 { try!(write!(f, " sof6"))}
        if self.sof7() != 0 { try!(write!(f, " sof7"))}
        if self.sof8() != 0 { try!(write!(f, " sof8"))}
        if self.sof9() != 0 { try!(write!(f, " sof9"))}
        if self.sof10() != 0 { try!(write!(f, " sof10"))}
        if self.sof11() != 0 { try!(write!(f, " sof11"))}
        if self.sof12() != 0 { try!(write!(f, " sof12"))}
        if self.sof13() != 0 { try!(write!(f, " sof13"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Channel Clear Flag Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfr(pub u32);
impl Cfr {
    #[doc="Synchronization Clear Overrun Flag 0"]
    #[inline] pub fn csof0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CSOF0 != 0"]
    #[inline] pub fn test_csof0(&self) -> bool {
        self.csof0() != 0
    }

    #[doc="Sets the CSOF0 field."]
    #[inline] pub fn set_csof0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 1"]
    #[inline] pub fn csof1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CSOF1 != 0"]
    #[inline] pub fn test_csof1(&self) -> bool {
        self.csof1() != 0
    }

    #[doc="Sets the CSOF1 field."]
    #[inline] pub fn set_csof1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 2"]
    #[inline] pub fn csof2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CSOF2 != 0"]
    #[inline] pub fn test_csof2(&self) -> bool {
        self.csof2() != 0
    }

    #[doc="Sets the CSOF2 field."]
    #[inline] pub fn set_csof2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 3"]
    #[inline] pub fn csof3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CSOF3 != 0"]
    #[inline] pub fn test_csof3(&self) -> bool {
        self.csof3() != 0
    }

    #[doc="Sets the CSOF3 field."]
    #[inline] pub fn set_csof3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 4"]
    #[inline] pub fn csof4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CSOF4 != 0"]
    #[inline] pub fn test_csof4(&self) -> bool {
        self.csof4() != 0
    }

    #[doc="Sets the CSOF4 field."]
    #[inline] pub fn set_csof4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 5"]
    #[inline] pub fn csof5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CSOF5 != 0"]
    #[inline] pub fn test_csof5(&self) -> bool {
        self.csof5() != 0
    }

    #[doc="Sets the CSOF5 field."]
    #[inline] pub fn set_csof5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 6"]
    #[inline] pub fn csof6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CSOF6 != 0"]
    #[inline] pub fn test_csof6(&self) -> bool {
        self.csof6() != 0
    }

    #[doc="Sets the CSOF6 field."]
    #[inline] pub fn set_csof6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 7"]
    #[inline] pub fn csof7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CSOF7 != 0"]
    #[inline] pub fn test_csof7(&self) -> bool {
        self.csof7() != 0
    }

    #[doc="Sets the CSOF7 field."]
    #[inline] pub fn set_csof7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 8"]
    #[inline] pub fn csof8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CSOF8 != 0"]
    #[inline] pub fn test_csof8(&self) -> bool {
        self.csof8() != 0
    }

    #[doc="Sets the CSOF8 field."]
    #[inline] pub fn set_csof8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 9"]
    #[inline] pub fn csof9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CSOF9 != 0"]
    #[inline] pub fn test_csof9(&self) -> bool {
        self.csof9() != 0
    }

    #[doc="Sets the CSOF9 field."]
    #[inline] pub fn set_csof9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 10"]
    #[inline] pub fn csof10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CSOF10 != 0"]
    #[inline] pub fn test_csof10(&self) -> bool {
        self.csof10() != 0
    }

    #[doc="Sets the CSOF10 field."]
    #[inline] pub fn set_csof10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 11"]
    #[inline] pub fn csof11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CSOF11 != 0"]
    #[inline] pub fn test_csof11(&self) -> bool {
        self.csof11() != 0
    }

    #[doc="Sets the CSOF11 field."]
    #[inline] pub fn set_csof11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 12"]
    #[inline] pub fn csof12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CSOF12 != 0"]
    #[inline] pub fn test_csof12(&self) -> bool {
        self.csof12() != 0
    }

    #[doc="Sets the CSOF12 field."]
    #[inline] pub fn set_csof12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Synchronization Clear Overrun Flag 13"]
    #[inline] pub fn csof13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CSOF13 != 0"]
    #[inline] pub fn test_csof13(&self) -> bool {
        self.csof13() != 0
    }

    #[doc="Sets the CSOF13 field."]
    #[inline] pub fn set_csof13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Cfr {
    #[inline]
    fn from(other: u32) -> Self {
         Cfr(other)
    }
}

impl ::core::fmt::Display for Cfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.csof0() != 0 { try!(write!(f, " csof0"))}
        if self.csof1() != 0 { try!(write!(f, " csof1"))}
        if self.csof2() != 0 { try!(write!(f, " csof2"))}
        if self.csof3() != 0 { try!(write!(f, " csof3"))}
        if self.csof4() != 0 { try!(write!(f, " csof4"))}
        if self.csof5() != 0 { try!(write!(f, " csof5"))}
        if self.csof6() != 0 { try!(write!(f, " csof6"))}
        if self.csof7() != 0 { try!(write!(f, " csof7"))}
        if self.csof8() != 0 { try!(write!(f, " csof8"))}
        if self.csof9() != 0 { try!(write!(f, " csof9"))}
        if self.csof10() != 0 { try!(write!(f, " csof10"))}
        if self.csof11() != 0 { try!(write!(f, " csof11"))}
        if self.csof12() != 0 { try!(write!(f, " csof12"))}
        if self.csof13() != 0 { try!(write!(f, " csof13"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Request Generator 0 Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rg0cr(pub u32);
impl Rg0cr {
    #[doc="Number of Request"]
    #[inline] pub fn gnbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if GNBREQ != 0"]
    #[inline] pub fn test_gnbreq(&self) -> bool {
        self.gnbreq() != 0
    }

    #[doc="Sets the GNBREQ field."]
    #[inline] pub fn set_gnbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Generation Polarity"]
    #[inline] pub fn gpol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if GPOL != 0"]
    #[inline] pub fn test_gpol(&self) -> bool {
        self.gpol() != 0
    }

    #[doc="Sets the GPOL field."]
    #[inline] pub fn set_gpol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Generation Enable"]
    #[inline] pub fn ge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GE != 0"]
    #[inline] pub fn test_ge(&self) -> bool {
        self.ge() != 0
    }

    #[doc="Sets the GE field."]
    #[inline] pub fn set_ge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn oie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OIE != 0"]
    #[inline] pub fn test_oie(&self) -> bool {
        self.oie() != 0
    }

    #[doc="Sets the OIE field."]
    #[inline] pub fn set_oie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Signal ID"]
    #[inline] pub fn sig_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SIG_ID != 0"]
    #[inline] pub fn test_sig_id(&self) -> bool {
        self.sig_id() != 0
    }

    #[doc="Sets the SIG_ID field."]
    #[inline] pub fn set_sig_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rg0cr {
    #[inline]
    fn from(other: u32) -> Self {
         Rg0cr(other)
    }
}

impl ::core::fmt::Display for Rg0cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rg0cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gnbreq() != 0 { try!(write!(f, " gnbreq=0x{:x}", self.gnbreq()))}
        if self.gpol() != 0 { try!(write!(f, " gpol=0x{:x}", self.gpol()))}
        if self.ge() != 0 { try!(write!(f, " ge"))}
        if self.oie() != 0 { try!(write!(f, " oie"))}
        if self.sig_id() != 0 { try!(write!(f, " sig_id=0x{:x}", self.sig_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Request Generator 1 Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rg1cr(pub u32);
impl Rg1cr {
    #[doc="Number of Request"]
    #[inline] pub fn gnbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if GNBREQ != 0"]
    #[inline] pub fn test_gnbreq(&self) -> bool {
        self.gnbreq() != 0
    }

    #[doc="Sets the GNBREQ field."]
    #[inline] pub fn set_gnbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Generation Polarity"]
    #[inline] pub fn gpol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if GPOL != 0"]
    #[inline] pub fn test_gpol(&self) -> bool {
        self.gpol() != 0
    }

    #[doc="Sets the GPOL field."]
    #[inline] pub fn set_gpol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Generation Enable"]
    #[inline] pub fn ge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GE != 0"]
    #[inline] pub fn test_ge(&self) -> bool {
        self.ge() != 0
    }

    #[doc="Sets the GE field."]
    #[inline] pub fn set_ge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn oie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OIE != 0"]
    #[inline] pub fn test_oie(&self) -> bool {
        self.oie() != 0
    }

    #[doc="Sets the OIE field."]
    #[inline] pub fn set_oie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Signal ID"]
    #[inline] pub fn sig_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SIG_ID != 0"]
    #[inline] pub fn test_sig_id(&self) -> bool {
        self.sig_id() != 0
    }

    #[doc="Sets the SIG_ID field."]
    #[inline] pub fn set_sig_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rg1cr {
    #[inline]
    fn from(other: u32) -> Self {
         Rg1cr(other)
    }
}

impl ::core::fmt::Display for Rg1cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rg1cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gnbreq() != 0 { try!(write!(f, " gnbreq=0x{:x}", self.gnbreq()))}
        if self.gpol() != 0 { try!(write!(f, " gpol=0x{:x}", self.gpol()))}
        if self.ge() != 0 { try!(write!(f, " ge"))}
        if self.oie() != 0 { try!(write!(f, " oie"))}
        if self.sig_id() != 0 { try!(write!(f, " sig_id=0x{:x}", self.sig_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Request Generator 2 Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rg2cr(pub u32);
impl Rg2cr {
    #[doc="Number of Request"]
    #[inline] pub fn gnbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if GNBREQ != 0"]
    #[inline] pub fn test_gnbreq(&self) -> bool {
        self.gnbreq() != 0
    }

    #[doc="Sets the GNBREQ field."]
    #[inline] pub fn set_gnbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Generation Polarity"]
    #[inline] pub fn gpol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if GPOL != 0"]
    #[inline] pub fn test_gpol(&self) -> bool {
        self.gpol() != 0
    }

    #[doc="Sets the GPOL field."]
    #[inline] pub fn set_gpol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Generation Enable"]
    #[inline] pub fn ge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GE != 0"]
    #[inline] pub fn test_ge(&self) -> bool {
        self.ge() != 0
    }

    #[doc="Sets the GE field."]
    #[inline] pub fn set_ge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn oie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OIE != 0"]
    #[inline] pub fn test_oie(&self) -> bool {
        self.oie() != 0
    }

    #[doc="Sets the OIE field."]
    #[inline] pub fn set_oie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Signal ID"]
    #[inline] pub fn sig_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SIG_ID != 0"]
    #[inline] pub fn test_sig_id(&self) -> bool {
        self.sig_id() != 0
    }

    #[doc="Sets the SIG_ID field."]
    #[inline] pub fn set_sig_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rg2cr {
    #[inline]
    fn from(other: u32) -> Self {
         Rg2cr(other)
    }
}

impl ::core::fmt::Display for Rg2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rg2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gnbreq() != 0 { try!(write!(f, " gnbreq=0x{:x}", self.gnbreq()))}
        if self.gpol() != 0 { try!(write!(f, " gpol=0x{:x}", self.gpol()))}
        if self.ge() != 0 { try!(write!(f, " ge"))}
        if self.oie() != 0 { try!(write!(f, " oie"))}
        if self.sig_id() != 0 { try!(write!(f, " sig_id=0x{:x}", self.sig_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Request Generator 3 Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rg3cr(pub u32);
impl Rg3cr {
    #[doc="Number of Request"]
    #[inline] pub fn gnbreq(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1f) as u8) } // [23:19]
    }

    #[doc="Returns true if GNBREQ != 0"]
    #[inline] pub fn test_gnbreq(&self) -> bool {
        self.gnbreq() != 0
    }

    #[doc="Sets the GNBREQ field."]
    #[inline] pub fn set_gnbreq<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Generation Polarity"]
    #[inline] pub fn gpol(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if GPOL != 0"]
    #[inline] pub fn test_gpol(&self) -> bool {
        self.gpol() != 0
    }

    #[doc="Sets the GPOL field."]
    #[inline] pub fn set_gpol<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Generation Enable"]
    #[inline] pub fn ge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GE != 0"]
    #[inline] pub fn test_ge(&self) -> bool {
        self.ge() != 0
    }

    #[doc="Sets the GE field."]
    #[inline] pub fn set_ge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn oie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OIE != 0"]
    #[inline] pub fn test_oie(&self) -> bool {
        self.oie() != 0
    }

    #[doc="Sets the OIE field."]
    #[inline] pub fn set_oie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Signal ID"]
    #[inline] pub fn sig_id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SIG_ID != 0"]
    #[inline] pub fn test_sig_id(&self) -> bool {
        self.sig_id() != 0
    }

    #[doc="Sets the SIG_ID field."]
    #[inline] pub fn set_sig_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rg3cr {
    #[inline]
    fn from(other: u32) -> Self {
         Rg3cr(other)
    }
}

impl ::core::fmt::Display for Rg3cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rg3cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gnbreq() != 0 { try!(write!(f, " gnbreq=0x{:x}", self.gnbreq()))}
        if self.gpol() != 0 { try!(write!(f, " gpol=0x{:x}", self.gpol()))}
        if self.ge() != 0 { try!(write!(f, " ge"))}
        if self.oie() != 0 { try!(write!(f, " oie"))}
        if self.sig_id() != 0 { try!(write!(f, " sig_id=0x{:x}", self.sig_id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Request Generator Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgsr(pub u32);
impl Rgsr {
    #[doc="Generator Overrun Flag 0"]
    #[inline] pub fn of0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OF0 != 0"]
    #[inline] pub fn test_of0(&self) -> bool {
        self.of0() != 0
    }

    #[doc="Sets the OF0 field."]
    #[inline] pub fn set_of0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Generator Overrun Flag 1"]
    #[inline] pub fn of1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OF1 != 0"]
    #[inline] pub fn test_of1(&self) -> bool {
        self.of1() != 0
    }

    #[doc="Sets the OF1 field."]
    #[inline] pub fn set_of1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Generator Overrun Flag 2"]
    #[inline] pub fn of2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OF2 != 0"]
    #[inline] pub fn test_of2(&self) -> bool {
        self.of2() != 0
    }

    #[doc="Sets the OF2 field."]
    #[inline] pub fn set_of2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Generator Overrun Flag 3"]
    #[inline] pub fn of3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OF3 != 0"]
    #[inline] pub fn test_of3(&self) -> bool {
        self.of3() != 0
    }

    #[doc="Sets the OF3 field."]
    #[inline] pub fn set_of3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Rgsr {
    #[inline]
    fn from(other: u32) -> Self {
         Rgsr(other)
    }
}

impl ::core::fmt::Display for Rgsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rgsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.of0() != 0 { try!(write!(f, " of0"))}
        if self.of1() != 0 { try!(write!(f, " of1"))}
        if self.of2() != 0 { try!(write!(f, " of2"))}
        if self.of3() != 0 { try!(write!(f, " of3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Request Generator Clear Flag Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgcfr(pub u32);
impl Rgcfr {
    #[doc="Generator Clear Overrun Flag 0"]
    #[inline] pub fn csof0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CSOF0 != 0"]
    #[inline] pub fn test_csof0(&self) -> bool {
        self.csof0() != 0
    }

    #[doc="Sets the CSOF0 field."]
    #[inline] pub fn set_csof0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Generator Clear Overrun Flag 1"]
    #[inline] pub fn csof1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CSOF1 != 0"]
    #[inline] pub fn test_csof1(&self) -> bool {
        self.csof1() != 0
    }

    #[doc="Sets the CSOF1 field."]
    #[inline] pub fn set_csof1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Generator Clear Overrun Flag 2"]
    #[inline] pub fn csof2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CSOF2 != 0"]
    #[inline] pub fn test_csof2(&self) -> bool {
        self.csof2() != 0
    }

    #[doc="Sets the CSOF2 field."]
    #[inline] pub fn set_csof2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Generator Clear Overrun Flag 3"]
    #[inline] pub fn csof3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CSOF3 != 0"]
    #[inline] pub fn test_csof3(&self) -> bool {
        self.csof3() != 0
    }

    #[doc="Sets the CSOF3 field."]
    #[inline] pub fn set_csof3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Rgcfr {
    #[inline]
    fn from(other: u32) -> Self {
         Rgcfr(other)
    }
}

impl ::core::fmt::Display for Rgcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rgcfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.csof0() != 0 { try!(write!(f, " csof0"))}
        if self.csof1() != 0 { try!(write!(f, " csof1"))}
        if self.csof2() != 0 { try!(write!(f, " csof2"))}
        if self.csof3() != 0 { try!(write!(f, " csof3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

