use ::bobbin_common::rw::*;
use hal::nvic::*;

#[test]
fn test_nvic() {
    add_region(0xE000E000, 0x1000);
    assert_eq!(NVIC.iser(0).setena(0), 0);
    NVIC.set_iser(0, |r| r.set_setena(0, 1));
    assert_eq!(NVIC.iser(0).setena(0), 1);
    assert_eq!(read_u32(0xe000_e100), 1);
}

#[test]
fn test_nvic_enabled() {
    add_region(0xE000E000, 0x1000);
    for i in 0..256 {
        assert_eq!(enabled(i), false);
        set_enabled(i, true);
        assert_eq!(enabled(i), true);
        assert_eq!(NVIC.iser(i >> 5).setena(i & 0b11111), 1);
    }
    reset_vm();
    add_region(0xE000E000, 0x1000);
    for i in 0..256 {
        set_enabled(i, false);
        assert_eq!(NVIC.icer(i >> 5).clrena(i & 0b11111), 1);
    }
}


#[test]
fn test_nvic_pending() {
    add_region(0xE000E000, 0x1000);
    for i in 0..256 {
        assert_eq!(pending(i), false);
        set_pending(i, true);
        assert_eq!(pending(i), true);
    }
    reset_vm();
    add_region(0xE000E000, 0x1000);
    for i in 0..256 {
        set_pending(i, false);
        assert_eq!(NVIC.icpr(i >> 5).clrpend(i & 0b11111), 1);
    }
}
