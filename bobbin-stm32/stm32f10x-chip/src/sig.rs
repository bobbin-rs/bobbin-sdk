//! Signals

pub trait Signal<T> {}

pub trait Tim {}
pub trait SignalTim<T> {}
pub trait Tx {}
pub trait SignalTx<T> {}
pub trait Rx {}
pub trait SignalRx<T> {}
pub trait Cts {}
pub trait SignalCts<T> {}
pub trait Rts {}
pub trait SignalRts<T> {}
pub trait Ck {}
pub trait SignalCk<T> {}

pub const TIM2_CH1: Tim2Ch1 = Tim2Ch1 {};
pub struct Tim2Ch1 {}
impl Tim for Tim2Ch1 {}

pub const TIM2_CH2: Tim2Ch2 = Tim2Ch2 {};
pub struct Tim2Ch2 {}
impl Tim for Tim2Ch2 {}

pub const TIM2_CH3: Tim2Ch3 = Tim2Ch3 {};
pub struct Tim2Ch3 {}
impl Tim for Tim2Ch3 {}

pub const TIM2_CH4: Tim2Ch4 = Tim2Ch4 {};
pub struct Tim2Ch4 {}
impl Tim for Tim2Ch4 {}

pub const TIM3_CH1: Tim3Ch1 = Tim3Ch1 {};
pub struct Tim3Ch1 {}
impl Tim for Tim3Ch1 {}

pub const TIM3_CH2: Tim3Ch2 = Tim3Ch2 {};
pub struct Tim3Ch2 {}
impl Tim for Tim3Ch2 {}

pub const TIM3_CH3: Tim3Ch3 = Tim3Ch3 {};
pub struct Tim3Ch3 {}
impl Tim for Tim3Ch3 {}

pub const TIM3_CH4: Tim3Ch4 = Tim3Ch4 {};
pub struct Tim3Ch4 {}
impl Tim for Tim3Ch4 {}

pub const TIM4_CH1: Tim4Ch1 = Tim4Ch1 {};
pub struct Tim4Ch1 {}
impl Tim for Tim4Ch1 {}

pub const TIM4_CH2: Tim4Ch2 = Tim4Ch2 {};
pub struct Tim4Ch2 {}
impl Tim for Tim4Ch2 {}

pub const TIM4_CH3: Tim4Ch3 = Tim4Ch3 {};
pub struct Tim4Ch3 {}
impl Tim for Tim4Ch3 {}

pub const TIM4_CH4: Tim4Ch4 = Tim4Ch4 {};
pub struct Tim4Ch4 {}
impl Tim for Tim4Ch4 {}

pub const USART1_TX: Usart1Tx = Usart1Tx {};
pub struct Usart1Tx {}
impl Tx for Usart1Tx {}

pub const USART1_RX: Usart1Rx = Usart1Rx {};
pub struct Usart1Rx {}
impl Rx for Usart1Rx {}

pub const USART1_CTS: Usart1Cts = Usart1Cts {};
pub struct Usart1Cts {}
impl Cts for Usart1Cts {}

pub const USART1_RTS: Usart1Rts = Usart1Rts {};
pub struct Usart1Rts {}
impl Rts for Usart1Rts {}

pub const USART1_CK: Usart1Ck = Usart1Ck {};
pub struct Usart1Ck {}
impl Ck for Usart1Ck {}

pub const USART2_TX: Usart2Tx = Usart2Tx {};
pub struct Usart2Tx {}
impl Tx for Usart2Tx {}

pub const USART2_RX: Usart2Rx = Usart2Rx {};
pub struct Usart2Rx {}
impl Rx for Usart2Rx {}

pub const USART2_CTS: Usart2Cts = Usart2Cts {};
pub struct Usart2Cts {}
impl Cts for Usart2Cts {}

pub const USART2_RTS: Usart2Rts = Usart2Rts {};
pub struct Usart2Rts {}
impl Rts for Usart2Rts {}

pub const USART2_CK: Usart2Ck = Usart2Ck {};
pub struct Usart2Ck {}
impl Ck for Usart2Ck {}

pub const USART3_TX: Usart3Tx = Usart3Tx {};
pub struct Usart3Tx {}
impl Tx for Usart3Tx {}

pub const USART3_RX: Usart3Rx = Usart3Rx {};
pub struct Usart3Rx {}
impl Rx for Usart3Rx {}

pub const USART3_CTS: Usart3Cts = Usart3Cts {};
pub struct Usart3Cts {}
impl Cts for Usart3Cts {}

pub const USART3_RTS: Usart3Rts = Usart3Rts {};
pub struct Usart3Rts {}
impl Rts for Usart3Rts {}

pub const USART3_CK: Usart3Ck = Usart3Ck {};
pub struct Usart3Ck {}
impl Ck for Usart3Ck {}

