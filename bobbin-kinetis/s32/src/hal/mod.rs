#![allow(non_camel_case_types)]

pub use bobbin_cortexm::hal::*;

pub mod clock;
pub mod pcc;
pub mod wdog;
pub mod adc;

pub mod port {
    pub use chip::port::*;    
    pub use kinetis_common::hal::port::*;
    pub use super::pcc::PccEnabled;
    use chip::gpio;
    use chip::sig::{SignalTx, SignalRx, SignalFtm, SignalAdc};

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

    // macro_rules! impl_adc_in {
    //     ($mode:ident, $fn:ident, $sig:ident) => (
    //         pub trait $mode<T, S> {
    //             fn $fn(&self, _: &S) -> &Self;
    //         }            

    //         impl<P, O, S, T> $mode<T, S> for Pin<P, O> where S: $sig<T>, P: AltFn<T> {
    //             fn $fn(&self, _: &S) -> &Self {
    //                 self.set_mux(self.id.alt_fn());
    //                 self
    //             }
    //         }                        
    //     )
    // }

    // impl_adc_in!(ModeAdcIn0, mode_adc_in_0, SignalAdcIn0);
    // impl_adc_in!(ModeAdcIn1, mode_adc_in_1, SignalAdcIn1);
    // impl_adc_in!(ModeAdcIn2, mode_adc_in_2, SignalAdcIn2);
    // impl_adc_in!(ModeAdcIn3, mode_adc_in_3, SignalAdcIn3);
    // impl_adc_in!(ModeAdcIn4, mode_adc_in_4, SignalAdcIn4);
    // impl_adc_in!(ModeAdcIn5, mode_adc_in_5, SignalAdcIn5);
    // impl_adc_in!(ModeAdcIn6, mode_adc_in_6, SignalAdcIn6);
    // impl_adc_in!(ModeAdcIn7, mode_adc_in_7, SignalAdcIn7);
    // impl_adc_in!(ModeAdcIn8, mode_adc_in_8, SignalAdcIn8);
    // impl_adc_in!(ModeAdcIn9, mode_adc_in_9, SignalAdcIn9);
    // impl_adc_in!(ModeAdcIn10, mode_adc_in_10, SignalAdcIn10);
    // impl_adc_in!(ModeAdcIn11, mode_adc_in_11, SignalAdcIn11);
    // impl_adc_in!(ModeAdcIn12, mode_adc_in_12, SignalAdcIn12);
    // impl_adc_in!(ModeAdcIn13, mode_adc_in_13, SignalAdcIn13);
    // impl_adc_in!(ModeAdcIn14, mode_adc_in_14, SignalAdcIn14);
    // impl_adc_in!(ModeAdcIn15, mode_adc_in_15, SignalAdcIn15);

}

pub mod gpio {
    pub use chip::gpio::*;
    pub use kinetis_common::hal::gpio::*;
    pub use super::pcc::PccEnabled;
}


pub mod lpuart {
    pub use chip::lpuart::*;
    pub use kinetis_common::hal::lpuart::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod lpit {
    pub use chip::lpit::*;
    pub use kinetis_common::hal::lpit::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod ftm {
    pub use chip::ftm::*;
    pub use bobbin_common::timer::*;
    pub use kinetis_common::hal::ftm::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod flexcan {
    pub use chip::flexcan::*;
    pub use kinetis_common::hal::flexcan::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod dma {
    pub use chip::dma::*;
    pub use kinetis_common::hal::dma::*;
}