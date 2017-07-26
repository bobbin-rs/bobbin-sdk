use ::chip::{gclk, sysctrl, nvmctrl, pm};
use ::chip::sysctrl::SYSCTRL;

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

pub const OSC32K: Hz = Some(32767);
pub const OSCULP32K: Hz = Some(32767);
pub const OSC8M: Hz = Some(8_000_000);

#[derive(Debug, PartialEq)]
pub enum Osc8MPrescaler {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
}

#[derive(Debug, PartialEq)]
pub enum DpllRefClock {
    Xosc32k = 0,
    Xosc = 1,
    GclkDpll = 2,
}

#[derive(Debug, PartialEq)]
pub enum Source {
    Xosc = 0x00,
    GclkIn = 0x01,
    GclkGen1 = 0x02,
    OscUlp32k = 0x3,
    Osc32k = 0x4,
    Xosc32K = 0x5,
    Osc8m = 0x6,
    Dffl48m = 0x7,
    Fdpll86m = 0x8
}

#[derive(Debug, PartialEq)]
pub enum Generator {
    GClkGen0 = 0,
    GClkGen1 = 1,
    GClkGen2 = 2,
    GClkGen3 = 3,
    GClkGen4 = 4,
    GClkGen5 = 5,
    GClkGen6 = 6,
    GClkGen7 = 7,
    GClkGen8 = 8,
}

#[derive(Debug, PartialEq)]
pub enum Multiplexer {
    Dfll48mRef = 0x00,
    Dpll = 0x01,
    Dpll32k = 0x02,
    Wdt = 0x03,
    Rtc = 0x04,
    Eic = 0x05,
    Usb = 0x06,
    EvsysCh0 = 0x07,
    EvsysCh1 = 0x08,
    EvsysCh2 = 0x09,
    EvsysCh3 = 0x0a,
    EvsysCh4 = 0x0b,
    EvsysCh5 = 0x0c,
    EvsysCh6 = 0x0d,
    EvsysCh7 = 0x0e,
    EvsysCh8 = 0x0f,
    EvsysCh9 = 0x10,
    EvsysCh10 = 0x11,
    EvsysCh11 = 0x12,
    SercomSlow = 0x13,
    Sercom0 = 0x14,
    Sercom1 = 0x15,
    Sercom2 = 0x16,
    Sercom3 = 0x17,
    Sercom4 = 0x18,
    Sercom5 = 0x19,
    Tcc0Tcc1 = 0x1a,
    Tcc2Tc3 = 0x1b,
    Tc4Tc5 = 0x1c,
    Tc6Tc7 = 0x1d,
    Adc = 0x1e,
    AcDig = 0x1f,
    AcAna = 0x21,
    Dac = 0x23,
    Ptc = 0x24,
    I2s0 = 0x25,
    I2s1 = 0x26,
}

pub type Hz = Option<u32>;

pub struct ClockTree {
    pub xosc: Hz,
    pub xosc32k: Hz,    
}

impl ClockTree {
    // XOSC

    pub fn xosc_xtal_enabled(&self) -> bool {
        SYSCTRL.xosc().xtalen() != 0
    }
    
    pub fn set_xosc_xtal_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_xosc(|r| r.set_xtalen(value));
        self
    }    

    pub fn xosc_enabled(&self) -> bool {
        SYSCTRL.xosc().enable() != 0
    }
    
    pub fn set_xosc_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_xosc(|r| r.set_enable(value));
        self
    }

    pub fn xosc_ondemand(&self) -> bool {
        SYSCTRL.xosc().ondemand() != 0
    }
    
    pub fn set_xosc_ondemand(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_xosc(|r| r.set_ondemand(value));
        self
    }
    pub fn xosc_rdy(&self) -> bool {
        SYSCTRL.pclksr().xoscrdy() != 0
    }

    pub fn xosc(&self) -> Hz {
        if self.xosc_rdy() {
            self.xosc
        } else {
            None
        }
    }

    // XOSC32K

    pub fn xosc32k_xtal_enabled(&self) -> bool {
        SYSCTRL.xosc32k().xtalen() != 0
    }
    
    pub fn set_xosc32k_xtal_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_xosc32k(|r| r.set_xtalen(value));
        self
    }        

    pub fn xosc32k_enabled(&self) -> bool {
        SYSCTRL.xosc32k().enable() != 0
    }
    
    pub fn set_xosc32k_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_xosc32k(|r| r.set_enable(value));
        self
    }

    pub fn xosc32k_ondemand(&self) -> bool {
        SYSCTRL.xosc32k().ondemand() != 0
    }
    
    pub fn set_xosc32k_ondemand(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_xosc32k(|r| r.set_ondemand(value));
        self
    }
    pub fn xosc32k_rdy(&self) -> bool {
        SYSCTRL.pclksr().xosc32krdy() != 0
    }

    pub fn xosc32k(&self) -> Hz {
        if self.xosc32k_rdy() {
            self.xosc32k
        } else {
            None
        }
    }

    // OSC32K

    pub fn osc32k_enabled(&self) -> bool {
        SYSCTRL.osc32k().enable() != 0
    }
    
    pub fn set_osc32k_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_osc32k(|r| r.set_enable(value));
        self
    }

    pub fn osc32k_ondemand(&self) -> bool {
        SYSCTRL.osc32k().ondemand() != 0
    }
    
    pub fn set_osc32k_ondemand(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_osc32k(|r| r.set_ondemand(value));
        self
    }
    pub fn osc32k_rdy(&self) -> bool {
        SYSCTRL.pclksr().osc32krdy() != 0
    }

    pub fn osc32k(&self) -> Hz {
        if self.osc32k_rdy() {
            OSC32K
        } else {
            None
        }
    }    

    // OSCULP32K

    pub fn osculp32k(&self) -> Hz {
        OSCULP32K
    }

    // OSC8M

    pub fn osc8m_pre(&self) -> Osc8MPrescaler {
        match SYSCTRL.osc8m().presc() {
            0 => Osc8MPrescaler::Div1,
            1 => Osc8MPrescaler::Div2,
            2 => Osc8MPrescaler::Div4,
            3 => Osc8MPrescaler::Div8,
            _ => unimplemented!(),
        }
    }

    pub fn set_osc8m_pre(&self, value: Osc8MPrescaler) -> &Self {
        SYSCTRL.with_osc8m(|r| r.set_presc(value as u32));
        self
    }

    pub fn osc8m_enabled(&self) -> bool {
        SYSCTRL.osc8m().enable() != 0
    }
    
    pub fn set_osc8m_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_osc8m(|r| r.set_enable(value));
        self
    }

    pub fn osc8m_ondemand(&self) -> bool {
        SYSCTRL.osc8m().ondemand() != 0
    }
    
    pub fn set_osc8m_ondemand(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_osc8m(|r| r.set_ondemand(value));
        self
    }

    pub fn osc8m_rdy(&self) -> bool {
        SYSCTRL.pclksr().osc8mrdy() != 0
    }
    
    pub fn osc8m(&self) -> Hz {
        if self.osc8m_rdy() {
            OSC8M.map(|v| v >> SYSCTRL.osc8m().presc())
        } else {
            None
        }
    }


    // DFLL

    pub fn dfll_enabled(&self) -> bool {
        SYSCTRL.dfllctrl().enable() != 0
    }
    
    pub fn set_dfll_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_dfllctrl(|r| r.set_enable(value));
        self
    }


    pub fn dfll_ondemand(&self) -> bool {
        SYSCTRL.dfllctrl().ondemand() != 0
    }
    
    pub fn set_dfll_ondemand(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_dfllctrl(|r| r.set_ondemand(value));
        self
    }

    pub fn dfll_mul(&self) -> u32 {
        SYSCTRL.dfllmul().mul()
    }

    pub fn set_dfll_mul(&self, value: u32) -> &Self {
        SYSCTRL.with_dfllmul(|r| r.set_mul(value));
        self
    }

    pub fn dfll_rdy(&self) -> bool {
        SYSCTRL.pclksr().dfllrdy() != 0
    }

    pub fn dfll(&self) -> Hz {
        if self.dfll_rdy() {
            OSC32K.map(|v| v * self.dfll_mul())
        } else {
            None
        }
    }  

    // DPLL

    pub fn dpll_enabled(&self) -> bool {
        SYSCTRL.dpllstatus().enable() != 0
    }
    
    pub fn set_dpll_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_dpllctrla(|r| r.set_enable(value));
        self
    }

    pub fn dpll_ondemand(&self) -> bool {
        SYSCTRL.dpllctrla().ondemand() != 0
    }
    
    pub fn set_dpll_ondemand(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        SYSCTRL.with_dpllctrla(|r| r.set_ondemand(value));
        self
    }
    

    pub fn dpll_mul(&self) -> u32 {
        SYSCTRL.dpllratio().ldr()
    }

    pub fn set_dpll_mul(&self, value: u32) -> &Self {
        SYSCTRL.with_dpllratio(|r| r.set_ldr(value));
        self
    }

    pub fn dpll_mul_frac(&self) -> u32 {
        SYSCTRL.dpllratio().ldrfrac()
    }

    pub fn set_dpll_mul_frac(&self, value: u32) -> &Self {
        SYSCTRL.with_dpllratio(|r| r.set_ldrfrac(value));
        self
    }

    pub fn dpll_div(&self) -> u32 {
        SYSCTRL.dpllctrlb().div()
    }

    pub fn set_dpll_div(&self, value: u32) -> &Self {
        SYSCTRL.with_dpllctrlb(|r| r.set_div(value));
        self
    }

    pub fn dpll_refclk(&self) -> DpllRefClock {
        match SYSCTRL.dpllctrlb().refclk() {
            0 => DpllRefClock::Xosc32k,
            1 => DpllRefClock::Xosc,
            2 => DpllRefClock::GclkDpll,
            _ => unimplemented!(),
        }
    }

    pub fn set_dpll_refclk(&self, value: DpllRefClock) -> &Self {
        SYSCTRL.with_dpllctrlb(|r| r.set_refclk(value as u32));
        self
    }

    pub fn dpll_divider_enable(&self) -> bool {
        SYSCTRL.dpllstatus().div() != 0
    }

    pub fn dpll_lock(&self) -> bool {
        SYSCTRL.dpllstatus().lock() != 0
    }    

    pub fn dpll_rdy(&self) -> bool {
        SYSCTRL.dpllstatus().clkrdy() != 0
    }

    pub fn dpll(&self) -> Hz {
        if self.dpll_rdy() {
            let clk = match SYSCTRL.dpllctrlb().refclk() {
                0 => self.xosc32k(),
                1 => self.xosc(),
                _ => unimplemented!(),
            };
            if self.dpll_div() != 0 {
                clk.map(|v| v * self.dpll_mul() / self.dpll_div())
            } else {
                clk.map(|v| v * self.dpll_mul())
            }
        } else {
            None
        }
    }    
}