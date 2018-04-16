use ::bobbin_mcu::clock::Clock;
use ::bobbin_mcu::tree::ClockFor;
use ::bobbin_mcu::hz::Hz;

#[derive(Default)]
pub struct Clocks<CP: ClockProvider> { provider: CP }

impl<CP: ClockProvider> ::core::ops::Deref for Clocks<CP> {
    type Target = CP;
    fn deref(&self) -> &CP { &self.provider }
}


// Define Global Clocks

#[derive(Default)]
pub struct Osc32k {}
impl Clock for Osc32k { fn hz() -> Hz { Hz::from_num(32768) } }

#[derive(Default)]
pub struct Osc8m {}
impl Clock for Osc8m { fn hz() -> Hz { Hz::from_num(8000000) } }

#[derive(Default)]
pub struct Osculp32k {}
impl Clock for Osculp32k { fn hz() -> Hz { Hz::from_num(32000) } }

#[derive(Default)]
pub struct Dfll48m {}
impl Clock for Dfll48m { fn hz() -> Hz { Hz::from_num(48000000) } }

#[derive(Default)]
pub struct Fdpll96m {}
impl Clock for Fdpll96m { fn hz() -> Hz { Hz::from_num(96000000) } }


pub trait ClockProvider : Default {
    type Xosc: Clock;
    type Xosc32k: Clock;
    fn xosc(&self) -> Hz { Self::Xosc::hz() }
    fn xosc32k(&self) -> Hz { Self::Xosc32k::hz() }
    fn osc32k(&self) -> Hz { Hz::from_num(32768) }
    fn osc8m(&self) -> Hz { Hz::from_num(8000000) }
    fn osculp32k(&self) -> Hz { Hz::from_num(32000) }
    fn dfll48m(&self) -> Hz { Hz::from_num(48000000) }
    fn fdpll96m(&self) -> Hz { Hz::from_num(96000000) }
    fn gclkgen0(&self) -> Hz { unimplemented!() }
    fn gclkgen1(&self) -> Hz { unimplemented!() }
    fn gclkgen2(&self) -> Hz { unimplemented!() }
    fn gclkgen3(&self) -> Hz { unimplemented!() }
    fn gclkgen4(&self) -> Hz { unimplemented!() }
    fn gclkgen5(&self) -> Hz { unimplemented!() }
    fn gclkgen6(&self) -> Hz { unimplemented!() }
    fn gclkgen7(&self) -> Hz { unimplemented!() }
    fn gclkgen8(&self) -> Hz { unimplemented!() }
    fn gclk_dffl48m_ref(&self) -> Hz { unimplemented!() }
    fn gclk_dpll(&self) -> Hz { unimplemented!() }
    fn gclk_dpll_32k(&self) -> Hz { unimplemented!() }
    fn gclk_wdt(&self) -> Hz { unimplemented!() }
    fn gclk_rtc(&self) -> Hz { unimplemented!() }
    fn gclk_eic(&self) -> Hz { unimplemented!() }
    fn gclk_usb(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_0(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_1(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_2(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_3(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_4(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_5(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_6(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_7(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_8(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_9(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_10(&self) -> Hz { unimplemented!() }
    fn gclk_evsys_channel_11(&self) -> Hz { unimplemented!() }
    fn gclk_sercomx_slow(&self) -> Hz { unimplemented!() }
    fn gclk_sercom0_core(&self) -> Hz { unimplemented!() }
    fn gclk_sercom1_core(&self) -> Hz { unimplemented!() }
    fn gclk_sercom2_core(&self) -> Hz { unimplemented!() }
    fn gclk_sercom3_core(&self) -> Hz { unimplemented!() }
    fn gclk_sercom4_core(&self) -> Hz { unimplemented!() }
    fn gclk_sercom5_core(&self) -> Hz { unimplemented!() }
    fn gclk_tcc0_tcc1(&self) -> Hz { unimplemented!() }
    fn gclk_tcc2_tc3(&self) -> Hz { unimplemented!() }
    fn gclk_tc4_tc5(&self) -> Hz { unimplemented!() }
    fn gclk_tc6_tc7(&self) -> Hz { unimplemented!() }
    fn gclk_adc(&self) -> Hz { unimplemented!() }
    fn gclk_adc_dig(&self) -> Hz { unimplemented!() }
    fn gclk_20(&self) -> Hz { unimplemented!() }
    fn gclk_ac_ana(&self) -> Hz { unimplemented!() }
    fn gclk_22(&self) -> Hz { unimplemented!() }
    fn gclk_dac(&self) -> Hz { unimplemented!() }
    fn gclk_ptc(&self) -> Hz { unimplemented!() }
    fn gclk_i2s_0(&self) -> Hz { unimplemented!() }
    fn gclk_i2s_1(&self) -> Hz { unimplemented!() }
}

impl<CP> ClockFor<::wdt::Wdt> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::wdt::Wdt) -> Hz { self.gclk_wdt() }
}

impl<CP> ClockFor<::rtc::Rtc> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::rtc::Rtc) -> Hz { self.gclk_rtc() }
}

impl<CP> ClockFor<::adc::Adc> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc) -> Hz { self.gclk_adc() }
}

impl<CP> ClockFor<::dac::Dac> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dac::Dac) -> Hz { self.gclk_dac() }
}

impl<CP> ClockFor<::tcc::Tcc0> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tcc::Tcc0) -> Hz { self.gclk_tcc0_tcc1() }
}

impl<CP> ClockFor<::tcc::Tcc1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tcc::Tcc1) -> Hz { self.gclk_tcc0_tcc1() }
}

impl<CP> ClockFor<::tcc::Tcc2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tcc::Tcc2) -> Hz { self.gclk_tcc2_tc3() }
}

impl<CP> ClockFor<::tc::Tc3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tc::Tc3) -> Hz { self.gclk_tcc2_tc3() }
}

impl<CP> ClockFor<::tc::Tc4> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tc::Tc4) -> Hz { self.gclk_tc4_tc5() }
}

impl<CP> ClockFor<::tc::Tc5> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::tc::Tc5) -> Hz { self.gclk_tc4_tc5() }
}

impl<CP> ClockFor<::sercom::Sercom0> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::sercom::Sercom0) -> Hz { self.gclk_sercom0_core() }
}

impl<CP> ClockFor<::sercom::Sercom1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::sercom::Sercom1) -> Hz { self.gclk_sercom1_core() }
}

impl<CP> ClockFor<::sercom::Sercom2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::sercom::Sercom2) -> Hz { self.gclk_sercom2_core() }
}

impl<CP> ClockFor<::sercom::Sercom3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::sercom::Sercom3) -> Hz { self.gclk_sercom3_core() }
}

impl<CP> ClockFor<::sercom::Sercom4> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::sercom::Sercom4) -> Hz { self.gclk_sercom4_core() }
}

impl<CP> ClockFor<::sercom::Sercom5> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::sercom::Sercom5) -> Hz { self.gclk_sercom5_core() }
}

