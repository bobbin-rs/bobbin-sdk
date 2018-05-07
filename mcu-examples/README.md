# MCU Examples

These crates show how to use the MCU crates directly. When using MCU crates, you will have access to most peripheral functionality, but you will have to implement and instantiate your own SystemProvider if you want to have System functionality (see [board](../board/) for examples). You will also need to provide a ClockProvider implementation and external Clock types to access the clock introspection API.

