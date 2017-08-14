#![allow(non_camel_case_types)]

pub use bobbin_cortexm::hal::*;

pub mod sim;
pub mod clock;

pub mod port {
    pub use chip::port::*;    
    pub use kinetis_common::hal::port::*;
    pub use super::sim::SimEnabled;
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

    // macro_rules! impl_adc {
    //     ($mode:ident, $fn:ident, $sig:ident) => (
    //         use chip::sig::$sig;

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

    // impl_adc!(ModeAdcDp0, mode_adc_dp0, SignalAdcDp0);
    // impl_adc!(ModeAdcDp1, mode_adc_dp1, SignalAdcDp1);
    // impl_adc!(ModeAdcDp2, mode_adc_dp2, SignalAdcDp2);
    // impl_adc!(ModeAdcDp3, mode_adc_dp3, SignalAdcDp3);

    // impl_adc!(ModeAdcDm0, mode_adc_dm0, SignalAdcDm0);
    // impl_adc!(ModeAdcDm1, mode_adc_dm1, SignalAdcDm1);
    // impl_adc!(ModeAdcDm2, mode_adc_dm2, SignalAdcDm2);
    // impl_adc!(ModeAdcDm3, mode_adc_dm3, SignalAdcDm3);

    // impl_adc!(ModeAdcSe4a, mode_adc_se4a, SignalAdcSe4a);
    // impl_adc!(ModeAdcSe5a, mode_adc_se5a, SignalAdcSe5a);
    // impl_adc!(ModeAdcSe6a, mode_adc_se6a, SignalAdcSe6a);
    // impl_adc!(ModeAdcSe7a, mode_adc_se7a, SignalAdcSe7a);

    // impl_adc!(ModeAdcSe4b, mode_adc_se4b, SignalAdcSe4b);
    // impl_adc!(ModeAdcSe5b, mode_adc_se5b, SignalAdcSe5b);
    // impl_adc!(ModeAdcSe6b, mode_adc_se6b, SignalAdcSe6b);
    // impl_adc!(ModeAdcSe7b, mode_adc_se7b, SignalAdcSe7b);

    // impl_adc!(ModeAdcSe8, mode_adc_se8, SignalAdcSe8);
    // impl_adc!(ModeAdcSe9, mode_adc_se9, SignalAdcSe9);
    // impl_adc!(ModeAdcSe10, mode_adc_se10, SignalAdcSe10);
    // impl_adc!(ModeAdcSe11, mode_adc_se11, SignalAdcSe11);
    // impl_adc!(ModeAdcSe12, mode_adc_se12, SignalAdcSe12);
    // impl_adc!(ModeAdcSe13, mode_adc_se13, SignalAdcSe13);
    // impl_adc!(ModeAdcSe14, mode_adc_se14, SignalAdcSe14);
    // impl_adc!(ModeAdcSe15, mode_adc_se15, SignalAdcSe15);
    // impl_adc!(ModeAdcSe16, mode_adc_se16, SignalAdcSe16);
    // impl_adc!(ModeAdcSe17, mode_adc_se17, SignalAdcSe17);
    // impl_adc!(ModeAdcSe18, mode_adc_se18, SignalAdcSe18);
    // impl_adc!(ModeAdcSe19, mode_adc_se19, SignalAdcSe19);
    // impl_adc!(ModeAdcSe20, mode_adc_se20, SignalAdcSe20);
    // impl_adc!(ModeAdcSe21, mode_adc_se21, SignalAdcSe21);
    // impl_adc!(ModeAdcSe22, mode_adc_se22, SignalAdcSe22);
    // impl_adc!(ModeAdcSe23, mode_adc_se23, SignalAdcSe23);

}

pub mod gpio {
    pub use chip::gpio::*;
    pub use kinetis_common::hal::gpio::*;
    pub use super::sim::SimEnabled;
}


pub mod uart {
    pub use chip::uart::*;
    pub use kinetis_common::hal::uart::*;
    pub use super::sim::SimEnabled;
}

pub mod ftm {
    pub use chip::ftm::*;
    pub use kinetis_common::hal::ftm::*;
    pub use super::sim::SimEnabled;
}

pub mod pit {
    pub use chip::pit::*;
    pub use kinetis_common::hal::pit::*;
    pub use super::sim::SimEnabled;
}

pub mod lptmr {
    pub use chip::lptmr::*;
    pub use kinetis_common::hal::lptmr::*;
    pub use super::sim::SimEnabled;
}

pub mod edma {
    pub use chip::edma::*;
    pub use kinetis_common::hal::edma::*;
    pub use super::sim::SimEnabled;
}

pub mod dmamux {
    pub use chip::dmamux::*;
    pub use kinetis_common::hal::dmamux::*;
    pub use super::sim::SimEnabled;
}

pub mod adc {
    pub use chip::adc::*;
    pub use kinetis_common::hal::adc::*;
    pub use super::sim::SimEnabled;
}


pub mod wdog {
    pub use chip::wdog::*;
    pub use kinetis_common::hal::wdog::*;
}