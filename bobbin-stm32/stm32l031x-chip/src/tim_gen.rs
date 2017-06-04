pub use stm32_common::chip::tim_gen::*;

pub const TIM2: Tim2 = Periph(0x40000000, Tim2Id {});
pub const TIM21: Tim21 = Periph(0x40010800, Tim21Id {});
pub const TIM22: Tim22 = Periph(0x40011400, Tim22Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim2Id {}
pub type Tim2 = Periph<Tim2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim21Id {}
pub type Tim21 = Periph<Tim21Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim22Id {}
pub type Tim22 = Periph<Tim22Id>;

impl super::sig::Signal<super::sig::Tim2Ch1> for Tim2 {}
impl super::sig::SignalCh1<super::sig::Tim2Ch1> for Tim2 {}
impl super::sig::Signal<super::sig::Tim2Ch2> for Tim2 {}
impl super::sig::SignalCh2<super::sig::Tim2Ch2> for Tim2 {}
impl super::sig::Signal<super::sig::Tim2Ch3> for Tim2 {}
impl super::sig::SignalCh3<super::sig::Tim2Ch3> for Tim2 {}
impl super::sig::Signal<super::sig::Tim2Ch4> for Tim2 {}
impl super::sig::SignalCh4<super::sig::Tim2Ch4> for Tim2 {}
impl super::sig::Signal<super::sig::Tim2Etr> for Tim2 {}
impl super::sig::SignalEtr<super::sig::Tim2Etr> for Tim2 {}

impl super::sig::Signal<super::sig::Tim21Ch1> for Tim21 {}
impl super::sig::SignalCh1<super::sig::Tim21Ch1> for Tim21 {}
impl super::sig::Signal<super::sig::Tim21Ch2> for Tim21 {}
impl super::sig::SignalCh2<super::sig::Tim21Ch2> for Tim21 {}
impl super::sig::Signal<super::sig::Tim21Etr> for Tim21 {}
impl super::sig::SignalEtr<super::sig::Tim21Etr> for Tim21 {}

impl super::sig::Signal<super::sig::Tim22Ch1> for Tim22 {}
impl super::sig::SignalCh1<super::sig::Tim22Ch1> for Tim22 {}
impl super::sig::Signal<super::sig::Tim22Ch2> for Tim22 {}
impl super::sig::SignalCh2<super::sig::Tim22Ch2> for Tim22 {}
impl super::sig::Signal<super::sig::Tim22Etr> for Tim22 {}
impl super::sig::SignalEtr<super::sig::Tim22Etr> for Tim22 {}


