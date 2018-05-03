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

Finally, there are two major "unified" C/C++ ecosystems: [PlatformIO](https://platformio.org) and [ARM Mbed](https://www.mbed.com/en/). Both of these are attempts to develop cross-vendor development environments while
attempting to provide centralized repositories of drivers and components as well as additional functionality.
The main effort appears to be improving tools (both PlatformIO and Mbed have made an effort to make
deployment to various devices easier and more unified) and providing new functionality (cloud-based compilation, CI, and deployment for IoT devices).

One difference between the two is that while PlatformIO is focused almost exclusively on providing a large
collection of frameworks and libraries, ARM Mbed does provide a unified [C/C++ SDK](https://os.mbed.com/users/mbed_official/code/mbed/) (part of [Mbed OS](https://os.mbed.com/docs/v5.8/reference/index.html)) which is designed as a cross-platform library.

## How does Rust stack up?

### embedded-hal

None of the ecosystems mentioned above has anything exactly like embedded-hal because of the limitations of
C and C++. It's pretty unique to be able to write a peripheral driver and be able to use it with
an arbitrary device that implements the appropriate traits. Even with Arduino, where there are a reasonable number of cross-device drivers, integration is typically done by copying source code into the application and
then editing it to make it work.

On the other hand, embedded-hal covers a very narrow set of functionality (typically sending and receiving
data through an I/O interface such as serial, I2C or SPI) involving the operation of a hardware device,
but nothing about instantiating or configuring a device. It also is currently a bit of a lowest-common-denominator
API so that it can be implemented broadly, which means that it doesn't expose other API models that
might be desired for more demanding applications.

### svd2rust

[svd2rust](https://github.com/japaric/svd2rust) is currently used by most Rust embedded crate authors to 
generate register maps from SVD files. There isn't really an equivalent in the C/C++ ecosystem because
in most cases users can use header files directly from device vendors.

`svd2rust` plays an incredibly important part in making it easier for users to create high-quality, consistent
crates for a wide range of devices. The code that it generates is well thought out and uses many Rust language
features to be as safe as possible while being as high performance as C/C++ header files.

At the same time, `svd2rust` has limitations from working with SVD files as a source. SVD files are designed
to support a single chip at a time, and the specification does not support the concept of peripherals
shared between different devices. As a result, each resulting crate is independent: from Rust's point of
view, a USART defined in one crate is a differemt type than a USART defined in a different crate, even if those
two crates are for two different models of the same chip that use the exact same USART device.

A related issue is that SVD files are often not high-enough quality to use directly from the source - there
may be inconsistencies in naming (some vendors add prefixes to all register and field names, for instance),
and very often, more "advanced" features of SVD such as register and field arrays are not implemented, leading
to extremely large Rust APIs. I believe this happens because the main use of SVD from a vendor point of view is
for graphical debuggers and hardware visualization tools, not for code generation.

Because of that, many crates end up maintaining large patches that must be applied to the SVD files before generation. While this isn't too bad from a maintainer's point of view (they can always work with a local
copy of the SVD file and generate the patch afterwards), this can lead to a big loss of discoverability if
the modified version isn't what is uploaded to repositories such as Github.

There are also issues with the visibility and usability of the single generated Rust file that `svd2rust` produces. For instance, [japaric/stm32f30x/src/lib.rs](https://github.com/japaric/stm32f30x/blob/master/src/lib.rs) is a single 9.29MB file that cannot be browsed on Github. There are examples of hardware crates where it's not even possible to view
the documentation online because the HTML files are tens of megabytes long.

In the long run there is also the issue that any codegen changes to `svd2rust` can potentially cause incompatible changes to any crate generated by `svd2rust`, leading to breakage in any downstream crates. This means that `svd2rust`
could become difficult to make improvements to, or alternatively, we might end up with crates that use different versions of `svd2rust` that present slightly different APIs. This might or might not be a problem.

This compares to vendor SDKs, where headers and low-level libraries are designed so that identical peripherals share
the same interface and implementation, drastically reducing the amount of code that needs to be understood. LibOpenCM3
is an even better example, with support for dozens of devices in a single hand-maintained library.

Finally, `svd2rust` (and SVD itself) doesn't concern itself with anything outside of memory-mapped registers except
for interrupts. Concepts such as pin allocation, clock trees, clock gates, and sub-peripheral components (DMA channels, timer channels, ADC channels) don't exist and can't be added.

### crates.io

Cargo and [crates.io[(https://crates.io)] are an integral part of the Rust programming experience. Nothing in the
C/C++ ecosystem compares, and the community is far too established and fragemented for anything to ever reach the
100% market share that crates.io has within the Rust community. Because of this, Rust has a better code-sharing
story than any other systems language.

On the other hand, crates.io is a mixed blessing for embedded developers. The biggest issue is discoverability,
at several levels.

First, there are tens of thousands of crates now, and likely hundreds of thousands in the near future. Only a tiny 
fraction of these are embedded-related or can be run without a standard library on embedded devices, and they can
be hard to find.

Second, because crates.io is first-come first-serve and has no curation tools, it can take a lot of research to know whether a particular crate is suitable or is maintained or designed to work with specific other crates. Downloads
and dependency counts can tell you how popular a crate is, but that's only a single (albeit useful) signal that tells
you how suitable it is for your project.

One specific issue that may arise in the near future is the issue of having different driver crates using different register definition crates.

### What's missing?

There are a lot of items from the first set of lists (the Vendor SDK list) that don't currently exist in the Rust community. A few key items, some of which have been touched upon previously:

- No support for common peripherals
- No support for modeling many import MCU concepts such as pin assignments, clock trees, and sub-peripheral components
- No support for board or driver initialization
- No support for a general-purpose Rust RTOS
- Poor discoverability of embedded-related crates that are supposed to work together
