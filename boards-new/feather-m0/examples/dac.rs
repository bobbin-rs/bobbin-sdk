#![no_std]
#![no_main]

#[macro_use]
extern crate feather_m0 as board;
extern crate examples;

// use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::dac::*;
use board::mcu::gclk;
// DAC = PA0 = PA02

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("DAC Test");

    let dac = DAC;
    // PA02.connect_to(dac); // NOTE - this may burn out chip!!!!
    dac.gate_enable();
    {

        // // Initialize DAC
        // // Setting clock
        // while ( GCLK->STATUS.reg & GCLK_STATUS_SYNCBUSY );
        // GCLK->CLKCTRL.reg = GCLK_CLKCTRL_ID( GCM_DAC ) | // Generic Clock ADC
        //                     GCLK_CLKCTRL_GEN_GCLK0     | // Generic Clock Generator 0 is source
        //                     GCLK_CLKCTRL_CLKEN ;

        // while ( DAC->STATUS.bit.SYNCBUSY == 1 ); // Wait for synchronization of registers between the clock domains
        // DAC->CTRLB.reg = DAC_CTRLB_REFSEL_AVCC | // Using the 3.3V reference
        //                 DAC_CTRLB_EOEN ;        // External Output Enable (Vout)

        while gclk::GCLK.status().syncbusy() != 0 {}
        println!("Setting GCLK");
        // Set GCLK_GEN0 as source for DAC
        gclk::GCLK.set_clkctrl(|r| r
            .set_id(0x23) // GCLK_DAC
            .set_gen(0x0)
            .set_clken(true)
        );    
        while DAC.status().syncbusy() != 0 {}
        println!("Setting CTRLB");
        DAC.set_ctrlb(|r| r.set_refsel(0x01).set_eoen(1));
        // while DAC.status().syncbusy() != 0 {}
        println!("Setting ENABLE");
        DAC.with_ctrla(|r| r.set_enable(1));
        // while DAC.status().syncbusy() != 0 {}
    }

    println!("Starting Loop");

    let mut v: u8 = 16;
    let s: u8 = 4;
    let mut d: bool = true;
    loop {
        // while DAC.status().syncbusy() != 0 {}
        DAC.set_data(|r| r.set_data((v as u16) << 2));
        // while DAC.status().syncbusy() != 0 {}
        if d {
            v += s;
            if v == 240 {                
                d = !d;
            }
        } else {
            v -= s;
            if v == 16 {
                d = !d;
            }
        }
        board::delay(5);
    }
}
