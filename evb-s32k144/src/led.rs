// use common::bits::*;
use common::timer::*;
use hal::port::*;
use hal::gpio::{self, GpioExt, DigitalOutput};
use hal::ftm;
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

pub const LED_RGB: LedRgb = LedRgb { led_b: LED_PWM0, led_r: LED_PWM1, led_g: LED_PWM2 };

pub struct LedRgb {
    pub led_b: LedPwm0,
    pub led_r: LedPwm1,
    pub led_g: LedPwm2,
}

impl LedRgb {
    pub fn init(&self) -> &Self {
        self.led_b.init();
        self.led_r.init();
        self.led_g.init();
        self
    }

    pub fn start(&self) -> &Self {
        self.led_b.start();
        self.led_r.start();
        self.led_g.start();
        self
    }

    pub fn set(&self, color: (u16, u16, u16)) -> &Self {
        self.led_r.set(color.0);
        self.led_g.set(color.1);
        self.led_b.set(color.2);
        self
    }
}

pub fn init() {
    LED0.init();
    LED1.init();
    LED2.init();
    // LED0.port().pcc_enable();
    // LED0.gpio_pin().set_dir_output().set_output(true);
    // LED0.set_mux_gpio();

    // LED1.port().pcc_enable();
    // LED1.gpio_pin().set_dir_output().set_output(true);
    // LED1.set_mux_gpio();

    // LED2.port().pcc_enable();
    // LED2.gpio_pin().set_dir_output().set_output(true);
    // LED2.set_mux_gpio();        
}