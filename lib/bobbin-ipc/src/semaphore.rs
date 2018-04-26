use core::ptr;
use core::cmp;
use core::marker::PhantomData;

pub struct SemaphoreIncr<'t, T: 't> {
    head: *mut T,
    tail: *mut T,
    _phantom: PhantomData<(&'t mut T, &'t mut T)>
}

impl<'t> SemaphoreIncr<'t, u32> {
    fn head(&self) -> u32 {
        unsafe { ptr::read_volatile(self.head) }
    }
    fn set_head(&self, value: u32) {
        unsafe { ptr::write_volatile(self.head, value) }
    }
    fn tail(&self) -> u32 {
        unsafe { ptr::read_volatile(self.tail) }
    }
    pub fn incr(&self, value: u32) {
        self.set_head(self.head().wrapping_add(value))
    }
    pub fn get(&self) -> u32 {
        self.head().wrapping_sub(self.tail())
    }
}


pub struct SemaphoreDecr<'t, T: 't> {
    head: *mut T,
    tail: *mut T,
    _phantom: PhantomData<(&'t mut T, &'t mut T)>
}

impl<'t> SemaphoreDecr<'t, u32> {
    fn head(&self) -> u32 {
        unsafe { ptr::read_volatile(self.head) }
    }
    fn tail(&self) -> u32 {
        unsafe { ptr::read_volatile(self.tail) }
    }
    fn set_tail(&self, value: u32) {
        unsafe { ptr::write_volatile(self.tail, value) }
    }    
    pub fn decr(&self, value: u32) {
        self.set_tail(self.tail().wrapping_add(cmp::min(value, self.get())))
    }
    pub fn get(&self) -> u32 {
        self.head().wrapping_sub(self.tail())
    }
}

pub fn semaphore_pair<'t, T>(head: &'t mut T, tail: &'t mut T) -> (SemaphoreIncr<'t, T>, SemaphoreDecr<'t, T>) {
    (
        SemaphoreIncr { head: head, tail: tail, _phantom: PhantomData },
        SemaphoreDecr { head: head, tail: tail, _phantom: PhantomData },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_semaphore() {
        let (mut a, mut b) = (0u32, 0u32);
        let (incr, decr) = semaphore_pair(&mut a, &mut b);
        assert_eq!(incr.get(), 0);
        assert_eq!(decr.get(), 0);
        incr.incr(1);
        assert_eq!(incr.get(), 1);
        assert_eq!(decr.get(), 1);
        decr.decr(1);
        assert_eq!(incr.get(), 0);
        assert_eq!(decr.get(), 0);
        decr.decr(1);
        assert_eq!(incr.get(), 0);
        assert_eq!(decr.get(), 0);
    }
}