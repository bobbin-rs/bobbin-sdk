use chip::can::{CAN0, CAN1, CAN2};
use hal::can::{self, CanDevice};
use hal::pcc;
use pin;

pub fn can0() -> CanDevice {
    // CAN0_RX = PTE4[5]
    // CAN0_TX = PTE5[5]

    let _can0_rx = pin::pte4().into_altfn(5);
    let _can0_tx = pin::pte5().into_altfn(5);

    pcc::set_can_enabled(CAN0, true);
    can::device(CAN0)
}

pub fn can0_unchecked() -> CanDevice {
    can::device(CAN0)
}

pub fn can1() -> CanDevice {
    pcc::set_can_enabled(CAN1, true);
    can::device(CAN1)
}

pub fn can1_unchecked() -> CanDevice {
    can::device(CAN1)
}

pub fn can2() -> CanDevice {
    pcc::set_can_enabled(CAN2, true);
    can::device(CAN2)
}

pub fn can2_unchecked() -> CanDevice {
    can::device(CAN2)
}
