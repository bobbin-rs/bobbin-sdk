use periph::Periph;
use signal::SignalType;

#[macro_export]
macro_rules! channel {
    ($id:ident, $ty:ident, $pty:ident, $num:expr) => {
        pub const $id: $ty = $ty {};
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        pub struct $ty {}
        impl $crate::channel::Channel<$pty> for $ty {
            fn index(&self) -> u8 { $num }
        }        
    }
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
