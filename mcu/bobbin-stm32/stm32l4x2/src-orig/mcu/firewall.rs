#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::firewall::*;

periph!( FIREWALL, Firewall, FIREWALL_PERIPH, FirewallPeriph, 0x40011c00, 0x0a);

