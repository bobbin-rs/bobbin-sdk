use {read_volatile, write_volatile};
use core::marker::PhantomData;

pub struct Register<T: Copy + Default> {
    base: *mut T,
    offset: isize
}

impl<T: Copy + Default> Register<T> {
    #[inline]
    pub const fn new(base: *mut T, offset: isize) -> Self {
        Self { base, offset }
    }

    #[inline]
    pub fn ptr(&self) -> *mut T { unsafe { self.base.offset(self.offset) } }

    #[inline]
    pub fn read(&self) -> T { unsafe { read_volatile(self.ptr()) } }

    #[inline]
    pub fn write(&self, value: T) { unsafe { write_volatile(self.ptr(), value) } }

    #[inline]
    pub fn with<F: FnOnce(T) -> T>(&self, f: F) {
        self.write(f(self.read()))
    }

    #[inline]
    pub fn set<F: FnOnce(T) -> T>(&self, f: F) {
        self.write(f(T::default()))
    }

    #[inline]
    pub fn clear(&self) {
        self.write(T::default())
    }
}

pub struct RegisterArray<T: Copy + Default, I: Copy + ::core::ops::Mul<isize, Output=isize>> {
    base: *mut T,
    offset: isize,
    incr: isize,
    _phantom: PhantomData<I>,
}
impl<T: Copy + Default, I: Copy + ::core::ops::Mul<isize, Output=isize>> RegisterArray<T, I>
{
    #[inline]
    pub const fn new(base: *mut T, offset: isize, incr: isize) -> Self {
        Self { base, offset, incr, _phantom: PhantomData }
    }

    #[inline]
    pub fn ptr(&self, index: I) -> *mut T { 
        unsafe { 
            (self.base as *mut u8).offset(self.offset + index * self.incr) as *mut T 
        } 
    }

    #[inline]
    pub fn read(&self, index: I) -> T { unsafe { read_volatile(self.ptr(index)) } }

    #[inline]
    pub fn write(&self, index: I, value: T) { unsafe { write_volatile(self.ptr(index), value) } }

    #[inline]
    pub fn with<F: FnOnce(T) -> T>(&self, index: I, f: F) {
        self.write(index, f(self.read(index)))
    }

    #[inline]
    pub fn set<F: FnOnce(T) -> T>(&self, index: I, f: F) {
        self.write(index, f(T::default()))
    }

    #[inline]
    pub fn clear(&self, index: I) {
        self.write(index, T::default())
    }
}

#[macro_export]
macro_rules! register {
    ($rid:ident, $rval:ty, $raddr:expr, $roff:expr) => {
        pub const $rid: Register<$rval> = Register::new($raddr as *mut $rval, $roff);
    };
    ($rid:ident, $rval:ty, $raddr:expr, $roff:expr, $rdim:ty, $incr:expr) => {
        pub const $rid: RegisterArray<$rval, $rdim> = RegisterArray::new($raddr as *mut $rval, $roff, $incr);
    };    
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    register!(GPIOA_MODER, u32, 0x1000_0000, 0x0);
    register!(REG_ARR, u32, 0x1000_0000, 0x0, isize, 0x10);


    struct Gpioa {}
    impl Gpioa {
        pub const fn reg_gpioa_moder(&self) -> Register<u32> { Register::new(0x1000_0000 as *mut u32, 0x10) }
        pub fn gpioa_moder(&self) -> u32 { self.read_gpioa_moder() }
        pub fn ptr_gpioa_moder(&self) -> *mut u32 { self.reg_gpioa_moder().ptr() }
        pub fn read_gpioa_moder(&self) -> u32 { self.reg_gpioa_moder().read() }
        pub fn write_gpioa_moder(&self, value: u32) -> &Self { self.reg_gpioa_moder().write(value); self }
        pub fn set_gpioa_moder<F: FnOnce(u32)->u32>(&self, f: F) -> &Self { self.reg_gpioa_moder().set(f); self }
        pub fn with_gpioa_moder<F: FnOnce(u32)->u32>(&self, f: F) -> &Self { self.reg_gpioa_moder().with(f); self }
    }


    #[test]
    pub fn test_ptr() {
        assert_eq!(GPIOA_MODER.ptr(), 0x1000_0000 as *mut u32);

        ::add_region(0x1000_0000, 0x1000);
        GPIOA_MODER.write(0x1234);
        assert_eq!(::read_u32(0x1000_0000 as *const u32), 0x1234);
    }

    #[test]
    pub fn test_arr() {
        assert_eq!(REG_ARR.ptr(1), 0x1000_0010 as *mut u32);        
    }
}