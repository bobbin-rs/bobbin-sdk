use hal::gpio;
pub use hal::pin::*;

pub fn d0() -> gpio::PinUnknown { pa10() }
pub fn d1() -> gpio::PinUnknown { pa9() }
pub fn d2() -> gpio::PinUnknown { pa12() }
pub fn d3() -> gpio::PinUnknown { pb0() }
pub fn d4() -> gpio::PinUnknown { pb7() }
pub fn d5() -> gpio::PinUnknown { pb6() }
pub fn d6() -> gpio::PinUnknown { pb1() }
pub fn d7() -> gpio::PinUnknown { pc14() }
pub fn d8() -> gpio::PinUnknown { pc15() }
pub fn d9() -> gpio::PinUnknown { pa8() }
pub fn d10() -> gpio::PinUnknown { pa11() }
pub fn d11() -> gpio::PinUnknown { pb5() }
pub fn d12() -> gpio::PinUnknown { pb4() }
pub fn d13() -> gpio::PinUnknown { pb3() }
pub fn a0() -> gpio::PinUnknown { pa0() }
pub fn a1() -> gpio::PinUnknown { pa1() }
pub fn a2() -> gpio::PinUnknown { pa3() }
pub fn a3() -> gpio::PinUnknown { pa4() }
pub fn a4() -> gpio::PinUnknown { pa5() }
pub fn a5() -> gpio::PinUnknown { pa6() }
pub fn a6() -> gpio::PinUnknown { pa7() }
pub fn a7() -> gpio::PinUnknown { pa2() }
