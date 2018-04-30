# Philosophy / Goals

## Provide easy low level access to peripherals

MCUs are organized the way they are in order to solve problems that MCU vendors have; whether
that organization makes sense for the end user is incidental. Because of that, the lowest
layer should be the simplest and have the least policy; they should map directly to what is
available without making judgements on what is and isn't allowed.

More opinionated layers using Rust's type system to enforce some notion of correctness should be
layered on top of these lowest level APIs.

## Make the problem bigger

> If a problem cannot be solved, enlarge it. - Dwight D. Eisenhower

A modern MCU is a system on a chip, with relationships between internal peripherals and a
network fabric between those peripherals and the pins connecting them to the outside world.

[CMSIS-SVD](http://arm-software.github.io/CMSIS_5/SVD/html/index.html) provides one view
of the core and peripherals of a chip - primarily a description of the peripherals, registers,
and fields in a MCU. This is enough for tools that have been used to boostrap the Rust embedded
ecosystem.

At the same time, the SVD approach has many limitations.

- SVD files describe individual chips and are self-contained files; there is no provision for 
sharing peripheral definitions between chips even if they are identical
- Even within SVD files, there are limited facilities for identifying that peripherals share
the same implementation.
- SVD doesn't describe sub-peripheral abstractions such as Pins and Channels.
- SVD doesn't deal with the relationship between Pins and Peripherals and/or Channels.
- SVD doesn't attempt to describe the MCU's clock tree or the relationship between peripherals
and the clock tree.

Vendors don't care much about these limitations because they address them through hand-coded
SDKs and proprietary code generation tools.

## Leverage the type system

