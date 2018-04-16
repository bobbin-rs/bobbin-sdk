#![no_std]

mod rw {
    pub use ::core::ptr::read_volatile as read;
    pub use ::core::ptr::write_volatile as write;
}

pub trait Register {
    type Value: Copy + Default;
    const ADDR: *mut Self::Value;

    fn ptr(&self) -> *mut Self::Value { Self::ADDR }

    fn read(&self) -> Self::Value { unsafe { rw::read(Self::ADDR) } }

    fn write(&self, value: Self::Value) { unsafe { rw::write(Self::ADDR, value) } }

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

#[macro_export]
macro_rules! register {
    ($rid:ident, $rty:ident, $rval:ty, $raddr:expr) => {
        pub const $rid: $rty = $rty {};
        pub struct $rty {}
        impl Register for $rty { 
            type Value = $rval;
            const ADDR: *mut $rval = $raddr as *mut $rval;
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    register!(GPIOA_MODER, GpioaModer, u32, 0x1000_0000);

    #[test]
    fn test_ptr() {
        assert_eq!(GPIOA_MODER.ptr(), 0x1000_0000 as *mut u32);
    }
}