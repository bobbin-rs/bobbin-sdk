# Nucleo-L031K6

The [Nucleo-L031K6](http://www.st.com/en/evaluation-tools/nucleo-l031k6.html) is an inexpensive development board with a [STM32L031K6](http://www.st.com/en/microcontrollers/stm32l031k6.html) MCU and onboard ST-Link V2.1 debugger.

## Reference Materials

- [Board Page](http://www.st.com/en/evaluation-tools/nucleo-l031k6.html)
- [Board User Manual](http://www.st.com/resource/en/user_manual/dm00231744.pdf)
- [MCU Page](http://www.st.com/en/microcontrollers/stm32l031k6.html)
- [MCU Datasheet](http://www.st.com/resource/en/datasheet/stm32l031k6.pdf)
- [MCU Reference](http://www.st.com/resource/en/reference_manual/dm00031936.pdf)

## Getting Started

Before getting started, please make sure that you have the following installed in addition to Rust Nightly (more details to follow)

- [OpenOCD](http://openocd.org)
- gcc-arm-embedded toolchain
- [xargo](https://github.com/japaric/xargo)

## Examples

To run examples, go to the crate root and use the "flash" utility with the "--example" parameter and optionally the "--console" flag. For instance, to run the "blinky" example:

```
$ flash run --example blinky
    Finished dev [optimized + debuginfo] target(s) in 0.0 secs
Build Complete
Running OpenOCD
OpenOCD Complete
$
```

To run the "console" example (using Control-C to exit):

```
$ flash run --example console --console
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