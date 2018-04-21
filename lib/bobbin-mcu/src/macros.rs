/// Defines a peripheral instance.
#[macro_export]
macro_rules! periph {
    ($id:ident, $ty:ident, $pid:ident, $pty:ident, $owned:ident, $ref_count:ident, $base:expr, $index: expr, $ord:expr) => {
        pub const $id: $ty = $ty{};
        pub const $pid: $pty = $pty($base);
        static mut $owned: bool = false;
        static mut $ref_count: u8 = 0;
        
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

        impl $crate::periph::AsPeriph for $ty {
            type Target = $pty;
            #[inline(always)]
            fn as_periph(&self) -> &'static Self::Target {
                &$pid
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
            fn index(&self) -> usize {
                $index
            }                 
            #[inline]
            fn ord(&self) -> usize {
                $ord
            }        
        }

        impl $crate::owned::Acquire for $ty {
            fn owned_mut() -> &'static mut bool {
                unsafe { &mut $owned }
            }
        }
        impl $crate::owned::RefCount for $ty {
            fn ref_count_mut() -> &'static mut u8 {
                unsafe { &mut $ref_count }
            }
        }        
        
    };
    ($id:ident, $ty:ident, $base:expr) => (    
        pub const $id: $ty = $ty($base);
    )
}

/// Associates an interrupt number with a peripheral and interrupt type.
#[macro_export]
macro_rules! periph_irq {
    ($pty:ident, $ity:ident, $irq:ident) => {
        impl $crate::irq::Irq<$ity, $irq> for $pty {
            fn irq(&self) -> $irq { $irq::default() }
        }
    }
}

/// Associates a signal with a peripheral.
#[macro_export]
macro_rules! periph_signal {
    ($ty:path, $sty:ident) => {
        $crate::signal!($ty, $sty);
    }
}

/// Defines a Pin instance.
#[macro_export]
macro_rules! pin {
    ($id:ident, $ty:ident, $meth:ident, $port_id:ident, $port_type:ident, $base_id:ident, $base_type:ident, $base_port:ident, $owned:ident, $ref_count: ident, $index:expr) => {
        pub const $id: $ty = $ty {};
        pub const $base_id: $base_type = $base_type { port: $base_port, index: $index };
        static mut $owned: bool = false;
        static mut $ref_count: u8 = 0;
       
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        pub struct $ty {}

        impl $crate::pin::Pin<$port_type> for $ty {
            #[inline(always)]
            fn port(&self) -> $port_type { $port_id }
            #[inline(always)]            
            fn index(&self) -> u8 { $index }
        }        

        impl ::core::ops::Deref for $ty {
            type Target = $base_type;
            #[inline(always)]            
            fn deref(&self) -> &$base_type {
                &$base_id
            }
        }
        impl Into<$base_type> for $ty {
            #[inline(always)]
            fn into(self) -> $base_type {
                $base_id
            }
        }        

        impl $crate::pin::PeriphPin<$base_type> for $ty {
            #[inline(always)]
            fn periph_pin(&self) -> &$base_type {
                &$base_id
            }
        }

        impl $port_type {
            pub fn $meth(&self) -> Option<$crate::owned::Owned<$ty>> { 
                use $crate::owned::Acquire;
                $ty::acquire() 
            }
        }

        impl $crate::owned::Acquire for $ty {
            fn owned_mut() -> &'static mut bool {
                unsafe { &mut $owned }
            }
            fn acquire() -> Option<$crate::owned::Owned<Self>> {
                use $crate::owned::RefCount;
                if !::core::mem::replace(Self::owned_mut(), true) {
                    $port_type::incr_ref();
                    Some($crate::owned::Owned::new(Self::default()))
                } else {
                    None
                }
            }
            fn release() {
                use $crate::owned::RefCount;
                ::core::mem::replace(Self::owned_mut(), false);
                $port_type::decr_ref();
            }            
        }

        impl $crate::owned::RefCount for $ty {
            fn ref_count_mut() -> &'static mut u8 {
                unsafe { &mut $ref_count }
            }
        }        
    }
}

/// Associates a signal and signal type with a pin, including an Alternate Function selector for that source.
#[macro_export]
macro_rules! pin_source {
    ($pin_ty:ident, $src:path, $sty:path, $num:expr) => {
        impl $crate::pin::PinSource<$sty, $src> for $pin_ty {
            fn alt_fn(&self) -> ::bobbin_bits::U4 { $num }
        }
    };
}

/// Defines a Channel.
#[macro_export]
macro_rules! channel {
    ($id:ident, $ty:ident, $meth:ident, $periph_id:ident, $periph_type:ident, $base_id:ident, $base_type:ident, $base_periph:ident, $owned:ident, $ref_count: ident, $index:expr) => (    
        pub const $id: $ty = $ty {};
        pub const $base_id: $base_type = $base_type { periph: $base_periph, index: $index };
        static mut $owned: bool = false;
        static mut $ref_count: u8 = 0;

        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        pub struct $ty {}
        impl $crate::channel::Channel<$periph_type> for $ty {
            #[inline(always)]
            fn periph(&self) -> $periph_type { $periph_id }
            #[inline(always)]            
            fn index(&self) -> u8 { $index }
        }

        impl ::core::ops::Deref for $ty {
            type Target = $base_type;
            #[inline(always)]            
            fn deref(&self) -> &$base_type {
                &$base_id
            }
        }

        impl Into<$base_type> for $ty {
            #[inline(always)]
            fn into(self) -> $base_type {
                $base_id
            }
        }

        impl $periph_type {
            pub fn $meth(&self) -> Option<$crate::owned::Owned<$ty>> { 
                use $crate::owned::Acquire;
                $ty::acquire() 
            }
        }        

        impl $crate::owned::Acquire for $ty {
            fn owned_mut() -> &'static mut bool {
                unsafe { &mut $owned }
            }
            fn acquire() -> Option<$crate::owned::Owned<Self>> {
                use $crate::owned::RefCount;
                if !::core::mem::replace(Self::owned_mut(), true) {
                    $periph_type::incr_ref();
                    Some($crate::owned::Owned::new(Self::default()))
                } else {
                    None
                }
            }
            fn release() {
                use $crate::owned::RefCount;
                ::core::mem::replace(Self::owned_mut(), false);
                $periph_type::decr_ref();
            }                
        }

        impl $crate::owned::RefCount for $ty {
            fn ref_count_mut() -> &'static mut u8 {
                unsafe { &mut $ref_count }
            }
        }            
    )
}

/// Associates an interrupt number with a Channel and Interrupt Type.
#[macro_export]
macro_rules! channel_irq {
    ($cty:ident, $ity:ident, $irq:ident) => {
        impl $crate::irq::Irq<$ity, $irq> for $cty {
            fn irq(&self) -> $irq { $irq::default() }
        }
    }
}

/// Associates a signal and signal type with a Channel.
#[macro_export]
macro_rules! channel_signal {
    ($ty:path, $sty:ident) => {
        $crate::signal!($ty, $sty);
    }
}

/// Associates a source and source type with a Channel, including a selector number.
#[macro_export]
macro_rules! channel_source {
    ($cty:ident, $src:ident, $sty:ident, $num:expr) => {
        impl ChannelSource<$sty, $src> for $cty {
            fn selector(&self) -> u8 { $num }
        }
    };
}

/// Defines a signal type.
#[macro_export]
macro_rules! signal_type {
    ($id:ident, $ty:ident) => {
        pub const $id: $ty = $ty {};
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]        
        pub struct $ty {}
        impl $crate::signal::SignalType for $ty {}        
    }
}

/// Defines a signal.
#[macro_export]
macro_rules! signal {
    ($ty:path, $sty:ident) => {
        impl $crate::signal::Signal<$sty> for $ty {}
    }
}

/// Defines an interrupt number.
#[macro_export]
macro_rules! irq_number {
    ($id:ident, $ty:ident, $num:expr) => {
        pub const $id: $ty = $ty {};
        #[derive(Debug, Default, Clone, Copy)]
        pub struct $ty {}
        impl $crate::irq::IrqNumber for $ty {
            fn irq_number() -> u8 { $num }
        }
    }    
}

/// Defines an interrupt type.
#[macro_export]
macro_rules! irq_type {
    ($id:ident, $ty:ident) => {
        pub const $id: $ty = $ty {};
        #[derive(Debug, Default, Clone, Copy)]
        pub struct $ty {}
        impl $crate::irq::IrqType for $ty {}
        
    };
}

/// Associates an interrupt number with a peripheral or channel and interrupt type.
#[macro_export]
macro_rules! irq {
    ($ty:path, $ity:ident, $inum:ident ) => {
        impl $crate::irq::Irq<$ity> for $ty { type Output = $inum; }        
    }
}

