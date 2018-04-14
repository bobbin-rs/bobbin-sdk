pub use ::bobbin_common::*;
pub use ::bobbin_common::tree::*;
pub use ::hz::Hz;

#[derive(Default)]
pub struct Clocks<CP: ClockProvider> { provider: CP }

impl<CP: ClockProvider> ::core::ops::Deref for Clocks<CP> {
    type Target = CP;
    fn deref(&self) -> &CP { &self.provider }
}


// Define Global Clocks

#[derive(Default)]
pub struct Irc48m {}
impl Clock for Irc48m { fn hz() -> Hz { Hz::from_num(48000000) } }

#[derive(Default)]
pub struct Irc4m {}
impl Clock for Irc4m { fn hz() -> Hz { Hz::from_num(4000000) } }

#[derive(Default)]
pub struct Irc32k {}
impl Clock for Irc32k { fn hz() -> Hz { Hz::from_num(32000) } }

#[derive(Default)]
pub struct Lpo {}
impl Clock for Lpo { fn hz() -> Hz { Hz::from_num(1000) } }


pub trait ClockProvider : Default {
    type Extal: Clock;
    type Extal32: Clock;
    fn extal(&self) -> Hz { Self::Extal::hz() }
    fn extal32(&self) -> Hz { Self::Extal32::hz() }
    fn irc48m(&self) -> Hz { Hz::from_num(48000000) }
    fn irc4m(&self) -> Hz { Hz::from_num(4000000) }
    fn irc32k(&self) -> Hz { Hz::from_num(32000) }
    fn lpo(&self) -> Hz { Hz::from_num(1000) }
    fn system(&self) -> Hz { unimplemented!() }
    fn bus(&self) -> Hz { unimplemented!() }
    fn flexbus(&self) -> Hz { unimplemented!() }
    fn flash(&self) -> Hz { unimplemented!() }
    fn mcgirclk(&self) -> Hz { unimplemented!() }
    fn erclk32k(&self) -> Hz { unimplemented!() }
    fn oscerclk(&self) -> Hz { unimplemented!() }
    fn systick(&self) -> Hz { unimplemented!() }
}

impl<CP> ClockFor<::enet::Enet> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::enet::Enet) -> Hz { self.bus() }
}

impl<CP> ClockFor<::crc::Crc> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::crc::Crc) -> Hz { self.bus() }
}

impl<CP> ClockFor<::dmamux::Dmamux> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dmamux::Dmamux) -> Hz { self.bus() }
}

impl<CP> ClockFor<::ftm::Ftm0> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::ftm::Ftm0) -> Hz { self.bus() }
}

impl<CP> ClockFor<::ftm::Ftm1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::ftm::Ftm1) -> Hz { self.bus() }
}

impl<CP> ClockFor<::ftm::Ftm2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::ftm::Ftm2) -> Hz { self.bus() }
}

impl<CP> ClockFor<::pit::Pit> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::pit::Pit) -> Hz { self.bus() }
}

impl<CP> ClockFor<::lptmr::Lptmr0> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::lptmr::Lptmr0) -> Hz { self.bus() }
}

impl<CP> ClockFor<::spi::Spi0> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::spi::Spi0) -> Hz { self.bus() }
}

impl<CP> ClockFor<::spi::Spi1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::spi::Spi1) -> Hz { self.bus() }
}

impl<CP> ClockFor<::spi::Spi2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::spi::Spi2) -> Hz { self.bus() }
}

impl<CP> ClockFor<::i2c::I2c0> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::i2c::I2c0) -> Hz { self.bus() }
}

impl<CP> ClockFor<::i2c::I2c1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::i2c::I2c1) -> Hz { self.bus() }
}

impl<CP> ClockFor<::uart::Uart0> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::uart::Uart0) -> Hz { self.system() }
}

impl<CP> ClockFor<::uart::Uart1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::uart::Uart1) -> Hz { self.system() }
}

impl<CP> ClockFor<::uart::Uart2> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::uart::Uart2) -> Hz { self.bus() }
}

impl<CP> ClockFor<::uart::Uart3> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::uart::Uart3) -> Hz { self.bus() }
}

impl<CP> ClockFor<::uart::Uart4> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::uart::Uart4) -> Hz { self.bus() }
}

impl<CP> ClockFor<::uart::Uart5> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::uart::Uart5) -> Hz { self.bus() }
}

impl<CP> ClockFor<::usb::Usb0> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::usb::Usb0) -> Hz { self.bus() }
}

impl<CP> ClockFor<::flexcan::Can0> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::flexcan::Can0) -> Hz { self.bus() }
}

impl<CP> ClockFor<::dac::Dac0> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dac::Dac0) -> Hz { self.bus() }
}

impl<CP> ClockFor<::dac::Dac1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::dac::Dac1) -> Hz { self.bus() }
}

impl<CP> ClockFor<::port::Porta> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::port::Porta) -> Hz { self.bus() }
}

impl<CP> ClockFor<::port::Portb> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::port::Portb) -> Hz { self.bus() }
}

impl<CP> ClockFor<::port::Portc> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::port::Portc) -> Hz { self.bus() }
}

impl<CP> ClockFor<::port::Portd> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::port::Portd) -> Hz { self.bus() }
}

impl<CP> ClockFor<::port::Porte> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::port::Porte) -> Hz { self.bus() }
}

impl<CP> ClockFor<::adc::Adc0> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc0) -> Hz { self.bus() }
}

impl<CP> ClockFor<::adc::Adc1> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::adc::Adc1) -> Hz { self.bus() }
}

