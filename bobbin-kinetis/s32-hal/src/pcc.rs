use ::chip::pcc::*;
use ::chip::port::*;
use ::chip::lpuart::*;
use ::chip::can::*;
use ::chip::lpspi::*;
// use ::chip::i2c::*;
// use ::chip::spi::*;
// use ::chip::enet::*;

pub enum Source {
    Disabled = 0b000,
    SOSCDIV2 = 0b001,
    SIRCDIV2 = 0b010,
    FIRCDIV2 = 0b011,
    SPLLDIV2 = 0b110,
}

pub fn set_port_enabled(port: Port, value: bool) {    
    let value = if value { 1 } else { 0 };
    unsafe {
        match port {
            PORTA => PCC.set_porta(Porta(0).set_pr(1).set_cgc(value)),
            PORTB => PCC.set_portb(Portb(0).set_pr(1).set_cgc(value)),
            PORTC => PCC.set_portc(Portc(0).set_pr(1).set_cgc(value)),
            PORTD => PCC.set_portd(Portd(0).set_pr(1).set_cgc(value)),
            PORTE => PCC.set_porte(Porte(0).set_pr(1).set_cgc(value)),
            _ => unimplemented!()
        }

    }
}

pub fn set_lpuart_enabled(lpuart: Lpuart, value: bool, source: Source) {    
    let value = if value { 1 } else { 0 };
    unsafe {
        match lpuart {
            LPUART0 => PCC.with_lpuart0(|r| r.set_cgc(value).set_pcs(source as u32)),
            LPUART1 => PCC.with_lpuart1(|r| r.set_cgc(value).set_pcs(source as u32)),
            LPUART2 => PCC.with_lpuart2(|r| r.set_cgc(value).set_pcs(source as u32)),
            _ => unimplemented!()
        }
    }
}

pub fn set_lpit_enabled(value: bool, source: Source) {    
    let value = if value { 1 } else { 0 };
    unsafe {
        PCC.with_lpit(|r| r.set_cgc(value).set_pcs(source as u32));
    }
}

pub fn set_can_enabled(can: Can, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match can {
            CAN0 => PCC.with_flexcan0(|r| r.set_cgc(value)),
            CAN1 => PCC.with_flexcan1(|r| r.set_cgc(value)),
            CAN2 => PCC.with_flexcan2(|r| r.set_cgc(value)),
            _ => unimplemented!()
        }
    }
}

pub fn set_lpspi_enabled(lpspi: Lpspi, value: bool, source: Source) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match lpspi {
            LPSPI0 => PCC.with_lpspi0(|r| r.set_cgc(value).set_pcs(source as u32)),
            LPSPI1 => PCC.with_lpspi1(|r| r.set_cgc(value).set_pcs(source as u32)),
            LPSPI2 => PCC.with_lpspi2(|r| r.set_cgc(value).set_pcs(source as u32)),
            _ => unimplemented!()
        }
    }
}

