# Bobbin Board Definitions

This repository contains board crates - Rust crates with all the information needed to build an application running on a specific embedded hardware platform.

Each board crate does the following:

  - Imports the appropriate chip and HAL crates for the processor on the board
  - Adds a linker file and build script for the specific board, specifying the amount
    of Flash and SRAM on board as well as the memory map.
  - Includes a .cargo/config file
  - Where appropriate, includes an openocd.cfg file that can be used to connect to
    the device
  - Defines panic and exception handlers, including any _reset handlers that set up
    the runtime environment
  - Adds a basic clock management module
  - Adds a module with pin definitions
  - Adds modules defining available LEDs and buttons
  - Adds a module defining a uart-based console and print macros
  - For Cortex-M3 / M4 devices, sets up console-based logging macros

Each board has a number of examples

  - minimal - runs an infinite loop
  - blinky - blinks the LED at 1hz
  - uart / usart - prints messages directly to the "console" serial port
  - console - prints messages using the console print macros
  - logger - prints messages using the logging macros

To run examples, go to the crate root and use the "flash" utility with the "--example" parameter and optionally the "--console" flag. For instance, to run the "blinky" example:

```
$ flash run --example blinky
   Compiling nucleo-f031k6 v0.1.0 (file:///Users/jcsoo/bobbin/bobbin-boards/nucleo-f031k6)
    Finished dev [optimized + debuginfo] target(s) in 0.0 secs
Build Complete
Running OpenOCD
OpenOCD Complete
$
```

To run the "console" example (using Control-C to exit):

```
$ flash run --example console --console
   Compiling nucleo-f031k6 v0.1.0 (file:///Users/jcsoo/bobbin/bobbin-boards/nucleo-f031k6)
    Finished dev [optimized + debuginfo] target(s) in 0.60 secs
Build Complete
Using console /dev/cu.usbmodem1413
Running OpenOCD
OpenOCD Complete
Opening console
Running Console
Hello, World! 0
Hello, World! 1
Hello, World! 2
^C
```
