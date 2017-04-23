# Bobbin: TM4C129XL Connected Launchpad

The [TM4C129XL Connected Launchpad](http://www.ti.com/tool/ek-tm4c1294xl) is an indexpensive development board produced by Texas Instruments.

## Reference Materials

- [Board Page](http://www.ti.com/tool/ek-tm4c1294xl)
- [Board User Manual](http://www.ti.com/lit/ug/spmu365c/spmu365c.pdf)
- [MCU Page](http://www.ti.com/product/tm4c1294ncpdt)
- [MCU Datasheet](http://www.ti.com/lit/ds/symlink/tm4c1294ncpdt.pdf)
- [MCU Errata](http://www.ti.com/lit/pdf/spmz850)

## Running Examples

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
$ flash run --example console --console /dev/cu.usbmodem1413
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
