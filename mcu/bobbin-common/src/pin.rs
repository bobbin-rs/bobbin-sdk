use signal::SignalType;
use periph::Periph;

#[macro_export]
macro_rules! pin {
    ($id:ident, $ty:ident, $port_id:ident, $port_type:ident, $base_id:ident, $base_type:ident, $base_port:ident, $index:expr) => (
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
    )
}


#[macro_export]
macro_rules! xpin {
    ($id:ident, $ty:ident, $port_id:ident, $port_type:ident, $base_id:ident, $base_type:ident, $base_port:ident, $index:expr) => (
        pub const $id: $ty = $ty {};     
        pub const $base_id: $base_type = $base_type { port: $base_port, index: $index };
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub struct $ty {}
        impl Pin<$port_type> for $ty {
            #[inline(always)]
            fn port(&self) -> $port_type { $port_id }
            #[inline(always)]            
            fn index(&self) -> usize { $index }
        }
        impl Deref for $ty {
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
macro_rules! alt_fn {
    ($ty:ty, $sig:ty, $num:expr) => (
        // impl AltFn<$sig> for $ty {
        //     #[inline(always)]            
        //     fn alt_fn(&self) -> usize { $num }
        // }        
    )

}

#[macro_export]
macro_rules! pin_source {
    ($pin_ty:ident, $src:ident, $sty:ident, $num:expr) => {
        impl $crate::pin::PinSource<$sty, $src> for $pin_ty {
            fn alt_fn(&self) -> u8 { $num}
        }
    };
}

pub trait PinSource<STY: SignalType, P> {
    fn alt_fn(&self) -> u8;
}

pub trait Pin<P: Periph> {
    fn port(&self) -> P { P::default() }
    fn index(&self) -> u8;
}


pub trait ConnectPin<SRC, STY, PERIPH, PIN>
where 
    STY: SignalType,
    PERIPH: Periph,
    PIN: Pin<PERIPH> + PinSource<STY, SRC>,
{
    fn connect_pin(&self, signal_type: STY, pin: PIN);
}