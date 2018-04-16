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
pub fn read<T>(addr: *const T) -> T {
    unsafe { read_volatile(addr as *const T) }
}

#[inline]
pub fn read_u32(addr: *const u32) -> u32 {
    read(addr)
}

#[inline]
pub fn read_u16(addr: *const u16) -> u16 {
    read(addr)
}

#[inline]
pub fn read_u8(addr: *const u8) -> u8 {
    read(addr)
}

#[inline]
pub fn write<T>(addr: *mut T, value: T) {
    unsafe { write_volatile(addr as *mut T, value) }
}

#[inline]
pub fn write_u32(addr: *mut u32, value: u32) {
    write(addr, value)
}

#[inline]
pub fn write_u16(addr: *mut u16, value: u16) {
    write(addr, value)
}

#[inline]
pub fn write_u8(addr: *mut u8, value: u8) {
    write(addr, value)
}    