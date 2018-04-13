#![no_std]
#![no_main]

#[macro_use]
extern crate ek_tm4c1294xl as board;

use board::hal::pwm::*;
use board::hal::gpio::ModePwm;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    let led0 = board::led::LED0;

    println!("PWM Test");
    
    let ch = PWM0_CH0;
    let pwm = ch.periph();
    pwm.sysctl_enable();
    led0.mode_pwm(&ch);

    // Enable the PWM clock by setting its corresponding bit in the RCGCPWM register in the System Control module (see page 398).
    // Enable the clock to the appropriate GPIO module via the RCGCGPIO register in the System Control module (see page 382).
    // In the GPIO module, enable the appropriate pins for their alternate function using the GPIOAFSEL register. To determine which GPIOs to configure, see Table 26-4 on page 1797.
    // Configure the PMCn fields in the GPIOPCTL register to assign the PWM signals to the appropriate pins (see page 787 and Table 26-5 on page 1808).
    // Configure the PWM Clock Configuration (PWMCC) register to use the PWM divide (USEPWMDIV) and set the divider (PWMDIV) to divide by 2 (0x0).
    // Configure the PWM generator for countdown mode with immediate updates to the parameters.
    // ■ Write the PWM0CTL register with a value of 0x0000.0000.
    // ■ Write the PWM0GENA register with a value of 0x0000.008C.
    // ■ Write the PWM0GENB register with a value of 0x0000.080C.
    // Set the period. For a 25-KHz frequency, the period = 1/25,000, or 40 microseconds. The PWM clock source is 10 MHz; the system clock divided by 2. Thus there are 400 clock ticks per period. Use this value to set the PWM0LOAD register. In Count-Down mode, set the LOAD field in the PWM0LOAD register to the requested period minus one.
    // ■ Write the PWM0LOAD register with a value of 0x0000.018F. 8. Set the pulse width of the MnPWM0 pin for a 25% duty cycle.
    // ■ Write the PWM0CMPA register with a value of 0x0000.012B. 9. Set the pulse width of the MnPWM1 pin for a 75% duty cycle.
    // ■ Write the PWM0CMPB register with a value of 0x0000.0063. 10. Start the timers in PWM generator 0.
    // ■ Write the PWM0CTL register with a value of 0x0000.0001. 11. Enable PWM outputs.
    // ■ Write the PWMENABLE register with a value of 0x0000.0003.
    
    
    // pwm.set_ch_gena(ch.index(), |_| ChGena(0x008c));
    // pwm.set_ch_gena(ch.index(), |r| r .set_actload(0x2).set_actcmpad(0x3));
    // println!("GENA: {:?}", pwm.ch_gena(ch.index()));
    // pwm.set_ch_load(ch.index(), |_| ChLoad(2000));
    // pwm.set_ch_cmpa(ch.index(), |_| ChCmpa(0));
    // pwm.set_ch_ctl(ch.index(), |_| ChCtl(0x0001));
    // pwm.with_enable(|r| r.set_pwmen(ch.index(), 1));

    pwm.set_output_enabled(ch.index(), true);

    

    println!("Timer Enabled");
       
    let max = 2000u16;
    let step = 20u16;
    let mut i: u16 = step; 
    let mut dir: bool = true;

    // ch.set_compare(0);
    // ch.set_period(max);
    // ch.set_enabled(true);
    ch.pwm_down_high(i, max);
    loop {        
        // pwm.set_ch_cmpa(ch.index(), |_| ChCmpa(i));
        ch.set_compare(i);
        
        if i == max - step { dir = false } else if i == 0 { dir = true;  board::delay(500) }
        if dir {
            i += step 
        } else {
            i -= step;
        }
        board::delay(10);
    }
}
