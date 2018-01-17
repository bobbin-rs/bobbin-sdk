use core::ptr;

pub fn uid() -> [u32; 3] {
    pub const UID_BASE: *const u32 = 0x1ff0_f420 as *const u32;
    unsafe {
        [
            ptr::read_volatile(UID_BASE.offset(0)),
            ptr::read_volatile(UID_BASE.offset(1)),
            ptr::read_volatile(UID_BASE.offset(2)),
        ]
    }
}

fn to_u8_be(v: u32) -> [u8; 4] {
    [
        (v >> 24) as u8,
        (v >> 16) as u8,
        (v >> 8) as u8,
        (v >> 0) as u8,
    ]
}

fn to_hex(v: u8) -> [u8; 2] {
    pub const HEX: &[u8] = b"0123456789abcdef";
    [
        HEX[((v >> 4) & 0xf) as usize],
        HEX[((v >> 0) & 0xf) as usize],
    ]    
}

pub fn write_uid(buf: &mut [u8]) -> usize {
    assert!(buf.len() >= 24);
    let uid = uid();
    for i in 0..3 {
        let v = to_u8_be(uid[i]);
        for j in 0..4 {
            let h = to_hex(v[j]);
            for k in 0..2 {
                buf[i * 8 + j * 2 + k] = h[k];
            }
            
        }        
    }
    24
}