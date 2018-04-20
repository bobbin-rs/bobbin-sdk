use core::result::Result;

pub trait UnwrapOrAbort<T> {
    fn unwrap_or_abort(self, msg: &'static str) -> T;
}

impl<T, E> UnwrapOrAbort<T> for Result<T, E> {
    #[inline]
    fn unwrap_or_abort(self, msg: &'static str) -> T {
        self.unwrap_or_else(|_| {
            abort!(msg);
        })
    }
}