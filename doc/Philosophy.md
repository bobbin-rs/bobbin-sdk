# Philosophy / Goals

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

Vendors don't care much about this. They've invested considerable effort into solving these
problems through extensive hand-written SDKs and code-generation tools and don't see much 
payoff from making it easier to program using other languages - particularly if those other
languages make it easier to switch between vendors.

On the other hand, the Rust community has a big interest in having a ecosystem that deals with
these issues.

The history of computing shows that big gains are often made by languages that make their 
problems bigger and find a way to solve them. Python decided to make "batteries included"
so that users could tackle a broad set of problems with just the standard library; Ruby as
a community embraced all aspects of writing web application servers and achieved widespread
usage in that domain; Go took that a step further by making concurrency easy while solving 
packaging and deployment with single-binary application.

Bobbin does the same by making the problem bigger. The goal isn't just to make it possible to
program MCUs with rust; embedded programming in Rust needs to be better (much better) than 
using vendor SDKs, code generation tools, and proprietary IDEs.

## Leverage the type system

## Provide easy low level access to peripherals

MCUs are organized the way they are in order to solve problems that MCU vendors have; whether
that organization makes sense for the end user is incidental. Because of that, the lowest
layer should be the simplest and have the least policy; they should map directly to what is
available without making judgements on what is and isn't allowed.

More opinionated layers using Rust's type system to enforce some notion of correctness should be
layered on top of these lowest level APIs.