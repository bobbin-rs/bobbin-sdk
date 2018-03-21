#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::sdio::*;

periph!( SDMMC, Sdmmc, SDMMC_PERIPH, SdioPeriph, 0x40012800, 0x00, 0x2f);

