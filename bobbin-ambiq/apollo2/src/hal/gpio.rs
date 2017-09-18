pub use bobbin_common::digital::*;
pub use ::chip::gpio::*;

use bobbin_common::bits::*;

impl GpioPeriph {
    pub fn unlock(&self) -> &Self {
        self.set_padkey(|r| r.set_padkey(0x73))
    }
}

impl GpioPin {
    pub fn unlock(&self) -> &Self {
        self.port.unlock();
        self
    }

    pub fn set_pad_pull(&self, value: bool) -> &Self {
        let (i, j) = (self.index >> 2, self.index & 0b11);
        self.port.with_padreg(i, |r| r.set_padpull(j, value));
        self
    }

    pub fn set_pad_pull_enabled(&self) -> &Self {
        self.set_pad_pull(true)
    }

    pub fn set_pad_pull_disabled(&self) -> &Self {
        self.set_pad_pull(false)
    }

    pub fn set_pad_input(&self, value: bool) -> &Self {
        let (i, j) = (self.index >> 2, self.index & 0b11);
        self.port.with_padreg(i, |r| r.set_padinpen(j, value));
        self
    }

    pub fn set_pad_input_enabled(&self) -> &Self {
        self.set_pad_input(true)
    }

    pub fn set_pad_input_disabled(&self) -> &Self {
        self.set_pad_input(false)
    }

    pub fn set_pad_drive_strength(&self, value: bool) -> &Self {
        let (i, j) = (self.index >> 2, self.index & 0b11);
        self.port.with_padreg(i, |r| r.set_padstrng(j, value));
        self
    }

    pub fn set_pad_drive_strength_low(&self) -> &Self {
        self.set_pad_drive_strength(false)
    }

    pub fn set_pad_drive_strength_high(&self) -> &Self {
        self.set_pad_drive_strength(true)
    }

    pub fn set_pad_function<V: Into<U3>>(&self, value: V) -> &Self {
        let (i, j) = (self.index >> 2, self.index & 0b11);
        self.port.with_padreg(i, |r| r.set_padfncsel(j, value));
        self
    }

    pub fn set_pad_gpio(&self) -> &Self {
        self.set_pad_function(0x3)
    }

    pub fn set_gpio_input(&self, value: bool) -> &Self {
        let (i, j) = (self.index >> 3, self.index & 0b111);
        self.port.with_cfg(i, |r| r.set_gpioincfg(j, value));
        self
    }

    pub fn set_gpio_input_enabled(&self) -> &Self {
        self.set_gpio_input(false)
    }

    pub fn set_gpio_input_disabled(&self) -> &Self {
        self.set_gpio_input(true)
    }

    pub fn set_gpio_output<V: Into<U2>>(&self, value: V) -> &Self {
        let (i, j) = (self.index >> 3, self.index & 0b111);
        self.port.with_cfg(i, |r| r.set_gpiooutcfg(j, value));
        self
    }

    pub fn set_gpio_output_disabled(&self) -> &Self {
        self.set_gpio_output(0x0)
    }

    pub fn set_gpio_output_pushpull(&self) -> &Self {
        self.set_gpio_output(0x1)
    }

    pub fn set_gpio_output_open_drain(&self) -> &Self {
        self.set_gpio_output(0x2)
    }

    pub fn set_gpio_output_tristate(&self) -> &Self {
        self.set_gpio_output(0x3)
    }

    pub fn set_gpio_int_dir(&self, value: bool) -> &Self {
        let (i, j) = (self.index >> 3, self.index & 0b111);
        self.port.with_cfg(i, |r| r.set_gpiointd(j, value));
        self
    }

    pub fn set_gpio_int_rising(&self) -> &Self {
        self.set_gpio_int_dir(false)
    }

    pub fn set_gpio_int_falling(&self) -> &Self {
        self.set_gpio_int_dir(true)
    }

    pub fn set_output_enabled(&self, value: bool) -> &Self {
        let (i, j) = (self.index >> 5, self.index & 0b11111);        
        if value {
            self.port.set_ens(i, |r| r.set_ens(j, 1));  
        } else {
            self.port.set_enc(i, |r| r.set_enc(j, 1));
        }        
        self
    }
}

impl DigitalInput for GpioPin {
    #[inline]
    fn input(&self) -> bool {
        let (i, j) = (self.index >> 5, self.index & 0b11111);        
        self.port.rd(i).test_rd(j)
    }
}

impl DigitalOutput for GpioPin {
    #[inline]
    fn output(&self) -> bool {
        let (i, j) = (self.index >> 5, self.index & 0b11111);        
        self.port.wt(i).test_wt(j)
    }

    #[inline]
    fn set_output(&self, value: bool) -> &Self {
        let (i, j) = (self.index >> 5, self.index & 0b11111);        
        if value {
            self.port.set_wts(i, |r| r.set_wts(j, 1));     
        } else {
            self.port.set_wtc(i, |r| r.set_wtc(j, 1));
        }
        self
    }
    #[inline]
    fn toggle_output(&self) -> &Self {
        self.set_output(!self.output())
    }
}