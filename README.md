# The Bobbin Project

This repository contains components of the Bobbin Project, a multi-platform SDK for
embedded systems development in the [Rust](https://www.rust-lang.org/en-US/) programming language.

Core Components

- [Bobbin DSL](./dsl/) - A DSL for describing MCUs, Boards and Peripherals
- [Bobbin Lib](./lib/) - Rust crates used by generated Bobbin MCU and Board crates.
- [Bobbin MCU](./mcu/) - Rust MCU Support crates
- [Bobbin Periph](./periph/) - Rust crates describing non-MCU peripherals
- [Bobbin Board](./board/) - Rust Board Support crates
- [Bobbin App](./app/) - Sample cross-platform rust applications

Additional Repositories

- [Bobbin Bits](https://github.com/bobbin-rs/bobbin-bits/) - Small bit fields and ranged integers for Rust
- [Bobbin CLI](https://github.com/bobbin-rs/bobbin-cli/) - A Rust command line tool to simplify embedded development and deployment.