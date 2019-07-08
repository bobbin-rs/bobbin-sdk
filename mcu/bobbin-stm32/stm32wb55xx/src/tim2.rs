::bobbin_mcu::periph!( TIM2, Tim2, TIM2_PERIPH, Tim2Periph, TIM2_OWNED, TIM2_REF_COUNT, 0x40000000, 0x00, 0x24);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TIM2 Peripheral"]
pub struct Tim2Periph(pub usize); 

impl Tim2Periph {
    #[doc="Get the CR1 Register."]
    #[inline] pub fn cr1_reg(&self) -> ::bobbin_mcu::register::Register<Cr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr1, 0x0)
    }

    #[doc="Get the *mut pointer for the CR1 register."]
    #[inline] pub fn cr1_mut(&self) -> *mut Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR1 register."]
    #[inline] pub fn cr1_ptr(&self) -> *const Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Read the CR1 register."]
    #[inline] pub fn cr1(&self) -> Cr1 { 
        self.cr1_reg().read()
    }

    #[doc="Write the CR1 register."]
    #[inline] pub fn write_cr1(&self, value: Cr1) -> &Self { 
        self.cr1_reg().write(value);
        self
    }

    #[doc="Set the CR1 register."]
    #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().set(f);
        self
    }

    #[doc="Modify the CR1 register."]
    #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().with(f);
        self
    }

    #[doc="Get the CR2 Register."]
    #[inline] pub fn cr2_reg(&self) -> ::bobbin_mcu::register::Register<Cr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr2, 0x4)
    }

    #[doc="Get the *mut pointer for the CR2 register."]
    #[inline] pub fn cr2_mut(&self) -> *mut Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR2 register."]
    #[inline] pub fn cr2_ptr(&self) -> *const Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Read the CR2 register."]
    #[inline] pub fn cr2(&self) -> Cr2 { 
        self.cr2_reg().read()
    }

    #[doc="Write the CR2 register."]
    #[inline] pub fn write_cr2(&self, value: Cr2) -> &Self { 
        self.cr2_reg().write(value);
        self
    }

    #[doc="Set the CR2 register."]
    #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().set(f);
        self
    }

    #[doc="Modify the CR2 register."]
    #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().with(f);
        self
    }

    #[doc="Get the SMCR Register."]
    #[inline] pub fn smcr_reg(&self) -> ::bobbin_mcu::register::Register<Smcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Smcr, 0x8)
    }

    #[doc="Get the *mut pointer for the SMCR register."]
    #[inline] pub fn smcr_mut(&self) -> *mut Smcr { 
        self.smcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SMCR register."]
    #[inline] pub fn smcr_ptr(&self) -> *const Smcr { 
        self.smcr_reg().ptr()
    }

    #[doc="Read the SMCR register."]
    #[inline] pub fn smcr(&self) -> Smcr { 
        self.smcr_reg().read()
    }

    #[doc="Write the SMCR register."]
    #[inline] pub fn write_smcr(&self, value: Smcr) -> &Self { 
        self.smcr_reg().write(value);
        self
    }

    #[doc="Set the SMCR register."]
    #[inline] pub fn set_smcr<F: FnOnce(Smcr) -> Smcr>(&self, f: F) -> &Self {
        self.smcr_reg().set(f);
        self
    }

    #[doc="Modify the SMCR register."]
    #[inline] pub fn with_smcr<F: FnOnce(Smcr) -> Smcr>(&self, f: F) -> &Self {
        self.smcr_reg().with(f);
        self
    }

    #[doc="Get the DIER Register."]
    #[inline] pub fn dier_reg(&self) -> ::bobbin_mcu::register::Register<Dier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dier, 0xc)
    }

    #[doc="Get the *mut pointer for the DIER register."]
    #[inline] pub fn dier_mut(&self) -> *mut Dier { 
        self.dier_reg().ptr()
    }

    #[doc="Get the *const pointer for the DIER register."]
    #[inline] pub fn dier_ptr(&self) -> *const Dier { 
        self.dier_reg().ptr()
    }

    #[doc="Read the DIER register."]
    #[inline] pub fn dier(&self) -> Dier { 
        self.dier_reg().read()
    }

    #[doc="Write the DIER register."]
    #[inline] pub fn write_dier(&self, value: Dier) -> &Self { 
        self.dier_reg().write(value);
        self
    }

    #[doc="Set the DIER register."]
    #[inline] pub fn set_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &Self {
        self.dier_reg().set(f);
        self
    }

    #[doc="Modify the DIER register."]
    #[inline] pub fn with_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &Self {
        self.dier_reg().with(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x10)
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

    #[doc="Write the SR register."]
    #[inline] pub fn write_sr(&self, value: Sr) -> &Self { 
        self.sr_reg().write(value);
        self
    }

    #[doc="Set the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().set(f);
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().with(f);
        self
    }

    #[doc="Get the EGR Register."]
    #[inline] pub fn egr_reg(&self) -> ::bobbin_mcu::register::Register<Egr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Egr, 0x14)
    }

    #[doc="Get the *mut pointer for the EGR register."]
    #[inline] pub fn egr_mut(&self) -> *mut Egr { 
        self.egr_reg().ptr()
    }

    #[doc="Get the *const pointer for the EGR register."]
    #[inline] pub fn egr_ptr(&self) -> *const Egr { 
        self.egr_reg().ptr()
    }

    #[doc="Write the EGR register."]
    #[inline] pub fn write_egr(&self, value: Egr) -> &Self { 
        self.egr_reg().write(value);
        self
    }

    #[doc="Set the EGR register."]
    #[inline] pub fn set_egr<F: FnOnce(Egr) -> Egr>(&self, f: F) -> &Self {
        self.egr_reg().set(f);
        self
    }

    #[doc="Get the CCMR1_OUTPUT Register."]
    #[inline] pub fn ccmr1_output_reg(&self) -> ::bobbin_mcu::register::Register<Ccmr1Output> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccmr1Output, 0x18)
    }

    #[doc="Get the *mut pointer for the CCMR1_OUTPUT register."]
    #[inline] pub fn ccmr1_output_mut(&self) -> *mut Ccmr1Output { 
        self.ccmr1_output_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCMR1_OUTPUT register."]
    #[inline] pub fn ccmr1_output_ptr(&self) -> *const Ccmr1Output { 
        self.ccmr1_output_reg().ptr()
    }

    #[doc="Read the CCMR1_OUTPUT register."]
    #[inline] pub fn ccmr1_output(&self) -> Ccmr1Output { 
        self.ccmr1_output_reg().read()
    }

    #[doc="Write the CCMR1_OUTPUT register."]
    #[inline] pub fn write_ccmr1_output(&self, value: Ccmr1Output) -> &Self { 
        self.ccmr1_output_reg().write(value);
        self
    }

    #[doc="Set the CCMR1_OUTPUT register."]
    #[inline] pub fn set_ccmr1_output<F: FnOnce(Ccmr1Output) -> Ccmr1Output>(&self, f: F) -> &Self {
        self.ccmr1_output_reg().set(f);
        self
    }

    #[doc="Modify the CCMR1_OUTPUT register."]
    #[inline] pub fn with_ccmr1_output<F: FnOnce(Ccmr1Output) -> Ccmr1Output>(&self, f: F) -> &Self {
        self.ccmr1_output_reg().with(f);
        self
    }

    #[doc="Get the CCMR1_INPUT Register."]
    #[inline] pub fn ccmr1_input_reg(&self) -> ::bobbin_mcu::register::Register<Ccmr1Input> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccmr1Input, 0x18)
    }

    #[doc="Get the *mut pointer for the CCMR1_INPUT register."]
    #[inline] pub fn ccmr1_input_mut(&self) -> *mut Ccmr1Input { 
        self.ccmr1_input_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCMR1_INPUT register."]
    #[inline] pub fn ccmr1_input_ptr(&self) -> *const Ccmr1Input { 
        self.ccmr1_input_reg().ptr()
    }

    #[doc="Read the CCMR1_INPUT register."]
    #[inline] pub fn ccmr1_input(&self) -> Ccmr1Input { 
        self.ccmr1_input_reg().read()
    }

    #[doc="Write the CCMR1_INPUT register."]
    #[inline] pub fn write_ccmr1_input(&self, value: Ccmr1Input) -> &Self { 
        self.ccmr1_input_reg().write(value);
        self
    }

    #[doc="Set the CCMR1_INPUT register."]
    #[inline] pub fn set_ccmr1_input<F: FnOnce(Ccmr1Input) -> Ccmr1Input>(&self, f: F) -> &Self {
        self.ccmr1_input_reg().set(f);
        self
    }

    #[doc="Modify the CCMR1_INPUT register."]
    #[inline] pub fn with_ccmr1_input<F: FnOnce(Ccmr1Input) -> Ccmr1Input>(&self, f: F) -> &Self {
        self.ccmr1_input_reg().with(f);
        self
    }

    #[doc="Get the CCMR2_OUTPUT Register."]
    #[inline] pub fn ccmr2_output_reg(&self) -> ::bobbin_mcu::register::Register<Ccmr2Output> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccmr2Output, 0x1c)
    }

    #[doc="Get the *mut pointer for the CCMR2_OUTPUT register."]
    #[inline] pub fn ccmr2_output_mut(&self) -> *mut Ccmr2Output { 
        self.ccmr2_output_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCMR2_OUTPUT register."]
    #[inline] pub fn ccmr2_output_ptr(&self) -> *const Ccmr2Output { 
        self.ccmr2_output_reg().ptr()
    }

    #[doc="Read the CCMR2_OUTPUT register."]
    #[inline] pub fn ccmr2_output(&self) -> Ccmr2Output { 
        self.ccmr2_output_reg().read()
    }

    #[doc="Write the CCMR2_OUTPUT register."]
    #[inline] pub fn write_ccmr2_output(&self, value: Ccmr2Output) -> &Self { 
        self.ccmr2_output_reg().write(value);
        self
    }

    #[doc="Set the CCMR2_OUTPUT register."]
    #[inline] pub fn set_ccmr2_output<F: FnOnce(Ccmr2Output) -> Ccmr2Output>(&self, f: F) -> &Self {
        self.ccmr2_output_reg().set(f);
        self
    }

    #[doc="Modify the CCMR2_OUTPUT register."]
    #[inline] pub fn with_ccmr2_output<F: FnOnce(Ccmr2Output) -> Ccmr2Output>(&self, f: F) -> &Self {
        self.ccmr2_output_reg().with(f);
        self
    }

    #[doc="Get the CCMR2_INPUT Register."]
    #[inline] pub fn ccmr2_input_reg(&self) -> ::bobbin_mcu::register::Register<Ccmr2Input> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccmr2Input, 0x1c)
    }

    #[doc="Get the *mut pointer for the CCMR2_INPUT register."]
    #[inline] pub fn ccmr2_input_mut(&self) -> *mut Ccmr2Input { 
        self.ccmr2_input_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCMR2_INPUT register."]
    #[inline] pub fn ccmr2_input_ptr(&self) -> *const Ccmr2Input { 
        self.ccmr2_input_reg().ptr()
    }

    #[doc="Read the CCMR2_INPUT register."]
    #[inline] pub fn ccmr2_input(&self) -> Ccmr2Input { 
        self.ccmr2_input_reg().read()
    }

    #[doc="Write the CCMR2_INPUT register."]
    #[inline] pub fn write_ccmr2_input(&self, value: Ccmr2Input) -> &Self { 
        self.ccmr2_input_reg().write(value);
        self
    }

    #[doc="Set the CCMR2_INPUT register."]
    #[inline] pub fn set_ccmr2_input<F: FnOnce(Ccmr2Input) -> Ccmr2Input>(&self, f: F) -> &Self {
        self.ccmr2_input_reg().set(f);
        self
    }

    #[doc="Modify the CCMR2_INPUT register."]
    #[inline] pub fn with_ccmr2_input<F: FnOnce(Ccmr2Input) -> Ccmr2Input>(&self, f: F) -> &Self {
        self.ccmr2_input_reg().with(f);
        self
    }

    #[doc="Get the CCER Register."]
    #[inline] pub fn ccer_reg(&self) -> ::bobbin_mcu::register::Register<Ccer> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccer, 0x20)
    }

    #[doc="Get the *mut pointer for the CCER register."]
    #[inline] pub fn ccer_mut(&self) -> *mut Ccer { 
        self.ccer_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCER register."]
    #[inline] pub fn ccer_ptr(&self) -> *const Ccer { 
        self.ccer_reg().ptr()
    }

    #[doc="Read the CCER register."]
    #[inline] pub fn ccer(&self) -> Ccer { 
        self.ccer_reg().read()
    }

    #[doc="Write the CCER register."]
    #[inline] pub fn write_ccer(&self, value: Ccer) -> &Self { 
        self.ccer_reg().write(value);
        self
    }

    #[doc="Set the CCER register."]
    #[inline] pub fn set_ccer<F: FnOnce(Ccer) -> Ccer>(&self, f: F) -> &Self {
        self.ccer_reg().set(f);
        self
    }

    #[doc="Modify the CCER register."]
    #[inline] pub fn with_ccer<F: FnOnce(Ccer) -> Ccer>(&self, f: F) -> &Self {
        self.ccer_reg().with(f);
        self
    }

    #[doc="Get the CNT Register."]
    #[inline] pub fn cnt_reg(&self) -> ::bobbin_mcu::register::Register<Cnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cnt, 0x24)
    }

    #[doc="Get the *mut pointer for the CNT register."]
    #[inline] pub fn cnt_mut(&self) -> *mut Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNT register."]
    #[inline] pub fn cnt_ptr(&self) -> *const Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Read the CNT register."]
    #[inline] pub fn cnt(&self) -> Cnt { 
        self.cnt_reg().read()
    }

    #[doc="Write the CNT register."]
    #[inline] pub fn write_cnt(&self, value: Cnt) -> &Self { 
        self.cnt_reg().write(value);
        self
    }

    #[doc="Set the CNT register."]
    #[inline] pub fn set_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        self.cnt_reg().set(f);
        self
    }

    #[doc="Modify the CNT register."]
    #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        self.cnt_reg().with(f);
        self
    }

    #[doc="Get the PSC Register."]
    #[inline] pub fn psc_reg(&self) -> ::bobbin_mcu::register::Register<Psc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Psc, 0x28)
    }

    #[doc="Get the *mut pointer for the PSC register."]
    #[inline] pub fn psc_mut(&self) -> *mut Psc { 
        self.psc_reg().ptr()
    }

    #[doc="Get the *const pointer for the PSC register."]
    #[inline] pub fn psc_ptr(&self) -> *const Psc { 
        self.psc_reg().ptr()
    }

    #[doc="Read the PSC register."]
    #[inline] pub fn psc(&self) -> Psc { 
        self.psc_reg().read()
    }

    #[doc="Write the PSC register."]
    #[inline] pub fn write_psc(&self, value: Psc) -> &Self { 
        self.psc_reg().write(value);
        self
    }

    #[doc="Set the PSC register."]
    #[inline] pub fn set_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &Self {
        self.psc_reg().set(f);
        self
    }

    #[doc="Modify the PSC register."]
    #[inline] pub fn with_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &Self {
        self.psc_reg().with(f);
        self
    }

    #[doc="Get the ARR Register."]
    #[inline] pub fn arr_reg(&self) -> ::bobbin_mcu::register::Register<Arr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Arr, 0x2c)
    }

    #[doc="Get the *mut pointer for the ARR register."]
    #[inline] pub fn arr_mut(&self) -> *mut Arr { 
        self.arr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ARR register."]
    #[inline] pub fn arr_ptr(&self) -> *const Arr { 
        self.arr_reg().ptr()
    }

    #[doc="Read the ARR register."]
    #[inline] pub fn arr(&self) -> Arr { 
        self.arr_reg().read()
    }

    #[doc="Write the ARR register."]
    #[inline] pub fn write_arr(&self, value: Arr) -> &Self { 
        self.arr_reg().write(value);
        self
    }

    #[doc="Set the ARR register."]
    #[inline] pub fn set_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
        self.arr_reg().set(f);
        self
    }

    #[doc="Modify the ARR register."]
    #[inline] pub fn with_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
        self.arr_reg().with(f);
        self
    }

    #[doc="Get the CCR1 Register."]
    #[inline] pub fn ccr1_reg(&self) -> ::bobbin_mcu::register::Register<Ccr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccr1, 0x34)
    }

    #[doc="Get the *mut pointer for the CCR1 register."]
    #[inline] pub fn ccr1_mut(&self) -> *mut Ccr1 { 
        self.ccr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCR1 register."]
    #[inline] pub fn ccr1_ptr(&self) -> *const Ccr1 { 
        self.ccr1_reg().ptr()
    }

    #[doc="Read the CCR1 register."]
    #[inline] pub fn ccr1(&self) -> Ccr1 { 
        self.ccr1_reg().read()
    }

    #[doc="Write the CCR1 register."]
    #[inline] pub fn write_ccr1(&self, value: Ccr1) -> &Self { 
        self.ccr1_reg().write(value);
        self
    }

    #[doc="Set the CCR1 register."]
    #[inline] pub fn set_ccr1<F: FnOnce(Ccr1) -> Ccr1>(&self, f: F) -> &Self {
        self.ccr1_reg().set(f);
        self
    }

    #[doc="Modify the CCR1 register."]
    #[inline] pub fn with_ccr1<F: FnOnce(Ccr1) -> Ccr1>(&self, f: F) -> &Self {
        self.ccr1_reg().with(f);
        self
    }

    #[doc="Get the CCR2 Register."]
    #[inline] pub fn ccr2_reg(&self) -> ::bobbin_mcu::register::Register<Ccr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccr2, 0x38)
    }

    #[doc="Get the *mut pointer for the CCR2 register."]
    #[inline] pub fn ccr2_mut(&self) -> *mut Ccr2 { 
        self.ccr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCR2 register."]
    #[inline] pub fn ccr2_ptr(&self) -> *const Ccr2 { 
        self.ccr2_reg().ptr()
    }

    #[doc="Read the CCR2 register."]
    #[inline] pub fn ccr2(&self) -> Ccr2 { 
        self.ccr2_reg().read()
    }

    #[doc="Write the CCR2 register."]
    #[inline] pub fn write_ccr2(&self, value: Ccr2) -> &Self { 
        self.ccr2_reg().write(value);
        self
    }

    #[doc="Set the CCR2 register."]
    #[inline] pub fn set_ccr2<F: FnOnce(Ccr2) -> Ccr2>(&self, f: F) -> &Self {
        self.ccr2_reg().set(f);
        self
    }

    #[doc="Modify the CCR2 register."]
    #[inline] pub fn with_ccr2<F: FnOnce(Ccr2) -> Ccr2>(&self, f: F) -> &Self {
        self.ccr2_reg().with(f);
        self
    }

    #[doc="Get the CCR3 Register."]
    #[inline] pub fn ccr3_reg(&self) -> ::bobbin_mcu::register::Register<Ccr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccr3, 0x3c)
    }

    #[doc="Get the *mut pointer for the CCR3 register."]
    #[inline] pub fn ccr3_mut(&self) -> *mut Ccr3 { 
        self.ccr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCR3 register."]
    #[inline] pub fn ccr3_ptr(&self) -> *const Ccr3 { 
        self.ccr3_reg().ptr()
    }

    #[doc="Read the CCR3 register."]
    #[inline] pub fn ccr3(&self) -> Ccr3 { 
        self.ccr3_reg().read()
    }

    #[doc="Write the CCR3 register."]
    #[inline] pub fn write_ccr3(&self, value: Ccr3) -> &Self { 
        self.ccr3_reg().write(value);
        self
    }

    #[doc="Set the CCR3 register."]
    #[inline] pub fn set_ccr3<F: FnOnce(Ccr3) -> Ccr3>(&self, f: F) -> &Self {
        self.ccr3_reg().set(f);
        self
    }

    #[doc="Modify the CCR3 register."]
    #[inline] pub fn with_ccr3<F: FnOnce(Ccr3) -> Ccr3>(&self, f: F) -> &Self {
        self.ccr3_reg().with(f);
        self
    }

    #[doc="Get the CCR4 Register."]
    #[inline] pub fn ccr4_reg(&self) -> ::bobbin_mcu::register::Register<Ccr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccr4, 0x40)
    }

    #[doc="Get the *mut pointer for the CCR4 register."]
    #[inline] pub fn ccr4_mut(&self) -> *mut Ccr4 { 
        self.ccr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCR4 register."]
    #[inline] pub fn ccr4_ptr(&self) -> *const Ccr4 { 
        self.ccr4_reg().ptr()
    }

    #[doc="Read the CCR4 register."]
    #[inline] pub fn ccr4(&self) -> Ccr4 { 
        self.ccr4_reg().read()
    }

    #[doc="Write the CCR4 register."]
    #[inline] pub fn write_ccr4(&self, value: Ccr4) -> &Self { 
        self.ccr4_reg().write(value);
        self
    }

    #[doc="Set the CCR4 register."]
    #[inline] pub fn set_ccr4<F: FnOnce(Ccr4) -> Ccr4>(&self, f: F) -> &Self {
        self.ccr4_reg().set(f);
        self
    }

    #[doc="Modify the CCR4 register."]
    #[inline] pub fn with_ccr4<F: FnOnce(Ccr4) -> Ccr4>(&self, f: F) -> &Self {
        self.ccr4_reg().with(f);
        self
    }

    #[doc="Get the DCR Register."]
    #[inline] pub fn dcr_reg(&self) -> ::bobbin_mcu::register::Register<Dcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dcr, 0x48)
    }

    #[doc="Get the *mut pointer for the DCR register."]
    #[inline] pub fn dcr_mut(&self) -> *mut Dcr { 
        self.dcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DCR register."]
    #[inline] pub fn dcr_ptr(&self) -> *const Dcr { 
        self.dcr_reg().ptr()
    }

    #[doc="Read the DCR register."]
    #[inline] pub fn dcr(&self) -> Dcr { 
        self.dcr_reg().read()
    }

    #[doc="Write the DCR register."]
    #[inline] pub fn write_dcr(&self, value: Dcr) -> &Self { 
        self.dcr_reg().write(value);
        self
    }

    #[doc="Set the DCR register."]
    #[inline] pub fn set_dcr<F: FnOnce(Dcr) -> Dcr>(&self, f: F) -> &Self {
        self.dcr_reg().set(f);
        self
    }

    #[doc="Modify the DCR register."]
    #[inline] pub fn with_dcr<F: FnOnce(Dcr) -> Dcr>(&self, f: F) -> &Self {
        self.dcr_reg().with(f);
        self
    }

    #[doc="Get the DMAR Register."]
    #[inline] pub fn dmar_reg(&self) -> ::bobbin_mcu::register::Register<Dmar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dmar, 0x4c)
    }

    #[doc="Get the *mut pointer for the DMAR register."]
    #[inline] pub fn dmar_mut(&self) -> *mut Dmar { 
        self.dmar_reg().ptr()
    }

    #[doc="Get the *const pointer for the DMAR register."]
    #[inline] pub fn dmar_ptr(&self) -> *const Dmar { 
        self.dmar_reg().ptr()
    }

    #[doc="Read the DMAR register."]
    #[inline] pub fn dmar(&self) -> Dmar { 
        self.dmar_reg().read()
    }

    #[doc="Write the DMAR register."]
    #[inline] pub fn write_dmar(&self, value: Dmar) -> &Self { 
        self.dmar_reg().write(value);
        self
    }

    #[doc="Set the DMAR register."]
    #[inline] pub fn set_dmar<F: FnOnce(Dmar) -> Dmar>(&self, f: F) -> &Self {
        self.dmar_reg().set(f);
        self
    }

    #[doc="Modify the DMAR register."]
    #[inline] pub fn with_dmar<F: FnOnce(Dmar) -> Dmar>(&self, f: F) -> &Self {
        self.dmar_reg().with(f);
        self
    }

    #[doc="Get the OR Register."]
    #[inline] pub fn or_reg(&self) -> ::bobbin_mcu::register::Register<Or> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Or, 0x50)
    }

    #[doc="Get the *mut pointer for the OR register."]
    #[inline] pub fn or_mut(&self) -> *mut Or { 
        self.or_reg().ptr()
    }

    #[doc="Get the *const pointer for the OR register."]
    #[inline] pub fn or_ptr(&self) -> *const Or { 
        self.or_reg().ptr()
    }

    #[doc="Read the OR register."]
    #[inline] pub fn or(&self) -> Or { 
        self.or_reg().read()
    }

    #[doc="Write the OR register."]
    #[inline] pub fn write_or(&self, value: Or) -> &Self { 
        self.or_reg().write(value);
        self
    }

    #[doc="Set the OR register."]
    #[inline] pub fn set_or<F: FnOnce(Or) -> Or>(&self, f: F) -> &Self {
        self.or_reg().set(f);
        self
    }

    #[doc="Modify the OR register."]
    #[inline] pub fn with_or<F: FnOnce(Or) -> Or>(&self, f: F) -> &Self {
        self.or_reg().with(f);
        self
    }

    #[doc="Get the AF Register."]
    #[inline] pub fn af_reg(&self) -> ::bobbin_mcu::register::Register<Af> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Af, 0x60)
    }

    #[doc="Get the *mut pointer for the AF register."]
    #[inline] pub fn af_mut(&self) -> *mut Af { 
        self.af_reg().ptr()
    }

    #[doc="Get the *const pointer for the AF register."]
    #[inline] pub fn af_ptr(&self) -> *const Af { 
        self.af_reg().ptr()
    }

    #[doc="Read the AF register."]
    #[inline] pub fn af(&self) -> Af { 
        self.af_reg().read()
    }

    #[doc="Write the AF register."]
    #[inline] pub fn write_af(&self, value: Af) -> &Self { 
        self.af_reg().write(value);
        self
    }

    #[doc="Set the AF register."]
    #[inline] pub fn set_af<F: FnOnce(Af) -> Af>(&self, f: F) -> &Self {
        self.af_reg().set(f);
        self
    }

    #[doc="Modify the AF register."]
    #[inline] pub fn with_af<F: FnOnce(Af) -> Af>(&self, f: F) -> &Self {
        self.af_reg().with(f);
        self
    }

}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="UIF status bit remapping"]
    #[inline] pub fn uifremap(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if UIFREMAP != 0"]
    #[inline] pub fn test_uifremap(&self) -> bool {
        self.uifremap() != 0
    }

    #[doc="Sets the UIFREMAP field."]
    #[inline] pub fn set_uifremap<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Clock division"]
    #[inline] pub fn ckd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if CKD != 0"]
    #[inline] pub fn test_ckd(&self) -> bool {
        self.ckd() != 0
    }

    #[doc="Sets the CKD field."]
    #[inline] pub fn set_ckd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Auto-reload preload enable"]
    #[inline] pub fn arpe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ARPE != 0"]
    #[inline] pub fn test_arpe(&self) -> bool {
        self.arpe() != 0
    }

    #[doc="Sets the ARPE field."]
    #[inline] pub fn set_arpe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Center-aligned mode selection"]
    #[inline] pub fn cms(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if CMS != 0"]
    #[inline] pub fn test_cms(&self) -> bool {
        self.cms() != 0
    }

    #[doc="Sets the CMS field."]
    #[inline] pub fn set_cms<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Direction"]
    #[inline] pub fn dir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="One-pulse mode"]
    #[inline] pub fn opm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OPM != 0"]
    #[inline] pub fn test_opm(&self) -> bool {
        self.opm() != 0
    }

    #[doc="Sets the OPM field."]
    #[inline] pub fn set_opm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Update request source"]
    #[inline] pub fn urs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if URS != 0"]
    #[inline] pub fn test_urs(&self) -> bool {
        self.urs() != 0
    }

    #[doc="Sets the URS field."]
    #[inline] pub fn set_urs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Update disable"]
    #[inline] pub fn udis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if UDIS != 0"]
    #[inline] pub fn test_udis(&self) -> bool {
        self.udis() != 0
    }

    #[doc="Sets the UDIS field."]
    #[inline] pub fn set_udis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter enable"]
    #[inline] pub fn cen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CEN != 0"]
    #[inline] pub fn test_cen(&self) -> bool {
        self.cen() != 0
    }

    #[doc="Sets the CEN field."]
    #[inline] pub fn set_cen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr1(other)
    }
}

impl ::core::fmt::Display for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.uifremap() != 0 { try!(write!(f, " uifremap"))}
        if self.ckd() != 0 { try!(write!(f, " ckd=0x{:x}", self.ckd()))}
        if self.arpe() != 0 { try!(write!(f, " arpe"))}
        if self.cms() != 0 { try!(write!(f, " cms=0x{:x}", self.cms()))}
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.opm() != 0 { try!(write!(f, " opm"))}
        if self.urs() != 0 { try!(write!(f, " urs"))}
        if self.udis() != 0 { try!(write!(f, " udis"))}
        if self.cen() != 0 { try!(write!(f, " cen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="TI1 selection"]
    #[inline] pub fn ti1s(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TI1S != 0"]
    #[inline] pub fn test_ti1s(&self) -> bool {
        self.ti1s() != 0
    }

    #[doc="Sets the TI1S field."]
    #[inline] pub fn set_ti1s<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Master mode selection"]
    #[inline] pub fn mms(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if MMS != 0"]
    #[inline] pub fn test_mms(&self) -> bool {
        self.mms() != 0
    }

    #[doc="Sets the MMS field."]
    #[inline] pub fn set_mms<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Capture/compare DMA selection"]
    #[inline] pub fn ccds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CCDS != 0"]
    #[inline] pub fn test_ccds(&self) -> bool {
        self.ccds() != 0
    }

    #[doc="Sets the CCDS field."]
    #[inline] pub fn set_ccds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr2(other)
    }
}

impl ::core::fmt::Display for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ti1s() != 0 { try!(write!(f, " ti1s"))}
        if self.mms() != 0 { try!(write!(f, " mms=0x{:x}", self.mms()))}
        if self.ccds() != 0 { try!(write!(f, " ccds"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="slave mode control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Smcr(pub u32);
impl Smcr {
    #[doc="Slave mode selection - bit 3"]
    #[inline] pub fn sms_3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SMS_3 != 0"]
    #[inline] pub fn test_sms_3(&self) -> bool {
        self.sms_3() != 0
    }

    #[doc="Sets the SMS_3 field."]
    #[inline] pub fn set_sms_3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="External trigger polarity"]
    #[inline] pub fn etp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ETP != 0"]
    #[inline] pub fn test_etp(&self) -> bool {
        self.etp() != 0
    }

    #[doc="Sets the ETP field."]
    #[inline] pub fn set_etp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="External clock enable"]
    #[inline] pub fn ece(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ECE != 0"]
    #[inline] pub fn test_ece(&self) -> bool {
        self.ece() != 0
    }

    #[doc="Sets the ECE field."]
    #[inline] pub fn set_ece<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="External trigger prescaler"]
    #[inline] pub fn etps(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if ETPS != 0"]
    #[inline] pub fn test_etps(&self) -> bool {
        self.etps() != 0
    }

    #[doc="Sets the ETPS field."]
    #[inline] pub fn set_etps<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="External trigger filter"]
    #[inline] pub fn etf(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if ETF != 0"]
    #[inline] pub fn test_etf(&self) -> bool {
        self.etf() != 0
    }

    #[doc="Sets the ETF field."]
    #[inline] pub fn set_etf<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Master/Slave mode"]
    #[inline] pub fn msm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MSM != 0"]
    #[inline] pub fn test_msm(&self) -> bool {
        self.msm() != 0
    }

    #[doc="Sets the MSM field."]
    #[inline] pub fn set_msm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Trigger selection"]
    #[inline] pub fn ts(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if TS != 0"]
    #[inline] pub fn test_ts(&self) -> bool {
        self.ts() != 0
    }

    #[doc="Sets the TS field."]
    #[inline] pub fn set_ts<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="OCREF clear selection"]
    #[inline] pub fn occs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OCCS != 0"]
    #[inline] pub fn test_occs(&self) -> bool {
        self.occs() != 0
    }

    #[doc="Sets the OCCS field."]
    #[inline] pub fn set_occs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Slave mode selection"]
    #[inline] pub fn sms(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SMS != 0"]
    #[inline] pub fn test_sms(&self) -> bool {
        self.sms() != 0
    }

    #[doc="Sets the SMS field."]
    #[inline] pub fn set_sms<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Smcr {
    #[inline]
    fn from(other: u32) -> Self {
         Smcr(other)
    }
}

impl ::core::fmt::Display for Smcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Smcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sms_3() != 0 { try!(write!(f, " sms_3"))}
        if self.etp() != 0 { try!(write!(f, " etp"))}
        if self.ece() != 0 { try!(write!(f, " ece"))}
        if self.etps() != 0 { try!(write!(f, " etps=0x{:x}", self.etps()))}
        if self.etf() != 0 { try!(write!(f, " etf=0x{:x}", self.etf()))}
        if self.msm() != 0 { try!(write!(f, " msm"))}
        if self.ts() != 0 { try!(write!(f, " ts=0x{:x}", self.ts()))}
        if self.occs() != 0 { try!(write!(f, " occs"))}
        if self.sms() != 0 { try!(write!(f, " sms=0x{:x}", self.sms()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA/Interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dier(pub u32);
impl Dier {
    #[doc="Capture/Compare 4 DMA request enable"]
    #[inline] pub fn cc4de(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CC4DE != 0"]
    #[inline] pub fn test_cc4de(&self) -> bool {
        self.cc4de() != 0
    }

    #[doc="Sets the CC4DE field."]
    #[inline] pub fn set_cc4de<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Capture/Compare 3 DMA request enable"]
    #[inline] pub fn cc3de(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CC3DE != 0"]
    #[inline] pub fn test_cc3de(&self) -> bool {
        self.cc3de() != 0
    }

    #[doc="Sets the CC3DE field."]
    #[inline] pub fn set_cc3de<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Capture/Compare 2 DMA request enable"]
    #[inline] pub fn cc2de(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CC2DE != 0"]
    #[inline] pub fn test_cc2de(&self) -> bool {
        self.cc2de() != 0
    }

    #[doc="Sets the CC2DE field."]
    #[inline] pub fn set_cc2de<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Capture/Compare 1 DMA request enable"]
    #[inline] pub fn cc1de(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CC1DE != 0"]
    #[inline] pub fn test_cc1de(&self) -> bool {
        self.cc1de() != 0
    }

    #[doc="Sets the CC1DE field."]
    #[inline] pub fn set_cc1de<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Update DMA request enable"]
    #[inline] pub fn ude(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if UDE != 0"]
    #[inline] pub fn test_ude(&self) -> bool {
        self.ude() != 0
    }

    #[doc="Sets the UDE field."]
    #[inline] pub fn set_ude<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Trigger interrupt enable"]
    #[inline] pub fn tie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIE != 0"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie() != 0
    }

    #[doc="Sets the TIE field."]
    #[inline] pub fn set_tie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Capture/Compare 4 interrupt enable"]
    #[inline] pub fn cc4ie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CC4IE != 0"]
    #[inline] pub fn test_cc4ie(&self) -> bool {
        self.cc4ie() != 0
    }

    #[doc="Sets the CC4IE field."]
    #[inline] pub fn set_cc4ie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Capture/Compare 3 interrupt enable"]
    #[inline] pub fn cc3ie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CC3IE != 0"]
    #[inline] pub fn test_cc3ie(&self) -> bool {
        self.cc3ie() != 0
    }

    #[doc="Sets the CC3IE field."]
    #[inline] pub fn set_cc3ie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Capture/Compare 2 interrupt enable"]
    #[inline] pub fn cc2ie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CC2IE != 0"]
    #[inline] pub fn test_cc2ie(&self) -> bool {
        self.cc2ie() != 0
    }

    #[doc="Sets the CC2IE field."]
    #[inline] pub fn set_cc2ie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Capture/Compare 1 interrupt enable"]
    #[inline] pub fn cc1ie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CC1IE != 0"]
    #[inline] pub fn test_cc1ie(&self) -> bool {
        self.cc1ie() != 0
    }

    #[doc="Sets the CC1IE field."]
    #[inline] pub fn set_cc1ie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Update interrupt enable"]
    #[inline] pub fn uie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UIE != 0"]
    #[inline] pub fn test_uie(&self) -> bool {
        self.uie() != 0
    }

    #[doc="Sets the UIE field."]
    #[inline] pub fn set_uie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dier {
    #[inline]
    fn from(other: u32) -> Self {
         Dier(other)
    }
}

impl ::core::fmt::Display for Dier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cc4de() != 0 { try!(write!(f, " cc4de"))}
        if self.cc3de() != 0 { try!(write!(f, " cc3de"))}
        if self.cc2de() != 0 { try!(write!(f, " cc2de"))}
        if self.cc1de() != 0 { try!(write!(f, " cc1de"))}
        if self.ude() != 0 { try!(write!(f, " ude"))}
        if self.tie() != 0 { try!(write!(f, " tie"))}
        if self.cc4ie() != 0 { try!(write!(f, " cc4ie"))}
        if self.cc3ie() != 0 { try!(write!(f, " cc3ie"))}
        if self.cc2ie() != 0 { try!(write!(f, " cc2ie"))}
        if self.cc1ie() != 0 { try!(write!(f, " cc1ie"))}
        if self.uie() != 0 { try!(write!(f, " uie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Capture/Compare 4 overcapture flag"]
    #[inline] pub fn cc4of(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CC4OF != 0"]
    #[inline] pub fn test_cc4of(&self) -> bool {
        self.cc4of() != 0
    }

    #[doc="Sets the CC4OF field."]
    #[inline] pub fn set_cc4of<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Capture/Compare 3 overcapture flag"]
    #[inline] pub fn cc3of(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CC3OF != 0"]
    #[inline] pub fn test_cc3of(&self) -> bool {
        self.cc3of() != 0
    }

    #[doc="Sets the CC3OF field."]
    #[inline] pub fn set_cc3of<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Capture/compare 2 overcapture flag"]
    #[inline] pub fn cc2of(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CC2OF != 0"]
    #[inline] pub fn test_cc2of(&self) -> bool {
        self.cc2of() != 0
    }

    #[doc="Sets the CC2OF field."]
    #[inline] pub fn set_cc2of<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Capture/Compare 1 overcapture flag"]
    #[inline] pub fn cc1of(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CC1OF != 0"]
    #[inline] pub fn test_cc1of(&self) -> bool {
        self.cc1of() != 0
    }

    #[doc="Sets the CC1OF field."]
    #[inline] pub fn set_cc1of<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Trigger interrupt flag"]
    #[inline] pub fn tif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIF != 0"]
    #[inline] pub fn test_tif(&self) -> bool {
        self.tif() != 0
    }

    #[doc="Sets the TIF field."]
    #[inline] pub fn set_tif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Capture/Compare 4 interrupt flag"]
    #[inline] pub fn cc4if(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CC4IF != 0"]
    #[inline] pub fn test_cc4if(&self) -> bool {
        self.cc4if() != 0
    }

    #[doc="Sets the CC4IF field."]
    #[inline] pub fn set_cc4if<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Capture/Compare 3 interrupt flag"]
    #[inline] pub fn cc3if(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CC3IF != 0"]
    #[inline] pub fn test_cc3if(&self) -> bool {
        self.cc3if() != 0
    }

    #[doc="Sets the CC3IF field."]
    #[inline] pub fn set_cc3if<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Capture/Compare 2 interrupt flag"]
    #[inline] pub fn cc2if(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CC2IF != 0"]
    #[inline] pub fn test_cc2if(&self) -> bool {
        self.cc2if() != 0
    }

    #[doc="Sets the CC2IF field."]
    #[inline] pub fn set_cc2if<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Capture/compare 1 interrupt flag"]
    #[inline] pub fn cc1if(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CC1IF != 0"]
    #[inline] pub fn test_cc1if(&self) -> bool {
        self.cc1if() != 0
    }

    #[doc="Sets the CC1IF field."]
    #[inline] pub fn set_cc1if<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Update interrupt flag"]
    #[inline] pub fn uif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UIF != 0"]
    #[inline] pub fn test_uif(&self) -> bool {
        self.uif() != 0
    }

    #[doc="Sets the UIF field."]
    #[inline] pub fn set_uif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.cc4of() != 0 { try!(write!(f, " cc4of"))}
        if self.cc3of() != 0 { try!(write!(f, " cc3of"))}
        if self.cc2of() != 0 { try!(write!(f, " cc2of"))}
        if self.cc1of() != 0 { try!(write!(f, " cc1of"))}
        if self.tif() != 0 { try!(write!(f, " tif"))}
        if self.cc4if() != 0 { try!(write!(f, " cc4if"))}
        if self.cc3if() != 0 { try!(write!(f, " cc3if"))}
        if self.cc2if() != 0 { try!(write!(f, " cc2if"))}
        if self.cc1if() != 0 { try!(write!(f, " cc1if"))}
        if self.uif() != 0 { try!(write!(f, " uif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="event generation register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Egr(pub u32);
impl Egr {
    #[doc="Trigger generation"]
    #[inline] pub fn tg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TG != 0"]
    #[inline] pub fn test_tg(&self) -> bool {
        self.tg() != 0
    }

    #[doc="Sets the TG field."]
    #[inline] pub fn set_tg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Capture/compare 4 generation"]
    #[inline] pub fn cc4g(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CC4G != 0"]
    #[inline] pub fn test_cc4g(&self) -> bool {
        self.cc4g() != 0
    }

    #[doc="Sets the CC4G field."]
    #[inline] pub fn set_cc4g<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Capture/compare 3 generation"]
    #[inline] pub fn cc3g(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CC3G != 0"]
    #[inline] pub fn test_cc3g(&self) -> bool {
        self.cc3g() != 0
    }

    #[doc="Sets the CC3G field."]
    #[inline] pub fn set_cc3g<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Capture/compare 2 generation"]
    #[inline] pub fn cc2g(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CC2G != 0"]
    #[inline] pub fn test_cc2g(&self) -> bool {
        self.cc2g() != 0
    }

    #[doc="Sets the CC2G field."]
    #[inline] pub fn set_cc2g<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Capture/compare 1 generation"]
    #[inline] pub fn cc1g(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CC1G != 0"]
    #[inline] pub fn test_cc1g(&self) -> bool {
        self.cc1g() != 0
    }

    #[doc="Sets the CC1G field."]
    #[inline] pub fn set_cc1g<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Update generation"]
    #[inline] pub fn ug(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UG != 0"]
    #[inline] pub fn test_ug(&self) -> bool {
        self.ug() != 0
    }

    #[doc="Sets the UG field."]
    #[inline] pub fn set_ug<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Egr {
    #[inline]
    fn from(other: u32) -> Self {
         Egr(other)
    }
}

impl ::core::fmt::Display for Egr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Egr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tg() != 0 { try!(write!(f, " tg"))}
        if self.cc4g() != 0 { try!(write!(f, " cc4g"))}
        if self.cc3g() != 0 { try!(write!(f, " cc3g"))}
        if self.cc2g() != 0 { try!(write!(f, " cc2g"))}
        if self.cc1g() != 0 { try!(write!(f, " cc1g"))}
        if self.ug() != 0 { try!(write!(f, " ug"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare mode register 1 (output mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccmr1Output(pub u32);
impl Ccmr1Output {
    #[doc="Output Compare 2 mode - bit 3"]
    #[inline] pub fn oc2m_3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if OC2M_3 != 0"]
    #[inline] pub fn test_oc2m_3(&self) -> bool {
        self.oc2m_3() != 0
    }

    #[doc="Sets the OC2M_3 field."]
    #[inline] pub fn set_oc2m_3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Output Compare 1 mode - bit 3"]
    #[inline] pub fn oc1m_3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if OC1M_3 != 0"]
    #[inline] pub fn test_oc1m_3(&self) -> bool {
        self.oc1m_3() != 0
    }

    #[doc="Sets the OC1M_3 field."]
    #[inline] pub fn set_oc1m_3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Output compare 2 clear enable"]
    #[inline] pub fn oc2ce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OC2CE != 0"]
    #[inline] pub fn test_oc2ce(&self) -> bool {
        self.oc2ce() != 0
    }

    #[doc="Sets the OC2CE field."]
    #[inline] pub fn set_oc2ce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Output compare 2 mode"]
    #[inline] pub fn oc2m(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if OC2M != 0"]
    #[inline] pub fn test_oc2m(&self) -> bool {
        self.oc2m() != 0
    }

    #[doc="Sets the OC2M field."]
    #[inline] pub fn set_oc2m<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Output compare 2 preload enable"]
    #[inline] pub fn oc2pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if OC2PE != 0"]
    #[inline] pub fn test_oc2pe(&self) -> bool {
        self.oc2pe() != 0
    }

    #[doc="Sets the OC2PE field."]
    #[inline] pub fn set_oc2pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Output compare 2 fast enable"]
    #[inline] pub fn oc2fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OC2FE != 0"]
    #[inline] pub fn test_oc2fe(&self) -> bool {
        self.oc2fe() != 0
    }

    #[doc="Sets the OC2FE field."]
    #[inline] pub fn set_oc2fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Capture/Compare 2 selection"]
    #[inline] pub fn cc2s(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if CC2S != 0"]
    #[inline] pub fn test_cc2s(&self) -> bool {
        self.cc2s() != 0
    }

    #[doc="Sets the CC2S field."]
    #[inline] pub fn set_cc2s<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Output compare 1 clear enable"]
    #[inline] pub fn oc1ce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OC1CE != 0"]
    #[inline] pub fn test_oc1ce(&self) -> bool {
        self.oc1ce() != 0
    }

    #[doc="Sets the OC1CE field."]
    #[inline] pub fn set_oc1ce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Output compare 1 mode"]
    #[inline] pub fn oc1m(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if OC1M != 0"]
    #[inline] pub fn test_oc1m(&self) -> bool {
        self.oc1m() != 0
    }

    #[doc="Sets the OC1M field."]
    #[inline] pub fn set_oc1m<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Output compare 1 preload enable"]
    #[inline] pub fn oc1pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OC1PE != 0"]
    #[inline] pub fn test_oc1pe(&self) -> bool {
        self.oc1pe() != 0
    }

    #[doc="Sets the OC1PE field."]
    #[inline] pub fn set_oc1pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Output compare 1 fast enable"]
    #[inline] pub fn oc1fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OC1FE != 0"]
    #[inline] pub fn test_oc1fe(&self) -> bool {
        self.oc1fe() != 0
    }

    #[doc="Sets the OC1FE field."]
    #[inline] pub fn set_oc1fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Capture/Compare 1 selection"]
    #[inline] pub fn cc1s(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CC1S != 0"]
    #[inline] pub fn test_cc1s(&self) -> bool {
        self.cc1s() != 0
    }

    #[doc="Sets the CC1S field."]
    #[inline] pub fn set_cc1s<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccmr1Output {
    #[inline]
    fn from(other: u32) -> Self {
         Ccmr1Output(other)
    }
}

impl ::core::fmt::Display for Ccmr1Output {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccmr1Output {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oc2m_3() != 0 { try!(write!(f, " oc2m_3"))}
        if self.oc1m_3() != 0 { try!(write!(f, " oc1m_3"))}
        if self.oc2ce() != 0 { try!(write!(f, " oc2ce"))}
        if self.oc2m() != 0 { try!(write!(f, " oc2m=0x{:x}", self.oc2m()))}
        if self.oc2pe() != 0 { try!(write!(f, " oc2pe"))}
        if self.oc2fe() != 0 { try!(write!(f, " oc2fe"))}
        if self.cc2s() != 0 { try!(write!(f, " cc2s=0x{:x}", self.cc2s()))}
        if self.oc1ce() != 0 { try!(write!(f, " oc1ce"))}
        if self.oc1m() != 0 { try!(write!(f, " oc1m=0x{:x}", self.oc1m()))}
        if self.oc1pe() != 0 { try!(write!(f, " oc1pe"))}
        if self.oc1fe() != 0 { try!(write!(f, " oc1fe"))}
        if self.cc1s() != 0 { try!(write!(f, " cc1s=0x{:x}", self.cc1s()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare mode register 1 (input mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccmr1Input(pub u32);
impl Ccmr1Input {
    #[doc="Input capture 2 filter"]
    #[inline] pub fn ic2f(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if IC2F != 0"]
    #[inline] pub fn test_ic2f(&self) -> bool {
        self.ic2f() != 0
    }

    #[doc="Sets the IC2F field."]
    #[inline] pub fn set_ic2f<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Input capture 2 prescaler"]
    #[inline] pub fn ic2psc(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if IC2PSC != 0"]
    #[inline] pub fn test_ic2psc(&self) -> bool {
        self.ic2psc() != 0
    }

    #[doc="Sets the IC2PSC field."]
    #[inline] pub fn set_ic2psc<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Capture/compare 2 selection"]
    #[inline] pub fn cc2s(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if CC2S != 0"]
    #[inline] pub fn test_cc2s(&self) -> bool {
        self.cc2s() != 0
    }

    #[doc="Sets the CC2S field."]
    #[inline] pub fn set_cc2s<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Input capture 1 filter"]
    #[inline] pub fn ic1f(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if IC1F != 0"]
    #[inline] pub fn test_ic1f(&self) -> bool {
        self.ic1f() != 0
    }

    #[doc="Sets the IC1F field."]
    #[inline] pub fn set_ic1f<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Input capture 1 prescaler"]
    #[inline] pub fn ic1psc(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if IC1PSC != 0"]
    #[inline] pub fn test_ic1psc(&self) -> bool {
        self.ic1psc() != 0
    }

    #[doc="Sets the IC1PSC field."]
    #[inline] pub fn set_ic1psc<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Capture/Compare 1 selection"]
    #[inline] pub fn cc1s(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CC1S != 0"]
    #[inline] pub fn test_cc1s(&self) -> bool {
        self.cc1s() != 0
    }

    #[doc="Sets the CC1S field."]
    #[inline] pub fn set_cc1s<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccmr1Input {
    #[inline]
    fn from(other: u32) -> Self {
         Ccmr1Input(other)
    }
}

impl ::core::fmt::Display for Ccmr1Input {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccmr1Input {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ic2f() != 0 { try!(write!(f, " ic2f=0x{:x}", self.ic2f()))}
        if self.ic2psc() != 0 { try!(write!(f, " ic2psc=0x{:x}", self.ic2psc()))}
        if self.cc2s() != 0 { try!(write!(f, " cc2s=0x{:x}", self.cc2s()))}
        if self.ic1f() != 0 { try!(write!(f, " ic1f=0x{:x}", self.ic1f()))}
        if self.ic1psc() != 0 { try!(write!(f, " ic1psc=0x{:x}", self.ic1psc()))}
        if self.cc1s() != 0 { try!(write!(f, " cc1s=0x{:x}", self.cc1s()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare mode register 2 (output mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccmr2Output(pub u32);
impl Ccmr2Output {
    #[doc="Output Compare 4 mode - bit 3"]
    #[inline] pub fn oc4m_3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if OC4M_3 != 0"]
    #[inline] pub fn test_oc4m_3(&self) -> bool {
        self.oc4m_3() != 0
    }

    #[doc="Sets the OC4M_3 field."]
    #[inline] pub fn set_oc4m_3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Output Compare 3 mode - bit 3"]
    #[inline] pub fn oc3m_3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if OC3M_3 != 0"]
    #[inline] pub fn test_oc3m_3(&self) -> bool {
        self.oc3m_3() != 0
    }

    #[doc="Sets the OC3M_3 field."]
    #[inline] pub fn set_oc3m_3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Output compare 4 clear enable"]
    #[inline] pub fn oc4ce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OC4CE != 0"]
    #[inline] pub fn test_oc4ce(&self) -> bool {
        self.oc4ce() != 0
    }

    #[doc="Sets the OC4CE field."]
    #[inline] pub fn set_oc4ce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Output compare 4 mode"]
    #[inline] pub fn oc4m(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if OC4M != 0"]
    #[inline] pub fn test_oc4m(&self) -> bool {
        self.oc4m() != 0
    }

    #[doc="Sets the OC4M field."]
    #[inline] pub fn set_oc4m<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Output compare 4 preload enable"]
    #[inline] pub fn oc4pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if OC4PE != 0"]
    #[inline] pub fn test_oc4pe(&self) -> bool {
        self.oc4pe() != 0
    }

    #[doc="Sets the OC4PE field."]
    #[inline] pub fn set_oc4pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Output compare 4 fast enable"]
    #[inline] pub fn oc4fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OC4FE != 0"]
    #[inline] pub fn test_oc4fe(&self) -> bool {
        self.oc4fe() != 0
    }

    #[doc="Sets the OC4FE field."]
    #[inline] pub fn set_oc4fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Capture/Compare 4 selection"]
    #[inline] pub fn cc4s(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if CC4S != 0"]
    #[inline] pub fn test_cc4s(&self) -> bool {
        self.cc4s() != 0
    }

    #[doc="Sets the CC4S field."]
    #[inline] pub fn set_cc4s<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Output compare 3 clear enable"]
    #[inline] pub fn oc3ce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OC3CE != 0"]
    #[inline] pub fn test_oc3ce(&self) -> bool {
        self.oc3ce() != 0
    }

    #[doc="Sets the OC3CE field."]
    #[inline] pub fn set_oc3ce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Output compare 3 mode"]
    #[inline] pub fn oc3m(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if OC3M != 0"]
    #[inline] pub fn test_oc3m(&self) -> bool {
        self.oc3m() != 0
    }

    #[doc="Sets the OC3M field."]
    #[inline] pub fn set_oc3m<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Output compare 3 preload enable"]
    #[inline] pub fn oc3pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OC3PE != 0"]
    #[inline] pub fn test_oc3pe(&self) -> bool {
        self.oc3pe() != 0
    }

    #[doc="Sets the OC3PE field."]
    #[inline] pub fn set_oc3pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Output compare 3 fast enable"]
    #[inline] pub fn oc3fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OC3FE != 0"]
    #[inline] pub fn test_oc3fe(&self) -> bool {
        self.oc3fe() != 0
    }

    #[doc="Sets the OC3FE field."]
    #[inline] pub fn set_oc3fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Capture/Compare 3 selection"]
    #[inline] pub fn cc3s(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CC3S != 0"]
    #[inline] pub fn test_cc3s(&self) -> bool {
        self.cc3s() != 0
    }

    #[doc="Sets the CC3S field."]
    #[inline] pub fn set_cc3s<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccmr2Output {
    #[inline]
    fn from(other: u32) -> Self {
         Ccmr2Output(other)
    }
}

impl ::core::fmt::Display for Ccmr2Output {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccmr2Output {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oc4m_3() != 0 { try!(write!(f, " oc4m_3"))}
        if self.oc3m_3() != 0 { try!(write!(f, " oc3m_3"))}
        if self.oc4ce() != 0 { try!(write!(f, " oc4ce"))}
        if self.oc4m() != 0 { try!(write!(f, " oc4m=0x{:x}", self.oc4m()))}
        if self.oc4pe() != 0 { try!(write!(f, " oc4pe"))}
        if self.oc4fe() != 0 { try!(write!(f, " oc4fe"))}
        if self.cc4s() != 0 { try!(write!(f, " cc4s=0x{:x}", self.cc4s()))}
        if self.oc3ce() != 0 { try!(write!(f, " oc3ce"))}
        if self.oc3m() != 0 { try!(write!(f, " oc3m=0x{:x}", self.oc3m()))}
        if self.oc3pe() != 0 { try!(write!(f, " oc3pe"))}
        if self.oc3fe() != 0 { try!(write!(f, " oc3fe"))}
        if self.cc3s() != 0 { try!(write!(f, " cc3s=0x{:x}", self.cc3s()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare mode register 2 (input mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccmr2Input(pub u32);
impl Ccmr2Input {
    #[doc="Input capture 4 filter"]
    #[inline] pub fn ic4f(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if IC4F != 0"]
    #[inline] pub fn test_ic4f(&self) -> bool {
        self.ic4f() != 0
    }

    #[doc="Sets the IC4F field."]
    #[inline] pub fn set_ic4f<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Input capture 4 prescaler"]
    #[inline] pub fn ic4psc(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if IC4PSC != 0"]
    #[inline] pub fn test_ic4psc(&self) -> bool {
        self.ic4psc() != 0
    }

    #[doc="Sets the IC4PSC field."]
    #[inline] pub fn set_ic4psc<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Capture/Compare 4 selection"]
    #[inline] pub fn cc4s(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if CC4S != 0"]
    #[inline] pub fn test_cc4s(&self) -> bool {
        self.cc4s() != 0
    }

    #[doc="Sets the CC4S field."]
    #[inline] pub fn set_cc4s<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Input capture 3 filter"]
    #[inline] pub fn ic3f(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if IC3F != 0"]
    #[inline] pub fn test_ic3f(&self) -> bool {
        self.ic3f() != 0
    }

    #[doc="Sets the IC3F field."]
    #[inline] pub fn set_ic3f<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Input capture 3 prescaler"]
    #[inline] pub fn ic3psc(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if IC3PSC != 0"]
    #[inline] pub fn test_ic3psc(&self) -> bool {
        self.ic3psc() != 0
    }

    #[doc="Sets the IC3PSC field."]
    #[inline] pub fn set_ic3psc<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Capture/Compare 3 selection"]
    #[inline] pub fn cc3s(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CC3S != 0"]
    #[inline] pub fn test_cc3s(&self) -> bool {
        self.cc3s() != 0
    }

    #[doc="Sets the CC3S field."]
    #[inline] pub fn set_cc3s<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccmr2Input {
    #[inline]
    fn from(other: u32) -> Self {
         Ccmr2Input(other)
    }
}

impl ::core::fmt::Display for Ccmr2Input {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccmr2Input {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ic4f() != 0 { try!(write!(f, " ic4f=0x{:x}", self.ic4f()))}
        if self.ic4psc() != 0 { try!(write!(f, " ic4psc=0x{:x}", self.ic4psc()))}
        if self.cc4s() != 0 { try!(write!(f, " cc4s=0x{:x}", self.cc4s()))}
        if self.ic3f() != 0 { try!(write!(f, " ic3f=0x{:x}", self.ic3f()))}
        if self.ic3psc() != 0 { try!(write!(f, " ic3psc=0x{:x}", self.ic3psc()))}
        if self.cc3s() != 0 { try!(write!(f, " cc3s=0x{:x}", self.cc3s()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccer(pub u32);
impl Ccer {
    #[doc="Capture/Compare 4 output Polarity"]
    #[inline] pub fn cc4np(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CC4NP != 0"]
    #[inline] pub fn test_cc4np(&self) -> bool {
        self.cc4np() != 0
    }

    #[doc="Sets the CC4NP field."]
    #[inline] pub fn set_cc4np<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Capture/Compare 3 output Polarity"]
    #[inline] pub fn cc4p(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CC4P != 0"]
    #[inline] pub fn test_cc4p(&self) -> bool {
        self.cc4p() != 0
    }

    #[doc="Sets the CC4P field."]
    #[inline] pub fn set_cc4p<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Capture/Compare 4 output enable"]
    #[inline] pub fn cc4e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CC4E != 0"]
    #[inline] pub fn test_cc4e(&self) -> bool {
        self.cc4e() != 0
    }

    #[doc="Sets the CC4E field."]
    #[inline] pub fn set_cc4e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Capture/Compare 3 output Polarity"]
    #[inline] pub fn cc3np(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CC3NP != 0"]
    #[inline] pub fn test_cc3np(&self) -> bool {
        self.cc3np() != 0
    }

    #[doc="Sets the CC3NP field."]
    #[inline] pub fn set_cc3np<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Capture/Compare 3 output Polarity"]
    #[inline] pub fn cc3p(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CC3P != 0"]
    #[inline] pub fn test_cc3p(&self) -> bool {
        self.cc3p() != 0
    }

    #[doc="Sets the CC3P field."]
    #[inline] pub fn set_cc3p<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Capture/Compare 3 output enable"]
    #[inline] pub fn cc3e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CC3E != 0"]
    #[inline] pub fn test_cc3e(&self) -> bool {
        self.cc3e() != 0
    }

    #[doc="Sets the CC3E field."]
    #[inline] pub fn set_cc3e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Capture/Compare 2 output Polarity"]
    #[inline] pub fn cc2np(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CC2NP != 0"]
    #[inline] pub fn test_cc2np(&self) -> bool {
        self.cc2np() != 0
    }

    #[doc="Sets the CC2NP field."]
    #[inline] pub fn set_cc2np<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Capture/Compare 2 output Polarity"]
    #[inline] pub fn cc2p(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CC2P != 0"]
    #[inline] pub fn test_cc2p(&self) -> bool {
        self.cc2p() != 0
    }

    #[doc="Sets the CC2P field."]
    #[inline] pub fn set_cc2p<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Capture/Compare 2 output enable"]
    #[inline] pub fn cc2e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CC2E != 0"]
    #[inline] pub fn test_cc2e(&self) -> bool {
        self.cc2e() != 0
    }

    #[doc="Sets the CC2E field."]
    #[inline] pub fn set_cc2e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Capture/Compare 1 output Polarity"]
    #[inline] pub fn cc1np(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CC1NP != 0"]
    #[inline] pub fn test_cc1np(&self) -> bool {
        self.cc1np() != 0
    }

    #[doc="Sets the CC1NP field."]
    #[inline] pub fn set_cc1np<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Capture/Compare 1 output Polarity"]
    #[inline] pub fn cc1p(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CC1P != 0"]
    #[inline] pub fn test_cc1p(&self) -> bool {
        self.cc1p() != 0
    }

    #[doc="Sets the CC1P field."]
    #[inline] pub fn set_cc1p<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Capture/Compare 1 output enable"]
    #[inline] pub fn cc1e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CC1E != 0"]
    #[inline] pub fn test_cc1e(&self) -> bool {
        self.cc1e() != 0
    }

    #[doc="Sets the CC1E field."]
    #[inline] pub fn set_cc1e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccer {
    #[inline]
    fn from(other: u32) -> Self {
         Ccer(other)
    }
}

impl ::core::fmt::Display for Ccer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cc4np() != 0 { try!(write!(f, " cc4np"))}
        if self.cc4p() != 0 { try!(write!(f, " cc4p"))}
        if self.cc4e() != 0 { try!(write!(f, " cc4e"))}
        if self.cc3np() != 0 { try!(write!(f, " cc3np"))}
        if self.cc3p() != 0 { try!(write!(f, " cc3p"))}
        if self.cc3e() != 0 { try!(write!(f, " cc3e"))}
        if self.cc2np() != 0 { try!(write!(f, " cc2np"))}
        if self.cc2p() != 0 { try!(write!(f, " cc2p"))}
        if self.cc2e() != 0 { try!(write!(f, " cc2e"))}
        if self.cc1np() != 0 { try!(write!(f, " cc1np"))}
        if self.cc1p() != 0 { try!(write!(f, " cc1p"))}
        if self.cc1e() != 0 { try!(write!(f, " cc1e"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="High counter value (TIM2 only)"]
    #[inline] pub fn cnt_h(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7fff) as u16) } // [30:16]
    }

    #[doc="Returns true if CNT_H != 0"]
    #[inline] pub fn test_cnt_h(&self) -> bool {
        self.cnt_h() != 0
    }

    #[doc="Sets the CNT_H field."]
    #[inline] pub fn set_cnt_h<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Low counter value"]
    #[inline] pub fn cnt_l(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CNT_L != 0"]
    #[inline] pub fn test_cnt_l(&self) -> bool {
        self.cnt_l() != 0
    }

    #[doc="Sets the CNT_L field."]
    #[inline] pub fn set_cnt_l<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Value depends on IUFREMAP in TIM2_CR1."]
    #[inline] pub fn uifcpy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if UIFCPY != 0"]
    #[inline] pub fn test_uifcpy(&self) -> bool {
        self.uifcpy() != 0
    }

    #[doc="Sets the UIFCPY field."]
    #[inline] pub fn set_uifcpy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Cnt {
    #[inline]
    fn from(other: u32) -> Self {
         Cnt(other)
    }
}

impl ::core::fmt::Display for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt_h() != 0 { try!(write!(f, " cnt_h=0x{:x}", self.cnt_h()))}
        if self.cnt_l() != 0 { try!(write!(f, " cnt_l=0x{:x}", self.cnt_l()))}
        if self.uifcpy() != 0 { try!(write!(f, " uifcpy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="prescaler"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Psc(pub u32);
impl Psc {
    #[doc="Prescaler value"]
    #[inline] pub fn psc(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PSC != 0"]
    #[inline] pub fn test_psc(&self) -> bool {
        self.psc() != 0
    }

    #[doc="Sets the PSC field."]
    #[inline] pub fn set_psc<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Psc {
    #[inline]
    fn from(other: u32) -> Self {
         Psc(other)
    }
}

impl ::core::fmt::Display for Psc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Psc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.psc() != 0 { try!(write!(f, " psc=0x{:x}", self.psc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="auto-reload register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Arr(pub u32);
impl Arr {
    #[doc="High Auto-reload value (TIM2 only)"]
    #[inline] pub fn arr_h(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if ARR_H != 0"]
    #[inline] pub fn test_arr_h(&self) -> bool {
        self.arr_h() != 0
    }

    #[doc="Sets the ARR_H field."]
    #[inline] pub fn set_arr_h<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Low Auto-reload value"]
    #[inline] pub fn arr_l(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if ARR_L != 0"]
    #[inline] pub fn test_arr_l(&self) -> bool {
        self.arr_l() != 0
    }

    #[doc="Sets the ARR_L field."]
    #[inline] pub fn set_arr_l<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Arr {
    #[inline]
    fn from(other: u32) -> Self {
         Arr(other)
    }
}

impl ::core::fmt::Display for Arr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Arr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.arr_h() != 0 { try!(write!(f, " arr_h=0x{:x}", self.arr_h()))}
        if self.arr_l() != 0 { try!(write!(f, " arr_l=0x{:x}", self.arr_l()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc="High Capture/Compare 1 value (TIM2 only)"]
    #[inline] pub fn ccr1_h(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CCR1_H != 0"]
    #[inline] pub fn test_ccr1_h(&self) -> bool {
        self.ccr1_h() != 0
    }

    #[doc="Sets the CCR1_H field."]
    #[inline] pub fn set_ccr1_h<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Low Capture/Compare 1 value"]
    #[inline] pub fn ccr1_l(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CCR1_L != 0"]
    #[inline] pub fn test_ccr1_l(&self) -> bool {
        self.ccr1_l() != 0
    }

    #[doc="Sets the CCR1_L field."]
    #[inline] pub fn set_ccr1_l<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr1(other)
    }
}

impl ::core::fmt::Display for Ccr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccr1_h() != 0 { try!(write!(f, " ccr1_h=0x{:x}", self.ccr1_h()))}
        if self.ccr1_l() != 0 { try!(write!(f, " ccr1_l=0x{:x}", self.ccr1_l()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr2(pub u32);
impl Ccr2 {
    #[doc="High Capture/Compare 2 value (TIM2 only)"]
    #[inline] pub fn ccr2_h(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CCR2_H != 0"]
    #[inline] pub fn test_ccr2_h(&self) -> bool {
        self.ccr2_h() != 0
    }

    #[doc="Sets the CCR2_H field."]
    #[inline] pub fn set_ccr2_h<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Low Capture/Compare 2 value"]
    #[inline] pub fn ccr2_l(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CCR2_L != 0"]
    #[inline] pub fn test_ccr2_l(&self) -> bool {
        self.ccr2_l() != 0
    }

    #[doc="Sets the CCR2_L field."]
    #[inline] pub fn set_ccr2_l<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr2(other)
    }
}

impl ::core::fmt::Display for Ccr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccr2_h() != 0 { try!(write!(f, " ccr2_h=0x{:x}", self.ccr2_h()))}
        if self.ccr2_l() != 0 { try!(write!(f, " ccr2_l=0x{:x}", self.ccr2_l()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr3(pub u32);
impl Ccr3 {
    #[doc="High Capture/Compare value (TIM2 only)"]
    #[inline] pub fn ccr3_h(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CCR3_H != 0"]
    #[inline] pub fn test_ccr3_h(&self) -> bool {
        self.ccr3_h() != 0
    }

    #[doc="Sets the CCR3_H field."]
    #[inline] pub fn set_ccr3_h<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Low Capture/Compare value"]
    #[inline] pub fn ccr3_l(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CCR3_L != 0"]
    #[inline] pub fn test_ccr3_l(&self) -> bool {
        self.ccr3_l() != 0
    }

    #[doc="Sets the CCR3_L field."]
    #[inline] pub fn set_ccr3_l<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr3(other)
    }
}

impl ::core::fmt::Display for Ccr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccr3_h() != 0 { try!(write!(f, " ccr3_h=0x{:x}", self.ccr3_h()))}
        if self.ccr3_l() != 0 { try!(write!(f, " ccr3_l=0x{:x}", self.ccr3_l()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr4(pub u32);
impl Ccr4 {
    #[doc="High Capture/Compare value (TIM2 only)"]
    #[inline] pub fn ccr4_h(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CCR4_H != 0"]
    #[inline] pub fn test_ccr4_h(&self) -> bool {
        self.ccr4_h() != 0
    }

    #[doc="Sets the CCR4_H field."]
    #[inline] pub fn set_ccr4_h<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Low Capture/Compare value"]
    #[inline] pub fn ccr4_l(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CCR4_L != 0"]
    #[inline] pub fn test_ccr4_l(&self) -> bool {
        self.ccr4_l() != 0
    }

    #[doc="Sets the CCR4_L field."]
    #[inline] pub fn set_ccr4_l<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr4(other)
    }
}

impl ::core::fmt::Display for Ccr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccr4_h() != 0 { try!(write!(f, " ccr4_h=0x{:x}", self.ccr4_h()))}
        if self.ccr4_l() != 0 { try!(write!(f, " ccr4_l=0x{:x}", self.ccr4_l()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcr(pub u32);
impl Dcr {
    #[doc="DMA burst length"]
    #[inline] pub fn dbl(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if DBL != 0"]
    #[inline] pub fn test_dbl(&self) -> bool {
        self.dbl() != 0
    }

    #[doc="Sets the DBL field."]
    #[inline] pub fn set_dbl<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA base address"]
    #[inline] pub fn dba(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if DBA != 0"]
    #[inline] pub fn test_dba(&self) -> bool {
        self.dba() != 0
    }

    #[doc="Sets the DBA field."]
    #[inline] pub fn set_dba<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcr {
    #[inline]
    fn from(other: u32) -> Self {
         Dcr(other)
    }
}

impl ::core::fmt::Display for Dcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbl() != 0 { try!(write!(f, " dbl=0x{:x}", self.dbl()))}
        if self.dba() != 0 { try!(write!(f, " dba=0x{:x}", self.dba()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA address for full transfer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmar(pub u32);
impl Dmar {
    #[doc="DMA register for burst accesses"]
    #[inline] pub fn dmab(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DMAB != 0"]
    #[inline] pub fn test_dmab(&self) -> bool {
        self.dmab() != 0
    }

    #[doc="Sets the DMAB field."]
    #[inline] pub fn set_dmab<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dmar {
    #[inline]
    fn from(other: u32) -> Self {
         Dmar(other)
    }
}

impl ::core::fmt::Display for Dmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dmab() != 0 { try!(write!(f, " dmab=0x{:x}", self.dmab()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TIM2 option register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Or(pub u32);
impl Or {
    #[doc="Input capture 4 remap"]
    #[inline] pub fn ti4_rmp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if TI4_RMP != 0"]
    #[inline] pub fn test_ti4_rmp(&self) -> bool {
        self.ti4_rmp() != 0
    }

    #[doc="Sets the TI4_RMP field."]
    #[inline] pub fn set_ti4_rmp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="External trigger remap"]
    #[inline] pub fn etr_rmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ETR_RMP != 0"]
    #[inline] pub fn test_etr_rmp(&self) -> bool {
        self.etr_rmp() != 0
    }

    #[doc="Sets the ETR_RMP field."]
    #[inline] pub fn set_etr_rmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Internal trigger remap"]
    #[inline] pub fn itr_rmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ITR_RMP != 0"]
    #[inline] pub fn test_itr_rmp(&self) -> bool {
        self.itr_rmp() != 0
    }

    #[doc="Sets the ITR_RMP field."]
    #[inline] pub fn set_itr_rmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Or {
    #[inline]
    fn from(other: u32) -> Self {
         Or(other)
    }
}

impl ::core::fmt::Display for Or {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Or {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ti4_rmp() != 0 { try!(write!(f, " ti4_rmp=0x{:x}", self.ti4_rmp()))}
        if self.etr_rmp() != 0 { try!(write!(f, " etr_rmp"))}
        if self.itr_rmp() != 0 { try!(write!(f, " itr_rmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TIM2 alternate function option register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Af(pub u32);
impl Af {
    #[doc="External trigger source selection"]
    #[inline] pub fn etrsel(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x7) as u8) } // [16:14]
    }

    #[doc="Returns true if ETRSEL != 0"]
    #[inline] pub fn test_etrsel(&self) -> bool {
        self.etrsel() != 0
    }

    #[doc="Sets the ETRSEL field."]
    #[inline] pub fn set_etrsel<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Af {
    #[inline]
    fn from(other: u32) -> Self {
         Af(other)
    }
}

impl ::core::fmt::Display for Af {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Af {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.etrsel() != 0 { try!(write!(f, " etrsel=0x{:x}", self.etrsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

