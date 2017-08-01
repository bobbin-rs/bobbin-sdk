pub use chip::udma::*;
pub use super::sysctl::SysctlEnabled;

pub trait UdmaExt {
}

impl<T> UdmaExt for Periph<T> {    
}

pub trait UdmaChExt {    
}

impl<P, T> UdmaChExt for Channel<P, T> {
}