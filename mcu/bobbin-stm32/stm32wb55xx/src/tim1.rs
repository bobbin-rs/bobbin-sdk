::bobbin_mcu::periph!( TIM1, Tim1, TIM1_PERIPH, Tim1Periph, TIM1_OWNED, TIM1_REF_COUNT, 0x40012c00, 0x00, 0x27);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TIM1 Peripheral"]
pub struct Tim1Periph(pub usize); 

impl Tim1Periph {
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

    #[doc="Get the RCR Register."]
    #[inline] pub fn rcr_reg(&self) -> ::bobbin_mcu::register::Register<Rcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rcr, 0x30)
    }

    #[doc="Get the *mut pointer for the RCR register."]
    #[inline] pub fn rcr_mut(&self) -> *mut Rcr { 
        self.rcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RCR register."]
    #[inline] pub fn rcr_ptr(&self) -> *const Rcr { 
        self.rcr_reg().ptr()
    }

    #[doc="Read the RCR register."]
    #[inline] pub fn rcr(&self) -> Rcr { 
        self.rcr_reg().read()
    }

    #[doc="Write the RCR register."]
    #[inline] pub fn write_rcr(&self, value: Rcr) -> &Self { 
        self.rcr_reg().write(value);
        self
    }

    #[doc="Set the RCR register."]
    #[inline] pub fn set_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &Self {
        self.rcr_reg().set(f);
        self
    }

    #[doc="Modify the RCR register."]
    #[inline] pub fn with_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &Self {
        self.rcr_reg().with(f);
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

    #[doc="Get the BDTR Register."]
    #[inline] pub fn bdtr_reg(&self) -> ::bobbin_mcu::register::Register<Bdtr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bdtr, 0x44)
    }

    #[doc="Get the *mut pointer for the BDTR register."]
    #[inline] pub fn bdtr_mut(&self) -> *mut Bdtr { 
        self.bdtr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BDTR register."]
    #[inline] pub fn bdtr_ptr(&self) -> *const Bdtr { 
        self.bdtr_reg().ptr()
    }

    #[doc="Read the BDTR register."]
    #[inline] pub fn bdtr(&self) -> Bdtr { 
        self.bdtr_reg().read()
    }

    #[doc="Write the BDTR register."]
    #[inline] pub fn write_bdtr(&self, value: Bdtr) -> &Self { 
        self.bdtr_reg().write(value);
        self
    }

    #[doc="Set the BDTR register."]
    #[inline] pub fn set_bdtr<F: FnOnce(Bdtr) -> Bdtr>(&self, f: F) -> &Self {
        self.bdtr_reg().set(f);
        self
    }

    #[doc="Modify the BDTR register."]
    #[inline] pub fn with_bdtr<F: FnOnce(Bdtr) -> Bdtr>(&self, f: F) -> &Self {
        self.bdtr_reg().with(f);
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

    #[doc="Get the CCMR3_OUTPUT Register."]
    #[inline] pub fn ccmr3_output_reg(&self) -> ::bobbin_mcu::register::Register<Ccmr3Output> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccmr3Output, 0x54)
    }

    #[doc="Get the *mut pointer for the CCMR3_OUTPUT register."]
    #[inline] pub fn ccmr3_output_mut(&self) -> *mut Ccmr3Output { 
        self.ccmr3_output_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCMR3_OUTPUT register."]
    #[inline] pub fn ccmr3_output_ptr(&self) -> *const Ccmr3Output { 
        self.ccmr3_output_reg().ptr()
    }

    #[doc="Read the CCMR3_OUTPUT register."]
    #[inline] pub fn ccmr3_output(&self) -> Ccmr3Output { 
        self.ccmr3_output_reg().read()
    }

    #[doc="Write the CCMR3_OUTPUT register."]
    #[inline] pub fn write_ccmr3_output(&self, value: Ccmr3Output) -> &Self { 
        self.ccmr3_output_reg().write(value);
        self
    }

    #[doc="Set the CCMR3_OUTPUT register."]
    #[inline] pub fn set_ccmr3_output<F: FnOnce(Ccmr3Output) -> Ccmr3Output>(&self, f: F) -> &Self {
        self.ccmr3_output_reg().set(f);
        self
    }

    #[doc="Modify the CCMR3_OUTPUT register."]
    #[inline] pub fn with_ccmr3_output<F: FnOnce(Ccmr3Output) -> Ccmr3Output>(&self, f: F) -> &Self {
        self.ccmr3_output_reg().with(f);
        self
    }

    #[doc="Get the CCR5 Register."]
    #[inline] pub fn ccr5_reg(&self) -> ::bobbin_mcu::register::Register<Ccr5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccr5, 0x58)
    }

    #[doc="Get the *mut pointer for the CCR5 register."]
    #[inline] pub fn ccr5_mut(&self) -> *mut Ccr5 { 
        self.ccr5_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCR5 register."]
    #[inline] pub fn ccr5_ptr(&self) -> *const Ccr5 { 
        self.ccr5_reg().ptr()
    }

    #[doc="Read the CCR5 register."]
    #[inline] pub fn ccr5(&self) -> Ccr5 { 
        self.ccr5_reg().read()
    }

    #[doc="Write the CCR5 register."]
    #[inline] pub fn write_ccr5(&self, value: Ccr5) -> &Self { 
        self.ccr5_reg().write(value);
        self
    }

    #[doc="Set the CCR5 register."]
    #[inline] pub fn set_ccr5<F: FnOnce(Ccr5) -> Ccr5>(&self, f: F) -> &Self {
        self.ccr5_reg().set(f);
        self
    }

    #[doc="Modify the CCR5 register."]
    #[inline] pub fn with_ccr5<F: FnOnce(Ccr5) -> Ccr5>(&self, f: F) -> &Self {
        self.ccr5_reg().with(f);
        self
    }

    #[doc="Get the CCR6 Register."]
    #[inline] pub fn ccr6_reg(&self) -> ::bobbin_mcu::register::Register<Ccr6> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccr6, 0x5c)
    }

    #[doc="Get the *mut pointer for the CCR6 register."]
    #[inline] pub fn ccr6_mut(&self) -> *mut Ccr6 { 
        self.ccr6_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCR6 register."]
    #[inline] pub fn ccr6_ptr(&self) -> *const Ccr6 { 
        self.ccr6_reg().ptr()
    }

    #[doc="Read the CCR6 register."]
    #[inline] pub fn ccr6(&self) -> Ccr6 { 
        self.ccr6_reg().read()
    }

    #[doc="Write the CCR6 register."]
    #[inline] pub fn write_ccr6(&self, value: Ccr6) -> &Self { 
        self.ccr6_reg().write(value);
        self
    }

    #[doc="Set the CCR6 register."]
    #[inline] pub fn set_ccr6<F: FnOnce(Ccr6) -> Ccr6>(&self, f: F) -> &Self {
        self.ccr6_reg().set(f);
        self
    }

    #[doc="Modify the CCR6 register."]
    #[inline] pub fn with_ccr6<F: FnOnce(Ccr6) -> Ccr6>(&self, f: F) -> &Self {
        self.ccr6_reg().with(f);
        self
    }

    #[doc="Get the AF1 Register."]
    #[inline] pub fn af1_reg(&self) -> ::bobbin_mcu::register::Register<Af1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Af1, 0x60)
    }

    #[doc="Get the *mut pointer for the AF1 register."]
    #[inline] pub fn af1_mut(&self) -> *mut Af1 { 
        self.af1_reg().ptr()
    }

    #[doc="Get the *const pointer for the AF1 register."]
    #[inline] pub fn af1_ptr(&self) -> *const Af1 { 
        self.af1_reg().ptr()
    }

    #[doc="Read the AF1 register."]
    #[inline] pub fn af1(&self) -> Af1 { 
        self.af1_reg().read()
    }

    #[doc="Write the AF1 register."]
    #[inline] pub fn write_af1(&self, value: Af1) -> &Self { 
        self.af1_reg().write(value);
        self
    }

    #[doc="Set the AF1 register."]
    #[inline] pub fn set_af1<F: FnOnce(Af1) -> Af1>(&self, f: F) -> &Self {
        self.af1_reg().set(f);
        self
    }

    #[doc="Modify the AF1 register."]
    #[inline] pub fn with_af1<F: FnOnce(Af1) -> Af1>(&self, f: F) -> &Self {
        self.af1_reg().with(f);
        self
    }

    #[doc="Get the AF2 Register."]
    #[inline] pub fn af2_reg(&self) -> ::bobbin_mcu::register::Register<Af2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Af2, 0x64)
    }

    #[doc="Get the *mut pointer for the AF2 register."]
    #[inline] pub fn af2_mut(&self) -> *mut Af2 { 
        self.af2_reg().ptr()
    }

    #[doc="Get the *const pointer for the AF2 register."]
    #[inline] pub fn af2_ptr(&self) -> *const Af2 { 
        self.af2_reg().ptr()
    }

    #[doc="Read the AF2 register."]
    #[inline] pub fn af2(&self) -> Af2 { 
        self.af2_reg().read()
    }

    #[doc="Write the AF2 register."]
    #[inline] pub fn write_af2(&self, value: Af2) -> &Self { 
        self.af2_reg().write(value);
        self
    }

    #[doc="Set the AF2 register."]
    #[inline] pub fn set_af2<F: FnOnce(Af2) -> Af2>(&self, f: F) -> &Self {
        self.af2_reg().set(f);
        self
    }

    #[doc="Modify the AF2 register."]
    #[inline] pub fn with_af2<F: FnOnce(Af2) -> Af2>(&self, f: F) -> &Self {
        self.af2_reg().with(f);
        self
    }

}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
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
        if self.cen() != 0 { try!(write!(f, " cen"))}
        if self.opm() != 0 { try!(write!(f, " opm"))}
        if self.udis() != 0 { try!(write!(f, " udis"))}
        if self.urs() != 0 { try!(write!(f, " urs"))}
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.cms() != 0 { try!(write!(f, " cms=0x{:x}", self.cms()))}
        if self.arpe() != 0 { try!(write!(f, " arpe"))}
        if self.ckd() != 0 { try!(write!(f, " ckd=0x{:x}", self.ckd()))}
        if self.uifremap() != 0 { try!(write!(f, " uifremap"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="Master mode selection 2"]
    #[inline] pub fn mms2(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if MMS2 != 0"]
    #[inline] pub fn test_mms2(&self) -> bool {
        self.mms2() != 0
    }

    #[doc="Sets the MMS2 field."]
    #[inline] pub fn set_mms2<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Output Idle state 6 (OC6 output)"]
    #[inline] pub fn ois6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if OIS6 != 0"]
    #[inline] pub fn test_ois6(&self) -> bool {
        self.ois6() != 0
    }

    #[doc="Sets the OIS6 field."]
    #[inline] pub fn set_ois6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Output Idle state 5 (OC5 output)"]
    #[inline] pub fn ois5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if OIS5 != 0"]
    #[inline] pub fn test_ois5(&self) -> bool {
        self.ois5() != 0
    }

    #[doc="Sets the OIS5 field."]
    #[inline] pub fn set_ois5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Output Idle state 4"]
    #[inline] pub fn ois4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if OIS4 != 0"]
    #[inline] pub fn test_ois4(&self) -> bool {
        self.ois4() != 0
    }

    #[doc="Sets the OIS4 field."]
    #[inline] pub fn set_ois4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Output Idle state 3"]
    #[inline] pub fn ois3n(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if OIS3N != 0"]
    #[inline] pub fn test_ois3n(&self) -> bool {
        self.ois3n() != 0
    }

    #[doc="Sets the OIS3N field."]
    #[inline] pub fn set_ois3n<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Output Idle state 3"]
    #[inline] pub fn ois3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if OIS3 != 0"]
    #[inline] pub fn test_ois3(&self) -> bool {
        self.ois3() != 0
    }

    #[doc="Sets the OIS3 field."]
    #[inline] pub fn set_ois3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Output Idle state 2"]
    #[inline] pub fn ois2n(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if OIS2N != 0"]
    #[inline] pub fn test_ois2n(&self) -> bool {
        self.ois2n() != 0
    }

    #[doc="Sets the OIS2N field."]
    #[inline] pub fn set_ois2n<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Output Idle state 2"]
    #[inline] pub fn ois2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OIS2 != 0"]
    #[inline] pub fn test_ois2(&self) -> bool {
        self.ois2() != 0
    }

    #[doc="Sets the OIS2 field."]
    #[inline] pub fn set_ois2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Output Idle state 1"]
    #[inline] pub fn ois1n(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if OIS1N != 0"]
    #[inline] pub fn test_ois1n(&self) -> bool {
        self.ois1n() != 0
    }

    #[doc="Sets the OIS1N field."]
    #[inline] pub fn set_ois1n<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Output Idle state 1"]
    #[inline] pub fn ois1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OIS1 != 0"]
    #[inline] pub fn test_ois1(&self) -> bool {
        self.ois1() != 0
    }

    #[doc="Sets the OIS1 field."]
    #[inline] pub fn set_ois1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

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

    #[doc="Capture/compare control update selection"]
    #[inline] pub fn ccus(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CCUS != 0"]
    #[inline] pub fn test_ccus(&self) -> bool {
        self.ccus() != 0
    }

    #[doc="Sets the CCUS field."]
    #[inline] pub fn set_ccus<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Capture/compare preloaded control"]
    #[inline] pub fn ccpc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CCPC != 0"]
    #[inline] pub fn test_ccpc(&self) -> bool {
        self.ccpc() != 0
    }

    #[doc="Sets the CCPC field."]
    #[inline] pub fn set_ccpc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.mms2() != 0 { try!(write!(f, " mms2=0x{:x}", self.mms2()))}
        if self.ois6() != 0 { try!(write!(f, " ois6"))}
        if self.ois5() != 0 { try!(write!(f, " ois5"))}
        if self.ois4() != 0 { try!(write!(f, " ois4"))}
        if self.ois3n() != 0 { try!(write!(f, " ois3n"))}
        if self.ois3() != 0 { try!(write!(f, " ois3"))}
        if self.ois2n() != 0 { try!(write!(f, " ois2n"))}
        if self.ois2() != 0 { try!(write!(f, " ois2"))}
        if self.ois1n() != 0 { try!(write!(f, " ois1n"))}
        if self.ois1() != 0 { try!(write!(f, " ois1"))}
        if self.ti1s() != 0 { try!(write!(f, " ti1s"))}
        if self.mms() != 0 { try!(write!(f, " mms=0x{:x}", self.mms()))}
        if self.ccds() != 0 { try!(write!(f, " ccds"))}
        if self.ccus() != 0 { try!(write!(f, " ccus"))}
        if self.ccpc() != 0 { try!(write!(f, " ccpc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="slave mode control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Smcr(pub u32);
impl Smcr {
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
        if self.sms() != 0 { try!(write!(f, " sms=0x{:x}", self.sms()))}
        if self.occs() != 0 { try!(write!(f, " occs"))}
        if self.ts() != 0 { try!(write!(f, " ts=0x{:x}", self.ts()))}
        if self.msm() != 0 { try!(write!(f, " msm"))}
        if self.etf() != 0 { try!(write!(f, " etf=0x{:x}", self.etf()))}
        if self.etps() != 0 { try!(write!(f, " etps=0x{:x}", self.etps()))}
        if self.ece() != 0 { try!(write!(f, " ece"))}
        if self.etp() != 0 { try!(write!(f, " etp"))}
        if self.sms_3() != 0 { try!(write!(f, " sms_3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA/Interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dier(pub u32);
impl Dier {
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

    #[doc="COM interrupt enable"]
    #[inline] pub fn comie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMIE != 0"]
    #[inline] pub fn test_comie(&self) -> bool {
        self.comie() != 0
    }

    #[doc="Sets the COMIE field."]
    #[inline] pub fn set_comie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
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

    #[doc="Break interrupt enable"]
    #[inline] pub fn bie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BIE != 0"]
    #[inline] pub fn test_bie(&self) -> bool {
        self.bie() != 0
    }

    #[doc="Sets the BIE field."]
    #[inline] pub fn set_bie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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

    #[doc="COM DMA request enable"]
    #[inline] pub fn comde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if COMDE != 0"]
    #[inline] pub fn test_comde(&self) -> bool {
        self.comde() != 0
    }

    #[doc="Sets the COMDE field."]
    #[inline] pub fn set_comde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Trigger DMA request enable"]
    #[inline] pub fn tde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TDE != 0"]
    #[inline] pub fn test_tde(&self) -> bool {
        self.tde() != 0
    }

    #[doc="Sets the TDE field."]
    #[inline] pub fn set_tde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
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
        if self.uie() != 0 { try!(write!(f, " uie"))}
        if self.cc1ie() != 0 { try!(write!(f, " cc1ie"))}
        if self.cc2ie() != 0 { try!(write!(f, " cc2ie"))}
        if self.cc3ie() != 0 { try!(write!(f, " cc3ie"))}
        if self.cc4ie() != 0 { try!(write!(f, " cc4ie"))}
        if self.comie() != 0 { try!(write!(f, " comie"))}
        if self.tie() != 0 { try!(write!(f, " tie"))}
        if self.bie() != 0 { try!(write!(f, " bie"))}
        if self.ude() != 0 { try!(write!(f, " ude"))}
        if self.cc1de() != 0 { try!(write!(f, " cc1de"))}
        if self.cc2de() != 0 { try!(write!(f, " cc2de"))}
        if self.cc3de() != 0 { try!(write!(f, " cc3de"))}
        if self.cc4de() != 0 { try!(write!(f, " cc4de"))}
        if self.comde() != 0 { try!(write!(f, " comde"))}
        if self.tde() != 0 { try!(write!(f, " tde"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
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

    #[doc="COM interrupt flag"]
    #[inline] pub fn comif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMIF != 0"]
    #[inline] pub fn test_comif(&self) -> bool {
        self.comif() != 0
    }

    #[doc="Sets the COMIF field."]
    #[inline] pub fn set_comif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
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

    #[doc="Break interrupt flag"]
    #[inline] pub fn bif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BIF != 0"]
    #[inline] pub fn test_bif(&self) -> bool {
        self.bif() != 0
    }

    #[doc="Sets the BIF field."]
    #[inline] pub fn set_bif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Break 2 interrupt flag"]
    #[inline] pub fn b2if(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if B2IF != 0"]
    #[inline] pub fn test_b2if(&self) -> bool {
        self.b2if() != 0
    }

    #[doc="Sets the B2IF field."]
    #[inline] pub fn set_b2if<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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

    #[doc="System Break interrupt flag"]
    #[inline] pub fn sbif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SBIF != 0"]
    #[inline] pub fn test_sbif(&self) -> bool {
        self.sbif() != 0
    }

    #[doc="Sets the SBIF field."]
    #[inline] pub fn set_sbif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Compare 5 interrupt flag"]
    #[inline] pub fn cc5if(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CC5IF != 0"]
    #[inline] pub fn test_cc5if(&self) -> bool {
        self.cc5if() != 0
    }

    #[doc="Sets the CC5IF field."]
    #[inline] pub fn set_cc5if<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Compare 6 interrupt flag"]
    #[inline] pub fn cc6if(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CC6IF != 0"]
    #[inline] pub fn test_cc6if(&self) -> bool {
        self.cc6if() != 0
    }

    #[doc="Sets the CC6IF field."]
    #[inline] pub fn set_cc6if<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
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
        if self.uif() != 0 { try!(write!(f, " uif"))}
        if self.cc1if() != 0 { try!(write!(f, " cc1if"))}
        if self.cc2if() != 0 { try!(write!(f, " cc2if"))}
        if self.cc3if() != 0 { try!(write!(f, " cc3if"))}
        if self.cc4if() != 0 { try!(write!(f, " cc4if"))}
        if self.comif() != 0 { try!(write!(f, " comif"))}
        if self.tif() != 0 { try!(write!(f, " tif"))}
        if self.bif() != 0 { try!(write!(f, " bif"))}
        if self.b2if() != 0 { try!(write!(f, " b2if"))}
        if self.cc1of() != 0 { try!(write!(f, " cc1of"))}
        if self.cc2of() != 0 { try!(write!(f, " cc2of"))}
        if self.cc3of() != 0 { try!(write!(f, " cc3of"))}
        if self.cc4of() != 0 { try!(write!(f, " cc4of"))}
        if self.sbif() != 0 { try!(write!(f, " sbif"))}
        if self.cc5if() != 0 { try!(write!(f, " cc5if"))}
        if self.cc6if() != 0 { try!(write!(f, " cc6if"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="event generation register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Egr(pub u32);
impl Egr {
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

    #[doc="Capture/Compare control update generation"]
    #[inline] pub fn comg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMG != 0"]
    #[inline] pub fn test_comg(&self) -> bool {
        self.comg() != 0
    }

    #[doc="Sets the COMG field."]
    #[inline] pub fn set_comg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

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

    #[doc="Break generation"]
    #[inline] pub fn bg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BG != 0"]
    #[inline] pub fn test_bg(&self) -> bool {
        self.bg() != 0
    }

    #[doc="Sets the BG field."]
    #[inline] pub fn set_bg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Break 2 generation"]
    #[inline] pub fn b2g(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if B2G != 0"]
    #[inline] pub fn test_b2g(&self) -> bool {
        self.b2g() != 0
    }

    #[doc="Sets the B2G field."]
    #[inline] pub fn set_b2g<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
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
        if self.ug() != 0 { try!(write!(f, " ug"))}
        if self.cc1g() != 0 { try!(write!(f, " cc1g"))}
        if self.cc2g() != 0 { try!(write!(f, " cc2g"))}
        if self.cc3g() != 0 { try!(write!(f, " cc3g"))}
        if self.cc4g() != 0 { try!(write!(f, " cc4g"))}
        if self.comg() != 0 { try!(write!(f, " comg"))}
        if self.tg() != 0 { try!(write!(f, " tg"))}
        if self.bg() != 0 { try!(write!(f, " bg"))}
        if self.b2g() != 0 { try!(write!(f, " b2g"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare mode register 1 (output mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccmr1Input(pub u32);
impl Ccmr1Input {
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

    #[doc="Input capture 1 filter"]
    #[inline] pub fn c1f(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if C1F != 0"]
    #[inline] pub fn test_c1f(&self) -> bool {
        self.c1f() != 0
    }

    #[doc="Sets the C1F field."]
    #[inline] pub fn set_c1f<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="capture/Compare 2 selection"]
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
        if self.cc1s() != 0 { try!(write!(f, " cc1s=0x{:x}", self.cc1s()))}
        if self.ic1psc() != 0 { try!(write!(f, " ic1psc=0x{:x}", self.ic1psc()))}
        if self.c1f() != 0 { try!(write!(f, " c1f=0x{:x}", self.c1f()))}
        if self.cc2s() != 0 { try!(write!(f, " cc2s=0x{:x}", self.cc2s()))}
        if self.ic2psc() != 0 { try!(write!(f, " ic2psc=0x{:x}", self.ic2psc()))}
        if self.ic2f() != 0 { try!(write!(f, " ic2f=0x{:x}", self.ic2f()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare mode register 1 (output mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccmr1Output(pub u32);
impl Ccmr1Output {
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

    #[doc="Output Compare 1 fast enable"]
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

    #[doc="Output Compare 1 preload enable"]
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

    #[doc="Output Compare 1 mode"]
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

    #[doc="Output Compare 1 clear enable"]
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

    #[doc="Output Compare 2 fast enable"]
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

    #[doc="Output Compare 2 preload enable"]
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

    #[doc="Output Compare 2 mode"]
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

    #[doc="Output Compare 2 clear enable"]
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
        if self.cc1s() != 0 { try!(write!(f, " cc1s=0x{:x}", self.cc1s()))}
        if self.oc1fe() != 0 { try!(write!(f, " oc1fe"))}
        if self.oc1pe() != 0 { try!(write!(f, " oc1pe"))}
        if self.oc1m() != 0 { try!(write!(f, " oc1m=0x{:x}", self.oc1m()))}
        if self.oc1ce() != 0 { try!(write!(f, " oc1ce"))}
        if self.cc2s() != 0 { try!(write!(f, " cc2s=0x{:x}", self.cc2s()))}
        if self.oc2fe() != 0 { try!(write!(f, " oc2fe"))}
        if self.oc2pe() != 0 { try!(write!(f, " oc2pe"))}
        if self.oc2m() != 0 { try!(write!(f, " oc2m=0x{:x}", self.oc2m()))}
        if self.oc2ce() != 0 { try!(write!(f, " oc2ce"))}
        if self.oc1m_3() != 0 { try!(write!(f, " oc1m_3"))}
        if self.oc2m_3() != 0 { try!(write!(f, " oc2m_3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare mode register 2 (output mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccmr2Output(pub u32);
impl Ccmr2Output {
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
        if self.cc3s() != 0 { try!(write!(f, " cc3s=0x{:x}", self.cc3s()))}
        if self.oc3fe() != 0 { try!(write!(f, " oc3fe"))}
        if self.oc3pe() != 0 { try!(write!(f, " oc3pe"))}
        if self.oc3m() != 0 { try!(write!(f, " oc3m=0x{:x}", self.oc3m()))}
        if self.oc3ce() != 0 { try!(write!(f, " oc3ce"))}
        if self.cc4s() != 0 { try!(write!(f, " cc4s=0x{:x}", self.cc4s()))}
        if self.oc4fe() != 0 { try!(write!(f, " oc4fe"))}
        if self.oc4pe() != 0 { try!(write!(f, " oc4pe"))}
        if self.oc4m() != 0 { try!(write!(f, " oc4m=0x{:x}", self.oc4m()))}
        if self.oc4ce() != 0 { try!(write!(f, " oc4ce"))}
        if self.oc3m_3() != 0 { try!(write!(f, " oc3m_3"))}
        if self.oc4m_3() != 0 { try!(write!(f, " oc4m_3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare mode register 2 (output mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccmr2Input(pub u32);
impl Ccmr2Input {
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

    #[doc="Input capture 3 prescaler"]
    #[inline] pub fn c3psc(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if C3PSC != 0"]
    #[inline] pub fn test_c3psc(&self) -> bool {
        self.c3psc() != 0
    }

    #[doc="Sets the C3PSC field."]
    #[inline] pub fn set_c3psc<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
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
        if self.cc3s() != 0 { try!(write!(f, " cc3s=0x{:x}", self.cc3s()))}
        if self.c3psc() != 0 { try!(write!(f, " c3psc=0x{:x}", self.c3psc()))}
        if self.ic3f() != 0 { try!(write!(f, " ic3f=0x{:x}", self.ic3f()))}
        if self.cc4s() != 0 { try!(write!(f, " cc4s=0x{:x}", self.cc4s()))}
        if self.ic4psc() != 0 { try!(write!(f, " ic4psc=0x{:x}", self.ic4psc()))}
        if self.ic4f() != 0 { try!(write!(f, " ic4f=0x{:x}", self.ic4f()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccer(pub u32);
impl Ccer {
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

    #[doc="Capture/Compare 1 complementary output enable"]
    #[inline] pub fn cc1ne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CC1NE != 0"]
    #[inline] pub fn test_cc1ne(&self) -> bool {
        self.cc1ne() != 0
    }

    #[doc="Sets the CC1NE field."]
    #[inline] pub fn set_cc1ne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
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

    #[doc="Capture/Compare 2 complementary output enable"]
    #[inline] pub fn cc2ne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CC2NE != 0"]
    #[inline] pub fn test_cc2ne(&self) -> bool {
        self.cc2ne() != 0
    }

    #[doc="Sets the CC2NE field."]
    #[inline] pub fn set_cc2ne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
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

    #[doc="Capture/Compare 3 complementary output enable"]
    #[inline] pub fn cc3ne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CC3NE != 0"]
    #[inline] pub fn test_cc3ne(&self) -> bool {
        self.cc3ne() != 0
    }

    #[doc="Sets the CC3NE field."]
    #[inline] pub fn set_cc3ne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
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

    #[doc="Capture/Compare 4 complementary output polarity"]
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

    #[doc="Capture/Compare 5 output enable"]
    #[inline] pub fn cc5e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CC5E != 0"]
    #[inline] pub fn test_cc5e(&self) -> bool {
        self.cc5e() != 0
    }

    #[doc="Sets the CC5E field."]
    #[inline] pub fn set_cc5e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Capture/Compare 5 output polarity"]
    #[inline] pub fn cc5p(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CC5P != 0"]
    #[inline] pub fn test_cc5p(&self) -> bool {
        self.cc5p() != 0
    }

    #[doc="Sets the CC5P field."]
    #[inline] pub fn set_cc5p<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Capture/Compare 6 output enable"]
    #[inline] pub fn cc6e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CC6E != 0"]
    #[inline] pub fn test_cc6e(&self) -> bool {
        self.cc6e() != 0
    }

    #[doc="Sets the CC6E field."]
    #[inline] pub fn set_cc6e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Capture/Compare 6 output polarity"]
    #[inline] pub fn cc6p(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if CC6P != 0"]
    #[inline] pub fn test_cc6p(&self) -> bool {
        self.cc6p() != 0
    }

    #[doc="Sets the CC6P field."]
    #[inline] pub fn set_cc6p<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
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
        if self.cc1e() != 0 { try!(write!(f, " cc1e"))}
        if self.cc1p() != 0 { try!(write!(f, " cc1p"))}
        if self.cc1ne() != 0 { try!(write!(f, " cc1ne"))}
        if self.cc1np() != 0 { try!(write!(f, " cc1np"))}
        if self.cc2e() != 0 { try!(write!(f, " cc2e"))}
        if self.cc2p() != 0 { try!(write!(f, " cc2p"))}
        if self.cc2ne() != 0 { try!(write!(f, " cc2ne"))}
        if self.cc2np() != 0 { try!(write!(f, " cc2np"))}
        if self.cc3e() != 0 { try!(write!(f, " cc3e"))}
        if self.cc3p() != 0 { try!(write!(f, " cc3p"))}
        if self.cc3ne() != 0 { try!(write!(f, " cc3ne"))}
        if self.cc3np() != 0 { try!(write!(f, " cc3np"))}
        if self.cc4e() != 0 { try!(write!(f, " cc4e"))}
        if self.cc4p() != 0 { try!(write!(f, " cc4p"))}
        if self.cc4np() != 0 { try!(write!(f, " cc4np"))}
        if self.cc5e() != 0 { try!(write!(f, " cc5e"))}
        if self.cc5p() != 0 { try!(write!(f, " cc5p"))}
        if self.cc6e() != 0 { try!(write!(f, " cc6e"))}
        if self.cc6p() != 0 { try!(write!(f, " cc6p"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="counter value"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UIF copy"]
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
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
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
    #[doc="Auto-reload value"]
    #[inline] pub fn arr(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if ARR != 0"]
    #[inline] pub fn test_arr(&self) -> bool {
        self.arr() != 0
    }

    #[doc="Sets the ARR field."]
    #[inline] pub fn set_arr<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
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
        if self.arr() != 0 { try!(write!(f, " arr=0x{:x}", self.arr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="repetition counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc="Repetition counter value"]
    #[inline] pub fn rep(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if REP != 0"]
    #[inline] pub fn test_rep(&self) -> bool {
        self.rep() != 0
    }

    #[doc="Sets the REP field."]
    #[inline] pub fn set_rep<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rcr {
    #[inline]
    fn from(other: u32) -> Self {
         Rcr(other)
    }
}

impl ::core::fmt::Display for Rcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rep() != 0 { try!(write!(f, " rep=0x{:x}", self.rep()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc="Capture/Compare 1 value"]
    #[inline] pub fn ccr1(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CCR1 != 0"]
    #[inline] pub fn test_ccr1(&self) -> bool {
        self.ccr1() != 0
    }

    #[doc="Sets the CCR1 field."]
    #[inline] pub fn set_ccr1<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
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
        if self.ccr1() != 0 { try!(write!(f, " ccr1=0x{:x}", self.ccr1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr2(pub u32);
impl Ccr2 {
    #[doc="Capture/Compare 2 value"]
    #[inline] pub fn ccr2(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CCR2 != 0"]
    #[inline] pub fn test_ccr2(&self) -> bool {
        self.ccr2() != 0
    }

    #[doc="Sets the CCR2 field."]
    #[inline] pub fn set_ccr2<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
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
        if self.ccr2() != 0 { try!(write!(f, " ccr2=0x{:x}", self.ccr2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr3(pub u32);
impl Ccr3 {
    #[doc="Capture/Compare value"]
    #[inline] pub fn ccr3(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CCR3 != 0"]
    #[inline] pub fn test_ccr3(&self) -> bool {
        self.ccr3() != 0
    }

    #[doc="Sets the CCR3 field."]
    #[inline] pub fn set_ccr3<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
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
        if self.ccr3() != 0 { try!(write!(f, " ccr3=0x{:x}", self.ccr3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr4(pub u32);
impl Ccr4 {
    #[doc="Capture/Compare value"]
    #[inline] pub fn ccr4(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CCR4 != 0"]
    #[inline] pub fn test_ccr4(&self) -> bool {
        self.ccr4() != 0
    }

    #[doc="Sets the CCR4 field."]
    #[inline] pub fn set_ccr4<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
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
        if self.ccr4() != 0 { try!(write!(f, " ccr4=0x{:x}", self.ccr4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="break and dead-time register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdtr(pub u32);
impl Bdtr {
    #[doc="Dead-time generator setup"]
    #[inline] pub fn dtg(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DTG != 0"]
    #[inline] pub fn test_dtg(&self) -> bool {
        self.dtg() != 0
    }

    #[doc="Sets the DTG field."]
    #[inline] pub fn set_dtg<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Lock configuration"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Off-state selection for Idle mode"]
    #[inline] pub fn ossi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OSSI != 0"]
    #[inline] pub fn test_ossi(&self) -> bool {
        self.ossi() != 0
    }

    #[doc="Sets the OSSI field."]
    #[inline] pub fn set_ossi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Off-state selection for Run mode"]
    #[inline] pub fn ossr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if OSSR != 0"]
    #[inline] pub fn test_ossr(&self) -> bool {
        self.ossr() != 0
    }

    #[doc="Sets the OSSR field."]
    #[inline] pub fn set_ossr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Break enable"]
    #[inline] pub fn bke(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if BKE != 0"]
    #[inline] pub fn test_bke(&self) -> bool {
        self.bke() != 0
    }

    #[doc="Sets the BKE field."]
    #[inline] pub fn set_bke<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Break polarity"]
    #[inline] pub fn bkp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if BKP != 0"]
    #[inline] pub fn test_bkp(&self) -> bool {
        self.bkp() != 0
    }

    #[doc="Sets the BKP field."]
    #[inline] pub fn set_bkp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Automatic output enable"]
    #[inline] pub fn aoe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if AOE != 0"]
    #[inline] pub fn test_aoe(&self) -> bool {
        self.aoe() != 0
    }

    #[doc="Sets the AOE field."]
    #[inline] pub fn set_aoe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Main output enable"]
    #[inline] pub fn moe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if MOE != 0"]
    #[inline] pub fn test_moe(&self) -> bool {
        self.moe() != 0
    }

    #[doc="Sets the MOE field."]
    #[inline] pub fn set_moe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Break filter"]
    #[inline] pub fn bkf(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if BKF != 0"]
    #[inline] pub fn test_bkf(&self) -> bool {
        self.bkf() != 0
    }

    #[doc="Sets the BKF field."]
    #[inline] pub fn set_bkf<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Break 2 filter"]
    #[inline] pub fn bk2f(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if BK2F != 0"]
    #[inline] pub fn test_bk2f(&self) -> bool {
        self.bk2f() != 0
    }

    #[doc="Sets the BK2F field."]
    #[inline] pub fn set_bk2f<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Break 2 enable"]
    #[inline] pub fn bk2e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if BK2E != 0"]
    #[inline] pub fn test_bk2e(&self) -> bool {
        self.bk2e() != 0
    }

    #[doc="Sets the BK2E field."]
    #[inline] pub fn set_bk2e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Break 2 polarity"]
    #[inline] pub fn bk2p(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if BK2P != 0"]
    #[inline] pub fn test_bk2p(&self) -> bool {
        self.bk2p() != 0
    }

    #[doc="Sets the BK2P field."]
    #[inline] pub fn set_bk2p<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

}

impl From<u32> for Bdtr {
    #[inline]
    fn from(other: u32) -> Self {
         Bdtr(other)
    }
}

impl ::core::fmt::Display for Bdtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dtg() != 0 { try!(write!(f, " dtg=0x{:x}", self.dtg()))}
        if self.lock() != 0 { try!(write!(f, " lock=0x{:x}", self.lock()))}
        if self.ossi() != 0 { try!(write!(f, " ossi"))}
        if self.ossr() != 0 { try!(write!(f, " ossr"))}
        if self.bke() != 0 { try!(write!(f, " bke"))}
        if self.bkp() != 0 { try!(write!(f, " bkp"))}
        if self.aoe() != 0 { try!(write!(f, " aoe"))}
        if self.moe() != 0 { try!(write!(f, " moe"))}
        if self.bkf() != 0 { try!(write!(f, " bkf=0x{:x}", self.bkf()))}
        if self.bk2f() != 0 { try!(write!(f, " bk2f=0x{:x}", self.bk2f()))}
        if self.bk2e() != 0 { try!(write!(f, " bk2e"))}
        if self.bk2p() != 0 { try!(write!(f, " bk2p"))}
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

#[doc="DMA address for full transfer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Or(pub u32);
impl Or {
    #[doc="TIM1_ETR_ADC1 remapping capability"]
    #[inline] pub fn tim1_etr_adc1_rmp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if TIM1_ETR_ADC1_RMP != 0"]
    #[inline] pub fn test_tim1_etr_adc1_rmp(&self) -> bool {
        self.tim1_etr_adc1_rmp() != 0
    }

    #[doc="Sets the TIM1_ETR_ADC1_RMP field."]
    #[inline] pub fn set_tim1_etr_adc1_rmp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Input Capture 1 remap"]
    #[inline] pub fn ti1_rmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TI1_RMP != 0"]
    #[inline] pub fn test_ti1_rmp(&self) -> bool {
        self.ti1_rmp() != 0
    }

    #[doc="Sets the TI1_RMP field."]
    #[inline] pub fn set_ti1_rmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
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
        if self.tim1_etr_adc1_rmp() != 0 { try!(write!(f, " tim1_etr_adc1_rmp=0x{:x}", self.tim1_etr_adc1_rmp()))}
        if self.ti1_rmp() != 0 { try!(write!(f, " ti1_rmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare mode register 2 (output mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccmr3Output(pub u32);
impl Ccmr3Output {
    #[doc="Output Compare 6 mode bit 3"]
    #[inline] pub fn oc6m_bit3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if OC6M_BIT3 != 0"]
    #[inline] pub fn test_oc6m_bit3(&self) -> bool {
        self.oc6m_bit3() != 0
    }

    #[doc="Sets the OC6M_BIT3 field."]
    #[inline] pub fn set_oc6m_bit3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Output Compare 5 mode bit 3"]
    #[inline] pub fn oc5m_bit3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if OC5M_BIT3 != 0"]
    #[inline] pub fn test_oc5m_bit3(&self) -> bool {
        self.oc5m_bit3() != 0
    }

    #[doc="Sets the OC5M_BIT3 field."]
    #[inline] pub fn set_oc5m_bit3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Output compare 6 clear enable"]
    #[inline] pub fn oc6ce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OC6CE != 0"]
    #[inline] pub fn test_oc6ce(&self) -> bool {
        self.oc6ce() != 0
    }

    #[doc="Sets the OC6CE field."]
    #[inline] pub fn set_oc6ce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Output compare 6 mode"]
    #[inline] pub fn oc6m(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if OC6M != 0"]
    #[inline] pub fn test_oc6m(&self) -> bool {
        self.oc6m() != 0
    }

    #[doc="Sets the OC6M field."]
    #[inline] pub fn set_oc6m<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Output compare 6 preload enable"]
    #[inline] pub fn oc6pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if OC6PE != 0"]
    #[inline] pub fn test_oc6pe(&self) -> bool {
        self.oc6pe() != 0
    }

    #[doc="Sets the OC6PE field."]
    #[inline] pub fn set_oc6pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Output compare 6 fast enable"]
    #[inline] pub fn oc6fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OC6FE != 0"]
    #[inline] pub fn test_oc6fe(&self) -> bool {
        self.oc6fe() != 0
    }

    #[doc="Sets the OC6FE field."]
    #[inline] pub fn set_oc6fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Output compare 5 clear enable"]
    #[inline] pub fn oc5ce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OC5CE != 0"]
    #[inline] pub fn test_oc5ce(&self) -> bool {
        self.oc5ce() != 0
    }

    #[doc="Sets the OC5CE field."]
    #[inline] pub fn set_oc5ce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Output compare 5 mode"]
    #[inline] pub fn oc5m(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if OC5M != 0"]
    #[inline] pub fn test_oc5m(&self) -> bool {
        self.oc5m() != 0
    }

    #[doc="Sets the OC5M field."]
    #[inline] pub fn set_oc5m<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Output compare 5 preload enable"]
    #[inline] pub fn oc5pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OC5PE != 0"]
    #[inline] pub fn test_oc5pe(&self) -> bool {
        self.oc5pe() != 0
    }

    #[doc="Sets the OC5PE field."]
    #[inline] pub fn set_oc5pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Output compare 5 fast enable"]
    #[inline] pub fn oc5fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OC5FE != 0"]
    #[inline] pub fn test_oc5fe(&self) -> bool {
        self.oc5fe() != 0
    }

    #[doc="Sets the OC5FE field."]
    #[inline] pub fn set_oc5fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Ccmr3Output {
    #[inline]
    fn from(other: u32) -> Self {
         Ccmr3Output(other)
    }
}

impl ::core::fmt::Display for Ccmr3Output {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccmr3Output {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oc6m_bit3() != 0 { try!(write!(f, " oc6m_bit3"))}
        if self.oc5m_bit3() != 0 { try!(write!(f, " oc5m_bit3"))}
        if self.oc6ce() != 0 { try!(write!(f, " oc6ce"))}
        if self.oc6m() != 0 { try!(write!(f, " oc6m=0x{:x}", self.oc6m()))}
        if self.oc6pe() != 0 { try!(write!(f, " oc6pe"))}
        if self.oc6fe() != 0 { try!(write!(f, " oc6fe"))}
        if self.oc5ce() != 0 { try!(write!(f, " oc5ce"))}
        if self.oc5m() != 0 { try!(write!(f, " oc5m=0x{:x}", self.oc5m()))}
        if self.oc5pe() != 0 { try!(write!(f, " oc5pe"))}
        if self.oc5fe() != 0 { try!(write!(f, " oc5fe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr5(pub u32);
impl Ccr5 {
    #[doc="Capture/Compare value"]
    #[inline] pub fn ccr5(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CCR5 != 0"]
    #[inline] pub fn test_ccr5(&self) -> bool {
        self.ccr5() != 0
    }

    #[doc="Sets the CCR5 field."]
    #[inline] pub fn set_ccr5<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Group Channel 5 and Channel 1"]
    #[inline] pub fn gc5c1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if GC5C1 != 0"]
    #[inline] pub fn test_gc5c1(&self) -> bool {
        self.gc5c1() != 0
    }

    #[doc="Sets the GC5C1 field."]
    #[inline] pub fn set_gc5c1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Group Channel 5 and Channel 2"]
    #[inline] pub fn gc5c2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if GC5C2 != 0"]
    #[inline] pub fn test_gc5c2(&self) -> bool {
        self.gc5c2() != 0
    }

    #[doc="Sets the GC5C2 field."]
    #[inline] pub fn set_gc5c2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Group Channel 5 and Channel 3"]
    #[inline] pub fn gc5c3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GC5C3 != 0"]
    #[inline] pub fn test_gc5c3(&self) -> bool {
        self.gc5c3() != 0
    }

    #[doc="Sets the GC5C3 field."]
    #[inline] pub fn set_gc5c3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ccr5 {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr5(other)
    }
}

impl ::core::fmt::Display for Ccr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccr5() != 0 { try!(write!(f, " ccr5=0x{:x}", self.ccr5()))}
        if self.gc5c1() != 0 { try!(write!(f, " gc5c1"))}
        if self.gc5c2() != 0 { try!(write!(f, " gc5c2"))}
        if self.gc5c3() != 0 { try!(write!(f, " gc5c3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr6(pub u32);
impl Ccr6 {
    #[doc="Capture/Compare value"]
    #[inline] pub fn ccr6(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CCR6 != 0"]
    #[inline] pub fn test_ccr6(&self) -> bool {
        self.ccr6() != 0
    }

    #[doc="Sets the CCR6 field."]
    #[inline] pub fn set_ccr6<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccr6 {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr6(other)
    }
}

impl ::core::fmt::Display for Ccr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccr6() != 0 { try!(write!(f, " ccr6=0x{:x}", self.ccr6()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA address for full transfer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Af1(pub u32);
impl Af1 {
    #[doc="BRK BKIN input enable"]
    #[inline] pub fn bkine(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BKINE != 0"]
    #[inline] pub fn test_bkine(&self) -> bool {
        self.bkine() != 0
    }

    #[doc="Sets the BKINE field."]
    #[inline] pub fn set_bkine<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="BRK COMP1 enable"]
    #[inline] pub fn bkcmp1e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BKCMP1E != 0"]
    #[inline] pub fn test_bkcmp1e(&self) -> bool {
        self.bkcmp1e() != 0
    }

    #[doc="Sets the BKCMP1E field."]
    #[inline] pub fn set_bkcmp1e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="BRK COMP2 enable"]
    #[inline] pub fn bkcmp2e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BKCMP2E != 0"]
    #[inline] pub fn test_bkcmp2e(&self) -> bool {
        self.bkcmp2e() != 0
    }

    #[doc="Sets the BKCMP2E field."]
    #[inline] pub fn set_bkcmp2e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="BRK BKIN input polarity"]
    #[inline] pub fn bkinp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BKINP != 0"]
    #[inline] pub fn test_bkinp(&self) -> bool {
        self.bkinp() != 0
    }

    #[doc="Sets the BKINP field."]
    #[inline] pub fn set_bkinp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BRK COMP1 input polarity"]
    #[inline] pub fn bkcmp1p(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BKCMP1P != 0"]
    #[inline] pub fn test_bkcmp1p(&self) -> bool {
        self.bkcmp1p() != 0
    }

    #[doc="Sets the BKCMP1P field."]
    #[inline] pub fn set_bkcmp1p<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="BRK COMP2 input polarity"]
    #[inline] pub fn bkcmp2p(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if BKCMP2P != 0"]
    #[inline] pub fn test_bkcmp2p(&self) -> bool {
        self.bkcmp2p() != 0
    }

    #[doc="Sets the BKCMP2P field."]
    #[inline] pub fn set_bkcmp2p<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="ETR source selection"]
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

impl From<u32> for Af1 {
    #[inline]
    fn from(other: u32) -> Self {
         Af1(other)
    }
}

impl ::core::fmt::Display for Af1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Af1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bkine() != 0 { try!(write!(f, " bkine"))}
        if self.bkcmp1e() != 0 { try!(write!(f, " bkcmp1e"))}
        if self.bkcmp2e() != 0 { try!(write!(f, " bkcmp2e"))}
        if self.bkinp() != 0 { try!(write!(f, " bkinp"))}
        if self.bkcmp1p() != 0 { try!(write!(f, " bkcmp1p"))}
        if self.bkcmp2p() != 0 { try!(write!(f, " bkcmp2p"))}
        if self.etrsel() != 0 { try!(write!(f, " etrsel=0x{:x}", self.etrsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA address for full transfer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Af2(pub u32);
impl Af2 {
    #[doc="BRK2 BKIN input enable"]
    #[inline] pub fn bk2ine(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BK2INE != 0"]
    #[inline] pub fn test_bk2ine(&self) -> bool {
        self.bk2ine() != 0
    }

    #[doc="Sets the BK2INE field."]
    #[inline] pub fn set_bk2ine<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="BRK2 COMP1 enable"]
    #[inline] pub fn bk2cmp1e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BK2CMP1E != 0"]
    #[inline] pub fn test_bk2cmp1e(&self) -> bool {
        self.bk2cmp1e() != 0
    }

    #[doc="Sets the BK2CMP1E field."]
    #[inline] pub fn set_bk2cmp1e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="BRK2 COMP2 enable"]
    #[inline] pub fn bk2cmp2e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BK2CMP2E != 0"]
    #[inline] pub fn test_bk2cmp2e(&self) -> bool {
        self.bk2cmp2e() != 0
    }

    #[doc="Sets the BK2CMP2E field."]
    #[inline] pub fn set_bk2cmp2e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="BRK2 DFSDM_BREAK0 enable"]
    #[inline] pub fn bk2dfbk0e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BK2DFBK0E != 0"]
    #[inline] pub fn test_bk2dfbk0e(&self) -> bool {
        self.bk2dfbk0e() != 0
    }

    #[doc="Sets the BK2DFBK0E field."]
    #[inline] pub fn set_bk2dfbk0e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BRK2 BKIN input polarity"]
    #[inline] pub fn bk2inp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BK2INP != 0"]
    #[inline] pub fn test_bk2inp(&self) -> bool {
        self.bk2inp() != 0
    }

    #[doc="Sets the BK2INP field."]
    #[inline] pub fn set_bk2inp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BRK2 COMP1 input polarity"]
    #[inline] pub fn bk2cmp1p(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BK2CMP1P != 0"]
    #[inline] pub fn test_bk2cmp1p(&self) -> bool {
        self.bk2cmp1p() != 0
    }

    #[doc="Sets the BK2CMP1P field."]
    #[inline] pub fn set_bk2cmp1p<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="BRK2 COMP2 input polarity"]
    #[inline] pub fn bk2cmp2p(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if BK2CMP2P != 0"]
    #[inline] pub fn test_bk2cmp2p(&self) -> bool {
        self.bk2cmp2p() != 0
    }

    #[doc="Sets the BK2CMP2P field."]
    #[inline] pub fn set_bk2cmp2p<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Af2 {
    #[inline]
    fn from(other: u32) -> Self {
         Af2(other)
    }
}

impl ::core::fmt::Display for Af2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Af2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bk2ine() != 0 { try!(write!(f, " bk2ine"))}
        if self.bk2cmp1e() != 0 { try!(write!(f, " bk2cmp1e"))}
        if self.bk2cmp2e() != 0 { try!(write!(f, " bk2cmp2e"))}
        if self.bk2dfbk0e() != 0 { try!(write!(f, " bk2dfbk0e"))}
        if self.bk2inp() != 0 { try!(write!(f, " bk2inp"))}
        if self.bk2cmp1p() != 0 { try!(write!(f, " bk2cmp1p"))}
        if self.bk2cmp2p() != 0 { try!(write!(f, " bk2cmp2p"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

