use ::chip::sysctl::*;

static mut SYSCLK_HZ: u32 = 16_000_000;

pub fn set_sysclk_hz(value: u32) {
    unsafe { SYSCLK_HZ = value }
}

pub fn sysclk_hz() -> u32 {
    unsafe { SYSCLK_HZ }
}

pub fn set_clock(psysdiv: u16, mint: u16, mfrac: u16, n: u8, q: u8) {
    unsafe {
        let mut s = SYSCTL;

        // MOSC Init
        s.with_moscctl(|r| r.set_noxtal(0).set_oscrng(1).set_pwrdn(0));

        // PLL Init
        s.with_rsclkcfg(|r| r.set_pllsrc(0x3).set_psysdiv(psysdiv as u32));
        s.with_pllfreq0(|r| r.set_pllpwr(1).set_mint(mint as u32).set_mfrac(mfrac as u32));
        s.with_pllfreq1(|r| r.set_n(n as u32).set_q(q as u32));
        s.with_rsclkcfg(|r| r.set_newfreq(1));
        while s.pllstat().lock() == 0 {}
        
        // Memory Init
        s.with_memtim0(|r| 
            r.set_fbcht(0x6)
                .set_ebcht(0x6)
                .set_fws(0x5)
                .set_ews(0x5)                    
        );
        s.with_rsclkcfg(|r| r.set_memtimu(1));

        // Use PLL
        s.with_rsclkcfg(|r| r.set_usepll(1));

    }
}