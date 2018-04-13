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