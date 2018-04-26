use core::ptr;
use core::marker::PhantomData;

pub struct CounterGet<'a, T: Sync + Copy + 'a> {
    counter: *mut T,
    _phantom: PhantomData<&'a mut T>
}

impl<'a, T: Sync + Copy + 'a> CounterGet<'a, T> {
    pub fn get(&self) -> T {
        unsafe { ptr::read_volatile(self.counter) }
    }
}

pub struct CounterSet<'a, T: Sync + Copy + 'a> {
    counter: *mut T,
    _phantom: PhantomData<&'a mut T>
}

impl<'a, T: Sync + Copy + 'a> CounterSet<'a, T> {
    pub fn get(&self) -> T {
        unsafe { ptr::read_volatile(self.counter) }
    }
    pub fn set(&self, value: T) {
        unsafe { ptr::write_volatile(self.counter, value) }
    }
}

pub fn counter_pair<'a, T: Sync + Copy + 'a>(counter: &'a mut T) -> (CounterGet<'a, T>, CounterSet<'a, T>) {
    (
        CounterGet { counter: counter as *mut T, _phantom: PhantomData },
        CounterSet { counter: counter as *mut T, _phantom: PhantomData },
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_counter() {
        let mut c = 0u32;
        let (c_get, c_set) = counter_pair(&mut c);
        assert_eq!(c_get.get(), 0);
        assert_eq!(c_set.get(), 0);

        c_set.set(1);
        assert_eq!(c_get.get(), 1);
        assert_eq!(c_set.get(), 1);
    }
}