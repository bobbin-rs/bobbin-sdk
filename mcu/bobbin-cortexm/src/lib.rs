#![no_std]
#![feature(asm)]

extern crate bobbin_common;

pub mod chip;
pub mod hal;

#[inline(always)]
pub fn wfi() {
    unsafe { asm!("wfi")}
}

#[inline(always)]
pub fn wfe() {
    unsafe { asm!("wfe")}
}

#[inline(always)]
pub fn nop() {
    unsafe { asm!{"nop"}}
}

#[cfg(test)]
mod tests;