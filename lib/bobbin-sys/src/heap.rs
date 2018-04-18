use ::core::{mem, ptr, slice, str, fmt};

extern "C" {
    static mut _sheap: u32;
    static mut _eheap: u32;
    static mut _estack: u32;
}

static mut HEAP_START: *mut u8 = unsafe { &_sheap as *const u32 as *mut u8 };
static mut HEAP_END: *mut u8 = unsafe { &_sheap as *const u32 as *mut u8 };

#[derive(Debug)]
pub enum Error {
    OutOfSpace,
    InvalidAlign,
}

pub struct Heap {}

impl Heap {
    pub unsafe fn extend(&mut self, size: usize) {
        HEAP_END = HEAP_START.offset(size as isize);
    }

    #[inline]
    pub fn ptr(&self) -> *mut u8 {
        unsafe { HEAP_START }
    }

    #[inline]
    pub fn set_ptr(&mut self, ptr: *mut u8) {
        unsafe { HEAP_START = ptr }
    }

    #[inline]
    pub fn end(&self) -> *mut u8 {
        unsafe { HEAP_END }
    }

    #[inline]
    pub fn set_end(&mut self, ptr: *mut u8) {
        unsafe { HEAP_END = ptr }
    }


    #[inline]
    pub fn size(&self) -> usize {
        self.end() as usize - self.ptr() as usize
    }

    #[inline]
    pub fn freeze(&mut self) {
        let ptr = self.ptr();
        self.set_end(ptr)
    }

    #[inline]
    pub fn try_advance(&mut self, len: usize) -> Result<*mut u8, Error> {
        let new = unsafe { self.ptr().offset(len as isize) };
        if new as usize <= self.end() as usize {
        // if new.offset_to(self.end()).unwrap() >= 0 {
            let ptr = self.ptr();
            self.set_ptr(new);
            Ok(ptr)
        } else {
            Err(Error::OutOfSpace)            
        }
    }

    #[inline]
    pub fn advance(&mut self, len: usize) -> *mut u8 {
        self.try_advance(len).expect("Out of space")
    }

    #[inline]
    pub fn try_align(&mut self, align: usize) -> Result<(), Error> {
        Ok({
            if align != 1 {
                if align.count_ones() == 1 {
                    let ptr = self.ptr();
                    self.try_advance(unsafe { align_offset(ptr as *const (), align) } )?;
                } else {
                    return Err(Error::InvalidAlign)
                }
            }
        })
    }

    #[inline]
    pub fn align(&mut self, align: usize) {
        self.try_align(align).expect("Invalid alignment")
    }    

    #[inline]
    pub fn try_align_to<T>(&mut self) -> Result<(), Error> {
        self.try_align(mem::align_of::<T>())
    }    

    #[inline]
    pub fn align_to<T>(&mut self) {
        self.align(mem::align_of::<T>())
    }

    #[inline]
    pub fn try_alloc<T>(&mut self, len: usize) -> Result<*mut T, Error> {
        Ok({
            self.try_align_to::<T>()?;
            self.try_advance(mem::size_of::<T>() * len)? as *mut T
        })
    }

    #[inline]
    pub fn alloc<T>(&mut self, len: usize) -> *mut T {
        self.align_to::<T>();
        self.advance(mem::size_of::<T>() * len) as *mut T
    }

    #[inline]
    pub fn try_alloc_one<T>(&mut self) -> Result<*mut T, Error> {
        Ok({
            self.try_align_to::<T>()?;
            self.try_advance(mem::size_of::<T>())? as *mut T
        })
    }

    #[inline]
    pub fn alloc_one<T>(&mut self) -> *mut T {
        self.align_to::<T>();
        self.advance(mem::size_of::<T>()) as *mut T
    }

    #[inline]
    pub unsafe fn try_slice_uninitialized<T>(&mut self, len: usize) -> Result<&'static mut [T], Error> {        
        Ok({
            slice::from_raw_parts_mut(self.try_alloc(len)?, len)
        })
    }

    #[inline]
    pub unsafe fn slice_uninitialized<T>(&mut self, len: usize) -> &'static mut [T] {
        slice::from_raw_parts_mut(self.alloc(len), len)
    }

    #[inline]
    pub fn try_slice<T: Copy>(&mut self, val: T, len: usize) -> Result<&'static mut [T], Error> {
        Ok({
            let dst = unsafe { self.try_slice_uninitialized(len)? };
            for v in dst.iter_mut() { 
                *v = val;
            }
            dst
        })
    }    

    #[inline]
    pub fn slice<T: Copy>(&mut self, val: T, len: usize) -> &'static mut [T] {
        let dst = unsafe { self.slice_uninitialized(len) };
        for v in dst.iter_mut() { 
            *v = val;
        }
        dst
    }    

    #[inline]
    pub fn try_new<T>(&mut self, src: T) -> Result<&'static mut T, Error> {
        Ok({
            unsafe {
                let dst = self.try_alloc_one::<T>()?;
                ptr::write(dst, src);
                &mut *dst
            }
        })
    }

    #[inline]
    pub fn new<T>(&mut self, src: T) -> &'static mut T {
        unsafe {
            let dst = self.alloc_one::<T>();
            ptr::write(dst, src);
            &mut *dst
        }
    }

    #[inline]
    pub fn try_copy<T: Copy>(&mut self, src: &T) -> Result<&'static mut T, Error> {
        Ok({
            unsafe {
                let dst = self.alloc_one::<T>();
                *dst = *src;
                &mut *dst
            }
        })
    }

    #[inline]
    pub fn copy<T: Copy>(&mut self, src: &T) -> &'static mut T {
        unsafe {
            let dst = self.alloc_one::<T>();
            *dst = *src;
            &mut *dst
        }
    }

    #[inline]
    pub fn try_copy_slice<T>(&mut self, src: &[T]) -> Result<&'static mut [T], Error> {
        Ok({
            unsafe {
                let len = src.len();
                let dst: *mut T = self.alloc(len);
                ptr::copy_nonoverlapping(src.as_ptr(), dst, len);
                slice::from_raw_parts_mut(dst, len)
            }
        })
    }

    #[inline]
    pub fn copy_slice<T>(&mut self, src: &[T]) -> &'static mut [T] {
        unsafe {
            let len = src.len();
            let dst: *mut T = self.alloc(len);
            ptr::copy_nonoverlapping(src.as_ptr(), dst, len);
            slice::from_raw_parts_mut(dst, len)
        }
    }

    #[inline]
    pub fn try_copy_str(&mut self, src: &str) -> Result<&'static str, Error> {
        Ok({
            unsafe { str::from_utf8_unchecked(self.try_copy_slice(src.as_bytes())?) }
        })
    }

    #[inline]
    pub fn copy_str(&mut self, src: &str) -> &'static str {
        unsafe { str::from_utf8_unchecked(self.copy_slice(src.as_bytes())) }
    }    
}

impl fmt::Debug for Heap {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Heap {{ ptr: {:p} end: {:p} size: 0x{:x} }}", self.ptr(), self.end(), self.size())?;
        Ok(())
    }
}

pub unsafe fn align_offset(ptr: *const (), align: usize) -> usize {
    let offset = ptr as usize % align;
    if offset == 0 {
        0
    } else {
        align.wrapping_sub(offset)
    }
}