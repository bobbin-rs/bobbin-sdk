
use core::ptr;
use core::marker::PhantomData;

pub struct FlagSet<'a> {
    flag: *mut bool,
    _phantom: PhantomData<&'a mut bool>,
}

impl<'a> FlagSet<'a> {
    pub fn is_set(&self) -> bool {
        unsafe { ptr::read_volatile(self.flag) }
    }
    pub fn is_clr(&self) -> bool {
        !self.is_set()
    }
    pub fn set(&self) {
        if !self.is_set() {
            unsafe { ptr::write_volatile(self.flag, true) }
        }
    }
}

pub struct FlagClr<'a> {
    flag: *mut bool,
    _phantom: PhantomData<&'a mut bool>,
}

impl<'a> FlagClr<'a> {
    pub fn is_set(&self) -> bool {
        unsafe { ptr::read_volatile(self.flag) }
    }
    pub fn is_clr(&self) -> bool {
        !self.is_set()
    }
    pub fn clr(&self) {
        if self.is_set() {
            unsafe { ptr::write_volatile(self.flag, false) }
        }
    }
}

pub fn flag_pair<'a>(flag: &'a mut bool) -> (FlagSet<'a>, FlagClr<'a>) {
    use core::mem;
    (        
        FlagSet { flag: flag as *mut bool, _phantom: PhantomData },
        FlagClr { flag: flag as *mut bool, _phantom: PhantomData },
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flag() {
        let mut f = false;
        let (f_set, f_clr) = flag_pair(&mut f);
        assert!(f_set.is_clr());
        assert!(f_clr.is_clr());

        f_set.set();

        assert!(f_set.is_set());
        assert!(f_clr.is_set());

        f_clr.clr();

        assert!(f_set.is_clr());
        assert!(f_clr.is_clr());
    }
}