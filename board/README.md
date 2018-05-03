# Board Support Crates

These crates support a range of popular development boards. Each provides the following:

## Cargo

- The crate for the specific [MCU](../mcu/) variant used on the board. The MCU crate will generate the correct
linker include files for the variant.
- [japaric/cortex-m-rt](https://github.com/japaric/cortex-m-rt) for startup and exception handler installation.
- [japaric/panic-abort](https://github.com/japaric/panic-abort) to set the panicking behavior to abort.
- A development and release profile.
- A .cargo/config file with the correct `rustc` options and target identifier.

## Additional Configuation Files

- An [OpenOCD](http://openocd.org) configuration file if needed for the board.
- A .gdbinit file.
- Additional linker files, if required.
- A simple Makefile that can be used to build and test the board.

## System Implementation

A [System](../lib/bobbin-sys/) implementation that does the following:
   - Early initialization (disabling watchdogs, etc.)
   - MCU setup (enabling instruction cache if available)
   - Clock setup (changing to a "default" high-speed run mode). The Clock manager
   also provides the ability to perform dynamic input clock calculations for
   peripherals, and in some cases the ability to dynamically update clock configurations.
   - Heap setup (reserving 4KB for an allocate-only heap)
   - IRQ Dispatcher setup (8 slots for dynamic registration of IRQ handlers)
   - Tick handler setup (simple 1KHz counter for timeouts and delays)
   - Console setup (configures a 115,200 baud serial port and sets up println! support)
   - LED setup (configures and enables on-board LEDs if available)
   - Button Setup (configures and enables on-board Buttons if available)

Additionally, the System may have traits implemented to provide access to peripherals and device drivers such as:

   - Flash Memory
   - Watchdog Timers
   - Hardware Timers
   - Serial, I2C, and SPI controllers
   - CAN, USB, and Ethernet interfaces

For boards that have additional on-board but off-chip peripherals, drivers may also be included for those peripherals.

## Examples

Each board crate also includes a wide variety of examples. These can be run simply by using `bobbin run --example *example-name*`.

These examples typically fall into a few categories.

### Basic Functionality

Each board should have a few examples that can be used to verify basic functionality - typically a `blinky` example, 
which flashes a LED, a `button` example that flashes a LED at different speeds based on whether a hardware button is
pressed, and a `console` example that prints to the serial console.

### Generic Examples

These examples show how to write board-agnostic applications using [bobbin-sys](../lib/bobbin-sys/) and [bobbin-hal](../lib/bobbin-hal/).

### Embedded-HAL Examples

[japaric/embedded-hal](https://github.com/japaric/embedded-hal) is a collection of traits describing a number
of peripheral types and protocols used in embedded programming. These examples show simple applications that use those APIs.

### Board-Specific Examples

These examples show how to access and use hardware and peripherals that may be specific to the board.

## Usage

In most cases, all that is needed is to create a new Cargo crate and add the board crate as a dependency. Your
`main.rs` file should include a `#![no_std]` attribute and import the crate (typically as `board`) and crate prelude. 

```
#![no_std]

#[macro_use]
extern crate frdm_k64f as board;

use board::prelude::*;

fn main() {
    /// main goes here
}

```

### Initializing The Board

Within main, the first step should always be to execute board::init() to obtain the `System` instance. All of the
board initialization steps take place here, including disabling global interrupts for the MCU.

> NOTE: Some boards (NXP Kinetis, for instance) require the watchdog to be configured early on in the boot
> process or will otherwise enter a reset loop that may be difficult to recover from. Running code before
> `board::init()` is not recommended.

```
fn main() {
    let mut sys = board::init(); // `let mut` used here becuase we will need to call method using &mut self later. 

    // Initialize system here. All interrupts are disabled.

    ...
}
```

### Initializing Your Application

At this point, interrupts are disabled and you have full ownership of the `System` instance. This is where you should
perform your main application configuration, including configuring pins, enabling and configuring peripherals, 
allocating memory, and setting up drivers. The system console is set up and running in blocking mode so that you can use 
`printf!` for reporting or debugging purposes.

You should have access to system services through the following methods on the System instance:

- console()
- mcu()
- mcu_mut()
- clk()
- clk_mut()
- heap()
- heap_mut()
- tick()
- tick_mut()
- dispatcher()
- dispatcher_mut()

### Running Your Application

After initialization is complete, you can run your application by passing a closure to the `run()` method.

```
fn main() {
    let mut sys = board::init();

    // Run the system

    sys.run(|sys| {
        println!("Starting Up...");
        loop {
            println!("Hello, World...);
            sys.tick().delay(500); // Delay 500ms
        }
    })

}
```

### &System and &mut System

The signature of `run()` is:

```
pub fn run<T, F: FnOnce(&Self)->T>(&mut self, f: F) -> T {}

```

This means that you need a &mut reference to call `run()`, but the closure receives a `&System` reference.

This is done to control the operations that can be done during the initialization phase (when interrupts are disabled)
and during the run phase (when interrupts are enabled). For instance, all `heap` operations that allocate require
a `&mut` reference to the Heap object, which can only be obtained through the `heap_mut()` method, which requires
a `&mut` reference to the System object.

This simplifies the implementation of the `Heap` object which can now assume that it can only be accessed
in a single-threaded, non-interrupt context. It also ensures that all top-level memory allocations are explicit and occur
only during the initialization phase, since the global Heap singleton can not be accessed while the `System` object exists.

### Run-time Allocation

Applications that want to make use of a run-time allocator or arena should allocate a chunk of memory from the `Heap` during
the initialization phase. Applications wanting to make use of the [alloc](https://doc.rust-lang.org/alloc/) crate
and fully dynamic allocation should select an allocator such as [japaric/alloc-cortex-m](https://github.com/japaric/alloc-cortex-m)
that can then be set as the global allocator using the [#[global_allocator]](https://github.com/rust-lang/rust/blob/master/src/doc/unstable-book/src/language-features/global-allocator.md) attribute.