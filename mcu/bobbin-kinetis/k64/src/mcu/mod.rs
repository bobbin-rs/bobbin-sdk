pub use ::bobbin_common::mcu::*;

pub mod sim;
pub mod mcg;
pub mod mpu;
pub mod osc;
pub mod rcm;
pub mod enet;
pub mod crc;
pub mod wdog;
pub mod dmamux;
pub mod edma;
pub mod ftm;
pub mod pit;
pub mod lptmr;
pub mod spi;
pub mod i2c;
pub mod uart;
pub mod usb;
pub mod flexcan;
pub mod gpio;
pub mod port;
pub mod adc;
pub mod sig;
pub mod pin;
pub mod irq;

#[derive(Debug, Default)]
pub struct K64 {}

impl Mcu for K64 {
    fn id(&self) -> &'static str { "K64" }
}

impl K64 {
    pub fn sim(&self) -> sim::Sim { sim::SIM }
    pub fn mcg(&self) -> mcg::Mcg { mcg::MCG }
    pub fn mpu(&self) -> mpu::Mpu { mpu::MPU }
    pub fn osc(&self) -> osc::Osc { osc::OSC }
    pub fn rcm(&self) -> rcm::Rcm { rcm::RCM }
    pub fn enet(&self) -> enet::Enet { enet::ENET }
    pub fn crc(&self) -> crc::Crc { crc::CRC }
    pub fn wdog(&self) -> wdog::Wdog { wdog::WDOG }
    pub fn dmamux(&self) -> dmamux::Dmamux { dmamux::DMAMUX }
    pub fn dma(&self) -> edma::Dma { edma::DMA }
    pub fn ftm0(&self) -> ftm::Ftm0 { ftm::FTM0 }
    pub fn ftm1(&self) -> ftm::Ftm1 { ftm::FTM1 }
    pub fn ftm2(&self) -> ftm::Ftm2 { ftm::FTM2 }
    pub fn pit(&self) -> pit::Pit { pit::PIT }
    pub fn lptmr0(&self) -> lptmr::Lptmr0 { lptmr::LPTMR0 }
    pub fn spi0(&self) -> spi::Spi0 { spi::SPI0 }
    pub fn spi1(&self) -> spi::Spi1 { spi::SPI1 }
    pub fn spi2(&self) -> spi::Spi2 { spi::SPI2 }
    pub fn i2c0(&self) -> i2c::I2c0 { i2c::I2C0 }
    pub fn i2c1(&self) -> i2c::I2c1 { i2c::I2C1 }
    pub fn uart0(&self) -> uart::Uart0 { uart::UART0 }
    pub fn uart1(&self) -> uart::Uart1 { uart::UART1 }
    pub fn uart2(&self) -> uart::Uart2 { uart::UART2 }
    pub fn uart3(&self) -> uart::Uart3 { uart::UART3 }
    pub fn uart4(&self) -> uart::Uart4 { uart::UART4 }
    pub fn uart5(&self) -> uart::Uart5 { uart::UART5 }
    pub fn usb0(&self) -> usb::Usb0 { usb::USB0 }
    pub fn can0(&self) -> flexcan::Can0 { flexcan::CAN0 }
    pub fn gpioa(&self) -> gpio::Gpioa { gpio::GPIOA }
    pub fn gpiob(&self) -> gpio::Gpiob { gpio::GPIOB }
    pub fn gpioc(&self) -> gpio::Gpioc { gpio::GPIOC }
    pub fn gpiod(&self) -> gpio::Gpiod { gpio::GPIOD }
    pub fn gpioe(&self) -> gpio::Gpioe { gpio::GPIOE }
    pub fn porta(&self) -> port::Porta { port::PORTA }
    pub fn portb(&self) -> port::Portb { port::PORTB }
    pub fn portc(&self) -> port::Portc { port::PORTC }
    pub fn portd(&self) -> port::Portd { port::PORTD }
    pub fn porte(&self) -> port::Porte { port::PORTE }
    pub fn adc0(&self) -> adc::Adc0 { adc::ADC0 }
    pub fn adc1(&self) -> adc::Adc1 { adc::ADC1 }
}

impl Get<sim::Sim> for K64 {
    fn get(&self) -> sim::Sim { sim::SIM }
}

impl GetPeriph<sim::SimPeriph> for K64 {
    fn get_periph(&self) -> sim::SimPeriph { sim::SIM_PERIPH }
}

impl GetPeriphInstance<sim::SimPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<sim::SimPeriph> {
        match index { 
            0 => Some(sim::SIM_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<mcg::Mcg> for K64 {
    fn get(&self) -> mcg::Mcg { mcg::MCG }
}

impl GetPeriph<mcg::McgPeriph> for K64 {
    fn get_periph(&self) -> mcg::McgPeriph { mcg::MCG_PERIPH }
}

impl GetPeriphInstance<mcg::McgPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<mcg::McgPeriph> {
        match index { 
            0 => Some(mcg::MCG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<mpu::Mpu> for K64 {
    fn get(&self) -> mpu::Mpu { mpu::MPU }
}

impl GetPeriph<mpu::MpuPeriph> for K64 {
    fn get_periph(&self) -> mpu::MpuPeriph { mpu::MPU_PERIPH }
}

impl GetPeriphInstance<mpu::MpuPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<mpu::MpuPeriph> {
        match index { 
            0 => Some(mpu::MPU_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<osc::Osc> for K64 {
    fn get(&self) -> osc::Osc { osc::OSC }
}

impl GetPeriph<osc::OscPeriph> for K64 {
    fn get_periph(&self) -> osc::OscPeriph { osc::OSC_PERIPH }
}

impl GetPeriphInstance<osc::OscPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<osc::OscPeriph> {
        match index { 
            0 => Some(osc::OSC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rcm::Rcm> for K64 {
    fn get(&self) -> rcm::Rcm { rcm::RCM }
}

impl GetPeriph<rcm::RcmPeriph> for K64 {
    fn get_periph(&self) -> rcm::RcmPeriph { rcm::RCM_PERIPH }
}

impl GetPeriphInstance<rcm::RcmPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<rcm::RcmPeriph> {
        match index { 
            0 => Some(rcm::RCM_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<enet::Enet> for K64 {
    fn get(&self) -> enet::Enet { enet::ENET }
}

impl GetPeriph<enet::EnetPeriph> for K64 {
    fn get_periph(&self) -> enet::EnetPeriph { enet::ENET_PERIPH }
}

impl GetPeriphInstance<enet::EnetPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<enet::EnetPeriph> {
        match index { 
            0 => Some(enet::ENET_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<crc::Crc> for K64 {
    fn get(&self) -> crc::Crc { crc::CRC }
}

impl GetPeriph<crc::CrcPeriph> for K64 {
    fn get_periph(&self) -> crc::CrcPeriph { crc::CRC_PERIPH }
}

impl GetPeriphInstance<crc::CrcPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<crc::CrcPeriph> {
        match index {
            0 => Some(crc::CRC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<wdog::Wdog> for K64 {
    fn get(&self) -> wdog::Wdog { wdog::WDOG }
}

impl GetPeriph<wdog::WdogPeriph> for K64 {
    fn get_periph(&self) -> wdog::WdogPeriph { wdog::WDOG_PERIPH }
}

impl GetPeriphInstance<wdog::WdogPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<wdog::WdogPeriph> {
        match index {
            0 => Some(wdog::WDOG_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dmamux::Dmamux> for K64 {
    fn get(&self) -> dmamux::Dmamux { dmamux::DMAMUX }
}

impl GetPeriph<dmamux::DmamuxPeriph> for K64 {
    fn get_periph(&self) -> dmamux::DmamuxPeriph { dmamux::DMAMUX_PERIPH }
}

impl GetPeriphInstance<dmamux::DmamuxPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<dmamux::DmamuxPeriph> {
        match index {
            0 => Some(dmamux::DMAMUX_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<edma::Dma> for K64 {
    fn get(&self) -> edma::Dma { edma::DMA }
}

impl GetPeriph<edma::EdmaPeriph> for K64 {
    fn get_periph(&self) -> edma::EdmaPeriph { edma::DMA_PERIPH }
}

impl GetPeriphInstance<edma::EdmaPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<edma::EdmaPeriph> {
        match index {
            0 => Some(edma::DMA_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<ftm::Ftm0> for K64 {
    fn get(&self) -> ftm::Ftm0 { ftm::FTM0 }
}

impl Get<ftm::Ftm1> for K64 {
    fn get(&self) -> ftm::Ftm1 { ftm::FTM1 }
}

impl Get<ftm::Ftm2> for K64 {
    fn get(&self) -> ftm::Ftm2 { ftm::FTM2 }
}

impl GetPeriphInstance<ftm::FtmPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<ftm::FtmPeriph> {
        match index {
            0 => Some(ftm::FTM0_PERIPH),
            1 => Some(ftm::FTM1_PERIPH),
            2 => Some(ftm::FTM2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<pit::Pit> for K64 {
    fn get(&self) -> pit::Pit { pit::PIT }
}

impl GetPeriph<pit::PitPeriph> for K64 {
    fn get_periph(&self) -> pit::PitPeriph { pit::PIT_PERIPH }
}

impl GetPeriphInstance<pit::PitPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<pit::PitPeriph> {
        match index {
            0 => Some(pit::PIT_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<lptmr::Lptmr0> for K64 {
    fn get(&self) -> lptmr::Lptmr0 { lptmr::LPTMR0 }
}

impl GetPeriph<lptmr::LptmrPeriph> for K64 {
    fn get_periph(&self) -> lptmr::LptmrPeriph { lptmr::LPTMR0_PERIPH }
}

impl GetPeriphInstance<lptmr::LptmrPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<lptmr::LptmrPeriph> {
        match index {
            0 => Some(lptmr::LPTMR0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<spi::Spi0> for K64 {
    fn get(&self) -> spi::Spi0 { spi::SPI0 }
}

impl Get<spi::Spi1> for K64 {
    fn get(&self) -> spi::Spi1 { spi::SPI1 }
}

impl Get<spi::Spi2> for K64 {
    fn get(&self) -> spi::Spi2 { spi::SPI2 }
}

impl GetPeriphInstance<spi::SpiPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<spi::SpiPeriph> {
        match index {
            0 => Some(spi::SPI0_PERIPH),
            1 => Some(spi::SPI1_PERIPH),
            2 => Some(spi::SPI2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<i2c::I2c0> for K64 {
    fn get(&self) -> i2c::I2c0 { i2c::I2C0 }
}

impl Get<i2c::I2c1> for K64 {
    fn get(&self) -> i2c::I2c1 { i2c::I2C1 }
}

impl GetPeriphInstance<i2c::I2cPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<i2c::I2cPeriph> {
        match index {
            0 => Some(i2c::I2C0_PERIPH),
            1 => Some(i2c::I2C1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<uart::Uart0> for K64 {
    fn get(&self) -> uart::Uart0 { uart::UART0 }
}

impl Get<uart::Uart1> for K64 {
    fn get(&self) -> uart::Uart1 { uart::UART1 }
}

impl Get<uart::Uart2> for K64 {
    fn get(&self) -> uart::Uart2 { uart::UART2 }
}

impl Get<uart::Uart3> for K64 {
    fn get(&self) -> uart::Uart3 { uart::UART3 }
}

impl Get<uart::Uart4> for K64 {
    fn get(&self) -> uart::Uart4 { uart::UART4 }
}

impl Get<uart::Uart5> for K64 {
    fn get(&self) -> uart::Uart5 { uart::UART5 }
}

impl GetPeriphInstance<uart::UartPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<uart::UartPeriph> {
        match index {
            0 => Some(uart::UART0_PERIPH),
            1 => Some(uart::UART1_PERIPH),
            2 => Some(uart::UART2_PERIPH),
            3 => Some(uart::UART3_PERIPH),
            4 => Some(uart::UART4_PERIPH),
            5 => Some(uart::UART5_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 6 }
}

impl Get<usb::Usb0> for K64 {
    fn get(&self) -> usb::Usb0 { usb::USB0 }
}

impl GetPeriph<usb::UsbPeriph> for K64 {
    fn get_periph(&self) -> usb::UsbPeriph { usb::USB0_PERIPH }
}

impl GetPeriphInstance<usb::UsbPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<usb::UsbPeriph> {
        match index {
            0 => Some(usb::USB0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<flexcan::Can0> for K64 {
    fn get(&self) -> flexcan::Can0 { flexcan::CAN0 }
}

impl GetPeriph<flexcan::FlexcanPeriph> for K64 {
    fn get_periph(&self) -> flexcan::FlexcanPeriph { flexcan::CAN0_PERIPH }
}

impl GetPeriphInstance<flexcan::FlexcanPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<flexcan::FlexcanPeriph> {
        match index {
            0 => Some(flexcan::CAN0_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<gpio::Gpioa> for K64 {
    fn get(&self) -> gpio::Gpioa { gpio::GPIOA }
}

impl Get<gpio::Gpiob> for K64 {
    fn get(&self) -> gpio::Gpiob { gpio::GPIOB }
}

impl Get<gpio::Gpioc> for K64 {
    fn get(&self) -> gpio::Gpioc { gpio::GPIOC }
}

impl Get<gpio::Gpiod> for K64 {
    fn get(&self) -> gpio::Gpiod { gpio::GPIOD }
}

impl Get<gpio::Gpioe> for K64 {
    fn get(&self) -> gpio::Gpioe { gpio::GPIOE }
}

impl GetPeriphInstance<gpio::GpioPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<gpio::GpioPeriph> {
        match index {
            0 => Some(gpio::GPIOA_PERIPH),
            1 => Some(gpio::GPIOB_PERIPH),
            2 => Some(gpio::GPIOC_PERIPH),
            3 => Some(gpio::GPIOD_PERIPH),
            4 => Some(gpio::GPIOE_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 5 }
}

impl Get<port::Porta> for K64 {
    fn get(&self) -> port::Porta { port::PORTA }
}

impl Get<port::Portb> for K64 {
    fn get(&self) -> port::Portb { port::PORTB }
}

impl Get<port::Portc> for K64 {
    fn get(&self) -> port::Portc { port::PORTC }
}

impl Get<port::Portd> for K64 {
    fn get(&self) -> port::Portd { port::PORTD }
}

impl Get<port::Porte> for K64 {
    fn get(&self) -> port::Porte { port::PORTE }
}

impl GetPeriphInstance<port::PortPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<port::PortPeriph> {
        match index {
            0 => Some(port::PORTA_PERIPH),
            1 => Some(port::PORTB_PERIPH),
            2 => Some(port::PORTC_PERIPH),
            3 => Some(port::PORTD_PERIPH),
            4 => Some(port::PORTE_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 5 }
}

impl Get<adc::Adc0> for K64 {
    fn get(&self) -> adc::Adc0 { adc::ADC0 }
}

impl Get<adc::Adc1> for K64 {
    fn get(&self) -> adc::Adc1 { adc::ADC1 }
}

impl GetPeriphInstance<adc::AdcPeriph> for K64 {
    fn get_periph_instance(&self, index: usize) -> Option<adc::AdcPeriph> {
        match index {
            0 => Some(adc::ADC0_PERIPH),
            1 => Some(adc::ADC1_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

