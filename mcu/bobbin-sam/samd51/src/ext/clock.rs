use crate::gclk::GCLK;
use crate::nvmctrl::NVMCTRL;
use crate::mclk::MCLK;
use crate::oscctrl::OSCCTRL;
use crate::cmcc::CMCC;

// Clock Notes
//   from https://github.com/atsamd-rs/atsamd/blob/master/hal/src/clock51.rs
//
// Set Flash to Half Auto Wait State
//    - NVMCTRLA.CTRLA.RWS = 0x1 (Half Auto Wait State)
//
// Enable GCLK APB
//    - MCLK.APBAMASK |= GCLK

// Set GCLK5 to 2MHz
//    - SRC = 0x06 (DFLL @ 48MHz)
//    - GENEN = 1
//    - DIV = 24 (divide by 24) => 2MHz
//
// Set DPLL0 (GCLK1) to use GCLK5 as source
//   - CHEN = 1
//   - GEN = 0x5 (GCLK5)
//
// Configure DPLL0 to output 120MHz (60 * 2MHz)
//   - DPLLRATIO0.LDR = 59 (multiply by 60)
//   - DPLLRATIO0.LDRFRAC = 0
//   - DPLLCTRLB.REFCLK = 0 (GCLK5)
//   - DPLLCTRLA.ENABLE = 1
//   - DPLLCTRLA.ONDEMAND = 0
// Set GCLK0 to use DPLL0
//   - SRC = 0x07 (DPLL0)
//   - DIV = 1
//   - OE = 1
//   - GENEN = 1
//
//  Set MCLK CPU Divider to 1 (120MHz)
//   - MCLK.CPUDIV = 1 (Divide by 1)
// 
//  Enable Cache
//   
// 11.6.2
// On reset, the cache controller data entries are all invalidated, and the cache is disabled. The cache is transparent to processor operations. 
// The cache controller is activated through the use of its configuration registers. The configuration interface is memory mapped in the APB bus.
// Use the following sequence to enable the cache controller:
// -  Verify that the CMCC is disabled, reading the value of the SR.CSTS.
// - Enable the CMCC by writing '1' in CTRL.CEN. The MODULE is disabled by writing a '0' in CTRL.CEN.
//

pub fn run_120mhz() {
    // Set Flash to Half Auto Wait State
    //    - NVMCTRLA.CTRLA.RWS = 0x1 (Half Auto Wait State)

    NVMCTRL.with_ctrla(|r| r.set_rws(0x1));

    // Enable GCLK APB
    //    - MCLK.APBAMASK |= GCLK

    MCLK.with_apbamask(|r| r.set_gclk(1));

    // Set GEN5 to 2MHz
    //    - GENCTRL[5].SRC = 0x06 (DFLL @ 48MHz)
    //    - GENCTRL[5].DIV = 24 (divide by 24) => 2MHz
    //    - GENCTRL[5].GENEN = 1

    GCLK.with_genctrl(5, |r| r.set_src(0x6).set_div(24).set_genen(1));

    // Set DPLL0 (GCLK1) to use GEN5 as source
    //   - PCHCTRL[1].CHEN = 1
    //   - PCHCTRL[1].GEN = 0x5 (GEN5)

    GCLK.with_pchctrl(1, |r| r.set_chen(1).set_gen(0x5));

    // Configure DPLL0 to output 120MHz (60 * 2MHz)
    //   - DPLLRATIO[0].LDR = 59 (multiply by 60)
    //   - DPLLRATIO[0].LDRFRAC = 0
    //   - DPLLCTRLB.REFCLK = 0 (GCLK5)
    //   - DPLLCTRLA.ENABLE = 1
    //   - DPLLCTRLA.ONDEMAND = 0

    OSCCTRL
        .with_dpllratio(0, |r| r.set_ldr(59).set_ldrfrac(0))
        .with_dpllctrlb(0, |r| r.set_refclk(0))
        .with_dpllctrla(0, |r| r.set_enable(1).set_ondemand(0));

    // Set GEN0 to use DPLL0
    //   - SRC = 0x07 (DPLL0)
    //   - DIV = 1
    //   - OE = 1
    //   - GENEN = 1

    GCLK.with_genctrl(0, |r| r.set_src(0x7).set_div(1).set_oe(1).set_genen(1));

    //  Set MCLK CPU Divider to 1 (120MHz)
    //   - MCLK.CPUDIV.DIV = 1 (Divide by 1)

    MCLK.with_cpudiv(|r| r.set_div(1));

    // Enable Cache

    CMCC.set_ctrl(|r| r.set_cen(1));    
}