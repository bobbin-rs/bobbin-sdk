use ::hal::clock;

pub fn init() {
    clock::set_clock(3, 96, 0, 4, 0);
    clock::set_sysclk_hz(120_000_000);
}

pub fn sysclk_hz() -> u32 {
    clock::sysclk_hz()
}