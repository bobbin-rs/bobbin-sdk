#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::firewall::*;

periph!( FIREWALL, Firewall, FIREWALL_PERIPH, FirewallPeriph, 0x40011c00, 0x00, 0x0a);

