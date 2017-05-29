use ::chip::rcc::RCC;
use ::chip::usart::*;
use ::chip::gpio::*;
use ::chip::i2c::*;
use ::chip::spi::*;
use ::chip::adc::*;
use ::chip::dac::*;
use ::chip::tim_gen::*;

pub fn gpio_index(gpio: Gpio) -> usize {
    match gpio {
        GPIOA => 0,
        GPIOB => 1,
        GPIOC => 2,
        GPIOD => 3,
        GPIOE => 4,
        GPIOF => 5,
        GPIOG => 6,
        GPIOH => 7,
        GPIOI => 8,
        GPIOJ => 9,
        GPIOK => 10,
        _ => unimplemented!(),
    }   
}

pub fn gpio_enabled(gpio: Gpio) -> bool {
    unsafe { RCC.ahb1enr().gpioen(gpio_index(gpio)) != 0 }
    
}

pub fn set_gpio_enabled(gpio: Gpio, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_ahb1enr(|r| r.set_gpioen(gpio_index(gpio), value)) }
}

pub fn set_usart_enabled(usart: Usart, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match usart {
            USART1 => RCC.with_apb2enr(|r| r.set_usart1en(value)),
            USART2 => RCC.with_apb1enr(|r| r.set_usart2en(value)),
            USART3 => RCC.with_apb1enr(|r| r.set_usart3en(value)),
            USART6 => RCC.with_apb2enr(|r| r.set_usart6en(value)),
            UART7 => RCC.with_apb1enr(|r| r.set_uart7enr(value)),
            UART8 => RCC.with_apb1enr(|r| r.set_uart8enr(value)),
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

pub fn set_spi_enabled(spi: Spi, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match spi {
            SPI1 => RCC.with_apb2enr(|r| r.set_spi1en(value)),
            SPI2 => RCC.with_apb1enr(|r| r.set_spi2en(value)),
            SPI3 => RCC.with_apb1enr(|r| r.set_spi3en(value)),
            SPI4 => RCC.with_apb2enr(|r| r.set_spi4enr(value)),
            SPI5 => RCC.with_apb2enr(|r| r.set_spi5enr(value)),
            SPI6 => RCC.with_apb2enr(|r| r.set_spi6enr(value)),
            _ => unimplemented!()
        }
    }
}

pub fn set_adc_enabled(adc: Adc, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match adc {
            ADC1 => RCC.with_apb2enr(|r| r.set_adc1en(value)),
            ADC2 => RCC.with_apb2enr(|r| r.set_adc2en(value)),
            ADC3 => RCC.with_apb2enr(|r| r.set_adc3en(value)),
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


pub fn set_tim_gen_enabled(tim: TimGen, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match tim {
            TIM2 => RCC.with_apb1enr(|r| r.set_tim2en(value)),
            TIM3 => RCC.with_apb1enr(|r| r.set_tim3en(value)),
            TIM4 => RCC.with_apb1enr(|r| r.set_tim4en(value)),
            TIM5 => RCC.with_apb1enr(|r| r.set_tim5en(value)),
            TIM9 => RCC.with_apb2enr(|r| r.set_tim9en(value)),
            TIM10 => RCC.with_apb2enr(|r| r.set_tim10en(value)),
            TIM11 => RCC.with_apb2enr(|r| r.set_tim11en(value)),
            TIM12 => RCC.with_apb1enr(|r| r.set_tim12en(value)),
            TIM13 => RCC.with_apb1enr(|r| r.set_tim13en(value)),
            TIM14 => RCC.with_apb1enr(|r| r.set_tim14en(value)),
            _ => unimplemented!()
        }
    }
}


pub fn ethmac_enabled() -> bool {
    unsafe { RCC.ahb1enr().ethmacen() != 0 }
}

pub fn set_ethmac_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_ahb1enr(|r| r.set_ethmacen(value)); }
}

pub fn ethmactx_enabled() -> bool {
    unsafe { RCC.ahb1enr().ethmactxen() != 0 }
}

pub fn set_ethmactx_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_ahb1enr(|r| r.set_ethmactxen(value)); }
}

pub fn ethmacrx_enabled() -> bool {
    unsafe { RCC.ahb1enr().ethmacrxen() != 0 }
}

pub fn set_ethmacrx_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_ahb1enr(|r| r.set_ethmacrxen(value)); }
}

pub fn ethmacptp_enabled() -> bool {
    unsafe { RCC.ahb1enr().ethmacptpen() != 0 }
}

pub fn set_ethmacptp_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_ahb1enr(|r| r.set_ethmacptpen(value)); }
}

pub fn set_syscfg_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_apb2enr(|r| r.set_syscfgen(value)); }
}

pub fn set_dma1_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_ahb1enr(|r| r.set_dma1en(value)); }
}

pub fn set_dma2_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_ahb1enr(|r| r.set_dma2en(value)); }
}

