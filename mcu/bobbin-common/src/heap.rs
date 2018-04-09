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
    pub unsafe fn extend(size: usize) {
        HEAP_END = HEAP_START.offset(size as isize);
    }

    #[inline]
    pub fn ptr() -> *mut u8 {
        unsafe { HEAP_START }
    }

    #[inline]
    pub fn set_ptr(ptr: *mut u8) {
        unsafe { HEAP_START = ptr }
    }

    #[inline]
    pub fn end() -> *mut u8 {
        unsafe { HEAP_END }
    }

    #[inline]
    pub fn set_end(ptr: *mut u8) {
        unsafe { HEAP_END = ptr }
    }


    #[inline]
    pub fn size() -> usize {
        Self::end() as usize - Self::ptr() as usize
    }

    #[inline]
    pub fn freeze() {
        Self::set_end(Self::ptr())
    }

    #[inline]
    pub fn try_advance(len: usize) -> Result<*mut u8, Error> {
        let new = unsafe { Self::ptr().offset(len as isize) };
        if new as usize <= Self::end() as usize {
        // if new.offset_to(Self::end()).unwrap() >= 0 {
            let ptr = Self::ptr();
            Self::set_ptr(new);
            Ok(ptr)
        } else {
            Err(Error::OutOfSpace)            
        }
    }

    #[inline]
    pub fn advance(len: usize) -> *mut u8 {
        Self::try_advance(len).expect("Out of space")
    }

    #[inline]
    pub fn try_align(align: usize) -> Result<(), Error> {
        Ok({
            if align != 1 {
                if align.count_ones() == 1 {
                    Self::try_advance(unsafe { align_offset(Self::ptr() as *const (), align) } )?;
                } else {
                    return Err(Error::InvalidAlign)
                }
            }
        })
    }

    #[inline]
    pub fn align(align: usize) {
        Self::try_align(align).expect("Invalid alignment")
    }    

    #[inline]
    pub fn try_align_to<T>() -> Result<(), Error> {
        Self::try_align(mem::align_of::<T>())
    }    

    #[inline]
    pub fn align_to<T>() {
        Self::align(mem::align_of::<T>())
    }

    #[inline]
    pub unsafe fn try_alloc<T>(len: usize) -> Result<*mut T, Error> {
        Ok({
            Self::try_align_to::<T>()?;
            Self::try_advance(mem::size_of::<T>() * len)? as *mut T
        })
    }

    #[inline]
    pub unsafe fn alloc<T>(len: usize) -> *mut T {
        Self::align_to::<T>();
        Self::advance(mem::size_of::<T>() * len) as *mut T
    }

    #[inline]
    pub unsafe fn try_alloc_one<T>() -> Result<*mut T, Error> {
        Ok({
            Self::try_align_to::<T>()?;
            Self::try_advance(mem::size_of::<T>())? as *mut T
        })
    }

    #[inline]
    pub unsafe fn alloc_one<T>() -> *mut T {
        Self::align_to::<T>();
        Self::advance(mem::size_of::<T>()) as *mut T
    }

    #[inline]
    pub unsafe fn try_slice_uninitialized<T>(len: usize) -> Result<&'static mut [T], Error> {        
        Ok({
            slice::from_raw_parts_mut(Self::try_alloc(len)?, len)
        })
    }

    #[inline]
    pub unsafe fn slice_uninitialized<T>(len: usize) -> &'static mut [T] {
        slice::from_raw_parts_mut(Self::alloc(len), len)
    }

    #[inline]
    pub fn try_slice<T: Copy>(val: T, len: usize) -> Result<&'static mut [T], Error> {
        Ok({
            let dst = unsafe { Self::try_slice_uninitialized(len)? };
            for i in 0..len { 
                dst[i] = val;
            }
            dst
        })
    }    

    #[inline]
    pub fn slice<T: Copy>(val: T, len: usize) -> &'static mut [T] {
        let dst = unsafe { Self::slice_uninitialized(len) };
        for i in 0..len { 
            dst[i] = val;
        }
        dst
    }    

    #[inline]
    pub fn try_new<T>(src: T) -> Result<&'static mut T, Error> {
        Ok({
            unsafe {
                let dst = Self::try_alloc_one::<T>()?;
                ptr::write(dst, src);
                &mut *dst
            }
        })
    }

    #[inline]
    pub fn new<T>(src: T) -> &'static mut T {
        unsafe {
            let dst = Self::alloc_one::<T>();
            ptr::write(dst, src);
            &mut *dst
        }
    }

    #[inline]
    pub fn try_copy<T: Copy>(src: &T) -> Result<&'static mut T, Error> {
        Ok({
            unsafe {
                let dst = Self::alloc_one::<T>();
                *dst = *src;
                &mut *dst
            }
        })
    }

    #[inline]
    pub fn copy<T: Copy>(src: &T) -> &'static mut T {
        unsafe {
            let dst = Self::alloc_one::<T>();
            *dst = *src;
            &mut *dst
        }
    }

    #[inline]
    pub fn try_copy_slice<T>(src: &[T]) -> Result<&'static mut [T], Error> {
        Ok({
            unsafe {
                let len = src.len();
                let dst: *mut T = Self::alloc(len);
                ptr::copy_nonoverlapping(src.as_ptr(), dst, len);
                slice::from_raw_parts_mut(dst, len)
            }
        })
    }

    #[inline]
    pub fn copy_slice<T>(src: &[T]) -> &'static mut [T] {
        unsafe {
            let len = src.len();
            let dst: *mut T = Self::alloc(len);
            ptr::copy_nonoverlapping(src.as_ptr(), dst, len);
            slice::from_raw_parts_mut(dst, len)
        }
    }

    #[inline]
    pub fn try_copy_str(src: &str) -> Result<&'static str, Error> {
        Ok({
            unsafe { str::from_utf8_unchecked(Self::try_copy_slice(src.as_bytes())?) }
        })
    }

    #[inline]
    pub fn copy_str(src: &str) -> &'static str {
        unsafe { str::from_utf8_unchecked(Self::copy_slice(src.as_bytes())) }
    }    
}

impl fmt::Debug for Heap {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Heap {{ ptr: {:p} end: {:p} size: 0x{:x} }}", Self::ptr(), Self::end(), Self::size())?;
        Ok(())
    }
}

pub unsafe fn align_offset(ptr: *const (), align: usize) -> usize {
    let offset = ptr as usize % align;
    if offset == 0 {
        0
    } else {
        align - offset
    }
}