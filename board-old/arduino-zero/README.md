# Bobbin: Arduino Zero

This crate contains board support for the [Arduino Zero](https://www.arduino.cc/en/Main/ArduinoBoardZero) with an [Atmel SAMD21](http://www.atmel.com/products/microcontrollers/arm/sam-d.aspx) Cortex-M0+ processor and an [Atmel EDBG](http://www.atmel.com/Images/Atmel-42096-Microcontrollers-Embedded-Debugger_User-Guide.pdf) on-board debugger.

- [Overview](https://www.arduino.cc/en/Main/ArduinoBoardZero)
- [Board Schematic](https://www.arduino.cc/en/uploads/Main/ArduinoZero-schematic.pdf)
- [SAMD21 Datasheet Summary](http://www.atmel.com/Images/Atmel-42181-SAM-D21_Summary.pdf)
- [SAMD21 Datasheet Complete](https://cdn.sparkfun.com/datasheets/Dev/Arduino/Boards/Atmel-42181-SAM-D21_Datasheet.pdf)

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