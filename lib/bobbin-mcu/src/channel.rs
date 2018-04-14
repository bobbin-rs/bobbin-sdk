use periph::Periph;
use signal::SignalType;

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
