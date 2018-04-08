pub use periph::*;

pub mod memory;
pub mod heap;
pub mod systick_ext;
pub mod nvic_ext;

pub use memory::*;
pub use heap::*;

#[inline(always)]
pub fn wfi() {
    #[cfg(target_os="none")]
    unsafe { asm!("wfi")}
}

#[inline(always)]
pub fn wfe() {
    #[cfg(target_os="none")]
    unsafe { asm!("wfe")}
}

#[inline(always)]
pub fn nop() {
    #[cfg(target_os="none")]
    unsafe { asm!{"nop"}}
}
