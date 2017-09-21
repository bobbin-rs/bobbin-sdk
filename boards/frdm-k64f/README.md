# Bobbin: FRDM-K64F

This crate contains board support for the [FRDM-K64F](http://www.nxp.com/products/software-and-tools/hardware-development-tools/freedom-development-boards/freedom-development-platform-for-kinetis-k64-k63-and-k24-mcus:FRDM-K64F) with a [Kinetis K64](http://www.nxp.com/products/microcontrollers-and-processors/arm-processors/kinetis-cortex-m-mcus/k-series-performance-m4/k6x-ethernet/kinetis-k64-120-mhz-256kb-sram-microcontrollers-mcus-based-on-arm-cortex-m4-core:K64_120) Cortex-M4 processor and an [OpenSDA] on-board debugger.

- [Overview](http://www.nxp.com/products/software-and-tools/hardware-development-tools/freedom-development-boards/freedom-development-platform-for-kinetis-k64-k63-and-k24-mcus:FRDM-K64F)
- [Board User Guide](http://www.nxp.com/assets/documents/data/en/user-guides/FRDMK64FUG.pdf)
- [K64f Datasheet](http://www.nxp.com/assets/documents/data/en/data-sheets/K64P144M120SF5.pdf)

## Getting Started

Before getting started, please make sure that you have the following installed in addition to Rust Nightly (more details to follow)

- [OpenOCD](http://openocd.org)
- gcc-arm-embedded toolchain
- [xargo](https://github.com/japaric/xargo)

You may also want to update the onboard debugger to the latest [DAPLINK](https://github.com/mbedmicro/DAPLink) firmware.

## Examples

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