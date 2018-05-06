pub trait Mcu : IrqEnable + GetActiveIrq + Pend + Sleep {
    fn id(&self) -> &'static str;
}

pub trait GetActiveIrq {
    fn get_active_irq() -> u8;
}

pub trait IrqEnable {
    fn irq_enabled(u8) -> bool;
    fn irq_enable(u8);
    fn irq_disable(u8);
}

pub trait Pend {
    fn pend();
}

pub trait Sleep {
    fn sleep();
}

pub trait Get<T> {
    fn get(&self) -> T;
}

pub trait GetPeriph<T> {
    fn get_periph(&self) -> T;
}

pub trait GetPeriphInstance<T> {
    fn get_periph_instance(&self, index: usize) -> Option<T>;
    fn get_periph_instance_count(&self) -> usize;
}