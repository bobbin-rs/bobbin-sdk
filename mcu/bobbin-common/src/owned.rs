use core::ops::{Deref, DerefMut};
use core::mem;

pub struct Owned<T: Acquire> {
    inner: T,
}

impl<T: Acquire> Owned<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T: Acquire> Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}


impl<T: Acquire> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Acquire> Drop for Owned<T> {
    fn drop(&mut self) {
        T::release()
    }
}

pub trait Acquire : Sized + Default {
    fn owned_mut() -> &'static mut bool;
    fn acquire() -> Option<Owned<Self>> {
        if !mem::replace(Self::owned_mut(), true) {
            Some(Owned::new(Self::default()))
        } else {
            None
        }
    }
    fn release() {
        mem::replace(Self::owned_mut(), false);
    }
    fn acquired() -> bool {
        *Self::owned_mut()
    }
}

pub trait RefCount {
    fn ref_count_mut() -> &'static mut u8;
    fn incr_ref() { *Self::ref_count_mut() += 1 }
    fn decr_ref() { *Self::ref_count_mut() -= 1 }
    fn ref_count() -> u8 { *Self::ref_count_mut() }
}