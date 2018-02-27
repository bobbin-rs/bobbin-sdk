#[macro_export]
macro_rules! periph {
    ($id:ident, $ty:ident, $pid:ident, $pty:ident, $base:expr, $ord:expr) => {
        pub const $id: $ty = $ty{};
        pub const $pid: $pty = $pty($base);
        
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        pub struct $ty {}

        impl ::core::ops::Deref for $ty {
            type Target = $pty;
            #[inline(always)]            
            fn deref(&self) -> &$pty {
                &$pid
            }
        }    

        impl Into<$pty> for $ty {
            #[inline(always)]
            fn into(self) -> $pty {
                $pid
            }
        }    

        impl $crate::periph::Periph for $ty {
            #[inline]
            fn id(&self) -> &'static str {
                stringify!($id)
            }
            #[inline]
            fn base(&self) -> *mut u32 {
                $base as *mut u32
            }
            #[inline]
            fn ord(&self) -> usize {
                $ord
            }            
        }
    };
    ($id:ident, $ty:ident, $base:expr) => (    
        pub const $id: $ty = $ty($base);
    )
}
#[macro_export]
macro_rules! xperiph {
    ($id:ident, $ty:ident, $pid:ident, $pty:ident, $base:expr, $ord:expr) => (
        pub const $id: $ty = $ty {};     
        pub const $pid: $pty = $pty($base);
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub struct $ty {}
        impl Deref for $ty {
            type Target = $pty;
            #[inline(always)]            
            fn deref(&self) -> &$pty {
                &$pid
            }
        }
        impl Into<$pty> for $ty {
            #[inline(always)]
            fn into(self) -> $pty {
                $pid
            }
        }    
        impl Periph for $ty {
            #[inline]
            fn id(&self) -> &'static str {
                stringify!($id)
            }
            #[inline]
            fn base(&self) -> *mut u32 {
                $base as *mut u32
            }
            #[inline]
            fn ord(&self) -> usize {
                $ord
            }
        }    
    );
    ($id:ident, $ty:ident, $base:expr) => (    
        pub const $id: $ty = $ty($base);
    )
}



#[macro_export]
macro_rules! periph_irq {
    ($pty:ident, $ity:ident, $irq:ident) => {
        impl $crate::irq::Irq<$ity, $irq> for $pty {
            fn irq(&self) -> $irq { $irq::default() }
        }
    }
}

#[macro_export]
macro_rules! periph_signal {
    ($ty:path, $sty:ident) => {
        signal!($ty, $sty);
    }
}

pub trait Periph: Default {
    fn id(&self) -> &'static str;
    fn base(&self) -> *mut u32;
    fn ord(&self) -> usize;
}

pub trait ClockEnabled : Periph {
    fn clock_enabled(&self) -> bool;
    fn set_clock_enabled(&self, value: bool);
    fn enable_clock(&self) { self.set_clock_enabled(true) }
    fn disable_clock(&self) { self.set_clock_enabled(true) }
}

pub trait Reset : Periph {
    fn clock_reset(&self) -> bool;
}