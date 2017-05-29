use ::chip::{gclk, sysctrl, nvmctrl, pm};

const VARIANT_MCK: u32 = 48_000_000;
const VARIANT_MAINOSC: u32 = 32_768;

pub fn run_48mhz() {
    // See https://github.com/arduino/ArduinoCore-samd/blob/master/bootloaders/zero/board_init.c
    //  * At reset:
    //  * - OSC8M clock source is enabled with a divider by 8 (1MHz).
    //  * - Generic Clock Generator 0 (GCLKMAIN) is using OSC8M as source.
    //  * We need to:
    //  * 1) Enable XOSC32K clock (External on-board 32.768Hz oscillator), will be used as DFLL48M reference.
    //  * 2) Put XOSC32K as source of Generic Clock Generator 1
    //  * 3) Put Generic Clock Generator 1 as source for Generic Clock Multiplexer 0 (DFLL48M reference)
    //  * 4) Enable DFLL48M clock
    //  * 5) Switch Generic Clock Generator 0 to DFLL48M. CPU will run at 48MHz.
    //  * 6) Modify PRESCaler value of OSCM to have 8MHz
    //  * 7) Put OSC8M as source for Generic Clock Generator 3
    unsafe {

        /* Set 1 Flash Wait State for 48MHz, cf tables 20.9 and 35.27 in SAMD21 Datasheet */
        // NVMCTRL->CTRLB.bit.RWS = NVMCTRL_CTRLB_RWS_HALF_Val;
        nvmctrl::NVMCTRL.with_ctrlb(|r| r.set_rws(0x1));

        /* Turn on the digital interface clock */
        pm::PM.with_apbamask(|r| r.set_gclk(1));

        // Enable XOSC32K clock (External on-board 32.768Hz oscillator), will be used as DFLL48M reference.
        sysctrl::SYSCTRL.set_xosc32k(sysctrl::Xosc32k(0)
            .set_startup(0x6)
            .set_en32k(1)
            .set_xtalen(1)
        );
        // separate call, as described in chapter 15.6.3 
        sysctrl::SYSCTRL.with_xosc32k(|r| r.set_enable(1));

        // Wait for oscillator stabilization

        while sysctrl::SYSCTRL.pclksr().xosc32krdy() == 0{}

        /* Software reset the module to ensure it is re-initialized correctly */
        /* Note: Due to synchronization, there is a delay from writing CTRL.SWRST until the reset is complete.
        * CTRL.SWRST and STATUS.SYNCBUSY will both be cleared when the reset is complete, as described in chapter 13.8.1
        */

        gclk::GCLK.set_ctrl(gclk::Ctrl(0).set_swrst(1));

        // Wait for reset to complete

        while gclk::GCLK.ctrl().swrst() != 0 && gclk::GCLK.status().syncbusy() != 0 {}

        // Put XOSC32K as source of Generic Clock Generator 1
        gclk::GCLK.set_gendiv(gclk::Gendiv(0).set_id(0x1));

        // Wait for register sync
        while gclk::GCLK.status().syncbusy() != 0 {}

        // Write Generic Clock Generator 1 configuration 
        gclk::GCLK.set_genctrl(gclk::Genctrl(0)
            .set_id(0x01) // XOSC32K
            .set_src(0x05) // XOSC32K
            .set_genen(1)
        );

        // Wait for register sync
        while gclk::GCLK.status().syncbusy() != 0 {}

        // Put Generic Clock Generator 1 as source for Generic Clock Multiplexer 0 (DFLL48M reference)
        gclk::GCLK.set_clkctrl(gclk::Clkctrl(0)
            .set_id(0x00) // DFLL48M
            .set_gen(0x1) // Clock Generator 1 is source
            .set_clken(1)
        );

        // Wait for register sync
        while gclk::GCLK.status().syncbusy() != 0 {}

        // Enable DFLL48M Clock

        // DFLL Configuration in Closed Loop mode, cf product datasheet chapter 15.6.7.1 - Closed-Loop Operation 

        // Remove the OnDemand mode, Bug http://avr32.icgroup.norway.atmel.com/bugzilla/show_bug.cgi?id=9905 
        sysctrl::SYSCTRL.with_dfllctrl(|r| r.set_ondemand(0));

        // Wait for synchronization
        while sysctrl::SYSCTRL.pclksr().dfllrdy() == 0 {}

        sysctrl::SYSCTRL.set_dfllmul(sysctrl::Dfllmul(0)
            .set_cstep(31) // coarse step 31, half of the max value
            .set_fstep(511) // fine step 511, half of the max value
            .set_mul(VARIANT_MCK / VARIANT_MAINOSC)
        );

        // Wait for synchronization
        while sysctrl::SYSCTRL.pclksr().dfllrdy() == 0 {}


        // Write full configuration to DFLL control register 

        sysctrl::SYSCTRL.with_dfllctrl(|r| r
            .set_mode(1)
            .set_waitlock(1)
            .set_qldis(1)
        );

        // Wait for synchronization
        while sysctrl::SYSCTRL.pclksr().dfllrdy() == 0 {}

        // Enable the DFLL
        sysctrl::SYSCTRL.with_dfllctrl(|r| r.set_enable(1));

        // Wait for lock flags
        while sysctrl::SYSCTRL.pclksr().dflllckc() == 0 || sysctrl::SYSCTRL.pclksr().dflllckf() == 0 {}

        // Wait for synchronization
        while sysctrl::SYSCTRL.pclksr().dfllrdy() == 0 {}

        // Switch Generic Clock Generator 0 to DFLL48M. CPU will run at 48MHz.
        gclk::GCLK.set_gendiv(gclk::Gendiv(0).set_id(0x0));

        // Wait for register sync
        while gclk::GCLK.status().syncbusy() != 0 {}

        // Write Generic Clock Generator 0 configuration

        gclk::GCLK.set_genctrl(gclk::Genctrl(0)
            .set_id(0x00)
            .set_src(0x07) // DFLL48M
            .set_idc(1)
            .set_genen(1)
        );

        // Wait for register sync
        while gclk::GCLK.status().syncbusy() != 0 {}

        // Write Generic Clock Generator 2 configuration
        // 1.024 kHz output for RTC

        gclk::GCLK.set_gendiv(gclk::Gendiv(0)
            .set_id(0x002)
            .set_div(4) // 2^5 == 32
        );

        gclk::GCLK.set_genctrl(gclk::Genctrl(0)
            .set_id(0x02)
            .set_src(0x05) // XOSC32K        
            .set_divsel(1) // Exponentiate Divider
            .set_genen(1)
        );

        // Write Generic Clock 3 configuration
        // 8Mhz output for ADC


        gclk::GCLK.set_genctrl(gclk::Genctrl(0)
            .set_id(0x03)
            .set_src(0x06) // OSC8M            
            .set_genen(1)
        );

        // Now that all system clocks are configured, we can set CPU and APBx BUS clocks.
        // These values are normally the ones present after Reset.

        pm::PM.set_cpusel(pm::Cpusel(0).set_cpudiv(0x0)); // DIV1
        pm::PM.set_apbasel(pm::Apbasel(0).set_apbadiv(0x0)); // DIV1
        pm::PM.set_apbbsel(pm::Apbbsel(0).set_apbbdiv(0x0)); // DIV1
        pm::PM.set_apbcsel(pm::Apbcsel(0).set_apbcdiv(0x0)); // DIV1
        
    } 
}

