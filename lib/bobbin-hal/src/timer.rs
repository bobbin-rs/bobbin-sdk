//! Traits for interacting with timers.
//!
//! TODO: Evaluate whether all of these traits are necessary
//! or if they should be bundled together.

/// Queries whether a timer is currently running.
pub trait Running {
    // Returns true if the timer is currently running.
    fn running(&self) -> bool;
}

/// Stops a timer.
pub trait Stop {
    // Stops the timer.
    fn stop(&self) -> &Self;
}

/// Query whether a Timeout flag is set.
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

/// Time the number of ticks elapsed during the execution of a closure.
pub trait Elapsed<T> {
    // Returns the number of ticks elapsed while executing `f`.
    // Returns None if the timer overflowed.
    fn elapsed<F: FnOnce()>(&self, f: F) -> Option<T>;
}

/// Get the counter of a timer.
pub trait Counter<T> {
    // Return the current counter value.
    fn counter(&self) -> T;
}

/// Set the counter of a timer.
pub trait SetCounter<T> {
    // Set the current counter value.
    fn set_counter(&self, value: T) -> &Self;
}

/// Clear the counter of a timer.
pub trait ClearCounter {
    // Set the current counter value to zero.
    fn clr_counter(&self) -> &Self;
}

/// Return the prescaler of a timer.
pub trait Prescale<T> {
    // Returns the prescaler for the timer.
    fn prescale(&self) -> T;
}

/// Set the prescaler of a timer.
pub trait SetPrescale<T> {
    // Set the prescaler for the timer.
    fn set_prescale(&self, value: T) -> &Self;
}

/// Return the period of a timer.
pub trait Period<T> {
    // Return the current period of the timer.
    fn period(&self) -> T;
}

/// Set the period of a timer.
pub trait SetPeriod<T> {
    // Set the current period of the timer.
    fn set_period(&self, value: T) ->&Self;
}

/// Block for a set number of timer ticks.
pub trait Delay<T> {
    // Delay for `value` timer ticks.
    fn delay(&self, value: T) -> &Self;
}

/// Start a repeating timer.
pub trait Start<T> {
    // Start a repeating timer with period `value`,
    // clearing timeout flag if available.
    fn start(&self, value: T) -> &Self;
}

/// Start a one-shot timer.
pub trait StartOnce<T> {
    // Start a one-shot timer with period `value`,
    // clearing timeout flag if available.
    fn start_once(&self, value: T) -> &Self;
}

/// Start a repeating down-counting timer.
pub trait StartDown<T> {
    // Start a repeating down-counting timer from `value` to zero, 
    // clearing timeout flag if available.
    fn start_down(&self, value: T) -> &Self;
}

/// Start a repeating up-counting timer.
pub trait StartUp<T> {
    // Start a repeating up-counting timer from zero to `value`,
    // clearing timeout flag if available.
    fn start_up(&self, value: T) -> &Self;
}

/// Start a one-shot down-counting timer.
pub trait StartDownOnce<T> {
    // Start a one-shot down-counting timer from `value` to zero,
    // clearing timeout flag if available.
    fn start_down_once(&self, value: T) -> &Self;
}

/// Start a one-shot up-counting timer.
pub trait StartUpOnce<T> {
    // Start a one-shot up-counting timer from zero to `value`
    // clearing timeout flag if available.
    fn start_up_once(&self, value: T) -> &Self;
}

/// Access and update the timer comparison value and flag.
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

/// Start a PWM timer in Output Low mode.
pub trait PwmLow<T> {
    // PWM, (Counter < Compare) => Output Low
    fn pwm_low(&self, compare: T, period: T) -> &Self;
}

/// Start a PWM timer in Output High mode.
pub trait PwmHigh<T> {
    // PWM, (Counter < Compare) => Output High
    fn pwm_high(&self, compare: T, period: T) -> &Self;
}

/// Start an up-counting timer in Output Low mode.
pub trait PwmUpLow<T> {
    // Up Counting PWM, (Counter < Compare) => Output Low
    fn pwm_up_low(&self, compare: T, period: T) -> &Self;
}

/// Start an up-counting timer in Output High mode.
pub trait PwmUpHigh<T> {
    // Up Counting PWM, (Counter < Compare) => Output High
    fn pwm_up_high(&self, compare: T, period: T) -> &Self;
}

/// Start an down-counting timer in Output Low mode.
pub trait PwmDownLow<T> {
    // Down Counting PWM, (Counter < Compare) => Output Low
    fn pwm_down_low(&self, compare: T, period: T) -> &Self;
}

/// Start an down-counting timer in Output High mode.
pub trait PwmDownHigh<T> {
    // Down Counting PWM, (Counter < Compare) => Output High
    fn pwm_down_high(&self, compare: T, period: T) -> &Self;
}

/// Start an center-aligned timer in Output Low mode.
pub trait PwmCenterLow<T> {
    // Center Aligned PWM, (Counter < Compare) => Output Low
    fn pwm_center_low(&self, compare: T, period: T) -> &Self;
}

/// Start an center-aligned timer in Output High mode.
pub trait PwmCenterHigh<T> {
    // Center Aligned PWM, (Counter < Compare) => Output High
    fn pwm_center_high(&self, compare: T, period: T) -> &Self;
}

/// Tests useful for verifying correct operation of timers.
pub mod tests {
    use super::*;

    /// Test operation of a repeating timer.
    pub fn test_timer<V, T>(t: &T, v: V) 
    where V: Into<u32>, T: Start<V> + Stop + Running + Timeout
    {
        let mut ticks = 0;
        t.start(v);
        
        while !t.test_timeout() {
            ticks += 1;
        }
        assert!(ticks > 0);
        t.clr_timeout();
        ticks = 0;
        while !t.test_timeout() {
            ticks += 1;
        }
        assert!(ticks > 0);
        t.stop();    
        assert!(!t.running());    
    }

    /// Test operation of a one-shot timer.
    pub fn test_timer_once<V, T>(t: &T, v: V) 
    where V: Into<u32>, T: StartOnce<V> + Running + Timeout
    {
        let mut ticks = 0;
        t.start_once(v);
        
        while !t.test_timeout() {
            ticks += 1;
        }
        assert!(ticks > 0);
        t.clr_timeout();
        assert!(!t.running());    
    }

    /// Test operation of a repeating up-counting timer.
    pub fn test_timer_up<V, T>(t: &T, v: V) 
    where V: Into<u32>, T: StartUp<V> + Stop + Running +Timeout
    {
        let mut ticks = 0;
        t.start_up(v);    
        while !t.test_timeout() {
            ticks += 1;
        }
        assert!(ticks > 0);
        t.clr_timeout();
        ticks = 0;
        while !t.test_timeout() {
            ticks += 1;
        }
        assert!(ticks > 0);
        t.stop();
        assert!(!t.running());    
    }

    /// Test operation of a single-shot up-counting timer.
    pub fn test_timer_up_once<V, T>(t: &T, v: V) 
    where V: Into<u32>, T: StartUpOnce<V> + Running + Timeout
    {
        let mut ticks = 0;
        t.start_up_once(v);    
        while !t.test_timeout() {
            ticks += 1;
        }
        assert!(ticks > 0);
        t.clr_timeout();
        assert!(!t.running());
    }

    /// Test operation of a repeating down-counting timer.
    pub fn test_timer_down<V, T>(t: &T, v: V) 
    where V: Into<u32>, T: StartDown<V> + Stop + Running + Timeout
    {
        let mut ticks = 0;
        t.start_down(v);        
        while !t.test_timeout() {
            ticks += 1;
        }
        assert!(ticks > 0);
        t.clr_timeout();
        ticks = 0;
        while !t.test_timeout() {
            ticks += 1;
        }
        assert!(ticks > 0);
        t.stop();
        assert!(!t.running());
    }

    /// Test operation of a single-shot down-counting timer.
    pub fn test_timer_down_once<V, T>(t: &T, v: V) 
    where V: Into<u32>, T: StartDownOnce<V> + Running + Timeout
    {
        let mut ticks = 0;
        t.start_down_once(v);        
        while !t.test_timeout() {
            ticks += 1;
        }
        assert!(ticks > 0);
        t.clr_timeout();
        assert!(!t.running());
    }
}