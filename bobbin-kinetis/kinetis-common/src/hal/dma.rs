pub use chip::dma::*;

pub trait DmaExt {

}

impl<T> DmaExt for Periph<T> {

}

pub trait DmaChExt {
}

impl<P, T> DmaChExt for Channel<P, T> {
}