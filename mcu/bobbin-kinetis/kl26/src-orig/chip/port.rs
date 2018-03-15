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

impl LinkGpioPin<super::gpio::Pa12 > for Pta12 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa12  { super::gpio::PA12 }
}

impl LinkGpioPin<super::gpio::Pa13 > for Pta13 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa13  { super::gpio::PA13 }
}

impl LinkGpioPin<super::gpio::Pa18 > for Pta18 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa18  { super::gpio::PA18 }
}

impl LinkGpioPin<super::gpio::Pa19 > for Pta19 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa19  { super::gpio::PA19 }
}

impl LinkGpioPin<super::gpio::Pa20 > for Pta20 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pa20  { super::gpio::PA20 }
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


impl LinkGpio<super::gpio::Gpioe > for Porte {
    #[inline] fn gpio(&self) -> super::gpio::Gpioe  { super::gpio::GPIOE }
}

impl LinkGpioPin<super::gpio::Pe0 > for Pte0 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe0  { super::gpio::PE0 }
}

impl LinkGpioPin<super::gpio::Pe1 > for Pte1 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe1  { super::gpio::PE1 }
}

impl LinkGpioPin<super::gpio::Pe20 > for Pte20 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe20  { super::gpio::PE20 }
}

impl LinkGpioPin<super::gpio::Pe21 > for Pte21 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe21  { super::gpio::PE21 }
}

impl LinkGpioPin<super::gpio::Pe22 > for Pte22 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe22  { super::gpio::PE22 }
}

impl LinkGpioPin<super::gpio::Pe23 > for Pte23 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe23  { super::gpio::PE23 }
}

impl LinkGpioPin<super::gpio::Pe24 > for Pte24 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe24  { super::gpio::PE24 }
}

impl LinkGpioPin<super::gpio::Pe25 > for Pte25 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe25  { super::gpio::PE25 }
}

impl LinkGpioPin<super::gpio::Pe29 > for Pte29 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe29  { super::gpio::PE29 }
}

impl LinkGpioPin<super::gpio::Pe30 > for Pte30 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe30  { super::gpio::PE30 }
}

impl LinkGpioPin<super::gpio::Pe31 > for Pte31 {
    #[inline] fn gpio_pin(&self) -> super::gpio::Pe31  { super::gpio::PE31 }
}



pin!(PTA0, Pta0, PORTA, Porta, _PTA0, PortPin, _PORTA, 0);
    alt_fn!(Pta0, super::sig::Tsi0Ch1, 0);
    alt_fn!(Pta0, super::sig::Pta0, 1);
    alt_fn!(Pta0, super::sig::Tpm0Ch5, 3);
    alt_fn!(Pta0, super::sig::SwdClk, 7);

pin!(PTA1, Pta1, PORTA, Porta, _PTA1, PortPin, _PORTA, 1);
    alt_fn!(Pta1, super::sig::Tsi0Ch2, 0);
    alt_fn!(Pta1, super::sig::Pta1, 1);
    alt_fn!(Pta1, super::sig::Uart0Rx, 2);
    alt_fn!(Pta1, super::sig::Tpm2Ch0, 3);

pin!(PTA2, Pta2, PORTA, Porta, _PTA2, PortPin, _PORTA, 2);
    alt_fn!(Pta2, super::sig::Tsi0Ch3, 0);
    alt_fn!(Pta2, super::sig::Pta2, 1);
    alt_fn!(Pta2, super::sig::Uart0Tx, 2);
    alt_fn!(Pta2, super::sig::Tpm2Ch1, 3);

pin!(PTA3, Pta3, PORTA, Porta, _PTA3, PortPin, _PORTA, 3);
    alt_fn!(Pta3, super::sig::Tsi0Ch4, 0);
    alt_fn!(Pta3, super::sig::Pta3, 1);
    alt_fn!(Pta3, super::sig::I2c1Scl, 2);
    alt_fn!(Pta3, super::sig::Tpm0Ch0, 3);
    alt_fn!(Pta3, super::sig::SwdDio, 7);

pin!(PTA4, Pta4, PORTA, Porta, _PTA4, PortPin, _PORTA, 4);
    alt_fn!(Pta4, super::sig::Tsi0Ch5, 0);
    alt_fn!(Pta4, super::sig::Pta4, 1);
    alt_fn!(Pta4, super::sig::I2c1Sda, 2);
    alt_fn!(Pta4, super::sig::Tpm0Ch1, 3);
    alt_fn!(Pta4, super::sig::NmiB, 7);

pin!(PTA5, Pta5, PORTA, Porta, _PTA5, PortPin, _PORTA, 5);
    alt_fn!(Pta5, super::sig::Pta5, 1);
    alt_fn!(Pta5, super::sig::UsbClkin, 2);
    alt_fn!(Pta5, super::sig::Tpm0Ch2, 3);
    alt_fn!(Pta5, super::sig::I2s0TxBclk, 6);

pin!(PTA12, Pta12, PORTA, Porta, _PTA12, PortPin, _PORTA, 12);
    alt_fn!(Pta12, super::sig::Pta12, 1);
    alt_fn!(Pta12, super::sig::Tpm1Ch0, 3);
    alt_fn!(Pta12, super::sig::I2s0Txd0, 6);

pin!(PTA13, Pta13, PORTA, Porta, _PTA13, PortPin, _PORTA, 13);
    alt_fn!(Pta13, super::sig::Pta13, 1);
    alt_fn!(Pta13, super::sig::Tpm1Ch1, 3);
    alt_fn!(Pta13, super::sig::I2s0TxFs, 6);

pin!(PTA18, Pta18, PORTA, Porta, _PTA18, PortPin, _PORTA, 18);
    alt_fn!(Pta18, super::sig::Extal0, 0);
    alt_fn!(Pta18, super::sig::Pta18, 1);
    alt_fn!(Pta18, super::sig::Uart1Rx, 3);
    alt_fn!(Pta18, super::sig::TpmClkin0, 4);

pin!(PTA19, Pta19, PORTA, Porta, _PTA19, PortPin, _PORTA, 19);
    alt_fn!(Pta19, super::sig::Xtal0, 0);
    alt_fn!(Pta19, super::sig::Pta19, 1);
    alt_fn!(Pta19, super::sig::Uart1Tx, 3);
    alt_fn!(Pta19, super::sig::TpmClkin1, 4);
    alt_fn!(Pta19, super::sig::Lptmr0Alt1, 6);

pin!(PTA20, Pta20, PORTA, Porta, _PTA20, PortPin, _PORTA, 20);
    alt_fn!(Pta20, super::sig::Pta20, 1);
    alt_fn!(Pta20, super::sig::ResetB, 7);

pin!(PTB0, Ptb0, PORTB, Portb, _PTB0, PortPin, _PORTB, 0);
    alt_fn!(Ptb0, super::sig::Adc0Se8, 0);
    alt_fn!(Ptb0, super::sig::Tsi0Ch0, 0);
    alt_fn!(Ptb0, super::sig::Ptb0, 1);
    alt_fn!(Ptb0, super::sig::I2c0Scl, 2);
    alt_fn!(Ptb0, super::sig::Tpm1Ch0, 3);

pin!(PTB1, Ptb1, PORTB, Portb, _PTB1, PortPin, _PORTB, 1);
    alt_fn!(Ptb1, super::sig::Adc0Se9, 0);
    alt_fn!(Ptb1, super::sig::Tsi0Ch6, 0);
    alt_fn!(Ptb1, super::sig::Ptb1, 1);
    alt_fn!(Ptb1, super::sig::I2c0Sda, 2);
    alt_fn!(Ptb1, super::sig::Tpm1Ch1, 3);

pin!(PTB2, Ptb2, PORTB, Portb, _PTB2, PortPin, _PORTB, 2);
    alt_fn!(Ptb2, super::sig::Adc0Se12, 0);
    alt_fn!(Ptb2, super::sig::Tsi0Ch7, 0);
    alt_fn!(Ptb2, super::sig::Ptb2, 1);
    alt_fn!(Ptb2, super::sig::I2c0Scl, 2);
    alt_fn!(Ptb2, super::sig::Tpm2Ch0, 3);

pin!(PTB3, Ptb3, PORTB, Portb, _PTB3, PortPin, _PORTB, 3);
    alt_fn!(Ptb3, super::sig::Adc0Se13, 0);
    alt_fn!(Ptb3, super::sig::Tsi0Ch8, 0);
    alt_fn!(Ptb3, super::sig::Ptb3, 1);
    alt_fn!(Ptb3, super::sig::I2c0Sda, 2);
    alt_fn!(Ptb3, super::sig::Tpm2Ch1, 3);

pin!(PTB16, Ptb16, PORTB, Portb, _PTB16, PortPin, _PORTB, 16);
    alt_fn!(Ptb16, super::sig::Tsi0Ch9, 0);
    alt_fn!(Ptb16, super::sig::Ptb16, 1);
    alt_fn!(Ptb16, super::sig::Spi1Mosi, 2);
    alt_fn!(Ptb16, super::sig::Uart0Rx, 3);
    alt_fn!(Ptb16, super::sig::TpmClkin0, 4);
    alt_fn!(Ptb16, super::sig::Spi1Miso, 5);

pin!(PTB17, Ptb17, PORTB, Portb, _PTB17, PortPin, _PORTB, 17);
    alt_fn!(Ptb17, super::sig::Tsi0Ch10, 0);
    alt_fn!(Ptb17, super::sig::Ptb17, 1);
    alt_fn!(Ptb17, super::sig::Spi1Miso, 2);
    alt_fn!(Ptb17, super::sig::Uart0Tx, 3);
    alt_fn!(Ptb17, super::sig::TpmClkin1, 4);
    alt_fn!(Ptb17, super::sig::Spi1Mosi, 5);

pin!(PTB18, Ptb18, PORTB, Portb, _PTB18, PortPin, _PORTB, 18);
    alt_fn!(Ptb18, super::sig::Tsi0Ch11, 0);
    alt_fn!(Ptb18, super::sig::Ptb18, 1);
    alt_fn!(Ptb18, super::sig::Tpm2Ch0, 3);
    alt_fn!(Ptb18, super::sig::I2s0TxBclk, 4);

pin!(PTB19, Ptb19, PORTB, Portb, _PTB19, PortPin, _PORTB, 19);
    alt_fn!(Ptb19, super::sig::Tsi0Ch12, 0);
    alt_fn!(Ptb19, super::sig::Ptb19, 1);
    alt_fn!(Ptb19, super::sig::Tpm2Ch1, 3);
    alt_fn!(Ptb19, super::sig::I2s0TxFs, 4);

pin!(PTC0, Ptc0, PORTC, Portc, _PTC0, PortPin, _PORTC, 0);
    alt_fn!(Ptc0, super::sig::Adc0Se14, 0);
    alt_fn!(Ptc0, super::sig::Tsi0Ch13, 0);
    alt_fn!(Ptc0, super::sig::Ptc0, 1);
    alt_fn!(Ptc0, super::sig::ExtrgIn, 3);
    alt_fn!(Ptc0, super::sig::AudiousbSofOut, 4);
    alt_fn!(Ptc0, super::sig::Cmp0Out, 5);
    alt_fn!(Ptc0, super::sig::I2s0Txd0, 6);

pin!(PTC1, Ptc1, PORTC, Portc, _PTC1, PortPin, _PORTC, 1);
    alt_fn!(Ptc1, super::sig::Adc0Se15, 0);
    alt_fn!(Ptc1, super::sig::Tsi0Ch14, 0);
    alt_fn!(Ptc1, super::sig::Ptc1, 1);
    alt_fn!(Ptc1, super::sig::I2c1Scl, 2);
    alt_fn!(Ptc1, super::sig::Tpm0Ch0, 4);
    alt_fn!(Ptc1, super::sig::I2s0Txd0, 6);

pin!(PTC2, Ptc2, PORTC, Portc, _PTC2, PortPin, _PORTC, 2);
    alt_fn!(Ptc2, super::sig::Adc0Se11, 0);
    alt_fn!(Ptc2, super::sig::Tsi0Ch15, 0);
    alt_fn!(Ptc2, super::sig::Ptc2, 1);
    alt_fn!(Ptc2, super::sig::I2c1Sda, 2);
    alt_fn!(Ptc2, super::sig::Tpm0Ch1, 4);
    alt_fn!(Ptc2, super::sig::I2s0TxFs, 6);

pin!(PTC3, Ptc3, PORTC, Portc, _PTC3, PortPin, _PORTC, 3);
    alt_fn!(Ptc3, super::sig::Ptc3, 1);
    alt_fn!(Ptc3, super::sig::Uart1Rx, 3);
    alt_fn!(Ptc3, super::sig::Tpm0Ch2, 4);
    alt_fn!(Ptc3, super::sig::Clkout, 5);
    alt_fn!(Ptc3, super::sig::I2s0TxBclk, 6);

pin!(PTC4, Ptc4, PORTC, Portc, _PTC4, PortPin, _PORTC, 4);
    alt_fn!(Ptc4, super::sig::Ptc4, 1);
    alt_fn!(Ptc4, super::sig::Spi0Pcs0, 2);
    alt_fn!(Ptc4, super::sig::Uart1Tx, 3);
    alt_fn!(Ptc4, super::sig::Tpm0Ch3, 4);
    alt_fn!(Ptc4, super::sig::I2s0Mclk, 5);

pin!(PTC5, Ptc5, PORTC, Portc, _PTC5, PortPin, _PORTC, 5);
    alt_fn!(Ptc5, super::sig::Ptc5, 1);
    alt_fn!(Ptc5, super::sig::Spi0Sck, 2);
    alt_fn!(Ptc5, super::sig::Lptmr0Alt2, 3);
    alt_fn!(Ptc5, super::sig::I2s0Rxd0, 4);
    alt_fn!(Ptc5, super::sig::Cmp0Out, 6);

pin!(PTC6, Ptc6, PORTC, Portc, _PTC6, PortPin, _PORTC, 6);
    alt_fn!(Ptc6, super::sig::Cmp0In0, 0);
    alt_fn!(Ptc6, super::sig::Ptc6, 1);
    alt_fn!(Ptc6, super::sig::Spi0Mosi, 2);
    alt_fn!(Ptc6, super::sig::ExtrgIn, 3);
    alt_fn!(Ptc6, super::sig::I2s0RxBclk, 4);
    alt_fn!(Ptc6, super::sig::Spi0Miso, 5);
    alt_fn!(Ptc6, super::sig::I2s0Mclk, 6);

pin!(PTC7, Ptc7, PORTC, Portc, _PTC7, PortPin, _PORTC, 7);
    alt_fn!(Ptc7, super::sig::Cmp0In1, 0);
    alt_fn!(Ptc7, super::sig::Ptc7, 1);
    alt_fn!(Ptc7, super::sig::Spi0Miso, 2);
    alt_fn!(Ptc7, super::sig::AudiousbSofOut, 3);
    alt_fn!(Ptc7, super::sig::I2s0RxFs, 4);
    alt_fn!(Ptc7, super::sig::Spi0Mosi, 5);

pin!(PTC8, Ptc8, PORTC, Portc, _PTC8, PortPin, _PORTC, 8);
    alt_fn!(Ptc8, super::sig::Cmp0In2, 0);
    alt_fn!(Ptc8, super::sig::Ptc8, 1);
    alt_fn!(Ptc8, super::sig::I2c0Scl, 2);
    alt_fn!(Ptc8, super::sig::Tpm0Ch4, 3);
    alt_fn!(Ptc8, super::sig::I2s0Mclk, 4);

pin!(PTC9, Ptc9, PORTC, Portc, _PTC9, PortPin, _PORTC, 9);
    alt_fn!(Ptc9, super::sig::Cmp0In3, 0);
    alt_fn!(Ptc9, super::sig::Ptc9, 1);
    alt_fn!(Ptc9, super::sig::I2c0Sda, 2);
    alt_fn!(Ptc9, super::sig::Tpm0Ch5, 3);
    alt_fn!(Ptc9, super::sig::I2s0RxBclk, 4);

pin!(PTC10, Ptc10, PORTC, Portc, _PTC10, PortPin, _PORTC, 10);
    alt_fn!(Ptc10, super::sig::Ptc10, 1);
    alt_fn!(Ptc10, super::sig::I2c1Scl, 2);
    alt_fn!(Ptc10, super::sig::I2s0RxFs, 4);

pin!(PTC11, Ptc11, PORTC, Portc, _PTC11, PortPin, _PORTC, 11);
    alt_fn!(Ptc11, super::sig::Ptc11, 1);
    alt_fn!(Ptc11, super::sig::I2c1Sda, 2);
    alt_fn!(Ptc11, super::sig::I2s0Rxd0, 4);

pin!(PTD0, Ptd0, PORTD, Portd, _PTD0, PortPin, _PORTD, 0);
    alt_fn!(Ptd0, super::sig::Ptd0, 1);
    alt_fn!(Ptd0, super::sig::Spi0Pcs0, 2);
    alt_fn!(Ptd0, super::sig::Tpm0Ch0, 4);

pin!(PTD1, Ptd1, PORTD, Portd, _PTD1, PortPin, _PORTD, 1);
    alt_fn!(Ptd1, super::sig::Adc0Se5b, 0);
    alt_fn!(Ptd1, super::sig::Ptd1, 1);
    alt_fn!(Ptd1, super::sig::Spi0Sck, 2);
    alt_fn!(Ptd1, super::sig::Tpm0Ch1, 4);

pin!(PTD2, Ptd2, PORTD, Portd, _PTD2, PortPin, _PORTD, 2);
    alt_fn!(Ptd2, super::sig::Ptd2, 1);
    alt_fn!(Ptd2, super::sig::Spi0Mosi, 2);
    alt_fn!(Ptd2, super::sig::Uart2Rx, 3);
    alt_fn!(Ptd2, super::sig::Tpm0Ch2, 4);
    alt_fn!(Ptd2, super::sig::Spi0Miso, 5);

pin!(PTD3, Ptd3, PORTD, Portd, _PTD3, PortPin, _PORTD, 3);
    alt_fn!(Ptd3, super::sig::Ptd3, 1);
    alt_fn!(Ptd3, super::sig::Spi0Miso, 2);
    alt_fn!(Ptd3, super::sig::Uart2Tx, 3);
    alt_fn!(Ptd3, super::sig::Tpm0Ch3, 4);
    alt_fn!(Ptd3, super::sig::Spi0Mosi, 5);

pin!(PTD4, Ptd4, PORTD, Portd, _PTD4, PortPin, _PORTD, 4);
    alt_fn!(Ptd4, super::sig::Ptd4, 1);
    alt_fn!(Ptd4, super::sig::Spi1Pcs0, 2);
    alt_fn!(Ptd4, super::sig::Uart2Rx, 3);
    alt_fn!(Ptd4, super::sig::Tpm0Ch4, 4);

pin!(PTD5, Ptd5, PORTD, Portd, _PTD5, PortPin, _PORTD, 5);
    alt_fn!(Ptd5, super::sig::Adc0Se6b, 0);
    alt_fn!(Ptd5, super::sig::Ptd5, 1);
    alt_fn!(Ptd5, super::sig::Spi1Sck, 2);
    alt_fn!(Ptd5, super::sig::Uart2Tx, 3);
    alt_fn!(Ptd5, super::sig::Tpm0Ch5, 4);

pin!(PTD6, Ptd6, PORTD, Portd, _PTD6, PortPin, _PORTD, 6);
    alt_fn!(Ptd6, super::sig::Adc0Se7b, 0);
    alt_fn!(Ptd6, super::sig::Ptd6, 1);
    alt_fn!(Ptd6, super::sig::Spi1Mosi, 2);
    alt_fn!(Ptd6, super::sig::Uart0Rx, 3);
    alt_fn!(Ptd6, super::sig::Spi1Miso, 5);

pin!(PTD7, Ptd7, PORTD, Portd, _PTD7, PortPin, _PORTD, 7);
    alt_fn!(Ptd7, super::sig::Ptd7, 1);
    alt_fn!(Ptd7, super::sig::Spi1Miso, 2);
    alt_fn!(Ptd7, super::sig::Uart0Tx, 3);
    alt_fn!(Ptd7, super::sig::Spi1Mosi, 5);

pin!(PTE0, Pte0, PORTE, Porte, _PTE0, PortPin, _PORTE, 0);
    alt_fn!(Pte0, super::sig::Pte0, 1);
    alt_fn!(Pte0, super::sig::Spi1Miso, 2);
    alt_fn!(Pte0, super::sig::Uart1Tx, 3);
    alt_fn!(Pte0, super::sig::RtcClkout, 4);
    alt_fn!(Pte0, super::sig::Cmp0Out, 5);
    alt_fn!(Pte0, super::sig::I2c1Sda, 6);

pin!(PTE1, Pte1, PORTE, Porte, _PTE1, PortPin, _PORTE, 1);
    alt_fn!(Pte1, super::sig::Pte1, 1);
    alt_fn!(Pte1, super::sig::Spi1Mosi, 2);
    alt_fn!(Pte1, super::sig::Uart1Rx, 3);
    alt_fn!(Pte1, super::sig::Spi1Miso, 5);
    alt_fn!(Pte1, super::sig::I2c1Scl, 6);

pin!(PTE20, Pte20, PORTE, Porte, _PTE20, PortPin, _PORTE, 20);
    alt_fn!(Pte20, super::sig::Adc0Dp0, 0);
    alt_fn!(Pte20, super::sig::Adc0Se0, 0);
    alt_fn!(Pte20, super::sig::Pte20, 1);
    alt_fn!(Pte20, super::sig::Tpm1Ch0, 3);
    alt_fn!(Pte20, super::sig::Uart0Tx, 4);

pin!(PTE21, Pte21, PORTE, Porte, _PTE21, PortPin, _PORTE, 21);
    alt_fn!(Pte21, super::sig::Adc0Dm0, 0);
    alt_fn!(Pte21, super::sig::Adc0Se4a, 0);
    alt_fn!(Pte21, super::sig::Pte21, 1);
    alt_fn!(Pte21, super::sig::Tpm1Ch1, 3);
    alt_fn!(Pte21, super::sig::Uart0Rx, 4);

pin!(PTE22, Pte22, PORTE, Porte, _PTE22, PortPin, _PORTE, 22);
    alt_fn!(Pte22, super::sig::Adc0Dp3, 0);
    alt_fn!(Pte22, super::sig::Adc0Se3, 0);
    alt_fn!(Pte22, super::sig::Pte22, 1);
    alt_fn!(Pte22, super::sig::Tpm2Ch0, 3);
    alt_fn!(Pte22, super::sig::Uart2Tx, 4);

pin!(PTE23, Pte23, PORTE, Porte, _PTE23, PortPin, _PORTE, 23);
    alt_fn!(Pte23, super::sig::Adc0Dm3, 0);
    alt_fn!(Pte23, super::sig::Adc0Se7a, 0);
    alt_fn!(Pte23, super::sig::Pte23, 1);
    alt_fn!(Pte23, super::sig::Tpm2Ch1, 3);
    alt_fn!(Pte23, super::sig::Uart2Rx, 4);

pin!(PTE24, Pte24, PORTE, Porte, _PTE24, PortPin, _PORTE, 24);
    alt_fn!(Pte24, super::sig::Pte24, 1);
    alt_fn!(Pte24, super::sig::Tpm0Ch0, 3);
    alt_fn!(Pte24, super::sig::I2c0Scl, 5);

pin!(PTE25, Pte25, PORTE, Porte, _PTE25, PortPin, _PORTE, 25);
    alt_fn!(Pte25, super::sig::Pte25, 1);
    alt_fn!(Pte25, super::sig::Tpm0Ch1, 3);
    alt_fn!(Pte25, super::sig::I2c0Sda, 5);

pin!(PTE29, Pte29, PORTE, Porte, _PTE29, PortPin, _PORTE, 29);
    alt_fn!(Pte29, super::sig::Cmp0In5, 0);
    alt_fn!(Pte29, super::sig::Adc0Se4b, 0);
    alt_fn!(Pte29, super::sig::Pte29, 1);
    alt_fn!(Pte29, super::sig::Tpm0Ch2, 3);
    alt_fn!(Pte29, super::sig::TpmClkin0, 4);

pin!(PTE30, Pte30, PORTE, Porte, _PTE30, PortPin, _PORTE, 30);
    alt_fn!(Pte30, super::sig::Dac0Out, 0);
    alt_fn!(Pte30, super::sig::Adc0Se23, 0);
    alt_fn!(Pte30, super::sig::Cmp0In4, 0);
    alt_fn!(Pte30, super::sig::Pte30, 1);
    alt_fn!(Pte30, super::sig::Tpm0Ch3, 3);
    alt_fn!(Pte30, super::sig::TpmClkin1, 4);

pin!(PTE31, Pte31, PORTE, Porte, _PTE31, PortPin, _PORTE, 31);
    alt_fn!(Pte31, super::sig::Pte31, 1);
    alt_fn!(Pte31, super::sig::Tpm0Ch4, 3);


