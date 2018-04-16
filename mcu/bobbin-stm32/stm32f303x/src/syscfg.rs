
::bobbin_mcu::periph!( SYSCFG, Syscfg, SYSCFG_PERIPH, SyscfgPeriph, SYSCFG_OWNED, SYSCFG_REF_COUNT, 0x40010000, 0x00, 0x01);


#[doc="System configuration controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SyscfgPeriph(pub usize);
impl SyscfgPeriph {
    #[doc="Get the CFGR1 Register."]
    #[inline] pub fn cfgr1_reg(&self) -> ::bobbin_mcu::register::Register<Cfgr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfgr1, 0x0)
    }

    #[doc="Get the *mut pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_mut(&self) -> *mut Cfgr1 { 
        self.cfgr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_ptr(&self) -> *const Cfgr1 { 
        self.cfgr1_reg().ptr()
    }

    #[doc="Read the CFGR1 register."]
    #[inline] pub fn cfgr1(&self) -> Cfgr1 { 
        self.cfgr1_reg().read()
    }

    #[doc="Write the CFGR1 register."]
    #[inline] pub fn write_cfgr1(&self, value: Cfgr1) -> &Self { 
        self.cfgr1_reg().write(value);
        self
    }

    #[doc="Set the CFGR1 register."]
    #[inline] pub fn set_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        self.cfgr1_reg().set(f);
        self
    }

    #[doc="Modify the CFGR1 register."]
    #[inline] pub fn with_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        self.cfgr1_reg().with(f);
        self
    }

    #[doc="Get the EXTICR1 Register."]
    #[inline] pub fn exticr1_reg(&self) -> ::bobbin_mcu::register::Register<Exticr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr1, 0x8)
    }

    #[doc="Get the *mut pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_mut(&self) -> *mut Exticr1 { 
        self.exticr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_ptr(&self) -> *const Exticr1 { 
        self.exticr1_reg().ptr()
    }

    #[doc="Read the EXTICR1 register."]
    #[inline] pub fn exticr1(&self) -> Exticr1 { 
        self.exticr1_reg().read()
    }

    #[doc="Write the EXTICR1 register."]
    #[inline] pub fn write_exticr1(&self, value: Exticr1) -> &Self { 
        self.exticr1_reg().write(value);
        self
    }

    #[doc="Set the EXTICR1 register."]
    #[inline] pub fn set_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        self.exticr1_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR1 register."]
    #[inline] pub fn with_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        self.exticr1_reg().with(f);
        self
    }

    #[doc="Get the EXTICR2 Register."]
    #[inline] pub fn exticr2_reg(&self) -> ::bobbin_mcu::register::Register<Exticr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr2, 0xc)
    }

    #[doc="Get the *mut pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_mut(&self) -> *mut Exticr2 { 
        self.exticr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_ptr(&self) -> *const Exticr2 { 
        self.exticr2_reg().ptr()
    }

    #[doc="Read the EXTICR2 register."]
    #[inline] pub fn exticr2(&self) -> Exticr2 { 
        self.exticr2_reg().read()
    }

    #[doc="Write the EXTICR2 register."]
    #[inline] pub fn write_exticr2(&self, value: Exticr2) -> &Self { 
        self.exticr2_reg().write(value);
        self
    }

    #[doc="Set the EXTICR2 register."]
    #[inline] pub fn set_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        self.exticr2_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR2 register."]
    #[inline] pub fn with_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        self.exticr2_reg().with(f);
        self
    }

    #[doc="Get the EXTICR3 Register."]
    #[inline] pub fn exticr3_reg(&self) -> ::bobbin_mcu::register::Register<Exticr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr3, 0x10)
    }

    #[doc="Get the *mut pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_mut(&self) -> *mut Exticr3 { 
        self.exticr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_ptr(&self) -> *const Exticr3 { 
        self.exticr3_reg().ptr()
    }

    #[doc="Read the EXTICR3 register."]
    #[inline] pub fn exticr3(&self) -> Exticr3 { 
        self.exticr3_reg().read()
    }

    #[doc="Write the EXTICR3 register."]
    #[inline] pub fn write_exticr3(&self, value: Exticr3) -> &Self { 
        self.exticr3_reg().write(value);
        self
    }

    #[doc="Set the EXTICR3 register."]
    #[inline] pub fn set_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        self.exticr3_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR3 register."]
    #[inline] pub fn with_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        self.exticr3_reg().with(f);
        self
    }

    #[doc="Get the EXTICR4 Register."]
    #[inline] pub fn exticr4_reg(&self) -> ::bobbin_mcu::register::Register<Exticr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr4, 0x14)
    }

    #[doc="Get the *mut pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_mut(&self) -> *mut Exticr4 { 
        self.exticr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_ptr(&self) -> *const Exticr4 { 
        self.exticr4_reg().ptr()
    }

    #[doc="Read the EXTICR4 register."]
    #[inline] pub fn exticr4(&self) -> Exticr4 { 
        self.exticr4_reg().read()
    }

    #[doc="Write the EXTICR4 register."]
    #[inline] pub fn write_exticr4(&self, value: Exticr4) -> &Self { 
        self.exticr4_reg().write(value);
        self
    }

    #[doc="Set the EXTICR4 register."]
    #[inline] pub fn set_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        self.exticr4_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR4 register."]
    #[inline] pub fn with_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        self.exticr4_reg().with(f);
        self
    }

    #[doc="Get the CFGR2 Register."]
    #[inline] pub fn cfgr2_reg(&self) -> ::bobbin_mcu::register::Register<Cfgr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfgr2, 0x18)
    }

    #[doc="Get the *mut pointer for the CFGR2 register."]
    #[inline] pub fn cfgr2_mut(&self) -> *mut Cfgr2 { 
        self.cfgr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFGR2 register."]
    #[inline] pub fn cfgr2_ptr(&self) -> *const Cfgr2 { 
        self.cfgr2_reg().ptr()
    }

    #[doc="Read the CFGR2 register."]
    #[inline] pub fn cfgr2(&self) -> Cfgr2 { 
        self.cfgr2_reg().read()
    }

    #[doc="Write the CFGR2 register."]
    #[inline] pub fn write_cfgr2(&self, value: Cfgr2) -> &Self { 
        self.cfgr2_reg().write(value);
        self
    }

    #[doc="Set the CFGR2 register."]
    #[inline] pub fn set_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
        self.cfgr2_reg().set(f);
        self
    }

    #[doc="Modify the CFGR2 register."]
    #[inline] pub fn with_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
        self.cfgr2_reg().with(f);
        self
    }

    #[doc="Get the RCR Register."]
    #[inline] pub fn rcr_reg(&self) -> ::bobbin_mcu::register::Register<Rcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rcr, 0x4)
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

}

#[doc="configuration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
    #[doc="Memory mapping selection bits"]
    #[inline] pub fn mem_mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MEM_MODE != 0"]
    #[inline] pub fn test_mem_mode(&self) -> bool {
        self.mem_mode() != 0
    }

    #[doc="Sets the MEM_MODE field."]
    #[inline] pub fn set_mem_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="USB interrupt remap"]
    #[inline] pub fn usb_it_rmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if USB_IT_RMP != 0"]
    #[inline] pub fn test_usb_it_rmp(&self) -> bool {
        self.usb_it_rmp() != 0
    }

    #[doc="Sets the USB_IT_RMP field."]
    #[inline] pub fn set_usb_it_rmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Timer 1 ITR3 selection"]
    #[inline] pub fn tim1_itr_rmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIM1_ITR_RMP != 0"]
    #[inline] pub fn test_tim1_itr_rmp(&self) -> bool {
        self.tim1_itr_rmp() != 0
    }

    #[doc="Sets the TIM1_ITR_RMP field."]
    #[inline] pub fn set_tim1_itr_rmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DAC trigger remap (when TSEL = 001)"]
    #[inline] pub fn dac_trig_rmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DAC_TRIG_RMP != 0"]
    #[inline] pub fn test_dac_trig_rmp(&self) -> bool {
        self.dac_trig_rmp() != 0
    }

    #[doc="Sets the DAC_TRIG_RMP field."]
    #[inline] pub fn set_dac_trig_rmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="ADC24 DMA remapping bit"]
    #[inline] pub fn adc24_dma_rmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADC24_DMA_RMP != 0"]
    #[inline] pub fn test_adc24_dma_rmp(&self) -> bool {
        self.adc24_dma_rmp() != 0
    }

    #[doc="Sets the ADC24_DMA_RMP field."]
    #[inline] pub fn set_adc24_dma_rmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TIM16 DMA request remapping bit"]
    #[inline] pub fn tim16_dma_rmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TIM16_DMA_RMP != 0"]
    #[inline] pub fn test_tim16_dma_rmp(&self) -> bool {
        self.tim16_dma_rmp() != 0
    }

    #[doc="Sets the TIM16_DMA_RMP field."]
    #[inline] pub fn set_tim16_dma_rmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="TIM17 DMA request remapping bit"]
    #[inline] pub fn tim17_dma_rmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TIM17_DMA_RMP != 0"]
    #[inline] pub fn test_tim17_dma_rmp(&self) -> bool {
        self.tim17_dma_rmp() != 0
    }

    #[doc="Sets the TIM17_DMA_RMP field."]
    #[inline] pub fn set_tim17_dma_rmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TIM6 and DAC1 DMA request remapping bit"]
    #[inline] pub fn tim6_dac1_dma_rmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TIM6_DAC1_DMA_RMP != 0"]
    #[inline] pub fn test_tim6_dac1_dma_rmp(&self) -> bool {
        self.tim6_dac1_dma_rmp() != 0
    }

    #[doc="Sets the TIM6_DAC1_DMA_RMP field."]
    #[inline] pub fn set_tim6_dac1_dma_rmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TIM7 and DAC2 DMA request remapping bit"]
    #[inline] pub fn tim7_dac2_dma_rmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TIM7_DAC2_DMA_RMP != 0"]
    #[inline] pub fn test_tim7_dac2_dma_rmp(&self) -> bool {
        self.tim7_dac2_dma_rmp() != 0
    }

    #[doc="Sets the TIM7_DAC2_DMA_RMP field."]
    #[inline] pub fn set_tim7_dac2_dma_rmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline] pub fn i2c_pb6_fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if I2C_PB6_FM != 0"]
    #[inline] pub fn test_i2c_pb6_fm(&self) -> bool {
        self.i2c_pb6_fm() != 0
    }

    #[doc="Sets the I2C_PB6_FM field."]
    #[inline] pub fn set_i2c_pb6_fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline] pub fn i2c_pb7_fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if I2C_PB7_FM != 0"]
    #[inline] pub fn test_i2c_pb7_fm(&self) -> bool {
        self.i2c_pb7_fm() != 0
    }

    #[doc="Sets the I2C_PB7_FM field."]
    #[inline] pub fn set_i2c_pb7_fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline] pub fn i2c_pb8_fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if I2C_PB8_FM != 0"]
    #[inline] pub fn test_i2c_pb8_fm(&self) -> bool {
        self.i2c_pb8_fm() != 0
    }

    #[doc="Sets the I2C_PB8_FM field."]
    #[inline] pub fn set_i2c_pb8_fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline] pub fn i2c_pb9_fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if I2C_PB9_FM != 0"]
    #[inline] pub fn test_i2c_pb9_fm(&self) -> bool {
        self.i2c_pb9_fm() != 0
    }

    #[doc="Sets the I2C_PB9_FM field."]
    #[inline] pub fn set_i2c_pb9_fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="I2C1 Fast Mode Plus"]
    #[inline] pub fn i2c1_fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if I2C1_FM != 0"]
    #[inline] pub fn test_i2c1_fm(&self) -> bool {
        self.i2c1_fm() != 0
    }

    #[doc="Sets the I2C1_FM field."]
    #[inline] pub fn set_i2c1_fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="I2C2 Fast Mode Plus"]
    #[inline] pub fn i2c2_fm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C2_FM != 0"]
    #[inline] pub fn test_i2c2_fm(&self) -> bool {
        self.i2c2_fm() != 0
    }

    #[doc="Sets the I2C2_FM field."]
    #[inline] pub fn set_i2c2_fm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Encoder mode"]
    #[inline] pub fn encoder_mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if ENCODER_MODE != 0"]
    #[inline] pub fn test_encoder_mode(&self) -> bool {
        self.encoder_mode() != 0
    }

    #[doc="Sets the ENCODER_MODE field."]
    #[inline] pub fn set_encoder_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Interrupt enable bits from FPU"]
    #[inline] pub fn fpu_it(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
    }

    #[doc="Returns true if FPU_IT != 0"]
    #[inline] pub fn test_fpu_it(&self) -> bool {
        self.fpu_it() != 0
    }

    #[doc="Sets the FPU_IT field."]
    #[inline] pub fn set_fpu_it<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Cfgr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr1(other)
    }
}

impl ::core::fmt::Display for Cfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mem_mode() != 0 { try!(write!(f, " mem_mode=0x{:x}", self.mem_mode()))}
        if self.usb_it_rmp() != 0 { try!(write!(f, " usb_it_rmp"))}
        if self.tim1_itr_rmp() != 0 { try!(write!(f, " tim1_itr_rmp"))}
        if self.dac_trig_rmp() != 0 { try!(write!(f, " dac_trig_rmp"))}
        if self.adc24_dma_rmp() != 0 { try!(write!(f, " adc24_dma_rmp"))}
        if self.tim16_dma_rmp() != 0 { try!(write!(f, " tim16_dma_rmp"))}
        if self.tim17_dma_rmp() != 0 { try!(write!(f, " tim17_dma_rmp"))}
        if self.tim6_dac1_dma_rmp() != 0 { try!(write!(f, " tim6_dac1_dma_rmp"))}
        if self.tim7_dac2_dma_rmp() != 0 { try!(write!(f, " tim7_dac2_dma_rmp"))}
        if self.i2c_pb6_fm() != 0 { try!(write!(f, " i2c_pb6_fm"))}
        if self.i2c_pb7_fm() != 0 { try!(write!(f, " i2c_pb7_fm"))}
        if self.i2c_pb8_fm() != 0 { try!(write!(f, " i2c_pb8_fm"))}
        if self.i2c_pb9_fm() != 0 { try!(write!(f, " i2c_pb9_fm"))}
        if self.i2c1_fm() != 0 { try!(write!(f, " i2c1_fm"))}
        if self.i2c2_fm() != 0 { try!(write!(f, " i2c2_fm"))}
        if self.encoder_mode() != 0 { try!(write!(f, " encoder_mode=0x{:x}", self.encoder_mode()))}
        if self.fpu_it() != 0 { try!(write!(f, " fpu_it=0x{:x}", self.fpu_it()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr1(pub u32);
impl Exticr1 {
    #[doc="EXTI 3 configuration bits"]
    #[inline] pub fn exti3(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI3 != 0"]
    #[inline] pub fn test_exti3(&self) -> bool {
        self.exti3() != 0
    }

    #[doc="Sets the EXTI3 field."]
    #[inline] pub fn set_exti3<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI 2 configuration bits"]
    #[inline] pub fn exti2(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI2 != 0"]
    #[inline] pub fn test_exti2(&self) -> bool {
        self.exti2() != 0
    }

    #[doc="Sets the EXTI2 field."]
    #[inline] pub fn set_exti2<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI 1 configuration bits"]
    #[inline] pub fn exti1(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI1 != 0"]
    #[inline] pub fn test_exti1(&self) -> bool {
        self.exti1() != 0
    }

    #[doc="Sets the EXTI1 field."]
    #[inline] pub fn set_exti1<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI 0 configuration bits"]
    #[inline] pub fn exti0(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI0 != 0"]
    #[inline] pub fn test_exti0(&self) -> bool {
        self.exti0() != 0
    }

    #[doc="Sets the EXTI0 field."]
    #[inline] pub fn set_exti0<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr1(other)
    }
}

impl ::core::fmt::Display for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti3() != 0 { try!(write!(f, " exti3=0x{:x}", self.exti3()))}
        if self.exti2() != 0 { try!(write!(f, " exti2=0x{:x}", self.exti2()))}
        if self.exti1() != 0 { try!(write!(f, " exti1=0x{:x}", self.exti1()))}
        if self.exti0() != 0 { try!(write!(f, " exti0=0x{:x}", self.exti0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr2(pub u32);
impl Exticr2 {
    #[doc="EXTI 7 configuration bits"]
    #[inline] pub fn exti7(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI7 != 0"]
    #[inline] pub fn test_exti7(&self) -> bool {
        self.exti7() != 0
    }

    #[doc="Sets the EXTI7 field."]
    #[inline] pub fn set_exti7<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI 6 configuration bits"]
    #[inline] pub fn exti6(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI6 != 0"]
    #[inline] pub fn test_exti6(&self) -> bool {
        self.exti6() != 0
    }

    #[doc="Sets the EXTI6 field."]
    #[inline] pub fn set_exti6<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI 5 configuration bits"]
    #[inline] pub fn exti5(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI5 != 0"]
    #[inline] pub fn test_exti5(&self) -> bool {
        self.exti5() != 0
    }

    #[doc="Sets the EXTI5 field."]
    #[inline] pub fn set_exti5<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI 4 configuration bits"]
    #[inline] pub fn exti4(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI4 != 0"]
    #[inline] pub fn test_exti4(&self) -> bool {
        self.exti4() != 0
    }

    #[doc="Sets the EXTI4 field."]
    #[inline] pub fn set_exti4<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr2(other)
    }
}

impl ::core::fmt::Display for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti7() != 0 { try!(write!(f, " exti7=0x{:x}", self.exti7()))}
        if self.exti6() != 0 { try!(write!(f, " exti6=0x{:x}", self.exti6()))}
        if self.exti5() != 0 { try!(write!(f, " exti5=0x{:x}", self.exti5()))}
        if self.exti4() != 0 { try!(write!(f, " exti4=0x{:x}", self.exti4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr3(pub u32);
impl Exticr3 {
    #[doc="EXTI 11 configuration bits"]
    #[inline] pub fn exti11(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI11 != 0"]
    #[inline] pub fn test_exti11(&self) -> bool {
        self.exti11() != 0
    }

    #[doc="Sets the EXTI11 field."]
    #[inline] pub fn set_exti11<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI 10 configuration bits"]
    #[inline] pub fn exti10(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI10 != 0"]
    #[inline] pub fn test_exti10(&self) -> bool {
        self.exti10() != 0
    }

    #[doc="Sets the EXTI10 field."]
    #[inline] pub fn set_exti10<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI 9 configuration bits"]
    #[inline] pub fn exti9(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI9 != 0"]
    #[inline] pub fn test_exti9(&self) -> bool {
        self.exti9() != 0
    }

    #[doc="Sets the EXTI9 field."]
    #[inline] pub fn set_exti9<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI 8 configuration bits"]
    #[inline] pub fn exti8(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI8 != 0"]
    #[inline] pub fn test_exti8(&self) -> bool {
        self.exti8() != 0
    }

    #[doc="Sets the EXTI8 field."]
    #[inline] pub fn set_exti8<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr3(other)
    }
}

impl ::core::fmt::Display for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti11() != 0 { try!(write!(f, " exti11=0x{:x}", self.exti11()))}
        if self.exti10() != 0 { try!(write!(f, " exti10=0x{:x}", self.exti10()))}
        if self.exti9() != 0 { try!(write!(f, " exti9=0x{:x}", self.exti9()))}
        if self.exti8() != 0 { try!(write!(f, " exti8=0x{:x}", self.exti8()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr4(pub u32);
impl Exticr4 {
    #[doc="EXTI 15 configuration bits"]
    #[inline] pub fn exti15(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI15 != 0"]
    #[inline] pub fn test_exti15(&self) -> bool {
        self.exti15() != 0
    }

    #[doc="Sets the EXTI15 field."]
    #[inline] pub fn set_exti15<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI 14 configuration bits"]
    #[inline] pub fn exti14(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI14 != 0"]
    #[inline] pub fn test_exti14(&self) -> bool {
        self.exti14() != 0
    }

    #[doc="Sets the EXTI14 field."]
    #[inline] pub fn set_exti14<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI 13 configuration bits"]
    #[inline] pub fn exti13(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI13 != 0"]
    #[inline] pub fn test_exti13(&self) -> bool {
        self.exti13() != 0
    }

    #[doc="Sets the EXTI13 field."]
    #[inline] pub fn set_exti13<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI 12 configuration bits"]
    #[inline] pub fn exti12(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI12 != 0"]
    #[inline] pub fn test_exti12(&self) -> bool {
        self.exti12() != 0
    }

    #[doc="Sets the EXTI12 field."]
    #[inline] pub fn set_exti12<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr4(other)
    }
}

impl ::core::fmt::Display for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti15() != 0 { try!(write!(f, " exti15=0x{:x}", self.exti15()))}
        if self.exti14() != 0 { try!(write!(f, " exti14=0x{:x}", self.exti14()))}
        if self.exti13() != 0 { try!(write!(f, " exti13=0x{:x}", self.exti13()))}
        if self.exti12() != 0 { try!(write!(f, " exti12=0x{:x}", self.exti12()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="configuration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
    #[doc="Cortex-M0 LOCKUP bit enable bit"]
    #[inline] pub fn locup_lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LOCUP_LOCK != 0"]
    #[inline] pub fn test_locup_lock(&self) -> bool {
        self.locup_lock() != 0
    }

    #[doc="Sets the LOCUP_LOCK field."]
    #[inline] pub fn set_locup_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SRAM parity lock bit"]
    #[inline] pub fn sram_parity_lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SRAM_PARITY_LOCK != 0"]
    #[inline] pub fn test_sram_parity_lock(&self) -> bool {
        self.sram_parity_lock() != 0
    }

    #[doc="Sets the SRAM_PARITY_LOCK field."]
    #[inline] pub fn set_sram_parity_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="PVD lock enable bit"]
    #[inline] pub fn pvd_lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PVD_LOCK != 0"]
    #[inline] pub fn test_pvd_lock(&self) -> bool {
        self.pvd_lock() != 0
    }

    #[doc="Sets the PVD_LOCK field."]
    #[inline] pub fn set_pvd_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Bypass address bit 29 in parity calculation"]
    #[inline] pub fn byp_add_par(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BYP_ADD_PAR != 0"]
    #[inline] pub fn test_byp_add_par(&self) -> bool {
        self.byp_add_par() != 0
    }

    #[doc="Sets the BYP_ADD_PAR field."]
    #[inline] pub fn set_byp_add_par<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SRAM parity flag"]
    #[inline] pub fn sram_pef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SRAM_PEF != 0"]
    #[inline] pub fn test_sram_pef(&self) -> bool {
        self.sram_pef() != 0
    }

    #[doc="Sets the SRAM_PEF field."]
    #[inline] pub fn set_sram_pef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Cfgr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr2(other)
    }
}

impl ::core::fmt::Display for Cfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.locup_lock() != 0 { try!(write!(f, " locup_lock"))}
        if self.sram_parity_lock() != 0 { try!(write!(f, " sram_parity_lock"))}
        if self.pvd_lock() != 0 { try!(write!(f, " pvd_lock"))}
        if self.byp_add_par() != 0 { try!(write!(f, " byp_add_par"))}
        if self.sram_pef() != 0 { try!(write!(f, " sram_pef"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CCM SRAM protection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc="CCM SRAM page write protection bit"]
    #[inline] pub fn page0_wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAGE0_WP != 0"]
    #[inline] pub fn test_page0_wp(&self) -> bool {
        self.page0_wp() != 0
    }

    #[doc="Sets the PAGE0_WP field."]
    #[inline] pub fn set_page0_wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CCM SRAM page write protection bit"]
    #[inline] pub fn page1_wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PAGE1_WP != 0"]
    #[inline] pub fn test_page1_wp(&self) -> bool {
        self.page1_wp() != 0
    }

    #[doc="Sets the PAGE1_WP field."]
    #[inline] pub fn set_page1_wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CCM SRAM page write protection bit"]
    #[inline] pub fn page2_wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAGE2_WP != 0"]
    #[inline] pub fn test_page2_wp(&self) -> bool {
        self.page2_wp() != 0
    }

    #[doc="Sets the PAGE2_WP field."]
    #[inline] pub fn set_page2_wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CCM SRAM page write protection bit"]
    #[inline] pub fn page3_wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PAGE3_WP != 0"]
    #[inline] pub fn test_page3_wp(&self) -> bool {
        self.page3_wp() != 0
    }

    #[doc="Sets the PAGE3_WP field."]
    #[inline] pub fn set_page3_wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="CCM SRAM page write protection bit"]
    #[inline] pub fn page4_wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAGE4_WP != 0"]
    #[inline] pub fn test_page4_wp(&self) -> bool {
        self.page4_wp() != 0
    }

    #[doc="Sets the PAGE4_WP field."]
    #[inline] pub fn set_page4_wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="CCM SRAM page write protection bit"]
    #[inline] pub fn page5_wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PAGE5_WP != 0"]
    #[inline] pub fn test_page5_wp(&self) -> bool {
        self.page5_wp() != 0
    }

    #[doc="Sets the PAGE5_WP field."]
    #[inline] pub fn set_page5_wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="CCM SRAM page write protection bit"]
    #[inline] pub fn page6_wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PAGE6_WP != 0"]
    #[inline] pub fn test_page6_wp(&self) -> bool {
        self.page6_wp() != 0
    }

    #[doc="Sets the PAGE6_WP field."]
    #[inline] pub fn set_page6_wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="CCM SRAM page write protection bit"]
    #[inline] pub fn page7_wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PAGE7_WP != 0"]
    #[inline] pub fn test_page7_wp(&self) -> bool {
        self.page7_wp() != 0
    }

    #[doc="Sets the PAGE7_WP field."]
    #[inline] pub fn set_page7_wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.page0_wp() != 0 { try!(write!(f, " page0_wp"))}
        if self.page1_wp() != 0 { try!(write!(f, " page1_wp"))}
        if self.page2_wp() != 0 { try!(write!(f, " page2_wp"))}
        if self.page3_wp() != 0 { try!(write!(f, " page3_wp"))}
        if self.page4_wp() != 0 { try!(write!(f, " page4_wp"))}
        if self.page5_wp() != 0 { try!(write!(f, " page5_wp"))}
        if self.page6_wp() != 0 { try!(write!(f, " page6_wp"))}
        if self.page7_wp() != 0 { try!(write!(f, " page7_wp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

