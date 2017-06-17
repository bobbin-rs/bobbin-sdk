pub use chip::tcc::*;

pub trait TccExt {
}

impl<T> TccExt for Periph<T> {
}

pub trait TccChExt {

}

impl<P, T> TccChExt for Channel<P, T> {

}