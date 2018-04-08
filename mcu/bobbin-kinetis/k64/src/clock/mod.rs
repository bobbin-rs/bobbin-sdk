pub use ::bobbin_common::*;
pub use ::bobbin_common::tree::*;
pub use ::hz::Hz;

pub struct ClockTree<T>(T);


// Define Global Clocks

pub struct Irc48m {}
impl Clock for Irc48m { fn hz() -> Hz { Hz::from_num(48000000) } }

pub struct Irc4m {}
impl Clock for Irc4m { fn hz() -> Hz { Hz::from_num(4000000) } }

pub struct Irc32k {}
impl Clock for Irc32k { fn hz() -> Hz { Hz::from_num(32000) } }

pub struct Lpo {}
impl Clock for Lpo { fn hz() -> Hz { Hz::from_num(1000) } }


pub trait Clocks {
    type Extal: Clock;
    type Extal32: Clock;
    fn extal() -> Hz { Self::Extal::hz() }
    fn extal32() -> Hz { Self::Extal32::hz() }
    fn irc48m() -> Hz { Hz::from_num(48000000) }
    fn irc4m() -> Hz { Hz::from_num(4000000) }
    fn irc32k() -> Hz { Hz::from_num(32000) }
    fn lpo() -> Hz { Hz::from_num(1000) }
    fn system() -> Hz { Hz::from_num(0) }
    fn bus() -> Hz { Hz::from_num(0) }
    fn flexbus() -> Hz { Hz::from_num(0) }
    fn flash() -> Hz { Hz::from_num(0) }
    fn mcgirclk() -> Hz { Hz::from_num(0) }
    fn erclk32k() -> Hz { Hz::from_num(0) }
    fn oscerclk() -> Hz { Hz::from_num(0) }
    fn systick() -> Hz { Hz::from_num(0) }
}

impl<T> ClockFor<::enet::Enet> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::enet::Enet) -> Hz { T::bus() }
}

impl<T> ClockFor<::crc::Crc> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::crc::Crc) -> Hz { T::bus() }
}

impl<T> ClockFor<::dmamux::Dmamux> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dmamux::Dmamux) -> Hz { T::bus() }
}

impl<T> ClockFor<::ftm::Ftm0> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::ftm::Ftm0) -> Hz { T::bus() }
}

impl<T> ClockFor<::ftm::Ftm1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::ftm::Ftm1) -> Hz { T::bus() }
}

impl<T> ClockFor<::ftm::Ftm2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::ftm::Ftm2) -> Hz { T::bus() }
}

impl<T> ClockFor<::pit::Pit> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::pit::Pit) -> Hz { T::bus() }
}

impl<T> ClockFor<::lptmr::Lptmr0> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::lptmr::Lptmr0) -> Hz { T::bus() }
}

impl<T> ClockFor<::spi::Spi0> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi0) -> Hz { T::bus() }
}

impl<T> ClockFor<::spi::Spi1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi1) -> Hz { T::bus() }
}

impl<T> ClockFor<::spi::Spi2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::spi::Spi2) -> Hz { T::bus() }
}

impl<T> ClockFor<::i2c::I2c0> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c0) -> Hz { T::bus() }
}

impl<T> ClockFor<::i2c::I2c1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::i2c::I2c1) -> Hz { T::bus() }
}

impl<T> ClockFor<::uart::Uart0> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::uart::Uart0) -> Hz { T::system() }
}

impl<T> ClockFor<::uart::Uart1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::uart::Uart1) -> Hz { T::system() }
}

impl<T> ClockFor<::uart::Uart2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::uart::Uart2) -> Hz { T::bus() }
}

impl<T> ClockFor<::uart::Uart3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::uart::Uart3) -> Hz { T::bus() }
}

impl<T> ClockFor<::uart::Uart4> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::uart::Uart4) -> Hz { T::bus() }
}

impl<T> ClockFor<::uart::Uart5> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::uart::Uart5) -> Hz { T::bus() }
}

impl<T> ClockFor<::usb::Usb0> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::usb::Usb0) -> Hz { T::bus() }
}

impl<T> ClockFor<::flexcan::Can0> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::flexcan::Can0) -> Hz { T::bus() }
}

impl<T> ClockFor<::dac::Dac0> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dac::Dac0) -> Hz { T::bus() }
}

impl<T> ClockFor<::dac::Dac1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dac::Dac1) -> Hz { T::bus() }
}

impl<T> ClockFor<::port::Porta> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::port::Porta) -> Hz { T::bus() }
}

impl<T> ClockFor<::port::Portb> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::port::Portb) -> Hz { T::bus() }
}

impl<T> ClockFor<::port::Portc> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::port::Portc) -> Hz { T::bus() }
}

impl<T> ClockFor<::port::Portd> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::port::Portd) -> Hz { T::bus() }
}

impl<T> ClockFor<::port::Porte> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::port::Porte) -> Hz { T::bus() }
}

impl<T> ClockFor<::adc::Adc0> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc0) -> Hz { T::bus() }
}

impl<T> ClockFor<::adc::Adc1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc1) -> Hz { T::bus() }
}

