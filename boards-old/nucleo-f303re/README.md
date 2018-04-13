# Nucleo-F303ZE

The [Nucleo-F303ZE](http://www.st.com/en/evaluation-tools/nucleo-f303ze.html) is an inexpensive 
development board produced by ST.

## Reference Materials

- [Board Page](http://www.st.com/en/evaluation-tools/nucleo-f303ze.html)
- [Board User Manual](http://www.st.com/resource/en/user_manual/dm00244518.pdf)
- [MCU Page](http://www.st.com/en/microcontrollers/stm32f303ze.html)
- [MCU Datasheet](http://www.st.com/resource/en/datasheet/stm32f303ze.pdf)
- [MCU Reference](http://www.st.com/resource/en/reference_manual/dm00043574.pdf)

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