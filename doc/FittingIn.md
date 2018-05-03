# Fitting In - Bobbin and the Rust Ecosystem

Rust has a growing embedded development community, anchored by the
[Embedded Devices Working Group](https://github.com/rust-lang-nursery/embedded-wg).

One of the most important Rust Embedded projects is [embedded-hal](https://github.com/japaric/embedded-hal),
which is a collection of traits that abstract many common hardware peripherals and protocols commonly used in embedded development.

Rust also has a growing number of MCU support crates that are generated from [CMSIS-SVD](https://www.keil.com/pack/doc/CMSIS/SVD/html/index.html) using [japaric/svd2rust](https://github.com/japaric/svd2rust), and many corresponding embedded-hal implementations for
those crates.

There are also a wide range of peripheral crates - drivers for all kinds of devices, using
embedded-hal traits.

Isn't all that enough?

To answer that, you have to look at the embedded development landscape that exists today, and what
embedded developers are used to working with.

First, the vendor ecosystems, each an isolated island with luxurious trappings. Typically, each vendor will
provide:

- A compiler and toolchain, now typically GNU based
- A SDK, including
   - Low level register access headers for all devices
   - Hardware Abstraction Layer that provides a unified higher-level API across all vendor peripherals
   - Middleware and network stacks for more complex protocols such as USB, Bluetooth, and Ethernet / IP
   - At least one royalty-free RTOS port with drivers for common peripherals
   - Board Support Packages for their development boards
   - Examples and sample applications, both board-specific and generic.
   - Project templates of varying complexity
   - Documentation for all APIs included in the SDK
- An IDE, typically based on Eclipse, with vendor-specific enhancements
   - MCU-specific debuggers and introspection tools
   - Debug Probe Support
   - Project wizards to create projects from templates
- Code generation tools that deal with
   - Pin assignment and conflict resolution
   - Clock tree configuration and validation
   - Peripheral initialization
   - Power state management

Not all users use all of these features, and in many cases code quality is questionable or not suited for specific types of applications, but they are an incredibly valuable resource. Even in cases where SDK code is not going directly into a product, the source code is an indispensible guide to how the underlying hardware really works, and sometimes even more important, a guide to how the hardware is broken.

Next, there are several major interlocking open source ecosystems. The one best known is probably [Arduino](https://github.com/arduino/Arduino), which now runs on a number of different hardware platforms.

Arduino provides many of the same tools found in the vendor ecosystems, with severe limitations. The API is fairly limited, and support for MCUs ("cores") other than AVR and SAMD21 is provided by third parties, [STM32Duino](http://www.stm32duino.com) and [Teensyduino](https://www.pjrc.com/teensy/teensyduino.html) being two of the most notable. At the same time, the ecosystem has a broad base of users and a large collection of open source drivers of varying levels of quality.

Another is [LibOpenCM3](http://libopencm3.org) which provides a library for a large variety of Cortex-M devices, including a USB stack. LibOpenCM3 takes a very different approach than Arduino, with a clean, carefully designed
cross-platform API that covers a much broader range of functionality, as well as unified documentation for the [entire API](http://libopencm3.org/docs/latest/html/). Overall, it appears to be much more production-oriented,
and their [wiki](https://github.com/libopencm3/libopencm3/wiki/Showcase) points to several fairly known
products based on the library.

Finally, there are two major "unified" C/C++ ecosystems: [PlatformIO](https://platformio.org) and [Arm Mbed](https://www.mbed.com/en/). Both of these are attempts to develop cross-vendor development environments while
attempting to provide centralized repositories of drivers and components as well as additional functionality.

