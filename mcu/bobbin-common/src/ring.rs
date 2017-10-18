use core::cell::UnsafeCell;
use core::cmp;
use core::ptr;
use core::marker::PhantomData;
use core::ops::Index;



pub struct Ring<'a, T: 'a + Copy> {
    reader: UnsafeCell<usize>,
    writer: UnsafeCell<usize>,
    buffer: *mut [T],
    _phantom: PhantomData<&'a mut[T]>,
}

impl<'a, T: 'a + Copy> Ring<'a, T> {
    pub fn new(buf: &mut[T]) -> Self {
        Ring {
            reader: UnsafeCell::new(0),
            writer: UnsafeCell::new(0),
            buffer: buf,
            _phantom: PhantomData,
        }
    }

    pub fn pair(&self) -> (RingReader<T>, RingWriter<T>) {
        (
            RingReader { rb: self},
            RingWriter { rb: self}
        )
    }

    fn as_ref(&self) -> &[T] {
        unsafe { &*self.buffer }
    }

    fn as_mut(&self) -> &mut [T]{
        unsafe { &mut *self.buffer }
    }    

    fn cap(&self) -> usize {
        self.as_ref().len()
    }

    pub fn writer(&self) -> usize {
        unsafe { ptr::read_volatile(self.writer.get()) }
    }

    pub fn set_writer(&self, value: usize) {
        unsafe { ptr::write_volatile(self.writer.get(), value) }
    }

    pub fn reader(&self) -> usize {
        unsafe { ptr::read_volatile(self.reader.get()) }
    }

    pub fn set_reader(&self, value: usize) {
        unsafe { ptr::write_volatile(self.reader.get(), value) }
    }
    pub fn len(&self) -> usize {
        self.writer().wrapping_sub(self.reader())
    }

    pub fn rem(&self) -> usize {
        self.cap().wrapping_sub(self.len())
    }

    pub fn is_empty(&self) -> bool {
        self.reader() == self.writer()
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.cap()
    }

    fn phy(&self, index: usize) -> usize {
        index % self.cap()
    }

    fn incr_reader(&self) {
        assert!(!self.is_empty(), "Attempted to increment empty reader");
        self.set_reader(self.reader().wrapping_add(1));
    }

    fn incr_writer(&self) {        
        assert!(!self.is_full(), "Attempted to increment full writer");
        self.set_writer(self.writer().wrapping_add(1));     
    }

    pub fn enqueue(&self, value: T) -> bool {
        if self.is_full() {
            false
        } else {
            let writer = self.phy(self.writer());
            self.as_mut()[writer] = value;
            self.incr_writer();
            true
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let reader = self.phy(self.reader());
            let value = self.as_ref()[reader];
            self.incr_reader();
            Some(value)
        }
    }

    pub fn write(&self, buf: &[T]) -> usize {
        let n = cmp::min(self.rem(), buf.len());
        for i in 0..n {
            self.enqueue(buf[i]);
        }
        n
    }

    pub fn read(&self, buf: &mut [T]) -> usize {
        let n = cmp::min(self.len(), buf.len());
        for i in 0..n {
            buf[i] = self.dequeue().expect("Ring buffer is empty");
        }
        n
    }    
}

unsafe impl<'a, T: 'a + Copy> Send for Ring<'a, T> {}
unsafe impl<'a, T: 'a + Copy> Sync for Ring<'a, T> {}

impl<'a, T: 'a + Copy> Index<usize> for Ring<'a, T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= self.len() {
            panic!("Index is {} but len is {}", index, self.len());
        }
        self.as_ref().index(self.phy(index.wrapping_add(self.reader())))
    }
}

pub struct RingReader<'a, T: 'a + Copy> {
    rb: &'a Ring<'a, T>,
}

impl<'a, T: Copy> RingReader<'a, T> {
    pub fn dequeue(&self) -> Option<T> {
        self.rb.dequeue()
    }
    pub fn read(&self, buf: &mut [T]) -> usize {
        self.rb.read(buf)
    }
}

unsafe impl<'a, T: 'a + Copy> Send for RingReader <'a, T> {}
unsafe impl<'a, T: 'a + Copy> Sync for RingReader <'a, T> {}

pub struct RingWriter<'a, T: 'a + Copy> {
    rb: &'a Ring<'a, T>,
}

impl<'a, T: Copy> RingWriter<'a, T> {
    pub fn enqueue(&self, value: T) -> bool {
        self.rb.enqueue(value)
    }
    pub fn write(&self, buf: &[T]) -> usize {    
        self.rb.write(buf)
    }
}

unsafe impl<'a, T: 'a + Copy> Send for RingWriter<'a, T> {}
unsafe impl<'a, T: 'a + Copy> Sync for RingWriter<'a, T> {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static() {
        let (r, w) = static_ring_buf!(16, u8, 0);
        w.write(b"Hello, World");
        let mut dst = [0u8; 64];
        let n = r.read(&mut dst);
        assert_eq!(&dst[..n], b"Hello, World");
    }


    #[test]
    fn test_enqueue_dequeue() {
        let (r, w) = static_ring_buf!(16, u8, 0);
        
        for i in 0..16 {
            assert_eq!(w.enqueue(i as u8), true);
        }
        assert_eq!(w.enqueue(0), false);
        for i in 0..16 {
            assert_eq!(r.dequeue(), Some(i as u8));
        }
        assert_eq!(r.dequeue(), None);
    }

     #[test]
    fn test_enqueue_dequeue_u32() {
        let (r, w) = static_ring_buf!(16, u32, 0);
        
        for i in 0..16 {
            assert_eq!(w.enqueue(i as u32), true);
        }
        assert_eq!(w.enqueue(0), false);
        for i in 0..16 {
            assert_eq!(r.dequeue(), Some(i as u32));
        }
        assert_eq!(r.dequeue(), None);
    }   

    #[test]
    fn test_write_read() {
        let (r, w) = static_ring_buf!(16, u8, 0);

        let src: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut dst = [0u8; 16];

        w.write(&src);
        let n = r.read(&mut dst);
        assert_eq!(n, 16);
        for i in 0..16 {
            assert_eq!(src[i], dst[i]);
        }
    }

    pub struct Driver<'a> {
        w: RingWriter<'a, u8>
    }

    impl<'a> Driver<'a> {
        pub fn run(&self) {
            self.w.enqueue(123);
        }
    }

    #[test]
    fn test_driver() {
        let (r, w) = static_ring_buf!(16, u8, 0);
        let drv = Driver { w: w };
        drv.run();
        assert_eq!(r.dequeue(), Some(123));
    }

    #[test]
    fn test_static_driver() {
        let (r, w) = static_ring_buf!(16, u8, 0);        
        static mut DRV: Option<Driver> = None;
        unsafe {
            DRV = Some(Driver { w: w});
            DRV.as_ref().unwrap().run();
        }
        assert_eq!(r.dequeue(), Some(123));
    }    

}