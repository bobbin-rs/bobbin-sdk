#![no_std]

pub trait Mcu : Default {
    fn id(&self) -> &'static str;
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