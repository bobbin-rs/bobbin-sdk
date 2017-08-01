# Bobbin: STM32F4Discovery

The [STM32F4Discovery](http://www.st.com/en/evaluation-tools/nucleo-f303ze.html) is an inexpensive development board produced by ST.

## Reference Materials

- [Board Page](http://www.st.com/en/evaluation-tools/stm32f4discovery.html)
- [Board User Manual](http://www.st.com/content/ccc/resource/technical/document/user_manual/70/fe/4a/3f/e7/e1/4f/7d/DM00039084.pdf/files/DM00039084.pdf/jcr:content/translations/en.DM00039084.pdf)
- [MCU Page](http://www.st.com/en/microcontrollers/stm32f407vg.html)
- [MCU Datasheet](http://www.st.com/content/ccc/resource/technical/document/datasheet/ef/92/76/6d/bb/c2/4f/f7/DM00037051.pdf/files/DM00037051.pdf/jcr:content/translations/en.DM00037051.pdf)
- [MCU Reference](http://www.st.com/content/ccc/resource/technical/document/reference_manual/3d/6d/5a/66/b4/99/40/d4/DM00031020.pdf/files/DM00031020.pdf/jcr:content/translations/en.DM00031020.pdf)

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

The STM32F4Discovery debugger does not include a built-in serial port. You will need to connect an external serial port to the appropriate pins. To run these examples, find the device path for the serial port you are using (ex. /dev/cu.usbmodemxxxx on OSX for /dev/ttyUSBx) and use it as a "--console" parameter. 
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
