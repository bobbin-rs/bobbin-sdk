#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::dma_f3::*;

pub const DMA1: Dma1 = Periph(0x40020000, Dma1Id {});
pub const DMA2: Dma2 = Periph(0x40020400, Dma2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Dma1Id {}
pub type Dma1 = Periph<Dma1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Dma2Id {}
pub type Dma2 = Periph<Dma2Id>;




pub const DMA1_CH1: Channel<Dma1Ch1Id, Dma1Id> = Channel { periph: DMA1, index: 0, id: Dma1Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch1Id {}
pub type Dma1Ch1 = Channel<Dma1Ch1Id, Dma1Id>;

pub const DMA1_CH2: Channel<Dma1Ch2Id, Dma1Id> = Channel { periph: DMA1, index: 1, id: Dma1Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch2Id {}
pub type Dma1Ch2 = Channel<Dma1Ch2Id, Dma1Id>;

pub const DMA1_CH3: Channel<Dma1Ch3Id, Dma1Id> = Channel { periph: DMA1, index: 2, id: Dma1Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch3Id {}
pub type Dma1Ch3 = Channel<Dma1Ch3Id, Dma1Id>;

pub const DMA1_CH4: Channel<Dma1Ch4Id, Dma1Id> = Channel { periph: DMA1, index: 3, id: Dma1Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch4Id {}
pub type Dma1Ch4 = Channel<Dma1Ch4Id, Dma1Id>;

pub const DMA1_CH5: Channel<Dma1Ch5Id, Dma1Id> = Channel { periph: DMA1, index: 4, id: Dma1Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch5Id {}
pub type Dma1Ch5 = Channel<Dma1Ch5Id, Dma1Id>;

pub const DMA1_CH6: Channel<Dma1Ch6Id, Dma1Id> = Channel { periph: DMA1, index: 5, id: Dma1Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch6Id {}
pub type Dma1Ch6 = Channel<Dma1Ch6Id, Dma1Id>;

pub const DMA1_CH7: Channel<Dma1Ch7Id, Dma1Id> = Channel { periph: DMA1, index: 6, id: Dma1Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch7Id {}
pub type Dma1Ch7 = Channel<Dma1Ch7Id, Dma1Id>;

pub const DMA2_CH1: Channel<Dma2Ch1Id, Dma2Id> = Channel { periph: DMA2, index: 0, id: Dma2Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Ch1Id {}
pub type Dma2Ch1 = Channel<Dma2Ch1Id, Dma2Id>;

pub const DMA2_CH2: Channel<Dma2Ch2Id, Dma2Id> = Channel { periph: DMA2, index: 1, id: Dma2Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Ch2Id {}
pub type Dma2Ch2 = Channel<Dma2Ch2Id, Dma2Id>;

pub const DMA2_CH3: Channel<Dma2Ch3Id, Dma2Id> = Channel { periph: DMA2, index: 2, id: Dma2Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Ch3Id {}
pub type Dma2Ch3 = Channel<Dma2Ch3Id, Dma2Id>;

pub const DMA2_CH4: Channel<Dma2Ch4Id, Dma2Id> = Channel { periph: DMA2, index: 3, id: Dma2Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Ch4Id {}
pub type Dma2Ch4 = Channel<Dma2Ch4Id, Dma2Id>;

pub const DMA2_CH5: Channel<Dma2Ch5Id, Dma2Id> = Channel { periph: DMA2, index: 4, id: Dma2Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma2Ch5Id {}
pub type Dma2Ch5 = Channel<Dma2Ch5Id, Dma2Id>;

pub trait IrqDma<T> {
   fn irq_dma(&self) -> super::irq::Irq<T>;
}

pub trait RegisterDmaHandler {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleDma {
   fn handle_dma(&self);
}

impl IrqDma<super::irq::Dma1Ch1Id> for Dma1Ch1 {
   fn irq_dma(&self) -> super::irq::IrqDma1Ch1 { super::irq::IRQ_DMA1_CH1 }
}

impl RegisterDmaHandler for Dma1Ch1 {
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

impl IrqDma<super::irq::Dma1Ch2Id> for Dma1Ch2 {
   fn irq_dma(&self) -> super::irq::IrqDma1Ch2 { super::irq::IRQ_DMA1_CH2 }
}

impl RegisterDmaHandler for Dma1Ch2 {
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

impl IrqDma<super::irq::Dma1Ch3Id> for Dma1Ch3 {
   fn irq_dma(&self) -> super::irq::IrqDma1Ch3 { super::irq::IRQ_DMA1_CH3 }
}

impl RegisterDmaHandler for Dma1Ch3 {
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

impl IrqDma<super::irq::Dma1Ch4Id> for Dma1Ch4 {
   fn irq_dma(&self) -> super::irq::IrqDma1Ch4 { super::irq::IRQ_DMA1_CH4 }
}

impl RegisterDmaHandler for Dma1Ch4 {
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

impl IrqDma<super::irq::Dma1Ch5Id> for Dma1Ch5 {
   fn irq_dma(&self) -> super::irq::IrqDma1Ch5 { super::irq::IRQ_DMA1_CH5 }
}

impl RegisterDmaHandler for Dma1Ch5 {
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

impl IrqDma<super::irq::Dma1Ch6Id> for Dma1Ch6 {
   fn irq_dma(&self) -> super::irq::IrqDma1Ch6 { super::irq::IRQ_DMA1_CH6 }
}

impl RegisterDmaHandler for Dma1Ch6 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(16, Some(wrapper::<F>));
       super::irq::IrqGuard::new(16)
   }
}

impl IrqDma<super::irq::Dma1Ch7Id> for Dma1Ch7 {
   fn irq_dma(&self) -> super::irq::IrqDma1Ch7 { super::irq::IRQ_DMA1_CH7 }
}

impl RegisterDmaHandler for Dma1Ch7 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(17, Some(wrapper::<F>));
       super::irq::IrqGuard::new(17)
   }
}

impl IrqDma<super::irq::Dma2Ch1Id> for Dma2Ch1 {
   fn irq_dma(&self) -> super::irq::IrqDma2Ch1 { super::irq::IRQ_DMA2_CH1 }
}

impl RegisterDmaHandler for Dma2Ch1 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(56, Some(wrapper::<F>));
       super::irq::IrqGuard::new(56)
   }
}

impl IrqDma<super::irq::Dma2Ch2Id> for Dma2Ch2 {
   fn irq_dma(&self) -> super::irq::IrqDma2Ch2 { super::irq::IRQ_DMA2_CH2 }
}

impl RegisterDmaHandler for Dma2Ch2 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(57, Some(wrapper::<F>));
       super::irq::IrqGuard::new(57)
   }
}

impl IrqDma<super::irq::Dma2Ch3Id> for Dma2Ch3 {
   fn irq_dma(&self) -> super::irq::IrqDma2Ch3 { super::irq::IRQ_DMA2_CH3 }
}

impl RegisterDmaHandler for Dma2Ch3 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(58, Some(wrapper::<F>));
       super::irq::IrqGuard::new(58)
   }
}

impl IrqDma<super::irq::Dma2Ch4Id> for Dma2Ch4 {
   fn irq_dma(&self) -> super::irq::IrqDma2Ch4 { super::irq::IRQ_DMA2_CH4 }
}

impl RegisterDmaHandler for Dma2Ch4 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(59, Some(wrapper::<F>));
       super::irq::IrqGuard::new(59)
   }
}

impl IrqDma<super::irq::Dma2Ch5Id> for Dma2Ch5 {
   fn irq_dma(&self) -> super::irq::IrqDma2Ch5 { super::irq::IRQ_DMA2_CH5 }
}

impl RegisterDmaHandler for Dma2Ch5 {
   fn register_dma_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleDma>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleDma>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_dma() }
       }
       super::irq::set_handler(59, Some(wrapper::<F>));
       super::irq::IrqGuard::new(59)
   }
}

