//! Hardware abstraction layer for the STM32F031x family of microcontrollers

#![feature(unsize)]
#![feature(never_type)]
#![feature(associated_consts)]
#![no_std]

extern crate cortex_m;
extern crate embedded_hal as hal;
extern crate nb;
extern crate bare_take_mut as take_mut;
pub extern crate stm32f031x;

pub mod adc;
pub mod flash;
pub mod rcc;
pub mod gpio;
pub mod i2c;
pub mod pwm;
pub mod time;

