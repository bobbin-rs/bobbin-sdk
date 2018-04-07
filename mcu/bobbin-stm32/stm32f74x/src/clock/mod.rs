pub use ::bobbin_common::*;
pub use ::bobbin_common::tree::*;
pub use ::hz::Hz;

pub struct ClockTree<T>(T);


// Define Global Clocks

pub struct Hsi {}
impl Clock for Hsi { fn hz() -> Hz { Hz::from_num(16000000) } }

pub struct Lsi {}
impl Clock for Lsi { fn hz() -> Hz { Hz::from_num(32000) } }


pub trait Clocks {
    type Osc: Clock;
    type Osc32: Clock;
    fn osc() -> Hz { Self::Osc::hz() }
    fn osc32() -> Hz { Self::Osc32::hz() }
    fn hsi() -> Hz { Hz::from_num(16000000) }
    fn hse() -> Hz { Self::osc() }
    fn lsi() -> Hz { Hz::from_num(32000) }
    fn lse() -> Hz { Self::osc32() }
    fn pllclk() -> Hz { Hz::from_num(0) }
    fn pll48clk() -> Hz { Hz::from_num(0) }
    fn sysclk() -> Hz { Hz::from_num(0) }
    fn i2s() -> Hz { Hz::from_num(0) }
    fn otg_hs_scl() -> Hz { Hz::from_num(0) }
    fn hclk() -> Hz { Hz::from_num(0) }
    fn systick() -> Hz { Hz::from_num(0) }
    fn fclk() -> Hz { Hz::from_num(0) }
    fn pclk1() -> Hz { Hz::from_num(0) }
    fn pclk2() -> Hz { Hz::from_num(0) }
    fn tim_pclk1() -> Hz { Hz::from_num(0) }
    fn tim_pclk2() -> Hz { Hz::from_num(0) }
    fn rtc() -> Hz { Hz::from_num(0) }
    fn sdmmc() -> Hz { Hz::from_num(0) }
    fn hdmi_cec() -> Hz { Hz::from_num(0) }
    fn spdif() -> Hz { Hz::from_num(0) }
    fn sai1() -> Hz { Hz::from_num(0) }
    fn sai2() -> Hz { Hz::from_num(0) }
    fn eth_mactx() -> Hz { Hz::from_num(0) }
    fn eth_macrx() -> Hz { Hz::from_num(0) }
    fn eth_macrmii() -> Hz { Hz::from_num(0) }
    fn usart1() -> Hz { Hz::from_num(0) }
    fn usart2() -> Hz { Hz::from_num(0) }
    fn usart3() -> Hz { Hz::from_num(0) }
    fn uart4() -> Hz { Hz::from_num(0) }
    fn uart5() -> Hz { Hz::from_num(0) }
    fn usart6() -> Hz { Hz::from_num(0) }
    fn uart7() -> Hz { Hz::from_num(0) }
    fn uart8() -> Hz { Hz::from_num(0) }
    fn i2c1() -> Hz { Hz::from_num(0) }
    fn i2c2() -> Hz { Hz::from_num(0) }
    fn i2c3() -> Hz { Hz::from_num(0) }
    fn i2c4() -> Hz { Hz::from_num(0) }
}

impl<T> ClockFor<::syscfg::Syscfg> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::syscfg::Syscfg) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::ethernet_mac::EthernetMac> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::ethernet_mac::EthernetMac) -> Hz { T::hclk() }
}

impl<T> ClockFor<::ethernet_ptp::EthernetPtp> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::ethernet_ptp::EthernetPtp) -> Hz { T::hclk() }
}

impl<T> ClockFor<::hash::Hash> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::hash::Hash) -> Hz { T::hclk() }
}

impl<T> ClockFor<::cryp::Cryp> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::cryp::Cryp) -> Hz { T::hclk() }
}

impl<T> ClockFor<::dac::Dac> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dac::Dac) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::dcmi::Dcmi> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dcmi::Dcmi) -> Hz { T::hclk() }
}

impl<T> ClockFor<::usb_fs_host::UsbFsHost> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usb_fs_host::UsbFsHost) -> Hz { T::pll48clk() }
}

impl<T> ClockFor<::usb_fs_device::UsbFsDevice> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usb_fs_device::UsbFsDevice) -> Hz { T::pll48clk() }
}

impl<T> ClockFor<::iwdg::Iwdg> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::iwdg::Iwdg) -> Hz { T::lsi() }
}

impl<T> ClockFor<::wwdg::Wwdg> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::wwdg::Wwdg) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::crc::Crc> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::crc::Crc) -> Hz { T::hclk() }
}

impl<T> ClockFor<::tim_bas::Tim6> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_bas::Tim6) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_bas::Tim7> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_bas::Tim7) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_gen::Tim2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim2) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_gen::Tim3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim3) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_gen::Tim4> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim4) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_gen::Tim5> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim5) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_gen::Tim9> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim9) -> Hz { T::tim_pclk2() }
}

impl<T> ClockFor<::tim_gen::Tim10> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim10) -> Hz { T::tim_pclk2() }
}

impl<T> ClockFor<::tim_gen::Tim11> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim11) -> Hz { T::tim_pclk2() }
}

impl<T> ClockFor<::tim_gen::Tim12> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim12) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_gen::Tim13> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim13) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_gen::Tim14> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_gen::Tim14) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::tim_adv::Tim1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_adv::Tim1) -> Hz { T::tim_pclk2() }
}

impl<T> ClockFor<::tim_adv::Tim8> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tim_adv::Tim8) -> Hz { T::tim_pclk2() }
}

impl<T> ClockFor<::lptim::Lptim1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::lptim::Lptim1) -> Hz { T::tim_pclk1() }
}

impl<T> ClockFor<::adc::Adc1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc1) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::adc::Adc2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc2) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::adc::Adc3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc3) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::spi::Spi1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi1) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::spi::Spi2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi2) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::spi::Spi3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi3) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::spi::Spi4> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi4) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::spi::Spi5> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi5) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::spi::Spi6> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi6) -> Hz { T::pclk2() }
}

impl<T> ClockFor<::i2c::I2c1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c1) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::i2c::I2c2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c2) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::i2c::I2c3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c3) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::i2c::I2c4> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c4) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::can::Can1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::can::Can1) -> Hz { T::pclk1() }
}

impl<T> ClockFor<::gpio::Gpioa> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpioa) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpiob> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpiob) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpioc> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpioc) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpiod> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpiod) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpioe> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpioe) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpiof> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpiof) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpiog> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpiog) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpioh> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpioh) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpioi> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpioi) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpioj> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpioj) -> Hz { T::hclk() }
}

impl<T> ClockFor<::gpio::Gpiok> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gpio::Gpiok) -> Hz { T::hclk() }
}

impl<T> ClockFor<::usart::Usart1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Usart1) -> Hz { T::usart1() }
}

impl<T> ClockFor<::usart::Usart2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Usart2) -> Hz { T::usart2() }
}

impl<T> ClockFor<::usart::Usart3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Usart3) -> Hz { T::usart3() }
}

impl<T> ClockFor<::usart::Uart4> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Uart4) -> Hz { T::uart4() }
}

impl<T> ClockFor<::usart::Uart5> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Uart5) -> Hz { T::uart5() }
}

impl<T> ClockFor<::usart::Usart6> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Usart6) -> Hz { T::usart6() }
}

impl<T> ClockFor<::usart::Uart7> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Uart7) -> Hz { T::uart7() }
}

impl<T> ClockFor<::usart::Uart8> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usart::Uart8) -> Hz { T::uart8() }
}

impl<T> ClockFor<::dma::Dma1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dma::Dma1) -> Hz { T::hclk() }
}

impl<T> ClockFor<::dma::Dma2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dma::Dma2) -> Hz { T::hclk() }
}

