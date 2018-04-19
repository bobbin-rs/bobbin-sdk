#![no_std]
#![feature(asm, use_extern_macros)]

pub extern crate bobbin_bits;
pub extern crate bobbin_mcu;
pub extern crate bobbin_hal;
pub extern crate bobbin_sys;

pub mod ext;
pub mod nvic;
pub mod scb;
pub mod systick;
pub mod mpu;
pub mod fpu;
pub mod dcb;
pub mod itm;
pub mod dwt;
pub mod sig;
pub mod pin;

pub const CORTEXM: Cortexm = Cortexm {};
pub struct Cortexm {}

impl Cortexm {
    pub fn is_cortexm(&self) -> bool {
        true
    }
}