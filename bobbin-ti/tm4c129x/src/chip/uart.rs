#[allow(unused_imports)] use bobbin_common::*;

periph!( UART0, Uart0, _UART0, UartPeriph, 0x4000c000);
periph!( UART1, Uart1, _UART1, UartPeriph, 0x4000d000);
periph!( UART2, Uart2, _UART2, UartPeriph, 0x4000e000);
periph!( UART3, Uart3, _UART3, UartPeriph, 0x4000f000);
periph!( UART4, Uart4, _UART4, UartPeriph, 0x40010000);
periph!( UART5, Uart5, _UART5, UartPeriph, 0x40011000);
periph!( UART6, Uart6, _UART6, UartPeriph, 0x40012000);
periph!( UART7, Uart7, _UART7, UartPeriph, 0x40013000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="UART Peripheral"]
pub struct UartPeriph(pub usize); 

impl super::sig::Signal<super::sig::U0rx> for Uart0 {}
impl super::sig::SignalRx<super::sig::U0rx> for Uart0 {}
impl super::sig::Signal<super::sig::U0tx> for Uart0 {}
impl super::sig::SignalTx<super::sig::U0tx> for Uart0 {}

impl super::sig::Signal<super::sig::U1rx> for Uart1 {}
impl super::sig::SignalRx<super::sig::U1rx> for Uart1 {}
impl super::sig::Signal<super::sig::U1tx> for Uart1 {}
impl super::sig::SignalTx<super::sig::U1tx> for Uart1 {}

impl super::sig::Signal<super::sig::U2rx> for Uart2 {}
impl super::sig::SignalRx<super::sig::U2rx> for Uart2 {}
impl super::sig::Signal<super::sig::U2tx> for Uart2 {}
impl super::sig::SignalTx<super::sig::U2tx> for Uart2 {}

impl super::sig::Signal<super::sig::U3rx> for Uart3 {}
impl super::sig::SignalRx<super::sig::U3rx> for Uart3 {}
impl super::sig::Signal<super::sig::U3tx> for Uart3 {}
impl super::sig::SignalTx<super::sig::U3tx> for Uart3 {}

impl super::sig::Signal<super::sig::U4rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::U4rx> for Uart4 {}
impl super::sig::Signal<super::sig::U4tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::U4tx> for Uart4 {}

impl super::sig::Signal<super::sig::U5rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::U5rx> for Uart5 {}
impl super::sig::Signal<super::sig::U5tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::U5tx> for Uart5 {}

impl super::sig::Signal<super::sig::U6rx> for Uart6 {}
impl super::sig::SignalRx<super::sig::U6rx> for Uart6 {}
impl super::sig::Signal<super::sig::U6tx> for Uart6 {}
impl super::sig::SignalTx<super::sig::U6tx> for Uart6 {}

impl super::sig::Signal<super::sig::U7rx> for Uart7 {}
impl super::sig::SignalRx<super::sig::U7rx> for Uart7 {}
impl super::sig::Signal<super::sig::U7tx> for Uart7 {}
impl super::sig::SignalTx<super::sig::U7tx> for Uart7 {}


impl UartPeriph {
#[doc="Get the *const pointer for the DR register."]
   #[inline] pub fn dr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the DR register."]
   #[inline] pub fn dr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the DR register."]
   #[inline] pub fn dr(&self) -> Dr { 
      unsafe {
         Dr(::core::ptr::read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the DR register."]
   #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
      let value = f(Dr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DR register."]
   #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
      let tmp = self.dr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RSR register."]
   #[inline] pub fn rsr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }
#[doc="Get the *mut pointer for the RSR register."]
   #[inline] pub fn rsr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }
#[doc="Read the RSR register."]
   #[inline] pub fn rsr(&self) -> Rsr { 
      unsafe {
         Rsr(::core::ptr::read_volatile((self.0 + 0x4) as *const u32))
      }
   }
#[doc="Write the RSR register."]
   #[inline] pub fn set_rsr<F: FnOnce(Rsr) -> Rsr>(&self, f: F) -> &Self {
      let value = f(Rsr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RSR register."]
   #[inline] pub fn with_rsr<F: FnOnce(Rsr) -> Rsr>(&self, f: F) -> &Self {
      let tmp = self.rsr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ECR register."]
   #[inline] pub fn ecr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }
#[doc="Get the *mut pointer for the ECR register."]
   #[inline] pub fn ecr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }
#[doc="Read the ECR register."]
   #[inline] pub fn ecr(&self) -> Ecr { 
      unsafe {
         Ecr(::core::ptr::read_volatile((self.0 + 0x4) as *const u32))
      }
   }
#[doc="Write the ECR register."]
   #[inline] pub fn set_ecr<F: FnOnce(Ecr) -> Ecr>(&self, f: F) -> &Self {
      let value = f(Ecr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ECR register."]
   #[inline] pub fn with_ecr<F: FnOnce(Ecr) -> Ecr>(&self, f: F) -> &Self {
      let tmp = self.ecr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FR register."]
   #[inline] pub fn fr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x18) as *const u32
   }
#[doc="Get the *mut pointer for the FR register."]
   #[inline] pub fn fr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x18) as *mut u32
   }
#[doc="Read the FR register."]
   #[inline] pub fn fr(&self) -> Fr { 
      unsafe {
         Fr(::core::ptr::read_volatile((self.0 + 0x18) as *const u32))
      }
   }
#[doc="Write the FR register."]
   #[inline] pub fn set_fr<F: FnOnce(Fr) -> Fr>(&self, f: F) -> &Self {
      let value = f(Fr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FR register."]
   #[inline] pub fn with_fr<F: FnOnce(Fr) -> Fr>(&self, f: F) -> &Self {
      let tmp = self.fr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ILPR register."]
   #[inline] pub fn ilpr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x20) as *const u32
   }
#[doc="Get the *mut pointer for the ILPR register."]
   #[inline] pub fn ilpr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x20) as *mut u32
   }
#[doc="Read the ILPR register."]
   #[inline] pub fn ilpr(&self) -> Ilpr { 
      unsafe {
         Ilpr(::core::ptr::read_volatile((self.0 + 0x20) as *const u32))
      }
   }
#[doc="Write the ILPR register."]
   #[inline] pub fn set_ilpr<F: FnOnce(Ilpr) -> Ilpr>(&self, f: F) -> &Self {
      let value = f(Ilpr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ILPR register."]
   #[inline] pub fn with_ilpr<F: FnOnce(Ilpr) -> Ilpr>(&self, f: F) -> &Self {
      let tmp = self.ilpr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IBRD register."]
   #[inline] pub fn ibrd_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x24) as *const u32
   }
#[doc="Get the *mut pointer for the IBRD register."]
   #[inline] pub fn ibrd_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x24) as *mut u32
   }
#[doc="Read the IBRD register."]
   #[inline] pub fn ibrd(&self) -> Ibrd { 
      unsafe {
         Ibrd(::core::ptr::read_volatile((self.0 + 0x24) as *const u32))
      }
   }
#[doc="Write the IBRD register."]
   #[inline] pub fn set_ibrd<F: FnOnce(Ibrd) -> Ibrd>(&self, f: F) -> &Self {
      let value = f(Ibrd(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the IBRD register."]
   #[inline] pub fn with_ibrd<F: FnOnce(Ibrd) -> Ibrd>(&self, f: F) -> &Self {
      let tmp = self.ibrd();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FBRD register."]
   #[inline] pub fn fbrd_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x28) as *const u32
   }
#[doc="Get the *mut pointer for the FBRD register."]
   #[inline] pub fn fbrd_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x28) as *mut u32
   }
#[doc="Read the FBRD register."]
   #[inline] pub fn fbrd(&self) -> Fbrd { 
      unsafe {
         Fbrd(::core::ptr::read_volatile((self.0 + 0x28) as *const u32))
      }
   }
#[doc="Write the FBRD register."]
   #[inline] pub fn set_fbrd<F: FnOnce(Fbrd) -> Fbrd>(&self, f: F) -> &Self {
      let value = f(Fbrd(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FBRD register."]
   #[inline] pub fn with_fbrd<F: FnOnce(Fbrd) -> Fbrd>(&self, f: F) -> &Self {
      let tmp = self.fbrd();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the LCRH register."]
   #[inline] pub fn lcrh_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x2c) as *const u32
   }
#[doc="Get the *mut pointer for the LCRH register."]
   #[inline] pub fn lcrh_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x2c) as *mut u32
   }
#[doc="Read the LCRH register."]
   #[inline] pub fn lcrh(&self) -> Lcrh { 
      unsafe {
         Lcrh(::core::ptr::read_volatile((self.0 + 0x2c) as *const u32))
      }
   }
#[doc="Write the LCRH register."]
   #[inline] pub fn set_lcrh<F: FnOnce(Lcrh) -> Lcrh>(&self, f: F) -> &Self {
      let value = f(Lcrh(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LCRH register."]
   #[inline] pub fn with_lcrh<F: FnOnce(Lcrh) -> Lcrh>(&self, f: F) -> &Self {
      let tmp = self.lcrh();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CTL register."]
   #[inline] pub fn ctl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x30) as *const u32
   }
#[doc="Get the *mut pointer for the CTL register."]
   #[inline] pub fn ctl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x30) as *mut u32
   }
#[doc="Read the CTL register."]
   #[inline] pub fn ctl(&self) -> Ctl { 
      unsafe {
         Ctl(::core::ptr::read_volatile((self.0 + 0x30) as *const u32))
      }
   }
#[doc="Write the CTL register."]
   #[inline] pub fn set_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
      let value = f(Ctl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CTL register."]
   #[inline] pub fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
      let tmp = self.ctl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IFLS register."]
   #[inline] pub fn ifls_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x34) as *const u32
   }
#[doc="Get the *mut pointer for the IFLS register."]
   #[inline] pub fn ifls_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x34) as *mut u32
   }
#[doc="Read the IFLS register."]
   #[inline] pub fn ifls(&self) -> Ifls { 
      unsafe {
         Ifls(::core::ptr::read_volatile((self.0 + 0x34) as *const u32))
      }
   }
#[doc="Write the IFLS register."]
   #[inline] pub fn set_ifls<F: FnOnce(Ifls) -> Ifls>(&self, f: F) -> &Self {
      let value = f(Ifls(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the IFLS register."]
   #[inline] pub fn with_ifls<F: FnOnce(Ifls) -> Ifls>(&self, f: F) -> &Self {
      let tmp = self.ifls();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IM register."]
   #[inline] pub fn im_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x38) as *const u32
   }
#[doc="Get the *mut pointer for the IM register."]
   #[inline] pub fn im_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x38) as *mut u32
   }
#[doc="Read the IM register."]
   #[inline] pub fn im(&self) -> Im { 
      unsafe {
         Im(::core::ptr::read_volatile((self.0 + 0x38) as *const u32))
      }
   }
#[doc="Write the IM register."]
   #[inline] pub fn set_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
      let value = f(Im(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x38) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the IM register."]
   #[inline] pub fn with_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
      let tmp = self.im();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x38) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RIS register."]
   #[inline] pub fn ris_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x3c) as *const u32
   }
#[doc="Get the *mut pointer for the RIS register."]
   #[inline] pub fn ris_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x3c) as *mut u32
   }
#[doc="Read the RIS register."]
   #[inline] pub fn ris(&self) -> Ris { 
      unsafe {
         Ris(::core::ptr::read_volatile((self.0 + 0x3c) as *const u32))
      }
   }
#[doc="Write the RIS register."]
   #[inline] pub fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
      let value = f(Ris(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x3c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RIS register."]
   #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
      let tmp = self.ris();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x3c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MIS register."]
   #[inline] pub fn mis_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x40) as *const u32
   }
#[doc="Get the *mut pointer for the MIS register."]
   #[inline] pub fn mis_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x40) as *mut u32
   }
#[doc="Read the MIS register."]
   #[inline] pub fn mis(&self) -> Mis { 
      unsafe {
         Mis(::core::ptr::read_volatile((self.0 + 0x40) as *const u32))
      }
   }
#[doc="Write the MIS register."]
   #[inline] pub fn set_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
      let value = f(Mis(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x40) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MIS register."]
   #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
      let tmp = self.mis();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x40) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ICR register."]
   #[inline] pub fn icr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x44) as *const u32
   }
#[doc="Get the *mut pointer for the ICR register."]
   #[inline] pub fn icr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x44) as *mut u32
   }
#[doc="Write the ICR register."]
   #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
      let value = f(Icr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x44) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DMACTL register."]
   #[inline] pub fn dmactl_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x48) as *const u32
   }
#[doc="Get the *mut pointer for the DMACTL register."]
   #[inline] pub fn dmactl_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x48) as *mut u32
   }
#[doc="Read the DMACTL register."]
   #[inline] pub fn dmactl(&self) -> Dmactl { 
      unsafe {
         Dmactl(::core::ptr::read_volatile((self.0 + 0x48) as *const u32))
      }
   }
#[doc="Write the DMACTL register."]
   #[inline] pub fn set_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&self, f: F) -> &Self {
      let value = f(Dmactl(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x48) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DMACTL register."]
   #[inline] pub fn with_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&self, f: F) -> &Self {
      let tmp = self.dmactl();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0x48) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the _9BITADDR register."]
   #[inline] pub fn _9bitaddr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xa4) as *const u32
   }
#[doc="Get the *mut pointer for the _9BITADDR register."]
   #[inline] pub fn _9bitaddr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xa4) as *mut u32
   }
#[doc="Read the _9BITADDR register."]
   #[inline] pub fn _9bitaddr(&self) -> _9bitaddr { 
      unsafe {
         _9bitaddr(::core::ptr::read_volatile((self.0 + 0xa4) as *const u32))
      }
   }
#[doc="Write the _9BITADDR register."]
   #[inline] pub fn set_9bitaddr<F: FnOnce(_9bitaddr) -> _9bitaddr>(&self, f: F) -> &Self {
      let value = f(_9bitaddr(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the _9BITADDR register."]
   #[inline] pub fn with_9bitaddr<F: FnOnce(_9bitaddr) -> _9bitaddr>(&self, f: F) -> &Self {
      let tmp = self._9bitaddr();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the _9BITAMASK register."]
   #[inline] pub fn _9bitamask_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xa8) as *const u32
   }
#[doc="Get the *mut pointer for the _9BITAMASK register."]
   #[inline] pub fn _9bitamask_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xa8) as *mut u32
   }
#[doc="Read the _9BITAMASK register."]
   #[inline] pub fn _9bitamask(&self) -> _9bitamask { 
      unsafe {
         _9bitamask(::core::ptr::read_volatile((self.0 + 0xa8) as *const u32))
      }
   }
#[doc="Write the _9BITAMASK register."]
   #[inline] pub fn set_9bitamask<F: FnOnce(_9bitamask) -> _9bitamask>(&self, f: F) -> &Self {
      let value = f(_9bitamask(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the _9BITAMASK register."]
   #[inline] pub fn with_9bitamask<F: FnOnce(_9bitamask) -> _9bitamask>(&self, f: F) -> &Self {
      let tmp = self._9bitamask();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xa8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PP register."]
   #[inline] pub fn pp_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xfc0) as *const u32
   }
#[doc="Get the *mut pointer for the PP register."]
   #[inline] pub fn pp_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xfc0) as *mut u32
   }
#[doc="Read the PP register."]
   #[inline] pub fn pp(&self) -> Pp { 
      unsafe {
         Pp(::core::ptr::read_volatile((self.0 + 0xfc0) as *const u32))
      }
   }
#[doc="Write the PP register."]
   #[inline] pub fn set_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
      let value = f(Pp(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xfc0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PP register."]
   #[inline] pub fn with_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
      let tmp = self.pp();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xfc0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CC register."]
   #[inline] pub fn cc_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xfc8) as *const u32
   }
#[doc="Get the *mut pointer for the CC register."]
   #[inline] pub fn cc_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xfc8) as *mut u32
   }
#[doc="Read the CC register."]
   #[inline] pub fn cc(&self) -> Cc { 
      unsafe {
         Cc(::core::ptr::read_volatile((self.0 + 0xfc8) as *const u32))
      }
   }
#[doc="Write the CC register."]
   #[inline] pub fn set_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
      let value = f(Cc(0));
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xfc8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CC register."]
   #[inline] pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
      let tmp = self.cc();
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.0 + 0xfc8) as *mut u32, value.0);
      }
      self
   }

}

#[doc="UART Data"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
#[doc="Data Transmitted or Received"]
   #[inline] pub fn data(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Data Transmitted or Received"]
   #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Framing Error"]
   #[inline] pub fn fe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="UART Framing Error"]
   #[inline] pub fn set_fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="UART Parity Error"]
   #[inline] pub fn pe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="UART Parity Error"]
   #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="UART Break Error"]
   #[inline] pub fn be(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="UART Break Error"]
   #[inline] pub fn set_be<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="UART Overrun Error"]
   #[inline] pub fn oe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="UART Overrun Error"]
   #[inline] pub fn set_oe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

}
impl ::core::fmt::Display for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      if self.fe() != 0 { try!(write!(f, " fe"))}
      if self.pe() != 0 { try!(write!(f, " pe"))}
      if self.be() != 0 { try!(write!(f, " be"))}
      if self.oe() != 0 { try!(write!(f, " oe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Receive Status/Error Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rsr(pub u32);
impl Rsr {
#[doc="UART Framing Error"]
   #[inline] pub fn fe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Framing Error"]
   #[inline] pub fn set_fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Parity Error"]
   #[inline] pub fn pe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Parity Error"]
   #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Break Error"]
   #[inline] pub fn be(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Break Error"]
   #[inline] pub fn set_be<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Overrun Error"]
   #[inline] pub fn oe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Overrun Error"]
   #[inline] pub fn set_oe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Rsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fe() != 0 { try!(write!(f, " fe"))}
      if self.pe() != 0 { try!(write!(f, " pe"))}
      if self.be() != 0 { try!(write!(f, " be"))}
      if self.oe() != 0 { try!(write!(f, " oe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Receive Status/Error Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ecr(pub u32);
impl Ecr {
#[doc="Error Clear"]
   #[inline] pub fn data(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Error Clear"]
   #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ecr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Flag"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fr(pub u32);
impl Fr {
#[doc="Clear To Send"]
   #[inline] pub fn cts(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Clear To Send"]
   #[inline] pub fn set_cts<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Data Set Ready"]
   #[inline] pub fn dsr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Data Set Ready"]
   #[inline] pub fn set_dsr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Data Carrier Detect"]
   #[inline] pub fn dcd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Data Carrier Detect"]
   #[inline] pub fn set_dcd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Busy"]
   #[inline] pub fn busy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Busy"]
   #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Receive FIFO Empty"]
   #[inline] pub fn rxfe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Receive FIFO Empty"]
   #[inline] pub fn set_rxfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Transmit FIFO Full"]
   #[inline] pub fn txff(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART Transmit FIFO Full"]
   #[inline] pub fn set_txff<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Receive FIFO Full"]
   #[inline] pub fn rxff(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Receive FIFO Full"]
   #[inline] pub fn set_rxff<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="UART Transmit FIFO Empty"]
   #[inline] pub fn txfe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Transmit FIFO Empty"]
   #[inline] pub fn set_txfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Ring Indicator"]
   #[inline] pub fn ri(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Ring Indicator"]
   #[inline] pub fn set_ri<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Fr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cts() != 0 { try!(write!(f, " cts"))}
      if self.dsr() != 0 { try!(write!(f, " dsr"))}
      if self.dcd() != 0 { try!(write!(f, " dcd"))}
      if self.busy() != 0 { try!(write!(f, " busy"))}
      if self.rxfe() != 0 { try!(write!(f, " rxfe"))}
      if self.txff() != 0 { try!(write!(f, " txff"))}
      if self.rxff() != 0 { try!(write!(f, " rxff"))}
      if self.txfe() != 0 { try!(write!(f, " txfe"))}
      if self.ri() != 0 { try!(write!(f, " ri"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART IrDA Low-Power Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ilpr(pub u32);
impl Ilpr {
#[doc="IrDA Low-Power Divisor"]
   #[inline] pub fn ilpdvsr(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="IrDA Low-Power Divisor"]
   #[inline] pub fn set_ilpdvsr<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ilpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ilpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ilpdvsr() != 0 { try!(write!(f, " ilpdvsr=0x{:x}", self.ilpdvsr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Integer Baud-Rate Divisor"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ibrd(pub u32);
impl Ibrd {
#[doc="Integer Baud-Rate Divisor"]
   #[inline] pub fn divint(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Integer Baud-Rate Divisor"]
   #[inline] pub fn set_divint<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ibrd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ibrd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.divint() != 0 { try!(write!(f, " divint=0x{:x}", self.divint()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Fractional Baud-Rate Divisor"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fbrd(pub u32);
impl Fbrd {
#[doc="Fractional Baud-Rate Divisor"]
   #[inline] pub fn divfrac(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
   }
#[doc="Fractional Baud-Rate Divisor"]
   #[inline] pub fn set_divfrac<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Fbrd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fbrd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.divfrac() != 0 { try!(write!(f, " divfrac=0x{:x}", self.divfrac()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Line Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lcrh(pub u32);
impl Lcrh {
#[doc="UART Send Break"]
   #[inline] pub fn brk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Send Break"]
   #[inline] pub fn set_brk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Parity Enable"]
   #[inline] pub fn pen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Parity Enable"]
   #[inline] pub fn set_pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Even Parity Select"]
   #[inline] pub fn eps(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Even Parity Select"]
   #[inline] pub fn set_eps<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Two Stop Bits Select"]
   #[inline] pub fn stp2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Two Stop Bits Select"]
   #[inline] pub fn set_stp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Enable FIFOs"]
   #[inline] pub fn fen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Enable FIFOs"]
   #[inline] pub fn set_fen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Word Length"]
   #[inline] pub fn wlen(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
   }
#[doc="UART Word Length"]
   #[inline] pub fn set_wlen<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Stick Parity Select"]
   #[inline] pub fn sps(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Stick Parity Select"]
   #[inline] pub fn set_sps<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Lcrh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lcrh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.brk() != 0 { try!(write!(f, " brk"))}
      if self.pen() != 0 { try!(write!(f, " pen"))}
      if self.eps() != 0 { try!(write!(f, " eps"))}
      if self.stp2() != 0 { try!(write!(f, " stp2"))}
      if self.fen() != 0 { try!(write!(f, " fen"))}
      if self.wlen() != 0 { try!(write!(f, " wlen=0x{:x}", self.wlen()))}
      if self.sps() != 0 { try!(write!(f, " sps"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctl(pub u32);
impl Ctl {
#[doc="UART Enable"]
   #[inline] pub fn uarten(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Enable"]
   #[inline] pub fn set_uarten<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART SIR Enable"]
   #[inline] pub fn siren(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART SIR Enable"]
   #[inline] pub fn set_siren<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART SIR Low-Power Mode"]
   #[inline] pub fn sirlp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART SIR Low-Power Mode"]
   #[inline] pub fn set_sirlp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="ISO 7816 Smart Card Support"]
   #[inline] pub fn smart(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="ISO 7816 Smart Card Support"]
   #[inline] pub fn set_smart<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="End of Transmission"]
   #[inline] pub fn eot(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="End of Transmission"]
   #[inline] pub fn set_eot<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="High-Speed Enable"]
   #[inline] pub fn hse(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="High-Speed Enable"]
   #[inline] pub fn set_hse<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Loop Back Enable"]
   #[inline] pub fn lbe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Loop Back Enable"]
   #[inline] pub fn set_lbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="UART Transmit Enable"]
   #[inline] pub fn txe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="UART Transmit Enable"]
   #[inline] pub fn set_txe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="UART Receive Enable"]
   #[inline] pub fn rxe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="UART Receive Enable"]
   #[inline] pub fn set_rxe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Data Terminal Ready"]
   #[inline] pub fn dtr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Data Terminal Ready"]
   #[inline] pub fn set_dtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Request to Send"]
   #[inline] pub fn rts(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Request to Send"]
   #[inline] pub fn set_rts<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Enable Request to Send"]
   #[inline] pub fn rtsen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="Enable Request to Send"]
   #[inline] pub fn set_rtsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Enable Clear To Send"]
   #[inline] pub fn ctsen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Enable Clear To Send"]
   #[inline] pub fn set_ctsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

}
impl ::core::fmt::Display for Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.uarten() != 0 { try!(write!(f, " uarten"))}
      if self.siren() != 0 { try!(write!(f, " siren"))}
      if self.sirlp() != 0 { try!(write!(f, " sirlp"))}
      if self.smart() != 0 { try!(write!(f, " smart"))}
      if self.eot() != 0 { try!(write!(f, " eot"))}
      if self.hse() != 0 { try!(write!(f, " hse"))}
      if self.lbe() != 0 { try!(write!(f, " lbe"))}
      if self.txe() != 0 { try!(write!(f, " txe"))}
      if self.rxe() != 0 { try!(write!(f, " rxe"))}
      if self.dtr() != 0 { try!(write!(f, " dtr"))}
      if self.rts() != 0 { try!(write!(f, " rts"))}
      if self.rtsen() != 0 { try!(write!(f, " rtsen"))}
      if self.ctsen() != 0 { try!(write!(f, " ctsen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Interrupt FIFO Level Select"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ifls(pub u32);
impl Ifls {
#[doc="UART Transmit Interrupt FIFO Level Select"]
   #[inline] pub fn tx(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="UART Transmit Interrupt FIFO Level Select"]
   #[inline] pub fn set_tx<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Receive Interrupt FIFO Level Select"]
   #[inline] pub fn rx(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
   }
#[doc="UART Receive Interrupt FIFO Level Select"]
   #[inline] pub fn set_rx<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Ifls {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ifls {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tx() != 0 { try!(write!(f, " tx=0x{:x}", self.tx()))}
      if self.rx() != 0 { try!(write!(f, " rx=0x{:x}", self.rx()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Interrupt Mask"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Im(pub u32);
impl Im {
#[doc="UART Ring Indicator Modem Interrupt Mask"]
   #[inline] pub fn rimim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Ring Indicator Modem Interrupt Mask"]
   #[inline] pub fn set_rimim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Clear to Send Modem Interrupt Mask"]
   #[inline] pub fn ctsmim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Clear to Send Modem Interrupt Mask"]
   #[inline] pub fn set_ctsmim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Data Carrier Detect Modem Interrupt Mask"]
   #[inline] pub fn dcdmim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Data Carrier Detect Modem Interrupt Mask"]
   #[inline] pub fn set_dcdmim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Data Set Ready Modem Interrupt Mask"]
   #[inline] pub fn dsrmim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Data Set Ready Modem Interrupt Mask"]
   #[inline] pub fn set_dsrmim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Receive Interrupt Mask"]
   #[inline] pub fn rxim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Receive Interrupt Mask"]
   #[inline] pub fn set_rxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Transmit Interrupt Mask"]
   #[inline] pub fn txim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART Transmit Interrupt Mask"]
   #[inline] pub fn set_txim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Receive Time-Out Interrupt Mask"]
   #[inline] pub fn rtim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Receive Time-Out Interrupt Mask"]
   #[inline] pub fn set_rtim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="UART Framing Error Interrupt Mask"]
   #[inline] pub fn feim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Framing Error Interrupt Mask"]
   #[inline] pub fn set_feim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="UART Parity Error Interrupt Mask"]
   #[inline] pub fn peim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="UART Parity Error Interrupt Mask"]
   #[inline] pub fn set_peim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="UART Break Error Interrupt Mask"]
   #[inline] pub fn beim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="UART Break Error Interrupt Mask"]
   #[inline] pub fn set_beim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="UART Overrun Error Interrupt Mask"]
   #[inline] pub fn oeim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="UART Overrun Error Interrupt Mask"]
   #[inline] pub fn set_oeim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="End of Transmission Interrupt Mask"]
   #[inline] pub fn eotim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="End of Transmission Interrupt Mask"]
   #[inline] pub fn set_eotim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="9-Bit Mode Interrupt Mask"]
   #[inline] pub fn _9bitim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="9-Bit Mode Interrupt Mask"]
   #[inline] pub fn set_9bitim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Receive DMA Interrupt Mask"]
   #[inline] pub fn dmarxim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Receive DMA Interrupt Mask"]
   #[inline] pub fn set_dmarxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Transmit DMA Interrupt Mask"]
   #[inline] pub fn dmatxim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Transmit DMA Interrupt Mask"]
   #[inline] pub fn set_dmatxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

}
impl ::core::fmt::Display for Im {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Im {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rimim() != 0 { try!(write!(f, " rimim"))}
      if self.ctsmim() != 0 { try!(write!(f, " ctsmim"))}
      if self.dcdmim() != 0 { try!(write!(f, " dcdmim"))}
      if self.dsrmim() != 0 { try!(write!(f, " dsrmim"))}
      if self.rxim() != 0 { try!(write!(f, " rxim"))}
      if self.txim() != 0 { try!(write!(f, " txim"))}
      if self.rtim() != 0 { try!(write!(f, " rtim"))}
      if self.feim() != 0 { try!(write!(f, " feim"))}
      if self.peim() != 0 { try!(write!(f, " peim"))}
      if self.beim() != 0 { try!(write!(f, " beim"))}
      if self.oeim() != 0 { try!(write!(f, " oeim"))}
      if self.eotim() != 0 { try!(write!(f, " eotim"))}
      if self._9bitim() != 0 { try!(write!(f, " _9bitim"))}
      if self.dmarxim() != 0 { try!(write!(f, " dmarxim"))}
      if self.dmatxim() != 0 { try!(write!(f, " dmatxim"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Raw Interrupt Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
#[doc="UART Ring Indicator Modem Raw Interrupt Status"]
   #[inline] pub fn riris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Ring Indicator Modem Raw Interrupt Status"]
   #[inline] pub fn set_riris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Clear to Send Modem Raw Interrupt Status"]
   #[inline] pub fn ctsris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Clear to Send Modem Raw Interrupt Status"]
   #[inline] pub fn set_ctsris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Data Carrier Detect Modem Raw Interrupt Status"]
   #[inline] pub fn dcdris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Data Carrier Detect Modem Raw Interrupt Status"]
   #[inline] pub fn set_dcdris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Data Set Ready Modem Raw Interrupt Status"]
   #[inline] pub fn dsrris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Data Set Ready Modem Raw Interrupt Status"]
   #[inline] pub fn set_dsrris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Receive Raw Interrupt Status"]
   #[inline] pub fn rxris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Receive Raw Interrupt Status"]
   #[inline] pub fn set_rxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Transmit Raw Interrupt Status"]
   #[inline] pub fn txris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART Transmit Raw Interrupt Status"]
   #[inline] pub fn set_txris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Receive Time-Out Raw Interrupt Status"]
   #[inline] pub fn rtris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Receive Time-Out Raw Interrupt Status"]
   #[inline] pub fn set_rtris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="UART Framing Error Raw Interrupt Status"]
   #[inline] pub fn feris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Framing Error Raw Interrupt Status"]
   #[inline] pub fn set_feris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="UART Parity Error Raw Interrupt Status"]
   #[inline] pub fn peris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="UART Parity Error Raw Interrupt Status"]
   #[inline] pub fn set_peris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="UART Break Error Raw Interrupt Status"]
   #[inline] pub fn beris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="UART Break Error Raw Interrupt Status"]
   #[inline] pub fn set_beris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="UART Overrun Error Raw Interrupt Status"]
   #[inline] pub fn oeris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="UART Overrun Error Raw Interrupt Status"]
   #[inline] pub fn set_oeris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="End of Transmission Raw Interrupt Status"]
   #[inline] pub fn eotris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="End of Transmission Raw Interrupt Status"]
   #[inline] pub fn set_eotris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="9-Bit Mode Raw Interrupt Status"]
   #[inline] pub fn _9bitris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="9-Bit Mode Raw Interrupt Status"]
   #[inline] pub fn set_9bitris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Receive DMA Raw Interrupt Status"]
   #[inline] pub fn dmarxris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Receive DMA Raw Interrupt Status"]
   #[inline] pub fn set_dmarxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Transmit DMA Raw Interrupt Status"]
   #[inline] pub fn dmatxris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Transmit DMA Raw Interrupt Status"]
   #[inline] pub fn set_dmatxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

}
impl ::core::fmt::Display for Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.riris() != 0 { try!(write!(f, " riris"))}
      if self.ctsris() != 0 { try!(write!(f, " ctsris"))}
      if self.dcdris() != 0 { try!(write!(f, " dcdris"))}
      if self.dsrris() != 0 { try!(write!(f, " dsrris"))}
      if self.rxris() != 0 { try!(write!(f, " rxris"))}
      if self.txris() != 0 { try!(write!(f, " txris"))}
      if self.rtris() != 0 { try!(write!(f, " rtris"))}
      if self.feris() != 0 { try!(write!(f, " feris"))}
      if self.peris() != 0 { try!(write!(f, " peris"))}
      if self.beris() != 0 { try!(write!(f, " beris"))}
      if self.oeris() != 0 { try!(write!(f, " oeris"))}
      if self.eotris() != 0 { try!(write!(f, " eotris"))}
      if self._9bitris() != 0 { try!(write!(f, " _9bitris"))}
      if self.dmarxris() != 0 { try!(write!(f, " dmarxris"))}
      if self.dmatxris() != 0 { try!(write!(f, " dmatxris"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Masked Interrupt Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
#[doc="UART Ring Indicator Modem Masked Interrupt Status"]
   #[inline] pub fn rimis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Ring Indicator Modem Masked Interrupt Status"]
   #[inline] pub fn set_rimis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Clear to Send Modem Masked Interrupt Status"]
   #[inline] pub fn ctsmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Clear to Send Modem Masked Interrupt Status"]
   #[inline] pub fn set_ctsmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Data Carrier Detect Modem Masked Interrupt Status"]
   #[inline] pub fn dcdmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Data Carrier Detect Modem Masked Interrupt Status"]
   #[inline] pub fn set_dcdmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Data Set Ready Modem Masked Interrupt Status"]
   #[inline] pub fn dsrmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Data Set Ready Modem Masked Interrupt Status"]
   #[inline] pub fn set_dsrmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Receive Masked Interrupt Status"]
   #[inline] pub fn rxmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Receive Masked Interrupt Status"]
   #[inline] pub fn set_rxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Transmit Masked Interrupt Status"]
   #[inline] pub fn txmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART Transmit Masked Interrupt Status"]
   #[inline] pub fn set_txmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Receive Time-Out Masked Interrupt Status"]
   #[inline] pub fn rtmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Receive Time-Out Masked Interrupt Status"]
   #[inline] pub fn set_rtmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="UART Framing Error Masked Interrupt Status"]
   #[inline] pub fn femis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Framing Error Masked Interrupt Status"]
   #[inline] pub fn set_femis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="UART Parity Error Masked Interrupt Status"]
   #[inline] pub fn pemis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="UART Parity Error Masked Interrupt Status"]
   #[inline] pub fn set_pemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="UART Break Error Masked Interrupt Status"]
   #[inline] pub fn bemis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="UART Break Error Masked Interrupt Status"]
   #[inline] pub fn set_bemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="UART Overrun Error Masked Interrupt Status"]
   #[inline] pub fn oemis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="UART Overrun Error Masked Interrupt Status"]
   #[inline] pub fn set_oemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="End of Transmission Masked Interrupt Status"]
   #[inline] pub fn eotmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="End of Transmission Masked Interrupt Status"]
   #[inline] pub fn set_eotmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="9-Bit Mode Masked Interrupt Status"]
   #[inline] pub fn _9bitmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="9-Bit Mode Masked Interrupt Status"]
   #[inline] pub fn set_9bitmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Receive DMA Masked Interrupt Status"]
   #[inline] pub fn dmarxmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Receive DMA Masked Interrupt Status"]
   #[inline] pub fn set_dmarxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Transmit DMA Masked Interrupt Status"]
   #[inline] pub fn dmatxmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Transmit DMA Masked Interrupt Status"]
   #[inline] pub fn set_dmatxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

}
impl ::core::fmt::Display for Mis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rimis() != 0 { try!(write!(f, " rimis"))}
      if self.ctsmis() != 0 { try!(write!(f, " ctsmis"))}
      if self.dcdmis() != 0 { try!(write!(f, " dcdmis"))}
      if self.dsrmis() != 0 { try!(write!(f, " dsrmis"))}
      if self.rxmis() != 0 { try!(write!(f, " rxmis"))}
      if self.txmis() != 0 { try!(write!(f, " txmis"))}
      if self.rtmis() != 0 { try!(write!(f, " rtmis"))}
      if self.femis() != 0 { try!(write!(f, " femis"))}
      if self.pemis() != 0 { try!(write!(f, " pemis"))}
      if self.bemis() != 0 { try!(write!(f, " bemis"))}
      if self.oemis() != 0 { try!(write!(f, " oemis"))}
      if self.eotmis() != 0 { try!(write!(f, " eotmis"))}
      if self._9bitmis() != 0 { try!(write!(f, " _9bitmis"))}
      if self.dmarxmis() != 0 { try!(write!(f, " dmarxmis"))}
      if self.dmatxmis() != 0 { try!(write!(f, " dmatxmis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Interrupt Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
#[doc="UART Ring Indicator Modem Interrupt Clear"]
   #[inline] pub fn rimic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Ring Indicator Modem Interrupt Clear"]
   #[inline] pub fn set_rimic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Clear to Send Modem Interrupt Clear"]
   #[inline] pub fn ctsmic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Clear to Send Modem Interrupt Clear"]
   #[inline] pub fn set_ctsmic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Data Carrier Detect Modem Interrupt Clear"]
   #[inline] pub fn dcdmic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Data Carrier Detect Modem Interrupt Clear"]
   #[inline] pub fn set_dcdmic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Data Set Ready Modem Interrupt Clear"]
   #[inline] pub fn dsrmic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Data Set Ready Modem Interrupt Clear"]
   #[inline] pub fn set_dsrmic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Receive Interrupt Clear"]
   #[inline] pub fn rxic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Receive Interrupt Clear"]
   #[inline] pub fn set_rxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Transmit Interrupt Clear"]
   #[inline] pub fn txic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Transmit Interrupt Clear"]
   #[inline] pub fn set_txic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Receive Time-Out Interrupt Clear"]
   #[inline] pub fn rtic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Receive Time-Out Interrupt Clear"]
   #[inline] pub fn set_rtic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Framing Error Interrupt Clear"]
   #[inline] pub fn feic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Framing Error Interrupt Clear"]
   #[inline] pub fn set_feic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Parity Error Interrupt Clear"]
   #[inline] pub fn peic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Parity Error Interrupt Clear"]
   #[inline] pub fn set_peic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Break Error Interrupt Clear"]
   #[inline] pub fn beic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Break Error Interrupt Clear"]
   #[inline] pub fn set_beic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Overrun Error Interrupt Clear"]
   #[inline] pub fn oeic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Overrun Error Interrupt Clear"]
   #[inline] pub fn set_oeic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="End of Transmission Interrupt Clear"]
   #[inline] pub fn eotic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="End of Transmission Interrupt Clear"]
   #[inline] pub fn set_eotic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="9-Bit Mode Interrupt Clear"]
   #[inline] pub fn _9bitic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="9-Bit Mode Interrupt Clear"]
   #[inline] pub fn set_9bitic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Receive DMA Interrupt Clear"]
   #[inline] pub fn dmarxic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Receive DMA Interrupt Clear"]
   #[inline] pub fn set_dmarxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Transmit DMA Interrupt Clear"]
   #[inline] pub fn dmatxic(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Transmit DMA Interrupt Clear"]
   #[inline] pub fn set_dmatxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

}
impl ::core::fmt::Display for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rimic() != 0 { try!(write!(f, " rimic"))}
      if self.ctsmic() != 0 { try!(write!(f, " ctsmic"))}
      if self.dcdmic() != 0 { try!(write!(f, " dcdmic"))}
      if self.dsrmic() != 0 { try!(write!(f, " dsrmic"))}
      if self.rxic() != 0 { try!(write!(f, " rxic"))}
      if self.txic() != 0 { try!(write!(f, " txic"))}
      if self.rtic() != 0 { try!(write!(f, " rtic"))}
      if self.feic() != 0 { try!(write!(f, " feic"))}
      if self.peic() != 0 { try!(write!(f, " peic"))}
      if self.beic() != 0 { try!(write!(f, " beic"))}
      if self.oeic() != 0 { try!(write!(f, " oeic"))}
      if self.eotic() != 0 { try!(write!(f, " eotic"))}
      if self._9bitic() != 0 { try!(write!(f, " _9bitic"))}
      if self.dmarxic() != 0 { try!(write!(f, " dmarxic"))}
      if self.dmatxic() != 0 { try!(write!(f, " dmatxic"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART DMA Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dmactl(pub u32);
impl Dmactl {
#[doc="Receive DMA Enable"]
   #[inline] pub fn rxdmae(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Receive DMA Enable"]
   #[inline] pub fn set_rxdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Transmit DMA Enable"]
   #[inline] pub fn txdmae(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Transmit DMA Enable"]
   #[inline] pub fn set_txdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="DMA on Error"]
   #[inline] pub fn dmaerr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="DMA on Error"]
   #[inline] pub fn set_dmaerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

}
impl ::core::fmt::Display for Dmactl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmactl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxdmae() != 0 { try!(write!(f, " rxdmae"))}
      if self.txdmae() != 0 { try!(write!(f, " txdmae"))}
      if self.dmaerr() != 0 { try!(write!(f, " dmaerr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART 9-Bit Self Address"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct _9bitaddr(pub u32);
impl _9bitaddr {
#[doc="Self Address for 9-Bit Mode"]
   #[inline] pub fn addr(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Self Address for 9-Bit Mode"]
   #[inline] pub fn set_addr<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Enable 9-Bit Mode"]
   #[inline] pub fn _9biten(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Enable 9-Bit Mode"]
   #[inline] pub fn set_9biten<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

}
impl ::core::fmt::Display for _9bitaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _9bitaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      if self._9biten() != 0 { try!(write!(f, " _9biten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART 9-Bit Self Address Mask"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct _9bitamask(pub u32);
impl _9bitamask {
#[doc="Self Address Mask for 9-Bit Mode"]
   #[inline] pub fn mask(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Self Address Mask for 9-Bit Mode"]
   #[inline] pub fn set_mask<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for _9bitamask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for _9bitamask {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mask() != 0 { try!(write!(f, " mask=0x{:x}", self.mask()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Peripheral Properties"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
#[doc="Smart Card Support"]
   #[inline] pub fn sc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Smart Card Support"]
   #[inline] pub fn set_sc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="9-Bit Support"]
   #[inline] pub fn nb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="9-Bit Support"]
   #[inline] pub fn set_nb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Modem Support"]
   #[inline] pub fn ms(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Modem Support"]
   #[inline] pub fn set_ms<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Modem Support Extended"]
   #[inline] pub fn mse(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Modem Support Extended"]
   #[inline] pub fn set_mse<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Pp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sc() != 0 { try!(write!(f, " sc"))}
      if self.nb() != 0 { try!(write!(f, " nb"))}
      if self.ms() != 0 { try!(write!(f, " ms"))}
      if self.mse() != 0 { try!(write!(f, " mse"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UART Clock Configuration"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
#[doc="UART Baud Clock Source"]
   #[inline] pub fn cs(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="UART Baud Clock Source"]
   #[inline] pub fn set_cs<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cs() != 0 { try!(write!(f, " cs=0x{:x}", self.cs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
