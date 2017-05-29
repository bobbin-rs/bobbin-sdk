use ::chip::rcc::RCC;
use ::chip::gpio::*;
use ::chip::usart::*;
use ::chip::i2c::*;
use ::chip::spi::*;
use ::chip::adc::*;
use ::chip::dac::*;
use ::chip::dma::*;
use ::chip::tim_bas::*;
use ::chip::tim_gen::*;
use ::chip::tim_adv::*;

pub enum RtcClock {
    None = 0b00,
    LSE = 0b01,
    LSI = 0b10,
    HSE = 0b11,
}

pub fn gpio_enabled(gpio: Gpio) -> bool {
    unsafe { 
        match gpio {
            GPIOA => RCC.ahbenr().iopaen() != 0,
            GPIOB => RCC.ahbenr().iopben() != 0,
            GPIOC => RCC.ahbenr().iopcen() != 0,
            GPIOD => RCC.ahbenr().iopden() != 0,
            GPIOE => RCC.ahbenr().iopeen() != 0,
            GPIOF => RCC.ahbenr().iopfen() != 0,
            GPIOG => RCC.ahbenr().iopgen() != 0,
            GPIOH => RCC.ahbenr().iophen() != 0,            
            _ => unimplemented!(),
        }
    }    
}

pub fn set_gpio_enabled(gpio: Gpio, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { 
        match gpio {
            GPIOA => RCC.with_ahbenr(|r| r.set_iopaen(value)),
            GPIOB => RCC.with_ahbenr(|r| r.set_iopben(value)),
            GPIOC => RCC.with_ahbenr(|r| r.set_iopcen(value)),
            GPIOD => RCC.with_ahbenr(|r| r.set_iopden(value)),
            GPIOE => RCC.with_ahbenr(|r| r.set_iopeen(value)),
            GPIOF => RCC.with_ahbenr(|r| r.set_iopfen(value)),
            GPIOG => RCC.with_ahbenr(|r| r.set_iopgen(value)),
            GPIOH => RCC.with_ahbenr(|r| r.set_iophen(value)),
            _ => unimplemented!(),
        }
    }
}

pub fn set_usart_enabled(usart: Usart, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match usart {
            USART1 => RCC.with_apb2enr(|r| r.set_usart1en(value)),
            USART2 => RCC.with_apb1enr(|r| r.set_usart2en(value)),
            USART3 => RCC.with_apb1enr(|r| r.set_usart3en(value)),
            UART4 => RCC.with_apb1enr(|r| r.set_uart4en(value)),
            UART5 => RCC.with_apb1enr(|r| r.set_uart5en(value)),
            _ => unimplemented!()
        }
    }
}
 
pub enum UsartClock {
    Pclk = 0b00,
    SysClk = 0b01,
    LSE = 0b10,
    HSI = 0b11,
}

pub fn set_usart_clock(usart: Usart, value: UsartClock) {
    unsafe {
        match usart {
            USART1 => RCC.with_cfgr3(|r| r.set_usart1sw(value as u32)),
            USART2 => RCC.with_cfgr3(|r| r.set_usart2sw(value as u32)),
            USART3 => RCC.with_cfgr3(|r| r.set_usart3sw(value as u32)),
            UART4 => RCC.with_cfgr3(|r| r.set_uart4sw(value as u32)),
            UART5 => RCC.with_cfgr3(|r| r.set_uart5sw(value as u32)),
            _ => unimplemented!()
        }
    }
}

pub fn set_i2c_enabled(i2c: I2c, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match i2c {
            I2C1 => RCC.with_apb1enr(|r| r.set_i2c1en(value)),
            I2C2 => RCC.with_apb1enr(|r| r.set_i2c2en(value)),
            I2C3 => RCC.with_apb1enr(|r| r.set_i2c3en(value)),
            _ => unimplemented!()
        }
    }
}

pub enum I2cClock {
    Hsi = 0b0,
    SysClk = 0b1,
}

pub fn set_i2c_clock(i2c: I2c, value: I2cClock) {
    unsafe {
        match i2c {
            I2C1 => RCC.with_cfgr3(|r| r.set_i2c1sw(value as u32)),
            I2C2 => RCC.with_cfgr3(|r| r.set_i2c2sw(value as u32)),
            I2C3 => RCC.with_cfgr3(|r| r.set_i2c3sw(value as u32)),
            _ => unimplemented!()
        }
    }
}

pub fn set_spi_enabled(spi: Spi, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match spi {
            SPI1 => RCC.with_apb2enr(|r| r.set_spi1en(value)),
            SPI2 => RCC.with_apb1enr(|r| r.set_spi2en(value)),
            SPI3 => RCC.with_apb1enr(|r| r.set_spi3en(value)),
            _ => unimplemented!()
        }
    }
}

pub fn set_adc_enabled(adc: Adc, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match adc {
            ADC1 => RCC.with_ahbenr(|r| r.set_adc12en(value)),
            ADC2 => RCC.with_ahbenr(|r| r.set_adc12en(value)),
            ADC3 => RCC.with_ahbenr(|r| r.set_adc34en(value)),
            ADC4 => RCC.with_ahbenr(|r| r.set_adc34en(value)),
            _ => unimplemented!()
        }
    }
}

pub fn set_dac_enabled(dac: Dac, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match dac {
            DAC => RCC.with_apb1enr(|r| r.set_dacen(value)),
            _ => unimplemented!()
        }
    }
}


pub fn set_tim_bas_enabled(tim: TimBas, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match tim {
            TIM6 => RCC.with_apb1enr(|r| r.set_tim6en(value)),
            TIM7 => RCC.with_apb1enr(|r| r.set_tim7en(value)),
            _ => unimplemented!()
        }
    }
}

pub fn set_tim_gen_enabled(tim: TimGen, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match tim {
            TIM2 => RCC.with_apb1enr(|r| r.set_tim2en(value)),
            TIM3 => RCC.with_apb1enr(|r| r.set_tim3en(value)),
            TIM4 => RCC.with_apb1enr(|r| r.set_tim4en(value)),
            TIM15 => RCC.with_apb2enr(|r| r.set_tim15en(value)),
            TIM16 => RCC.with_apb2enr(|r| r.set_tim16en(value)),
            TIM17 => RCC.with_apb2enr(|r| r.set_tim17en(value)),
            _ => unimplemented!()
        }
    }
}

pub fn set_tim_adv_enabled(tim: TimAdv, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match tim {
            TIM1 => RCC.with_apb2enr(|r| r.set_tim1en(value)),
            TIM8 => RCC.with_apb2enr(|r| r.set_tim8en(value)),
            TIM20 => RCC.with_apb2enr(|r| r.set_tim20en(value)),
            _ => unimplemented!()
        }
    }
}

pub fn set_syscfg_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_apb2enr(|r| r.set_syscfgen(value)); }
}

pub fn set_wwdg_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_apb1enr(|r| r.set_wwdgen(value)); }
}

pub fn set_crc_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_ahbenr(|r| r.set_crcen(value)); }
}

pub fn set_pwr_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_apb1enr(|r| r.set_pwren(value)); }
}


pub fn set_dma_enabled(dma: Dma, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { 
        match dma {
            DMA1 => RCC.with_ahbenr(|r| r.set_dmaen(value)),
            DMA2 => RCC.with_ahbenr(|r| r.set_dma2en(value)),
            _ => unimplemented!(),
        }
    }
}

pub fn set_rtc_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        RCC.with_bdcr(|r| r.set_rtcen(value));
    }    
}


pub fn set_lse_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        RCC.with_bdcr(|r| r.set_lseon(value));
        if value != 0 {
            while RCC.bdcr().lserdy() == 0 {}
        }
    }
}

pub fn lse_ready() -> bool {
    unsafe {
        RCC.bdcr().lserdy() != 0
    }
}

pub fn set_rtc_clock(clocksel: RtcClock) {
    unsafe {
        RCC.with_bdcr(|r| r.set_rtcsel(clocksel as u32))
    }

}