pub trait SignalType {}

pub trait Signal<ST: SignalType> {}

pub trait Selector<ST: SignalType> {
    fn selector(&self) -> u8;
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
    ($ty:ident, $sty:ident) => {
        impl Signal<$sty> for $ty {}
    }
}