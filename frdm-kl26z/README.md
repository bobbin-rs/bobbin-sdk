# Bobbin: FRDM-KL26Z

This crate contains board support for the [FRDM-KL26Z](http://www.nxp.com/products/software-and-tools/hardware-development-tools/freedom-development-boards/freedom-development-platform-for-kinetis-kl16-and-kl26-mcus-up-to-128-kb-flash:FRDM-KL26Z) with a [Kinetis KL26](http://www.nxp.com/products/microcontrollers-and-processors/arm-processors/kinetis-cortex-m-mcus/l-series-ultra-low-power-m0-plus/kinetis-kl2x-48-mhz-usb-ultra-low-power-microcontrollers-mcus-based-on-arm-cortex-m0-plus-core:KL2x) Cortex-M0+ processor and an [OpenSDA] on-board debugger.

NOTE: OpenOCD works for debugging, but doesn't appear to be able to flash binaries larger than about 8k. Use the DAPLINK MSD interface for loading.

- [Overview](http://www.nxp.com/products/software-and-tools/hardware-development-tools/freedom-development-boards/freedom-development-platform-for-kinetis-k64-k63-and-k24-mcus:FRDM-K64F)
- [Board User Guide](http://www.nxp.com/assets/documents/data/en/user-guides/FRDMKL26ZUG.zip)
= [MCU Home Page](http://www.nxp.com/products/microcontrollers-and-processors/arm-processors/kinetis-cortex-m-mcus/l-series-ultra-low-power-m0-plus/kinetis-kl2x-48-mhz-usb-ultra-low-power-microcontrollers-mcus-based-on-arm-cortex-m0-plus-core:KL2x)
- [MCU Datasheet](http://www.nxp.com/assets/documents/data/en/data-sheets/KL26P36M48SF5.pdf)

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