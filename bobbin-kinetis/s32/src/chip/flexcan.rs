#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::flexcan::*;

periph!( CAN0, Can0, _CAN0, FlexcanPeriph, 0x40024000);
periph!( CAN1, Can1, _CAN1, FlexcanPeriph, 0x40025000);
periph!( CAN2, Can2, _CAN2, FlexcanPeriph, 0x4002b000);






pub trait IrqCan<T> {
   fn irq_can(&self) -> T;
}

pub trait IrqCanError<T> {
   fn irq_can_error(&self) -> T;
}

pub trait IrqCanWakeUp<T> {
   fn irq_can_wake_up(&self) -> T;
}

pub trait IrqCan015<T> {
   fn irq_can_0_15(&self) -> T;
}

pub trait IrqCan1631<T> {
   fn irq_can_16_31(&self) -> T;
}

impl IrqCan<super::irq::IrqCan0Ored> for Can0 {
   fn irq_can(&self) -> super::irq::IrqCan0Ored { super::irq::IRQ_CAN0_ORED }
}

impl IrqCanError<super::irq::IrqCan0Error> for Can0 {
   fn irq_can_error(&self) -> super::irq::IrqCan0Error { super::irq::IRQ_CAN0_ERROR }
}

impl IrqCanWakeUp<super::irq::IrqCan0WakeUp> for Can0 {
   fn irq_can_wake_up(&self) -> super::irq::IrqCan0WakeUp { super::irq::IRQ_CAN0_WAKE_UP }
}

impl IrqCan015<super::irq::IrqCan0Ored015Mb> for Can0 {
   fn irq_can_0_15(&self) -> super::irq::IrqCan0Ored015Mb { super::irq::IRQ_CAN0_ORED_0_15_MB }
}

impl IrqCan1631<super::irq::IrqCan0Ored1631Mb> for Can0 {
   fn irq_can_16_31(&self) -> super::irq::IrqCan0Ored1631Mb { super::irq::IRQ_CAN0_ORED_16_31_MB }
}

impl IrqCan<super::irq::IrqCan1Ored> for Can1 {
   fn irq_can(&self) -> super::irq::IrqCan1Ored { super::irq::IRQ_CAN1_ORED }
}

impl IrqCanError<super::irq::IrqCan1Error> for Can1 {
   fn irq_can_error(&self) -> super::irq::IrqCan1Error { super::irq::IRQ_CAN1_ERROR }
}

impl IrqCan015<super::irq::IrqCan1Ored015Mb> for Can1 {
   fn irq_can_0_15(&self) -> super::irq::IrqCan1Ored015Mb { super::irq::IRQ_CAN1_ORED_0_15_MB }
}

impl IrqCan<super::irq::IrqCan2Ored> for Can2 {
   fn irq_can(&self) -> super::irq::IrqCan2Ored { super::irq::IRQ_CAN2_ORED }
}

impl IrqCanError<super::irq::IrqCan2Error> for Can2 {
   fn irq_can_error(&self) -> super::irq::IrqCan2Error { super::irq::IRQ_CAN2_ERROR }
}

impl IrqCan015<super::irq::IrqCan2Ored015Mb> for Can2 {
   fn irq_can_0_15(&self) -> super::irq::IrqCan2Ored015Mb { super::irq::IRQ_CAN2_ORED_0_15_MB }
}

