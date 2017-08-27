#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

pub extern crate bobbin_bits as bits;
pub mod clock;
pub mod timer;
pub mod digital;
pub mod analog;
pub mod serial;

pub use core::ops::Deref;

#[cfg(not(test))] use core::ptr::{read_volatile, write_volatile};


#[cfg(test)] mod vm;
#[cfg(test)] use vm::Vm;
#[cfg(test)] use std::cell::RefCell;
#[cfg(test)] thread_local!(pub static MEM: RefCell<Vm> = RefCell::new(Vm::new()));

#[cfg(not(test))]
pub trait Base {
    #[inline(always)]
    fn base(&self) -> usize;

    #[inline(always)]
    fn addr<T>(&self, offset: usize) -> *mut T {
        (self.base() + offset) as *mut T
    }

    #[inline(always)]
    fn read<T>(&self, offset: usize) -> T {
        unsafe { read_volatile(self.addr(offset)) }
    }    

    #[inline(always)]
    fn write<T>(&self, offset: usize, value: T) -> &Self {
        unsafe { write_volatile(self.addr(offset), value); }
        self
    }
}

#[cfg(test)]
pub trait Base {
    #[inline(always)]
    fn base(&self) -> usize;

    fn addr<T>(&self, offset: usize) -> *mut T {
        (self.base() + offset) as *mut T
    }

    fn read<T>(&self, offset: usize) -> T {
        let addr = self.addr(offset);
        MEM.with(|m| m.borrow().read(addr))
    }

    fn write<T>(&self, offset: usize, value: T) -> &Self {
        let addr = self.addr(offset);
        MEM.with(|m| m.borrow_mut().write(addr, value));
        self
    }
}

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

pub trait Irq<T> {
    fn irq(&self) -> T;
}

pub trait IrqNum {
    fn irq_num(&self) -> u8;
}

pub type Handler = extern "C" fn();

pub trait HandleIrq {
   fn handle_irq(&self);
}

pub trait WrapHandleIrq {
    fn wrap_handle_irq<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleIrq>(&self, f: &F) -> Handler;
}

#[macro_export]
macro_rules! periph {
    ($id:ident, $ty:ident, $pid:ident, $pty:ident, $base:expr) => (
        pub const $id: $ty = $ty {};     
        pub const $pid: $pty = $pty($base);
        pub struct $ty {}
        impl Deref for $ty {
            type Target = $pty;
            fn deref(&self) -> &$pty {
                &$pid
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
        pub struct $ty {}
        impl Pin<$port_type> for $ty {
            fn port(&self) -> $port_type { $port_id }
            fn index(&self) -> usize { $index }
        }
        impl Deref for $ty {
            type Target = $base_type;
            fn deref(&self) -> &$base_type {
                &$base_id
            }
        }
    )
}

#[macro_export]
macro_rules! channel {
    ($id:ident, $ty:ident, $periph_id:ident, $periph_type:ident, $base_id:ident, $base_type:ident, $base_periph:ident, $index:expr) => (
        pub const $id: $ty = $ty {};     
        pub const $base_id: $base_type = $base_type { periph: $base_periph, index: $index };
        pub struct $ty {}
        impl Channel<$periph_type> for $ty {
            fn periph(&self) -> $periph_type { $periph_id }
            fn index(&self) -> usize { $index }
        }
        impl Deref for $ty {
            type Target = $base_type;
            fn deref(&self) -> &$base_type {
                &$base_id
            }
        }
    )    
}

#[macro_export]
macro_rules! alt_fn {
    ($ty:ty, $sig:ty, $num:expr) => (
        impl AltFn<$sig> for $ty {
            fn alt_fn(&self) -> usize { $num }
        }
        
    )

}

#[macro_export]
macro_rules! irq {
    ($id:ident, $ty:ident, $num:expr) => (
        pub const $id: $ty = $ty {};
        pub struct $ty {}
        impl IrqNum for $ty {
            fn irq_num(&self) -> u8 { $num }
        }
        impl WrapHandleIrq for $ty {
            fn wrap_handle_irq<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleIrq>(&self, f: &F) -> Handler {
                static mut HANDLER: Option<usize> = None;                
                unsafe { 
                    assert!(HANDLER.is_none(), "Irq is already wrapping a function");
                    HANDLER = Some(f as *const F as usize)
                }
                extern "C" fn wrapper<W: HandleIrq>() {
                    unsafe { (*(HANDLER.unwrap() as *const W)).handle_irq() }
                }
                wrapper::<F>
            }
        }
    )
}
