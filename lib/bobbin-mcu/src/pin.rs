use bits::*;
use signal::SignalType;
use periph::Periph;
use gate::GateEn;

pub trait PinSource<STY: SignalType, SRC> {
    fn alt_fn(&self) -> U4;
    #[inline]
    fn alt_fn_for(&self, _src: SRC) -> U4 { self.alt_fn() }
}

pub trait Pin<P: Periph> {
    fn port(&self) -> P { P::default() }
    fn index(&self) -> u8;    
}

pub trait PeriphPin<PIN> {
    fn periph_pin(&self) -> &PIN;
}

pub trait SetSource {
    fn set_source<V: Into<U4>>(&self, src: V);
}

pub trait ConnectTo<STY: SignalType, SRC, PIN> {
    fn connect_to(&self, src: SRC);
}

impl<STY, SRC, PIN, T> ConnectTo<STY, SRC, PIN> for T
where 
    STY: SignalType, 
    PIN: SetSource,
    Self: PeriphPin<PIN> + PinSource<STY, SRC> 
{
    fn connect_to(&self, src: SRC) {
        let alt_fn = self.alt_fn_for(src);
        self.periph_pin().set_source(alt_fn);
    }
}

pub trait PortGateEn<PORT> : Pin<PORT> where PORT: GateEn + Periph {
    #[inline]
    fn port_gate_enable(&self) -> &Self { self.port().gate_enable(); self }
}

impl<PORT, T> PortGateEn<PORT> for T
where 
    PORT: GateEn + Periph,
    Self: Pin<PORT>
{}