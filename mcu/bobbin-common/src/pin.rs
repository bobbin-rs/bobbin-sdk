use signal::SignalType;
use periph::Periph;

pub trait PinSource<STY: SignalType, P> {
    fn alt_fn(&self) -> u8;
}

pub trait Pin<P: Periph> {
    fn port(&self) -> P { P::default() }
    fn index(&self) -> u8;
}