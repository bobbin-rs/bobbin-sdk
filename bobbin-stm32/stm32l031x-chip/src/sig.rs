pub trait Signal<T> {}

pub trait Smba {}
pub trait SignalSmba<T> {}
pub trait Scl {}
pub trait SignalScl<T> {}
pub trait Sda {}
pub trait SignalSda<T> {}
pub trait Ch1 {}
pub trait SignalCh1<T> {}
pub trait Ch2 {}
pub trait SignalCh2<T> {}
pub trait Ch3 {}
pub trait SignalCh3<T> {}
pub trait Ch4 {}
pub trait SignalCh4<T> {}
pub trait Etr {}
pub trait SignalEtr<T> {}
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
pub trait Nss {}
pub trait SignalNss<T> {}
pub trait Miso {}
pub trait SignalMiso<T> {}
pub trait Mosi {}
pub trait SignalMosi<T> {}
pub trait Sck {}
pub trait SignalSck<T> {}

pub const I2C1_SMBA: I2c1Smba = I2c1Smba {};
pub struct I2c1Smba {}
impl Smba for I2c1Smba {}

pub const I2C1_SCL: I2c1Scl = I2c1Scl {};
pub struct I2c1Scl {}
impl Scl for I2c1Scl {}

pub const I2C1_SDA: I2c1Sda = I2c1Sda {};
pub struct I2c1Sda {}
impl Sda for I2c1Sda {}

pub const TIM2_CH1: Tim2Ch1 = Tim2Ch1 {};
pub struct Tim2Ch1 {}
impl Ch1 for Tim2Ch1 {}

pub const TIM2_CH2: Tim2Ch2 = Tim2Ch2 {};
pub struct Tim2Ch2 {}
impl Ch2 for Tim2Ch2 {}

pub const TIM2_CH3: Tim2Ch3 = Tim2Ch3 {};
pub struct Tim2Ch3 {}
impl Ch3 for Tim2Ch3 {}

pub const TIM2_CH4: Tim2Ch4 = Tim2Ch4 {};
pub struct Tim2Ch4 {}
impl Ch4 for Tim2Ch4 {}

pub const TIM2_ETR: Tim2Etr = Tim2Etr {};
pub struct Tim2Etr {}
impl Etr for Tim2Etr {}

pub const TIM21_CH1: Tim21Ch1 = Tim21Ch1 {};
pub struct Tim21Ch1 {}
impl Ch1 for Tim21Ch1 {}

pub const TIM21_CH2: Tim21Ch2 = Tim21Ch2 {};
pub struct Tim21Ch2 {}
impl Ch2 for Tim21Ch2 {}

pub const TIM21_ETR: Tim21Etr = Tim21Etr {};
pub struct Tim21Etr {}
impl Etr for Tim21Etr {}

pub const TIM22_CH1: Tim22Ch1 = Tim22Ch1 {};
pub struct Tim22Ch1 {}
impl Ch1 for Tim22Ch1 {}

pub const TIM22_CH2: Tim22Ch2 = Tim22Ch2 {};
pub struct Tim22Ch2 {}
impl Ch2 for Tim22Ch2 {}

pub const TIM22_ETR: Tim22Etr = Tim22Etr {};
pub struct Tim22Etr {}
impl Etr for Tim22Etr {}

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

pub const LPUART1_TX: Lpuart1Tx = Lpuart1Tx {};
pub struct Lpuart1Tx {}
impl Tx for Lpuart1Tx {}

pub const LPUART1_RX: Lpuart1Rx = Lpuart1Rx {};
pub struct Lpuart1Rx {}
impl Rx for Lpuart1Rx {}

pub const LPUART1_CTS: Lpuart1Cts = Lpuart1Cts {};
pub struct Lpuart1Cts {}
impl Cts for Lpuart1Cts {}

pub const LPUART1_RTS: Lpuart1Rts = Lpuart1Rts {};
pub struct Lpuart1Rts {}
impl Rts for Lpuart1Rts {}

pub const SPI1_NSS: Spi1Nss = Spi1Nss {};
pub struct Spi1Nss {}
impl Nss for Spi1Nss {}

pub const SPI1_MISO: Spi1Miso = Spi1Miso {};
pub struct Spi1Miso {}
impl Miso for Spi1Miso {}

pub const SPI1_MOSI: Spi1Mosi = Spi1Mosi {};
pub struct Spi1Mosi {}
impl Mosi for Spi1Mosi {}

pub const SPI1_SCK: Spi1Sck = Spi1Sck {};
pub struct Spi1Sck {}
impl Sck for Spi1Sck {}

pub const LPTIM1_IN1: Lptim1In1 = Lptim1In1 {};
pub struct Lptim1In1 {}

pub const COMP1_OUT: Comp1Out = Comp1Out {};
pub struct Comp1Out {}

pub const EVENTOUT: Eventout = Eventout {};
pub struct Eventout {}

pub const LPTIM1_IN2: Lptim1In2 = Lptim1In2 {};
pub struct Lptim1In2 {}

pub const COMP2_OUT: Comp2Out = Comp2Out {};
pub struct Comp2Out {}

pub const UART2_CK: Uart2Ck = Uart2Ck {};
pub struct Uart2Ck {}

pub const SPI_MISO: SpiMiso = SpiMiso {};
pub struct SpiMiso {}

pub const LPTIM1_ETR: Lptim1Etr = Lptim1Etr {};
pub struct Lptim1Etr {}

pub const LPTIM1_OUT: Lptim1Out = Lptim1Out {};
pub struct Lptim1Out {}

pub const MCO: Mco = Mco {};
pub struct Mco {}

pub const SPI1_MIO: Spi1Mio = Spi1Mio {};
pub struct Spi1Mio {}

pub const SWDIO: Swdio = Swdio {};
pub struct Swdio {}

pub const SWCLK: Swclk = Swclk {};
pub struct Swclk {}

pub const SPI_SCK: SpiSck = SpiSck {};
pub struct SpiSck {}

pub const RTC_OUT: RtcOut = RtcOut {};
pub struct RtcOut {}

pub const RTC_REFIN: RtcRefin = RtcRefin {};
pub struct RtcRefin {}

