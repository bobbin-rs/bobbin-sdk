#![no_std]

mod rw {
    pub use ::core::ptr::read_volatile as read;
    pub use ::core::ptr::write_volatile as write;
}

pub trait Register {
    type Value: Copy + Default;
    const ADDR: *mut Self::Value;

    fn ptr(&self) -> *mut Self::Value { Self::ADDR }

    fn read(&self) -> Self::Value { unsafe { rw::read(self.ptr()) } }

    fn write(&self, value: Self::Value) { unsafe { rw::write(self.ptr(), value) } }

    fn with<F: FnOnce(Self::Value) -> Self::Value>(&self, f: F) {
        self.write(f(self.read()))
    }

    fn set<F: FnOnce(Self::Value) -> Self::Value>(&self, f: F) {
        self.write(f(Self::Value::default()))
    }

    fn clear(&self) {
        self.write(Self::Value::default())
    }
}

pub trait RegisterArray 
{
    type Value: Copy + Default;
    type Index: Copy + ::core::ops::Mul<isize, Output=isize>;
    const ADDR: *mut Self::Value;
    const INCR: isize;

    fn ptr(&self, index: Self::Index) -> *mut Self::Value { unsafe { (Self::ADDR as *mut u8).offset(index * Self::INCR) as *mut Self::Value } }

    fn read(&self, index: Self::Index) -> Self::Value { unsafe { rw::read(self.ptr(index)) } }

    fn write(&self, index: Self::Index, value: Self::Value) { unsafe { rw::write(self.ptr(index), value) } }

    fn with<F: FnOnce(Self::Value) -> Self::Value>(&self, index: Self::Index, f: F) {
        self.write(index, f(self.read(index)))
    }

    fn set<F: FnOnce(Self::Value) -> Self::Value>(&self, index: Self::Index, f: F) {
        self.write(index, f(Self::Value::default()))
    }

    fn clear(&self, index: Self::Index) {
        self.write(index, Self::Value::default())
    }
}

#[macro_export]
macro_rules! register {
    ($rid:ident, $rty:ident, $rval:ty, $raddr:expr) => {
        pub const $rid: $rty = $rty {};
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $rty {}
        impl Register for $rty { 
            type Value = $rval;
            const ADDR: *mut $rval = $raddr as *mut $rval;
        }
    };
}

#[macro_export]
macro_rules! register_array {
    ($rid:ident, $rty:ident, $rval:ty, $raddr:expr, $rdim:ty, $incr:expr) => {
        pub const $rid: $rty = $rty {};
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $rty {}
        impl RegisterArray for $rty { 
            type Value = $rval;
            type Index = $rdim;            
            const ADDR: *mut $rval = $raddr as *mut $rval;
            const INCR: isize = $incr;
        }
    };
}



#[cfg(test)]
mod tests {
    use super::*;

    register!(GPIOA_MODER, GpioaModer, u32, 0x1000_0000);
    register_array!(REG_ARR, RegArr, u32, 0x1000_0000, isize, 0x10);

    struct Periph {}
    impl Periph {
        pub fn gpioa_moder_register(&self) -> GpioaModer { GPIOA_MODER }
        pub fn gpioa_moder(&self) -> u32 { GPIOA_MODER.read() }
        pub fn gpioa_moder_ptr(&self) -> *mut u32 { GPIOA_MODER.ptr() }
        pub fn read_gpioa_moder(&self) -> u32 { GPIOA_MODER.read() }
        pub fn write_gpioa_moder(&self, value: u32) -> &Self { GPIOA_MODER.write(value); self }
        pub fn set_gpioa_moder<F: FnOnce(u32)->u32>(&self, f: F) -> &Self { GPIOA_MODER.set(f); self }
        pub fn with_gpioa_moder<F: FnOnce(u32)->u32>(&self, f: F) -> &Self { GPIOA_MODER.with(f); self }
    }


    #[test]
    fn test_ptr() {
        assert_eq!(GPIOA_MODER.ptr(), 0x1000_0000 as *mut u32);
    }
    #[test]
    fn test_arr() {
        assert_eq!(REG_ARR.ptr(1), 0x1000_0010 as *mut u32);
    }
}