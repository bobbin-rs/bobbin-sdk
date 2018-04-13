pub use clock::*;
pub use systick_ext::SystickHz;
use gclk::*;
use bobbin_common::bits::*;


macro_rules! impl_gclk {
    ($id:ident, $index:expr) => {
        fn $id(&self) -> Hz { self.gclk($index)}        
    };
}

macro_rules! impl_gclkgen {
    ($id:ident, $index:expr) => {
        fn $id(&self) -> Hz { self.gclkgen($index)}        
    };
}

#[derive(Default)]
pub struct DynamicClock<XOSC: Clock, XOSC32K: Clock>(XOSC, XOSC32K);

impl<XOSC: Clock, XOSC32K: Clock> DynamicClock<XOSC, XOSC32K> {
    pub fn read_clkctrl(&self, id: U6) -> Clkctrl {
        unsafe {
            ::core::ptr::write_volatile(GCLK.clkctrl_mut() as *mut u8, id.value());
        }
        GCLK.clkctrl()
    }

    pub fn read_genctrl(&self, id: U4) -> Genctrl {
        unsafe {
            ::core::ptr::write_volatile(GCLK.genctrl_mut() as *mut u8, id.value());
        }
        GCLK.genctrl()
    }

    pub fn read_gendiv(&self, id: U4) -> Gendiv {
        unsafe {
            ::core::ptr::write_volatile(GCLK.gendiv_mut() as *mut u8, id.value());
        }
        GCLK.gendiv()
    }

    pub fn src(&self, id: U5) -> Hz {
        match id {
            U5::B00000 => self.xosc(),
            // GCLKIN is not supported
            // U5::B00001 => self.gclkin(),
            U5::B00010 => self.gclkgen1(),
            U5::B00011 => self.osculp32k(),
            U5::B00100 => self.osc32k(),
            U5::B00101 => self.xosc32k(),
            U5::B00110 => self.osc8m(),
            U5::B00111 => self.dfll48m(),
            U5::B01000 => self.fdpll96m(),
            _ => Hz::from_num(0),            
        }
    }

    pub fn gclkgen(&self, id: U4) -> Hz {
        let gc = self.read_genctrl(id);
        let gd = self.read_gendiv(id);
        if gc.test_genen() {
            let div = match gc.divsel() {
                U1::B0 => match gd.div().value() {
                    0 => 1,
                    v @ _ => v,
                },
                U1::B1 => 1 << (gd.div().value() + 1),
            };
            self.src(gc.src()) / div as u32
        } else {
            Hz::from_num(0)
        }
    }

    pub fn gclk(&self, id: U6) -> Hz {
        let cc = self.read_clkctrl(id);
        if cc.test_clken() {
            self.gclkgen(cc.gen())
        } else {
            Hz::from_num(0)
        }
    }
}

impl <XOSC: Clock, XOSC32K: Clock> ClockProvider for DynamicClock<XOSC, XOSC32K> {
    type Xosc = XOSC;
    type Xosc32k = XOSC32K;
    impl_gclkgen!(gclkgen0, U4::B0000);
    impl_gclkgen!(gclkgen1, U4::B0001);
    impl_gclkgen!(gclkgen2, U4::B0010);
    impl_gclkgen!(gclkgen3, U4::B0011);
    impl_gclkgen!(gclkgen4, U4::B0100);
    impl_gclkgen!(gclkgen5, U4::B0101);
    impl_gclkgen!(gclkgen6, U4::B0110);
    impl_gclkgen!(gclkgen7, U4::B0111);
    impl_gclkgen!(gclkgen8, U4::B1000);

    impl_gclk!(gclk_dffl48m_ref, U6::B000000); // 0x00
    impl_gclk!(gclk_dpll, U6::B000001); // 0x01
    impl_gclk!(gclk_dpll_32k, U6::B000010); // 0x02
    impl_gclk!(gclk_wdt, U6::B000011); // 0x03
    impl_gclk!(gclk_rtc, U6::B000100); // 0x04
    impl_gclk!(gclk_eic, U6::B000101); // 0x05
    impl_gclk!(gclk_usb, U6::B000110); // 0x06
    impl_gclk!(gclk_evsys_channel_0, U6::B000111); // 0x07
    impl_gclk!(gclk_evsys_channel_1, U6::B001000); // 0x08
    impl_gclk!(gclk_evsys_channel_2, U6::B001001); // 0x09
    impl_gclk!(gclk_evsys_channel_3, U6::B001010); // 0x0a
    impl_gclk!(gclk_evsys_channel_4, U6::B001011); // 0x0b
    impl_gclk!(gclk_evsys_channel_5, U6::B001100); // 0x0c
    impl_gclk!(gclk_evsys_channel_6, U6::B001101); // 0x0d
    impl_gclk!(gclk_evsys_channel_7, U6::B001110); // 0x0e
    impl_gclk!(gclk_evsys_channel_8, U6::B001111); // 0x0f
    impl_gclk!(gclk_evsys_channel_9, U6::B010000); // 0x10
    impl_gclk!(gclk_evsys_channel_10, U6::B010001); // 0x11
    impl_gclk!(gclk_evsys_channel_11, U6::B010010); // 0x12
    impl_gclk!(gclk_sercomx_slow, U6::B010011); // 0x13
    impl_gclk!(gclk_sercom0_core, U6::B010100); // 0x14
    impl_gclk!(gclk_sercom1_core, U6::B010101); // 0x15
    impl_gclk!(gclk_sercom2_core, U6::B010110); // 0x16
    impl_gclk!(gclk_sercom3_core, U6::B010111); // 0x17
    impl_gclk!(gclk_sercom4_core, U6::B011000); // 0x18
    impl_gclk!(gclk_sercom5_core, U6::B011001); // 0x19
    impl_gclk!(gclk_tcc0_tcc1, U6::B011010); // 0x1a
    impl_gclk!(gclk_tcc2_tc3, U6::B011011); // 0x1b
    impl_gclk!(gclk_tc4_tc5, U6::B011100); // 0x1c
    impl_gclk!(gclk_tc6_tc7, U6::B011101); // 0x1d
    impl_gclk!(gclk_adc, U6::B011110); // 0x1e
    impl_gclk!(gclk_adc_dig, U6::B011111); // 0x1f
    impl_gclk!(gclk_20, U6::B100000); // 0x20
    impl_gclk!(gclk_ac_ana, U6::B100001); // 0x21
    impl_gclk!(gclk_22, U6::B100010); // 0x22
    impl_gclk!(gclk_dac, U6::B100011); // 0x23
    impl_gclk!(gclk_ptc, U6::B100100); // 0x24
    impl_gclk!(gclk_i2s_0, U6::B100101); // 0x25
    impl_gclk!(gclk_i2s_1, U6::B100110); // 0x26
}

impl <XOSC: Clock, XOSC32K: Clock> SystickHz for DynamicClock<XOSC, XOSC32K> {
    fn systick_hz(&self, ) -> Hz {
        self.gclkgen0()
    }
}
