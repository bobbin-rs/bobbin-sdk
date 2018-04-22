//! A single-reader single-writer ring buffer.
//! 
//! This ring buffer supports concurrent reads and writes by a single reader
//! and writer. Only power-of-two sizes are supported.
//! 
//! The ring buffer is implemented as two virtual `usize` counters indexing into a shared
//! buffer.


use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::{mem, ptr, slice};

/// A single-reader single-writer ring buffer.
pub struct Ring<'a, T: 'a> {
    ptr: *mut T,
    cap: usize,
    head: UnsafeCell<usize>,
    tail: UnsafeCell<usize>,
    _phantom: PhantomData<&'a mut[T]>,
}

impl<'a, T: 'a> Ring<'a, T> {
    /// Returns a new ring buffer. The length of `buf` must be an exact power of two.
    pub fn new(buf: &'a mut [T]) -> Self {
        let ptr = buf.as_mut_ptr();
        let cap = buf.len();
        assert!(cap > 0 && cap.count_ones() == 1, "Only power-of-two sizes larger than zero are supported");
        mem::forget(buf);
        Self { ptr, cap, head: UnsafeCell::new(0), tail: UnsafeCell::new(0), _phantom: PhantomData }
    }

    /// Converts the ring into a slice.
    pub fn into_slice(self) -> &'a [T] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr, self.cap)
        }
    }

    /// Resets the ring counters.
    pub fn reset(&self) {
        unsafe {
            *self.head.get() = 0;
            *self.tail.get() = 0;
        }
    }

    #[inline]
    fn phy(&self, index: usize) -> usize {
        index & (self.cap.wrapping_sub(1))
    }

    fn phy_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.cap) }
    }

    #[inline]
    unsafe fn elt(&self, index: usize) -> &mut T {
        &mut *self.ptr.offset(self.phy(index) as isize)
    }

    /// Returns the number of elements the ring can hold.
    #[inline]
    pub fn cap(&self) -> usize {
        self.cap
    }

    /// Returns the number of elements currently contained in the ring.
    #[inline]
    pub fn len(&self) -> usize {
        self.head().wrapping_sub(self.tail())
    }

    /// Returns the remaining number of elements that can be added to the ring.
    #[inline]
    pub fn rem(&self) -> usize {
        self.cap() - self.len()
    }

    /// Returns the virtual head index. This index will wrap after `usize::MAX` elements
    /// have been added.
    #[inline]
    pub fn head(&self) -> usize {
        unsafe { *self.head.get() }
    }

    /// Returns the virtual tail index. This index will wrap after `usize::MAX` elements
    /// have been read.
    #[inline]
    pub fn tail(&self) -> usize {
        unsafe { *self.tail.get()}
    }

    /// Returns a mutable reference to the element that the virtual head is indexing if
    /// an element may be added to the ring, otherwise None.
    /// 
    /// NOTE: probably not usable if `incr_head` has been made private.
    pub fn head_elt(&self) -> Option<&mut T> {
        unsafe {
            if self.len() < self.cap() {
                Some(self.elt(self.head()))
            } else {
                None
            }
        }
    }

    /// Returns a reference to the element that the virtual tail is indexing if there
    /// is at least one element in the ring, otherwise None.
    /// 
    /// NOTE: should be renamed `peek()`?
    pub fn tail_elt(&self) -> Option<&mut T> {
        unsafe {
            if self.len() > 0 {
                Some(self.elt(self.tail()))
            } else {
                None
            }
        }
    }

    #[inline]
    fn incr_head(&self) -> usize {
        let head = self.head().wrapping_add(1);
        // assert!(self.len() <= self.cap());
        unsafe { ptr::replace(self.head.get(), head) }
    }

    #[inline]
    fn incr_tail(&self) -> usize {
        let tail = self.tail().wrapping_add(1);
        // assert!(self.tail() <= self.head());
        unsafe { ptr::replace(self.tail.get(), tail) }
    }

    /// Returns two contiguous slices accessing the
    /// underlying elements of the ring. One or both
    /// slices may be empty, and the second slice (if non-empty) will contain
    /// elements logically following those of the first slice. The length
    /// of the two slices should be the same as `len()`.
    pub fn slices(&self) -> (&[T], &[T]) {
        let head = self.head();
        let tail = self.tail();

        if head == tail {
            (&[], &[])
        } else {
            let phy_head = self.phy(self.head());
            let phy_tail = self.phy(self.tail());
            if phy_head > phy_tail {
                let a = &self.phy_slice()[phy_tail..phy_head];
                let b = &[];
                (a, b)
            } else {
                let a = &self.phy_slice()[phy_tail..];
                let b = &self.phy_slice()[..phy_head];
                (a, b)
            }
        }        
    }

    /// Returns a `Reader` for the ring.
    pub fn reader(&self) -> Reader<T> {
        Reader { inner: self }
    }

    /// Returns a `Writer` for the ring.
    pub fn writer(&self) -> Writer<T> {
        Writer { inner: self }
    }

}

impl<'a, T: 'a + Copy> Ring<'a, T> {
    /// Adds an element to the head of the ring, returning None if the
    /// ring is full.
    pub fn put(&self, value: T) -> Option<()> {
        if let Some(elt) = self.head_elt() {
            *elt = value;
            self.incr_head();
            Some(())
        } else {
            None
        }
    }

    /// Copies elements from `buf` to the head of the ring, returning
    /// the number of items added.
    pub fn write(&self, buf: &[T]) -> usize {
        let mut i: usize = 0;
        for b in buf.iter() {
            if let Some(_) = self.put(*b) {
                i = i.wrapping_add(1);
            }
        }
        i
    }

    /// Removes an element from the tail of the ring, returning None
    /// if the ring is full.
    pub fn get(&self) -> Option<T> {
        if let Some(elt) = self.tail_elt() {
            let value = *elt;
            self.incr_tail();
            Some(value)
        } else {
            None
        }
    }

    /// Reads elements from the tail of the ring into `buf`, returning
    /// the number of items read.
    pub fn read(&self, buf: &mut [T]) -> usize {
        let mut i: usize = 0;
        for b in buf.iter_mut() {
            if let Some(v) = self.get() {
                *b = v;
                i = i.wrapping_add(1);
            }
        }
        i
    }
}

/// A read handle for a ring buffer.
pub struct Reader<'a, T: 'a> {
    inner: &'a Ring<'a, T>
}

impl<'a, T: 'a> Reader<'a, T> {
    /// Returns the number of items in the ring.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns a reference to the item at the tail of the ring,
    /// returning None if the ring is empty.
    pub fn tail_elt(&self) -> Option<&mut T> {
        self.inner.tail_elt()
    }

    // pub fn incr_tail(&self) -> usize {
    //     self.inner.incr_tail()
    // }
}

impl<'a, T: 'a + Copy> Reader<'a, T> {
    /// Removes an element from the tail of the ring, returning
    /// None if the ring is empty.
    pub fn get(&self) -> Option<T> {    
        self.inner.get()
    }

    /// Reads elements from the tail of the ring into `buf`, returning
    /// the number of elements read.
    pub fn read(&self, buf: &mut [T]) -> usize {    
        self.inner.read(buf)
    }
}

unsafe impl<'a, T: 'a> Send for Reader<'a, T> {}
unsafe impl<'a, T: 'a> Sync for Reader<'a, T> {}

/// A write handle for a ring buffer.
pub struct Writer<'a, T: 'a> {
    inner: &'a Ring<'a, T>
}

impl<'a, T: 'a> Writer<'a, T> {
    /// Returns the number of items that can currently be added
    /// to the ring.
    pub fn rem(&self) -> usize {
        self.inner.rem()
    }

    /// Returns a mutable reference to the item at the head of the
    /// ring if `rem()` is greater than zero, otherwise None.
    pub fn head_elt(&self) -> Option<&mut T> {
        self.inner.head_elt()
    }

    // pub fn incr_head(&self) -> usize {
    //     self.inner.incr_head()
    // }    
}

impl<'a, T: 'a + Copy> Writer<'a, T> {
    /// Adds an element to the head of the ring, returning None if the
    /// ring is full.
    pub fn put(&self, value: T) -> Option<()> {
        self.inner.put(value)
    }    

    /// Copies elements from `buf` to the head of the ring, returning
    /// the number of items added.
    pub fn write(&self, buf: &[T]) -> usize {
        self.inner.write(buf)
    }
}

unsafe impl<'a, T: 'a> Send for Writer<'a, T> {}
unsafe impl<'a, T: 'a> Sync for Writer<'a, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring() {
        let mut buf = [0u8; 16];
        let ring = Ring::new(&mut buf);

        for i in 0..16 {
            if let Some(elt) = ring.head_elt() {
                *elt = i as u8;
                ring.incr_head();
            }            
        }

        for i in 0..16 {
            if let Some(elt) = ring.tail_elt() {
                assert_eq!(*elt, i as u8);
                ring.incr_tail();
            }
        }

        let buf = ring.into_slice();

        for i in 0..16 {
            assert_eq!(buf[i], i as u8);
        }
    }
    #[test]
    fn test_copy_ring() {
        let mut buf = [0u8; 16];
        let r = Ring::new(&mut buf);
        for i in 0..16 { 
            r.put(i as u8);
        }
        for i in 0..16 {
            assert_eq!(r.get(), Some(i as u8));
        }
    }

    #[test]
    fn test_read_write_ring() {
        let mut buf = [0u8; 16];
        let r = Ring::new(&mut buf);
        assert_eq!(r.write(&[0, 1, 2, 3]), 4);
        let mut dst = [0u8; 4];
        assert_eq!(r.read(&mut dst), 4);
        assert_eq!(dst, [0, 1, 2, 3])
    }    


    #[test]
    fn test_reader_writer_ring() {
        let mut buf = [0u8; 16];
        let ring = Ring::new(&mut buf);
        let r = ring.reader();
        let w = ring.writer();

        for _ in 0..16 {
            assert_eq!(w.write(&[0, 1, 2, 3]), 4);
            let mut dst = [0u8; 4];
            assert_eq!(r.read(&mut dst), 4);

            assert_eq!(dst, [0, 1, 2, 3])
        }
        assert_eq!(ring.head(), 16 * 4);
        assert_eq!(ring.tail(), 16 * 4);
    }        
}