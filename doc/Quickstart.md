# Quick Start

## Using MCU Crates

You can use a MCU crate directly. You will need to use cortex-m-rt as your runtime, as well as a panic crate.

Steps:

- Create a new application crate

```
$ cargo new bobbin-stm32f746zg-example
```


- Add a new binary section
- Add MCU crate dependency and variant
- Add cortex-m-rt dependency
- Add panic-abort dependency

```
[package]
name = "bobbin-stm32f746zg-example"
version = "0.1.0"
authors = ["Jonathan Soo <jcsoo@agora.com>"]

[[bin]]
name = "blinky"
path = "src/blinky.rs"
doc = false

[dependencies]
cortex-m-rt = "0.4.0"
panic-abort = "0.1.1"

[dependencies.stm32f74x]
path = "[path to dependency]"
features = ["STM32F74xxG"]

[profile.dev]
panic = "abort"
opt-level = "s"
incremental = false
lto = true

[profile.release]
panic = "abort"
opt-level = "s"
debug = true
incremental = false
lto = true

```

Set up simple blinky.rs that blinks LEDs

```
#![no_std]
#![feature(asm)]

extern crate cortex_m_rt;
extern crate panic_abort;
extern crate stm32f74x as mcu;

// LED0 = PB0;

use mcu::bobbin_mcu::prelude::*;
use mcu::bobbin_hal::prelude::*;
use mcu::pin::*;

pub const LED: Pb0 = PB0;

fn main() {
    LED
        .port_gate_enable()
        .mode_output();

    loop {
        LED.toggle_output();
        for _ in 0..5_000_000 { unsafe { asm!("nop") }}
    }    
}
```

Compile and run the binary using `bobbin run --bin blinky`

```
$ bobbin run --bin blinky
   Compiling bobbin-stm32f746zg-example v0.1.0 (file:///Users/jcsoo/bobbin-dev/bobbin-dev/mcu-examples/bobbin-stm32f746zg-example)
    Finished dev [optimized + debuginfo] target(s) in 1.84 secs
   text	   data	    bss	    dec	    hex	filename
    824	    456	      0	   1280	    500	target/thumbv7em-none-eabihf/debug/blinky
     Loading target/thumbv7em-none-eabihf/debug/blinky
    Complete Successfully flashed device
      Loader Load Complete
     Console Opening Console
```

You should see a blinking LED. Use `Control-C` to terminate the console.

Next up - get serial output running.

Add anothery entry to Cargo.toml:
```
[[bin]]
name = "console"
path = "src/console.rs"
doc = false
```

And create a new file, `console.rs`:

```
#![no_std]
#![feature(asm)]

extern crate cortex_m_rt;
extern crate panic_abort;
extern crate stm32f74x as mcu;

use mcu::bobbin_mcu::prelude::*;
use mcu::bobbin_hal::prelude::*;
use mcu::ext::rcc::DedicatedClock;
use mcu::usart::*;
use mcu::pin::*;

pub const USART: Usart3 = USART3;
const USART_TX: Pd8 = PD8;
const USART_RX: Pd9 = PD9;
const USART_CLOCK: u32 = 16_000_000; // Use HSI Clock
const USART_BAUD: u32 = 115_200;

fn main() {
    mcu::ext::init();
    USART_TX
        .port_gate_enable()
        .connect_to(USART);

    USART_RX
        .port_gate_enable()
        .connect_to(USART);

    USART
        .set_clock_source(DedicatedClock::Hsi)
        .gate_enable()
        .set_config(|c| c.set_baud_clock(USART_BAUD, USART_CLOCK))
        .enable();

    loop {
        USART.write(b"Hello, World\r\n");
        for _ in 0..10_000_000 { unsafe { asm!("nop") }}
    }    
}

```

Run this example with `bobbin run --bin console` and you should see something like the following:

```
$ bobbin run --bin console
   Compiling bobbin-stm32f746zg-example v0.1.0 (file:///Users/jcsoo/bobbin-dev/bobbin-dev/mcu-examples/bobbin-stm32f746zg-example)
    Finished dev [optimized + debuginfo] target(s) in 1.52 secs
   text	   data	    bss	    dec	    hex	filename
    996	    456	      0	   1452	    5ac	target/thumbv7em-none-eabihf/debug/console
     Loading target/thumbv7em-none-eabihf/debug/console
    Complete Successfully flashed device
      Loader Load Complete
     Console Opening Console
Hello, World
Hello, World
Hello, World
...
```



## Using Board Crates

Board crates include a runtime and tools to make development easier including a serial console, dynamic clock,
dynamic interrupt dispatcher, and basic heap.

Create a new application crate

```
$ bobbin new bobbin-nucleo-f746zg-example
```

- Copy .cargo/config
- Copy openocd.cfg
- Update Cargo.toml to add the binaries and dependencies
- Update the profiles

```
[package]
name = "bobbin-nucleo-f746zg-example"
version = "0.1.0"
authors = ["Jonathan Soo <jcsoo@agora.com>"]

[[bin]]
name = "blinky"
path = "src/blinky.rs"
doc = false

[[bin]]
name = "console"
path = "src/console.rs"
doc = false

[dependencies]
nucleo-f746zg = { path = "../../board/nucleo-f746zg" }

[profile.dev]
panic = "abort"
opt-level = "s"
incremental = false
lto = true

[profile.release]
panic = "abort"
opt-level = "s"
debug = true
incremental = false
lto = true
```

First, let's write a new blinky.rs example. It looks similar to the MCU crate example except that we are using the board's LED method.

```
#![no_std]

extern crate nucleo_f746zg as board;

use board::prelude::*;

fn main() {
    // Initialize the board and get the System.
    let mut sys = board::init();

    // Application initialization goes here. Interrupts are disabled.

    // Enable interrupts and enter the main loop.
    sys.run(|sys| loop {
        sys.led0().toggle(); // Toggle LED0        
        sys.tick().delay(500); // Delay 500ms
    })
}
```

Build and run it (use Control-C to exit):

```
$ bobbin run --bin blinky
   Compiling bobbin-nucleo-f746zg-example v0.1.0 (file:///Users/jcsoo/bobbin-dev/bobbin-dev/board-examples/bobbin-nucleo-f746zg-example)
    Finished dev [optimized + debuginfo] target(s) in 1.49 secs
   text	   data	    bss	    dec	    hex	filename
   3734	    468	     24	   4226	   1082	target/thumbv7em-none-eabihf/debug/blinky
     Loading target/thumbv7em-none-eabihf/debug/blinky
    Complete Successfully flashed device
      Loader Load Complete
     Console Opening Console
^C
```

`console.rs` example is even simpler.

```
#![no_std]

// Use the println! macro
#[macro_use]
extern crate nucleo_f746zg as board;

use board::prelude::*;

fn main() {
    // Initialize the board and get the System.
    let mut sys = board::init();

    // Application initialization goes here. Interrupts are disabled.

    // Enable interrupts and enter the main loop.
    sys.run(|sys| loop {
        println!("Hello, World"); // Print "Hello, World"
        sys.tick().delay(500); // Delay 500ms
    })
}
```

Build and run it (use Control-C to exit):

```
$ bobbin run --bin console
    Finished dev [optimized + debuginfo] target(s) in 0.04 secs
   text	   data	    bss	    dec	    hex	filename
   3756	    468	     24	   4248	   1098	target/thumbv7em-none-eabihf/debug/console
     Loading target/thumbv7em-none-eabihf/debug/console
    Complete Successfully flashed device
      Loader Load Complete
     Console Opening Console
Hello, World
Hello, World
Hello, World
Hello, World
...
^C
```
