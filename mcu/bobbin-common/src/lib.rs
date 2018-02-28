#![no_std]
#![feature(use_extern_macros, const_fn, core_intrinsics, allocator_api, alloc)]

#[cfg(not(target_os="none"))]
#[macro_use]
extern crate std;

// pub extern crate alloc;
pub extern crate bobbin_bits as bits;

#[macro_use]
mod macros;

pub mod periph;
pub mod pin;
pub mod channel;
pub mod irq;
pub mod signal;
pub mod gate;

pub mod clock;
pub mod crc;
pub mod timer;
pub mod digital;
pub mod analog;
pub mod serial;
pub mod spi;
pub mod i2c;
pub mod can;
pub mod watchdog;
pub mod configure;
pub mod enabled;
pub mod ring;
// pub mod heap;

pub use periph::*;
pub use pin::*;
pub use channel::*;
pub use irq::*;


#[cfg(not(target_os="none"))]
mod vm;

pub use core::ops::Deref;

#[cfg(target_os="none")]
pub use core::ptr::{read_volatile, write_volatile};

#[cfg(not(target_os="none"))]
pub mod rw;

#[cfg(not(target_os="none"))]
pub use rw::*;

pub trait En {
    fn en(&self) -> bits::U1;
    fn set_en<V: Into<bits::U1>>(&self, value: V);
}    

pub trait ClockEnable: En {
    fn clock_enable(&self) -> &Self {
        self.set_en(true);
        self
    }
}

impl<T> ClockEnable for T where T: En {}


// impl<PORT, T> ClockEnable for T
// where 
//     PORT: En + Periph,
//     Self: pin::Pin<PORT>
// {
//     fn clock_enable(&self) -> &Self {
//         self.port().set_en(true);
//         self
//     }    
// }



pub trait PortEn<PORT> : pin::Pin<PORT> where PORT: En + Periph {
    fn port_en(&self) -> bits::U1 { self.port().en() }
    fn port_set_en<V: Into<bits::U1>>(&self, value: V) -> &Self { self.port().set_en(value); self }
    fn port_enable(&self) -> &Self { self.port_set_en(true) }
    fn port_disable(&self) -> &Self { self.port_set_en(false) }
}

impl<PORT, T> PortEn<PORT> for T
where 
    PORT: En + Periph,
    Self: pin::Pin<PORT>
{}

// impl<PORT> PortEn<PORT> for pin::Pin<PORT>
// {
//     fn en(&self) -> bits::U1 {
//         self.port().en()
//     }

//     fn set_en<V: Into<bits::U1>>(&self, value: V) {
//         self.port().set_en(value);
//     }    
// }

// impl<PORT: En + periph::Periph> En for pin::Pin<PORT>
// {
//     fn en(&self) -> bits::U1 {
//         self.port().en()
//     }

//     fn set_en<V: Into<bits::U1>>(&self, value: V) {
//         self.port().set_en(value);
//     }
// }