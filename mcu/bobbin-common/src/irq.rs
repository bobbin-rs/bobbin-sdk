#[macro_export]
macro_rules! irq {
    ($id:ident, $ty:ident, $num:expr) => {
        pub const $id: $ty = $ty {};
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        pub struct $ty {}
        impl $crate::irq::IrqNumber for $ty {
            fn number(&self) -> u8 { $num }
        }
    }    
}

#[macro_export]
macro_rules! xirq {
    ($id:ident, $ty:ident, $num:expr) => (
        pub const $id: $ty = $ty {};
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct $ty {}
        impl Irq for $ty {
            #[inline(always)]            
            fn irq_num(&self) -> u8 { $num }

            fn wrap<'a, F: ::core::marker::Sync + ::core::marker::Send + Poll>(&self, f: &F) -> extern "C" fn() {
                static mut HANDLER: Option<usize> = None;                
                unsafe { 
                    // assert!(HANDLER.is_none(), "Irq is already wrapping a function");
                    HANDLER = Some(f as *const F as usize)
                }
                extern "C" fn wrapper<W: Poll>() {
                    unsafe { (*(HANDLER.unwrap() as *const W)).poll() }
                }
                wrapper::<F>
            }
        }
        impl ::core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                write!(f, "[{} @ {}]", stringify!($id), $num)
            }
        }    
        // unsafe impl Sync for $ty {}
        // unsafe impl Send for $ty {}
    )
}


#[macro_export]
macro_rules! irq_type {
    ($id:ident, $ty:ident) => {
        pub const $id: $ty = $ty {};
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        pub struct $ty {}
        impl IrqType for $ty {}        
    }
}

pub trait IrqType {}
pub trait IrqNumber {
    fn number(&self) -> u8;
}
pub trait Irq<IT: IrqType, IN: IrqNumber> {
    fn irq(&self) -> IN;
}
