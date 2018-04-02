#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::mpu::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( MPU, Mpu, MPU_PERIPH, MpuPeriph, MPU_OWNED, MPU_REF_COUNT, 0x4000d000, 0x00, 0x02);


