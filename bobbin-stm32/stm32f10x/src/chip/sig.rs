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

pub struct Tim2Ch1 {}
pub struct Tim2Ch2 {}
pub struct Tim2Ch3 {}
pub struct Tim2Ch4 {}
pub struct Tim3Ch1 {}
pub struct Tim3Ch2 {}
pub struct Tim3Ch3 {}
pub struct Tim3Ch4 {}
pub struct Tim4Ch1 {}
pub struct Tim4Ch2 {}
pub struct Tim4Ch3 {}
pub struct Tim4Ch4 {}
pub struct Usart1Tx {}
pub struct Usart1Rx {}
pub struct Usart1Cts {}
pub struct Usart1Rts {}
pub struct Usart1Ck {}
pub struct Usart2Tx {}
pub struct Usart2Rx {}
pub struct Usart2Cts {}
pub struct Usart2Rts {}
pub struct Usart2Ck {}
pub struct Usart3Tx {}
pub struct Usart3Rx {}
pub struct Usart3Cts {}
pub struct Usart3Rts {}
pub struct Usart3Ck {}
