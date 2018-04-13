pub trait Board : Default {
    type Mcu;
    fn id(&self) -> &'static str;
    fn mcu(&self) -> Self::Mcu;
}