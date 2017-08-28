// use common::bits::*;
use common::timer::*;
use hal::port::*;
use hal::gpio::{self, GpioExt, DigitalOutput};
use hal::ftm::{self, FtmExt, FtmChExt};
use chip::sig;
use chip::pcc::{Cgc, Pcs};
use hal::pcc::{self, PccClockSource};
use core::marker::PhantomData;

// LED BLUE = PTD0
// LED Red = PTD15
// LED Green = PTD16

pub const LED_B: Led0 = LED0;
pub const LED_R: Led1 = LED1;
pub const LED_G: Led2 = LED2;

// pub const LED0: Ptd0 = PTD0;
pub const LED0: Led0 = Led { pin: PTD0, _phantom: PhantomData };
pub const LED1: Led1 = Led { pin: PTD15, _phantom: PhantomData };
pub const LED2: Led2 = Led { pin: PTD16, _phantom: PhantomData };

pub struct Led<P, T, G> {
    pub pin: Pin<P, T>,
    _phantom: PhantomData<G>,
}

pub type Led0 = Led<Ptd0Id, PortdId, gpio::GpiodId>;
pub type Led1 = Led<Ptd15Id, PortdId, gpio::GpiodId>;
pub type Led2 = Led<Ptd16Id, PortdId, gpio::GpiodId>;

impl<P, T, G> Led<P, T, G> 
where Pin<P, T> : GpioPin<P, G>, Periph<T> : Cgc
{
    pub fn init(&self) -> &Self {
        self.pin.port().pcc_enable();
        self.pin.set_mux_gpio();
        self.pin.gpio_pin().set_dir_output().set_output(true);
        self
    }
}

impl<P, T, G> DigitalOutput for Led<P, T, G>
where Pin<P, T> : GpioPin<P, G>
{
    fn output(&self) -> bool {
        !self.pin.gpio_pin().output()
    }    
    fn set_output(&self, value: bool) -> &Self {
        self.pin.gpio_pin().set_output(!value);
        self
    }
    fn toggle_output(&self) -> &Self {
        self.set_output(!self.output())
    }
}

pub const LED_PWM0: LedPwm0 = LedPwm { pin: PTD0, tim_ch: ftm::FTM0_CH2, _phantom: PhantomData };
pub const LED_PWM1: LedPwm1 = LedPwm { pin: PTD15, tim_ch: ftm::FTM0_CH0, _phantom: PhantomData };
pub const LED_PWM2: LedPwm2 = LedPwm { pin: PTD16, tim_ch: ftm::FTM0_CH1, _phantom: PhantomData };

pub type LedPwm0 = LedPwm<Ptd0Id, PortdId, gpio::GpiodId, ftm::Ftm0Id, ftm::Ftm0Ch2Id, sig::Ftm0Ch2>;
pub type LedPwm1 = LedPwm<Ptd15Id, PortdId, gpio::GpiodId, ftm::Ftm0Id, ftm::Ftm0Ch0Id, sig::Ftm0Ch0>;
pub type LedPwm2 = LedPwm<Ptd16Id, PortdId, gpio::GpiodId, ftm::Ftm0Id, ftm::Ftm0Ch1Id, sig::Ftm0Ch1>;

pub struct LedPwm<P, T, G, TIM, TIMCH, SIG> {
    pub pin: Pin<P, T>,
    pub tim_ch: ftm::Channel<TIMCH, TIM>,
    _phantom: PhantomData<(G, SIG)>, 
}

impl<P, T, G, TIM, TIMCH, SIG> LedPwm<P, T, G, TIM, TIMCH, SIG>
where 
    Periph<T>: Cgc,
    ftm::Periph<TIM>: Cgc + Pcs,
    Pin<P, T>: ModeFtm<SIG, ftm::Channel<TIMCH, TIM>>,
{
    pub fn init(&self) -> &Self {
        self.pin.port().pcc_enable();
        self.pin.mode_ftm(&self.tim_ch);
        self.tim_ch.periph()
            .pcc_set_clock_source(pcc::ClockSource::SPLLDIV2)
            .pcc_set_enabled(true)
            .set_prescale(64);
        self
    }
}

pub trait LedPwmSet {
    fn start(&self) -> &Self;
    fn set(&self, value: u16) -> &Self;
}

impl<P, T, G, TIM, TIMCH, SIG> LedPwmSet for LedPwm<P, T, G, TIM, TIMCH, SIG> 
where ftm::Channel<TIMCH, TIM>: PwmLow<u16>
{
    fn start(&self) -> &Self {
        // LED is active low, use pwm_low
        self.tim_ch.pwm_low(0, 1024);
        self
    }

    fn set(&self, value: u16) -> &Self {
        self.tim_ch.set_compare(value);
        self
    }
}

pub const LED_RGB: LedRgb = LedRgb {
    port: PORTD,
    pin_r: PTD15,
    pin_g: PTD16,
    pin_b: PTD0,
    tim: ftm::FTM0,
    tim_r: ftm::FTM0_CH0,
    tim_g: ftm::FTM0_CH1,
    tim_b: ftm::FTM0_CH2,
};

pub struct LedRgb {
    pub port: Portd,
    pub pin_r: Ptd15,
    pub pin_g: Ptd16,
    pub pin_b: Ptd0,
    pub tim: ftm::Ftm0,
    pub tim_r: ftm::Ftm0Ch0,
    pub tim_g: ftm::Ftm0Ch1,
    pub tim_b: ftm::Ftm0Ch2,
}

impl LedRgb {
    pub fn init(&self) -> &Self {
        self.port().pcc_set_enabled(true);
        self.pin_r.mode_ftm(&self.tim_r);
        self.pin_g.mode_ftm(&self.tim_g);
        self.pin_b.mode_ftm(&self.tim_b);

        self.tim
            .pcc_set_clock_source(pcc::ClockSource::SPLLDIV2)
            .pcc_set_enabled(true)
            .set_prescale(64)
            .set_period(1024);

        self.tim_r
            .set_pwmen(true)
            .with_csc(|r| r.set_msb(1).set_msa(0).set_elsb(0).set_elsa(1))
            .set_value(0);

        self.tim_g
            .set_pwmen(true)
            .with_csc(|r| r.set_msb(1).set_msa(0).set_elsb(0).set_elsa(1))
            .set_value(0);

        self.tim_b
            .set_pwmen(true)
            .with_csc(|r| r.set_msb(1).set_msa(0).set_elsb(0).set_elsa(1))
            .set_value(0);

        self
    }

    pub fn start(&self) -> &Self {
        self.tim.set_clock(ftm::ClockSource::SystemClk);
        self
    }

    pub fn set(&self, color: (u16, u16, u16)) -> &Self {
        self.tim_r.set_compare(color.0);
        self.tim_g.set_compare(color.1);
        self.tim_b.set_compare(color.2);
        self
    }
}

pub fn init() {
    LED0.init();
    LED1.init();
    LED2.init();       
}