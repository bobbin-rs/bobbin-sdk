::bobbin_mcu::periph!( TIM16, Tim16, TIM16_PERIPH, Tim16Periph, TIM16_OWNED, TIM16_REF_COUNT, 0x40014400, 0x00, 0x25);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TIM16 Peripheral"]
pub struct Tim16Periph(pub usize); 

impl Tim16Periph {
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

}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
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

    #[doc="BRK COMP2 input polarit"]
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
        if self.bkine() != 0 { try!(write!(f, " bkine"))}
        if self.bkcmp1e() != 0 { try!(write!(f, " bkcmp1e"))}
        if self.bkcmp2e() != 0 { try!(write!(f, " bkcmp2e"))}
        if self.bkinp() != 0 { try!(write!(f, " bkinp"))}
        if self.bkcmp1p() != 0 { try!(write!(f, " bkcmp1p"))}
        if self.bkcmp2p() != 0 { try!(write!(f, " bkcmp2p"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
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
        if self.ois1n() != 0 { try!(write!(f, " ois1n"))}
        if self.ois1() != 0 { try!(write!(f, " ois1"))}
        if self.ccds() != 0 { try!(write!(f, " ccds"))}
        if self.ccus() != 0 { try!(write!(f, " ccus"))}
        if self.ccpc() != 0 { try!(write!(f, " ccpc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA/Interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dier(pub u32);
impl Dier {
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

    #[doc="BRK COMP2 input polarit"]
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
        if self.bkine() != 0 { try!(write!(f, " bkine"))}
        if self.bkcmp1e() != 0 { try!(write!(f, " bkcmp1e"))}
        if self.bkcmp2e() != 0 { try!(write!(f, " bkcmp2e"))}
        if self.bkinp() != 0 { try!(write!(f, " bkinp"))}
        if self.bkcmp1p() != 0 { try!(write!(f, " bkcmp1p"))}
        if self.bkcmp2p() != 0 { try!(write!(f, " bkcmp2p"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
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
        if self.cc1of() != 0 { try!(write!(f, " cc1of"))}
        if self.bif() != 0 { try!(write!(f, " bif"))}
        if self.tif() != 0 { try!(write!(f, " tif"))}
        if self.comif() != 0 { try!(write!(f, " comif"))}
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
        if self.bg() != 0 { try!(write!(f, " bg"))}
        if self.tg() != 0 { try!(write!(f, " tg"))}
        if self.comg() != 0 { try!(write!(f, " comg"))}
        if self.cc1g() != 0 { try!(write!(f, " cc1g"))}
        if self.ug() != 0 { try!(write!(f, " ug"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare mode register (output mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccmr1Output(pub u32);
impl Ccmr1Output {
    #[doc="Output Compare 1 mode"]
    #[inline] pub fn oc1m_2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if OC1M_2 != 0"]
    #[inline] pub fn test_oc1m_2(&self) -> bool {
        self.oc1m_2() != 0
    }

    #[doc="Sets the OC1M_2 field."]
    #[inline] pub fn set_oc1m_2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
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
        if self.oc1m_2() != 0 { try!(write!(f, " oc1m_2"))}
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
        if self.ic1f() != 0 { try!(write!(f, " ic1f=0x{:x}", self.ic1f()))}
        if self.ic1psc() != 0 { try!(write!(f, " ic1psc=0x{:x}", self.ic1psc()))}
        if self.cc1s() != 0 { try!(write!(f, " cc1s=0x{:x}", self.cc1s()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="capture/compare enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccer(pub u32);
impl Ccer {
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
        if self.cc1np() != 0 { try!(write!(f, " cc1np"))}
        if self.cc1ne() != 0 { try!(write!(f, " cc1ne"))}
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

    #[doc="UIF Copy"]
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
    #[inline] pub fn rep(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if REP != 0"]
    #[inline] pub fn test_rep(&self) -> bool {
        self.rep() != 0
    }

    #[doc="Sets the REP field."]
    #[inline] pub fn set_rep<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
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

#[doc="TIM16 option register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Or(pub u32);
impl Or {
    #[doc="Input capture 1 remap"]
    #[inline] pub fn ti1_rmp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if TI1_RMP != 0"]
    #[inline] pub fn test_ti1_rmp(&self) -> bool {
        self.ti1_rmp() != 0
    }

    #[doc="Sets the TI1_RMP field."]
    #[inline] pub fn set_ti1_rmp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
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
        if self.ti1_rmp() != 0 { try!(write!(f, " ti1_rmp=0x{:x}", self.ti1_rmp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TIM17 option register 1"]
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

    #[doc="BRK COMP2 input polarit"]
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
        try!(write!(f, "]"));
        Ok(())
    }
}

