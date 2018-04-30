# Hardware Support

> **NOTE:** In order to focus on an initial preview release, the number of supported
> boards and MCUs has been kept to a minimum. In the next few months, many of the MCUs
> and boards listed below will be updated to the current standard and released.

## Initial Targets

These targets were selected to cover range modern MCUs used in a number of widely used 
development boards.

### MCUs

- STM32
   - STM32F30x
   - STM32F7x
   - STM32L4x
- Kinetis
   - K64
- SAM
   - SAMD21

### Boards

- Discovery / Nucleo
   - Discovery-STM32F3
   - Nucleo-F746ZG
   - Nucleo-L432KC
- FRDM
   - FRDM-K64F
- Arduino
   - Arduino Zero
- Feather
   - Feather M0

## Secondary Targets

These targets are not currently in HEAD but have implementations that need to be updated to the current
version.

### MCUs

- STM32
   - STM32F10x
   - STM32F20x
   - STM32F4x
   - STM32L0x
- Kinetis
   - K20
   - KL26
   - S32   
- TI
   - TM4C129x
- SILabs
   - EFM32PG12 
- Ambiq
   - Apollo2

### Boards

- Discovery / Nucleo
   - Discovery-STM32F4
   - Nucleo-F103RB
   - Nucleo-F207ZG
   - Nucleo-F302R8
   - Nucleo-F303RE
   - Nucleo-F429ZI
   - Nucleo-L031K6
- FRDM
   - FRDM-KL26Z
   - EVB-S32K144
- Teensy
   - Teensy LC
   - Teensy 3.2
   - Teensy 3.5
   - Teensy 3.6
- TI
   - EK-TM4C1294XL
- SILabs
   - SLSTK3401a
- OneBit
   - OneBit 1Bitsy
   - OneBit Elle0
- OpenMV
   - OpenMV M7
- Apollo
   - AMAPH1KK-KBR
- NERO
   - Nero F7
- Generic / Chinese
   - Blue Pill
   - VCC-GND-F407

## Tertiary Targets

I have hardware available for these targets but do not have a board or MCU definition. 

### MCUs

- STM32
   - STM32F0x
- MSP430
   - MSP430G2553
   - MSP430FR2433
- NXP
   - iMXRT1050
- Nordic
   - nRF52
- Atmel
   - SAMD51
   - SAM4L
- RISCV
   - HiFive1