//! This crate contains sample cross-platform applications using bobbin-hal and embedded-hal.
#![no_std]

extern crate bobbin_hal;
extern crate bobbin_mcu;
#[macro_use] extern crate bobbin_sys;
extern crate bobbin_ipc;
extern crate embedded_hal;

pub mod tick;
pub mod led;
pub mod btn;
pub mod leds;
pub mod pwm;
pub mod adc;
pub mod dac;
//pub mod serial_driver;
pub mod flash;
pub mod tick_handler;
pub mod pend_handler;
pub mod ipc;

pub mod mb85rc;
pub mod mb85rs;