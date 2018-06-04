
::bobbin_mcu::periph!( ETHERNET_MAC, Ethernet, ETHERNET_MAC_PERIPH, EthernetPeriph, ETHERNET_MAC_OWNED, ETHERNET_MAC_REF_COUNT, 0x40028000, 0x00, 0x05);


#[doc="Ethernet: media access control (MAC)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EthernetPeriph(pub usize);
impl EthernetPeriph {
    #[doc="Get the MACCR Register."]
    #[inline] pub fn maccr_reg(&self) -> ::bobbin_mcu::register::Register<Maccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Maccr, 0x0)
    }

    #[doc="Get the *mut pointer for the MACCR register."]
    #[inline] pub fn maccr_mut(&self) -> *mut Maccr { 
        self.maccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACCR register."]
    #[inline] pub fn maccr_ptr(&self) -> *const Maccr { 
        self.maccr_reg().ptr()
    }

    #[doc="Read the MACCR register."]
    #[inline] pub fn maccr(&self) -> Maccr { 
        self.maccr_reg().read()
    }

    #[doc="Write the MACCR register."]
    #[inline] pub fn write_maccr(&self, value: Maccr) -> &Self { 
        self.maccr_reg().write(value);
        self
    }

    #[doc="Set the MACCR register."]
    #[inline] pub fn set_maccr<F: FnOnce(Maccr) -> Maccr>(&self, f: F) -> &Self {
        self.maccr_reg().set(f);
        self
    }

    #[doc="Modify the MACCR register."]
    #[inline] pub fn with_maccr<F: FnOnce(Maccr) -> Maccr>(&self, f: F) -> &Self {
        self.maccr_reg().with(f);
        self
    }

    #[doc="Get the MACFFR Register."]
    #[inline] pub fn macffr_reg(&self) -> ::bobbin_mcu::register::Register<Macffr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Macffr, 0x4)
    }

    #[doc="Get the *mut pointer for the MACFFR register."]
    #[inline] pub fn macffr_mut(&self) -> *mut Macffr { 
        self.macffr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACFFR register."]
    #[inline] pub fn macffr_ptr(&self) -> *const Macffr { 
        self.macffr_reg().ptr()
    }

    #[doc="Read the MACFFR register."]
    #[inline] pub fn macffr(&self) -> Macffr { 
        self.macffr_reg().read()
    }

    #[doc="Write the MACFFR register."]
    #[inline] pub fn write_macffr(&self, value: Macffr) -> &Self { 
        self.macffr_reg().write(value);
        self
    }

    #[doc="Set the MACFFR register."]
    #[inline] pub fn set_macffr<F: FnOnce(Macffr) -> Macffr>(&self, f: F) -> &Self {
        self.macffr_reg().set(f);
        self
    }

    #[doc="Modify the MACFFR register."]
    #[inline] pub fn with_macffr<F: FnOnce(Macffr) -> Macffr>(&self, f: F) -> &Self {
        self.macffr_reg().with(f);
        self
    }

    #[doc="Get the MACHTHR Register."]
    #[inline] pub fn machthr_reg(&self) -> ::bobbin_mcu::register::Register<Machthr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Machthr, 0x8)
    }

    #[doc="Get the *mut pointer for the MACHTHR register."]
    #[inline] pub fn machthr_mut(&self) -> *mut Machthr { 
        self.machthr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACHTHR register."]
    #[inline] pub fn machthr_ptr(&self) -> *const Machthr { 
        self.machthr_reg().ptr()
    }

    #[doc="Read the MACHTHR register."]
    #[inline] pub fn machthr(&self) -> Machthr { 
        self.machthr_reg().read()
    }

    #[doc="Write the MACHTHR register."]
    #[inline] pub fn write_machthr(&self, value: Machthr) -> &Self { 
        self.machthr_reg().write(value);
        self
    }

    #[doc="Set the MACHTHR register."]
    #[inline] pub fn set_machthr<F: FnOnce(Machthr) -> Machthr>(&self, f: F) -> &Self {
        self.machthr_reg().set(f);
        self
    }

    #[doc="Modify the MACHTHR register."]
    #[inline] pub fn with_machthr<F: FnOnce(Machthr) -> Machthr>(&self, f: F) -> &Self {
        self.machthr_reg().with(f);
        self
    }

    #[doc="Get the MACHTLR Register."]
    #[inline] pub fn machtlr_reg(&self) -> ::bobbin_mcu::register::Register<Machtlr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Machtlr, 0xc)
    }

    #[doc="Get the *mut pointer for the MACHTLR register."]
    #[inline] pub fn machtlr_mut(&self) -> *mut Machtlr { 
        self.machtlr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACHTLR register."]
    #[inline] pub fn machtlr_ptr(&self) -> *const Machtlr { 
        self.machtlr_reg().ptr()
    }

    #[doc="Read the MACHTLR register."]
    #[inline] pub fn machtlr(&self) -> Machtlr { 
        self.machtlr_reg().read()
    }

    #[doc="Write the MACHTLR register."]
    #[inline] pub fn write_machtlr(&self, value: Machtlr) -> &Self { 
        self.machtlr_reg().write(value);
        self
    }

    #[doc="Set the MACHTLR register."]
    #[inline] pub fn set_machtlr<F: FnOnce(Machtlr) -> Machtlr>(&self, f: F) -> &Self {
        self.machtlr_reg().set(f);
        self
    }

    #[doc="Modify the MACHTLR register."]
    #[inline] pub fn with_machtlr<F: FnOnce(Machtlr) -> Machtlr>(&self, f: F) -> &Self {
        self.machtlr_reg().with(f);
        self
    }

    #[doc="Get the MACMIIAR Register."]
    #[inline] pub fn macmiiar_reg(&self) -> ::bobbin_mcu::register::Register<Macmiiar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Macmiiar, 0x10)
    }

    #[doc="Get the *mut pointer for the MACMIIAR register."]
    #[inline] pub fn macmiiar_mut(&self) -> *mut Macmiiar { 
        self.macmiiar_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACMIIAR register."]
    #[inline] pub fn macmiiar_ptr(&self) -> *const Macmiiar { 
        self.macmiiar_reg().ptr()
    }

    #[doc="Read the MACMIIAR register."]
    #[inline] pub fn macmiiar(&self) -> Macmiiar { 
        self.macmiiar_reg().read()
    }

    #[doc="Write the MACMIIAR register."]
    #[inline] pub fn write_macmiiar(&self, value: Macmiiar) -> &Self { 
        self.macmiiar_reg().write(value);
        self
    }

    #[doc="Set the MACMIIAR register."]
    #[inline] pub fn set_macmiiar<F: FnOnce(Macmiiar) -> Macmiiar>(&self, f: F) -> &Self {
        self.macmiiar_reg().set(f);
        self
    }

    #[doc="Modify the MACMIIAR register."]
    #[inline] pub fn with_macmiiar<F: FnOnce(Macmiiar) -> Macmiiar>(&self, f: F) -> &Self {
        self.macmiiar_reg().with(f);
        self
    }

    #[doc="Get the MACMIIDR Register."]
    #[inline] pub fn macmiidr_reg(&self) -> ::bobbin_mcu::register::Register<Macmiidr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Macmiidr, 0x14)
    }

    #[doc="Get the *mut pointer for the MACMIIDR register."]
    #[inline] pub fn macmiidr_mut(&self) -> *mut Macmiidr { 
        self.macmiidr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACMIIDR register."]
    #[inline] pub fn macmiidr_ptr(&self) -> *const Macmiidr { 
        self.macmiidr_reg().ptr()
    }

    #[doc="Read the MACMIIDR register."]
    #[inline] pub fn macmiidr(&self) -> Macmiidr { 
        self.macmiidr_reg().read()
    }

    #[doc="Write the MACMIIDR register."]
    #[inline] pub fn write_macmiidr(&self, value: Macmiidr) -> &Self { 
        self.macmiidr_reg().write(value);
        self
    }

    #[doc="Set the MACMIIDR register."]
    #[inline] pub fn set_macmiidr<F: FnOnce(Macmiidr) -> Macmiidr>(&self, f: F) -> &Self {
        self.macmiidr_reg().set(f);
        self
    }

    #[doc="Modify the MACMIIDR register."]
    #[inline] pub fn with_macmiidr<F: FnOnce(Macmiidr) -> Macmiidr>(&self, f: F) -> &Self {
        self.macmiidr_reg().with(f);
        self
    }

    #[doc="Get the MACFCR Register."]
    #[inline] pub fn macfcr_reg(&self) -> ::bobbin_mcu::register::Register<Macfcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Macfcr, 0x18)
    }

    #[doc="Get the *mut pointer for the MACFCR register."]
    #[inline] pub fn macfcr_mut(&self) -> *mut Macfcr { 
        self.macfcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACFCR register."]
    #[inline] pub fn macfcr_ptr(&self) -> *const Macfcr { 
        self.macfcr_reg().ptr()
    }

    #[doc="Read the MACFCR register."]
    #[inline] pub fn macfcr(&self) -> Macfcr { 
        self.macfcr_reg().read()
    }

    #[doc="Write the MACFCR register."]
    #[inline] pub fn write_macfcr(&self, value: Macfcr) -> &Self { 
        self.macfcr_reg().write(value);
        self
    }

    #[doc="Set the MACFCR register."]
    #[inline] pub fn set_macfcr<F: FnOnce(Macfcr) -> Macfcr>(&self, f: F) -> &Self {
        self.macfcr_reg().set(f);
        self
    }

    #[doc="Modify the MACFCR register."]
    #[inline] pub fn with_macfcr<F: FnOnce(Macfcr) -> Macfcr>(&self, f: F) -> &Self {
        self.macfcr_reg().with(f);
        self
    }

    #[doc="Get the MACVLANTR Register."]
    #[inline] pub fn macvlantr_reg(&self) -> ::bobbin_mcu::register::Register<Macvlantr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Macvlantr, 0x1c)
    }

    #[doc="Get the *mut pointer for the MACVLANTR register."]
    #[inline] pub fn macvlantr_mut(&self) -> *mut Macvlantr { 
        self.macvlantr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACVLANTR register."]
    #[inline] pub fn macvlantr_ptr(&self) -> *const Macvlantr { 
        self.macvlantr_reg().ptr()
    }

    #[doc="Read the MACVLANTR register."]
    #[inline] pub fn macvlantr(&self) -> Macvlantr { 
        self.macvlantr_reg().read()
    }

    #[doc="Write the MACVLANTR register."]
    #[inline] pub fn write_macvlantr(&self, value: Macvlantr) -> &Self { 
        self.macvlantr_reg().write(value);
        self
    }

    #[doc="Set the MACVLANTR register."]
    #[inline] pub fn set_macvlantr<F: FnOnce(Macvlantr) -> Macvlantr>(&self, f: F) -> &Self {
        self.macvlantr_reg().set(f);
        self
    }

    #[doc="Modify the MACVLANTR register."]
    #[inline] pub fn with_macvlantr<F: FnOnce(Macvlantr) -> Macvlantr>(&self, f: F) -> &Self {
        self.macvlantr_reg().with(f);
        self
    }

    #[doc="Get the MACPMTCSR Register."]
    #[inline] pub fn macpmtcsr_reg(&self) -> ::bobbin_mcu::register::Register<Macpmtcsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Macpmtcsr, 0x2c)
    }

    #[doc="Get the *mut pointer for the MACPMTCSR register."]
    #[inline] pub fn macpmtcsr_mut(&self) -> *mut Macpmtcsr { 
        self.macpmtcsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACPMTCSR register."]
    #[inline] pub fn macpmtcsr_ptr(&self) -> *const Macpmtcsr { 
        self.macpmtcsr_reg().ptr()
    }

    #[doc="Read the MACPMTCSR register."]
    #[inline] pub fn macpmtcsr(&self) -> Macpmtcsr { 
        self.macpmtcsr_reg().read()
    }

    #[doc="Write the MACPMTCSR register."]
    #[inline] pub fn write_macpmtcsr(&self, value: Macpmtcsr) -> &Self { 
        self.macpmtcsr_reg().write(value);
        self
    }

    #[doc="Set the MACPMTCSR register."]
    #[inline] pub fn set_macpmtcsr<F: FnOnce(Macpmtcsr) -> Macpmtcsr>(&self, f: F) -> &Self {
        self.macpmtcsr_reg().set(f);
        self
    }

    #[doc="Modify the MACPMTCSR register."]
    #[inline] pub fn with_macpmtcsr<F: FnOnce(Macpmtcsr) -> Macpmtcsr>(&self, f: F) -> &Self {
        self.macpmtcsr_reg().with(f);
        self
    }

    #[doc="Get the MACDBGR Register."]
    #[inline] pub fn macdbgr_reg(&self) -> ::bobbin_mcu::register::Register<Macdbgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Macdbgr, 0x34)
    }

    #[doc="Get the *mut pointer for the MACDBGR register."]
    #[inline] pub fn macdbgr_mut(&self) -> *mut Macdbgr { 
        self.macdbgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACDBGR register."]
    #[inline] pub fn macdbgr_ptr(&self) -> *const Macdbgr { 
        self.macdbgr_reg().ptr()
    }

    #[doc="Read the MACDBGR register."]
    #[inline] pub fn macdbgr(&self) -> Macdbgr { 
        self.macdbgr_reg().read()
    }

    #[doc="Get the MACSR Register."]
    #[inline] pub fn macsr_reg(&self) -> ::bobbin_mcu::register::Register<Macsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Macsr, 0x38)
    }

    #[doc="Get the *mut pointer for the MACSR register."]
    #[inline] pub fn macsr_mut(&self) -> *mut Macsr { 
        self.macsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACSR register."]
    #[inline] pub fn macsr_ptr(&self) -> *const Macsr { 
        self.macsr_reg().ptr()
    }

    #[doc="Read the MACSR register."]
    #[inline] pub fn macsr(&self) -> Macsr { 
        self.macsr_reg().read()
    }

    #[doc="Write the MACSR register."]
    #[inline] pub fn write_macsr(&self, value: Macsr) -> &Self { 
        self.macsr_reg().write(value);
        self
    }

    #[doc="Set the MACSR register."]
    #[inline] pub fn set_macsr<F: FnOnce(Macsr) -> Macsr>(&self, f: F) -> &Self {
        self.macsr_reg().set(f);
        self
    }

    #[doc="Modify the MACSR register."]
    #[inline] pub fn with_macsr<F: FnOnce(Macsr) -> Macsr>(&self, f: F) -> &Self {
        self.macsr_reg().with(f);
        self
    }

    #[doc="Get the MACIMR Register."]
    #[inline] pub fn macimr_reg(&self) -> ::bobbin_mcu::register::Register<Macimr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Macimr, 0x3c)
    }

    #[doc="Get the *mut pointer for the MACIMR register."]
    #[inline] pub fn macimr_mut(&self) -> *mut Macimr { 
        self.macimr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACIMR register."]
    #[inline] pub fn macimr_ptr(&self) -> *const Macimr { 
        self.macimr_reg().ptr()
    }

    #[doc="Read the MACIMR register."]
    #[inline] pub fn macimr(&self) -> Macimr { 
        self.macimr_reg().read()
    }

    #[doc="Write the MACIMR register."]
    #[inline] pub fn write_macimr(&self, value: Macimr) -> &Self { 
        self.macimr_reg().write(value);
        self
    }

    #[doc="Set the MACIMR register."]
    #[inline] pub fn set_macimr<F: FnOnce(Macimr) -> Macimr>(&self, f: F) -> &Self {
        self.macimr_reg().set(f);
        self
    }

    #[doc="Modify the MACIMR register."]
    #[inline] pub fn with_macimr<F: FnOnce(Macimr) -> Macimr>(&self, f: F) -> &Self {
        self.macimr_reg().with(f);
        self
    }

    #[doc="Get the MACA0HR Register."]
    #[inline] pub fn maca0hr_reg(&self) -> ::bobbin_mcu::register::Register<Maca0hr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Maca0hr, 0x40)
    }

    #[doc="Get the *mut pointer for the MACA0HR register."]
    #[inline] pub fn maca0hr_mut(&self) -> *mut Maca0hr { 
        self.maca0hr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACA0HR register."]
    #[inline] pub fn maca0hr_ptr(&self) -> *const Maca0hr { 
        self.maca0hr_reg().ptr()
    }

    #[doc="Read the MACA0HR register."]
    #[inline] pub fn maca0hr(&self) -> Maca0hr { 
        self.maca0hr_reg().read()
    }

    #[doc="Write the MACA0HR register."]
    #[inline] pub fn write_maca0hr(&self, value: Maca0hr) -> &Self { 
        self.maca0hr_reg().write(value);
        self
    }

    #[doc="Set the MACA0HR register."]
    #[inline] pub fn set_maca0hr<F: FnOnce(Maca0hr) -> Maca0hr>(&self, f: F) -> &Self {
        self.maca0hr_reg().set(f);
        self
    }

    #[doc="Modify the MACA0HR register."]
    #[inline] pub fn with_maca0hr<F: FnOnce(Maca0hr) -> Maca0hr>(&self, f: F) -> &Self {
        self.maca0hr_reg().with(f);
        self
    }

    #[doc="Get the MACA0LR Register."]
    #[inline] pub fn maca0lr_reg(&self) -> ::bobbin_mcu::register::Register<Maca0lr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Maca0lr, 0x44)
    }

    #[doc="Get the *mut pointer for the MACA0LR register."]
    #[inline] pub fn maca0lr_mut(&self) -> *mut Maca0lr { 
        self.maca0lr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACA0LR register."]
    #[inline] pub fn maca0lr_ptr(&self) -> *const Maca0lr { 
        self.maca0lr_reg().ptr()
    }

    #[doc="Read the MACA0LR register."]
    #[inline] pub fn maca0lr(&self) -> Maca0lr { 
        self.maca0lr_reg().read()
    }

    #[doc="Write the MACA0LR register."]
    #[inline] pub fn write_maca0lr(&self, value: Maca0lr) -> &Self { 
        self.maca0lr_reg().write(value);
        self
    }

    #[doc="Set the MACA0LR register."]
    #[inline] pub fn set_maca0lr<F: FnOnce(Maca0lr) -> Maca0lr>(&self, f: F) -> &Self {
        self.maca0lr_reg().set(f);
        self
    }

    #[doc="Modify the MACA0LR register."]
    #[inline] pub fn with_maca0lr<F: FnOnce(Maca0lr) -> Maca0lr>(&self, f: F) -> &Self {
        self.maca0lr_reg().with(f);
        self
    }

    #[doc="Get the MACA1HR Register."]
    #[inline] pub fn maca1hr_reg(&self) -> ::bobbin_mcu::register::Register<Maca1hr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Maca1hr, 0x48)
    }

    #[doc="Get the *mut pointer for the MACA1HR register."]
    #[inline] pub fn maca1hr_mut(&self) -> *mut Maca1hr { 
        self.maca1hr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACA1HR register."]
    #[inline] pub fn maca1hr_ptr(&self) -> *const Maca1hr { 
        self.maca1hr_reg().ptr()
    }

    #[doc="Read the MACA1HR register."]
    #[inline] pub fn maca1hr(&self) -> Maca1hr { 
        self.maca1hr_reg().read()
    }

    #[doc="Write the MACA1HR register."]
    #[inline] pub fn write_maca1hr(&self, value: Maca1hr) -> &Self { 
        self.maca1hr_reg().write(value);
        self
    }

    #[doc="Set the MACA1HR register."]
    #[inline] pub fn set_maca1hr<F: FnOnce(Maca1hr) -> Maca1hr>(&self, f: F) -> &Self {
        self.maca1hr_reg().set(f);
        self
    }

    #[doc="Modify the MACA1HR register."]
    #[inline] pub fn with_maca1hr<F: FnOnce(Maca1hr) -> Maca1hr>(&self, f: F) -> &Self {
        self.maca1hr_reg().with(f);
        self
    }

    #[doc="Get the MACA1LR Register."]
    #[inline] pub fn maca1lr_reg(&self) -> ::bobbin_mcu::register::Register<Maca1lr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Maca1lr, 0x4c)
    }

    #[doc="Get the *mut pointer for the MACA1LR register."]
    #[inline] pub fn maca1lr_mut(&self) -> *mut Maca1lr { 
        self.maca1lr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACA1LR register."]
    #[inline] pub fn maca1lr_ptr(&self) -> *const Maca1lr { 
        self.maca1lr_reg().ptr()
    }

    #[doc="Read the MACA1LR register."]
    #[inline] pub fn maca1lr(&self) -> Maca1lr { 
        self.maca1lr_reg().read()
    }

    #[doc="Write the MACA1LR register."]
    #[inline] pub fn write_maca1lr(&self, value: Maca1lr) -> &Self { 
        self.maca1lr_reg().write(value);
        self
    }

    #[doc="Set the MACA1LR register."]
    #[inline] pub fn set_maca1lr<F: FnOnce(Maca1lr) -> Maca1lr>(&self, f: F) -> &Self {
        self.maca1lr_reg().set(f);
        self
    }

    #[doc="Modify the MACA1LR register."]
    #[inline] pub fn with_maca1lr<F: FnOnce(Maca1lr) -> Maca1lr>(&self, f: F) -> &Self {
        self.maca1lr_reg().with(f);
        self
    }

    #[doc="Get the MACA2HR Register."]
    #[inline] pub fn maca2hr_reg(&self) -> ::bobbin_mcu::register::Register<Maca2hr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Maca2hr, 0x50)
    }

    #[doc="Get the *mut pointer for the MACA2HR register."]
    #[inline] pub fn maca2hr_mut(&self) -> *mut Maca2hr { 
        self.maca2hr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACA2HR register."]
    #[inline] pub fn maca2hr_ptr(&self) -> *const Maca2hr { 
        self.maca2hr_reg().ptr()
    }

    #[doc="Read the MACA2HR register."]
    #[inline] pub fn maca2hr(&self) -> Maca2hr { 
        self.maca2hr_reg().read()
    }

    #[doc="Write the MACA2HR register."]
    #[inline] pub fn write_maca2hr(&self, value: Maca2hr) -> &Self { 
        self.maca2hr_reg().write(value);
        self
    }

    #[doc="Set the MACA2HR register."]
    #[inline] pub fn set_maca2hr<F: FnOnce(Maca2hr) -> Maca2hr>(&self, f: F) -> &Self {
        self.maca2hr_reg().set(f);
        self
    }

    #[doc="Modify the MACA2HR register."]
    #[inline] pub fn with_maca2hr<F: FnOnce(Maca2hr) -> Maca2hr>(&self, f: F) -> &Self {
        self.maca2hr_reg().with(f);
        self
    }

    #[doc="Get the MACA2LR Register."]
    #[inline] pub fn maca2lr_reg(&self) -> ::bobbin_mcu::register::Register<Maca2lr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Maca2lr, 0x54)
    }

    #[doc="Get the *mut pointer for the MACA2LR register."]
    #[inline] pub fn maca2lr_mut(&self) -> *mut Maca2lr { 
        self.maca2lr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACA2LR register."]
    #[inline] pub fn maca2lr_ptr(&self) -> *const Maca2lr { 
        self.maca2lr_reg().ptr()
    }

    #[doc="Read the MACA2LR register."]
    #[inline] pub fn maca2lr(&self) -> Maca2lr { 
        self.maca2lr_reg().read()
    }

    #[doc="Write the MACA2LR register."]
    #[inline] pub fn write_maca2lr(&self, value: Maca2lr) -> &Self { 
        self.maca2lr_reg().write(value);
        self
    }

    #[doc="Set the MACA2LR register."]
    #[inline] pub fn set_maca2lr<F: FnOnce(Maca2lr) -> Maca2lr>(&self, f: F) -> &Self {
        self.maca2lr_reg().set(f);
        self
    }

    #[doc="Modify the MACA2LR register."]
    #[inline] pub fn with_maca2lr<F: FnOnce(Maca2lr) -> Maca2lr>(&self, f: F) -> &Self {
        self.maca2lr_reg().with(f);
        self
    }

    #[doc="Get the MACA3HR Register."]
    #[inline] pub fn maca3hr_reg(&self) -> ::bobbin_mcu::register::Register<Maca3hr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Maca3hr, 0x58)
    }

    #[doc="Get the *mut pointer for the MACA3HR register."]
    #[inline] pub fn maca3hr_mut(&self) -> *mut Maca3hr { 
        self.maca3hr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACA3HR register."]
    #[inline] pub fn maca3hr_ptr(&self) -> *const Maca3hr { 
        self.maca3hr_reg().ptr()
    }

    #[doc="Read the MACA3HR register."]
    #[inline] pub fn maca3hr(&self) -> Maca3hr { 
        self.maca3hr_reg().read()
    }

    #[doc="Write the MACA3HR register."]
    #[inline] pub fn write_maca3hr(&self, value: Maca3hr) -> &Self { 
        self.maca3hr_reg().write(value);
        self
    }

    #[doc="Set the MACA3HR register."]
    #[inline] pub fn set_maca3hr<F: FnOnce(Maca3hr) -> Maca3hr>(&self, f: F) -> &Self {
        self.maca3hr_reg().set(f);
        self
    }

    #[doc="Modify the MACA3HR register."]
    #[inline] pub fn with_maca3hr<F: FnOnce(Maca3hr) -> Maca3hr>(&self, f: F) -> &Self {
        self.maca3hr_reg().with(f);
        self
    }

    #[doc="Get the MACA3LR Register."]
    #[inline] pub fn maca3lr_reg(&self) -> ::bobbin_mcu::register::Register<Maca3lr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Maca3lr, 0x5c)
    }

    #[doc="Get the *mut pointer for the MACA3LR register."]
    #[inline] pub fn maca3lr_mut(&self) -> *mut Maca3lr { 
        self.maca3lr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MACA3LR register."]
    #[inline] pub fn maca3lr_ptr(&self) -> *const Maca3lr { 
        self.maca3lr_reg().ptr()
    }

    #[doc="Read the MACA3LR register."]
    #[inline] pub fn maca3lr(&self) -> Maca3lr { 
        self.maca3lr_reg().read()
    }

    #[doc="Write the MACA3LR register."]
    #[inline] pub fn write_maca3lr(&self, value: Maca3lr) -> &Self { 
        self.maca3lr_reg().write(value);
        self
    }

    #[doc="Set the MACA3LR register."]
    #[inline] pub fn set_maca3lr<F: FnOnce(Maca3lr) -> Maca3lr>(&self, f: F) -> &Self {
        self.maca3lr_reg().set(f);
        self
    }

    #[doc="Modify the MACA3LR register."]
    #[inline] pub fn with_maca3lr<F: FnOnce(Maca3lr) -> Maca3lr>(&self, f: F) -> &Self {
        self.maca3lr_reg().with(f);
        self
    }

}

#[doc="Ethernet MAC configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Maccr(pub u32);
impl Maccr {
    #[doc="RE"]
    #[inline] pub fn re(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RE != 0"]
    #[inline] pub fn test_re(&self) -> bool {
        self.re() != 0
    }

    #[doc="Sets the RE field."]
    #[inline] pub fn set_re<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TE"]
    #[inline] pub fn te(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TE != 0"]
    #[inline] pub fn test_te(&self) -> bool {
        self.te() != 0
    }

    #[doc="Sets the TE field."]
    #[inline] pub fn set_te<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DC"]
    #[inline] pub fn dc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DC != 0"]
    #[inline] pub fn test_dc(&self) -> bool {
        self.dc() != 0
    }

    #[doc="Sets the DC field."]
    #[inline] pub fn set_dc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="BL"]
    #[inline] pub fn bl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if BL != 0"]
    #[inline] pub fn test_bl(&self) -> bool {
        self.bl() != 0
    }

    #[doc="Sets the BL field."]
    #[inline] pub fn set_bl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="APCS"]
    #[inline] pub fn apcs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if APCS != 0"]
    #[inline] pub fn test_apcs(&self) -> bool {
        self.apcs() != 0
    }

    #[doc="Sets the APCS field."]
    #[inline] pub fn set_apcs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="RD"]
    #[inline] pub fn rd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RD != 0"]
    #[inline] pub fn test_rd(&self) -> bool {
        self.rd() != 0
    }

    #[doc="Sets the RD field."]
    #[inline] pub fn set_rd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="IPCO"]
    #[inline] pub fn ipco(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if IPCO != 0"]
    #[inline] pub fn test_ipco(&self) -> bool {
        self.ipco() != 0
    }

    #[doc="Sets the IPCO field."]
    #[inline] pub fn set_ipco<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="DM"]
    #[inline] pub fn dm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DM != 0"]
    #[inline] pub fn test_dm(&self) -> bool {
        self.dm() != 0
    }

    #[doc="Sets the DM field."]
    #[inline] pub fn set_dm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="LM"]
    #[inline] pub fn lm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if LM != 0"]
    #[inline] pub fn test_lm(&self) -> bool {
        self.lm() != 0
    }

    #[doc="Sets the LM field."]
    #[inline] pub fn set_lm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="ROD"]
    #[inline] pub fn rod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ROD != 0"]
    #[inline] pub fn test_rod(&self) -> bool {
        self.rod() != 0
    }

    #[doc="Sets the ROD field."]
    #[inline] pub fn set_rod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="FES"]
    #[inline] pub fn fes(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FES != 0"]
    #[inline] pub fn test_fes(&self) -> bool {
        self.fes() != 0
    }

    #[doc="Sets the FES field."]
    #[inline] pub fn set_fes<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="CSD"]
    #[inline] pub fn csd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CSD != 0"]
    #[inline] pub fn test_csd(&self) -> bool {
        self.csd() != 0
    }

    #[doc="Sets the CSD field."]
    #[inline] pub fn set_csd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="IFG"]
    #[inline] pub fn ifg(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
    }

    #[doc="Returns true if IFG != 0"]
    #[inline] pub fn test_ifg(&self) -> bool {
        self.ifg() != 0
    }

    #[doc="Sets the IFG field."]
    #[inline] pub fn set_ifg<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="JD"]
    #[inline] pub fn jd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if JD != 0"]
    #[inline] pub fn test_jd(&self) -> bool {
        self.jd() != 0
    }

    #[doc="Sets the JD field."]
    #[inline] pub fn set_jd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="WD"]
    #[inline] pub fn wd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if WD != 0"]
    #[inline] pub fn test_wd(&self) -> bool {
        self.wd() != 0
    }

    #[doc="Sets the WD field."]
    #[inline] pub fn set_wd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="CSTF"]
    #[inline] pub fn cstf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CSTF != 0"]
    #[inline] pub fn test_cstf(&self) -> bool {
        self.cstf() != 0
    }

    #[doc="Sets the CSTF field."]
    #[inline] pub fn set_cstf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

}

impl From<u32> for Maccr {
    #[inline]
    fn from(other: u32) -> Self {
         Maccr(other)
    }
}

impl ::core::fmt::Display for Maccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Maccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.re() != 0 { try!(write!(f, " re"))}
        if self.te() != 0 { try!(write!(f, " te"))}
        if self.dc() != 0 { try!(write!(f, " dc"))}
        if self.bl() != 0 { try!(write!(f, " bl=0x{:x}", self.bl()))}
        if self.apcs() != 0 { try!(write!(f, " apcs"))}
        if self.rd() != 0 { try!(write!(f, " rd"))}
        if self.ipco() != 0 { try!(write!(f, " ipco"))}
        if self.dm() != 0 { try!(write!(f, " dm"))}
        if self.lm() != 0 { try!(write!(f, " lm"))}
        if self.rod() != 0 { try!(write!(f, " rod"))}
        if self.fes() != 0 { try!(write!(f, " fes"))}
        if self.csd() != 0 { try!(write!(f, " csd"))}
        if self.ifg() != 0 { try!(write!(f, " ifg=0x{:x}", self.ifg()))}
        if self.jd() != 0 { try!(write!(f, " jd"))}
        if self.wd() != 0 { try!(write!(f, " wd"))}
        if self.cstf() != 0 { try!(write!(f, " cstf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC frame filter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Macffr(pub u32);
impl Macffr {
    #[doc="no description available"]
    #[inline] pub fn pm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PM != 0"]
    #[inline] pub fn test_pm(&self) -> bool {
        self.pm() != 0
    }

    #[doc="Sets the PM field."]
    #[inline] pub fn set_pm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn hu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HU != 0"]
    #[inline] pub fn test_hu(&self) -> bool {
        self.hu() != 0
    }

    #[doc="Sets the HU field."]
    #[inline] pub fn set_hu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn hm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HM != 0"]
    #[inline] pub fn test_hm(&self) -> bool {
        self.hm() != 0
    }

    #[doc="Sets the HM field."]
    #[inline] pub fn set_hm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn daif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DAIF != 0"]
    #[inline] pub fn test_daif(&self) -> bool {
        self.daif() != 0
    }

    #[doc="Sets the DAIF field."]
    #[inline] pub fn set_daif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ram(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RAM != 0"]
    #[inline] pub fn test_ram(&self) -> bool {
        self.ram() != 0
    }

    #[doc="Sets the RAM field."]
    #[inline] pub fn set_ram<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn bfd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BFD != 0"]
    #[inline] pub fn test_bfd(&self) -> bool {
        self.bfd() != 0
    }

    #[doc="Sets the BFD field."]
    #[inline] pub fn set_bfd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn pcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PCF != 0"]
    #[inline] pub fn test_pcf(&self) -> bool {
        self.pcf() != 0
    }

    #[doc="Sets the PCF field."]
    #[inline] pub fn set_pcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn saif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SAIF != 0"]
    #[inline] pub fn test_saif(&self) -> bool {
        self.saif() != 0
    }

    #[doc="Sets the SAIF field."]
    #[inline] pub fn set_saif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn saf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SAF != 0"]
    #[inline] pub fn test_saf(&self) -> bool {
        self.saf() != 0
    }

    #[doc="Sets the SAF field."]
    #[inline] pub fn set_saf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn hpf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HPF != 0"]
    #[inline] pub fn test_hpf(&self) -> bool {
        self.hpf() != 0
    }

    #[doc="Sets the HPF field."]
    #[inline] pub fn set_hpf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ra(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if RA != 0"]
    #[inline] pub fn test_ra(&self) -> bool {
        self.ra() != 0
    }

    #[doc="Sets the RA field."]
    #[inline] pub fn set_ra<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Macffr {
    #[inline]
    fn from(other: u32) -> Self {
         Macffr(other)
    }
}

impl ::core::fmt::Display for Macffr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Macffr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pm() != 0 { try!(write!(f, " pm"))}
        if self.hu() != 0 { try!(write!(f, " hu"))}
        if self.hm() != 0 { try!(write!(f, " hm"))}
        if self.daif() != 0 { try!(write!(f, " daif"))}
        if self.ram() != 0 { try!(write!(f, " ram"))}
        if self.bfd() != 0 { try!(write!(f, " bfd"))}
        if self.pcf() != 0 { try!(write!(f, " pcf"))}
        if self.saif() != 0 { try!(write!(f, " saif"))}
        if self.saf() != 0 { try!(write!(f, " saf"))}
        if self.hpf() != 0 { try!(write!(f, " hpf"))}
        if self.ra() != 0 { try!(write!(f, " ra"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC hash table high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Machthr(pub u32);
impl Machthr {
    #[doc="no description available"]
    #[inline] pub fn hth(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if HTH != 0"]
    #[inline] pub fn test_hth(&self) -> bool {
        self.hth() != 0
    }

    #[doc="Sets the HTH field."]
    #[inline] pub fn set_hth<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Machthr {
    #[inline]
    fn from(other: u32) -> Self {
         Machthr(other)
    }
}

impl ::core::fmt::Display for Machthr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Machthr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC hash table low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Machtlr(pub u32);
impl Machtlr {
    #[doc="no description available"]
    #[inline] pub fn htl(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if HTL != 0"]
    #[inline] pub fn test_htl(&self) -> bool {
        self.htl() != 0
    }

    #[doc="Sets the HTL field."]
    #[inline] pub fn set_htl<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Machtlr {
    #[inline]
    fn from(other: u32) -> Self {
         Machtlr(other)
    }
}

impl ::core::fmt::Display for Machtlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Machtlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC MII address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Macmiiar(pub u32);
impl Macmiiar {
    #[doc="no description available"]
    #[inline] pub fn mb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MB != 0"]
    #[inline] pub fn test_mb(&self) -> bool {
        self.mb() != 0
    }

    #[doc="Sets the MB field."]
    #[inline] pub fn set_mb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MW != 0"]
    #[inline] pub fn test_mw(&self) -> bool {
        self.mw() != 0
    }

    #[doc="Sets the MW field."]
    #[inline] pub fn set_mw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn cr(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
    }

    #[doc="Returns true if CR != 0"]
    #[inline] pub fn test_cr(&self) -> bool {
        self.cr() != 0
    }

    #[doc="Sets the CR field."]
    #[inline] pub fn set_cr<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mr(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
    }

    #[doc="Returns true if MR != 0"]
    #[inline] pub fn test_mr(&self) -> bool {
        self.mr() != 0
    }

    #[doc="Sets the MR field."]
    #[inline] pub fn set_mr<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn pa(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1f) as u8) } // [15:11]
    }

    #[doc="Returns true if PA != 0"]
    #[inline] pub fn test_pa(&self) -> bool {
        self.pa() != 0
    }

    #[doc="Sets the PA field."]
    #[inline] pub fn set_pa<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Macmiiar {
    #[inline]
    fn from(other: u32) -> Self {
         Macmiiar(other)
    }
}

impl ::core::fmt::Display for Macmiiar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Macmiiar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mb() != 0 { try!(write!(f, " mb"))}
        if self.mw() != 0 { try!(write!(f, " mw"))}
        if self.cr() != 0 { try!(write!(f, " cr=0x{:x}", self.cr()))}
        if self.mr() != 0 { try!(write!(f, " mr=0x{:x}", self.mr()))}
        if self.pa() != 0 { try!(write!(f, " pa=0x{:x}", self.pa()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC MII data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Macmiidr(pub u32);
impl Macmiidr {
    #[doc="no description available"]
    #[inline] pub fn td(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if TD != 0"]
    #[inline] pub fn test_td(&self) -> bool {
        self.td() != 0
    }

    #[doc="Sets the TD field."]
    #[inline] pub fn set_td<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Macmiidr {
    #[inline]
    fn from(other: u32) -> Self {
         Macmiidr(other)
    }
}

impl ::core::fmt::Display for Macmiidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Macmiidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.td() != 0 { try!(write!(f, " td=0x{:x}", self.td()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC flow control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Macfcr(pub u32);
impl Macfcr {
    #[doc="no description available"]
    #[inline] pub fn fcb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FCB != 0"]
    #[inline] pub fn test_fcb(&self) -> bool {
        self.fcb() != 0
    }

    #[doc="Sets the FCB field."]
    #[inline] pub fn set_fcb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tfce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TFCE != 0"]
    #[inline] pub fn test_tfce(&self) -> bool {
        self.tfce() != 0
    }

    #[doc="Sets the TFCE field."]
    #[inline] pub fn set_tfce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn rfce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RFCE != 0"]
    #[inline] pub fn test_rfce(&self) -> bool {
        self.rfce() != 0
    }

    #[doc="Sets the RFCE field."]
    #[inline] pub fn set_rfce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn upfd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if UPFD != 0"]
    #[inline] pub fn test_upfd(&self) -> bool {
        self.upfd() != 0
    }

    #[doc="Sets the UPFD field."]
    #[inline] pub fn set_upfd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn plt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if PLT != 0"]
    #[inline] pub fn test_plt(&self) -> bool {
        self.plt() != 0
    }

    #[doc="Sets the PLT field."]
    #[inline] pub fn set_plt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn zqpd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ZQPD != 0"]
    #[inline] pub fn test_zqpd(&self) -> bool {
        self.zqpd() != 0
    }

    #[doc="Sets the ZQPD field."]
    #[inline] pub fn set_zqpd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn pt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if PT != 0"]
    #[inline] pub fn test_pt(&self) -> bool {
        self.pt() != 0
    }

    #[doc="Sets the PT field."]
    #[inline] pub fn set_pt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Macfcr {
    #[inline]
    fn from(other: u32) -> Self {
         Macfcr(other)
    }
}

impl ::core::fmt::Display for Macfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Macfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fcb() != 0 { try!(write!(f, " fcb"))}
        if self.tfce() != 0 { try!(write!(f, " tfce"))}
        if self.rfce() != 0 { try!(write!(f, " rfce"))}
        if self.upfd() != 0 { try!(write!(f, " upfd"))}
        if self.plt() != 0 { try!(write!(f, " plt=0x{:x}", self.plt()))}
        if self.zqpd() != 0 { try!(write!(f, " zqpd"))}
        if self.pt() != 0 { try!(write!(f, " pt=0x{:x}", self.pt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC VLAN tag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Macvlantr(pub u32);
impl Macvlantr {
    #[doc="no description available"]
    #[inline] pub fn vlanti(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if VLANTI != 0"]
    #[inline] pub fn test_vlanti(&self) -> bool {
        self.vlanti() != 0
    }

    #[doc="Sets the VLANTI field."]
    #[inline] pub fn set_vlanti<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn vlantc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if VLANTC != 0"]
    #[inline] pub fn test_vlantc(&self) -> bool {
        self.vlantc() != 0
    }

    #[doc="Sets the VLANTC field."]
    #[inline] pub fn set_vlantc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Macvlantr {
    #[inline]
    fn from(other: u32) -> Self {
         Macvlantr(other)
    }
}

impl ::core::fmt::Display for Macvlantr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Macvlantr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vlanti() != 0 { try!(write!(f, " vlanti=0x{:x}", self.vlanti()))}
        if self.vlantc() != 0 { try!(write!(f, " vlantc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC PMT control and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Macpmtcsr(pub u32);
impl Macpmtcsr {
    #[doc="no description available"]
    #[inline] pub fn pd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PD != 0"]
    #[inline] pub fn test_pd(&self) -> bool {
        self.pd() != 0
    }

    #[doc="Sets the PD field."]
    #[inline] pub fn set_pd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mpe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MPE != 0"]
    #[inline] pub fn test_mpe(&self) -> bool {
        self.mpe() != 0
    }

    #[doc="Sets the MPE field."]
    #[inline] pub fn set_mpe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn wfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WFE != 0"]
    #[inline] pub fn test_wfe(&self) -> bool {
        self.wfe() != 0
    }

    #[doc="Sets the WFE field."]
    #[inline] pub fn set_wfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mpr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MPR != 0"]
    #[inline] pub fn test_mpr(&self) -> bool {
        self.mpr() != 0
    }

    #[doc="Sets the MPR field."]
    #[inline] pub fn set_mpr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn wfr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if WFR != 0"]
    #[inline] pub fn test_wfr(&self) -> bool {
        self.wfr() != 0
    }

    #[doc="Sets the WFR field."]
    #[inline] pub fn set_wfr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn gu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GU != 0"]
    #[inline] pub fn test_gu(&self) -> bool {
        self.gu() != 0
    }

    #[doc="Sets the GU field."]
    #[inline] pub fn set_gu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn wffrpr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if WFFRPR != 0"]
    #[inline] pub fn test_wffrpr(&self) -> bool {
        self.wffrpr() != 0
    }

    #[doc="Sets the WFFRPR field."]
    #[inline] pub fn set_wffrpr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Macpmtcsr {
    #[inline]
    fn from(other: u32) -> Self {
         Macpmtcsr(other)
    }
}

impl ::core::fmt::Display for Macpmtcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Macpmtcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pd() != 0 { try!(write!(f, " pd"))}
        if self.mpe() != 0 { try!(write!(f, " mpe"))}
        if self.wfe() != 0 { try!(write!(f, " wfe"))}
        if self.mpr() != 0 { try!(write!(f, " mpr"))}
        if self.wfr() != 0 { try!(write!(f, " wfr"))}
        if self.gu() != 0 { try!(write!(f, " gu"))}
        if self.wffrpr() != 0 { try!(write!(f, " wffrpr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC debug register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Macdbgr(pub u32);
impl Macdbgr {
    #[doc="CR"]
    #[inline] pub fn cr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CR != 0"]
    #[inline] pub fn test_cr(&self) -> bool {
        self.cr() != 0
    }

    #[doc="Sets the CR field."]
    #[inline] pub fn set_cr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CSR"]
    #[inline] pub fn csr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CSR != 0"]
    #[inline] pub fn test_csr(&self) -> bool {
        self.csr() != 0
    }

    #[doc="Sets the CSR field."]
    #[inline] pub fn set_csr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="ROR"]
    #[inline] pub fn ror(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ROR != 0"]
    #[inline] pub fn test_ror(&self) -> bool {
        self.ror() != 0
    }

    #[doc="Sets the ROR field."]
    #[inline] pub fn set_ror<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="MCF"]
    #[inline] pub fn mcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if MCF != 0"]
    #[inline] pub fn test_mcf(&self) -> bool {
        self.mcf() != 0
    }

    #[doc="Sets the MCF field."]
    #[inline] pub fn set_mcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="MCP"]
    #[inline] pub fn mcp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MCP != 0"]
    #[inline] pub fn test_mcp(&self) -> bool {
        self.mcp() != 0
    }

    #[doc="Sets the MCP field."]
    #[inline] pub fn set_mcp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="MCFHP"]
    #[inline] pub fn mcfhp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MCFHP != 0"]
    #[inline] pub fn test_mcfhp(&self) -> bool {
        self.mcfhp() != 0
    }

    #[doc="Sets the MCFHP field."]
    #[inline] pub fn set_mcfhp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Macdbgr {
    #[inline]
    fn from(other: u32) -> Self {
         Macdbgr(other)
    }
}

impl ::core::fmt::Display for Macdbgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Macdbgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cr() != 0 { try!(write!(f, " cr"))}
        if self.csr() != 0 { try!(write!(f, " csr"))}
        if self.ror() != 0 { try!(write!(f, " ror"))}
        if self.mcf() != 0 { try!(write!(f, " mcf"))}
        if self.mcp() != 0 { try!(write!(f, " mcp"))}
        if self.mcfhp() != 0 { try!(write!(f, " mcfhp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Macsr(pub u32);
impl Macsr {
    #[doc="no description available"]
    #[inline] pub fn pmts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PMTS != 0"]
    #[inline] pub fn test_pmts(&self) -> bool {
        self.pmts() != 0
    }

    #[doc="Sets the PMTS field."]
    #[inline] pub fn set_pmts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mmcs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MMCS != 0"]
    #[inline] pub fn test_mmcs(&self) -> bool {
        self.mmcs() != 0
    }

    #[doc="Sets the MMCS field."]
    #[inline] pub fn set_mmcs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mmcrs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MMCRS != 0"]
    #[inline] pub fn test_mmcrs(&self) -> bool {
        self.mmcrs() != 0
    }

    #[doc="Sets the MMCRS field."]
    #[inline] pub fn set_mmcrs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mmcts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if MMCTS != 0"]
    #[inline] pub fn test_mmcts(&self) -> bool {
        self.mmcts() != 0
    }

    #[doc="Sets the MMCTS field."]
    #[inline] pub fn set_mmcts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TSTS != 0"]
    #[inline] pub fn test_tsts(&self) -> bool {
        self.tsts() != 0
    }

    #[doc="Sets the TSTS field."]
    #[inline] pub fn set_tsts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Macsr {
    #[inline]
    fn from(other: u32) -> Self {
         Macsr(other)
    }
}

impl ::core::fmt::Display for Macsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Macsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pmts() != 0 { try!(write!(f, " pmts"))}
        if self.mmcs() != 0 { try!(write!(f, " mmcs"))}
        if self.mmcrs() != 0 { try!(write!(f, " mmcrs"))}
        if self.mmcts() != 0 { try!(write!(f, " mmcts"))}
        if self.tsts() != 0 { try!(write!(f, " tsts"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC interrupt mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Macimr(pub u32);
impl Macimr {
    #[doc="no description available"]
    #[inline] pub fn pmtim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PMTIM != 0"]
    #[inline] pub fn test_pmtim(&self) -> bool {
        self.pmtim() != 0
    }

    #[doc="Sets the PMTIM field."]
    #[inline] pub fn set_pmtim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tstim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TSTIM != 0"]
    #[inline] pub fn test_tstim(&self) -> bool {
        self.tstim() != 0
    }

    #[doc="Sets the TSTIM field."]
    #[inline] pub fn set_tstim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Macimr {
    #[inline]
    fn from(other: u32) -> Self {
         Macimr(other)
    }
}

impl ::core::fmt::Display for Macimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Macimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pmtim() != 0 { try!(write!(f, " pmtim"))}
        if self.tstim() != 0 { try!(write!(f, " tstim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC address 0 high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Maca0hr(pub u32);
impl Maca0hr {
    #[doc="MAC address0 high"]
    #[inline] pub fn maca0h(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MACA0H != 0"]
    #[inline] pub fn test_maca0h(&self) -> bool {
        self.maca0h() != 0
    }

    #[doc="Sets the MACA0H field."]
    #[inline] pub fn set_maca0h<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Always 1"]
    #[inline] pub fn mo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if MO != 0"]
    #[inline] pub fn test_mo(&self) -> bool {
        self.mo() != 0
    }

    #[doc="Sets the MO field."]
    #[inline] pub fn set_mo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Maca0hr {
    #[inline]
    fn from(other: u32) -> Self {
         Maca0hr(other)
    }
}

impl ::core::fmt::Display for Maca0hr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Maca0hr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.maca0h() != 0 { try!(write!(f, " maca0h=0x{:x}", self.maca0h()))}
        if self.mo() != 0 { try!(write!(f, " mo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC address 0 low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Maca0lr(pub u32);
impl Maca0lr {
    #[doc="0"]
    #[inline] pub fn maca0l(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if MACA0L != 0"]
    #[inline] pub fn test_maca0l(&self) -> bool {
        self.maca0l() != 0
    }

    #[doc="Sets the MACA0L field."]
    #[inline] pub fn set_maca0l<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Maca0lr {
    #[inline]
    fn from(other: u32) -> Self {
         Maca0lr(other)
    }
}

impl ::core::fmt::Display for Maca0lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Maca0lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC address 1 high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Maca1hr(pub u32);
impl Maca1hr {
    #[doc="no description available"]
    #[inline] pub fn maca1h(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MACA1H != 0"]
    #[inline] pub fn test_maca1h(&self) -> bool {
        self.maca1h() != 0
    }

    #[doc="Sets the MACA1H field."]
    #[inline] pub fn set_maca1h<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mbc(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if MBC != 0"]
    #[inline] pub fn test_mbc(&self) -> bool {
        self.mbc() != 0
    }

    #[doc="Sets the MBC field."]
    #[inline] pub fn set_mbc<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn sa(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if SA != 0"]
    #[inline] pub fn test_sa(&self) -> bool {
        self.sa() != 0
    }

    #[doc="Sets the SA field."]
    #[inline] pub fn set_sa<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if AE != 0"]
    #[inline] pub fn test_ae(&self) -> bool {
        self.ae() != 0
    }

    #[doc="Sets the AE field."]
    #[inline] pub fn set_ae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Maca1hr {
    #[inline]
    fn from(other: u32) -> Self {
         Maca1hr(other)
    }
}

impl ::core::fmt::Display for Maca1hr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Maca1hr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.maca1h() != 0 { try!(write!(f, " maca1h=0x{:x}", self.maca1h()))}
        if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
        if self.sa() != 0 { try!(write!(f, " sa"))}
        if self.ae() != 0 { try!(write!(f, " ae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC address1 low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Maca1lr(pub u32);
impl Maca1lr {
    #[doc="no description available"]
    #[inline] pub fn maca1lr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if MACA1LR != 0"]
    #[inline] pub fn test_maca1lr(&self) -> bool {
        self.maca1lr() != 0
    }

    #[doc="Sets the MACA1LR field."]
    #[inline] pub fn set_maca1lr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Maca1lr {
    #[inline]
    fn from(other: u32) -> Self {
         Maca1lr(other)
    }
}

impl ::core::fmt::Display for Maca1lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Maca1lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC address 2 high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Maca2hr(pub u32);
impl Maca2hr {
    #[doc="no description available"]
    #[inline] pub fn mac2ah(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MAC2AH != 0"]
    #[inline] pub fn test_mac2ah(&self) -> bool {
        self.mac2ah() != 0
    }

    #[doc="Sets the MAC2AH field."]
    #[inline] pub fn set_mac2ah<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mbc(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if MBC != 0"]
    #[inline] pub fn test_mbc(&self) -> bool {
        self.mbc() != 0
    }

    #[doc="Sets the MBC field."]
    #[inline] pub fn set_mbc<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn sa(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if SA != 0"]
    #[inline] pub fn test_sa(&self) -> bool {
        self.sa() != 0
    }

    #[doc="Sets the SA field."]
    #[inline] pub fn set_sa<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if AE != 0"]
    #[inline] pub fn test_ae(&self) -> bool {
        self.ae() != 0
    }

    #[doc="Sets the AE field."]
    #[inline] pub fn set_ae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Maca2hr {
    #[inline]
    fn from(other: u32) -> Self {
         Maca2hr(other)
    }
}

impl ::core::fmt::Display for Maca2hr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Maca2hr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mac2ah() != 0 { try!(write!(f, " mac2ah=0x{:x}", self.mac2ah()))}
        if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
        if self.sa() != 0 { try!(write!(f, " sa"))}
        if self.ae() != 0 { try!(write!(f, " ae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC address 2 low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Maca2lr(pub u32);
impl Maca2lr {
    #[doc="no description available"]
    #[inline] pub fn maca2l(&self) -> ::bobbin_bits::U31 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fffffff) as u32) } // [30:0]
    }

    #[doc="Returns true if MACA2L != 0"]
    #[inline] pub fn test_maca2l(&self) -> bool {
        self.maca2l() != 0
    }

    #[doc="Sets the MACA2L field."]
    #[inline] pub fn set_maca2l<V: Into<::bobbin_bits::U31>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U31 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Maca2lr {
    #[inline]
    fn from(other: u32) -> Self {
         Maca2lr(other)
    }
}

impl ::core::fmt::Display for Maca2lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Maca2lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.maca2l() != 0 { try!(write!(f, " maca2l=0x{:x}", self.maca2l()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC address 3 high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Maca3hr(pub u32);
impl Maca3hr {
    #[doc="no description available"]
    #[inline] pub fn maca3h(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MACA3H != 0"]
    #[inline] pub fn test_maca3h(&self) -> bool {
        self.maca3h() != 0
    }

    #[doc="Sets the MACA3H field."]
    #[inline] pub fn set_maca3h<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn mbc(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if MBC != 0"]
    #[inline] pub fn test_mbc(&self) -> bool {
        self.mbc() != 0
    }

    #[doc="Sets the MBC field."]
    #[inline] pub fn set_mbc<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn sa(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if SA != 0"]
    #[inline] pub fn test_sa(&self) -> bool {
        self.sa() != 0
    }

    #[doc="Sets the SA field."]
    #[inline] pub fn set_sa<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if AE != 0"]
    #[inline] pub fn test_ae(&self) -> bool {
        self.ae() != 0
    }

    #[doc="Sets the AE field."]
    #[inline] pub fn set_ae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Maca3hr {
    #[inline]
    fn from(other: u32) -> Self {
         Maca3hr(other)
    }
}

impl ::core::fmt::Display for Maca3hr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Maca3hr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.maca3h() != 0 { try!(write!(f, " maca3h=0x{:x}", self.maca3h()))}
        if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
        if self.sa() != 0 { try!(write!(f, " sa"))}
        if self.ae() != 0 { try!(write!(f, " ae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet MAC address 3 low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Maca3lr(pub u32);
impl Maca3lr {
    #[doc="no description available"]
    #[inline] pub fn mbca3l(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if MBCA3L != 0"]
    #[inline] pub fn test_mbca3l(&self) -> bool {
        self.mbca3l() != 0
    }

    #[doc="Sets the MBCA3L field."]
    #[inline] pub fn set_mbca3l<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Maca3lr {
    #[inline]
    fn from(other: u32) -> Self {
         Maca3lr(other)
    }
}

impl ::core::fmt::Display for Maca3lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Maca3lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

