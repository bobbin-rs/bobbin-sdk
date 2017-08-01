use ::chip::sysctl::*;
use ::chip::gpio::*;
use ::chip::uart::*;
use ::chip::i2c::*;
use ::chip::ssi::*;
use ::chip::emac::*;

pub fn set_gpio_enabled(gpio: Gpio, value: bool) {    
    let value = if value { 1 } else { 0 };
    unsafe {
        match gpio {
            GPIOA => SYSCTL.with_rcgcgpio(|r| r.set_r0(value)),
            GPIOA_AHB => SYSCTL.with_rcgcgpio(|r| r.set_r0(value)),
            GPIOB => SYSCTL.with_rcgcgpio(|r| r.set_r1(value)),
            GPIOB_AHB => SYSCTL.with_rcgcgpio(|r| r.set_r1(value)),
            GPIOC => SYSCTL.with_rcgcgpio(|r| r.set_r2(value)),
            GPIOC_AHB => SYSCTL.with_rcgcgpio(|r| r.set_r2(value)),
            GPIOD => SYSCTL.with_rcgcgpio(|r| r.set_r3(value)),
            GPIOD_AHB => SYSCTL.with_rcgcgpio(|r| r.set_r3(value)),
            GPIOE => SYSCTL.with_rcgcgpio(|r| r.set_r4(value)),
            GPIOE_AHB => SYSCTL.with_rcgcgpio(|r| r.set_r4(value)),
            GPIOF => SYSCTL.with_rcgcgpio(|r| r.set_r5(value)),
            GPIOF_AHB => SYSCTL.with_rcgcgpio(|r| r.set_r5(value)),
            GPIOG => SYSCTL.with_rcgcgpio(|r| r.set_r6(value)),
            GPIOG_AHB => SYSCTL.with_rcgcgpio(|r| r.set_r6(value)),
            GPIOH => SYSCTL.with_rcgcgpio(|r| r.set_r7(value)),
            GPIOH_AHB => SYSCTL.with_rcgcgpio(|r| r.set_r7(value)),
            GPIOJ => SYSCTL.with_rcgcgpio(|r| r.set_r8(value)),
            GPIOJ_AHB => SYSCTL.with_rcgcgpio(|r| r.set_r8(value)),
            GPIOK => SYSCTL.with_rcgcgpio(|r| r.set_r9(value)),
            GPIOL => SYSCTL.with_rcgcgpio(|r| r.set_r10(value)),
            GPIOM => SYSCTL.with_rcgcgpio(|r| r.set_r11(value)),
            GPION => SYSCTL.with_rcgcgpio(|r| r.set_r12(value)),
            GPIOP => SYSCTL.with_rcgcgpio(|r| r.set_r13(value)),
            GPIOQ => SYSCTL.with_rcgcgpio(|r| r.set_r14(value)),
            _ => unimplemented!(),
        }
    }
}

pub fn set_uart_enabled(uart: Uart, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match uart {
            UART0 => SYSCTL.with_rcgcuart(|r| r.set_r0(value)),
            UART1 => SYSCTL.with_rcgcuart(|r| r.set_r1(value)),
            UART2 => SYSCTL.with_rcgcuart(|r| r.set_r2(value)),
            UART3 => SYSCTL.with_rcgcuart(|r| r.set_r3(value)),
            UART4 => SYSCTL.with_rcgcuart(|r| r.set_r4(value)),
            UART5 => SYSCTL.with_rcgcuart(|r| r.set_r5(value)),
            UART6 => SYSCTL.with_rcgcuart(|r| r.set_r6(value)),
            UART7 => SYSCTL.with_rcgcuart(|r| r.set_r7(value)),
            _ => unimplemented!(),
        }
    }    
}

pub fn set_i2c_enabled(i2c: I2c, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match i2c {
            I2C0 => SYSCTL.with_rcgci2c(|r| r.set_r0(value)),
            I2C1 => SYSCTL.with_rcgci2c(|r| r.set_r1(value)),
            I2C2 => SYSCTL.with_rcgci2c(|r| r.set_r2(value)),
            I2C3 => SYSCTL.with_rcgci2c(|r| r.set_r3(value)),
            _ => unimplemented!(),
        }
    }    
}


pub fn set_ssi_enabled(ssi: Ssi, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match ssi {
            SSI0 => SYSCTL.with_rcgcssi(|r| r.set_r0(value)),
            SSI1 => SYSCTL.with_rcgcssi(|r| r.set_r1(value)),
            SSI2 => SYSCTL.with_rcgcssi(|r| r.set_r2(value)),
            SSI3 => SYSCTL.with_rcgcssi(|r| r.set_r3(value)),
            _ => unimplemented!(),
        }
    }    
}

pub fn set_emac_enabled(emac: Emac, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match emac {
            EMAC0 => SYSCTL.with_rcgcemac(|r| r.set_r0(value)),
            _ => unimplemented!(),
        }
    }    
}


pub fn set_ephy_enabled(value: bool) {   
    let value = if value { 1 } else { 0 };    
    unsafe {
        SYSCTL.with_rcgcephy(|r| r.set_r0(value));
    }     
}        

pub fn set_ephy_power(value: bool) {   
    let value = if value { 1 } else { 0 };
    unsafe {
        SYSCTL.with_pcephy(|r| r.set_p0(value));
    }     
}  

pub fn emac_ready() -> bool {
    unsafe { SYSCTL.premac().r0() != 0 }
}

pub fn ephy_ready() -> bool {
    unsafe { SYSCTL.prephy().r0() != 0 }
}    