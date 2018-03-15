use core::ptr;

#[cfg(any(feature="STM32F72xxC", feature="STM32F72xxE"))]
pub const UID_BASE: *const u32 = 0x1ff0_7a10 as *const u32;

#[cfg(any(feature="STM32F74xxE", feature="STM32F74xxG", feature="STM32F76xxG", feature="STM32F76xxI"))]
pub const UID_BASE: *const u32 = 0x1ff0_f420 as *const u32;

#[cfg(any(feature="STM32F72xxC", feature="STM32F72xxE", feature="STM32F74xxE", feature="STM32F74xxG", feature="STM32F76xxG", feature="STM32F76xxI"))]
pub fn uid() -> [u32; 3] {
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

#[cfg(any(feature="STM32F72xxC", feature="STM32F72xxE", feature="STM32F74xxE", feature="STM32F74xxG", feature="STM32F76xxG", feature="STM32F76xxI"))]
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