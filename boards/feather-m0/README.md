# Bobbin: Feather M0

This crate contains board support for the [Feather M0](https://www.adafruit.com/product/2772) with an [Atmel SAMD21](http://www.atmel.com/products/microcontrollers/arm/sam-d.aspx) Cortex-M0+ processor.

- [Overview](https://www.adafruit.com/product/2772)
- [Board Schematic](https://learn.adafruit.com/assets/28801)
- [SAMD21 Datasheet Summary](http://www.atmel.com/Images/Atmel-42181-SAM-D21_Summary.pdf)
- [SAMD21 Datasheet Complete](https://cdn.sparkfun.com/datasheets/Dev/Arduino/Boards/Atmel-42181-SAM-D21_Datasheet.pdf)

## Getting Started

Before getting started, please make sure that you have the following installed in addition to Rust Nightly (more details to follow)

- [OpenOCD](http://openocd.org)
- gcc-arm-embedded toolchain
- [xargo](https://github.com/japaric/xargo)
- [bossa](http://www.shumatech.com/web/products/bossa)

## Examples

To run examples using the default bootloader, go to the crate root and use the "flash" utility with the "--example" parameter and optionally the "--console" flag. You will
need to put the board into bootloader mode first by pushing the reset button twice.

To run the "blinky" example:

```
$ flash run --example blinky
   Compiling nucleo-f031k6 v0.1.0 (file:///Users/jcsoo/bobbin/bobbin-boards/nucleo-f031k6)
    Finished dev [optimized + debuginfo] target(s) in 0.0 secs
Build Complete
Running OpenOCD
OpenOCD Complete
$
```

The Feather M0 does not have an on-board debugger or virtual serial port. You will need to connect an external serial port to the appropriate pins. To run these examples, find the device path for the serial port you are using (ex. /dev/cu.usbmodemxxxx on OSX for /dev/ttyUSBx) and use it as a "--console" parameter. 
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