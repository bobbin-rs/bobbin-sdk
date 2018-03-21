use common::*;
use common::board::Board;
use common::delay::Delay;

pub fn run<B>(b: B) -> !
where
    B: Board + Delay     
{    
    loop {
        println!("Tick");
        b.delay_ms(500);
    }
}