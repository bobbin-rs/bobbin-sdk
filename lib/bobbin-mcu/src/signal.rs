pub trait SignalType {}

pub trait Signal<ST: SignalType> {}

pub trait Selector<ST: SignalType> {
    fn selector(&self) -> u8;
}
