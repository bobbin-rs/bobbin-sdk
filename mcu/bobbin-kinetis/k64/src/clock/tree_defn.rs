pub use ::bobbin_common::*;
pub use ::bobbin_common::tree::*;
pub use ::hz::Hz;


// Define Global Clocks

pub const IRC48M_HZ: Hz = Hz::from_num(48000000);
pub const IRC4M_HZ: Hz = Hz::from_num(4000000);
pub const IRC32K_HZ: Hz = Hz::from_num(32000);
pub const LPO_HZ: Hz = Hz::from_num(1000);

tree_defn! {
    id: TREE_DEFN: TreeDefn,
    clock: {
        EXTAL: Extal,
        EXTAL32: Extal32,
        IRC48M: Irc48m,
        IRC4M: Irc4m,
        IRC32K: Irc32k,
        LPO: Lpo,
        SYSTEM: System,
        BUS: Bus,
        FLEXBUS: Flexbus,
        FLASH: Flash,
        MCGIRCLK: Mcgirclk,
        ERCLK32K: Erclk32k,
        OSCERCLK: Oscerclk,
        SYSTICK: Systick,
    },
    type: {
        ::enet::Enet: Bus,
        ::crc::Crc: Bus,
        ::dmamux::Dmamux: Bus,
        ::ftm::Ftm0: Bus,
        ::ftm::Ftm1: Bus,
        ::ftm::Ftm2: Bus,
        ::pit::Pit: Bus,
        ::lptmr::Lptmr0: Bus,
        ::spi::Spi0: Bus,
        ::spi::Spi1: Bus,
        ::spi::Spi2: Bus,
        ::i2c::I2c0: Bus,
        ::i2c::I2c1: Bus,
        ::uart::Uart0: Bus,
        ::uart::Uart1: Bus,
        ::uart::Uart2: Bus,
        ::uart::Uart3: Bus,
        ::uart::Uart4: Bus,
        ::uart::Uart5: Bus,
        ::usb::Usb0: Bus,
        ::flexcan::Can0: Bus,
        ::port::Porta: Bus,
        ::port::Portb: Bus,
        ::port::Portc: Bus,
        ::port::Portd: Bus,
        ::port::Porte: Bus,
        ::adc::Adc0: Bus,
        ::adc::Adc1: Bus,
    },
}
