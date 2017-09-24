use hal::cmu;

pub fn init() {
    cmu::enable_lfxo();
    cmu::init_hfxo();
}