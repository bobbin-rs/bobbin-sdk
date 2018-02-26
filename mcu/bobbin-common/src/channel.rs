use periph::Periph;
use signal::SignalType;

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
macro_rules! xchannel {
    ($id:ident, $ty:ident, $periph_id:ident, $periph_type:ident, $base_id:ident, $base_type:ident, $base_periph:ident, $index:expr) => (
        pub const $id: $ty = $ty {};     
        pub const $base_id: $base_type = $base_type { periph: $base_periph, index: $index };
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub struct $ty {}
        impl Channel<$periph_type> for $ty {
            #[inline(always)]
            fn periph(&self) -> $periph_type { $periph_id }
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
macro_rules! channel_irq {
    ($cty:ident, $ity:ident, $irq:ident) => {
        impl $crate::irq::Irq<$ity, $irq> for $cty {
            fn irq(&self) -> $irq { $irq::default() }
        }
    }
}

#[macro_export]
macro_rules! channel_signal {
    ($ty:ident, $sty:ident) => {
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

pub trait Channel<P: Periph> {
    fn periph(&self) -> P { P::default() }
    fn index(&self) -> u8;
}

pub trait ChannelSource<ST: SignalType, SRC> {
    fn selector(&self) -> u8;
}

pub trait ConnectChannel<SRC, STY, PERIPH, CH>
where 
    STY: SignalType,
    PERIPH: Periph,
    CH: Channel<PERIPH> + ChannelSource<STY, SRC>,
{
    fn connect_channel(&self, signal_type: STY, channel: CH);
}
