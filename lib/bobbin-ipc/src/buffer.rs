#![no_std]
#![allow(unused_imports, dead_code)]

use core::mem;
use core::ops;
use core::slice;

pub struct Buffer {
    pub header: BufferHeader,
}

#[repr(C)]
pub struct BufferHeader {
    len: u32,
    pos: u32,
}

impl Buffer {
    pub fn slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts((self as *const Buffer as *const u8).offset(mem::size_of::<BufferHeader>() as isize), self.header.len as usize)
        }
    }
    pub fn slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut((self as *mut Buffer as *mut u8).offset(mem::size_of::<BufferHeader>() as isize), self.header.len as usize)
        }
    }    
}





#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_buffer() {
        static mut DATA: [u8; 64] = [0u8; 64];
        let b = unsafe {
            let b = &mut *(DATA.as_mut_ptr() as *mut Buffer);
            b.header = BufferHeader {
                len: (DATA.len() - mem::size_of::<BufferHeader>()) as u32,
                pos: 0,
            };
            b
        };
        for (i, d) in b.slice_mut().iter_mut().enumerate() {
            *d = i as u8;
        }
        unsafe {
            for i in 0..56 {
                assert_eq!(DATA[i + 8], i as u8);
            }
        }
    }
}