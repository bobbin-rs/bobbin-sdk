#![no_std]

extern crate bobbin_common;

pub mod chip;
pub mod hal;

#[cfg(test)]
mod test {
    use ::bobbin_common::*;

    #[test]
    fn test_region() {
        add_region(0x1000, 0x100);
        write_u32(0x1000, 0x1234);
        assert_eq!(read_u32(0x1000), 0x1234);
    }    

    #[test]
    #[should_panic]
    fn test_no_region() {
        read_u32(0x1000);
    }
}