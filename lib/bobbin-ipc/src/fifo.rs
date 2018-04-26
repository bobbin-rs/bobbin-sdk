use core::{mem, ptr, cmp, slice};
use core::marker::PhantomData;

struct FifoHeader {
    head: u32,
    tail: u32,
    size: u32,
    cap: u32,
}

impl FifoHeader {
    fn new(size: u32) -> Self {
        let cap = 1 << (31 - size.leading_zeros());
        FifoHeader { head: 0u32.wrapping_sub(64), tail: 0u32.wrapping_sub(64), size, cap }
    }

    fn cap(&self) -> u32 {
        self.cap
    }

    fn len(&self) -> u32 {
        self.head.wrapping_sub(self.tail)
    }

    fn rem(&self) -> u32 {
        self.cap().wrapping_sub(self.len())
    }

    fn phy(&self, index: u32) -> u32 {
        index & (self.cap.wrapping_sub(1))
    }

    fn head(&self) -> u32 {
        self.head
    }

    fn head_incr(&mut self, value: u32) -> u32 {
        let value = self.head.wrapping_add(value);
        mem::replace(&mut self.head, value)
    }

    fn tail(&self) -> u32 {
        self.tail
    }

    fn tail_incr(&mut self, value: u32) -> u32 {
        let value = self.tail.wrapping_add(value);
        mem::replace(&mut self.tail, value)
    }
}

pub struct FifoSender<'a, T: Send + 'a> {
    ptr: *mut T,
    _phantom: PhantomData<&'a mut T>,
}

impl<'a, T: Send + 'a> FifoSender<'a, T> {
    fn hdr_ptr(&self) -> *mut FifoHeader {
        unsafe { (self.ptr as *mut u8).offset(-(mem::size_of::<FifoHeader>() as isize)) as *mut FifoHeader }
    }
    fn hdr(&self) -> &FifoHeader {
        unsafe { &*self.hdr_ptr() }
    }
    fn hdr_mut(&self) -> &mut FifoHeader {
        unsafe { &mut *self.hdr_ptr() }
    }    

    fn phy_slice_mut(&self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.hdr().cap() as usize) }
    }

    pub fn cap(&self) -> u32 {
        self.hdr().cap()
    }

    pub fn len(&self) -> u32 {
        self.hdr().len()
    }

    pub fn rem(&self) -> u32 {
        self.hdr().rem()
    }
    
    pub fn send(&self, value: T) -> Option<T> {
        if self.hdr().rem() > 0 {
            unsafe { 
                ptr::write_volatile(self.ptr.offset(self.hdr().phy(self.hdr_mut().head_incr(1)) as isize), value); 
            }
            None
        } else {
            Some(value)
        }
    }

    /// Returns two contiguous slices accessing the
    /// unassigned elements of the ring. One or both
    /// slices may be empty, and the second slice (if non-empty) will contain
    /// elements logically following those of the first slice. The length
    /// of the two slices should be the same as `cap()`.
    pub fn slices_mut(&self) -> (&mut [T], &mut [T]) {
        if self.cap() == 0 {
            (&mut [], &mut [])
        } else {
            let phy_head = self.hdr().phy(self.hdr().head()) as usize;
            let phy_tail = self.hdr().phy(self.hdr().tail()) as usize;
            if phy_head < phy_tail {
                let a = &mut self.phy_slice_mut()[phy_head..phy_tail];
                let b = &mut [];                
                (a, b)
            } else {
                let a = &mut self.phy_slice_mut()[phy_head..];
                let b = &mut self.phy_slice_mut()[..phy_tail];                
                (a, b)
            }
        }        
    }    
}

impl<'a, T: Send + Copy + 'a> FifoSender<'a, T> {
    pub fn write(&self, buf: &[T]) -> usize {
        let mut n = 0;
        // let (a, b) = self.slices_mut();
        // if a.len() > 0 {
        //     let len = cmp::min(buf.len(), a.len());
        //     &mut a[..len].copy_from_slice(&buf[..len]);
        //     n += len;
        // }
        // if n < buf.len() && b.len() > 0 {
        //     let len = cmp::min(buf.len() - n, b.len());
        //     &mut b[..len].copy_from_slice(&buf[..len]);
        //     n += len;
        // }
        for b in buf.iter() {
            if let Some(_) = self.send(*b) {
                return n
            }
            n += 1;
        }
        n
    }
}

pub struct FifoReceiver<'a, T: Send + 'a> {
    ptr: *mut T,
    _phantom: PhantomData<&'a mut T>,
}

impl<'a, T: Send + 'a> FifoReceiver<'a, T> {
    fn hdr_ptr(&self) -> *mut FifoHeader {
        unsafe { (self.ptr as *mut u8).offset(-(mem::size_of::<FifoHeader>() as isize)) as *mut FifoHeader }
    }
    fn hdr(&self) -> &FifoHeader {
        unsafe { &*self.hdr_ptr() }
    }
    fn hdr_mut(&self) -> &mut FifoHeader {
        unsafe { &mut *self.hdr_ptr() }
    }

    fn phy_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.hdr().cap() as usize) }
    }

    pub fn cap(&self) -> u32 {
        self.hdr().cap()
    }

    pub fn len(&self) -> u32 {
        self.hdr().len()
    }

    pub fn rem(&self) -> u32 {
        self.hdr().rem()
    }

    pub fn consume(&self, value: u32) {
        self.hdr_mut().tail_incr(cmp::min(value, self.len()));
    }

    pub fn recv(&self) -> Option<T> {
        if self.hdr().len() > 0 {
            unsafe { 
                Some(ptr::read_volatile(self.ptr.offset(self.hdr().phy(self.hdr_mut().tail_incr(1)) as isize)))
            }
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.hdr().len() > 0 {
            unsafe { 
                Some(&*self.ptr.offset(self.hdr().phy(self.hdr_mut().tail()) as isize))
            }
        } else {
            None
        }        
    }

    pub fn peek_mut(&self) -> Option<&mut T> {
        if self.hdr().len() > 0 {
            unsafe { 
                Some(&mut *self.ptr.offset(self.hdr().phy(self.hdr_mut().tail()) as isize))
            }
        } else {
            None
        }        
    }    

    /// Returns two contiguous slices accessing the
    /// underlying elements of the ring. One or both
    /// slices may be empty, and the second slice (if non-empty) will contain
    /// elements logically following those of the first slice. The length
    /// of the two slices should be the same as `len()`.
    pub fn slices(&self) -> (&[T], &[T]) {
        if self.len() == 0 {
            (&[], &[])
        } else {
            let phy_head = self.hdr().phy(self.hdr().head()) as usize;
            let phy_tail = self.hdr().phy(self.hdr().tail()) as usize;
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
}

impl<'a, T: Send + Copy + 'a> FifoReceiver<'a, T> {
    pub fn read(&self, buf: &mut [T]) -> usize {
        let mut n = 0;
        for b in buf.iter_mut() {
            if let Some(v) = self.recv() {
                *b = v;
            } else {
                return n
            }
            n += 1;
        }
        n
    }
}


pub fn fifo_pair<'a, T: Send + 'a>(data: &mut [T]) -> (FifoSender<'a, T>, FifoReceiver<'a, T>) {
    (
        FifoSender { ptr: data.as_mut_ptr(), _phantom: PhantomData },
        FifoReceiver { ptr: data.as_mut_ptr(), _phantom: PhantomData },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fifo() {
        let mut fifo = (FifoHeader::new(8), [0u8; 8]);
        let head_ptr = &mut fifo.0 as *mut FifoHeader;
        let data_ptr = fifo.1.as_mut_ptr();
        let (fifo_send, fifo_recv) = fifo_pair(&mut fifo.1);
        assert_eq!(fifo_send.hdr_ptr(), head_ptr);
        assert_eq!(fifo_send.ptr, data_ptr);

        for i in 0..8 {
            assert_eq!(fifo_send.send(i as u8), None)
        }
        assert_eq!(fifo_send.send(0), Some(0));
        assert_eq!(fifo_send.rem(), 0);
        assert_eq!(fifo_recv.len(), 8);

        for i in 0..8 {
            assert_eq!(fifo_recv.recv(), Some(i as u8));
        }

        assert_eq!(fifo_send.rem(), 8);
        assert_eq!(fifo_recv.len(), 0);
        assert_eq!(fifo_recv.recv(), None);        

        for i in 0..1024 {
            assert_eq!(fifo_send.send(i as u8), None);
            assert_eq!(fifo_recv.recv(), Some(i as u8));
        }

        assert_eq!(fifo_send.send(0xab), None);
        if let Some(value) = fifo_recv.peek_mut() {
            *value += 1;
        }
        if let Some(value) = fifo_recv.peek() {
            assert_eq!(*value, 0xac);
        }
        assert_eq!(fifo_recv.recv(), Some(0xac));

        let mut buf = [0u8; 64];
        for i in 0..buf.len() {
            buf[i] = i as u8;
        }

        // let (a, b) = fifo_send.slices_mut();
        // assert_eq!(a.len() + b.len(), 8);
        // assert_eq!(fifo_send.write(&buf[..]), 8);
        // assert_eq!(fifo_send.len(), 8);
        // let mut dst = [0u8; 64];
        // assert_eq!(fifo_recv.read(&mut dst), 8);
        // for i in 0..8 {
        //     assert_eq!(dst[i], i as u8);
        // }
    }
    #[test]
    fn test_fifo_slice() {
        let mut fifo = (FifoHeader::new(8), [0u8; 8]);
        let head_ptr = &mut fifo.0 as *mut FifoHeader;
        let data_ptr = fifo.1.as_mut_ptr();
        let (fifo_send, fifo_recv) = fifo_pair(&mut fifo.1);    

        assert_eq!(fifo_send.hdr().phy(fifo_send.hdr().head()), 0);
        assert_eq!(fifo_send.hdr().phy(fifo_send.hdr().tail()), 0);

        let (a, b) = fifo_send.slices_mut();
        assert_eq!(a.len(), 8);
        assert_eq!(b.len(), 0);
        fifo_send.send(0);
        fifo_recv.recv();

        assert_eq!(fifo_send.hdr().phy(fifo_send.hdr().head()), 1);
        assert_eq!(fifo_send.hdr().phy(fifo_send.hdr().tail()), 1);
        let (a, b) = fifo_send.slices_mut();
        assert_eq!(a.len(), 7);
        assert_eq!(b.len(), 1);

        fifo_send.send(0);
        fifo_recv.recv();
        assert_eq!(fifo_send.hdr().phy(fifo_send.hdr().head()), 2);
        assert_eq!(fifo_send.hdr().phy(fifo_send.hdr().tail()), 2);
        let (a, b) = fifo_send.slices_mut();
        assert_eq!(a.len(), 6);
        assert_eq!(b.len(), 2);

        for i in 0..6 {
            fifo_send.send(0);
            fifo_recv.recv();
        }
        assert_eq!(fifo_send.hdr().phy(fifo_send.hdr().head()), 0);
        assert_eq!(fifo_send.hdr().phy(fifo_send.hdr().tail()), 0);
        let (a, b) = fifo_send.slices_mut();
        assert_eq!(a.len() + b.len(), 8);


    }
}