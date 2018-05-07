# Applications

This directory contains examples of cross-platform application development using [bobbin-sys](../lib/bobbin-sys/), [bobbin-hal](../lib/bobbin-hal), and [embedded-hal](https://github.com/japaric/embedded-hal).

## Embedded-HAL Applications

Some application examples use simple wrappers implementing `embedded-hal` traits. [NOTE: These wrappers should be moved into the either MCU implementation crates, [bobbin-hal](../lib/bobbin-hal/), or in a separate `embedded-hal` compatibility crate]. To run these applications, you will need to instantiate the appropriate peripherals and wrappers for the device that you are using and then pass them to the application example.

## Bobbin-Sys Applications

[bobby-sys](../lib/bobbin-sys/) applications work at a higher level and can perform self-initialization, including setting up pins, enabling peripherals, and registering interrupt handlers. In many cases these applications can be run with no additional configuration if the defaults for the device are appropriate.

In some cases, there may be several possible hardware configurations appropriate for an application. Here are a few strategies for handling configuration:

### Trait-Based Configuration

With this technique, create a trait to be implemented on the local SystemProvider that provides methods to retrieve configured peripherals. For instance, if your application requires a SPI peripheral, an additional GPIO pin, and a ADC, create a trait with methods that will return those peripherals and add it as a type constraint for instantiating your application. The implementer is responsible for instantiating and configuring the peripherals and pins in a way that is appropriate for the specific hardware configuration.

### Configuration Type-based Configuration

An alternative is to create a configuration type that contains the peripherals that your application needs and have the user pass this to the application.

### Pin Configuration

In some cases you might want to have the user pass pins explicitly and configure them within your application, particularly if you need to do dynamic configuration during application execution (for instance, switching between input and output modes). You must use type constraints to ensure that the pins provided by the user are consistent with any peripherals that are also passed in.

### Clock Calculations

Many peripherals will need access to system clock tree calculations for configuration (to set dividers for timers, for instance). The easiest way to do this is to have your application accept a `System` reference so that you can access the clock tree using `.clk()` and then use the `clock_for()` methods to retrieve the dynamic input clock.

### Console

All boards will have a `Console` defined, though whether this is a serial, ITM, or other type of console will vary. It's possible that a resource constrained board might define a dummy console providing no output at all.

Although board implementers may implement high-performance buffered output for their console devices, most will implement the simplest blocking device driver possible. Assume that writing to the console will block.

### Buttons and LEDS

Some but not all boards will have LEDs and buttons; the order and color of each may not be what you expect. If your application requires a specific configuration of LEDs (for instance a RGB LED implemented as three independent channels) you should have the user implement a trait or pass in the specific LEDs when instantiating your application.

### Tick, Pend, and Interrupt Handlers

The board implementer will usually provide a limited number of handler slots for ticks, pends, and interrupts. You should check the status of all registrations and return an error (and perhaps display an error message) if the number of handler slots is not sufficient.