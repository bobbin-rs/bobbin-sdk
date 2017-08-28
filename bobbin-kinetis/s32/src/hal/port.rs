pub use bobbin_common::{Pin, AltFn};
pub use chip::port::*;    
pub use kinetis_common::hal::port::*;
pub use super::pcc::PccEnabled;
use chip::gpio;
use chip::sig::{SignalTx, SignalRx, SignalFtm, SignalAdc};
use chip::sig::{SignalSpiSck, SignalSpiSin, SignalSpiSout};
use chip::sig::{SignalSpiPcs0, SignalSpiPcs1, SignalSpiPcs2, SignalSpiPcs3};

use core::ops::Deref;

// pub trait GpioPin<T> {
//     fn gpio_pin(&self) -> T;
// }

// impl<PIN_ID, PORT_ID, GPIO_ID> GpioPin<PIN_ID, GPIO_ID> for Pin<PIN_ID, PORT_ID>
//     where PIN_ID: Copy, Periph<PORT_ID>: LinkGpio<gpio::Periph<GPIO_ID>> {
//     fn gpio_pin(&self) -> gpio::Pin<PIN_ID, GPIO_ID> {
//         gpio::Pin {
//             port: self.port.gpio(),
//             index: self.index,
//             id: self.id,
//         }
//     }
// }

pub trait ModeTx<SIG, PERIPH> {
    fn mode_tx(&self, _: &PERIPH) -> &Self;
}

pub trait ModeRx<SIG, PERIPH> {
    fn mode_rx(&self, _: &PERIPH) -> &Self;
}

pub trait ModeFtm<SIG, PERIPH> {
    fn mode_ftm(&self, _: &PERIPH) -> &Self;
}    

pub trait ModeAdc<SIG, PERIPH> {
    fn mode_adc(&self, _: &PERIPH) -> &Self;
}    

pub trait ModeSpiSck<SIG, PERIPH> {
    fn mode_spi_sck(&self, _: &PERIPH) -> &Self;
}    

pub trait ModeSpiSin<SIG, PERIPH> {
    fn mode_spi_sin(&self, _: &PERIPH) -> &Self;
}    

pub trait ModeSpiSout<SIG, PERIPH> {
    fn mode_spi_sout(&self, _: &PERIPH) -> &Self;
}    

pub trait ModeSpiPcs0<SIG, PERIPH> {
    fn mode_spi_pcs0(&self, _: &PERIPH) -> &Self;
}    

pub trait ModeSpiPcs1<SIG, PERIPH> {
    fn mode_spi_pcs1(&self, _: &PERIPH) -> &Self;
}    

pub trait ModeSpiPcs2<SIG, PERIPH> {
    fn mode_spi_pcs2(&self, _: &PERIPH) -> &Self;
}    

pub trait ModeSpiPcs3<SIG, PERIPH> {
    fn mode_spi_pcs3(&self, _: &PERIPH) -> &Self;
}    

impl<PERIPH, PIN, SIG> ModeTx<SIG, PERIPH> for PIN where PERIPH: SignalTx<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    fn mode_tx(&self, _: &PERIPH) -> &Self {
        self.set_mux(self.alt_fn());
        self
    }
}

impl<PERIPH, PIN, SIG> ModeRx<SIG, PERIPH> for PIN where PERIPH: SignalRx<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    fn mode_rx(&self, _: &PERIPH) -> &Self {
        self.set_mux(self.alt_fn());
        self
    }
}

impl<PERIPH, PIN, SIG> ModeFtm<SIG, PERIPH> for PIN where PERIPH: SignalFtm<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    fn mode_ftm(&self, _: &PERIPH) -> &Self {
        self.set_mux(self.alt_fn());
        self
    }
}          

impl<PERIPH, PIN, SIG> ModeAdc<SIG, PERIPH> for PIN where PERIPH: SignalAdc<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    fn mode_adc(&self, _: &PERIPH) -> &Self {
        self.set_mux(self.alt_fn());
        self
    }
}         

impl<PERIPH, PIN, SIG> ModeSpiSck<SIG, PERIPH> for PIN where PERIPH: SignalSpiSck<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    fn mode_spi_sck(&self, _: &PERIPH) -> &Self {
        self.set_mux(self.alt_fn());
        self
    }
}         

impl<PERIPH, PIN, SIG> ModeSpiSout<SIG, PERIPH> for PIN where PERIPH: SignalSpiSout<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    fn mode_spi_sout(&self, _: &PERIPH) -> &Self {
        self.set_mux(self.alt_fn());
        self
    }
}

impl<PERIPH, PIN, SIG> ModeSpiSin<SIG, PERIPH> for PIN where PERIPH: SignalSpiSin<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    fn mode_spi_sin(&self, _: &PERIPH) -> &Self {
        self.set_mux(self.alt_fn());
        self
    }
}         

impl<PERIPH, PIN, SIG> ModeSpiPcs0<SIG, PERIPH> for PIN where PERIPH: SignalSpiPcs0<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    fn mode_spi_pcs0(&self, _: &PERIPH) -> &Self {
        self.set_mux(self.alt_fn());
        self
    }
}         

impl<PERIPH, PIN, SIG> ModeSpiPcs1<SIG, PERIPH> for PIN where PERIPH: SignalSpiPcs1<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    fn mode_spi_pcs1(&self, _: &PERIPH) -> &Self {
        self.set_mux(self.alt_fn());
        self
    }
}         

impl<PERIPH, PIN, SIG> ModeSpiPcs2<SIG, PERIPH> for PIN where PERIPH: SignalSpiPcs2<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    fn mode_spi_pcs2(&self, _: &PERIPH) -> &Self {
        self.set_mux(self.alt_fn());
        self
    }
}         

impl<PERIPH, PIN, SIG> ModeSpiPcs3<SIG, PERIPH> for PIN where PERIPH: SignalSpiPcs3<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=PortPin> {
    fn mode_spi_pcs3(&self, _: &PERIPH) -> &Self {
        self.set_mux(self.alt_fn());
        self
    }
}      
