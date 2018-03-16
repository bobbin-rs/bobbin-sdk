#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::tcc::*;

// //! Timer/Counter for Control Applications

periph!( TCC0, Tcc0, TCC0_PERIPH, TccPeriph, 0x42002000, 0x08);
periph!( TCC1, Tcc1, TCC1_PERIPH, TccPeriph, 0x42002400, 0x09);
periph!( TCC2, Tcc2, TCC2_PERIPH, TccPeriph, 0x42002800, 0x0a);

pub struct TccCh { pub periph: TccPeriph, pub index: usize }
channel!(TCC0_CH0, Tcc0Ch0, TCC0, Tcc0, TCC0_CH0_CH, TccCh, TCC0_PERIPH, 0);
channel!(TCC0_CH1, Tcc0Ch1, TCC0, Tcc0, TCC0_CH1_CH, TccCh, TCC0_PERIPH, 1);
channel!(TCC0_CH2, Tcc0Ch2, TCC0, Tcc0, TCC0_CH2_CH, TccCh, TCC0_PERIPH, 2);
channel!(TCC0_CH3, Tcc0Ch3, TCC0, Tcc0, TCC0_CH3_CH, TccCh, TCC0_PERIPH, 3);
channel!(TCC1_CH0, Tcc1Ch0, TCC1, Tcc1, TCC1_CH0_CH, TccCh, TCC1_PERIPH, 0);
channel!(TCC1_CH1, Tcc1Ch1, TCC1, Tcc1, TCC1_CH1_CH, TccCh, TCC1_PERIPH, 1);
channel!(TCC2_CH0, Tcc2Ch0, TCC2, Tcc2, TCC2_CH0_CH, TccCh, TCC2_PERIPH, 0);
channel!(TCC2_CH1, Tcc2Ch1, TCC2, Tcc2, TCC2_CH1_CH, TccCh, TCC2_PERIPH, 1);