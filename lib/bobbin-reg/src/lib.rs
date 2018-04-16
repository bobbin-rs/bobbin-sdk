#![no_std]

mod rw {
    pub use ::core::ptr::read_volatile as read;
    pub use ::core::ptr::write_volatile as write;
}

pub trait Register<T: Copy + Default> {
    const ADDR: *mut T;

    fn read() -> T { unsafe { rw::read(Self::ADDR) } }

    fn write(value: T) { unsafe { rw::write(Self::ADDR, value) } }

    fn with<F: FnOnce(T) -> T>(f: F) {
        Self::write(f(Self::read()))
    }

    fn set<F: FnOnce(T) -> T>(f: F) {
        Self::write(f(T::default()))
    }

    fn clear() {
        Self::write(T::default())
    }
}