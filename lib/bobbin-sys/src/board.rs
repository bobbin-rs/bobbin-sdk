pub trait Board {
    type Mcu;
    fn id(&self) -> &'static str;
    fn mcu(&self) -> Self::Mcu;
}