# Bobbin SDK

This repository contains Bobbin SDK, a multi-platform Software Development Kit for
embedded systems development using the [Rust](https://www.rust-lang.org/en-US/) programming language.

NOTE (February 12, 2020) - Due to personal time constraints, this project is currently unmaintained. 
There is a significant amount of work that needs to be done to make it compatible with the greater 
Embedded Rust ecosystem that exists today and to support new hardware.

If anyone is interested in becoming a maintainer of the project as a whole or of specific pieces of the project,
please feel free to open a pull request or contact me at jcsoo@bobbin.io.

NOTE (May 11, 2019) - IRQ handling needs to be revamped to be compatible with cortex-m-rt, and has 
temporarily been removed from MCU codegen. The design of the board support crates also need to be
updated to use cortex-m-rt attributes instead of macros.

## Organization

- [DSL](./dsl/) - A DSL for describing MCUs, Boards and Peripherals
- [Lib](./lib/) - Rust crates used by generated Bobbin MCU and Board crates.
    - [Bobbin Build](./lib/bobbin-build/) - Build-time Tools
    - [Bobbin Hz](./lib/bobbin-hz/) - Fractional Frequency Calculations
    - [Bobbin MCU](./lib/bobbin-ipc/) - MCU Traits and Macros
    - [Bobbin HAL](./lib/bobbin-hal/) - Hardware Abstraction Layer
    - [Bobbin Sys](./lib/bobbin-sys/) - System Abstraction Layer
    - [Bobbin IPC](./lib/bobbin-ipc/) - Simple IPC primitives
- [MCU](./mcu/) - MCU crates
- [MCU Source](./mcu-src/) - MCU crate source files
- [MCU Examples](./mcu-examples/) - MCU usage examples
- [Board](./board/) - Rust board support crates
- [Board Source](./board-src/) - Board crate source files
- [Board Examples](./board-examples/) - Board usage examples
- [Periph](./periph/) - Non-MCU peripheral crates
- [Periph Source](./periph-src/) - Peripheral crate source files
- [App](./app/) - Sample cross-platform rust applications

Additional Repositories

- [Bobbin Bits](https://github.com/bobbin-rs/bobbin-bits/) - Small bit fields and ranged integers for Rust
- [Bobbin CLI](https://github.com/bobbin-rs/bobbin-cli/) - A Rust command line tool to simplify embedded development and deployment.

## Goals

Why is Bobbin designed the way it is? See [Goals](./doc/Goals.md).

## Features

What makes Bobbin special? See [Features](./doc/Features.md).

## Hardware

See a list of supported [Hardware](./doc/Hardware.md).

## Quick Start

Get started with a MCU or Board crate example. See [Quick Start](./doc/Quickstart.md).

## More Documentation

More documentation in available the [doc](doc/) directory.
