use ::chip::rcc::RCC;
use ::chip::usart::*;
use ::chip::gpio::*;
use ::chip::tim_gen::*;
use ::chip::lptim::*;
use ::chip::dma::*;
use ::chip::crc::*;
use ::chip::wwdg::*;
use ::chip::spi::*;
use ::chip::i2c::*;

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

pub fn gpio_index(gpio: Gpio) -> usize {
    match gpio {
        GPIOA => 0,
        GPIOB => 1,
        GPIOC => 2,
        GPIOH => 7,
        _ => unimplemented!(),
    }   
}

pub fn gpio_enabled(gpio: Gpio) -> bool {
    unsafe { RCC.iopenr().iopen(gpio_index(gpio)) != 0 }
    
}

pub fn set_gpio_enabled(gpio: Gpio, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_iopenr(|r| r.set_iopen(gpio_index(gpio), value)) }
}

pub fn set_pin_enabled(pin: (Gpio, usize), value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_iopenr(|r| r.set_iopen(gpio_index(pin.0), value)) }
}

pub fn set_pinfn_enabled(pinfn: (Gpio, usize, usize), value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_iopenr(|r| r.set_iopen(gpio_index(pinfn.0), value)) }
}

pub fn set_usart_enabled(usart: Usart, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match usart {
            USART1 => RCC.with_apb2enr(|r| r.set_usart1en(value)),
            USART2 => RCC.with_apb1enr(|r| r.set_usart2en(value)),
            _ => unimplemented!()
        }        
    }
}

pub fn set_tim_gen_enabled(tim: TimGen, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match tim {
            TIM2 => RCC.with_apb1enr(|r| r.set_tim2en(value)),
            TIM21 => RCC.with_apb2enr(|r| r.set_tim21en(value)),
            TIM22 => RCC.with_apb2enr(|r| r.set_tim22en(value)),
            _ => unimplemented!()
        }
    }
}

pub fn set_lptim_enabled(lptim: Lptim, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match lptim {
            LPTIM => RCC.with_apb1enr(|r| r.set_lptim1en(value)),
            _ => unimplemented!()
        }
    }
}

pub fn set_lptim_clock(lptim: Lptim, clksel: LPTimClock) {
    unsafe {
        match lptim {            
            LPTIM => RCC.with_ccipr(|r| r.set_lptim1sel(clksel as u32)),
            _ => unimplemented!()
        }
    }
}

pub fn set_pwr_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        RCC.with_apb1enr(|r| r.set_pwren(value));
    }
}

pub fn set_lse_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        RCC.with_csr(|r| r.set_lseon(value));
        if value != 0 {
            while RCC.csr().lserdy() == 0 {}
        }
    }
}

pub fn lse_ready() -> bool {
    unsafe {
        RCC.csr().lserdy() != 0
    }
}

pub fn reset_rtc() {
    unsafe {
        RCC.with_csr(|r| r.set_rtcrst(1));
        RCC.with_csr(|r| r.set_rtcrst(0));
    }
}

pub fn set_rtc_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        RCC.with_csr(|r| r.set_rtcen(value));
    }    
}

pub fn set_rtc_clock(clocksel: RtcClock) {
    unsafe {
        RCC.with_csr(|r| r.set_rtcsel(clocksel as u32))
    }

}

pub fn set_dma_enabled(dma: Dma, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match dma {
            DMA1 => RCC.with_ahbenr(|r| r.set_dmaen(value)),
            _ => unimplemented!()
        }
    }        
}

pub fn set_crc_enabled(crc: Crc, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match crc {
            CRC => RCC.with_ahbenr(|r| r.set_crcen(value)),
            _ => unimplemented!()
        }
    }
}

pub fn set_wwdg_enabled(wwdg: Wwdg, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match wwdg {
            WWDG => RCC.with_apb1enr(|r| r.set_wwdgen(value)),
            _ => unimplemented!()
        }
    }
}


pub fn set_spi_enabled(spi: Spi, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match spi {
            SPI1 => RCC.with_apb2enr(|r| r.set_spi1en(value)),
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
            _ => unimplemented!()
        }
    }
}


pub fn set_syscfg_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe { RCC.with_apb2enr(|r| r.set_syscfgen(value)); }
}

// pub fn set_dma1_enabled(value: bool) {
//     let value = if value { 1 } else { 0 };
//     unsafe { RCC.with_ahb1enr(|r| r.set_dma1en(value)); }
// }

// pub fn set_dma2_enabled(value: bool) {
//     let value = if value { 1 } else { 0 };
//     unsafe { RCC.with_ahb1enr(|r| r.set_dma2en(value)); }
// }

