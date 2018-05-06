# Goals

> If a problem cannot be solved, enlarge it. - Dwight D. Eisenhower

Bobbin SDK aims to provide a modern cross-platform software development platform for embedded
development using the [Rust](https://rust-lang.org/) programming language.

## Easy To Use

Bobbin SDK makes it easy to get up to speed, whether you are learning embedded development for
the first time, starting with a new embedded vendor or platform, or getting a new project off
the ground. Documentation and examples provide a starting point for building new applications
or learning how peripherals work.

## Cross Platform

Bobbin SDK makes cross-platform development possible while keeping it easy to take advantage
of vendor and MCU specific features. It also makes it possible to use vendor peripherals in a 
way that allows code reuse across MCUs.

# Comprehensive

Bobbin SDK doesn't cover just peripherals; it addresses MCU pin configuration, channel assignment,
and clock management too. It also provides RTOS primitives and services to support software architectures from pure-interrupt driven to mainloop / superloop to message-passing tasks.

Bobbin SDK will also provide cross-platform frameworks for implementing filesystem and networking stacks for FAT, USB, Bluetooth, IPv4 / IPv6 and IoT protocols, among others.

# Safe and Powerful

Bobbin SDK uses Rust's type system to build zero-overhead and near-zero overhead abstractions that do the heavy lifting at compile time. APIs are designed to support multiple types of abstractions with differing tradeoffs, from simple blocking IO to zero-copy, DMA-assisted asynchronous block
passing. Where possible, these are designed to be memory and concurrency safe, but with well defined escape hatches for developers that are building their own abstractions.

# Tools Included

Bobbin SDK includes tools that simplify working with flash loaders and debuggers and that make modern 
continous integration testing easier to implement. These tools are also exposed as APIs that can be used to build more complex systems.