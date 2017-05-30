pub use ::chip::rcc::*;
// use ::chip::usart::*;
// use ::chip::gpio::*;
// use ::chip::tim_gen::*;
// use ::chip::lptim::*;
// use ::chip::dma::*;
// use ::chip::crc::*;
// use ::chip::wwdg::*;
// use ::chip::spi::*;
// use ::chip::i2c::*;

pub enum LPTimClock {
    APB = 0b00,
    LSI = 0b01,
    HSI16 = 0b10,
    LSE = 0b11,
}

pub enum RtcClock {
    None = 0b00,
    LSE = 0b01,
    LSI = 0b10,
    HSE = 0b11,
}

pub trait RccExt {
    fn enabled<P: En>(&self, p: &P) -> bool;
    fn set_enabled<P: En>(&self, p: &P, value: bool) -> &Self;
}

impl RccExt for Rcc {
    fn enabled<P: En>(&self, p: &P) -> bool {
        self.en(p) != 0
    }
    fn set_enabled<P: En>(&self, p: &P, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.set_en(p, value);
        self
    }
    
}


// pub fn gpio_index(gpio: &GpioImpl) -> usize {
//     match gpio {
//         &GPIOA_IMPL => 0,
//         &GPIOB_IMPL => 1,
//         &GPIOC_IMPL => 2,
//         &GPIOH_IMPL => 7,
//         _ => unimplemented!(),
//     }
// }

// pub fn gpio_enabled(gpio: &GpioImpl) -> bool {
//     RCC.iopenr().iopen(gpio_index(gpio)) != 0    
// }

// pub fn set_gpio_enabled(gpio: &GpioImpl, value: bool) {
//     let value = if value { 1 } else { 0 };
//     RCC.with_iopenr(|r| r.set_iopen(gpio_index(gpio), value));
// }

// pub fn set_pin_enabled(pin: (&GpioImpl, usize), value: bool) {
//     let value = if value { 1 } else { 0 };
//     RCC.with_iopenr(|r| r.set_iopen(gpio_index(pin.0), value));
// }

// pub fn set_pinfn_enabled(pinfn: (&GpioImpl, usize, usize), value: bool) {
//     let value = if value { 1 } else { 0 };
//     RCC.with_iopenr(|r| r.set_iopen(gpio_index(pinfn.0), value));
// }

// pub fn set_usart_enabled(usart: &UsartImpl, value: bool) {
//     let value = if value { 1 } else { 0 };
//     match usart {
//         &USART1_IMPL => { RCC.with_apb2enr(|r| r.set_usart1en(value)); },
//         &USART2_IMPL => { RCC.with_apb1enr(|r| r.set_usart2en(value)); },
//         _ => unimplemented!()
//     };
// }

// pub fn set_tim_gen_enabled(tim: &TimGenImpl, value: bool) {
//     let value = if value { 1 } else { 0 };
//     match tim {
//         &TIM2_IMPL => { RCC.with_apb1enr(|r| r.set_tim2en(value)); },
//         &TIM21_IMPL => { RCC.with_apb2enr(|r| r.set_tim21en(value)); },
//         &TIM22_IMPL => { RCC.with_apb2enr(|r| r.set_tim22en(value)); },
//         _ => unimplemented!()
//     };
// }

// pub fn set_lptim_enabled(lptim: Lptim, value: bool) {
//     let value = if value { 1 } else { 0 };
//     match lptim {
//         LPTIM => { RCC.with_apb1enr(|r| r.set_lptim1en(value)); },
//         _ => unimplemented!()
//     };
// }

// pub fn set_lptim_clock(lptim: Lptim, clksel: LPTimClock) {
//     match lptim {            
//         LPTIM => { RCC.with_ccipr(|r| r.set_lptim1sel(clksel as u32)); },
//         _ => unimplemented!()
//     };
// }

// pub fn set_pwr_enabled(value: bool) {
//     let value = if value { 1 } else { 0 };
//     RCC.with_apb1enr(|r| r.set_pwren(value));
// }

// pub fn set_lse_enabled(value: bool) {
//     let value = if value { 1 } else { 0 };
//     RCC.with_csr(|r| r.set_lseon(value));
//     if value != 0 {
//         while RCC.csr().lserdy() == 0 {}
//     }
// }

// pub fn lse_ready() -> bool {
//     RCC.csr().lserdy() != 0
// }

// pub fn reset_rtc() {
//     RCC.with_csr(|r| r.set_rtcrst(1));
//     RCC.with_csr(|r| r.set_rtcrst(0));
// }

// pub fn set_rtc_enabled(value: bool) {
//     let value = if value { 1 } else { 0 };
//     RCC.with_csr(|r| r.set_rtcen(value));
// }

// pub fn set_rtc_clock(clocksel: RtcClock) {
//     RCC.with_csr(|r| r.set_rtcsel(clocksel as u32));
// }

// pub fn set_dma_enabled(dma: Dma, value: bool) {
//     let value = if value { 1 } else { 0 };
//     match dma {
//         DMA1 => { RCC.with_ahbenr(|r| r.set_dmaen(value)); },
//         _ => unimplemented!()
//     };
// }

// pub fn set_crc_enabled(crc: Crc, value: bool) {
//     let value = if value { 1 } else { 0 };
//     match crc {
//         CRC => { RCC.with_ahbenr(|r| r.set_crcen(value)); },
//         _ => unimplemented!()
//     };
// }

// pub fn set_wwdg_enabled(wwdg: Wwdg, value: bool) {
//     let value = if value { 1 } else { 0 };
//     match wwdg {
//         WWDG => { RCC.with_apb1enr(|r| r.set_wwdgen(value)); },
//         _ => unimplemented!()
//     };
// }


// pub fn set_spi_enabled(spi: SpiImpl, value: bool) {
//     let value = if value { 1 } else { 0 };
//     match spi {
//         SPI1_IMPL => { RCC.with_apb2enr(|r| r.set_spi1en(value)); },
//         _ => unimplemented!()
//     };
// }

// pub fn set_i2c_enabled(i2c: I2cImpl, value: bool) {
//     let value = if value { 1 } else { 0 };
//     match i2c {
//         I2C1_IMPL => { RCC.with_apb1enr(|r| r.set_i2c1en(value)); },
//         I2C2_IMPL => { RCC.with_apb1enr(|r| r.set_i2c2en(value)); },
//         _ => unimplemented!()
//     };
// }


// pub fn set_syscfg_enabled(value: bool) {
//     let value = if value { 1 } else { 0 };
//     RCC.with_apb2enr(|r| r.set_syscfgen(value));
// }

// // pub fn set_dma1_enabled(value: bool) {
// //     let value = if value { 1 } else { 0 };
// //     unsafe { RCC.with_ahb1enr(|r| r.set_dma1en(value)); }
// // }

// // pub fn set_dma2_enabled(value: bool) {
// //     let value = if value { 1 } else { 0 };
// //     unsafe { RCC.with_ahb1enr(|r| r.set_dma2en(value)); }
// // }

