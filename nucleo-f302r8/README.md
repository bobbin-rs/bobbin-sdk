# Nucleo-F303ZE

The [Nucleo-F303ZE](http://www.st.com/en/evaluation-tools/nucleo-f303ze.html) is an indexpensive 
development board produced by ST.

## Reference Materials

- [Board Page](http://www.st.com/en/evaluation-tools/nucleo-f303ze.html)
- [Board User Manual](http://www.st.com/content/ccc/resource/technical/document/user_manual/group0/26/49/90/2e/33/0d/4a/da/DM00244518/files/DM00244518.pdf/jcr:content/translations/en.DM00244518.pdf)
- [MCU Page](http://www.st.com/en/microcontrollers/stm32f303ze.html)
- [MCU Datasheet](http://www.st.com/resource/en/datasheet/stm32f303ze.pdf)
- [MCU Reference](http://www.st.com/resource/en/reference_manual/dm00043574.pdf)

## Running Examples

Before getting started, please make sure that you have the following installed in addition to Rust Nightly (more details to follow)

- [OpenOCD](http://openocd.org)
- gcc-arm-embedded toolchain
- [xargo](https://github.com/japaric/xargo)

Connect your Nucleo board via USB and in a separate shell session, start openocd:

```
$ openocd
Open On-Chip Debugger 0.10.0+dev-00092-g77189db (2017-03-01-20:42)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
none separate
srst_only separate srst_nogate srst_open_drain connect_deassert_srst
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v25 API v2 SWIM v14 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 3.258614
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints

```

Build the example using xargo:

```
$ xargo build --example blinky
   Compiling nucleo-f303ze v0.1.0 (file:///Users/jcsoo/bobbin/bobbin-boards/nucleo-f303ze)
   Compiling bobbin-cortexm v0.1.0 (https://github.com/bobbin-rs/bobbin-cortexm#5e18c164)
   Compiling r0 v0.1.0
   Compiling gcc v0.3.45
   Compiling stm32f3-chip v0.1.0 (https://github.com/bobbin-rs/bobbin-stm32#706162fc)
   Compiling compiler_builtins v0.1.0 (https://github.com/rust-lang-nursery/compiler-builtins#280d19f1)
   Compiling stm32f3-hal v0.1.0 (https://github.com/bobbin-rs/bobbin-stm32#706162fc)
   Compiling stm32f3-driver v0.1.0 (https://github.com/bobbin-rs/bobbin-stm32#706162fc)
   Compiling stm32f3 v0.1.0 (https://github.com/bobbin-rs/bobbin-stm32#706162fc)
    Finished dev [optimized + debuginfo] target(s) in 17.98 secs
```

Check the size of the produced binary:

```
$ arm-none-eabi-size target/thumbv7em-none-eabihf/debug/examples/blinky
   text	   data	    bss	    dec	    hex	filename
   4792	    392	    392	   5576	   15c8	target/thumbv7em-none-eabihf/debug/examples/blinky
```

Start up GDB, load the binary, and run it.
```
$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/examples/blinky
GNU gdb (GNU Tools for ARM Embedded Processors) 7.10.1.20160923-cvs
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "--host=x86_64-apple-darwin10 --target=arm-none-eabi".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from target/thumbv7em-none-eabihf/debug/examples/blinky...done.
core::iter::range::{{impl}}::next<usize> (self=<optimized out>)
    at /Users/jcsoo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libcore/iter/range.rs:508
508	        if self.start < self.end {
(gdb) load
Loading section .vector, size 0x188 lma 0x8000000
Loading section .text, size 0x12ac lma 0x8000190
Loading section .ARM.extab.text._ZN11stm32f3_hal3rcc16set_gpio_enabled17ha0732a7b69fdce9fE, size 0xc lma 0x800143c
Start address 0x8000190, load size 5184
Transfer rate: 12 KB/sec, 1728 bytes/write.
(gdb) c
Continuing.
```

At this point, you should see a green LED (LD2) blinking roughly twice per second.

Control-C to stop running, and Control-D (or "exit") to exit GDB.

You can build and run other examples in the "examples" directory similarly. Some examples may require additional
hardware to run.