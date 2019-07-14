::bobbin_mcu::periph!( USB, Usb, USB_PERIPH, UsbPeriph, USB_OWNED, USB_REF_COUNT, 0x40006800, 0x00, 0x29);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USB Peripheral"]
pub struct UsbPeriph(pub usize); 

impl UsbPeriph {
    #[doc="Get the EP0R Register."]
    #[inline] pub fn ep0r_reg(&self) -> ::bobbin_mcu::register::Register<Ep0r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ep0r, 0x0)
    }

    #[doc="Get the *mut pointer for the EP0R register."]
    #[inline] pub fn ep0r_mut(&self) -> *mut Ep0r { 
        self.ep0r_reg().ptr()
    }

    #[doc="Get the *const pointer for the EP0R register."]
    #[inline] pub fn ep0r_ptr(&self) -> *const Ep0r { 
        self.ep0r_reg().ptr()
    }

    #[doc="Read the EP0R register."]
    #[inline] pub fn ep0r(&self) -> Ep0r { 
        self.ep0r_reg().read()
    }

    #[doc="Write the EP0R register."]
    #[inline] pub fn write_ep0r(&self, value: Ep0r) -> &Self { 
        self.ep0r_reg().write(value);
        self
    }

    #[doc="Set the EP0R register."]
    #[inline] pub fn set_ep0r<F: FnOnce(Ep0r) -> Ep0r>(&self, f: F) -> &Self {
        self.ep0r_reg().set(f);
        self
    }

    #[doc="Modify the EP0R register."]
    #[inline] pub fn with_ep0r<F: FnOnce(Ep0r) -> Ep0r>(&self, f: F) -> &Self {
        self.ep0r_reg().with(f);
        self
    }

    #[doc="Get the EP1R Register."]
    #[inline] pub fn ep1r_reg(&self) -> ::bobbin_mcu::register::Register<Ep1r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ep1r, 0x4)
    }

    #[doc="Get the *mut pointer for the EP1R register."]
    #[inline] pub fn ep1r_mut(&self) -> *mut Ep1r { 
        self.ep1r_reg().ptr()
    }

    #[doc="Get the *const pointer for the EP1R register."]
    #[inline] pub fn ep1r_ptr(&self) -> *const Ep1r { 
        self.ep1r_reg().ptr()
    }

    #[doc="Read the EP1R register."]
    #[inline] pub fn ep1r(&self) -> Ep1r { 
        self.ep1r_reg().read()
    }

    #[doc="Write the EP1R register."]
    #[inline] pub fn write_ep1r(&self, value: Ep1r) -> &Self { 
        self.ep1r_reg().write(value);
        self
    }

    #[doc="Set the EP1R register."]
    #[inline] pub fn set_ep1r<F: FnOnce(Ep1r) -> Ep1r>(&self, f: F) -> &Self {
        self.ep1r_reg().set(f);
        self
    }

    #[doc="Modify the EP1R register."]
    #[inline] pub fn with_ep1r<F: FnOnce(Ep1r) -> Ep1r>(&self, f: F) -> &Self {
        self.ep1r_reg().with(f);
        self
    }

    #[doc="Get the EP2R Register."]
    #[inline] pub fn ep2r_reg(&self) -> ::bobbin_mcu::register::Register<Ep2r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ep2r, 0x8)
    }

    #[doc="Get the *mut pointer for the EP2R register."]
    #[inline] pub fn ep2r_mut(&self) -> *mut Ep2r { 
        self.ep2r_reg().ptr()
    }

    #[doc="Get the *const pointer for the EP2R register."]
    #[inline] pub fn ep2r_ptr(&self) -> *const Ep2r { 
        self.ep2r_reg().ptr()
    }

    #[doc="Read the EP2R register."]
    #[inline] pub fn ep2r(&self) -> Ep2r { 
        self.ep2r_reg().read()
    }

    #[doc="Write the EP2R register."]
    #[inline] pub fn write_ep2r(&self, value: Ep2r) -> &Self { 
        self.ep2r_reg().write(value);
        self
    }

    #[doc="Set the EP2R register."]
    #[inline] pub fn set_ep2r<F: FnOnce(Ep2r) -> Ep2r>(&self, f: F) -> &Self {
        self.ep2r_reg().set(f);
        self
    }

    #[doc="Modify the EP2R register."]
    #[inline] pub fn with_ep2r<F: FnOnce(Ep2r) -> Ep2r>(&self, f: F) -> &Self {
        self.ep2r_reg().with(f);
        self
    }

    #[doc="Get the EP3R Register."]
    #[inline] pub fn ep3r_reg(&self) -> ::bobbin_mcu::register::Register<Ep3r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ep3r, 0xc)
    }

    #[doc="Get the *mut pointer for the EP3R register."]
    #[inline] pub fn ep3r_mut(&self) -> *mut Ep3r { 
        self.ep3r_reg().ptr()
    }

    #[doc="Get the *const pointer for the EP3R register."]
    #[inline] pub fn ep3r_ptr(&self) -> *const Ep3r { 
        self.ep3r_reg().ptr()
    }

    #[doc="Read the EP3R register."]
    #[inline] pub fn ep3r(&self) -> Ep3r { 
        self.ep3r_reg().read()
    }

    #[doc="Write the EP3R register."]
    #[inline] pub fn write_ep3r(&self, value: Ep3r) -> &Self { 
        self.ep3r_reg().write(value);
        self
    }

    #[doc="Set the EP3R register."]
    #[inline] pub fn set_ep3r<F: FnOnce(Ep3r) -> Ep3r>(&self, f: F) -> &Self {
        self.ep3r_reg().set(f);
        self
    }

    #[doc="Modify the EP3R register."]
    #[inline] pub fn with_ep3r<F: FnOnce(Ep3r) -> Ep3r>(&self, f: F) -> &Self {
        self.ep3r_reg().with(f);
        self
    }

    #[doc="Get the EP4R Register."]
    #[inline] pub fn ep4r_reg(&self) -> ::bobbin_mcu::register::Register<Ep4r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ep4r, 0x10)
    }

    #[doc="Get the *mut pointer for the EP4R register."]
    #[inline] pub fn ep4r_mut(&self) -> *mut Ep4r { 
        self.ep4r_reg().ptr()
    }

    #[doc="Get the *const pointer for the EP4R register."]
    #[inline] pub fn ep4r_ptr(&self) -> *const Ep4r { 
        self.ep4r_reg().ptr()
    }

    #[doc="Read the EP4R register."]
    #[inline] pub fn ep4r(&self) -> Ep4r { 
        self.ep4r_reg().read()
    }

    #[doc="Write the EP4R register."]
    #[inline] pub fn write_ep4r(&self, value: Ep4r) -> &Self { 
        self.ep4r_reg().write(value);
        self
    }

    #[doc="Set the EP4R register."]
    #[inline] pub fn set_ep4r<F: FnOnce(Ep4r) -> Ep4r>(&self, f: F) -> &Self {
        self.ep4r_reg().set(f);
        self
    }

    #[doc="Modify the EP4R register."]
    #[inline] pub fn with_ep4r<F: FnOnce(Ep4r) -> Ep4r>(&self, f: F) -> &Self {
        self.ep4r_reg().with(f);
        self
    }

    #[doc="Get the EP5R Register."]
    #[inline] pub fn ep5r_reg(&self) -> ::bobbin_mcu::register::Register<Ep5r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ep5r, 0x14)
    }

    #[doc="Get the *mut pointer for the EP5R register."]
    #[inline] pub fn ep5r_mut(&self) -> *mut Ep5r { 
        self.ep5r_reg().ptr()
    }

    #[doc="Get the *const pointer for the EP5R register."]
    #[inline] pub fn ep5r_ptr(&self) -> *const Ep5r { 
        self.ep5r_reg().ptr()
    }

    #[doc="Read the EP5R register."]
    #[inline] pub fn ep5r(&self) -> Ep5r { 
        self.ep5r_reg().read()
    }

    #[doc="Write the EP5R register."]
    #[inline] pub fn write_ep5r(&self, value: Ep5r) -> &Self { 
        self.ep5r_reg().write(value);
        self
    }

    #[doc="Set the EP5R register."]
    #[inline] pub fn set_ep5r<F: FnOnce(Ep5r) -> Ep5r>(&self, f: F) -> &Self {
        self.ep5r_reg().set(f);
        self
    }

    #[doc="Modify the EP5R register."]
    #[inline] pub fn with_ep5r<F: FnOnce(Ep5r) -> Ep5r>(&self, f: F) -> &Self {
        self.ep5r_reg().with(f);
        self
    }

    #[doc="Get the EP6R Register."]
    #[inline] pub fn ep6r_reg(&self) -> ::bobbin_mcu::register::Register<Ep6r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ep6r, 0x18)
    }

    #[doc="Get the *mut pointer for the EP6R register."]
    #[inline] pub fn ep6r_mut(&self) -> *mut Ep6r { 
        self.ep6r_reg().ptr()
    }

    #[doc="Get the *const pointer for the EP6R register."]
    #[inline] pub fn ep6r_ptr(&self) -> *const Ep6r { 
        self.ep6r_reg().ptr()
    }

    #[doc="Read the EP6R register."]
    #[inline] pub fn ep6r(&self) -> Ep6r { 
        self.ep6r_reg().read()
    }

    #[doc="Write the EP6R register."]
    #[inline] pub fn write_ep6r(&self, value: Ep6r) -> &Self { 
        self.ep6r_reg().write(value);
        self
    }

    #[doc="Set the EP6R register."]
    #[inline] pub fn set_ep6r<F: FnOnce(Ep6r) -> Ep6r>(&self, f: F) -> &Self {
        self.ep6r_reg().set(f);
        self
    }

    #[doc="Modify the EP6R register."]
    #[inline] pub fn with_ep6r<F: FnOnce(Ep6r) -> Ep6r>(&self, f: F) -> &Self {
        self.ep6r_reg().with(f);
        self
    }

    #[doc="Get the EP7R Register."]
    #[inline] pub fn ep7r_reg(&self) -> ::bobbin_mcu::register::Register<Ep7r> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ep7r, 0x1c)
    }

    #[doc="Get the *mut pointer for the EP7R register."]
    #[inline] pub fn ep7r_mut(&self) -> *mut Ep7r { 
        self.ep7r_reg().ptr()
    }

    #[doc="Get the *const pointer for the EP7R register."]
    #[inline] pub fn ep7r_ptr(&self) -> *const Ep7r { 
        self.ep7r_reg().ptr()
    }

    #[doc="Read the EP7R register."]
    #[inline] pub fn ep7r(&self) -> Ep7r { 
        self.ep7r_reg().read()
    }

    #[doc="Write the EP7R register."]
    #[inline] pub fn write_ep7r(&self, value: Ep7r) -> &Self { 
        self.ep7r_reg().write(value);
        self
    }

    #[doc="Set the EP7R register."]
    #[inline] pub fn set_ep7r<F: FnOnce(Ep7r) -> Ep7r>(&self, f: F) -> &Self {
        self.ep7r_reg().set(f);
        self
    }

    #[doc="Modify the EP7R register."]
    #[inline] pub fn with_ep7r<F: FnOnce(Ep7r) -> Ep7r>(&self, f: F) -> &Self {
        self.ep7r_reg().with(f);
        self
    }

    #[doc="Get the CNTR Register."]
    #[inline] pub fn cntr_reg(&self) -> ::bobbin_mcu::register::Register<Cntr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cntr, 0x40)
    }

    #[doc="Get the *mut pointer for the CNTR register."]
    #[inline] pub fn cntr_mut(&self) -> *mut Cntr { 
        self.cntr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNTR register."]
    #[inline] pub fn cntr_ptr(&self) -> *const Cntr { 
        self.cntr_reg().ptr()
    }

    #[doc="Read the CNTR register."]
    #[inline] pub fn cntr(&self) -> Cntr { 
        self.cntr_reg().read()
    }

    #[doc="Write the CNTR register."]
    #[inline] pub fn write_cntr(&self, value: Cntr) -> &Self { 
        self.cntr_reg().write(value);
        self
    }

    #[doc="Set the CNTR register."]
    #[inline] pub fn set_cntr<F: FnOnce(Cntr) -> Cntr>(&self, f: F) -> &Self {
        self.cntr_reg().set(f);
        self
    }

    #[doc="Modify the CNTR register."]
    #[inline] pub fn with_cntr<F: FnOnce(Cntr) -> Cntr>(&self, f: F) -> &Self {
        self.cntr_reg().with(f);
        self
    }

    #[doc="Get the ISTR Register."]
    #[inline] pub fn istr_reg(&self) -> ::bobbin_mcu::register::Register<Istr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Istr, 0x44)
    }

    #[doc="Get the *mut pointer for the ISTR register."]
    #[inline] pub fn istr_mut(&self) -> *mut Istr { 
        self.istr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ISTR register."]
    #[inline] pub fn istr_ptr(&self) -> *const Istr { 
        self.istr_reg().ptr()
    }

    #[doc="Read the ISTR register."]
    #[inline] pub fn istr(&self) -> Istr { 
        self.istr_reg().read()
    }

    #[doc="Write the ISTR register."]
    #[inline] pub fn write_istr(&self, value: Istr) -> &Self { 
        self.istr_reg().write(value);
        self
    }

    #[doc="Set the ISTR register."]
    #[inline] pub fn set_istr<F: FnOnce(Istr) -> Istr>(&self, f: F) -> &Self {
        self.istr_reg().set(f);
        self
    }

    #[doc="Modify the ISTR register."]
    #[inline] pub fn with_istr<F: FnOnce(Istr) -> Istr>(&self, f: F) -> &Self {
        self.istr_reg().with(f);
        self
    }

    #[doc="Get the FNR Register."]
    #[inline] pub fn fnr_reg(&self) -> ::bobbin_mcu::register::Register<Fnr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fnr, 0x48)
    }

    #[doc="Get the *mut pointer for the FNR register."]
    #[inline] pub fn fnr_mut(&self) -> *mut Fnr { 
        self.fnr_reg().ptr()
    }

    #[doc="Get the *const pointer for the FNR register."]
    #[inline] pub fn fnr_ptr(&self) -> *const Fnr { 
        self.fnr_reg().ptr()
    }

    #[doc="Read the FNR register."]
    #[inline] pub fn fnr(&self) -> Fnr { 
        self.fnr_reg().read()
    }

    #[doc="Get the DADDR Register."]
    #[inline] pub fn daddr_reg(&self) -> ::bobbin_mcu::register::Register<Daddr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Daddr, 0x4c)
    }

    #[doc="Get the *mut pointer for the DADDR register."]
    #[inline] pub fn daddr_mut(&self) -> *mut Daddr { 
        self.daddr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DADDR register."]
    #[inline] pub fn daddr_ptr(&self) -> *const Daddr { 
        self.daddr_reg().ptr()
    }

    #[doc="Read the DADDR register."]
    #[inline] pub fn daddr(&self) -> Daddr { 
        self.daddr_reg().read()
    }

    #[doc="Write the DADDR register."]
    #[inline] pub fn write_daddr(&self, value: Daddr) -> &Self { 
        self.daddr_reg().write(value);
        self
    }

    #[doc="Set the DADDR register."]
    #[inline] pub fn set_daddr<F: FnOnce(Daddr) -> Daddr>(&self, f: F) -> &Self {
        self.daddr_reg().set(f);
        self
    }

    #[doc="Modify the DADDR register."]
    #[inline] pub fn with_daddr<F: FnOnce(Daddr) -> Daddr>(&self, f: F) -> &Self {
        self.daddr_reg().with(f);
        self
    }

    #[doc="Get the BTABLE Register."]
    #[inline] pub fn btable_reg(&self) -> ::bobbin_mcu::register::Register<Btable> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Btable, 0x50)
    }

    #[doc="Get the *mut pointer for the BTABLE register."]
    #[inline] pub fn btable_mut(&self) -> *mut Btable { 
        self.btable_reg().ptr()
    }

    #[doc="Get the *const pointer for the BTABLE register."]
    #[inline] pub fn btable_ptr(&self) -> *const Btable { 
        self.btable_reg().ptr()
    }

    #[doc="Read the BTABLE register."]
    #[inline] pub fn btable(&self) -> Btable { 
        self.btable_reg().read()
    }

    #[doc="Write the BTABLE register."]
    #[inline] pub fn write_btable(&self, value: Btable) -> &Self { 
        self.btable_reg().write(value);
        self
    }

    #[doc="Set the BTABLE register."]
    #[inline] pub fn set_btable<F: FnOnce(Btable) -> Btable>(&self, f: F) -> &Self {
        self.btable_reg().set(f);
        self
    }

    #[doc="Modify the BTABLE register."]
    #[inline] pub fn with_btable<F: FnOnce(Btable) -> Btable>(&self, f: F) -> &Self {
        self.btable_reg().with(f);
        self
    }

    #[doc="Get the COUNT0_TX Register."]
    #[inline] pub fn count0_tx_reg(&self) -> ::bobbin_mcu::register::Register<Count0Tx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count0Tx, 0x52)
    }

    #[doc="Get the *mut pointer for the COUNT0_TX register."]
    #[inline] pub fn count0_tx_mut(&self) -> *mut Count0Tx { 
        self.count0_tx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT0_TX register."]
    #[inline] pub fn count0_tx_ptr(&self) -> *const Count0Tx { 
        self.count0_tx_reg().ptr()
    }

    #[doc="Read the COUNT0_TX register."]
    #[inline] pub fn count0_tx(&self) -> Count0Tx { 
        self.count0_tx_reg().read()
    }

    #[doc="Write the COUNT0_TX register."]
    #[inline] pub fn write_count0_tx(&self, value: Count0Tx) -> &Self { 
        self.count0_tx_reg().write(value);
        self
    }

    #[doc="Set the COUNT0_TX register."]
    #[inline] pub fn set_count0_tx<F: FnOnce(Count0Tx) -> Count0Tx>(&self, f: F) -> &Self {
        self.count0_tx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT0_TX register."]
    #[inline] pub fn with_count0_tx<F: FnOnce(Count0Tx) -> Count0Tx>(&self, f: F) -> &Self {
        self.count0_tx_reg().with(f);
        self
    }

    #[doc="Get the COUNT1_TX Register."]
    #[inline] pub fn count1_tx_reg(&self) -> ::bobbin_mcu::register::Register<Count1Tx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count1Tx, 0x5a)
    }

    #[doc="Get the *mut pointer for the COUNT1_TX register."]
    #[inline] pub fn count1_tx_mut(&self) -> *mut Count1Tx { 
        self.count1_tx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT1_TX register."]
    #[inline] pub fn count1_tx_ptr(&self) -> *const Count1Tx { 
        self.count1_tx_reg().ptr()
    }

    #[doc="Read the COUNT1_TX register."]
    #[inline] pub fn count1_tx(&self) -> Count1Tx { 
        self.count1_tx_reg().read()
    }

    #[doc="Write the COUNT1_TX register."]
    #[inline] pub fn write_count1_tx(&self, value: Count1Tx) -> &Self { 
        self.count1_tx_reg().write(value);
        self
    }

    #[doc="Set the COUNT1_TX register."]
    #[inline] pub fn set_count1_tx<F: FnOnce(Count1Tx) -> Count1Tx>(&self, f: F) -> &Self {
        self.count1_tx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT1_TX register."]
    #[inline] pub fn with_count1_tx<F: FnOnce(Count1Tx) -> Count1Tx>(&self, f: F) -> &Self {
        self.count1_tx_reg().with(f);
        self
    }

    #[doc="Get the COUNT2_TX Register."]
    #[inline] pub fn count2_tx_reg(&self) -> ::bobbin_mcu::register::Register<Count2Tx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count2Tx, 0x62)
    }

    #[doc="Get the *mut pointer for the COUNT2_TX register."]
    #[inline] pub fn count2_tx_mut(&self) -> *mut Count2Tx { 
        self.count2_tx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT2_TX register."]
    #[inline] pub fn count2_tx_ptr(&self) -> *const Count2Tx { 
        self.count2_tx_reg().ptr()
    }

    #[doc="Read the COUNT2_TX register."]
    #[inline] pub fn count2_tx(&self) -> Count2Tx { 
        self.count2_tx_reg().read()
    }

    #[doc="Write the COUNT2_TX register."]
    #[inline] pub fn write_count2_tx(&self, value: Count2Tx) -> &Self { 
        self.count2_tx_reg().write(value);
        self
    }

    #[doc="Set the COUNT2_TX register."]
    #[inline] pub fn set_count2_tx<F: FnOnce(Count2Tx) -> Count2Tx>(&self, f: F) -> &Self {
        self.count2_tx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT2_TX register."]
    #[inline] pub fn with_count2_tx<F: FnOnce(Count2Tx) -> Count2Tx>(&self, f: F) -> &Self {
        self.count2_tx_reg().with(f);
        self
    }

    #[doc="Get the COUNT3_TX Register."]
    #[inline] pub fn count3_tx_reg(&self) -> ::bobbin_mcu::register::Register<Count3Tx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count3Tx, 0x6a)
    }

    #[doc="Get the *mut pointer for the COUNT3_TX register."]
    #[inline] pub fn count3_tx_mut(&self) -> *mut Count3Tx { 
        self.count3_tx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT3_TX register."]
    #[inline] pub fn count3_tx_ptr(&self) -> *const Count3Tx { 
        self.count3_tx_reg().ptr()
    }

    #[doc="Read the COUNT3_TX register."]
    #[inline] pub fn count3_tx(&self) -> Count3Tx { 
        self.count3_tx_reg().read()
    }

    #[doc="Write the COUNT3_TX register."]
    #[inline] pub fn write_count3_tx(&self, value: Count3Tx) -> &Self { 
        self.count3_tx_reg().write(value);
        self
    }

    #[doc="Set the COUNT3_TX register."]
    #[inline] pub fn set_count3_tx<F: FnOnce(Count3Tx) -> Count3Tx>(&self, f: F) -> &Self {
        self.count3_tx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT3_TX register."]
    #[inline] pub fn with_count3_tx<F: FnOnce(Count3Tx) -> Count3Tx>(&self, f: F) -> &Self {
        self.count3_tx_reg().with(f);
        self
    }

    #[doc="Get the COUNT4_TX Register."]
    #[inline] pub fn count4_tx_reg(&self) -> ::bobbin_mcu::register::Register<Count4Tx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count4Tx, 0x72)
    }

    #[doc="Get the *mut pointer for the COUNT4_TX register."]
    #[inline] pub fn count4_tx_mut(&self) -> *mut Count4Tx { 
        self.count4_tx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT4_TX register."]
    #[inline] pub fn count4_tx_ptr(&self) -> *const Count4Tx { 
        self.count4_tx_reg().ptr()
    }

    #[doc="Read the COUNT4_TX register."]
    #[inline] pub fn count4_tx(&self) -> Count4Tx { 
        self.count4_tx_reg().read()
    }

    #[doc="Write the COUNT4_TX register."]
    #[inline] pub fn write_count4_tx(&self, value: Count4Tx) -> &Self { 
        self.count4_tx_reg().write(value);
        self
    }

    #[doc="Set the COUNT4_TX register."]
    #[inline] pub fn set_count4_tx<F: FnOnce(Count4Tx) -> Count4Tx>(&self, f: F) -> &Self {
        self.count4_tx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT4_TX register."]
    #[inline] pub fn with_count4_tx<F: FnOnce(Count4Tx) -> Count4Tx>(&self, f: F) -> &Self {
        self.count4_tx_reg().with(f);
        self
    }

    #[doc="Get the COUNT5_TX Register."]
    #[inline] pub fn count5_tx_reg(&self) -> ::bobbin_mcu::register::Register<Count5Tx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count5Tx, 0x7a)
    }

    #[doc="Get the *mut pointer for the COUNT5_TX register."]
    #[inline] pub fn count5_tx_mut(&self) -> *mut Count5Tx { 
        self.count5_tx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT5_TX register."]
    #[inline] pub fn count5_tx_ptr(&self) -> *const Count5Tx { 
        self.count5_tx_reg().ptr()
    }

    #[doc="Read the COUNT5_TX register."]
    #[inline] pub fn count5_tx(&self) -> Count5Tx { 
        self.count5_tx_reg().read()
    }

    #[doc="Write the COUNT5_TX register."]
    #[inline] pub fn write_count5_tx(&self, value: Count5Tx) -> &Self { 
        self.count5_tx_reg().write(value);
        self
    }

    #[doc="Set the COUNT5_TX register."]
    #[inline] pub fn set_count5_tx<F: FnOnce(Count5Tx) -> Count5Tx>(&self, f: F) -> &Self {
        self.count5_tx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT5_TX register."]
    #[inline] pub fn with_count5_tx<F: FnOnce(Count5Tx) -> Count5Tx>(&self, f: F) -> &Self {
        self.count5_tx_reg().with(f);
        self
    }

    #[doc="Get the COUNT6_TX Register."]
    #[inline] pub fn count6_tx_reg(&self) -> ::bobbin_mcu::register::Register<Count6Tx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count6Tx, 0x82)
    }

    #[doc="Get the *mut pointer for the COUNT6_TX register."]
    #[inline] pub fn count6_tx_mut(&self) -> *mut Count6Tx { 
        self.count6_tx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT6_TX register."]
    #[inline] pub fn count6_tx_ptr(&self) -> *const Count6Tx { 
        self.count6_tx_reg().ptr()
    }

    #[doc="Read the COUNT6_TX register."]
    #[inline] pub fn count6_tx(&self) -> Count6Tx { 
        self.count6_tx_reg().read()
    }

    #[doc="Write the COUNT6_TX register."]
    #[inline] pub fn write_count6_tx(&self, value: Count6Tx) -> &Self { 
        self.count6_tx_reg().write(value);
        self
    }

    #[doc="Set the COUNT6_TX register."]
    #[inline] pub fn set_count6_tx<F: FnOnce(Count6Tx) -> Count6Tx>(&self, f: F) -> &Self {
        self.count6_tx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT6_TX register."]
    #[inline] pub fn with_count6_tx<F: FnOnce(Count6Tx) -> Count6Tx>(&self, f: F) -> &Self {
        self.count6_tx_reg().with(f);
        self
    }

    #[doc="Get the COUNT7_TX Register."]
    #[inline] pub fn count7_tx_reg(&self) -> ::bobbin_mcu::register::Register<Count7Tx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count7Tx, 0x8a)
    }

    #[doc="Get the *mut pointer for the COUNT7_TX register."]
    #[inline] pub fn count7_tx_mut(&self) -> *mut Count7Tx { 
        self.count7_tx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT7_TX register."]
    #[inline] pub fn count7_tx_ptr(&self) -> *const Count7Tx { 
        self.count7_tx_reg().ptr()
    }

    #[doc="Read the COUNT7_TX register."]
    #[inline] pub fn count7_tx(&self) -> Count7Tx { 
        self.count7_tx_reg().read()
    }

    #[doc="Write the COUNT7_TX register."]
    #[inline] pub fn write_count7_tx(&self, value: Count7Tx) -> &Self { 
        self.count7_tx_reg().write(value);
        self
    }

    #[doc="Set the COUNT7_TX register."]
    #[inline] pub fn set_count7_tx<F: FnOnce(Count7Tx) -> Count7Tx>(&self, f: F) -> &Self {
        self.count7_tx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT7_TX register."]
    #[inline] pub fn with_count7_tx<F: FnOnce(Count7Tx) -> Count7Tx>(&self, f: F) -> &Self {
        self.count7_tx_reg().with(f);
        self
    }

    #[doc="Get the ADDR0_RX Register."]
    #[inline] pub fn addr0_rx_reg(&self) -> ::bobbin_mcu::register::Register<Addr0Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr0Rx, 0x54)
    }

    #[doc="Get the *mut pointer for the ADDR0_RX register."]
    #[inline] pub fn addr0_rx_mut(&self) -> *mut Addr0Rx { 
        self.addr0_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR0_RX register."]
    #[inline] pub fn addr0_rx_ptr(&self) -> *const Addr0Rx { 
        self.addr0_rx_reg().ptr()
    }

    #[doc="Read the ADDR0_RX register."]
    #[inline] pub fn addr0_rx(&self) -> Addr0Rx { 
        self.addr0_rx_reg().read()
    }

    #[doc="Write the ADDR0_RX register."]
    #[inline] pub fn write_addr0_rx(&self, value: Addr0Rx) -> &Self { 
        self.addr0_rx_reg().write(value);
        self
    }

    #[doc="Set the ADDR0_RX register."]
    #[inline] pub fn set_addr0_rx<F: FnOnce(Addr0Rx) -> Addr0Rx>(&self, f: F) -> &Self {
        self.addr0_rx_reg().set(f);
        self
    }

    #[doc="Modify the ADDR0_RX register."]
    #[inline] pub fn with_addr0_rx<F: FnOnce(Addr0Rx) -> Addr0Rx>(&self, f: F) -> &Self {
        self.addr0_rx_reg().with(f);
        self
    }

    #[doc="Get the ADDR1_RX Register."]
    #[inline] pub fn addr1_rx_reg(&self) -> ::bobbin_mcu::register::Register<Addr1Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr1Rx, 0x5c)
    }

    #[doc="Get the *mut pointer for the ADDR1_RX register."]
    #[inline] pub fn addr1_rx_mut(&self) -> *mut Addr1Rx { 
        self.addr1_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR1_RX register."]
    #[inline] pub fn addr1_rx_ptr(&self) -> *const Addr1Rx { 
        self.addr1_rx_reg().ptr()
    }

    #[doc="Read the ADDR1_RX register."]
    #[inline] pub fn addr1_rx(&self) -> Addr1Rx { 
        self.addr1_rx_reg().read()
    }

    #[doc="Write the ADDR1_RX register."]
    #[inline] pub fn write_addr1_rx(&self, value: Addr1Rx) -> &Self { 
        self.addr1_rx_reg().write(value);
        self
    }

    #[doc="Set the ADDR1_RX register."]
    #[inline] pub fn set_addr1_rx<F: FnOnce(Addr1Rx) -> Addr1Rx>(&self, f: F) -> &Self {
        self.addr1_rx_reg().set(f);
        self
    }

    #[doc="Modify the ADDR1_RX register."]
    #[inline] pub fn with_addr1_rx<F: FnOnce(Addr1Rx) -> Addr1Rx>(&self, f: F) -> &Self {
        self.addr1_rx_reg().with(f);
        self
    }

    #[doc="Get the ADDR2_RX Register."]
    #[inline] pub fn addr2_rx_reg(&self) -> ::bobbin_mcu::register::Register<Addr2Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr2Rx, 0x64)
    }

    #[doc="Get the *mut pointer for the ADDR2_RX register."]
    #[inline] pub fn addr2_rx_mut(&self) -> *mut Addr2Rx { 
        self.addr2_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR2_RX register."]
    #[inline] pub fn addr2_rx_ptr(&self) -> *const Addr2Rx { 
        self.addr2_rx_reg().ptr()
    }

    #[doc="Read the ADDR2_RX register."]
    #[inline] pub fn addr2_rx(&self) -> Addr2Rx { 
        self.addr2_rx_reg().read()
    }

    #[doc="Write the ADDR2_RX register."]
    #[inline] pub fn write_addr2_rx(&self, value: Addr2Rx) -> &Self { 
        self.addr2_rx_reg().write(value);
        self
    }

    #[doc="Set the ADDR2_RX register."]
    #[inline] pub fn set_addr2_rx<F: FnOnce(Addr2Rx) -> Addr2Rx>(&self, f: F) -> &Self {
        self.addr2_rx_reg().set(f);
        self
    }

    #[doc="Modify the ADDR2_RX register."]
    #[inline] pub fn with_addr2_rx<F: FnOnce(Addr2Rx) -> Addr2Rx>(&self, f: F) -> &Self {
        self.addr2_rx_reg().with(f);
        self
    }

    #[doc="Get the ADDR3_RX Register."]
    #[inline] pub fn addr3_rx_reg(&self) -> ::bobbin_mcu::register::Register<Addr3Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr3Rx, 0x6c)
    }

    #[doc="Get the *mut pointer for the ADDR3_RX register."]
    #[inline] pub fn addr3_rx_mut(&self) -> *mut Addr3Rx { 
        self.addr3_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR3_RX register."]
    #[inline] pub fn addr3_rx_ptr(&self) -> *const Addr3Rx { 
        self.addr3_rx_reg().ptr()
    }

    #[doc="Read the ADDR3_RX register."]
    #[inline] pub fn addr3_rx(&self) -> Addr3Rx { 
        self.addr3_rx_reg().read()
    }

    #[doc="Write the ADDR3_RX register."]
    #[inline] pub fn write_addr3_rx(&self, value: Addr3Rx) -> &Self { 
        self.addr3_rx_reg().write(value);
        self
    }

    #[doc="Set the ADDR3_RX register."]
    #[inline] pub fn set_addr3_rx<F: FnOnce(Addr3Rx) -> Addr3Rx>(&self, f: F) -> &Self {
        self.addr3_rx_reg().set(f);
        self
    }

    #[doc="Modify the ADDR3_RX register."]
    #[inline] pub fn with_addr3_rx<F: FnOnce(Addr3Rx) -> Addr3Rx>(&self, f: F) -> &Self {
        self.addr3_rx_reg().with(f);
        self
    }

    #[doc="Get the ADDR4_RX Register."]
    #[inline] pub fn addr4_rx_reg(&self) -> ::bobbin_mcu::register::Register<Addr4Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr4Rx, 0x74)
    }

    #[doc="Get the *mut pointer for the ADDR4_RX register."]
    #[inline] pub fn addr4_rx_mut(&self) -> *mut Addr4Rx { 
        self.addr4_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR4_RX register."]
    #[inline] pub fn addr4_rx_ptr(&self) -> *const Addr4Rx { 
        self.addr4_rx_reg().ptr()
    }

    #[doc="Read the ADDR4_RX register."]
    #[inline] pub fn addr4_rx(&self) -> Addr4Rx { 
        self.addr4_rx_reg().read()
    }

    #[doc="Write the ADDR4_RX register."]
    #[inline] pub fn write_addr4_rx(&self, value: Addr4Rx) -> &Self { 
        self.addr4_rx_reg().write(value);
        self
    }

    #[doc="Set the ADDR4_RX register."]
    #[inline] pub fn set_addr4_rx<F: FnOnce(Addr4Rx) -> Addr4Rx>(&self, f: F) -> &Self {
        self.addr4_rx_reg().set(f);
        self
    }

    #[doc="Modify the ADDR4_RX register."]
    #[inline] pub fn with_addr4_rx<F: FnOnce(Addr4Rx) -> Addr4Rx>(&self, f: F) -> &Self {
        self.addr4_rx_reg().with(f);
        self
    }

    #[doc="Get the ADDR5_RX Register."]
    #[inline] pub fn addr5_rx_reg(&self) -> ::bobbin_mcu::register::Register<Addr5Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr5Rx, 0x7c)
    }

    #[doc="Get the *mut pointer for the ADDR5_RX register."]
    #[inline] pub fn addr5_rx_mut(&self) -> *mut Addr5Rx { 
        self.addr5_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR5_RX register."]
    #[inline] pub fn addr5_rx_ptr(&self) -> *const Addr5Rx { 
        self.addr5_rx_reg().ptr()
    }

    #[doc="Read the ADDR5_RX register."]
    #[inline] pub fn addr5_rx(&self) -> Addr5Rx { 
        self.addr5_rx_reg().read()
    }

    #[doc="Write the ADDR5_RX register."]
    #[inline] pub fn write_addr5_rx(&self, value: Addr5Rx) -> &Self { 
        self.addr5_rx_reg().write(value);
        self
    }

    #[doc="Set the ADDR5_RX register."]
    #[inline] pub fn set_addr5_rx<F: FnOnce(Addr5Rx) -> Addr5Rx>(&self, f: F) -> &Self {
        self.addr5_rx_reg().set(f);
        self
    }

    #[doc="Modify the ADDR5_RX register."]
    #[inline] pub fn with_addr5_rx<F: FnOnce(Addr5Rx) -> Addr5Rx>(&self, f: F) -> &Self {
        self.addr5_rx_reg().with(f);
        self
    }

    #[doc="Get the ADDR6_RX Register."]
    #[inline] pub fn addr6_rx_reg(&self) -> ::bobbin_mcu::register::Register<Addr6Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr6Rx, 0x84)
    }

    #[doc="Get the *mut pointer for the ADDR6_RX register."]
    #[inline] pub fn addr6_rx_mut(&self) -> *mut Addr6Rx { 
        self.addr6_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR6_RX register."]
    #[inline] pub fn addr6_rx_ptr(&self) -> *const Addr6Rx { 
        self.addr6_rx_reg().ptr()
    }

    #[doc="Read the ADDR6_RX register."]
    #[inline] pub fn addr6_rx(&self) -> Addr6Rx { 
        self.addr6_rx_reg().read()
    }

    #[doc="Write the ADDR6_RX register."]
    #[inline] pub fn write_addr6_rx(&self, value: Addr6Rx) -> &Self { 
        self.addr6_rx_reg().write(value);
        self
    }

    #[doc="Set the ADDR6_RX register."]
    #[inline] pub fn set_addr6_rx<F: FnOnce(Addr6Rx) -> Addr6Rx>(&self, f: F) -> &Self {
        self.addr6_rx_reg().set(f);
        self
    }

    #[doc="Modify the ADDR6_RX register."]
    #[inline] pub fn with_addr6_rx<F: FnOnce(Addr6Rx) -> Addr6Rx>(&self, f: F) -> &Self {
        self.addr6_rx_reg().with(f);
        self
    }

    #[doc="Get the ADDR7_RX Register."]
    #[inline] pub fn addr7_rx_reg(&self) -> ::bobbin_mcu::register::Register<Addr7Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr7Rx, 0x8c)
    }

    #[doc="Get the *mut pointer for the ADDR7_RX register."]
    #[inline] pub fn addr7_rx_mut(&self) -> *mut Addr7Rx { 
        self.addr7_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR7_RX register."]
    #[inline] pub fn addr7_rx_ptr(&self) -> *const Addr7Rx { 
        self.addr7_rx_reg().ptr()
    }

    #[doc="Read the ADDR7_RX register."]
    #[inline] pub fn addr7_rx(&self) -> Addr7Rx { 
        self.addr7_rx_reg().read()
    }

    #[doc="Write the ADDR7_RX register."]
    #[inline] pub fn write_addr7_rx(&self, value: Addr7Rx) -> &Self { 
        self.addr7_rx_reg().write(value);
        self
    }

    #[doc="Set the ADDR7_RX register."]
    #[inline] pub fn set_addr7_rx<F: FnOnce(Addr7Rx) -> Addr7Rx>(&self, f: F) -> &Self {
        self.addr7_rx_reg().set(f);
        self
    }

    #[doc="Modify the ADDR7_RX register."]
    #[inline] pub fn with_addr7_rx<F: FnOnce(Addr7Rx) -> Addr7Rx>(&self, f: F) -> &Self {
        self.addr7_rx_reg().with(f);
        self
    }

    #[doc="Get the COUNT0_RX Register."]
    #[inline] pub fn count0_rx_reg(&self) -> ::bobbin_mcu::register::Register<Count0Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count0Rx, 0x56)
    }

    #[doc="Get the *mut pointer for the COUNT0_RX register."]
    #[inline] pub fn count0_rx_mut(&self) -> *mut Count0Rx { 
        self.count0_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT0_RX register."]
    #[inline] pub fn count0_rx_ptr(&self) -> *const Count0Rx { 
        self.count0_rx_reg().ptr()
    }

    #[doc="Read the COUNT0_RX register."]
    #[inline] pub fn count0_rx(&self) -> Count0Rx { 
        self.count0_rx_reg().read()
    }

    #[doc="Write the COUNT0_RX register."]
    #[inline] pub fn write_count0_rx(&self, value: Count0Rx) -> &Self { 
        self.count0_rx_reg().write(value);
        self
    }

    #[doc="Set the COUNT0_RX register."]
    #[inline] pub fn set_count0_rx<F: FnOnce(Count0Rx) -> Count0Rx>(&self, f: F) -> &Self {
        self.count0_rx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT0_RX register."]
    #[inline] pub fn with_count0_rx<F: FnOnce(Count0Rx) -> Count0Rx>(&self, f: F) -> &Self {
        self.count0_rx_reg().with(f);
        self
    }

    #[doc="Get the COUNT1_RX Register."]
    #[inline] pub fn count1_rx_reg(&self) -> ::bobbin_mcu::register::Register<Count1Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count1Rx, 0x5e)
    }

    #[doc="Get the *mut pointer for the COUNT1_RX register."]
    #[inline] pub fn count1_rx_mut(&self) -> *mut Count1Rx { 
        self.count1_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT1_RX register."]
    #[inline] pub fn count1_rx_ptr(&self) -> *const Count1Rx { 
        self.count1_rx_reg().ptr()
    }

    #[doc="Read the COUNT1_RX register."]
    #[inline] pub fn count1_rx(&self) -> Count1Rx { 
        self.count1_rx_reg().read()
    }

    #[doc="Write the COUNT1_RX register."]
    #[inline] pub fn write_count1_rx(&self, value: Count1Rx) -> &Self { 
        self.count1_rx_reg().write(value);
        self
    }

    #[doc="Set the COUNT1_RX register."]
    #[inline] pub fn set_count1_rx<F: FnOnce(Count1Rx) -> Count1Rx>(&self, f: F) -> &Self {
        self.count1_rx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT1_RX register."]
    #[inline] pub fn with_count1_rx<F: FnOnce(Count1Rx) -> Count1Rx>(&self, f: F) -> &Self {
        self.count1_rx_reg().with(f);
        self
    }

    #[doc="Get the COUNT2_RX Register."]
    #[inline] pub fn count2_rx_reg(&self) -> ::bobbin_mcu::register::Register<Count2Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count2Rx, 0x66)
    }

    #[doc="Get the *mut pointer for the COUNT2_RX register."]
    #[inline] pub fn count2_rx_mut(&self) -> *mut Count2Rx { 
        self.count2_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT2_RX register."]
    #[inline] pub fn count2_rx_ptr(&self) -> *const Count2Rx { 
        self.count2_rx_reg().ptr()
    }

    #[doc="Read the COUNT2_RX register."]
    #[inline] pub fn count2_rx(&self) -> Count2Rx { 
        self.count2_rx_reg().read()
    }

    #[doc="Write the COUNT2_RX register."]
    #[inline] pub fn write_count2_rx(&self, value: Count2Rx) -> &Self { 
        self.count2_rx_reg().write(value);
        self
    }

    #[doc="Set the COUNT2_RX register."]
    #[inline] pub fn set_count2_rx<F: FnOnce(Count2Rx) -> Count2Rx>(&self, f: F) -> &Self {
        self.count2_rx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT2_RX register."]
    #[inline] pub fn with_count2_rx<F: FnOnce(Count2Rx) -> Count2Rx>(&self, f: F) -> &Self {
        self.count2_rx_reg().with(f);
        self
    }

    #[doc="Get the COUNT3_RX Register."]
    #[inline] pub fn count3_rx_reg(&self) -> ::bobbin_mcu::register::Register<Count3Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count3Rx, 0x6e)
    }

    #[doc="Get the *mut pointer for the COUNT3_RX register."]
    #[inline] pub fn count3_rx_mut(&self) -> *mut Count3Rx { 
        self.count3_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT3_RX register."]
    #[inline] pub fn count3_rx_ptr(&self) -> *const Count3Rx { 
        self.count3_rx_reg().ptr()
    }

    #[doc="Read the COUNT3_RX register."]
    #[inline] pub fn count3_rx(&self) -> Count3Rx { 
        self.count3_rx_reg().read()
    }

    #[doc="Write the COUNT3_RX register."]
    #[inline] pub fn write_count3_rx(&self, value: Count3Rx) -> &Self { 
        self.count3_rx_reg().write(value);
        self
    }

    #[doc="Set the COUNT3_RX register."]
    #[inline] pub fn set_count3_rx<F: FnOnce(Count3Rx) -> Count3Rx>(&self, f: F) -> &Self {
        self.count3_rx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT3_RX register."]
    #[inline] pub fn with_count3_rx<F: FnOnce(Count3Rx) -> Count3Rx>(&self, f: F) -> &Self {
        self.count3_rx_reg().with(f);
        self
    }

    #[doc="Get the COUNT4_RX Register."]
    #[inline] pub fn count4_rx_reg(&self) -> ::bobbin_mcu::register::Register<Count4Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count4Rx, 0x76)
    }

    #[doc="Get the *mut pointer for the COUNT4_RX register."]
    #[inline] pub fn count4_rx_mut(&self) -> *mut Count4Rx { 
        self.count4_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT4_RX register."]
    #[inline] pub fn count4_rx_ptr(&self) -> *const Count4Rx { 
        self.count4_rx_reg().ptr()
    }

    #[doc="Read the COUNT4_RX register."]
    #[inline] pub fn count4_rx(&self) -> Count4Rx { 
        self.count4_rx_reg().read()
    }

    #[doc="Write the COUNT4_RX register."]
    #[inline] pub fn write_count4_rx(&self, value: Count4Rx) -> &Self { 
        self.count4_rx_reg().write(value);
        self
    }

    #[doc="Set the COUNT4_RX register."]
    #[inline] pub fn set_count4_rx<F: FnOnce(Count4Rx) -> Count4Rx>(&self, f: F) -> &Self {
        self.count4_rx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT4_RX register."]
    #[inline] pub fn with_count4_rx<F: FnOnce(Count4Rx) -> Count4Rx>(&self, f: F) -> &Self {
        self.count4_rx_reg().with(f);
        self
    }

    #[doc="Get the COUNT5_RX Register."]
    #[inline] pub fn count5_rx_reg(&self) -> ::bobbin_mcu::register::Register<Count5Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count5Rx, 0x7e)
    }

    #[doc="Get the *mut pointer for the COUNT5_RX register."]
    #[inline] pub fn count5_rx_mut(&self) -> *mut Count5Rx { 
        self.count5_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT5_RX register."]
    #[inline] pub fn count5_rx_ptr(&self) -> *const Count5Rx { 
        self.count5_rx_reg().ptr()
    }

    #[doc="Read the COUNT5_RX register."]
    #[inline] pub fn count5_rx(&self) -> Count5Rx { 
        self.count5_rx_reg().read()
    }

    #[doc="Write the COUNT5_RX register."]
    #[inline] pub fn write_count5_rx(&self, value: Count5Rx) -> &Self { 
        self.count5_rx_reg().write(value);
        self
    }

    #[doc="Set the COUNT5_RX register."]
    #[inline] pub fn set_count5_rx<F: FnOnce(Count5Rx) -> Count5Rx>(&self, f: F) -> &Self {
        self.count5_rx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT5_RX register."]
    #[inline] pub fn with_count5_rx<F: FnOnce(Count5Rx) -> Count5Rx>(&self, f: F) -> &Self {
        self.count5_rx_reg().with(f);
        self
    }

    #[doc="Get the COUNT6_RX Register."]
    #[inline] pub fn count6_rx_reg(&self) -> ::bobbin_mcu::register::Register<Count6Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count6Rx, 0x86)
    }

    #[doc="Get the *mut pointer for the COUNT6_RX register."]
    #[inline] pub fn count6_rx_mut(&self) -> *mut Count6Rx { 
        self.count6_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT6_RX register."]
    #[inline] pub fn count6_rx_ptr(&self) -> *const Count6Rx { 
        self.count6_rx_reg().ptr()
    }

    #[doc="Read the COUNT6_RX register."]
    #[inline] pub fn count6_rx(&self) -> Count6Rx { 
        self.count6_rx_reg().read()
    }

    #[doc="Write the COUNT6_RX register."]
    #[inline] pub fn write_count6_rx(&self, value: Count6Rx) -> &Self { 
        self.count6_rx_reg().write(value);
        self
    }

    #[doc="Set the COUNT6_RX register."]
    #[inline] pub fn set_count6_rx<F: FnOnce(Count6Rx) -> Count6Rx>(&self, f: F) -> &Self {
        self.count6_rx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT6_RX register."]
    #[inline] pub fn with_count6_rx<F: FnOnce(Count6Rx) -> Count6Rx>(&self, f: F) -> &Self {
        self.count6_rx_reg().with(f);
        self
    }

    #[doc="Get the COUNT7_RX Register."]
    #[inline] pub fn count7_rx_reg(&self) -> ::bobbin_mcu::register::Register<Count7Rx> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count7Rx, 0x8e)
    }

    #[doc="Get the *mut pointer for the COUNT7_RX register."]
    #[inline] pub fn count7_rx_mut(&self) -> *mut Count7Rx { 
        self.count7_rx_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT7_RX register."]
    #[inline] pub fn count7_rx_ptr(&self) -> *const Count7Rx { 
        self.count7_rx_reg().ptr()
    }

    #[doc="Read the COUNT7_RX register."]
    #[inline] pub fn count7_rx(&self) -> Count7Rx { 
        self.count7_rx_reg().read()
    }

    #[doc="Write the COUNT7_RX register."]
    #[inline] pub fn write_count7_rx(&self, value: Count7Rx) -> &Self { 
        self.count7_rx_reg().write(value);
        self
    }

    #[doc="Set the COUNT7_RX register."]
    #[inline] pub fn set_count7_rx<F: FnOnce(Count7Rx) -> Count7Rx>(&self, f: F) -> &Self {
        self.count7_rx_reg().set(f);
        self
    }

    #[doc="Modify the COUNT7_RX register."]
    #[inline] pub fn with_count7_rx<F: FnOnce(Count7Rx) -> Count7Rx>(&self, f: F) -> &Self {
        self.count7_rx_reg().with(f);
        self
    }

    #[doc="Get the LPMCSR Register."]
    #[inline] pub fn lpmcsr_reg(&self) -> ::bobbin_mcu::register::Register<Lpmcsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Lpmcsr, 0x54)
    }

    #[doc="Get the *mut pointer for the LPMCSR register."]
    #[inline] pub fn lpmcsr_mut(&self) -> *mut Lpmcsr { 
        self.lpmcsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the LPMCSR register."]
    #[inline] pub fn lpmcsr_ptr(&self) -> *const Lpmcsr { 
        self.lpmcsr_reg().ptr()
    }

    #[doc="Read the LPMCSR register."]
    #[inline] pub fn lpmcsr(&self) -> Lpmcsr { 
        self.lpmcsr_reg().read()
    }

    #[doc="Write the LPMCSR register."]
    #[inline] pub fn write_lpmcsr(&self, value: Lpmcsr) -> &Self { 
        self.lpmcsr_reg().write(value);
        self
    }

    #[doc="Set the LPMCSR register."]
    #[inline] pub fn set_lpmcsr<F: FnOnce(Lpmcsr) -> Lpmcsr>(&self, f: F) -> &Self {
        self.lpmcsr_reg().set(f);
        self
    }

    #[doc="Modify the LPMCSR register."]
    #[inline] pub fn with_lpmcsr<F: FnOnce(Lpmcsr) -> Lpmcsr>(&self, f: F) -> &Self {
        self.lpmcsr_reg().with(f);
        self
    }

    #[doc="Get the BCDR Register."]
    #[inline] pub fn bcdr_reg(&self) -> ::bobbin_mcu::register::Register<Bcdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bcdr, 0x58)
    }

    #[doc="Get the *mut pointer for the BCDR register."]
    #[inline] pub fn bcdr_mut(&self) -> *mut Bcdr { 
        self.bcdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BCDR register."]
    #[inline] pub fn bcdr_ptr(&self) -> *const Bcdr { 
        self.bcdr_reg().ptr()
    }

    #[doc="Read the BCDR register."]
    #[inline] pub fn bcdr(&self) -> Bcdr { 
        self.bcdr_reg().read()
    }

    #[doc="Write the BCDR register."]
    #[inline] pub fn write_bcdr(&self, value: Bcdr) -> &Self { 
        self.bcdr_reg().write(value);
        self
    }

    #[doc="Set the BCDR register."]
    #[inline] pub fn set_bcdr<F: FnOnce(Bcdr) -> Bcdr>(&self, f: F) -> &Self {
        self.bcdr_reg().set(f);
        self
    }

    #[doc="Modify the BCDR register."]
    #[inline] pub fn with_bcdr<F: FnOnce(Bcdr) -> Bcdr>(&self, f: F) -> &Self {
        self.bcdr_reg().with(f);
        self
    }

}

#[doc="endpoint 0 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep0r(pub u16);
impl Ep0r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ep0r {
    #[inline]
    fn from(other: u16) -> Self {
         Ep0r(other)
    }
}

impl ::core::fmt::Display for Ep0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep0r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 1 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep1r(pub u16);
impl Ep1r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ep1r {
    #[inline]
    fn from(other: u16) -> Self {
         Ep1r(other)
    }
}

impl ::core::fmt::Display for Ep1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 2 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep2r(pub u16);
impl Ep2r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ep2r {
    #[inline]
    fn from(other: u16) -> Self {
         Ep2r(other)
    }
}

impl ::core::fmt::Display for Ep2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep2r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 3 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep3r(pub u16);
impl Ep3r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ep3r {
    #[inline]
    fn from(other: u16) -> Self {
         Ep3r(other)
    }
}

impl ::core::fmt::Display for Ep3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep3r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 4 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep4r(pub u16);
impl Ep4r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ep4r {
    #[inline]
    fn from(other: u16) -> Self {
         Ep4r(other)
    }
}

impl ::core::fmt::Display for Ep4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep4r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 5 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep5r(pub u16);
impl Ep5r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ep5r {
    #[inline]
    fn from(other: u16) -> Self {
         Ep5r(other)
    }
}

impl ::core::fmt::Display for Ep5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep5r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 6 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep6r(pub u16);
impl Ep6r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ep6r {
    #[inline]
    fn from(other: u16) -> Self {
         Ep6r(other)
    }
}

impl ::core::fmt::Display for Ep6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep6r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="endpoint 7 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ep7r(pub u16);
impl Ep7r {
    #[doc="Endpoint address"]
    #[inline] pub fn ea(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EA != 0"]
    #[inline] pub fn test_ea(&self) -> bool {
        self.ea() != 0
    }

    #[doc="Sets the EA field."]
    #[inline] pub fn set_ea<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Status bits, for transmission transfers"]
    #[inline] pub fn stat_tx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STAT_TX != 0"]
    #[inline] pub fn test_stat_tx(&self) -> bool {
        self.stat_tx() != 0
    }

    #[doc="Sets the STAT_TX field."]
    #[inline] pub fn set_stat_tx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data Toggle, for transmission transfers"]
    #[inline] pub fn dtog_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DTOG_TX != 0"]
    #[inline] pub fn test_dtog_tx(&self) -> bool {
        self.dtog_tx() != 0
    }

    #[doc="Sets the DTOG_TX field."]
    #[inline] pub fn set_dtog_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Correct Transfer for transmission"]
    #[inline] pub fn ctr_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTR_TX != 0"]
    #[inline] pub fn test_ctr_tx(&self) -> bool {
        self.ctr_tx() != 0
    }

    #[doc="Sets the CTR_TX field."]
    #[inline] pub fn set_ctr_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Endpoint kind"]
    #[inline] pub fn ep_kind(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EP_KIND != 0"]
    #[inline] pub fn test_ep_kind(&self) -> bool {
        self.ep_kind() != 0
    }

    #[doc="Sets the EP_KIND field."]
    #[inline] pub fn set_ep_kind<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn ep_type(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if EP_TYPE != 0"]
    #[inline] pub fn test_ep_type(&self) -> bool {
        self.ep_type() != 0
    }

    #[doc="Sets the EP_TYPE field."]
    #[inline] pub fn set_ep_type<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Setup transaction completed"]
    #[inline] pub fn setup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SETUP != 0"]
    #[inline] pub fn test_setup(&self) -> bool {
        self.setup() != 0
    }

    #[doc="Sets the SETUP field."]
    #[inline] pub fn set_setup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Status bits, for reception transfers"]
    #[inline] pub fn stat_rx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STAT_RX != 0"]
    #[inline] pub fn test_stat_rx(&self) -> bool {
        self.stat_rx() != 0
    }

    #[doc="Sets the STAT_RX field."]
    #[inline] pub fn set_stat_rx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data Toggle, for reception transfers"]
    #[inline] pub fn dtog_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DTOG_RX != 0"]
    #[inline] pub fn test_dtog_rx(&self) -> bool {
        self.dtog_rx() != 0
    }

    #[doc="Sets the DTOG_RX field."]
    #[inline] pub fn set_dtog_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer for reception"]
    #[inline] pub fn ctr_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR_RX != 0"]
    #[inline] pub fn test_ctr_rx(&self) -> bool {
        self.ctr_rx() != 0
    }

    #[doc="Sets the CTR_RX field."]
    #[inline] pub fn set_ctr_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ep7r {
    #[inline]
    fn from(other: u16) -> Self {
         Ep7r(other)
    }
}

impl ::core::fmt::Display for Ep7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ep7r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ea() != 0 { try!(write!(f, " ea=0x{:x}", self.ea()))}
        if self.stat_tx() != 0 { try!(write!(f, " stat_tx=0x{:x}", self.stat_tx()))}
        if self.dtog_tx() != 0 { try!(write!(f, " dtog_tx"))}
        if self.ctr_tx() != 0 { try!(write!(f, " ctr_tx"))}
        if self.ep_kind() != 0 { try!(write!(f, " ep_kind"))}
        if self.ep_type() != 0 { try!(write!(f, " ep_type=0x{:x}", self.ep_type()))}
        if self.setup() != 0 { try!(write!(f, " setup"))}
        if self.stat_rx() != 0 { try!(write!(f, " stat_rx=0x{:x}", self.stat_rx()))}
        if self.dtog_rx() != 0 { try!(write!(f, " dtog_rx"))}
        if self.ctr_rx() != 0 { try!(write!(f, " ctr_rx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cntr(pub u16);
impl Cntr {
    #[doc="Force USB Reset"]
    #[inline] pub fn fres(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FRES != 0"]
    #[inline] pub fn test_fres(&self) -> bool {
        self.fres() != 0
    }

    #[doc="Sets the FRES field."]
    #[inline] pub fn set_fres<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Power down"]
    #[inline] pub fn pdwn(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PDWN != 0"]
    #[inline] pub fn test_pdwn(&self) -> bool {
        self.pdwn() != 0
    }

    #[doc="Sets the PDWN field."]
    #[inline] pub fn set_pdwn<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Low-power mode"]
    #[inline] pub fn lpmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LPMODE != 0"]
    #[inline] pub fn test_lpmode(&self) -> bool {
        self.lpmode() != 0
    }

    #[doc="Sets the LPMODE field."]
    #[inline] pub fn set_lpmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Force suspend"]
    #[inline] pub fn fsusp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FSUSP != 0"]
    #[inline] pub fn test_fsusp(&self) -> bool {
        self.fsusp() != 0
    }

    #[doc="Sets the FSUSP field."]
    #[inline] pub fn set_fsusp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Resume request"]
    #[inline] pub fn resume(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RESUME != 0"]
    #[inline] pub fn test_resume(&self) -> bool {
        self.resume() != 0
    }

    #[doc="Sets the RESUME field."]
    #[inline] pub fn set_resume<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="LPM L1 Resume request"]
    #[inline] pub fn l1resume(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if L1RESUME != 0"]
    #[inline] pub fn test_l1resume(&self) -> bool {
        self.l1resume() != 0
    }

    #[doc="Sets the L1RESUME field."]
    #[inline] pub fn set_l1resume<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="LPM L1 state request interrupt mask"]
    #[inline] pub fn l1reqm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if L1REQM != 0"]
    #[inline] pub fn test_l1reqm(&self) -> bool {
        self.l1reqm() != 0
    }

    #[doc="Sets the L1REQM field."]
    #[inline] pub fn set_l1reqm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Expected start of frame interrupt mask"]
    #[inline] pub fn esofm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ESOFM != 0"]
    #[inline] pub fn test_esofm(&self) -> bool {
        self.esofm() != 0
    }

    #[doc="Sets the ESOFM field."]
    #[inline] pub fn set_esofm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Start of frame interrupt mask"]
    #[inline] pub fn sofm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SOFM != 0"]
    #[inline] pub fn test_sofm(&self) -> bool {
        self.sofm() != 0
    }

    #[doc="Sets the SOFM field."]
    #[inline] pub fn set_sofm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="USB reset interrupt mask"]
    #[inline] pub fn resetm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RESETM != 0"]
    #[inline] pub fn test_resetm(&self) -> bool {
        self.resetm() != 0
    }

    #[doc="Sets the RESETM field."]
    #[inline] pub fn set_resetm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Suspend mode interrupt mask"]
    #[inline] pub fn suspm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SUSPM != 0"]
    #[inline] pub fn test_suspm(&self) -> bool {
        self.suspm() != 0
    }

    #[doc="Sets the SUSPM field."]
    #[inline] pub fn set_suspm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Wakeup interrupt mask"]
    #[inline] pub fn wkupm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WKUPM != 0"]
    #[inline] pub fn test_wkupm(&self) -> bool {
        self.wkupm() != 0
    }

    #[doc="Sets the WKUPM field."]
    #[inline] pub fn set_wkupm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Error interrupt mask"]
    #[inline] pub fn errm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ERRM != 0"]
    #[inline] pub fn test_errm(&self) -> bool {
        self.errm() != 0
    }

    #[doc="Sets the ERRM field."]
    #[inline] pub fn set_errm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Packet memory area over / underrun interrupt mask"]
    #[inline] pub fn pmaovrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PMAOVRM != 0"]
    #[inline] pub fn test_pmaovrm(&self) -> bool {
        self.pmaovrm() != 0
    }

    #[doc="Sets the PMAOVRM field."]
    #[inline] pub fn set_pmaovrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer interrupt mask"]
    #[inline] pub fn ctrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTRM != 0"]
    #[inline] pub fn test_ctrm(&self) -> bool {
        self.ctrm() != 0
    }

    #[doc="Sets the CTRM field."]
    #[inline] pub fn set_ctrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Cntr {
    #[inline]
    fn from(other: u16) -> Self {
         Cntr(other)
    }
}

impl ::core::fmt::Display for Cntr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cntr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fres() != 0 { try!(write!(f, " fres"))}
        if self.pdwn() != 0 { try!(write!(f, " pdwn"))}
        if self.lpmode() != 0 { try!(write!(f, " lpmode"))}
        if self.fsusp() != 0 { try!(write!(f, " fsusp"))}
        if self.resume() != 0 { try!(write!(f, " resume"))}
        if self.l1resume() != 0 { try!(write!(f, " l1resume"))}
        if self.l1reqm() != 0 { try!(write!(f, " l1reqm"))}
        if self.esofm() != 0 { try!(write!(f, " esofm"))}
        if self.sofm() != 0 { try!(write!(f, " sofm"))}
        if self.resetm() != 0 { try!(write!(f, " resetm"))}
        if self.suspm() != 0 { try!(write!(f, " suspm"))}
        if self.wkupm() != 0 { try!(write!(f, " wkupm"))}
        if self.errm() != 0 { try!(write!(f, " errm"))}
        if self.pmaovrm() != 0 { try!(write!(f, " pmaovrm"))}
        if self.ctrm() != 0 { try!(write!(f, " ctrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Istr(pub u16);
impl Istr {
    #[doc="Endpoint Identifier"]
    #[inline] pub fn ep_id(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EP_ID != 0"]
    #[inline] pub fn test_ep_id(&self) -> bool {
        self.ep_id() != 0
    }

    #[doc="Sets the EP_ID field."]
    #[inline] pub fn set_ep_id<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Direction of transaction"]
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="LPM L1 state request"]
    #[inline] pub fn l1req(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if L1REQ != 0"]
    #[inline] pub fn test_l1req(&self) -> bool {
        self.l1req() != 0
    }

    #[doc="Sets the L1REQ field."]
    #[inline] pub fn set_l1req<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Expected start frame"]
    #[inline] pub fn esof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ESOF != 0"]
    #[inline] pub fn test_esof(&self) -> bool {
        self.esof() != 0
    }

    #[doc="Sets the ESOF field."]
    #[inline] pub fn set_esof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="start of frame"]
    #[inline] pub fn sof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SOF != 0"]
    #[inline] pub fn test_sof(&self) -> bool {
        self.sof() != 0
    }

    #[doc="Sets the SOF field."]
    #[inline] pub fn set_sof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="reset request"]
    #[inline] pub fn _reset(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RESET != 0"]
    #[inline] pub fn test_reset(&self) -> bool {
        self._reset() != 0
    }

    #[doc="Sets the RESET field."]
    #[inline] pub fn set_reset<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Suspend mode request"]
    #[inline] pub fn susp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Wakeup"]
    #[inline] pub fn wkup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WKUP != 0"]
    #[inline] pub fn test_wkup(&self) -> bool {
        self.wkup() != 0
    }

    #[doc="Sets the WKUP field."]
    #[inline] pub fn set_wkup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Error"]
    #[inline] pub fn err(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Packet memory area over / underrun"]
    #[inline] pub fn pmaovr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PMAOVR != 0"]
    #[inline] pub fn test_pmaovr(&self) -> bool {
        self.pmaovr() != 0
    }

    #[doc="Sets the PMAOVR field."]
    #[inline] pub fn set_pmaovr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Correct transfer"]
    #[inline] pub fn ctr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTR != 0"]
    #[inline] pub fn test_ctr(&self) -> bool {
        self.ctr() != 0
    }

    #[doc="Sets the CTR field."]
    #[inline] pub fn set_ctr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Istr {
    #[inline]
    fn from(other: u16) -> Self {
         Istr(other)
    }
}

impl ::core::fmt::Display for Istr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Istr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ep_id() != 0 { try!(write!(f, " ep_id=0x{:x}", self.ep_id()))}
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.l1req() != 0 { try!(write!(f, " l1req"))}
        if self.esof() != 0 { try!(write!(f, " esof"))}
        if self.sof() != 0 { try!(write!(f, " sof"))}
        if self._reset() != 0 { try!(write!(f, " _reset"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        if self.wkup() != 0 { try!(write!(f, " wkup"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.pmaovr() != 0 { try!(write!(f, " pmaovr"))}
        if self.ctr() != 0 { try!(write!(f, " ctr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="frame number register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fnr(pub u16);
impl Fnr {
    #[doc="Frame number"]
    #[inline] pub fn _fn(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if FN != 0"]
    #[inline] pub fn test_fn(&self) -> bool {
        self._fn() != 0
    }

    #[doc="Sets the FN field."]
    #[inline] pub fn set_fn<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Lost SOF"]
    #[inline] pub fn lsof(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="Returns true if LSOF != 0"]
    #[inline] pub fn test_lsof(&self) -> bool {
        self.lsof() != 0
    }

    #[doc="Sets the LSOF field."]
    #[inline] pub fn set_lsof<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Locked"]
    #[inline] pub fn lck(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if LCK != 0"]
    #[inline] pub fn test_lck(&self) -> bool {
        self.lck() != 0
    }

    #[doc="Sets the LCK field."]
    #[inline] pub fn set_lck<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Receive data - line status"]
    #[inline] pub fn rxdm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RXDM != 0"]
    #[inline] pub fn test_rxdm(&self) -> bool {
        self.rxdm() != 0
    }

    #[doc="Sets the RXDM field."]
    #[inline] pub fn set_rxdm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Receive data + line status"]
    #[inline] pub fn rxdp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RXDP != 0"]
    #[inline] pub fn test_rxdp(&self) -> bool {
        self.rxdp() != 0
    }

    #[doc="Sets the RXDP field."]
    #[inline] pub fn set_rxdp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Fnr {
    #[inline]
    fn from(other: u16) -> Self {
         Fnr(other)
    }
}

impl ::core::fmt::Display for Fnr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fnr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self._fn() != 0 { try!(write!(f, " fn=0x{:x}", self._fn()))}
        if self.lsof() != 0 { try!(write!(f, " lsof=0x{:x}", self.lsof()))}
        if self.lck() != 0 { try!(write!(f, " lck"))}
        if self.rxdm() != 0 { try!(write!(f, " rxdm"))}
        if self.rxdp() != 0 { try!(write!(f, " rxdp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Daddr(pub u16);
impl Daddr {
    #[doc="Device address"]
    #[inline] pub fn add(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if ADD != 0"]
    #[inline] pub fn test_add(&self) -> bool {
        self.add() != 0
    }

    #[doc="Sets the ADD field."]
    #[inline] pub fn set_add<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable function"]
    #[inline] pub fn ef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EF != 0"]
    #[inline] pub fn test_ef(&self) -> bool {
        self.ef() != 0
    }

    #[doc="Sets the EF field."]
    #[inline] pub fn set_ef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u16> for Daddr {
    #[inline]
    fn from(other: u16) -> Self {
         Daddr(other)
    }
}

impl ::core::fmt::Display for Daddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Daddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.add() != 0 { try!(write!(f, " add=0x{:x}", self.add()))}
        if self.ef() != 0 { try!(write!(f, " ef"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Buffer table address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Btable(pub u16);
impl Btable {
    #[doc="Buffer table"]
    #[inline] pub fn btable(&self) -> ::bobbin_bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1fff) as u16) } // [15:3]
    }

    #[doc="Returns true if BTABLE != 0"]
    #[inline] pub fn test_btable(&self) -> bool {
        self.btable() != 0
    }

    #[doc="Sets the BTABLE field."]
    #[inline] pub fn set_btable<V: Into<::bobbin_bits::U13>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U13 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1fff << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u16> for Btable {
    #[inline]
    fn from(other: u16) -> Self {
         Btable(other)
    }
}

impl ::core::fmt::Display for Btable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Btable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.btable() != 0 { try!(write!(f, " btable=0x{:x}", self.btable()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmission byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count0Tx(pub u16);
impl Count0Tx {
    #[doc="Transmission byte count"]
    #[inline] pub fn count0_tx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT0_TX != 0"]
    #[inline] pub fn test_count0_tx(&self) -> bool {
        self.count0_tx() != 0
    }

    #[doc="Sets the COUNT0_TX field."]
    #[inline] pub fn set_count0_tx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Count0Tx {
    #[inline]
    fn from(other: u16) -> Self {
         Count0Tx(other)
    }
}

impl ::core::fmt::Display for Count0Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count0Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count0_tx() != 0 { try!(write!(f, " count0_tx=0x{:x}", self.count0_tx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmission byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count1Tx(pub u16);
impl Count1Tx {
    #[doc="Transmission byte count"]
    #[inline] pub fn count1_tx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT1_TX != 0"]
    #[inline] pub fn test_count1_tx(&self) -> bool {
        self.count1_tx() != 0
    }

    #[doc="Sets the COUNT1_TX field."]
    #[inline] pub fn set_count1_tx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Count1Tx {
    #[inline]
    fn from(other: u16) -> Self {
         Count1Tx(other)
    }
}

impl ::core::fmt::Display for Count1Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count1Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count1_tx() != 0 { try!(write!(f, " count1_tx=0x{:x}", self.count1_tx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmission byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count2Tx(pub u16);
impl Count2Tx {
    #[doc="Transmission byte count"]
    #[inline] pub fn count2_tx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT2_TX != 0"]
    #[inline] pub fn test_count2_tx(&self) -> bool {
        self.count2_tx() != 0
    }

    #[doc="Sets the COUNT2_TX field."]
    #[inline] pub fn set_count2_tx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Count2Tx {
    #[inline]
    fn from(other: u16) -> Self {
         Count2Tx(other)
    }
}

impl ::core::fmt::Display for Count2Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count2Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count2_tx() != 0 { try!(write!(f, " count2_tx=0x{:x}", self.count2_tx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmission byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count3Tx(pub u16);
impl Count3Tx {
    #[doc="Transmission byte count"]
    #[inline] pub fn count3_tx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT3_TX != 0"]
    #[inline] pub fn test_count3_tx(&self) -> bool {
        self.count3_tx() != 0
    }

    #[doc="Sets the COUNT3_TX field."]
    #[inline] pub fn set_count3_tx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Count3Tx {
    #[inline]
    fn from(other: u16) -> Self {
         Count3Tx(other)
    }
}

impl ::core::fmt::Display for Count3Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count3Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count3_tx() != 0 { try!(write!(f, " count3_tx=0x{:x}", self.count3_tx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmission byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count4Tx(pub u16);
impl Count4Tx {
    #[doc="Transmission byte count"]
    #[inline] pub fn count4_tx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT4_TX != 0"]
    #[inline] pub fn test_count4_tx(&self) -> bool {
        self.count4_tx() != 0
    }

    #[doc="Sets the COUNT4_TX field."]
    #[inline] pub fn set_count4_tx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Count4Tx {
    #[inline]
    fn from(other: u16) -> Self {
         Count4Tx(other)
    }
}

impl ::core::fmt::Display for Count4Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count4Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count4_tx() != 0 { try!(write!(f, " count4_tx=0x{:x}", self.count4_tx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmission byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count5Tx(pub u16);
impl Count5Tx {
    #[doc="Transmission byte count"]
    #[inline] pub fn count5_tx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT5_TX != 0"]
    #[inline] pub fn test_count5_tx(&self) -> bool {
        self.count5_tx() != 0
    }

    #[doc="Sets the COUNT5_TX field."]
    #[inline] pub fn set_count5_tx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Count5Tx {
    #[inline]
    fn from(other: u16) -> Self {
         Count5Tx(other)
    }
}

impl ::core::fmt::Display for Count5Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count5Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count5_tx() != 0 { try!(write!(f, " count5_tx=0x{:x}", self.count5_tx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmission byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count6Tx(pub u16);
impl Count6Tx {
    #[doc="Transmission byte count"]
    #[inline] pub fn count6_tx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT6_TX != 0"]
    #[inline] pub fn test_count6_tx(&self) -> bool {
        self.count6_tx() != 0
    }

    #[doc="Sets the COUNT6_TX field."]
    #[inline] pub fn set_count6_tx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Count6Tx {
    #[inline]
    fn from(other: u16) -> Self {
         Count6Tx(other)
    }
}

impl ::core::fmt::Display for Count6Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count6Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count6_tx() != 0 { try!(write!(f, " count6_tx=0x{:x}", self.count6_tx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmission byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count7Tx(pub u16);
impl Count7Tx {
    #[doc="Transmission byte count"]
    #[inline] pub fn count7_tx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT7_TX != 0"]
    #[inline] pub fn test_count7_tx(&self) -> bool {
        self.count7_tx() != 0
    }

    #[doc="Sets the COUNT7_TX field."]
    #[inline] pub fn set_count7_tx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Count7Tx {
    #[inline]
    fn from(other: u16) -> Self {
         Count7Tx(other)
    }
}

impl ::core::fmt::Display for Count7Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count7Tx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count7_tx() != 0 { try!(write!(f, " count7_tx=0x{:x}", self.count7_tx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception buffer address 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr0Rx(pub u16);
impl Addr0Rx {
    #[doc="Reception buffer address"]
    #[inline] pub fn addr0_rx(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7fff) as u16) } // [15:1]
    }

    #[doc="Returns true if ADDR0_RX != 0"]
    #[inline] pub fn test_addr0_rx(&self) -> bool {
        self.addr0_rx() != 0
    }

    #[doc="Sets the ADDR0_RX field."]
    #[inline] pub fn set_addr0_rx<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7fff << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u16> for Addr0Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Addr0Rx(other)
    }
}

impl ::core::fmt::Display for Addr0Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr0Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr0_rx() != 0 { try!(write!(f, " addr0_rx=0x{:x}", self.addr0_rx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception buffer address 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr1Rx(pub u16);
impl Addr1Rx {
    #[doc="Reception buffer address"]
    #[inline] pub fn addr1_rx(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7fff) as u16) } // [15:1]
    }

    #[doc="Returns true if ADDR1_RX != 0"]
    #[inline] pub fn test_addr1_rx(&self) -> bool {
        self.addr1_rx() != 0
    }

    #[doc="Sets the ADDR1_RX field."]
    #[inline] pub fn set_addr1_rx<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7fff << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u16> for Addr1Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Addr1Rx(other)
    }
}

impl ::core::fmt::Display for Addr1Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr1Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr1_rx() != 0 { try!(write!(f, " addr1_rx=0x{:x}", self.addr1_rx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception buffer address 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr2Rx(pub u16);
impl Addr2Rx {
    #[doc="Reception buffer address"]
    #[inline] pub fn addr2_rx(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7fff) as u16) } // [15:1]
    }

    #[doc="Returns true if ADDR2_RX != 0"]
    #[inline] pub fn test_addr2_rx(&self) -> bool {
        self.addr2_rx() != 0
    }

    #[doc="Sets the ADDR2_RX field."]
    #[inline] pub fn set_addr2_rx<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7fff << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u16> for Addr2Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Addr2Rx(other)
    }
}

impl ::core::fmt::Display for Addr2Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr2Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr2_rx() != 0 { try!(write!(f, " addr2_rx=0x{:x}", self.addr2_rx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception buffer address 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr3Rx(pub u16);
impl Addr3Rx {
    #[doc="Reception buffer address"]
    #[inline] pub fn addr3_rx(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7fff) as u16) } // [15:1]
    }

    #[doc="Returns true if ADDR3_RX != 0"]
    #[inline] pub fn test_addr3_rx(&self) -> bool {
        self.addr3_rx() != 0
    }

    #[doc="Sets the ADDR3_RX field."]
    #[inline] pub fn set_addr3_rx<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7fff << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u16> for Addr3Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Addr3Rx(other)
    }
}

impl ::core::fmt::Display for Addr3Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr3Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr3_rx() != 0 { try!(write!(f, " addr3_rx=0x{:x}", self.addr3_rx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception buffer address 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr4Rx(pub u16);
impl Addr4Rx {
    #[doc="Reception buffer address"]
    #[inline] pub fn addr4_rx(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7fff) as u16) } // [15:1]
    }

    #[doc="Returns true if ADDR4_RX != 0"]
    #[inline] pub fn test_addr4_rx(&self) -> bool {
        self.addr4_rx() != 0
    }

    #[doc="Sets the ADDR4_RX field."]
    #[inline] pub fn set_addr4_rx<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7fff << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u16> for Addr4Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Addr4Rx(other)
    }
}

impl ::core::fmt::Display for Addr4Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr4Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr4_rx() != 0 { try!(write!(f, " addr4_rx=0x{:x}", self.addr4_rx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception buffer address 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr5Rx(pub u16);
impl Addr5Rx {
    #[doc="Reception buffer address"]
    #[inline] pub fn addr5_rx(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7fff) as u16) } // [15:1]
    }

    #[doc="Returns true if ADDR5_RX != 0"]
    #[inline] pub fn test_addr5_rx(&self) -> bool {
        self.addr5_rx() != 0
    }

    #[doc="Sets the ADDR5_RX field."]
    #[inline] pub fn set_addr5_rx<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7fff << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u16> for Addr5Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Addr5Rx(other)
    }
}

impl ::core::fmt::Display for Addr5Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr5Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr5_rx() != 0 { try!(write!(f, " addr5_rx=0x{:x}", self.addr5_rx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception buffer address 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr6Rx(pub u16);
impl Addr6Rx {
    #[doc="Reception buffer address"]
    #[inline] pub fn addr6_rx(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7fff) as u16) } // [15:1]
    }

    #[doc="Returns true if ADDR6_RX != 0"]
    #[inline] pub fn test_addr6_rx(&self) -> bool {
        self.addr6_rx() != 0
    }

    #[doc="Sets the ADDR6_RX field."]
    #[inline] pub fn set_addr6_rx<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7fff << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u16> for Addr6Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Addr6Rx(other)
    }
}

impl ::core::fmt::Display for Addr6Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr6Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr6_rx() != 0 { try!(write!(f, " addr6_rx=0x{:x}", self.addr6_rx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception buffer address 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr7Rx(pub u16);
impl Addr7Rx {
    #[doc="Reception buffer address"]
    #[inline] pub fn addr7_rx(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7fff) as u16) } // [15:1]
    }

    #[doc="Returns true if ADDR7_RX != 0"]
    #[inline] pub fn test_addr7_rx(&self) -> bool {
        self.addr7_rx() != 0
    }

    #[doc="Sets the ADDR7_RX field."]
    #[inline] pub fn set_addr7_rx<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7fff << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u16> for Addr7Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Addr7Rx(other)
    }
}

impl ::core::fmt::Display for Addr7Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr7Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr7_rx() != 0 { try!(write!(f, " addr7_rx=0x{:x}", self.addr7_rx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count0Rx(pub u16);
impl Count0Rx {
    #[doc="Reception byte count"]
    #[inline] pub fn count0_rx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT0_RX != 0"]
    #[inline] pub fn test_count0_rx(&self) -> bool {
        self.count0_rx() != 0
    }

    #[doc="Sets the COUNT0_RX field."]
    #[inline] pub fn set_count0_rx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of blocks"]
    #[inline] pub fn num_block(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if NUM_BLOCK != 0"]
    #[inline] pub fn test_num_block(&self) -> bool {
        self.num_block() != 0
    }

    #[doc="Sets the NUM_BLOCK field."]
    #[inline] pub fn set_num_block<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Block size"]
    #[inline] pub fn bl_size(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BL_SIZE != 0"]
    #[inline] pub fn test_bl_size(&self) -> bool {
        self.bl_size() != 0
    }

    #[doc="Sets the BL_SIZE field."]
    #[inline] pub fn set_bl_size<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Count0Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Count0Rx(other)
    }
}

impl ::core::fmt::Display for Count0Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count0Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count0_rx() != 0 { try!(write!(f, " count0_rx=0x{:x}", self.count0_rx()))}
        if self.num_block() != 0 { try!(write!(f, " num_block=0x{:x}", self.num_block()))}
        if self.bl_size() != 0 { try!(write!(f, " bl_size"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count1Rx(pub u16);
impl Count1Rx {
    #[doc="Reception byte count"]
    #[inline] pub fn count1_rx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT1_RX != 0"]
    #[inline] pub fn test_count1_rx(&self) -> bool {
        self.count1_rx() != 0
    }

    #[doc="Sets the COUNT1_RX field."]
    #[inline] pub fn set_count1_rx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of blocks"]
    #[inline] pub fn num_block(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if NUM_BLOCK != 0"]
    #[inline] pub fn test_num_block(&self) -> bool {
        self.num_block() != 0
    }

    #[doc="Sets the NUM_BLOCK field."]
    #[inline] pub fn set_num_block<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Block size"]
    #[inline] pub fn bl_size(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BL_SIZE != 0"]
    #[inline] pub fn test_bl_size(&self) -> bool {
        self.bl_size() != 0
    }

    #[doc="Sets the BL_SIZE field."]
    #[inline] pub fn set_bl_size<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Count1Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Count1Rx(other)
    }
}

impl ::core::fmt::Display for Count1Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count1Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count1_rx() != 0 { try!(write!(f, " count1_rx=0x{:x}", self.count1_rx()))}
        if self.num_block() != 0 { try!(write!(f, " num_block=0x{:x}", self.num_block()))}
        if self.bl_size() != 0 { try!(write!(f, " bl_size"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count2Rx(pub u16);
impl Count2Rx {
    #[doc="Reception byte count"]
    #[inline] pub fn count2_rx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT2_RX != 0"]
    #[inline] pub fn test_count2_rx(&self) -> bool {
        self.count2_rx() != 0
    }

    #[doc="Sets the COUNT2_RX field."]
    #[inline] pub fn set_count2_rx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of blocks"]
    #[inline] pub fn num_block(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if NUM_BLOCK != 0"]
    #[inline] pub fn test_num_block(&self) -> bool {
        self.num_block() != 0
    }

    #[doc="Sets the NUM_BLOCK field."]
    #[inline] pub fn set_num_block<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Block size"]
    #[inline] pub fn bl_size(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BL_SIZE != 0"]
    #[inline] pub fn test_bl_size(&self) -> bool {
        self.bl_size() != 0
    }

    #[doc="Sets the BL_SIZE field."]
    #[inline] pub fn set_bl_size<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Count2Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Count2Rx(other)
    }
}

impl ::core::fmt::Display for Count2Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count2Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count2_rx() != 0 { try!(write!(f, " count2_rx=0x{:x}", self.count2_rx()))}
        if self.num_block() != 0 { try!(write!(f, " num_block=0x{:x}", self.num_block()))}
        if self.bl_size() != 0 { try!(write!(f, " bl_size"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count3Rx(pub u16);
impl Count3Rx {
    #[doc="Reception byte count"]
    #[inline] pub fn count3_rx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT3_RX != 0"]
    #[inline] pub fn test_count3_rx(&self) -> bool {
        self.count3_rx() != 0
    }

    #[doc="Sets the COUNT3_RX field."]
    #[inline] pub fn set_count3_rx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of blocks"]
    #[inline] pub fn num_block(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if NUM_BLOCK != 0"]
    #[inline] pub fn test_num_block(&self) -> bool {
        self.num_block() != 0
    }

    #[doc="Sets the NUM_BLOCK field."]
    #[inline] pub fn set_num_block<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Block size"]
    #[inline] pub fn bl_size(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BL_SIZE != 0"]
    #[inline] pub fn test_bl_size(&self) -> bool {
        self.bl_size() != 0
    }

    #[doc="Sets the BL_SIZE field."]
    #[inline] pub fn set_bl_size<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Count3Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Count3Rx(other)
    }
}

impl ::core::fmt::Display for Count3Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count3Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count3_rx() != 0 { try!(write!(f, " count3_rx=0x{:x}", self.count3_rx()))}
        if self.num_block() != 0 { try!(write!(f, " num_block=0x{:x}", self.num_block()))}
        if self.bl_size() != 0 { try!(write!(f, " bl_size"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count4Rx(pub u16);
impl Count4Rx {
    #[doc="Reception byte count"]
    #[inline] pub fn count4_rx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT4_RX != 0"]
    #[inline] pub fn test_count4_rx(&self) -> bool {
        self.count4_rx() != 0
    }

    #[doc="Sets the COUNT4_RX field."]
    #[inline] pub fn set_count4_rx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of blocks"]
    #[inline] pub fn num_block(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if NUM_BLOCK != 0"]
    #[inline] pub fn test_num_block(&self) -> bool {
        self.num_block() != 0
    }

    #[doc="Sets the NUM_BLOCK field."]
    #[inline] pub fn set_num_block<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Block size"]
    #[inline] pub fn bl_size(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BL_SIZE != 0"]
    #[inline] pub fn test_bl_size(&self) -> bool {
        self.bl_size() != 0
    }

    #[doc="Sets the BL_SIZE field."]
    #[inline] pub fn set_bl_size<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Count4Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Count4Rx(other)
    }
}

impl ::core::fmt::Display for Count4Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count4Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count4_rx() != 0 { try!(write!(f, " count4_rx=0x{:x}", self.count4_rx()))}
        if self.num_block() != 0 { try!(write!(f, " num_block=0x{:x}", self.num_block()))}
        if self.bl_size() != 0 { try!(write!(f, " bl_size"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count5Rx(pub u16);
impl Count5Rx {
    #[doc="Reception byte count"]
    #[inline] pub fn count5_rx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT5_RX != 0"]
    #[inline] pub fn test_count5_rx(&self) -> bool {
        self.count5_rx() != 0
    }

    #[doc="Sets the COUNT5_RX field."]
    #[inline] pub fn set_count5_rx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of blocks"]
    #[inline] pub fn num_block(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if NUM_BLOCK != 0"]
    #[inline] pub fn test_num_block(&self) -> bool {
        self.num_block() != 0
    }

    #[doc="Sets the NUM_BLOCK field."]
    #[inline] pub fn set_num_block<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Block size"]
    #[inline] pub fn bl_size(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BL_SIZE != 0"]
    #[inline] pub fn test_bl_size(&self) -> bool {
        self.bl_size() != 0
    }

    #[doc="Sets the BL_SIZE field."]
    #[inline] pub fn set_bl_size<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Count5Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Count5Rx(other)
    }
}

impl ::core::fmt::Display for Count5Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count5Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count5_rx() != 0 { try!(write!(f, " count5_rx=0x{:x}", self.count5_rx()))}
        if self.num_block() != 0 { try!(write!(f, " num_block=0x{:x}", self.num_block()))}
        if self.bl_size() != 0 { try!(write!(f, " bl_size"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count6Rx(pub u16);
impl Count6Rx {
    #[doc="Reception byte count"]
    #[inline] pub fn count6_rx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT6_RX != 0"]
    #[inline] pub fn test_count6_rx(&self) -> bool {
        self.count6_rx() != 0
    }

    #[doc="Sets the COUNT6_RX field."]
    #[inline] pub fn set_count6_rx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of blocks"]
    #[inline] pub fn num_block(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if NUM_BLOCK != 0"]
    #[inline] pub fn test_num_block(&self) -> bool {
        self.num_block() != 0
    }

    #[doc="Sets the NUM_BLOCK field."]
    #[inline] pub fn set_num_block<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Block size"]
    #[inline] pub fn bl_size(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BL_SIZE != 0"]
    #[inline] pub fn test_bl_size(&self) -> bool {
        self.bl_size() != 0
    }

    #[doc="Sets the BL_SIZE field."]
    #[inline] pub fn set_bl_size<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Count6Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Count6Rx(other)
    }
}

impl ::core::fmt::Display for Count6Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count6Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count6_rx() != 0 { try!(write!(f, " count6_rx=0x{:x}", self.count6_rx()))}
        if self.num_block() != 0 { try!(write!(f, " num_block=0x{:x}", self.num_block()))}
        if self.bl_size() != 0 { try!(write!(f, " bl_size"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reception byte count 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count7Rx(pub u16);
impl Count7Rx {
    #[doc="Reception byte count"]
    #[inline] pub fn count7_rx(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if COUNT7_RX != 0"]
    #[inline] pub fn test_count7_rx(&self) -> bool {
        self.count7_rx() != 0
    }

    #[doc="Sets the COUNT7_RX field."]
    #[inline] pub fn set_count7_rx<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Number of blocks"]
    #[inline] pub fn num_block(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1f) as u8) } // [14:10]
    }

    #[doc="Returns true if NUM_BLOCK != 0"]
    #[inline] pub fn test_num_block(&self) -> bool {
        self.num_block() != 0
    }

    #[doc="Sets the NUM_BLOCK field."]
    #[inline] pub fn set_num_block<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Block size"]
    #[inline] pub fn bl_size(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BL_SIZE != 0"]
    #[inline] pub fn test_bl_size(&self) -> bool {
        self.bl_size() != 0
    }

    #[doc="Sets the BL_SIZE field."]
    #[inline] pub fn set_bl_size<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Count7Rx {
    #[inline]
    fn from(other: u16) -> Self {
         Count7Rx(other)
    }
}

impl ::core::fmt::Display for Count7Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count7Rx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count7_rx() != 0 { try!(write!(f, " count7_rx=0x{:x}", self.count7_rx()))}
        if self.num_block() != 0 { try!(write!(f, " num_block=0x{:x}", self.num_block()))}
        if self.bl_size() != 0 { try!(write!(f, " bl_size"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpmcsr(pub u16);
impl Lpmcsr {
    #[doc="LPM support enable"]
    #[inline] pub fn lpmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPMEN != 0"]
    #[inline] pub fn test_lpmen(&self) -> bool {
        self.lpmen() != 0
    }

    #[doc="Sets the LPMEN field."]
    #[inline] pub fn set_lpmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="LPM Token acknowledge enable"]
    #[inline] pub fn lpmack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LPMACK != 0"]
    #[inline] pub fn test_lpmack(&self) -> bool {
        self.lpmack() != 0
    }

    #[doc="Sets the LPMACK field."]
    #[inline] pub fn set_lpmack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="RemoteWake value"]
    #[inline] pub fn remwake(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if REMWAKE != 0"]
    #[inline] pub fn test_remwake(&self) -> bool {
        self.remwake() != 0
    }

    #[doc="Sets the REMWAKE field."]
    #[inline] pub fn set_remwake<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="BESL value"]
    #[inline] pub fn besl(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if BESL != 0"]
    #[inline] pub fn test_besl(&self) -> bool {
        self.besl() != 0
    }

    #[doc="Sets the BESL field."]
    #[inline] pub fn set_besl<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u16> for Lpmcsr {
    #[inline]
    fn from(other: u16) -> Self {
         Lpmcsr(other)
    }
}

impl ::core::fmt::Display for Lpmcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lpmcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lpmen() != 0 { try!(write!(f, " lpmen"))}
        if self.lpmack() != 0 { try!(write!(f, " lpmack"))}
        if self.remwake() != 0 { try!(write!(f, " remwake"))}
        if self.besl() != 0 { try!(write!(f, " besl=0x{:x}", self.besl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Battery charging detector("]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bcdr(pub u16);
impl Bcdr {
    #[doc="Battery charging detector (BCD) enable"]
    #[inline] pub fn bcden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BCDEN != 0"]
    #[inline] pub fn test_bcden(&self) -> bool {
        self.bcden() != 0
    }

    #[doc="Sets the BCDEN field."]
    #[inline] pub fn set_bcden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data contact detection (DCD) mode enable"]
    #[inline] pub fn dcden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DCDEN != 0"]
    #[inline] pub fn test_dcden(&self) -> bool {
        self.dcden() != 0
    }

    #[doc="Sets the DCDEN field."]
    #[inline] pub fn set_dcden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Primary detection (PD) mode enable"]
    #[inline] pub fn pden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PDEN != 0"]
    #[inline] pub fn test_pden(&self) -> bool {
        self.pden() != 0
    }

    #[doc="Sets the PDEN field."]
    #[inline] pub fn set_pden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Secondary detection (SD) mode enable"]
    #[inline] pub fn sden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SDEN != 0"]
    #[inline] pub fn test_sden(&self) -> bool {
        self.sden() != 0
    }

    #[doc="Sets the SDEN field."]
    #[inline] pub fn set_sden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data contact detection (DCD) status"]
    #[inline] pub fn dcdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DCDET != 0"]
    #[inline] pub fn test_dcdet(&self) -> bool {
        self.dcdet() != 0
    }

    #[doc="Sets the DCDET field."]
    #[inline] pub fn set_dcdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Primary detection (PD) status"]
    #[inline] pub fn pdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PDET != 0"]
    #[inline] pub fn test_pdet(&self) -> bool {
        self.pdet() != 0
    }

    #[doc="Sets the PDET field."]
    #[inline] pub fn set_pdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Secondary detection (SD) status"]
    #[inline] pub fn sdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SDET != 0"]
    #[inline] pub fn test_sdet(&self) -> bool {
        self.sdet() != 0
    }

    #[doc="Sets the SDET field."]
    #[inline] pub fn set_sdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DM pull-up detection status"]
    #[inline] pub fn ps2det(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PS2DET != 0"]
    #[inline] pub fn test_ps2det(&self) -> bool {
        self.ps2det() != 0
    }

    #[doc="Sets the PS2DET field."]
    #[inline] pub fn set_ps2det<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DP pull-up control"]
    #[inline] pub fn dppu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DPPU != 0"]
    #[inline] pub fn test_dppu(&self) -> bool {
        self.dppu() != 0
    }

    #[doc="Sets the DPPU field."]
    #[inline] pub fn set_dppu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Bcdr {
    #[inline]
    fn from(other: u16) -> Self {
         Bcdr(other)
    }
}

impl ::core::fmt::Display for Bcdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bcdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bcden() != 0 { try!(write!(f, " bcden"))}
        if self.dcden() != 0 { try!(write!(f, " dcden"))}
        if self.pden() != 0 { try!(write!(f, " pden"))}
        if self.sden() != 0 { try!(write!(f, " sden"))}
        if self.dcdet() != 0 { try!(write!(f, " dcdet"))}
        if self.pdet() != 0 { try!(write!(f, " pdet"))}
        if self.sdet() != 0 { try!(write!(f, " sdet"))}
        if self.ps2det() != 0 { try!(write!(f, " ps2det"))}
        if self.dppu() != 0 { try!(write!(f, " dppu"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

