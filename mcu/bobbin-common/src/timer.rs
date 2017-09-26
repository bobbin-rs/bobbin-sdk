// pub enum Direction {
//     Down,
//     Up,
// }


pub trait Start<T> {
    fn start(&self, value: T) -> &Self;
}

pub trait StartOnce<T> {
    fn start_once(&self, value: T) -> &Self;
}

pub trait Running {
    fn running(&self) -> bool;
}

pub trait Stop {
    fn stop(&self) -> &Self;
}

pub trait Timeout {
    fn test_timeout(&self) -> bool;
    fn clr_timeout(&self) -> &Self;

    fn wait_timeout(&self) -> &Self {
        while !self.test_timeout() {}
        self
    }        
}

pub trait Counter<T> {
    fn counter(&self) -> T;
}

pub trait SetCounter<T> {
    fn set_counter(&self, value: T) -> &Self;
}

pub trait Prescale<T> {
    fn prescale(&self) -> T;
}

pub trait SetPrescale<T> {
    fn set_prescale(&self, value: T) -> &Self;
}

pub trait Period<T> {
    fn period(&self) -> T;
}

pub trait SetPeriod<T> {
    fn set_period(&self, value: T) ->&Self;
}

pub trait Delay<T> {
    fn delay(&self, value: T) -> &Self;
}

// pub trait Timer<T> {
//     fn stop(&self) -> &Self;
//     fn running(&self) -> bool;

//     fn period(&self) -> T;
//     fn set_period(&self, value: T) -> &Self;

//     fn counter(&self) -> T;

//     fn test_timeout(&self) -> bool;
//     fn clr_timeout(&self) -> &Self;

//     fn wait_timeout(&self) -> &Self {
//         while !self.test_timeout() {}
//         self
//     }    
// }


pub trait StartDown<T> {
    fn start_down(&self, value: T) -> &Self;
}

pub trait StartUp<T> {
    fn start_up(&self, value: T) -> &Self;
}

pub trait StartDownOnce<T> {
    fn start_down_once(&self, value: T) -> &Self;
}

pub trait StartUpOnce<T> {
    fn start_up_once(&self, value: T) -> &Self;
}

pub trait Compare<T> {
    fn compare(&self) -> T;
    fn set_compare(&self, value: T) -> &Self;

    fn test_compare(&self) -> bool;
    fn clr_compare(&self) -> &Self;

    fn wait_test_compare(&self) -> &Self {
        while !self.test_compare() {}
        self
    }
}

pub trait PwmLow<T> {
    // PWM, (Counter < Compare) => Output Low
    fn pwm_low(&self, compare: T, period: T) -> &Self;
}

pub trait PwmHigh<T> {
    // PWM, (Counter < Compare) => Output High
    fn pwm_high(&self, compare: T, period: T) -> &Self;
}


pub trait PwmUpLow<T> {
    // Up Counting PWM, (Counter < Compare) => Output Low
    fn pwm_up_low(&self, compare: T, period: T) -> &Self;
}

pub trait PwmUpHigh<T> {
    // Up Counting PWM, (Counter < Compare) => Output High
    fn pwm_up_high(&self, compare: T, period: T) -> &Self;
}

pub trait PwmDownLow<T> {
    // Down Counting PWM, (Counter < Compare) => Output Low
    fn pwm_down_low(&self, compare: T, period: T) -> &Self;
}

pub trait PwmDownHigh<T> {
    // Down Counting PWM, (Counter < Compare) => Output High
    fn pwm_down_high(&self, compare: T, period: T) -> &Self;
}

pub trait PwmCenterLow<T> {
    // Center Aligned PWM, (Counter < Compare) => Output Low
    fn pwm_center_low(&self, compare: T, period: T) -> &Self;
}

pub trait PwmCenterHigh<T> {
    // Center Aligned PWM, (Counter < Compare) => Output High
    fn pwm_center_high(&self, compare: T, period: T) -> &Self;
}