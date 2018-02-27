use signal::SignalType;
use periph::Periph;

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