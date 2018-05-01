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

- Channels
   - Some peripherals contain groups of sub-components that are partially or fully independent.
   - ADC, DAC, Timers, DMAs are common examples
   - Channels allow those sub-components to be treated as first class
       - Channels can have signals assigned so that pins can be connected to them (ADC, DAC and Timers use this).
       - Interrupts can be assigned to channels where appropriate.
       - Traits can be implemented for channels as well as for peripherals.
   - Channels allow writing drivers and applications that abstract at boundaries that reflect the underlying
   peripheral architecture.


- Pins and Signals
   - Pins and Signals allow compile-time type-checked connections between MCU pins and the peripherals they are
   associated with.
   - For instance, a UART may have TX and RX signals, and only the pins that provide those signals may be connected to
   that UART.
   - Connecting a pin to a peripheral automatically configures the alternate function and pin mode. Additional
   pin configuration may be necessary in some cases - enabling pull-up or open-drain, for example.
   - Pins and Signals allow writing drivers and applications that support type-checked configuration of user-supplied
   peripherals and pins across MCUs.


Example: Connect TX and RX pins to a USART.

```
use board::prelude::*;
use mcu::usart::*;
use mcu::pins::*;

const USART: Usart3 = USART3;
const USART_TX: Pd8 = PD8;
const USART_RX: Pd9 = PD9;

fn main() {

    // Connect USART_TX and USART_RX to USART
    //   - will fail at compile time if invalid

    USART_TX.connect(USART); 
    USART_RX.connect(USART); 

    // Continue using USART

}
```

Example: Connect a pin to a timer channel.

```
use board::prelude::*;
use board::mcu::pin::*;
use board::mcu::tim_gen::*;

// PWM output on PB0 / TIM3_CH3

const TIM: Tim3 = TIM3;
const TIM_CH: Tim3Ch3 = TIM3_CH3;
const TIM_PIN: Pb0 = PB0;

fn main() {

    // Connect TIM_PIN to TIM_CH
    //   - will fail at compile time if invalid

    TIM_PIN.connect_to(TIM_CH);

    // Continue using TIM_CH
}
```

Example: Connecting a pin to an ADC channel.
```
use board::prelude::*;
use board::mcu::pin::*;
use board::mcu::adc::*;

const ADC_CH: Adc1Ch3 = ADC1_CH3;
const ADC_PIN: Pa3 = PA3;

fn main() {

    // Connect ADC_PIN to ADC_CH
    //    - will fail at compile time if invalid

    ADC_PIN.connect_to(ADC_CH);

    // Continue using ADC_CH
}
```

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

Example: Print the input clock for the selected timer.

```
use board::prelude::*;
use board::prelude::*;
use board::mcu::tim_gen::*;

const TIM: Tim3 = TIM3;

fn main() {
    // Assume board::clock() returns a system clock handle

    let clk = board::clock();
    let tim_hz = clk.clock_for(TIM);

    println!("Timer Input Clock: {}", tim_hz.as_u32());
}

```

- Gates
   - Peripherals also support `Gates`, which enable or disable input clocks for the peripheral.
   - Often there are additional gates such as `Reset`, which resets the peripheral, and other types of
   gates that allow the peripheral to run in sleep or low power modes.
   - These peripheral gate traits make it possible to enable or disable the peripheral by updating the
   appropriate bit in the register of the clock control peripheral for the MCU.
   - This allows writing drivers and applications that can enable or disable peripherals as needed.


Example: Enable a Timer Peripheral

```
use board::prelude::*;
use board::mcu::tim_gen::*;

const TIM: Tim3 = TIM3;

fn main() {

    // Enable TIM clock gate

    TIM.gate_enable();

    // continue working with TIM

}

```

Example: Enable an ADC and pin

```
use board::prelude::*;
use board::mcu::pin::*;
use board::mcu::adc::*;

const ADC_CH: Adc1Ch3 = ADC1_CH3;
const ADC_PIN: Pa3 = PA3;

fn main() {

    // Enable the peripheral that the ADC channel belongs to

    ADC_CH.periph().gate_enable();

    // Enable the port that the ADC Pin belongs to

    ADC_PIN.port().gate_enable();

    // Continue using ADC_CH
}

```

Example: Enable and connect USART, TX and RX pins. Configure the USART baud rate.
```
use board::prelude::*:
use mcu::usart::*;
use mcu::pins::*;

const USART: Usart3 = USART3;
const USART_TX: Pd8 = PD8;
const USART_RX: Pd9 = PD9;
const USART_BAUD: u32 = 115_200;

fn main() {

    // Enable USART Clock

    USART.gate_enable();

    // Enable TX and RX ports Clock
    //   - The TX and RX port are the same in this case so it gets enabled twice, which is OK.
    //   - also could use the method port_gate_enable() to do the same thing.

    USART_TX.port().gate_enable();
    USART_RX.port().gate_enable();

    // Connect the TX and RX pins to the USART
    //   - fails at compile time if invalid

    USART_TX.connect(USART);
    USART_RX.connect(USART);

    // Get the USART input clock
    //   - Assume board::clock() returns a system clock handle

    let clk = board::clock();
    let usart_hz = clk.clock_for(USART);
    
    // Set the baud clock for the USART.
    USART.set_baud_clock(USART_BAUD, usart_hz.as_u32());

    // Enable the USART 

    USART.enable();

    // Continue using USART

}
```

- Interrupts
   - Each MCU crate defines a list of valid interrupts and defines default handlers that can be overridden
   using the cortex-m-rt interrupt handler macros.
   - Additionally, each crate defines traits that associate individual interrupts with specific peripherals
   and peripheral channels. Some peripherals may support multiple types of interrupts - for instance, both
   General and Error interrupts.
   - These traits support writing drivers and applications that can self-register appropriate interrupts based
   on the MCU and specific peripheral being used.

Example: Print the interrupt number for the USART

```
use board::prelude::*:
use mcu::usart::*;
use mcu::irq::*;

const USART: Usart3 = USART3;

fn main() {

    // Get the interrupt number for the USART.
    //   - peripherals may have more than one type of interrupt, so IRQ_USART
    //     must be specified.

    let irq_num = USART.irq_number_for(IRQ_USART);

    println!("USART: Interrupt {}", irq_num);
}

```

- MCU Traits
   - Additional traits can be implemented for MCUs to allow MCU-agnostic and even architecture-agnostic devices 
   and applications. For instance, current MCU crates implement traits for enabling / disabling interrupts globally 
   as well as enabling / disabling specific interrupts.

- Introspection
   - All peripheral registers have Debug output traits implemented, showing the register value as well as values of individual fields.

```

use board::prelude::*;
use board::mcu::usart::USART3;

fn main() {
    println!("USART3.CR1:   {:?}", USART3.cr1());
    println!("USART3.CR2:   {:?}", USART3.cr2());
    println!("USART3.BRR:   {:?}", USART3.brr());

    // produces the following output similar to the following
    //
    // USART3.CR1:   [0x0000000d te re ue]
    // USART3.CR2:   [0x00000000]
    // USART3.BRR:   [0x0000008a div_mantissa=0x8 div_fraction=0xa]
}
```




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

- Examples
   - Each board crate comes with a set of examples