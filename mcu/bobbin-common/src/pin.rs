use signal::SignalType;
use periph::Periph;

pub trait PinSource<STY: SignalType, SRC> {
    fn alt_fn(&self) -> u8;
    #[inline]
    fn alt_fn_for(&self, _src: SRC) -> u8 { self.alt_fn() }
}

pub trait Pin<P: Periph> {
    fn port(&self) -> P { P::default() }
    fn index(&self) -> u8;
}
