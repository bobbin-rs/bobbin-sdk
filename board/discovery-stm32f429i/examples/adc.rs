#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f429i as board;
extern crate examples;

use board::prelude::*;
use board::mcu::pin::*;
use board::mcu::adc::*;
use board::mcu::c_adc::*;
use board::bobbin_bits::*;

// PA0 / ADC1_IN0

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init().run(|brd| {
        let a0 = PA0;  // A0

        let adc = ADC1;
        let adc_ch = ADC1_CH0;

        adc.gate_enable();
        a0.port().gate_enable();
        a0.connect_to(adc_ch);
        a0.mode_analog();

//        C_ADC.with_ccr(|r| r.set_ckmode(0b01));
    //    adc.init();

        let adc_ch: AdcCh = adc_ch.into();

        let mut app = examples::adc::AdcExample::new(adc_ch, brd.tick(), 500, U12::from(0));

        app.run()
    })
}
//
//pub trait Init {
//    fn init_test(&self) -> &Self;
//}
//
//impl Init for AdcPeriph {
//    fn init_test(&self) -> &Self {
//        println!("CCR: {:?}", C_ADC.ccr());
//        println!("A");
//        self.with_cr(|r| r.set_aden(0));
//        while self.isr().adrdy() != 0 {}
//
//        println!("B");
//        // Enable Analog Voltage Regulator
//        self.with_cr(|r| r.set_advregen(0b00));
//        self.with_cr(|r| r.set_advregen(0b01));
//
//        println!("C");
//        // Calibrate
//        self.with_cr(|r| r.set_adcaldif(0));
//        self.with_cr(|r| r.set_adcal(1));
//        for _ in 0..100 {
//            let _ = self.cr();
//        }
//        while self.cr().adcal() != 0 {}
//        println!("D");
//        // Enable ADC
//        self.with_cr(|r| r.set_aden(1));
//        println!("E");
//        // Wait until ADC is ready
//        while self.isr().adrdy() == 0 {}
//
//        self
//    }
//}