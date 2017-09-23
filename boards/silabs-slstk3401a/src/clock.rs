use hal::cmu;

pub fn init() {
    cmu::init_hfxo();
}