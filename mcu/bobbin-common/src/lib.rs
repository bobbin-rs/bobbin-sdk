#![no_std]

#[cfg(not(target_os="none"))]
#[macro_use]
extern crate std;

pub extern crate bobbin_bits as bits;
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
pub mod reset;


#[cfg(not(target_os="none"))]
mod vm;

pub use core::ops::Deref;

#[cfg(target_os="none")]
pub use core::ptr::{read_volatile, write_volatile};

#[cfg(not(target_os="none"))]
pub mod rw {
    use vm::Vm;
    use std::cell::RefCell;

    thread_local!(pub static MEM: RefCell<Vm> = RefCell::new(Vm::new()));

    pub fn reset_vm() {
        MEM.with(|m| m.borrow_mut().reset());
    }    

    pub fn add_region(addr: usize, len: usize) {
        MEM.with(|m| m.borrow_mut().add_region(addr, len));
    }    

    pub unsafe fn read_volatile<T>(addr: *const T) -> T {
        MEM.with(|m| m.borrow().read(addr))
    }

    pub unsafe fn write_volatile<T>(addr: *mut T, value: T) {
        MEM.with(|m| m.borrow_mut().write(addr, value));
    }



    #[inline]
    pub fn read<T>(addr: usize) -> T {
        unsafe { read_volatile(addr as *const T) }
    }

    #[inline]
    pub fn read_u32(addr: usize) -> u32 {
        read(addr)
    }

    #[inline]
    pub fn read_u16(addr: usize) -> u16 {
        read(addr)
    }

    #[inline]
    pub fn read_u8(addr: usize) -> u8 {
        read(addr)
    }

    #[inline]
    pub fn write<T>(addr: usize, value: T) {
        unsafe { write_volatile(addr as *mut T, value) }
    }

    #[inline]
    pub fn write_u32(addr: usize, value: u32) {
        write(addr, value)
    }

    #[inline]
    pub fn write_u16(addr: usize, value: u16) {
        write(addr, value)
    }

    #[inline]
    pub fn write_u8(addr: usize, value: u8) {
        write(addr, value)
    }    
}

#[cfg(not(target_os="none"))]
pub use rw::*;

pub trait Pin<T> {
    fn port(&self) -> T;
    fn index(&self) -> usize;
}

pub trait Channel<T> {
    fn periph(&self) -> T;
    fn index(&self) -> usize;
}

pub trait AltFn<T> {
    fn alt_fn(&self) -> usize;
}

pub trait Irq {
    fn irq_num(&self) -> u8;
    fn handler(&self) -> Option<Handler>;
    fn set_handler(&self, Option<Handler>);
    fn wrap<'a, F: ::core::marker::Sync + ::core::marker::Send + Poll>(&self, f: &F) -> extern "C" fn();   
}

pub type Handler = extern "C" fn();

pub trait Poll {
   fn poll(&self);
}

impl<T: Fn()> Poll for T {
    fn poll(&self) {
        self()
    }
}

#[macro_export]
macro_rules! periph {
    ($id:ident, $ty:ident, $pid:ident, $pty:ident, $base:expr) => (
        pub const $id: $ty = $ty {};     
        pub const $pid: $pty = $pty($base);
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub struct $ty {}
        impl Deref for $ty {
            type Target = $pty;
            #[inline(always)]            
            fn deref(&self) -> &$pty {
                &$pid
            }
        }
        impl Into<$pty> for $ty {
            #[inline(always)]
            fn into(self) -> $pty {
                $pid
            }
        }        
    );
    ($id:ident, $ty:ident, $base:expr) => (    
        pub const $id: $ty = $ty($base);
    )
}

#[macro_export]
macro_rules! pin {
    ($id:ident, $ty:ident, $port_id:ident, $port_type:ident, $base_id:ident, $base_type:ident, $base_port:ident, $index:expr) => (
        pub const $id: $ty = $ty {};     
        pub const $base_id: $base_type = $base_type { port: $base_port, index: $index };
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub struct $ty {}
        impl Pin<$port_type> for $ty {
            #[inline(always)]
            fn port(&self) -> $port_type { $port_id }
            #[inline(always)]            
            fn index(&self) -> usize { $index }
        }
        impl Deref for $ty {
            type Target = $base_type;
            #[inline(always)]            
            fn deref(&self) -> &$base_type {
                &$base_id
            }
        }
        impl Into<$base_type> for $ty {
            #[inline(always)]
            fn into(self) -> $base_type {
                $base_id
            }
        }
    )
}

#[macro_export]
macro_rules! channel {
    ($id:ident, $ty:ident, $periph_id:ident, $periph_type:ident, $base_id:ident, $base_type:ident, $base_periph:ident, $index:expr) => (
        pub const $id: $ty = $ty {};     
        pub const $base_id: $base_type = $base_type { periph: $base_periph, index: $index };
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub struct $ty {}
        impl Channel<$periph_type> for $ty {
            #[inline(always)]
            fn periph(&self) -> $periph_type { $periph_id }
            #[inline(always)]            
            fn index(&self) -> usize { $index }
        }
        impl Deref for $ty {
            type Target = $base_type;
            #[inline(always)]            
            fn deref(&self) -> &$base_type {
                &$base_id
            }
        }
        impl Into<$base_type> for $ty {
            #[inline(always)]
            fn into(self) -> $base_type {
                $base_id
            }
        }        
    )    
}

#[macro_export]
macro_rules! alt_fn {
    ($ty:ty, $sig:ty, $num:expr) => (
        impl AltFn<$sig> for $ty {
            #[inline(always)]            
            fn alt_fn(&self) -> usize { $num }
        }
        
    )

}

#[macro_export]
macro_rules! irq {
    ($id:ident, $ty:ident, $num:expr) => (
        pub const $id: $ty = $ty {};
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct $ty {}
        impl Irq for $ty {
            #[inline(always)]            
            fn irq_num(&self) -> u8 { $num }

            #[inline]
            fn handler(&self) ->  Option<Handler> {
                handler($num)
            }

            #[inline]
            fn set_handler(&self, h: Option<Handler>) {
                set_handler($num, h);
            }

            fn wrap<'a, F: ::core::marker::Sync + ::core::marker::Send + Poll>(&self, f: &F) -> extern "C" fn() {
                static mut HANDLER: Option<usize> = None;                
                unsafe { 
                    // assert!(HANDLER.is_none(), "Irq is already wrapping a function");
                    HANDLER = Some(f as *const F as usize)
                }
                extern "C" fn wrapper<W: Poll>() {
                    unsafe { (*(HANDLER.unwrap() as *const W)).poll() }
                }
                wrapper::<F>
            }
        }
        impl ::core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                write!(f, "[{} @ {}]", stringify!($id), $num)
            }
        }    
        unsafe impl Sync for $ty {}
        unsafe impl Send for $ty {}
    )
}
