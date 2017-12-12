use core::cell::UnsafeCell;
// use core::mem;
use core::ptr;
use core::intrinsics;
use alloc::allocator::{Alloc, Layout, AllocErr};

pub struct Heap {
    ptr: UnsafeCell<usize>,
    end: UnsafeCell<usize>,
    allocated: UnsafeCell<usize>,
}

impl Heap {
    pub const fn empty() -> Self {
        Heap {
            ptr: UnsafeCell::new(0),
            end: UnsafeCell::new(0),
            allocated: UnsafeCell::new(0),
        }
    }

    pub unsafe fn init(&mut self, buf: &'static mut [u8]) {
        let len = buf.len();
        let ptr = buf.as_ptr();
        let end = ptr.offset(len as isize);
        self.set_ptr(ptr as *const u8 as usize);
        self.set_end(end as *const u8 as usize);
        self.set_allocated(0);
    }

    #[inline]
    pub fn ptr(&self) -> usize {
        unsafe { ptr::read_volatile(self.ptr.get()) }
    }

    #[inline]
    pub fn set_ptr(&self, value: usize) {
        unsafe { ptr::write_volatile(self.ptr.get(), value) }
    }

    #[inline]
    pub fn end(&self) -> usize {
        unsafe { ptr::read_volatile(self.end.get()) }
    }

    #[inline]
    pub fn set_end(&self, value: usize) {
        unsafe { ptr::write_volatile(self.end.get(), value) }
    }

    #[inline]
    pub fn allocated(&self) -> usize {
        unsafe { ptr::read_volatile(self.allocated.get()) }
    }

    #[inline]
    pub fn set_allocated(&self, value: usize) {
        unsafe { ptr::write_volatile(self.allocated.get(), value) }
    }

    pub fn available(&self) -> usize {
        self.end() - self.allocated()
    }
}

unsafe impl Sync for Heap {}

unsafe impl<'a> Alloc for &'a Heap {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let ptr = self.ptr();
        let size = layout.size();
        let next = ptr + size;
        if next <= self.end() {
            self.set_ptr(next);
            self.set_allocated(self.allocated() +size);
            Ok(ptr as *mut u8)
        } else {
            intrinsics::abort()
        }
    }

    unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        
    }
}
