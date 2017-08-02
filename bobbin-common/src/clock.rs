pub type Hz = Option<u32>;

pub const HZ: Hz = Some(1);
pub const KHZ: Hz = Some(1_000);
pub const MHZ: Hz = Some(1_000_000);
pub const GHZ: Hz = Some(1_000_000_000);

pub trait Systick {
    fn systick(&self) -> Hz;
}