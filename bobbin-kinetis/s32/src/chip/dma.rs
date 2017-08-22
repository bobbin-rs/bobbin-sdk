#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::edma::*;

pub const DMA: Dma = Periph(0x40008000, DmaId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct DmaId {}
pub type Dma = Periph<DmaId>;



pub const DMA0: Channel<Dma0Id, DmaId> = Channel { periph: DMA, index: 0, id: Dma0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma0Id {}
pub type Dma0 = Channel<Dma0Id, DmaId>;

pub const DMA1: Channel<Dma1Id, DmaId> = Channel { periph: DMA, index: 1, id: Dma1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Id {}
pub type Dma1 = Channel<Dma1Id, DmaId>;

pub const DMA2: Channel<Dma2Id, DmaId> = Channel { periph: DMA, index: 2, id: Dma2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Id {}
pub type Dma2 = Channel<Dma2Id, DmaId>;

pub const DMA3: Channel<Dma3Id, DmaId> = Channel { periph: DMA, index: 3, id: Dma3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma3Id {}
pub type Dma3 = Channel<Dma3Id, DmaId>;

pub const DMA4: Channel<Dma4Id, DmaId> = Channel { periph: DMA, index: 4, id: Dma4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma4Id {}
pub type Dma4 = Channel<Dma4Id, DmaId>;

pub const DMA5: Channel<Dma5Id, DmaId> = Channel { periph: DMA, index: 5, id: Dma5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma5Id {}
pub type Dma5 = Channel<Dma5Id, DmaId>;

pub const DMA6: Channel<Dma6Id, DmaId> = Channel { periph: DMA, index: 6, id: Dma6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma6Id {}
pub type Dma6 = Channel<Dma6Id, DmaId>;

pub const DMA7: Channel<Dma7Id, DmaId> = Channel { periph: DMA, index: 7, id: Dma7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma7Id {}
pub type Dma7 = Channel<Dma7Id, DmaId>;

pub const DMA8: Channel<Dma8Id, DmaId> = Channel { periph: DMA, index: 8, id: Dma8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma8Id {}
pub type Dma8 = Channel<Dma8Id, DmaId>;

pub const DMA9: Channel<Dma9Id, DmaId> = Channel { periph: DMA, index: 9, id: Dma9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma9Id {}
pub type Dma9 = Channel<Dma9Id, DmaId>;

pub const DMA10: Channel<Dma10Id, DmaId> = Channel { periph: DMA, index: 10, id: Dma10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma10Id {}
pub type Dma10 = Channel<Dma10Id, DmaId>;

pub const DMA11: Channel<Dma11Id, DmaId> = Channel { periph: DMA, index: 11, id: Dma11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma11Id {}
pub type Dma11 = Channel<Dma11Id, DmaId>;

pub const DMA12: Channel<Dma12Id, DmaId> = Channel { periph: DMA, index: 12, id: Dma12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma12Id {}
pub type Dma12 = Channel<Dma12Id, DmaId>;

pub const DMA13: Channel<Dma13Id, DmaId> = Channel { periph: DMA, index: 13, id: Dma13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma13Id {}
pub type Dma13 = Channel<Dma13Id, DmaId>;

pub const DMA14: Channel<Dma14Id, DmaId> = Channel { periph: DMA, index: 14, id: Dma14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma14Id {}
pub type Dma14 = Channel<Dma14Id, DmaId>;

pub const DMA15: Channel<Dma15Id, DmaId> = Channel { periph: DMA, index: 15, id: Dma15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma15Id {}
pub type Dma15 = Channel<Dma15Id, DmaId>;

pub trait IrqDmaError<T> {
   fn irq_dma_error(&self) -> super::irq::Irq<T>;
}

pub trait RegisterDmaErrorHandler {
   fn register_dma_error_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDmaError>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleDmaError {
   fn handle_dma_error(&self);
}

pub trait IrqDma<T> {
   fn irq_dma(&self) -> super::irq::Irq<T>;
}

pub trait RegisterDmaHandler {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleDma {
   fn handle_dma(&self);
}

impl IrqDmaError<super::irq::DmaErrorId> for Dma {
   fn irq_dma_error(&self) -> super::irq::IrqDmaError { super::irq::IRQ_DMA_ERROR }
}

impl RegisterDmaErrorHandler for Dma {
   fn register_dma_error_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDmaError>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDmaError>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma_error() }
       }
       super::irq::set_handler(16, Some(wrapper::<F>));
       super::irq::IrqGuard::new(16)
   }
}

impl IrqDma<super::irq::Dma0Id> for Dma0 {
   fn irq_dma(&self) -> super::irq::IrqDma0 { super::irq::IRQ_DMA0 }
}

impl RegisterDmaHandler for Dma0 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(0, Some(wrapper::<F>));
       super::irq::IrqGuard::new(0)
   }
}

impl IrqDma<super::irq::Dma1Id> for Dma1 {
   fn irq_dma(&self) -> super::irq::IrqDma1 { super::irq::IRQ_DMA1 }
}

impl RegisterDmaHandler for Dma1 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(1, Some(wrapper::<F>));
       super::irq::IrqGuard::new(1)
   }
}

impl IrqDma<super::irq::Dma2Id> for Dma2 {
   fn irq_dma(&self) -> super::irq::IrqDma2 { super::irq::IRQ_DMA2 }
}

impl RegisterDmaHandler for Dma2 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(2, Some(wrapper::<F>));
       super::irq::IrqGuard::new(2)
   }
}

impl IrqDma<super::irq::Dma3Id> for Dma3 {
   fn irq_dma(&self) -> super::irq::IrqDma3 { super::irq::IRQ_DMA3 }
}

impl RegisterDmaHandler for Dma3 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(3, Some(wrapper::<F>));
       super::irq::IrqGuard::new(3)
   }
}

impl IrqDma<super::irq::Dma4Id> for Dma4 {
   fn irq_dma(&self) -> super::irq::IrqDma4 { super::irq::IRQ_DMA4 }
}

impl RegisterDmaHandler for Dma4 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(4, Some(wrapper::<F>));
       super::irq::IrqGuard::new(4)
   }
}

impl IrqDma<super::irq::Dma5Id> for Dma5 {
   fn irq_dma(&self) -> super::irq::IrqDma5 { super::irq::IRQ_DMA5 }
}

impl RegisterDmaHandler for Dma5 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(5, Some(wrapper::<F>));
       super::irq::IrqGuard::new(5)
   }
}

impl IrqDma<super::irq::Dma6Id> for Dma6 {
   fn irq_dma(&self) -> super::irq::IrqDma6 { super::irq::IRQ_DMA6 }
}

impl RegisterDmaHandler for Dma6 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(6, Some(wrapper::<F>));
       super::irq::IrqGuard::new(6)
   }
}

impl IrqDma<super::irq::Dma7Id> for Dma7 {
   fn irq_dma(&self) -> super::irq::IrqDma7 { super::irq::IRQ_DMA7 }
}

impl RegisterDmaHandler for Dma7 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(7, Some(wrapper::<F>));
       super::irq::IrqGuard::new(7)
   }
}

impl IrqDma<super::irq::Dma8Id> for Dma8 {
   fn irq_dma(&self) -> super::irq::IrqDma8 { super::irq::IRQ_DMA8 }
}

impl RegisterDmaHandler for Dma8 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(8, Some(wrapper::<F>));
       super::irq::IrqGuard::new(8)
   }
}

impl IrqDma<super::irq::Dma9Id> for Dma9 {
   fn irq_dma(&self) -> super::irq::IrqDma9 { super::irq::IRQ_DMA9 }
}

impl RegisterDmaHandler for Dma9 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(9, Some(wrapper::<F>));
       super::irq::IrqGuard::new(9)
   }
}

impl IrqDma<super::irq::Dma10Id> for Dma10 {
   fn irq_dma(&self) -> super::irq::IrqDma10 { super::irq::IRQ_DMA10 }
}

impl RegisterDmaHandler for Dma10 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(10, Some(wrapper::<F>));
       super::irq::IrqGuard::new(10)
   }
}

impl IrqDma<super::irq::Dma11Id> for Dma11 {
   fn irq_dma(&self) -> super::irq::IrqDma11 { super::irq::IRQ_DMA11 }
}

impl RegisterDmaHandler for Dma11 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(11, Some(wrapper::<F>));
       super::irq::IrqGuard::new(11)
   }
}

impl IrqDma<super::irq::Dma12Id> for Dma12 {
   fn irq_dma(&self) -> super::irq::IrqDma12 { super::irq::IRQ_DMA12 }
}

impl RegisterDmaHandler for Dma12 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(12, Some(wrapper::<F>));
       super::irq::IrqGuard::new(12)
   }
}

impl IrqDma<super::irq::Dma13Id> for Dma13 {
   fn irq_dma(&self) -> super::irq::IrqDma13 { super::irq::IRQ_DMA13 }
}

impl RegisterDmaHandler for Dma13 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(13, Some(wrapper::<F>));
       super::irq::IrqGuard::new(13)
   }
}

impl IrqDma<super::irq::Dma14Id> for Dma14 {
   fn irq_dma(&self) -> super::irq::IrqDma14 { super::irq::IRQ_DMA14 }
}

impl RegisterDmaHandler for Dma14 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(14, Some(wrapper::<F>));
       super::irq::IrqGuard::new(14)
   }
}

impl IrqDma<super::irq::Dma15Id> for Dma15 {
   fn irq_dma(&self) -> super::irq::IrqDma15 { super::irq::IRQ_DMA15 }
}

impl RegisterDmaHandler for Dma15 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(15, Some(wrapper::<F>));
       super::irq::IrqGuard::new(15)
   }
}

