use common::board::Board;
use common::delay::Delay;
use common::led::*;

pub fn run<B>(b: B) -> !
where
    B: Board + Delay + GetLed
{    
    let n = b.get_led_count();
    loop {
        for i in 0..n {
            b.get_led(i).on();
            b.delay_ms(500);
        }
        for i in 0..n {
            b.get_led(i).off();
            b.delay_ms(500);
        }
    }        

}