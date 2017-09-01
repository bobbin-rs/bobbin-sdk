#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::port::*;

pub trait LinkGpio<T> {
    fn gpio(&self) -> T;
}

pub trait LinkGpioPin<T> {
    fn gpio_pin(&self) -> T;
}

periph!( PORTA, Porta, _PORTA, PortPeriph, 0x40049000);
periph!( PORTB, Portb, _PORTB, PortPeriph, 0x4004a000);
periph!( PORTC, Portc, _PORTC, PortPeriph, 0x4004b000);
periph!( PORTD, Portd, _PORTD, PortPeriph, 0x4004c000);
periph!( PORTE, Porte, _PORTE, PortPeriph, 0x4004d000);

impl LinkGpio<super::gpio::Gpioa > for Porta {
    #[inline] fn gpio(&self) -> super::gpio::Gpioa  { super::gpio::GPIOA }
}

impl LinkGpioPin<super::gpio::Pa0 > for Pta0 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa0  { super::gpio::PA0 }
}

impl LinkGpioPin<super::gpio::Pa1 > for Pta1 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa1  { super::gpio::PA1 }
}

impl LinkGpioPin<super::gpio::Pa2 > for Pta2 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa2  { super::gpio::PA2 }
}

impl LinkGpioPin<super::gpio::Pa3 > for Pta3 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa3  { super::gpio::PA3 }
}

impl LinkGpioPin<super::gpio::Pa4 > for Pta4 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa4  { super::gpio::PA4 }
}

impl LinkGpioPin<super::gpio::Pa5 > for Pta5 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa5  { super::gpio::PA5 }
}

impl LinkGpioPin<super::gpio::Pa6 > for Pta6 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa6  { super::gpio::PA6 }
}

impl LinkGpioPin<super::gpio::Pa7 > for Pta7 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa7  { super::gpio::PA7 }
}

impl LinkGpioPin<super::gpio::Pa8 > for Pta8 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa8  { super::gpio::PA8 }
}

impl LinkGpioPin<super::gpio::Pa9 > for Pta9 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa9  { super::gpio::PA9 }
}

impl LinkGpioPin<super::gpio::Pa10 > for Pta10 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa10  { super::gpio::PA10 }
}

impl LinkGpioPin<super::gpio::Pa11 > for Pta11 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa11  { super::gpio::PA11 }
}

impl LinkGpioPin<super::gpio::Pa12 > for Pta12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa12  { super::gpio::PA12 }
}

impl LinkGpioPin<super::gpio::Pa13 > for Pta13 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa13  { super::gpio::PA13 }
}

impl LinkGpioPin<super::gpio::Pa14 > for Pta14 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa14  { super::gpio::PA14 }
}

impl LinkGpioPin<super::gpio::Pa15 > for Pta15 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa15  { super::gpio::PA15 }
}

impl LinkGpioPin<super::gpio::Pa16 > for Pta16 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa16  { super::gpio::PA16 }
}

impl LinkGpioPin<super::gpio::Pa17 > for Pta17 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa17  { super::gpio::PA17 }
}

impl LinkGpioPin<super::gpio::Pa18 > for Pta18 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa18  { super::gpio::PA18 }
}

impl LinkGpioPin<super::gpio::Pa19 > for Pta19 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa19  { super::gpio::PA19 }
}

impl LinkGpioPin<super::gpio::Pa24 > for Pta24 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa24  { super::gpio::PA24 }
}

impl LinkGpioPin<super::gpio::Pa25 > for Pta25 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa25  { super::gpio::PA25 }
}

impl LinkGpioPin<super::gpio::Pa26 > for Pta26 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa26  { super::gpio::PA26 }
}

impl LinkGpioPin<super::gpio::Pa27 > for Pta27 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa27  { super::gpio::PA27 }
}

impl LinkGpioPin<super::gpio::Pa28 > for Pta28 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa28  { super::gpio::PA28 }
}

impl LinkGpioPin<super::gpio::Pa29 > for Pta29 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa29  { super::gpio::PA29 }
}


impl LinkGpio<super::gpio::Gpiob > for Portb {
    #[inline] fn gpio(&self) -> super::gpio::Gpiob  { super::gpio::GPIOB }
}

impl LinkGpioPin<super::gpio::Pb0 > for Ptb0 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb0  { super::gpio::PB0 }
}

impl LinkGpioPin<super::gpio::Pb1 > for Ptb1 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb1  { super::gpio::PB1 }
}

impl LinkGpioPin<super::gpio::Pb2 > for Ptb2 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb2  { super::gpio::PB2 }
}

impl LinkGpioPin<super::gpio::Pb3 > for Ptb3 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb3  { super::gpio::PB3 }
}

impl LinkGpioPin<super::gpio::Pb4 > for Ptb4 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb4  { super::gpio::PB4 }
}

impl LinkGpioPin<super::gpio::Pb5 > for Ptb5 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb5  { super::gpio::PB5 }
}

impl LinkGpioPin<super::gpio::Pb6 > for Ptb6 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb6  { super::gpio::PB6 }
}

impl LinkGpioPin<super::gpio::Pb7 > for Ptb7 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb7  { super::gpio::PB7 }
}

impl LinkGpioPin<super::gpio::Pb8 > for Ptb8 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb8  { super::gpio::PB8 }
}

impl LinkGpioPin<super::gpio::Pb9 > for Ptb9 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb9  { super::gpio::PB9 }
}

impl LinkGpioPin<super::gpio::Pb10 > for Ptb10 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb10  { super::gpio::PB10 }
}

impl LinkGpioPin<super::gpio::Pb11 > for Ptb11 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb11  { super::gpio::PB11 }
}

impl LinkGpioPin<super::gpio::Pb12 > for Ptb12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb12  { super::gpio::PB12 }
}

impl LinkGpioPin<super::gpio::Pb13 > for Ptb13 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb13  { super::gpio::PB13 }
}

impl LinkGpioPin<super::gpio::Pb16 > for Ptb16 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb16  { super::gpio::PB16 }
}

impl LinkGpioPin<super::gpio::Pb17 > for Ptb17 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb17  { super::gpio::PB17 }
}

impl LinkGpioPin<super::gpio::Pb18 > for Ptb18 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb18  { super::gpio::PB18 }
}

impl LinkGpioPin<super::gpio::Pb19 > for Ptb19 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb19  { super::gpio::PB19 }
}

impl LinkGpioPin<super::gpio::Pb20 > for Ptb20 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb20  { super::gpio::PB20 }
}

impl LinkGpioPin<super::gpio::Pb21 > for Ptb21 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb21  { super::gpio::PB21 }
}

impl LinkGpioPin<super::gpio::Pb22 > for Ptb22 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb22  { super::gpio::PB22 }
}

impl LinkGpioPin<super::gpio::Pb23 > for Ptb23 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pb23  { super::gpio::PB23 }
}


impl LinkGpio<super::gpio::Gpioc > for Portc {
    #[inline] fn gpio(&self) -> super::gpio::Gpioc  { super::gpio::GPIOC }
}

impl LinkGpioPin<super::gpio::Pc0 > for Ptc0 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc0  { super::gpio::PC0 }
}

impl LinkGpioPin<super::gpio::Pc1 > for Ptc1 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc1  { super::gpio::PC1 }
}

impl LinkGpioPin<super::gpio::Pc2 > for Ptc2 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc2  { super::gpio::PC2 }
}

impl LinkGpioPin<super::gpio::Pc3 > for Ptc3 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc3  { super::gpio::PC3 }
}

impl LinkGpioPin<super::gpio::Pc4 > for Ptc4 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc4  { super::gpio::PC4 }
}

impl LinkGpioPin<super::gpio::Pc5 > for Ptc5 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc5  { super::gpio::PC5 }
}

impl LinkGpioPin<super::gpio::Pc6 > for Ptc6 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc6  { super::gpio::PC6 }
}

impl LinkGpioPin<super::gpio::Pc7 > for Ptc7 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc7  { super::gpio::PC7 }
}

impl LinkGpioPin<super::gpio::Pc8 > for Ptc8 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc8  { super::gpio::PC8 }
}

impl LinkGpioPin<super::gpio::Pc9 > for Ptc9 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc9  { super::gpio::PC9 }
}

impl LinkGpioPin<super::gpio::Pc10 > for Ptc10 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc10  { super::gpio::PC10 }
}

impl LinkGpioPin<super::gpio::Pc11 > for Ptc11 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc11  { super::gpio::PC11 }
}

impl LinkGpioPin<super::gpio::Pc12 > for Ptc12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc12  { super::gpio::PC12 }
}

impl LinkGpioPin<super::gpio::Pc13 > for Ptc13 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc13  { super::gpio::PC13 }
}

impl LinkGpioPin<super::gpio::Pc14 > for Ptc14 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc14  { super::gpio::PC14 }
}

impl LinkGpioPin<super::gpio::Pc15 > for Ptc15 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc15  { super::gpio::PC15 }
}

impl LinkGpioPin<super::gpio::Pc16 > for Ptc16 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc16  { super::gpio::PC16 }
}

impl LinkGpioPin<super::gpio::Pc17 > for Ptc17 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc17  { super::gpio::PC17 }
}

impl LinkGpioPin<super::gpio::Pc18 > for Ptc18 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc18  { super::gpio::PC18 }
}

impl LinkGpioPin<super::gpio::Pc19 > for Ptc19 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pc19  { super::gpio::PC19 }
}


impl LinkGpio<super::gpio::Gpiod > for Portd {
    #[inline] fn gpio(&self) -> super::gpio::Gpiod  { super::gpio::GPIOD }
}

impl LinkGpioPin<super::gpio::Pd0 > for Ptd0 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd0  { super::gpio::PD0 }
}

impl LinkGpioPin<super::gpio::Pd1 > for Ptd1 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd1  { super::gpio::PD1 }
}

impl LinkGpioPin<super::gpio::Pd2 > for Ptd2 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd2  { super::gpio::PD2 }
}

impl LinkGpioPin<super::gpio::Pd3 > for Ptd3 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd3  { super::gpio::PD3 }
}

impl LinkGpioPin<super::gpio::Pd4 > for Ptd4 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd4  { super::gpio::PD4 }
}

impl LinkGpioPin<super::gpio::Pd5 > for Ptd5 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd5  { super::gpio::PD5 }
}

impl LinkGpioPin<super::gpio::Pd6 > for Ptd6 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd6  { super::gpio::PD6 }
}

impl LinkGpioPin<super::gpio::Pd7 > for Ptd7 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd7  { super::gpio::PD7 }
}

impl LinkGpioPin<super::gpio::Pd8 > for Ptd8 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd8  { super::gpio::PD8 }
}

impl LinkGpioPin<super::gpio::Pd9 > for Ptd9 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd9  { super::gpio::PD9 }
}

impl LinkGpioPin<super::gpio::Pd10 > for Ptd10 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd10  { super::gpio::PD10 }
}

impl LinkGpioPin<super::gpio::Pd11 > for Ptd11 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd11  { super::gpio::PD11 }
}

impl LinkGpioPin<super::gpio::Pd12 > for Ptd12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd12  { super::gpio::PD12 }
}

impl LinkGpioPin<super::gpio::Pd13 > for Ptd13 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd13  { super::gpio::PD13 }
}

impl LinkGpioPin<super::gpio::Pd14 > for Ptd14 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd14  { super::gpio::PD14 }
}

impl LinkGpioPin<super::gpio::Pd15 > for Ptd15 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pd15  { super::gpio::PD15 }
}


impl LinkGpio<super::gpio::Gpioe > for Porte {
    #[inline] fn gpio(&self) -> super::gpio::Gpioe  { super::gpio::GPIOE }
}

impl LinkGpioPin<super::gpio::Pe0 > for Pte0 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe0  { super::gpio::PE0 }
}

impl LinkGpioPin<super::gpio::Pe1 > for Pte1 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe1  { super::gpio::PE1 }
}

impl LinkGpioPin<super::gpio::Pe2 > for Pte2 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe2  { super::gpio::PE2 }
}

impl LinkGpioPin<super::gpio::Pe3 > for Pte3 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe3  { super::gpio::PE3 }
}

impl LinkGpioPin<super::gpio::Pe4 > for Pte4 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe4  { super::gpio::PE4 }
}

impl LinkGpioPin<super::gpio::Pe5 > for Pte5 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe5  { super::gpio::PE5 }
}

impl LinkGpioPin<super::gpio::Pe6 > for Pte6 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe6  { super::gpio::PE6 }
}

impl LinkGpioPin<super::gpio::Pe7 > for Pte7 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe7  { super::gpio::PE7 }
}

impl LinkGpioPin<super::gpio::Pe8 > for Pte8 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe8  { super::gpio::PE8 }
}

impl LinkGpioPin<super::gpio::Pe9 > for Pte9 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe9  { super::gpio::PE9 }
}

impl LinkGpioPin<super::gpio::Pe10 > for Pte10 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe10  { super::gpio::PE10 }
}

impl LinkGpioPin<super::gpio::Pe11 > for Pte11 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe11  { super::gpio::PE11 }
}

impl LinkGpioPin<super::gpio::Pe12 > for Pte12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe12  { super::gpio::PE12 }
}

impl LinkGpioPin<super::gpio::Pe24 > for Pte24 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe24  { super::gpio::PE24 }
}

impl LinkGpioPin<super::gpio::Pe25 > for Pte25 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe25  { super::gpio::PE25 }
}

impl LinkGpioPin<super::gpio::Pe26 > for Pte26 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe26  { super::gpio::PE26 }
}

impl LinkGpioPin<super::gpio::Pe27 > for Pte27 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe27  { super::gpio::PE27 }
}

impl LinkGpioPin<super::gpio::Pe28 > for Pte28 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe28  { super::gpio::PE28 }
}



pin!(PTA0, Pta0, PORTA, Porta, _PTA0, PortPin, _PORTA, 0);
    alt_fn!(Pta0, super::sig::Pta0, 1);
    alt_fn!(Pta0, super::sig::Uart0CtsB, 2);
    alt_fn!(Pta0, super::sig::Uart0ColB, 2);
    alt_fn!(Pta0, super::sig::Ftm0Ch5, 3);
    alt_fn!(Pta0, super::sig::JtagTclk, 7);
    alt_fn!(Pta0, super::sig::SwdClk, 7);

pin!(PTA1, Pta1, PORTA, Porta, _PTA1, PortPin, _PORTA, 1);
    alt_fn!(Pta1, super::sig::Pta1, 1);
    alt_fn!(Pta1, super::sig::Uart0Rx, 2);
    alt_fn!(Pta1, super::sig::Ftm0Ch6, 3);
    alt_fn!(Pta1, super::sig::JtagTdi, 7);

pin!(PTA2, Pta2, PORTA, Porta, _PTA2, PortPin, _PORTA, 2);
    alt_fn!(Pta2, super::sig::Pta2, 1);
    alt_fn!(Pta2, super::sig::Uart0Tx, 2);
    alt_fn!(Pta2, super::sig::Ftm0Ch7, 3);
    alt_fn!(Pta2, super::sig::JtagTdo, 7);
    alt_fn!(Pta2, super::sig::TraceSwo, 7);

pin!(PTA3, Pta3, PORTA, Porta, _PTA3, PortPin, _PORTA, 3);
    alt_fn!(Pta3, super::sig::Pta3, 1);
    alt_fn!(Pta3, super::sig::Uart0RtsB, 2);
    alt_fn!(Pta3, super::sig::Ftm0Ch0, 3);
    alt_fn!(Pta3, super::sig::JtagTms, 7);
    alt_fn!(Pta3, super::sig::SwdDio, 7);

pin!(PTA4, Pta4, PORTA, Porta, _PTA4, PortPin, _PORTA, 4);
    alt_fn!(Pta4, super::sig::Pta4, 1);
    alt_fn!(Pta4, super::sig::Ftm0Ch1, 3);
    alt_fn!(Pta4, super::sig::NmiB, 7);

pin!(PTA5, Pta5, PORTA, Porta, _PTA5, PortPin, _PORTA, 5);
    alt_fn!(Pta5, super::sig::Pta5, 1);
    alt_fn!(Pta5, super::sig::UsbClkin, 2);
    alt_fn!(Pta5, super::sig::Ftm0Ch2, 3);
    alt_fn!(Pta5, super::sig::Rmii0Rxer, 4);
    alt_fn!(Pta5, super::sig::Mii0Rxer, 4);
    alt_fn!(Pta5, super::sig::Cmp2Out, 5);
    alt_fn!(Pta5, super::sig::I2s0TxBclk, 6);
    alt_fn!(Pta5, super::sig::JtagTrstB, 7);

pin!(PTA6, Pta6, PORTA, Porta, _PTA6, PortPin, _PORTA, 6);
    alt_fn!(Pta6, super::sig::Pta6, 1);
    alt_fn!(Pta6, super::sig::Ftm0Ch3, 3);
    alt_fn!(Pta6, super::sig::Clkout, 5);
    alt_fn!(Pta6, super::sig::TraceClkout, 7);

pin!(PTA7, Pta7, PORTA, Porta, _PTA7, PortPin, _PORTA, 7);
    alt_fn!(Pta7, super::sig::Adc0Se10, 0);
    alt_fn!(Pta7, super::sig::Pta7, 1);
    alt_fn!(Pta7, super::sig::Ftm0Ch4, 3);
    alt_fn!(Pta7, super::sig::TraceD3, 7);

pin!(PTA8, Pta8, PORTA, Porta, _PTA8, PortPin, _PORTA, 8);
    alt_fn!(Pta8, super::sig::Adc0Se11, 0);
    alt_fn!(Pta8, super::sig::Pta8, 1);
    alt_fn!(Pta8, super::sig::Ftm1Ch0, 3);
    alt_fn!(Pta8, super::sig::Ftm1QdPha, 6);
    alt_fn!(Pta8, super::sig::TraceD2, 7);

pin!(PTA9, Pta9, PORTA, Porta, _PTA9, PortPin, _PORTA, 9);
    alt_fn!(Pta9, super::sig::Pta9, 1);
    alt_fn!(Pta9, super::sig::Ftm1Ch1, 3);
    alt_fn!(Pta9, super::sig::Mii0Rxd3, 4);
    alt_fn!(Pta9, super::sig::Ftm1QdPhb, 6);
    alt_fn!(Pta9, super::sig::TraceD1, 7);

pin!(PTA10, Pta10, PORTA, Porta, _PTA10, PortPin, _PORTA, 10);
    alt_fn!(Pta10, super::sig::Pta10, 1);
    alt_fn!(Pta10, super::sig::Ftm2Ch0, 3);
    alt_fn!(Pta10, super::sig::Mii0Rxd2, 4);
    alt_fn!(Pta10, super::sig::Ftm2QdPha, 6);
    alt_fn!(Pta10, super::sig::TraceD0, 7);

pin!(PTA11, Pta11, PORTA, Porta, _PTA11, PortPin, _PORTA, 11);
    alt_fn!(Pta11, super::sig::Pta11, 1);
    alt_fn!(Pta11, super::sig::Ftm2Ch1, 3);
    alt_fn!(Pta11, super::sig::Mii0Rxclk, 4);
    alt_fn!(Pta11, super::sig::I2c2Sda, 5);
    alt_fn!(Pta11, super::sig::Ftm2QdPhb, 6);

pin!(PTA12, Pta12, PORTA, Porta, _PTA12, PortPin, _PORTA, 12);
    alt_fn!(Pta12, super::sig::Cmp2In0, 0);
    alt_fn!(Pta12, super::sig::Pta12, 1);
    alt_fn!(Pta12, super::sig::Can0Tx, 2);
    alt_fn!(Pta12, super::sig::Ftm1Ch0, 3);
    alt_fn!(Pta12, super::sig::Rmii0Rxd1, 4);
    alt_fn!(Pta12, super::sig::Mii0Rxd1, 4);
    alt_fn!(Pta12, super::sig::I2c2Scl, 5);
    alt_fn!(Pta12, super::sig::I2s0Txd0, 6);
    alt_fn!(Pta12, super::sig::Ftm1QdPha, 7);

pin!(PTA13, Pta13, PORTA, Porta, _PTA13, PortPin, _PORTA, 13);
    alt_fn!(Pta13, super::sig::Cmp2In1, 0);
    alt_fn!(Pta13, super::sig::Pta13, 1);
    alt_fn!(Pta13, super::sig::Can0Rx, 2);
    alt_fn!(Pta13, super::sig::Ftm1Ch1, 3);
    alt_fn!(Pta13, super::sig::Rmii0Rxd0, 4);
    alt_fn!(Pta13, super::sig::Mii0Rxd0, 4);
    alt_fn!(Pta13, super::sig::I2c2Sda, 5);
    alt_fn!(Pta13, super::sig::I2s0TxFs, 6);
    alt_fn!(Pta13, super::sig::Ftm1QdPhb, 7);

pin!(PTA14, Pta14, PORTA, Porta, _PTA14, PortPin, _PORTA, 14);
    alt_fn!(Pta14, super::sig::Pta14, 1);
    alt_fn!(Pta14, super::sig::Spi0Pcs0, 2);
    alt_fn!(Pta14, super::sig::Uart0Tx, 3);
    alt_fn!(Pta14, super::sig::Rmii0CrsDv, 4);
    alt_fn!(Pta14, super::sig::Mii0Rxdv, 4);
    alt_fn!(Pta14, super::sig::I2c2Scl, 5);
    alt_fn!(Pta14, super::sig::I2s0RxBclk, 6);
    alt_fn!(Pta14, super::sig::I2s0Txd1, 7);

pin!(PTA15, Pta15, PORTA, Porta, _PTA15, PortPin, _PORTA, 15);
    alt_fn!(Pta15, super::sig::Pta15, 1);
    alt_fn!(Pta15, super::sig::Spi0Sck, 2);
    alt_fn!(Pta15, super::sig::Uart0Rx, 3);
    alt_fn!(Pta15, super::sig::Rmii0Txen, 4);
    alt_fn!(Pta15, super::sig::Mii0Txen, 4);
    alt_fn!(Pta15, super::sig::I2s0Rxd0, 6);

pin!(PTA16, Pta16, PORTA, Porta, _PTA16, PortPin, _PORTA, 16);
    alt_fn!(Pta16, super::sig::Pta16, 1);
    alt_fn!(Pta16, super::sig::Spi0Sout, 2);
    alt_fn!(Pta16, super::sig::Uart0CtsB, 3);
    alt_fn!(Pta16, super::sig::Uart0ColB, 3);
    alt_fn!(Pta16, super::sig::Rmii0Txd0, 4);
    alt_fn!(Pta16, super::sig::Mii0Txd0, 4);
    alt_fn!(Pta16, super::sig::I2s0RxFs, 6);
    alt_fn!(Pta16, super::sig::I2s0Rxd1, 7);

pin!(PTA17, Pta17, PORTA, Porta, _PTA17, PortPin, _PORTA, 17);
    alt_fn!(Pta17, super::sig::Adc1Se17, 0);
    alt_fn!(Pta17, super::sig::Pta17, 1);
    alt_fn!(Pta17, super::sig::Spi0Sin, 2);
    alt_fn!(Pta17, super::sig::Uart0RtsB, 3);
    alt_fn!(Pta17, super::sig::Rmii0Txd1, 4);
    alt_fn!(Pta17, super::sig::Mii0Txd1, 4);
    alt_fn!(Pta17, super::sig::I2s0Mclk, 6);

pin!(PTA18, Pta18, PORTA, Porta, _PTA18, PortPin, _PORTA, 18);
    alt_fn!(Pta18, super::sig::Extal0, 0);
    alt_fn!(Pta18, super::sig::Pta18, 1);
    alt_fn!(Pta18, super::sig::Ftm0Flt2, 3);
    alt_fn!(Pta18, super::sig::FtmClkin0, 4);

pin!(PTA19, Pta19, PORTA, Porta, _PTA19, PortPin, _PORTA, 19);
    alt_fn!(Pta19, super::sig::Xtal0, 0);
    alt_fn!(Pta19, super::sig::Pta19, 1);
    alt_fn!(Pta19, super::sig::Ftm1Flt0, 3);
    alt_fn!(Pta19, super::sig::FtmClkin1, 4);
    alt_fn!(Pta19, super::sig::Lptmr0Alt1, 6);

pin!(PTA24, Pta24, PORTA, Porta, _PTA24, PortPin, _PORTA, 24);
    alt_fn!(Pta24, super::sig::Pta24, 1);
    alt_fn!(Pta24, super::sig::Mii0Txd2, 4);
    alt_fn!(Pta24, super::sig::FbA29, 6);

pin!(PTA25, Pta25, PORTA, Porta, _PTA25, PortPin, _PORTA, 25);
    alt_fn!(Pta25, super::sig::Pta25, 1);
    alt_fn!(Pta25, super::sig::Mii0Txclk, 4);
    alt_fn!(Pta25, super::sig::FbA28, 6);

pin!(PTA26, Pta26, PORTA, Porta, _PTA26, PortPin, _PORTA, 26);
    alt_fn!(Pta26, super::sig::Pta26, 1);
    alt_fn!(Pta26, super::sig::Mii0Txd3, 4);
    alt_fn!(Pta26, super::sig::FbA27, 6);

pin!(PTA27, Pta27, PORTA, Porta, _PTA27, PortPin, _PORTA, 27);
    alt_fn!(Pta27, super::sig::Pta27, 1);
    alt_fn!(Pta27, super::sig::Mii0Crs, 4);
    alt_fn!(Pta27, super::sig::FbA26, 6);

pin!(PTA28, Pta28, PORTA, Porta, _PTA28, PortPin, _PORTA, 28);
    alt_fn!(Pta28, super::sig::Pta28, 1);
    alt_fn!(Pta28, super::sig::Mii0Txer, 4);
    alt_fn!(Pta28, super::sig::FbA25, 6);

pin!(PTA29, Pta29, PORTA, Porta, _PTA29, PortPin, _PORTA, 29);
    alt_fn!(Pta29, super::sig::Pta29, 1);
    alt_fn!(Pta29, super::sig::Mii0Col, 4);
    alt_fn!(Pta29, super::sig::FbA24, 6);

pin!(PTB0, Ptb0, PORTB, Portb, _PTB0, PortPin, _PORTB, 0);
    alt_fn!(Ptb0, super::sig::Adc0Se8, 0);
    alt_fn!(Ptb0, super::sig::Adc1Se8, 0);
    alt_fn!(Ptb0, super::sig::Ptb0, 1);
    alt_fn!(Ptb0, super::sig::I2c0Scl, 2);
    alt_fn!(Ptb0, super::sig::Ftm1Ch0, 3);
    alt_fn!(Ptb0, super::sig::Rmii0Mdio, 4);
    alt_fn!(Ptb0, super::sig::Mii0Mdio, 4);
    alt_fn!(Ptb0, super::sig::Ftm1QdPha, 6);

pin!(PTB1, Ptb1, PORTB, Portb, _PTB1, PortPin, _PORTB, 1);
    alt_fn!(Ptb1, super::sig::Adc0Se9, 0);
    alt_fn!(Ptb1, super::sig::Adc1Se9, 0);
    alt_fn!(Ptb1, super::sig::Ptb1, 1);
    alt_fn!(Ptb1, super::sig::I2c0Sda, 2);
    alt_fn!(Ptb1, super::sig::Ftm1Ch1, 3);
    alt_fn!(Ptb1, super::sig::Rmii0Mdc, 4);
    alt_fn!(Ptb1, super::sig::Mii0Mdc, 4);
    alt_fn!(Ptb1, super::sig::Ftm1QdPhb, 6);

pin!(PTB2, Ptb2, PORTB, Portb, _PTB2, PortPin, _PORTB, 2);
    alt_fn!(Ptb2, super::sig::Adc0Se12, 0);
    alt_fn!(Ptb2, super::sig::Ptb2, 1);
    alt_fn!(Ptb2, super::sig::I2c0Scl, 2);
    alt_fn!(Ptb2, super::sig::Uart0RtsB, 3);
    alt_fn!(Ptb2, super::sig::Enet01588Tmr0, 4);
    alt_fn!(Ptb2, super::sig::Ftm0Flt3, 6);

pin!(PTB3, Ptb3, PORTB, Portb, _PTB3, PortPin, _PORTB, 3);
    alt_fn!(Ptb3, super::sig::Adc0Se13, 0);
    alt_fn!(Ptb3, super::sig::Ptb3, 1);
    alt_fn!(Ptb3, super::sig::I2c0Sda, 2);
    alt_fn!(Ptb3, super::sig::Uart0CtsB, 3);
    alt_fn!(Ptb3, super::sig::Uart0ColB, 3);
    alt_fn!(Ptb3, super::sig::Enet01588Tmr1, 4);
    alt_fn!(Ptb3, super::sig::Ftm0Flt0, 6);

pin!(PTB4, Ptb4, PORTB, Portb, _PTB4, PortPin, _PORTB, 4);
    alt_fn!(Ptb4, super::sig::Adc1Se10, 0);
    alt_fn!(Ptb4, super::sig::Ptb4, 1);
    alt_fn!(Ptb4, super::sig::Enet01588Tmr2, 4);
    alt_fn!(Ptb4, super::sig::Ftm1Flt0, 6);

pin!(PTB5, Ptb5, PORTB, Portb, _PTB5, PortPin, _PORTB, 5);
    alt_fn!(Ptb5, super::sig::Adc1Se11, 0);
    alt_fn!(Ptb5, super::sig::Ptb5, 1);
    alt_fn!(Ptb5, super::sig::Enet01588Tmr3, 4);
    alt_fn!(Ptb5, super::sig::Ftm2Flt0, 6);

pin!(PTB6, Ptb6, PORTB, Portb, _PTB6, PortPin, _PORTB, 6);
    alt_fn!(Ptb6, super::sig::Adc1Se12, 0);
    alt_fn!(Ptb6, super::sig::Ptb6, 1);
    alt_fn!(Ptb6, super::sig::FbAd23, 5);

pin!(PTB7, Ptb7, PORTB, Portb, _PTB7, PortPin, _PORTB, 7);
    alt_fn!(Ptb7, super::sig::Adc1Se13, 0);
    alt_fn!(Ptb7, super::sig::Ptb7, 1);
    alt_fn!(Ptb7, super::sig::FbAd22, 5);

pin!(PTB8, Ptb8, PORTB, Portb, _PTB8, PortPin, _PORTB, 8);
    alt_fn!(Ptb8, super::sig::Ptb8, 1);
    alt_fn!(Ptb8, super::sig::Uart3RtsB, 3);
    alt_fn!(Ptb8, super::sig::FbAd21, 5);

pin!(PTB9, Ptb9, PORTB, Portb, _PTB9, PortPin, _PORTB, 9);
    alt_fn!(Ptb9, super::sig::Ptb9, 1);
    alt_fn!(Ptb9, super::sig::Spi1Pcs1, 2);
    alt_fn!(Ptb9, super::sig::Uart3CtsB, 3);
    alt_fn!(Ptb9, super::sig::FbAd20, 5);

pin!(PTB10, Ptb10, PORTB, Portb, _PTB10, PortPin, _PORTB, 10);
    alt_fn!(Ptb10, super::sig::Adc1Se14, 0);
    alt_fn!(Ptb10, super::sig::Ptb10, 1);
    alt_fn!(Ptb10, super::sig::Spi1Pcs0, 2);
    alt_fn!(Ptb10, super::sig::Uart3Rx, 3);
    alt_fn!(Ptb10, super::sig::FbAd19, 5);
    alt_fn!(Ptb10, super::sig::Ftm0Flt1, 6);

pin!(PTB11, Ptb11, PORTB, Portb, _PTB11, PortPin, _PORTB, 11);
    alt_fn!(Ptb11, super::sig::Adc1Se15, 0);
    alt_fn!(Ptb11, super::sig::Ptb11, 1);
    alt_fn!(Ptb11, super::sig::Spi1Sck, 2);
    alt_fn!(Ptb11, super::sig::Uart3Tx, 3);
    alt_fn!(Ptb11, super::sig::FbAd18, 5);
    alt_fn!(Ptb11, super::sig::Ftm0Flt2, 6);

pin!(PTB12, Ptb12, PORTB, Portb, _PTB12, PortPin, _PORTB, 12);
    alt_fn!(Ptb12, super::sig::Ptb12, 1);
    alt_fn!(Ptb12, super::sig::Uart3RtsB, 2);
    alt_fn!(Ptb12, super::sig::Ftm1Ch0, 3);
    alt_fn!(Ptb12, super::sig::Ftm0Ch4, 4);
    alt_fn!(Ptb12, super::sig::Ftm1QdPha, 6);

pin!(PTB13, Ptb13, PORTB, Portb, _PTB13, PortPin, _PORTB, 13);
    alt_fn!(Ptb13, super::sig::Ptb13, 1);
    alt_fn!(Ptb13, super::sig::Uart3CtsB, 2);
    alt_fn!(Ptb13, super::sig::Ftm1Ch1, 3);
    alt_fn!(Ptb13, super::sig::Ftm0Ch5, 4);
    alt_fn!(Ptb13, super::sig::Ftm1QdPhb, 6);

pin!(PTB16, Ptb16, PORTB, Portb, _PTB16, PortPin, _PORTB, 16);
    alt_fn!(Ptb16, super::sig::Ptb16, 1);
    alt_fn!(Ptb16, super::sig::Spi1Sout, 2);
    alt_fn!(Ptb16, super::sig::Uart0Rx, 3);
    alt_fn!(Ptb16, super::sig::FtmClkin0, 4);
    alt_fn!(Ptb16, super::sig::FbAd17, 5);
    alt_fn!(Ptb16, super::sig::EwmIn, 6);

pin!(PTB17, Ptb17, PORTB, Portb, _PTB17, PortPin, _PORTB, 17);
    alt_fn!(Ptb17, super::sig::Ptb17, 1);
    alt_fn!(Ptb17, super::sig::Spi1Sin, 2);
    alt_fn!(Ptb17, super::sig::Uart0Tx, 3);
    alt_fn!(Ptb17, super::sig::FtmClkin1, 4);
    alt_fn!(Ptb17, super::sig::FbAd16, 5);
    alt_fn!(Ptb17, super::sig::EwmOutB, 6);

pin!(PTB18, Ptb18, PORTB, Portb, _PTB18, PortPin, _PORTB, 18);
    alt_fn!(Ptb18, super::sig::Ptb18, 1);
    alt_fn!(Ptb18, super::sig::Can0Tx, 2);
    alt_fn!(Ptb18, super::sig::Ftm2Ch0, 3);
    alt_fn!(Ptb18, super::sig::I2s0TxBclk, 4);
    alt_fn!(Ptb18, super::sig::FbAd15, 5);
    alt_fn!(Ptb18, super::sig::Ftm2QdPha, 6);

pin!(PTB19, Ptb19, PORTB, Portb, _PTB19, PortPin, _PORTB, 19);
    alt_fn!(Ptb19, super::sig::Ptb19, 1);
    alt_fn!(Ptb19, super::sig::Can0Rx, 2);
    alt_fn!(Ptb19, super::sig::Ftm2Ch1, 3);
    alt_fn!(Ptb19, super::sig::I2s0TxFs, 4);
    alt_fn!(Ptb19, super::sig::FbOeB, 5);
    alt_fn!(Ptb19, super::sig::Ftm2QdPhb, 6);

pin!(PTB20, Ptb20, PORTB, Portb, _PTB20, PortPin, _PORTB, 20);
    alt_fn!(Ptb20, super::sig::Ptb20, 1);
    alt_fn!(Ptb20, super::sig::Spi2Pcs0, 2);
    alt_fn!(Ptb20, super::sig::FbAd31, 5);
    alt_fn!(Ptb20, super::sig::Cmp0Out, 6);

pin!(PTB21, Ptb21, PORTB, Portb, _PTB21, PortPin, _PORTB, 21);
    alt_fn!(Ptb21, super::sig::Ptb21, 1);
    alt_fn!(Ptb21, super::sig::Spi2Sck, 2);
    alt_fn!(Ptb21, super::sig::FbAd30, 5);
    alt_fn!(Ptb21, super::sig::Cmp1Out, 6);

pin!(PTB22, Ptb22, PORTB, Portb, _PTB22, PortPin, _PORTB, 22);
    alt_fn!(Ptb22, super::sig::Ptb22, 1);
    alt_fn!(Ptb22, super::sig::Spi2Sout, 2);
    alt_fn!(Ptb22, super::sig::FbAd29, 5);
    alt_fn!(Ptb22, super::sig::Cmp2Out, 6);

pin!(PTB23, Ptb23, PORTB, Portb, _PTB23, PortPin, _PORTB, 23);
    alt_fn!(Ptb23, super::sig::Ptb23, 1);
    alt_fn!(Ptb23, super::sig::Spi2Sin, 2);
    alt_fn!(Ptb23, super::sig::Spi0Pcs5, 3);
    alt_fn!(Ptb23, super::sig::FbAd28, 5);

pin!(PTC0, Ptc0, PORTC, Portc, _PTC0, PortPin, _PORTC, 0);
    alt_fn!(Ptc0, super::sig::Adc0Se14, 0);
    alt_fn!(Ptc0, super::sig::Ptc0, 1);
    alt_fn!(Ptc0, super::sig::Spi0Pcs4, 2);
    alt_fn!(Ptc0, super::sig::Pdb0Extrg, 3);
    alt_fn!(Ptc0, super::sig::UsbSofOut, 4);
    alt_fn!(Ptc0, super::sig::FbAd14, 5);
    alt_fn!(Ptc0, super::sig::I2s0Txd1, 6);

pin!(PTC1, Ptc1, PORTC, Portc, _PTC1, PortPin, _PORTC, 1);
    alt_fn!(Ptc1, super::sig::Adc0Se15, 0);
    alt_fn!(Ptc1, super::sig::Ptc1, 1);
    alt_fn!(Ptc1, super::sig::Spi0Pcs3, 2);
    alt_fn!(Ptc1, super::sig::Uart1RtsB, 3);
    alt_fn!(Ptc1, super::sig::Ftm0Ch0, 4);
    alt_fn!(Ptc1, super::sig::FbAd13, 5);
    alt_fn!(Ptc1, super::sig::I2s0Txd0, 6);

pin!(PTC2, Ptc2, PORTC, Portc, _PTC2, PortPin, _PORTC, 2);
    alt_fn!(Ptc2, super::sig::Adc0Se4b, 0);
    alt_fn!(Ptc2, super::sig::Cmp1In0, 0);
    alt_fn!(Ptc2, super::sig::Ptc2, 1);
    alt_fn!(Ptc2, super::sig::Spi0Pcs2, 2);
    alt_fn!(Ptc2, super::sig::Uart1CtsB, 3);
    alt_fn!(Ptc2, super::sig::Ftm0Ch1, 4);
    alt_fn!(Ptc2, super::sig::FbAd12, 5);
    alt_fn!(Ptc2, super::sig::I2s0TxFs, 6);

pin!(PTC3, Ptc3, PORTC, Portc, _PTC3, PortPin, _PORTC, 3);
    alt_fn!(Ptc3, super::sig::Cmp1In1, 0);
    alt_fn!(Ptc3, super::sig::Ptc3, 1);
    alt_fn!(Ptc3, super::sig::Spi0Pcs1, 2);
    alt_fn!(Ptc3, super::sig::Uart1Rx, 3);
    alt_fn!(Ptc3, super::sig::Ftm0Ch2, 4);
    alt_fn!(Ptc3, super::sig::Clkout, 5);
    alt_fn!(Ptc3, super::sig::I2s0TxBclk, 6);

pin!(PTC4, Ptc4, PORTC, Portc, _PTC4, PortPin, _PORTC, 4);
    alt_fn!(Ptc4, super::sig::Ptc4, 1);
    alt_fn!(Ptc4, super::sig::Spi0Pcs0, 2);
    alt_fn!(Ptc4, super::sig::Uart1Tx, 3);
    alt_fn!(Ptc4, super::sig::Ftm0Ch3, 4);
    alt_fn!(Ptc4, super::sig::FbAd11, 5);
    alt_fn!(Ptc4, super::sig::Cmp1Out, 6);

pin!(PTC5, Ptc5, PORTC, Portc, _PTC5, PortPin, _PORTC, 5);
    alt_fn!(Ptc5, super::sig::Ptc5, 1);
    alt_fn!(Ptc5, super::sig::Spi0Sck, 2);
    alt_fn!(Ptc5, super::sig::Lptmr0Alt2, 3);
    alt_fn!(Ptc5, super::sig::I2s0Rxd0, 4);
    alt_fn!(Ptc5, super::sig::FbAd10, 5);
    alt_fn!(Ptc5, super::sig::Cmp0Out, 6);
    alt_fn!(Ptc5, super::sig::Ftm0Ch2, 7);

pin!(PTC6, Ptc6, PORTC, Portc, _PTC6, PortPin, _PORTC, 6);
    alt_fn!(Ptc6, super::sig::Cmp0In0, 0);
    alt_fn!(Ptc6, super::sig::Ptc6, 1);
    alt_fn!(Ptc6, super::sig::Spi0Sout, 2);
    alt_fn!(Ptc6, super::sig::Pdb0Extrg, 3);
    alt_fn!(Ptc6, super::sig::I2s0RxBclk, 4);
    alt_fn!(Ptc6, super::sig::FbAd9, 5);
    alt_fn!(Ptc6, super::sig::I2s0Mclk, 6);

pin!(PTC7, Ptc7, PORTC, Portc, _PTC7, PortPin, _PORTC, 7);
    alt_fn!(Ptc7, super::sig::Cmp0In1, 0);
    alt_fn!(Ptc7, super::sig::Ptc7, 1);
    alt_fn!(Ptc7, super::sig::Spi0Sin, 2);
    alt_fn!(Ptc7, super::sig::UsbSofOut, 3);
    alt_fn!(Ptc7, super::sig::I2s0RxFs, 4);
    alt_fn!(Ptc7, super::sig::FbAd8, 5);

pin!(PTC8, Ptc8, PORTC, Portc, _PTC8, PortPin, _PORTC, 8);
    alt_fn!(Ptc8, super::sig::Adc1Se4b, 0);
    alt_fn!(Ptc8, super::sig::Cmp0In2, 0);
    alt_fn!(Ptc8, super::sig::Ptc8, 1);
    alt_fn!(Ptc8, super::sig::Ftm3Ch4, 3);
    alt_fn!(Ptc8, super::sig::I2s0Mclk, 4);
    alt_fn!(Ptc8, super::sig::FbAd7, 5);

pin!(PTC9, Ptc9, PORTC, Portc, _PTC9, PortPin, _PORTC, 9);
    alt_fn!(Ptc9, super::sig::Adc1Se5b, 0);
    alt_fn!(Ptc9, super::sig::Cmp0In3, 0);
    alt_fn!(Ptc9, super::sig::Ptc9, 1);
    alt_fn!(Ptc9, super::sig::Ftm3Ch5, 3);
    alt_fn!(Ptc9, super::sig::I2s0RxBclk, 4);
    alt_fn!(Ptc9, super::sig::FbAd6, 5);
    alt_fn!(Ptc9, super::sig::Ftm2Flt0, 6);

pin!(PTC10, Ptc10, PORTC, Portc, _PTC10, PortPin, _PORTC, 10);
    alt_fn!(Ptc10, super::sig::Adc1Se6b, 0);
    alt_fn!(Ptc10, super::sig::Ptc10, 1);
    alt_fn!(Ptc10, super::sig::I2c1Scl, 2);
    alt_fn!(Ptc10, super::sig::Ftm3Ch6, 3);
    alt_fn!(Ptc10, super::sig::I2s0RxFs, 4);
    alt_fn!(Ptc10, super::sig::FbAd5, 5);

pin!(PTC11, Ptc11, PORTC, Portc, _PTC11, PortPin, _PORTC, 11);
    alt_fn!(Ptc11, super::sig::Adc1Se7b, 0);
    alt_fn!(Ptc11, super::sig::Ptc11, 1);
    alt_fn!(Ptc11, super::sig::I2c1Sda, 2);
    alt_fn!(Ptc11, super::sig::Ftm3Ch7, 3);
    alt_fn!(Ptc11, super::sig::I2s0Rxd1, 4);
    alt_fn!(Ptc11, super::sig::FbRwB, 5);

pin!(PTC12, Ptc12, PORTC, Portc, _PTC12, PortPin, _PORTC, 12);
    alt_fn!(Ptc12, super::sig::Ptc12, 1);
    alt_fn!(Ptc12, super::sig::Uart4RtsB, 3);
    alt_fn!(Ptc12, super::sig::FbAd27, 5);
    alt_fn!(Ptc12, super::sig::Ftm3Flt0, 6);

pin!(PTC13, Ptc13, PORTC, Portc, _PTC13, PortPin, _PORTC, 13);
    alt_fn!(Ptc13, super::sig::Ptc13, 1);
    alt_fn!(Ptc13, super::sig::Uart4CtsB, 3);
    alt_fn!(Ptc13, super::sig::FbAd26, 5);

pin!(PTC14, Ptc14, PORTC, Portc, _PTC14, PortPin, _PORTC, 14);
    alt_fn!(Ptc14, super::sig::Ptc14, 1);
    alt_fn!(Ptc14, super::sig::Uart4Rx, 3);
    alt_fn!(Ptc14, super::sig::FbAd25, 5);

pin!(PTC15, Ptc15, PORTC, Portc, _PTC15, PortPin, _PORTC, 15);
    alt_fn!(Ptc15, super::sig::Ptc15, 1);
    alt_fn!(Ptc15, super::sig::Uart4Tx, 3);
    alt_fn!(Ptc15, super::sig::FbAd24, 5);

pin!(PTC16, Ptc16, PORTC, Portc, _PTC16, PortPin, _PORTC, 16);
    alt_fn!(Ptc16, super::sig::Ptc16, 1);
    alt_fn!(Ptc16, super::sig::Uart3Rx, 3);
    alt_fn!(Ptc16, super::sig::Enet01588Tmr0, 4);
    alt_fn!(Ptc16, super::sig::FbCs5B, 5);
    alt_fn!(Ptc16, super::sig::FbTsiz1, 5);
    alt_fn!(Ptc16, super::sig::FbBe2316Bls158B, 5);

pin!(PTC17, Ptc17, PORTC, Portc, _PTC17, PortPin, _PORTC, 17);
    alt_fn!(Ptc17, super::sig::Ptc17, 1);
    alt_fn!(Ptc17, super::sig::Uart3Tx, 3);
    alt_fn!(Ptc17, super::sig::Enet01588Tmr1, 4);
    alt_fn!(Ptc17, super::sig::FbCs4B, 5);
    alt_fn!(Ptc17, super::sig::FbTsiz0, 5);
    alt_fn!(Ptc17, super::sig::FbBe3124Bls70B, 5);

pin!(PTC18, Ptc18, PORTC, Portc, _PTC18, PortPin, _PORTC, 18);
    alt_fn!(Ptc18, super::sig::Ptc18, 1);
    alt_fn!(Ptc18, super::sig::Uart3RtsB, 3);
    alt_fn!(Ptc18, super::sig::Enet01588Tmr2, 4);
    alt_fn!(Ptc18, super::sig::FbTbstB, 5);
    alt_fn!(Ptc18, super::sig::FbCs2B, 5);
    alt_fn!(Ptc18, super::sig::FbBe158Bls2316B, 5);

pin!(PTC19, Ptc19, PORTC, Portc, _PTC19, PortPin, _PORTC, 19);
    alt_fn!(Ptc19, super::sig::Ptc19, 1);
    alt_fn!(Ptc19, super::sig::Uart3CtsB, 3);
    alt_fn!(Ptc19, super::sig::Enet01588Tmr3, 4);
    alt_fn!(Ptc19, super::sig::FbCs3B, 5);
    alt_fn!(Ptc19, super::sig::FbBe70Bls3124B, 5);
    alt_fn!(Ptc19, super::sig::FbTaB, 6);

pin!(PTD0, Ptd0, PORTD, Portd, _PTD0, PortPin, _PORTD, 0);
    alt_fn!(Ptd0, super::sig::Ptd0, 1);
    alt_fn!(Ptd0, super::sig::Spi0Pcs0, 2);
    alt_fn!(Ptd0, super::sig::Uart2RtsB, 3);
    alt_fn!(Ptd0, super::sig::Ftm3Ch0, 4);
    alt_fn!(Ptd0, super::sig::FbAle, 5);
    alt_fn!(Ptd0, super::sig::FbCs1B, 5);
    alt_fn!(Ptd0, super::sig::FbTsB, 5);

pin!(PTD1, Ptd1, PORTD, Portd, _PTD1, PortPin, _PORTD, 1);
    alt_fn!(Ptd1, super::sig::Adc0Se5b, 0);
    alt_fn!(Ptd1, super::sig::Ptd1, 1);
    alt_fn!(Ptd1, super::sig::Spi0Sck, 2);
    alt_fn!(Ptd1, super::sig::Uart2CtsB, 3);
    alt_fn!(Ptd1, super::sig::Ftm3Ch1, 4);
    alt_fn!(Ptd1, super::sig::FbCs0B, 5);

pin!(PTD2, Ptd2, PORTD, Portd, _PTD2, PortPin, _PORTD, 2);
    alt_fn!(Ptd2, super::sig::Ptd2, 1);
    alt_fn!(Ptd2, super::sig::Spi0Sout, 2);
    alt_fn!(Ptd2, super::sig::Uart2Rx, 3);
    alt_fn!(Ptd2, super::sig::Ftm3Ch2, 4);
    alt_fn!(Ptd2, super::sig::FbAd4, 5);
    alt_fn!(Ptd2, super::sig::I2c0Scl, 7);

pin!(PTD3, Ptd3, PORTD, Portd, _PTD3, PortPin, _PORTD, 3);
    alt_fn!(Ptd3, super::sig::Ptd3, 1);
    alt_fn!(Ptd3, super::sig::Spi0Sin, 2);
    alt_fn!(Ptd3, super::sig::Uart2Tx, 3);
    alt_fn!(Ptd3, super::sig::Ftm3Ch3, 4);
    alt_fn!(Ptd3, super::sig::FbAd3, 5);
    alt_fn!(Ptd3, super::sig::I2c0Sda, 7);

pin!(PTD4, Ptd4, PORTD, Portd, _PTD4, PortPin, _PORTD, 4);
    alt_fn!(Ptd4, super::sig::Ptd4, 1);
    alt_fn!(Ptd4, super::sig::Spi0Pcs1, 2);
    alt_fn!(Ptd4, super::sig::Uart0RtsB, 3);
    alt_fn!(Ptd4, super::sig::Ftm0Ch4, 4);
    alt_fn!(Ptd4, super::sig::FbAd2, 5);
    alt_fn!(Ptd4, super::sig::EwmIn, 6);
    alt_fn!(Ptd4, super::sig::Spi1Pcs0, 7);

pin!(PTD5, Ptd5, PORTD, Portd, _PTD5, PortPin, _PORTD, 5);
    alt_fn!(Ptd5, super::sig::Adc0Se6b, 0);
    alt_fn!(Ptd5, super::sig::Ptd5, 1);
    alt_fn!(Ptd5, super::sig::Spi0Pcs2, 2);
    alt_fn!(Ptd5, super::sig::Uart0CtsB, 3);
    alt_fn!(Ptd5, super::sig::Uart0ColB, 3);
    alt_fn!(Ptd5, super::sig::Ftm0Ch5, 4);
    alt_fn!(Ptd5, super::sig::FbAd1, 5);
    alt_fn!(Ptd5, super::sig::EwmOutB, 6);
    alt_fn!(Ptd5, super::sig::Spi1Sck, 7);

pin!(PTD6, Ptd6, PORTD, Portd, _PTD6, PortPin, _PORTD, 6);
    alt_fn!(Ptd6, super::sig::Adc0Se7b, 0);
    alt_fn!(Ptd6, super::sig::Ptd6, 1);
    alt_fn!(Ptd6, super::sig::Spi0Pcs3, 2);
    alt_fn!(Ptd6, super::sig::Uart0Rx, 3);
    alt_fn!(Ptd6, super::sig::Ftm0Ch6, 4);
    alt_fn!(Ptd6, super::sig::FbAd0, 5);
    alt_fn!(Ptd6, super::sig::Ftm0Flt0, 6);
    alt_fn!(Ptd6, super::sig::Spi1Sout, 7);

pin!(PTD7, Ptd7, PORTD, Portd, _PTD7, PortPin, _PORTD, 7);
    alt_fn!(Ptd7, super::sig::Ptd7, 1);
    alt_fn!(Ptd7, super::sig::CmtIro, 2);
    alt_fn!(Ptd7, super::sig::Uart0Tx, 3);
    alt_fn!(Ptd7, super::sig::Ftm0Ch7, 4);
    alt_fn!(Ptd7, super::sig::Ftm0Flt1, 6);
    alt_fn!(Ptd7, super::sig::Spi1Sin, 7);

pin!(PTD8, Ptd8, PORTD, Portd, _PTD8, PortPin, _PORTD, 8);
    alt_fn!(Ptd8, super::sig::Ptd8, 1);
    alt_fn!(Ptd8, super::sig::I2c0Scl, 2);
    alt_fn!(Ptd8, super::sig::Uart5Rx, 3);
    alt_fn!(Ptd8, super::sig::FbA16, 6);

pin!(PTD9, Ptd9, PORTD, Portd, _PTD9, PortPin, _PORTD, 9);
    alt_fn!(Ptd9, super::sig::Ptd9, 1);
    alt_fn!(Ptd9, super::sig::I2c0Sda, 2);
    alt_fn!(Ptd9, super::sig::Uart5Tx, 3);
    alt_fn!(Ptd9, super::sig::FbA17, 6);

pin!(PTD10, Ptd10, PORTD, Portd, _PTD10, PortPin, _PORTD, 10);
    alt_fn!(Ptd10, super::sig::Ptd10, 1);
    alt_fn!(Ptd10, super::sig::Uart5RtsB, 3);
    alt_fn!(Ptd10, super::sig::FbA18, 6);

pin!(PTD11, Ptd11, PORTD, Portd, _PTD11, PortPin, _PORTD, 11);
    alt_fn!(Ptd11, super::sig::Ptd11, 1);
    alt_fn!(Ptd11, super::sig::Spi2Pcs0, 2);
    alt_fn!(Ptd11, super::sig::Uart5CtsB, 3);
    alt_fn!(Ptd11, super::sig::Sdhc0Clkin, 4);
    alt_fn!(Ptd11, super::sig::FbA19, 6);

pin!(PTD12, Ptd12, PORTD, Portd, _PTD12, PortPin, _PORTD, 12);
    alt_fn!(Ptd12, super::sig::Ptd12, 1);
    alt_fn!(Ptd12, super::sig::Spi2Sck, 2);
    alt_fn!(Ptd12, super::sig::Ftm3Flt0, 3);
    alt_fn!(Ptd12, super::sig::Sdhc0D4, 4);
    alt_fn!(Ptd12, super::sig::FbA20, 6);

pin!(PTD13, Ptd13, PORTD, Portd, _PTD13, PortPin, _PORTD, 13);
    alt_fn!(Ptd13, super::sig::Ptd13, 1);
    alt_fn!(Ptd13, super::sig::Spi2Sout, 2);
    alt_fn!(Ptd13, super::sig::Sdhc0D5, 4);
    alt_fn!(Ptd13, super::sig::FbA21, 6);

pin!(PTD14, Ptd14, PORTD, Portd, _PTD14, PortPin, _PORTD, 14);
    alt_fn!(Ptd14, super::sig::Ptd14, 1);
    alt_fn!(Ptd14, super::sig::Spi2Sin, 2);
    alt_fn!(Ptd14, super::sig::Sdhc0D6, 4);
    alt_fn!(Ptd14, super::sig::FbA22, 6);

pin!(PTD15, Ptd15, PORTD, Portd, _PTD15, PortPin, _PORTD, 15);
    alt_fn!(Ptd15, super::sig::Ptd15, 1);
    alt_fn!(Ptd15, super::sig::Spi2Pcs1, 2);
    alt_fn!(Ptd15, super::sig::Sdhc0D7, 4);
    alt_fn!(Ptd15, super::sig::FbA23, 6);

pin!(PTE0, Pte0, PORTE, Porte, _PTE0, PortPin, _PORTE, 0);
    alt_fn!(Pte0, super::sig::Adc1Se4a, 0);
    alt_fn!(Pte0, super::sig::Pte0, 1);
    alt_fn!(Pte0, super::sig::Spi1Pcs1, 2);
    alt_fn!(Pte0, super::sig::Uart1Tx, 3);
    alt_fn!(Pte0, super::sig::Sdhc0D1, 4);
    alt_fn!(Pte0, super::sig::TraceClkout, 5);
    alt_fn!(Pte0, super::sig::I2c1Sda, 6);
    alt_fn!(Pte0, super::sig::RtcClkout, 7);

pin!(PTE1, Pte1, PORTE, Porte, _PTE1, PortPin, _PORTE, 1);
    alt_fn!(Pte1, super::sig::Adc1Se5a, 0);
    alt_fn!(Pte1, super::sig::Pte1, 1);
    alt_fn!(Pte1, super::sig::Spi1Sout, 2);
    alt_fn!(Pte1, super::sig::Uart1Rx, 3);
    alt_fn!(Pte1, super::sig::Sdhc0D0, 4);
    alt_fn!(Pte1, super::sig::TraceD3, 5);
    alt_fn!(Pte1, super::sig::I2c1Scl, 6);
    alt_fn!(Pte1, super::sig::Spi1Sin, 7);

pin!(PTE2, Pte2, PORTE, Porte, _PTE2, PortPin, _PORTE, 2);
    alt_fn!(Pte2, super::sig::Adc0Dp2, 0);
    alt_fn!(Pte2, super::sig::Adc1Se6a, 0);
    alt_fn!(Pte2, super::sig::Pte2, 1);
    alt_fn!(Pte2, super::sig::Spi1Sck, 2);
    alt_fn!(Pte2, super::sig::Uart1CtsB, 3);
    alt_fn!(Pte2, super::sig::Sdhc0Dclk, 4);
    alt_fn!(Pte2, super::sig::TraceD2, 5);

pin!(PTE3, Pte3, PORTE, Porte, _PTE3, PortPin, _PORTE, 3);
    alt_fn!(Pte3, super::sig::Adc0Dm2, 0);
    alt_fn!(Pte3, super::sig::Adc1Se7a, 0);
    alt_fn!(Pte3, super::sig::Pte3, 1);
    alt_fn!(Pte3, super::sig::Spi1Sin, 2);
    alt_fn!(Pte3, super::sig::Uart1RtsB, 3);
    alt_fn!(Pte3, super::sig::Sdhc0Cmd, 4);
    alt_fn!(Pte3, super::sig::TraceD1, 5);
    alt_fn!(Pte3, super::sig::Spi1Sout, 7);

pin!(PTE4, Pte4, PORTE, Porte, _PTE4, PortPin, _PORTE, 4);
    alt_fn!(Pte4, super::sig::Pte4, 1);
    alt_fn!(Pte4, super::sig::Spi1Pcs0, 2);
    alt_fn!(Pte4, super::sig::Uart3Tx, 3);
    alt_fn!(Pte4, super::sig::Sdhc0D3, 4);
    alt_fn!(Pte4, super::sig::TraceD0, 5);

pin!(PTE5, Pte5, PORTE, Porte, _PTE5, PortPin, _PORTE, 5);
    alt_fn!(Pte5, super::sig::Pte5, 1);
    alt_fn!(Pte5, super::sig::Spi1Pcs2, 2);
    alt_fn!(Pte5, super::sig::Uart3Rx, 3);
    alt_fn!(Pte5, super::sig::Sdhc0D2, 4);
    alt_fn!(Pte5, super::sig::Ftm3Ch0, 6);

pin!(PTE6, Pte6, PORTE, Porte, _PTE6, PortPin, _PORTE, 6);
    alt_fn!(Pte6, super::sig::Pte6, 1);
    alt_fn!(Pte6, super::sig::Spi1Pcs3, 2);
    alt_fn!(Pte6, super::sig::Uart3CtsB, 3);
    alt_fn!(Pte6, super::sig::I2s0Mclk, 4);
    alt_fn!(Pte6, super::sig::Ftm3Ch1, 6);
    alt_fn!(Pte6, super::sig::UsbSofOut, 7);

pin!(PTE7, Pte7, PORTE, Porte, _PTE7, PortPin, _PORTE, 7);
    alt_fn!(Pte7, super::sig::Pte7, 1);
    alt_fn!(Pte7, super::sig::Uart3RtsB, 3);
    alt_fn!(Pte7, super::sig::I2s0Rxd0, 4);
    alt_fn!(Pte7, super::sig::Ftm3Ch2, 6);

pin!(PTE8, Pte8, PORTE, Porte, _PTE8, PortPin, _PORTE, 8);
    alt_fn!(Pte8, super::sig::Pte8, 1);
    alt_fn!(Pte8, super::sig::I2s0Rxd1, 2);
    alt_fn!(Pte8, super::sig::Uart5Tx, 3);
    alt_fn!(Pte8, super::sig::I2s0RxFs, 4);
    alt_fn!(Pte8, super::sig::Ftm3Ch3, 6);

pin!(PTE9, Pte9, PORTE, Porte, _PTE9, PortPin, _PORTE, 9);
    alt_fn!(Pte9, super::sig::Pte9, 1);
    alt_fn!(Pte9, super::sig::I2s0Txd1, 2);
    alt_fn!(Pte9, super::sig::Uart5Rx, 3);
    alt_fn!(Pte9, super::sig::I2s0RxBclk, 4);
    alt_fn!(Pte9, super::sig::Ftm3Ch4, 6);

pin!(PTE10, Pte10, PORTE, Porte, _PTE10, PortPin, _PORTE, 10);
    alt_fn!(Pte10, super::sig::Pte10, 1);
    alt_fn!(Pte10, super::sig::Uart5CtsB, 3);
    alt_fn!(Pte10, super::sig::I2s0Txd0, 4);
    alt_fn!(Pte10, super::sig::Ftm3Ch5, 6);

pin!(PTE11, Pte11, PORTE, Porte, _PTE11, PortPin, _PORTE, 11);
    alt_fn!(Pte11, super::sig::Pte11, 1);
    alt_fn!(Pte11, super::sig::Uart5RtsB, 3);
    alt_fn!(Pte11, super::sig::I2s0TxFs, 4);
    alt_fn!(Pte11, super::sig::Ftm3Ch6, 6);

pin!(PTE12, Pte12, PORTE, Porte, _PTE12, PortPin, _PORTE, 12);
    alt_fn!(Pte12, super::sig::Pte12, 1);
    alt_fn!(Pte12, super::sig::I2s0TxBclk, 4);
    alt_fn!(Pte12, super::sig::Ftm3Ch7, 6);

pin!(PTE24, Pte24, PORTE, Porte, _PTE24, PortPin, _PORTE, 24);
    alt_fn!(Pte24, super::sig::Adc0Se17, 0);
    alt_fn!(Pte24, super::sig::Pte24, 1);
    alt_fn!(Pte24, super::sig::Uart4Tx, 3);
    alt_fn!(Pte24, super::sig::I2c0Scl, 5);
    alt_fn!(Pte24, super::sig::EwmOutB, 6);

pin!(PTE25, Pte25, PORTE, Porte, _PTE25, PortPin, _PORTE, 25);
    alt_fn!(Pte25, super::sig::Adc0Se18, 0);
    alt_fn!(Pte25, super::sig::Pte25, 1);
    alt_fn!(Pte25, super::sig::Uart4Rx, 3);
    alt_fn!(Pte25, super::sig::I2c0Sda, 5);
    alt_fn!(Pte25, super::sig::EwmIn, 6);

pin!(PTE26, Pte26, PORTE, Porte, _PTE26, PortPin, _PORTE, 26);
    alt_fn!(Pte26, super::sig::Pte26, 1);
    alt_fn!(Pte26, super::sig::Enet1588Clkin, 2);
    alt_fn!(Pte26, super::sig::Uart4CtsB, 3);
    alt_fn!(Pte26, super::sig::RtcClkout, 6);
    alt_fn!(Pte26, super::sig::UsbClkin, 7);

pin!(PTE27, Pte27, PORTE, Porte, _PTE27, PortPin, _PORTE, 27);
    alt_fn!(Pte27, super::sig::Pte27, 1);
    alt_fn!(Pte27, super::sig::Uart4RtsB, 3);

pin!(PTE28, Pte28, PORTE, Porte, _PTE28, PortPin, _PORTE, 28);
    alt_fn!(Pte28, super::sig::Pte28, 1);


pub trait IrqPort<T> {
    fn irq_port(&self) -> T;
}

impl IrqPort<super::irq::IrqPorta> for Porta {
    fn irq_port(&self) -> super::irq::IrqPorta { super::irq::IRQ_PORTA }
}

impl IrqPort<super::irq::IrqPortb> for Portb {
    fn irq_port(&self) -> super::irq::IrqPortb { super::irq::IRQ_PORTB }
}

impl IrqPort<super::irq::IrqPortc> for Portc {
    fn irq_port(&self) -> super::irq::IrqPortc { super::irq::IRQ_PORTC }
}

impl IrqPort<super::irq::IrqPortd> for Portd {
    fn irq_port(&self) -> super::irq::IrqPortd { super::irq::IRQ_PORTD }
}

impl IrqPort<super::irq::IrqPorte> for Porte {
    fn irq_port(&self) -> super::irq::IrqPorte { super::irq::IRQ_PORTE }
}

