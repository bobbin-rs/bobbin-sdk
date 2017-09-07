#![allow(non_camel_case_types)]

pub use bobbin_cortexm::hal::*;

pub mod clock;
pub mod pcc;
pub mod wdog;
pub mod adc;
// pub mod lpspi;
pub mod port;

pub mod gpio {
    pub use chip::gpio::*;
    pub use kinetis_common::hal::gpio::*;
    pub use super::pcc::PccEnabled;
}


pub mod lpuart {
    pub use chip::lpuart::*;
    pub use kinetis_common::hal::lpuart::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod lpit {
    pub use chip::lpit::*;
    pub use kinetis_common::hal::lpit::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod lptmr {
    pub use chip::lptmr::*;
    pub use kinetis_common::hal::lptmr::*;
    pub use super::pcc::{PccEnabled, PccClockSource, PccClockDivider, PccClockDividerFrac};
}

pub mod ftm {
    pub use chip::ftm::*;
    pub use bobbin_common::timer::*;
    pub use kinetis_common::hal::ftm::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod flexcan {
    pub use chip::flexcan::*;
    pub use kinetis_common::hal::flexcan::*;
    pub use super::pcc::{PccEnabled, PccClockSource};
}

pub mod edma {
    pub use chip::edma::*;
    pub use kinetis_common::hal::edma::*;
}

pub mod rtc {
    pub use chip::rtc::*;
    pub use super::pcc::PccEnabled;
}

pub mod crc {
    pub use chip::crc::*;
    pub use kinetis_common::hal::crc::*;
    pub use super::pcc::PccEnabled;
}