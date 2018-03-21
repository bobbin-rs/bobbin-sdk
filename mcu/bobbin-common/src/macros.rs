
#[macro_export]
macro_rules! periph {
    ($id:ident, $ty:ident, $pid:ident, $pty:ident, $base:expr, $index: expr, $ord:expr) => {
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

        impl IntoPeriph for $ty {
            type Target = $pty;
            #[inline(always)]
            fn into_periph(&self) -> &'static Self::Target {
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
    };
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
#[macro_export]
macro_rules! pin {
    ($id:ident, $ty:ident, $port_id:ident, $port_type:ident, $base_id:ident, $base_type:ident, $base_port:ident, $index:expr) => {
        pub const $id: $ty = $ty {};
        pub const $base_id: $base_type = $base_type { port: $base_port, index: $index };
       
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

        impl PeriphPin<$base_type> for $ty {
            #[inline(always)]
            fn periph_pin(&self) -> &$base_type {
                &$base_id
            }
        }
    }
}

#[macro_export]
macro_rules! pin_source {
    ($pin_ty:ident, $src:path, $sty:path, $num:expr) => {
        impl $crate::pin::PinSource<$sty, $src> for $pin_ty {
            fn alt_fn(&self) -> u8 { $num}
        }
    };
}

#[macro_export]
macro_rules! channel {
    ($id:ident, $ty:ident, $periph_id:ident, $periph_type:ident, $base_id:ident, $base_type:ident, $base_periph:ident, $index:expr) => (    
        pub const $id: $ty = $ty {};
        pub const $base_id: $base_type = $base_type { periph: $base_periph, index: $index };
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
    )
}

#[macro_export]
macro_rules! channel_irq {
    ($cty:ident, $ity:ident, $irq:ident) => {
        impl $crate::irq::Irq<$ity, $irq> for $cty {
            fn irq(&self) -> $irq { $irq::default() }
        }
    }
}

#[macro_export]
macro_rules! channel_signal {
    ($ty:path, $sty:ident) => {
        signal!($ty, $sty);
    }
}

#[macro_export]
macro_rules! channel_source {
    ($cty:ident, $src:ident, $sty:ident, $num:expr) => {
        impl ChannelSource<$sty, $src> for $cty {
            fn selector(&self) -> u8 { $num }
        }
    };
}
#[macro_export]
macro_rules! signal_type {
    ($id:ident, $ty:ident) => {
        pub const $id: $ty = $ty {};
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]        
        pub struct $ty {}
        impl $crate::signal::SignalType for $ty {}        
    }
}

#[macro_export]
macro_rules! signal {
    ($ty:path, $sty:ident) => {
        impl $crate::signal::Signal<$sty> for $ty {}
    }
}

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

#[macro_export]
macro_rules! irq_type {
    ($id:ident, $ty:ident) => {
        pub const $id: $ty = $ty {};
        #[derive(Debug, Default, Clone, Copy)]
        pub struct $ty {}
        impl $crate::irq::IrqType for $ty {}
        
    };
}

#[macro_export]
macro_rules! irq {
    ($ty:path, $ity:ident, $inum:ident ) => {
        impl $crate::irq::Irq<$ity> for $ty { type Output = $inum; }        
    }
}

// #[macro_export]
// macro_rules! clocktree {
//     ($id:ident, $ty:ident) => {
//         pub const $id: LocalTree<$ty> = LocalTree($ty {});
//         #[derive(Debug, Clone, Copy, Default)]        
//         pub struct $ty {}
//         impl $crate::clock::ClockTree for LocalTree<$ty> {}
//     };
// }

// #[macro_export]
// macro_rules! clocktree_clock {
//     ($tr:ident, $meth:ident) => {
//         pub trait $tr: $crate::clock::ClockTree where Self::Out: $crate::clock::Clock {
//             type Out;
//             fn $meth(&self) -> Self::Out { Self::Out::default() }
//         } 
//     };
// }

// #[macro_export]
// macro_rules! clocktree_periph {
//     ($pty:path, $tr:ident) => {
//         impl<TR> $crate::clock::ClockFor<$pty> for LocalTree<TR>
//         where
//             Self: $tr
//         {
//             type Out = <Self as $tr>::Out;
//         }        
//     };
// }

// #[macro_export]
// macro_rules! clock_global {
//     ($tr:ident, $clock:ident, $expr:expr) => {
//         #[derive(Debug, Clone, Copy, Default)]
//         pub struct $clock {}

//         impl $crate::clock::Clock for LocalClock<$clock> {
//             fn hz() -> Hz { $expr }
//         }

//         impl<T: $crate::clock::ClockTree> $tr for T {
//             type Out = LocalClock<$clock>;
//         }               
//     };
// }

// #[macro_export]
// macro_rules! clock {
//     ($tree:ident, $tr:ident, $clock:ident, $expr:expr) => {
//         #[derive(Debug, Clone, Copy, Default)]
//         pub struct $clock {}

//         impl $crate::clock::Clock for LocalClock<$clock> {
//             fn hz() -> Hz { $expr }
//         }

//         impl $tr for LocalTree<$tree> {
//             type Out = LocalClock<$clock>;
//         }               
//     };
// }


/// Macro for sending `print!`-formatted messages over the Console
#[macro_export]
macro_rules! print {
    ($s:expr) => {
        $crate::console::write_str($s)
    };
    ($($arg:tt)*) => {
        $crate::console::write_fmt(format_args!($($arg)*))
    };
}

/// Macro for sending `print!`-formatted messages over the Console, with a
/// newline
#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\n"), $($arg)*)
    };
}

