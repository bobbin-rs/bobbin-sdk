::bobbin_mcu::periph!( HSEM, Hsem, HSEM_PERIPH, HardwareSemaphorePeriph, HSEM_OWNED, HSEM_REF_COUNT, 0x58001400, 0x00, 0x22);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="HARDWARE_SEMAPHORE Peripheral"]
pub struct HardwareSemaphorePeriph(pub usize); 

impl HardwareSemaphorePeriph {
    #[doc="Get the R0 Register."]
    #[inline] pub fn r0_reg(&self) -> ::bobbin_mcu::register::Register<R0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R0, 0x0)
    }

    #[doc="Get the *mut pointer for the R0 register."]
    #[inline] pub fn r0_mut(&self) -> *mut R0 { 
        self.r0_reg().ptr()
    }

    #[doc="Get the *const pointer for the R0 register."]
    #[inline] pub fn r0_ptr(&self) -> *const R0 { 
        self.r0_reg().ptr()
    }

    #[doc="Read the R0 register."]
    #[inline] pub fn r0(&self) -> R0 { 
        self.r0_reg().read()
    }

    #[doc="Write the R0 register."]
    #[inline] pub fn write_r0(&self, value: R0) -> &Self { 
        self.r0_reg().write(value);
        self
    }

    #[doc="Set the R0 register."]
    #[inline] pub fn set_r0<F: FnOnce(R0) -> R0>(&self, f: F) -> &Self {
        self.r0_reg().set(f);
        self
    }

    #[doc="Modify the R0 register."]
    #[inline] pub fn with_r0<F: FnOnce(R0) -> R0>(&self, f: F) -> &Self {
        self.r0_reg().with(f);
        self
    }

    #[doc="Get the R1 Register."]
    #[inline] pub fn r1_reg(&self) -> ::bobbin_mcu::register::Register<R1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R1, 0x4)
    }

    #[doc="Get the *mut pointer for the R1 register."]
    #[inline] pub fn r1_mut(&self) -> *mut R1 { 
        self.r1_reg().ptr()
    }

    #[doc="Get the *const pointer for the R1 register."]
    #[inline] pub fn r1_ptr(&self) -> *const R1 { 
        self.r1_reg().ptr()
    }

    #[doc="Read the R1 register."]
    #[inline] pub fn r1(&self) -> R1 { 
        self.r1_reg().read()
    }

    #[doc="Write the R1 register."]
    #[inline] pub fn write_r1(&self, value: R1) -> &Self { 
        self.r1_reg().write(value);
        self
    }

    #[doc="Set the R1 register."]
    #[inline] pub fn set_r1<F: FnOnce(R1) -> R1>(&self, f: F) -> &Self {
        self.r1_reg().set(f);
        self
    }

    #[doc="Modify the R1 register."]
    #[inline] pub fn with_r1<F: FnOnce(R1) -> R1>(&self, f: F) -> &Self {
        self.r1_reg().with(f);
        self
    }

    #[doc="Get the R2 Register."]
    #[inline] pub fn r2_reg(&self) -> ::bobbin_mcu::register::Register<R2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R2, 0x8)
    }

    #[doc="Get the *mut pointer for the R2 register."]
    #[inline] pub fn r2_mut(&self) -> *mut R2 { 
        self.r2_reg().ptr()
    }

    #[doc="Get the *const pointer for the R2 register."]
    #[inline] pub fn r2_ptr(&self) -> *const R2 { 
        self.r2_reg().ptr()
    }

    #[doc="Read the R2 register."]
    #[inline] pub fn r2(&self) -> R2 { 
        self.r2_reg().read()
    }

    #[doc="Write the R2 register."]
    #[inline] pub fn write_r2(&self, value: R2) -> &Self { 
        self.r2_reg().write(value);
        self
    }

    #[doc="Set the R2 register."]
    #[inline] pub fn set_r2<F: FnOnce(R2) -> R2>(&self, f: F) -> &Self {
        self.r2_reg().set(f);
        self
    }

    #[doc="Modify the R2 register."]
    #[inline] pub fn with_r2<F: FnOnce(R2) -> R2>(&self, f: F) -> &Self {
        self.r2_reg().with(f);
        self
    }

    #[doc="Get the R3 Register."]
    #[inline] pub fn r3_reg(&self) -> ::bobbin_mcu::register::Register<R3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R3, 0xc)
    }

    #[doc="Get the *mut pointer for the R3 register."]
    #[inline] pub fn r3_mut(&self) -> *mut R3 { 
        self.r3_reg().ptr()
    }

    #[doc="Get the *const pointer for the R3 register."]
    #[inline] pub fn r3_ptr(&self) -> *const R3 { 
        self.r3_reg().ptr()
    }

    #[doc="Read the R3 register."]
    #[inline] pub fn r3(&self) -> R3 { 
        self.r3_reg().read()
    }

    #[doc="Write the R3 register."]
    #[inline] pub fn write_r3(&self, value: R3) -> &Self { 
        self.r3_reg().write(value);
        self
    }

    #[doc="Set the R3 register."]
    #[inline] pub fn set_r3<F: FnOnce(R3) -> R3>(&self, f: F) -> &Self {
        self.r3_reg().set(f);
        self
    }

    #[doc="Modify the R3 register."]
    #[inline] pub fn with_r3<F: FnOnce(R3) -> R3>(&self, f: F) -> &Self {
        self.r3_reg().with(f);
        self
    }

    #[doc="Get the R4 Register."]
    #[inline] pub fn r4_reg(&self) -> ::bobbin_mcu::register::Register<R4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R4, 0x10)
    }

    #[doc="Get the *mut pointer for the R4 register."]
    #[inline] pub fn r4_mut(&self) -> *mut R4 { 
        self.r4_reg().ptr()
    }

    #[doc="Get the *const pointer for the R4 register."]
    #[inline] pub fn r4_ptr(&self) -> *const R4 { 
        self.r4_reg().ptr()
    }

    #[doc="Read the R4 register."]
    #[inline] pub fn r4(&self) -> R4 { 
        self.r4_reg().read()
    }

    #[doc="Write the R4 register."]
    #[inline] pub fn write_r4(&self, value: R4) -> &Self { 
        self.r4_reg().write(value);
        self
    }

    #[doc="Set the R4 register."]
    #[inline] pub fn set_r4<F: FnOnce(R4) -> R4>(&self, f: F) -> &Self {
        self.r4_reg().set(f);
        self
    }

    #[doc="Modify the R4 register."]
    #[inline] pub fn with_r4<F: FnOnce(R4) -> R4>(&self, f: F) -> &Self {
        self.r4_reg().with(f);
        self
    }

    #[doc="Get the R5 Register."]
    #[inline] pub fn r5_reg(&self) -> ::bobbin_mcu::register::Register<R5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R5, 0x14)
    }

    #[doc="Get the *mut pointer for the R5 register."]
    #[inline] pub fn r5_mut(&self) -> *mut R5 { 
        self.r5_reg().ptr()
    }

    #[doc="Get the *const pointer for the R5 register."]
    #[inline] pub fn r5_ptr(&self) -> *const R5 { 
        self.r5_reg().ptr()
    }

    #[doc="Read the R5 register."]
    #[inline] pub fn r5(&self) -> R5 { 
        self.r5_reg().read()
    }

    #[doc="Write the R5 register."]
    #[inline] pub fn write_r5(&self, value: R5) -> &Self { 
        self.r5_reg().write(value);
        self
    }

    #[doc="Set the R5 register."]
    #[inline] pub fn set_r5<F: FnOnce(R5) -> R5>(&self, f: F) -> &Self {
        self.r5_reg().set(f);
        self
    }

    #[doc="Modify the R5 register."]
    #[inline] pub fn with_r5<F: FnOnce(R5) -> R5>(&self, f: F) -> &Self {
        self.r5_reg().with(f);
        self
    }

    #[doc="Get the R6 Register."]
    #[inline] pub fn r6_reg(&self) -> ::bobbin_mcu::register::Register<R6> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R6, 0x18)
    }

    #[doc="Get the *mut pointer for the R6 register."]
    #[inline] pub fn r6_mut(&self) -> *mut R6 { 
        self.r6_reg().ptr()
    }

    #[doc="Get the *const pointer for the R6 register."]
    #[inline] pub fn r6_ptr(&self) -> *const R6 { 
        self.r6_reg().ptr()
    }

    #[doc="Read the R6 register."]
    #[inline] pub fn r6(&self) -> R6 { 
        self.r6_reg().read()
    }

    #[doc="Write the R6 register."]
    #[inline] pub fn write_r6(&self, value: R6) -> &Self { 
        self.r6_reg().write(value);
        self
    }

    #[doc="Set the R6 register."]
    #[inline] pub fn set_r6<F: FnOnce(R6) -> R6>(&self, f: F) -> &Self {
        self.r6_reg().set(f);
        self
    }

    #[doc="Modify the R6 register."]
    #[inline] pub fn with_r6<F: FnOnce(R6) -> R6>(&self, f: F) -> &Self {
        self.r6_reg().with(f);
        self
    }

    #[doc="Get the R7 Register."]
    #[inline] pub fn r7_reg(&self) -> ::bobbin_mcu::register::Register<R7> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R7, 0x1c)
    }

    #[doc="Get the *mut pointer for the R7 register."]
    #[inline] pub fn r7_mut(&self) -> *mut R7 { 
        self.r7_reg().ptr()
    }

    #[doc="Get the *const pointer for the R7 register."]
    #[inline] pub fn r7_ptr(&self) -> *const R7 { 
        self.r7_reg().ptr()
    }

    #[doc="Read the R7 register."]
    #[inline] pub fn r7(&self) -> R7 { 
        self.r7_reg().read()
    }

    #[doc="Write the R7 register."]
    #[inline] pub fn write_r7(&self, value: R7) -> &Self { 
        self.r7_reg().write(value);
        self
    }

    #[doc="Set the R7 register."]
    #[inline] pub fn set_r7<F: FnOnce(R7) -> R7>(&self, f: F) -> &Self {
        self.r7_reg().set(f);
        self
    }

    #[doc="Modify the R7 register."]
    #[inline] pub fn with_r7<F: FnOnce(R7) -> R7>(&self, f: F) -> &Self {
        self.r7_reg().with(f);
        self
    }

    #[doc="Get the R8 Register."]
    #[inline] pub fn r8_reg(&self) -> ::bobbin_mcu::register::Register<R8> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R8, 0x20)
    }

    #[doc="Get the *mut pointer for the R8 register."]
    #[inline] pub fn r8_mut(&self) -> *mut R8 { 
        self.r8_reg().ptr()
    }

    #[doc="Get the *const pointer for the R8 register."]
    #[inline] pub fn r8_ptr(&self) -> *const R8 { 
        self.r8_reg().ptr()
    }

    #[doc="Read the R8 register."]
    #[inline] pub fn r8(&self) -> R8 { 
        self.r8_reg().read()
    }

    #[doc="Write the R8 register."]
    #[inline] pub fn write_r8(&self, value: R8) -> &Self { 
        self.r8_reg().write(value);
        self
    }

    #[doc="Set the R8 register."]
    #[inline] pub fn set_r8<F: FnOnce(R8) -> R8>(&self, f: F) -> &Self {
        self.r8_reg().set(f);
        self
    }

    #[doc="Modify the R8 register."]
    #[inline] pub fn with_r8<F: FnOnce(R8) -> R8>(&self, f: F) -> &Self {
        self.r8_reg().with(f);
        self
    }

    #[doc="Get the R9 Register."]
    #[inline] pub fn r9_reg(&self) -> ::bobbin_mcu::register::Register<R9> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R9, 0x24)
    }

    #[doc="Get the *mut pointer for the R9 register."]
    #[inline] pub fn r9_mut(&self) -> *mut R9 { 
        self.r9_reg().ptr()
    }

    #[doc="Get the *const pointer for the R9 register."]
    #[inline] pub fn r9_ptr(&self) -> *const R9 { 
        self.r9_reg().ptr()
    }

    #[doc="Read the R9 register."]
    #[inline] pub fn r9(&self) -> R9 { 
        self.r9_reg().read()
    }

    #[doc="Write the R9 register."]
    #[inline] pub fn write_r9(&self, value: R9) -> &Self { 
        self.r9_reg().write(value);
        self
    }

    #[doc="Set the R9 register."]
    #[inline] pub fn set_r9<F: FnOnce(R9) -> R9>(&self, f: F) -> &Self {
        self.r9_reg().set(f);
        self
    }

    #[doc="Modify the R9 register."]
    #[inline] pub fn with_r9<F: FnOnce(R9) -> R9>(&self, f: F) -> &Self {
        self.r9_reg().with(f);
        self
    }

    #[doc="Get the R10 Register."]
    #[inline] pub fn r10_reg(&self) -> ::bobbin_mcu::register::Register<R10> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R10, 0x28)
    }

    #[doc="Get the *mut pointer for the R10 register."]
    #[inline] pub fn r10_mut(&self) -> *mut R10 { 
        self.r10_reg().ptr()
    }

    #[doc="Get the *const pointer for the R10 register."]
    #[inline] pub fn r10_ptr(&self) -> *const R10 { 
        self.r10_reg().ptr()
    }

    #[doc="Read the R10 register."]
    #[inline] pub fn r10(&self) -> R10 { 
        self.r10_reg().read()
    }

    #[doc="Write the R10 register."]
    #[inline] pub fn write_r10(&self, value: R10) -> &Self { 
        self.r10_reg().write(value);
        self
    }

    #[doc="Set the R10 register."]
    #[inline] pub fn set_r10<F: FnOnce(R10) -> R10>(&self, f: F) -> &Self {
        self.r10_reg().set(f);
        self
    }

    #[doc="Modify the R10 register."]
    #[inline] pub fn with_r10<F: FnOnce(R10) -> R10>(&self, f: F) -> &Self {
        self.r10_reg().with(f);
        self
    }

    #[doc="Get the R11 Register."]
    #[inline] pub fn r11_reg(&self) -> ::bobbin_mcu::register::Register<R11> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R11, 0x2c)
    }

    #[doc="Get the *mut pointer for the R11 register."]
    #[inline] pub fn r11_mut(&self) -> *mut R11 { 
        self.r11_reg().ptr()
    }

    #[doc="Get the *const pointer for the R11 register."]
    #[inline] pub fn r11_ptr(&self) -> *const R11 { 
        self.r11_reg().ptr()
    }

    #[doc="Read the R11 register."]
    #[inline] pub fn r11(&self) -> R11 { 
        self.r11_reg().read()
    }

    #[doc="Write the R11 register."]
    #[inline] pub fn write_r11(&self, value: R11) -> &Self { 
        self.r11_reg().write(value);
        self
    }

    #[doc="Set the R11 register."]
    #[inline] pub fn set_r11<F: FnOnce(R11) -> R11>(&self, f: F) -> &Self {
        self.r11_reg().set(f);
        self
    }

    #[doc="Modify the R11 register."]
    #[inline] pub fn with_r11<F: FnOnce(R11) -> R11>(&self, f: F) -> &Self {
        self.r11_reg().with(f);
        self
    }

    #[doc="Get the R12 Register."]
    #[inline] pub fn r12_reg(&self) -> ::bobbin_mcu::register::Register<R12> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R12, 0x30)
    }

    #[doc="Get the *mut pointer for the R12 register."]
    #[inline] pub fn r12_mut(&self) -> *mut R12 { 
        self.r12_reg().ptr()
    }

    #[doc="Get the *const pointer for the R12 register."]
    #[inline] pub fn r12_ptr(&self) -> *const R12 { 
        self.r12_reg().ptr()
    }

    #[doc="Read the R12 register."]
    #[inline] pub fn r12(&self) -> R12 { 
        self.r12_reg().read()
    }

    #[doc="Write the R12 register."]
    #[inline] pub fn write_r12(&self, value: R12) -> &Self { 
        self.r12_reg().write(value);
        self
    }

    #[doc="Set the R12 register."]
    #[inline] pub fn set_r12<F: FnOnce(R12) -> R12>(&self, f: F) -> &Self {
        self.r12_reg().set(f);
        self
    }

    #[doc="Modify the R12 register."]
    #[inline] pub fn with_r12<F: FnOnce(R12) -> R12>(&self, f: F) -> &Self {
        self.r12_reg().with(f);
        self
    }

    #[doc="Get the R13 Register."]
    #[inline] pub fn r13_reg(&self) -> ::bobbin_mcu::register::Register<R13> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R13, 0x34)
    }

    #[doc="Get the *mut pointer for the R13 register."]
    #[inline] pub fn r13_mut(&self) -> *mut R13 { 
        self.r13_reg().ptr()
    }

    #[doc="Get the *const pointer for the R13 register."]
    #[inline] pub fn r13_ptr(&self) -> *const R13 { 
        self.r13_reg().ptr()
    }

    #[doc="Read the R13 register."]
    #[inline] pub fn r13(&self) -> R13 { 
        self.r13_reg().read()
    }

    #[doc="Write the R13 register."]
    #[inline] pub fn write_r13(&self, value: R13) -> &Self { 
        self.r13_reg().write(value);
        self
    }

    #[doc="Set the R13 register."]
    #[inline] pub fn set_r13<F: FnOnce(R13) -> R13>(&self, f: F) -> &Self {
        self.r13_reg().set(f);
        self
    }

    #[doc="Modify the R13 register."]
    #[inline] pub fn with_r13<F: FnOnce(R13) -> R13>(&self, f: F) -> &Self {
        self.r13_reg().with(f);
        self
    }

    #[doc="Get the R14 Register."]
    #[inline] pub fn r14_reg(&self) -> ::bobbin_mcu::register::Register<R14> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R14, 0x38)
    }

    #[doc="Get the *mut pointer for the R14 register."]
    #[inline] pub fn r14_mut(&self) -> *mut R14 { 
        self.r14_reg().ptr()
    }

    #[doc="Get the *const pointer for the R14 register."]
    #[inline] pub fn r14_ptr(&self) -> *const R14 { 
        self.r14_reg().ptr()
    }

    #[doc="Read the R14 register."]
    #[inline] pub fn r14(&self) -> R14 { 
        self.r14_reg().read()
    }

    #[doc="Write the R14 register."]
    #[inline] pub fn write_r14(&self, value: R14) -> &Self { 
        self.r14_reg().write(value);
        self
    }

    #[doc="Set the R14 register."]
    #[inline] pub fn set_r14<F: FnOnce(R14) -> R14>(&self, f: F) -> &Self {
        self.r14_reg().set(f);
        self
    }

    #[doc="Modify the R14 register."]
    #[inline] pub fn with_r14<F: FnOnce(R14) -> R14>(&self, f: F) -> &Self {
        self.r14_reg().with(f);
        self
    }

    #[doc="Get the R15 Register."]
    #[inline] pub fn r15_reg(&self) -> ::bobbin_mcu::register::Register<R15> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R15, 0x3c)
    }

    #[doc="Get the *mut pointer for the R15 register."]
    #[inline] pub fn r15_mut(&self) -> *mut R15 { 
        self.r15_reg().ptr()
    }

    #[doc="Get the *const pointer for the R15 register."]
    #[inline] pub fn r15_ptr(&self) -> *const R15 { 
        self.r15_reg().ptr()
    }

    #[doc="Read the R15 register."]
    #[inline] pub fn r15(&self) -> R15 { 
        self.r15_reg().read()
    }

    #[doc="Write the R15 register."]
    #[inline] pub fn write_r15(&self, value: R15) -> &Self { 
        self.r15_reg().write(value);
        self
    }

    #[doc="Set the R15 register."]
    #[inline] pub fn set_r15<F: FnOnce(R15) -> R15>(&self, f: F) -> &Self {
        self.r15_reg().set(f);
        self
    }

    #[doc="Modify the R15 register."]
    #[inline] pub fn with_r15<F: FnOnce(R15) -> R15>(&self, f: F) -> &Self {
        self.r15_reg().with(f);
        self
    }

    #[doc="Get the R16 Register."]
    #[inline] pub fn r16_reg(&self) -> ::bobbin_mcu::register::Register<R16> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R16, 0x40)
    }

    #[doc="Get the *mut pointer for the R16 register."]
    #[inline] pub fn r16_mut(&self) -> *mut R16 { 
        self.r16_reg().ptr()
    }

    #[doc="Get the *const pointer for the R16 register."]
    #[inline] pub fn r16_ptr(&self) -> *const R16 { 
        self.r16_reg().ptr()
    }

    #[doc="Read the R16 register."]
    #[inline] pub fn r16(&self) -> R16 { 
        self.r16_reg().read()
    }

    #[doc="Write the R16 register."]
    #[inline] pub fn write_r16(&self, value: R16) -> &Self { 
        self.r16_reg().write(value);
        self
    }

    #[doc="Set the R16 register."]
    #[inline] pub fn set_r16<F: FnOnce(R16) -> R16>(&self, f: F) -> &Self {
        self.r16_reg().set(f);
        self
    }

    #[doc="Modify the R16 register."]
    #[inline] pub fn with_r16<F: FnOnce(R16) -> R16>(&self, f: F) -> &Self {
        self.r16_reg().with(f);
        self
    }

    #[doc="Get the R17 Register."]
    #[inline] pub fn r17_reg(&self) -> ::bobbin_mcu::register::Register<R17> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R17, 0x44)
    }

    #[doc="Get the *mut pointer for the R17 register."]
    #[inline] pub fn r17_mut(&self) -> *mut R17 { 
        self.r17_reg().ptr()
    }

    #[doc="Get the *const pointer for the R17 register."]
    #[inline] pub fn r17_ptr(&self) -> *const R17 { 
        self.r17_reg().ptr()
    }

    #[doc="Read the R17 register."]
    #[inline] pub fn r17(&self) -> R17 { 
        self.r17_reg().read()
    }

    #[doc="Write the R17 register."]
    #[inline] pub fn write_r17(&self, value: R17) -> &Self { 
        self.r17_reg().write(value);
        self
    }

    #[doc="Set the R17 register."]
    #[inline] pub fn set_r17<F: FnOnce(R17) -> R17>(&self, f: F) -> &Self {
        self.r17_reg().set(f);
        self
    }

    #[doc="Modify the R17 register."]
    #[inline] pub fn with_r17<F: FnOnce(R17) -> R17>(&self, f: F) -> &Self {
        self.r17_reg().with(f);
        self
    }

    #[doc="Get the R18 Register."]
    #[inline] pub fn r18_reg(&self) -> ::bobbin_mcu::register::Register<R18> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R18, 0x48)
    }

    #[doc="Get the *mut pointer for the R18 register."]
    #[inline] pub fn r18_mut(&self) -> *mut R18 { 
        self.r18_reg().ptr()
    }

    #[doc="Get the *const pointer for the R18 register."]
    #[inline] pub fn r18_ptr(&self) -> *const R18 { 
        self.r18_reg().ptr()
    }

    #[doc="Read the R18 register."]
    #[inline] pub fn r18(&self) -> R18 { 
        self.r18_reg().read()
    }

    #[doc="Write the R18 register."]
    #[inline] pub fn write_r18(&self, value: R18) -> &Self { 
        self.r18_reg().write(value);
        self
    }

    #[doc="Set the R18 register."]
    #[inline] pub fn set_r18<F: FnOnce(R18) -> R18>(&self, f: F) -> &Self {
        self.r18_reg().set(f);
        self
    }

    #[doc="Modify the R18 register."]
    #[inline] pub fn with_r18<F: FnOnce(R18) -> R18>(&self, f: F) -> &Self {
        self.r18_reg().with(f);
        self
    }

    #[doc="Get the R19 Register."]
    #[inline] pub fn r19_reg(&self) -> ::bobbin_mcu::register::Register<R19> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R19, 0x4c)
    }

    #[doc="Get the *mut pointer for the R19 register."]
    #[inline] pub fn r19_mut(&self) -> *mut R19 { 
        self.r19_reg().ptr()
    }

    #[doc="Get the *const pointer for the R19 register."]
    #[inline] pub fn r19_ptr(&self) -> *const R19 { 
        self.r19_reg().ptr()
    }

    #[doc="Read the R19 register."]
    #[inline] pub fn r19(&self) -> R19 { 
        self.r19_reg().read()
    }

    #[doc="Write the R19 register."]
    #[inline] pub fn write_r19(&self, value: R19) -> &Self { 
        self.r19_reg().write(value);
        self
    }

    #[doc="Set the R19 register."]
    #[inline] pub fn set_r19<F: FnOnce(R19) -> R19>(&self, f: F) -> &Self {
        self.r19_reg().set(f);
        self
    }

    #[doc="Modify the R19 register."]
    #[inline] pub fn with_r19<F: FnOnce(R19) -> R19>(&self, f: F) -> &Self {
        self.r19_reg().with(f);
        self
    }

    #[doc="Get the R20 Register."]
    #[inline] pub fn r20_reg(&self) -> ::bobbin_mcu::register::Register<R20> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R20, 0x50)
    }

    #[doc="Get the *mut pointer for the R20 register."]
    #[inline] pub fn r20_mut(&self) -> *mut R20 { 
        self.r20_reg().ptr()
    }

    #[doc="Get the *const pointer for the R20 register."]
    #[inline] pub fn r20_ptr(&self) -> *const R20 { 
        self.r20_reg().ptr()
    }

    #[doc="Read the R20 register."]
    #[inline] pub fn r20(&self) -> R20 { 
        self.r20_reg().read()
    }

    #[doc="Write the R20 register."]
    #[inline] pub fn write_r20(&self, value: R20) -> &Self { 
        self.r20_reg().write(value);
        self
    }

    #[doc="Set the R20 register."]
    #[inline] pub fn set_r20<F: FnOnce(R20) -> R20>(&self, f: F) -> &Self {
        self.r20_reg().set(f);
        self
    }

    #[doc="Modify the R20 register."]
    #[inline] pub fn with_r20<F: FnOnce(R20) -> R20>(&self, f: F) -> &Self {
        self.r20_reg().with(f);
        self
    }

    #[doc="Get the R21 Register."]
    #[inline] pub fn r21_reg(&self) -> ::bobbin_mcu::register::Register<R21> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R21, 0x54)
    }

    #[doc="Get the *mut pointer for the R21 register."]
    #[inline] pub fn r21_mut(&self) -> *mut R21 { 
        self.r21_reg().ptr()
    }

    #[doc="Get the *const pointer for the R21 register."]
    #[inline] pub fn r21_ptr(&self) -> *const R21 { 
        self.r21_reg().ptr()
    }

    #[doc="Read the R21 register."]
    #[inline] pub fn r21(&self) -> R21 { 
        self.r21_reg().read()
    }

    #[doc="Write the R21 register."]
    #[inline] pub fn write_r21(&self, value: R21) -> &Self { 
        self.r21_reg().write(value);
        self
    }

    #[doc="Set the R21 register."]
    #[inline] pub fn set_r21<F: FnOnce(R21) -> R21>(&self, f: F) -> &Self {
        self.r21_reg().set(f);
        self
    }

    #[doc="Modify the R21 register."]
    #[inline] pub fn with_r21<F: FnOnce(R21) -> R21>(&self, f: F) -> &Self {
        self.r21_reg().with(f);
        self
    }

    #[doc="Get the R22 Register."]
    #[inline] pub fn r22_reg(&self) -> ::bobbin_mcu::register::Register<R22> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R22, 0x58)
    }

    #[doc="Get the *mut pointer for the R22 register."]
    #[inline] pub fn r22_mut(&self) -> *mut R22 { 
        self.r22_reg().ptr()
    }

    #[doc="Get the *const pointer for the R22 register."]
    #[inline] pub fn r22_ptr(&self) -> *const R22 { 
        self.r22_reg().ptr()
    }

    #[doc="Read the R22 register."]
    #[inline] pub fn r22(&self) -> R22 { 
        self.r22_reg().read()
    }

    #[doc="Write the R22 register."]
    #[inline] pub fn write_r22(&self, value: R22) -> &Self { 
        self.r22_reg().write(value);
        self
    }

    #[doc="Set the R22 register."]
    #[inline] pub fn set_r22<F: FnOnce(R22) -> R22>(&self, f: F) -> &Self {
        self.r22_reg().set(f);
        self
    }

    #[doc="Modify the R22 register."]
    #[inline] pub fn with_r22<F: FnOnce(R22) -> R22>(&self, f: F) -> &Self {
        self.r22_reg().with(f);
        self
    }

    #[doc="Get the R23 Register."]
    #[inline] pub fn r23_reg(&self) -> ::bobbin_mcu::register::Register<R23> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R23, 0x5c)
    }

    #[doc="Get the *mut pointer for the R23 register."]
    #[inline] pub fn r23_mut(&self) -> *mut R23 { 
        self.r23_reg().ptr()
    }

    #[doc="Get the *const pointer for the R23 register."]
    #[inline] pub fn r23_ptr(&self) -> *const R23 { 
        self.r23_reg().ptr()
    }

    #[doc="Read the R23 register."]
    #[inline] pub fn r23(&self) -> R23 { 
        self.r23_reg().read()
    }

    #[doc="Write the R23 register."]
    #[inline] pub fn write_r23(&self, value: R23) -> &Self { 
        self.r23_reg().write(value);
        self
    }

    #[doc="Set the R23 register."]
    #[inline] pub fn set_r23<F: FnOnce(R23) -> R23>(&self, f: F) -> &Self {
        self.r23_reg().set(f);
        self
    }

    #[doc="Modify the R23 register."]
    #[inline] pub fn with_r23<F: FnOnce(R23) -> R23>(&self, f: F) -> &Self {
        self.r23_reg().with(f);
        self
    }

    #[doc="Get the R24 Register."]
    #[inline] pub fn r24_reg(&self) -> ::bobbin_mcu::register::Register<R24> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R24, 0x60)
    }

    #[doc="Get the *mut pointer for the R24 register."]
    #[inline] pub fn r24_mut(&self) -> *mut R24 { 
        self.r24_reg().ptr()
    }

    #[doc="Get the *const pointer for the R24 register."]
    #[inline] pub fn r24_ptr(&self) -> *const R24 { 
        self.r24_reg().ptr()
    }

    #[doc="Read the R24 register."]
    #[inline] pub fn r24(&self) -> R24 { 
        self.r24_reg().read()
    }

    #[doc="Write the R24 register."]
    #[inline] pub fn write_r24(&self, value: R24) -> &Self { 
        self.r24_reg().write(value);
        self
    }

    #[doc="Set the R24 register."]
    #[inline] pub fn set_r24<F: FnOnce(R24) -> R24>(&self, f: F) -> &Self {
        self.r24_reg().set(f);
        self
    }

    #[doc="Modify the R24 register."]
    #[inline] pub fn with_r24<F: FnOnce(R24) -> R24>(&self, f: F) -> &Self {
        self.r24_reg().with(f);
        self
    }

    #[doc="Get the R25 Register."]
    #[inline] pub fn r25_reg(&self) -> ::bobbin_mcu::register::Register<R25> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R25, 0x64)
    }

    #[doc="Get the *mut pointer for the R25 register."]
    #[inline] pub fn r25_mut(&self) -> *mut R25 { 
        self.r25_reg().ptr()
    }

    #[doc="Get the *const pointer for the R25 register."]
    #[inline] pub fn r25_ptr(&self) -> *const R25 { 
        self.r25_reg().ptr()
    }

    #[doc="Read the R25 register."]
    #[inline] pub fn r25(&self) -> R25 { 
        self.r25_reg().read()
    }

    #[doc="Write the R25 register."]
    #[inline] pub fn write_r25(&self, value: R25) -> &Self { 
        self.r25_reg().write(value);
        self
    }

    #[doc="Set the R25 register."]
    #[inline] pub fn set_r25<F: FnOnce(R25) -> R25>(&self, f: F) -> &Self {
        self.r25_reg().set(f);
        self
    }

    #[doc="Modify the R25 register."]
    #[inline] pub fn with_r25<F: FnOnce(R25) -> R25>(&self, f: F) -> &Self {
        self.r25_reg().with(f);
        self
    }

    #[doc="Get the R26 Register."]
    #[inline] pub fn r26_reg(&self) -> ::bobbin_mcu::register::Register<R26> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R26, 0x68)
    }

    #[doc="Get the *mut pointer for the R26 register."]
    #[inline] pub fn r26_mut(&self) -> *mut R26 { 
        self.r26_reg().ptr()
    }

    #[doc="Get the *const pointer for the R26 register."]
    #[inline] pub fn r26_ptr(&self) -> *const R26 { 
        self.r26_reg().ptr()
    }

    #[doc="Read the R26 register."]
    #[inline] pub fn r26(&self) -> R26 { 
        self.r26_reg().read()
    }

    #[doc="Write the R26 register."]
    #[inline] pub fn write_r26(&self, value: R26) -> &Self { 
        self.r26_reg().write(value);
        self
    }

    #[doc="Set the R26 register."]
    #[inline] pub fn set_r26<F: FnOnce(R26) -> R26>(&self, f: F) -> &Self {
        self.r26_reg().set(f);
        self
    }

    #[doc="Modify the R26 register."]
    #[inline] pub fn with_r26<F: FnOnce(R26) -> R26>(&self, f: F) -> &Self {
        self.r26_reg().with(f);
        self
    }

    #[doc="Get the R27 Register."]
    #[inline] pub fn r27_reg(&self) -> ::bobbin_mcu::register::Register<R27> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R27, 0x6c)
    }

    #[doc="Get the *mut pointer for the R27 register."]
    #[inline] pub fn r27_mut(&self) -> *mut R27 { 
        self.r27_reg().ptr()
    }

    #[doc="Get the *const pointer for the R27 register."]
    #[inline] pub fn r27_ptr(&self) -> *const R27 { 
        self.r27_reg().ptr()
    }

    #[doc="Read the R27 register."]
    #[inline] pub fn r27(&self) -> R27 { 
        self.r27_reg().read()
    }

    #[doc="Write the R27 register."]
    #[inline] pub fn write_r27(&self, value: R27) -> &Self { 
        self.r27_reg().write(value);
        self
    }

    #[doc="Set the R27 register."]
    #[inline] pub fn set_r27<F: FnOnce(R27) -> R27>(&self, f: F) -> &Self {
        self.r27_reg().set(f);
        self
    }

    #[doc="Modify the R27 register."]
    #[inline] pub fn with_r27<F: FnOnce(R27) -> R27>(&self, f: F) -> &Self {
        self.r27_reg().with(f);
        self
    }

    #[doc="Get the R28 Register."]
    #[inline] pub fn r28_reg(&self) -> ::bobbin_mcu::register::Register<R28> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R28, 0x70)
    }

    #[doc="Get the *mut pointer for the R28 register."]
    #[inline] pub fn r28_mut(&self) -> *mut R28 { 
        self.r28_reg().ptr()
    }

    #[doc="Get the *const pointer for the R28 register."]
    #[inline] pub fn r28_ptr(&self) -> *const R28 { 
        self.r28_reg().ptr()
    }

    #[doc="Read the R28 register."]
    #[inline] pub fn r28(&self) -> R28 { 
        self.r28_reg().read()
    }

    #[doc="Write the R28 register."]
    #[inline] pub fn write_r28(&self, value: R28) -> &Self { 
        self.r28_reg().write(value);
        self
    }

    #[doc="Set the R28 register."]
    #[inline] pub fn set_r28<F: FnOnce(R28) -> R28>(&self, f: F) -> &Self {
        self.r28_reg().set(f);
        self
    }

    #[doc="Modify the R28 register."]
    #[inline] pub fn with_r28<F: FnOnce(R28) -> R28>(&self, f: F) -> &Self {
        self.r28_reg().with(f);
        self
    }

    #[doc="Get the R29 Register."]
    #[inline] pub fn r29_reg(&self) -> ::bobbin_mcu::register::Register<R29> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R29, 0x74)
    }

    #[doc="Get the *mut pointer for the R29 register."]
    #[inline] pub fn r29_mut(&self) -> *mut R29 { 
        self.r29_reg().ptr()
    }

    #[doc="Get the *const pointer for the R29 register."]
    #[inline] pub fn r29_ptr(&self) -> *const R29 { 
        self.r29_reg().ptr()
    }

    #[doc="Read the R29 register."]
    #[inline] pub fn r29(&self) -> R29 { 
        self.r29_reg().read()
    }

    #[doc="Write the R29 register."]
    #[inline] pub fn write_r29(&self, value: R29) -> &Self { 
        self.r29_reg().write(value);
        self
    }

    #[doc="Set the R29 register."]
    #[inline] pub fn set_r29<F: FnOnce(R29) -> R29>(&self, f: F) -> &Self {
        self.r29_reg().set(f);
        self
    }

    #[doc="Modify the R29 register."]
    #[inline] pub fn with_r29<F: FnOnce(R29) -> R29>(&self, f: F) -> &Self {
        self.r29_reg().with(f);
        self
    }

    #[doc="Get the R30 Register."]
    #[inline] pub fn r30_reg(&self) -> ::bobbin_mcu::register::Register<R30> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R30, 0x78)
    }

    #[doc="Get the *mut pointer for the R30 register."]
    #[inline] pub fn r30_mut(&self) -> *mut R30 { 
        self.r30_reg().ptr()
    }

    #[doc="Get the *const pointer for the R30 register."]
    #[inline] pub fn r30_ptr(&self) -> *const R30 { 
        self.r30_reg().ptr()
    }

    #[doc="Read the R30 register."]
    #[inline] pub fn r30(&self) -> R30 { 
        self.r30_reg().read()
    }

    #[doc="Write the R30 register."]
    #[inline] pub fn write_r30(&self, value: R30) -> &Self { 
        self.r30_reg().write(value);
        self
    }

    #[doc="Set the R30 register."]
    #[inline] pub fn set_r30<F: FnOnce(R30) -> R30>(&self, f: F) -> &Self {
        self.r30_reg().set(f);
        self
    }

    #[doc="Modify the R30 register."]
    #[inline] pub fn with_r30<F: FnOnce(R30) -> R30>(&self, f: F) -> &Self {
        self.r30_reg().with(f);
        self
    }

    #[doc="Get the R31 Register."]
    #[inline] pub fn r31_reg(&self) -> ::bobbin_mcu::register::Register<R31> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R31, 0x7c)
    }

    #[doc="Get the *mut pointer for the R31 register."]
    #[inline] pub fn r31_mut(&self) -> *mut R31 { 
        self.r31_reg().ptr()
    }

    #[doc="Get the *const pointer for the R31 register."]
    #[inline] pub fn r31_ptr(&self) -> *const R31 { 
        self.r31_reg().ptr()
    }

    #[doc="Read the R31 register."]
    #[inline] pub fn r31(&self) -> R31 { 
        self.r31_reg().read()
    }

    #[doc="Write the R31 register."]
    #[inline] pub fn write_r31(&self, value: R31) -> &Self { 
        self.r31_reg().write(value);
        self
    }

    #[doc="Set the R31 register."]
    #[inline] pub fn set_r31<F: FnOnce(R31) -> R31>(&self, f: F) -> &Self {
        self.r31_reg().set(f);
        self
    }

    #[doc="Modify the R31 register."]
    #[inline] pub fn with_r31<F: FnOnce(R31) -> R31>(&self, f: F) -> &Self {
        self.r31_reg().with(f);
        self
    }

    #[doc="Get the RLR0 Register."]
    #[inline] pub fn rlr0_reg(&self) -> ::bobbin_mcu::register::Register<Rlr0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr0, 0x80)
    }

    #[doc="Get the *mut pointer for the RLR0 register."]
    #[inline] pub fn rlr0_mut(&self) -> *mut Rlr0 { 
        self.rlr0_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR0 register."]
    #[inline] pub fn rlr0_ptr(&self) -> *const Rlr0 { 
        self.rlr0_reg().ptr()
    }

    #[doc="Read the RLR0 register."]
    #[inline] pub fn rlr0(&self) -> Rlr0 { 
        self.rlr0_reg().read()
    }

    #[doc="Get the RLR1 Register."]
    #[inline] pub fn rlr1_reg(&self) -> ::bobbin_mcu::register::Register<Rlr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr1, 0x84)
    }

    #[doc="Get the *mut pointer for the RLR1 register."]
    #[inline] pub fn rlr1_mut(&self) -> *mut Rlr1 { 
        self.rlr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR1 register."]
    #[inline] pub fn rlr1_ptr(&self) -> *const Rlr1 { 
        self.rlr1_reg().ptr()
    }

    #[doc="Read the RLR1 register."]
    #[inline] pub fn rlr1(&self) -> Rlr1 { 
        self.rlr1_reg().read()
    }

    #[doc="Get the RLR2 Register."]
    #[inline] pub fn rlr2_reg(&self) -> ::bobbin_mcu::register::Register<Rlr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr2, 0x88)
    }

    #[doc="Get the *mut pointer for the RLR2 register."]
    #[inline] pub fn rlr2_mut(&self) -> *mut Rlr2 { 
        self.rlr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR2 register."]
    #[inline] pub fn rlr2_ptr(&self) -> *const Rlr2 { 
        self.rlr2_reg().ptr()
    }

    #[doc="Read the RLR2 register."]
    #[inline] pub fn rlr2(&self) -> Rlr2 { 
        self.rlr2_reg().read()
    }

    #[doc="Get the RLR3 Register."]
    #[inline] pub fn rlr3_reg(&self) -> ::bobbin_mcu::register::Register<Rlr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr3, 0x8c)
    }

    #[doc="Get the *mut pointer for the RLR3 register."]
    #[inline] pub fn rlr3_mut(&self) -> *mut Rlr3 { 
        self.rlr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR3 register."]
    #[inline] pub fn rlr3_ptr(&self) -> *const Rlr3 { 
        self.rlr3_reg().ptr()
    }

    #[doc="Read the RLR3 register."]
    #[inline] pub fn rlr3(&self) -> Rlr3 { 
        self.rlr3_reg().read()
    }

    #[doc="Get the RLR4 Register."]
    #[inline] pub fn rlr4_reg(&self) -> ::bobbin_mcu::register::Register<Rlr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr4, 0x90)
    }

    #[doc="Get the *mut pointer for the RLR4 register."]
    #[inline] pub fn rlr4_mut(&self) -> *mut Rlr4 { 
        self.rlr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR4 register."]
    #[inline] pub fn rlr4_ptr(&self) -> *const Rlr4 { 
        self.rlr4_reg().ptr()
    }

    #[doc="Read the RLR4 register."]
    #[inline] pub fn rlr4(&self) -> Rlr4 { 
        self.rlr4_reg().read()
    }

    #[doc="Get the RLR5 Register."]
    #[inline] pub fn rlr5_reg(&self) -> ::bobbin_mcu::register::Register<Rlr5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr5, 0x94)
    }

    #[doc="Get the *mut pointer for the RLR5 register."]
    #[inline] pub fn rlr5_mut(&self) -> *mut Rlr5 { 
        self.rlr5_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR5 register."]
    #[inline] pub fn rlr5_ptr(&self) -> *const Rlr5 { 
        self.rlr5_reg().ptr()
    }

    #[doc="Read the RLR5 register."]
    #[inline] pub fn rlr5(&self) -> Rlr5 { 
        self.rlr5_reg().read()
    }

    #[doc="Get the RLR6 Register."]
    #[inline] pub fn rlr6_reg(&self) -> ::bobbin_mcu::register::Register<Rlr6> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr6, 0x98)
    }

    #[doc="Get the *mut pointer for the RLR6 register."]
    #[inline] pub fn rlr6_mut(&self) -> *mut Rlr6 { 
        self.rlr6_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR6 register."]
    #[inline] pub fn rlr6_ptr(&self) -> *const Rlr6 { 
        self.rlr6_reg().ptr()
    }

    #[doc="Read the RLR6 register."]
    #[inline] pub fn rlr6(&self) -> Rlr6 { 
        self.rlr6_reg().read()
    }

    #[doc="Get the RLR7 Register."]
    #[inline] pub fn rlr7_reg(&self) -> ::bobbin_mcu::register::Register<Rlr7> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr7, 0x9c)
    }

    #[doc="Get the *mut pointer for the RLR7 register."]
    #[inline] pub fn rlr7_mut(&self) -> *mut Rlr7 { 
        self.rlr7_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR7 register."]
    #[inline] pub fn rlr7_ptr(&self) -> *const Rlr7 { 
        self.rlr7_reg().ptr()
    }

    #[doc="Read the RLR7 register."]
    #[inline] pub fn rlr7(&self) -> Rlr7 { 
        self.rlr7_reg().read()
    }

    #[doc="Get the RLR8 Register."]
    #[inline] pub fn rlr8_reg(&self) -> ::bobbin_mcu::register::Register<Rlr8> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr8, 0xa0)
    }

    #[doc="Get the *mut pointer for the RLR8 register."]
    #[inline] pub fn rlr8_mut(&self) -> *mut Rlr8 { 
        self.rlr8_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR8 register."]
    #[inline] pub fn rlr8_ptr(&self) -> *const Rlr8 { 
        self.rlr8_reg().ptr()
    }

    #[doc="Read the RLR8 register."]
    #[inline] pub fn rlr8(&self) -> Rlr8 { 
        self.rlr8_reg().read()
    }

    #[doc="Get the RLR9 Register."]
    #[inline] pub fn rlr9_reg(&self) -> ::bobbin_mcu::register::Register<Rlr9> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr9, 0xa4)
    }

    #[doc="Get the *mut pointer for the RLR9 register."]
    #[inline] pub fn rlr9_mut(&self) -> *mut Rlr9 { 
        self.rlr9_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR9 register."]
    #[inline] pub fn rlr9_ptr(&self) -> *const Rlr9 { 
        self.rlr9_reg().ptr()
    }

    #[doc="Read the RLR9 register."]
    #[inline] pub fn rlr9(&self) -> Rlr9 { 
        self.rlr9_reg().read()
    }

    #[doc="Get the RLR10 Register."]
    #[inline] pub fn rlr10_reg(&self) -> ::bobbin_mcu::register::Register<Rlr10> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr10, 0xa8)
    }

    #[doc="Get the *mut pointer for the RLR10 register."]
    #[inline] pub fn rlr10_mut(&self) -> *mut Rlr10 { 
        self.rlr10_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR10 register."]
    #[inline] pub fn rlr10_ptr(&self) -> *const Rlr10 { 
        self.rlr10_reg().ptr()
    }

    #[doc="Read the RLR10 register."]
    #[inline] pub fn rlr10(&self) -> Rlr10 { 
        self.rlr10_reg().read()
    }

    #[doc="Get the RLR11 Register."]
    #[inline] pub fn rlr11_reg(&self) -> ::bobbin_mcu::register::Register<Rlr11> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr11, 0xac)
    }

    #[doc="Get the *mut pointer for the RLR11 register."]
    #[inline] pub fn rlr11_mut(&self) -> *mut Rlr11 { 
        self.rlr11_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR11 register."]
    #[inline] pub fn rlr11_ptr(&self) -> *const Rlr11 { 
        self.rlr11_reg().ptr()
    }

    #[doc="Read the RLR11 register."]
    #[inline] pub fn rlr11(&self) -> Rlr11 { 
        self.rlr11_reg().read()
    }

    #[doc="Get the RLR12 Register."]
    #[inline] pub fn rlr12_reg(&self) -> ::bobbin_mcu::register::Register<Rlr12> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr12, 0xb0)
    }

    #[doc="Get the *mut pointer for the RLR12 register."]
    #[inline] pub fn rlr12_mut(&self) -> *mut Rlr12 { 
        self.rlr12_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR12 register."]
    #[inline] pub fn rlr12_ptr(&self) -> *const Rlr12 { 
        self.rlr12_reg().ptr()
    }

    #[doc="Read the RLR12 register."]
    #[inline] pub fn rlr12(&self) -> Rlr12 { 
        self.rlr12_reg().read()
    }

    #[doc="Get the RLR13 Register."]
    #[inline] pub fn rlr13_reg(&self) -> ::bobbin_mcu::register::Register<Rlr13> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr13, 0xb4)
    }

    #[doc="Get the *mut pointer for the RLR13 register."]
    #[inline] pub fn rlr13_mut(&self) -> *mut Rlr13 { 
        self.rlr13_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR13 register."]
    #[inline] pub fn rlr13_ptr(&self) -> *const Rlr13 { 
        self.rlr13_reg().ptr()
    }

    #[doc="Read the RLR13 register."]
    #[inline] pub fn rlr13(&self) -> Rlr13 { 
        self.rlr13_reg().read()
    }

    #[doc="Get the RLR14 Register."]
    #[inline] pub fn rlr14_reg(&self) -> ::bobbin_mcu::register::Register<Rlr14> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr14, 0xb8)
    }

    #[doc="Get the *mut pointer for the RLR14 register."]
    #[inline] pub fn rlr14_mut(&self) -> *mut Rlr14 { 
        self.rlr14_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR14 register."]
    #[inline] pub fn rlr14_ptr(&self) -> *const Rlr14 { 
        self.rlr14_reg().ptr()
    }

    #[doc="Read the RLR14 register."]
    #[inline] pub fn rlr14(&self) -> Rlr14 { 
        self.rlr14_reg().read()
    }

    #[doc="Get the RLR15 Register."]
    #[inline] pub fn rlr15_reg(&self) -> ::bobbin_mcu::register::Register<Rlr15> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr15, 0xbc)
    }

    #[doc="Get the *mut pointer for the RLR15 register."]
    #[inline] pub fn rlr15_mut(&self) -> *mut Rlr15 { 
        self.rlr15_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR15 register."]
    #[inline] pub fn rlr15_ptr(&self) -> *const Rlr15 { 
        self.rlr15_reg().ptr()
    }

    #[doc="Read the RLR15 register."]
    #[inline] pub fn rlr15(&self) -> Rlr15 { 
        self.rlr15_reg().read()
    }

    #[doc="Get the RLR16 Register."]
    #[inline] pub fn rlr16_reg(&self) -> ::bobbin_mcu::register::Register<Rlr16> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr16, 0xc0)
    }

    #[doc="Get the *mut pointer for the RLR16 register."]
    #[inline] pub fn rlr16_mut(&self) -> *mut Rlr16 { 
        self.rlr16_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR16 register."]
    #[inline] pub fn rlr16_ptr(&self) -> *const Rlr16 { 
        self.rlr16_reg().ptr()
    }

    #[doc="Read the RLR16 register."]
    #[inline] pub fn rlr16(&self) -> Rlr16 { 
        self.rlr16_reg().read()
    }

    #[doc="Get the RLR17 Register."]
    #[inline] pub fn rlr17_reg(&self) -> ::bobbin_mcu::register::Register<Rlr17> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr17, 0xc4)
    }

    #[doc="Get the *mut pointer for the RLR17 register."]
    #[inline] pub fn rlr17_mut(&self) -> *mut Rlr17 { 
        self.rlr17_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR17 register."]
    #[inline] pub fn rlr17_ptr(&self) -> *const Rlr17 { 
        self.rlr17_reg().ptr()
    }

    #[doc="Read the RLR17 register."]
    #[inline] pub fn rlr17(&self) -> Rlr17 { 
        self.rlr17_reg().read()
    }

    #[doc="Get the RLR18 Register."]
    #[inline] pub fn rlr18_reg(&self) -> ::bobbin_mcu::register::Register<Rlr18> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr18, 0xc8)
    }

    #[doc="Get the *mut pointer for the RLR18 register."]
    #[inline] pub fn rlr18_mut(&self) -> *mut Rlr18 { 
        self.rlr18_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR18 register."]
    #[inline] pub fn rlr18_ptr(&self) -> *const Rlr18 { 
        self.rlr18_reg().ptr()
    }

    #[doc="Read the RLR18 register."]
    #[inline] pub fn rlr18(&self) -> Rlr18 { 
        self.rlr18_reg().read()
    }

    #[doc="Get the RLR19 Register."]
    #[inline] pub fn rlr19_reg(&self) -> ::bobbin_mcu::register::Register<Rlr19> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr19, 0xcc)
    }

    #[doc="Get the *mut pointer for the RLR19 register."]
    #[inline] pub fn rlr19_mut(&self) -> *mut Rlr19 { 
        self.rlr19_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR19 register."]
    #[inline] pub fn rlr19_ptr(&self) -> *const Rlr19 { 
        self.rlr19_reg().ptr()
    }

    #[doc="Read the RLR19 register."]
    #[inline] pub fn rlr19(&self) -> Rlr19 { 
        self.rlr19_reg().read()
    }

    #[doc="Get the RLR20 Register."]
    #[inline] pub fn rlr20_reg(&self) -> ::bobbin_mcu::register::Register<Rlr20> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr20, 0xd0)
    }

    #[doc="Get the *mut pointer for the RLR20 register."]
    #[inline] pub fn rlr20_mut(&self) -> *mut Rlr20 { 
        self.rlr20_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR20 register."]
    #[inline] pub fn rlr20_ptr(&self) -> *const Rlr20 { 
        self.rlr20_reg().ptr()
    }

    #[doc="Read the RLR20 register."]
    #[inline] pub fn rlr20(&self) -> Rlr20 { 
        self.rlr20_reg().read()
    }

    #[doc="Get the RLR21 Register."]
    #[inline] pub fn rlr21_reg(&self) -> ::bobbin_mcu::register::Register<Rlr21> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr21, 0xd4)
    }

    #[doc="Get the *mut pointer for the RLR21 register."]
    #[inline] pub fn rlr21_mut(&self) -> *mut Rlr21 { 
        self.rlr21_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR21 register."]
    #[inline] pub fn rlr21_ptr(&self) -> *const Rlr21 { 
        self.rlr21_reg().ptr()
    }

    #[doc="Read the RLR21 register."]
    #[inline] pub fn rlr21(&self) -> Rlr21 { 
        self.rlr21_reg().read()
    }

    #[doc="Get the RLR22 Register."]
    #[inline] pub fn rlr22_reg(&self) -> ::bobbin_mcu::register::Register<Rlr22> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr22, 0xd8)
    }

    #[doc="Get the *mut pointer for the RLR22 register."]
    #[inline] pub fn rlr22_mut(&self) -> *mut Rlr22 { 
        self.rlr22_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR22 register."]
    #[inline] pub fn rlr22_ptr(&self) -> *const Rlr22 { 
        self.rlr22_reg().ptr()
    }

    #[doc="Read the RLR22 register."]
    #[inline] pub fn rlr22(&self) -> Rlr22 { 
        self.rlr22_reg().read()
    }

    #[doc="Get the RLR23 Register."]
    #[inline] pub fn rlr23_reg(&self) -> ::bobbin_mcu::register::Register<Rlr23> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr23, 0xdc)
    }

    #[doc="Get the *mut pointer for the RLR23 register."]
    #[inline] pub fn rlr23_mut(&self) -> *mut Rlr23 { 
        self.rlr23_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR23 register."]
    #[inline] pub fn rlr23_ptr(&self) -> *const Rlr23 { 
        self.rlr23_reg().ptr()
    }

    #[doc="Read the RLR23 register."]
    #[inline] pub fn rlr23(&self) -> Rlr23 { 
        self.rlr23_reg().read()
    }

    #[doc="Get the RLR24 Register."]
    #[inline] pub fn rlr24_reg(&self) -> ::bobbin_mcu::register::Register<Rlr24> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr24, 0xe0)
    }

    #[doc="Get the *mut pointer for the RLR24 register."]
    #[inline] pub fn rlr24_mut(&self) -> *mut Rlr24 { 
        self.rlr24_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR24 register."]
    #[inline] pub fn rlr24_ptr(&self) -> *const Rlr24 { 
        self.rlr24_reg().ptr()
    }

    #[doc="Read the RLR24 register."]
    #[inline] pub fn rlr24(&self) -> Rlr24 { 
        self.rlr24_reg().read()
    }

    #[doc="Get the RLR25 Register."]
    #[inline] pub fn rlr25_reg(&self) -> ::bobbin_mcu::register::Register<Rlr25> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr25, 0xe4)
    }

    #[doc="Get the *mut pointer for the RLR25 register."]
    #[inline] pub fn rlr25_mut(&self) -> *mut Rlr25 { 
        self.rlr25_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR25 register."]
    #[inline] pub fn rlr25_ptr(&self) -> *const Rlr25 { 
        self.rlr25_reg().ptr()
    }

    #[doc="Read the RLR25 register."]
    #[inline] pub fn rlr25(&self) -> Rlr25 { 
        self.rlr25_reg().read()
    }

    #[doc="Get the RLR26 Register."]
    #[inline] pub fn rlr26_reg(&self) -> ::bobbin_mcu::register::Register<Rlr26> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr26, 0xe8)
    }

    #[doc="Get the *mut pointer for the RLR26 register."]
    #[inline] pub fn rlr26_mut(&self) -> *mut Rlr26 { 
        self.rlr26_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR26 register."]
    #[inline] pub fn rlr26_ptr(&self) -> *const Rlr26 { 
        self.rlr26_reg().ptr()
    }

    #[doc="Read the RLR26 register."]
    #[inline] pub fn rlr26(&self) -> Rlr26 { 
        self.rlr26_reg().read()
    }

    #[doc="Get the RLR27 Register."]
    #[inline] pub fn rlr27_reg(&self) -> ::bobbin_mcu::register::Register<Rlr27> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr27, 0xec)
    }

    #[doc="Get the *mut pointer for the RLR27 register."]
    #[inline] pub fn rlr27_mut(&self) -> *mut Rlr27 { 
        self.rlr27_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR27 register."]
    #[inline] pub fn rlr27_ptr(&self) -> *const Rlr27 { 
        self.rlr27_reg().ptr()
    }

    #[doc="Read the RLR27 register."]
    #[inline] pub fn rlr27(&self) -> Rlr27 { 
        self.rlr27_reg().read()
    }

    #[doc="Get the RLR28 Register."]
    #[inline] pub fn rlr28_reg(&self) -> ::bobbin_mcu::register::Register<Rlr28> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr28, 0xf0)
    }

    #[doc="Get the *mut pointer for the RLR28 register."]
    #[inline] pub fn rlr28_mut(&self) -> *mut Rlr28 { 
        self.rlr28_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR28 register."]
    #[inline] pub fn rlr28_ptr(&self) -> *const Rlr28 { 
        self.rlr28_reg().ptr()
    }

    #[doc="Read the RLR28 register."]
    #[inline] pub fn rlr28(&self) -> Rlr28 { 
        self.rlr28_reg().read()
    }

    #[doc="Get the RLR29 Register."]
    #[inline] pub fn rlr29_reg(&self) -> ::bobbin_mcu::register::Register<Rlr29> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr29, 0xf4)
    }

    #[doc="Get the *mut pointer for the RLR29 register."]
    #[inline] pub fn rlr29_mut(&self) -> *mut Rlr29 { 
        self.rlr29_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR29 register."]
    #[inline] pub fn rlr29_ptr(&self) -> *const Rlr29 { 
        self.rlr29_reg().ptr()
    }

    #[doc="Read the RLR29 register."]
    #[inline] pub fn rlr29(&self) -> Rlr29 { 
        self.rlr29_reg().read()
    }

    #[doc="Get the RLR30 Register."]
    #[inline] pub fn rlr30_reg(&self) -> ::bobbin_mcu::register::Register<Rlr30> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr30, 0xf8)
    }

    #[doc="Get the *mut pointer for the RLR30 register."]
    #[inline] pub fn rlr30_mut(&self) -> *mut Rlr30 { 
        self.rlr30_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR30 register."]
    #[inline] pub fn rlr30_ptr(&self) -> *const Rlr30 { 
        self.rlr30_reg().ptr()
    }

    #[doc="Read the RLR30 register."]
    #[inline] pub fn rlr30(&self) -> Rlr30 { 
        self.rlr30_reg().read()
    }

    #[doc="Get the RLR31 Register."]
    #[inline] pub fn rlr31_reg(&self) -> ::bobbin_mcu::register::Register<Rlr31> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rlr31, 0xfc)
    }

    #[doc="Get the *mut pointer for the RLR31 register."]
    #[inline] pub fn rlr31_mut(&self) -> *mut Rlr31 { 
        self.rlr31_reg().ptr()
    }

    #[doc="Get the *const pointer for the RLR31 register."]
    #[inline] pub fn rlr31_ptr(&self) -> *const Rlr31 { 
        self.rlr31_reg().ptr()
    }

    #[doc="Read the RLR31 register."]
    #[inline] pub fn rlr31(&self) -> Rlr31 { 
        self.rlr31_reg().read()
    }

    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x140)
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

    #[doc="Get the KEYR Register."]
    #[inline] pub fn keyr_reg(&self) -> ::bobbin_mcu::register::Register<Keyr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr, 0x144)
    }

    #[doc="Get the *mut pointer for the KEYR register."]
    #[inline] pub fn keyr_mut(&self) -> *mut Keyr { 
        self.keyr_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR register."]
    #[inline] pub fn keyr_ptr(&self) -> *const Keyr { 
        self.keyr_reg().ptr()
    }

    #[doc="Read the KEYR register."]
    #[inline] pub fn keyr(&self) -> Keyr { 
        self.keyr_reg().read()
    }

    #[doc="Write the KEYR register."]
    #[inline] pub fn write_keyr(&self, value: Keyr) -> &Self { 
        self.keyr_reg().write(value);
        self
    }

    #[doc="Set the KEYR register."]
    #[inline] pub fn set_keyr<F: FnOnce(Keyr) -> Keyr>(&self, f: F) -> &Self {
        self.keyr_reg().set(f);
        self
    }

    #[doc="Modify the KEYR register."]
    #[inline] pub fn with_keyr<F: FnOnce(Keyr) -> Keyr>(&self, f: F) -> &Self {
        self.keyr_reg().with(f);
        self
    }

    #[doc="Get the HWCFGR2 Register."]
    #[inline] pub fn hwcfgr2_reg(&self) -> ::bobbin_mcu::register::Register<Hwcfgr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hwcfgr2, 0x3ec)
    }

    #[doc="Get the *mut pointer for the HWCFGR2 register."]
    #[inline] pub fn hwcfgr2_mut(&self) -> *mut Hwcfgr2 { 
        self.hwcfgr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the HWCFGR2 register."]
    #[inline] pub fn hwcfgr2_ptr(&self) -> *const Hwcfgr2 { 
        self.hwcfgr2_reg().ptr()
    }

    #[doc="Read the HWCFGR2 register."]
    #[inline] pub fn hwcfgr2(&self) -> Hwcfgr2 { 
        self.hwcfgr2_reg().read()
    }

    #[doc="Get the HWCFGR1 Register."]
    #[inline] pub fn hwcfgr1_reg(&self) -> ::bobbin_mcu::register::Register<Hwcfgr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hwcfgr1, 0x3f0)
    }

    #[doc="Get the *mut pointer for the HWCFGR1 register."]
    #[inline] pub fn hwcfgr1_mut(&self) -> *mut Hwcfgr1 { 
        self.hwcfgr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the HWCFGR1 register."]
    #[inline] pub fn hwcfgr1_ptr(&self) -> *const Hwcfgr1 { 
        self.hwcfgr1_reg().ptr()
    }

    #[doc="Read the HWCFGR1 register."]
    #[inline] pub fn hwcfgr1(&self) -> Hwcfgr1 { 
        self.hwcfgr1_reg().read()
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

    #[doc="Get the C1IER0 Register."]
    #[inline] pub fn c1ier0_reg(&self) -> ::bobbin_mcu::register::Register<C1ier0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1ier0, 0x100)
    }

    #[doc="Get the *mut pointer for the C1IER0 register."]
    #[inline] pub fn c1ier0_mut(&self) -> *mut C1ier0 { 
        self.c1ier0_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1IER0 register."]
    #[inline] pub fn c1ier0_ptr(&self) -> *const C1ier0 { 
        self.c1ier0_reg().ptr()
    }

    #[doc="Read the C1IER0 register."]
    #[inline] pub fn c1ier0(&self) -> C1ier0 { 
        self.c1ier0_reg().read()
    }

    #[doc="Write the C1IER0 register."]
    #[inline] pub fn write_c1ier0(&self, value: C1ier0) -> &Self { 
        self.c1ier0_reg().write(value);
        self
    }

    #[doc="Set the C1IER0 register."]
    #[inline] pub fn set_c1ier0<F: FnOnce(C1ier0) -> C1ier0>(&self, f: F) -> &Self {
        self.c1ier0_reg().set(f);
        self
    }

    #[doc="Modify the C1IER0 register."]
    #[inline] pub fn with_c1ier0<F: FnOnce(C1ier0) -> C1ier0>(&self, f: F) -> &Self {
        self.c1ier0_reg().with(f);
        self
    }

    #[doc="Get the C1ICR Register."]
    #[inline] pub fn c1icr_reg(&self) -> ::bobbin_mcu::register::Register<C1icr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1icr, 0x104)
    }

    #[doc="Get the *mut pointer for the C1ICR register."]
    #[inline] pub fn c1icr_mut(&self) -> *mut C1icr { 
        self.c1icr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1ICR register."]
    #[inline] pub fn c1icr_ptr(&self) -> *const C1icr { 
        self.c1icr_reg().ptr()
    }

    #[doc="Read the C1ICR register."]
    #[inline] pub fn c1icr(&self) -> C1icr { 
        self.c1icr_reg().read()
    }

    #[doc="Write the C1ICR register."]
    #[inline] pub fn write_c1icr(&self, value: C1icr) -> &Self { 
        self.c1icr_reg().write(value);
        self
    }

    #[doc="Set the C1ICR register."]
    #[inline] pub fn set_c1icr<F: FnOnce(C1icr) -> C1icr>(&self, f: F) -> &Self {
        self.c1icr_reg().set(f);
        self
    }

    #[doc="Modify the C1ICR register."]
    #[inline] pub fn with_c1icr<F: FnOnce(C1icr) -> C1icr>(&self, f: F) -> &Self {
        self.c1icr_reg().with(f);
        self
    }

    #[doc="Get the C1ISR Register."]
    #[inline] pub fn c1isr_reg(&self) -> ::bobbin_mcu::register::Register<C1isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1isr, 0x108)
    }

    #[doc="Get the *mut pointer for the C1ISR register."]
    #[inline] pub fn c1isr_mut(&self) -> *mut C1isr { 
        self.c1isr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1ISR register."]
    #[inline] pub fn c1isr_ptr(&self) -> *const C1isr { 
        self.c1isr_reg().ptr()
    }

    #[doc="Read the C1ISR register."]
    #[inline] pub fn c1isr(&self) -> C1isr { 
        self.c1isr_reg().read()
    }

    #[doc="Get the C1MISR Register."]
    #[inline] pub fn c1misr_reg(&self) -> ::bobbin_mcu::register::Register<C1misr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1misr, 0x10c)
    }

    #[doc="Get the *mut pointer for the C1MISR register."]
    #[inline] pub fn c1misr_mut(&self) -> *mut C1misr { 
        self.c1misr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1MISR register."]
    #[inline] pub fn c1misr_ptr(&self) -> *const C1misr { 
        self.c1misr_reg().ptr()
    }

    #[doc="Read the C1MISR register."]
    #[inline] pub fn c1misr(&self) -> C1misr { 
        self.c1misr_reg().read()
    }

    #[doc="Get the C2IER0 Register."]
    #[inline] pub fn c2ier0_reg(&self) -> ::bobbin_mcu::register::Register<C2ier0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2ier0, 0x110)
    }

    #[doc="Get the *mut pointer for the C2IER0 register."]
    #[inline] pub fn c2ier0_mut(&self) -> *mut C2ier0 { 
        self.c2ier0_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2IER0 register."]
    #[inline] pub fn c2ier0_ptr(&self) -> *const C2ier0 { 
        self.c2ier0_reg().ptr()
    }

    #[doc="Read the C2IER0 register."]
    #[inline] pub fn c2ier0(&self) -> C2ier0 { 
        self.c2ier0_reg().read()
    }

    #[doc="Write the C2IER0 register."]
    #[inline] pub fn write_c2ier0(&self, value: C2ier0) -> &Self { 
        self.c2ier0_reg().write(value);
        self
    }

    #[doc="Set the C2IER0 register."]
    #[inline] pub fn set_c2ier0<F: FnOnce(C2ier0) -> C2ier0>(&self, f: F) -> &Self {
        self.c2ier0_reg().set(f);
        self
    }

    #[doc="Modify the C2IER0 register."]
    #[inline] pub fn with_c2ier0<F: FnOnce(C2ier0) -> C2ier0>(&self, f: F) -> &Self {
        self.c2ier0_reg().with(f);
        self
    }

    #[doc="Get the C2ICR Register."]
    #[inline] pub fn c2icr_reg(&self) -> ::bobbin_mcu::register::Register<C2icr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2icr, 0x114)
    }

    #[doc="Get the *mut pointer for the C2ICR register."]
    #[inline] pub fn c2icr_mut(&self) -> *mut C2icr { 
        self.c2icr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2ICR register."]
    #[inline] pub fn c2icr_ptr(&self) -> *const C2icr { 
        self.c2icr_reg().ptr()
    }

    #[doc="Read the C2ICR register."]
    #[inline] pub fn c2icr(&self) -> C2icr { 
        self.c2icr_reg().read()
    }

    #[doc="Write the C2ICR register."]
    #[inline] pub fn write_c2icr(&self, value: C2icr) -> &Self { 
        self.c2icr_reg().write(value);
        self
    }

    #[doc="Set the C2ICR register."]
    #[inline] pub fn set_c2icr<F: FnOnce(C2icr) -> C2icr>(&self, f: F) -> &Self {
        self.c2icr_reg().set(f);
        self
    }

    #[doc="Modify the C2ICR register."]
    #[inline] pub fn with_c2icr<F: FnOnce(C2icr) -> C2icr>(&self, f: F) -> &Self {
        self.c2icr_reg().with(f);
        self
    }

    #[doc="Get the C2ISR Register."]
    #[inline] pub fn c2isr_reg(&self) -> ::bobbin_mcu::register::Register<C2isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2isr, 0x118)
    }

    #[doc="Get the *mut pointer for the C2ISR register."]
    #[inline] pub fn c2isr_mut(&self) -> *mut C2isr { 
        self.c2isr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2ISR register."]
    #[inline] pub fn c2isr_ptr(&self) -> *const C2isr { 
        self.c2isr_reg().ptr()
    }

    #[doc="Read the C2ISR register."]
    #[inline] pub fn c2isr(&self) -> C2isr { 
        self.c2isr_reg().read()
    }

    #[doc="Get the C2MISR Register."]
    #[inline] pub fn c2misr_reg(&self) -> ::bobbin_mcu::register::Register<C2misr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2misr, 0x11c)
    }

    #[doc="Get the *mut pointer for the C2MISR register."]
    #[inline] pub fn c2misr_mut(&self) -> *mut C2misr { 
        self.c2misr_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2MISR register."]
    #[inline] pub fn c2misr_ptr(&self) -> *const C2misr { 
        self.c2misr_reg().ptr()
    }

    #[doc="Read the C2MISR register."]
    #[inline] pub fn c2misr(&self) -> C2misr { 
        self.c2misr_reg().read()
    }

}

#[doc="Semaphore 0 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R0(pub u32);
impl R0 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R0 {
    #[inline]
    fn from(other: u32) -> Self {
         R0(other)
    }
}

impl ::core::fmt::Display for R0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 1 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R1(pub u32);
impl R1 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R1 {
    #[inline]
    fn from(other: u32) -> Self {
         R1(other)
    }
}

impl ::core::fmt::Display for R1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 2 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R2(pub u32);
impl R2 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R2 {
    #[inline]
    fn from(other: u32) -> Self {
         R2(other)
    }
}

impl ::core::fmt::Display for R2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 3 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R3(pub u32);
impl R3 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R3 {
    #[inline]
    fn from(other: u32) -> Self {
         R3(other)
    }
}

impl ::core::fmt::Display for R3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 4 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R4(pub u32);
impl R4 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R4 {
    #[inline]
    fn from(other: u32) -> Self {
         R4(other)
    }
}

impl ::core::fmt::Display for R4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 5 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R5(pub u32);
impl R5 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R5 {
    #[inline]
    fn from(other: u32) -> Self {
         R5(other)
    }
}

impl ::core::fmt::Display for R5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 6 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R6(pub u32);
impl R6 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R6 {
    #[inline]
    fn from(other: u32) -> Self {
         R6(other)
    }
}

impl ::core::fmt::Display for R6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 7 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R7(pub u32);
impl R7 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R7 {
    #[inline]
    fn from(other: u32) -> Self {
         R7(other)
    }
}

impl ::core::fmt::Display for R7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 8 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R8(pub u32);
impl R8 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R8 {
    #[inline]
    fn from(other: u32) -> Self {
         R8(other)
    }
}

impl ::core::fmt::Display for R8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 9 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R9(pub u32);
impl R9 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R9 {
    #[inline]
    fn from(other: u32) -> Self {
         R9(other)
    }
}

impl ::core::fmt::Display for R9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 10 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R10(pub u32);
impl R10 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R10 {
    #[inline]
    fn from(other: u32) -> Self {
         R10(other)
    }
}

impl ::core::fmt::Display for R10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 11 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R11(pub u32);
impl R11 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R11 {
    #[inline]
    fn from(other: u32) -> Self {
         R11(other)
    }
}

impl ::core::fmt::Display for R11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 12 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R12(pub u32);
impl R12 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R12 {
    #[inline]
    fn from(other: u32) -> Self {
         R12(other)
    }
}

impl ::core::fmt::Display for R12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 13 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R13(pub u32);
impl R13 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R13 {
    #[inline]
    fn from(other: u32) -> Self {
         R13(other)
    }
}

impl ::core::fmt::Display for R13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 14 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R14(pub u32);
impl R14 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R14 {
    #[inline]
    fn from(other: u32) -> Self {
         R14(other)
    }
}

impl ::core::fmt::Display for R14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 15 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R15(pub u32);
impl R15 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R15 {
    #[inline]
    fn from(other: u32) -> Self {
         R15(other)
    }
}

impl ::core::fmt::Display for R15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 16 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R16(pub u32);
impl R16 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R16 {
    #[inline]
    fn from(other: u32) -> Self {
         R16(other)
    }
}

impl ::core::fmt::Display for R16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 17 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R17(pub u32);
impl R17 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R17 {
    #[inline]
    fn from(other: u32) -> Self {
         R17(other)
    }
}

impl ::core::fmt::Display for R17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 18 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R18(pub u32);
impl R18 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R18 {
    #[inline]
    fn from(other: u32) -> Self {
         R18(other)
    }
}

impl ::core::fmt::Display for R18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 19 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R19(pub u32);
impl R19 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R19 {
    #[inline]
    fn from(other: u32) -> Self {
         R19(other)
    }
}

impl ::core::fmt::Display for R19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 20 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R20(pub u32);
impl R20 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R20 {
    #[inline]
    fn from(other: u32) -> Self {
         R20(other)
    }
}

impl ::core::fmt::Display for R20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 21 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R21(pub u32);
impl R21 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R21 {
    #[inline]
    fn from(other: u32) -> Self {
         R21(other)
    }
}

impl ::core::fmt::Display for R21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 22 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R22(pub u32);
impl R22 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R22 {
    #[inline]
    fn from(other: u32) -> Self {
         R22(other)
    }
}

impl ::core::fmt::Display for R22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 23 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R23(pub u32);
impl R23 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R23 {
    #[inline]
    fn from(other: u32) -> Self {
         R23(other)
    }
}

impl ::core::fmt::Display for R23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 24 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R24(pub u32);
impl R24 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R24 {
    #[inline]
    fn from(other: u32) -> Self {
         R24(other)
    }
}

impl ::core::fmt::Display for R24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 25 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R25(pub u32);
impl R25 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R25 {
    #[inline]
    fn from(other: u32) -> Self {
         R25(other)
    }
}

impl ::core::fmt::Display for R25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 26 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R26(pub u32);
impl R26 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R26 {
    #[inline]
    fn from(other: u32) -> Self {
         R26(other)
    }
}

impl ::core::fmt::Display for R26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 27 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R27(pub u32);
impl R27 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R27 {
    #[inline]
    fn from(other: u32) -> Self {
         R27(other)
    }
}

impl ::core::fmt::Display for R27 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R27 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 28 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R28(pub u32);
impl R28 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R28 {
    #[inline]
    fn from(other: u32) -> Self {
         R28(other)
    }
}

impl ::core::fmt::Display for R28 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R28 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 29 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R29(pub u32);
impl R29 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R29 {
    #[inline]
    fn from(other: u32) -> Self {
         R29(other)
    }
}

impl ::core::fmt::Display for R29 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R29 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 30 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R30(pub u32);
impl R30 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R30 {
    #[inline]
    fn from(other: u32) -> Self {
         R30(other)
    }
}

impl ::core::fmt::Display for R30 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R30 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 31 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R31(pub u32);
impl R31 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R31 {
    #[inline]
    fn from(other: u32) -> Self {
         R31(other)
    }
}

impl ::core::fmt::Display for R31 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R31 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 0 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr0(pub u32);
impl Rlr0 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr0(other)
    }
}

impl ::core::fmt::Display for Rlr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 1 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr1(pub u32);
impl Rlr1 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr1(other)
    }
}

impl ::core::fmt::Display for Rlr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 2 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr2(pub u32);
impl Rlr2 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr2(other)
    }
}

impl ::core::fmt::Display for Rlr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 3 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr3(pub u32);
impl Rlr3 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr3(other)
    }
}

impl ::core::fmt::Display for Rlr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 4 read lock read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr4(pub u32);
impl Rlr4 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr4(other)
    }
}

impl ::core::fmt::Display for Rlr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 5 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr5(pub u32);
impl Rlr5 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr5 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr5(other)
    }
}

impl ::core::fmt::Display for Rlr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 6 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr6(pub u32);
impl Rlr6 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr6 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr6(other)
    }
}

impl ::core::fmt::Display for Rlr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 7 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr7(pub u32);
impl Rlr7 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr7 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr7(other)
    }
}

impl ::core::fmt::Display for Rlr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 8 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr8(pub u32);
impl Rlr8 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr8 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr8(other)
    }
}

impl ::core::fmt::Display for Rlr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 9 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr9(pub u32);
impl Rlr9 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr9 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr9(other)
    }
}

impl ::core::fmt::Display for Rlr9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 10 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr10(pub u32);
impl Rlr10 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr10 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr10(other)
    }
}

impl ::core::fmt::Display for Rlr10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 11 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr11(pub u32);
impl Rlr11 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr11 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr11(other)
    }
}

impl ::core::fmt::Display for Rlr11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 12 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr12(pub u32);
impl Rlr12 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr12 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr12(other)
    }
}

impl ::core::fmt::Display for Rlr12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 13 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr13(pub u32);
impl Rlr13 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr13 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr13(other)
    }
}

impl ::core::fmt::Display for Rlr13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 14 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr14(pub u32);
impl Rlr14 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr14 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr14(other)
    }
}

impl ::core::fmt::Display for Rlr14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 15 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr15(pub u32);
impl Rlr15 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr15 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr15(other)
    }
}

impl ::core::fmt::Display for Rlr15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 16 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr16(pub u32);
impl Rlr16 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr16 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr16(other)
    }
}

impl ::core::fmt::Display for Rlr16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 17 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr17(pub u32);
impl Rlr17 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr17 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr17(other)
    }
}

impl ::core::fmt::Display for Rlr17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 18 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr18(pub u32);
impl Rlr18 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr18 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr18(other)
    }
}

impl ::core::fmt::Display for Rlr18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr18 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 19 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr19(pub u32);
impl Rlr19 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr19 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr19(other)
    }
}

impl ::core::fmt::Display for Rlr19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 20 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr20(pub u32);
impl Rlr20 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr20 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr20(other)
    }
}

impl ::core::fmt::Display for Rlr20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 21 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr21(pub u32);
impl Rlr21 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr21 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr21(other)
    }
}

impl ::core::fmt::Display for Rlr21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 22 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr22(pub u32);
impl Rlr22 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr22 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr22(other)
    }
}

impl ::core::fmt::Display for Rlr22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 23 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr23(pub u32);
impl Rlr23 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr23 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr23(other)
    }
}

impl ::core::fmt::Display for Rlr23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 24 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr24(pub u32);
impl Rlr24 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr24 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr24(other)
    }
}

impl ::core::fmt::Display for Rlr24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 25 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr25(pub u32);
impl Rlr25 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr25 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr25(other)
    }
}

impl ::core::fmt::Display for Rlr25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 26 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr26(pub u32);
impl Rlr26 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr26 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr26(other)
    }
}

impl ::core::fmt::Display for Rlr26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 27 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr27(pub u32);
impl Rlr27 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr27 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr27(other)
    }
}

impl ::core::fmt::Display for Rlr27 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr27 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 28 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr28(pub u32);
impl Rlr28 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr28 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr28(other)
    }
}

impl ::core::fmt::Display for Rlr28 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr28 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 29 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr29(pub u32);
impl Rlr29 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr29 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr29(other)
    }
}

impl ::core::fmt::Display for Rlr29 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr29 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 30 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr30(pub u32);
impl Rlr30 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr30 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr30(other)
    }
}

impl ::core::fmt::Display for Rlr30 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr30 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore 31 read lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr31(pub u32);
impl Rlr31 {
    #[doc="lock indication"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Semaphore CoreID"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Semaphore ProcessID"]
    #[inline] pub fn procid(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROCID != 0"]
    #[inline] pub fn test_procid(&self) -> bool {
        self.procid() != 0
    }

    #[doc="Sets the PROCID field."]
    #[inline] pub fn set_procid<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr31 {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr31(other)
    }
}

impl ::core::fmt::Display for Rlr31 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr31 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        if self.procid() != 0 { try!(write!(f, " procid=0x{:x}", self.procid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore Clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Semaphore clear Key"]
    #[inline] pub fn key(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if KEY != 0"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Sets the KEY field."]
    #[inline] pub fn set_key<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CoreID of semaphore to be cleared"]
    #[inline] pub fn coreid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if COREID != 0"]
    #[inline] pub fn test_coreid(&self) -> bool {
        self.coreid() != 0
    }

    #[doc="Sets the COREID field."]
    #[inline] pub fn set_coreid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
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
        if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
        if self.coreid() != 0 { try!(write!(f, " coreid=0x{:x}", self.coreid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr(pub u32);
impl Keyr {
    #[doc="Semaphore Clear Key"]
    #[inline] pub fn key(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if KEY != 0"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Sets the KEY field."]
    #[inline] pub fn set_key<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Keyr {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr(other)
    }
}

impl ::core::fmt::Display for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore hardware configuration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hwcfgr2(pub u32);
impl Hwcfgr2 {
    #[doc="Hardware Configuration valid bus masters ID4"]
    #[inline] pub fn masterid4(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if MASTERID4 != 0"]
    #[inline] pub fn test_masterid4(&self) -> bool {
        self.masterid4() != 0
    }

    #[doc="Sets the MASTERID4 field."]
    #[inline] pub fn set_masterid4<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Hardware Configuration valid bus masters ID3"]
    #[inline] pub fn masterid3(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MASTERID3 != 0"]
    #[inline] pub fn test_masterid3(&self) -> bool {
        self.masterid3() != 0
    }

    #[doc="Sets the MASTERID3 field."]
    #[inline] pub fn set_masterid3<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Hardware Configuration valid bus masters ID2"]
    #[inline] pub fn masterid2(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if MASTERID2 != 0"]
    #[inline] pub fn test_masterid2(&self) -> bool {
        self.masterid2() != 0
    }

    #[doc="Sets the MASTERID2 field."]
    #[inline] pub fn set_masterid2<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Hardware Configuration valid bus masters ID1"]
    #[inline] pub fn masterid1(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MASTERID1 != 0"]
    #[inline] pub fn test_masterid1(&self) -> bool {
        self.masterid1() != 0
    }

    #[doc="Sets the MASTERID1 field."]
    #[inline] pub fn set_masterid1<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hwcfgr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Hwcfgr2(other)
    }
}

impl ::core::fmt::Display for Hwcfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hwcfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.masterid4() != 0 { try!(write!(f, " masterid4=0x{:x}", self.masterid4()))}
        if self.masterid3() != 0 { try!(write!(f, " masterid3=0x{:x}", self.masterid3()))}
        if self.masterid2() != 0 { try!(write!(f, " masterid2=0x{:x}", self.masterid2()))}
        if self.masterid1() != 0 { try!(write!(f, " masterid1=0x{:x}", self.masterid1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Semaphore hardware configuration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hwcfgr1(pub u32);
impl Hwcfgr1 {
    #[doc="Hardware Configuration number of interrupts supported number of master IDs"]
    #[inline] pub fn nbint(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if NBINT != 0"]
    #[inline] pub fn test_nbint(&self) -> bool {
        self.nbint() != 0
    }

    #[doc="Sets the NBINT field."]
    #[inline] pub fn set_nbint<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Hardware Configuration number of semaphores"]
    #[inline] pub fn nbsem(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if NBSEM != 0"]
    #[inline] pub fn test_nbsem(&self) -> bool {
        self.nbsem() != 0
    }

    #[doc="Sets the NBSEM field."]
    #[inline] pub fn set_nbsem<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Hwcfgr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Hwcfgr1(other)
    }
}

impl ::core::fmt::Display for Hwcfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hwcfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nbint() != 0 { try!(write!(f, " nbint=0x{:x}", self.nbint()))}
        if self.nbsem() != 0 { try!(write!(f, " nbsem=0x{:x}", self.nbsem()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HSEM version register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verr(pub u32);
impl Verr {
    #[doc="Major Revision"]
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

    #[doc="Minor Revision"]
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

#[doc="HSEM indentification register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ipidr(pub u32);
impl Ipidr {
    #[doc="Identification Code"]
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

#[doc="HSEM size indentification register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sidr(pub u32);
impl Sidr {
    #[doc="Size Identification Code"]
    #[inline] pub fn sid(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SID != 0"]
    #[inline] pub fn test_sid(&self) -> bool {
        self.sid() != 0
    }

    #[doc="Sets the SID field."]
    #[inline] pub fn set_sid<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
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

#[doc="HSEM Interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1ier0(pub u32);
impl C1ier0 {
    #[doc="CPU(n) semaphore m enable bit"]
    #[inline] pub fn isem(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ISEM != 0"]
    #[inline] pub fn test_isem(&self) -> bool {
        self.isem() != 0
    }

    #[doc="Sets the ISEM field."]
    #[inline] pub fn set_isem<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C1ier0 {
    #[inline]
    fn from(other: u32) -> Self {
         C1ier0(other)
    }
}

impl ::core::fmt::Display for C1ier0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1ier0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HSEM Interrupt clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1icr(pub u32);
impl C1icr {
    #[doc="CPU(n) semaphore m clear bit"]
    #[inline] pub fn iscm(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ISCM != 0"]
    #[inline] pub fn test_iscm(&self) -> bool {
        self.iscm() != 0
    }

    #[doc="Sets the ISCM field."]
    #[inline] pub fn set_iscm<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C1icr {
    #[inline]
    fn from(other: u32) -> Self {
         C1icr(other)
    }
}

impl ::core::fmt::Display for C1icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HSEM Interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1isr(pub u32);
impl C1isr {
    #[doc="CPU(n) semaphore m status bit before enable (mask)"]
    #[inline] pub fn isfm(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ISFM != 0"]
    #[inline] pub fn test_isfm(&self) -> bool {
        self.isfm() != 0
    }

    #[doc="Sets the ISFM field."]
    #[inline] pub fn set_isfm<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C1isr {
    #[inline]
    fn from(other: u32) -> Self {
         C1isr(other)
    }
}

impl ::core::fmt::Display for C1isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HSEM Masked interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1misr(pub u32);
impl C1misr {
    #[doc="masked CPU(n) semaphore m status bit after enable (mask)."]
    #[inline] pub fn misfm(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if MISFM != 0"]
    #[inline] pub fn test_misfm(&self) -> bool {
        self.misfm() != 0
    }

    #[doc="Sets the MISFM field."]
    #[inline] pub fn set_misfm<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C1misr {
    #[inline]
    fn from(other: u32) -> Self {
         C1misr(other)
    }
}

impl ::core::fmt::Display for C1misr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1misr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HSEM Interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2ier0(pub u32);
impl C2ier0 {
    #[doc="CPU(2) semaphore m enable bit."]
    #[inline] pub fn isem(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ISEM != 0"]
    #[inline] pub fn test_isem(&self) -> bool {
        self.isem() != 0
    }

    #[doc="Sets the ISEM field."]
    #[inline] pub fn set_isem<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2ier0 {
    #[inline]
    fn from(other: u32) -> Self {
         C2ier0(other)
    }
}

impl ::core::fmt::Display for C2ier0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2ier0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HSEM Interrupt clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2icr(pub u32);
impl C2icr {
    #[doc="CPU(2) semaphore m clear bit"]
    #[inline] pub fn iscm(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ISCM != 0"]
    #[inline] pub fn test_iscm(&self) -> bool {
        self.iscm() != 0
    }

    #[doc="Sets the ISCM field."]
    #[inline] pub fn set_iscm<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2icr {
    #[inline]
    fn from(other: u32) -> Self {
         C2icr(other)
    }
}

impl ::core::fmt::Display for C2icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HSEM Interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2isr(pub u32);
impl C2isr {
    #[doc="CPU(2) semaphore m status bit before enable (mask)."]
    #[inline] pub fn isfm(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ISFM != 0"]
    #[inline] pub fn test_isfm(&self) -> bool {
        self.isfm() != 0
    }

    #[doc="Sets the ISFM field."]
    #[inline] pub fn set_isfm<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2isr {
    #[inline]
    fn from(other: u32) -> Self {
         C2isr(other)
    }
}

impl ::core::fmt::Display for C2isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HSEM Masked interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2misr(pub u32);
impl C2misr {
    #[doc="masked CPU(2) semaphore m status bit after enable (mask)."]
    #[inline] pub fn misfm(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if MISFM != 0"]
    #[inline] pub fn test_misfm(&self) -> bool {
        self.misfm() != 0
    }

    #[doc="Sets the MISFM field."]
    #[inline] pub fn set_misfm<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for C2misr {
    #[inline]
    fn from(other: u32) -> Self {
         C2misr(other)
    }
}

impl ::core::fmt::Display for C2misr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2misr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

