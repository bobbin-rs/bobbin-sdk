# Bobbin: Teensy LC

This crate contains board support for the [Teensy LC](https://www.pjrc.com/store/teensylc.html) with a [Freescale MKL26Z64VFT4](http://www.nxp.com/products/microcontrollers-and-processors/arm-processors/kinetis-cortex-m-mcus/l-series-ultra-low-power-m0-plus/kinetis-kl2x-48-mhz-usb-ultra-low-power-microcontrollers-mcus-based-on-arm-cortex-m0-plus-core:KL2x) Cortex-M0+ processor.

- [Board Overview](https://www.pjrc.com/store/teensylc.html)
- [Board Schematic](https://www.pjrc.com/teensy/schematic.html)
- [Board Datasheet](https://www.pjrc.com/teensy/KL26P64M48SF5.pdf)
- [Board Reference](https://www.pjrc.com/teensy/KL26P121M48SF4RM.pdf)

## Getting Started

Before getting started, please make sure that you have the following installed in addition to Rust Nightly (more details to follow)

- [OpenOCD](http://openocd.org)
- gcc-arm-embedded toolchain
- [xargo](https://github.com/japaric/xargo)
- [teensy_loader_cli](https://www.pjrc.com/teensy/loader_cli.html)

## Examples

The Teensy LC does not have an on-board debugger or virtual serial port. You will need to update your .bobbin/config directory to include a loader directive.

To run examples using the default bootloader, go to the crate root and use the "flash" utility with the "--example" parameter and optionally the "--console" flag. You will
need to put the board into bootloader mode first by pushing the reset button twice.

To run the "blinky" example:

```
$ flash run --example blinky
    Finished dev [optimized + debuginfo] target(s) in 0.0 secs
Build Complete
Running OpenOCD
OpenOCD Complete
$
```

The Teensy LC does not have an on-board debugger or virtual serial port. 
You will need to connect an external serial port to the appropriate pins. To run these examples, find the device path for the serial port you are using (ex. /dev/cu.usbmodemxxxx on OSX for /dev/ttyUSBx) and use it as a "--console" parameter. 
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