use chip::lpspi::{LPSPI0, LPSPI1, LPSPI2};
use hal::lpspi::{self, LpspiDevice};
use hal::pcc;
use pin;

pub fn lpspi0() -> LpspiDevice {
    pcc::set_lpspi_enabled(LPSPI0, true, pcc::Source::SPLLDIV2);
    lpspi::device(LPSPI0)
}

pub fn lpspi0_unchecked() -> LpspiDevice {
    lpspi::device(LPSPI0)
}

pub fn lpspi1() -> LpspiDevice {
    let _sck = pin::ptb14().into_alt_fn(3);
    let _miso = pin::ptb15().into_alt_fn(3);
    let _mosi = pin::ptb16().into_alt_fn(3);
    let _cs3 = pin::ptb17().into_alt_fn(3);    
    pcc::set_lpspi_enabled(LPSPI1, true, pcc::Source::SPLLDIV2);
    lpspi::device(LPSPI1)
}

pub fn lpspi1_unchecked() -> LpspiDevice {
    lpspi::device(LPSPI1)
}

pub fn lpspi2() -> LpspiDevice {
    pcc::set_lpspi_enabled(LPSPI2, true, pcc::Source::SPLLDIV2);
    lpspi::device(LPSPI2)
}

pub fn lpspi2_unchecked() -> LpspiDevice {
    lpspi::device(LPSPI2)
}
