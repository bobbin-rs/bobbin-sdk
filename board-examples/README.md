# Board Examples

These crates show how to create a new project using a board crate. In addition to creating the Cargo binary project and adding the board crate as a dependency, you will need to add development and release profiles that enable `panic=abort` and set appropriate optimization levels. Building with `debug` optimization is not recommended becauese of severe code size and performance issues.

Each example crate also includes a `.cargo/config` file (copied from the board crate) which defines the required rustc arguments needed to build the project for the target.

Finally, each board example includes a few simple applications that use the LEDs and serial console. Make sure that you have [bobbin-cli](https://github.com/bobbin-rs/bobbin-cli) installed. These applications can be built and run using:

```
$ bobbin run --bin [application name]
```
