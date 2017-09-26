pub trait Start<T> {
    // Start a repeating timer with period `value`.
    fn start(&self, value: T) -> &Self;
}

pub trait StartOnce<T> {
    // Start a one-shot timer with period `value`.
    fn start_once(&self, value: T) -> &Self;
}

pub trait Running {
    // Returns true if the timer is currently running.
    fn running(&self) -> bool;
}

pub trait Stop {
    // Stops the timer.
    fn stop(&self) -> &Self;
}

pub trait Timeout {
    // Returns true if the timeout flag is set.
    fn test_timeout(&self) -> bool;

    // Clear the timeout flag.
    fn clr_timeout(&self) -> &Self;

    // Busy-wait until the timeout flag is set.
    fn wait_timeout(&self) -> &Self {
        while !self.test_timeout() {}
        self
    }        
}

pub trait Counter<T> {
    // Return the current counter value.
    fn counter(&self) -> T;
}

pub trait SetCounter<T> {
    // Set the current counter value.
    fn set_counter(&self, value: T) -> &Self;
}

pub trait Prescale<T> {
    // Returns the prescaler for the timer.
    fn prescale(&self) -> T;
}

pub trait SetPrescale<T> {
    // Set the prescaler for the timer.
    fn set_prescale(&self, value: T) -> &Self;
}

pub trait Period<T> {
    // Return the current period of the timer.
    fn period(&self) -> T;
}

pub trait SetPeriod<T> {
    // Set the current period of the timer.
    fn set_period(&self, value: T) ->&Self;
}

pub trait Delay<T> {
    // Delay for `value` timer ticks.
    fn delay(&self, value: T) -> &Self;
}

pub trait StartDown<T> {
    // Start a repeating down-counting timer from `value` to zero.
    fn start_down(&self, value: T) -> &Self;
}

pub trait StartUp<T> {
    // Start a repeating up-counting timer from zero to `value`.
    fn start_up(&self, value: T) -> &Self;
}

pub trait StartDownOnce<T> {
    // Start a one-shot down-counting timer from `value` to zero.
    fn start_down_once(&self, value: T) -> &Self;
}

pub trait StartUpOnce<T> {
    // Start a one-shot up-counting timer from zero to `value`.
    fn start_up_once(&self, value: T) -> &Self;
}

pub trait Compare<T> {
    // Returns the current comparison value.
    fn compare(&self) -> T;
    // Sets the comparison value that will set the compare flag.
    fn set_compare(&self, value: T) -> &Self;

    // Returns true if the compare flag is set.
    fn test_compare(&self) -> bool;

    // Clears the compare flag.
    fn clr_compare(&self) -> &Self;

    // Busy-wait until the compare flag is set.
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