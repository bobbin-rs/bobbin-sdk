# Peripherals

## Peripheral Basics

When you are working with peripherals, you will usually be working with two related types: the **Periph** type and the **Instance** type.

The **Periph** type has a name ending in *Periph* and provides access to the peripheral registers through a set of getter and setter methods described below. If a MCU has multiple copies of a peripheral, each of them can be accessed through an instance of the **Periph** type. Usually, if you want to create a struct that contains a peripheral or implement a new trait, you will use the **Periph** type.

Each peripheral in a MCU also has a unique **Instance** type. This **Instance** type is a smart pointer type that
implements `Deref<Target=PeriphType>`, so you can call methods from the underlying **Periph** directly. You can also use the `.as_periph()` method to get a reference to the underlying **Periph** instance.

Having unique **Instance** types makes it possible to implement traits for specific MCU peripherals and to use them as type parameters, enabling powerful compile-time functionality. See [FEATURES](../FEATURES.md) for more details.

Most of the time you don't have to think about this. If you are writing code for a specific peripheral on a specific MCU, you will usually be working directly with an **Instance** - in fact, a const instance of an **Instance** type.

### Peripheral Modules

All constants and types associated with a peripheral are defined in a module named after the peripheral base name. For instance, the WWDG peripheral is in the `wwdg` module and the GPIO peripherals are in the `gpio` module (if you import the MCU crate as `mcu` you would find them in `mcu::wwdg` and `mcu::gpio`).

Each peripheral in the MCU has a corresponding constant that is the name of the peripheral in uppercase. For instance, the `wwdg` module contains a `WWDG` constant that is an instance of the peripheral **Instance** type; the `gpio` module contains constants `GPIOA`, `GPIOB`, `GPIOC`, with corresponding **Instance** types.

Each module also defines single **Periph** type named using the base name in camelcase + `Periph`, as well as a type for each register.

The contents might look like this:

```
mod wwdg {
    // Instance Types
    pub const WWDG: Wwdg = Wwdg {};

    // Periph Types
    pub struct WwdgPeriph { ... }

    // Register Types
    pub struct Cr { ... }
    pub struct Cfr { ... }
    pub struct Sr { ... }
}

mod gpio {
    // Instance Types
    pub const GPIOA: Gpioa = Gpioa {};
    pub const GPIOB: Gpiob = Gpiob {};
    pub const GPIOC: Gpioc = Gpioc {};

    // Periph Types
    pub struct GpioPeriph { ... }

    // Register Types
    pub struct Moder { ... }
    pub struct Otyper { ... }
    pub struct Ospeedr { ... }
    ...
}
```



### Periph Methods

Each peripheral has several methods defined for each associated registers. The WwdgPeriph has these methods for the CFR register, which contains values of type `Cfr`:

- `cfg(&self) -> Cfr` - reads the register value
- `write_cfg(&self, value: Cfr) -> &Self` - writes the register value
- `set_cfg<F: FnOnce(Cfr)->Cfr>(&self, f: F) -> &Self` - writes the register value using a closure provided with the zero value.
- `with_cfg<F: FnOnce(Cfr)->Cfr>(&self, f: F) -> &Self` - writes the register value using a closure provided with the current value.
- `cfg_reg(&self) -> Register<Cfr>` - returns the register accessor object
- `cfg_mut(&self) -> *mut Cfr` - returns the *mut pointer to the register
- `cfg_ptr(&self) -> *const Cfr` - returns the *const pointer to the register

In most cases, `cfg()`, `set_cfg()` and `with_cfg()` are all that are needed for
reading, setting, and modifying the CFR register.

### Register Methods

Each register type has a number of methods to read, test, and update fields within the value. 

In this case the CFR register has the following fields:

- `EWI[9]`
- `WDGTB[8:7]`
- `W[6:0]`

and the following accessors:

- `ewi(&self) -> U1`
- `test_ewi(&self) -> bool`
- `set_ewi<V: Into<U1>>(mut self, value: V) -> Self`
- `wdgtb(&self) -> U2`
- `test_wdgtb(&self) -> bool`
- `set_wdgtb<V: Into<U2>>(mut self, value: V) -> Self`
- `w(&self) -> U7`
- `test_w(&self) -> bool`
- `set_w<V: Into<U7>>(mut self, value: V) -> Self`

The types U1, U2, and U7 indicate that 1-bit, 2-bit and 7-bit values are valid and
are come from [bobbin-bits](https://github.com/bobbin-rs/bobbin-bits). These
values convert easily to and from primitive values using `.value()`, `.from()` and `.into()`.

It's important to remember that field values are just that: values. Updating them in place
doesn't do anything to the underlying register or peripheral; you need to use the peripheral register methods to write them back. Usually you can use the `set_xxx()` and `with_xxx()` methods on the peripheral to modify the individual fields that you care about.

Also, these values types are Copy and the `set_xxx()` field methods will return a new, modified value instead of modifying the value in place. You always want to do

```
value = value.set_w(0x30);
```
instead of
```
value.set_w(0x30); // constructs a new value and throws it away.
```

Some examples:

```
// Set the CR register so that the T field is 0x30 and the WDGA field is zero
// Performs a write

WWDG.set_cr(|r| r.set_t(0x30));

// Update the EWI and W fields of the CFR register, leaving the WDGTB field unmodified
// Performs a read and then write

WWDG.with_cfr(|r| r.set_ewi(1).set_w(0x50));

// Chain together updates - results in a write, read, then write sequence.

WWDG
   .set_cr(|r| r.set_t(0x30))
   .with_cfr(|r| r.set_ewi(1).set_w(0x50));


// loop while the EWIF flag of the SR field is 0 (false)

while !WWDG.sr().test_ewif() {}

// Another way of looping while the SR field is 0 (probably clearer)

while WWDG.sr().ewif() == 0 {}

// do something if CFR.W is less than 64

if WWDG.cfr().w() < 64 {
    // do something
}
```

Value types are simple wrappers around the underlying unsigned integer of the same width as the register, so you can
construct them directly using the type as a constructor and access the raw value using tuple notation. In that case, there is no type checking and you are responsible for ensuring that the value is valid for the register.

```
let v_cfr: Cfr = Cfr(0x10); // Construct the Cfr from a u32;
let v_int: u32 = Cfr.0; // Get the u32 value from the Cfr.
```


### Indexed Registers

Many peripherals contain arrays of registers of the same type. Accessing these registers is easy: the
same methods are used, except that there is an additional `index` parameter.

For instance, the STM32 DmaPeriph has an array of 8 SCR registers, which can be accessed with the following methods:

- `scr<I: Into<R8>>(index: I) -> Scr`
- `set_scr<I: Into<R8>, F: FnOnce(Scr) -> Scr>(index: I, f: F) -> &Self`
- `with_scr<I: Into<R8>, F: FnOnce(Scr) -> Scr>(index: I, f: F) -> &Self`
- `write_scr<I: Into<R8>>(index: I, value: Scr) -> &Self`
- `scr_reg<I: Into<R8>>(index: I) -> RegisterArray<Scr, R8>`
- `scr_mut<I: Into<R8>>(index: I) -> *mut Scr`
- `scr_ptr<I: Into<R8>>(index: I) -> *const Scr`

For this register array, the `R8` type indicates that the index must be an integer value from 0 to 7. The `R8` type
implements `From` for most primitive integer types so you can simply write

```
if DMA0.scr(3).test_mburst() {
    // do something
}
```

### Indexed Fields

Many registers contain arrays of fields. Similar to indexed registers, you access a specific element by
using the same field methods with an additional `index` parameter.

For instance the STM32 GPIO.MODER register contains an array of 16 2-bit fields, also named `moder`. The Moder type has the follosing methods:

- `moder<I: Into<R16>>(&self, index: I) -> U2`
- `test_moder<I: Into<R16>>(&self, index: I) -> bool`
- `set_moder<I: Into<R16>, V: Into<U2>>(self, index: I, value: U2) -> Self`

As in the register array, the `R16` type for the index indicates that the index must be an integer value from 0 to 16.

An example:

```
// Update GPIOA.MODER.MODER[0] = 0b01, MODER[1] = 0b10, MODER[2] = 0b01

GPIOA.with_moder(|r| r
    .set_moder(0, U2::B01)
    .set_moder(1, U2::B10)
    .set_moder(2, U2::B01)
);