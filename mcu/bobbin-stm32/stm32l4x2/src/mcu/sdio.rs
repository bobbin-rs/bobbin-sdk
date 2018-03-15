#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::sdio::*;

periph!( SDMMC, Sdmmc, SDMMC_PERIPH, SdioPeriph, 0x40012800, 0x2f);

