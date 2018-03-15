#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::tpm::*;

periph!( TPM0, Tpm0, TPM0_PERIPH, TpmPeriph, 0x40038000, 0x07);
periph!( TPM1, Tpm1, TPM1_PERIPH, TpmPeriph, 0x40039000, 0x08);
periph!( TPM2, Tpm2, TPM2_PERIPH, TpmPeriph, 0x4003a000, 0x09);

channel!(TPM0_CH0, Tpm0Ch0, TPM0, Tpm0, TPM0_CH0_CH, TpmCh, TPM0_PERIPH, 0);
channel!(TPM0_CH1, Tpm0Ch1, TPM0, Tpm0, TPM0_CH1_CH, TpmCh, TPM0_PERIPH, 1);
channel!(TPM0_CH2, Tpm0Ch2, TPM0, Tpm0, TPM0_CH2_CH, TpmCh, TPM0_PERIPH, 2);
channel!(TPM0_CH3, Tpm0Ch3, TPM0, Tpm0, TPM0_CH3_CH, TpmCh, TPM0_PERIPH, 3);
channel!(TPM0_CH4, Tpm0Ch4, TPM0, Tpm0, TPM0_CH4_CH, TpmCh, TPM0_PERIPH, 4);
channel!(TPM0_CH5, Tpm0Ch5, TPM0, Tpm0, TPM0_CH5_CH, TpmCh, TPM0_PERIPH, 5);
channel!(TPM1_CH0, Tpm1Ch0, TPM1, Tpm1, TPM1_CH0_CH, TpmCh, TPM1_PERIPH, 0);
channel!(TPM1_CH1, Tpm1Ch1, TPM1, Tpm1, TPM1_CH1_CH, TpmCh, TPM1_PERIPH, 1);
channel!(TPM1_CH2, Tpm1Ch2, TPM1, Tpm1, TPM1_CH2_CH, TpmCh, TPM1_PERIPH, 2);
channel!(TPM1_CH3, Tpm1Ch3, TPM1, Tpm1, TPM1_CH3_CH, TpmCh, TPM1_PERIPH, 3);
channel!(TPM1_CH4, Tpm1Ch4, TPM1, Tpm1, TPM1_CH4_CH, TpmCh, TPM1_PERIPH, 4);
channel!(TPM1_CH5, Tpm1Ch5, TPM1, Tpm1, TPM1_CH5_CH, TpmCh, TPM1_PERIPH, 5);
channel!(TPM2_CH0, Tpm2Ch0, TPM2, Tpm2, TPM2_CH0_CH, TpmCh, TPM2_PERIPH, 0);
channel!(TPM2_CH1, Tpm2Ch1, TPM2, Tpm2, TPM2_CH1_CH, TpmCh, TPM2_PERIPH, 1);
channel!(TPM2_CH2, Tpm2Ch2, TPM2, Tpm2, TPM2_CH2_CH, TpmCh, TPM2_PERIPH, 2);
channel!(TPM2_CH3, Tpm2Ch3, TPM2, Tpm2, TPM2_CH3_CH, TpmCh, TPM2_PERIPH, 3);
channel!(TPM2_CH4, Tpm2Ch4, TPM2, Tpm2, TPM2_CH4_CH, TpmCh, TPM2_PERIPH, 4);
channel!(TPM2_CH5, Tpm2Ch5, TPM2, Tpm2, TPM2_CH5_CH, TpmCh, TPM2_PERIPH, 5);
