#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "DMA", peripherals: [], prototype: Some(Peripheral { derived_from: None, group_name: Some("DMA"), name: "", address: 0, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [], clusters: [], registers: [Register { name: "SAR", offset: 256, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Source Address Register"), fields: [Field { name: "SAR", bit_offset: 0, bit_width: 32, access: Some(ReadWrite), description: Some("Each SAR contains the byte address used by the DMA controller to read data"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(4), dim_increment: Some(16), dim_index: None }, Register { name: "DAR", offset: 260, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("Destination Address Register"), fields: [Field { name: "DAR", bit_offset: 0, bit_width: 32, access: Some(ReadWrite), description: Some("Each DAR contains the byte address used by the DMA controller to write data"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(4), dim_increment: Some(16), dim_index: None }, Register { name: "DSR_BCR", offset: 264, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("DMA Status Register / Byte Count Register"), fields: [Field { name: "BCR", bit_offset: 0, bit_width: 24, access: Some(ReadWrite), description: Some("This field contains the number of bytes yet to be transferred for a given block"), enumerated_values: [], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DONE", bit_offset: 24, bit_width: 1, access: Some(ReadWrite), description: Some("Transactions Done"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("DMA transfer is not yet complete. Writing a 0 has no effect.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("DMA transfer completed. Writing a 1 to this bit clears all DMA status bits and should be used in an interrupt service routine to clear the DMA interrupt and error bits.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BSY", bit_offset: 25, bit_width: 1, access: Some(ReadOnly), description: Some("Busy"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("DMA channel is inactive. Cleared when the DMA has finished the last transaction.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("BSY is set the first time the channel is enabled after a transfer is initiated.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "REQ", bit_offset: 26, bit_width: 1, access: Some(ReadOnly), description: Some("Request"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No request is pending or the channel is currently active. Cleared when the channel is selected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The DMA channel has a transfer remaining and the channel is not selected.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BED", bit_offset: 28, bit_width: 1, access: Some(ReadOnly), description: Some("Bus Error on Destination"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No bus error occurred.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The DMA channel terminated with a bus error during the write portion of a transfer.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "BES", bit_offset: 29, bit_width: 1, access: Some(ReadOnly), description: Some("Bus Error on Source"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No bus error occurred.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The DMA channel terminated with a bus error during the read portion of a transfer.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CE", bit_offset: 30, bit_width: 1, access: Some(ReadOnly), description: Some("Configuration Error"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No configuration error exists.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("A configuration error has occurred.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(4), dim_increment: Some(16), dim_index: None }, Register { name: "DSR", offset: 267, size: Some(8), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(255), description: Some("DMA_DSR0 register."), fields: [], dim: Some(4), dim_increment: Some(16), dim_index: None }, Register { name: "DCR", offset: 268, size: Some(32), access: Some(ReadWrite), reset_value: Some(0), reset_mask: Some(4294967295), description: Some("DMA Control Register"), fields: [Field { name: "LCH2", bit_offset: 0, bit_width: 2, access: Some(ReadWrite), description: Some("Link Channel 2"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("DMA Channel 0") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("DMA Channel 1") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("DMA Channel 2") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("DMA Channel 3") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LCH1", bit_offset: 2, bit_width: 2, access: Some(ReadWrite), description: Some("Link Channel 1"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("DMA Channel 0") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("DMA Channel 1") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("DMA Channel 2") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("DMA Channel 3") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "LINKCC", bit_offset: 4, bit_width: 2, access: Some(ReadWrite), description: Some("Link Channel Control"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("No channel-to-channel linking") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("Perform a link to channel LCH1 after each cycle-steal transfer followed by a link to LCH2 after the BCR decrements to 0.") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("Perform a link to channel LCH1 after each cycle-steal transfer") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("Perform a link to channel LCH1 after the BCR decrements to 0.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "D_REQ", bit_offset: 7, bit_width: 1, access: Some(ReadWrite), description: Some("Disable Request"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("ERQ bit is not affected.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("ERQ bit is cleared when the BCR is exhausted.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DMOD", bit_offset: 8, bit_width: 4, access: Some(ReadWrite), description: Some("Destination Address Modulo"), enumerated_values: [EnumeratedValue { value: "#0000", name: Some("0000"), description: Some("Buffer disabled") }, EnumeratedValue { value: "#0001", name: Some("0001"), description: Some("Circular buffer size is 16 bytes") }, EnumeratedValue { value: "#0010", name: Some("0010"), description: Some("Circular buffer size is 32 bytes") }, EnumeratedValue { value: "#0011", name: Some("0011"), description: Some("Circular buffer size is 64 bytes") }, EnumeratedValue { value: "#0100", name: Some("0100"), description: Some("Circular buffer size is 128 bytes") }, EnumeratedValue { value: "#0101", name: Some("0101"), description: Some("Circular buffer size is 256 bytes") }, EnumeratedValue { value: "#0110", name: Some("0110"), description: Some("Circular buffer size is 512 bytes") }, EnumeratedValue { value: "#0111", name: Some("0111"), description: Some("Circular buffer size is 1 KB") }, EnumeratedValue { value: "#1000", name: Some("1000"), description: Some("Circular buffer size is 2 KB") }, EnumeratedValue { value: "#1001", name: Some("1001"), description: Some("Circular buffer size is 4 KB") }, EnumeratedValue { value: "#1010", name: Some("1010"), description: Some("Circular buffer size is 8 KB") }, EnumeratedValue { value: "#1011", name: Some("1011"), description: Some("Circular buffer size is 16 KB") }, EnumeratedValue { value: "#1100", name: Some("1100"), description: Some("Circular buffer size is 32 KB") }, EnumeratedValue { value: "#1101", name: Some("1101"), description: Some("Circular buffer size is 64 KB") }, EnumeratedValue { value: "#1110", name: Some("1110"), description: Some("Circular buffer size is 128 KB") }, EnumeratedValue { value: "#1111", name: Some("1111"), description: Some("Circular buffer size is 256 KB") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SMOD", bit_offset: 12, bit_width: 4, access: Some(ReadWrite), description: Some("Source Address Modulo"), enumerated_values: [EnumeratedValue { value: "#0000", name: Some("0000"), description: Some("Buffer disabled") }, EnumeratedValue { value: "#0001", name: Some("0001"), description: Some("Circular buffer size is 16 bytes.") }, EnumeratedValue { value: "#0010", name: Some("0010"), description: Some("Circular buffer size is 32 bytes.") }, EnumeratedValue { value: "#0011", name: Some("0011"), description: Some("Circular buffer size is 64 bytes.") }, EnumeratedValue { value: "#0100", name: Some("0100"), description: Some("Circular buffer size is 128 bytes.") }, EnumeratedValue { value: "#0101", name: Some("0101"), description: Some("Circular buffer size is 256 bytes.") }, EnumeratedValue { value: "#0110", name: Some("0110"), description: Some("Circular buffer size is 512 bytes.") }, EnumeratedValue { value: "#0111", name: Some("0111"), description: Some("Circular buffer size is 1 KB.") }, EnumeratedValue { value: "#1000", name: Some("1000"), description: Some("Circular buffer size is 2 KB.") }, EnumeratedValue { value: "#1001", name: Some("1001"), description: Some("Circular buffer size is 4 KB.") }, EnumeratedValue { value: "#1010", name: Some("1010"), description: Some("Circular buffer size is 8 KB.") }, EnumeratedValue { value: "#1011", name: Some("1011"), description: Some("Circular buffer size is 16 KB.") }, EnumeratedValue { value: "#1100", name: Some("1100"), description: Some("Circular buffer size is 32 KB.") }, EnumeratedValue { value: "#1101", name: Some("1101"), description: Some("Circular buffer size is 64 KB.") }, EnumeratedValue { value: "#1110", name: Some("1110"), description: Some("Circular buffer size is 128 KB.") }, EnumeratedValue { value: "#1111", name: Some("1111"), description: Some("Circular buffer size is 256 KB.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "START", bit_offset: 16, bit_width: 1, access: Some(WriteOnly), description: Some("Start Transfer"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("DMA inactive") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The DMA begins the transfer in accordance to the values in the TCDn. START is cleared automatically after one module clock and always reads as logic 0.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DSIZE", bit_offset: 17, bit_width: 2, access: Some(ReadWrite), description: Some("Destination Size"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("32-bit") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("8-bit") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("16-bit") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("Reserved (generates a configuration error (DSRn[CE]) if incorrectly specified at time of channel activation)") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "DINC", bit_offset: 19, bit_width: 1, access: Some(ReadWrite), description: Some("Destination Increment"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No change to the DAR after a successful transfer.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The DAR increments by 1, 2, 4 depending upon the size of the transfer.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SSIZE", bit_offset: 20, bit_width: 2, access: Some(ReadWrite), description: Some("Source Size"), enumerated_values: [EnumeratedValue { value: "#00", name: Some("00"), description: Some("32-bit") }, EnumeratedValue { value: "#01", name: Some("01"), description: Some("8-bit") }, EnumeratedValue { value: "#10", name: Some("10"), description: Some("16-bit") }, EnumeratedValue { value: "#11", name: Some("11"), description: Some("Reserved (generates a configuration error (DSRn[CE]) if incorrectly specified at time of channel activation)") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "SINC", bit_offset: 22, bit_width: 1, access: Some(ReadWrite), description: Some("Source Increment"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No change to SAR after a successful transfer.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("The SAR increments by 1, 2, 4 as determined by the transfer size.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "EADREQ", bit_offset: 23, bit_width: 1, access: Some(ReadWrite), description: Some("Enable asynchronous DMA requests"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Enabled") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "AA", bit_offset: 28, bit_width: 1, access: Some(ReadWrite), description: Some("Auto-align"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Auto-align disabled") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("If SSIZE indicates a transfer no smaller than DSIZE, source accesses are auto-aligned; otherwise, destination accesses are auto-aligned. Source alignment takes precedence over destination alignment. If auto-alignment is enabled, the appropriate address register increments, regardless of DINC or SINC.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "CS", bit_offset: 29, bit_width: 1, access: Some(ReadWrite), description: Some("Cycle Steal"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("DMA continuously makes read/write transfers until the BCR decrements to 0.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Forces a single read/write transfer per request.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "ERQ", bit_offset: 30, bit_width: 1, access: Some(ReadWrite), description: Some("Enable Peripheral Request"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("Peripheral request is ignored.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Enables peripheral request to initiate transfer. A software-initiated request (setting START) is always enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }, Field { name: "EINT", bit_offset: 31, bit_width: 1, access: Some(ReadWrite), description: Some("Enable Interrupt on Completion of Transfer"), enumerated_values: [EnumeratedValue { value: "#0", name: Some("0"), description: Some("No interrupt is generated.") }, EnumeratedValue { value: "#1", name: Some("1"), description: Some("Interrupt signal is enabled.") }], links: [], dim: None, dim_increment: None, dim_index: None }], dim: Some(4), dim_increment: Some(16), dim_index: None }], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }), modules: [], has_pins: false, has_channels: true, description: None }

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMA Peripheral"]
pub struct DmaPeriph(pub usize); 


impl DmaPeriph {
#[doc="Get the *const pointer for the SAR register."]
   #[inline] pub fn sar_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x100 + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the SAR register."]
   #[inline] pub fn sar_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x100 + (index << 4)) as *mut u32
   }
#[doc="Read the SAR register."]
   #[inline] pub fn sar<I: Into<bits::R4>>(&self, index: I) -> Sar { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Sar(::core::ptr::read_volatile((self.0 + 0x100 + (index << 4)) as *const u32))
      }
   }
#[doc="Write the SAR register."]
   #[inline] pub fn set_sar<I: Into<bits::R4>, F: FnOnce(Sar) -> Sar>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Sar(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x100 + (index << 4)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SAR register."]
   #[inline] pub fn with_sar<I: Into<bits::R4> + Copy, F: FnOnce(Sar) -> Sar>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.sar(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x100 + (index << 4)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DAR register."]
   #[inline] pub fn dar_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x104 + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the DAR register."]
   #[inline] pub fn dar_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x104 + (index << 4)) as *mut u32
   }
#[doc="Read the DAR register."]
   #[inline] pub fn dar<I: Into<bits::R4>>(&self, index: I) -> Dar { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Dar(::core::ptr::read_volatile((self.0 + 0x104 + (index << 4)) as *const u32))
      }
   }
#[doc="Write the DAR register."]
   #[inline] pub fn set_dar<I: Into<bits::R4>, F: FnOnce(Dar) -> Dar>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Dar(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x104 + (index << 4)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DAR register."]
   #[inline] pub fn with_dar<I: Into<bits::R4> + Copy, F: FnOnce(Dar) -> Dar>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.dar(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x104 + (index << 4)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DSR_BCR register."]
   #[inline] pub fn dsr_bcr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x108 + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the DSR_BCR register."]
   #[inline] pub fn dsr_bcr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x108 + (index << 4)) as *mut u32
   }
#[doc="Read the DSR_BCR register."]
   #[inline] pub fn dsr_bcr<I: Into<bits::R4>>(&self, index: I) -> DsrBcr { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         DsrBcr(::core::ptr::read_volatile((self.0 + 0x108 + (index << 4)) as *const u32))
      }
   }
#[doc="Write the DSR_BCR register."]
   #[inline] pub fn set_dsr_bcr<I: Into<bits::R4>, F: FnOnce(DsrBcr) -> DsrBcr>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(DsrBcr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x108 + (index << 4)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DSR_BCR register."]
   #[inline] pub fn with_dsr_bcr<I: Into<bits::R4> + Copy, F: FnOnce(DsrBcr) -> DsrBcr>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.dsr_bcr(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x108 + (index << 4)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DSR register."]
   #[inline] pub fn dsr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u8 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10b + (index << 4)) as *const u8
   }
#[doc="Get the *mut pointer for the DSR register."]
   #[inline] pub fn dsr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u8 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10b + (index << 4)) as *mut u8
   }
#[doc="Read the DSR register."]
   #[inline] pub fn dsr<I: Into<bits::R4>>(&self, index: I) -> Dsr { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Dsr(::core::ptr::read_volatile((self.0 + 0x10b + (index << 4)) as *const u8))
      }
   }
#[doc="Write the DSR register."]
   #[inline] pub fn set_dsr<I: Into<bits::R4>, F: FnOnce(Dsr) -> Dsr>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Dsr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10b + (index << 4)) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the DSR register."]
   #[inline] pub fn with_dsr<I: Into<bits::R4> + Copy, F: FnOnce(Dsr) -> Dsr>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.dsr(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10b + (index << 4)) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCR register."]
   #[inline] pub fn dcr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10c + (index << 4)) as *const u32
   }
#[doc="Get the *mut pointer for the DCR register."]
   #[inline] pub fn dcr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      ((self.0 as usize) + 0x10c + (index << 4)) as *mut u32
   }
#[doc="Read the DCR register."]
   #[inline] pub fn dcr<I: Into<bits::R4>>(&self, index: I) -> Dcr { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         Dcr(::core::ptr::read_volatile((self.0 + 0x10c + (index << 4)) as *const u32))
      }
   }
#[doc="Write the DCR register."]
   #[inline] pub fn set_dcr<I: Into<bits::R4>, F: FnOnce(Dcr) -> Dcr>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(Dcr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10c + (index << 4)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCR register."]
   #[inline] pub fn with_dcr<I: Into<bits::R4> + Copy, F: FnOnce(Dcr) -> Dcr>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = self.dcr(index);
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x10c + (index << 4)) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Source Address Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sar(pub u32);
impl Sar {
#[doc="Each SAR contains the byte address used by the DMA controller to read data"]
   #[inline] pub fn sar(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Each SAR contains the byte address used by the DMA controller to read data"]
   #[inline] pub fn set_sar<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Sar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Destination Address Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dar(pub u32);
impl Dar {
#[doc="Each DAR contains the byte address used by the DMA controller to write data"]
   #[inline] pub fn dar(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Each DAR contains the byte address used by the DMA controller to write data"]
   #[inline] pub fn set_dar<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Status Register / Byte Count Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DsrBcr(pub u32);
impl DsrBcr {
#[doc="This field contains the number of bytes yet to be transferred for a given block"]
   #[inline] pub fn bcr(&self) -> bits::U24 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
   }
#[doc="This field contains the number of bytes yet to be transferred for a given block"]
   #[inline] pub fn set_bcr<V: Into<bits::U24>>(mut self, value: V) -> Self {
      let value: bits::U24 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transactions Done"]
   #[inline] pub fn done(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
   }
#[doc="Transactions Done"]
   #[inline] pub fn set_done<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Busy"]
   #[inline] pub fn bsy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Busy"]
   #[inline] pub fn set_bsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="Request"]
   #[inline] pub fn req(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
   }
#[doc="Request"]
   #[inline] pub fn set_req<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 26);
      self.0 |= value << 26;
      self
   }

#[doc="Bus Error on Destination"]
   #[inline] pub fn bed(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="Bus Error on Destination"]
   #[inline] pub fn set_bed<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="Bus Error on Source"]
   #[inline] pub fn bes(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="Bus Error on Source"]
   #[inline] pub fn set_bes<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="Configuration Error"]
   #[inline] pub fn ce(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Configuration Error"]
   #[inline] pub fn set_ce<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

}
impl ::core::fmt::Display for DsrBcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for DsrBcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bcr() != 0 { try!(write!(f, " bcr=0x{:x}", self.bcr()))}
      if self.done() != 0 { try!(write!(f, " done"))}
      if self.bsy() != 0 { try!(write!(f, " bsy"))}
      if self.req() != 0 { try!(write!(f, " req"))}
      if self.bed() != 0 { try!(write!(f, " bed"))}
      if self.bes() != 0 { try!(write!(f, " bes"))}
      if self.ce() != 0 { try!(write!(f, " ce"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA_DSR0 register."]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dsr(pub u8);
impl Dsr {
}
impl ::core::fmt::Display for Dsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcr(pub u32);
impl Dcr {
#[doc="Link Channel 2"]
   #[inline] pub fn lch2(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Link Channel 2"]
   #[inline] pub fn set_lch2<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Link Channel 1"]
   #[inline] pub fn lch1(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="Link Channel 1"]
   #[inline] pub fn set_lch1<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Link Channel Control"]
   #[inline] pub fn linkcc(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
   }
#[doc="Link Channel Control"]
   #[inline] pub fn set_linkcc<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Disable Request"]
   #[inline] pub fn d_req(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Disable Request"]
   #[inline] pub fn set_d_req<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Destination Address Modulo"]
   #[inline] pub fn dmod(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
   }
#[doc="Destination Address Modulo"]
   #[inline] pub fn set_dmod<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Source Address Modulo"]
   #[inline] pub fn smod(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
   }
#[doc="Source Address Modulo"]
   #[inline] pub fn set_smod<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Start Transfer"]
   #[inline] pub fn start(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Start Transfer"]
   #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Destination Size"]
   #[inline] pub fn dsize(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
   }
#[doc="Destination Size"]
   #[inline] pub fn set_dsize<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Destination Increment"]
   #[inline] pub fn dinc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
   }
#[doc="Destination Increment"]
   #[inline] pub fn set_dinc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="Source Size"]
   #[inline] pub fn ssize(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
   }
#[doc="Source Size"]
   #[inline] pub fn set_ssize<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 20);
      self.0 |= value << 20;
      self
   }

#[doc="Source Increment"]
   #[inline] pub fn sinc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="Source Increment"]
   #[inline] pub fn set_sinc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="Enable asynchronous DMA requests"]
   #[inline] pub fn eadreq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="Enable asynchronous DMA requests"]
   #[inline] pub fn set_eadreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

#[doc="Auto-align"]
   #[inline] pub fn aa(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="Auto-align"]
   #[inline] pub fn set_aa<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="Cycle Steal"]
   #[inline] pub fn cs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="Cycle Steal"]
   #[inline] pub fn set_cs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="Enable Peripheral Request"]
   #[inline] pub fn erq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Enable Peripheral Request"]
   #[inline] pub fn set_erq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Enable Interrupt on Completion of Transfer"]
   #[inline] pub fn eint(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Enable Interrupt on Completion of Transfer"]
   #[inline] pub fn set_eint<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
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
      if self.lch2() != 0 { try!(write!(f, " lch2=0x{:x}", self.lch2()))}
      if self.lch1() != 0 { try!(write!(f, " lch1=0x{:x}", self.lch1()))}
      if self.linkcc() != 0 { try!(write!(f, " linkcc=0x{:x}", self.linkcc()))}
      if self.d_req() != 0 { try!(write!(f, " d_req"))}
      if self.dmod() != 0 { try!(write!(f, " dmod=0x{:x}", self.dmod()))}
      if self.smod() != 0 { try!(write!(f, " smod=0x{:x}", self.smod()))}
      if self.start() != 0 { try!(write!(f, " start"))}
      if self.dsize() != 0 { try!(write!(f, " dsize=0x{:x}", self.dsize()))}
      if self.dinc() != 0 { try!(write!(f, " dinc"))}
      if self.ssize() != 0 { try!(write!(f, " ssize=0x{:x}", self.ssize()))}
      if self.sinc() != 0 { try!(write!(f, " sinc"))}
      if self.eadreq() != 0 { try!(write!(f, " eadreq"))}
      if self.aa() != 0 { try!(write!(f, " aa"))}
      if self.cs() != 0 { try!(write!(f, " cs"))}
      if self.erq() != 0 { try!(write!(f, " erq"))}
      if self.eint() != 0 { try!(write!(f, " eint"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub struct DmaCh { pub periph: DmaPeriph, pub index: usize }
