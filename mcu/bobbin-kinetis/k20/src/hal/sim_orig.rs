use ::chip::sim::*;
use ::chip::port::*;
use ::chip::uart::*;
use ::chip::i2c::*;
use ::chip::spi::*;
use ::chip::enet::*;

pub fn set_port_enabled(port: Port, value: bool) {    
    let value = if value { 1 } else { 0 };
    unsafe {
        match port {
            PORTA => SIM.with_scgc5(|r| r.set_porta(value)),
            PORTB => SIM.with_scgc5(|r| r.set_portb(value)),
            PORTC => SIM.with_scgc5(|r| r.set_portc(value)),
            PORTD => SIM.with_scgc5(|r| r.set_portd(value)),
            PORTE => SIM.with_scgc5(|r| r.set_porte(value)),
            _ => unimplemented!()
        }

    }
}


pub fn set_uart_enabled(uart: Uart, value: bool) {    
    let value = if value { 1 } else { 0 };
    unsafe {
        match uart {
            UART0 => SIM.with_scgc4(|r| r.set_uart0(value)),
            UART1 => SIM.with_scgc4(|r| r.set_uart1(value)),
            UART2 => SIM.with_scgc4(|r| r.set_uart2(value)),
            UART3 => SIM.with_scgc4(|r| r.set_uart3(value)),
            UART4 => SIM.with_scgc1(|r| r.set_uart4(value)),
            UART5 => SIM.with_scgc1(|r| r.set_uart5(value)),
            _ => unimplemented!()
        }

    }
}

pub fn set_i2c_enabled(i2c: I2c, value: bool) {    
    let value = if value { 1 } else { 0 };
    unsafe {
        match i2c {
            I2C0 => SIM.with_scgc4(|r| r.set_i2c0(value)),
            I2C1 => SIM.with_scgc4(|r| r.set_i2c1(value)),
            _ => unimplemented!()
        }

    }
}

pub fn set_spi_enabled(spi: Spi, value: bool) {    
    let value = if value { 1 } else { 0 };
    unsafe {
        match spi {
            SPI0 => SIM.with_scgc6(|r| r.set_spi0(value)),
            SPI1 => SIM.with_scgc6(|r| r.set_spi1(value)),
            SPI2 => SIM.with_scgc3(|r| r.set_spi2(value)),
            _ => unimplemented!()
        }

    }
}


pub fn set_enet_enabled(enet: Enet, value: bool) {    
    let value = if value { 1 } else { 0 };
    unsafe {
        match enet {
            ENET => SIM.with_scgc2(|r| r.set_enet(value)),
            _ => unimplemented!()
        }
    }
}

pub fn set_pit_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SIM.with_scgc6(|r| r.set_pit(value))
    }
}