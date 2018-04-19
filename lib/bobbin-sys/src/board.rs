use system::System;

pub trait Board {
    type Mcu;    
    type Clk;
    fn id(&self) -> &'static str;
    fn sys(&self) -> &System<Self::Mcu, Self::Clk>;
    fn sys_mut(&mut self) -> &mut System<Self::Mcu, Self::Clk>;
    fn mcu(&self) -> Self::Mcu;
    fn clk(&self) -> Self::Clk;
    fn run<T, F: FnMut(&Self)->T>(&mut self, mut f: F) -> T {
        f(&*self)
    }
}