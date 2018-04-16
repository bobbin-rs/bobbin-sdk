#![no_std]

mod rw {
    pub use ::core::ptr::read_volatile as read;
    pub use ::core::ptr::write_volatile as write;
}

pub trait Register<T: Copy + Default> {
    const ADDR: *mut T;

    fn ptr(&self) -> *mut T { Self::ADDR }

    fn read(&self) -> T { unsafe { rw::read(Self::ADDR) } }

    fn write(&self, value: T) { unsafe { rw::write(Self::ADDR, value) } }

    fn with<F: FnOnce(T) -> T>(&self, f: F) {
        self.write(f(self.read()))
    }

    fn set<F: FnOnce(T) -> T>(&self, f: F) {
        self.write(f(T::default()))
    }

    fn clear(&self) {
        self.write(T::default())
    }
}

#[macro_export]
macro_rules! register {
    ($rid:ident, $rty:ident, $rval:ty, $raddr:expr) => {
        pub const $rid: $rty = $rty {};
        pub struct $rty {}
        impl Register<$rval> for $rty { 
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