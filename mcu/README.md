# Bobbin MCU Support Crates

## Organization

- [bobbin-cortexm](./bobbin-cortexm/) - Common register and trait definitions for ARM Cortex-M MCUs.
- [bobbin-kinetis](./bobbin-kinetis/) - NXP Kinetis MCU Support
    - [kinetis-common](./kinetis-common/) - Kinetis common peripherals and traits
    - [k64](./k64/) - Kinetis K64 MCU
- [bobbin-sam](./bobbin-sam/) - Atmel SAM MCU Support
    - [sam-common](./sam-common/) - SAM common peripherals and traits
    - [samd21](./samd21/) - SAMD21 MCU
- [bobbin-stm32](./bobbin-stm32/) - STM32 MCU Support
    - [stm32-common](./stm32-common/) - STM32 common peripherals and traits
    - [stm32f74x](./stm32f74x/) - STM32F74x / STM32F76x MCU Support
    - [stm32f303x](./stm32f303x/) - STM32F303x MCU Support
    - [stm32l432x](./stm32l432x/) - STM32L432x MCU Support

MCU crates depend on vendor-common crates which then depend on architecture-common crates. Currently only Cortex-M0+, M3, M4 and M7 processors are supported but MSP430 and RISCV support is planned.

Each MCU crate has feature support for a number of variants. Currently this is only used for build-time linker file selection, but in the future peripheral definitions in crates may vary depending on variant type.

Most MCU crates have a combination of locally-defined peripherals and shared peripherals defined in vendor-common and architecture-common crates. System configuration peripherals (particularly clocking and power control peripherals) will always be MCU specific, but some other peripherals may over time migrate to vendor-common crates.

Some MCU crates may be missing definitions for some peripheral types. [bobbin-svd](../dsl/bobbin-svd/) may be used to generate
peripheral definitions that may be added to the source crate in [mcu-src](../mcu-src/). New peripherals should be cleaned up (make sure field and register names match what is in the datasheet, add channels and consolidate registers and field arrays whenever possible) and then tested first in a MCU crate before being added to a vendor-common crate.

## MCU Crate Structure

MCU crates are generated using the [bobbin-chip](../dsl/bobbin-chip/) tool, a source file in the [mcu-src](../mcu-src/) directory, and a crate template directory (also in [mcu-src](../mcu-src/)).

Each MCU crate contains:

- A Cargo.toml file
- A build.rs file that uses [bobbin-build](../lib/bobbin-build/) to select a linker file based on selected variant
- A link/ directory that has a [variant].x file corresponding to each variant name
- A .cargo/config file with the correct configuration for building the MCU crate
- A simple Makefile that can be used to build the crate.
- A src/ directory containing:
    - `lib.rs` which imports locally-defined and external vendor-common peripheral modules
    - `irq.rs` which contains interrupt definitions and traits
    - `sig.rs` which defines signals used for peripheral connection management
    - `pin.rs` which defines MCU pins and signals
    - `clock.rs` which defines the MCU clock tree and peripheral clock traits
    - A module for each peripheral or peripheral group, containing the peripheral, channels, and possibly implementation as well as trait definitions.
    - A ext/ directory which usually includes:
        - `mod.rs` which imports submodules and contains the main MCU trait definitions,
        - `clock.rs` which contains clock initialization and clock tree implementation code
        - A module for each peripheral group containing peripheral and channel implementations as well as extension trait definitions.

All code directly in the `src/` directory is automatically generated and may be overwritten, while the `src/ext` directory is intended for manually written code and will always be preserved.

## Coding Style

The coding style in the `ext/` directory is still evolving. In some MCU crates, additional inherent methods have been directly implement for peripherals; over time, these will should be converted to vendor-common extension traits. MCU crates provide a fairly low level
programming interface with few safety guards and restriction. In the long run, most applications should use higher-level cross-platform (or at least cross-MCU) drivers that provide a safer, simpler API, and the APIs provided by the MCU crates should be focused primarily at those users.

## Peripheral, Pin and Channel Definitions

MCU crates depend heavily on the [bobbin-mcu](../lib/bobbin-mcu/) crate, which defines most of of the data types, traits, and macros used by code generation. You may wish to review [Peripherals](../doc/Peripherals.md) in order to understand the relationship between the unique peripheral / pin / channel instance types and the underlying concrete implementation types.