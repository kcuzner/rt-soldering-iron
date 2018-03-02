//! Hardware abstraction layer for the STM32F031x family of microcontrollers

#![feature(unsize)]
#![feature(never_type)]
#![no_std]

extern crate cortex_m;
extern crate embedded_hal as hal;
extern crate nb;
pub extern crate stm32f031x;

pub mod flash;
pub mod rcc;
pub mod gpio;
pub mod pwm;
pub mod time;

