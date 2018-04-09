use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::{mem, ptr, slice};

pub struct Ring<'a, T: 'a> {
    ptr: *mut T,
    cap: usize,
    head: UnsafeCell<usize>,
    tail: UnsafeCell<usize>,
    _phantom: PhantomData<&'a mut[T]>,
}

impl<'a, T: 'a> Ring<'a, T> {
    pub fn new(buf: &'a mut [T]) -> Self {
        let ptr = buf.as_mut_ptr();
        let cap = buf.len();
        assert!(cap > 0 && cap.count_ones() == 1, "Only power-of-two sizes larger than zero are supported");
        mem::forget(buf);
        Self { ptr, cap, head: UnsafeCell::new(0), tail: UnsafeCell::new(0), _phantom: PhantomData }
    }

    pub fn into_slice(self) -> &'a [T] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr, self.cap)
        }
    }

    pub fn reset(&self) {
        unsafe {
            *self.head.get() = 0;
            *self.tail.get() = 0;
        }
    }

    #[inline]
    fn phy(&self, index: usize) -> usize {
        index & (self.cap - 1)
    }

    fn phy_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.cap) }
    }

    #[inline]
    unsafe fn elt(&self, index: usize) -> &mut T {
        &mut *self.ptr.offset(self.phy(index) as isize)
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.cap
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.head().wrapping_sub(self.tail())
    }

    #[inline]
    pub fn head(&self) -> usize {
        unsafe { *self.head.get() }
    }

    #[inline]
    pub fn tail(&self) -> usize {
        unsafe { *self.tail.get()}
    }


    pub fn head_elt(&self) -> Option<&mut T> {
        unsafe {
            if self.len() < self.cap() {
                Some(self.elt(self.head()))
            } else {
                None
            }
        }
    }

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
    pub fn incr_head(&self) -> usize {
        let head = self.head().wrapping_add(1);
        assert!(self.len() <= self.cap());
        unsafe { ptr::replace(self.head.get(), head) }
    }

    #[inline]
    pub fn incr_tail(&self) -> usize {
        let tail = self.tail().wrapping_add(1);
        assert!(self.tail() <= self.head());
        unsafe { ptr::replace(self.tail.get(), tail) }
    }

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

    pub fn reader(&self) -> Reader<T> {
        Reader { inner: self }
    }

    pub fn writer(&self) -> Writer<T> {
        Writer { inner: self }
    }

}

impl<'a, T: 'a + Copy> Ring<'a, T> {
    pub fn put(&self, value: T) -> Option<()> {
        if let Some(elt) = self.head_elt() {
            *elt = value;
            self.incr_head();
            Some(())
        } else {
            None
        }
    }

    pub fn write(&self, buf: &[T]) -> usize {
        let mut i = 0;
        while i < buf.len() {
            if let Some(_) = self.put(buf[i]) {
                i += 1;
            } else {
                break
            }
        }
        i
    }

    pub fn get(&self) -> Option<T> {
        if let Some(elt) = self.tail_elt() {
            let value = *elt;
            self.incr_tail();
            Some(value)
        } else {
            None
        }
    }

    pub fn read(&self, buf: &mut [T]) -> usize {
        let mut i = 0;
        while i < buf.len() {
            if let Some(v) = self.get() {
                buf[i] = v;
                i += 1;
            } else {
                break
            }
        }
        i
    }
}

pub struct Reader<'a, T: 'a> {
    inner: &'a Ring<'a, T>
}

impl<'a, T: 'a> Reader<'a, T> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn tail_elt(&self) -> Option<&mut T> {
        self.inner.tail_elt()
    }

    pub fn incr_tail(&self) -> usize {
        self.inner.incr_tail()
    }
}

impl<'a, T: 'a + Copy> Reader<'a, T> {
    pub fn get(&self) -> Option<T> {    
        self.inner.get()
    }

    pub fn read(&self, buf: &mut [T]) -> usize {    
        self.inner.read(buf)
    }
}

unsafe impl<'a, T: 'a> Send for Reader<'a, T> {}
unsafe impl<'a, T: 'a> Sync for Reader<'a, T> {}

pub struct Writer<'a, T: 'a> {
    inner: &'a Ring<'a, T>
}

impl<'a, T: 'a> Writer<'a, T> {
    pub fn head_elt(&self) -> Option<&mut T> {
        self.inner.head_elt()
    }

    pub fn incr_head(&self) -> usize {
        self.inner.incr_head()
    }    
}

impl<'a, T: 'a + Copy> Writer<'a, T> {
    pub fn put(&self, value: T) -> Option<()> {
        self.inner.put(value)
    }    

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