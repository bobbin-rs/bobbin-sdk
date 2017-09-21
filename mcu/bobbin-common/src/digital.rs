pub trait DigitalOutput {
    fn output(&self) -> bool;
    fn set_output(&self, value: bool) -> &Self;
    fn toggle_output(&self) -> &Self;
}

pub trait DigitalInput {
    fn input(&self) -> bool;
}