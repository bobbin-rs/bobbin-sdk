use bobbin_bits::*;
use bobbin_mcu::clock::Clock;
use bobbin_mcu::hz::Hz;

use clock::*;
use ext::systick::SystickHz;

use gclk::*;
use {gclk, sysctrl, nvmctrl, pm};



macro_rules! impl_gclk {
    ($id:ident, $index:expr) => {
        fn $id(&self) -> Hz { self.gclk($index)}        
    };
}

macro_rules! impl_gclkgen {
    ($id:ident, $index:expr) => {
        fn $id(&self) -> Hz { self.gclkgen($index)}        
    };
}

#[derive(Default)]
pub struct Osc48m {}
impl Clock for Osc48m {
    fn hz() -> Hz { Hz::from(48_000_000)}
}

#[derive(Default)]
pub struct Osc32k {}
impl Clock for Osc32k {
    fn hz() -> Hz { Hz::from(32768)}
}

#[derive(Default)]
pub struct DynamicClock<XOSC: Clock, XOSC32K: Clock>(XOSC, XOSC32K);

impl<XOSC: Clock, XOSC32K: Clock> DynamicClock<XOSC, XOSC32K> {
    pub fn read_clkctrl(&self, id: U6) -> Clkctrl {
        unsafe {
            ::core::ptr::write_volatile(GCLK.clkctrl_mut() as *mut u8, id.value());
        }
        GCLK.clkctrl()
    }

    pub fn read_genctrl(&self, id: U4) -> Genctrl {
        unsafe {
            ::core::ptr::write_volatile(GCLK.genctrl_mut() as *mut u8, id.value());
        }
        GCLK.genctrl()
    }

    pub fn read_gendiv(&self, id: U4) -> Gendiv {
        unsafe {
            ::core::ptr::write_volatile(GCLK.gendiv_mut() as *mut u8, id.value());
        }
        GCLK.gendiv()
    }

    pub fn src(&self, id: U5) -> Hz {
        match id {
            U5::B00000 => self.xosc(),
            // GCLKIN is not supported
            // U5::B00001 => self.gclkin(),
            U5::B00010 => self.gclkgen1(),
            U5::B00011 => self.osculp32k(),
            U5::B00100 => self.osc32k(),
            U5::B00101 => self.xosc32k(),
            U5::B00110 => self.osc8m(),
            U5::B00111 => self.dfll48m(),
            U5::B01000 => self.fdpll96m(),
            _ => Hz::from_num(0),            
        }
    }

    pub fn gclkgen(&self, id: U4) -> Hz {
        let gc = self.read_genctrl(id);
        let gd = self.read_gendiv(id);
        if gc.test_genen() {
            let div = match gc.divsel() {
                U1::B0 => match gd.div().value() {
                    0 => 1,
                    v @ _ => v,
                },
                U1::B1 => 1 << (gd.div().value() + 1),
            };
            self.src(gc.src()) / div as u32
        } else {
            Hz::from_num(0)
        }
    }

    pub fn gclk(&self, id: U6) -> Hz {
        let cc = self.read_clkctrl(id);
        if cc.test_clken() {
            self.gclkgen(cc.gen())
        } else {
            Hz::from_num(0)
        }
    }
}

impl <XOSC: Clock, XOSC32K: Clock> ClockProvider for DynamicClock<XOSC, XOSC32K> {
    type Xosc = XOSC;
    type Xosc32k = XOSC32K;
    impl_gclkgen!(gclkgen0, U4::B0000);
    impl_gclkgen!(gclkgen1, U4::B0001);
    impl_gclkgen!(gclkgen2, U4::B0010);
    impl_gclkgen!(gclkgen3, U4::B0011);
    impl_gclkgen!(gclkgen4, U4::B0100);
    impl_gclkgen!(gclkgen5, U4::B0101);
    impl_gclkgen!(gclkgen6, U4::B0110);
    impl_gclkgen!(gclkgen7, U4::B0111);
    impl_gclkgen!(gclkgen8, U4::B1000);

    impl_gclk!(gclk_dffl48m_ref, U6::B000000); // 0x00
    impl_gclk!(gclk_dpll, U6::B000001); // 0x01
    impl_gclk!(gclk_dpll_32k, U6::B000010); // 0x02
    impl_gclk!(gclk_wdt, U6::B000011); // 0x03
    impl_gclk!(gclk_rtc, U6::B000100); // 0x04
    impl_gclk!(gclk_eic, U6::B000101); // 0x05
    impl_gclk!(gclk_usb, U6::B000110); // 0x06
    impl_gclk!(gclk_evsys_channel_0, U6::B000111); // 0x07
    impl_gclk!(gclk_evsys_channel_1, U6::B001000); // 0x08
    impl_gclk!(gclk_evsys_channel_2, U6::B001001); // 0x09
    impl_gclk!(gclk_evsys_channel_3, U6::B001010); // 0x0a
    impl_gclk!(gclk_evsys_channel_4, U6::B001011); // 0x0b
    impl_gclk!(gclk_evsys_channel_5, U6::B001100); // 0x0c
    impl_gclk!(gclk_evsys_channel_6, U6::B001101); // 0x0d
    impl_gclk!(gclk_evsys_channel_7, U6::B001110); // 0x0e
    impl_gclk!(gclk_evsys_channel_8, U6::B001111); // 0x0f
    impl_gclk!(gclk_evsys_channel_9, U6::B010000); // 0x10
    impl_gclk!(gclk_evsys_channel_10, U6::B010001); // 0x11
    impl_gclk!(gclk_evsys_channel_11, U6::B010010); // 0x12
    impl_gclk!(gclk_sercomx_slow, U6::B010011); // 0x13
    impl_gclk!(gclk_sercom0_core, U6::B010100); // 0x14
    impl_gclk!(gclk_sercom1_core, U6::B010101); // 0x15
    impl_gclk!(gclk_sercom2_core, U6::B010110); // 0x16
    impl_gclk!(gclk_sercom3_core, U6::B010111); // 0x17
    impl_gclk!(gclk_sercom4_core, U6::B011000); // 0x18
    impl_gclk!(gclk_sercom5_core, U6::B011001); // 0x19
    impl_gclk!(gclk_tcc0_tcc1, U6::B011010); // 0x1a
    impl_gclk!(gclk_tcc2_tc3, U6::B011011); // 0x1b
    impl_gclk!(gclk_tc4_tc5, U6::B011100); // 0x1c
    impl_gclk!(gclk_tc6_tc7, U6::B011101); // 0x1d
    impl_gclk!(gclk_adc, U6::B011110); // 0x1e
    impl_gclk!(gclk_adc_dig, U6::B011111); // 0x1f
    impl_gclk!(gclk_20, U6::B100000); // 0x20
    impl_gclk!(gclk_ac_ana, U6::B100001); // 0x21
    impl_gclk!(gclk_22, U6::B100010); // 0x22
    impl_gclk!(gclk_dac, U6::B100011); // 0x23
    impl_gclk!(gclk_ptc, U6::B100100); // 0x24
    impl_gclk!(gclk_i2s_0, U6::B100101); // 0x25
    impl_gclk!(gclk_i2s_1, U6::B100110); // 0x26
}

impl <XOSC: Clock, XOSC32K: Clock> SystickHz for DynamicClock<XOSC, XOSC32K> {
    fn systick_hz(&self) -> Hz {
        self.gclkgen0()
    }
}

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
    sysctrl::SYSCTRL.set_xosc32k(|r| r
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

    gclk::GCLK.set_ctrl(|r| r.set_swrst(1));

    // Wait for reset to complete

    while gclk::GCLK.ctrl().swrst() != 0 && gclk::GCLK.status().syncbusy() != 0 {}

    // Put XOSC32K as source of Generic Clock Generator 1
    gclk::GCLK.set_gendiv(|r| r.set_id(0x1));

    // Wait for register sync
    while gclk::GCLK.status().syncbusy() != 0 {}

    // Write Generic Clock Generator 1 configuration 
    gclk::GCLK.set_genctrl(|r| r
        .set_id(0x01) // XOSC32K
        .set_src(0x05) // XOSC32K
        .set_genen(1)
    );

    // Wait for register sync
    while gclk::GCLK.status().syncbusy() != 0 {}

    // Put Generic Clock Generator 1 as source for Generic Clock Multiplexer 0 (DFLL48M reference)
    gclk::GCLK.set_clkctrl(|r| r
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

    sysctrl::SYSCTRL.set_dfllmul(|r| r
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
    gclk::GCLK.set_gendiv(|r| r.set_id(0x0));

    // Wait for register sync
    while gclk::GCLK.status().syncbusy() != 0 {}

    // Write Generic Clock Generator 0 configuration

    gclk::GCLK.set_genctrl(|r| r
        .set_id(0x00)
        .set_src(0x07) // DFLL48M
        .set_idc(1)
        .set_genen(1)
    );

    // Wait for register sync
    while gclk::GCLK.status().syncbusy() != 0 {}

    // Write Generic Clock Generator 2 configuration
    // 1.024 kHz output for RTC

    gclk::GCLK.set_gendiv(|r| r
        .set_id(0x002)
        .set_div(4) // 2^5 == 32
    );

    gclk::GCLK.set_genctrl(|r| r
        .set_id(0x02)
        .set_src(0x05) // XOSC32K        
        .set_divsel(1) // Exponentiate Divider
        .set_genen(1)
    );

    // Write Generic Clock 3 configuration
    // 8Mhz output for ADC


    gclk::GCLK.set_genctrl(|r| r
        .set_id(0x03)
        .set_src(0x06) // OSC8M            
        .set_genen(1)
    );

    // Now that all system clocks are configured, we can set CPU and APBx BUS clocks.
    // These values are normally the ones present after Reset.

    pm::PM.set_cpusel(|r| r.set_cpudiv(0x0)); // DIV1
    pm::PM.set_apbasel(|r| r.set_apbadiv(0x0)); // DIV1
    pm::PM.set_apbbsel(|r| r.set_apbbdiv(0x0)); // DIV1
    pm::PM.set_apbcsel(|r| r.set_apbcdiv(0x0)); // DIV1
        
}

