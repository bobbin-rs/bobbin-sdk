//! Memory allocation
//! 
//! The Heap service allocates device memory using a simple bump
//! allocator. It does not support freeing memory.
//! 
//! All allocations return a `&'static mut` reference to avoid 
//! dependencies on the `alloc` crate.
//! 
//! The service is implemented as a global singleton. It is
//! typically accessed through [System](../system/index.html), which
//! will ensure that it is only used in a single-threaded environment
//! with interrupts disabled. Only one instance may exist at
//! any time.
//! 
//! The [System](../system/index.html) sizes the heap at initialization time. 
//! The heap may also be extended at run time. All operations sizing the
//! heap are inherently unsafe and must consider physical memory limits
//! as well as stack size limits.

use ::core::{mem, ptr, slice, str, fmt};

extern "C" {
    static mut _sheap: u32;
    static mut _eheap: u32;
    static mut _estack: u32;
}

struct HeapToken;
static mut HEAP_TOKEN: Option<HeapToken> = Some(HeapToken);
#[no_debug]
static mut HEAP_START: *mut u8 = unsafe { &_sheap as *const u32 as *mut u8 };
#[no_debug]
static mut HEAP_END: *mut u8 = unsafe { &_sheap as *const u32 as *mut u8 };

/// An error resulting from a heap operation.
#[derive(Debug)]
pub enum Error {
    OutOfSpace,
    InvalidAlign,
    Overflow,
}

// pub trait GetHeap {
//     fn heap(&mut self) -> &mut Heap { unsafe { &mut HEAP } }
// }

/// An alloc-only allocator.
pub struct Heap {
    _private: (),
}

impl Heap {
    /// Acquire the global heap singleton, busy waiting if it is not available.
    pub fn take() -> Heap {
        unsafe { while let None = HEAP_TOKEN {} }
        Heap { _private: () }        
    }

    /// Release the global heap singleton.
    pub fn release(_heap: Heap)  {
        unsafe {
            HEAP_TOKEN = Some(HeapToken)
        }
    }

    /// Extend the global heap by `size`. The caller is responsinble for ensuring that
    /// the limits are safe for the application and device.
    pub unsafe fn extend(size: usize)  {
        HEAP_END = HEAP_END.offset(size as isize);
    }

    /// Returns the heap pointer, which is the address of the next byte available for allocation.
    pub fn ptr(&self) -> *mut u8 {
        unsafe { HEAP_START }
    }

    // Sets the heap pointer, which is the address of the next byte available for allocation.
    fn set_ptr(&mut self, ptr: *mut u8) {
        unsafe { HEAP_START = ptr }
    }

    /// Returns the heap end pointer, which is the address of the byte after the last byte
    /// available for allocation.
    pub fn end(&self) -> *mut u8 {
        unsafe { HEAP_END }
    }

    /// Sets the heap end pointer, which is the address of the byte after the last byte available
    /// for allocation.
    fn set_end(&mut self, ptr: *mut u8) {
        unsafe { HEAP_END = ptr }
    }

    /// Returns the size of the heap in bytes.
    pub fn size(&self) -> usize {
        self.end() as usize - self.ptr() as usize
    }

    /// Sets the heap end pointer to the current heap pointer, preventing further allocations.
    pub fn freeze(&mut self) {
        let ptr = self.ptr();
        self.set_end(ptr)
    }

    /// Try advancing the heap pointer by `len` bytes, returning `Error::OutOfSpace` if
    /// the resulting heap pointer would be greater than the heap end pointer.
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

    /// Advance the heap pointer by `len` bytes, panicking if the resulting heap pointer
    /// would be greater than the heap end pointer.
    pub fn advance(&mut self, len: usize) -> *mut u8 {
        self.try_advance(len).expect("Out of space")
    }

    /// Try to align the heap pointer to alignment `align`, returning `Error::InvalidAlign` if
    /// the alignment is not an exact power of two, or `Error::OutOfSpace` if the resulting
    /// heap pointer would be greater than the heap end pointer.
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

    /// Align the heap pointer to alignment `align`, panicking if
    /// the alignment is not an exact power of two, or if the resulting
    /// heap pointer would be greater than the heap end pointer.
    pub fn align(&mut self, align: usize) {
        self.try_align(align).expect("Invalid alignment")
    }    

    /// Try to align the heap pointer to the alignment of type `T`, returning `Error::InvalidAlign` if
    /// the alignment is not an exact power of two, or `Error::OutOfSpace` if the resulting
    /// heap pointer would be greater than the heap end pointer.
    pub fn try_align_to<T>(&mut self) -> Result<(), Error> {
        self.try_align(mem::align_of::<T>())
    }    

    /// Align the heap pointer to the alignment of type `T`, panicking if
    /// the alignment is not an exact power of two, or if the resulting
    /// heap pointer would be greater than the heap end pointer    
    pub fn align_to<T>(&mut self) {
        self.align(mem::align_of::<T>())
    }

    /// Tries to allocate a pointer to memory suitable for storing `len` items of type `T`, 
    /// returning `Error::OutOfSpace` if
    /// the heap pointer after the allocation would be greater than the heap
    /// end pointer or `Error::Overflow` if `mem::size_of::<T>() * len` overflows.
    /// 
    /// The memory pointed to by the returned pointer is uninitialized.
    /// 
    /// NOTE: Assumes `mem::size_of::<T>()` includes the padding required to make the
    /// next T aligned.
    pub fn try_alloc<T>(&mut self, len: usize) -> Result<*mut T, Error> {
        Ok({
            self.try_align_to::<T>()?;
            if let Some(len) = mem::size_of::<T>().checked_mul(len) {
                self.try_advance(len)? as *mut T
            } else {
                return Err(Error::Overflow)
            }
        })
    }

    /// Allocates a pointer to memory suitable for storing `len` items of type `T`, 
    /// the heap pointer after the allocation would be greater than the heap
    /// end pointer or if `mem::size_of::<T>() * len` overflows.
    /// 
    /// The memory pointed to by the returned pointer is uninitialized.
    /// 
    /// NOTE: Assumes `mem::size_of::<T>()` includes the padding required to make the
    /// next T aligned.
    pub fn alloc<T>(&mut self, len: usize) -> *mut T {
        self.align_to::<T>();
        self.advance(mem::size_of::<T>() * len) as *mut T
    }

    /// Tries to allocate a pointer to memory suitable for storing an item of type `T`, 
    /// returning `Error::OutOfSpace` if the heap pointer after the allocation would be
    /// greater than the heap end pointer.
    /// 
    /// The memory pointed to by the returned pointer is uninitialized.
    pub fn try_alloc_one<T>(&mut self) -> Result<*mut T, Error> {
        Ok({
            self.try_align_to::<T>()?;
            self.try_advance(mem::size_of::<T>())? as *mut T
        })
    }

    /// Allocates a pointer to memory suitable for storing an item of type `T`, 
    /// panicking if the heap pointer after the allocation would be
    /// greater than the heap end pointer.
    /// 
    /// The memory pointed to by the returned pointer is uninitialized.
    pub fn alloc_one<T>(&mut self) -> *mut T {
        self.align_to::<T>();
        self.advance(mem::size_of::<T>()) as *mut T
    }

    /// Tries to allocate a slice of `len` items of type `T`,  returning `Error::OutOfSpace` 
    /// if the heap pointer after the allocation would be greater than the heap end pointer.
    pub unsafe fn try_slice_uninitialized<T>(&mut self, len: usize) -> Result<&'static mut [T], Error> {        
        Ok({
            slice::from_raw_parts_mut(self.try_alloc(len)?, len)
        })
    }

    /// Allocates a slice of `len` items of type `T`,  panicking if the heap pointer after 
    /// the allocation would be greater than the heap end pointer.
    pub unsafe fn slice_uninitialized<T>(&mut self, len: usize) -> &'static mut [T] {
        slice::from_raw_parts_mut(self.alloc(len), len)
    }

    /// Tries to allocate a slice of `len` items of type `T` initialized to `val`,
    /// returning `Error::OutOfSpace` if the heap pointer after the allocation would 
    /// be greater than the heap end pointer.
    pub fn try_slice<T: Copy>(&mut self, val: T, len: usize) -> Result<&'static mut [T], Error> {
        Ok({
            let dst = unsafe { self.try_slice_uninitialized(len)? };
            for v in dst.iter_mut() { 
                *v = val;
            }
            dst
        })
    }    

    /// Allocates a slice of `len` items of type `T` initialized to `val`,
    /// panicking if the heap pointer after the allocation would be greater 
    /// than the heap end pointer.
    pub fn slice<T: Copy>(&mut self, val: T, len: usize) -> &'static mut [T] {
        let dst = unsafe { self.slice_uninitialized(len) };
        for v in dst.iter_mut() { 
            *v = val;
        }
        dst
    }    

    /// Tries to allocate a new `T` containing `src`, returning `Error::OutOfSpace` 
    /// if the heap pointer after the allocation would be greater than the heap end pointer.
    pub fn try_new<T>(&mut self, src: T) -> Result<&'static mut T, Error> {
        Ok({
            unsafe {
                let dst = self.try_alloc_one::<T>()?;
                ptr::write(dst, src);
                &mut *dst
            }
        })
    }

    /// Allocates a new `T` containing `src`, panicking if the heap pointer after the allocation
    /// would be greater than the heap end pointer.
    pub fn new<T>(&mut self, src: T) -> &'static mut T {
        unsafe {
            let dst = self.alloc_one::<T>();
            ptr::write(dst, src);
            &mut *dst
        }
    }

    /// Tries to allocate a new `T` containing a copy of `src`, returning `Error::OutOfSpace` 
    /// if the heap pointer after the allocation would be greater than the heap end pointer.
    pub fn try_copy<T: Copy>(&mut self, src: &T) -> Result<&'static mut T, Error> {
        Ok({
            unsafe {
                let dst = self.alloc_one::<T>();
                *dst = *src;
                &mut *dst
            }
        })
    }

    /// Allocates a new `T` containing a copy of `src`, panicking if the heap pointer after the
    /// allocation would be greater than the heap end pointer.
    pub fn copy<T: Copy>(&mut self, src: &T) -> &'static mut T {
        unsafe {
            let dst = self.alloc_one::<T>();
            *dst = *src;
            &mut *dst
        }
    }

    /// Tries to allocate a new `T` containing a copy of the `src`, returning `Error::OutOfSpace` 
    /// if the heap pointer after the allocation would be greater than the heap end pointer.
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

    /// Allocates a new `T` containing a copy of `src`, panicking if the heap pointer after the
    /// allocation would be greater than the heap end pointer.
    pub fn copy_slice<T>(&mut self, src: &[T]) -> &'static mut [T] {
        unsafe {
            let len = src.len();
            let dst: *mut T = self.alloc(len);
            ptr::copy_nonoverlapping(src.as_ptr(), dst, len);
            slice::from_raw_parts_mut(dst, len)
        }
    }

    /// Tries to allocate a new `T` containing a copy of the `src`, returning `Error::OutOfSpace` 
    /// if the heap pointer after the allocation would be greater than the heap end pointer.
    pub fn try_copy_str(&mut self, src: &str) -> Result<&'static str, Error> {
        Ok({
            unsafe { str::from_utf8_unchecked(self.try_copy_slice(src.as_bytes())?) }
        })
    }

    /// Allocates a new `T` containing a copy of `src`, panicking if the heap pointer after the
    /// allocation would be greater than the heap end pointer.
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

unsafe fn align_offset(ptr: *const (), align: usize) -> usize {
    let offset = ptr as usize % align;
    if offset == 0 {
        0
    } else {
        align.wrapping_sub(offset)
    }
}