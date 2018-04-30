# Features

## MCU Crates

- Vendor Crates
   - Many MCUs share peripherals with other MCUs from the same vendor. 
   - These common peripherals have been consolidated into vendor-common crates,
   which are then re-exported by the MCU specific crate.
   - In some cases, there may be several major variants of a peripheral used
   in differerent MCUs in the family. The MCU specific crate will use the appropriate peripheral
   variant.
   - This allows driver and application writers to target a specific peripheral in a
   vendor-common crate and use that code on any MCU supporting that peripheral.

- Variants
   - Each MCU crate defines a number of variants corresponding to different models that
   may have different Flash, SRAM, and even peripheral variations. The specific variant
   used can be selected by enabling a Cargo feature.
   - Currently, the main use of variants is to automatically select the linker memory
   map for the variant selected.
   - Variants can also be used to enable / disable specific peripherals and peripheral features
   at compile time.

- Pins and Signals
   - Pins and Signals allow compile-time type-checked connections between MCU pins and the peripherals they are
   associated with.
   - For instance, a UART may have TX and RX signals, and only the pins that provide those signals may be connected to
   that UART.
   - Connecting a pin to a peripheral automatically configures the alternate function and pin mode. Additional
   pin configuration may be necessary in some cases - enabling pull-up or open-drain, for example.
   - Pins and Signals allow writing drivers and applications that support type-checked configuration of user-supplied
   peripherals and pins across MCUs.

- Channels
   - Some peripherals contain groups of sub-components that are partially or fully independent.
   - ADC, DAC, Timers, DMAs are common examples
   - Channels allow those sub-components to be treated as first class
       - Channels can have signals assigned so that pins can be connected to them (ADC, DAC and Timers use this).
       - Interrupts can be assigned to channels where appropriate.
       - Traits can be implemented for channels as well as for peripherals.
   - Channels allow writing drivers and applications that abstract at boundaries that reflect the underlying
   peripheral architecture.

- Clocks 
   - Each MCU crate has a trait that defines all of the clocks for the MCU
   - Most peripherals have traits specify the MCU clock that is the input for the peripheral.
   - Most MCU crates provide a dynamic clock implementation that supports run-time clock calculation
   for MCU clocks and peripherals.
   - This allows writing drivers and applications that can perform clock-related configuration
   across peripherals, MCUs, external clock sources, and even run-time changes.
   - Most crates also provide basic clock initialization that supports configuring the clock in
   a high-performance mode. Future crates will provide more sophisticated clock drivers that support
   full runtime configuration of the clock subsystem.

- Gates
   - Peripherals also support `Gates`, which enable or disable input clocks for the peripheral.
   - Often there are additional gates such as `Reset`, which resets the peripheral, and other types of
   gates that allow the peripheral to run in sleep or low power modes.
   - These peripheral gate traits make it possible to enable or disable the peripheral by updating the
   appropriate bit in the register of the clock control peripheral for the MCU.
   - This allows writing drivers and applications that can enable or disable peripherals as needed.

- Interrupts
   - Each MCU crate defines a list of valid interrupts and defines default handlers that can be overridden
   using the cortex-m-rt interrupt handler macros.
   - Additionally, each crate defines traits that associate individual interrupts with specific peripherals
   and peripheral channels. Some peripherals may support multiple types of interrupts - for instance, both
   General and Error interrupts.
   - These traits support writing drivers and applications that can self-register appropriate interrupts based
   on the MCU and specific peripheral being used.

- MCU Traits
   - Additional traits can be implemented for MCUs to allow MCU-agnostic and even architecture-agnostic devices 
   and applications. For instance, current MCU crates implement traits for enabling / disabling interrupts globally 
   as well as enabling / disabling specific interrupts.

- Introspection
   - All peripheral registers have Debug output traits implemented, showing the register value as well as values of individual fields.

- Ownership and Reference Counting
   - All peripherals, pins, and channels have static variables and methods defined to support ownership and reference counts. The MCU crate itself does not use these traits, but they can be used by higher-level crates or by drivers and applications to ensure that peripherals, pins, and channels are used according to some policy or to implement automatic resource management.

## Board Crates

- Configuration
   - Use the specific MCU crate and variant that is on the board. This sets up the appropriate linker configuration
   - Includes a script for OpenOCD or other debugging tools as needed.
   - Set up the dynamic clock driver with the specific external clocks used on the board.
   - Defines the standard console, usually one connected to an on-board debugger.
   - Defines on-board LEDs and buttons
   - Defines aliases from MCU pin IDs to connector-specific IDs - for instance, Arduino pin names.
   - Board-specific initialization of clocks and peripherals
   - Optionally, include higher-level drivers for external on-board peripherals.

- System Services
   - Console - `printf!` macros as well as low-level console output methods.
   - Clock - access is provided to the dynamic clock provider for the board.
   - Heap - a simple allocate-only heap allocator is provided for applications and drivers that don't want to
   depend on the `alloc` crate. This allocator returns raw pointers and &'static mut references and does
   not support freeing objects. The allocator is only available during board intialization and does not allow
   allocation during runtime.
   - Timer - a basic millisecond timer is instantiated and made available to drivers and applications
   that need basic timekeeping for delays and timeouts. Future versions of this timer will support registering
   one-shot and periodic tick callback handlers.
   - Interrupt Dispatcher - a dynamic interrupt dispatcher allows drivers and applications to register
   interrupt handlers at run time. Guards are used to ensure that handlers are automatically unregistered
   and disabled when they they are dropped. This makes it possible to safely write interrupt handlers that are stored on the stack instead of in statically allocated memory.
   - Collectively, these system services make it possible to write portable drivers and applications that can run on many different boards and MCUs. In most cases, some amount of configuration will be needed to select the specific peripherals and pins to be used, but that configuration may be checked at compile time to ensure consistency.