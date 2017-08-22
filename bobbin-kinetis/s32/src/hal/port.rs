pub use chip::port::*;    
pub use kinetis_common::hal::port::*;
pub use super::pcc::PccEnabled;
use chip::gpio;
use chip::sig::{SignalTx, SignalRx, SignalFtm, SignalAdc};
use chip::sig::{SignalSpiSck, SignalSpiSin, SignalSpiSout};
use chip::sig::{SignalSpiPcs0, SignalSpiPcs1, SignalSpiPcs2, SignalSpiPcs3};

pub trait GpioPin<PIN_ID, GPIO_ID> {
    fn gpio_pin(&self) -> gpio::Pin<PIN_ID, GPIO_ID>;
}

impl<PIN_ID, PORT_ID, GPIO_ID> GpioPin<PIN_ID, GPIO_ID> for Pin<PIN_ID, PORT_ID>
    where PIN_ID: Copy, Periph<PORT_ID>: LinkGpio<gpio::Periph<GPIO_ID>> {
    fn gpio_pin(&self) -> gpio::Pin<PIN_ID, GPIO_ID> {
        gpio::Pin {
            port: self.port.gpio(),
            index: self.index,
            id: self.id,
        }
    }
}

pub trait ModeTx<T, S> {
    fn mode_tx(&self, _: &S) -> &Self;
}

pub trait ModeRx<T, S> {
    fn mode_rx(&self, _: &S) -> &Self;
}

pub trait ModeFtm<T, S> {
    fn mode_ftm(&self, _: &S) -> &Self;
}    

pub trait ModeAdc<T, S> {
    fn mode_adc(&self, _: &S) -> &Self;
}    

pub trait ModeSpiSck<T, S> {
    fn mode_spi_sck(&self, _: &S) -> &Self;
}    

pub trait ModeSpiSin<T, S> {
    fn mode_spi_sin(&self, _: &S) -> &Self;
}    

pub trait ModeSpiSout<T, S> {
    fn mode_spi_sout(&self, _: &S) -> &Self;
}    

pub trait ModeSpiPcs0<T, S> {
    fn mode_spi_pcs0(&self, _: &S) -> &Self;
}    

pub trait ModeSpiPcs1<T, S> {
    fn mode_spi_pcs1(&self, _: &S) -> &Self;
}    

pub trait ModeSpiPcs2<T, S> {
    fn mode_spi_pcs2(&self, _: &S) -> &Self;
}    

pub trait ModeSpiPcs3<T, S> {
    fn mode_spi_pcs3(&self, _: &S) -> &Self;
}    

impl<P, O, S, T> ModeTx<T, S> for Pin<P, O> where S: SignalTx<T>, P: AltFn<T> {
    fn mode_tx(&self, _: &S) -> &Self {
        self.set_mux(self.id.alt_fn());
        self
    }
}

impl<P, O, S, T> ModeRx<T, S> for Pin<P, O> where S: SignalRx<T>, P: AltFn<T> {
    fn mode_rx(&self, _: &S) -> &Self {
        self.set_mux(self.id.alt_fn());
        self
    }
}

impl<P, O, S, T> ModeFtm<T, S> for Pin<P, O> where S: SignalFtm<T>, P: AltFn<T> {
    fn mode_ftm(&self, _: &S) -> &Self {
        self.set_mux(self.id.alt_fn());
        self
    }
}          

impl<P, O, S, T> ModeAdc<T, S> for Pin<P, O> where S: SignalAdc<T>, P: AltFn<T> {
    fn mode_adc(&self, _: &S) -> &Self {
        self.set_mux(self.id.alt_fn());
        self
    }
}         

impl<P, O, S, T> ModeSpiSck<T, S> for Pin<P, O> where S: SignalSpiSck<T>, P: AltFn<T> {
    fn mode_spi_sck(&self, _: &S) -> &Self {
        self.set_mux(self.id.alt_fn());
        self
    }
}         

impl<P, O, S, T> ModeSpiSout<T, S> for Pin<P, O> where S: SignalSpiSout<T>, P: AltFn<T> {
    fn mode_spi_sout(&self, _: &S) -> &Self {
        self.set_mux(self.id.alt_fn());
        self
    }
}

impl<P, O, S, T> ModeSpiSin<T, S> for Pin<P, O> where S: SignalSpiSin<T>, P: AltFn<T> {
    fn mode_spi_sin(&self, _: &S) -> &Self {
        self.set_mux(self.id.alt_fn());
        self
    }
}         

impl<P, O, S, T> ModeSpiPcs0<T, S> for Pin<P, O> where S: SignalSpiPcs0<T>, P: AltFn<T> {
    fn mode_spi_pcs0(&self, _: &S) -> &Self {
        self.set_mux(self.id.alt_fn());
        self
    }
}         

impl<P, O, S, T> ModeSpiPcs1<T, S> for Pin<P, O> where S: SignalSpiPcs1<T>, P: AltFn<T> {
    fn mode_spi_pcs1(&self, _: &S) -> &Self {
        self.set_mux(self.id.alt_fn());
        self
    }
}         

impl<P, O, S, T> ModeSpiPcs2<T, S> for Pin<P, O> where S: SignalSpiPcs2<T>, P: AltFn<T> {
    fn mode_spi_pcs2(&self, _: &S) -> &Self {
        self.set_mux(self.id.alt_fn());
        self
    }
}         

impl<P, O, S, T> ModeSpiPcs3<T, S> for Pin<P, O> where S: SignalSpiPcs3<T>, P: AltFn<T> {
    fn mode_spi_pcs3(&self, _: &S) -> &Self {
        self.set_mux(self.id.alt_fn());
        self
    }
}      
